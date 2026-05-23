// Auto-generated from MetaModelica source
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

pub static ASSERTIONLEVEL_ERROR: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::ENUM_LITERAL { name: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("AssertionLevel")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("error")).clone() }) }), index: 2 }) });

pub static ASSERTIONLEVEL_WARNING: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::ENUM_LITERAL { name: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("AssertionLevel")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("warning")).clone() }) }), index: 1 }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Algorithm {
    pub statementLst: Arc<metamodelica::List<Arc<Statement>>>,
}

pub type ALGORITHM_STMTS = Algorithm;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attributes {
    pub connectorType: Arc<ConnectorType>,
    pub parallelism: SCode::Parallelism,
    pub variability: SCode::Variability,
    pub direction: Absyn::Direction,
    pub innerOuter: Absyn::InnerOuter,
    pub visibility: SCode::Visibility,
}

pub type ATTR = Attributes;


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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BindingSource {
    BINDING_FROM_DEFAULT_VALUE,
    BINDING_FROM_START_VALUE,
    BINDING_FROM_RECORD_SUBMODS,
    BINDING_FROM_DERIVED_RECORD_DECL,
}
pub use self::BindingSource::{BINDING_FROM_DEFAULT_VALUE,BINDING_FROM_START_VALUE,BINDING_FROM_RECORD_SUBMODS,BINDING_FROM_DERIVED_RECORD_DECL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallAttributes {
    pub ty: Arc<Type>,
    pub tuple_: bool,
    pub builtin: bool,
    pub isImpure: bool,
    pub isFunctionPointerCall: bool,
    pub inlineType: InlineType,
    pub tailCall: TailCall,
}

pub type CALL_ATTR = CallAttributes;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassAttributes {
    pub objetiveE: Option<Arc<Exp>>,
    pub objectiveIntegrandE: Option<Arc<Exp>>,
    pub startTimeE: Option<Arc<Exp>>,
    pub finalTimeE: Option<Arc<Exp>>,
}

pub type OPTIMIZATION_ATTRS = ClassAttributes;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassPrefix {
    pub variability: SCode::Variability,
}

pub type CLASSPRE = ClassPrefix;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockKind {
    INFERRED_CLOCK,
    RATIONAL_CLOCK {
        intervalCounter: Arc<Exp>,
        resolution: Arc<Exp>,
    },
    REAL_CLOCK {
        interval: Arc<Exp>,
    },
    EVENT_CLOCK {
        condition: Arc<Exp>,
        startInterval: Arc<Exp>,
    },
    SOLVER_CLOCK {
        c: Arc<Exp>,
        solverMethod: Arc<Exp>,
    },
}
pub use self::ClockKind::{INFERRED_CLOCK,RATIONAL_CLOCK,REAL_CLOCK,EVENT_CLOCK,SOLVER_CLOCK};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodeType {
    C_EXPRESSION,
    C_EXPRESSION_OR_MODIFICATION,
    C_MODIFICATION,
    C_TYPENAME,
    C_VARIABLENAME,
    C_VARIABLENAMES,
}
pub use self::CodeType::{C_EXPRESSION,C_EXPRESSION_OR_MODIFICATION,C_MODIFICATION,C_TYPENAME,C_VARIABLENAME,C_VARIABLENAMES};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentPrefix {
    PRE {
        prefix: ArcStr,
        dimensions: Arc<metamodelica::List<Arc<Dimension>>>,
        subscripts: Arc<metamodelica::List<Arc<Subscript>>>,
        next: Arc<ComponentPrefix>,
        ci_state: ClassInf::State,
        info: SourceInfo,
    },
    NOCOMPPRE,
}
pub use self::ComponentPrefix::{PRE,NOCOMPPRE};

pub type ComponentPrefixOpt = Option<Arc<ComponentPrefix>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComponentRef {
    CREF_QUAL {
        ident: Ident,
        identType: Arc<Type>,
        subscriptLst: Arc<metamodelica::List<Arc<Subscript>>>,
        componentRef: Arc<ComponentRef>,
    },
    CREF_IDENT {
        ident: Ident,
        identType: Arc<Type>,
        subscriptLst: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    OPTIMICA_ATTR_INST_CREF {
        componentRef: Arc<ComponentRef>,
        instant: ArcStr,
    },
    WILD,
}
pub use self::ComponentRef::{CREF_QUAL,CREF_IDENT,OPTIMICA_ATTR_INST_CREF,WILD};

pub mod Connect {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConnectorElement {
        pub name: Arc<ComponentRef>,
        pub face: Face,
        pub ty: ConnectorType,
        pub source: Arc<ElementSource>,
        pub set: i32,
    }

    pub type CONNECTOR_ELEMENT = ConnectorElement;


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
    pub enum Face {
        INSIDE,
        OUTSIDE,
        NO_FACE,
    }
    pub use self::Face::{INSIDE,OUTSIDE,NO_FACE};

    pub const NEW_SET: i32 = -1;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OuterConnect {
        pub scope: Prefix,
        pub cr1: Arc<ComponentRef>,
        pub io1: Absyn::InnerOuter,
        pub f1: Face,
        pub cr2: Arc<ComponentRef>,
        pub io2: Absyn::InnerOuter,
        pub f2: Face,
        pub source: Arc<ElementSource>,
    }

    pub type OUTERCONNECT = OuterConnect;


    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Set {
        SET {
            ty: ConnectorType,
            elements: Arc<metamodelica::List<ConnectorElement>>,
        },
        SET_POINTER {
            index: i32,
        },
    }
    pub use self::Set::{SET,SET_POINTER};

    pub type SetConnection = (i32, i32);

    pub type SetTrie = Arc<SetTrieNode>;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum SetTrieNode {
        SET_TRIE_NODE {
            name: ArcStr,
            cref: Arc<ComponentRef>,
            nodes: Arc<metamodelica::List<Arc<SetTrieNode>>>,
            connectCount: i32,
        },
        SET_TRIE_LEAF {
            name: ArcStr,
            insideElement: Option<ConnectorElement>,
            outsideElement: Option<ConnectorElement>,
            flowAssociation: Option<Arc<ComponentRef>>,
            connectCount: i32,
        },
    }
    pub use self::SetTrieNode::{SET_TRIE_NODE,SET_TRIE_LEAF};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Sets {
        pub sets: SetTrie,
        pub setCount: i32,
        pub connections: Arc<metamodelica::List<(i32, i32)>>,
        pub outerConnects: Arc<metamodelica::List<OuterConnect>>,
    }

    pub type SETS = Sets;


    pub static emptySet: std::sync::LazyLock<Sets> = std::sync::LazyLock::new(|| { Sets { sets: Arc::new(SetTrieNode::SET_TRIE_NODE { name: (literal!("")).clone(), cref: Arc::new(crate::DAE::ComponentRef::WILD), nodes: metamodelica::nil(), connectCount: 0 }), setCount: 0, connections: metamodelica::nil(), outerConnects: metamodelica::nil() } });

}

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
pub enum Const {
    C_CONST,
    C_PARAM,
    C_VAR,
    C_UNKNOWN,
}
pub use self::Const::{C_CONST,C_PARAM,C_VAR,C_UNKNOWN};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Constraint {
    CONSTRAINT_EXPS {
        constraintLst: Arc<metamodelica::List<Arc<Exp>>>,
    },
    CONSTRAINT_DT {
        constraint: Arc<Exp>,
        localCon: bool,
    },
}
pub use self::Constraint::{CONSTRAINT_EXPS,CONSTRAINT_DT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DAElist {
    pub elementLst: Arc<metamodelica::List<Arc<Element>>>,
}

pub type DAE = DAElist;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dimension {
    DIM_INTEGER {
        integer: i32,
    },
    DIM_BOOLEAN,
    DIM_ENUM {
        enumTypeName: Arc<Absyn::Path>,
        literals: Arc<metamodelica::List<ArcStr>>,
        size: i32,
    },
    DIM_EXP {
        exp: Arc<Exp>,
    },
    DIM_UNKNOWN,
}
pub use self::Dimension::{DIM_INTEGER,DIM_BOOLEAN,DIM_ENUM,DIM_EXP,DIM_UNKNOWN};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DimensionBinding {
    DIM_UNBOUND,
    DIM_BOUND {
        binding: Arc<Exp>,
        constrains: Dimensions,
    },
}
pub use self::DimensionBinding::{DIM_UNBOUND,DIM_BOUND};

pub type Dimensions = Arc<metamodelica::List<Arc<Dimension>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distribution {
    pub name: Arc<Exp>,
    pub params: Arc<Exp>,
    pub paramNames: Arc<Exp>,
}

pub type DISTRIBUTION = Distribution;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    VAR {
        componentRef: Arc<ComponentRef>,
        kind: VarKind,
        direction: VarDirection,
        parallelism: VarParallelism,
        protection: VarVisibility,
        ty: Arc<Type>,
        binding: Option<Arc<Exp>>,
        dims: InstDims,
        connectorType: Arc<ConnectorType>,
        source: Arc<ElementSource>,
        variableAttributesOption: Option<Arc<VariableAttributes>>,
        comment: Option<Arc<SCode::Comment>>,
        innerOuter: Absyn::InnerOuter,
        encrypted: bool,
    },
    DEFINE {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIALDEFINE {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    EQUATION {
        exp: Arc<Exp>,
        scalar: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    EQUEQUATION {
        cr1: Arc<ComponentRef>,
        cr2: Arc<ComponentRef>,
        source: Arc<ElementSource>,
    },
    ARRAY_EQUATION {
        dimension: Dimensions,
        exp: Arc<Exp>,
        array: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_ARRAY_EQUATION {
        dimension: Dimensions,
        exp: Arc<Exp>,
        array: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    CONNECT_EQUATION {
        lhsElement: Arc<Element>,
        lhsFace: Connect::Face,
        rhsElement: Arc<Element>,
        rhsFace: Connect::Face,
        source: Arc<ElementSource>,
    },
    COMPLEX_EQUATION {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_COMPLEX_EQUATION {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    WHEN_EQUATION {
        condition: Arc<Exp>,
        equations: Arc<metamodelica::List<Arc<Element>>>,
        elsewhen_: Option<Arc<Element>>,
        source: Arc<ElementSource>,
    },
    INITIAL_FOR_EQUATION {
        type_: Arc<Type>,
        iterIsArray: bool,
        iter: Ident,
        index: i32,
        range: Arc<Exp>,
        equations: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
    },
    FOR_EQUATION {
        type_: Arc<Type>,
        iterIsArray: bool,
        iter: Ident,
        index: i32,
        range: Arc<Exp>,
        equations: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
    },
    IF_EQUATION {
        condition1: Arc<metamodelica::List<Arc<Exp>>>,
        equations2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Element>>>>>,
        equations3: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
    },
    INITIAL_IF_EQUATION {
        condition1: Arc<metamodelica::List<Arc<Exp>>>,
        equations2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Element>>>>>,
        equations3: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
    },
    INITIALEQUATION {
        exp1: Arc<Exp>,
        exp2: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    ALGORITHM {
        algorithm_: Arc<Algorithm>,
        source: Arc<ElementSource>,
    },
    INITIALALGORITHM {
        algorithm_: Arc<Algorithm>,
        source: Arc<ElementSource>,
    },
    COMP {
        ident: Ident,
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
        source: Arc<ElementSource>,
        comment: Option<Arc<SCode::Comment>>,
    },
    EXTOBJECTCLASS {
        path: Arc<Absyn::Path>,
        source: Arc<ElementSource>,
    },
    ASSERT {
        condition: Arc<Exp>,
        message: Arc<Exp>,
        level: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_ASSERT {
        condition: Arc<Exp>,
        message: Arc<Exp>,
        level: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    TERMINATE {
        message: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_TERMINATE {
        message: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    REINIT {
        componentRef: Arc<ComponentRef>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    NORETCALL {
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    INITIAL_NORETCALL {
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    CONSTRAINT {
        constraints: Arc<Constraint>,
        source: Arc<ElementSource>,
    },
    CLASS_ATTRIBUTES {
        classAttrs: Arc<ClassAttributes>,
    },
    FLAT_SM {
        ident: Ident,
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
    },
    SM_COMP {
        componentRef: Arc<ComponentRef>,
        dAElist: Arc<metamodelica::List<Arc<Element>>>,
    },
    COMMENT {
        cmt: Arc<SCode::Comment>,
    },
}
pub use self::Element::{VAR,DEFINE,INITIALDEFINE,EQUATION,EQUEQUATION,ARRAY_EQUATION,INITIAL_ARRAY_EQUATION,CONNECT_EQUATION,COMPLEX_EQUATION,INITIAL_COMPLEX_EQUATION,WHEN_EQUATION,INITIAL_FOR_EQUATION,FOR_EQUATION,IF_EQUATION,INITIAL_IF_EQUATION,INITIALEQUATION,ALGORITHM,INITIALALGORITHM,COMP,EXTOBJECTCLASS,ASSERT,INITIAL_ASSERT,TERMINATE,INITIAL_TERMINATE,REINIT,NORETCALL,INITIAL_NORETCALL,CONSTRAINT,CLASS_ATTRIBUTES,FLAT_SM,SM_COMP,COMMENT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementSource {
    pub info: SourceInfo,
    pub partOfLst: Arc<metamodelica::List<Absyn::Within>>,
    pub instance: Arc<ComponentPrefix>,
    pub connectEquationOptLst: Arc<metamodelica::List<(Arc<ComponentRef>, Arc<ComponentRef>)>>,
    pub typeLst: Arc<metamodelica::List<Arc<Absyn::Path>>>,
    pub operations: Arc<metamodelica::List<Arc<SymbolicOperation>>>,
    pub comment: Arc<metamodelica::List<Arc<SCode::Comment>>>,
}

pub type SOURCE = ElementSource;


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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EqMod {
    TYPED {
        modifierAsExp: Arc<Exp>,
        modifierAsValue: Option<Arc<Values::Value>>,
        properties: Properties,
        modifierAsAbsynExp: Arc<Absyn::Exp>,
        info: SourceInfo,
    },
    UNTYPED {
        exp: Arc<Absyn::Exp>,
    },
}
pub use self::EqMod::{TYPED,UNTYPED};

pub type EqualityConstraint = Option<(Arc<Absyn::Path>, i32, InlineType)>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EquationExp {
    PARTIAL_EQUATION {
        exp: Arc<Exp>,
    },
    RESIDUAL_EXP {
        exp: Arc<Exp>,
    },
    EQUALITY_EXPS {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
    },
}
pub use self::EquationExp::{PARTIAL_EQUATION,RESIDUAL_EXP,EQUALITY_EXPS};

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exp {
    ICONST {
        integer: i32,
    },
    RCONST {
        real: metamodelica::Real,
    },
    SCONST {
        string: ArcStr,
    },
    BCONST {
        bool: bool,
    },
    CLKCONST {
        clk: Arc<ClockKind>,
    },
    ENUM_LITERAL {
        name: Arc<Absyn::Path>,
        index: i32,
    },
    CREF {
        componentRef: Arc<ComponentRef>,
        ty: Arc<Type>,
    },
    BINARY {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
    },
    UNARY {
        operator: Operator,
        exp: Arc<Exp>,
    },
    LBINARY {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
    },
    LUNARY {
        operator: Operator,
        exp: Arc<Exp>,
    },
    RELATION {
        exp1: Arc<Exp>,
        operator: Operator,
        exp2: Arc<Exp>,
        index: i32,
        optionExpisASUB: Option<(Arc<Exp>, i32, i32)>,
    },
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
    RECORD {
        path: Arc<Absyn::Path>,
        exps: Arc<metamodelica::List<Arc<Exp>>>,
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
        scalar: bool,
        array: Arc<metamodelica::List<Arc<Exp>>>,
    },
    MATRIX {
        ty: Arc<Type>,
        integer: i32,
        matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Exp>>>>>,
    },
    RANGE {
        ty: Arc<Type>,
        start: Arc<Exp>,
        step: Option<Arc<Exp>>,
        stop: Arc<Exp>,
    },
    TUPLE {
        PR: Arc<metamodelica::List<Arc<Exp>>>,
    },
    CAST {
        ty: Arc<Type>,
        exp: Arc<Exp>,
    },
    ASUB {
        exp: Arc<Exp>,
        sub: Arc<metamodelica::List<Arc<Subscript>>>,
    },
    TSUB {
        exp: Arc<Exp>,
        ix: i32,
        ty: Arc<Type>,
    },
    RSUB {
        exp: Arc<Exp>,
        ix: i32,
        fieldName: ArcStr,
        ty: Arc<Type>,
    },
    SIZE {
        exp: Arc<Exp>,
        sz: Option<Arc<Exp>>,
    },
    CODE {
        code: Arc<Absyn::CodeNode>,
        ty: Arc<Type>,
    },
    EMPTY {
        scope: ArcStr,
        name: Arc<ComponentRef>,
        ty: Arc<Type>,
        tyStr: ArcStr,
    },
    REDUCTION {
        reductionInfo: Arc<ReductionInfo>,
        expr: Arc<Exp>,
        iterators: ReductionIterators,
    },
    LIST {
        valList: Arc<metamodelica::List<Arc<Exp>>>,
    },
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
        aliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>,
        localDecls: Arc<metamodelica::List<Arc<Element>>>,
        cases: Arc<metamodelica::List<Arc<MatchCase>>>,
        et: Arc<Type>,
    },
    BOX {
        exp: Arc<Exp>,
    },
    UNBOX {
        exp: Arc<Exp>,
        ty: Arc<Type>,
    },
    SHARED_LITERAL {
        index: i32,
        exp: Arc<Exp>,
    },
    PATTERN {
        pattern: Arc<Pattern>,
    },
}
pub use self::Exp::{ICONST,RCONST,SCONST,BCONST,CLKCONST,ENUM_LITERAL,CREF,BINARY,UNARY,LBINARY,LUNARY,RELATION,IFEXP,CALL,RECORD,PARTEVALFUNCTION,ARRAY,MATRIX,RANGE,TUPLE,CAST,ASUB,TSUB,RSUB,SIZE,CODE,EMPTY,REDUCTION,LIST,CONS,META_TUPLE,META_OPTION,METARECORDCALL,MATCHEXPRESSION,BOX,UNBOX,SHARED_LITERAL,PATTERN};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expand {
    EXPAND,
    NOT_EXPAND,
}
pub use self::Expand::{EXPAND,NOT_EXPAND};

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


pub static FUNCTION_ATTRIBUTES_BUILTIN: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::PURE.clone(), isFunctionPointer: false, isBuiltin: FunctionBuiltin::FUNCTION_BUILTIN { name: None, unboxArgs: false }, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_BUILTIN_IMPURE: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::IMPURE.clone(), isFunctionPointer: false, isBuiltin: FunctionBuiltin::FUNCTION_BUILTIN { name: None, unboxArgs: false }, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_DEFAULT: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::DEFAULT_INLINE, generateEvents: false, purity: Purity::PURE.clone(), isFunctionPointer: false, isBuiltin: crate::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

pub static FUNCTION_ATTRIBUTES_IMPURE: std::sync::LazyLock<FunctionAttributes> = std::sync::LazyLock::new(|| { FunctionAttributes { inline: crate::DAE::InlineType::NO_INLINE, generateEvents: false, purity: Purity::IMPURE.clone(), isFunctionPointer: false, isBuiltin: crate::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: crate::DAE::FunctionParallelism::FP_NON_PARALLEL } });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncArg {
    pub name: ArcStr,
    pub ty: Arc<Type>,
    pub r#const: Const,
    pub par: VarParallelism,
    pub defaultBinding: Option<Arc<Exp>>,
}

pub type FUNCARG = FuncArg;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Function {
    FUNCTION {
        path: Arc<Absyn::Path>,
        functions: Arc<metamodelica::List<FunctionDefinition>>,
        type_: Arc<Type>,
        visibility: SCode::Visibility,
        partialPrefix: bool,
        isImpure: bool,
        inlineType: InlineType,
        unusedInputs: Arc<metamodelica::List<i32>>,
        source: Arc<ElementSource>,
        comment: Option<Arc<SCode::Comment>>,
    },
    RECORD_CONSTRUCTOR {
        path: Arc<Absyn::Path>,
        type_: Arc<Type>,
        source: Arc<ElementSource>,
    },
}
pub use self::Function::{FUNCTION,RECORD_CONSTRUCTOR};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionAttributes {
    pub inline: InlineType,
    pub generateEvents: bool,
    pub purity: Purity,
    pub isFunctionPointer: bool,
    pub isBuiltin: FunctionBuiltin,
    pub functionParallelism: FunctionParallelism,
}

pub type FUNCTION_ATTRIBUTES = FunctionAttributes;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionBuiltin {
    FUNCTION_NOT_BUILTIN,
    FUNCTION_BUILTIN {
        name: Option<ArcStr>,
        unboxArgs: bool,
    },
    FUNCTION_BUILTIN_PTR,
}
pub use self::FunctionBuiltin::{FUNCTION_NOT_BUILTIN,FUNCTION_BUILTIN,FUNCTION_BUILTIN_PTR};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionDefinition {
    FUNCTION_DEF {
        body: Arc<metamodelica::List<Arc<Element>>>,
    },
    FUNCTION_EXT {
        body: Arc<metamodelica::List<Arc<Element>>>,
        externalDecl: ExternalDecl,
    },
    FUNCTION_DER_MAPPER {
        derivedFunction: Arc<Absyn::Path>,
        derivativeFunction: Arc<Absyn::Path>,
        derivativeOrder: i32,
        conditionRefs: Arc<metamodelica::List<(i32, derivativeCond)>>,
        defaultDerivative: Option<Arc<Absyn::Path>>,
        lowerOrderDerivatives: Arc<metamodelica::List<Arc<Absyn::Path>>>,
    },
    FUNCTION_INVERSE {
        inputParam: Arc<ComponentRef>,
        inverseCall: Arc<Exp>,
    },
    FUNCTION_PARTIAL_DERIVATIVE {
        derivedFunction: Arc<Absyn::Path>,
        derivedVars: Arc<metamodelica::List<ArcStr>>,
    },
}
pub use self::FunctionDefinition::{FUNCTION_DEF,FUNCTION_EXT,FUNCTION_DER_MAPPER,FUNCTION_INVERSE,FUNCTION_PARTIAL_DERIVATIVE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionParallelism {
    FP_NON_PARALLEL,
    FP_PARALLEL_FUNCTION,
    FP_KERNEL_FUNCTION,
}
pub use self::FunctionParallelism::{FP_NON_PARALLEL,FP_PARALLEL_FUNCTION,FP_KERNEL_FUNCTION};

pub type Ident = ArcStr;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlineType {
    NORM_INLINE,
    BUILTIN_EARLY_INLINE,
    EARLY_INLINE,
    DEFAULT_INLINE,
    NO_INLINE,
    AFTER_INDEX_RED_INLINE,
}
pub use self::InlineType::{NORM_INLINE,BUILTIN_EARLY_INLINE,EARLY_INLINE,DEFAULT_INLINE,NO_INLINE,AFTER_INDEX_RED_INLINE};

pub type InstDims = Arc<metamodelica::List<Arc<Dimension>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchCase {
    pub patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    pub patternGuard: Option<Arc<Exp>>,
    pub localDecls: Arc<metamodelica::List<Arc<Element>>>,
    pub body: Arc<metamodelica::List<Arc<Statement>>>,
    pub result: Option<Arc<Exp>>,
    pub resultInfo: SourceInfo,
    pub jump: i32,
    pub info: SourceInfo,
}

pub type CASE = MatchCase;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchType {
    MATCHCONTINUE,
    TRY_STACKOVERFLOW,
    MATCH {
        switch: Option<(i32, Arc<Type>, i32)>,
    },
}
pub use self::MatchType::{MATCHCONTINUE,TRY_STACKOVERFLOW,MATCH};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mod {
    MOD {
        finalPrefix: SCode::Final,
        eachPrefix: SCode::Each,
        subModLst: Arc<metamodelica::List<Arc<SubMod>>>,
        binding: Option<EqMod>,
        info: SourceInfo,
    },
    REDECL {
        finalPrefix: SCode::Final,
        eachPrefix: SCode::Each,
        element: Arc<SCode::Element>,
        r#mod: Arc<Mod>,
    },
    NOMOD,
}
pub use self::Mod::{MOD,REDECL,NOMOD};

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
    MUL_ARR {
        ty: Arc<Type>,
    },
    DIV_ARR {
        ty: Arc<Type>,
    },
    MUL_ARRAY_SCALAR {
        ty: Arc<Type>,
    },
    ADD_ARRAY_SCALAR {
        ty: Arc<Type>,
    },
    SUB_SCALAR_ARRAY {
        ty: Arc<Type>,
    },
    MUL_SCALAR_PRODUCT {
        ty: Arc<Type>,
    },
    MUL_MATRIX_PRODUCT {
        ty: Arc<Type>,
    },
    DIV_ARRAY_SCALAR {
        ty: Arc<Type>,
    },
    DIV_SCALAR_ARRAY {
        ty: Arc<Type>,
    },
    POW_ARRAY_SCALAR {
        ty: Arc<Type>,
    },
    POW_SCALAR_ARRAY {
        ty: Arc<Type>,
    },
    POW_ARR {
        ty: Arc<Type>,
    },
    POW_ARR2 {
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
        fqName: Arc<Absyn::Path>,
    },
}
pub use self::Operator::{ADD,SUB,MUL,DIV,POW,UMINUS,UMINUS_ARR,ADD_ARR,SUB_ARR,MUL_ARR,DIV_ARR,MUL_ARRAY_SCALAR,ADD_ARRAY_SCALAR,SUB_SCALAR_ARRAY,MUL_SCALAR_PRODUCT,MUL_MATRIX_PRODUCT,DIV_ARRAY_SCALAR,DIV_SCALAR_ARRAY,POW_ARRAY_SCALAR,POW_SCALAR_ARRAY,POW_ARR,POW_ARR2,AND,OR,NOT,LESS,LESSEQ,GREATER,GREATEREQ,EQUAL,NEQUAL,USERDEFINED};

pub static PI: std::sync::LazyLock<Arc<Exp>> = std::sync::LazyLock::new(|| { Arc::new(Exp::RCONST { real: metamodelica::OrderedFloat(3.1415926535897932384626433832795028841971693993751058_f64) }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pattern {
    PAT_WILD,
    PAT_CONSTANT {
        ty: Option<Arc<Type>>,
        exp: Arc<Exp>,
    },
    PAT_AS {
        id: ArcStr,
        ty: Option<Arc<Type>>,
        attr: Arc<Attributes>,
        pat: Arc<Pattern>,
    },
    PAT_AS_FUNC_PTR {
        id: ArcStr,
        pat: Arc<Pattern>,
    },
    PAT_META_TUPLE {
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    },
    PAT_CALL_TUPLE {
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
    },
    PAT_CONS {
        head: Arc<Pattern>,
        tail: Arc<Pattern>,
    },
    PAT_CALL {
        name: Arc<Absyn::Path>,
        index: i32,
        patterns: Arc<metamodelica::List<Arc<Pattern>>>,
        fields: Arc<metamodelica::List<Arc<Var>>>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        knownSingleton: bool,
    },
    PAT_CALL_NAMED {
        name: Arc<Absyn::Path>,
        patterns: Arc<metamodelica::List<(Arc<Pattern>, ArcStr, Arc<Type>)>>,
    },
    PAT_SOME {
        pat: Arc<Pattern>,
    },
}
pub use self::Pattern::{PAT_WILD,PAT_CONSTANT,PAT_AS,PAT_AS_FUNC_PTR,PAT_META_TUPLE,PAT_CALL_TUPLE,PAT_CONS,PAT_CALL,PAT_CALL_NAMED,PAT_SOME};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prefix {
    NOPRE,
    PREFIX {
        compPre: Arc<ComponentPrefix>,
        classPre: ClassPrefix,
    },
}
pub use self::Prefix::{NOPRE,PREFIX};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Properties {
    PROP {
        type_: Arc<Type>,
        constFlag: Const,
    },
    PROP_TUPLE {
        type_: Arc<Type>,
        tupleConst: Arc<TupleConst>,
    },
}
pub use self::Properties::{PROP,PROP_TUPLE};

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReductionInfo {
    pub path: Arc<Absyn::Path>,
    pub iterType: Absyn::ReductionIterType,
    pub exprType: Arc<Type>,
    pub defaultValue: Option<Arc<Values::Value>>,
    pub foldName: ArcStr,
    pub resultName: ArcStr,
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


pub type ReductionIterators = Arc<metamodelica::List<Arc<ReductionIterator>>>;

pub type StartValue = Option<Arc<Exp>>;

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
pub enum Statement {
    STMT_ASSIGN {
        type_: Arc<Type>,
        exp1: Arc<Exp>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_TUPLE_ASSIGN {
        type_: Arc<Type>,
        expExpLst: Arc<metamodelica::List<Arc<Exp>>>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_ASSIGN_ARR {
        type_: Arc<Type>,
        lhs: Arc<Exp>,
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_IF {
        exp: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        else_: Arc<Else>,
        source: Arc<ElementSource>,
    },
    STMT_FOR {
        type_: Arc<Type>,
        iterIsArray: bool,
        iter: Ident,
        range: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        source: Arc<ElementSource>,
    },
    STMT_PARFOR {
        type_: Arc<Type>,
        iterIsArray: bool,
        iter: Ident,
        range: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        loopPrlVars: Arc<metamodelica::List<(Arc<ComponentRef>, SourceInfo)>>,
        source: Arc<ElementSource>,
    },
    STMT_WHILE {
        exp: Arc<Exp>,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        source: Arc<ElementSource>,
    },
    STMT_WHEN {
        exp: Arc<Exp>,
        conditions: Arc<metamodelica::List<Arc<ComponentRef>>>,
        initialCall: bool,
        statementLst: Arc<metamodelica::List<Arc<Statement>>>,
        elseWhen: Option<Arc<Statement>>,
        source: Arc<ElementSource>,
    },
    STMT_ASSERT {
        cond: Arc<Exp>,
        msg: Arc<Exp>,
        level: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_TERMINATE {
        msg: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_REINIT {
        var: Arc<Exp>,
        value: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_NORETCALL {
        exp: Arc<Exp>,
        source: Arc<ElementSource>,
    },
    STMT_RETURN {
        source: Arc<ElementSource>,
    },
    STMT_BREAK {
        source: Arc<ElementSource>,
    },
    STMT_CONTINUE {
        source: Arc<ElementSource>,
    },
    STMT_ARRAY_INIT {
        name: ArcStr,
        ty: Arc<Type>,
        source: Arc<ElementSource>,
    },
    STMT_FAILURE {
        body: Arc<metamodelica::List<Arc<Statement>>>,
        source: Arc<ElementSource>,
    },
}
pub use self::Statement::{STMT_ASSIGN,STMT_TUPLE_ASSIGN,STMT_ASSIGN_ARR,STMT_IF,STMT_FOR,STMT_PARFOR,STMT_WHILE,STMT_WHEN,STMT_ASSERT,STMT_TERMINATE,STMT_REINIT,STMT_NORETCALL,STMT_RETURN,STMT_BREAK,STMT_CONTINUE,STMT_ARRAY_INIT,STMT_FAILURE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubMod {
    pub ident: Ident,
    pub r#mod: Arc<Mod>,
}

pub type NAMEMOD = SubMod;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Subscript {
    WHOLEDIM,
    SLICE {
        exp: Arc<Exp>,
    },
    INDEX {
        exp: Arc<Exp>,
    },
    WHOLE_NONEXP {
        exp: Arc<Exp>,
    },
}
pub use self::Subscript::{WHOLEDIM,SLICE,INDEX,WHOLE_NONEXP};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SymbolicOperation {
    FLATTEN {
        scode: Arc<SCode::Equation>,
        dae: Option<Arc<Element>>,
    },
    SIMPLIFY {
        before: Arc<EquationExp>,
        after: Arc<EquationExp>,
    },
    SUBSTITUTION {
        substitutions: Arc<metamodelica::List<Arc<Exp>>>,
        source: Arc<Exp>,
    },
    OP_INLINE {
        before: Arc<EquationExp>,
        after: Arc<EquationExp>,
    },
    OP_SCALARIZE {
        before: Arc<EquationExp>,
        index: i32,
        after: Arc<EquationExp>,
    },
    OP_DIFFERENTIATE {
        cr: Arc<ComponentRef>,
        before: Arc<Exp>,
        after: Arc<Exp>,
    },
    SOLVE {
        cr: Arc<ComponentRef>,
        exp1: Arc<Exp>,
        exp2: Arc<Exp>,
        res: Arc<Exp>,
        assertConds: Arc<metamodelica::List<Arc<Exp>>>,
    },
    SOLVED {
        cr: Arc<ComponentRef>,
        exp: Arc<Exp>,
    },
    LINEAR_SOLVED {
        vars: Arc<metamodelica::List<Arc<ComponentRef>>>,
        jac: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>,
        rhs: Arc<metamodelica::List<metamodelica::Real>>,
        result: Arc<metamodelica::List<metamodelica::Real>>,
    },
    NEW_DUMMY_DER {
        chosen: Arc<ComponentRef>,
        candidates: Arc<metamodelica::List<Arc<ComponentRef>>>,
    },
    OP_RESIDUAL {
        e1: Arc<Exp>,
        e2: Arc<Exp>,
        e: Arc<Exp>,
    },
}
pub use self::SymbolicOperation::{FLATTEN,SIMPLIFY,SUBSTITUTION,OP_INLINE,OP_SCALARIZE,OP_DIFFERENTIATE,SOLVE,SOLVED,LINEAR_SOLVED,NEW_DUMMY_DER,OP_RESIDUAL};

pub static T_ANYTYPE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ANYTYPE { anyClassType: None }) });

pub static T_ARRAY_BOOL_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_BOOL_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_INT_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_INTEGER_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_REAL_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_REAL_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ARRAY_STRING_NODIM: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ARRAY { ty: T_STRING_DEFAULT.clone(), dims: list![Arc::new(crate::DAE::Dimension::DIM_UNKNOWN)] }) });

pub static T_ASSERTIONLEVEL: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (literal!("AssertionLevel")).clone() }) }), names: list![(literal!("warning")).clone(), (literal!("error")).clone()], literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }) });

pub static T_BOOL_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_BOOL_DEFAULT.clone() }) });

pub static T_BOOL_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_BOOL { varLst: metamodelica::nil() }) });

pub static T_CLOCK_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_CLOCK { varLst: metamodelica::nil() }) });

pub static T_COMPLEX_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }) });

