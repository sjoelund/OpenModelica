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

/// An identifier, for example a variable name
pub type Ident = ArcStr;

/// For Iterator - these are used in:
///   * for loops where the expression part can be NONE() and then the range
///     is taken from an array variable that the iterator is used to index,
///     see 3.3.3.2 Several Iterators from Modelica Specification.
///   * in array iterators where the expression should always be SOME(Exp),
///     see 3.4.4.2 Array constructor with iterators from Specification
///   * the guard is a MetaModelica extension; it's a Boolean expression that
///     filters out items in the range.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForIterator {
    pub name: ArcStr,
    pub guardExp: Option<Arc<Exp>>,
    pub range: Option<Arc<Exp>>,
}

impl Default for ForIterator {
    fn default() -> Self {
        Self {
            name: Default::default(),
            guardExp: Default::default(),
            range: Default::default(),
        }
    }
}

pub type ITERATOR = ForIterator;


/// For Iterators -
///   these are used in:
///   * for loops where the expression part can be NONE() and then the range
///     is taken from an array variable that the iterator is used to index,
///     see 3.3.3.2 Several Iterators from Modelica Specification.
///   * in array iterators where the expression should always be SOME(Exp),
///     see 3.4.4.2 Array constructor with iterators from Specification
pub type ForIterators = Arc<metamodelica::List<Arc<ForIterator>>>;

/// - Programs, the top level construct
///   A program is simply a list of class definitions declared at top
///   level in the source file, combined with a within statement that
///   indicates the hieractical position of the program.
/// PROGRAM, the top level construct
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Program {
    /// List of classes
    pub classes: Arc<metamodelica::List<Arc<Class>>>,
    /// Within clause
    pub within_: Within,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            classes: Default::default(),
            within_: Default::default(),
        }
    }
}

pub type PROGRAM = Program;


/// Within Clauses
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Within {
    /// the within clause
    WITHIN {
        /// the path for within
        path: Arc<Path>,
    },
    TOP,
}
impl Default for Within {
    fn default() -> Self { Self::TOP }
}
pub use self::Within::{WITHIN,TOP};

pub type Info = SourceInfo;

/// A class definition consists of a name, a flag to indicate
///  if this class is declared as partial, the declared class restriction,
///  and the body of the declaration.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Class {
    pub name: Ident,
    /// true if partial
    pub partialPrefix: bool,
    /// true if final
    pub finalPrefix: bool,
    /// true if encapsulated
    pub encapsulatedPrefix: bool,
    /// Restriction
    pub restriction: Restriction,
    pub body: Arc<ClassDef>,
    /// when a class is the first one in the file and has a comment before it
    pub commentsBeforeClass: Arc<metamodelica::List<ArcStr>>,
    /// when a class has comments before its end
    pub commentsBeforeEnd: Arc<metamodelica::List<ArcStr>>,
    /// when the class has comments after its end, before the next class or the end of the file
    pub commentsAfterEnd: Arc<metamodelica::List<ArcStr>>,
    /// Information: FileName is the class is defined in +
    ///               isReadOnly bool + start line no + start column no +
    ///               end line no + end column no
    pub info: Info,
}

pub type CLASS = Class;


/// The ClassDef type contains thClasse definition part of a class declaration.
/// The definition is either explicit, with a list of parts
/// (public, protected, equation, and algorithm), or it is a definition
/// derived from another class or an enumeration type.
/// For a derived type, the  type contains the name of the derived class
/// and an optional array dimension and a list of modifications.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClassDef {
    PARTS {
        /// class A<B,C> ... has type variables B,C
        typeVars: Arc<metamodelica::List<ArcStr>>,
        /// optimization Op (objective=...) end Op. A list arguments attributing a
        ///    class declaration. Currently used only for Optimica extensions
        classAttrs: Arc<metamodelica::List<Arc<NamedArg>>>,
        classParts: Arc<metamodelica::List<Arc<ClassPart>>>,
        /// Modelica2 allowed multiple class-annotations
        ann: Arc<metamodelica::List<Arc<Annotation>>>,
        comment: Option<ArcStr>,
    },
    DERIVED {
        /// typeSpec specification includes array dimensions
        typeSpec: Arc<TypeSpec>,
        attributes: ElementAttributes,
        arguments: Arc<metamodelica::List<Arc<ElementArg>>>,
        comment: Option<Arc<Comment>>,
    },
    ENUMERATION {
        enumLiterals: Arc<EnumDef>,
        comment: Option<Arc<Comment>>,
    },
    OVERLOAD {
        functionNames: Arc<metamodelica::List<Arc<Path>>>,
        comment: Option<Arc<Comment>>,
    },
    CLASS_EXTENDS {
        /// name of class to extend
        baseClassName: Ident,
        /// modifications to be applied to the base class
        modifications: Arc<metamodelica::List<Arc<ElementArg>>>,
        /// comment
        comment: Option<ArcStr>,
        /// class parts
        parts: Arc<metamodelica::List<Arc<ClassPart>>>,
        ann: Arc<metamodelica::List<Arc<Annotation>>>,
    },
    PDER {
        functionName: Arc<Path>,
        /// derived variables
        vars: Arc<metamodelica::List<ArcStr>>,
        /// comment
        comment: Option<Arc<Comment>>,
    },
}
impl Default for ClassDef {
    fn default() -> Self {
        Self::ENUMERATION {
            enumLiterals: Default::default(),
            comment: Default::default(),
        }
    }
}
pub use self::ClassDef::{PARTS,DERIVED,ENUMERATION,OVERLOAD,CLASS_EXTENDS,PDER};

