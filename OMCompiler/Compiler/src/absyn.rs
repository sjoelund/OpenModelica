//! Translation of FrontEnd/Absyn.mo
//!
//! This module defines the abstract syntax for Modelica in Rust.
//! It contains the types for constructing the abstract syntax tree (AST).
//!
//! This is a direct translation of the Absyn package from MetaModelica.
//! All uniontypes are translated to Rust enums with struct variants.
//! All records are translated to Rust structs.
//! All simple type aliases are translated to Rust type aliases.

use im::Vector;
use std::fmt;

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = Vector<T>;

// ============================================================================
// SourceInfo - external type defined elsewhere in the compiler runtime
// ============================================================================

/// SourceInfo (Info) - file information (filename, read-only flag, line/column positions)
/// This is a built-in compiler type, defined outside of Absyn.mo.
/// Fields observed from C runtime: fileName, isReadOnly, startLine, startColumn, endLine, endColumn
#[derive(Debug, Clone, PartialEq)]
pub struct SourceInfo {
    pub file_name: String,
    pub is_read_only: bool,
    pub start_line: i32,
    pub start_column: i32,
    pub end_line: i32,
    pub end_column: i32,
}

// ============================================================================
// Simple type aliases
// ============================================================================

/// An identifier, for example a variable name
pub type Ident = String;

/// Information: FileName + isReadOnly + start/end line and column numbers
pub type Info = SourceInfo;

/// ForIterators - used in for loops and array iterators
pub type ForIterators = List<ForIterator>;

/// Array dimensions
pub type ArrayDim = List<Subscript>;

/// Component attributes are properties of components applied by type prefixes
pub type ComponentCondition = Exp;

// ============================================================================
// ForIterator
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ForIterator {
    ITERATOR {
        name: String,
        guard_exp: Option<Exp>,
        range: Option<Exp>,
    },
}

// ============================================================================
// Program - top level construct
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Program {
    PROGRAM {
        classes: List<Class>,
        within_: Within,
    },
}

// ============================================================================
// Within - Within Clauses
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Within {
    WITHIN { path: Path },
    TOP,
}

// ============================================================================
// Class - A class definition
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Class {
    CLASS {
        name: Ident,
        partial_prefix: bool,
        final_prefix: bool,
        encapsulated_prefix: bool,
        restriction: Restriction,
        body: ClassDef,
        comments_before_class: List<String>,
        comments_before_end: List<String>,
        comments_after_end: List<String>,
        info: Info,
    },
}

// ============================================================================
// ClassDef - The class definition part of a class declaration
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ClassDef {
    PARTS {
        type_vars: List<String>,
        class_attrs: List<NamedArg>,
        class_parts: List<ClassPart>,
        ann: List<Annotation>,
        comment: Option<String>,
    },
    DERIVED {
        type_spec: TypeSpec,
        attributes: ElementAttributes,
        arguments: List<ElementArg>,
        comment: Option<Comment>,
    },
    ENUMERATION {
        enum_literals: EnumDef,
        comment: Option<Comment>,
    },
    OVERLOAD {
        function_names: List<Path>,
        comment: Option<Comment>,
    },
    CLASS_EXTENDS {
        base_class_name: Ident,
        modifications: List<ElementArg>,
        comment: Option<String>,
        parts: List<ClassPart>,
        ann: List<Annotation>,
    },
    PDER {
        function_name: Path,
        vars: List<Ident>,
        comment: Option<Comment>,
    },
}

// ============================================================================
// TypeSpec - Type specification
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TypeSpec {
    TPATH {
        path: Path,
        array_dim: Option<ArrayDim>,
    },
    TCOMPLEX {
        path: Path,
        type_specs: List<TypeSpec>,
        array_dim: Option<ArrayDim>,
    },
}

// ============================================================================
// EnumDef - The definition of an enumeration
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EnumDef {
    ENUMLITERALS {
        enum_literals: List<EnumLiteral>,
    },
    ENUM_COLON,
}

// ============================================================================
// EnumLiteral - A name in an enumeration with optional comment
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EnumLiteral {
    ENUMLITERAL {
        literal: Ident,
        comment: Option<Comment>,
    },
}