pub static T_COMPLEX_DEFAULT_RECORD: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }) });

pub static T_ENUMERATION_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_ENUMERATION { index: None, path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }) });

pub static T_INTEGER_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_INTEGER_DEFAULT.clone() }) });

pub static T_INTEGER_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_INTEGER { varLst: metamodelica::nil() }) });

pub static T_METABOXED_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_METALIST_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METALIST { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_METATYPE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METATYPE { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_NONE_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METAOPTION { ty: T_UNKNOWN_DEFAULT.clone() }) });

pub static T_NORETCALL_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(crate::DAE::Type::T_NORETCALL) });

pub static T_REAL_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_REAL_DEFAULT.clone() }) });

pub static T_REAL_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_REAL { varLst: metamodelica::nil() }) });

pub static T_SOURCEINFO_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METAUNIONTYPE { paths: list![Arc::new(Absyn::Path::QUALIFIED { name: (literal!("SourceInfo")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SOURCEINFO")).clone() }) })], typeVars: metamodelica::nil(), knownSingleton: true, singletonType: Arc::new(EvaluateSingletonType::EVAL_SINGLETON_KNOWN_TYPE { ty: T_SOURCEINFO_DEFAULT_METARECORD.clone() }), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SourceInfo")).clone() }) }) });

pub static T_SOURCEINFO_DEFAULT_METARECORD: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METARECORD { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("SourceInfo")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SOURCEINFO")).clone() }) }), utPath: Arc::new(Absyn::Path::IDENT { name: (literal!("SourceInfo")).clone() }), typeVars: metamodelica::nil(), index: 1, fields: list![Arc::new(Var { name: (literal!("fileName")).clone(), attributes: dummyAttrVar.clone(), ty: T_STRING_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("isReadOnly")).clone(), attributes: dummyAttrVar.clone(), ty: T_BOOL_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lineNumberStart")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("columnNumberStart")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lineNumberEnd")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("columnNumberEnd")).clone(), attributes: dummyAttrVar.clone(), ty: T_INTEGER_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(Var { name: (literal!("lastModification")).clone(), attributes: dummyAttrVar.clone(), ty: T_REAL_DEFAULT.clone(), binding: Arc::new(crate::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], knownSingleton: true }) });