/// Component attributes are
///  properties of components which are applied by type prefixes.
///  As an example, declaring a component as `input Real x;\' will
///  give the attributes `ATTR({},false,VAR,INPUT)\'.
///  Components in Modelica can be scalar or arrays with one or more
///  dimensions. This type is used to indicate the dimensionality
///  of a component or a type definition.
/// - Array dimensions
pub type ArrayDim = Arc<metamodelica::List<Arc<Subscript>>>;

/// ModExtension: new MetaModelica type specification!
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeSpec {
    TPATH {
        path: Arc<Path>,
        arrayDim: Option<Arc<metamodelica::List<Arc<Subscript>>>>,
    },
    TCOMPLEX {
        path: Arc<Path>,
        typeSpecs: Arc<metamodelica::List<Arc<TypeSpec>>>,
        arrayDim: Option<Arc<metamodelica::List<Arc<Subscript>>>>,
    },
}
pub use self::TypeSpec::{TPATH,TCOMPLEX};

/// The definition of an enumeration is either a list of literals
///     or a colon, \':\', which defines a supertype of all enumerations
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumDef {
    ENUMLITERALS {
        enumLiterals: Arc<metamodelica::List<Arc<EnumLiteral>>>,
    },
    ENUM_COLON,
}
impl Default for EnumDef {
    fn default() -> Self { Self::ENUM_COLON }
}
pub use self::EnumDef::{ENUMLITERALS,ENUM_COLON};

/// EnumLiteral, which is a name in an enumeration and an optional
///   Comment.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumLiteral {
    pub literal: Ident,
    pub comment: Option<Arc<Comment>>,
}

impl Default for EnumLiteral {
    fn default() -> Self {
        Self {
            literal: Default::default(),
            comment: Default::default(),
        }
    }
}

pub type ENUMLITERAL = EnumLiteral;


/// A class definition contains several parts.  There are public and
///  protected component declarations, type definitions and `extends\'
///  clauses, collectively called elements.  There are also equation
///  sections and algorithm sections. The EXTERNAL part is used only by functions
///  which can be declared as external C or FORTRAN functions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClassPart {
    PUBLIC {
        contents: Arc<metamodelica::List<Arc<ElementItem>>>,
    },
    PROTECTED {
        contents: Arc<metamodelica::List<Arc<ElementItem>>>,
    },
    CONSTRAINTS {
        contents: Arc<metamodelica::List<Arc<Exp>>>,
    },
    EQUATIONS {
        contents: Arc<metamodelica::List<Arc<EquationItem>>>,
    },
    INITIALEQUATIONS {
        contents: Arc<metamodelica::List<Arc<EquationItem>>>,
    },
    ALGORITHMS {
        contents: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    INITIALALGORITHMS {
        contents: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    EXTERNAL {
        /// externalDecl
        externalDecl: Arc<ExternalDecl>,
        /// annotation
        annotation_: Option<Arc<Annotation>>,
    },
}
impl Default for ClassPart {
    fn default() -> Self {
        Self::PUBLIC {
            contents: Default::default(),
        }
    }
}
pub use self::ClassPart::{PUBLIC,PROTECTED,CONSTRAINTS,EQUATIONS,INITIALEQUATIONS,ALGORITHMS,INITIALALGORITHMS,EXTERNAL};

/// An element item is either an element or an annotation
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementItem {
    ELEMENTITEM {
        element: Arc<Element>,
    },
    LEXER_COMMENT {
        comment: ArcStr,
    },
}
impl Default for ElementItem {
    fn default() -> Self {
        Self::LEXER_COMMENT {
            comment: Default::default(),
        }
    }
}
pub use self::ElementItem::{ELEMENTITEM,LEXER_COMMENT};

/// Elements
///  The basic element type in Modelica
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    ELEMENT {
        finalPrefix: bool,
        /// replaceable, redeclare
        redeclareKeywords: Option<RedeclareKeywords>,
        /// inner/outer
        innerOuter: InnerOuter,
        /// Actual element specification
        specification: Arc<ElementSpec>,
        /// File name the class is defined in + line no + column no
        info: Info,
        /// only valid for classdef and component
        constrainClass: Option<Arc<ConstrainClass>>,
    },
    DEFINEUNIT {
        name: Ident,
        args: Arc<metamodelica::List<Arc<NamedArg>>>,
        info: Info,
    },
    TEXT {
        /// optName : optional name of text, e.g. model with syntax error.
        ///                                       We need the name to be able to browse it...
        optName: Option<ArcStr>,
        string: ArcStr,
        info: Info,
    },
}
pub use self::Element::{ELEMENT,DEFINEUNIT,TEXT};

/// Constraining type, must be extends
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstrainClass {
    /// must be extends
    pub elementSpec: Arc<ElementSpec>,
    /// comment
    pub comment: Option<Arc<Comment>>,
}

pub type CONSTRAINCLASS = ConstrainClass;


