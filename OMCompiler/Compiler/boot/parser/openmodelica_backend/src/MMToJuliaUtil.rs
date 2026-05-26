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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Util;

//import DoubleEnded;
//import Global;
//import List;
// function getAllPartsExceptRecords
//   input Absyn.Class cls;
//   output list<Absyn.ClassPart> parts;
// algorithm
// end getAllPartsExceptRecords;
// function getPartsThatAreRecords
//   input Absyn.Class cls;
//   output list<Absyn.ClassPart> parts;
// algorithm
// end getPartsThatAreRecords;
// function splitRecordsAndOtherElements
// "This functions separates the records from the other elements of a given class."
//   input Absyn.Class cls;
//   output list<Absyn.ClassPart> bodyWithOnlyRecords = {};
//   output list<Absyn.ClassPart> otherElements = {};
// algorithm
//   bodyWithOnlyRecords := getPartsThatAreRecords(cls);
//   otherElements := getAllPartsExceptRecords(cls);
// end splitRecordsAndOtherElements;
// function restrictionIsRecord
//   input Absyn.Restriction restriction;
//   output Boolean isRecord;
// algorithm
//   isRecord := match restriction
//     case R_RECORD(__) then true;
//     else false;
//   end match;
// end restrictionIsRecord;
// public
// function refactorNonStandardUniontypes
//   input Absyn.Program inProgram;
//   output Absyn.Program outProgram;
// protected
//   constant Integer UNUSED;
//   Absyn.Program tmpProgram = inProgram;
//   Class tmpClass;
// algorithm
//   //Traverse all classes and create a package around each uniontype containing functions
//   outProgram := AbsynUtil.traverseClasses(program,
//                                           NONE(),
//                                           createPackageAroundUniontypeIfContainsFuncs,
//                                           UNUSED,
//                                           true);
//   //Traverse all classes and replace all uniontypes containing functions and other crap with with uniontypes containing only records.
//   //AbsynUtil.traverseClasses()
//   //Traverse all classes and replace all references to the old uniontype with <package>.<uniontype> instead
//   //AbsynUtil.traverseClasses()
// end refactorNonStandardUniontypes;
// function refactorUniontypesWithFunctions
//   input Absyn.Program inProgram;
//   output Absyn.Program outProgram;
// algorithm
//   //Replace all uniontype containing functions with uniontypes containing only records.
// end refactorUniontypesWithFunctions;
// function createPackageAroundUniontypeIfContainsFuncs
//   input tuple<Absyn.Class, Option<Absyn.Path>, Integer> inTuple;
//   output tuple<Absyn.Class, Option<Absyn.Path>, Integer> outTuple;
// protected
//   Absyn.ClassDef classDef = Util.tuple31(inTuple);
//   constant Boolean VISIT_PROTECTED = true;
//   constant String PACKAGE_NAME = "P" + AbsynUtil.getClassName(Util.tuple31(inTuple));
//   constant Integer UNUSED = 0;
//   list<Absyn.ClassPart> bodyWithOnlyRecords = {};
//   list<Absyn.ClassPart> otherElements = {};
// algorithm
//   if not AbsynUtil.isUniontype(cls) then
//     classDef := Util.tuple31(inTuple);
//   end if;
//   (bodyWithOnlyRecords, otherElements) := splitRecordsAndOtherElements(cls);
//   classDef := PARTS({} /*Assume no typevars for the package..*/,
//                     {}/*Class Attributes. Only for Optimica, not used*/,
//                     /*classParts*/ otherElements,
//                     {}/* Annotations, they are kept in the nested uniontype */,
//                     SOME("Generated top level package")/*Class comment*/);
// // From these parts we create a package and inside this package we store things accordingly
//  cls :=  CLASS(PACKAGE_NAME,
//                false,
//                false,
//                Absyn.R_PACKAGE(),
//                packageClsDef,
//                cls.info);
//    outTuple := (cls, NONE(), UNUSED);
// end createPackageAroundUniontypeIfContainsFuncs;
//TODO first figure out what we should rename
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Context {
    FUNCTION {
        /// Contains return values
        retValsStr: ArcStr,
    },
    FUNCTION_RETURN_CONTEXT {
        /// Contains return values
        retValsStr: ArcStr,
        /// String of the type we are currently operating on
        ty_str: ArcStr,
    },
    PACKAGE,
    UNIONTYPE {
        name: ArcStr,
    },
    NO_CONTEXT,
    INPUT_CONTEXT {
        ty_str: ArcStr,
    },
    MATCH_CONTEXT {
        inputExp: Arc<Absyn::Exp>,
    },
}
pub use self::Context::{FUNCTION,FUNCTION_RETURN_CONTEXT,PACKAGE,UNIONTYPE,NO_CONTEXT,INPUT_CONTEXT,MATCH_CONTEXT};

pub static packageContext: Context = crate::MMToJuliaUtil::Context::PACKAGE;

pub static noContext: Context = crate::MMToJuliaUtil::Context::NO_CONTEXT;

pub static functionContext: Context = Context::FUNCTION { retValsStr: literal!("") };

pub static returnContext: Context = Context::FUNCTION_RETURN_CONTEXT { retValsStr: literal!(""), ty_str: literal!("") };

