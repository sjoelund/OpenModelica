// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_ast::Absyn;

#[derive(Clone, Debug, PartialEq)]
pub struct AlgorithmSection {
    pub statements: Arc<metamodelica::List<Arc<Statement>>>,
}

pub type ALGORITHM = AlgorithmSection;


#[derive(Clone, Debug, PartialEq)]
pub struct Annotation {
    pub modification: Arc<Mod>,
}

pub type ANNOTATION = Annotation;


#[derive(Clone, Debug, PartialEq)]
pub struct Attributes {
    pub arrayDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>,
    pub connectorType: ConnectorType,
    pub parallelism: Parallelism,
    pub variability: Variability,
    pub direction: Absyn::Direction,
    pub isField: Absyn::IsField,
}

pub type ATTR = Attributes;


#[derive(Clone, Debug, PartialEq)]
pub enum ClassDef {
    PARTS {
        elementLst: Arc<metamodelica::List<Arc<Element>>>,
        normalEquationLst: Arc<metamodelica::List<Arc<Equation>>>,
        initialEquationLst: Arc<metamodelica::List<Arc<Equation>>>,
        normalAlgorithmLst: Arc<metamodelica::List<Arc<AlgorithmSection>>>,
        initialAlgorithmLst: Arc<metamodelica::List<Arc<AlgorithmSection>>>,
        constraintLst: Arc<metamodelica::List<ConstraintSection>>,
        clsattrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>,
        externalDecl: Option<Arc<ExternalDecl>>,
    },
    CLASS_EXTENDS {
        modifications: Arc<Mod>,
        composition: Arc<ClassDef>,
    },
    DERIVED {
        typeSpec: Arc<Absyn::TypeSpec>,
        modifications: Arc<Mod>,
        attributes: Attributes,
    },
    ENUMERATION {
        enumLst: Arc<metamodelica::List<Arc<Enum>>>,
    },
    OVERLOAD {
        pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>,
    },
    PDER {
        functionPath: Arc<Absyn::Path>,
        derivedVariables: Arc<metamodelica::List<ArcStr>>,
    },
}
pub use self::ClassDef::{PARTS,CLASS_EXTENDS,DERIVED,ENUMERATION,OVERLOAD,PDER};

#[derive(Clone, Debug, PartialEq)]
pub struct Comment {
    pub annotation_: Option<Arc<Annotation>>,
    pub comment: Option<ArcStr>,
}

pub type COMMENT = Comment;


#[derive(Clone, Debug, PartialEq)]
pub enum ConnectorType {
    POTENTIAL,
    FLOW,
    STREAM,
}
pub use self::ConnectorType::{POTENTIAL,FLOW,STREAM};

#[derive(Clone, Debug, PartialEq)]
pub struct ConstrainClass {
    pub constrainingClass: Arc<Absyn::Path>,
    pub modifier: Arc<Mod>,
    pub comment: Arc<Comment>,
}

pub type CONSTRAINCLASS = ConstrainClass;


#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintSection {
    pub constraints: Arc<metamodelica::List<Arc<Absyn::Exp>>>,
}

pub type CONSTRAINTS = ConstraintSection;


#[derive(Clone, Debug, PartialEq)]
pub enum Each {
    EACH,
    NOT_EACH,
}
pub use self::Each::{EACH,NOT_EACH};

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    IMPORT {
        imp: Absyn::Import,
        visibility: Visibility,
        info: SourceInfo,
    },
    EXTENDS {
        baseClassPath: Path,
        visibility: Visibility,
        modifications: Arc<Mod>,
        ann: Option<Arc<Annotation>>,
        info: SourceInfo,
    },
    CLASS {
        name: Ident,
        prefixes: Arc<Prefixes>,
        encapsulatedPrefix: Encapsulated,
        partialPrefix: Partial,
        restriction: Restriction,
        classDef: Arc<ClassDef>,
        cmt: Arc<Comment>,
        info: SourceInfo,
    },
    COMPONENT {
        name: Ident,
        prefixes: Arc<Prefixes>,
        attributes: Attributes,
        typeSpec: Arc<Absyn::TypeSpec>,
        modifications: Arc<Mod>,
        comment: Arc<Comment>,
        condition: Option<Arc<Absyn::Exp>>,
        info: SourceInfo,
    },
    DEFINEUNIT {
        name: Ident,
        visibility: Visibility,
        exp: Option<ArcStr>,
        weight: Option<f64>,
        info: SourceInfo,
    },
}
pub use self::Element::{IMPORT,EXTENDS,CLASS,COMPONENT,DEFINEUNIT};