/// An element is something that occurs in a public or protected
///    section in a class definition.  There is one constructor in the
///    `ElementSpec\' type for each possible element type.  There are
///    class definitions (`CLASSDEF\'), `extends\' clauses (`EXTENDS\')
///    and component declarations (`COMPONENTS\').
///
///    As an example, if the element `extends TwoPin;\' appears
///    in the source, it is represented in the AST as
///    `EXTENDS(IDENT(\"TwoPin\"),{})\'.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementSpec {
    CLASSDEF {
        /// replaceable
        replaceable_: bool,
        /// class
        class_: Arc<Class>,
    },
    EXTENDS {
        /// path
        path: Arc<Path>,
        /// elementArg
        elementArg: Arc<metamodelica::List<Arc<ElementArg>>>,
        /// optional annotation
        annotationOpt: Option<Arc<Annotation>>,
    },
    IMPORT {
        /// import
        import_: Import,
        /// comment
        comment: Option<Arc<Comment>>,
        info: Info,
    },
    COMPONENTS {
        /// attributes
        attributes: ElementAttributes,
        /// typeSpec
        typeSpec: Arc<TypeSpec>,
        /// components
        components: Arc<metamodelica::List<Arc<ComponentItem>>>,
    },
}
pub use self::ElementSpec::{CLASSDEF,EXTENDS,IMPORT,COMPONENTS};

/// One of the keyword inner and outer CAN be given to reference an
///   inner or outer element. Thus there are three disjoint possibilities.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InnerOuter {
    /// an inner prefix
    INNER,
    /// an outer prefix
    OUTER,
    /// an inner outer prefix
    INNER_OUTER,
    /// no inner outer prefix
    NOT_INNER_OUTER,
}
impl Default for InnerOuter {
    fn default() -> Self { Self::INNER }
}
pub use self::InnerOuter::{INNER,OUTER,INNER_OUTER,NOT_INNER_OUTER};

/// Import statements, different kinds
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Import {
    NAMED_IMPORT {
        /// name
        name: Ident,
        /// path
        path: Arc<Path>,
    },
    QUAL_IMPORT {
        /// path
        path: Arc<Path>,
    },
    UNQUAL_IMPORT {
        /// path
        path: Arc<Path>,
    },
    GROUP_IMPORT {
        prefix: Arc<Path>,
        groups: Arc<metamodelica::List<GroupImport>>,
    },
}
pub use self::Import::{NAMED_IMPORT,QUAL_IMPORT,UNQUAL_IMPORT,GROUP_IMPORT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroupImport {
    GROUP_IMPORT_NAME {
        name: ArcStr,
    },
    GROUP_IMPORT_RENAME {
        rename: ArcStr,
        name: ArcStr,
    },
}
impl Default for GroupImport {
    fn default() -> Self {
        Self::GROUP_IMPORT_NAME {
            name: Default::default(),
        }
    }
}
pub use self::GroupImport::{GROUP_IMPORT_NAME,GROUP_IMPORT_RENAME};

/// A componentItem can have a condition that must be fulfilled if
///  the component should be instantiated.
pub type ComponentCondition = Arc<Exp>;

/// Collection of component and an optional comment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentItem {
    /// component
    pub component: Component,
    /// condition
    pub condition: Option<Arc<Exp>>,
    /// comment
    pub comment: Option<Arc<Comment>>,
}

impl Default for ComponentItem {
    fn default() -> Self {
        Self {
            component: Default::default(),
            condition: Default::default(),
            comment: Default::default(),
        }
    }
}

pub type COMPONENTITEM = ComponentItem;


/// Some kind of Modelica entity (object or variable)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Component {
    /// name
    pub name: Ident,
    /// Array dimensions, if any
    pub arrayDim: ArrayDim,
    /// Optional modification
    pub modification: Option<Arc<Modification>>,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            name: Default::default(),
            arrayDim: Default::default(),
            modification: Default::default(),
        }
    }
}

pub type COMPONENT = Component;


/// Several component declarations can be grouped together in one
///  `ElementSpec\' by writing them on the same line in the source.
///  This type contains the information specific to one component.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquationItem {
    EQUATIONITEM {
        /// equation
        equation_: Arc<Equation>,
        /// comment
        comment: Option<Arc<Comment>>,
        /// line number
        info: Info,
    },
    EQUATIONITEMCOMMENT {
        comment: ArcStr,
    },
}
impl Default for EquationItem {
    fn default() -> Self {
        Self::EQUATIONITEMCOMMENT {
            comment: Default::default(),
        }
    }
}
pub use self::EquationItem::{EQUATIONITEM,EQUATIONITEMCOMMENT};

/// Info specific for an algorithm item.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlgorithmItem {
    ALGORITHMITEM {
        /// algorithm
        algorithm_: Arc<Algorithm>,
        /// comment
        comment: Option<Arc<Comment>>,
        /// line number
        info: Info,
    },
    /// A comment from the lexer
    ALGORITHMITEMCOMMENT {
        comment: ArcStr,
    },
}
impl Default for AlgorithmItem {
    fn default() -> Self {
        Self::ALGORITHMITEMCOMMENT {
            comment: Default::default(),
        }
    }
}
pub use self::AlgorithmItem::{ALGORITHMITEM,ALGORITHMITEMCOMMENT};