// ============================================================================
// ClassPart - Parts of a class definition
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ClassPart {
    PUBLIC { contents: List<ElementItem> },
    PROTECTED { contents: List<ElementItem> },
    CONSTRAINTS { contents: List<Exp> },
    EQUATIONS { contents: List<EquationItem> },
    INITIALEQUATIONS { contents: List<EquationItem> },
    ALGORITHMS { contents: List<AlgorithmItem> },
    INITIALALGORITHMS { contents: List<AlgorithmItem> },
    EXTERNAL {
        external_decl: ExternalDecl,
        annotation_: Option<Annotation>,
    },
}

// ============================================================================
// ElementItem - Either an element or an annotation
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ElementItem {
    ELEMENTITEM { element: Element },
    LEXER_COMMENT { comment: String },
}

// ============================================================================
// Element - The basic element type in Modelica
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Element {
    ELEMENT {
        final_prefix: bool,
        redeclare_keywords: Option<RedeclareKeywords>,
        inner_outer: InnerOuter,
        specification: ElementSpec,
        info: Info,
        constrain_class: Option<ConstrainClass>,
    },
    DEFINEUNIT {
        name: Ident,
        args: List<NamedArg>,
        info: Info,
    },
    TEXT {
        opt_name: Option<Ident>,
        string: String,
        info: Info,
    },
}

// ============================================================================
// ConstrainClass - Constraining type, must be extends
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ConstrainClass {
    CONSTRAINCLASS {
        element_spec: ElementSpec,
        comment: Option<Comment>,
    },
}

// ============================================================================
// ElementSpec - Element specification (CLASSDEF, EXTENDS, IMPORT, COMPONENTS)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ElementSpec {
    CLASSDEF { replaceable_: bool, class_: Class },
    EXTENDS {
        path: Path,
        element_arg: List<ElementArg>,
        annotation_opt: Option<Annotation>,
    },
    IMPORT {
        import_: Import,
        comment: Option<Comment>,
        info: Info,
    },
    COMPONENTS {
        attributes: ElementAttributes,
        type_spec: TypeSpec,
        components: List<ComponentItem>,
    },
}

// ============================================================================
// InnerOuter - inner/outer keyword
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum InnerOuter {
    INNER,
    OUTER,
    INNER_OUTER,
    NOT_INNER_OUTER,
}

// ============================================================================
// Import - Import statements
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Import {
    NAMED_IMPORT { name: Ident, path: Path },
    QUAL_IMPORT { path: Path },
    UNQUAL_IMPORT { path: Path },
    GROUP_IMPORT {
        prefix: Path,
        groups: List<GroupImport>,
    },
}

// ============================================================================
// GroupImport
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GroupImport {
    GROUP_IMPORT_NAME { name: String },
    GROUP_IMPORT_RENAME { rename: String, name: String },
}

// ============================================================================
// ComponentItem - Collection of component and an optional comment
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ComponentItem {
    COMPONENTITEM {
        component: Component,
        condition: Option<ComponentCondition>,
        comment: Option<Comment>,
    },
}

// ============================================================================
// Component - Some kind of Modelica entity
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Component {
    COMPONENT {
        name: Ident,
        array_dim: ArrayDim,
        modification: Option<Modification>,
    },
}

// ============================================================================
// EquationItem - Grouped component declarations
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EquationItem {
    EQUATIONITEM {
        equation_: Box<Equation>,
        comment: Option<Comment>,
        info: Info,
    },
    EQUATIONITEMCOMMENT { comment: String },
}

// ============================================================================
// AlgorithmItem - Info specific for an algorithm item
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AlgorithmItem {
    ALGORITHMITEM {
        algorithm_: Algorithm,
        comment: Option<Comment>,
        info: Info,
    },
    ALGORITHMITEMCOMMENT { comment: String },
}

// ============================================================================
// Equation - One kind of equation
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Equation {
    EQ_IF {
        if_exp: Box<Exp>,
        equation_true_items: List<EquationItem>,
        else_if_branches: List<(Box<Exp>, List<EquationItem>)>,
        equation_else_items: List<EquationItem>,
    },
    EQ_EQUALS {
        left_side: Box<Exp>,
        right_side: Box<Exp>,
    },
    EQ_PDE {
        left_side: Box<Exp>,
        right_side: Box<Exp>,
        domain: ComponentRef,
    },
    EQ_CONNECT {
        connector1: ComponentRef,
        connector2: ComponentRef,
    },
    EQ_FOR {
        iterators: ForIterators,
        for_equations: List<EquationItem>,
    },
    EQ_WHEN_E {
        when_exp: Box<Exp>,
        when_equations: List<EquationItem>,
        else_when_equations: List<(Box<Exp>, List<EquationItem>)>,
    },
    EQ_NORETCALL {
        function_name: ComponentRef,
        function_args: FunctionArgs,
    },
    EQ_FAILURE { equ: Box<EquationItem> },
}