#[derive(Clone, Debug, PartialEq)]
pub enum Encapsulated {
    ENCAPSULATED,
    NOT_ENCAPSULATED,
}
pub use self::Encapsulated::{ENCAPSULATED,NOT_ENCAPSULATED};

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub literal: Ident,
    pub comment: Arc<Comment>,
}

pub type ENUM = Enum;


#[derive(Clone, Debug, PartialEq)]
pub enum Equation {
    EQ_IF {
        condition: Arc<metamodelica::List<Arc<Absyn::Exp>>>,
        thenBranch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Equation>>>>>,
        elseBranch: Arc<metamodelica::List<Arc<Equation>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_EQUALS {
        expLeft: Arc<Absyn::Exp>,
        expRight: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_PDE {
        expLeft: Arc<Absyn::Exp>,
        expRight: Arc<Absyn::Exp>,
        domain: Arc<Absyn::ComponentRef>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_CONNECT {
        crefLeft: Arc<Absyn::ComponentRef>,
        crefRight: Arc<Absyn::ComponentRef>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_FOR {
        index: Ident,
        range: Option<Arc<Absyn::Exp>>,
        eEquationLst: Arc<metamodelica::List<Arc<Equation>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_WHEN {
        condition: Arc<Absyn::Exp>,
        eEquationLst: Arc<metamodelica::List<Arc<Equation>>>,
        elseBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Equation>>>)>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_ASSERT {
        condition: Arc<Absyn::Exp>,
        message: Arc<Absyn::Exp>,
        level: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_TERMINATE {
        message: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_REINIT {
        cref: Arc<Absyn::Exp>,
        expReinit: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    EQ_NORETCALL {
        exp: Arc<Absyn::Exp>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
}
pub use self::Equation::{EQ_IF,EQ_EQUALS,EQ_PDE,EQ_CONNECT,EQ_FOR,EQ_WHEN,EQ_ASSERT,EQ_TERMINATE,EQ_REINIT,EQ_NORETCALL};

#[derive(Clone, Debug, PartialEq)]
pub struct ExternalDecl {
    pub funcName: Option<ArcStr>,
    pub lang: Option<ArcStr>,
    pub output_: Option<Arc<Absyn::ComponentRef>>,
    pub args: Arc<metamodelica::List<Arc<Absyn::Exp>>>,
    pub annotation_: Option<Arc<Annotation>>,
}

pub type EXTERNALDECL = ExternalDecl;


#[derive(Clone, Debug, PartialEq)]
pub enum Final {
    FINAL,
    NOT_FINAL,
}
pub use self::Final::{FINAL,NOT_FINAL};

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionRestriction {
    FR_NORMAL_FUNCTION {
        purity: Absyn::FunctionPurity,
    },
    FR_EXTERNAL_FUNCTION {
        purity: Absyn::FunctionPurity,
    },
    FR_OPERATOR_FUNCTION,
    FR_RECORD_CONSTRUCTOR,
    FR_PARALLEL_FUNCTION,
    FR_KERNEL_FUNCTION,
}
pub use self::FunctionRestriction::{FR_NORMAL_FUNCTION,FR_EXTERNAL_FUNCTION,FR_OPERATOR_FUNCTION,FR_RECORD_CONSTRUCTOR,FR_PARALLEL_FUNCTION,FR_KERNEL_FUNCTION};

pub type Ident = ArcStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Initial {
    INITIAL,
    NON_INITIAL,
}
pub use self::Initial::{INITIAL,NON_INITIAL};

#[derive(Clone, Debug, PartialEq)]
pub enum Mod {
    MOD {
        finalPrefix: Final,
        eachPrefix: Each,
        subModLst: Arc<metamodelica::List<Arc<SubMod>>>,
        binding: Option<Arc<Absyn::Exp>>,
        comment: Option<ArcStr>,
        info: SourceInfo,
    },
    REDECL {
        finalPrefix: Final,
        eachPrefix: Each,
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
pub use self::Mod::{MOD,REDECL,BREAK_COMPONENT,BREAK_CONNECT,NOMOD};

#[derive(Clone, Debug, PartialEq)]
pub enum Parallelism {
    PARGLOBAL,
    PARLOCAL,
    NON_PARALLEL,
}
pub use self::Parallelism::{PARGLOBAL,PARLOCAL,NON_PARALLEL};

#[derive(Clone, Debug, PartialEq)]
pub enum Partial {
    PARTIAL,
    NOT_PARTIAL,
}
pub use self::Partial::{PARTIAL,NOT_PARTIAL};

pub type Path = Arc<Absyn::Path>;

#[derive(Clone, Debug, PartialEq)]
pub struct Prefixes {
    pub visibility: Visibility,
    pub redeclarePrefix: Redeclare,
    pub finalPrefix: Final,
    pub innerOuter: Absyn::InnerOuter,
    pub replaceablePrefix: Arc<Replaceable>,
}

pub type PREFIXES = Prefixes;


pub type Program = Arc<metamodelica::List<Arc<Element>>>;

#[derive(Clone, Debug, PartialEq)]
pub enum Redeclare {
    REDECLARE,
    NOT_REDECLARE,
}
pub use self::Redeclare::{REDECLARE,NOT_REDECLARE};

#[derive(Clone, Debug, PartialEq)]
pub enum Replaceable {
    REPLACEABLE {
        cc: Option<Arc<ConstrainClass>>,
    },
    NOT_REPLACEABLE,
}
pub use self::Replaceable::{REPLACEABLE,NOT_REPLACEABLE};

#[derive(Clone, Debug, PartialEq)]
pub enum Restriction {
    R_CLASS,
    R_OPTIMIZATION,
    R_MODEL,
    R_RECORD {
        isOperator: bool,
    },
    R_BLOCK,
    R_CONNECTOR {
        isExpandable: bool,
    },
    R_OPERATOR,
    R_TYPE,
    R_PACKAGE,
    R_FUNCTION {
        functionRestriction: FunctionRestriction,
    },
    R_ENUMERATION,
    R_PREDEFINED_INTEGER,
    R_PREDEFINED_REAL,
    R_PREDEFINED_STRING,
    R_PREDEFINED_BOOLEAN,
    R_PREDEFINED_ENUMERATION,
    R_PREDEFINED_CLOCK,
    R_METARECORD {
        name: Arc<Absyn::Path>,
        index: i32,
        singleton: bool,
        moved: bool,
        typeVars: Arc<metamodelica::List<ArcStr>>,
    },
    R_UNIONTYPE {
        typeVars: Arc<metamodelica::List<ArcStr>>,
    },
}
pub use self::Restriction::{R_CLASS,R_OPTIMIZATION,R_MODEL,R_RECORD,R_BLOCK,R_CONNECTOR,R_OPERATOR,R_TYPE,R_PACKAGE,R_FUNCTION,R_ENUMERATION,R_PREDEFINED_INTEGER,R_PREDEFINED_REAL,R_PREDEFINED_STRING,R_PREDEFINED_BOOLEAN,R_PREDEFINED_ENUMERATION,R_PREDEFINED_CLOCK,R_METARECORD,R_UNIONTYPE};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    ALG_ASSIGN {
        assignComponent: Arc<Absyn::Exp>,
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
        index: Ident,
        range: Option<Arc<Absyn::Exp>>,
        forBody: Arc<metamodelica::List<Arc<Statement>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_PARFOR {
        index: Ident,
        range: Option<Arc<Absyn::Exp>>,
        parforBody: Arc<metamodelica::List<Arc<Statement>>>,
        comment: Arc<Comment>,
        info: SourceInfo,
    },
    ALG_WHILE {
        boolExpr: Arc<Absyn::Exp>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct SubMod {
    pub ident: Ident,
    pub r#mod: Arc<Mod>,
}

pub type NAMEMOD = SubMod;


pub type Subscript = Arc<Absyn::Subscript>;

#[derive(Clone, Debug, PartialEq)]
pub enum Variability {
    VAR,
    DISCRETE,
    PARAM,
    CONST,
}
pub use self::Variability::{VAR,DISCRETE,PARAM,CONST};

#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    PUBLIC,
    PROTECTED,
}
pub use self::Visibility::{PUBLIC,PROTECTED};

pub static defaultConstAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static defaultInputAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static defaultOutputAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::OUTPUT, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static defaultParamAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static defaultPrefixes: std::sync::LazyLock<Arc<Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(Prefixes { visibility: crate::SCode::Visibility::PUBLIC, redeclarePrefix: crate::SCode::Redeclare::NOT_REDECLARE, finalPrefix: crate::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE) }) });

pub static defaultProtectedPrefixes: std::sync::LazyLock<Arc<Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(Prefixes { visibility: crate::SCode::Visibility::PROTECTED, redeclarePrefix: crate::SCode::Redeclare::NOT_REDECLARE, finalPrefix: crate::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE) }) });

pub static defaultVarAttr: std::sync::LazyLock<Attributes> = std::sync::LazyLock::new(|| { Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });

pub static noComment: std::sync::LazyLock<Arc<Comment>> = std::sync::LazyLock::new(|| { Arc::new(Comment { annotation_: None, comment: None }) });