/// Information on one (kind) of equation, different constructors for different
///     kinds of equations
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Equation {
    EQ_IF {
        /// Conditional expression
        ifExp: Arc<Exp>,
        /// true branch
        equationTrueItems: Arc<metamodelica::List<Arc<EquationItem>>>,
        /// elseIfBranches
        elseIfBranches: Arc<metamodelica::List<(Arc<Exp>, Arc<metamodelica::List<Arc<EquationItem>>>)>>,
        /// equationElseItems Standard 2-side eqn
        equationElseItems: Arc<metamodelica::List<Arc<EquationItem>>>,
    },
    EQ_EQUALS {
        /// leftSide
        leftSide: Arc<Exp>,
        /// rightSide Connect stmt
        rightSide: Arc<Exp>,
    },
    EQ_PDE {
        /// leftSide
        leftSide: Arc<Exp>,
        /// rightSide Connect stmt
        rightSide: Arc<Exp>,
        /// domain for PDEs
        domain: Arc<ComponentRef>,
    },
    EQ_CONNECT {
        /// connector1
        connector1: Arc<ComponentRef>,
        /// connector2
        connector2: Arc<ComponentRef>,
    },
    EQ_FOR {
        iterators: ForIterators,
        /// forEquations
        forEquations: Arc<metamodelica::List<Arc<EquationItem>>>,
    },
    EQ_WHEN_E {
        /// whenExp
        whenExp: Arc<Exp>,
        /// whenEquations
        whenEquations: Arc<metamodelica::List<Arc<EquationItem>>>,
        /// elseWhenEquations
        elseWhenEquations: Arc<metamodelica::List<(Arc<Exp>, Arc<metamodelica::List<Arc<EquationItem>>>)>>,
    },
    EQ_NORETCALL {
        /// functionName
        functionName: Arc<ComponentRef>,
        /// functionArgs; fcalls without return value
        functionArgs: Arc<FunctionArgs>,
    },
    EQ_FAILURE {
        equ: Arc<EquationItem>,
    },
}
impl Default for Equation {
    fn default() -> Self {
        Self::EQ_FAILURE {
            equ: Default::default(),
        }
    }
}
pub use self::Equation::{EQ_IF,EQ_EQUALS,EQ_PDE,EQ_CONNECT,EQ_FOR,EQ_WHEN_E,EQ_NORETCALL,EQ_FAILURE};

