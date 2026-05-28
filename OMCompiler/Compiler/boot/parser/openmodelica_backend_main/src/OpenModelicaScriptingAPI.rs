// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CevalScript;
use openmodelica_ast::Absyn;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::Parser;
use openmodelica_frontend::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::Values;

pub static dummyMsg: Absyn::Msg = Absyn::Msg::MSG { info: SourceInfo { fileName: literal!("<interactive>"), isReadOnly: false, lineNumberStart: 1, columnNumberStart: 1, lineNumberEnd: 1, columnNumberEnd: 1, lastModification: metamodelica::OrderedFloat(0.0_f64) } };

pub fn oms_getVersion() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getVersion")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_terminate(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_terminate")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_stepUntil(mut cref: ArcStr, mut stopTime: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_stepUntil")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: stopTime.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_simulate(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_simulate")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setWorkingDirectory(mut newWorkingDir: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setWorkingDirectory")).clone(), list![Arc::new(Values::Value::STRING { string: (newWorkingDir.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setVariableStepSize(mut cref: ArcStr, mut initialStepSize: metamodelica::Real, mut minimumStepSize: metamodelica::Real, mut maximumStepSize: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setVariableStepSize")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: initialStepSize.clone() }), Arc::new(Values::Value::REAL { real: minimumStepSize.clone() }), Arc::new(Values::Value::REAL { real: maximumStepSize.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setTolerance(mut cref: ArcStr, mut absoluteTolerance: metamodelica::Real, mut relativeTolerance: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setTolerance")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: absoluteTolerance.clone() }), Arc::new(Values::Value::REAL { real: relativeTolerance.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setTLMSocketData(mut cref: ArcStr, mut address: ArcStr, mut managerPort: i32, mut monitorPort: i32) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setTLMSocketData")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (address.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: managerPort.clone() }), Arc::new(Values::Value::INTEGER { integer: monitorPort.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setTLMPositionAndOrientation(mut cref: ArcStr, mut x1: metamodelica::Real, mut x2: metamodelica::Real, mut x3: metamodelica::Real, mut A11: metamodelica::Real, mut A12: metamodelica::Real, mut A13: metamodelica::Real, mut A21: metamodelica::Real, mut A22: metamodelica::Real, mut A23: metamodelica::Real, mut A31: metamodelica::Real, mut A32: metamodelica::Real, mut A33: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setTLMPositionAndOrientation")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: x1.clone() }), Arc::new(Values::Value::REAL { real: x2.clone() }), Arc::new(Values::Value::REAL { real: x3.clone() }), Arc::new(Values::Value::REAL { real: A11.clone() }), Arc::new(Values::Value::REAL { real: A12.clone() }), Arc::new(Values::Value::REAL { real: A13.clone() }), Arc::new(Values::Value::REAL { real: A21.clone() }), Arc::new(Values::Value::REAL { real: A22.clone() }), Arc::new(Values::Value::REAL { real: A23.clone() }), Arc::new(Values::Value::REAL { real: A31.clone() }), Arc::new(Values::Value::REAL { real: A32.clone() }), Arc::new(Values::Value::REAL { real: A33.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setTempDirectory(mut newTempDir: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setTempDirectory")).clone(), list![Arc::new(Values::Value::STRING { string: (newTempDir.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setStopTime(mut cref: ArcStr, mut stopTime: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setStopTime")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: stopTime.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setStartTime(mut cref: ArcStr, mut startTime: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setStartTime")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: startTime.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setSignalFilter(mut cref: ArcStr, mut regex: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setSignalFilter")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (regex.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setResultFile(mut cref: ArcStr, mut filename: ArcStr, mut bufferSize: i32) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setResultFile")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: bufferSize.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setRealInputDerivative(mut cref: ArcStr, mut value: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setRealInputDerivative")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: value.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setReal(mut cref: ArcStr, mut value: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setReal")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: value.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setLoggingLevel(mut logLevel: i32) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setLoggingLevel")).clone(), list![Arc::new(Values::Value::INTEGER { integer: logLevel.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setLoggingInterval(mut cref: ArcStr, mut loggingInterval: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setLoggingInterval")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: loggingInterval.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setLogFile(mut filename: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setLogFile")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setInteger(mut cref: ArcStr, mut value: i32) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setInteger")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: value.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setFixedStepSize(mut cref: ArcStr, mut stepSize: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setFixedStepSize")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::REAL { real: stepSize.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setCommandLineOption(mut cmd: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setCommandLineOption")).clone(), list![Arc::new(Values::Value::STRING { string: (cmd.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_setBoolean(mut cref: ArcStr, mut value: bool) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_setBoolean")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: value.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_RunFile(mut filename: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_RunFile")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_reset(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_reset")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_rename(mut cref: ArcStr, mut newCref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_rename")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (newCref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_removeSignalsFromResults(mut cref: ArcStr, mut regex: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_removeSignalsFromResults")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (regex.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_newModel(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_newModel")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_loadSnapshot(mut cref: ArcStr, mut snapshot: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr = arcstr::literal!("");
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_loadSnapshot")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (snapshot.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_listUnconnectedConnectors(mut cref: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr = arcstr::literal!("");
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_listUnconnectedConnectors")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_list(mut cref: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr = arcstr::literal!("");
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_list")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_instantiate(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_instantiate")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_initialize(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_initialize")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_importSnapshot(mut cref: ArcStr, mut snapshot: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_importSnapshot")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (snapshot.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_importFile(mut filename: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr = arcstr::literal!("");
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_importFile")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getVariableStepSize(mut cref: ArcStr) -> Result<(metamodelica::Real, metamodelica::Real, metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res4: i32 = 0;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getVariableStepSize")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa3 }, tail: Deref @ metamodelica::List::Nil } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    res3 = __pa2.clone();
    res4 = __pa3.clone();
    Ok((res1, res2, res3, res4))
}

pub fn oms_getTolerance(mut cref: ArcStr) -> Result<(metamodelica::Real, metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res3: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getTolerance")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa2 }, tail: Deref @ metamodelica::List::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    res3 = __pa2.clone();
    Ok((res1, res2, res3))
}

pub fn oms_getSystemType(mut cref: ArcStr) -> Result<(i32, i32)> {
    let mut res1: i32 = 0;
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getSystemType")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getSubModelPath(mut cref: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr = arcstr::literal!("");
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getSubModelPath")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getStopTime(mut cref: ArcStr) -> Result<(metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getStopTime")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getStartTime(mut cref: ArcStr) -> Result<(metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getStartTime")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getSolver(mut cref: ArcStr) -> Result<(i32, i32)> {
    let mut res1: i32 = 0;
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getSolver")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getReal(mut cref: ArcStr) -> Result<(metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getReal")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getModelState(mut cref: ArcStr) -> Result<(i32, i32)> {
    let mut res1: i32 = 0;
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getModelState")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getInteger(mut cref: ArcStr, mut value: i32) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getInteger")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: value.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_getFixedStepSize(mut cref: ArcStr) -> Result<(metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getFixedStepSize")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_getBoolean(mut cref: ArcStr) -> Result<(bool, i32)> {
    let mut res1: bool = false;
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_getBoolean")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_extractFMIKind(mut filename: ArcStr) -> Result<(i32, i32)> {
    let mut res1: i32 = 0;
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_extractFMIKind")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_exportSnapshot(mut cref: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr = arcstr::literal!("");
    let mut res2: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_exportSnapshot")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn oms_exportDependencyGraphs(mut cref: ArcStr, mut initialization: ArcStr, mut event: ArcStr, mut simulation: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_exportDependencyGraphs")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (initialization.clone()).clone() }), Arc::new(Values::Value::STRING { string: (event.clone()).clone() }), Arc::new(Values::Value::STRING { string: (simulation.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_export(mut cref: ArcStr, mut filename: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_export")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (filename.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_deleteConnectorFromTLMBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_deleteConnectorFromTLMBus")).clone(), list![Arc::new(Values::Value::STRING { string: (busCref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (connectorCref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_deleteConnectorFromBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_deleteConnectorFromBus")).clone(), list![Arc::new(Values::Value::STRING { string: (busCref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (connectorCref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_deleteConnection(mut crefA: ArcStr, mut crefB: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_deleteConnection")).clone(), list![Arc::new(Values::Value::STRING { string: (crefA.clone()).clone() }), Arc::new(Values::Value::STRING { string: (crefB.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_delete(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_delete")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_copySystem(mut source: ArcStr, mut target: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_copySystem")).clone(), list![Arc::new(Values::Value::STRING { string: (source.clone()).clone() }), Arc::new(Values::Value::STRING { string: (target.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_compareSimulationResults(mut filenameA: ArcStr, mut filenameB: ArcStr, mut var: ArcStr, mut relTol: metamodelica::Real, mut absTol: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_compareSimulationResults")).clone(), list![Arc::new(Values::Value::STRING { string: (filenameA.clone()).clone() }), Arc::new(Values::Value::STRING { string: (filenameB.clone()).clone() }), Arc::new(Values::Value::STRING { string: (var.clone()).clone() }), Arc::new(Values::Value::REAL { real: relTol.clone() }), Arc::new(Values::Value::REAL { real: absTol.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addTLMConnection(mut crefA: ArcStr, mut crefB: ArcStr, mut delay: metamodelica::Real, mut alpha: metamodelica::Real, mut linearimpedance: metamodelica::Real, mut angularimpedance: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addTLMConnection")).clone(), list![Arc::new(Values::Value::STRING { string: (crefA.clone()).clone() }), Arc::new(Values::Value::STRING { string: (crefB.clone()).clone() }), Arc::new(Values::Value::REAL { real: delay.clone() }), Arc::new(Values::Value::REAL { real: alpha.clone() }), Arc::new(Values::Value::REAL { real: linearimpedance.clone() }), Arc::new(Values::Value::REAL { real: angularimpedance.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addTimeIndicator(mut signal: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addTimeIndicator")).clone(), list![Arc::new(Values::Value::STRING { string: (signal.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addSubModel(mut cref: ArcStr, mut fmuPath: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addSubModel")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (fmuPath.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addStaticValueIndicator(mut signal: ArcStr, mut lower: metamodelica::Real, mut upper: metamodelica::Real, mut stepSize: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addStaticValueIndicator")).clone(), list![Arc::new(Values::Value::STRING { string: (signal.clone()).clone() }), Arc::new(Values::Value::REAL { real: lower.clone() }), Arc::new(Values::Value::REAL { real: upper.clone() }), Arc::new(Values::Value::REAL { real: stepSize.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addSignalsToResults(mut cref: ArcStr, mut regex: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addSignalsToResults")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (regex.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addExternalModel(mut cref: ArcStr, mut path: ArcStr, mut startscript: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addExternalModel")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (path.clone()).clone() }), Arc::new(Values::Value::STRING { string: (startscript.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addEventIndicator(mut signal: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addEventIndicator")).clone(), list![Arc::new(Values::Value::STRING { string: (signal.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addDynamicValueIndicator(mut signal: ArcStr, mut lower: ArcStr, mut upper: ArcStr, mut stepSize: metamodelica::Real) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addDynamicValueIndicator")).clone(), list![Arc::new(Values::Value::STRING { string: (signal.clone()).clone() }), Arc::new(Values::Value::STRING { string: (lower.clone()).clone() }), Arc::new(Values::Value::STRING { string: (upper.clone()).clone() }), Arc::new(Values::Value::REAL { real: stepSize.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addConnectorToTLMBus(mut busCref: ArcStr, mut connectorCref: ArcStr, mut type_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addConnectorToTLMBus")).clone(), list![Arc::new(Values::Value::STRING { string: (busCref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (connectorCref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (type_.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addConnectorToBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addConnectorToBus")).clone(), list![Arc::new(Values::Value::STRING { string: (busCref.clone()).clone() }), Arc::new(Values::Value::STRING { string: (connectorCref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addConnection(mut crefA: ArcStr, mut crefB: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addConnection")).clone(), list![Arc::new(Values::Value::STRING { string: (crefA.clone()).clone() }), Arc::new(Values::Value::STRING { string: (crefB.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn oms_addBus(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("oms_addBus")).clone(), list![Arc::new(Values::Value::STRING { string: (cref.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn unloadOMSimulator() -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("unloadOMSimulator")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn loadOMSimulator() -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadOMSimulator")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn reverseLookup(mut name: ArcStr, mut scope: ArcStr, mut exactMatch: bool, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("reverseLookup")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((name.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((scope.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: exactMatch.clone() }), Arc::new(Values::Value::BOOL { boolean: prettyPrint.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getDefinitions(mut addFunctions: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getDefinitions")).clone(), list![Arc::new(Values::Value::BOOL { boolean: addFunctions.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn qualifyPath(mut classPath: ArcStr, mut path: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let mut res_path: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("qualifyPath")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((classPath.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((path.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res_path = __pa0.clone();
    res = (AbsynUtil::pathString(res_path.clone(), (literal!(".")).clone(), true, false)?).clone();
    Ok(res)
}

pub fn restoreAST(mut id: i32) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("restoreAST")).clone(), list![Arc::new(Values::Value::INTEGER { integer: id.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn storeAST() -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("storeAST")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn modifierToJSON(mut modifier: ArcStr, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("modifierToJSON")).clone(), list![Arc::new(Values::Value::STRING { string: (modifier.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: prettyPrint.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getModelInstanceAnnotation(mut className: ArcStr, mut filter: Arc<metamodelica::List<ArcStr>>, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getModelInstanceAnnotation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut filter_iter in (filter.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (filter_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::BOOL { boolean: prettyPrint.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getModelInstance(mut className: ArcStr, mut context: ArcStr, mut modifier: ArcStr, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getModelInstance")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((context.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (modifier.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: prettyPrint.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn convertPackageToLibrary(mut packageToConvert: ArcStr, mut library: ArcStr, mut libraryVersion: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("convertPackageToLibrary")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((packageToConvert.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((library.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (libraryVersion.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn runConversionScript(mut packageToConvert: ArcStr, mut scriptFile: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("runConversionScript")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((packageToConvert.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (scriptFile.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn generateScriptingAPI(mut cl: ArcStr, mut name: ArcStr) -> Result<(bool, ArcStr, ArcStr, ArcStr)> {
    let mut res1: bool = false;
    let mut res2: ArcStr = arcstr::literal!("");
    let mut res3: ArcStr = arcstr::literal!("");
    let mut res4: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateScriptingAPI")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (name.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa3 }, tail: Deref @ metamodelica::List::Nil } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    res3 = __pa2.clone();
    res4 = __pa3.clone();
    Ok((res1, res2, res3, res4))
}

pub fn deleteInitialState(mut cl: ArcStr, mut state: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("deleteInitialState")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (state.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getInitialStates(mut cl: ArcStr) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInitialStates")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter_iter in (ValuesUtil::arrayValues(res_arr_iter.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn deleteTransition(mut cl: ArcStr, mut from: ArcStr, mut to: ArcStr, mut condition: ArcStr, mut immediate: bool, mut reset: bool, mut synchronize: bool, mut priority: i32) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("deleteTransition")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (from.clone()).clone() }), Arc::new(Values::Value::STRING { string: (to.clone()).clone() }), Arc::new(Values::Value::STRING { string: (condition.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: immediate.clone() }), Arc::new(Values::Value::BOOL { boolean: reset.clone() }), Arc::new(Values::Value::BOOL { boolean: synchronize.clone() }), Arc::new(Values::Value::INTEGER { integer: priority.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getTransitions(mut cl: ArcStr) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getTransitions")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter_iter in (ValuesUtil::arrayValues(res_arr_iter.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getEnumerationLiterals(mut className: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getEnumerationLiterals")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getDefaultComponentPrefixes(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getDefaultComponentPrefixes")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getDefaultComponentName(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getDefaultComponentName")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getClassInformation(mut cl: ArcStr) -> Result<(ArcStr, ArcStr, bool, bool, bool, ArcStr, bool, i32, i32, i32, i32, Arc<metamodelica::List<ArcStr>>, bool, bool, ArcStr, ArcStr, bool, ArcStr, ArcStr, ArcStr, ArcStr, ArcStr)> {
    let mut res1: ArcStr = arcstr::literal!("");
    let mut res2: ArcStr = arcstr::literal!("");
    let mut res3: bool = false;
    let mut res4: bool = false;
    let mut res5: bool = false;
    let mut res6: ArcStr = arcstr::literal!("");
    let mut res7: bool = false;
    let mut res8: i32 = 0;
    let mut res9: i32 = 0;
    let mut res10: i32 = 0;
    let mut res11: i32 = 0;
    let mut res12: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res13: bool = false;
    let mut res14: bool = false;
    let mut res15: ArcStr = arcstr::literal!("");
    let mut res16: ArcStr = arcstr::literal!("");
    let mut res17: bool = false;
    let mut res18: ArcStr = arcstr::literal!("");
    let mut res19: ArcStr = arcstr::literal!("");
    let mut res20: ArcStr = arcstr::literal!("");
    let mut res21: ArcStr = arcstr::literal!("");
    let mut res22: ArcStr = arcstr::literal!("");
    let mut res12_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15, __pa16, __pa17, __pa18, __pa19, __pa20, __pa21) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getClassInformation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa4 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa5 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa6 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa7 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa8 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa9 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa10 }, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa12 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa13 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa14 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa15 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa16 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa17 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa18 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa19 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa20 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa21 }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } } } } } } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    res3 = __pa2.clone();
    res4 = __pa3.clone();
    res5 = __pa4.clone();
    res6 = __pa5.clone();
    res7 = __pa6.clone();
    res8 = __pa7.clone();
    res9 = __pa8.clone();
    res10 = __pa9.clone();
    res11 = __pa10.clone();
    res12_arr = __pa11.clone();
    res13 = __pa12.clone();
    res14 = __pa13.clone();
    res15 = __pa14.clone();
    res16 = __pa15.clone();
    res17 = __pa16.clone();
    res18 = __pa17.clone();
    res19 = __pa18.clone();
    res20 = __pa19.clone();
    res21 = __pa20.clone();
    res22 = __pa21.clone();
    res12 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res12_arr_iter in (ValuesUtil::arrayValues(res12_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res12_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res12_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2, res3, res4, res5, res6, res7, res8, res9, res10, res11, res12, res13, res14, res15, res16, res17, res18, res19, res20, res21, res22))
}

pub fn sortStrings(mut arr: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("sortStrings")).clone(), list![ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut arr_iter in (arr.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (arr_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn checkInterfaceOfPackages(mut cl: ArcStr, mut dependencyMatrix: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("checkInterfaceOfPackages")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut dependencyMatrix_iter in (dependencyMatrix.clone()).into_iter().cloned() {
            let __x = ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut dependencyMatrix_iter_iter in (dependencyMatrix_iter.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (dependencyMatrix_iter_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn GC_set_max_heap_size(mut size: i32) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("GC_set_max_heap_size")).clone(), list![Arc::new(Values::Value::INTEGER { integer: size.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn GC_expand_hp(mut size: i32) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("GC_expand_hp")).clone(), list![Arc::new(Values::Value::INTEGER { integer: size.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn GC_gcollect_and_unmap() -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("GC_gcollect_and_unmap")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL) => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn getMemorySize() -> Result<metamodelica::Real> {
    let mut res: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getMemorySize")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn threadWorkFailed() -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("threadWorkFailed")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL) => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn exit(mut status: i32) -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("exit")).clone(), list![Arc::new(Values::Value::INTEGER { integer: status.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL) => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn runScriptParallel(mut scripts: Arc<metamodelica::List<ArcStr>>, mut numThreads: i32, mut useThreads: bool) -> Result<Arc<metamodelica::List<bool>>> {
    let mut res: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("runScriptParallel")).clone(), list![ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut scripts_iter in (scripts.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (scripts_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::INTEGER { integer: numThreads.clone() }), Arc::new(Values::Value::BOOL { boolean: useThreads.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::BOOL { .. } => var_field!((*res_arr_iter).boolean, Values::Value::BOOL).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn numProcessors() -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("numProcessors")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn generateEntryPoint(mut fileName: ArcStr, mut entryPoint: ArcStr, mut url: ArcStr) -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateEntryPoint")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((entryPoint.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (url.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL) => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn getDerivedClassModifierValue(mut className: ArcStr, mut modifierName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getDerivedClassModifierValue")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((modifierName.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getDerivedClassModifierNames(mut className: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getDerivedClassModifierNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getConversionsFromVersions(mut pack: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut res1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res1_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut res2_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getConversionsFromVersions")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((pack.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1_arr = __pa0.clone();
    res2_arr = __pa1.clone();
    res1 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(res1_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res1_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res1_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(res2_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res2_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res2_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getUses(mut pack: ArcStr) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getUses")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((pack.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter_iter in (ValuesUtil::arrayValues(res_arr_iter.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn upgradeInstalledPackages(mut installNewestVersions: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("upgradeInstalledPackages")).clone(), list![Arc::new(Values::Value::BOOL { boolean: installNewestVersions.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAvailablePackageConversionsFrom(mut pkg: ArcStr, mut version: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAvailablePackageConversionsFrom")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((pkg.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (version.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getAvailablePackageConversionsTo(mut pkg: ArcStr, mut version: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAvailablePackageConversionsTo")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((pkg.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (version.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getAvailablePackageVersions(mut pkg: ArcStr, mut version: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAvailablePackageVersions")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((pkg.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (version.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn updatePackageIndex() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("updatePackageIndex")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn installPackage(mut pkg: ArcStr, mut version: ArcStr, mut exactMatch: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("installPackage")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((pkg.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (version.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: exactMatch.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAvailableLibraryVersions(mut libraryName: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAvailableLibraryVersions")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((libraryName.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getAvailableLibraries() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAvailableLibraries")).clone(), metamodelica::nil(), dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn searchClassNames(mut searchText: ArcStr, mut findInText: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("searchClassNames")).clone(), list![Arc::new(Values::Value::STRING { string: (searchText.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: findInText.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn extendsFrom(mut className: ArcStr, mut baseClassName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("extendsFrom")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((baseClassName.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getBooleanClassAnnotation(mut className: ArcStr, mut annotationName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getBooleanClassAnnotation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((annotationName.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn classAnnotationExists(mut className: ArcStr, mut annotationName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("classAnnotationExists")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((annotationName.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAnnotationModifierValue(mut className: ArcStr, mut annotationName: ArcStr, mut modifierName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAnnotationModifierValue")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (annotationName.clone()).clone() }), Arc::new(Values::Value::STRING { string: (modifierName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAnnotationNamedModifiers(mut className: ArcStr, mut annotationName: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAnnotationNamedModifiers")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (annotationName.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getSimulationOptions(mut name: ArcStr, mut defaultStartTime: metamodelica::Real, mut defaultStopTime: metamodelica::Real, mut defaultTolerance: metamodelica::Real, mut defaultNumberOfIntervals: i32, mut defaultInterval: metamodelica::Real) -> Result<(metamodelica::Real, metamodelica::Real, metamodelica::Real, i32, metamodelica::Real)> {
    let mut res1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res4: i32 = 0;
    let mut res5: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getSimulationOptions")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((name.clone()).clone())? }) }), Arc::new(Values::Value::REAL { real: defaultStartTime.clone() }), Arc::new(Values::Value::REAL { real: defaultStopTime.clone() }), Arc::new(Values::Value::REAL { real: defaultTolerance.clone() }), Arc::new(Values::Value::INTEGER { integer: defaultNumberOfIntervals.clone() }), Arc::new(Values::Value::REAL { real: defaultInterval.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa4 }, tail: Deref @ metamodelica::List::Nil } } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    res3 = __pa2.clone();
    res4 = __pa3.clone();
    res5 = __pa4.clone();
    Ok((res1, res2, res3, res4, res5))
}

pub fn isExperiment(mut name: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isExperiment")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((name.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthInheritedClass(mut className: ArcStr, mut n: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let mut res_path: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthInheritedClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: n.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res_path = __pa0.clone();
    res = (AbsynUtil::pathString(res_path.clone(), (literal!(".")).clone(), true, false)?).clone();
    Ok(res)
}

pub fn getInheritedClasses(mut name: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInheritedClasses")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((name.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getInheritanceCount(mut className: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInheritanceCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isProtected(mut componentName: ArcStr, mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isProtected")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((componentName.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isConstant(mut componentName: ArcStr, mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isConstant")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((componentName.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isParameter(mut componentName: ArcStr, mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isParameter")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((componentName.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isPrimitive(mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isPrimitive")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getBuiltinType(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getBuiltinType")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isProtectedClass(mut cl: ArcStr, mut c2: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isProtectedClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (c2.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isOperatorFunction(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isOperatorFunction")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isOperatorRecord(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isOperatorRecord")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isOperator(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isOperator")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isEnumeration(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isEnumeration")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isOptimization(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isOptimization")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isConnector(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isConnector")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isModel(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isModel")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isRedeclare(mut element: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isRedeclare")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((element.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isReplaceable(mut element: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isReplaceable")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((element.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isPartial(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isPartial")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isFunction(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isFunction")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isBlock(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isBlock")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isRecord(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isRecord")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isClass(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isPackage(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isPackage")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isType(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isType")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getClassRestriction(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getClassRestriction")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn existClass(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("existClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn basename(mut path: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("basename")).clone(), list![Arc::new(Values::Value::STRING { string: (path.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn dirname(mut path: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("dirname")).clone(), list![Arc::new(Values::Value::STRING { string: (path.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getClassComment(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getClassComment")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn typeNameStrings(mut cl: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("typeNameStrings")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn typeNameString(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("typeNameString")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn stringTypeName(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let mut res_path: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("stringTypeName")).clone(), list![Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res_path = __pa0.clone();
    res = (AbsynUtil::pathString(res_path.clone(), (literal!(".")).clone(), true, false)?).clone();
    Ok(res)
}

pub fn getTimeStamp(mut cl: ArcStr) -> Result<(metamodelica::Real, ArcStr)> {
    let mut res1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res2: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getTimeStamp")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn setDocumentationAnnotation(mut class_: ArcStr, mut info: ArcStr, mut revisions: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setDocumentationAnnotation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (info.clone()).clone() }), Arc::new(Values::Value::STRING { string: (revisions.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getDocumentationAnnotation(mut cl: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getDocumentationAnnotation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn iconv(mut string: ArcStr, mut from: ArcStr, mut to: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("iconv")).clone(), list![Arc::new(Values::Value::STRING { string: (string.clone()).clone() }), Arc::new(Values::Value::STRING { string: (from.clone()).clone() }), Arc::new(Values::Value::STRING { string: (to.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthImport(mut class_: ArcStr, mut index: i32) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthImport")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getImportedNames(mut class_: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut res1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res1_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut res2_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getImportedNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1_arr = __pa0.clone();
    res2_arr = __pa1.clone();
    res1 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(res1_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res1_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res1_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(res2_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res2_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res2_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getMMfileTotalDependencies(mut in_package_name: ArcStr, mut public_imports_dir: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getMMfileTotalDependencies")).clone(), list![Arc::new(Values::Value::STRING { string: (in_package_name.clone()).clone() }), Arc::new(Values::Value::STRING { string: (public_imports_dir.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getImportCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getImportCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthAnnotationString(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthAnnotationString")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAnnotationCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAnnotationCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthInitialEquationItem(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthInitialEquationItem")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getInitialEquationItemsCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInitialEquationItemsCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthEquationItem(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthEquationItem")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getEquationItemsCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getEquationItemsCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthInitialEquation(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthInitialEquation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getInitialEquationCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInitialEquationCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthEquation(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthEquation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getEquationCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getEquationCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthInitialAlgorithmItem(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthInitialAlgorithmItem")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getInitialAlgorithmItemsCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInitialAlgorithmItemsCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthAlgorithmItem(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthAlgorithmItem")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAlgorithmItemsCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAlgorithmItemsCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthInitialAlgorithm(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthInitialAlgorithm")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getInitialAlgorithmCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInitialAlgorithmCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthAlgorithm(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthAlgorithm")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAlgorithmCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAlgorithmCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn updateEquation(mut className: ArcStr, mut oldEq: ArcStr, mut newEq: ArcStr, mut matchAll: bool, mut matchShallow: bool, mut matchDescription: bool, mut mergeDescription: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("updateEquation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (oldEq.clone()).clone() }), Arc::new(Values::Value::STRING { string: (newEq.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: matchAll.clone() }), Arc::new(Values::Value::BOOL { boolean: matchShallow.clone() }), Arc::new(Values::Value::BOOL { boolean: matchDescription.clone() }), Arc::new(Values::Value::BOOL { boolean: mergeDescription.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn addEquation(mut className: ArcStr, mut eq: ArcStr, mut isInitial: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("addEquation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (eq.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: isInitial.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getConnectionList(mut className: ArcStr) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getConnectionList")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter_iter in (ValuesUtil::arrayValues(res_arr_iter.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getNthConnection(mut className: ArcStr, mut index: i32) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthConnection")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: index.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getConnectionCount(mut className: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getConnectionCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn updateConnectionNames(mut className: ArcStr, mut from: ArcStr, mut to: ArcStr, mut fromNew: ArcStr, mut toNew: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("updateConnectionNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (from.clone()).clone() }), Arc::new(Values::Value::STRING { string: (to.clone()).clone() }), Arc::new(Values::Value::STRING { string: (fromNew.clone()).clone() }), Arc::new(Values::Value::STRING { string: (toNew.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn updateConnectionAnnotation(mut className: ArcStr, mut from: ArcStr, mut to: ArcStr, mut annotate: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("updateConnectionAnnotation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (from.clone()).clone() }), Arc::new(Values::Value::STRING { string: (to.clone()).clone() }), Arc::new(Values::Value::STRING { string: (annotate.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getConnectorCount(mut className: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getConnectorCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setComponentProperties(mut className: ArcStr, mut componentName: ArcStr, mut prefixArray: Arc<metamodelica::List<bool>>, mut variability: Arc<metamodelica::List<ArcStr>>, mut innerOuter: Arc<metamodelica::List<bool>>, mut direction: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setComponentProperties")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((componentName.clone()).clone())? }) }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut prefixArray_iter in (prefixArray.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::BOOL { boolean: prefixArray_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut variability_iter in (variability.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (variability_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut innerOuter_iter in (innerOuter.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::BOOL { boolean: innerOuter_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut direction_iter in (direction.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (direction_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setComponentComment(mut className: ArcStr, mut componentName: ArcStr, mut comment: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setComponentComment")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((componentName.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (comment.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getComponentComment(mut className: ArcStr, mut componentName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getComponentComment")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((componentName.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn removeExtendsModifiers(mut className: ArcStr, mut baseClassName: ArcStr, mut keepRedeclares: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("removeExtendsModifiers")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((baseClassName.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: keepRedeclares.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getInstantiatedParametersAndValues(mut cls: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInstantiatedParametersAndValues")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cls.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getElementAnnotation(mut elementName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getElementAnnotation")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((elementName.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNthComponentCondition(mut className: ArcStr, mut n: i32) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNthComponentCondition")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: n.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getComponentCount(mut classPath: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getComponentCount")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((classPath.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isExtendsModifierFinal(mut className: ArcStr, mut extendsName: ArcStr, mut modifierName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isExtendsModifierFinal")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((extendsName.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((modifierName.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn removeElementModifiers(mut className: ArcStr, mut componentName: ArcStr, mut keepRedeclares: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("removeElementModifiers")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (componentName.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: keepRedeclares.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getElementModifierValues(mut className: ArcStr, mut modifier: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getElementModifierValues")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((modifier.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getElementModifierValue(mut className: ArcStr, mut modifier: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getElementModifierValue")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((modifier.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getExtendsModifierNames(mut className: ArcStr, mut extendsName: ArcStr, mut useQuotes: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getExtendsModifierNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((extendsName.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: useQuotes.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getElementModifierNames(mut className: ArcStr, mut elementName: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getElementModifierNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (elementName.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn removeComponentModifiers(mut class_: ArcStr, mut componentName: ArcStr, mut keepRedeclares: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("removeComponentModifiers")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (componentName.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: keepRedeclares.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getComponentModifierValues(mut class_: ArcStr, mut modifier: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getComponentModifierValues")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((modifier.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getComponentModifierValue(mut class_: ArcStr, mut modifier: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getComponentModifierValue")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((modifier.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getComponentModifierNames(mut class_: ArcStr, mut componentName: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getComponentModifierNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (componentName.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getElements(mut className: ArcStr, mut useQuotes: bool) -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getElements")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: useQuotes.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL) => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn getComponents(mut className: ArcStr, mut useQuotes: bool) -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getComponents")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: useQuotes.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL) => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn getParameterValue(mut class_: ArcStr, mut parameterName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getParameterValue")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (parameterName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getParameterNames(mut class_: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getParameterNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn deleteComponent(mut componentName: ArcStr, mut classPath: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("deleteComponent")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((componentName.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((classPath.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn closeSimulationResultFile() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("closeSimulationResultFile")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn checkCodeGraph(mut graphfile: ArcStr, mut codefile: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("checkCodeGraph")).clone(), list![Arc::new(Values::Value::STRING { string: (graphfile.clone()).clone() }), Arc::new(Values::Value::STRING { string: (codefile.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn checkTaskGraph(mut filename: ArcStr, mut reffilename: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("checkTaskGraph")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (reffilename.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn diffSimulationResultsHtml(mut var: ArcStr, mut actualFile: ArcStr, mut expectedFile: ArcStr, mut relTol: metamodelica::Real, mut relTolDiffMinMax: metamodelica::Real, mut rangeDelta: metamodelica::Real) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("diffSimulationResultsHtml")).clone(), list![Arc::new(Values::Value::STRING { string: (var.clone()).clone() }), Arc::new(Values::Value::STRING { string: (actualFile.clone()).clone() }), Arc::new(Values::Value::STRING { string: (expectedFile.clone()).clone() }), Arc::new(Values::Value::REAL { real: relTol.clone() }), Arc::new(Values::Value::REAL { real: relTolDiffMinMax.clone() }), Arc::new(Values::Value::REAL { real: rangeDelta.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn diffSimulationResults(mut actualFile: ArcStr, mut expectedFile: ArcStr, mut diffPrefix: ArcStr, mut relTol: metamodelica::Real, mut relTolDiffMinMax: metamodelica::Real, mut rangeDelta: metamodelica::Real, mut vars: Arc<metamodelica::List<ArcStr>>, mut keepEqualResults: bool) -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> {
    let mut res1: bool = false;
    let mut res2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res2_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("diffSimulationResults")).clone(), list![Arc::new(Values::Value::STRING { string: (actualFile.clone()).clone() }), Arc::new(Values::Value::STRING { string: (expectedFile.clone()).clone() }), Arc::new(Values::Value::STRING { string: (diffPrefix.clone()).clone() }), Arc::new(Values::Value::REAL { real: relTol.clone() }), Arc::new(Values::Value::REAL { real: relTolDiffMinMax.clone() }), Arc::new(Values::Value::REAL { real: rangeDelta.clone() }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut vars_iter in (vars.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (vars_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::BOOL { boolean: keepEqualResults.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2_arr = __pa1.clone();
    res2 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(res2_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res2_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res2_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn deltaSimulationResults(mut filename: ArcStr, mut reffilename: ArcStr, mut method: ArcStr, mut vars: Arc<metamodelica::List<ArcStr>>) -> Result<metamodelica::Real> {
    let mut res: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("deltaSimulationResults")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (reffilename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (method.clone()).clone() }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut vars_iter in (vars.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (vars_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn compareSimulationResults(mut filename: ArcStr, mut reffilename: ArcStr, mut logfilename: ArcStr, mut relTol: metamodelica::Real, mut absTol: metamodelica::Real, mut vars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("compareSimulationResults")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (reffilename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (logfilename.clone()).clone() }), Arc::new(Values::Value::REAL { real: relTol.clone() }), Arc::new(Values::Value::REAL { real: absTol.clone() }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut vars_iter in (vars.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (vars_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn filterSimulationResults(mut inFile: ArcStr, mut outFile: ArcStr, mut vars: Arc<metamodelica::List<ArcStr>>, mut numberOfIntervals: i32, mut removeDescription: bool, mut hintReadAllVars: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("filterSimulationResults")).clone(), list![Arc::new(Values::Value::STRING { string: (inFile.clone()).clone() }), Arc::new(Values::Value::STRING { string: (outFile.clone()).clone() }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut vars_iter in (vars.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (vars_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::INTEGER { integer: numberOfIntervals.clone() }), Arc::new(Values::Value::BOOL { boolean: removeDescription.clone() }), Arc::new(Values::Value::BOOL { boolean: hintReadAllVars.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn readSimulationResultVars(mut fileName: ArcStr, mut readParameters: bool, mut openmodelicaStyle: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("readSimulationResultVars")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: readParameters.clone() }), Arc::new(Values::Value::BOOL { boolean: openmodelicaStyle.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn readSimulationResultSize(mut fileName: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("readSimulationResultSize")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn plotAll(mut externalWindow: bool, mut fileName: ArcStr, mut title: ArcStr, mut grid: ArcStr, mut logX: bool, mut logY: bool, mut xLabel: ArcStr, mut yLabel: ArcStr, mut xRange: Arc<metamodelica::List<metamodelica::Real>>, mut yRange: Arc<metamodelica::List<metamodelica::Real>>, mut curveWidth: metamodelica::Real, mut curveStyle: i32, mut legendPosition: ArcStr, mut footer: ArcStr, mut autoScale: bool, mut forceOMPlot: bool, mut yAxis: ArcStr, mut yLabelRight: ArcStr, mut yRangeRight: Arc<metamodelica::List<metamodelica::Real>>) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("plotAll")).clone(), list![Arc::new(Values::Value::BOOL { boolean: externalWindow.clone() }), Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::STRING { string: (title.clone()).clone() }), Arc::new(Values::Value::STRING { string: (grid.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: logX.clone() }), Arc::new(Values::Value::BOOL { boolean: logY.clone() }), Arc::new(Values::Value::STRING { string: (xLabel.clone()).clone() }), Arc::new(Values::Value::STRING { string: (yLabel.clone()).clone() }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut xRange_iter in (xRange.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::REAL { real: xRange_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut yRange_iter in (yRange.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::REAL { real: yRange_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::REAL { real: curveWidth.clone() }), Arc::new(Values::Value::INTEGER { integer: curveStyle.clone() }), Arc::new(Values::Value::STRING { string: (legendPosition.clone()).clone() }), Arc::new(Values::Value::STRING { string: (footer.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: autoScale.clone() }), Arc::new(Values::Value::BOOL { boolean: forceOMPlot.clone() }), Arc::new(Values::Value::STRING { string: (yAxis.clone()).clone() }), Arc::new(Values::Value::STRING { string: (yLabelRight.clone()).clone() }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut yRangeRight_iter in (yRangeRight.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::REAL { real: yRangeRight_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getReplaceableChoices(mut baseClass: ArcStr, mut parentClass: ArcStr, mut includePartial: bool, mut sort: bool) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getReplaceableChoices")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((baseClass.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((parentClass.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: includePartial.clone() }), Arc::new(Values::Value::BOOL { boolean: sort.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter_iter in (ValuesUtil::arrayValues(res_arr_iter.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getAllSubtypeOf(mut className: ArcStr, mut parentClass: ArcStr, mut qualified: bool, mut includePartial: bool, mut sort: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAllSubtypeOf")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((parentClass.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: qualified.clone() }), Arc::new(Values::Value::BOOL { boolean: includePartial.clone() }), Arc::new(Values::Value::BOOL { boolean: sort.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getPackages(mut class_: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getPackages")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getUsedClassNames(mut className: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getUsedClassNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getClassNames(mut class_: ArcStr, mut recursive: bool, mut qualified: bool, mut sort: bool, mut builtin: bool, mut showProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getClassNames")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: recursive.clone() }), Arc::new(Values::Value::BOOL { boolean: qualified.clone() }), Arc::new(Values::Value::BOOL { boolean: sort.clone() }), Arc::new(Values::Value::BOOL { boolean: builtin.clone() }), Arc::new(Values::Value::BOOL { boolean: showProtected.clone() }), Arc::new(Values::Value::BOOL { boolean: includeConstants.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn setClassComment(mut class_: ArcStr, mut filename: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setClassComment")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (filename.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn isShortDefinition(mut class_: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("isShortDefinition")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setSourceFile(mut class_: ArcStr, mut filename: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setSourceFile")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (filename.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getSourceFile(mut class_: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getSourceFile")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn refactorClass(mut className: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("refactorClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn deleteClass(mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("deleteClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn renameClass(mut oldName: ArcStr, mut newName: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("renameClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((oldName.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((newName.clone()).clone())? }) })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn copyClass(mut className: ArcStr, mut newClassName: ArcStr, mut withIn: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("copyClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (newClassName.clone()).clone() }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((withIn.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn moveClassToBottom(mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("moveClassToBottom")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn moveClassToTop(mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("moveClassToTop")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn moveClass(mut className: ArcStr, mut offset: i32) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("moveClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: offset.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn newModel(mut className: ArcStr, mut withinPath: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("newModel")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((withinPath.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn createModel(mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("createModel")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn translateResidualsDAE(mut className: ArcStr, mut fileNamePrefix: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("translateResidualsDAE")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (fileNamePrefix.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn reduceTerms(mut className: ArcStr, mut startTime: metamodelica::Real, mut stopTime: metamodelica::Real, mut numberOfIntervals: i32, mut tolerance: metamodelica::Real, mut method: ArcStr, mut fileNamePrefix: ArcStr, mut options: ArcStr, mut outputFormat: ArcStr, mut variableFilter: ArcStr, mut cflags: ArcStr, mut simflags: ArcStr, mut labelstoCancel: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("reduceTerms")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::REAL { real: startTime.clone() }), Arc::new(Values::Value::REAL { real: stopTime.clone() }), Arc::new(Values::Value::INTEGER { integer: numberOfIntervals.clone() }), Arc::new(Values::Value::REAL { real: tolerance.clone() }), Arc::new(Values::Value::STRING { string: (method.clone()).clone() }), Arc::new(Values::Value::STRING { string: (fileNamePrefix.clone()).clone() }), Arc::new(Values::Value::STRING { string: (options.clone()).clone() }), Arc::new(Values::Value::STRING { string: (outputFormat.clone()).clone() }), Arc::new(Values::Value::STRING { string: (variableFilter.clone()).clone() }), Arc::new(Values::Value::STRING { string: (cflags.clone()).clone() }), Arc::new(Values::Value::STRING { string: (simflags.clone()).clone() }), Arc::new(Values::Value::STRING { string: (labelstoCancel.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn buildLabel(mut className: ArcStr, mut startTime: metamodelica::Real, mut stopTime: metamodelica::Real, mut numberOfIntervals: i32, mut tolerance: metamodelica::Real, mut method: ArcStr, mut fileNamePrefix: ArcStr, mut options: ArcStr, mut outputFormat: ArcStr, mut variableFilter: ArcStr, mut cflags: ArcStr, mut simflags: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("buildLabel")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::REAL { real: startTime.clone() }), Arc::new(Values::Value::REAL { real: stopTime.clone() }), Arc::new(Values::Value::INTEGER { integer: numberOfIntervals.clone() }), Arc::new(Values::Value::REAL { real: tolerance.clone() }), Arc::new(Values::Value::STRING { string: (method.clone()).clone() }), Arc::new(Values::Value::STRING { string: (fileNamePrefix.clone()).clone() }), Arc::new(Values::Value::STRING { string: (options.clone()).clone() }), Arc::new(Values::Value::STRING { string: (outputFormat.clone()).clone() }), Arc::new(Values::Value::STRING { string: (variableFilter.clone()).clone() }), Arc::new(Values::Value::STRING { string: (cflags.clone()).clone() }), Arc::new(Values::Value::STRING { string: (simflags.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn buildEncryptedPackage(mut className: ArcStr, mut encrypt: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("buildEncryptedPackage")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: encrypt.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn buildModelFMU(mut className: ArcStr, mut version: ArcStr, mut fmuType: ArcStr, mut fileNamePrefix: ArcStr, mut platforms: Arc<metamodelica::List<ArcStr>>, mut includeResources: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("buildModelFMU")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (version.clone()).clone() }), Arc::new(Values::Value::STRING { string: (fmuType.clone()).clone() }), Arc::new(Values::Value::STRING { string: (fileNamePrefix.clone()).clone() }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut platforms_iter in (platforms.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (platforms_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::BOOL { boolean: includeResources.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn translateModelFMU(mut className: ArcStr, mut version: ArcStr, mut fmuType: ArcStr, mut fileNamePrefix: ArcStr, mut platforms: Arc<metamodelica::List<ArcStr>>, mut includeResources: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("translateModelFMU")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (version.clone()).clone() }), Arc::new(Values::Value::STRING { string: (fmuType.clone()).clone() }), Arc::new(Values::Value::STRING { string: (fileNamePrefix.clone()).clone() }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut platforms_iter in (platforms.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (platforms_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::BOOL { boolean: includeResources.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn importFMUModelDescription(mut filename: ArcStr, mut workdir: ArcStr, mut loglevel: i32, mut fullPath: bool, mut debugLogging: bool, mut generateInputConnectors: bool, mut generateOutputConnectors: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("importFMUModelDescription")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (workdir.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: loglevel.clone() }), Arc::new(Values::Value::BOOL { boolean: fullPath.clone() }), Arc::new(Values::Value::BOOL { boolean: debugLogging.clone() }), Arc::new(Values::Value::BOOL { boolean: generateInputConnectors.clone() }), Arc::new(Values::Value::BOOL { boolean: generateOutputConnectors.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn importFMU(mut filename: ArcStr, mut workdir: ArcStr, mut loglevel: i32, mut fullPath: bool, mut debugLogging: bool, mut generateInputConnectors: bool, mut generateOutputConnectors: bool, mut modelName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("importFMU")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (workdir.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: loglevel.clone() }), Arc::new(Values::Value::BOOL { boolean: fullPath.clone() }), Arc::new(Values::Value::BOOL { boolean: debugLogging.clone() }), Arc::new(Values::Value::BOOL { boolean: generateInputConnectors.clone() }), Arc::new(Values::Value::BOOL { boolean: generateOutputConnectors.clone() }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((modelName.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn solveLinearSystem(mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, mut B: Arc<metamodelica::List<metamodelica::Real>>) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, i32)> {
    let mut res1: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut res2: i32 = 0;
    let mut res1_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("solveLinearSystem")).clone(), list![ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut A_iter in (A.clone()).into_iter().cloned() {
            let __x = ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut A_iter_iter in (A_iter.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::REAL { real: A_iter_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut B_iter in (B.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::REAL { real: B_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1_arr = __pa0.clone();
    res2 = __pa1.clone();
    res1 = ({
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(res1_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res1_arr_iter.clone()) {
        Deref @ Values::Value::REAL { .. } => var_field!((*res1_arr_iter).real, Values::Value::REAL).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getLoadedLibraries() -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getLoadedLibraries")).clone(), metamodelica::nil(), dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter_iter in (ValuesUtil::arrayValues(res_arr_iter.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn uriToFilename(mut uri: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("uriToFilename")).clone(), list![Arc::new(Values::Value::STRING { string: (uri.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn realpath(mut name: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("realpath")).clone(), list![Arc::new(Values::Value::STRING { string: (name.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn rewriteBlockCall(mut className: ArcStr, mut inDefs: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("rewriteBlockCall")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((inDefs.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn generateVerificationScenarios(mut path: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateVerificationScenarios")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((path.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn inferBindings(mut path: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("inferBindings")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((path.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn exportToFigaro(mut path: ArcStr, mut directory: ArcStr, mut database: ArcStr, mut mode: ArcStr, mut options: ArcStr, mut processor: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("exportToFigaro")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((path.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (directory.clone()).clone() }), Arc::new(Values::Value::STRING { string: (database.clone()).clone() }), Arc::new(Values::Value::STRING { string: (mode.clone()).clone() }), Arc::new(Values::Value::STRING { string: (options.clone()).clone() }), Arc::new(Values::Value::STRING { string: (processor.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn listFile(mut class_: ArcStr, mut nestedClasses: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("listFile")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((class_.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: nestedClasses.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn stringReplace(mut r#str: ArcStr, mut source: ArcStr, mut target: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("stringReplace")).clone(), list![Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }), Arc::new(Values::Value::STRING { string: (source.clone()).clone() }), Arc::new(Values::Value::STRING { string: (target.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn stringSplit(mut string: ArcStr, mut token: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("stringSplit")).clone(), list![Arc::new(Values::Value::STRING { string: (string.clone()).clone() }), Arc::new(Values::Value::STRING { string: (token.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn strtok(mut string: ArcStr, mut token: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("strtok")).clone(), list![Arc::new(Values::Value::STRING { string: (string.clone()).clone() }), Arc::new(Values::Value::STRING { string: (token.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn listVariables() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("listVariables")).clone(), metamodelica::nil(), dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getDerivedUnits(mut baseUnit: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getDerivedUnits")).clone(), list![Arc::new(Values::Value::STRING { string: (baseUnit.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn convertUnits(mut s1: ArcStr, mut s2: ArcStr) -> Result<(bool, metamodelica::Real, metamodelica::Real)> {
    let mut res1: bool = false;
    let mut res2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("convertUnits")).clone(), list![Arc::new(Values::Value::STRING { string: (s1.clone()).clone() }), Arc::new(Values::Value::STRING { string: (s2.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa2 }, tail: Deref @ metamodelica::List::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    res3 = __pa2.clone();
    Ok((res1, res2, res3))
}

pub fn dumpXMLDAE(mut className: ArcStr, mut translationLevel: ArcStr, mut addOriginalAdjacencyMatrix: bool, mut addSolvingInfo: bool, mut addMathMLCode: bool, mut dumpResiduals: bool, mut fileNamePrefix: ArcStr, mut rewriteRulesFile: ArcStr) -> Result<(bool, ArcStr)> {
    let mut res1: bool = false;
    let mut res2: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("dumpXMLDAE")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (translationLevel.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix.clone() }), Arc::new(Values::Value::BOOL { boolean: addSolvingInfo.clone() }), Arc::new(Values::Value::BOOL { boolean: addMathMLCode.clone() }), Arc::new(Values::Value::BOOL { boolean: dumpResiduals.clone() }), Arc::new(Values::Value::STRING { string: (fileNamePrefix.clone()).clone() }), Arc::new(Values::Value::STRING { string: (rewriteRulesFile.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    Ok((res1, res2))
}

pub fn translateGraphics(mut className: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("translateGraphics")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn save(mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("save")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn saveTotalModelDebug(mut filename: ArcStr, mut className: ArcStr, mut stripAnnotations: bool, mut stripComments: bool, mut obfuscate: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("saveTotalModelDebug")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: stripAnnotations.clone() }), Arc::new(Values::Value::BOOL { boolean: stripComments.clone() }), Arc::new(Values::Value::BOOL { boolean: obfuscate.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getTotalModel(mut className: ArcStr, mut stripAnnotations: bool, mut stripComments: bool, mut obfuscate: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getTotalModel")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: stripAnnotations.clone() }), Arc::new(Values::Value::BOOL { boolean: stripComments.clone() }), Arc::new(Values::Value::BOOL { boolean: obfuscate.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn saveTotalModel(mut fileName: ArcStr, mut className: ArcStr, mut stripAnnotations: bool, mut stripComments: bool, mut obfuscate: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("saveTotalModel")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: stripAnnotations.clone() }), Arc::new(Values::Value::BOOL { boolean: stripComments.clone() }), Arc::new(Values::Value::BOOL { boolean: obfuscate.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn saveModel(mut fileName: ArcStr, mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("saveModel")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn deleteFile(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("deleteFile")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn loadModel(mut className: ArcStr, mut priorityVersion: Arc<metamodelica::List<ArcStr>>, mut notify: bool, mut languageStandard: ArcStr, mut requireExactVersion: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadModel")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut priorityVersion_iter in (priorityVersion.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (priorityVersion_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::BOOL { boolean: notify.clone() }), Arc::new(Values::Value::STRING { string: (languageStandard.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: requireExactVersion.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn generateCode(mut className: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateCode")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn instantiateModel(mut className: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("instantiateModel")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn checkAllModelsRecursive(mut className: ArcStr, mut checkProtected: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("checkAllModelsRecursive")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: checkProtected.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn checkModel(mut className: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("checkModel")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn remove(mut path: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("remove")).clone(), list![Arc::new(Values::Value::STRING { string: (path.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn copy(mut source: ArcStr, mut destination: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("copy")).clone(), list![Arc::new(Values::Value::STRING { string: (source.clone()).clone() }), Arc::new(Values::Value::STRING { string: (destination.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn mkdir(mut newDirectory: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("mkdir")).clone(), list![Arc::new(Values::Value::STRING { string: (newDirectory.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn cd(mut newWorkingDirectory: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("cd")).clone(), list![Arc::new(Values::Value::STRING { string: (newWorkingDirectory.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAstAsCorbaString(mut fileName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAstAsCorbaString")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getLanguageStandard() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getLanguageStandard")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getOrderConnections() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getOrderConnections")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getShowAnnotations() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getShowAnnotations")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setShowAnnotations(mut show: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setShowAnnotations")).clone(), list![Arc::new(Values::Value::BOOL { boolean: show.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getDefaultOpenCLDevice() -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getDefaultOpenCLDevice")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getVectorizationLimit() -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getVectorizationLimit")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setNoSimplify(mut noSimplify: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setNoSimplify")).clone(), list![Arc::new(Values::Value::BOOL { boolean: noSimplify.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getNoSimplify() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getNoSimplify")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAnnotationVersion() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAnnotationVersion")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn echo(mut setEcho: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("echo")).clone(), list![Arc::new(Values::Value::BOOL { boolean: setEcho.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn runScript(mut fileName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("runScript")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn clearMessages() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("clearMessages")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn countMessages() -> Result<(i32, i32, i32)> {
    let mut res1: i32 = 0;
    let mut res2: i32 = 0;
    let mut res3: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("countMessages")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa2 }, tail: Deref @ metamodelica::List::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    res3 = __pa2.clone();
    Ok((res1, res2, res3))
}

pub fn getErrorString(mut warningsAsErrors: bool) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getErrorString")).clone(), list![Arc::new(Values::Value::BOOL { boolean: warningsAsErrors.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn alarm(mut seconds: i32) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("alarm")).clone(), list![Arc::new(Values::Value::INTEGER { integer: seconds.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn compareFiles(mut file1: ArcStr, mut file2: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("compareFiles")).clone(), list![Arc::new(Values::Value::STRING { string: (file1.clone()).clone() }), Arc::new(Values::Value::STRING { string: (file2.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn compareFilesAndMove(mut newFile: ArcStr, mut oldFile: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("compareFilesAndMove")).clone(), list![Arc::new(Values::Value::STRING { string: (newFile.clone()).clone() }), Arc::new(Values::Value::STRING { string: (oldFile.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn writeFile(mut fileName: ArcStr, mut data: ArcStr, mut append: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("writeFile")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::STRING { string: (data.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: append.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn readFile(mut fileName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("readFile")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn stat(mut fileName: ArcStr) -> Result<(bool, metamodelica::Real, metamodelica::Real)> {
    let mut res1: bool = false;
    let mut res2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut res3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("stat")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: __pa2 }, tail: Deref @ metamodelica::List::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1 = __pa0.clone();
    res2 = __pa1.clone();
    res3 = __pa2.clone();
    Ok((res1, res2, res3))
}

pub fn directoryExists(mut dirName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("directoryExists")).clone(), list![Arc::new(Values::Value::STRING { string: (dirName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn regularFileExists(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("regularFileExists")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getVersion(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getVersion")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((cl.clone()).clone())? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn clearCommandLineOptions() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("clearCommandLineOptions")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getConfigFlagValidOptions(mut flag: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<ArcStr>>)> {
    let mut res1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res2: ArcStr = arcstr::literal!("");
    let mut res3: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res1_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut res3_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getConfigFlagValidOptions")).clone(), list![Arc::new(Values::Value::STRING { string: (flag.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1_arr = __pa0.clone();
    res2 = __pa1.clone();
    res3_arr = __pa2.clone();
    res1 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(res1_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res1_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res1_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res3 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res3_arr_iter in (ValuesUtil::arrayValues(res3_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res3_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res3_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2, res3))
}

pub fn getCommandLineOptions() -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getCommandLineOptions")).clone(), metamodelica::nil(), dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn setCommandLineOptions(mut options: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setCommandLineOptions")).clone(), list![Arc::new(Values::Value::STRING { string: (options.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAvailableTearingMethods() -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut res1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res1_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut res2_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAvailableTearingMethods")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1_arr = __pa0.clone();
    res2_arr = __pa1.clone();
    res1 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(res1_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res1_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res1_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(res2_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res2_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res2_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getTearingMethod() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getTearingMethod")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAvailableIndexReductionMethods() -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut res1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res1_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut res2_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAvailableIndexReductionMethods")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1_arr = __pa0.clone();
    res2_arr = __pa1.clone();
    res1 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(res1_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res1_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res1_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(res2_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res2_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res2_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getIndexReductionMethod() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getIndexReductionMethod")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getAvailableMatchingAlgorithms() -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut res1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res1_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut res2_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getAvailableMatchingAlgorithms")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    res1_arr = __pa0.clone();
    res2_arr = __pa1.clone();
    res1 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(res1_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res1_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res1_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(res2_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res2_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res2_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getMatchingAlgorithm() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getMatchingAlgorithm")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn clearDebugFlags() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("clearDebugFlags")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn disableNewInstantiation() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("disableNewInstantiation")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn enableNewInstantiation() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("enableNewInstantiation")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setCompilerFlags(mut compilerFlags: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setCompilerFlags")).clone(), list![Arc::new(Values::Value::STRING { string: (compilerFlags.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getHomeDirectoryPath() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getHomeDirectoryPath")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getModelicaPath() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getModelicaPath")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setModelicaPath(mut modelicaPath: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setModelicaPath")).clone(), list![Arc::new(Values::Value::STRING { string: (modelicaPath.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getInstallationDirectoryPath() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getInstallationDirectoryPath")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setInstallationDirectoryPath(mut installationDirectoryPath: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setInstallationDirectoryPath")).clone(), list![Arc::new(Values::Value::STRING { string: (installationDirectoryPath.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setEnvironmentVar(mut var: ArcStr, mut value: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setEnvironmentVar")).clone(), list![Arc::new(Values::Value::STRING { string: (var.clone()).clone() }), Arc::new(Values::Value::STRING { string: (value.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getEnvironmentVar(mut var: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getEnvironmentVar")).clone(), list![Arc::new(Values::Value::STRING { string: (var.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getTempDirectoryPath() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getTempDirectoryPath")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setTempDirectoryPath(mut tempDirectoryPath: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setTempDirectoryPath")).clone(), list![Arc::new(Values::Value::STRING { string: (tempDirectoryPath.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setCXXCompiler(mut compiler: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setCXXCompiler")).clone(), list![Arc::new(Values::Value::STRING { string: (compiler.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getCXXCompiler() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getCXXCompiler")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setCFlags(mut inString: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setCFlags")).clone(), list![Arc::new(Values::Value::STRING { string: (inString.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getCFlags() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getCFlags")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setCompiler(mut compiler: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setCompiler")).clone(), list![Arc::new(Values::Value::STRING { string: (compiler.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getCompiler() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getCompiler")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setLinkerFlags(mut linkerFlags: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setLinkerFlags")).clone(), list![Arc::new(Values::Value::STRING { string: (linkerFlags.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getLinkerFlags() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getLinkerFlags")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn setLinker(mut linker: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("setLinker")).clone(), list![Arc::new(Values::Value::STRING { string: (linker.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn getLinker() -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("getLinker")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn generateSeparateCodeDependenciesMakefile(mut filename: ArcStr, mut directory: ArcStr, mut suffix: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateSeparateCodeDependenciesMakefile")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (directory.clone()).clone() }), Arc::new(Values::Value::STRING { string: (suffix.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn generateSeparateCodeDependencies(mut stampSuffix: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateSeparateCodeDependencies")).clone(), list![Arc::new(Values::Value::STRING { string: (stampSuffix.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::STRING { .. } => var_field!((*res_arr_iter).string, Values::Value::STRING).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn generateSeparateCode(mut className: ArcStr, mut cleanCache: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateSeparateCode")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::BOOL { boolean: cleanCache.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn generateJuliaHeader(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateJuliaHeader")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn generateHeader(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("generateHeader")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn clearVariables() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("clearVariables")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn clearProgram() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("clearProgram")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn clear() -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("clear")).clone(), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn help(mut topic: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("help")).clone(), list![Arc::new(Values::Value::STRING { string: (topic.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn saveAll(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("saveAll")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn system_parallel(mut callStr: Arc<metamodelica::List<ArcStr>>, mut numThreads: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut res: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("system_parallel")).clone(), list![ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut callStr_iter in (callStr.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (callStr_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::INTEGER { integer: numThreads.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(res_arr_iter.clone()) {
        Deref @ Values::Value::INTEGER { .. } => var_field!((*res_arr_iter).integer, Values::Value::INTEGER).clone(),
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn system(mut callStr: ArcStr, mut outputFile: ArcStr) -> Result<i32> {
    let mut res: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("system")).clone(), list![Arc::new(Values::Value::STRING { string: (callStr.clone()).clone() }), Arc::new(Values::Value::STRING { string: (outputFile.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn loadFileInteractive(mut filename: ArcStr, mut encoding: ArcStr, mut uses: bool, mut notify: bool, mut requireExactVersion: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadFileInteractive")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (encoding.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: uses.clone() }), Arc::new(Values::Value::BOOL { boolean: notify.clone() }), Arc::new(Values::Value::BOOL { boolean: requireExactVersion.clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn loadFileInteractiveQualified(mut filename: ArcStr, mut encoding: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadFileInteractiveQualified")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (encoding.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn parseFile(mut filename: ArcStr, mut encoding: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("parseFile")).clone(), list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (encoding.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn parseString(mut data: ArcStr, mut filename: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("parseString")).clone(), list![Arc::new(Values::Value::STRING { string: (data.clone()).clone() }), Arc::new(Values::Value::STRING { string: (filename.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn loadClassContentString(mut data: ArcStr, mut className: ArcStr, mut offsetX: i32, mut offsetY: i32) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadClassContentString")).clone(), list![Arc::new(Values::Value::STRING { string: (data.clone()).clone() }), Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((className.clone()).clone())? }) }), Arc::new(Values::Value::INTEGER { integer: offsetX.clone() }), Arc::new(Values::Value::INTEGER { integer: offsetY.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn loadString(mut data: ArcStr, mut filename: ArcStr, mut encoding: ArcStr, mut merge: bool, mut uses: bool, mut notify: bool, mut requireExactVersion: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadString")).clone(), list![Arc::new(Values::Value::STRING { string: (data.clone()).clone() }), Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::STRING { string: (encoding.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: merge.clone() }), Arc::new(Values::Value::BOOL { boolean: uses.clone() }), Arc::new(Values::Value::BOOL { boolean: notify.clone() }), Arc::new(Values::Value::BOOL { boolean: requireExactVersion.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn reloadClass(mut name: ArcStr, mut encoding: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("reloadClass")).clone(), list![Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((name.clone()).clone())? }) }), Arc::new(Values::Value::STRING { string: (encoding.clone()).clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn loadEncryptedPackage(mut fileName: ArcStr, mut workdir: ArcStr, mut skipUnzip: bool, mut uses: bool, mut notify: bool, mut requireExactVersion: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadEncryptedPackage")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::STRING { string: (workdir.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: skipUnzip.clone() }), Arc::new(Values::Value::BOOL { boolean: uses.clone() }), Arc::new(Values::Value::BOOL { boolean: notify.clone() }), Arc::new(Values::Value::BOOL { boolean: requireExactVersion.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn parseEncryptedPackage(mut fileName: ArcStr, mut workdir: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("parseEncryptedPackage")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::STRING { string: (workdir.clone()).clone() })], dummyMsg.clone())?;
    res = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(res_arr.clone())?).into_iter().cloned() {
            let __x = ValuesDump::valString(res_arr_iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn loadFiles(mut fileNames: Arc<metamodelica::List<ArcStr>>, mut encoding: ArcStr, mut numThreads: i32, mut uses: bool, mut notify: bool, mut requireExactVersion: bool, mut allowWithin: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadFiles")).clone(), list![ValuesMake::makeArray(({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut fileNames_iter in (fileNames.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (fileNames_iter.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Arc::new(Values::Value::STRING { string: (encoding.clone()).clone() }), Arc::new(Values::Value::INTEGER { integer: numThreads.clone() }), Arc::new(Values::Value::BOOL { boolean: uses.clone() }), Arc::new(Values::Value::BOOL { boolean: notify.clone() }), Arc::new(Values::Value::BOOL { boolean: requireExactVersion.clone() }), Arc::new(Values::Value::BOOL { boolean: allowWithin.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

pub fn loadFile(mut fileName: ArcStr, mut encoding: ArcStr, mut uses: bool, mut notify: bool, mut requireExactVersion: bool, mut allowWithin: bool) -> Result<bool> {
    let mut res: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), (literal!("loadFile")).clone(), list![Arc::new(Values::Value::STRING { string: (fileName.clone()).clone() }), Arc::new(Values::Value::STRING { string: (encoding.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: uses.clone() }), Arc::new(Values::Value::BOOL { boolean: notify.clone() }), Arc::new(Values::Value::BOOL { boolean: requireExactVersion.clone() }), Arc::new(Values::Value::BOOL { boolean: allowWithin.clone() })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    res = __pa0.clone();
    Ok(res)
}

