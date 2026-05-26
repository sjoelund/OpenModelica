// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::NFComponentRef as ComponentRef;
use crate::NFInstNode::InstNode;
use crate::NFType as Type;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub type CrefToUnitTable = UnorderedMap::UnorderedMap<Unit, Arc<ComponentRef::NFComponentRef>>;

pub type StringToUnitTable = UnorderedMap::UnorderedMap<Unit, String>;

pub enum Token {
    T_NUMBER {
        number: i32,
    },
    T_UNIT {
        unit: String,
    },
    T_MUL,
    T_DIV,
    T_LPAREN,
    T_RPAREN,
}
pub use Token::*;

pub enum Unit {
    UNIT {
        s: i32,
        m: i32,
        g: i32,
        A: i32,
        K: i32,
        mol: i32,
        cd: i32,
        factor: f64,
    },
    MASTER {
        varList: metamodelica::List<Arc<ComponentRef::NFComponentRef>>,
    },
    UNKNOWN {
        unit: String,
    },
}
pub use Unit::*;

pub type UnitToStringTable = UnorderedMap::UnorderedMap<String, Unit>;

pub fn getKnownUnits() -> UnorderedMap::UnorderedMap<Unit, String> {
    todo!()
}

pub fn getKnownUnitsInverse() -> UnorderedMap::UnorderedMap<String, Unit> {
    todo!()
}

fn getPrefix(inS: String, inS2: String) -> (f64, String) {
    todo!()
}

pub fn hash(inKey: Unit) -> i32 {
    todo!()
}

pub fn isEqual(unit1: Unit, unit2: Unit) -> bool {
    todo!()
}

pub fn isMaster(unit: Unit) -> bool {
    todo!()
}

pub fn isUnit(inUnit: Unit) -> bool {
    todo!()
}

fn lexer(inCharList: metamodelica::List<String>) -> metamodelica::List<Token> {
    todo!()
}

pub fn newCrefUnitTable(size: i32) -> UnorderedMap::UnorderedMap<Unit, Arc<ComponentRef::NFComponentRef>> {
    todo!()
}

pub fn parseUnitString(inUnitString: String, inKnownUnits: UnorderedMap::UnorderedMap<Unit, String>, info: SourceInfo) -> Unit {
    todo!()
}

fn parser3(inMul: metamodelica::List<bool>, inTokenList: metamodelica::List<Token>, inUnit: Unit, inHtS2U: UnorderedMap::UnorderedMap<Unit, String>) -> Unit {
    todo!()
}

fn popNumber(inCharList: metamodelica::List<String>) -> (metamodelica::List<String>, String) {
    todo!()
}

fn popUnit(inCharList: metamodelica::List<String>) -> (metamodelica::List<String>, String) {
    todo!()
}

fn prefix2String(inReal: f64) -> String {
    todo!()
}

pub fn realAlmostEqRel(a: f64, b: f64, relTol: f64) -> bool {
    todo!()
}

pub fn unit2string(unit: Unit) -> String {
    todo!()
}

pub fn unitDiv(inUnit1: Unit, inUnit2: Unit) -> Unit {
    todo!()
}

pub fn unitMul(inUnit1: Unit, inUnit2: Unit) -> Unit {
    todo!()
}

pub fn unitMulReal(inUnit: Unit, inFactor: f64) -> Unit {
    todo!()
}

pub fn unitPow(inUnit: Unit, inExp: i32) -> Unit {
    todo!()
}

pub fn unitRoot(inUnit: Unit, inExponent: f64) -> Unit {
    todo!()
}

pub fn unitString(inUnit: Unit, inHtU2S: UnorderedMap::UnorderedMap<String, Unit>) -> String {
    todo!()
}

fn unitToken2unit(inS: String, inHtS2U: UnorderedMap::UnorderedMap<Unit, String>) -> Unit {
    todo!()
}

