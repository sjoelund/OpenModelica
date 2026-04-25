//! Translation of FrontEnd/SCode.mo
//!
//! This module defines the simplified code (SCode) types for Modelica.
//! It contains the types for the intermediate representation between
//! the Absyn AST and the final code generation.
//!
//! This is a direct translation of the SCode package from MetaModelica.
//! All uniontypes are translated to Rust enums with struct variants.
//! All records are translated to Rust structs.
//! All simple type aliases are translated to Rust type aliases.

use im::Vector;

use crate::absyn::{self, ArrayDim, ComponentRef, Exp, FunctionPurity, Import, InnerOuter, Path, TypeSpec};

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = Vector<T>;

// ============================================================================
// Simple type aliases
// ============================================================================

/// An identifier (re-exported from Absyn)
pub type Ident = String;

/// Program is a list of elements
pub type Program = List<Element>;

// ============================================================================
// Restriction - Class restriction types (SCode version)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Restriction {
    R_CLASS,
    R_OPTIMIZATION,
    R_MODEL,
    R_RECORD { is_operator: bool },
    R_BLOCK,
    R_CONNECTOR { is_expandable: bool },
    R_OPERATOR,
    R_TYPE,
    R_PACKAGE,
    R_FUNCTION { function_restriction: FunctionRestriction },
    R_ENUMERATION,
    R_PREDEFINED_INTEGER,
    R_PREDEFINED_REAL,
    R_PREDEFINED_STRING,
    R_PREDEFINED_BOOLEAN,
    R_PREDEFINED_ENUMERATION,
    R_PREDEFINED_CLOCK,
    R_METARECORD {
        name: Path,
        index: i32,
        singleton: bool,
        moved: bool,
        type_vars: List<String>,
    },
    R_UNIONTYPE { type_vars: List<String> },
}

// ============================================================================
// FunctionRestriction
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FunctionRestriction {
    FR_NORMAL_FUNCTION { purity: FunctionPurity },
    FR_EXTERNAL_FUNCTION { purity: FunctionPurity },
    FR_OPERATOR_FUNCTION,
    FR_RECORD_CONSTRUCTOR,
    FR_PARALLEL_FUNCTION,
    FR_KERNEL_FUNCTION,
}

// ============================================================================
// Mod - Modification types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Mod {
    MOD {
        final_prefix: Final,
        each_prefix: Each,
        sub_mod_lst: List<SubMod>,
        binding: Option<Exp>,
        comment: Option<String>,
        info: absyn::SourceInfo,
    },
    REDECL {
        final_prefix: Final,
        each_prefix: Each,
        element: Box<Element>,
    },
    BREAK_COMPONENT { info: absyn::SourceInfo },
    BREAK_CONNECT {
        lhs: ComponentRef,
        rhs: ComponentRef,
        info: absyn::SourceInfo,
    },
    NOMOD,
}

// ============================================================================
// SubMod
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SubMod {
    NAMEMOD { ident: Ident, mod_: Mod },
}

// ============================================================================
// Enum - enumeration literal
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Enum {
    ENUM {
        literal: Ident,
        comment: Comment,
    },
}

// ============================================================================
// ClassDef - class definition body
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ClassDef {
    PARTS {
        element_lst: List<Element>,
        normal_equation_lst: List<Equation>,
        initial_equation_lst: List<Equation>,
        normal_algorithm_lst: List<AlgorithmSection>,
        initial_algorithm_lst: List<AlgorithmSection>,
        constraint_lst: List<ConstraintSection>,
        clsattrs: List<absyn::NamedArg>,
        external_decl: Option<ExternalDecl>,
    },
    CLASS_EXTENDS {
        modifications: Mod,
        composition: Box<ClassDef>,
    },
    DERIVED {
        type_spec: TypeSpec,
        modifications: Mod,
        attributes: Attributes,
    },
    ENUMERATION { enum_lst: List<Enum> },
    OVERLOAD { path_lst: List<Path> },
    PDER {
        function_path: Path,
        derived_variables: List<Ident>,
    },
}

// ============================================================================
// Comment
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Comment {
    COMMENT {
        annotation_: Option<Annotation>,
        comment: Option<String>,
    },
}

/// Constant: no comment
pub const NO_COMMENT: Comment = Comment::COMMENT {
    annotation_: None,
    comment: None,
};

// ============================================================================
// Annotation
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Annotation {
    ANNOTATION { modification: Mod },
}

