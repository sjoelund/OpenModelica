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

use crate::ClassInf;
use crate::SCode;
use crate::Values;
use openmodelica_ast::Absyn;

// public imports
pub type Ident = ArcStr;

pub type InstDims = Arc<metamodelica::List<Arc<Dimension>>>;

pub type StartValue = Option<Arc<Exp>>;

pub const UNIQUEIO: &'static str = "$unique$outer$";

pub const derivativeNamePrefix: &'static str = "$DER";

pub const partialDerivativeNamePrefix: &'static str = "$pDER";

pub const preNamePrefix: &'static str = "$PRE";

pub const previousNamePrefix: &'static str = "$CLKPRE";

pub const startNamePrefix: &'static str = "$START";

pub const auxNamePrefix: &'static str = "$AUX";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarKind {
    /// variable
    VARIABLE,
    /// discrete
    DISCRETE,
    /// parameter
    PARAM,
    /// constant
    CONST,
}
pub use self::VarKind::{VARIABLE,DISCRETE,PARAM,CONST};

/// The type of a connector element.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectorType {
    POTENTIAL,
    FLOW,
    STREAM {
        associatedFlow: Option<Arc<ComponentRef>>,
    },
    NON_CONNECTOR,
}
pub use self::ConnectorType::{POTENTIAL,FLOW,STREAM,NON_CONNECTOR};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarDirection {
    /// input
    INPUT,
    /// output
    OUTPUT,
    /// neither input or output
    BIDIR,
}
pub use self::VarDirection::{INPUT,OUTPUT,BIDIR};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarParallelism {
    /// Global variables for CUDA and OpenCL
    PARGLOBAL,
    /// Shared for CUDA and local for OpenCL
    PARLOCAL,
    /// Non parallel/Normal variables
    NON_PARALLEL,
}
pub use self::VarParallelism::{PARGLOBAL,PARLOCAL,NON_PARALLEL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarVisibility {
    /// public variables
    PUBLIC,
    /// protected variables
    PROTECTED,
}
pub use self::VarVisibility::{PUBLIC,PROTECTED};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarInnerOuter {
    /// an inner prefix
    INNER,
    /// an outer prefix
    OUTER,
    /// an inner outer prefix
    INNER_OUTER,
    /// no inner outer prefix
    NOT_INNER_OUTER,
}
pub use self::VarInnerOuter::{INNER,OUTER,INNER_OUTER,NOT_INNER_OUTER};

/// gives information about the origin of the element
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementSource {
    /// the line and column numbers of the equations and algorithms this element came from
    pub info: SourceInfo,
    /// the model(s) this element came from
    pub partOfLst: Arc<metamodelica::List<Absyn::Within>>,
    /// the instance(s) this element is part of
    pub instance: Arc<ComponentPrefix>,
    /// this element came from this connect(s)
    pub connectEquationOptLst: Arc<metamodelica::List<(Arc<ComponentRef>, Arc<ComponentRef>)>>,
    /// the classes where the type(s) of the element is defined
    pub typeLst: Arc<metamodelica::List<Arc<Absyn::Path>>>,
    /// the symbolic operations used to end up with the final state of the element
    pub operations: Arc<metamodelica::List<Arc<SymbolicOperation>>>,
    pub comment: Arc<metamodelica::List<Arc<SCode::Comment>>>,
}

pub type SOURCE = ElementSource;


pub static emptyElementSource: std::sync::LazyLock<Arc<ElementSource>> = std::sync::LazyLock::new(|| { Arc::new(ElementSource { info: Absyn::dummyInfo.clone(), partOfLst: metamodelica::nil(), instance: Arc::new(crate::DAE::ComponentPrefix::NOCOMPPRE), connectEquationOptLst: metamodelica::nil(), typeLst: metamodelica::nil(), operations: metamodelica::nil(), comment: metamodelica::nil() }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SymbolicOperation {
    /// From one equation/statement to an element
    FLATTEN {
        scode: Arc<SCode::Equation>,
        dae: Option<Arc<Element>>,
    },
    /// Before and after expression is equivalent
    SIMPLIFY {
        before: Arc<EquationExp>,
        after: Arc<EquationExp>,
    },
    /// A chain of substitutions
    SUBSTITUTION {
        substitutions: Arc<metamodelica::List<Arc<Exp>>>,
        source: Arc<Exp>,
    },
    /// Before and after inlining of function calls
    OP_INLINE {
        before: Arc<EquationExp>,
        after: Arc<EquationExp>,
    },
    /// Convert array equation into scalar equations; x = {1,2}, [1] => x[1] = {1}
    OP_SCALARIZE {
        before: Arc<EquationExp>,
        index: i32,
        after: Arc<EquationExp>,
    },
    /// Differentiate w.r.t. cr
    OP_DIFFERENTIATE {
        cr: Arc<ComponentRef>,
        before: Arc<Exp>,
        after: Arc<Exp>,
    },
    /// Solve equation, exp1 = exp2 => cr = exp; note that assertions may have been generated for example in case of divisions
    SOLVE {
        cr: Arc<ComponentRef>,
        exp1: Arc<Exp>,
        exp2: Arc<Exp>,
        res: Arc<Exp>,
        assertConds: Arc<metamodelica::List<Arc<Exp>>>,
    },
    /// Equation is solved
    SOLVED {
        cr: Arc<ComponentRef>,
        exp: Arc<Exp>,
    },
    /// Solved linear system of equations
    LINEAR_SOLVED {
        vars: Arc<metamodelica::List<Arc<ComponentRef>>>,
        jac: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>,
        rhs: Arc<metamodelica::List<metamodelica::Real>>,
        result: Arc<metamodelica::List<metamodelica::Real>>,
    },
    /// Introduced a dummy derivative (from index reduction)
    NEW_DUMMY_DER {
        chosen: Arc<ComponentRef>,
        candidates: Arc<metamodelica::List<Arc<ComponentRef>>>,
    },
    /// Converted the equation into residual form, to use nonlinear equation solvers 0=e (0=e1-e2)
    OP_RESIDUAL {
        e1: Arc<Exp>,
        e2: Arc<Exp>,
        e: Arc<Exp>,
    },
}
impl Default for SymbolicOperation {
    fn default() -> Self {
        Self::LINEAR_SOLVED {
            vars: Default::default(),
            jac: Default::default(),
            rhs: Default::default(),
            result: Default::default(),
        }
    }
}
pub use self::SymbolicOperation::{FLATTEN,SIMPLIFY,SUBSTITUTION,OP_INLINE,OP_SCALARIZE,OP_DIFFERENTIATE,SOLVE,SOLVED,LINEAR_SOLVED,NEW_DUMMY_DER,OP_RESIDUAL};

/// An equation on residual or equality form has 1 or 2 expressions. For use with symbolic operation tracing.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquationExp {
    /// An expression that is part of the whole equation
    PARTIAL_EQUATION {
        exp: Arc<Exp>,
    },
    /// 0 = exp
    RESIDUAL_EXP {
        exp: Arc<Exp>,
    },
    /// lhs = rhs
    EQUALITY_EXPS {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
    },
}
pub use self::EquationExp::{PARTIAL_EQUATION,RESIDUAL_EXP,EQUALITY_EXPS};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    VAR {
        /// The variable name
        componentRef: Arc<ComponentRef>,
        /// varible kind: variable, constant, parameter, discrete etc.
        kind: VarKind,
        /// input, output or bidir
        direction: VarDirection,
        /// parglobal, parlocal, or non_parallel
        parallelism: VarParallelism,
        /// if protected or public
        protection: VarVisibility,
        /// Full type information required
        ty: Arc<Type>,
        /// Binding expression e.g. for parameters ; value of start attribute
        binding: Option<Arc<Exp>>,
        /// dimensions
        dims: InstDims,
        /// The connector type: flow, stream, no prefix, or not a connector element.
        connectorType: Arc<ConnectorType>,
        /// the origins of the component/equation/algorithm
        source: Arc<ElementSource>,
        variableAttributesOption: Option<Arc<VariableAttributes>>,
        comment: Option<Arc<SCode::Comment>>,
        /// inner/outer required to 'change' outer references
        innerOuter: Absyn::InnerOuter,
        /// true if the variable belongs to an encrypted class
        encrypted: bool,
    },
    /// A solved equation
    DEFINE {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// A solved initial equation
    INITIALDEFINE {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// Scalar equation
    EQUATION {
        exp: Arc<Exp>,
        scalar: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// effort variable equality
    EQUEQUATION {
        cr1: Arc<ComponentRef>,
        cr2: Arc<ComponentRef>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// an array equation
    ARRAY_EQUATION {
        /// dimension sizes
        dimension: Dimensions,
        exp: Arc<Exp>,
        array: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// An initial array equation
    INITIAL_ARRAY_EQUATION {
        /// dimension sizes
        dimension: Dimensions,
        exp: Arc<Exp>,
        array: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// a connect equation
    CONNECT_EQUATION {
        lhsElement: Arc<Element>,
        lhsFace: Connect::Face,
        rhsElement: Arc<Element>,
        rhsFace: Connect::Face,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// an equation of complex type, e.g. record = func(..)
    COMPLEX_EQUATION {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// an initial equation of complex type, e.g. record = func(..)
    INITIAL_COMPLEX_EQUATION {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// a when equation
    WHEN_EQUATION {
        /// Condition
        condition: Arc<Exp>,
        /// Equations
        equations: Arc<metamodelica::List<Arc<Element>>>,
        /// Elsewhen should be of type WHEN_EQUATION
        elsewhen_: Option<Arc<Element>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// an initial for-equation
    INITIAL_FOR_EQUATION {
        /// this is the type of the iterator
        type_: Arc<Type>,
        /// True if the iterator has an array type, otherwise false.
        iterIsArray: bool,
        /// the iterator variable
        iter: Ident,
        /// the index of the iterator variable, to make it unique; used by the new inst
        index: i32,
        /// range for the loop
        range: Arc<Exp>,
        /// Equations
        equations: Arc<metamodelica::List<Arc<Element>>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// a for-equation
    FOR_EQUATION {
        /// this is the type of the iterator
        type_: Arc<Type>,
        /// True if the iterator has an array type, otherwise false.
        iterIsArray: bool,
        /// the iterator variable
        iter: Ident,
        /// the index of the iterator variable, to make it unique; used by the new inst
        index: i32,
        /// range for the loop
        range: Arc<Exp>,
        /// Equations
        equations: Arc<metamodelica::List<Arc<Element>>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// an if-equation
    IF_EQUATION {
        /// Condition
        condition1: Arc<metamodelica::List<Arc<Exp>>>,
        /// Equations of true branch
        equations2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Element>>>>>,
        /// Equations of false branch
        equations3: Arc<metamodelica::List<Arc<Element>>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// An initial if-equation
    INITIAL_IF_EQUATION {
        /// Condition
        condition1: Arc<metamodelica::List<Arc<Exp>>>,
        /// Equations of true branch
        equations2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Element>>>>>,
        /// Equations of false branch
        equations3: Arc<metamodelica::List<Arc<Element>>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// An initial equaton
    INITIALEQUATION {
        exp1: Arc<Exp>,
        exp2: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// An algorithm section
    ALGORITHM {
        algorithm_: Arc<Algorithm>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// An initial algorithm section
    INITIALALGORITHM {
        algorithm_: Arc<Algorithm>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    COMP {
        ident: Ident,
        /// a component with subelements, normally only used at top level.
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
        comment: Option<Arc<SCode::Comment>>,
    },
    /// The 'class' of an external object
    EXTOBJECTCLASS {
        /// className of external object
        path: Arc<Absyn::Path>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// The Modelica builtin assert
    ASSERT {
        condition: Arc<Exp>,
        message: Arc<Exp>,
        level: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// The Modelica builtin assert
    INITIAL_ASSERT {
        condition: Arc<Exp>,
        message: Arc<Exp>,
        level: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// The Modelica builtin terminate(msg)
    TERMINATE {
        message: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// The Modelica builtin terminate(msg)
    INITIAL_TERMINATE {
        message: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// reinit operator for reinitialization of states
    REINIT {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// call with no return value, i.e. no equation.
    ///    Typically sideeffect call of external function but also
    ///    Connections.* i.e. Connections.root(...) functions.
    NORETCALL {
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// call with no return value, i.e. no equation.
    ///    Typically sideeffect call of external function but also
    ///    Connections.* i.e. Connections.root(...) functions.
    INITIAL_NORETCALL {
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// constraint section
    CONSTRAINT {
        constraints: Arc<Constraint>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    CLASS_ATTRIBUTES {
        classAttrs: Arc<ClassAttributes>,
    },
    /// Flat state machine section
    FLAT_SM {
        ident: Ident,
        /// The states/modes transitions and variable
        ///                      merging equations within the the flat state machine
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
    },
    /// A state/mode component in a state machine
    SM_COMP {
        componentRef: Arc<ComponentRef>,
        /// a component with subelements
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
    },
    COMMENT {
        /// Functions store the inherited class annotations in the DAE
        cmt: Arc<SCode::Comment>,
    },
}
pub use self::Element::{VAR,DEFINE,INITIALDEFINE,EQUATION,EQUEQUATION,ARRAY_EQUATION,INITIAL_ARRAY_EQUATION,CONNECT_EQUATION,COMPLEX_EQUATION,INITIAL_COMPLEX_EQUATION,WHEN_EQUATION,INITIAL_FOR_EQUATION,FOR_EQUATION,IF_EQUATION,INITIAL_IF_EQUATION,INITIALEQUATION,ALGORITHM,INITIALALGORITHM,COMP,EXTOBJECTCLASS,ASSERT,INITIAL_ASSERT,TERMINATE,INITIAL_TERMINATE,REINIT,NORETCALL,INITIAL_NORETCALL,CONSTRAINT,CLASS_ATTRIBUTES,FLAT_SM,SM_COMP,COMMENT};

pub static T_ASSERTIONLEVEL: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("AssertionLevel")).clone() }) }), names: list![(literal!("warning")).clone(), (literal!("error")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }) });

pub static ASSERTIONLEVEL_WARNING: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::ENUM_LITERAL { name: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("AssertionLevel")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("warning")).clone() }) }), index: 1 }) });

pub static ASSERTIONLEVEL_ERROR: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::ENUM_LITERAL { name: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("AssertionLevel")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("error")).clone() }) }), index: 2 }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Function {
    /// A Modelica function
    FUNCTION {
        path: Arc<Absyn::Path>,
        /// contains the body and an optional function derivative mapping
        functions: Arc<metamodelica::List<FunctionDefinition>>,
        type_: Arc<Type>,
        visibility: SCode::Visibility,
        /// MetaModelica extension
        partialPrefix: bool,
        /// Modelica 3.3 impure/pure, by default isImpure = false all the time only if prefix *impure* function is specified
        isImpure: bool,
        inlineType: InlineType,
        /// The indices of any inputs not used in the function.
        unusedInputs: Arc<metamodelica::List<i32>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
        comment: Option<Arc<SCode::Comment>>,
    },
    /// A Modelica record constructor. The function can be generated from the Path and Type alone.
    RECORD_CONSTRUCTOR {
        path: Arc<Absyn::Path>,
        type_: Arc<Type>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
}
pub use self::Function::{FUNCTION,RECORD_CONSTRUCTOR};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlineType {
    /// Normal inline, inline as soon as possible
    NORM_INLINE,
    /// Inline even if inlining is globally disabled by flags.
    BUILTIN_EARLY_INLINE,
    /// Inline even earlier than NORM_INLINE. This will display the inlined code in the flattened model and also works for functions calling other functions that should be inlined.
    EARLY_INLINE,
    /// no user option, tool can inline this functio if necessary
    DEFAULT_INLINE,
    /// don't inline this function, set with Inline=false
    NO_INLINE,
    /// Try to inline after index reduction
    AFTER_INDEX_RED_INLINE,
}
impl Default for InlineType {
    fn default() -> Self { Self::NORM_INLINE }
}
pub use self::InlineType::{NORM_INLINE,BUILTIN_EARLY_INLINE,EARLY_INLINE,DEFAULT_INLINE,NO_INLINE,AFTER_INDEX_RED_INLINE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionDefinition {
    /// Normal function body
    FUNCTION_DEF {
        body: Arc<metamodelica::List<Arc<Element>>>,
    },
    /// Normal external function declaration
    FUNCTION_EXT {
        body: Arc<metamodelica::List<Arc<Element>>>,
        externalDecl: ExternalDecl,
    },
    /// Contains derivatives for function
    FUNCTION_DER_MAPPER {
        /// Function that is derived
        derivedFunction: Arc<Absyn::Path>,
        /// Path to derivative function
        derivativeFunction: Arc<Absyn::Path>,
        /// in case a function have multiple derivatives, include all
        derivativeOrder: i32,
        conditionRefs: Arc<metamodelica::List<(i32, derivativeCond)>>,
        /// if conditions fails, use default derivative if exists
        defaultDerivative: Option<Arc<Absyn::Path>>,
        lowerOrderDerivatives: Arc<metamodelica::List<Arc<Absyn::Path>>>,
    },
    /// A function inverse declaration
    FUNCTION_INVERSE {
        /// The input parameter the inverse is for
        inputParam: Arc<ComponentRef>,
        /// The inverse function call
        inverseCall: Arc<Exp>,
    },
    FUNCTION_PARTIAL_DERIVATIVE {
        derivedFunction: Arc<Absyn::Path>,
        derivedVars: Arc<metamodelica::List<ArcStr>>,
    },
}
pub use self::FunctionDefinition::{FUNCTION_DEF,FUNCTION_EXT,FUNCTION_DER_MAPPER,FUNCTION_INVERSE,FUNCTION_PARTIAL_DERIVATIVE};

/// Different conditions on derivatives
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum derivativeCond {
    ZERO_DERIVATIVE,
    NO_DERIVATIVE {
        binding: Arc<Exp>,
    },
}
pub use self::derivativeCond::{ZERO_DERIVATIVE,NO_DERIVATIVE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VariableAttributes {
    VAR_ATTR_REAL {
        /// quantity
        quantity: Option<Arc<Exp>>,
        /// unit
        unit: Option<Arc<Exp>>,
        /// displayUnit
        displayUnit: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        /// start value
        start: Option<Arc<Exp>>,
        /// fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<Arc<Exp>>,
        /// nominal
        nominal: Option<Arc<Exp>>,
        stateSelectOption: Option<StateSelect>,
        uncertainOption: Option<Uncertainty>,
        distributionOption: Option<Arc<Distribution>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        /// where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_INT {
        /// quantity
        quantity: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        /// start value
        start: Option<Arc<Exp>>,
        /// fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<Arc<Exp>>,
        uncertainOption: Option<Uncertainty>,
        distributionOption: Option<Arc<Distribution>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        /// where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_BOOL {
        /// quantity
        quantity: Option<Arc<Exp>>,
        /// start value
        start: Option<Arc<Exp>>,
        /// fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        /// where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_CLOCK {
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
    },
    VAR_ATTR_STRING {
        /// quantity
        quantity: Option<Arc<Exp>>,
        /// start value
        start: Option<Arc<Exp>>,
        /// new in Modelica 3.4; fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        /// where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_ENUMERATION {
        /// quantity
        quantity: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        /// start
        start: Option<Arc<Exp>>,
        /// fixed - true: default for parameter/constant, false - default for other variables
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        /// where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)
        startOrigin: Option<Arc<Exp>>,
    },
}
pub use self::VariableAttributes::{VAR_ATTR_REAL,VAR_ATTR_INT,VAR_ATTR_BOOL,VAR_ATTR_CLOCK,VAR_ATTR_STRING,VAR_ATTR_ENUMERATION};

pub static emptyVarAttrReal: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrInt: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_INT { quantity: None, min: None, max: None, start: None, fixed: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrBool: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_BOOL { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrClock: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_CLOCK { isProtected: None, finalPrefix: None }) });

pub static emptyVarAttrString: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_STRING { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrEnum: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_ENUMERATION { quantity: None, min: None, max: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StateSelect {
    NEVER,
    AVOID,
    DEFAULT,
    PREFER,
    ALWAYS,
}
pub use self::StateSelect::{NEVER,AVOID,DEFAULT,PREFER,ALWAYS};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Uncertainty {
    GIVEN,
    SOUGHT,
    REFINE,
    PROPAGATE,
}
pub use self::Uncertainty::{GIVEN,SOUGHT,REFINE,PROPAGATE};

/// see Distribution record in Distribution
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distribution {
    pub name: Arc<Exp>,
    pub params: Arc<Exp>,
    pub paramNames: Arc<Exp>,
}

pub type DISTRIBUTION = Distribution;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExtArg {
    EXTARG {
        componentRef: Arc<ComponentRef>,
        direction: Absyn::Direction,
        type_: Arc<Type>,
    },
    EXTARGEXP {
        exp: Arc<Exp>,
        type_: Arc<Type>,
    },
    EXTARGSIZE {
        componentRef: Arc<ComponentRef>,
        type_: Arc<Type>,
        exp: Arc<Exp>,
    },
    NOEXTARG,
}
pub use self::ExtArg::{EXTARG,EXTARGEXP,EXTARGSIZE,NOEXTARG};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalDecl {
    pub name: ArcStr,
    pub args: Arc<metamodelica::List<ExtArg>>,
    pub returnArg: ExtArg,
    pub language: ArcStr,
    pub ann: Option<Arc<SCode::Annotation>>,
}

pub type EXTERNALDECL = ExternalDecl;


/// A DAElist is a list of Elements. Variables, equations, functions,
///  algorithms, etc. are all found in this list.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DAElist {
    pub elementLst: Arc<metamodelica::List<Arc<Element>>>,
}

pub type DAE = DAElist;


/* -- Algorithm.mo -- */
/// The `Algorithm\' type corresponds to a whole algorithm section.
///  It is simple a list of algorithm statements.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Algorithm {
    pub statementLst: Arc<metamodelica::List<Arc<Statement>>>,
}

pub type ALGORITHM_STMTS = Algorithm;


/// Optimica extension: The `Constraints\' type corresponds to a whole Constraint section.
///  It is simple a list of expressions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Constraint {
    CONSTRAINT_EXPS {
        constraintLst: Arc<metamodelica::List<Arc<Exp>>>,
    },
    /// Constraints needed for proper Dynamic Tearing
    CONSTRAINT_DT {
        constraint: Arc<Exp>,
        /// local or global constraint; local constraints depend on variables that are computed within the algebraic loop itself
        localCon: bool,
    },
}
pub use self::Constraint::{CONSTRAINT_EXPS,CONSTRAINT_DT};

/// currently for Optimica extension: these are the objectives of optimization class
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassAttributes {
    pub objetiveE: Option<Arc<Exp>>,
    pub objectiveIntegrandE: Option<Arc<Exp>>,
    pub startTimeE: Option<Arc<Exp>>,
    pub finalTimeE: Option<Arc<Exp>>,
}

pub type OPTIMIZATION_ATTRS = ClassAttributes;


/* TODO: create a backend and a simcode uniontype */
/// There are four kinds of statements:
///    1. assignments ('a := b;')
///    2. if statements ('if A then B; elseif C; else D;')
///    3. for loops ('for i in 1:10 loop ...; end for;')
///    4. when statements ('when E do S; end when;')
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Statement {
    STMT_ASSIGN {
        type_: Arc<Type>,
        exp1: Arc<Exp>,
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_TUPLE_ASSIGN {
        type_: Arc<Type>,
        expExpLst: Arc<metamodelica::List<Arc<Exp>>>,
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_ASSIGN_ARR {
        type_: Arc<Type>,
        lhs: Arc<Exp>,
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_IF {
        exp: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        else_: Arc<Else>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_FOR {
        /// this is the type of the iterator
        type_: Arc<Type>,
        /// True if the iterator has an array type, otherwise false.
        iterIsArray: bool,
        /// the iterator variable
        iter: Ident,
        /// range for the loop
        range: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_PARFOR {
        /// this is the type of the iterator
        type_: Arc<Type>,
        /// True if the iterator has an array type, otherwise false.
        iterIsArray: bool,
        /// the iterator variable
        iter: Ident,
        /// range for the loop
        range: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        /// list of parallel variables used/referenced in the parfor loop
        loopPrlVars: Arc<metamodelica::List<(Arc<ComponentRef>, SourceInfo)>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_WHILE {
        exp: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_WHEN {
        exp: Arc<Exp>,
        conditions: Arc<metamodelica::List<Arc<ComponentRef>>>,
        initialCall: bool,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        elseWhen: Option<Arc<Statement>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// assert(cond,msg)
    STMT_ASSERT {
        cond: Arc<Exp>,
        msg: Arc<Exp>,
        level: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// terminate(msg)
    STMT_TERMINATE {
        msg: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_REINIT {
        /// Variable
        var: Arc<Exp>,
        /// Value
        value: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// call with no return value, i.e. no equation.
    ///       Typically sideeffect call of external function.
    STMT_NORETCALL {
        exp: Arc<Exp>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_RETURN {
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_BREAK {
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_CONTINUE {
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    /// For function initialization
    STMT_ARRAY_INIT {
        name: ArcStr,
        ty: Arc<Type>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
    STMT_FAILURE {
        body: Arc<metamodelica::List<Arc<Statement>>>,
        /// the origin of the component/equation/algorithm
        source: Arc<ElementSource>,
    },
}
pub use self::Statement::{STMT_ASSIGN,STMT_TUPLE_ASSIGN,STMT_ASSIGN_ARR,STMT_IF,STMT_FOR,STMT_PARFOR,STMT_WHILE,STMT_WHEN,STMT_ASSERT,STMT_TERMINATE,STMT_REINIT,STMT_NORETCALL,STMT_RETURN,STMT_BREAK,STMT_CONTINUE,STMT_ARRAY_INIT,STMT_FAILURE};

/// An if statements can one or more `elseif\' branches and an
///    optional `else\' branch.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Else {
    NOELSE,
    ELSEIF {
        exp: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        else_: Arc<Else>,
    },
    ELSE {
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
    },
}
pub use self::Else::{NOELSE,ELSEIF,ELSE};

/* -- End Algorithm.mo -- */
/* -- Start Types.mo -- */
/// - Variables
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var {
    /// name
    pub name: Ident,
    /// attributes
    pub attributes: Arc<Attributes>,
    /// type
    pub ty: Arc<Type>,
    /// equation modification
    pub binding: Arc<Binding>,
    /// true if the binding has come from out of scope. This happens for derived record classes.
    ///                                   e.g. record A = B(k=exp). here the modification 'exp' is a binding from outside. We need
    ///                                   this infor to correctly generate default constructors at codegen time. This binding exp
    ///                                   will have to be supplied from outside for a default constructor of the owner record type
    pub bind_from_outside: bool,
    /// the constant-ness of the range if this is a for iterator, NONE() if is NOT a for iterator
    pub constOfForIteratorRange: Option<Const>,
}

pub type TYPES_VAR = Var;


/// - Attributes
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attributes {
    /// flow, stream or unspecified
    pub connectorType: Arc<ConnectorType>,
    /// parallelism
    pub parallelism: SCode::Parallelism,
    /// variability
    pub variability: SCode::Variability,
    /// direction
    pub direction: Absyn::Direction,
    /// inner, outer,  inner outer or unspecified
    pub innerOuter: Absyn::InnerOuter,
    /// public, protected
    pub visibility: SCode::Visibility,
}

pub type ATTR = Attributes;


pub static dummyAttrVar: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrParam: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrConst: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrInput: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

/// where this binding came from: either default binding or start value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BindingSource {
    /// the binding came from the default value
    BINDING_FROM_DEFAULT_VALUE,
    /// the binding came from the start value
    BINDING_FROM_START_VALUE,
    /// the EQ binding is created from the submods of a record VARIABLE declration e.g. 'R r(i=2)' tranformed by instantiation to 'R r = R(i=2)'
    BINDING_FROM_RECORD_SUBMODS,
    /// the binding is created from the submods of a DERIVED record DECLARATION e.g. 'record K = R(i=3)'
    BINDING_FROM_DERIVED_RECORD_DECL,
}
pub use self::BindingSource::{BINDING_FROM_DEFAULT_VALUE,BINDING_FROM_START_VALUE,BINDING_FROM_RECORD_SUBMODS,BINDING_FROM_DERIVED_RECORD_DECL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Binding {
    UNBOUND,
    EQBOUND {
        exp: Arc<Exp>,
        evaluatedExp: Option<Arc<Values::Value>>,
        constant_: Const,
        source: BindingSource,
    },
    VALBOUND {
        valBound: Arc<Values::Value>,
        source: BindingSource,
    },
}
pub use self::Binding::{UNBOUND,EQBOUND,VALBOUND};

/// contains the path to the equalityConstraint function,
///   the dimension of the output and the inline type of the function
pub type EqualityConstraint = Option<(Arc<Absyn::Path>, i32, InlineType)>;

// default constants that can be used
pub static T_REAL_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_REAL { varLst: metamodelica::nil() }) });

pub static T_INTEGER_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_INTEGER { varLst: metamodelica::nil() }) });

pub static T_STRING_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_STRING { varLst: metamodelica::nil() }) });

pub static T_BOOL_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_BOOL { varLst: metamodelica::nil() }) });

pub static T_CLOCK_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_CLOCK { varLst: metamodelica::nil() }) });

pub static T_ENUMERATION_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }) });

pub static T_REAL_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_REAL_DEFAULT.clone() }) });

pub static T_INTEGER_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_INTEGER_DEFAULT.clone() }) });