/// The Algorithm type describes one algorithm statement in an
///  algorithm section.  It does not describe a whole algorithm.  The
///  reason this type is named like this is that the name of the
///  grammar rule for algorithm statements is `algorithm\'.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Algorithm {
    ALG_ASSIGN {
        /// assignComponent
        assignComponent: Arc<Exp>,
        /// value
        value: Arc<Exp>,
    },
    ALG_IF {
        /// ifExp
        ifExp: Arc<Exp>,
        /// trueBranch
        trueBranch: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
        /// elseIfAlgorithmBranch
        elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Exp>, Arc<metamodelica::List<Arc<AlgorithmItem>>>)>>,
        /// elseBranch
        elseBranch: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_FOR {
        iterators: ForIterators,
        /// forBody
        forBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_PARFOR {
        iterators: ForIterators,
        /// parallel for loop Body
        parforBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_WHILE {
        /// boolExpr
        boolExpr: Arc<Exp>,
        /// whileBody
        whileBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_WHEN_A {
        /// boolExpr
        boolExpr: Arc<Exp>,
        /// whenBody
        whenBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
        /// elseWhenAlgorithmBranch
        elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Exp>, Arc<metamodelica::List<Arc<AlgorithmItem>>>)>>,
    },
    ALG_NORETCALL {
        /// functionCall
        functionCall: Arc<ComponentRef>,
        /// functionArgs; general fcalls without return value
        functionArgs: Arc<FunctionArgs>,
    },
    ALG_RETURN,
    ALG_BREAK,
    ALG_FAILURE {
        equ: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_TRY {
        body: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
        elseBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_CONTINUE,
}
impl Default for Algorithm {
    fn default() -> Self { Self::ALG_RETURN }
}
pub use self::Algorithm::{ALG_ASSIGN,ALG_IF,ALG_FOR,ALG_PARFOR,ALG_WHILE,ALG_WHEN_A,ALG_NORETCALL,ALG_RETURN,ALG_BREAK,ALG_FAILURE,ALG_TRY,ALG_CONTINUE};

pub static emptyMod: std::sync::LazyLock<Arc<Modification>> = std::sync::LazyLock::new(|| { Arc::new(Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(crate::Absyn::EqMod::NOMOD) }) });

/// Modifications are described by the `Modification\' type.  There
///  are two forms of modifications: redeclarations and component
///  modifications.
///  - Modifications
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Modification {
    pub elementArgLst: Arc<metamodelica::List<Arc<ElementArg>>>,
    pub eqMod: Arc<EqMod>,
}

impl Default for Modification {
    fn default() -> Self {
        Self {
            elementArgLst: Default::default(),
            eqMod: Default::default(),
        }
    }
}

pub type CLASSMOD = Modification;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EqMod {
    NOMOD,
    EQMOD {
        exp: Arc<Exp>,
        info: Info,
    },
}
impl Default for EqMod {
    fn default() -> Self { Self::NOMOD }
}
pub use self::EqMod::{NOMOD,EQMOD};

/// Wrapper for things that modify elements, modifications and redeclarations
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementArg {
    MODIFICATION {
        /// final prefix
        finalPrefix: bool,
        /// each
        eachPrefix: Each,
        path: Arc<Path>,
        /// modification
        modification: Option<Arc<Modification>>,
        /// comment
        comment: Option<ArcStr>,
        info: Info,
    },
    REDECLARATION {
        /// final prefix
        finalPrefix: bool,
        /// redeclare  or replaceable
        redeclareKeywords: RedeclareKeywords,
        /// each prefix
        eachPrefix: Each,
        /// elementSpec
        elementSpec: Arc<ElementSpec>,
        /// class definition or declaration
        constrainClass: Option<Arc<ConstrainClass>>,
        /// needed because ElementSpec does not contain this info; Element does
        info: Info,
    },
    /// A lexer comment
    ELEMENTARGCOMMENT {
        comment: ArcStr,
    },
    /// break is either an ident or an equation
    ///    we save the ident as connect(ident, break) to keep it simple
    INHERITANCEBREAK {
        cnct: Arc<Equation>,
        info: Info,
    },
}
impl Default for ElementArg {
    fn default() -> Self {
        Self::ELEMENTARGCOMMENT {
            comment: Default::default(),
        }
    }
}
pub use self::ElementArg::{MODIFICATION,REDECLARATION,ELEMENTARGCOMMENT,INHERITANCEBREAK};

/// The keywords redeclare and replacable can be given in three different kombinations, each one by themself or the both combined.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RedeclareKeywords {
    REDECLARE,
    REPLACEABLE,
    REDECLARE_REPLACEABLE,
}
impl Default for RedeclareKeywords {
    fn default() -> Self { Self::REDECLARE }
}
pub use self::RedeclareKeywords::{REDECLARE,REPLACEABLE,REDECLARE_REPLACEABLE};

/// The each keyword can be present in both MODIFICATION\'s and REDECLARATION\'s.
///  - Each attribute
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Each {
    EACH,
    NON_EACH,
}
impl Default for Each {
    fn default() -> Self { Self::EACH }
}
pub use self::Each::{EACH,NON_EACH};

/// Element attributes
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementAttributes {
    /// flow
    pub flowPrefix: bool,
    /// stream
    pub streamPrefix: bool,
    /// for OpenCL/CUDA parglobal, parlocal ...
    pub parallelism: Parallelism,
    /// parameter, constant etc.
    pub variability: Variability,
    /// input/output
    pub direction: Direction,
    /// non-field / field
    pub isField: IsField,
    /// array dimensions
    pub arrayDim: ArrayDim,
}

impl Default for ElementAttributes {
    fn default() -> Self {
        Self {
            flowPrefix: Default::default(),
            streamPrefix: Default::default(),
            parallelism: Default::default(),
            variability: Default::default(),
            direction: Default::default(),
            isField: Default::default(),
            arrayDim: Default::default(),
        }
    }
}

pub type ATTR = ElementAttributes;


/// Is field
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IsField {
    /// variable is not a field
    NONFIELD,
    /// variable is a field
    FIELD,
}
impl Default for IsField {
    fn default() -> Self { Self::NONFIELD }
}
pub use self::IsField::{NONFIELD,FIELD};

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlowStream {
    FLOW,
    STREAM,
    NOT_FLOW_STREAM,
}
impl Default for FlowStream {
    fn default() -> Self { Self::FLOW }
}
pub use self::FlowStream::{FLOW,STREAM,NOT_FLOW_STREAM};

/// Variability
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variability {
    VAR,
    DISCRETE,
    PARAM,
    CONST,
}
impl Default for Variability {
    fn default() -> Self { Self::VAR }
}
pub use self::Variability::{VAR,DISCRETE,PARAM,CONST};

/// Direction
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    /// direction is input
    INPUT,
    /// direction is output
    OUTPUT,
    /// direction is not specified, neither input nor output
    BIDIR,
    /// direction is both input and output (OM extension; syntactic sugar for functions)
    INPUT_OUTPUT,
}
impl Default for Direction {
    fn default() -> Self { Self::INPUT }
}
pub use self::Direction::{INPUT,OUTPUT,BIDIR,INPUT_OUTPUT};

