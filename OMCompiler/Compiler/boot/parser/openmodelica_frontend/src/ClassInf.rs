// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::AbsynUtil;
use crate::SCode;
use crate::SCodeDump;
use crate::SCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    FOUND_EQUATION,
    FOUND_ALGORITHM,
    FOUND_CONSTRAINT,
    FOUND_EXT_DECL,
    NEWDEF,
    FOUND_COMPONENT {
        name: ArcStr,
    },
}
pub use self::Event::{FOUND_EQUATION,FOUND_ALGORITHM,FOUND_CONSTRAINT,FOUND_EXT_DECL,NEWDEF,FOUND_COMPONENT};

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    UNKNOWN {
        path: Arc<Absyn::Path>,
    },
    OPTIMIZATION {
        path: Arc<Absyn::Path>,
    },
    MODEL {
        path: Arc<Absyn::Path>,
    },
    RECORD {
        path: Arc<Absyn::Path>,
    },
    BLOCK {
        path: Arc<Absyn::Path>,
    },
    CONNECTOR {
        path: Arc<Absyn::Path>,
        isExpandable: bool,
    },
    TYPE {
        path: Arc<Absyn::Path>,
    },
    PACKAGE {
        path: Arc<Absyn::Path>,
    },
    FUNCTION {
        path: Arc<Absyn::Path>,
        isImpure: bool,
    },
    ENUMERATION {
        path: Arc<Absyn::Path>,
    },
    HAS_RESTRICTIONS {
        path: Arc<Absyn::Path>,
        hasEquations: bool,
        hasAlgorithms: bool,
        hasConstraints: bool,
    },
    TYPE_INTEGER {
        path: Arc<Absyn::Path>,
    },
    TYPE_REAL {
        path: Arc<Absyn::Path>,
    },
    TYPE_STRING {
        path: Arc<Absyn::Path>,
    },
    TYPE_BOOL {
        path: Arc<Absyn::Path>,
    },
    TYPE_CLOCK {
        path: Arc<Absyn::Path>,
    },
    TYPE_ENUM {
        path: Arc<Absyn::Path>,
    },
    EXTERNAL_OBJ {
        path: Arc<Absyn::Path>,
    },
    META_TUPLE {
        path: Arc<Absyn::Path>,
    },
    META_LIST {
        path: Arc<Absyn::Path>,
    },
    META_OPTION {
        path: Arc<Absyn::Path>,
    },
    META_RECORD {
        path: Arc<Absyn::Path>,
    },
    META_UNIONTYPE {
        path: Arc<Absyn::Path>,
        typeVars: Arc<metamodelica::List<ArcStr>>,
    },
    META_ARRAY {
        path: Arc<Absyn::Path>,
    },
    META_POLYMORPHIC {
        path: Arc<Absyn::Path>,
    },
}
pub use self::State::{UNKNOWN,OPTIMIZATION,MODEL,RECORD,BLOCK,CONNECTOR,TYPE,PACKAGE,FUNCTION,ENUMERATION,HAS_RESTRICTIONS,TYPE_INTEGER,TYPE_REAL,TYPE_STRING,TYPE_BOOL,TYPE_CLOCK,TYPE_ENUM,EXTERNAL_OBJ,META_TUPLE,META_LIST,META_OPTION,META_RECORD,META_UNIONTYPE,META_ARRAY,META_POLYMORPHIC};

