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

// Some definitions are aliased from Absyn
pub type Ident = ArcStr;

pub type Path = Arc<Absyn::Path>;

pub type Subscript = Arc<Absyn::Subscript>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Restriction {
    R_CLASS,
    R_OPTIMIZATION,
    R_MODEL,
    R_RECORD {
        isOperator: bool,
    },
    R_BLOCK,
    /// a connector
    R_CONNECTOR {
        /// is expandable?
        isExpandable: bool,
    },
    R_OPERATOR,
    R_TYPE,
    R_PACKAGE,
    R_FUNCTION {
        functionRestriction: FunctionRestriction,
    },
    R_ENUMERATION,
    /// predefined IntegerType
    R_PREDEFINED_INTEGER,
    /// predefined RealType
    R_PREDEFINED_REAL,
    /// predefined StringType
    R_PREDEFINED_STRING,
    /// predefined BooleanType
    R_PREDEFINED_BOOLEAN,
    /// predefined EnumType
    R_PREDEFINED_ENUMERATION,
    /// predefined ClockType
    R_PREDEFINED_CLOCK,
    /// Metamodelica extension
    R_METARECORD {
        name: Arc<Absyn::Path>,
        index: i32,
        singleton: bool,
        moved: bool,
        typeVars: Arc<metamodelica::List<ArcStr>>,
    },
    /// Metamodelica extension
    R_UNIONTYPE {
        typeVars: Arc<metamodelica::List<ArcStr>>,
    },
}
impl Default for Restriction {
    fn default() -> Self { Self::R_CLASS }
}
pub use self::Restriction::{R_CLASS,R_OPTIMIZATION,R_MODEL,R_RECORD,R_BLOCK,R_CONNECTOR,R_OPERATOR,R_TYPE,R_PACKAGE,R_FUNCTION,R_ENUMERATION,R_PREDEFINED_INTEGER,R_PREDEFINED_REAL,R_PREDEFINED_STRING,R_PREDEFINED_BOOLEAN,R_PREDEFINED_ENUMERATION,R_PREDEFINED_CLOCK,R_METARECORD,R_UNIONTYPE};

// Same as Absyn.FunctionRestriction except this contains
// FR_EXTERNAL_FUNCTION and FR_RECORD_CONSTRUCTOR.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionRestriction {
    /// a normal function
    FR_NORMAL_FUNCTION {
        purity: Absyn::FunctionPurity,
    },
    /// an external function
    FR_EXTERNAL_FUNCTION {
        purity: Absyn::FunctionPurity,
    },
    /// an operator function
    FR_OPERATOR_FUNCTION,
    /// record constructor
    FR_RECORD_CONSTRUCTOR,
    /// an OpenCL/CUDA parallel/device function
    FR_PARALLEL_FUNCTION,
    /// an OpenCL/CUDA kernel function
    FR_KERNEL_FUNCTION,
}
impl Default for FunctionRestriction {
    fn default() -> Self { Self::FR_OPERATOR_FUNCTION }
}
pub use self::FunctionRestriction::{FR_NORMAL_FUNCTION,FR_EXTERNAL_FUNCTION,FR_OPERATOR_FUNCTION,FR_RECORD_CONSTRUCTOR,FR_PARALLEL_FUNCTION,FR_KERNEL_FUNCTION};

/// - Modifications
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mod {
    MOD {
        /// final prefix
        finalPrefix: Final,
        /// each prefix
        eachPrefix: Each,
        subModLst: Arc<metamodelica::List<Arc<SubMod>>>,
        binding: Option<Arc<Absyn::Exp>>,
        comment: Option<ArcStr>,
        info: SourceInfo,
    },
    REDECL {
        /// final prefix
        finalPrefix: Final,
        /// each prefix
        eachPrefix: Each,
        /// The new element declaration.
        element: Arc<Element>,
    },
    BREAK_COMPONENT {
        info: SourceInfo,
    },
    BREAK_CONNECT {
        lhs: Arc<Absyn::ComponentRef>,
        rhs: Arc<Absyn::ComponentRef>,
        info: SourceInfo,
    },
    NOMOD,
}
impl Default for Mod {
    fn default() -> Self { Self::NOMOD }
}
pub use self::Mod::{MOD,REDECL,BREAK_COMPONENT,BREAK_CONNECT,NOMOD};

