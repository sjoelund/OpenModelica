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

pub type Ident = ArcStr;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForIterator {
    pub name: ArcStr,
    pub guardExp: Option<Arc<Exp>>,
    pub range: Option<Arc<Exp>>,
}

pub type ITERATOR = ForIterator;


pub type ForIterators = Arc<metamodelica::List<Arc<ForIterator>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Program {
    pub classes: Arc<metamodelica::List<Arc<Class>>>,
    pub within_: Within,
}

pub type PROGRAM = Program;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Within {
    WITHIN {
        path: Arc<Path>,
    },
    TOP,
}
pub use self::Within::{WITHIN,TOP};

pub type Info = SourceInfo;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Class {
    pub name: Ident,
    pub partialPrefix: bool,
    pub finalPrefix: bool,
    pub encapsulatedPrefix: bool,
    pub restriction: Restriction,
    pub body: Arc<ClassDef>,
    pub commentsBeforeClass: Arc<metamodelica::List<ArcStr>>,
    pub commentsBeforeEnd: Arc<metamodelica::List<ArcStr>>,
    pub commentsAfterEnd: Arc<metamodelica::List<ArcStr>>,
    pub info: Info,
}

pub type CLASS = Class;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClassDef {
    PARTS {
        typeVars: Arc<metamodelica::List<ArcStr>>,
        classAttrs: Arc<metamodelica::List<Arc<NamedArg>>>,
        classParts: Arc<metamodelica::List<Arc<ClassPart>>>,
        ann: Arc<metamodelica::List<Arc<Annotation>>>,
        comment: Option<ArcStr>,
    },
    DERIVED {
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
        baseClassName: Ident,
        modifications: Arc<metamodelica::List<Arc<ElementArg>>>,
        comment: Option<ArcStr>,
        parts: Arc<metamodelica::List<Arc<ClassPart>>>,
        ann: Arc<metamodelica::List<Arc<Annotation>>>,
    },
    PDER {
        functionName: Arc<Path>,
        vars: Arc<metamodelica::List<ArcStr>>,
        comment: Option<Arc<Comment>>,
    },
}
pub use self::ClassDef::{PARTS,DERIVED,ENUMERATION,OVERLOAD,CLASS_EXTENDS,PDER};

pub type ArrayDim = Arc<metamodelica::List<Arc<Subscript>>>;

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumDef {
    ENUMLITERALS {
        enumLiterals: Arc<metamodelica::List<Arc<EnumLiteral>>>,
    },
    ENUM_COLON,
}
pub use self::EnumDef::{ENUMLITERALS,ENUM_COLON};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumLiteral {
    pub literal: Ident,
    pub comment: Option<Arc<Comment>>,
}