pub static T_STRING_BOXED: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_METABOXED { ty: T_STRING_DEFAULT.clone() }) });

pub static T_STRING_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(Type::T_STRING { varLst: metamodelica::nil() }) });

pub static T_UNKNOWN_DEFAULT: std::sync::LazyLock<Arc<Type>> = std::sync::LazyLock::new(|| { Arc::new(crate::DAE::Type::T_UNKNOWN) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TailCall {
    NO_TAIL,
    TAIL {
        vars: Arc<metamodelica::List<ArcStr>>,
        outVars: Arc<metamodelica::List<ArcStr>>,
    },
}
pub use self::TailCall::{NO_TAIL,TAIL};

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
    T_ENUMERATION {
        index: Option<i32>,
        path: Arc<Absyn::Path>,
        names: Arc<metamodelica::List<ArcStr>>,
        literalVarLst: Arc<metamodelica::List<Arc<Var>>>,
        attributeLst: Arc<metamodelica::List<Arc<Var>>>,
    },
    T_ARRAY {
        ty: Arc<Type>,
        dims: Dimensions,
    },
    T_NORETCALL,
    T_UNKNOWN,
    T_COMPLEX {
        complexClassType: ClassInf::State,
        varLst: Arc<metamodelica::List<Arc<Var>>>,
        equalityConstraint: EqualityConstraint,
        usedExternally: bool,
    },
    T_SUBTYPE_BASIC {
        complexClassType: ClassInf::State,
        varLst: Arc<metamodelica::List<Arc<Var>>>,
        complexType: Arc<Type>,
        equalityConstraint: EqualityConstraint,
    },
    T_FUNCTION {
        funcArg: Arc<metamodelica::List<Arc<FuncArg>>>,
        funcResultType: Arc<Type>,
        functionAttributes: FunctionAttributes,
        path: Arc<Absyn::Path>,
    },
    T_FUNCTION_REFERENCE_VAR {
        functionType: Arc<Type>,
    },
    T_FUNCTION_REFERENCE_FUNC {
        builtin: bool,
        functionType: Arc<Type>,
    },
    T_TUPLE {
        types: Arc<metamodelica::List<Arc<Type>>>,
        names: Option<Arc<metamodelica::List<ArcStr>>>,
    },
    T_CODE {
        ty: CodeType,
    },
    T_ANYTYPE {
        anyClassType: Option<ClassInf::State>,
    },
    T_METALIST {
        ty: Arc<Type>,
    },
    T_METATUPLE {
        types: Arc<metamodelica::List<Arc<Type>>>,
    },
    T_METAOPTION {
        ty: Arc<Type>,
    },
    T_METAUNIONTYPE {
        paths: Arc<metamodelica::List<Arc<Absyn::Path>>>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        knownSingleton: bool,
        singletonType: Arc<EvaluateSingletonType>,
        path: Arc<Absyn::Path>,
    },
    T_METARECORD {
        path: Arc<Absyn::Path>,
        utPath: Arc<Absyn::Path>,
        typeVars: Arc<metamodelica::List<Arc<Type>>>,
        index: i32,
        fields: Arc<metamodelica::List<Arc<Var>>>,
        knownSingleton: bool,
    },
    T_METAARRAY {
        ty: Arc<Type>,
    },
    T_METABOXED {
        ty: Arc<Type>,
    },
    T_METAPOLYMORPHIC {
        name: ArcStr,
    },
    T_METATYPE {
        ty: Arc<Type>,
    },
}
pub use self::Type::{T_INTEGER,T_REAL,T_STRING,T_BOOL,T_CLOCK,T_ENUMERATION,T_ARRAY,T_NORETCALL,T_UNKNOWN,T_COMPLEX,T_SUBTYPE_BASIC,T_FUNCTION,T_FUNCTION_REFERENCE_VAR,T_FUNCTION_REFERENCE_FUNC,T_TUPLE,T_CODE,T_ANYTYPE,T_METALIST,T_METATUPLE,T_METAOPTION,T_METAUNIONTYPE,T_METARECORD,T_METAARRAY,T_METABOXED,T_METAPOLYMORPHIC,T_METATYPE};

pub const UNIQUEIO: &'static str = "$unique$outer$";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Uncertainty {
    GIVEN,
    SOUGHT,
    REFINE,
    PROPAGATE,
}
pub use self::Uncertainty::{GIVEN,SOUGHT,REFINE,PROPAGATE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var {
    pub name: Ident,
    pub attributes: Arc<Attributes>,
    pub ty: Arc<Type>,
    pub binding: Arc<Binding>,
    pub bind_from_outside: bool,
    pub constOfForIteratorRange: Option<Const>,
}

pub type TYPES_VAR = Var;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarDirection {
    INPUT,
    OUTPUT,
    BIDIR,
}
pub use self::VarDirection::{INPUT,OUTPUT,BIDIR};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarInnerOuter {
    INNER,
    OUTER,
    INNER_OUTER,
    NOT_INNER_OUTER,
}
pub use self::VarInnerOuter::{INNER,OUTER,INNER_OUTER,NOT_INNER_OUTER};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarKind {
    VARIABLE,
    DISCRETE,
    PARAM,
    CONST,
}
pub use self::VarKind::{VARIABLE,DISCRETE,PARAM,CONST};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarParallelism {
    PARGLOBAL,
    PARLOCAL,
    NON_PARALLEL,
}
pub use self::VarParallelism::{PARGLOBAL,PARLOCAL,NON_PARALLEL};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VarVisibility {
    PUBLIC,
    PROTECTED,
}
pub use self::VarVisibility::{PUBLIC,PROTECTED};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VariableAttributes {
    VAR_ATTR_REAL {
        quantity: Option<Arc<Exp>>,
        unit: Option<Arc<Exp>>,
        displayUnit: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        nominal: Option<Arc<Exp>>,
        stateSelectOption: Option<StateSelect>,
        uncertainOption: Option<Uncertainty>,
        distributionOption: Option<Arc<Distribution>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_INT {
        quantity: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        uncertainOption: Option<Uncertainty>,
        distributionOption: Option<Arc<Distribution>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_BOOL {
        quantity: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_CLOCK {
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
    },
    VAR_ATTR_STRING {
        quantity: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
    VAR_ATTR_ENUMERATION {
        quantity: Option<Arc<Exp>>,
        min: Option<Arc<Exp>>,
        max: Option<Arc<Exp>>,
        start: Option<Arc<Exp>>,
        fixed: Option<Arc<Exp>>,
        equationBound: Option<Arc<Exp>>,
        isProtected: Option<bool>,
        finalPrefix: Option<bool>,
        startOrigin: Option<Arc<Exp>>,
    },
}
pub use self::VariableAttributes::{VAR_ATTR_REAL,VAR_ATTR_INT,VAR_ATTR_BOOL,VAR_ATTR_CLOCK,VAR_ATTR_STRING,VAR_ATTR_ENUMERATION};

pub const auxNamePrefix: &'static str = "$AUX";

pub static callAttrBuiltinBool: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_BOOL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureBool: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_BOOL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureInteger: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_INTEGER_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureReal: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_REAL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinImpureString: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_STRING_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinInteger: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_INTEGER_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinOther: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_UNKNOWN_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinReal: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_REAL_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrBuiltinString: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_STRING_DEFAULT.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static callAttrOther: std::sync::LazyLock<Arc<CallAttributes>> = std::sync::LazyLock::new(|| { Arc::new(CallAttributes { ty: T_UNKNOWN_DEFAULT.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: crate::DAE::InlineType::NO_INLINE, tailCall: crate::DAE::TailCall::NO_TAIL }) });

pub static crefTime: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("time")).clone(), identType: T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

pub static crefTimeState: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("$time")).clone(), identType: T_REAL_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum derivativeCond {
    ZERO_DERIVATIVE,
    NO_DERIVATIVE {
        binding: Arc<Exp>,
    },
}
pub use self::derivativeCond::{ZERO_DERIVATIVE,NO_DERIVATIVE};

pub const derivativeNamePrefix: &'static str = "$DER";

pub static dummyAttrConst: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::CONST, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrInput: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::INPUT, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrParam: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::PARAM, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static dummyAttrVar: std::sync::LazyLock<Arc<Attributes>> = std::sync::LazyLock::new(|| { Arc::new(Attributes { connectorType: Arc::new(crate::DAE::ConnectorType::NON_CONNECTOR), parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, visibility: crate::SCode::Visibility::PUBLIC }) });

pub static emptyCref: std::sync::LazyLock<Arc<ComponentRef>> = std::sync::LazyLock::new(|| { Arc::new(ComponentRef::CREF_IDENT { ident: (literal!("")).clone(), identType: T_UNKNOWN_DEFAULT.clone(), subscriptLst: metamodelica::nil() }) });

pub static emptyDae: std::sync::LazyLock<DAElist> = std::sync::LazyLock::new(|| { DAElist { elementLst: metamodelica::nil() } });

pub static emptyElementSource: std::sync::LazyLock<Arc<ElementSource>> = std::sync::LazyLock::new(|| { Arc::new(ElementSource { info: Absyn::dummyInfo.clone(), partOfLst: metamodelica::nil(), instance: Arc::new(crate::DAE::ComponentPrefix::NOCOMPPRE), connectEquationOptLst: metamodelica::nil(), typeLst: metamodelica::nil(), operations: metamodelica::nil(), comment: metamodelica::nil() }) });

pub static emptyVarAttrBool: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_BOOL { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrClock: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_CLOCK { isProtected: None, finalPrefix: None }) });

pub static emptyVarAttrEnum: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_ENUMERATION { quantity: None, min: None, max: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrInt: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_INT { quantity: None, min: None, max: None, start: None, fixed: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrReal: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, uncertainOption: None, distributionOption: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub static emptyVarAttrString: std::sync::LazyLock<Arc<VariableAttributes>> = std::sync::LazyLock::new(|| { Arc::new(VariableAttributes::VAR_ATTR_STRING { quantity: None, start: None, fixed: None, equationBound: None, isProtected: None, finalPrefix: None, startOrigin: None }) });

pub const partialDerivativeNamePrefix: &'static str = "$pDER";

pub const preNamePrefix: &'static str = "$PRE";

pub const previousNamePrefix: &'static str = "$CLKPRE";

pub const startNamePrefix: &'static str = "$START";