/// Modifications are represented in an more structured way than in
///    the `Absyn\' module.  Modifications using qualified names
///    (such as in `x.y =  z\') are normalized (to `x(y = z)\').
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubMod {
    pub ident: Ident,
    /// A named component
    pub r#mod: Arc<Mod>,
}

impl Default for SubMod {
    fn default() -> Self {
        Self {
            ident: Default::default(),
            r#mod: Default::default(),
        }
    }
}

pub type NAMEMOD = SubMod;


/// - Programs
/// As in the AST, a program is simply a list of class definitions.
pub type Program = Arc<metamodelica::List<Arc<Element>>>;

/// Enum, which is a name in an enumeration and an optional Comment.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enum {
    pub literal: Ident,
    pub comment: Arc<Comment>,
}

impl Default for Enum {
    fn default() -> Self {
        Self {
            literal: Default::default(),
            comment: Default::default(),
        }
    }
}

pub type ENUM = Enum;


/// The major difference between these types and their Absyn
/// counterparts is that the PARTS constructor contains separate
/// lists for elements, equations and algorithms.
///
/// SCode.PARTS contains elements of a class definition. For instance,
///    model A
///      extends B;
///      C c;
///    end A;
/// Here PARTS contains two elements ('extends B' and 'C c')
/// SCode.DERIVED is used for short class definitions, i.e:
///  class A = B[ArrayDims](modifiers);
/// SCode.CLASS_EXTENDS is used for extended class definition, i.e:
///  class extends A (modifier)
///    new elements;
///  end A;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClassDef {
    /// a class made of parts
    PARTS {
        /// the list of elements
        elementLst: Arc<metamodelica::List<Arc<Element>>>,
        /// the list of equations
        normalEquationLst: Arc<metamodelica::List<Arc<Equation>>>,
        /// the list of initial equations
        initialEquationLst: Arc<metamodelica::List<Arc<Equation>>>,
        /// the list of algorithms
        normalAlgorithmLst: Arc<metamodelica::List<Arc<AlgorithmSection>>>,
        /// the list of initial algorithms
        initialAlgorithmLst: Arc<metamodelica::List<Arc<AlgorithmSection>>>,
        /// the list of constraints
        constraintLst: Arc<metamodelica::List<ConstraintSection>>,
        /// the list of class attributes. Currently for Optimica extensions
        clsattrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>,
        /// used by external functions
        externalDecl: Option<Arc<ExternalDecl>>,
    },
    /// an extended class definition plus the additional parts
    CLASS_EXTENDS {
        /// the modifications that need to be applied to the base class
        modifications: Arc<Mod>,
        /// the new composition
        composition: Arc<ClassDef>,
    },
    /// a derived class
    DERIVED {
        /// typeSpec: type specification
        typeSpec: Arc<Absyn::TypeSpec>,
        /// the modifications
        modifications: Arc<Mod>,
        /// the element attributes
        attributes: Attributes,
    },
    /// an enumeration
    ENUMERATION {
        /// if the list is empty it means :, the supertype of all enumerations
        enumLst: Arc<metamodelica::List<Arc<Enum>>>,
    },
    /// an overloaded function
    OVERLOAD {
        /// the path lists
        pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>,
    },
    /// the partial derivative
    PDER {
        /// function name
        functionPath: Arc<Absyn::Path>,
        /// derived variables
        derivedVariables: Arc<metamodelica::List<ArcStr>>,
    },
}
impl Default for ClassDef {
    fn default() -> Self {
        Self::ENUMERATION {
            enumLst: Default::default(),
        }
    }
}
pub use self::ClassDef::{PARTS,CLASS_EXTENDS,DERIVED,ENUMERATION,OVERLOAD,PDER};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Comment {
    pub annotation_: Option<Arc<Annotation>>,
    pub comment: Option<ArcStr>,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            annotation_: Default::default(),
            comment: Default::default(),
        }
    }
}