pub static T_STRING_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_STRING_DEFAULT.clone() }) });

pub static T_BOOL_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_BOOL_DEFAULT.clone() }) });

pub static T_METABOXED_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_METALIST_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METALIST { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_NONE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METAOPTION { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_ANYTYPE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ANYTYPE { anyClassType: None }) });

pub static T_UNKNOWN_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(crate::DAE::Type::T_UNKNOWN) });

pub static T_NORETCALL_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(crate::DAE::Type::T_NORETCALL) });

pub static T_METATYPE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METATYPE { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_COMPLEX_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }) });

pub static T_COMPLEX_DEFAULT_RECORD: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }) });

pub static T_SOURCEINFO_DEFAULT_METARECORD: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METARECORD { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("SourceInfo")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SOURCEINFO")).clone() }) }), utPath: Arc::new(Absyn::Path::IDENT { name: (literal!("SourceInfo")).clone() }), typeVars: metamodelica::nil(), index: 1, fields: list![Arc::new(Var { name: (literal!("fileName")).clone(), attributes: dummyAttrVar.clone(), ty: T_STRING_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("isReadOnly")).clone(), attributes: dummyAttrVar.clone(), ty: T_BOOL_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lineNumberStart")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("columnNumberStart")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lineNumberEnd")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("columnNumberEnd")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lastModification")).clone(), attributes: dummyAttrVar.clone(), ty: T_REAL_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], knownSingleton: true }) });

