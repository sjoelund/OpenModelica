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
// =============================================================================
// For any request about the implementation of this package,
// please contact Filippo Donida (donida@elet.polimi.it).
// =============================================================================
// =============================================================================
// Important discrete states are not recognised as states.
// The varKind shoud be varVariability and another method
// and also the relative structure for the variable should
// be implemented to output the information like: state,
// dummy der, dummy state,...
// =============================================================================
// =============================================================================
// With a delaration like:
// parameter Real a = 1;
// record is everytime empty.  Why?
// =============================================================================
// =============================================================================
// In order to compile the XMLDump module (XMLDump.mo package)
// XMLDump.mo text in the Compiler/Makefile.common file (SRCMO
// variable) has been added.
// =============================================================================
// =============================================================================
// Probably it's better to put a link to the corresponging
// algorithm/variable/when/zeroCross/...
// One solution could be to add an attribute like: Algorithm_Number
// to the algorith tab, like:
// <ALGORITHM LABEL=algorithm_Number>
// and then when dumping the algorithm reference in this function put
// the corresponding tag:
// <ANCHOR id=algorithm_Number/>
// within the equation element.
// =============================================================================
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
use crate::BackendEquation;
use crate::BackendVariable;
use crate::ZeroCrossings;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// for stringReplace
pub const HEADER: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>";

pub const DAE_OPEN: &'static str = "dae xmlns:p1=\"http://www.w3.org/1998/Math/MathML\"\n                                                xmlns:xlink=\"http://www.w3.org/1999/xlink\"\n                                                xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n                                                xsi:noNamespaceSchemaLocation=\"http://home.dei.polimi.it/donida/Projects/AutoEdit/Images/DAE.xsd\"";

pub const DAE_CLOSE: &'static str = "dae";

pub const LABEL: &'static str = "label";

pub const ANCHOR: &'static str = "anchor";

pub const ALGORITHM_NAME: &'static str = "algorithmName";

/*
  This String is used in:
    1 - dunmAbsynPathList - function to print a list of paths:
        <ELEMENT>
          Content
        </ELEMENT>
        <ELEMENT>
          ...
    2 - dumpCrefIdxLst to print a list of BackendDAE.CrefIndex:
        <ELEMENT ID=...>CrefIndex</ELEMENT>
        ...
    3 - dumpStrLst to print a list of String
        <ELEMENT>FirstStringOfList</ELEMENT>
        ...
        <ELEMENT>LastStringOfList</ELEMENT>
  */
pub const ELEMENT: &'static str = "element";

pub const ELEMENT_: &'static str = "Element";

pub const INDEX: &'static str = "index";

pub const INTERVAL: &'static str = "interval";

pub const START: &'static str = "start";

pub const VALUE: &'static str = "value";

pub const LIST_: &'static str = "List";

//Is the Dimension attribute of a list element.
pub const DIMENSION: &'static str = "dimension";

//Is the reference attribute for an element.
pub const ID: &'static str = "id";

pub const ID_: &'static str = "Id";

pub const CONDITION: &'static str = "Condition";

pub const REINIT: &'static str = "reinit";

pub const ASSERT: &'static str = "assert";

pub const TERMINATE: &'static str = "terminate";

//This is the String attribute for the textual representation of the expressions.
pub const EXP_STRING: &'static str = "string";

//This constant is used when is necessary to bind equations, variables, whenequations,..
pub const INVOLVED: &'static str = "involved";

pub const ADDITIONAL_INFO: &'static str = "additionalInfo";

pub const SOLVING_INFO: &'static str = "solvingInfo";

//This is the name that identifies the Variables' block. It's also used to compose the other
//Variables' names, such as KnownVariables, OrderedVariables, and so on.
pub const VARIABLES: &'static str = "variables";

pub const VARIABLES_: &'static str = "Variables";

pub const ORDERED: &'static str = "ordered";

pub const KNOWN: &'static str = "known";

pub const EXTERNAL: &'static str = "external";

pub const ALIAS: &'static str = "alias";

pub const CLASSES: &'static str = "classes";

pub const CLASSES_: &'static str = "Classes";

pub const CLASS: &'static str = "class";

pub const CLASS_: &'static str = "Class";

pub const NAMES_: &'static str = "Names";

//This is used all the time a variable is referenced.
pub const VARIABLE: &'static str = "variable";

pub const VAR_ID: &'static str = ID;

pub const VAR_NAME: &'static str = "name";

pub const VAR_INDEX: &'static str = "differentiatedIndex";

pub const VAR_DERNAME: &'static str = "derivativeName";

pub const VAR_ORIGNAME: &'static str = "origName";

pub const STATE_SELECT_NEVER: &'static str = "Never";

pub const STATE_SELECT_AVOID: &'static str = "Avoid";

pub const STATE_SELECT_DEFAULT: &'static str = "Default";

pub const STATE_SELECT_PREFER: &'static str = "Prefer";

pub const STATE_SELECT_ALWAYS: &'static str = "Always";

pub const VAR_FLOW: &'static str = "flow";

pub const VAR_FLOW_FLOW: &'static str = "Flow";

pub const VAR_FLOW_NONFLOW: &'static str = "NonFlow";

pub const VAR_FLOW_NONCONNECTOR: &'static str = "NonConnector";

pub const VAR_STREAM: &'static str = "stream";

pub const VAR_STREAM_STREAM: &'static str = "Stream";

pub const VAR_STREAM_NONSTREAM: &'static str = "NonStream";

pub const VAR_STREAM_NONSTREAM_CONNECTOR: &'static str = "NonStreamConnector";

// /  TO CORRECT WITHIN THE OMC!!!  ///
// The variability is related to the
// possible values a variable can assume
// In this case also information for the
// variable are stored. For example it would be useful
// to print the information about state, dummyState, dummyDer separately.
//In addition to this there's a problem with the discrete states,
//since they aren't recognised as states.
pub const VAR_VARIABILITY: &'static str = "variability";

pub const VARIABILITY_CONTINUOUS: &'static str = "continuous";

pub const VARIABILITY_CONTINUOUS_STATE: &'static str = "continuousState";

pub const VARIABILITY_CONTINUOUS_DUMMYDER: &'static str = "continuousDummyDer";

pub const VARIABILITY_CONTINUOUS_DUMMYSTATE: &'static str = "continuousDummyState";

pub const VARIABILITY_DISCRETE: &'static str = "discrete";

pub const VARIABILITY_PARAMETER: &'static str = "parameter";

pub const VARIABILITY_CONSTANT: &'static str = "constant";

pub const VARIABILITY_EXTERNALOBJECT: &'static str = "externalObject";

pub const VAR_TYPE: &'static str = "type";

pub const VARTYPE_INTEGER: &'static str = "Integer";

pub const VARTYPE_REAL: &'static str = "Real";

pub const VARTYPE_STRING: &'static str = "String";

pub const VARTYPE_BOOLEAN: &'static str = "Boolean";

pub const VARTYPE_ENUM: &'static str = "Enum";

pub const VARTYPE_ENUMERATION: &'static str = "enumeration";

pub const VARTYPE_EXTERNALOBJECT: &'static str = "ExternalObject";

pub const VAR_DIRECTION: &'static str = "direction";

pub const VARDIR_INPUT: &'static str = "input";

pub const VARDIR_OUTPUT: &'static str = "output";

pub const VARDIR_NONE: &'static str = "none";

pub const VAR_FIXED: &'static str = "fixed";

pub const VAR_COMMENT: &'static str = "comment";

pub const VAR_ATTRIBUTES_VALUES: &'static str = "attributesValues";

pub const VAR_ATTR_QUANTITY: &'static str = "quantity";

pub const VAR_ATTR_UNIT: &'static str = "unit";

pub const VAR_ATTR_DISPLAY_UNIT: &'static str = "displayUnit";

pub const VAR_ATTR_STATESELECT: &'static str = "stateSelect";

pub const VAR_ATTR_MINVALUE: &'static str = "minValue";

pub const VAR_ATTR_MAXVALUE: &'static str = "maxValue";

pub const VAR_ATTR_NOMINAL: &'static str = "nominal";

pub const VAR_ATTR_INITIALVALUE: &'static str = "initialValue";

pub const VAR_ATTR_FIXED: &'static str = "fixed";

//Name of the element containing the binding information
//for the variables (bindExpression)
//For example consider:
//parameter Real a = 3*2+e; //With Real constant e = 3;
//BindExpression 3*2+e
pub const BIND_EXPRESSION: &'static str = "bindExpression";

//Name of the element representing the subscript, for example the array's index.
pub const SUBSCRIPT: &'static str = "subscript";

//Additional info for variables.
pub const HASH_TB_CREFS_LIST: &'static str = "hashTb";

pub const HASH_TB_STRING_LIST_OLDVARS: &'static str = "hashTbOldVars";

//All this constants below are used in the dumpBackendDAE method.
pub const EQUATIONS: &'static str = "equations";

pub const EQUATIONS_: &'static str = "Equations";

pub const SIMPLE: &'static str = "simple";

pub const INITIAL: &'static str = "initial";

pub const ZERO_CROSSING: &'static str = "zeroCrossing";

pub const SAMPLES: &'static str = "Samples";

pub const ARRAY_OF_EQUATIONS: &'static str = "arrayOfEquations";

//This is used also in the dumpEquation method.
pub const COMPLEX_EQUATION: &'static str = "complexequations";

pub const EQUATION: &'static str = "equation";

pub const EQUATION_: &'static str = "Equation";

pub const SOLVED: &'static str = "solved";

pub const SOLVED_: &'static str = "Solved";

pub const WHEN: &'static str = "when";

pub const WHEN_: &'static str = "When";

pub const WHEN_OPERATORS: &'static str = "WhenOperators";

pub const WHEN_OPERATOR: &'static str = "WhenOperator";

pub const RESIDUAL: &'static str = "residual";

pub const RESIDUAL_: &'static str = "Residual";

/*
  This String constant is used in:
    1 - dumpAlgorithms to print out the list of Algorithms:
        <ALGORITHM LABEL=Algorithm_ID>
          ...
        </ALGORITHM>
    2 - dumpEquation if the equation element is an algorithm:
        <ALGORITHM ID=...>
          <AlgorithmID>...</AlgorithmID>
          <ANCHOR ALGORITHM_NAME=Algorithm_No></ANCHOR>
        </ALGORITHM>
  */
pub const ALGORITHM: &'static str = "algorithm";

/*
  This String constant is used to print the reference to the
  corresponding algorithm.
  */
pub const ALGORITHM_REF: &'static str = "algorithm_ref";

pub const CONSTRAINT: &'static str = "constraint";

pub const CONSTRAINT_REF: &'static str = "constraint_ref";

/*
  This String constant represents the single equation of an array of
  equations and it is used in:
    1 - dumpArrayEqns to print the list of equations
    2 - dumpEquation to print the list of equations corresponding to
        the array
  */
pub const ARRAY_EQUATION: &'static str = "arrayEquation";

pub const ALGORITHMS: &'static str = "algorithms";

pub const CONSTRAINTS: &'static str = "constraints";

pub const FUNCTIONS: &'static str = "functions";

pub const FUNCTION: &'static str = "function";

pub const FUNCTION_NAME: &'static str = "name";

pub const FUNCTION_ORIGNAME: &'static str = VAR_ORIGNAME;

pub const NAME_BINDINGS: &'static str = "nameBindings";

pub const C_NAME: &'static str = "cName";

pub const C_IMPLEMENTATIONS: &'static str = "cImplementations";

pub const MODELICA_IMPLEMENTATION: &'static str = "ModelicaImplementation";

/*This strings here below are used for printing additionalInfo
  concerning the DAE system of equations, such as:
   - the original adjacency matrix (before performing matching and BLT
   - the matching algorithm output
   - the blocks obtained after running the BLT algorithm (Tarjan)
   */
pub const MATCHING_ALGORITHM: &'static str = "matchingAlgorithm";

pub const SOLVED_IN: &'static str = "solvedIn";

pub const BLT_REPRESENTATION: &'static str = "bltRepresentation";

pub const BLT_BLOCK: &'static str = "bltBlock";

pub const ORIGINAL_ADJACENCY_MATRIX: &'static str = "originalAdjacencyMatrix";

pub const MATH: &'static str = "math";

pub const MathML: &'static str = "MathML";

pub const MathMLApply: &'static str = "apply";

pub const MathMLWeb: &'static str = "http://www.w3.org/1998/Math/MathML";

pub const MathMLXmlns: &'static str = "xmlns";

pub const MathMLType: &'static str = "type";

pub const MathMLNumber: &'static str = "cn";

pub const MathMLVariable: &'static str = "ci";

pub const MathMLConstant: &'static str = "constant";

pub const MathMLInteger: &'static str = "integer";

pub const MathMLReal: &'static str = "real";

pub const MathMLVector: &'static str = "vector";

pub const MathMLMatrixrow: &'static str = "matrixrow";

pub const MathMLMatrix: &'static str = "matrix";

pub const MathMLTrue: &'static str = "true";

pub const MathMLFalse: &'static str = "false";

pub const MathMLAnd: &'static str = "and";

pub const MathMLOr: &'static str = "or";

pub const MathMLNot: &'static str = "not";

pub const MathMLEqual: &'static str = "eq";

pub const MathMLLessThan: &'static str = "lt";