pub type COMMENT = Comment;


pub static noComment: std::sync::LazyLock<Arc<Comment>> = std::sync::LazyLock::new(|| { Arc::new(Comment { annotation_: None, comment: None }) });

// stefan
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Annotation {
    pub modification: Arc<Mod>,
}

impl Default for Annotation {
    fn default() -> Self {
        Self {
            modification: Default::default(),
        }
    }
}

pub type ANNOTATION = Annotation;


/// Declaration of an external function call - ExternalDecl
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalDecl {
    /// The name of the external function
    pub funcName: Option<ArcStr>,
    /// Language of the external function
    pub lang: Option<ArcStr>,
    /// output parameter as return value
    pub output_: Option<Arc<Absyn::ComponentRef>>,
    /// only positional arguments, i.e. expression list
    pub args: Arc<metamodelica::List<Arc<Absyn::Exp>>>,
    pub annotation_: Option<Arc<Annotation>>,
}

impl Default for ExternalDecl {
    fn default() -> Self {
        Self {
            funcName: Default::default(),
            lang: Default::default(),
            output_: Default::default(),
            args: Default::default(),
            annotation_: Default::default(),
        }
    }
}

pub type EXTERNALDECL = ExternalDecl;


/// These represent equations and are almost identical to their Absyn versions.
/// In EQ_IF the elseif branches are represented as normal else branches with
/// a single if statement in them.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Equation {
    EQ_IF {
        /// conditional
        condition: Arc<metamodelica::List<Arc<Absyn::Exp>>>,
        /// the true (then) branch
        thenBranch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Equation>>>>>,
        /// the false (else) branch
        elseBranch: Arc<metamodelica::List<Arc<Equation>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// the equality equation
    EQ_EQUALS {
        /// the expression on the left side of the operator
        expLeft: Arc<Absyn::Exp>,
        /// the expression on the right side of the operator
        expRight: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// partial differential equation or boundary condition
    EQ_PDE {
        /// the expression on the left side of the operator
        expLeft: Arc<Absyn::Exp>,
        /// the expression on the right side of the operator
        expRight: Arc<Absyn::Exp>,
        /// domain for PDEs
        domain: Arc<Absyn::ComponentRef>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// the connect equation
    EQ_CONNECT {
        /// the connector/component reference on the left side
        crefLeft: Arc<Absyn::ComponentRef>,
        /// the connector/component reference on the right side
        crefRight: Arc<Absyn::ComponentRef>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// the for equation
    EQ_FOR {
        /// the index name
        index: Ident,
        /// the range of the index
        range: Option<Arc<Absyn::Exp>>,
        /// the equation list
        eEquationLst: Arc<metamodelica::List<Arc<Equation>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// the when equation
    EQ_WHEN {
        /// the when condition
        condition: Arc<Absyn::Exp>,
        /// the equation list
        eEquationLst: Arc<metamodelica::List<Arc<Equation>>>,
        /// the elsewhen expression and equation list
        elseBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Equation>>>)>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// the assert equation
    EQ_ASSERT {
        /// the assert condition
        condition: Arc<Absyn::Exp>,
        /// the assert message
        message: Arc<Absyn::Exp>,
        level: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// the terminate equation
    EQ_TERMINATE {
        /// the terminate message
        message: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// a reinit equation
    EQ_REINIT {
        /// the variable to initialize
        cref: Arc<Absyn::Exp>,
        /// the new value
        expReinit: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    /// function calls without return value
    EQ_NORETCALL {
        exp: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
}
pub use self::Equation::{EQ_IF,EQ_EQUALS,EQ_PDE,EQ_CONNECT,EQ_FOR,EQ_WHEN,EQ_ASSERT,EQ_TERMINATE,EQ_REINIT,EQ_NORETCALL};

/// - Algorithms
///  The Absyn module uses the terminology from the
///  grammar, where algorithm means an algorithmic
///  statement. But here, an Algorithm means a whole
///  algorithm section.
/// the algorithm section
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlgorithmSection {
    /// the algorithm statements
    pub statements: Arc<metamodelica::List<Arc<Statement>>>,
}

impl Default for AlgorithmSection {
    fn default() -> Self {
        Self {
            statements: Default::default(),
        }
    }
}

pub type ALGORITHM = AlgorithmSection;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstraintSection {
    pub constraints: Arc<metamodelica::List<Arc<Absyn::Exp>>>,
}

impl Default for ConstraintSection {
    fn default() -> Self {
        Self {
            constraints: Default::default(),
        }
    }
}

pub type CONSTRAINTS = ConstraintSection;


/// The Statement type describes one algorithm statement in an algorithm section.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Statement {
    ALG_ASSIGN {
        /// assignComponent
        assignComponent: Arc<Absyn::Exp>,
        /// value
        value: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_IF {
        boolExpr: Arc<Absyn::Exp>,
        trueBranch: Arc<metamodelica::List<Arc<Statement>>>,
        elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Statement>>>)>>,
        elseBranch: Arc<metamodelica::List<Arc<Statement>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_FOR {
        /// the index name
        index: Ident,
        /// the range of the index
        range: Option<Arc<Absyn::Exp>>,
        /// forBody
        forBody: Arc<metamodelica::List<Arc<Statement>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_PARFOR {
        /// the index name
        index: Ident,
        /// the range of the index
        range: Option<Arc<Absyn::Exp>>,
        /// parallel for loop body
        parforBody: Arc<metamodelica::List<Arc<Statement>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_WHILE {
        /// boolExpr
        boolExpr: Arc<Absyn::Exp>,
        /// whileBody
        whileBody: Arc<metamodelica::List<Arc<Statement>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_WHEN_A {
        branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Statement>>>)>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_ASSERT {
        condition: Arc<Absyn::Exp>,
        message: Arc<Absyn::Exp>,
        level: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_TERMINATE {
        message: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_REINIT {
        cref: Arc<Absyn::Exp>,
        newValue: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_NORETCALL {
        exp: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_RETURN {
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_BREAK {
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_FAILURE {
        stmts: Arc<metamodelica::List<Arc<Statement>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_TRY {
        body: Arc<metamodelica::List<Arc<Statement>>>,
        elseBody: Arc<metamodelica::List<Arc<Statement>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_CONTINUE {
        comment: Arc<Comment>,
        info: SourceInfo,
    },
}
pub use self::Statement::{ALG_ASSIGN,ALG_IF,ALG_FOR,ALG_PARFOR,ALG_WHILE,ALG_WHEN_A,ALG_ASSERT,ALG_TERMINATE,ALG_REINIT,ALG_NORETCALL,ALG_RETURN,ALG_BREAK,ALG_FAILURE,ALG_TRY,ALG_CONTINUE};

// common prefixes to elements
/// the visibility prefix
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Visibility {
    /// a public element
    PUBLIC,
    /// a protected element
    PROTECTED,
}
impl Default for Visibility {
    fn default() -> Self { Self::PUBLIC }
}
pub use self::Visibility::{PUBLIC,PROTECTED};

/// the redeclare prefix
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Redeclare {
    /// a redeclare prefix
    REDECLARE,
    /// a non redeclare prefix
    NOT_REDECLARE,
}
impl Default for Redeclare {
    fn default() -> Self { Self::REDECLARE }
}
pub use self::Redeclare::{REDECLARE,NOT_REDECLARE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstrainClass {
    pub constrainingClass: Arc<Absyn::Path>,
    pub modifier: Arc<Mod>,
    pub comment: Arc<Comment>,
}

pub type CONSTRAINCLASS = ConstrainClass;


/// the replaceable prefix
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Replaceable {
    /// a replaceable prefix containing an optional constraint
    REPLACEABLE {
        /// the constraint class
        cc: Option<Arc<ConstrainClass>>,
    },
    /// a non replaceable prefix
    NOT_REPLACEABLE,
}
impl Default for Replaceable {
    fn default() -> Self { Self::NOT_REPLACEABLE }
}
pub use self::Replaceable::{REPLACEABLE,NOT_REPLACEABLE};

/// the final prefix
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Final {
    /// a final prefix
    FINAL,
    /// a non final prefix
    NOT_FINAL,
}
impl Default for Final {
    fn default() -> Self { Self::FINAL }
}
pub use self::Final::{FINAL,NOT_FINAL};

/// the each prefix
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Each {
    /// a each prefix
    EACH,
    /// a non each prefix
    NOT_EACH,
}
impl Default for Each {
    fn default() -> Self { Self::EACH }
}
pub use self::Each::{EACH,NOT_EACH};

/// the encapsulated prefix
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Encapsulated {
    /// a encapsulated prefix
    ENCAPSULATED,
    /// a non encapsulated prefix
    NOT_ENCAPSULATED,
}
impl Default for Encapsulated {
    fn default() -> Self { Self::ENCAPSULATED }
}
pub use self::Encapsulated::{ENCAPSULATED,NOT_ENCAPSULATED};

/// the partial prefix
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Partial {
    /// a partial prefix
    PARTIAL,
    /// a non partial prefix
    NOT_PARTIAL,
}
impl Default for Partial {
    fn default() -> Self { Self::PARTIAL }
}
pub use self::Partial::{PARTIAL,NOT_PARTIAL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectorType {
    /// No connector type prefix.
    POTENTIAL,
    /// A flow prefix.
    FLOW,
    /// A stream prefix.
    STREAM,
}
impl Default for ConnectorType {
    fn default() -> Self { Self::POTENTIAL }
}
pub use self::ConnectorType::{POTENTIAL,FLOW,STREAM};

/// the common class or component prefixes
/// the common class or component prefixes
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Prefixes {
    /// the protected/public prefix
    pub visibility: Visibility,
    /// redeclare prefix
    pub redeclarePrefix: Redeclare,
    /// final prefix, be it at the element or top level
    pub finalPrefix: Final,
    /// the inner/outer/innerouter prefix
    pub innerOuter: Absyn::InnerOuter,
    /// replaceable prefix
    pub replaceablePrefix: Arc<Replaceable>,
}

impl Default for Prefixes {
    fn default() -> Self {
        Self {
            visibility: Default::default(),
            redeclarePrefix: Default::default(),
            finalPrefix: Default::default(),
            innerOuter: Default::default(),
            replaceablePrefix: Default::default(),
        }
    }
}

pub type PREFIXES = Prefixes;


/// - Elements
///  There are four types of elements in a declaration, represented by the constructors:
///  IMPORT     (for import clauses)
///  EXTENDS    (for extends clauses),
///  CLASS      (for top/local class definitions)
///  COMPONENT  (for local variables)
///  DEFINEUNIT (for units)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    /// an import element
    IMPORT {
        /// the import definition
        imp: Absyn::Import,
        /// the protected/public prefix
        visibility: Visibility,
        /// the import information
        info: SourceInfo,
    },
    /// the extends element
    EXTENDS {
        /// the extends path
        baseClassPath: Path,
        /// the protected/public prefix
        visibility: Visibility,
        /// the modifications applied to the base class
        modifications: Arc<Mod>,
        /// the extends annotation
        ann: Option<Arc<Annotation>>,
        /// the extends info
        info: SourceInfo,
    },
    /// a class definition
    CLASS {
        /// the name of the class
        name: Ident,
        /// the common class or component prefixes
        prefixes: Arc<Prefixes>,
        /// the encapsulated prefix
        encapsulatedPrefix: Encapsulated,
        /// the partial prefix
        partialPrefix: Partial,
        /// the restriction of the class
        restriction: Restriction,
        /// the class specification
        classDef: Arc<ClassDef>,
        /// the class annotation and string-comment
        cmt: Arc<Comment>,
        /// the class information
        info: SourceInfo,
    },
    /// a component
    COMPONENT {
        /// the component name
        name: Ident,
        /// the common class or component prefixes
        prefixes: Arc<Prefixes>,
        /// the component attributes
        attributes: Attributes,
        /// the type specification
        typeSpec: Arc<Absyn::TypeSpec>,
        /// the modifications to be applied to the component
        modifications: Arc<Mod>,
        /// this if for extraction of comments and annotations from Absyn
        comment: Arc<Comment>,
        /// the conditional declaration of a component
        condition: Option<Arc<Absyn::Exp>>,
        /// this is for line and column numbers, also file name.
        info: SourceInfo,
    },
    /// a unit defintion has a name and the two optional parameters exp, and weight
    DEFINEUNIT {
        name: Ident,
        /// the protected/public prefix
        visibility: Visibility,
        /// the unit expression
        exp: Option<ArcStr>,
        /// the weight
        weight: Option<metamodelica::Real>,
        /// The source information
        info: SourceInfo,
    },
}
pub use self::Element::{IMPORT,EXTENDS,CLASS,COMPONENT,DEFINEUNIT};

/// - Attributes
/// the attributes of the component
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attributes {
    /// the array dimensions of the component
    pub arrayDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>,
    /// The connector type: flow, stream or nothing.
    pub connectorType: ConnectorType,
    /// parallelism prefix: parglobal, parlocal, parprivate
    pub parallelism: Parallelism,
    /// the variability: parameter, discrete, variable, constant
    pub variability: Variability,
    /// the direction: input, output or bidirectional
    pub direction: Absyn::Direction,
    /// non-fiel / field
    pub isField: Absyn::IsField,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            arrayDims: Default::default(),
            connectorType: Default::default(),
            parallelism: Default::default(),
            variability: Default::default(),
            direction: Default::default(),
            isField: Default::default(),
        }
    }
}

pub type ATTR = Attributes;


/// Parallelism
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Parallelism {
    /// Global variables for CUDA and OpenCL
    PARGLOBAL,
    /// Shared for CUDA and local for OpenCL
    PARLOCAL,
    /// Non parallel/Normal variables
    NON_PARALLEL,
}
impl Default for Parallelism {
    fn default() -> Self { Self::PARGLOBAL }
}
pub use self::Parallelism::{PARGLOBAL,PARLOCAL,NON_PARALLEL};

/// the variability of a component
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variability {
    /// a variable
    VAR,
    /// a discrete variable
    DISCRETE,
    /// a parameter
    PARAM,
    /// a constant
    CONST,
}
impl Default for Variability {
    fn default() -> Self { Self::VAR }
}
pub use self::Variability::{VAR,DISCRETE,PARAM,CONST};

/* adrpo: previously present in Inst.mo */
/// the initial attribute of an algorithm or equation
/// Intial is used as argument to instantiation-function for
/// specifying if equations or algorithms are initial or not.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Initial {
    /// an initial equation or algorithm
    INITIAL,
    /// a normal equation or algorithm
    NON_INITIAL,
}
impl Default for Initial {
    fn default() -> Self { Self::INITIAL }
}
pub use self::Initial::{INITIAL,NON_INITIAL};

pub static defaultPrefixes: std::sync::LazyLock<Arc<Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(Prefixes { visibility: crate::SCode::Visibility::PUBLIC, redeclarePrefix: crate::SCode::Redeclare::NOT_REDECLARE, finalPrefix: crate::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE) }) });

pub static defaultProtectedPrefixes: std::sync::LazyLock<Arc<Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(Prefixes { visibility: crate::SCode::Visibility::PROTECTED, redeclarePrefix: crate::SCode::Redeclare::NOT_REDECLARE, finalPrefix: crate::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE) }) });

pub static defaultVarAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static defaultParamAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static defaultConstAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static defaultInputAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static defaultOutputAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::OUTPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