pub type ENUMLITERAL = EnumLiteral;


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
        externalDecl: Arc<ExternalDecl>,
        annotation_: Option<Arc<Annotation>>,
    },
}
pub use self::ClassPart::{PUBLIC,PROTECTED,CONSTRAINTS,EQUATIONS,INITIALEQUATIONS,ALGORITHMS,INITIALALGORITHMS,EXTERNAL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementItem {
    ELEMENTITEM {
        element: Arc<Element>,
    },
    LEXER_COMMENT {
        comment: ArcStr,
    },
}
pub use self::ElementItem::{ELEMENTITEM,LEXER_COMMENT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    ELEMENT {
        finalPrefix: bool,
        redeclareKeywords: Option<RedeclareKeywords>,
        innerOuter: InnerOuter,
        specification: Arc<ElementSpec>,
        info: Info,
        constrainClass: Option<Arc<ConstrainClass>>,
    },
    DEFINEUNIT {
        name: Ident,
        args: Arc<metamodelica::List<Arc<NamedArg>>>,
        info: Info,
    },
    TEXT {
        optName: Option<ArcStr>,
        string: ArcStr,
        info: Info,
    },
}
pub use self::Element::{ELEMENT,DEFINEUNIT,TEXT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstrainClass {
    pub elementSpec: Arc<ElementSpec>,
    pub comment: Option<Arc<Comment>>,
}

pub type CONSTRAINCLASS = ConstrainClass;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementSpec {
    CLASSDEF {
        replaceable_: bool,
        class_: Arc<Class>,
    },
    EXTENDS {
        path: Arc<Path>,
        elementArg: Arc<metamodelica::List<Arc<ElementArg>>>,
        annotationOpt: Option<Arc<Annotation>>,
    },
    IMPORT {
        import_: Import,
        comment: Option<Arc<Comment>>,
        info: Info,
    },
    COMPONENTS {
        attributes: ElementAttributes,
        typeSpec: Arc<TypeSpec>,
        components: Arc<metamodelica::List<Arc<ComponentItem>>>,
    },
}
pub use self::ElementSpec::{CLASSDEF,EXTENDS,IMPORT,COMPONENTS};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InnerOuter {
    INNER,
    OUTER,
    INNER_OUTER,
    NOT_INNER_OUTER,
}
pub use self::InnerOuter::{INNER,OUTER,INNER_OUTER,NOT_INNER_OUTER};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Import {
    NAMED_IMPORT {
        name: Ident,
        path: Arc<Path>,
    },
    QUAL_IMPORT {
        path: Arc<Path>,
    },
    UNQUAL_IMPORT {
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
pub use self::GroupImport::{GROUP_IMPORT_NAME,GROUP_IMPORT_RENAME};

pub type ComponentCondition = Arc<Exp>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentItem {
    pub component: Component,
    pub condition: Option<Arc<Exp>>,
    pub comment: Option<Arc<Comment>>,
}

pub type COMPONENTITEM = ComponentItem;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Component {
    pub name: Ident,
    pub arrayDim: ArrayDim,
    pub modification: Option<Arc<Modification>>,
}

pub type COMPONENT = Component;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquationItem {
    EQUATIONITEM {
        equation_: Arc<Equation>,
        comment: Option<Arc<Comment>>,
        info: Info,
    },
    EQUATIONITEMCOMMENT {
        comment: ArcStr,
    },
}
pub use self::EquationItem::{EQUATIONITEM,EQUATIONITEMCOMMENT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlgorithmItem {
    ALGORITHMITEM {
        algorithm_: Arc<Algorithm>,
        comment: Option<Arc<Comment>>,
        info: Info,
    },
    ALGORITHMITEMCOMMENT {
        comment: ArcStr,
    },
}
pub use self::AlgorithmItem::{ALGORITHMITEM,ALGORITHMITEMCOMMENT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Equation {
    EQ_IF {
        ifExp: Arc<Exp>,
        equationTrueItems: Arc<metamodelica::List<Arc<EquationItem>>>,
        elseIfBranches: Arc<metamodelica::List<(Arc<Exp>, Arc<metamodelica::List<Arc<EquationItem>>>)>>,
        equationElseItems: Arc<metamodelica::List<Arc<EquationItem>>>,
    },
    EQ_EQUALS {
        leftSide: Arc<Exp>,
        rightSide: Arc<Exp>,
    },
    EQ_PDE {
        leftSide: Arc<Exp>,
        rightSide: Arc<Exp>,
        domain: Arc<ComponentRef>,
    },
    EQ_CONNECT {
        connector1: Arc<ComponentRef>,
        connector2: Arc<ComponentRef>,
    },
    EQ_FOR {
        iterators: ForIterators,
        forEquations: Arc<metamodelica::List<Arc<EquationItem>>>,
    },
    EQ_WHEN_E {
        whenExp: Arc<Exp>,
        whenEquations: Arc<metamodelica::List<Arc<EquationItem>>>,
        elseWhenEquations: Arc<metamodelica::List<(Arc<Exp>, Arc<metamodelica::List<Arc<EquationItem>>>)>>,
    },
    EQ_NORETCALL {
        functionName: Arc<ComponentRef>,
        functionArgs: Arc<FunctionArgs>,
    },
    EQ_FAILURE {
        equ: Arc<EquationItem>,
    },
}
pub use self::Equation::{EQ_IF,EQ_EQUALS,EQ_PDE,EQ_CONNECT,EQ_FOR,EQ_WHEN_E,EQ_NORETCALL,EQ_FAILURE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Algorithm {
    ALG_ASSIGN {
        assignComponent: Arc<Exp>,
        value: Arc<Exp>,
    },
    ALG_IF {
        ifExp: Arc<Exp>,
        trueBranch: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
        elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Exp>, Arc<metamodelica::List<Arc<AlgorithmItem>>>)>>,
        elseBranch: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_FOR {
        iterators: ForIterators,
        forBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_PARFOR {
        iterators: ForIterators,
        parforBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_WHILE {
        boolExpr: Arc<Exp>,
        whileBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
    },
    ALG_WHEN_A {
        boolExpr: Arc<Exp>,
        whenBody: Arc<metamodelica::List<Arc<AlgorithmItem>>>,
        elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Exp>, Arc<metamodelica::List<Arc<AlgorithmItem>>>)>>,
    },
    ALG_NORETCALL {
        functionCall: Arc<ComponentRef>,
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
pub use self::Algorithm::{ALG_ASSIGN,ALG_IF,ALG_FOR,ALG_PARFOR,ALG_WHILE,ALG_WHEN_A,ALG_NORETCALL,ALG_RETURN,ALG_BREAK,ALG_FAILURE,ALG_TRY,ALG_CONTINUE};

pub static emptyMod: std::sync::LazyLock<Arc<Modification>> = std::sync::LazyLock::new(|| { Arc::new(Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(crate::Absyn::EqMod::NOMOD) }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Modification {
    pub elementArgLst: Arc<metamodelica::List<Arc<ElementArg>>>,
    pub eqMod: Arc<EqMod>,
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
pub use self::EqMod::{NOMOD,EQMOD};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementArg {
    MODIFICATION {
        finalPrefix: bool,
        eachPrefix: Each,
        path: Arc<Path>,
        modification: Option<Arc<Modification>>,
        comment: Option<ArcStr>,
        info: Info,
    },
    REDECLARATION {
        finalPrefix: bool,
        redeclareKeywords: RedeclareKeywords,
        eachPrefix: Each,
        elementSpec: Arc<ElementSpec>,
        constrainClass: Option<Arc<ConstrainClass>>,
        info: Info,
    },
    ELEMENTARGCOMMENT {
        comment: ArcStr,
    },
    INHERITANCEBREAK {
        cnct: Arc<Equation>,
        info: Info,
    },
}
pub use self::ElementArg::{MODIFICATION,REDECLARATION,ELEMENTARGCOMMENT,INHERITANCEBREAK};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RedeclareKeywords {
    REDECLARE,
    REPLACEABLE,
    REDECLARE_REPLACEABLE,
}
pub use self::RedeclareKeywords::{REDECLARE,REPLACEABLE,REDECLARE_REPLACEABLE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Each {
    EACH,
    NON_EACH,
}
pub use self::Each::{EACH,NON_EACH};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementAttributes {
    pub flowPrefix: bool,
    pub streamPrefix: bool,
    pub parallelism: Parallelism,
    pub variability: Variability,
    pub direction: Direction,
    pub isField: IsField,
    pub arrayDim: ArrayDim,
}

pub type ATTR = ElementAttributes;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IsField {
    NONFIELD,
    FIELD,
}
pub use self::IsField::{NONFIELD,FIELD};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Parallelism {
    PARGLOBAL,
    PARLOCAL,
    NON_PARALLEL,
}
pub use self::Parallelism::{PARGLOBAL,PARLOCAL,NON_PARALLEL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlowStream {
    FLOW,
    STREAM,
    NOT_FLOW_STREAM,
}
pub use self::FlowStream::{FLOW,STREAM,NOT_FLOW_STREAM};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variability {
    VAR,
    DISCRETE,
    PARAM,
    CONST,
}
pub use self::Variability::{VAR,DISCRETE,PARAM,CONST};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    INPUT,
    OUTPUT,
    BIDIR,
    INPUT_OUTPUT,
}
pub use self::Direction::{INPUT,OUTPUT,BIDIR,INPUT_OUTPUT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exp {
    INTEGER {
        value: i32,
    },
    REAL {
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
    BINARY {
        exp1: Arc<Exp>,
        op: Operator,
        exp2: Arc<Exp>,
    },
    UNARY {
        op: Operator,
        exp: Arc<Exp>,
    },
    LBINARY {
        exp1: Arc<Exp>,
        op: Operator,
        exp2: Arc<Exp>,
    },
    LUNARY {
        op: Operator,
        exp: Arc<Exp>,
    },
    RELATION {
        exp1: Arc<Exp>,
        op: Operator,
        exp2: Arc<Exp>,
    },
    IFEXP {
        ifExp: Arc<Exp>,
        trueBranch: Arc<Exp>,
        elseBranch: Arc<Exp>,
        elseIfBranch: Arc<metamodelica::List<(Arc<Exp>, Arc<Exp>)>>,
    },
    CALL {
        function_: Arc<ComponentRef>,
        functionArgs: Arc<FunctionArgs>,
        typeVars: Arc<metamodelica::List<Arc<Path>>>,
    },
    PARTEVALFUNCTION {
        function_: Arc<ComponentRef>,
        functionArgs: Arc<FunctionArgs>,
    },
    ARRAY {
        arrayExp: Arc<metamodelica::List<Arc<Exp>>>,
    },
    MATRIX {
        matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Exp>>>>>,
    },
    RANGE {
        start: Arc<Exp>,
        step: Option<Arc<Exp>>,
        stop: Arc<Exp>,
    },
    TUPLE {
        expressions: Arc<metamodelica::List<Arc<Exp>>>,
    },
    END,
    CODE {
        code: Arc<CodeNode>,
    },
    AS {
        id: Ident,
        exp: Arc<Exp>,
    },
    CONS {
        head: Arc<Exp>,
        rest: Arc<Exp>,
    },
    MATCHEXP {
        matchTy: MatchType,
        inputExp: Arc<Exp>,
        localDecls: Arc<metamodelica::List<Arc<ElementItem>>>,
        cases: Arc<metamodelica::List<Arc<Case>>>,
        comment: Option<ArcStr>,
    },
    LIST {
        exps: Arc<metamodelica::List<Arc<Exp>>>,
    },
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
pub use self::Exp::{INTEGER,REAL,CREF,STRING,BOOL,BINARY,UNARY,LBINARY,LUNARY,RELATION,IFEXP,CALL,PARTEVALFUNCTION,ARRAY,MATRIX,RANGE,TUPLE,END,CODE,AS,CONS,MATCHEXP,LIST,DOT,EXPRESSIONCOMMENT,SUBSCRIPTED_EXP,BREAK};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Case {
    CASE {
        pattern: Arc<Exp>,
        patternGuard: Option<Arc<Exp>>,
        patternInfo: Info,
        localDecls: Arc<metamodelica::List<Arc<ElementItem>>>,
        classPart: Arc<ClassPart>,
        result: Arc<Exp>,
        resultInfo: Info,
        comment: Option<ArcStr>,
        info: Info,
    },
    ELSE {
        localDecls: Arc<metamodelica::List<Arc<ElementItem>>>,
        classPart: Arc<ClassPart>,
        result: Arc<Exp>,
        resultInfo: Info,
        comment: Option<ArcStr>,
        info: Info,
    },
}
pub use self::Case::{CASE,ELSE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchType {
    MATCH,
    MATCHCONTINUE,
}
pub use self::MatchType::{MATCH,MATCHCONTINUE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodeNode {
    C_TYPENAME {
        path: Arc<Path>,
    },
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
pub use self::CodeNode::{C_TYPENAME,C_VARIABLENAME,C_CONSTRAINTSECTION,C_EQUATIONSECTION,C_ALGORITHMSECTION,C_ELEMENT,C_EXPRESSION,C_MODIFICATION};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionArgs {
    FUNCTIONARGS {
        args: Arc<metamodelica::List<Arc<Exp>>>,
        argNames: Arc<metamodelica::List<Arc<NamedArg>>>,
    },
    FOR_ITER_FARG {
        exp: Arc<Exp>,
        iterType: ReductionIterType,
        iterators: ForIterators,
    },
}
pub use self::FunctionArgs::{FUNCTIONARGS,FOR_ITER_FARG};

pub static emptyFunctionArgs: std::sync::LazyLock<Arc<FunctionArgs>> = std::sync::LazyLock::new(|| { Arc::new(FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReductionIterType {
    COMBINE,
    THREAD,
}
pub use self::ReductionIterType::{COMBINE,THREAD};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedArg {
    pub argName: Ident,
    pub argValue: Arc<Exp>,
}

pub type NAMEDARG = NamedArg;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    UPLUS,
    UMINUS,
    ADD_EW,
    SUB_EW,
    MUL_EW,
    DIV_EW,
    POW_EW,
    UPLUS_EW,
    UMINUS_EW,
    AND,
    OR,
    NOT,
    LESS,
    LESSEQ,
    GREATER,
    GREATEREQ,
    EQUAL,
    NEQUAL,
}
pub use self::Operator::{ADD,SUB,MUL,DIV,POW,UPLUS,UMINUS,ADD_EW,SUB_EW,MUL_EW,DIV_EW,POW_EW,UPLUS_EW,UMINUS_EW,AND,OR,NOT,LESS,LESSEQ,GREATER,GREATEREQ,EQUAL,NEQUAL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Subscript {
    NOSUB,
    SUBSCRIPT {
        subscript: Arc<Exp>,
    },
}
pub use self::Subscript::{NOSUB,SUBSCRIPT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentRef {
    CREF_FULLYQUALIFIED {
        componentRef: Arc<ComponentRef>,
    },
    CREF_QUAL {
        name: Ident,
        subscripts: Arc<metamodelica::List<Arc<Subscript>>>,
        componentRef: Arc<ComponentRef>,
    },
    CREF_IDENT {
        name: Ident,
        subscripts: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    WILD,
    ALLWILD,
}
pub use self::ComponentRef::{CREF_FULLYQUALIFIED,CREF_QUAL,CREF_IDENT,WILD,ALLWILD};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Path {
    QUALIFIED {
        name: Ident,
        path: Arc<Path>,
    },
    IDENT {
        name: Ident,
    },
    FULLYQUALIFIED {
        path: Arc<Path>,
    },
}
pub use self::Path::{QUALIFIED,IDENT,FULLYQUALIFIED};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    R_FUNCTION {
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
    R_PREDEFINED_CLOCK,
    R_UNIONTYPE,
    R_METARECORD {
        name: Arc<Path>,
        index: i32,
        singleton: bool,
        moved: bool,
        typeVars: Arc<metamodelica::List<ArcStr>>,
    },
    R_UNKNOWN,
}
pub use self::Restriction::{R_CLASS,R_OPTIMIZATION,R_MODEL,R_RECORD,R_BLOCK,R_CONNECTOR,R_EXP_CONNECTOR,R_TYPE,R_PACKAGE,R_FUNCTION,R_OPERATOR,R_OPERATOR_RECORD,R_ENUMERATION,R_PREDEFINED_INTEGER,R_PREDEFINED_REAL,R_PREDEFINED_STRING,R_PREDEFINED_BOOLEAN,R_PREDEFINED_ENUMERATION,R_PREDEFINED_CLOCK,R_UNIONTYPE,R_METARECORD,R_UNKNOWN};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionPurity {
    PURE,
    IMPURE,
    NO_PURITY,
}
pub use self::FunctionPurity::{PURE,IMPURE,NO_PURITY};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionRestriction {
    FR_NORMAL_FUNCTION {
        purity: FunctionPurity,
    },
    FR_OPERATOR_FUNCTION,
    FR_PARALLEL_FUNCTION,
    FR_KERNEL_FUNCTION,
}
pub use self::FunctionRestriction::{FR_NORMAL_FUNCTION,FR_OPERATOR_FUNCTION,FR_PARALLEL_FUNCTION,FR_KERNEL_FUNCTION};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Annotation {
    pub elementArgs: Arc<metamodelica::List<Arc<ElementArg>>>,
}

pub type ANNOTATION = Annotation;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Comment {
    pub annotation_: Option<Arc<Annotation>>,
    pub comment: Option<ArcStr>,
}

pub type COMMENT = Comment;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalDecl {
    pub funcName: Option<ArcStr>,
    pub lang: Option<ArcStr>,
    pub output_: Option<Arc<ComponentRef>>,
    pub args: Arc<metamodelica::List<Arc<Exp>>>,
    pub annotation_: Option<Arc<Annotation>>,
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Msg {
    MSG {
        info: Info,
    },
    NO_MSG,
}
pub use self::Msg::{MSG,NO_MSG};

pub static dummyParts: std::sync::LazyLock<Arc<ClassDef>> = std::sync::LazyLock::new(|| { Arc::new(ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: metamodelica::nil(), ann: metamodelica::nil(), comment: None }) });

pub static dummyInfo: SourceInfo = SourceInfo { fileName: literal!(""), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };

pub static dummyProgram: std::sync::LazyLock<Program> = std::sync::LazyLock::new(|| { Program { classes: metamodelica::nil(), within_: crate::Absyn::Within::TOP } });