pub const MathMLLessEqualThan: &'static str = "leq";

pub const MathMLGreaterThan: &'static str = "gt";

pub const MathMLGreaterEqualThan: &'static str = "geq";

pub const MathMLEquivalent: &'static str = "equivalent";

pub const MathMLNotEqual: &'static str = "neq";

pub const MathMLPlus: &'static str = "plus";

pub const MathMLMinus: &'static str = "minus";

pub const MathMLTimes: &'static str = "times";

pub const MathMLDivide: &'static str = "divide";

pub const MathMLPower: &'static str = "power";

pub const MathMLTranspose: &'static str = "transpose";

pub const MathMLScalarproduct: &'static str = "scalarproduct";

pub const MathMLVectorproduct: &'static str = "vectorproduct";

pub const MathMLInterval: &'static str = "interval";

pub const MathMLSelector: &'static str = "selector";

pub const MathMLIfClause: &'static str = "piecewise";

pub const MathMLIfBranch: &'static str = "piece";

pub const MathMLElseBranch: &'static str = "otherwise";

pub const MathMLOperator: &'static str = "mo";

pub const MathMLArccos: &'static str = "arccos";

pub const MathMLArcsin: &'static str = "arcsin";

pub const MathMLArctan: &'static str = "arctan";

pub const MathMLLn: &'static str = "ln";

pub const MathMLLog: &'static str = "log";

fn binopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        mut op => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (binopSymbol2(op.clone())?).clone();
            s.clone()
        },
    })).clone();
    Ok(outString)
}