// ============================================================================
// Algorithm - One algorithm statement in an algorithm section
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Algorithm {
    ALG_ASSIGN {
        assign_component: Box<Exp>,
        value: Box<Exp>,
    },
    ALG_IF {
        if_exp: Box<Exp>,
        true_branch: List<AlgorithmItem>,
        else_if_algorithm_branch: List<(Box<Exp>, List<AlgorithmItem>)>,
        else_branch: List<AlgorithmItem>,
    },
    ALG_FOR {
        iterators: ForIterators,
        for_body: List<AlgorithmItem>,
    },
    ALG_PARFOR {
        iterators: ForIterators,
        parfor_body: List<AlgorithmItem>,
    },
    ALG_WHILE {
        bool_expr: Box<Exp>,
        while_body: List<AlgorithmItem>,
    },
    ALG_WHEN_A {
        bool_expr: Box<Exp>,
        when_body: List<AlgorithmItem>,
        else_when_algorithm_branch: List<(Box<Exp>, List<AlgorithmItem>)>,
    },
    ALG_NORETCALL {
        function_call: ComponentRef,
        function_args: FunctionArgs,
    },
    ALG_RETURN,
    ALG_BREAK,
    ALG_FAILURE { equ: List<AlgorithmItem> },
    ALG_TRY {
        body: List<AlgorithmItem>,
        else_body: List<AlgorithmItem>,
    },
    ALG_CONTINUE,
}

// ============================================================================
// Modification - Modifications (redeclarations and component modifications)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Modification {
    CLASSMOD {
        element_arg_lst: List<ElementArg>,
        eq_mod: EqMod,
    },
}

// ============================================================================
// EqMod - Equation modification
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EqMod {
    NOMOD,
    EQMOD { exp: Exp, info: Info },
}

// ============================================================================
// ElementArg - Wrapper for things that modify elements, modifications and redeclarations
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ElementArg {
    MODIFICATION {
        final_prefix: bool,
        each_prefix: Each,
        path: Path,
        modification: Option<Modification>,
        comment: Option<String>,
        info: Info,
    },
    REDECLARATION {
        final_prefix: bool,
        redeclare_keywords: RedeclareKeywords,
        each_prefix: Each,
        element_spec: ElementSpec,
        constrain_class: Option<ConstrainClass>,
        info: Info,
    },
    ELEMENTARGCOMMENT { comment: String },
    INHERITANCEBREAK { cnct: Equation, info: Info },
}

// ============================================================================
// RedeclareKeywords - redeclare and replaceable keywords
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum RedeclareKeywords {
    REDECLARE,
    REPLACEABLE,
    REDECLARE_REPLACEABLE,
}

// ============================================================================
// Each - The each keyword
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Each {
    EACH,
    NON_EACH,
}

// ============================================================================
// ElementAttributes - Element attributes
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ElementAttributes {
    ATTR {
        flow_prefix: bool,
        stream_prefix: bool,
        parallelism: Parallelism,
        variability: Variability,
        direction: Direction,
        is_field: IsField,
        array_dim: ArrayDim,
    },
}

// ============================================================================
// IsField - Is field
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IsField {
    NONFIELD,
    FIELD,
}

// ============================================================================
// Parallelism - Parallelism for CUDA/OpenCL
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Parallelism {
    PARGLOBAL,
    PARLOCAL,
    NON_PARALLEL,
}

// ============================================================================
// FlowStream - Flow/Stream
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FlowStream {
    FLOW,
    STREAM,
    NOT_FLOW_STREAM,
}

// ============================================================================
// Variability - Variability (parameter, constant, etc.)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Variability {
    VAR,
    DISCRETE,
    PARAM,
    CONST,
}

