#![ allow( unused_parens, while_true, non_snake_case, nonstandard_style, unused ) ]
/// Translation of MetaModelica to Rust
///
/// This module provides code generation from Absyn to Rust.

use crate::metamodelica::*;
use std::sync::Arc;

// mod Absyn


/// Forward declarations needed for Rust enum variants

/// Forward declaration of uniontype ForIterator
/// Forward declaration of uniontype Program
/// Forward declaration of uniontype Within
/// Forward declaration of uniontype Class
/// Forward declaration of uniontype ClassDef
/// Forward declaration of uniontype TypeSpec
/// Forward declaration of uniontype EnumDef
/// Forward declaration of uniontype EnumLiteral
/// Forward declaration of uniontype ClassPart
/// Forward declaration of uniontype ElementItem
/// Forward declaration of uniontype Element
/// Forward declaration of uniontype ConstrainClass
/// Forward declaration of uniontype ElementSpec
/// Forward declaration of uniontype InnerOuter
/// Forward declaration of uniontype Import
/// Forward declaration of uniontype GroupImport
/// Forward declaration of uniontype ComponentItem
/// Forward declaration of uniontype Component
/// Forward declaration of uniontype EquationItem
/// Forward declaration of uniontype AlgorithmItem
/// Forward declaration of uniontype Equation
/// Forward declaration of uniontype Algorithm
/// Forward declaration of uniontype Modification
/// Forward declaration of uniontype EqMod
/// Forward declaration of uniontype ElementArg
/// Forward declaration of uniontype RedeclareKeywords
/// Forward declaration of uniontype Each
/// Forward declaration of uniontype ElementAttributes
/// Forward declaration of uniontype IsField
/// Forward declaration of uniontype Parallelism
/// Forward declaration of uniontype FlowStream
/// Forward declaration of uniontype Variability
/// Forward declaration of uniontype Direction
/// Forward declaration of uniontype Exp
/// Forward declaration of uniontype Case
/// Forward declaration of uniontype MatchType
/// Forward declaration of uniontype CodeNode
/// Forward declaration of uniontype FunctionArgs
/// Forward declaration of uniontype ReductionIterType
/// Forward declaration of uniontype NamedArg
/// Forward declaration of uniontype Operator
/// Forward declaration of uniontype Subscript
/// Forward declaration of uniontype ComponentRef
/// Forward declaration of uniontype Path
/// Forward declaration of uniontype Restriction
/// Forward declaration of uniontype FunctionPurity
/// Forward declaration of uniontype FunctionRestriction
/// Forward declaration of uniontype Annotation
/// Forward declaration of uniontype Comment
/// Forward declaration of uniontype ExternalDecl
/// Forward declaration of uniontype Ref
/// Forward declaration of uniontype Msg


    pub type Ident = String/* An identifier, for example a variable name */
    ;
      /* For Iterator - these are used in:
         * for loops where the expression part can be NONE() and then the range
           is taken from an array variable that the iterator is used to index,
           see 3.3.3.2 Several Iterators from Modelica Specification.
         * in array iterators where the expression should always be SOME(Exp),
           see 3.4.4.2 Array constructor with iterators from Specification
         * the guard is a MetaModelica extension; it's a Boolean expression that
           filters out items in the range. */
      /// Uniontype ForIterator
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ForIterator {
             ITERATOR{

                   name: String,
                   guardExp: Option<Arc<Exp>>,
                   range: Option<Arc<Exp>>,
            },
      }

    pub type ForIterators = List<ForIterator>/* For Iterators -
       these are used in:
       * for loops where the expression part can be NONE() and then the range
         is taken from an array variable that the iterator is used to index,
         see 3.3.3.2 Several Iterators from Modelica Specification.
       * in array iterators where the expression should always be SOME(Exp),
         see 3.4.4.2 Array constructor with iterators from Specification */
    ;
      /* - Programs, the top level construct
         A program is simply a list of class definitions declared at top
         level in the source file, combined with a within statement that
         indicates the hieractical position of the program. */
      /// Uniontype Program
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Program {
             PROGRAM{

                   classes: List<Class>/* List of classes */
                   ,
                   within_: Within/* Within clause */
                   ,
            },
      }

      /* Within Clauses */
      /// Uniontype Within
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Within {
             WITHIN{

                   path: Path/* the path for within */
                   ,
            },

             TOP,
      }


    pub type Info = SourceInfo;
      /* A class definition consists of a name, a flag to indicate
        if this class is declared as partial, the declared class restriction,
        and the body of the declaration. */
      /// Uniontype Class
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Class {
             CLASS{

                   name: Ident,
                   partialPrefix: bool/* true if partial */
                   ,
                   finalPrefix: bool/* true if final */
                   ,
                   encapsulatedPrefix: bool/* true if encapsulated */
                   ,
                   restriction: Restriction/* Restriction */
                   ,
                   body: Arc<ClassDef>,
                   commentsBeforeClass: List<String>/* when a class is the first one in the file and has a comment before it */
                   ,
                   commentsBeforeEnd: List<String>/* when a class has comments before its end */
                   ,
                   commentsAfterEnd: List<String>/* when the class has comments after its end, before the next class or the end of the file */
                   ,
                   info: Info/* Information: FileName is the class is defined in +
                                  isReadOnly bool + start line no + start column no +
                                  end line no + end column no */
                   ,
            },
      }

      /* The ClassDef type contains thClasse definition part of a class declaration.
       The definition is either explicit, with a list of parts
       (public, protected, equation, and algorithm), or it is a definition
       derived from another class or an enumeration type.
       For a derived type, the  type contains the name of the derived class
       and an optional array dimension and a list of modifications.
        */
      /// Uniontype ClassDef
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ClassDef {
             PARTS{

                   typeVars: List<String>/* class A<B,C> ... has type variables B,C */
                   ,
                   classAttrs: List<NamedArg>/* optimization Op (objective=...) end Op. A list arguments attributing a
                       class declaration. Currently used only for Optimica extensions */
                   ,
                   classParts: List<ClassPart>,
                   ann: List<Annotation>/* Modelica2 allowed multiple class-annotations */
                   ,
                   comment: Option<String>,
            },

             DERIVED{

                   typeSpec: TypeSpec/* typeSpec specification includes array dimensions */
                   ,
                   attributes: ElementAttributes,
                   arguments: List<Arc<ElementArg>>,
                   comment: Option<Comment>,
            },

             ENUMERATION{

                   enumLiterals: EnumDef,
                   comment: Option<Comment>,
            },

             OVERLOAD{

                   functionNames: List<Path>,
                   comment: Option<Comment>,
            },

             CLASS_EXTENDS{

                   baseClassName: Ident/* name of class to extend */
                   ,
                   modifications: List<Arc<ElementArg>>/* modifications to be applied to the base class */
                   ,
                   comment: Option<String>/* comment */
                   ,
                   parts: List<ClassPart>/* class parts */
                   ,
                   ann: List<Annotation>,
            },

             PDER{

                   functionName: Path,
                   vars: List<Ident>/* derived variables */
                   ,
                   comment: Option<Comment>/* comment */
                   ,
            },
      }

    pub type ArrayDim = List<Subscript>/* Component attributes are
      properties of components which are applied by type prefixes.
      As an example, declaring a component as `input Real x;\' will
      give the attributes `ATTR({},false,VAR,INPUT)\'.
      Components in Modelica can be scalar or arrays with one or more
      dimensions. This type is used to indicate the dimensionality
      of a component or a type definition.
    - Array dimensions */
    ;
      /* ModExtension: new MetaModelica type specification! */
      /// Uniontype TypeSpec
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum TypeSpec {
             TPATH{

                   path: Path,
                   arrayDim: Option<ArrayDim>,
            },

             TCOMPLEX{

                   path: Path,
                   typeSpecs: List<Arc<TypeSpec>>,
                   arrayDim: Option<ArrayDim>,
            },
      }

      /* The definition of an enumeration is either a list of literals
           or a colon, \':\', which defines a supertype of all enumerations */
      /// Uniontype EnumDef
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum EnumDef {
             ENUMLITERALS{

                   enumLiterals: List<EnumLiteral>,
            },

             ENUM_COLON,
      }

      /* EnumLiteral, which is a name in an enumeration and an optional
         Comment. */
      /// Uniontype EnumLiteral
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum EnumLiteral {
             ENUMLITERAL{

                   literal: Ident,
                   comment: Option<Comment>,
            },
      }

      /* A class definition contains several parts.  There are public and
        protected component declarations, type definitions and `extends\'
        clauses, collectively called elements.  There are also equation
        sections and algorithm sections. The EXTERNAL part is used only by functions
        which can be declared as external C or FORTRAN functions. */
      /// Uniontype ClassPart
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ClassPart {
             PUBLIC{

                   contents: List<ElementItem>,
            },

             PROTECTED{

                   contents: List<ElementItem>,
            },

             CONSTRAINTS{

                   contents: List<Arc<Exp>>,
            },

             EQUATIONS{

                   contents: List<EquationItem>,
            },

             INITIALEQUATIONS{

                   contents: List<EquationItem>,
            },

             ALGORITHMS{

                   contents: List<AlgorithmItem>,
            },

             INITIALALGORITHMS{

                   contents: List<AlgorithmItem>,
            },

             EXTERNAL{

                   externalDecl: ExternalDecl/* externalDecl */
                   ,
                   annotation_: Option<Annotation>/* annotation */
                   ,
            },
      }

      /* An element item is either an element or an annotation */
      /// Uniontype ElementItem
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ElementItem {
             ELEMENTITEM{

                   element: Element,
            },

             LEXER_COMMENT{

                   comment: String,
            },
      }

      /* Elements
        The basic element type in Modelica */
      /// Uniontype Element
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Element {
             ELEMENT{

                   finalPrefix: bool,
                   redeclareKeywords: Option<RedeclareKeywords>/* replaceable, redeclare */
                   ,
                   innerOuter: InnerOuter/* inner/outer */
                   ,
                   specification: ElementSpec/* Actual element specification */
                   ,
                   info: Info/* File name the class is defined in + line no + column no */
                   ,
                   constrainClass: Option<ConstrainClass>/* only valid for classdef and component */
                   ,
            },

             DEFINEUNIT{

                   name: Ident,
                   args: List<NamedArg>,
                   info: Info,
            },

             TEXT{

                   optName: Option<Ident>/* optName : optional name of text, e.g. model with syntax error.
                                                          We need the name to be able to browse it... */
                   ,
                   string: String,
                   info: Info,
            },
      }

      /* Constraining type, must be extends */
      /// Uniontype ConstrainClass
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ConstrainClass {
             CONSTRAINCLASS{

                   elementSpec: ElementSpec/* must be extends */
                   ,
                   comment: Option<Comment>/* comment */
                   ,
            },
      }

      /* An element is something that occurs in a public or protected
          section in a class definition.  There is one constructor in the
          `ElementSpec\' type for each possible element type.  There are
          class definitions (`CLASSDEF\'), `extends\' clauses (`EXTENDS\')
          and component declarations (`COMPONENTS\').

          As an example, if the element `extends TwoPin;\' appears
          in the source, it is represented in the AST as
          `EXTENDS(IDENT(\"TwoPin\"),{})\'.
       */
      /// Uniontype ElementSpec
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ElementSpec {
             CLASSDEF{

                   replaceable_: bool/* replaceable */
                   ,
                   class_: Arc<Class>/* class */
                   ,
            },

             EXTENDS{

                   path: Path/* path */
                   ,
                   elementArg: List<Arc<ElementArg>>/* elementArg */
                   ,
                   annotationOpt: Option<Annotation>/* optional annotation */
                   ,
            },

             IMPORT{

                   import_: Import/* import */
                   ,
                   comment: Option<Comment>/* comment */
                   ,
                   info: Info,
            },

             COMPONENTS{

                   attributes: ElementAttributes/* attributes */
                   ,
                   typeSpec: TypeSpec/* typeSpec */
                   ,
                   components: List<Arc<ComponentItem>>/* components */
                   ,
            },
      }

      /* One of the keyword inner and outer CAN be given to reference an
         inner or outer element. Thus there are three disjoint possibilities. */
      /// Uniontype InnerOuter
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum InnerOuter {
             INNER,
             OUTER,
             INNER_OUTER,
             NOT_INNER_OUTER,
      }

      /* Import statements, different kinds */
      /// Uniontype Import
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Import {
            // A named import is a import statement to a variable ex;

            // NAMED_IMPORT("SI",QUALIFIED("Modelica",IDENT("SIunits")));


             NAMED_IMPORT{

                   name: Ident/* name */
                   ,
                   path: Path/* path */
                   ,
            },

             QUAL_IMPORT{

                   path: Path/* path */
                   ,
            },

             UNQUAL_IMPORT{

                   path: Path/* path */
                   ,
            },

             GROUP_IMPORT{

                   prefix: Path,
                   groups: List<GroupImport>,
            },
      }

      /// Uniontype GroupImport
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum GroupImport {
             GROUP_IMPORT_NAME{

                   name: String,
            },

             GROUP_IMPORT_RENAME{

                   rename: String,
                   name: String,
            },
      }

    pub type ComponentCondition = Exp/* A componentItem can have a condition that must be fulfilled if
      the component should be instantiated.
     */
    ;
      /* Collection of component and an optional comment */
      /// Uniontype ComponentItem
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ComponentItem {
             COMPONENTITEM{

                   component: Component/* component */
                   ,
                   condition: Option<ComponentCondition>/* condition */
                   ,
                   comment: Option<Comment>/* comment */
                   ,
            },
      }

      /* Some kind of Modelica entity (object or variable) */
      /// Uniontype Component
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Component {
             COMPONENT{

                   name: Ident/* name */
                   ,
                   arrayDim: ArrayDim/* Array dimensions, if any */
                   ,
                   modification: Option<Modification>/* Optional modification */
                   ,
            },
      }

      /* Several component declarations can be grouped together in one
        `ElementSpec\' by writing them on the same line in the source.
        This type contains the information specific to one component. */
      /// Uniontype EquationItem
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum EquationItem {
             EQUATIONITEM{

                   equation_: Arc<Equation>/* equation */
                   ,
                   comment: Option<Comment>/* comment */
                   ,
                   info: Info/* line number */
                   ,
            },

             EQUATIONITEMCOMMENT{

                   comment: String,
            },
      }

      /* Info specific for an algorithm item. */
      /// Uniontype AlgorithmItem
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum AlgorithmItem {
             ALGORITHMITEM{

                   algorithm_: Arc<Algorithm>/* algorithm */
                   ,
                   comment: Option<Comment>/* comment */
                   ,
                   info: Info/* line number */
                   ,
            },

             ALGORITHMITEMCOMMENT{

                   comment: String,
            },
      }

      /* Information on one (kind) of equation, different constructors for different
           kinds of equations */
      /// Uniontype Equation
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Equation {
             EQ_IF{

                   ifExp: Exp/* Conditional expression */
                   ,
                   equationTrueItems: List<Arc<EquationItem>>/* true branch */
                   ,
                   elseIfBranches: List<(Exp, List<Arc<EquationItem>>)>/* elseIfBranches */
                   ,
                   equationElseItems: List<Arc<EquationItem>>/* equationElseItems Standard 2-side eqn */
                   ,
            },

             EQ_EQUALS{

                   leftSide: Exp/* leftSide */
                   ,
                   rightSide: Exp/* rightSide Connect stmt */
                   ,
            },

             EQ_PDE{

                   leftSide: Exp/* leftSide */
                   ,
                   rightSide: Exp/* rightSide Connect stmt */
                   ,
                   domain: ComponentRef/* domain for PDEs */
                   ,
            },

             EQ_CONNECT{

                   connector1: ComponentRef/* connector1 */
                   ,
                   connector2: ComponentRef/* connector2 */
                   ,
            },

             EQ_FOR{

                   iterators: ForIterators,
                   forEquations: List<Arc<EquationItem>>/* forEquations */
                   ,
            },

             EQ_WHEN_E{

                   whenExp: Exp/* whenExp */
                   ,
                   whenEquations: List<Arc<EquationItem>>/* whenEquations */
                   ,
                   elseWhenEquations: List<(Exp, List<Arc<EquationItem>>)>/* elseWhenEquations */
                   ,
            },

             EQ_NORETCALL{

                   functionName: ComponentRef/* functionName */
                   ,
                   functionArgs: FunctionArgs/* functionArgs; fcalls without return value */
                   ,
            },

             EQ_FAILURE{

                   equ: EquationItem,
            },
      }

      /* The Algorithm type describes one algorithm statement in an
        algorithm section.  It does not describe a whole algorithm.  The
        reason this type is named like this is that the name of the
        grammar rule for algorithm statements is `algorithm\'. */
      /// Uniontype Algorithm
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Algorithm {
             ALG_ASSIGN{

                   assignComponent: Exp/* assignComponent */
                   ,
                   value: Exp/* value */
                   ,
            },

             ALG_IF{

                   ifExp: Exp/* ifExp */
                   ,
                   trueBranch: List<AlgorithmItem>/* trueBranch */
                   ,
                   elseIfAlgorithmBranch: List<(Exp, List<AlgorithmItem>)>/* elseIfAlgorithmBranch */
                   ,
                   elseBranch: List<AlgorithmItem>/* elseBranch */
                   ,
            },

             ALG_FOR{

                   iterators: ForIterators,
                   forBody: List<AlgorithmItem>/* forBody */
                   ,
            },

             ALG_PARFOR{

                   iterators: ForIterators,
                   parforBody: List<AlgorithmItem>/* parallel for loop Body */
                   ,
            },

             ALG_WHILE{

                   boolExpr: Exp/* boolExpr */
                   ,
                   whileBody: List<AlgorithmItem>/* whileBody */
                   ,
            },

             ALG_WHEN_A{

                   boolExpr: Exp/* boolExpr */
                   ,
                   whenBody: List<AlgorithmItem>/* whenBody */
                   ,
                   elseWhenAlgorithmBranch: List<(Exp, List<AlgorithmItem>)>/* elseWhenAlgorithmBranch */
                   ,
            },

             ALG_NORETCALL{

                   functionCall: ComponentRef/* functionCall */
                   ,
                   functionArgs: FunctionArgs/* functionArgs; general fcalls without return value */
                   ,
            },

             ALG_RETURN,

             ALG_BREAK,

            // MetaModelica extensions


             ALG_FAILURE{

                   equ: List<AlgorithmItem>,
            },

             ALG_TRY{

                   body: List<AlgorithmItem>,
                   elseBody: List<AlgorithmItem>,
            },

             ALG_CONTINUE,
      }

     const emptyMod: Modification = Modification::CLASSMOD{elementArgLst: List::Nil(), eqMod: EqMod::NOMOD{}};

      /* Modifications are described by the `Modification\' type.  There
        are two forms of modifications: redeclarations and component
        modifications.
        - Modifications */
      /// Uniontype Modification
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Modification {
             CLASSMOD{

                   elementArgLst: List<Arc<ElementArg>>,
                   eqMod: EqMod,
            },
      }

      /// Uniontype EqMod
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum EqMod {
             NOMOD,

             EQMOD{

                   exp: Arc<Exp>,
                   info: Info,
            },
      }

      /* Wrapper for things that modify elements, modifications and redeclarations */
      /// Uniontype ElementArg
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ElementArg {
             MODIFICATION{

                   finalPrefix: bool/* final prefix */
                   ,
                   eachPrefix: Each/* each */
                   ,
                   path: Path,
                   modification: Option<Modification>/* modification */
                   ,
                   comment: Option<String>/* comment */
                   ,
                   info: Info,
            },

             REDECLARATION{

                   finalPrefix: bool/* final prefix */
                   ,
                   redeclareKeywords: RedeclareKeywords/* redeclare  or replaceable  */
                   ,
                   eachPrefix: Each/* each prefix */
                   ,
                   elementSpec: ElementSpec/* elementSpec */
                   ,
                   constrainClass: Option<ConstrainClass>/* class definition or declaration */
                   ,
                   info: Info/* needed because ElementSpec does not contain this info; Element does */
                   ,
            },

             ELEMENTARGCOMMENT{

                   comment: String,
            },

             INHERITANCEBREAK{

                   cnct: Equation,
                   info: Info,
            },
      }

      /* The keywords redeclare and replacable can be given in three different kombinations, each one by themself or the both combined. */
      /// Uniontype RedeclareKeywords
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum RedeclareKeywords {
             REDECLARE,
             REPLACEABLE,
             REDECLARE_REPLACEABLE,
      }

      /* The each keyword can be present in both MODIFICATION\'s and REDECLARATION\'s.
        - Each attribute */
      /// Uniontype Each
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Each {
             EACH,
             NON_EACH,
      }

      /* Element attributes */
      /// Uniontype ElementAttributes
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ElementAttributes {
             ATTR{

                   flowPrefix: bool/* flow */
                   ,
                   streamPrefix: bool/* stream */
                   ,
                   parallelism: Parallelism/* for OpenCL/CUDA parglobal, parlocal ... */
                   ,
                   variability: Variability/* parameter, constant etc. */
                   ,
                   direction: Direction/* input/output */
                   ,
                   isField: IsField/* non-field / field */
                   ,
                   arrayDim: ArrayDim/* array dimensions */
                   ,
            },
      }

      /* Is field */
      /// Uniontype IsField
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum IsField {
             NONFIELD,
             FIELD,
      }

      /* Parallelism */
      /// Uniontype Parallelism
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Parallelism {
             PARGLOBAL,
             PARLOCAL,
             NON_PARALLEL,
      }

      /// Uniontype FlowStream
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum FlowStream {
             FLOW,
             STREAM,
             NOT_FLOW_STREAM,
      }

      /* Variability */
      /// Uniontype Variability
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Variability {
             VAR,
             DISCRETE,
             PARAM,
             CONST,
      }

      /* Direction */
      /// Uniontype Direction
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Direction {
             INPUT,
             OUTPUT,
             BIDIR,
             INPUT_OUTPUT,
      }

      /* The Exp uniontype is the container of a Modelica expression.
        - Expressions */
      /// Uniontype Exp
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Exp {
             INTEGER{

                   value: i32,
            },

             REAL{

                   value: String/* String representation of a Real, in order to unparse without changing the user's display preference */
                   ,
            },

             CREF{

                   componentRef: Arc<ComponentRef>,
            },

             STRING{

                   value: String,
            },

             BOOL{

                   value: bool,
            },

             BINARY{

                   exp1: Arc<Exp>,
                   op: Operator,
                   exp2: Arc<Exp>,
            },

             UNARY{

                   op: Operator/* op */
                   ,
                   exp: Arc<Exp>/* exp - any arithmetic expression */
                   ,
            },

             LBINARY{

                   exp1: Arc<Exp>/* exp1 */
                   ,
                   op: Operator/* op */
                   ,
                   exp2: Arc<Exp>,
            },

             LUNARY{

                   op: Operator/* op */
                   ,
                   exp: Arc<Exp>/* exp - any logical or relation expression */
                   ,
            },

             RELATION{

                   exp1: Arc<Exp>/* exp1 */
                   ,
                   op: Operator/* op */
                   ,
                   exp2: Arc<Exp>,
            },

             IFEXP{

                   ifExp: Arc<Exp>/* ifExp */
                   ,
                   trueBranch: Arc<Exp>/* trueBranch */
                   ,
                   elseBranch: Arc<Exp>/* elseBranch */
                   ,
                   elseIfBranch: List<(Arc<Exp>, Arc<Exp>)>/* elseIfBranch Function calls */
                   ,
            },

             CALL{

                   function_: Arc<ComponentRef>/* function */
                   ,
                   functionArgs: FunctionArgs,
                   typeVars: List<Path>,
            },

            // stefan


             PARTEVALFUNCTION{

                   function_: Arc<ComponentRef>/* function */
                   ,
                   functionArgs: FunctionArgs,
            },

             ARRAY{

                   arrayExp: List<Arc<Exp>>,
            },

             MATRIX{

                   matrix: List<List<Arc<Exp>>>,
            },

             RANGE{

                   start: Arc<Exp>/* start */
                   ,
                   step: Option<Arc<Exp>>/* step */
                   ,
                   stop: Arc<Exp>/* stop */
                   ,
            },

             TUPLE{

                   expressions: List<Arc<Exp>>/* comma-separated expressions */
                   ,
            },

            END,

             CODE{

                   code: CodeNode,
            },

            // MetaModelica expressions follow below!


             AS{

                   id: Ident/*  only an id  */
                   ,
                   exp: Arc<Exp>/*  expression to bind to the id  */
                   ,
            },

             CONS{

                   head: Arc<Exp>/*  head of the list  */
                   ,
                   rest: Arc<Exp>/*  rest of the list  */
                   ,
            },

             MATCHEXP{

                   matchTy: MatchType/*  match or matchcontinue       */
                   ,
                   inputExp: Arc<Exp>/*  match expression of          */
                   ,
                   localDecls: List<Arc<ElementItem>>/*  local declarations           */
                   ,
                   cases: List<Case>/*  case list + else in the end  */
                   ,
                   comment: Option<String>/*  match expr comment_optional  */
                   ,
            },

            // The following are only used internally in the compiler


             LIST{

                   exps: List<Arc<Exp>>,
            },

             DOT{

                   exp: Arc<Exp>,
                   index: Arc<Exp>,
            },

             EXPRESSIONCOMMENT{

                   commentsBefore: List<String>,
                   exp: Arc<Exp>,
                   commentsAfter: List<String>,
            },

             SUBSCRIPTED_EXP{

                   exp: Arc<Exp>,
                   subscripts: List<Arc<Subscript>>,
            },

            BREAK,
      }


      /* case in match or matchcontinue */
      /// Uniontype Case
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Case {
             CASE{

                   pattern: Arc<Exp>/*  patterns to be matched  */
                   ,
                   patternGuard: Option<Arc<Exp>>,
                   patternInfo: Info/* file information of the pattern */
                   ,
                   localDecls: List<Arc<ElementItem>>/*  local decls  */
                   ,
                   classPart: ClassPart/*  equation or algorithm section  */
                   ,
                   result: Arc<Exp>/*  result  */
                   ,
                   resultInfo: Info/* file information of the result-exp */
                   ,
                   comment: Option<String>/*  comment after case like: case pattern string_comment  */
                   ,
                   info: Info/* file information of the whole case */
                   ,
            },

             ELSE{

                   localDecls: List<Arc<ElementItem>>/*  local decls  */
                   ,
                   classPart: ClassPart/*  equation or algorithm section  */
                   ,
                   result: Arc<Exp>/*  result  */
                   ,
                   resultInfo: Info/* file information of the result-exp */
                   ,
                   comment: Option<String>/*  comment after case like: case pattern string_comment  */
                   ,
                   info: Info/* file information of the whole case */
                   ,
            },
      }


      /// Uniontype MatchType
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum MatchType {
             MATCH,
             MATCHCONTINUE,
      }


      /* The Code uniontype is used for Meta-programming. It originates from the $Code quoting mechanism. See paper in Modelica2003 conference */
      /// Uniontype CodeNode
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum CodeNode {
             C_TYPENAME{

                   path: Path,
            },

             C_VARIABLENAME{

                   componentRef: Arc<ComponentRef>,
            },

             C_CONSTRAINTSECTION{

                   boolean: bool,
                   equationItemLst: List<Arc<EquationItem>>,
            },

             C_EQUATIONSECTION{

                   boolean: bool,
                   equationItemLst: List<Arc<EquationItem>>,
            },

             C_ALGORITHMSECTION{

                   boolean: bool,
                   algorithmItemLst: List<Arc<AlgorithmItem>>,
            },

             C_ELEMENT{

                   element: Element,
            },

             C_EXPRESSION{
                   exp: Arc<Exp>,
            },

             C_MODIFICATION{

                   modification: Modification,
            },
      }


      /* The FunctionArgs uniontype consists of a list of positional arguments
        followed by a list of named arguments (Modelica v2.0) */
      /// Uniontype FunctionArgs
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum FunctionArgs {
             FUNCTIONARGS{

                   args: List<Arc<Exp>>/* args */
                   ,
                   argNames: List<Arc<NamedArg>>/* argNames */
                   ,
            },

             FOR_ITER_FARG{

                   exp: Arc<Exp>/* iterator expression */
                   ,
                   iterType: ReductionIterType,
                   iterators: ForIterators,
            },
      }


      // const emptyFunctionArgs: FunctionArgs = FUNCTIONARGS{};

      /// Uniontype ReductionIterType
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ReductionIterType {
             COMBINE,
             THREAD,
      }


      /* The NamedArg uniontype consist of an Identifier for the argument and an expression
        giving the value of the argument */
      /// Uniontype NamedArg
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum NamedArg {
             NAMEDARG{

                   argName: Ident/* argName */
                   ,
                   argValue: Arc<Exp>/* argValue */
                   ,
            },
      }


      /* Expression operators */
      /// Uniontype Operator
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Operator {
            /* arithmetic operators */
             ADD,
             SUB,
             MUL,
             DIV,
             POW,
             UPLUS,
             UMINUS,
            /* element-wise arithmetic operators */
             ADD_EW,
             SUB_EW,
             MUL_EW,
             DIV_EW,
             POW_EW,
             UPLUS_EW,
             UMINUS_EW,

            /* logical operators */

             AND,
             OR,
             NOT,

            /* relational operators */

             LESS,
             LESSEQ,
             GREATER,

             GREATEREQ,
             EQUAL,
             NEQUAL,
      }


      /* The Subscript uniontype is used both in array declarations and
        component references.  This might seem strange, but it is
        inherited from the grammar.  The NOSUB constructor means that
        the dimension size is undefined when used in a declaration, and
        when it is used in a component reference it means a slice of the
        whole dimension.
        - Subscripts */
      /// Uniontype Subscript
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Subscript {
             NOSUB,

             SUBSCRIPT{

                   subscript: Arc<Exp>/* subscript */
                   ,
            },
      }


      /* A component reference is the fully or partially qualified name of
        a component.  It is represented as a list of
        identifier--subscript pairs.
        - Component references and paths */
      /// Uniontype ComponentRef
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ComponentRef {
             CREF_FULLYQUALIFIED{

                   componentRef: Arc<ComponentRef>,
            },

             CREF_QUAL{

                   name: Ident/* name */
                   ,
                   subscripts: List<Arc<Subscript>>/* subscripts */
                   ,
                   componentRef: Arc<ComponentRef>/* componentRef */
                   ,
            },

             CREF_IDENT{

                   name: Ident/* name */
                   ,
                   subscripts: List<Arc<Subscript>>/* subscripts */
                   ,
            },

             WILD,
             ALLWILD,
      }


      /* The type `Path\', on the other hand,
        is used to store references to class names, or names inside
        class definitions. */
      /// Uniontype Path
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Path {
             QUALIFIED{

                   name: Ident/* name */
                   ,
                   path: Arc<Path>/* path */
                   ,
            },

             IDENT{

                   name: Ident/* name */
                   ,
            },

             FULLYQUALIFIED{

                   path: Arc<Path>,
            },
      }


      /* These constructors each correspond to a different kind of class
        declaration in Modelica, except the last four, which are used
        for the predefined types.  The parser assigns each class
        declaration one of the restrictions, and the actual class
        definition is checked for conformance during translation.  The
        predefined types are created in the Builtin module and are
        assigned special restrictions.
        */
      /// Uniontype Restriction
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Restriction {
             R_CLASS,
             R_OPTIMIZATION,
             R_MODEL,
             R_RECORD,
             R_BLOCK,
             R_CONNECTOR,
             R_EXP_CONNECTOR,
             R_TYPE,
             R_PACKAGE,
             R_FUNCTION{

                   functionRestriction: FunctionRestriction,
            },
             R_OPERATOR,
             R_OPERATOR_RECORD,
             R_ENUMERATION,
             R_PREDEFINED_INTEGER,
             R_PREDEFINED_REAL,
             R_PREDEFINED_STRING,
             R_PREDEFINED_BOOLEAN,
             R_PREDEFINED_ENUMERATION,

            // BTH


             R_PREDEFINED_CLOCK,

            // MetaModelica


             R_UNIONTYPE,
             R_METARECORD{

                  //MetaModelica extension, added by simbj

                   name: Path,
                  //Name of the uniontype

                   index: i32,
                  //Index in the uniontype

                   singleton: bool,
                   moved: bool,
                  // true if moved outside uniontype, otherwise false.

                   typeVars: List<String>,
            },

             R_UNKNOWN,

            /* added by simbj */
      }

      /* function purity */
      /// Uniontype FunctionPurity
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum FunctionPurity {
             PURE,
             IMPURE,
             NO_PURITY,
      }

      /// Uniontype FunctionRestriction
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum FunctionRestriction {
             FR_NORMAL_FUNCTION{

                   purity: FunctionPurity/* function purity */
                   ,
            },

             FR_OPERATOR_FUNCTION,
             FR_PARALLEL_FUNCTION,
             FR_KERNEL_FUNCTION,
      }

      /* An Annotation is a class_modification.
        - Annotation */
      /// Uniontype Annotation
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Annotation {
             ANNOTATION{

                   elementArgs: List<Arc<ElementArg>>/* elementArgs */
                   ,
            },
      }

      /* Comment */
      /// Uniontype Comment
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Comment {
             COMMENT{

                   annotation_: Option<Annotation>/* annotation */
                   ,
                   comment: Option<String>/* comment */
                   ,
            },
      }

      /* Declaration of an external function call - ExternalDecl */
      /// Uniontype ExternalDecl
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum ExternalDecl {
             EXTERNALDECL{

                   funcName: Option<Ident>/* The name of the external function */
                   ,
                   lang: Option<String>/* Language of the external function */
                   ,
                   output_: Option<ComponentRef>/* output parameter as return value */
                   ,
                   args: List<Arc<Exp>>/* only positional arguments, i.e. expression list */
                   ,
                   annotation_: Option<Annotation>,
            },
      }

      /// Uniontype Ref
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Ref {
             RCR{

                   cr: ComponentRef,
            },

             RTS{

                   ts: TypeSpec,
            },

             RIM{

                   im: Import,
            },
      }

      /* Controls output of error-messages */
      /// Uniontype Msg
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum Msg {
             MSG{

                   info: Info,
            },

             NO_MSG,
      }