pub static T_SOURCEINFO_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METAUNIONTYPE { paths: list![Arc::new(Absyn::Path::QUALIFIED { name: (literal!("SourceInfo")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SOURCEINFO")).clone() }) })], typeVars: metamodelica::nil(), knownSingleton: true, singletonType: Arc::new(EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: T_SOURCEINFO_DEFAULT_METARECORD.clone() }), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SourceInfo")).clone() }) }) });

// Arrays of unknown dimension, eg. Real[:]
pub static T_ARRAY_REAL_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_REAL_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_INT_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_INTEGER_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_BOOL_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_BOOL_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_STRING_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_STRING_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

/// models the different front-end and back-end types
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    T_INTEGER {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_REAL {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_STRING {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_BOOL {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_CLOCK {
        varLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    /// If the list of names is empty, this is the super-enumeration that is the super-class of all enumerations
    T_ENUMERATION {
        /// the enumeration value index, SOME for element, NONE() for type
        index: Option<i32>,
        /// enumeration path
        path: Arc<Absyn::Path>,
        /// names
        names: Arc<metamodelica::List<ArcStr>>,
        literalVarLst: Arc<metamodelica::List<Arc<Var>>>,
        attributeLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    /// an array can be represented in two equivalent ways:
    ///       1. T_ARRAY(non_array_type, {dim1, dim2, dim3})
    ///       2. T_ARRAY(T_ARRAY(T_ARRAY(non_array_type, {dim1}), {dim2}), {dim3})
    ///       In general Inst generates 1 and all the others generates 2
    T_ARRAY {
        /// Type
        ty: Arc<Type>,
        /// dims
        dims: Dimensions,
    },
    /// For functions not returning any values.
    T_NORETCALL,
    /// Used when type is not yet determined
    T_UNKNOWN,
    T_COMPLEX {
        /// The type of a class
        complexClassType: ClassInf::State,
        /// The variables of a complex type
        varLst: Arc<metamodelica::List<Arc<Var>>>,
        equalityConstraint: EqualityConstraint,
        /// If the record is passed to an external function at any point, we need to generate conversion functions for it (for instance to convert 'modelica_integer' to 'int')
        usedExternally: bool,
    },
    T_SUBTYPE_BASIC {
        /// The type of a class
        complexClassType: ClassInf::State,
        /// complexVarLst; The variables of a complex type! Should be empty, kept here to verify!
        varLst: Arc<metamodelica::List<Arc<Var>>>,
        /// complexType; A complex type can be a subtype of another (primitive) type (through extends)
        complexType: Arc<Type>,
        equalityConstraint: EqualityConstraint,
    },
    T_FUNCTION {
        /// funcArg
        funcArg: Arc<metamodelica::List<Arc<FuncArg>>>,
        /// Only single-result
        funcResultType: Arc<Type>,
        functionAttributes: FunctionAttributes,
        path: Arc<Absyn::Path>,
    },
    /// MetaModelica Function Reference that is a variable
    T_FUNCTION_REFERENCE_VAR {
        /// the type of the function
        functionType: Arc<Type>,
    },
    /// MetaModelica Function Reference that is a direct reference to a function
    T_FUNCTION_REFERENCE_FUNC {
        builtin: bool,
        /// type of the non-boxptr function
        functionType: Arc<Type>,
    },
    T_TUPLE {
        /// For functions returning multiple values.
        types: Arc<metamodelica::List<Arc<Type>>>,
        /// For tuples elements that have names (function outputs)
        names: Option<Arc<metamodelica::List<ArcStr>>>,
    },
    T_CODE {
        ty: CodeType,
    },
    T_ANYTYPE {
        /// anyClassType - used for generic types. When class state present the type is assumed to be a complex type which has that restriction.
        anyClassType: Option<ClassInf::State>,
    },
    /// MetaModelica list type
    T_METALIST {
        /// listType
        ty: Arc<Type>,
    },
    /// MetaModelica tuple type
    T_METATUPLE {
        types: Arc<metamodelica::List<Arc<Type>>>,
    },
    /// MetaModelica option type
    T_METAOPTION {
        ty: Arc<Type>,
    },
    /// MetaModelica Uniontype, added by simbj
    T_METAUNIONTYPE {
        paths: Arc<metamodelica::List<Arc<Absyn::Path>>>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        /// The runtime system (dynload), does not know if the value is a singleton. But optimizations are safe if this is true.
        knownSingleton: bool,
        singletonType: Arc<EvaluateSingletonType>,
        path: Arc<Absyn::Path>,
    },
    /// MetaModelica Record, used by Uniontypes. added by simbj
    T_METARECORD {
        /// the path to the record
        path: Arc<Absyn::Path>,
        /// the path to its uniontype; this is what we match the type against
        utPath: Arc<Absyn::Path>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        index: i32,
        fields: Arc<metamodelica::List<Arc<Var>>>,
        /// The runtime system (dynload), does not know if the value is a singleton. But optimizations are safe if this is true.
        knownSingleton: bool,
    },
    T_METAARRAY {
        ty: Arc<Type>,
    },
    /// Used for MetaModelica generic types
    T_METABOXED {
        ty: Arc<Type>,
    },
    T_METAPOLYMORPHIC {
        name: ArcStr,
    },
    /// this type contains all the meta types
    T_METATYPE {
        ty: Arc<Type>,
    },
}
pub use self::Type::{T_INTEGER,T_REAL,T_STRING,T_BOOL,T_CLOCK,T_ENUMERATION,T_ARRAY,T_NORETCALL,T_UNKNOWN,T_COMPLEX,T_SUBTYPE_BASIC,T_FUNCTION,T_FUNCTION_REFERENCE_VAR,T_FUNCTION_REFERENCE_FUNC,T_TUPLE,T_CODE,T_ANYTYPE,T_METALIST,T_METATUPLE,T_METAOPTION,T_METAUNIONTYPE,T_METARECORD,T_METAARRAY,T_METABOXED,T_METAPOLYMORPHIC,T_METATYPE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodeType {
    C_EXPRESSION,
    C_EXPRESSION_OR_MODIFICATION,
    C_MODIFICATION,
    C_TYPENAME,
    C_VARIABLENAME,
    /// Array of VariableName
    C_VARIABLENAMES,
}
pub use self::CodeType::{C_EXPRESSION,C_EXPRESSION_OR_MODIFICATION,C_MODIFICATION,C_TYPENAME,C_VARIABLENAME,C_VARIABLENAMES};

/// Is here because constants are not allowed to contain function pointers for some reason
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvaluateSingletonType {
    EVAL_SINGLETON_TYPE_FUNCTION {
        fun: EvaluateSingletonTypeFunction,
    },
    EVAL_SINGLETON_KNOWN_TYPE {
        ty: Arc<Type>,
    },
    NOT_SINGLETON,
}
pub use self::EvaluateSingletonType::{EVAL_SINGLETON_TYPE_FUNCTION,EVAL_SINGLETON_KNOWN_TYPE,NOT_SINGLETON};

pub type EvaluateSingletonTypeFunction = fn() -> Result<Arc<Type>>;

pub static FUNCTION_ATTRIBUTES_BUILTIN: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::PURE.clone(), isFunctionPointer: false, isBuiltin: FunctionBuiltin::FUNCTION_BUILTIN { name: None, unboxArgs: false }, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_DEFAULT: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::DEFAULT_INLINE, generateEvents: false, purity: Purity::PURE.clone(), isFunctionPointer: false, isBuiltin: crate::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_IMPURE: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::IMPURE.clone(), isFunctionPointer: false, isBuiltin: crate::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_BUILTIN_IMPURE: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::IMPURE.clone(), isFunctionPointer: false, isBuiltin: FunctionBuiltin::FUNCTION_BUILTIN { name: None, unboxArgs: false }, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Purity {
    PURE = 1,
    IMPURE = 2,
    UNDEFINED = 3,
    OM_IMPURE = 4,
}
impl PartialOrd for Purity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Purity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

// Function with pure prefix
// Function with impure prefix
// Function with neither pure nor impure prefix
// Function with __OpenModelica_Impure=true annotation (only used by the OF)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionAttributes {
    pub inline: InlineType,
    pub generateEvents: bool,
    pub purity: Purity,
    /// if the function is a local variable
    pub isFunctionPointer: bool,
    pub isBuiltin: FunctionBuiltin,
    pub functionParallelism: FunctionParallelism,
}

pub type FUNCTION_ATTRIBUTES = FunctionAttributes;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionBuiltin {
    /// Function is not builtin
    FUNCTION_NOT_BUILTIN,
    /// Function is builtin
    FUNCTION_BUILTIN {
        name: Option<ArcStr>,
        unboxArgs: bool,
    },
    /// The function has a body, but its function pointer is builtin. This means inline code+optimized pointer if need be.
    FUNCTION_BUILTIN_PTR,
}
impl Default for FunctionBuiltin {
    fn default() -> Self { Self::FUNCTION_NOT_BUILTIN }
}
pub use self::FunctionBuiltin::{FUNCTION_NOT_BUILTIN,FUNCTION_BUILTIN,FUNCTION_BUILTIN_PTR};

//This was a function restriction in SCode and Absyn
//Now it is part of function attributes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionParallelism {
    /// a normal function i.e non_parallel
    FP_NON_PARALLEL,
    /// an OpenCL/CUDA parallel/device function
    FP_PARALLEL_FUNCTION,
    /// an OpenCL/CUDA kernel function
    FP_KERNEL_FUNCTION,
}
impl Default for FunctionParallelism {
    fn default() -> Self { Self::FP_NON_PARALLEL }
}
pub use self::FunctionParallelism::{FP_NON_PARALLEL,FP_PARALLEL_FUNCTION,FP_KERNEL_FUNCTION};

/// a list of dimensions
pub type Dimensions = Arc<metamodelica::List<Arc<Dimension>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dimension {
    /// Dimension given by an integer.
    DIM_INTEGER {
        integer: i32,
    },
    /// Dimension given by Boolean
    DIM_BOOLEAN,
    /// Dimension given by an enumeration.
    DIM_ENUM {
        /// The enumeration type name.
        enumTypeName: Arc<Absyn::Path>,
        /// A list of the literals in the enumeration.
        literals: Arc<metamodelica::List<ArcStr>>,
        /// The size of the enumeration.
        size: i32,
    },
    /// Dimension given by an expression.
    DIM_EXP {
        exp: Arc<Exp>,
    },
    /// Dimension with unknown size.
    DIM_UNKNOWN,
}
pub use self::Dimension::{DIM_INTEGER,DIM_BOOLEAN,DIM_ENUM,DIM_EXP,DIM_UNKNOWN};

// adrpo: this is used to bind unknown dimensions to an expression
//        and when we do subtyping we add constrains to this expression.
//        this should be used for typechecking with unknown dimensions
//        when running checkModel. the binding acts like a type variable.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DimensionBinding {
    /// dimension is not bound
    DIM_UNBOUND,
    /// dimension is bound to an expression with constrains
    DIM_BOUND {
        /// the dimension is bound to this expression
        binding: Arc<Exp>,
        /// the bound has these constrains (collected when doing subtyping)
        constrains: Dimensions,
    },
}
pub use self::DimensionBinding::{DIM_UNBOUND,DIM_BOUND};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncArg {
    pub name: ArcStr,
    pub ty: Arc<Type>,
    pub r#const: Const,
    pub par: VarParallelism,
    pub defaultBinding: Option<Arc<Exp>>,
}

pub type FUNCARG = FuncArg;


/// The degree of constantness of an expression is determined by the Const
///    datatype. Variables declared as \'constant\' will get C_CONST constantness.
///    Variables declared as \'parameter\' will get C_PARAM constantness and
///    all other variables are not constant and will get C_VAR constantness.
///
///  - Variable properties
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Const {
    /// constant
    C_CONST,
    /// parameter
    C_PARAM,
    /// continuous
    C_VAR,
    C_UNKNOWN,
}
pub use self::Const::{C_CONST,C_PARAM,C_VAR,C_UNKNOWN};

/// A tuple is added to the Types. This is used by functions whom returns multiple arguments.
///  Used by split_props
///  - Tuple constants
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TupleConst {
    SINGLE_CONST {
        r#const: Const,
    },
    TUPLE_CONST {
        tupleConstLst: Arc<metamodelica::List<Arc<TupleConst>>>,
    },
}
pub use self::TupleConst::{SINGLE_CONST,TUPLE_CONST};

/// P.R 1.1 for multiple return arguments from functions,
///    one constant flag for each return argument.
///
///  The datatype `Properties\' contain information about an
///    expression.  The properties are created by analyzing the
///    expressions.
///  - Expression properties
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Properties {
    PROP {
        /// type
        type_: Arc<Type>,
        /// constFlag; if the type is a tuple, each element
        ///                  have a const flag.
        constFlag: Const,
    },
    PROP_TUPLE {
        type_: Arc<Type>,
        /// tupleConst; The elements might be
        ///                  tuple themselfs.
        tupleConst: Arc<TupleConst>,
    },
}
pub use self::Properties::{PROP,PROP_TUPLE};

/// To generate the correct set of equations, the translator has to
///  differentiate between the primitive types `Real\', `Integer\',
///  `String\', `Boolean\' and types directly derived from then from
///  other, complex types.  For arrays and matrices the type
///  `T_ARRAY\' is used, with the first argument being the number of
///  dimensions, and the second being the type of the objects in the
///  array.  The `Type\' type is used to store
///  information about whether a class is derived from a primitive
///  type, and whether a variable is of one of these types.
///  - Modification datatype, was originally in Mod
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EqMod {
    TYPED {
        /// modifier as expression
        modifierAsExp: Arc<Exp>,
        /// modifier as Value option
        modifierAsValue: Option<Arc<Values::Value>>,
        /// properties
        properties: Properties,
        /// keep the untyped modifier as an absyn expression for modification comparison
        modifierAsAbsynExp: Arc<Absyn::Exp>,
        info: SourceInfo,
    },
    UNTYPED {
        exp: Arc<Absyn::Exp>,
    },
}
pub use self::EqMod::{TYPED,UNTYPED};

/// -Sub Modification
/// named modification, i.e. (a = 5)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubMod {
    /// component name
    pub ident: Ident,
    /// modification
    pub r#mod: Arc<Mod>,
}

pub type NAMEMOD = SubMod;


/// Modification
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mod {
    MOD {
        /// final prefix
        finalPrefix: SCode::Final,
        /// each prefix
        eachPrefix: SCode::Each,
        subModLst: Arc<metamodelica::List<Arc<SubMod>>>,
        binding: Option<EqMod>,
        info: SourceInfo,
    },
    REDECL {
        /// final prefix
        finalPrefix: SCode::Final,
        /// each prefix
        eachPrefix: SCode::Each,
        element: Arc<SCode::Element>,
        r#mod: Arc<Mod>,
    },
    NOMOD,
}
pub use self::Mod::{MOD,REDECL,NOMOD};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockKind {
    INFERRED_CLOCK,
    RATIONAL_CLOCK {
        /// integer type >= 0
        intervalCounter: Arc<Exp>,
        /// integer type >= 1, defaults to 1
        resolution: Arc<Exp>,
    },
    REAL_CLOCK {
        /// real type > 0
        interval: Arc<Exp>,
    },
    EVENT_CLOCK {
        condition: Arc<Exp>,
        /// real type >= 0.0
        startInterval: Arc<Exp>,
    },
    SOLVER_CLOCK {
        /// clock type
        c: Arc<Exp>,
        /// string type
        solverMethod: Arc<Exp>,
    },
}
pub use self::ClockKind::{INFERRED_CLOCK,RATIONAL_CLOCK,REAL_CLOCK,EVENT_CLOCK,SOLVER_CLOCK};

/* -- End Types.mo -- */
/// Expressions
///  The 'Exp' datatype closely corresponds to the 'Absyn.Exp' datatype, but
///  is used for statically analyzed expressions. It includes explicit type
///  promotions and typed (non-overloaded) operators. It also contains expression
///  indexing with the 'ASUB' constructor. Indexing arbitrary array expressions
///  is currently not supported in Modelica, but it is needed here.
///
///  When making additions, update at least the following functions:
///  * Expression.traverseExp
///  * Expression.traverseExpTopDown
///  * Expression.traverseExpBiDir
///  * ExpressionBasics.printExpStr
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exp {
    ICONST {
        /// Integer constants
        integer: i32,
    },
    RCONST {
        /// Real constants
        real: metamodelica::Real,
    },
    SCONST {
        /// String constants
        string: ArcStr,
    },
    BCONST {
        /// Bool constants
        bool: bool,
    },
    /// Clock constructors
    CLKCONST {
        /// Clock kinds
        clk: Arc<ClockKind>,
    },
    /// Enumeration literal
    ENUM_LITERAL {
        name: Arc<Absyn::Path>,
        index: i32,
    },
    /// component references, e.g. a.b{2}.c{1}
    CREF {
        componentRef: Arc<ComponentRef>,
        ty: Arc<Type>,
    },
    /// Binary operations, e.g. a+4
    BINARY {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
    },
    /// Unary operations, -(4x)
    UNARY {
        operator: Operator,
        exp: Arc<Exp>,
    },
    /// Logical binary operations: and, or
    LBINARY {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
    },
    /// Logical unary operations: not
    LUNARY {
        operator: Operator,
        exp: Arc<Exp>,
    },
    /// Relation, e.g. a <= 0
    ///    Index contains normal an Integer for every ZeroCrossing
    ///    but if Relation is in algorithm with for loop the iterator and the range
    ///    of static iterator is needed for codegen
    RELATION {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
        /// Use -1 as a default; other indexes are used in the backend for some silly reasons
        index: i32,
        optionExpisASUB: Option<(Arc<Exp>, i32, i32)>,
    },
    /// If expressions
    IFEXP {
        expCond: Arc<Exp>,
        expThen: Arc<Exp>,
        expElse: Arc<Exp>,
    },
    CALL {
        path: Arc<Absyn::Path>,
        expLst: Arc<metamodelica::List<Arc<Exp>>>,
        attr: Arc<CallAttributes>,
    },
    /// A record value cannot be represented as a call to its constructor. This record also contains the protected components.
    RECORD {
        path: Arc<Absyn::Path>,
        /// component values
        exps: Arc<metamodelica::List<Arc<Exp>>>,
        /// component name
        comp: Arc<metamodelica::List<ArcStr>>,
        ty: Arc<Type>,
    },
    PARTEVALFUNCTION {
        path: Arc<Absyn::Path>,
        expList: Arc<metamodelica::List<Arc<Exp>>>,
        ty: Arc<Type>,
        origType: Arc<Type>,
    },
    ARRAY {
        ty: Arc<Type>,
        /// scalar for codegen
        scalar: bool,
        /// Array constructor, e.g. {1,3,4}
        array: Arc<metamodelica::List<Arc<Exp>>>,
    },
    MATRIX {
        ty: Arc<Type>,
        /// Size of the first dimension
        integer: i32,
        matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Exp>>>>>,
    },
    RANGE {
        /// the (array) type of the expression
        ty: Arc<Type>,
        /// start value
        start: Arc<Exp>,
        /// step value
        step: Option<Arc<Exp>>,
        /// stop value
        stop: Arc<Exp>,
    },
    TUPLE {
        /// PR. Tuples, used in func calls returning several
        ///                  arguments
        PR: Arc<metamodelica::List<Arc<Exp>>>,
    },
    /// Cast operator
    CAST {
        /// This is the full type of this expression, i.e. ET_ARRAY(...) for arrays and matrices
        ty: Arc<Type>,
        exp: Arc<Exp>,
    },
    /// Array subscripts
    ASUB {
        exp: Arc<Exp>,
        sub: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    /// Tuple 'subscript' (accessing only single values in calls)
    TSUB {
        exp: Arc<Exp>,
        ix: i32,
        ty: Arc<Type>,
    },
    /// Record field indexing
    RSUB {
        exp: Arc<Exp>,
        ix: i32,
        fieldName: ArcStr,
        ty: Arc<Type>,
    },
    /// The size operator
    SIZE {
        exp: Arc<Exp>,
        sz: Option<Arc<Exp>>,
    },
    /// Modelica AST constructor
    CODE {
        code: Arc<Absyn::CodeNode>,
        ty: Arc<Type>,
    },
    /// an empty expression, meaning a constant without a binding. is used to be able to continue the evaluation of a model even if there are
    ///     constants with no bindings. at the end, when we have the DAE we should have no EMPTY values or expressions in it when we need to simulate
    ///     the model.
    ///     From Modelica specification: a package may we look inside should not be partial in a simulation model!
    EMPTY {
        /// the scope where we could not find the binding
        scope: ArcStr,
        /// the name of the variable
        name: Arc<ComponentRef>,
        /// the type of the variable
        ty: Arc<Type>,
        tyStr: ArcStr,
    },
    /// e.g. sum(i*i+1 for i in 1:4)
    REDUCTION {
        reductionInfo: Arc<ReductionInfo>,
        /// expr, e.g i*i+1
        expr: Arc<Exp>,
        iterators: ReductionIterators,
    },
    /// MetaModelica list
    LIST {
        valList: Arc<metamodelica::List<Arc<Exp>>>,
    },
    /// MetaModelica list cons
    CONS {
        car: Arc<Exp>,
        cdr: Arc<Exp>,
    },
    META_TUPLE {
        listExp: Arc<metamodelica::List<Arc<Exp>>>,
    },
    META_OPTION {
        exp: Option<Arc<Exp>>,
    },
    METARECORDCALL {
        path: Arc<Absyn::Path>,
        args: Arc<metamodelica::List<Arc<Exp>>>,
        fieldNames: Arc<metamodelica::List<ArcStr>>,
        index: i32,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
    },
    MATCHEXPRESSION {
        matchType: MatchType,
        inputs: Arc<metamodelica::List<Arc<Exp>>>,
        /// input aliases (input as-bindings)
        aliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>,
        localDecls: Arc<metamodelica::List<Arc<Element>>>,
        cases: Arc<metamodelica::List<Arc<MatchCase>>>,
        et: Arc<Type>,
    },
    /// MetaModelica boxed value
    BOX {
        exp: Arc<Exp>,
    },
    /// MetaModelica value unboxing (similar to a cast)
    UNBOX {
        exp: Arc<Exp>,
        ty: Arc<Type>,
    },
    /// Before code generation, we make a pass that replaces constant literals
    ///    with a SHARED_LITERAL expression. Any immutable type can be shared:
    ///    basic MetaModelica types and Modelica strings are fine. There is no point
    ///    to share Real, Integer, Boolean or Enum though.
    SHARED_LITERAL {
        /// A unique indexing that can be used to point to a single shared literal in generated code
        index: i32,
        /// For printing strings, code generators that do not support this kind of literal, or for getting the type in case the code generator needs that
        exp: Arc<Exp>,
    },
    /// (x,1,ROOT(a as _,false,_)) := rhs; MetaModelica extension
    PATTERN {
        pattern: Arc<Pattern>,
    },
}
pub use self::Exp::{ICONST,RCONST,SCONST,BCONST,CLKCONST,ENUM_LITERAL,CREF,BINARY,UNARY,LBINARY,LUNARY,RELATION,IFEXP,CALL,RECORD,PARTEVALFUNCTION,ARRAY,MATRIX,RANGE,TUPLE,CAST,ASUB,TSUB,RSUB,SIZE,CODE,EMPTY,REDUCTION,LIST,CONS,META_TUPLE,META_OPTION,METARECORDCALL,MATCHEXPRESSION,BOX,UNBOX,SHARED_LITERAL,PATTERN};

/* mathematica constants */
pub static PI: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::RCONST { real: metamodelica::OrderedFloat(3.1415926535897932384626433832795028841971693993751058_f64) }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TailCall {
    /// Not tail-recursive
    NO_TAIL,
    TAIL {
        vars: Arc<metamodelica::List<ArcStr>>,
        outVars: Arc<metamodelica::List<ArcStr>>,
    },
}
pub use self::TailCall::{NO_TAIL,TAIL};

pub static callAttrBuiltinBool: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_BOOL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinInteger: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_INTEGER_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinReal: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_REAL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinString: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_STRING_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinOther: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_UNKNOWN_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureBool: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_BOOL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureInteger: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_INTEGER_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureReal: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_REAL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureString: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_STRING_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrOther: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_UNKNOWN_DEFAULT.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallAttributes {
    /// The type of the return value, if several return values this is undefined
    pub ty: Arc<Type>,
    /// tuple
    pub tuple_: bool,
    /// builtin Function call
    pub builtin: bool,
    /// if the function has prefix *impure* is true, else false
    pub isImpure: bool,
    pub isFunctionPointerCall: bool,
    pub inlineType: InlineType,
    /// Input variables of the function if the call is tail-recursive
    pub tailCall: TailCall,
}

pub type CALL_ATTR = CallAttributes;


/// A separate uniontype containing the information not required by traverseExp, etc
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReductionInfo {
    /// array, sum,..
    pub path: Arc<Absyn::Path>,
    pub iterType: Absyn::ReductionIterType,
    pub exprType: Arc<Type>,
    /// if there is no default value, the reduction is not defined for 0-length arrays/lists
    pub defaultValue: Option<Arc<Values::Value>>,
    pub foldName: ArcStr,
    /// Unique identifier for the resulting expression
    pub resultName: ArcStr,
    /// For example, max(ident,$res) or ident+$res; array() does not use this feature; DO NOT TRAVERSE THIS EXPRESSION!
    pub foldExp: Option<Arc<Exp>>,
}

pub type REDUCTIONINFO = ReductionInfo;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReductionIterator {
    pub id: ArcStr,
    pub exp: Arc<Exp>,
    pub guardExp: Option<Arc<Exp>>,
    pub ty: Arc<Type>,
}

pub type REDUCTIONITER = ReductionIterator;


/// NOTE: OMC only handles one iterator for now
pub type ReductionIterators = Arc<metamodelica::List<Arc<ReductionIterator>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchCase {
    /// ELSE is handled by not doing pattern-matching
    pub patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    /// Guard-expression
    pub patternGuard: Option<Arc<Exp>>,
    pub localDecls: Arc<metamodelica::List<Arc<Element>>>,
    pub body: Arc<metamodelica::List<Arc<Statement>>>,
    pub result: Option<Arc<Exp>>,
    /// We need to keep the line info here so we can set a breakpoint at the last statement of a match-expression
    pub resultInfo: SourceInfo,
    /// the number of iterations we should skip if we succeed with pattern-matching, but don't succeed
    pub jump: i32,
    pub info: SourceInfo,
}

pub type CASE = MatchCase;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchType {
    MATCHCONTINUE,
    TRY_STACKOVERFLOW,
    MATCH {
        /// The index of the pattern to switch over, its type and the value to divide string hashes with
        switch: Option<(i32, Arc<Type>, i32)>,
    },
}
pub use self::MatchType::{MATCHCONTINUE,TRY_STACKOVERFLOW,MATCH};

/// Patterns deconstruct expressions
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pattern {
    /// _
    PAT_WILD,
    /// compare to this constant value using equality
    PAT_CONSTANT {
        /// so we can unbox if needed
        ty: Option<Arc<Type>>,
        exp: Arc<Exp>,
    },
    /// id as pat
    PAT_AS {
        id: ArcStr,
        /// so we can unbox if needed
        ty: Option<Arc<Type>>,
        /// so we know if the ident is parameter or assignable
        attr: Arc<Attributes>,
        pat: Arc<Pattern>,
    },
    /// id as pat
    PAT_AS_FUNC_PTR {
        id: ArcStr,
        pat: Arc<Pattern>,
    },
    /// (pat1,...,patn)
    PAT_META_TUPLE {
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    },
    /// (pat1,...,patn)
    PAT_CALL_TUPLE {
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    },
    /// head::tail
    PAT_CONS {
        head: Arc<Pattern>,
        tail: Arc<Pattern>,
    },
    /// RECORD(pat1,...,patn); all patterns are positional
    PAT_CALL {
        name: Arc<Absyn::Path>,
        index: i32,
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
        fields: Arc<metamodelica::List<Arc<Var>>>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        /// The runtime system (dynload), does not know if the value is a singleton. But optimizations are safe if this is true.
        knownSingleton: bool,
    },
    /// RECORD(pat1,...,patn); all patterns are named
    PAT_CALL_NAMED {
        name: Arc<Absyn::Path>,
        patterns: Arc<metamodelica::List<(Arc<Pattern>, ArcStr, Arc<Type>)>>,
    },
    /// SOME(pat)
    PAT_SOME {
        pat: Arc<Pattern>,
    },
}
pub use self::Pattern::{PAT_WILD,PAT_CONSTANT,PAT_AS,PAT_AS_FUNC_PTR,PAT_META_TUPLE,PAT_CALL_TUPLE,PAT_CONS,PAT_CALL,PAT_CALL_NAMED,PAT_SOME};

/// Operators which are overloaded in the abstract syntax are here
///    made type-specific.  The integer addition operator (`ADD(INT)\')
///    and the real addition operator (`ADD(REAL)\') are two distinct
///    operators.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operator {
    ADD {
        ty: Arc<Type>,
    },
    SUB {
        ty: Arc<Type>,
    },
    MUL {
        ty: Arc<Type>,
    },
    DIV {
        ty: Arc<Type>,
    },
    POW {
        ty: Arc<Type>,
    },
    UMINUS {
        ty: Arc<Type>,
    },
    UMINUS_ARR {
        ty: Arc<Type>,
    },
    ADD_ARR {
        ty: Arc<Type>,
    },
    SUB_ARR {
        ty: Arc<Type>,
    },
    /// Element-wise array multiplication
    MUL_ARR {
        ty: Arc<Type>,
    },
    DIV_ARR {
        ty: Arc<Type>,
    },
    /// {a,b,c} * s
    MUL_ARRAY_SCALAR {
        /// type of the array
        ty: Arc<Type>,
    },
    /// {a,b,c} .+ s
    ADD_ARRAY_SCALAR {
        /// type of the array
        ty: Arc<Type>,
    },
    /// s .- {a,b,c}
    SUB_SCALAR_ARRAY {
        /// type of the array
        ty: Arc<Type>,
    },
    /// {a,b,c} * {c,d,e} => a*c+b*d+c*e
    MUL_SCALAR_PRODUCT {
        /// type of the array
        ty: Arc<Type>,
    },
    /// M1 * M2, matrix dot product
    MUL_MATRIX_PRODUCT {
        /// {{..},..}  {{..},{..}}
        ty: Arc<Type>,
    },
    /// {a, b} / c
    DIV_ARRAY_SCALAR {
        /// type of the array
        ty: Arc<Type>,
    },
    /// c / {a,b}
    DIV_SCALAR_ARRAY {
        /// type of the array
        ty: Arc<Type>,
    },
    POW_ARRAY_SCALAR {
        /// type of the array
        ty: Arc<Type>,
    },
    POW_SCALAR_ARRAY {
        /// type of the array
        ty: Arc<Type>,
    },
    /// Power of a matrix: {{1,2,3},{4,5.0,6},{7,8,9}}^2
    POW_ARR {
        /// type of the array
        ty: Arc<Type>,
    },
    /// elementwise power of arrays: {1,2,3}.^{3,2,1}
    POW_ARR2 {
        /// type of the array
        ty: Arc<Type>,
    },
    AND {
        ty: Arc<Type>,
    },
    OR {
        ty: Arc<Type>,
    },
    NOT {
        ty: Arc<Type>,
    },
    LESS {
        ty: Arc<Type>,
    },
    LESSEQ {
        ty: Arc<Type>,
    },
    GREATER {
        ty: Arc<Type>,
    },
    GREATEREQ {
        ty: Arc<Type>,
    },
    EQUAL {
        ty: Arc<Type>,
    },
    NEQUAL {
        ty: Arc<Type>,
    },
    USERDEFINED {
        /// The FQ name of the overloaded operator function
        fqName: Arc<Absyn::Path>,
    },
}
pub use self::Operator::{ADD,SUB,MUL,DIV,POW,UMINUS,UMINUS_ARR,ADD_ARR,SUB_ARR,MUL_ARR,DIV_ARR,MUL_ARRAY_SCALAR,ADD_ARRAY_SCALAR,SUB_SCALAR_ARRAY,MUL_SCALAR_PRODUCT,MUL_MATRIX_PRODUCT,DIV_ARRAY_SCALAR,DIV_SCALAR_ARRAY,POW_ARRAY_SCALAR,POW_SCALAR_ARRAY,POW_ARR,POW_ARR2,AND,OR,NOT,LESS,LESSEQ,GREATER,GREATEREQ,EQUAL,NEQUAL,USERDEFINED};

/// - Component references
///    CREF_QUAL(...) is used for qualified component names, e.g. a.b.c
///    CREF_IDENT(..) is used for non-qualifed component names, e.g. x
///    Outermost CREF_QUAL(...) is leftmost name. e.g. CREF_QUAL(a, CREF_IDENT(b)) -> a.b
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentRef {
    CREF_QUAL {
        ident: Ident,
        /// type of the identifier, without considering the subscripts
        identType: Arc<Type>,
        subscriptLst: Arc<metamodelica::List<Arc<Subscript>>>,
        componentRef: Arc<ComponentRef>,
    },
    CREF_IDENT {
        ident: Ident,
        /// type of the identifier, without considering the subscripts
        identType: Arc<Type>,
        subscriptLst: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    /// An Optimica component reference with the time instant in it. e.g x2(finalTime)
    OPTIMICA_ATTR_INST_CREF {
        componentRef: Arc<ComponentRef>,
        instant: ArcStr,
    },
    WILD,
}
pub use self::ComponentRef::{CREF_QUAL,CREF_IDENT,OPTIMICA_ATTR_INST_CREF,WILD};

pub static crefTime: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("time")).clone(), identType: T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

pub static crefTimeState: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("$time")).clone(), identType: T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

pub static emptyCref: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("")).clone(), identType: T_UNKNOWN_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