fn binopSymbol2(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::ADD { .. } => {
            arcstr::literal!(MathMLPlus)
        },
        DAE::Operator::SUB { .. } => {
            arcstr::literal!(MathMLMinus)
        },
        DAE::Operator::MUL { .. } => {
            arcstr::literal!(MathMLTimes)
        },
        DAE::Operator::DIV { .. } => {
            arcstr::literal!(MathMLDivide)
        },
        DAE::Operator::POW { .. } => {
            arcstr::literal!(MathMLPower)
        },
        DAE::Operator::ADD_ARR { .. } => {
            arcstr::literal!(MathMLPlus)
        },
        DAE::Operator::SUB_ARR { .. } => {
            arcstr::literal!(MathMLMinus)
        },
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => {
            arcstr::literal!(MathMLTimes)
        },
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => {
            arcstr::literal!(MathMLScalarproduct)
        },
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => {
            arcstr::literal!(MathMLVectorproduct)
        },
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => {
            arcstr::literal!(MathMLDivide)
        },
        _ => {
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.binopSymbol2 - Unknown operator: ")).clone();
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*ExpressionDump::debugBinopSymbol(inOperator.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn dumpAbsynPathLst(mut absynPathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut Content: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((absynPathLst.clone(), Content.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (_, _) => {
            dumpStrOpenTag((Content.clone()).clone())?;
            dumpAbsynPathLst2(absynPathLst.clone())?;
            dumpStrCloseTag((Content.clone()).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpAbsynPathLst2(mut absynPathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(absynPathLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: ap, tail: apLst } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = AbsynUtil::pathStringNoQual(ap.clone(), (literal!(".")).clone(), true, false)?;
            dumpStrTagContent((arcstr::literal!(ELEMENT)).clone(), (r#str.clone()).clone())?;
            dumpAbsynPathLst2(apLst.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpConstraints(mut constrs: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(constrs.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        _ => {
            let mut len: i32 = 0;
            len = (constrs.clone().len() as i32);
            dumpStrOpenTagAttr((arcstr::literal!(CONSTRAINTS)).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(len.clone())).clone())?;
            dumpConstraints2(constrs.clone(), 0)?;
            dumpStrCloseTag((arcstr::literal!(CONSTRAINTS)).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpConstraints2(mut iConstrs: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut inConsNo: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((iConstrs.clone(), inConsNo.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps }, tail: constrs }, conNo) => {
            let mut conNo_1: i32 = 0;
            dumpStrOpenTagAttr((arcstr::literal!(CONSTRAINT)).clone(), (arcstr::literal!(LABEL)).clone(), (stringAppend((stringAppend((arcstr::literal!(CONSTRAINT_REF)).clone(), (literal!("_")).clone())).clone(), (intString(conNo.clone())).clone())).clone())?;
            Print::printBuf((Util::xmlEscape((DAEDump::dumpConstraintsStr(list![Arc::new(DAE::Element::CONSTRAINT { constraints: Arc::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps.clone() }), source: DAE::emptyElementSource.clone() })])?).clone())?).clone())?;
            dumpStrCloseTag((arcstr::literal!(CONSTRAINT)).clone())?;
            conNo_1 = conNo.clone() + 1;
            dumpConstraints2(constrs.clone(), conNo_1.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpBltInvolvedEquations(mut inComp: Arc<BackendDAE::StrongComponent>, mut offset: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inComp.clone(), offset.clone())) {
        (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, .. }, _) => {
            dumpStrTagAttrNoChild((stringAppend((arcstr::literal!(INVOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (stringAppend((arcstr::literal!(EQUATION)).clone(), (arcstr::literal!(ID_)).clone())).clone(), (intString(e.clone() + offset.clone())).clone())?;
            ()
        },
        (_, _) => {
            let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (elst, _) = BackendDAETransform::getEquationAndSolvedVarIndxes(inComp.clone())?;
            dumpBltInvolvedEquations1(elst.clone(), offset.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpBltInvolvedEquations1(mut inList: Arc<metamodelica::List<i32>>, mut offset: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inList.clone(), offset.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: el, tail: remList }, _) => {
            dumpStrTagAttrNoChild((stringAppend((arcstr::literal!(INVOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (stringAppend((arcstr::literal!(EQUATION)).clone(), (arcstr::literal!(ID_)).clone())).clone(), (intString(el.clone() + offset.clone())).clone())?;
            dumpBltInvolvedEquations1(remList.clone(), offset.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpBindExpression(mut inOptExpExp: Option<Arc<DAE::Exp>>, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inOptExpExp.clone()) {
        None => (),
        Some(_) => {
            dumpOptExp(inOptExpExp.clone(), (arcstr::literal!(BIND_EXPRESSION)).clone(), addMathMLCode.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpComment(mut inComment: ArcStr) -> Result<()> {
    Print::printBuf((literal!("<!--")).clone())?;
    Print::printBuf((Util::xmlEscape((inComment.clone()).clone())?).clone())?;
    Print::printBuf((literal!("-->")).clone())?;
    Ok(())
}

fn dumpComponents(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    dumpStrOpenTag((arcstr::literal!(BLT_REPRESENTATION)).clone())?;
    let _ = BackendDAEUtil::foldEqSystem(dae.clone(), Arc::new(dumpComponentsWork), (0, 0))?;
    dumpStrCloseTag((arcstr::literal!(BLT_REPRESENTATION)).clone())?;
    Ok(())
}

fn dumpComponentsWork(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut inOffset: (i32, i32)) -> Result<(i32, i32)> {
    let mut outOffset: (i32, i32);
    let mut v1: metamodelica::Array<i32>;
    let mut v2: metamodelica::Array<i32>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut voffset: i32 = 0;
    let mut eoffset: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: __pa2 }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    v1 = __pa0.clone();
    v2 = __pa1.clone();
    comps = __pa2.clone();
    (voffset, eoffset) = inOffset.clone();
    dumpStrOpenTag((arcstr::literal!(BLT_REPRESENTATION)).clone())?;
    dumpComponents1(comps.clone(), voffset.clone(), eoffset.clone())?;
    dumpStrCloseTag((arcstr::literal!(BLT_REPRESENTATION)).clone())?;
    outOffset = (voffset.clone() + (v2.clone().borrow().len() as i32), eoffset.clone() + (v1.clone().borrow().len() as i32));
    Ok(outOffset)
}

fn dumpComponents1(mut l: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut voffset: i32, mut eoffset: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (l.clone(), voffset.clone(), eoffset.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    dumpComponents2(l.clone(), 1 + voffset.clone(), eoffset.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpComponents2(mut inIntegerLstLst: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut i: i32, mut offset: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inIntegerLstLst.clone(), i.clone(), offset.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: l, tail: lst }, _, _) => {
            dumpStrOpenTagAttr((arcstr::literal!(BLT_BLOCK)).clone(), (arcstr::literal!(ID)).clone(), (intString(i.clone())).clone())?;
            dumpBltInvolvedEquations(l.clone(), offset.clone())?;
            dumpStrCloseTag((arcstr::literal!(BLT_BLOCK)).clone())?;
            dumpComponents2(lst.clone(), i.clone() + 1, offset.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpCrefIdxLstArr(mut crefIdxLstArr: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>, mut Content: ArcStr, mut inInteger: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (crefIdxLstArr.clone(), Content.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            if !((crefIdxLstArr.borrow()[(inInteger.clone()-1) as usize].clone().is_empty())) { bail!("guard") }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            dumpCrefIdxLst(crefIdxLstArr.borrow()[(inInteger.clone()-1) as usize].clone(), (Content.clone()).clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.dumpCrefIdxLstArr - failed for var number:")).clone();
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*intString(inInteger.clone())); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpCrefIdxLst(mut crefIdxLst: Arc<metamodelica::List<BackendDAE::CrefIndex>>, mut Content: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((crefIdxLst.clone(), Content.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        _ => {
            dumpStrOpenTag((Content.clone()).clone())?;
            dumpCrefIdxLst2(crefIdxLst.clone())?;
            dumpStrCloseTag((Content.clone()).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpCrefIdxLst2(mut crefIdxLst: Arc<metamodelica::List<BackendDAE::CrefIndex>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(crefIdxLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::CrefIndex { index: index_c, cref: cref_c }, tail: crefIndexList } => {
            let mut cref: ArcStr = arcstr::literal!("");
            cref = (ComponentReference::crefStr(cref_c.clone())?).clone();
            dumpStrOpenTagAttr((arcstr::literal!(ELEMENT)).clone(), (arcstr::literal!(ID)).clone(), (intString(index_c.clone())).clone())?;
            Print::printBuf((cref.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(ELEMENT)).clone())?;
            dumpCrefIdxLst2(crefIndexList.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpDAEInstDims(mut arry_Dim: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut Content: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (arry_Dim.clone(), Content.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    dumpStrOpenTag((Content.clone()).clone())?;
                    dumpDAEInstDims2(arry_Dim.clone())?;
                    dumpStrCloseTag((Content.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpDAEInstDims2(mut arry_Dim: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(arry_Dim.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: dim, tail: lDim } => {
            dumpStrOpenTag((arcstr::literal!(DIMENSION)).clone())?;
            dumpDimension(dim.clone())?;
            dumpStrCloseTag((arcstr::literal!(DIMENSION)).clone())?;
            dumpDAEInstDims2(lDim.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn dumpDAEXML(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut fileNamePrefix: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { shared: BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: __pa0, .. }, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    fileNamePrefix = __pa0.clone();
    Print::clearBuf();
    dumpBackendDAE(inDAE.clone(), false, false, false, false, false)?;
    Print::writeBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileNamePrefix.clone()); __mm_s.push_str(&*literal!(".xml")); ArcStr::from(__mm_s) }).clone())?;
    Print::clearBuf();
    Ok(outDAE)
}

pub fn dumpBackendDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut addOriginalAdjacencyMatrix: bool, mut addSolvingInfo: bool, mut addMathMLCode: bool, mut dumpResiduals: bool, mut dumpSolvedEquations: bool) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inBackendDAE.clone(), addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), dumpSolvedEquations.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE { UNIQUEIO: systs, derivativeNamePrefix: BackendDAE::Shared { globalKnownVars: vars_knownVars @ BackendDAE::Variables { crefIndices: crefIdxLstArr_knownVars, .. }, localKnownVars: _, externalObjects: vars_externalObject @ BackendDAE::Variables { crefIndices: crefIdxLstArr_externalObject, .. }, aliasVars: vars_aliasVars @ BackendDAE::Variables { crefIndices: crefIdxLstArr_aliasVars, .. }, initialEqs: ieqns, removedEqs: _, constraints: constrs, classAttrs: _, cache: _, graph: _, functionTree: funcs, eventInfo, extObjClasses: extObjCls, backendDAEType: _, symjacs: _, info: _, .. }, .. }, addOrInMatrix, addSolInfo, addMML, dumpRes, false) => {
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut knvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut extvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut aliasvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut eqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut reqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut ieqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut reqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut functionsElems: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    knvars = BackendVariable::varList(vars_knownVars.clone())?;
                    extvars = BackendVariable::varList(vars_externalObject.clone())?;
                    aliasvars = BackendVariable::varList(vars_aliasVars.clone())?;
                    reqns = BackendDAEUtil::collapseRemovedEqs(inBackendDAE.clone())?;
                    Print::printBuf((arcstr::literal!(HEADER)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(DAE_OPEN)).clone())?;
                    dumpStrOpenTagAttr((arcstr::literal!(VARIABLES)).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(List::fold(List::map(systs.clone(), Arc::new(fnptr!(BackendDAEUtil::systemSize, Arc<BackendDAE::EqSystem>))), Arc::new(fnptr!(intAdd, i32, i32)), 0) + (knvars.clone().len() as i32) + (extvars.clone().len() as i32) + (aliasvars.clone().len() as i32))).clone())?;
                    vars = List::fold(systs.clone(), Arc::new(getOrderedVars), metamodelica::nil());
                    dumpVars(vars.clone(), arrayCreate(1, metamodelica::nil()), (stringAppend((arcstr::literal!(ORDERED)).clone(), (arcstr::literal!(VARIABLES_)).clone())).clone(), addMML.clone())?;
                    dumpVars(knvars.clone(), crefIdxLstArr_knownVars.clone(), (stringAppend((arcstr::literal!(KNOWN)).clone(), (arcstr::literal!(VARIABLES_)).clone())).clone(), addMML.clone())?;
                    dumpVars(extvars.clone(), crefIdxLstArr_externalObject.clone(), (stringAppend((arcstr::literal!(EXTERNAL)).clone(), (arcstr::literal!(VARIABLES_)).clone())).clone(), addMML.clone())?;
                    dumpVars(aliasvars.clone(), crefIdxLstArr_aliasVars.clone(), (stringAppend((arcstr::literal!(ALIAS)).clone(), (arcstr::literal!(VARIABLES_)).clone())).clone(), addMML.clone())?;
                    dumpExtObjCls(extObjCls.clone(), (stringAppend((arcstr::literal!(EXTERNAL)).clone(), (arcstr::literal!(CLASSES_)).clone())).clone())?;
                    dumpStrCloseTag((arcstr::literal!(VARIABLES)).clone())?;
                    eqnsl = List::fold(systs.clone(), Arc::new(fnptr!(getEqsList, Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)), metamodelica::nil());
                    dumpEqns(eqnsl.clone(), (arcstr::literal!(EQUATIONS)).clone(), addMML.clone(), dumpRes.clone(), false)?;
                    reqnsl = BackendEquation::equationList(reqns.clone());
                    dumpEqns(reqnsl.clone(), (stringAppend((arcstr::literal!(SIMPLE)).clone(), (arcstr::literal!(EQUATIONS_)).clone())).clone(), addMML.clone(), dumpRes.clone(), false)?;
                    ieqnsl = BackendEquation::equationList(ieqns.clone());
                    dumpEqns(ieqnsl.clone(), (stringAppend((arcstr::literal!(INITIAL)).clone(), (arcstr::literal!(EQUATIONS_)).clone())).clone(), addMML.clone(), dumpRes.clone(), false)?;
                    dumpEventInfo(eventInfo.clone(), addMML.clone())?;
                    dumpConstraints(constrs.clone())?;
                    functionsElems = DAEUtil::getFunctionList(funcs.clone(), false)?;
                    dumpFunctions(functionsElems.clone())?;
                    dumpSolvingInfo(addOrInMatrix.clone(), addSolInfo.clone(), inBackendDAE.clone())?;
                    dumpStrCloseTag((arcstr::literal!(DAE_CLOSE)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE { UNIQUEIO: systs, derivativeNamePrefix: BackendDAE::Shared { globalKnownVars: vars_knownVars @ BackendDAE::Variables { crefIndices: crefIdxLstArr_knownVars, .. }, localKnownVars: _, externalObjects: vars_externalObject @ BackendDAE::Variables { crefIndices: crefIdxLstArr_externalObject, .. }, aliasVars: vars_aliasVars @ BackendDAE::Variables { crefIndices: crefIdxLstArr_aliasVars, .. }, initialEqs: ieqns, removedEqs: _, constraints: constrs, classAttrs: _, cache: _, graph: _, functionTree: funcs, eventInfo, extObjClasses: extObjCls, backendDAEType: _, symjacs: _, info: _, partitionsInfo: _, .. }, .. }, addOrInMatrix, addSolInfo, addMML, dumpRes, true) => {
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut knvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut extvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut aliasvars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut reqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut ieqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut reqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut functionsElems: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    let mut eqnsVarsinOrderLst: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
                    knvars = BackendVariable::varList(vars_knownVars.clone())?;
                    extvars = BackendVariable::varList(vars_externalObject.clone())?;
                    aliasvars = BackendVariable::varList(vars_aliasVars.clone())?;
                    reqns = BackendDAEUtil::collapseRemovedEqs(inBackendDAE.clone())?;
                    Print::printBuf((arcstr::literal!(HEADER)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(DAE_OPEN)).clone())?;
                    dumpStrOpenTagAttr((arcstr::literal!(VARIABLES)).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(List::fold(List::map(systs.clone(), Arc::new(fnptr!(BackendDAEUtil::systemSize, Arc<BackendDAE::EqSystem>))), Arc::new(fnptr!(intAdd, i32, i32)), 0) + (knvars.clone().len() as i32) + (extvars.clone().len() as i32) + (aliasvars.clone().len() as i32))).clone())?;
                    vars = List::fold(systs.clone(), Arc::new(getOrderedVars), metamodelica::nil());
                    dumpVars(vars.clone(), arrayCreate(1, metamodelica::nil()), (stringAppend((arcstr::literal!(ORDERED)).clone(), (arcstr::literal!(VARIABLES_)).clone())).clone(), addMML.clone())?;
                    dumpVars(knvars.clone(), crefIdxLstArr_knownVars.clone(), (stringAppend((arcstr::literal!(KNOWN)).clone(), (arcstr::literal!(VARIABLES_)).clone())).clone(), addMML.clone())?;
                    dumpVars(extvars.clone(), crefIdxLstArr_externalObject.clone(), (stringAppend((arcstr::literal!(EXTERNAL)).clone(), (arcstr::literal!(VARIABLES_)).clone())).clone(), addMML.clone())?;
                    dumpVars(aliasvars.clone(), crefIdxLstArr_aliasVars.clone(), (stringAppend((arcstr::literal!(ALIAS)).clone(), (arcstr::literal!(VARIABLES_)).clone())).clone(), addMML.clone())?;
                    dumpExtObjCls(extObjCls.clone(), (stringAppend((arcstr::literal!(EXTERNAL)).clone(), (arcstr::literal!(CLASSES_)).clone())).clone())?;
                    dumpStrCloseTag((arcstr::literal!(VARIABLES)).clone())?;
                    eqnsVarsinOrderLst = List::fold(systs.clone(), Arc::new(getOrderedEqsandVars), metamodelica::nil());
                    dumpStrOpenTagAttr((arcstr::literal!(EQUATIONS)).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString((eqnsVarsinOrderLst.clone().len() as i32))).clone())?;
                    dumpSolvedEqns(eqnsVarsinOrderLst.clone(), 1, (arcstr::literal!(EQUATIONS)).clone(), addMML.clone(), dumpRes.clone(), true)?;
                    dumpStrCloseTag((arcstr::literal!(EQUATIONS)).clone())?;
                    reqnsl = BackendEquation::equationList(reqns.clone());
                    dumpEqns(reqnsl.clone(), (stringAppend((arcstr::literal!(SIMPLE)).clone(), (arcstr::literal!(EQUATIONS_)).clone())).clone(), addMML.clone(), dumpRes.clone(), false)?;
                    ieqnsl = BackendEquation::equationList(ieqns.clone());
                    dumpEqns(ieqnsl.clone(), (stringAppend((arcstr::literal!(INITIAL)).clone(), (arcstr::literal!(EQUATIONS_)).clone())).clone(), addMML.clone(), dumpRes.clone(), false)?;
                    dumpEventInfo(eventInfo.clone(), addMML.clone())?;
                    dumpConstraints(constrs.clone())?;
                    functionsElems = DAEUtil::getFunctionList(funcs.clone(), false)?;
                    dumpFunctions(functionsElems.clone())?;
                    dumpSolvingInfo(addOrInMatrix.clone(), addSolInfo.clone(), inBackendDAE.clone())?;
                    dumpStrCloseTag((arcstr::literal!(DAE_CLOSE)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("XMLDump.dumpBackendDAE failed")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpEventInfo(mut inEventInfo: BackendDAE::EventInfo, mut addMML: bool) -> Result<()> {
    let _ = (match (inEventInfo.clone(), addMML.clone()) {
        (BackendDAE::EventInfo { zeroCrossings: mut zc, timeEvents: mut timeEvents, .. }, _) => {
            dumpTimeEvents(timeEvents.clone(), (stringAppend((arcstr::literal!(SAMPLES)).clone(), (arcstr::literal!(LIST_)).clone())).clone(), addMML.clone())?;
            dumpZeroCrossing(ZeroCrossings::toList(zc.clone()), (stringAppend((arcstr::literal!(ZERO_CROSSING)).clone(), (arcstr::literal!(LIST_)).clone())).clone(), addMML.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn getOrderedVars(mut syst: Arc<BackendDAE::EqSystem>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    vars = BackendVariable::varList(BackendVariable::daeVars(syst.clone()))?;
    outVars = listAppend(inVars.clone(), vars.clone());
    Ok(outVars)
}

fn getEqsList(mut syst: Arc<BackendDAE::EqSystem>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    eqnsl = BackendEquation::equationList(BackendEquation::getEqnsFromEqSystem(syst.clone()));
    outEqns = listAppend(inEqns.clone(), eqnsl.clone());
    outEqns
}

fn getOrderedEqsandVars(mut syst: Arc<BackendDAE::EqSystem>, mut inEqnsVars: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>>) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>>> {
    let mut outEqnsVars: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
    let mut eqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, orderedVars: __pa1, orderedEqs: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    vars = __pa1.clone();
    eqns = __pa2.clone();
    outEqnsVars = getOrderedEqs2(comps.clone(), eqns.clone(), vars.clone(), inEqnsVars.clone())?;
    Ok(outEqnsVars)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getOrderedEqs2(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut inAccum: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>>) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>>> {
    let mut outEqnsVarsLst: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
    outEqnsVarsLst = (::match_deref::match_deref! { match &((inComps.clone(), eqns.clone(), vars.clone(), inAccum.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            inAccum.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: v, eqn: e }, tail: rest }, _, _, _) => {
            let mut var: BackendDAE::Var;
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut result: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
            var = BackendVariable::getVarAt(vars.clone(), v.clone())?;
            eqn = BackendEquation::get(eqns.clone(), e.clone());
            result = listAppend(inAccum.clone(), list![(list![eqn.clone()], list![var.clone()])]);
            result = getOrderedEqs2(rest.clone(), eqns.clone(), vars.clone(), result.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: vlst, eqns: elst, .. }, tail: rest }, _, _, _) => {
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut result: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
            varlst = List::map1r(vlst.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
            eqnlst = BackendEquation::getList(elst.clone(), eqns.clone());
            result = listAppend(inAccum.clone(), list![(eqnlst.clone(), varlst.clone())]);
            result = getOrderedEqs2(rest.clone(), eqns.clone(), vars.clone(), result.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: vlst, eqn: e }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut result: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
            varlst = List::map1r(vlst.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
            eqn = BackendEquation::get(eqns.clone(), e.clone());
            result = listAppend(inAccum.clone(), list![(list![eqn.clone()], varlst.clone())]);
            result = getOrderedEqs2(rest.clone(), eqns.clone(), vars.clone(), result.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: vlst, eqn: e }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut result: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
            varlst = List::map1r(vlst.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
            eqn = BackendEquation::get(eqns.clone(), e.clone());
            result = listAppend(inAccum.clone(), list![(list![eqn.clone()], varlst.clone())]);
            result = getOrderedEqs2(rest.clone(), eqns.clone(), vars.clone(), result.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: vlst, eqn: e }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut result: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
            varlst = List::map1r(vlst.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
            eqn = BackendEquation::get(eqns.clone(), e.clone());
            result = listAppend(inAccum.clone(), list![(list![eqn.clone()], varlst.clone())]);
            result = getOrderedEqs2(rest.clone(), eqns.clone(), vars.clone(), result.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: vlst, eqn: e }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut result: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
            varlst = List::map1r(vlst.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
            eqn = BackendEquation::get(eqns.clone(), e.clone());
            result = listAppend(inAccum.clone(), list![(list![eqn.clone()], varlst.clone())]);
            result = getOrderedEqs2(rest.clone(), eqns.clone(), vars.clone(), result.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: vlst, eqn: e }, tail: rest }, _, _, _) => {
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut result: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
            varlst = List::map1r(vlst.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
            eqn = BackendEquation::get(eqns.clone(), e.clone());
            result = listAppend(inAccum.clone(), list![(list![eqn.clone()], varlst.clone())]);
            result = getOrderedEqs2(rest.clone(), eqns.clone(), vars.clone(), result.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { innerEquations, residualequations: elst, tearingvars: vlst, .. }, .. }, tail: rest }, _, _, _) => {
            let mut vlst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut elst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vlst1Lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut varlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut varlst1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eqnlst1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut result: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>> = metamodelica::nil();
            (elst1, vlst1Lst, _) = List::map_3(innerEquations.clone(), Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation));
            vlst1 = List::flatten(vlst1Lst.clone());
            varlst1 = List::map1r(vlst1.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
            varlst = List::map1r(vlst.clone(), Arc::new(BackendVariable::getVarAt), vars.clone());
            varlst = listAppend(varlst1.clone(), varlst.clone());
            eqnlst1 = BackendEquation::getList(elst1.clone(), eqns.clone());
            eqnlst = BackendEquation::getList(elst.clone(), eqns.clone());
            eqnlst = listAppend(eqnlst1.clone(), eqnlst.clone());
            result = listAppend(inAccum.clone(), list![(eqnlst.clone(), varlst.clone())]);
            result = getOrderedEqs2(rest.clone(), eqns.clone(), vars.clone(), result.clone())?;
            result.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _, _, _) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("XMLDump.getOrderedEqs2 failed!")).clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqnsVarsLst)
}

fn dumpDAEVariableAttributes(mut dae_var_attr: Option<Arc<DAE::VariableAttributes>>, mut Content: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (dae_var_attr.clone(), Content.clone(), addMathMLCode.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: _, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: None, min: None, max: None, start: None, fixed: None, uncertainOption: _, distributionOption: _, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: None, start: None, fixed: None, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: None, start: None, fixed: _, equationBound: _, isProtected: _, finalPrefix: _, .. }), _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: None, min: None, max: None, start: None, fixed: None, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quant, unit, displayUnit, min, max, start: Initial, fixed, nominal, stateSelectOption: stateSel, uncertainOption: _, distributionOption: _, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _, addMMLCode) => {
                    dumpStrOpenTag((Content.clone()).clone())?;
                    dumpOptExp(quant.clone(), (arcstr::literal!(VAR_ATTR_QUANTITY)).clone(), addMMLCode.clone())?;
                    dumpOptExp(unit.clone(), (arcstr::literal!(VAR_ATTR_UNIT)).clone(), addMMLCode.clone())?;
                    dumpOptExp(displayUnit.clone(), (arcstr::literal!(VAR_ATTR_DISPLAY_UNIT)).clone(), addMMLCode.clone())?;
                    dumpOptionDAEStateSelect(stateSel.clone(), (arcstr::literal!(VAR_ATTR_STATESELECT)).clone())?;
                    dumpOptExp(min.clone(), (arcstr::literal!(VAR_ATTR_MINVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(max.clone(), (arcstr::literal!(VAR_ATTR_MAXVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(nominal.clone(), (arcstr::literal!(VAR_ATTR_NOMINAL)).clone(), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), (arcstr::literal!(VAR_ATTR_INITIALVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(fixed.clone(), (arcstr::literal!(VAR_ATTR_FIXED)).clone(), addMMLCode.clone())?;
                    dumpStrCloseTag((Content.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: quant, min, max, start: Initial, fixed, uncertainOption: _, distributionOption: _, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _, addMMLCode) => {
                    dumpStrOpenTag((Content.clone()).clone())?;
                    dumpOptExp(quant.clone(), (arcstr::literal!(VAR_ATTR_QUANTITY)).clone(), addMMLCode.clone())?;
                    dumpOptExp(min.clone(), (arcstr::literal!(VAR_ATTR_MINVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(max.clone(), (arcstr::literal!(VAR_ATTR_MAXVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), (arcstr::literal!(VAR_ATTR_INITIALVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(fixed.clone(), (arcstr::literal!(VAR_ATTR_FIXED)).clone(), addMMLCode.clone())?;
                    dumpStrCloseTag((Content.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quant, start: Initial, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _, addMMLCode) => {
                    dumpStrOpenTag((Content.clone()).clone())?;
                    dumpOptExp(quant.clone(), (arcstr::literal!(VAR_ATTR_QUANTITY)).clone(), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), (arcstr::literal!(VAR_ATTR_INITIALVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(fixed.clone(), (arcstr::literal!(VAR_ATTR_FIXED)).clone(), addMMLCode.clone())?;
                    dumpStrCloseTag((Content.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quant, start: Initial, fixed: _, equationBound: _, isProtected: _, finalPrefix: _, .. }), _, addMMLCode) => {
                    dumpStrOpenTag((Content.clone()).clone())?;
                    dumpOptExp(quant.clone(), (arcstr::literal!(VAR_ATTR_QUANTITY)).clone(), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), (arcstr::literal!(VAR_ATTR_INITIALVALUE)).clone(), addMMLCode.clone())?;
                    dumpStrCloseTag((Content.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quant, min, max, start: Initial, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin: _ }), _, addMMLCode) => {
                    dumpStrOpenTag((Content.clone()).clone())?;
                    dumpOptExp(quant.clone(), (arcstr::literal!(VAR_ATTR_QUANTITY)).clone(), addMMLCode.clone())?;
                    dumpOptExp(min.clone(), (arcstr::literal!(VAR_ATTR_MINVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(max.clone(), (arcstr::literal!(VAR_ATTR_MAXVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(Initial.clone(), (arcstr::literal!(VAR_ATTR_INITIALVALUE)).clone(), addMMLCode.clone())?;
                    dumpOptExp(fixed.clone(), (arcstr::literal!(VAR_ATTR_FIXED)).clone(), addMMLCode.clone())?;
                    dumpStrCloseTag((Content.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    dumpComment((literal!("unknown VariableAttributes")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpDirectionStr(mut inVarDirection: DAE::VarDirection) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVarDirection.clone() {
        DAE::VarDirection::INPUT => {
            arcstr::literal!(VARDIR_INPUT)
        },
        DAE::VarDirection::OUTPUT => {
            arcstr::literal!(VARDIR_OUTPUT)
        },
        DAE::VarDirection::BIDIR => {
            arcstr::literal!(VARDIR_NONE)
        },
        _ => {
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.dumpDirectionStr - Unknown var direction")).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn dumpSolvedEqns(mut eqns: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)>>, mut inCount: i32, mut inContent: ArcStr, mut addMathMLCode: bool, mut dumpResiduals: bool, mut dumpSolved: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((eqns.clone(), inCount.clone(), inContent.clone(), addMathMLCode.clone(), dumpResiduals.clone(), dumpSolved.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: (Deref @ metamodelica::List::Nil, _), tail: rest }, _, _, _, _, _) => {
            dumpSolvedEqns(rest.clone(), inCount.clone(), (inContent.clone()).clone(), addMathMLCode.clone(), dumpResiduals.clone(), dumpSolved.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: (eqnsLst, varLst), tail: rest }, _, _, addMMLCode, _, _) => {
            dumpEqns2(eqnsLst.clone(), varLst.clone(), inCount.clone(), addMMLCode.clone(), dumpResiduals.clone(), dumpSolved.clone())?;
            dumpSolvedEqns(rest.clone(), inCount.clone() + 1, (inContent.clone()).clone(), addMathMLCode.clone(), dumpResiduals.clone(), dumpSolved.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpEqns(mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inContent: ArcStr, mut addMathMLCode: bool, mut dumpResiduals: bool, mut dumpSolved: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((eqns.clone(), inContent.clone(), addMathMLCode.clone(), dumpResiduals.clone(), dumpSolved.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            ()
        },
        (_, _, addMMLCode, _, _) => {
            let mut len: i32 = 0;
            len = (eqns.clone().len() as i32);
            dumpStrOpenTagAttr((inContent.clone()).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(len.clone())).clone())?;
            dumpEqns2(eqns.clone(), metamodelica::nil(), 1, addMMLCode.clone(), dumpResiduals.clone(), dumpSolved.clone())?;
            dumpStrCloseTag((inContent.clone()).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpEqns2(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inInteger: i32, mut addMathMLCode: bool, mut dumpResiduals: bool, mut dumpSolved: bool) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inEquationLst.clone(), inVarLst.clone(), inInteger.clone(), addMathMLCode.clone(), dumpResiduals.clone(), dumpSolved.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, _, index, addMMLCode, false, false) => {
                    dumpEquation(eqn.clone(), (intString(index.clone())).clone(), addMMLCode.clone())?;
                    dumpEqns2(eqns.clone(), inVarLst.clone(), index.clone() + 1, addMMLCode.clone(), false, false)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, _, index, addMMLCode, true, false) => {
                    dumpResidual(eqn.clone(), (intString(index.clone())).clone(), addMMLCode.clone())?;
                    dumpEqns2(eqns.clone(), inVarLst.clone(), index.clone() + 1, addMMLCode.clone(), true, false)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, Deref @ metamodelica::List::Cons { head: var, tail: vars }, index, addMMLCode, false, true) => {
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut varexp: Arc<DAE::Exp>;
                    let mut eqn = (*eqn).clone();
                    cref = BackendVariable::varCref(var.clone())?;
                    varexp = Expression::crefExp(cref.clone())?;
                    varexp = if (BackendVariable::isStateVar(var.clone())) {Expression::expDer(varexp.clone())} else {varexp.clone()};
                    eqn = BackendEquation::solveEquation(eqn.clone(), varexp.clone(), None)?;
                    dumpEquation(eqn.clone(), (intString(index.clone())).clone(), addMMLCode.clone())?;
                    dumpEqns2(eqns.clone(), vars.clone(), index.clone() + 1, addMMLCode.clone(), false, true)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, Deref @ metamodelica::List::Cons { head: _, tail: vars }, index, addMMLCode, false, true) => {
                    dumpEquation(eqn.clone(), (intString(index.clone())).clone(), addMMLCode.clone())?;
                    dumpEqns2(eqns.clone(), vars.clone(), index.clone() + 1, addMMLCode.clone(), false, true)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpEquation(mut inEquation: Arc<BackendDAE::Equation>, mut inIndexNumber: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inEquation.clone(), inIndexNumber.clone(), addMathMLCode.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, indexS, true) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(EQUATION)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpExp2(e1.clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((arcstr::literal!(EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(EQUATION)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }, _, true) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            s = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!("\n")).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(ARRAY_EQUATION)).clone(), (arcstr::literal!(EXP_STRING)).clone(), (s.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpExp2(e1.clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((arcstr::literal!(ARRAY_EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(ARRAY_OF_EQUATIONS)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(ARRAY_OF_EQUATIONS)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. }, _, true) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            s = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!("\n")).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(COMPLEX_EQUATION)).clone(), (arcstr::literal!(EXP_STRING)).clone(), (s.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpExp2(e1.clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((arcstr::literal!(COMPLEX_EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. }, indexS, _) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(COMPLEX_EQUATION)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(COMPLEX_EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. }, indexS, true) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" := ")).clone(), (s2.clone()).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(SOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpStrMathMLVariable((s1.clone()).clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(SOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" := ")).clone(), (s2.clone()).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(SOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(SOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst, condition: e1, .. }, .. }, indexS, true) => {
            let mut is: ArcStr = arcstr::literal!("");
            is = (printExpStr(e1.clone())?).clone();
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            dumpWhenOperatorLst(whenStmtLst.clone(), addMathMLCode.clone())?;
            dumpStrOpenTag((stringAppend((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(CONDITION)).clone())).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            Print::printBuf((is.clone()).clone())?;
            dumpExp(e1.clone(), true)?;
            dumpStrCloseTag((stringAppend((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(CONDITION)).clone())).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst, condition: e1, .. }, .. }, indexS, false) => {
            let mut is: ArcStr = arcstr::literal!("");
            is = (printExpStr(e1.clone())?).clone();
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            dumpWhenOperatorLst(whenStmtLst.clone(), addMathMLCode.clone())?;
            dumpStrTagContent((stringAppend((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(CONDITION)).clone())).clone(), (is.clone()).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, indexS, true) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = 0")).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(RESIDUAL)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpExp2(e.clone())?;
            dumpStrMathMLNumber((literal!("0")).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(RESIDUAL)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = 0")).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(RESIDUAL)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(RESIDUAL)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { source, alg: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, indexS, _) => {
            dumpStrOpenTagAttr((arcstr::literal!(ALGORITHM)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((Util::xmlEscape((DAEDump::dumpAlgorithmsStr(list![Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts.clone() }), source: source.clone() })])?).clone())?).clone())?;
            dumpStrCloseTag((arcstr::literal!(ALGORITHM)).clone())?;
            ()
        },
        _ => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (literal!("in XMLDump.dumpEquation - Unknown equation")).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(res.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpExp(mut e: Arc<DAE::Exp>, mut addMathMLCode: bool) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (e.clone(), addMathMLCode.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inExp, true) => {
                    dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
                    dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
                    dumpExp2(inExp.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpExp2(mut inExp: Arc<DAE::Exp>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { integer: x } => {
                    dumpStrMathMLNumberAttr((intString(x.clone())).clone(), (arcstr::literal!(MathMLType)).clone(), (arcstr::literal!(MathMLInteger)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { real: rval } => {
                    dumpStrMathMLNumberAttr((realString(rval.clone())).clone(), (arcstr::literal!(MathMLType)).clone(), (arcstr::literal!(MathMLReal)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    dumpStrMathMLNumberAttr((Util::xmlEscape((s.clone()).clone())?).clone(), (arcstr::literal!(MathMLType)).clone(), (arcstr::literal!(MathMLConstant)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: false } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLFalse)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: true } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLTrue)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: c, .. } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    dumpStrMathMLVariable((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    sym = (binopSymbol(op.clone())?).clone();
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((sym.clone()).clone())?;
                    dumpExp2(e1.clone())?;
                    dumpExp2(e2.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    sym = (unaryopSymbol(op.clone())?).clone();
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((sym.clone()).clone())?;
                    dumpExp2(e1.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    sym = (lbinopSymbol(op.clone())?).clone();
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((sym.clone()).clone())?;
                    dumpExp2(e1.clone())?;
                    dumpExp2(e2.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    sym = (lunaryopSymbol(op.clone())?).clone();
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((sym.clone()).clone())?;
                    dumpExp2(e1.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { exp2: e2, operator: op, exp1: e1, .. } => {
                    let mut sym: ArcStr = arcstr::literal!("");
                    sym = (relopSymbol(op.clone())?).clone();
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((sym.clone()).clone())?;
                    dumpExp2(e1.clone())?;
                    dumpExp2(e2.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: tb, expElse: fb } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLIfClause)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLIfBranch)).clone())?;
                    dumpExp2(tb.clone())?;
                    dumpExp2(cond.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLIfBranch)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLElseBranch)).clone())?;
                    dumpExp2(fb.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLElseBranch)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLIfClause)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((literal!("diff")).clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLArccos)).clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLArcsin)).clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLArctan)).clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!("atan2")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!("(")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpComment((literal!("atan2 is not a MathML element it could be possible to use arg in future")).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLLn)).clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLLog)).clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: args, path: fcn, .. } => {
                    let mut fs: ArcStr = arcstr::literal!("");
                    fs = AbsynUtil::pathStringNoQual(fcn.clone(), (literal!(".")).clone(), true, false)?;
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((fs.clone()).clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: es, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLTranspose)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLVector)).clone())?;
                    dumpList(es.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLVector)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: es } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLTranspose)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLVector)).clone())?;
                    dumpList(es.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLVector)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: ebs, .. } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLMatrix)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLMatrixrow)).clone())?;
                    dumpListSeparator(ebs.clone(), Arc::new(dumpRow), stringAppendList(list![(literal!("\n</")).clone(), (arcstr::literal!(MathMLMatrixrow)).clone(), (literal!(">\n<")).clone(), (arcstr::literal!(MathMLMatrixrow)).clone(), (literal!(">")).clone()]))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLMatrixrow)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLMatrix)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start, step: None, stop } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLInterval)).clone())?;
                    dumpExp2(start.clone())?;
                    dumpExp2(stop.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLInterval)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start, step: Some(step), stop } => {
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!("{")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpExp2(start.clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!(":")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpExp2(step.clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!(":")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpExp2(stop.clone())?;
                    dumpComment((literal!("Interval range specification is not supported by MathML standard")).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!("}")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp: Deref @ DAE::Exp::ICONST { integer: ival }, ty: Deref @ DAE::Type::T_REAL { .. } } => {
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut rval: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let false = (Config::modelicaOutput()?) else { bail!("pattern mismatch") };
                    rval = intReal(ival.clone());
                    res = (realString(rval.clone())).clone();
                    dumpStrMathMLNumberAttr((res.clone()).clone(), (arcstr::literal!(MathMLType)).clone(), (arcstr::literal!(MathMLReal)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::ICONST { integer: ival }, operator: DAE::Operator::UMINUS { .. } }, ty: Deref @ DAE::Type::T_REAL { .. } } => {
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut rval: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let false = (Config::modelicaOutput()?) else { bail!("pattern mismatch") };
                    rval = intReal(ival.clone());
                    res = (realString(rval.clone())).clone();
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLMinus)).clone())?;
                    dumpStrMathMLNumberAttr((res.clone()).clone(), (arcstr::literal!(MathMLType)).clone(), (arcstr::literal!(MathMLReal)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp: e, ty: Deref @ DAE::Type::T_REAL { .. } } => {
                    let false = (Config::modelicaOutput()?) else { bail!("pattern mismatch") };
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLReal)).clone())?;
                    dumpExp2(e.clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp: e, ty: Deref @ DAE::Type::T_REAL { .. } } => {
                    let true = (Config::modelicaOutput()?) else { bail!("pattern mismatch") };
                    dumpExp2(e.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp: e, ty: tp } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (TypesDump::unparseType(tp.clone())?).clone();
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!("(")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!("CAST as ")).clone())?;
                    Print::printBuf((r#str.clone()).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpExp2(e.clone())?;
                    dumpComment((literal!("CAST operator is not supported by MathML standard.")).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MathMLOperator)).clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLOperator)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { sub: subs, exp: e1 } => {
                    let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    args = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
                    dumpStrVoidTag((arcstr::literal!(MathMLSelector)).clone())?;
                    dumpExp2(e1.clone())?;
                    dumpList(args.clone(), Arc::new(dumpExp2))?;
                    dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ENUM_LITERAL { name: fcn, .. } => {
                    dumpStrMathMLVariable(AbsynUtil::pathStringNoQual(fcn.clone(), (literal!(".")).clone(), true, false)?)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { sz: Some(_), .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { sz: None, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LIST { .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CONS { .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    dumpComment(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNKNOWN EXPRESSION: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpExtObjCls(mut cls: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>, mut Content: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((cls.clone(), Content.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (xs, _) => {
            let mut len: i32 = 0;
            len = (xs.clone().len() as i32);
            dumpStrOpenTagAttr((stringAppend((stringAppend((arcstr::literal!(EXTERNAL)).clone(), (arcstr::literal!(CLASSES_)).clone())).clone(), (arcstr::literal!(LIST_)).clone())).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(len.clone())).clone())?;
            dumpExtObjCls2(xs.clone(), (stringAppend((arcstr::literal!(EXTERNAL)).clone(), (arcstr::literal!(CLASS_)).clone())).clone())?;
            dumpStrCloseTag((stringAppend((stringAppend((arcstr::literal!(EXTERNAL)).clone(), (arcstr::literal!(CLASSES_)).clone())).clone(), (arcstr::literal!(LIST_)).clone())).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpExtObjCls2(mut cls: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>, mut Content: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((cls.clone(), Content.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::ExternalObjectClass { path, source: _ }, tail: xs }, c) => {
            dumpStrOpenTag((c.clone()).clone())?;
            Print::printBuf((literal!("class ")).clone())?;
            Print::printBuf(AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?)?;
            Print::printBuf((literal!("\n  extends ExternalObject")).clone())?;
            Print::printBuf((literal!("end")).clone())?;
            Print::printBuf(AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?)?;
            dumpStrCloseTag((c.clone()).clone())?;
            dumpExtObjCls2(xs.clone(), (c.clone()).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpFlowStr(mut inVarFlow: Arc<DAE::ConnectorType>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inVarFlow.clone()) {
        Deref @ DAE::ConnectorType::FLOW => arcstr::literal!(VAR_FLOW_FLOW),
        Deref @ DAE::ConnectorType::POTENTIAL => arcstr::literal!(VAR_FLOW_NONFLOW),
        Deref @ DAE::ConnectorType::STREAM { .. } => arcstr::literal!(VAR_FLOW_NONFLOW),
        Deref @ DAE::ConnectorType::NON_CONNECTOR => arcstr::literal!(VAR_FLOW_NONCONNECTOR),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn dumpFunctions(mut funcelems: Arc<metamodelica::List<DAE::Function>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = funcelems.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    dumpStrOpenTag((arcstr::literal!(FUNCTIONS)).clone())?;
                    dumpFunctions2(funcelems.clone())?;
                    dumpStrCloseTag((arcstr::literal!(FUNCTIONS)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpFunctions2(mut funcelems: Arc<metamodelica::List<DAE::Function>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(funcelems.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: fun, tail: rem_fun } => {
            dumpFunctions3(fun.clone())?;
            dumpFunctions2(rem_fun.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpFunctions3(mut fun: DAE::Function) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = fun.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { type_: Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: _, .. }, .. }, .. }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    dumpStrOpenTagAttr((arcstr::literal!(FUNCTION)).clone(), (arcstr::literal!(FUNCTION_NAME)).clone(), (Util::xmlEscape(AbsynUtil::pathStringNoQual(DAEUtil::functionName(fun.clone())?, (literal!(".")).clone(), true, false)?)?).clone())?;
                    dumpStrOpenTag((arcstr::literal!(MODELICA_IMPLEMENTATION)).clone())?;
                    Print::printBuf((Util::xmlEscape((DAEDump::dumpFunctionStr(fun.clone())?).clone())?).clone())?;
                    dumpStrCloseTag((arcstr::literal!(MODELICA_IMPLEMENTATION)).clone())?;
                    dumpStrCloseTag((arcstr::literal!(FUNCTION)).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpAdjacencyMatrix(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
    dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
    dumpStrOpenTag((arcstr::literal!(MathMLMatrix)).clone())?;
    let _ = BackendDAEUtil::foldEqSystem(dae.clone(), Arc::new(dumpAdjacencyMatrixWork.clone()), 0)?;
    dumpStrCloseTag((arcstr::literal!(MathMLMatrix)).clone())?;
    dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
    dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
    Ok(())
}

fn dumpAdjacencyMatrix2(mut row: Arc<metamodelica::List<i32>>, mut inTpl: (i32, i32)) -> Result<(i32, i32)> {
    let mut outTpl: (i32, i32);
    let mut offset: i32 = 0;
    let mut c: i32 = 0;
    (offset, c) = inTpl.clone();
    dumpStrOpenTagAttr((arcstr::literal!(MathMLMatrixrow)).clone(), (literal!("id")).clone(), (intString(c.clone())).clone())?;
    List::map1_0(row.clone(), Arc::new(dumpMatrixIntegerRow), offset.clone());
    dumpStrCloseTag((arcstr::literal!(MathMLMatrixrow)).clone())?;
    outTpl = (offset.clone(), c.clone() + 1);
    Ok(outTpl)
}

fn dumpMatrixIntegerRow(mut x: i32, mut offset: i32) -> Result<()> {
    let mut e: i32 = 0;
    let mut s: ArcStr = arcstr::literal!("");
    e = if (intGt(x.clone(), 0)) {x.clone() + offset.clone()} else {x.clone() - offset.clone()};
    s = (intString(e.clone())).clone();
    dumpStrOpenTag((arcstr::literal!(MathMLVariable)).clone())?;
    Print::printBuf((s.clone()).clone())?;
    dumpStrCloseTag((arcstr::literal!(MathMLVariable)).clone())?;
    Ok(())
}

fn dumpKind(mut inVarKind: BackendDAE::VarKind) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVarKind.clone() {
        BackendDAE::VarKind::VARIABLE => {
            arcstr::literal!(VARIABILITY_CONTINUOUS)
        },
        BackendDAE::VarKind::STATE { .. } => {
            arcstr::literal!(VARIABILITY_CONTINUOUS_STATE)
        },
        BackendDAE::VarKind::DUMMY_DER => {
            arcstr::literal!(VARIABILITY_CONTINUOUS_DUMMYDER)
        },
        BackendDAE::VarKind::DUMMY_STATE => {
            arcstr::literal!(VARIABILITY_CONTINUOUS_DUMMYSTATE)
        },
        BackendDAE::VarKind::DISCRETE => {
            arcstr::literal!(VARIABILITY_DISCRETE)
        },
        BackendDAE::VarKind::PARAM => {
            arcstr::literal!(VARIABILITY_PARAMETER)
        },
        BackendDAE::VarKind::CONST => {
            arcstr::literal!(VARIABILITY_CONSTANT)
        },
        BackendDAE::VarKind::EXTOBJ { fullClassName: ref path } => {
            stringAppend((arcstr::literal!(VARIABILITY_EXTERNALOBJECT)).clone(), (stringAppend((literal!(":")).clone(), AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?)).clone())
        },
        _ => {
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.dumpKind - Unknown kind")).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn dumpList<Type_a: Clone + 'static>(mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone> = fn(Type_a) -> Result<()>;

    let _ = 'mc: {
        let __mc_input = (inTypeALst.clone(), inFuncTypeTypeATo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, r) => {
                    r(h.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, r) => {
                    r(h.clone())?;
                    dumpList(t.clone(), r.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpListSeparator<Type_a: Clone + 'static>(mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>, mut inString: ArcStr) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone> = fn(Type_a) -> Result<()>;

    let _ = 'mc: {
        let __mc_input = (inTypeALst.clone(), inFuncTypeTypeATo.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, r, _) => {
                    r(h.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, r, sep) => {
                    r(h.clone())?;
                    Print::printBuf((sep.clone()).clone())?;
                    dumpListSeparator(t.clone(), r.clone(), (sep.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printExpStr(mut e: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (Util::xmlEscape((ExpressionBasics::printExpStr(e.clone())?).clone())?).clone();
    Ok(s)
}

fn dumpLstInt(mut inLstStr: Arc<metamodelica::List<i32>>, mut inElementName: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inLstStr.clone(), inElementName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, Deref @ "") => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, _) => {
                    dumpStrTagContent((inElementName.clone()).clone(), (intString(h.clone())).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, _) => {
                    dumpStrTagContent((inElementName.clone()).clone(), (intString(h.clone())).clone())?;
                    dumpLstInt(t.clone(), (inElementName.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpLstIntAttr(mut lst: Arc<metamodelica::List<i32>>, mut inContent: ArcStr, mut inElementContent: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (lst.clone(), inContent.clone(), inElementContent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (l, inLst, inEl) => {
                    dumpStrOpenTag((inLst.clone()).clone())?;
                    dumpLstInt(l.clone(), (inEl.clone()).clone())?;
                    dumpStrCloseTag((inLst.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpMatching(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    dumpStrOpenTag((arcstr::literal!(MATCHING_ALGORITHM)).clone())?;
    let _ = BackendDAEUtil::foldEqSystem(dae.clone(), Arc::new(dumpMatchingWork), (0, 0))?;
    dumpStrCloseTag((arcstr::literal!(MATCHING_ALGORITHM)).clone())?;
    Ok(())
}

fn dumpMatchingWork(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut inOffset: (i32, i32)) -> Result<(i32, i32)> {
    let mut outOffset: (i32, i32);
    let mut v1: metamodelica::Array<i32>;
    let mut v2: metamodelica::Array<i32>;
    let mut voffset: i32 = 0;
    let mut eoffset: i32 = 0;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: _ }, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    v1 = __pa0.clone();
    v2 = __pa1.clone();
    (voffset, eoffset) = inOffset.clone();
    dumpMatching1(v1.clone(), voffset.clone(), eoffset.clone())?;
    outOffset = (voffset.clone() + (v1.clone().borrow().len() as i32), eoffset.clone() + (v2.clone().borrow().len() as i32));
    Ok(outOffset)
}

fn dumpMatching1(mut v: metamodelica::Array<i32>, mut voffset: i32, mut eoffset: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (v.clone(), voffset.clone(), eoffset.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt((v.clone().borrow().len() as i32), 0)) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt((v.clone().borrow().len() as i32), 0)) else { bail!("pattern mismatch") };
            let _ = Array::fold(v.clone(), Arc::new(dumpMatching2), (1, voffset.clone(), eoffset.clone()));
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpMatching2(mut eqn: i32, mut inTpl: (i32, i32, i32)) -> Result<(i32, i32, i32)> {
    let mut outTpl: (i32, i32, i32);
    let mut v: i32 = 0;
    let mut voffset: i32 = 0;
    let mut eoffset: i32 = 0;
    let mut s: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    (v, voffset, eoffset) = inTpl.clone();
    s = (intString(v.clone() + voffset.clone())).clone();
    s2 = (intString(eqn.clone() + eoffset.clone())).clone();
    Print::printBuf(stringAppendList(list![(literal!("\n<")).clone(), (arcstr::literal!(SOLVED_IN)).clone(), (literal!(" ")).clone(), (arcstr::literal!(VARIABLE)).clone(), (arcstr::literal!(ID_)).clone(), (literal!("=\"")).clone(), (s.clone()).clone(), (literal!("\" ")).clone(), (arcstr::literal!(EQUATION)).clone(), (arcstr::literal!(ID_)).clone(), (literal!("=\"")).clone(), (s2.clone()).clone(), (literal!("\" ")).clone(), (literal!("/>")).clone()]))?;
    outTpl = (v.clone() + 1, voffset.clone(), eoffset.clone());
    Ok(outTpl)
}

fn dumpOptExp(mut inExpExpOption: Option<Arc<DAE::Exp>>, mut Content: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inExpExpOption.clone(), Content.clone(), addMathMLCode.clone())) {
        (None, _, _) => {
            ()
        },
        (Some(e), _, _) => {
            dumpStrOpenTagAttr((Content.clone()).clone(), (arcstr::literal!(EXP_STRING)).clone(), (printExpStr(e.clone())?).clone())?;
            dumpExp(e.clone(), addMathMLCode.clone())?;
            dumpStrCloseTag((Content.clone()).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpOptInteger(mut inOption: Option<i32>, mut Content: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = (match (inOption.clone(), Content.clone(), addMathMLCode.clone()) {
        (None, _, _) => {
            ()
        },
        (Some(mut i), _, _) => {
            dumpStrOpenTagAttr((Content.clone()).clone(), (arcstr::literal!(INDEX)).clone(), (intString(i.clone())).clone())?;
            dumpStrCloseTag((Content.clone()).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn dumpOptionDAEStateSelect(mut ss: Option<DAE::StateSelect>, mut Content: ArcStr) -> Result<()> {
    let _ = (match (ss.clone(), Content.clone()) {
        (None, _) => {
            Print::printBuf((literal!("")).clone())?;
            ()
        },
        (Some(DAE::StateSelect::NEVER), _) => {
            dumpStrTagContent((Content.clone()).clone(), (arcstr::literal!(STATE_SELECT_NEVER)).clone())?;
            ()
        },
        (Some(DAE::StateSelect::AVOID), _) => {
            dumpStrTagContent((Content.clone()).clone(), (arcstr::literal!(STATE_SELECT_AVOID)).clone())?;
            ()
        },
        (Some(DAE::StateSelect::DEFAULT), _) => {
            dumpStrTagContent((Content.clone()).clone(), (arcstr::literal!(STATE_SELECT_DEFAULT)).clone())?;
            ()
        },
        (Some(DAE::StateSelect::PREFER), _) => {
            dumpStrTagContent((Content.clone()).clone(), (arcstr::literal!(STATE_SELECT_PREFER)).clone())?;
            ()
        },
        (Some(DAE::StateSelect::ALWAYS), _) => {
            dumpStrTagContent((Content.clone()).clone(), (arcstr::literal!(STATE_SELECT_ALWAYS)).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn dumpRow(mut es_1: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<()> {
    dumpList(es_1.clone(), Arc::new(dumpExp2))?;
    Ok(())
}

fn dumpSolvingInfo(mut addOriginalAdjacencyMatrix: bool, mut addSolvingInfo: bool, mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), inBackendDAE.clone())) {
        (false, false, _) => {
            ()
        },
        (true, true, _) => {
            let mut dlow: Arc<BackendDAE::BackendDAE>;
            dlow = BackendDAEUtil::transformBackendDAE(inBackendDAE.clone(), None, None, None)?;
            dumpStrOpenTag((arcstr::literal!(ADDITIONAL_INFO)).clone())?;
            dumpStrOpenTag((arcstr::literal!(ORIGINAL_ADJACENCY_MATRIX)).clone())?;
            dumpAdjacencyMatrix(dlow.clone())?;
            dumpStrCloseTag((arcstr::literal!(ORIGINAL_ADJACENCY_MATRIX)).clone())?;
            dumpStrOpenTag((arcstr::literal!(SOLVING_INFO)).clone())?;
            dumpMatching(dlow.clone())?;
            dumpComponents(dlow.clone())?;
            dumpStrCloseTag((arcstr::literal!(SOLVING_INFO)).clone())?;
            dumpStrCloseTag((arcstr::literal!(ADDITIONAL_INFO)).clone())?;
            ()
        },
        (true, false, _) => {
            dumpStrOpenTag((arcstr::literal!(ADDITIONAL_INFO)).clone())?;
            dumpStrOpenTag((arcstr::literal!(ORIGINAL_ADJACENCY_MATRIX)).clone())?;
            dumpAdjacencyMatrix(inBackendDAE.clone())?;
            dumpStrCloseTag((arcstr::literal!(ORIGINAL_ADJACENCY_MATRIX)).clone())?;
            dumpStrCloseTag((arcstr::literal!(ADDITIONAL_INFO)).clone())?;
            ()
        },
        (false, true, _) => {
            let mut dlow: Arc<BackendDAE::BackendDAE>;
            dlow = BackendDAEUtil::transformBackendDAE(inBackendDAE.clone(), None, None, None)?;
            dumpStrOpenTag((arcstr::literal!(ADDITIONAL_INFO)).clone())?;
            dumpStrOpenTag((arcstr::literal!(SOLVING_INFO)).clone())?;
            dumpMatching(dlow.clone())?;
            dumpComponents(dlow.clone())?;
            dumpStrCloseTag((arcstr::literal!(SOLVING_INFO)).clone())?;
            dumpStrCloseTag((arcstr::literal!(ADDITIONAL_INFO)).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn transformModelicaIdentifierToXMLElementTag(mut modelicaIdentifier: ArcStr) -> Result<ArcStr> {
    let mut xmlElementTag: ArcStr = arcstr::literal!("");
    xmlElementTag = (System::stringReplace((modelicaIdentifier.clone()).clone(), (literal!("$")).clone(), (literal!("_dollar_")).clone())?).clone();
    Ok(xmlElementTag)
}

fn dumpStrCloseTag(mut inContent: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inContent.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                inString => {
                    Print::printBuf((literal!("\n</")).clone())?;
                    Print::printBuf((transformModelicaIdentifierToXMLElementTag((inString.clone()).clone())?).clone())?;
                    Print::printBuf((literal!(">")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpStreamStr(mut inVarStream: Arc<DAE::ConnectorType>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inVarStream.clone()) {
        Deref @ DAE::ConnectorType::STREAM { .. } => arcstr::literal!(VAR_STREAM_STREAM),
        Deref @ DAE::ConnectorType::POTENTIAL => arcstr::literal!(VAR_STREAM_NONSTREAM),
        Deref @ DAE::ConnectorType::FLOW => arcstr::literal!(VAR_STREAM_NONSTREAM),
        Deref @ DAE::ConnectorType::NON_CONNECTOR => arcstr::literal!(VAR_STREAM_NONSTREAM_CONNECTOR),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn dumpStrMathMLNumber(mut inNumber: ArcStr) -> Result<()> {
    dumpStrOpenTag((arcstr::literal!(MathMLNumber)).clone())?;
    Print::printBuf((inNumber.clone()).clone())?;
    dumpStrCloseTag((arcstr::literal!(MathMLNumber)).clone())?;
    Ok(())
}

fn dumpStrMathMLNumberAttr(mut inNumber: ArcStr, mut inAttribute: ArcStr, mut inAttributeContent: ArcStr) -> Result<()> {
    dumpStrOpenTagAttr((arcstr::literal!(MathMLNumber)).clone(), (inAttribute.clone()).clone(), (inAttributeContent.clone()).clone())?;
    Print::printBuf((inNumber.clone()).clone())?;
    dumpStrCloseTag((arcstr::literal!(MathMLNumber)).clone())?;
    Ok(())
}

fn dumpStrMathMLVariable(mut inVariable: ArcStr) -> Result<()> {
    dumpStrOpenTag((arcstr::literal!(MathMLVariable)).clone())?;
    Print::printBuf((inVariable.clone()).clone())?;
    dumpStrCloseTag((arcstr::literal!(MathMLVariable)).clone())?;
    Ok(())
}

fn dumpStrOpenTag(mut inContent: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inContent.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    Print::printBuf((literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                inString => {
                    Print::printBuf((literal!("\n<")).clone())?;
                    Print::printBuf((transformModelicaIdentifierToXMLElementTag((inString.clone()).clone())?).clone())?;
                    Print::printBuf((literal!(">")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpStrOpenTagAttr(mut inContent: ArcStr, mut Attribute: ArcStr, mut AttributeContent: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inContent.clone(), Attribute.clone(), AttributeContent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", _, _) => {
                    Print::printBuf((literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ "", _) => {
                    Print::printBuf((literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ "") => {
                    Print::printBuf((literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, Deref @ "", _) => {
                    dumpStrOpenTag((transformModelicaIdentifierToXMLElementTag((inString.clone()).clone())?).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, _, Deref @ "") => {
                    dumpStrOpenTag((transformModelicaIdentifierToXMLElementTag((inString.clone()).clone())?).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, _, inAttributeContent) => {
                    Print::printBuf((literal!("\n<")).clone())?;
                    Print::printBuf((transformModelicaIdentifierToXMLElementTag((inString.clone()).clone())?).clone())?;
                    Print::printBuf((literal!(" ")).clone())?;
                    Print::printBuf((Attribute.clone()).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((inAttributeContent.clone()).clone())?;
                    Print::printBuf((literal!("\">")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpStrTagAttrNoChild(mut inContent: ArcStr, mut Attribute: ArcStr, mut AttributeContent: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inContent.clone(), Attribute.clone(), AttributeContent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", _, _) => {
                    Print::printBuf((literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ "", _) => {
                    Print::printBuf((literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ "") => {
                    Print::printBuf((literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, Deref @ "", _) => {
                    dumpStrOpenTag((transformModelicaIdentifierToXMLElementTag((inString.clone()).clone())?).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, _, Deref @ "") => {
                    dumpStrOpenTag((transformModelicaIdentifierToXMLElementTag((inString.clone()).clone())?).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inString, _, inAttributeContent) => {
                    Print::printBuf((literal!("\n<")).clone())?;
                    Print::printBuf((transformModelicaIdentifierToXMLElementTag((inString.clone()).clone())?).clone())?;
                    Print::printBuf((literal!(" ")).clone())?;
                    Print::printBuf((Attribute.clone()).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((inAttributeContent.clone()).clone())?;
                    Print::printBuf((literal!("\" />")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpStrTagContent(mut inElementName: ArcStr, mut inContent: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inElementName.clone(), inContent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ "") => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inTagString, inTagContent) => {
                    dumpStrOpenTag((inTagString.clone()).clone())?;
                    Print::printBuf((literal!("\n")).clone())?;
                    Print::printBuf((inTagContent.clone()).clone())?;
                    dumpStrCloseTag((inTagString.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpStrVoidTag(mut inElementName: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElementName.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ElementName => {
                    Print::printBuf((literal!("\n<")).clone())?;
                    Print::printBuf((transformModelicaIdentifierToXMLElementTag((ElementName.clone()).clone())?).clone())?;
                    Print::printBuf((literal!("/>")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpDimension(mut inDimension: Arc<DAE::Dimension>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inDimension.clone()) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            Print::printBuf((intString(i.clone())).clone())?;
            ()
        },
        Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: _, literals: _, size: _ } => {
            Print::printBuf((literal!("Dim Enum")).clone())?;
            ()
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: e1 } => {
            Print::printBuf((printExpStr(e1.clone())?).clone())?;
            ()
        },
        Deref @ DAE::Dimension::DIM_UNKNOWN => {
            Print::printBuf((literal!(":")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpTypeStr(mut inType: Arc<DAE::Type>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            arcstr::literal!(VARTYPE_INTEGER)
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            arcstr::literal!(VARTYPE_REAL)
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            arcstr::literal!(VARTYPE_BOOLEAN)
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            arcstr::literal!(VARTYPE_STRING)
        },
        Deref @ DAE::Type::T_ENUMERATION { names: l, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            s1 = stringDelimitList(l.clone(), (literal!(", ")).clone());
            s2 = (stringAppend((arcstr::literal!(VARTYPE_ENUMERATION)).clone(), (stringAppend((literal!("(")).clone(), (s1.clone()).clone())).clone())).clone();
            r#str = (stringAppend((s2.clone()).clone(), (literal!(")")).clone())).clone();
            r#str.clone()
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. } => {
            arcstr::literal!(VARTYPE_EXTERNALOBJECT)
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn dumpVariable(mut varno: ArcStr, mut cr: ArcStr, mut kind: ArcStr, mut dir: ArcStr, mut var_type: ArcStr, mut indx: ArcStr, mut derName: ArcStr, mut varFixed: ArcStr, mut flowPrefix: ArcStr, mut streamPrefix: ArcStr, mut comment: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (varno.clone(), cr.clone(), kind.clone(), dir.clone(), var_type.clone(), indx.clone(), derName.clone(), varFixed.clone(), flowPrefix.clone(), streamPrefix.clone(), comment.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, Deref @ "") => {
                    Print::printBuf((literal!("\n<")).clone())?;
                    Print::printBuf((arcstr::literal!(VARIABLE)).clone())?;
                    Print::printBuf((literal!(" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_ID)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((varno.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_NAME)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((cr.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_VARIABILITY)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((kind.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_DIRECTION)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((dir.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_TYPE)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((var_type.clone()).clone())?;
                    printIndexAndDerName((indx.clone()).clone(), (derName.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_FIXED)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((varFixed.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_FLOW)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((flowPrefix.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_STREAM)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((streamPrefix.clone()).clone())?;
                    Print::printBuf((literal!("\">")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _) => {
                    Print::printBuf((literal!("\n<")).clone())?;
                    Print::printBuf((arcstr::literal!(VARIABLE)).clone())?;
                    Print::printBuf((literal!(" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_ID)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((varno.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_NAME)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((cr.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_VARIABILITY)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((kind.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_DIRECTION)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((dir.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_TYPE)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((var_type.clone()).clone())?;
                    printIndexAndDerName((indx.clone()).clone(), (derName.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_FIXED)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((varFixed.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_FLOW)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((flowPrefix.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_STREAM)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((streamPrefix.clone()).clone())?;
                    Print::printBuf((literal!("\" ")).clone())?;
                    Print::printBuf((arcstr::literal!(VAR_COMMENT)).clone())?;
                    Print::printBuf((literal!("=\"")).clone())?;
                    Print::printBuf((Util::xmlEscape((comment.clone()).clone())?).clone())?;
                    Print::printBuf((literal!("\">")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printIndexAndDerName(mut indx: ArcStr, mut derName: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((indx.clone(), derName.clone())) {
        (Deref @ "", Deref @ "") => (),
        (_, Deref @ "") => {
            Print::printBuf((literal!("\" ")).clone())?;
            Print::printBuf((arcstr::literal!(VAR_INDEX)).clone())?;
            Print::printBuf((literal!("=\"")).clone())?;
            Print::printBuf((indx.clone()).clone())?;
            ()
        },
        (Deref @ "", _) => {
            Print::printBuf((literal!("\" ")).clone())?;
            Print::printBuf((arcstr::literal!(VAR_DERNAME)).clone())?;
            Print::printBuf((literal!("=\"")).clone())?;
            Print::printBuf((derName.clone()).clone())?;
            ()
        },
        (_, _) => {
            Print::printBuf((literal!("\" ")).clone())?;
            Print::printBuf((arcstr::literal!(VAR_INDEX)).clone())?;
            Print::printBuf((literal!("=\"")).clone())?;
            Print::printBuf((indx.clone()).clone())?;
            Print::printBuf((literal!("\" ")).clone())?;
            Print::printBuf((arcstr::literal!(VAR_DERNAME)).clone())?;
            Print::printBuf((literal!("=\"")).clone())?;
            Print::printBuf((derName.clone()).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpVarsAdditionalInfo(mut crefIdxLstArr: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>, mut i: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (crefIdxLstArr.clone(), i.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            if !((crefIdxLstArr.borrow()[(1-1) as usize].clone().is_empty())) { bail!("guard") }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            dumpStrOpenTag((arcstr::literal!(ADDITIONAL_INFO)).clone())?;
            dumpCrefIdxLstArr(crefIdxLstArr.clone(), (arcstr::literal!(HASH_TB_CREFS_LIST)).clone(), i.clone())?;
            dumpStrCloseTag((arcstr::literal!(ADDITIONAL_INFO)).clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.dumpVarsAdditionalInfo - Unknown info")).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpVars(mut vars: Arc<metamodelica::List<BackendDAE::Var>>, mut crefIdxLstArr: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>, mut Content: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (vars.clone(), crefIdxLstArr.clone(), Content.clone(), addMathMLCode.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, addMMLCode) => {
                    if !((!(crefIdxLstArr.borrow()[(1-1) as usize].clone().is_empty()))) { bail!("guard") }
                    let mut len: i32 = 0;
                    len = (vars.clone().len() as i32);
                    dumpStrOpenTagAttr((Content.clone()).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(len.clone())).clone())?;
                    dumpStrOpenTag((stringAppend((arcstr::literal!(VARIABLES)).clone(), (arcstr::literal!(LIST_)).clone())).clone())?;
                    dumpVars2(vars.clone(), 1, addMMLCode.clone())?;
                    dumpStrCloseTag((stringAppend((arcstr::literal!(VARIABLES)).clone(), (arcstr::literal!(LIST_)).clone())).clone())?;
                    dumpStrCloseTag((Content.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, addMMLCode) => {
                    let mut len: i32 = 0;
                    len = (vars.clone().len() as i32);
                    dumpStrOpenTagAttr((Content.clone()).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(len.clone())).clone())?;
                    dumpStrOpenTag((stringAppend((arcstr::literal!(VARIABLES)).clone(), (arcstr::literal!(LIST_)).clone())).clone())?;
                    dumpVars2(vars.clone(), 1, addMMLCode.clone())?;
                    dumpStrCloseTag((stringAppend((arcstr::literal!(VARIABLES)).clone(), (arcstr::literal!(LIST_)).clone())).clone())?;
                    dumpStrCloseTag((Content.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getIndex(mut kind: BackendDAE::VarKind) -> ArcStr {
    let mut diffIndex: ArcStr = arcstr::literal!("");
    diffIndex = ((match kind.clone() {
        BackendDAE::VarKind::STATE { index: mut di, .. } => {
            intString(di.clone())
        },
        _ => {
            literal!("")
        },
    })).clone();
    diffIndex
}

fn getDerName(mut kind: BackendDAE::VarKind) -> Result<ArcStr> {
    let mut derName: ArcStr = arcstr::literal!("");
    derName = ((::match_deref::match_deref! { match &(kind.clone()) {
        BackendDAE::VarKind::STATE { derName: Some(cr), .. } => {
            let mut dn: ArcStr = arcstr::literal!("");
            dn = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            dn.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(derName)
}

fn dumpVars2(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inInteger: i32, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inVarLst.clone(), inInteger.clone(), addMathMLCode.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: v @ BackendDAE::Var { .. }, tail: xs }, varno, addMMLCode) if (BackendVariable::isParam(v.clone()) && Types::isArray(v.varType.clone())) => {
            let mut scalarVar: BackendDAE::Var;
            let mut scalar_crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut varno = (*varno).clone();
            scalar_crefs = ComponentReference::expandCref(v.varName.clone(), false)?;
            for mut cref in &*scalar_crefs.clone() {
                let mut cref = cref.clone();
                scalarVar = BackendVariable::copyVarNewName(cref.clone(), v.clone());
                scalarVar.varType = ComponentReference::crefTypeFull(cref.clone())?;
                dumpVars2(list![scalarVar.clone()], varno.clone(), addMMLCode.clone())?;
                varno = varno.clone() + 1;
            }
            dumpVars2(xs.clone(), varno.clone(), addMMLCode.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: v @ BackendDAE::Var { connectorType: ct, comment, values: dae_var_attr, source, bindExp: e, varType: var_type, varDirection: dir, varKind: kind, varName: cr, .. }, tail: xs }, varno, addMMLCode) => {
            let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut var_1: i32 = 0;
            dumpVariable((intString(varno.clone())).clone(), (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone(), (dumpKind(kind.clone())?).clone(), (dumpDirectionStr(dir.clone())?).clone(), (dumpTypeStr(var_type.clone())?).clone(), (getIndex(kind.clone())).clone(), (getDerName(kind.clone())?).clone(), (boolString(BackendVariable::varFixed(v.clone()))).clone(), (dumpFlowStr(ct.clone())?).clone(), (dumpStreamStr(ct.clone())?).clone(), (unparseCommentOptionNoAnnotation(comment.clone())?).clone())?;
            dumpBindExpression(e.clone(), addMMLCode.clone())?;
            paths = ElementSource::getElementSourceTypes(source.clone());
            dumpAbsynPathLst(paths.clone(), (stringAppend((arcstr::literal!(CLASSES)).clone(), (arcstr::literal!(NAMES_)).clone())).clone())?;
            dumpDAEVariableAttributes(dae_var_attr.clone(), (arcstr::literal!(VAR_ATTRIBUTES_VALUES)).clone(), addMMLCode.clone())?;
            dumpStrCloseTag((arcstr::literal!(VARIABLE)).clone())?;
            var_1 = varno.clone() + 1;
            dumpVars2(xs.clone(), var_1.clone(), addMMLCode.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpVarsAdds2(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut crefIdxLstArr: metamodelica::Array<Arc<metamodelica::List<BackendDAE::CrefIndex>>>, mut inInteger: i32, mut addMMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inVarLst.clone(), crefIdxLstArr.clone(), inInteger.clone(), addMMLCode.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: v @ BackendDAE::Var { connectorType: ct, comment, values: dae_var_attr, source, bindExp: e, varType: var_type, varDirection: dir, varKind: kind, varName: cr, .. }, tail: xs }, _, varno, _) => {
            let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut var_1: i32 = 0;
            dumpVariable((intString(varno.clone())).clone(), (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone(), (dumpKind(kind.clone())?).clone(), (dumpDirectionStr(dir.clone())?).clone(), (dumpTypeStr(var_type.clone())?).clone(), (getIndex(kind.clone())).clone(), (getDerName(kind.clone())?).clone(), (boolString(BackendVariable::varFixed(v.clone()))).clone(), (dumpFlowStr(ct.clone())?).clone(), (dumpStreamStr(ct.clone())?).clone(), (DAEDumpTypes::dumpCommentAnnotationStr(comment.clone())?).clone())?;
            dumpBindExpression(e.clone(), addMMLCode.clone())?;
            paths = ElementSource::getElementSourceTypes(source.clone());
            dumpAbsynPathLst(paths.clone(), (stringAppend((arcstr::literal!(CLASSES)).clone(), (arcstr::literal!(NAMES_)).clone())).clone())?;
            dumpDAEVariableAttributes(dae_var_attr.clone(), (arcstr::literal!(VAR_ATTRIBUTES_VALUES)).clone(), addMMLCode.clone())?;
            dumpVarsAdditionalInfo(crefIdxLstArr.clone(), varno.clone())?;
            dumpStrCloseTag((arcstr::literal!(VARIABLE)).clone())?;
            var_1 = varno.clone() + 1;
            dumpVarsAdds2(xs.clone(), crefIdxLstArr.clone(), var_1.clone(), addMMLCode.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: xs }, _, varno, _) => {
            let mut var_1: i32 = 0;
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.dumpVarsAdds2 - Unknown var: ")).clone();
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*intString(varno.clone())); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            var_1 = varno.clone() + 1;
            dumpVarsAdds2(xs.clone(), crefIdxLstArr.clone(), var_1.clone(), addMMLCode.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpWhenOperators(mut inWhenOperators: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut inContent: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inWhenOperators.clone(), inContent.clone(), addMathMLCode.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (lst, _, _) => {
            let mut len: i32 = 0;
            len = (lst.clone().len() as i32);
            dumpStrOpenTagAttr((inContent.clone()).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(len.clone())).clone())?;
            dumpWhenOperatorLst(lst.clone(), addMathMLCode.clone())?;
            dumpStrCloseTag((inContent.clone()).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpWhenOperatorLst(mut inWhenOperators: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inWhenOperators.clone(), addMathMLCode.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left, right: value, source: _ }, tail: lst }, true) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(left.clone())?).clone();
            s2 = (printExpStr(value.clone())?).clone();
            r#str = stringAppendList(list![(s1.clone()).clone(), (literal!(" := ")).clone(), (s2.clone()).clone()]);
            dumpStrOpenTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpStrMathMLVariable((s1.clone()).clone())?;
            dumpExp2(value.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpWhenOperatorLst(lst.clone(), addMathMLCode.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left, right: value, source: _ }, tail: lst }, false) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(left.clone())?).clone();
            s2 = (printExpStr(value.clone())?).clone();
            r#str = stringAppendList(list![(s1.clone()).clone(), (literal!(" := ")).clone(), (s2.clone()).clone()]);
            dumpStrOpenTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            dumpWhenOperatorLst(lst.clone(), addMathMLCode.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { stateVar, value, source: _ }, tail: lst }, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut call: Arc<DAE::Exp>;
            let mut r#str: ArcStr = arcstr::literal!("");
            e = Expression::makeCrefExp(stateVar.clone(), DAE::T_UNKNOWN_DEFAULT.clone())?;
            call = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (arcstr::literal!(REINIT)).clone() }), expLst: list![e.clone(), value.clone()], attr: DAE::callAttrBuiltinOther.clone() });
            r#str = (printExpStr(call.clone())?).clone();
            dumpStrOpenTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            dumpExp(call.clone(), addMathMLCode.clone())?;
            dumpStrCloseTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            dumpWhenOperatorLst(lst.clone(), addMathMLCode.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSERT { condition: cond, message: msg, level, source: _ }, tail: lst }, _) => {
            let mut call: Arc<DAE::Exp>;
            let mut r#str: ArcStr = arcstr::literal!("");
            call = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (arcstr::literal!(ASSERT)).clone() }), expLst: list![cond.clone(), msg.clone(), level.clone()], attr: DAE::callAttrBuiltinOther.clone() });
            r#str = (printExpStr(call.clone())?).clone();
            dumpStrOpenTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            dumpExp(call.clone(), addMathMLCode.clone())?;
            dumpStrCloseTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            dumpWhenOperatorLst(lst.clone(), addMathMLCode.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::TERMINATE { message: msg, source: _ }, tail: lst }, _) => {
            let mut call: Arc<DAE::Exp>;
            let mut r#str: ArcStr = arcstr::literal!("");
            call = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (arcstr::literal!(TERMINATE)).clone() }), expLst: list![msg.clone()], attr: DAE::callAttrBuiltinOther.clone() });
            r#str = (printExpStr(call.clone())?).clone();
            dumpStrOpenTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            dumpExp(call.clone(), addMathMLCode.clone())?;
            dumpStrCloseTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            dumpWhenOperatorLst(lst.clone(), addMathMLCode.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: call, .. }, tail: lst }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (printExpStr(call.clone())?).clone();
            dumpStrOpenTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            dumpExp(call.clone(), addMathMLCode.clone())?;
            dumpStrCloseTag((arcstr::literal!(WHEN_OPERATOR)).clone())?;
            dumpWhenOperatorLst(lst.clone(), addMathMLCode.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpTimeEvents(mut inTimeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>>, mut inContent: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inTimeEvents.clone(), inContent.clone(), addMathMLCode.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (_, _, _) => {
            let mut len: i32 = 0;
            len = (inTimeEvents.clone().len() as i32);
            dumpStrOpenTagAttr((inContent.clone()).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(len.clone())).clone())?;
            dumpSampleLst(inTimeEvents.clone(), addMathMLCode.clone())?;
            dumpStrCloseTag((inContent.clone()).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpSampleLst(mut inSamples: Arc<metamodelica::List<BackendDAE::TimeEvent>>, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inSamples.clone(), addMathMLCode.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::TimeEvent::SIMPLE_TIME_EVENT, tail: lst }, addMMLCode) => {
            dumpSampleLst(lst.clone(), addMMLCode.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { index: i, startExp: e1, intervalExp: e2 }, tail: lst }, addMMLCode) => {
            dumpStrOpenTag((stringAppend((arcstr::literal!(SAMPLES)).clone(), (arcstr::literal!(ELEMENT_)).clone())).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(INDEX)).clone(), (arcstr::literal!(VALUE)).clone(), (intString(i.clone())).clone())?;
            dumpExp(e1.clone(), addMMLCode.clone())?;
            dumpStrCloseTag((arcstr::literal!(INDEX)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(START)).clone(), (arcstr::literal!(EXP_STRING)).clone(), (printExpStr(e1.clone())?).clone())?;
            dumpExp(e1.clone(), addMMLCode.clone())?;
            dumpStrCloseTag((arcstr::literal!(START)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(INTERVAL)).clone(), (arcstr::literal!(EXP_STRING)).clone(), (printExpStr(e2.clone())?).clone())?;
            dumpExp(e2.clone(), addMMLCode.clone())?;
            dumpStrCloseTag((arcstr::literal!(INTERVAL)).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(SAMPLES)).clone(), (arcstr::literal!(ELEMENT_)).clone())).clone())?;
            dumpSampleLst(lst.clone(), addMMLCode.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpZeroCrossing(mut zeroCross: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, mut inContent: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((zeroCross.clone(), inContent.clone(), addMathMLCode.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (_, _, _) => {
            let mut len: i32 = 0;
            len = (zeroCross.clone().len() as i32);
            dumpStrOpenTagAttr((inContent.clone()).clone(), (arcstr::literal!(DIMENSION)).clone(), (intString(len.clone())).clone())?;
            dumpZcLst(zeroCross.clone(), addMathMLCode.clone())?;
            dumpStrCloseTag((inContent.clone()).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpZcLst(mut inZeroCrossingLst: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inZeroCrossingLst.clone(), addMathMLCode.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: BackendDAE::ZeroCrossing { occurEquLst: eq, relation_: e, .. }, tail: zcLst }, addMMLCode) => {
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(ZERO_CROSSING)).clone(), (arcstr::literal!(ELEMENT_)).clone())).clone(), (arcstr::literal!(EXP_STRING)).clone(), (printExpStr(e.clone())?).clone())?;
            dumpExp(e.clone(), addMMLCode.clone())?;
            dumpLstIntAttr(eq.clone(), (stringAppend((arcstr::literal!(INVOLVED)).clone(), (arcstr::literal!(EQUATIONS_)).clone())).clone(), (stringAppend((arcstr::literal!(EQUATION)).clone(), (arcstr::literal!(ID_)).clone())).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(ZERO_CROSSING)).clone(), (arcstr::literal!(ELEMENT_)).clone())).clone())?;
            dumpZcLst(zcLst.clone(), addMMLCode.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn lbinopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::AND { .. } => {
            arcstr::literal!(MathMLAnd)
        },
        DAE::Operator::OR { .. } => {
            arcstr::literal!(MathMLOr)
        },
        _ => {
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.lbinopSymbol - Unknown operator")).clone();
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*ExpressionDump::debugBinopSymbol(inOperator.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn lunaryopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::NOT { .. } => {
            arcstr::literal!(MathMLNot)
        },
        _ => {
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.lunaryopSymbol - Unknown operator")).clone();
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*ExpressionDump::debugBinopSymbol(inOperator.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn relopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::LESS { .. } => {
            arcstr::literal!(MathMLLessThan)
        },
        DAE::Operator::LESSEQ { .. } => {
            arcstr::literal!(MathMLLessEqualThan)
        },
        DAE::Operator::GREATER { .. } => {
            arcstr::literal!(MathMLGreaterThan)
        },
        DAE::Operator::GREATEREQ { .. } => {
            arcstr::literal!(MathMLGreaterEqualThan)
        },
        DAE::Operator::EQUAL { .. } => {
            arcstr::literal!(MathMLEquivalent)
        },
        DAE::Operator::NEQUAL { .. } => {
            arcstr::literal!(MathMLNotEqual)
        },
        _ => {
            let mut error_msg: ArcStr = arcstr::literal!("");
            error_msg = (literal!("in XMLDump.relopSymbol - Unknown operator")).clone();
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*ExpressionDump::debugBinopSymbol(inOperator.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn dumpResidual(mut inEquation: Arc<BackendDAE::Equation>, mut inIndexNumber: ArcStr, mut addMathMLCode: bool) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inEquation.clone(), inIndexNumber.clone(), addMathMLCode.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, indexS, true) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" ( ")).clone(), (s2.clone()).clone(), (literal!(") = 0")).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(EQUATION)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLMinus)).clone())?;
            dumpExp2(e1.clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpExp2(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((arcstr::literal!(EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" - ( ")).clone(), (s2.clone()).clone(), (literal!(" ) = 0")).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(EQUATION)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }, _, true) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            s = stringAppendList(list![(s1.clone()).clone(), (literal!(" - (")).clone(), (s2.clone()).clone(), (literal!(") = 0\n")).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(ARRAY_EQUATION)).clone(), (arcstr::literal!(EXP_STRING)).clone(), (s.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLMinus)).clone())?;
            dumpExp2(e1.clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpExp2(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((arcstr::literal!(ARRAY_EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" - ( ")).clone(), (s2.clone()).clone(), (literal!(" ) = 0")).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(ARRAY_OF_EQUATIONS)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(ARRAY_OF_EQUATIONS)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. }, _, true) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            s = stringAppendList(list![(s1.clone()).clone(), (literal!(" - (")).clone(), (s2.clone()).clone(), (literal!(") = 0\n")).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(COMPLEX_EQUATION)).clone(), (arcstr::literal!(EXP_STRING)).clone(), (s.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLMinus)).clone())?;
            dumpExp2(e1.clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpExp2(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((arcstr::literal!(COMPLEX_EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e1.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" - ( ")).clone(), (s2.clone()).clone(), (literal!(" ) = 0")).clone()]);
            dumpStrOpenTagAttr((arcstr::literal!(COMPLEX_EQUATION)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((arcstr::literal!(COMPLEX_EQUATION)).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. }, indexS, true) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" - ( ")).clone(), (s2.clone()).clone(), (literal!(" ) := 0")).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(SOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLMinus)).clone())?;
            Print::printBuf((s1.clone()).clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpExp2(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(SOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e2, componentRef: cr, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" - (")).clone(), (s2.clone()).clone(), (literal!(") := 0")).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(SOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(SOLVED)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e2, left: e, .. }, tail: Deref @ metamodelica::List::Nil }, condition: e1, .. }, .. }, indexS, true) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            let mut is: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            is = (printExpStr(e1.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" - (")).clone(), (s2.clone()).clone(), (literal!(") := 0")).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLMinus)).clone())?;
            Print::printBuf((s1.clone()).clone())?;
            dumpExp2(e2.clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpExp2(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTag((stringAppend((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(CONDITION)).clone())).clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            Print::printBuf((is.clone()).clone())?;
            dumpExp(e1.clone(), true)?;
            dumpStrCloseTag((stringAppend((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(CONDITION)).clone())).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e2, left: e, .. }, tail: Deref @ metamodelica::List::Nil }, condition: e1, .. }, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            let mut is: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e.clone())?).clone();
            s2 = (printExpStr(e2.clone())?).clone();
            is = (printExpStr(e1.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" - (")).clone(), (s2.clone()).clone(), (literal!(") := 0")).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrTagContent((stringAppend((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(CONDITION)).clone())).clone(), (is.clone()).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(WHEN)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, indexS, true) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = 0")).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(RESIDUAL)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathML)).clone())?;
            dumpStrOpenTagAttr((arcstr::literal!(MATH)).clone(), (arcstr::literal!(MathMLXmlns)).clone(), (arcstr::literal!(MathMLWeb)).clone())?;
            dumpStrOpenTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrVoidTag((arcstr::literal!(MathMLEquivalent)).clone())?;
            dumpExp2(e.clone())?;
            dumpStrMathMLNumber((literal!("0")).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathMLApply)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MATH)).clone())?;
            dumpStrCloseTag((arcstr::literal!(MathML)).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(RESIDUAL)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }, indexS, false) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s1 = (printExpStr(e.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = 0")).clone()]);
            dumpStrOpenTagAttr((stringAppend((arcstr::literal!(RESIDUAL)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((res.clone()).clone())?;
            dumpStrCloseTag((stringAppend((arcstr::literal!(RESIDUAL)).clone(), (arcstr::literal!(EQUATION_)).clone())).clone())?;
            ()
        },
        (Deref @ BackendDAE::Equation::ALGORITHM { source, alg: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, indexS, _) => {
            dumpStrOpenTagAttr((arcstr::literal!(ALGORITHM)).clone(), (arcstr::literal!(ID)).clone(), (indexS.clone()).clone())?;
            Print::printBuf((Util::xmlEscape((DAEDump::dumpAlgorithmsStr(list![Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts.clone() }), source: source.clone() })])?).clone())?).clone())?;
            dumpStrCloseTag((arcstr::literal!(ALGORITHM)).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn unaryopSymbol(mut inOperator: DAE::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        DAE::Operator::UMINUS { .. } => arcstr::literal!(MathMLMinus),
        DAE::Operator::UMINUS_ARR { .. } => arcstr::literal!(MathMLMinus),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn unparseCommentOptionNoAnnotation(mut inAbsynCommentOption: Option<Arc<SCode::Comment>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inAbsynCommentOption.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ SCode::Comment { annotation_: _, comment: Some(cmt) }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (cmt.clone()).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