pub static inputContext: Context = Context::INPUT_CONTEXT { ty_str: literal!("") };

pub fn makeUniontypeContext(mut name: ArcStr) -> Context {
    let mut context: Context = Context::NO_CONTEXT;
    context = Context::UNIONTYPE { name: (name.clone()).clone() };
    context
}

pub fn makeInputContext(mut ty_str: ArcStr) -> Context {
    let mut context: Context = Context::NO_CONTEXT;
    context = Context::INPUT_CONTEXT { ty_str: (ty_str.clone()).clone() };
    context
}

pub fn makeFunctionContext(mut returnValuesStr: ArcStr) -> Context {
    let mut context: Context = Context::NO_CONTEXT;
    context = Context::FUNCTION { retValsStr: (returnValuesStr.clone()).clone() };
    context
}

pub fn makeFunctionReturnContext(mut returnValuesStr: ArcStr, mut ty_str: ArcStr) -> Context {
    let mut context: Context = Context::NO_CONTEXT;
    context = Context::FUNCTION_RETURN_CONTEXT { retValsStr: (returnValuesStr.clone()).clone(), ty_str: (ty_str.clone()).clone() };
    context
}

pub fn makeMatchContext(mut iExp: Arc<Absyn::Exp>) -> Context {
    let mut context: Context = Context::NO_CONTEXT;
    context = Context::MATCH_CONTEXT { inputExp: iExp.clone() };
    context
}

pub fn makeInputDirection() -> Absyn::Direction {
    let mut direction: Absyn::Direction = Absyn::Direction::BIDIR;
    direction = openmodelica_ast::Absyn::Direction::INPUT;
    direction
}

pub fn makeOutputDirection() -> Absyn::Direction {
    let mut direction: Absyn::Direction = Absyn::Direction::BIDIR;
    direction = openmodelica_ast::Absyn::Direction::OUTPUT;
    direction
}

pub fn makeInputOutputDirection() -> Absyn::Direction {
    let mut direction: Absyn::Direction = Absyn::Direction::BIDIR;
    direction = openmodelica_ast::Absyn::Direction::INPUT_OUTPUT;
    direction
}

pub fn makeBDirection() -> Absyn::Direction {
    let mut direction: Absyn::Direction = Absyn::Direction::BIDIR;
    direction = openmodelica_ast::Absyn::Direction::BIDIR;
    direction
}

pub fn isFunctionContext(mut givenCTX: Context) -> bool {
    let mut isFuncCTX: bool = false;
    isFuncCTX = (match givenCTX.clone() {
        Context::FUNCTION { retValsStr: _ } => true,
        _ => false,
    });
    isFuncCTX
}

pub fn filterOnDirection(mut inputs: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut direction: Absyn::Direction) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut outputs: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut ioDirection: Absyn::Direction = makeInputOutputDirection();
    let mut directionEQ: bool = false;
    for mut i in &*inputs.clone() {
        let mut i = i.clone();
        directionEQ = AbsynUtil::directionEqual(direction.clone(), AbsynUtil::getDirection(i.clone())) || AbsynUtil::directionEqual(ioDirection.clone(), AbsynUtil::getDirection(i.clone()));
        if directionEQ.clone() {
            outputs = cons(i.clone(), outputs.clone());
        }
    }
    outputs
}

pub fn elementSpecIsBIDIR(mut spec: Arc<Absyn::ElementSpec>) -> bool {
    let mut isBidir: bool = false;
    isBidir = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::COMPONENTS { attributes, .. } => {
            (match attributes.direction.clone() {
        Absyn::Direction::BIDIR => true,
        _ => false,
    })
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBidir
}

pub fn elementSpecIsOUTPUT(mut spec: Arc<Absyn::ElementSpec>) -> bool {
    let mut isOutput: bool = false;
    isOutput = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::COMPONENTS { attributes, .. } => {
            (match attributes.direction.clone() {
        Absyn::Direction::OUTPUT => true,
        _ => false,
    })
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOutput
}

pub fn elementSpecIsOUTPUT_OR_BIDIR(mut spec: Arc<Absyn::ElementSpec>) -> bool {
    let mut isOutput: bool = false;
    isOutput = elementSpecIsOUTPUT(spec.clone()) || elementSpecIsBIDIR(spec.clone());
    isOutput
}

pub fn explicitReturnInClassPart(mut classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> bool {
    let mut existsImplicitReturn: bool = false;
    for mut cp in &*classParts.clone() {
        let mut cp = cp.clone();
        existsImplicitReturn = (::match_deref::match_deref! { match &(cp.clone()) {
        Deref @ Absyn::ClassPart::ALGORITHMS { contents } => {
            algorithmItemsContainsReturn(contents.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    existsImplicitReturn
}

pub fn algorithmItemsContainsReturn(mut contents: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> bool {
    let mut existsReturn: bool = false;
    for mut item in &*contents.clone() {
        let mut item = item.clone();
        existsReturn = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg, .. } => {
            (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::Algorithm::ALG_RETURN => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    existsReturn
}

pub fn mMKeywordToJLKeyword() -> () {
    ()
}