/// The Exp uniontype is the container of a Modelica expression.
///  - Expressions
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exp {
    INTEGER {
        value: i32,
    },
    REAL {
        /// String representation of a Real, in order to unparse without changing the user's display preference
        value: ArcStr,
    },
    CREF {
        componentRef: Arc<ComponentRef>,
    },
    STRING {
        value: ArcStr,
    },
    BOOL {
        value: bool,
    },
    /// Binary operations, e.g. a*b
    BINARY {
        exp1: Arc<Exp>,
        op: Operator,
        exp2: Arc<Exp>,
    },
    /// Unary operations, e.g. -(x), +(x)
    UNARY {
        /// op
        op: Operator,
        /// exp - any arithmetic expression
        exp: Arc<Exp>,
    },
    LBINARY {
        /// exp1
        exp1: Arc<Exp>,
        /// op
        op: Operator,
        exp2: Arc<Exp>,
    },
    /// Logical unary operations: not
    LUNARY {
        /// op
        op: Operator,
        /// exp - any logical or relation expression
        exp: Arc<Exp>,
    },
    RELATION {
        /// exp1
        exp1: Arc<Exp>,
        /// op
        op: Operator,
        exp2: Arc<Exp>,
    },
    /// If expressions
    IFEXP {
        /// ifExp
        ifExp: Arc<Exp>,
        /// trueBranch
        trueBranch: Arc<Exp>,
        /// elseBranch
        elseBranch: Arc<Exp>,
        /// elseIfBranch Function calls
        elseIfBranch: Arc<metamodelica::List<(Arc<Exp>, Arc<Exp>)>>,
    },
    CALL {
        /// function
        function_: Arc<ComponentRef>,
        functionArgs: Arc<FunctionArgs>,
        typeVars: Arc<metamodelica::List<Arc<Path>>>,
    },
    /// Partially evaluated function
    PARTEVALFUNCTION {
        /// function
        function_: Arc<ComponentRef>,
        functionArgs: Arc<FunctionArgs>,
    },
    /// Array construction using {, }, or array
    ARRAY {
        arrayExp: Arc<metamodelica::List<Arc<Exp>>>,
    },
    /// Matrix construction using {, }
    MATRIX {
        matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Exp>>>>>,
    },
    /// Range expressions, e.g. 1:10 or 1:0.5:10
    RANGE {
        /// start
        start: Arc<Exp>,
        /// step
        step: Option<Arc<Exp>>,
        /// stop
        stop: Arc<Exp>,
    },
    /// Tuples used in function calls returning several values
    TUPLE {
        /// comma-separated expressions
        expressions: Arc<metamodelica::List<Arc<Exp>>>,
    },
    /// array access operator for last element, e.g. a{end}:=1;
    END,
    /// Modelica AST Code constructors - OpenModelica extension
    CODE {
        code: Arc<CodeNode>,
    },
    /// as operator
    AS {
        /// only an id
        id: Ident,
        /// expression to bind to the id
        exp: Arc<Exp>,
    },
    /// list cons or :: operator
    CONS {
        /// head of the list
        head: Arc<Exp>,
        /// rest of the list
        rest: Arc<Exp>,
    },
    /// matchcontinue expression
    MATCHEXP {
        /// match or matchcontinue
        matchTy: MatchType,
        /// match expression of
        inputExp: Arc<Exp>,
        /// local declarations
        localDecls: Arc<metamodelica::List<Arc<ElementItem>>>,
        /// case list + else in the end
        cases: Arc<metamodelica::List<Arc<Case>>>,
        /// TODO: Remove this as it was removed from the grammar
        comment: Option<ArcStr>,
    },
    /// Part of MetaModelica extension
    LIST {
        exps: Arc<metamodelica::List<Arc<Exp>>>,
    },
    /// exp.index
    DOT {
        exp: Arc<Exp>,
        index: Arc<Exp>,
    },
    EXPRESSIONCOMMENT {
        commentsBefore: Arc<metamodelica::List<ArcStr>>,
        exp: Arc<Exp>,
        commentsAfter: Arc<metamodelica::List<ArcStr>>,
    },
    SUBSCRIPTED_EXP {
        exp: Arc<Exp>,
        subscripts: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    BREAK,
}
impl Default for Exp {
    fn default() -> Self { Self::END }
}
pub use self::Exp::{INTEGER,REAL,CREF,STRING,BOOL,BINARY,UNARY,LBINARY,LUNARY,RELATION,IFEXP,CALL,PARTEVALFUNCTION,ARRAY,MATRIX,RANGE,TUPLE,END,CODE,AS,CONS,MATCHEXP,LIST,DOT,EXPRESSIONCOMMENT,SUBSCRIPTED_EXP,BREAK};

/// case in match or matchcontinue
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Case {
    CASE {
        /// patterns to be matched
        pattern: Arc<Exp>,
        patternGuard: Option<Arc<Exp>>,
        /// file information of the pattern
        patternInfo: Info,
        /// TODO: Remove this as it was removed from the grammar
        localDecls: Arc<metamodelica::List<Arc<ElementItem>>>,
        /// equation or algorithm section
        classPart: Arc<ClassPart>,
        /// result
        result: Arc<Exp>,
        /// file information of the result-exp
        resultInfo: Info,
        /// TODO: Remove this as it was removed from the grammar
        comment: Option<ArcStr>,
        /// file information of the whole case
        info: Info,
    },
    /// else in match or matchcontinue
    ELSE {
        /// TODO: Remove this as it was removed from the grammar
        localDecls: Arc<metamodelica::List<Arc<ElementItem>>>,
        /// equation or algorithm section
        classPart: Arc<ClassPart>,
        /// result
        result: Arc<Exp>,
        /// file information of the result-exp
        resultInfo: Info,
        /// TODO: Remove this as it was removed from the grammar
        comment: Option<ArcStr>,
        /// file information of the whole case
        info: Info,
    },
}
pub use self::Case::{CASE,ELSE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchType {
    MATCH,
    MATCHCONTINUE,
}
impl Default for MatchType {
    fn default() -> Self { Self::MATCH }
}
pub use self::MatchType::{MATCH,MATCHCONTINUE};

/// The Code uniontype is used for Meta-programming. It originates from the $Code quoting mechanism. See paper in Modelica2003 conference
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodeNode {
    /// Cannot be parsed; used by Static for API calls
    C_TYPENAME {
        path: Arc<Path>,
    },
    /// Cannot be parsed; used by Static for API calls
    C_VARIABLENAME {
        componentRef: Arc<ComponentRef>,
    },
    C_CONSTRAINTSECTION {
        boolean: bool,
        equationItemLst: Arc<metamodelica::List<Arc<EquationItem>>>,
    },
    C_EQUATIONSECTION {
        boolean: bool,
        equationItemLst: Arc<metamodelica::List<Arc<EquationItem>>>,
    },
    C_ALGORITHMSECTION {
        boolean: bool,
        algorithmItemLst: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    C_ELEMENT {
        element: Arc<Element>,
    },
    C_EXPRESSION {
        exp: Arc<Exp>,
    },
    C_MODIFICATION {
        modification: Arc<Modification>,
    },
}
impl Default for CodeNode {
    fn default() -> Self {
        Self::C_EXPRESSION {
            exp: Default::default(),
        }
    }
}
pub use self::CodeNode::{C_TYPENAME,C_VARIABLENAME,C_CONSTRAINTSECTION,C_EQUATIONSECTION,C_ALGORITHMSECTION,C_ELEMENT,C_EXPRESSION,C_MODIFICATION};

/// The FunctionArgs uniontype consists of a list of positional arguments
///  followed by a list of named arguments (Modelica v2.0)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionArgs {
    FUNCTIONARGS {
        /// args
        args: Arc<metamodelica::List<Arc<Exp>>>,
        /// argNames
        argNames: Arc<metamodelica::List<Arc<NamedArg>>>,
    },
    FOR_ITER_FARG {
        /// iterator expression
        exp: Arc<Exp>,
        iterType: ReductionIterType,
        iterators: ForIterators,
    },
}
impl Default for FunctionArgs {
    fn default() -> Self {
        Self::FUNCTIONARGS {
            args: Default::default(),
            argNames: Default::default(),
        }
    }
}
pub use self::FunctionArgs::{FUNCTIONARGS,FOR_ITER_FARG};

pub static emptyFunctionArgs: std::sync::LazyLock<Arc<FunctionArgs>> = std::sync::LazyLock::new(|| { Arc::new(FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReductionIterType {
    /// Reductions are by default calculated as all combinations of the iterators
    COMBINE,
    /// With this option, all iterators must have the same length
    THREAD,
}
impl Default for ReductionIterType {
    fn default() -> Self { Self::COMBINE }
}
pub use self::ReductionIterType::{COMBINE,THREAD};

/// The NamedArg uniontype consist of an Identifier for the argument and an expression
///  giving the value of the argument
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedArg {
    /// argName
    pub argName: Ident,
    /// argValue
    pub argValue: Arc<Exp>,
}

impl Default for NamedArg {
    fn default() -> Self {
        Self {
            argName: Default::default(),
            argValue: Default::default(),
        }
    }
}

pub type NAMEDARG = NamedArg;


/// Expression operators
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operator {
    /// addition
    ADD,
    /// subtraction
    SUB,
    /// multiplication
    MUL,
    /// division
    DIV,
    /// power
    POW,
    /// unary plus
    UPLUS,
    /// unary minus
    UMINUS,
    /// element-wise addition
    ADD_EW,
    /// element-wise subtraction
    SUB_EW,
    /// element-wise multiplication
    MUL_EW,
    /// element-wise division
    DIV_EW,
    /// element-wise power
    POW_EW,
    /// element-wise unary minus
    UPLUS_EW,
    /// element-wise unary plus
    UMINUS_EW,
    /// logical and
    AND,
    /// logical or
    OR,
    /// logical not
    NOT,
    /// less than
    LESS,
    /// less than or equal
    LESSEQ,
    /// greater than
    GREATER,
    /// greater than or equal
    GREATEREQ,
    /// relational equal
    EQUAL,
    /// relational not equal
    NEQUAL,
}
impl Default for Operator {
    fn default() -> Self { Self::ADD }
}
pub use self::Operator::{ADD,SUB,MUL,DIV,POW,UPLUS,UMINUS,ADD_EW,SUB_EW,MUL_EW,DIV_EW,POW_EW,UPLUS_EW,UMINUS_EW,AND,OR,NOT,LESS,LESSEQ,GREATER,GREATEREQ,EQUAL,NEQUAL};

/// The Subscript uniontype is used both in array declarations and
///  component references.  This might seem strange, but it is
///  inherited from the grammar.  The NOSUB constructor means that
///  the dimension size is undefined when used in a declaration, and
///  when it is used in a component reference it means a slice of the
///  whole dimension.
///  - Subscripts
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Subscript {
    /// unknown array dimension
    NOSUB,
    /// dimension as an expression
    SUBSCRIPT {
        /// subscript
        subscript: Arc<Exp>,
    },
}
pub use self::Subscript::{NOSUB,SUBSCRIPT};

/// A component reference is the fully or partially qualified name of
///  a component.  It is represented as a list of
///  identifier--subscript pairs.
///  - Component references and paths
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentRef {
    CREF_FULLYQUALIFIED {
        componentRef: Arc<ComponentRef>,
    },
    CREF_QUAL {
        /// name
        name: Ident,
        /// subscripts
        subscripts: Arc<metamodelica::List<Arc<Subscript>>>,
        /// componentRef
        componentRef: Arc<ComponentRef>,
    },
    CREF_IDENT {
        /// name
        name: Ident,
        /// subscripts
        subscripts: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    WILD,
    ALLWILD,
}
pub use self::ComponentRef::{CREF_FULLYQUALIFIED,CREF_QUAL,CREF_IDENT,WILD,ALLWILD};

/// The type `Path\', on the other hand,
///  is used to store references to class names, or names inside
///  class definitions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Path {
    QUALIFIED {
        /// name
        name: Ident,
        /// path
        path: Arc<Path>,
    },
    IDENT {
        /// name
        name: Ident,
    },
    /// Used during instantiation for names that are fully qualified,
    ///    i.e. the names are looked up from top scope directly like for instance Modelica.SIunits.Voltage
    ///    Note: Not created during parsing, only during instantation to speedup/simplify lookup.
    ///
    FULLYQUALIFIED {
        path: Arc<Path>,
    },
}
pub use self::Path::{QUALIFIED,IDENT,FULLYQUALIFIED};

/// These constructors each correspond to a different kind of class
///  declaration in Modelica, except the last four, which are used
///  for the predefined types.  The parser assigns each class
///  declaration one of the restrictions, and the actual class
///  definition is checked for conformance during translation.  The
///  predefined types are created in the Builtin module and are
///  assigned special restrictions.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Restriction {
    R_CLASS,
    R_OPTIMIZATION,
    R_MODEL,
    R_RECORD,
    R_BLOCK,
    /// connector class
    R_CONNECTOR,
    /// expandable connector class
    R_EXP_CONNECTOR,
    R_TYPE,
    R_PACKAGE,
    R_FUNCTION {
        functionRestriction: FunctionRestriction,
    },
    /// an operator
    R_OPERATOR,
    /// an operator record
    R_OPERATOR_RECORD,
    R_ENUMERATION,
    R_PREDEFINED_INTEGER,
    R_PREDEFINED_REAL,
    R_PREDEFINED_STRING,
    R_PREDEFINED_BOOLEAN,
    R_PREDEFINED_ENUMERATION,
    R_PREDEFINED_CLOCK,
    /// MetaModelica uniontype
    R_UNIONTYPE,
    /// Metamodelica record
    R_METARECORD {
        name: Arc<Path>,
        index: i32,
        singleton: bool,
        moved: bool,
        typeVars: Arc<metamodelica::List<ArcStr>>,
    },
    /// Helper restriction
    R_UNKNOWN,
}
impl Default for Restriction {
    fn default() -> Self { Self::R_CLASS }
}
pub use self::Restriction::{R_CLASS,R_OPTIMIZATION,R_MODEL,R_RECORD,R_BLOCK,R_CONNECTOR,R_EXP_CONNECTOR,R_TYPE,R_PACKAGE,R_FUNCTION,R_OPERATOR,R_OPERATOR_RECORD,R_ENUMERATION,R_PREDEFINED_INTEGER,R_PREDEFINED_REAL,R_PREDEFINED_STRING,R_PREDEFINED_BOOLEAN,R_PREDEFINED_ENUMERATION,R_PREDEFINED_CLOCK,R_UNIONTYPE,R_METARECORD,R_UNKNOWN};

/// function purity
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionPurity {
    PURE,
    IMPURE,
    NO_PURITY,
}
impl Default for FunctionPurity {
    fn default() -> Self { Self::PURE }
}
pub use self::FunctionPurity::{PURE,IMPURE,NO_PURITY};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionRestriction {
    /// a normal function
    FR_NORMAL_FUNCTION {
        /// function purity
        purity: FunctionPurity,
    },
    /// an operator function
    FR_OPERATOR_FUNCTION,
    /// an OpenCL/CUDA parallel/device function
    FR_PARALLEL_FUNCTION,
    /// an OpenCL/CUDA kernel function
    FR_KERNEL_FUNCTION,
}
impl Default for FunctionRestriction {
    fn default() -> Self { Self::FR_OPERATOR_FUNCTION }
}
pub use self::FunctionRestriction::{FR_NORMAL_FUNCTION,FR_OPERATOR_FUNCTION,FR_PARALLEL_FUNCTION,FR_KERNEL_FUNCTION};

/// An Annotation is a class_modification.
///  - Annotation
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Annotation {
    /// elementArgs
    pub elementArgs: Arc<metamodelica::List<Arc<ElementArg>>>,
}

impl Default for Annotation {
    fn default() -> Self {
        Self {
            elementArgs: Default::default(),
        }
    }
}

pub type ANNOTATION = Annotation;


/// Comment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Comment {
    /// annotation
    pub annotation_: Option<Arc<Annotation>>,
    /// comment
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


/// Declaration of an external function call - ExternalDecl
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalDecl {
    /// The name of the external function
    pub funcName: Option<ArcStr>,
    /// Language of the external function
    pub lang: Option<ArcStr>,
    /// output parameter as return value
    pub output_: Option<Arc<ComponentRef>>,
    /// only positional arguments, i.e. expression list
    pub args: Arc<metamodelica::List<Arc<Exp>>>,
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


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ref {
    RCR {
        cr: Arc<ComponentRef>,
    },
    RTS {
        ts: Arc<TypeSpec>,
    },
    RIM {
        im: Import,
    },
}
pub use self::Ref::{RCR,RTS,RIM};

/// Controls output of error-messages
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Msg {
    /// Give error message
    MSG {
        info: Info,
    },
    /// Do not give error message
    NO_MSG,
}
impl Default for Msg {
    fn default() -> Self { Self::NO_MSG }
}
pub use self::Msg::{MSG,NO_MSG};

pub static dummyParts: std::sync::LazyLock<Arc<ClassDef>> = std::sync::LazyLock::new(|| { Arc::new(ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: metamodelica::nil(), ann: metamodelica::nil(), comment: None }) });

pub static dummyInfo: SourceInfo = SourceInfo { fileName: literal!(""), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };

pub static dummyProgram: std::sync::LazyLock<Program> = std::sync::LazyLock::new(|| { Program { classes: metamodelica::nil(), within_: crate::Absyn::Within::TOP } });