// ============================================================================
// Direction - Direction (input/output)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Direction {
    INPUT,
    OUTPUT,
    BIDIR,
    INPUT_OUTPUT,
}

// ============================================================================
// Exp - Expressions
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Exp {
    INTEGER { value: i32 },
    REAL { value: String },
    CREF { component_ref: ComponentRef },
    STRING { value: String },
    BOOL { value: bool },
    BINARY {
        exp1: Box<Exp>,
        op: Operator,
        exp2: Box<Exp>,
    },
    UNARY {
        op: Operator,
        exp: Box<Exp>,
    },
    LBINARY {
        exp1: Box<Exp>,
        op: Operator,
        exp2: Box<Exp>,
    },
    LUNARY {
        op: Operator,
        exp: Box<Exp>,
    },
    RELATION {
        exp1: Box<Exp>,
        op: Operator,
        exp2: Box<Exp>,
    },
    IFEXP {
        if_exp: Box<Exp>,
        true_branch: Box<Exp>,
        else_branch: Box<Exp>,
        else_if_branch: List<(Box<Exp>, Box<Exp>)>,
    },
    CALL {
        function_: ComponentRef,
        function_args: Box<FunctionArgs>,
        type_vars: List<Path>,
    },
    PARTEVALFUNCTION {
        function_: ComponentRef,
        function_args: Box<FunctionArgs>,
    },
    ARRAY { array_exp: List<Exp> },
    MATRIX { matrix: List<List<Exp>> },
    RANGE {
        start: Box<Exp>,
        step: Option<Box<Exp>>,
        stop: Box<Exp>,
    },
    TUPLE { expressions: List<Box<Exp>> },
    END,
    CODE { code: Box<CodeNode> },
    AS {
        id: Ident,
        exp: Box<Exp>,
    },
    CONS {
        head: Box<Exp>,
        rest: Box<Exp>,
    },
    MATCHEXP {
        match_ty: MatchType,
        input_exp: Box<Exp>,
        local_decls: List<ElementItem>,
        cases: List<Case>,
        comment: Option<String>,
    },
    LIST { exps: List<Exp> },
    DOT {
        exp: Box<Exp>,
        index: Box<Exp>,
    },
    EXPRESSIONCOMMENT {
        comments_before: List<String>,
        exp: Box<Exp>,
        comments_after: List<String>,
    },
    SUBSCRIPTED_EXP {
        exp: Box<Exp>,
        subscripts: List<Subscript>,
    },
    BREAK,
}

// ============================================================================
// Case - case in match or matchcontinue
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Case {
    CASE {
        pattern: Box<Exp>,
        pattern_guard: Option<Box<Exp>>,
        pattern_info: Info,
        local_decls: List<ElementItem>,
        class_part: ClassPart,
        result: Box<Exp>,
        result_info: Info,
        comment: Option<String>,
        info: Info,
    },
    ELSE_ {
        local_decls: List<ElementItem>,
        class_part: ClassPart,
        result: Box<Exp>,
        result_info: Info,
        comment: Option<String>,
        info: Info,
    },
}

// ============================================================================
// MatchType - match or matchcontinue
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MatchType {
    MATCH,
    MATCHCONTINUE,
}

// ============================================================================
// CodeNode - Meta-programming Code
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CodeNode {
    C_TYPENAME { path: Path },
    C_VARIABLENAME { component_ref: ComponentRef },
    C_CONSTRAINTSECTION {
        boolean: bool,
        equation_item_lst: List<EquationItem>,
    },
    C_EQUATIONSECTION {
        boolean: bool,
        equation_item_lst: List<EquationItem>,
    },
    C_ALGORITHMSECTION {
        boolean: bool,
        algorithm_item_lst: List<AlgorithmItem>,
    },
    C_ELEMENT { element: Element },
    C_EXPRESSION { exp: Box<Exp> },
    C_MODIFICATION { modification: Modification },
}

// ============================================================================
// FunctionArgs - Positional and named function arguments
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FunctionArgs {
    FUNCTIONARGS {
        args: List<Exp>,
        arg_names: List<NamedArg>,
    },
    FOR_ITER_FARG {
        exp: Box<Exp>,
        iter_type: ReductionIterType,
        iterators: ForIterators,
    },
}

// ============================================================================
// ReductionIterType - Reduction iterator type
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ReductionIterType {
    COMBINE,
    THREAD,
}