pub fn assertTrans(inState: State, event: Event, info: SourceInfo) -> Result<State> {
    let mut outState: State;
    outState = 'mc: {
        let __mc_input = (inState.clone(), event.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut st, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            let mut str3: ArcStr;
            Ok(trans(st.clone(), event.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut st, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            let mut str3: ArcStr;
            str1 = (AbsynUtil::pathString(getStateName(st.clone()), (literal!(".")).clone(), true, false)?).clone();
            str2 = (printStateStr(st.clone())).clone();
            str3 = (printEventStr(event.clone())).clone();
            Error::addSourceMessage(Error::TRANS_VIOLATION.clone(), list![(str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone()], info.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outState)
}

pub fn assertValid(inState: State, inRestriction: SCode::Restriction, info: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inState.clone(), inRestriction.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut st, mut re, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            let mut str3: ArcStr;
            valid(st.clone(), re.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut st, mut re, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            let mut str3: ArcStr;
            str1 = (AbsynUtil::pathString(getStateName(st.clone()), (literal!(".")).clone(), true, false)?).clone();
            str2 = (printStateStr(st.clone())).clone();
            str3 = (SCodeDump::restrictionStringPP(re.clone())?).clone();
            Error::addSourceMessage(Error::RESTRICTION_VIOLATION.clone(), list![(str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone()], info.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub static basicTypeMods: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("quantity")).clone(), (literal!("unit")).clone(), (literal!("displayUnit")).clone(), (literal!("min")).clone(), (literal!("max")).clone(), (literal!("start")).clone(), (literal!("fixed")).clone(), (literal!("nominal")).clone(), (literal!("stateSelect")).clone(), (literal!("uncertain")).clone(), (literal!("distribution")).clone()] });

pub fn getStateName(inState: State) -> Arc<Absyn::Path> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (match inState.clone() {
        UNKNOWN { path: ref p } => p.clone(),
        OPTIMIZATION { path: ref p } => p.clone(),
        MODEL { path: ref p } => p.clone(),
        RECORD { path: ref p } => p.clone(),
        BLOCK { path: ref p } => p.clone(),
        CONNECTOR { path: ref p, .. } => p.clone(),
        TYPE { path: ref p } => p.clone(),
        PACKAGE { path: ref p } => p.clone(),
        FUNCTION { path: ref p, .. } => p.clone(),
        ENUMERATION { path: ref p } => p.clone(),
        HAS_RESTRICTIONS { path: ref p, .. } => p.clone(),
        TYPE_INTEGER { path: ref p } => p.clone(),
        TYPE_REAL { path: ref p } => p.clone(),
        TYPE_STRING { path: ref p } => p.clone(),
        TYPE_BOOL { path: ref p } => p.clone(),
        TYPE_CLOCK { path: ref p } => p.clone(),
        TYPE_ENUM { path: ref p } => p.clone(),
        EXTERNAL_OBJ { path: ref p } => p.clone(),
        META_TUPLE { path: ref p } => p.clone(),
        META_LIST { path: ref p } => p.clone(),
        META_OPTION { path: ref p } => p.clone(),
        META_RECORD { path: ref p } => p.clone(),
        META_UNIONTYPE { path: ref p, .. } => p.clone(),
        META_ARRAY { path: ref p } => p.clone(),
        META_POLYMORPHIC { path: ref p } => p.clone(),
        _ => Arc::new(Absyn::Path::IDENT { name: (literal!("#getStateName failed#")).clone() }),
    });
    outPath
}

pub fn isBasicTypeComponentName(name: ArcStr) -> bool {
    let mut res: bool;
    res = listMember((name.clone()).clone(), basicTypeMods.clone());
    res
}

pub fn isConnector(inState: State) -> Result<()> {
    let _ = (match inState.clone() {
        CONNECTOR { .. } => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn isFunction(inState: State) -> bool {
    let mut b: bool;
    b = (match inState.clone() {
        FUNCTION { .. } => true,
        _ => false,
    });
    b
}

pub fn isFunctionOrRecord(inState: State) -> bool {
    let mut b: bool;
    b = (match inState.clone() {
        FUNCTION { .. } => true,
        RECORD { .. } => true,
        _ => false,
    });
    b
}

pub fn isMetaRecord(inState: State) -> bool {
    let mut outIsRecord: bool;
    outIsRecord = (match inState.clone() {
        META_RECORD { .. } => true,
        _ => false,
    });
    outIsRecord
}

pub fn isRecord(inState: State) -> bool {
    let mut outIsRecord: bool;
    outIsRecord = (match inState.clone() {
        RECORD { .. } => true,
        _ => false,
    });
    outIsRecord
}

pub fn isTypeOrRecord(inState: State) -> bool {
    let mut outIsTypeOrRecord: bool;
    outIsTypeOrRecord = (match inState.clone() {
        TYPE { .. } => true,
        RECORD { .. } => true,
        _ => false,
    });
    outIsTypeOrRecord
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn matchingState(inState: State, inStateLst: Arc<metamodelica::List<State>>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &((inState.clone(), inStateLst.clone())) {
        (_, Deref @ metamodelica::List::Nil) => false,
        (UNKNOWN { .. }, Deref @ metamodelica::List::Cons { head: UNKNOWN { .. }, tail: _ }) => true,
        (MODEL { .. }, Deref @ metamodelica::List::Cons { head: MODEL { .. }, tail: _ }) => true,
        (RECORD { .. }, Deref @ metamodelica::List::Cons { head: RECORD { .. }, tail: _ }) => true,
        (BLOCK { .. }, Deref @ metamodelica::List::Cons { head: BLOCK { .. }, tail: _ }) => true,
        (CONNECTOR { .. }, Deref @ metamodelica::List::Cons { head: CONNECTOR { .. }, tail: _ }) => true,
        (TYPE { .. }, Deref @ metamodelica::List::Cons { head: TYPE { .. }, tail: _ }) => true,
        (PACKAGE { .. }, Deref @ metamodelica::List::Cons { head: PACKAGE { .. }, tail: _ }) => true,
        (FUNCTION { .. }, Deref @ metamodelica::List::Cons { head: FUNCTION { .. }, tail: _ }) => true,
        (ENUMERATION { .. }, Deref @ metamodelica::List::Cons { head: ENUMERATION { .. }, tail: _ }) => true,
        (TYPE_INTEGER { .. }, Deref @ metamodelica::List::Cons { head: TYPE_INTEGER { .. }, tail: _ }) => true,
        (TYPE_REAL { .. }, Deref @ metamodelica::List::Cons { head: TYPE_REAL { .. }, tail: _ }) => true,
        (TYPE_STRING { .. }, Deref @ metamodelica::List::Cons { head: TYPE_STRING { .. }, tail: _ }) => true,
        (TYPE_BOOL { .. }, Deref @ metamodelica::List::Cons { head: TYPE_BOOL { .. }, tail: _ }) => true,
        (TYPE_CLOCK { .. }, Deref @ metamodelica::List::Cons { head: TYPE_CLOCK { .. }, tail: _ }) => true,
        (TYPE_ENUM { .. }, Deref @ metamodelica::List::Cons { head: TYPE_ENUM { .. }, tail: _ }) => true,
        (_, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut res: bool;
            res = matchingState(inState.clone(), rest.clone())?;
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBoolean)
}

fn printEventStr(inEvent: Event) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = ((match inEvent.clone() {
        FOUND_EQUATION => literal!("equation"),
        FOUND_CONSTRAINT => literal!("constraint"),
        NEWDEF => literal!("new definition"),
        FOUND_COMPONENT { name: mut name } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) },
        FOUND_EXT_DECL => literal!("external function declaration"),
        _ => literal!("Unknown event"),
    })).clone();
    r#str
}

pub fn printState(inState: State) -> Result<()> {
    let _ = (match inState.clone() {
        UNKNOWN { path: ref p } => {
            Print::printBuf((literal!("UNKNOWN ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        OPTIMIZATION { path: ref p } => {
            Print::printBuf((literal!("OPTIMIZATION ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        MODEL { path: ref p } => {
            Print::printBuf((literal!("MODEL ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        RECORD { path: ref p } => {
            Print::printBuf((literal!("RECORD ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        BLOCK { path: ref p } => {
            Print::printBuf((literal!("BLOCK ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        CONNECTOR { path: ref p, .. } => {
            Print::printBuf((literal!("CONNECTOR ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        TYPE { path: ref p } => {
            Print::printBuf((literal!("TYPE ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        PACKAGE { path: ref p } => {
            Print::printBuf((literal!("PACKAGE ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        FUNCTION { isImpure: true, path: ref p } => {
            Print::printBuf((literal!("IMPURE FUNCTION ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        FUNCTION { path: ref p, .. } => {
            Print::printBuf((literal!("FUNCTION ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        TYPE_INTEGER { path: ref p } => {
            Print::printBuf((literal!("TYPE_INTEGER ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        TYPE_REAL { path: ref p } => {
            Print::printBuf((literal!("TYPE_REAL ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        TYPE_STRING { path: ref p } => {
            Print::printBuf((literal!("TYPE_STRING ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        TYPE_BOOL { path: ref p } => {
            Print::printBuf((literal!("TYPE_BOOL ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        TYPE_CLOCK { path: ref p } => {
            Print::printBuf((literal!("TYPE_CLOCK ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            ()
        },
        HAS_RESTRICTIONS { path: ref p, .. } => {
            Print::printBuf((literal!("HAS_RESTRICTIONS ")).clone())?;
            Print::printBuf((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone())?;
            Print::printBuf((printStateStr(inState.clone())).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn printStateStr(inState: State) -> ArcStr {
    let mut outString: ArcStr;
    outString = ((match inState.clone() {
        UNKNOWN { .. } => literal!("unknown"),
        OPTIMIZATION { .. } => literal!("optimization"),
        MODEL { .. } => literal!("model"),
        RECORD { .. } => literal!("record"),
        BLOCK { .. } => literal!("block"),
        CONNECTOR { .. } => literal!("connector"),
        TYPE { .. } => literal!("type"),
        PACKAGE { .. } => literal!("package"),
        FUNCTION { isImpure: true, .. } => literal!("impure function"),
        FUNCTION { .. } => literal!("function"),
        TYPE_INTEGER { .. } => literal!("Integer"),
        TYPE_REAL { .. } => literal!("Real"),
        TYPE_STRING { .. } => literal!("String"),
        TYPE_BOOL { .. } => literal!("Boolean"),
        TYPE_CLOCK { .. } => literal!("Clock"),
        HAS_RESTRICTIONS { hasConstraints: false, hasAlgorithms: false, hasEquations: false, .. } => literal!("new def"),
        HAS_RESTRICTIONS { hasAlgorithms: mut b2, hasEquations: mut b1, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("has")); __mm_s.push_str(&*if (b1.clone()) {literal!(" equations")} else {literal!("")}); __mm_s.push_str(&*if (b2.clone()) {literal!(" algorithms")} else {literal!("")}); __mm_s.push_str(&*if (b1.clone()) {literal!(" constraints")} else {literal!("")}); ArcStr::from(__mm_s) },
        EXTERNAL_OBJ { .. } => literal!("ExternalObject"),
        META_TUPLE { .. } => literal!("tuple"),
        META_LIST { .. } => literal!("list"),
        META_OPTION { .. } => literal!("Option"),
        META_RECORD { .. } => literal!("meta_record"),
        META_POLYMORPHIC { .. } => literal!("polymorphic"),
        META_ARRAY { .. } => literal!("meta_array"),
        META_UNIONTYPE { .. } => literal!("uniontype"),
        _ => literal!("#printStateStr failed#"),
    })).clone();
    outString
}

pub fn start(inRestriction: SCode::Restriction, inPath: Arc<Absyn::Path>) -> Result<State> {
    let mut outState: State;
    outState = start_dispatch(inRestriction.clone(), AbsynUtil::makeFullyQualified(inPath.clone()))?;
    Ok(outState)
}

fn start_dispatch(inRestriction: SCode::Restriction, inPath: Arc<Absyn::Path>) -> Result<State> {
    let mut outState: State;
    outState = (::match_deref::match_deref! { match &((inRestriction.clone(), inPath.clone())) {
        (SCode::R_CLASS, p) => State::UNKNOWN { path: p.clone() },
        (SCode::R_OPTIMIZATION, p) => State::OPTIMIZATION { path: p.clone() },
        (SCode::R_MODEL, p) => State::MODEL { path: p.clone() },
        (SCode::R_RECORD { isOperator: _ }, p) => State::RECORD { path: p.clone() },
        (SCode::R_BLOCK, p) => State::BLOCK { path: p.clone() },
        (SCode::R_CONNECTOR { isExpandable }, p) => State::CONNECTOR { path: p.clone(), isExpandable: isExpandable.clone() },
        (SCode::R_TYPE, p) => State::TYPE { path: p.clone() },
        (SCode::R_PACKAGE, p) => State::PACKAGE { path: p.clone() },
        (SCode::R_FUNCTION { .. }, p) => State::FUNCTION { path: p.clone(), isImpure: SCodeUtil::isRestrictionImpure(inRestriction.clone(), true) },
        (SCode::R_OPERATOR, p) => State::FUNCTION { path: p.clone(), isImpure: false },
        (SCode::R_ENUMERATION, p) => State::ENUMERATION { path: p.clone() },
        (SCode::R_PREDEFINED_INTEGER, p) => State::TYPE_INTEGER { path: p.clone() },
        (SCode::R_PREDEFINED_REAL, p) => State::TYPE_REAL { path: p.clone() },
        (SCode::R_PREDEFINED_STRING, p) => State::TYPE_STRING { path: p.clone() },
        (SCode::R_PREDEFINED_BOOLEAN, p) => State::TYPE_BOOL { path: p.clone() },
        (SCode::R_PREDEFINED_CLOCK, p) => {
            let mut isExpandable: bool;
            let mut isImpure: bool;
            let true = (Config::synchronousFeaturesAllowed()) else { bail!("pattern mismatch") };
            State::TYPE_CLOCK { path: p.clone() }
        },
        (SCode::R_PREDEFINED_ENUMERATION, p) => State::TYPE_ENUM { path: p.clone() },
        (SCode::R_UNIONTYPE { .. }, p) => State::META_UNIONTYPE { path: p.clone(), typeVars: var_field!(inRestriction.typeVars, SCode::Restriction::R_UNIONTYPE).clone() },
        (SCode::R_METARECORD { .. }, p) => State::META_RECORD { path: p.clone() },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outState)
}

pub fn trans(inState: State, inEvent: Event) -> Result<State> {
    let mut outState: State;
    outState = (match (inState.clone(), inEvent.clone()) {
        (UNKNOWN { path: ref p }, NEWDEF) => State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: false, hasAlgorithms: false, hasConstraints: false },
        (OPTIMIZATION { .. }, NEWDEF) => inState.clone(),
        (MODEL { .. }, NEWDEF) => inState.clone(),
        (RECORD { .. }, NEWDEF) => inState.clone(),
        (BLOCK { .. }, NEWDEF) => inState.clone(),
        (CONNECTOR { .. }, NEWDEF) => inState.clone(),
        (TYPE { path: ref p }, NEWDEF) => State::TYPE { path: p.clone() },
        (PACKAGE { path: ref p }, NEWDEF) => State::PACKAGE { path: p.clone() },
        (FUNCTION { .. }, NEWDEF) => inState.clone(),
        (ENUMERATION { .. }, NEWDEF) => inState.clone(),
        (TYPE_INTEGER { .. }, NEWDEF) => inState.clone(),
        (TYPE_REAL { .. }, NEWDEF) => inState.clone(),
        (TYPE_STRING { .. }, NEWDEF) => inState.clone(),
        (TYPE_BOOL { .. }, NEWDEF) => inState.clone(),
        (TYPE_CLOCK { .. }, NEWDEF) => inState.clone(),
        (TYPE_ENUM { .. }, NEWDEF) => inState.clone(),
        (META_UNIONTYPE { .. }, NEWDEF) => inState.clone(),
        (META_RECORD { .. }, NEWDEF) => inState.clone(),
        (UNKNOWN { path: ref p }, FOUND_COMPONENT { .. }) => State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: false, hasAlgorithms: false, hasConstraints: false },
        (OPTIMIZATION { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (MODEL { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (RECORD { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (BLOCK { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (CONNECTOR { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (TYPE { path: ref p }, FOUND_COMPONENT { name: mut s }) => {
            let mut st: State;
            let mut ev: Event;
            let mut isExpandable: bool;
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut isImpure: bool;
            let mut msg: Arc<metamodelica::List<ArcStr>>;
            if !(isBasicTypeComponentName((s.clone()).clone())) {
                Error::addMessage(Error::TYPE_NOT_FROM_PREDEFINED.clone(), list![(AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone()])?;
                bail!("fail");
            }
            State::TYPE { path: p.clone() }
        },
        (PACKAGE { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (FUNCTION { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (ENUMERATION { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (HAS_RESTRICTIONS { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (TYPE_INTEGER { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (TYPE_REAL { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (TYPE_STRING { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (TYPE_BOOL { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (TYPE_CLOCK { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (TYPE_ENUM { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (META_RECORD { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (META_UNIONTYPE { .. }, FOUND_COMPONENT { .. }) => inState.clone(),
        (UNKNOWN { path: ref p }, FOUND_EQUATION) => State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: true, hasAlgorithms: false, hasConstraints: false },
        (OPTIMIZATION { .. }, FOUND_EQUATION) => inState.clone(),
        (OPTIMIZATION { .. }, FOUND_CONSTRAINT) => inState.clone(),
        (OPTIMIZATION { .. }, FOUND_ALGORITHM) => inState.clone(),
        (MODEL { .. }, FOUND_EQUATION) => inState.clone(),
        (BLOCK { .. }, FOUND_EQUATION) => inState.clone(),
        (MODEL { .. }, FOUND_ALGORITHM) => inState.clone(),
        (BLOCK { .. }, FOUND_ALGORITHM) => inState.clone(),
        (FUNCTION { .. }, FOUND_ALGORITHM) => inState.clone(),
        (HAS_RESTRICTIONS { hasConstraints: mut b3, hasAlgorithms: mut b2, path: ref p, .. }, FOUND_EQUATION) => State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: true, hasAlgorithms: b2.clone(), hasConstraints: b3.clone() },
        (HAS_RESTRICTIONS { hasAlgorithms: mut b2, hasEquations: mut b1, path: ref p, .. }, FOUND_CONSTRAINT) => State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: b1.clone(), hasAlgorithms: b2.clone(), hasConstraints: true },
        (HAS_RESTRICTIONS { hasConstraints: mut b3, hasEquations: mut b1, path: ref p, .. }, FOUND_ALGORITHM) => State::HAS_RESTRICTIONS { path: p.clone(), hasEquations: b1.clone(), hasAlgorithms: true, hasConstraints: b3.clone() },
        (FUNCTION { .. }, FOUND_EXT_DECL) => inState.clone(),
        (_, FOUND_EXT_DECL) => bail!("fail"),
        (_, FOUND_EQUATION) => bail!("fail"),
        (_, FOUND_CONSTRAINT) => bail!("fail"),
        (mut st, mut ev) => {
            let mut p: Arc<Absyn::Path>;
            let mut isExpandable: bool;
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut isImpure: bool;
            let mut s: ArcStr;
            let mut msg: Arc<metamodelica::List<ArcStr>>;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ClassInf.trans failed: ")); __mm_s.push_str(&*printStateStr(st.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*printEventStr(ev.clone())); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
    });
    Ok(outState)
}

pub fn valid(inState: State, inRestriction: SCode::Restriction) -> Result<()> {
    let _ = (match (inState.clone(), inRestriction.clone()) {
        (UNKNOWN { .. }, _) => (),
        (HAS_RESTRICTIONS { .. }, SCode::R_CLASS) => (),
        (HAS_RESTRICTIONS { .. }, SCode::R_MODEL) => (),
        (HAS_RESTRICTIONS { .. }, SCode::R_OPTIMIZATION) => (),
        (MODEL { .. }, SCode::R_MODEL) => (),
        (RECORD { .. }, SCode::R_RECORD { isOperator: _ }) => (),
        (RECORD { .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (HAS_RESTRICTIONS { hasAlgorithms: false, hasConstraints: false, hasEquations: false, .. }, SCode::R_RECORD { isOperator: _ }) => (),
        (BLOCK { .. }, SCode::R_BLOCK) => (),
        (MODEL { .. }, SCode::R_MODEL) => (),
        (CONNECTOR { .. }, SCode::R_TYPE) => (),
        (CONNECTOR { isExpandable: false, .. }, SCode::R_CONNECTOR { isExpandable: false }) => (),
        (CONNECTOR { isExpandable: true, .. }, SCode::R_CONNECTOR { isExpandable: true }) => (),
        (HAS_RESTRICTIONS { hasAlgorithms: false, hasConstraints: false, hasEquations: false, .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (TYPE_INTEGER { .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (TYPE_REAL { .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (TYPE_STRING { .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (TYPE_BOOL { .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (TYPE_CLOCK { .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (TYPE_ENUM { .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (ENUMERATION { .. }, SCode::R_CONNECTOR { isExpandable: _ }) => (),
        (TYPE { .. }, SCode::R_CONNECTOR { .. }) => (),
        (TYPE { .. }, SCode::R_TYPE) => (),
        (TYPE_INTEGER { .. }, SCode::R_TYPE) => (),
        (TYPE_REAL { .. }, SCode::R_TYPE) => (),
        (TYPE_STRING { .. }, SCode::R_TYPE) => (),
        (TYPE_BOOL { .. }, SCode::R_TYPE) => (),
        (TYPE_CLOCK { .. }, SCode::R_TYPE) => (),
        (TYPE_ENUM { .. }, SCode::R_TYPE) => (),
        (ENUMERATION { .. }, SCode::R_TYPE) => (),
        (PACKAGE { .. }, SCode::R_PACKAGE) => (),
        (HAS_RESTRICTIONS { hasAlgorithms: false, hasConstraints: false, hasEquations: false, .. }, SCode::R_PACKAGE) => (),
        (FUNCTION { .. }, SCode::R_FUNCTION { functionRestriction: _ }) => (),
        (HAS_RESTRICTIONS { hasConstraints: false, hasEquations: false, .. }, SCode::R_FUNCTION { functionRestriction: _ }) => (),
        (META_TUPLE { .. }, SCode::R_TYPE) => (),
        (META_LIST { .. }, SCode::R_TYPE) => (),
        (META_OPTION { .. }, SCode::R_TYPE) => (),
        (META_RECORD { .. }, SCode::R_TYPE) => (),
        (META_ARRAY { .. }, SCode::R_TYPE) => (),
        (META_UNIONTYPE { .. }, SCode::R_TYPE) => (),
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