// ============================================================================
// ExternalDecl
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ExternalDecl {
    EXTERNALDECL {
        func_name: Option<Ident>,
        lang: Option<String>,
        output_: Option<ComponentRef>,
        args: List<Exp>,
        annotation_: Option<Annotation>,
    },
}

// ============================================================================
// Equation - simplified equation types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Equation {
    EQ_IF {
        condition: List<Exp>,
        then_branch: List<List<Equation>>,
        else_branch: List<Equation>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_EQUALS {
        exp_left: Exp,
        exp_right: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_PDE {
        exp_left: Exp,
        exp_right: Exp,
        domain: ComponentRef,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_CONNECT {
        cref_left: ComponentRef,
        cref_right: ComponentRef,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_FOR {
        index: Ident,
        range: Option<Exp>,
        e_equation_lst: List<Equation>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_WHEN {
        condition: Exp,
        e_equation_lst: List<Equation>,
        else_branches: List<(Exp, List<Equation>)>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_ASSERT {
        condition: Exp,
        message: Exp,
        level: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_TERMINATE {
        message: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_REINIT {
        cref: Exp,
        exp_reinit: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    EQ_NORETCALL {
        exp: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
}

// ============================================================================
// AlgorithmSection
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AlgorithmSection {
    ALGORITHM { statements: List<Statement> },
}

// ============================================================================
// ConstraintSection
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ConstraintSection {
    CONSTRAINTS { constraints: List<Exp> },
}

// ============================================================================
// Statement
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Statement {
    ALG_ASSIGN {
        assign_component: Exp,
        value: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_IF {
        bool_expr: Exp,
        true_branch: List<Statement>,
        else_if_branch: List<(Exp, List<Statement>)>,
        else_branch: List<Statement>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_FOR {
        index: Ident,
        range: Option<Exp>,
        for_body: List<Statement>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_PARFOR {
        index: Ident,
        range: Option<Exp>,
        parfor_body: List<Statement>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_WHILE {
        bool_expr: Exp,
        while_body: List<Statement>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_WHEN_A {
        branches: List<(Exp, List<Statement>)>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_ASSERT {
        condition: Exp,
        message: Exp,
        level: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_TERMINATE {
        message: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_REINIT {
        cref: Exp,
        new_value: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_NORETCALL {
        exp: Exp,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_RETURN {
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_BREAK {
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_FAILURE {
        stmts: List<Statement>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_TRY {
        body: List<Statement>,
        else_body: List<Statement>,
        comment: Comment,
        info: absyn::SourceInfo,
    },
    ALG_CONTINUE {
        comment: Comment,
        info: absyn::SourceInfo,
    },
}

// ============================================================================
// Visibility
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Visibility {
    PUBLIC,
    PROTECTED,
}

// ============================================================================
// Redeclare
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Redeclare {
    REDECLARE,
    NOT_REDECLARE,
}

// ============================================================================
// ConstrainClass
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ConstrainClass {
    CONSTRAINCLASS {
        constraining_class: Path,
        modifier: Mod,
        comment: Comment,
    },
}

// ============================================================================
// Replaceable
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Replaceable {
    REPLACEABLE { cc: Option<ConstrainClass> },
    NOT_REPLACEABLE,
}

// ============================================================================
// Final
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Final {
    FINAL,
    NOT_FINAL,
}

// ============================================================================
// Each
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Each {
    EACH,
    NOT_EACH,
}

// ============================================================================
// Encapsulated
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Encapsulated {
    ENCAPSULATED,
    NOT_ENCAPSULATED,
}

// ============================================================================
// Partial
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Partial {
    PARTIAL,
    NOT_PARTIAL,
}

// ============================================================================
// ConnectorType
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ConnectorType {
    POTENTIAL,
    FLOW,
    STREAM,
}

// ============================================================================
// Prefixes
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Prefixes {
    PREFIXES {
        visibility: Visibility,
        redeclare_prefix: Redeclare,
        final_prefix: Final,
        inner_outer: InnerOuter,
        replaceable_prefix: Replaceable,
    },
}

/// Default prefixes (public, not redeclare, not final, not inner/outer, not replaceable)
pub const DEFAULT_PREFIXES: Prefixes = Prefixes::PREFIXES {
    visibility: Visibility::PUBLIC,
    redeclare_prefix: Redeclare::NOT_REDECLARE,
    final_prefix: Final::NOT_FINAL,
    inner_outer: InnerOuter::NOT_INNER_OUTER,
    replaceable_prefix: Replaceable::NOT_REPLACEABLE,
};

/// Default protected prefixes
pub const DEFAULT_PROTECTED_PREFIXES: Prefixes = Prefixes::PREFIXES {
    visibility: Visibility::PROTECTED,
    redeclare_prefix: Redeclare::NOT_REDECLARE,
    final_prefix: Final::NOT_FINAL,
    inner_outer: InnerOuter::NOT_INNER_OUTER,
    replaceable_prefix: Replaceable::NOT_REPLACEABLE,
};

// ============================================================================
// Element - simplified code elements
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Element {
    IMPORT {
        imp: Import,
        visibility: Visibility,
        info: absyn::SourceInfo,
    },
    EXTENDS {
        base_class_path: Path,
        visibility: Visibility,
        modifications: Box<Mod>,
        ann: Option<Annotation>,
        info: absyn::SourceInfo,
    },
    CLASS {
        name: Ident,
        prefixes: Prefixes,
        encapsulated_prefix: Encapsulated,
        partial_prefix: Partial,
        restriction: Restriction,
        class_def: ClassDef,
        cmt: Comment,
        info: absyn::SourceInfo,
    },
    COMPONENT {
        name: Ident,
        prefixes: Prefixes,
        attributes: Attributes,
        type_spec: TypeSpec,
        modifications: Box<Mod>,
        comment: Comment,
        condition: Option<Exp>,
        info: absyn::SourceInfo,
    },
    DEFINEUNIT {
        name: Ident,
        visibility: Visibility,
        exp: Option<String>,
        weight: Option<f64>,
        info: absyn::SourceInfo,
    },
}

// ============================================================================
// Attributes
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Attributes {
    ATTR {
        array_dims: ArrayDim,
        connector_type: ConnectorType,
        parallelism: Parallelism,
        variability: Variability,
        direction: absyn::Direction,
        is_field: absyn::IsField,
    },
}

/// Default variable attributes
pub fn default_var_attr() -> Attributes {
    Attributes::ATTR {
        array_dims: Vector::new(),
        connector_type: ConnectorType::POTENTIAL,
        parallelism: Parallelism::NON_PARALLEL,
        variability: Variability::VAR,
        direction: absyn::Direction::BIDIR,
        is_field: absyn::IsField::NONFIELD,
    }
}

/// Default parameter attributes
pub fn default_param_attr() -> Attributes {
    Attributes::ATTR {
        array_dims: Vector::new(),
        connector_type: ConnectorType::POTENTIAL,
        parallelism: Parallelism::NON_PARALLEL,
        variability: Variability::PARAM,
        direction: absyn::Direction::BIDIR,
        is_field: absyn::IsField::NONFIELD,
    }
}

/// Default const attributes
pub fn default_const_attr() -> Attributes {
    Attributes::ATTR {
        array_dims: Vector::new(),
        connector_type: ConnectorType::POTENTIAL,
        parallelism: Parallelism::NON_PARALLEL,
        variability: Variability::CONST,
        direction: absyn::Direction::BIDIR,
        is_field: absyn::IsField::NONFIELD,
    }
}

/// Default input attributes
pub fn default_input_attr() -> Attributes {
    Attributes::ATTR {
        array_dims: Vector::new(),
        connector_type: ConnectorType::POTENTIAL,
        parallelism: Parallelism::NON_PARALLEL,
        variability: Variability::VAR,
        direction: absyn::Direction::INPUT,
        is_field: absyn::IsField::NONFIELD,
    }
}

/// Default output attributes
pub fn default_output_attr() -> Attributes {
    Attributes::ATTR {
        array_dims: Vector::new(),
        connector_type: ConnectorType::POTENTIAL,
        parallelism: Parallelism::NON_PARALLEL,
        variability: Variability::VAR,
        direction: absyn::Direction::OUTPUT,
        is_field: absyn::IsField::NONFIELD,
    }
}

// ============================================================================
// Parallelism
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Parallelism {
    PARGLOBAL,
    PARLOCAL,
    NON_PARALLEL,
}

// ============================================================================
// Variability
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Variability {
    VAR,
    DISCRETE,
    PARAM,
    CONST,
}

// ============================================================================
// Initial
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Initial {
    INITIAL,
    NON_INITIAL,
}