// ============================================================================
// NamedArg - Named argument (identifier + expression)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum NamedArg {
    NAMEDARG {
        arg_name: Ident,
        arg_value: Exp,
    },
}

// ============================================================================
// Operator - Expression operators
// ============================================================================

#[derive(Clone, PartialEq, Copy)]
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

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Operator::ADD => "ADD",
            Operator::SUB => "SUB",
            Operator::MUL => "MUL",
            Operator::DIV => "DIV",
            Operator::POW => "POW",
            Operator::UPLUS => "UPLUS",
            Operator::UMINUS => "UMINUS",
            Operator::ADD_EW => "ADD_EW",
            Operator::SUB_EW => "SUB_EW",
            Operator::MUL_EW => "MUL_EW",
            Operator::DIV_EW => "DIV_EW",
            Operator::POW_EW => "POW_EW",
            Operator::UPLUS_EW => "UPLUS_EW",
            Operator::UMINUS_EW => "UMINUS_EW",
            Operator::AND => "AND",
            Operator::OR => "OR",
            Operator::NOT => "NOT",
            Operator::LESS => "LESS",
            Operator::LESSEQ => "LESSEQ",
            Operator::GREATER => "GREATER",
            Operator::GREATEREQ => "GREATEREQ",
            Operator::EQUAL => "EQUAL",
            Operator::NEQUAL => "NEQUAL",
        };
        f.write_str(name)
    }
}

// ============================================================================
// Subscript - Subscripts for array declarations and component references
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Subscript {
    NOSUB,
    SUBSCRIPT { subscript: Exp },
}

// ============================================================================
// ComponentRef - Fully or partially qualified name of a component
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ComponentRef {
    CREF_FULLYQUALIFIED { component_ref: Box<ComponentRef> },
    CREF_QUAL {
        name: Ident,
        subscripts: List<Subscript>,
        component_ref: Box<ComponentRef>,
    },
    CREF_IDENT {
        name: Ident,
        subscripts: List<Subscript>,
    },
    WILD,
    ALLWILD,
}

// ============================================================================
// Path - References to class names inside class definitions
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Path {
    QUALIFIED { name: Ident, path: Box<Path> },
    IDENT { name: Ident },
    FULLYQUALIFIED { path: Box<Path> },
}

// ============================================================================
// Restriction - Class declaration restriction types
// ============================================================================

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
    R_FUNCTION { function_restriction: FunctionRestriction },
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
        name: Path,
        index: i32,
        singleton: bool,
        moved: bool,
        type_vars: List<String>,
    },
    R_UNKNOWN,
}

// ============================================================================
// FunctionPurity - function purity
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FunctionPurity {
    PURE,
    IMPURE,
    NO_PURITY,
}

// ============================================================================
// FunctionRestriction - Function restriction types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FunctionRestriction {
    FR_NORMAL_FUNCTION { purity: FunctionPurity },
    FR_OPERATOR_FUNCTION,
    FR_PARALLEL_FUNCTION,
    FR_KERNEL_FUNCTION,
}

// ============================================================================
// Annotation - A class_modification
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Annotation {
    ANNOTATION { element_args: List<ElementArg> },
}

// ============================================================================
// Comment - Comment
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Comment {
    COMMENT {
        annotation_: Option<Annotation>,
        comment: Option<String>,
    },
}

// ============================================================================
// ExternalDecl - Declaration of an external function call
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
// Ref - Reference types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Ref {
    RCR { cr: ComponentRef },
    RTS { ts: TypeSpec },
    RIM { im: Import },
}

// ============================================================================
// Msg - Controls output of error messages
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Msg {
    MSG { info: Info },
    NO_MSG,
}

// ============================================================================
// Constants
// ============================================================================

/// Constant: Modification.emptyMod
pub fn empty_mod() -> Modification {
    Modification::CLASSMOD {
        element_arg_lst: List::new(),
        eq_mod: EqMod::NOMOD,
    }
}

/// Constant: FunctionArgs.emptyFunctionArgs = FUNCTIONARGS({}, {})
pub fn empty_function_args() -> FunctionArgs {
    FunctionArgs::FUNCTIONARGS {
        args: List::new(),
        arg_names: List::new(),
    }
}