/// The `Subscript\' and `ComponentRef\' datatypes are simple
///  translations of the corresponding types in the `Absyn\' module.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Subscript {
    /// a{:,1}
    WHOLEDIM,
    SLICE {
        /// a{1:3,1}, a{1:2:10,2}
        exp: Arc<Exp>,
    },
    INDEX {
        /// a[i+1]
        exp: Arc<Exp>,
    },
    /// Used for non-expanded arrays. Should probably be combined with WHOLEDIM
    ///    into one case with Option<Exp> argument.
    WHOLE_NONEXP {
        exp: Arc<Exp>,
    },
}
pub use self::Subscript::{WHOLEDIM,SLICE,INDEX,WHOLE_NONEXP};

/* -- End Expression.mo -- */
/// array cref expansion strategy
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expand {
    /// expand crefs
    EXPAND,
    /// not expand crefs
    NOT_EXPAND,
}
pub use self::Expand::{EXPAND,NOT_EXPAND};

pub static emptyDae: std::sync::LazyLock<DAElist> = std::sync::LazyLock::new(|| { DAElist { elementLst: metamodelica::nil() } });

/// A Prefix has a component prefix and a class prefix.
/// The component prefix consist of a name an a list of constant valued subscripts.
/// The class prefix contains the variability of the class, i.e unspecified, parameter or constant.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prefix {
    /// No prefix information
    NOPRE,
    PREFIX {
        /// component prefixes are stored in inverse order c.b.a
        compPre: Arc<ComponentPrefix>,
        /// the class prefix, i.e. variability, var, discrete, param, const
        classPre: ClassPrefix,
    },
}
pub use self::Prefix::{NOPRE,PREFIX};

/// a type alias for an optional component prefix
pub type ComponentPrefixOpt = Option<Arc<ComponentPrefix>>;

/// Prefix for component name, e.g. a.b[2].c.
/// NOTE: Component prefixes are stored in inverse order c.b[2].a!
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentPrefix {
    PRE {
        /// prefix name
        prefix: ArcStr,
        /// dimensions
        dimensions: Arc<metamodelica::List<Arc<Dimension>>>,
        /// subscripts
        subscripts: Arc<metamodelica::List<Arc<Subscript>>>,
        /// next prefix
        next: Arc<ComponentPrefix>,
        /// to be able to at least partially fill in type information properly for DAE.VAR
        ci_state: ClassInf::State,
        info: SourceInfo,
    },
    NOCOMPPRE,
}
impl Default for ComponentPrefix {
    fn default() -> Self { Self::NOCOMPPRE }
}
pub use self::ComponentPrefix::{PRE,NOCOMPPRE};

/// Prefix for classes is its variability
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassPrefix {
    /// VAR, DISCRETE, PARAM, or CONST
    pub variability: SCode::Variability,
}

pub type CLASSPRE = ClassPrefix;


pub mod Connect {
    use super::*;
    pub const NEW_SET: i32 = -1;

    /// This type indicates whether a connector is an inside or an outside connector.
    ///   Note: this is not the same as inner and outer references.
    ///   A connector is inside if it connects from the outside into a component and it
    ///   is outside if it connects out from the component.  This is important when
    ///   generating equations for flow variables, where outside connectors are
    ///   multiplied with -1 (since flow is always into a component).
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Face {
        /// This is an inside connection
        INSIDE,
        /// This is an outside connection
        OUTSIDE,
        NO_FACE,
    }
    pub use self::Face::{INSIDE,OUTSIDE,NO_FACE};

    /// The type of a connector element.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ConnectorType {
        EQU,
        FLOW,
        STREAM {
            associatedFlow: Option<Arc<ComponentRef>>,
        },
        NO_TYPE,
    }
    pub use self::ConnectorType::{EQU,FLOW,STREAM,NO_TYPE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConnectorElement {
        pub name: Arc<ComponentRef>,
        pub face: Face,
        pub ty: ConnectorType,
        pub source: Arc<ElementSource>,
        /// Which set this element belongs to.
        pub set: i32,
    }

    pub type CONNECTOR_ELEMENT = ConnectorElement;


    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum SetTrieNode {
        /// A trie node has a name and contains a list of child nodes.
        SET_TRIE_NODE {
            name: ArcStr,
            cref: Arc<ComponentRef>,
            nodes: Arc<metamodelica::List<Arc<SetTrieNode>>>,
            connectCount: i32,
        },
        /// A trie leaf contains information about a connector element. Each connector
        ///     might be connected as both inside and outside, and stream connector
        ///     elements have an associated flow element.
        SET_TRIE_LEAF {
            name: ArcStr,
            /// The inside element.
            insideElement: Option<ConnectorElement>,
            /// The outside element.
            outsideElement: Option<ConnectorElement>,
            /// The name of the associated flow
            ///      variable, if the leaf represents a stream variable.
            flowAssociation: Option<Arc<ComponentRef>>,
            /// How many times this connector has been connected.
            connectCount: i32,
        },
    }
    pub use self::SetTrieNode::{SET_TRIE_NODE,SET_TRIE_LEAF};

    /// A trie, a.k.a. prefix tree, that maps crefs to sets.
    pub type SetTrie = Arc<SetTrieNode>;

    /// A connection between two sets.
    pub type SetConnection = (i32, i32);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OuterConnect {
        /// the scope where this connect was created
        pub scope: Prefix,
        /// the lhs component reference
        pub cr1: Arc<ComponentRef>,
        /// inner/outer attribute for cr1 component
        pub io1: Absyn::InnerOuter,
        /// the face of the lhs component
        pub f1: Face,
        /// the rhs component reference
        pub cr2: Arc<ComponentRef>,
        /// inner/outer attribute for cr2 component
        pub io2: Absyn::InnerOuter,
        /// the face of the rhs component
        pub f2: Face,
        /// the element origin
        pub source: Arc<ElementSource>,
    }

    pub type OUTERCONNECT = OuterConnect;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Sets {
        pub sets: SetTrie,
        /// How many sets the trie contains.
        pub setCount: i32,
        pub connections: Arc<metamodelica::List<(i32, i32)>>,
        /// Connect statements to propagate upwards.
        pub outerConnects: Arc<metamodelica::List<OuterConnect>>,
    }

    pub type SETS = Sets;


    /// A set of connection elements.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Set {
        /// A set with a type and a list of elements.
        SET {
            ty: ConnectorType,
            elements: Arc<metamodelica::List<ConnectorElement>>,
        },
        /// A pointer to another set.
        SET_POINTER {
            index: i32,
        },
    }
    pub use self::Set::{SET,SET_POINTER};

    pub static emptySet: std::sync::LazyLock<Sets> = std::sync::LazyLock::new(|| { Sets { sets: Arc::new(SetTrieNode::SET_TRIE_NODE { name: (literal!("")).clone(), cref: Arc::new(crate::DAE::ComponentRef::WILD), nodes: metamodelica::nil(), connectCount: 0 }), setCount: 0, connections: metamodelica::nil(), outerConnects: metamodelica::nil() } });

}

