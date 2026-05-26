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

use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
//NF imports
// Util imports
pub mod BackendInfo {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct BackendInfo {
        /// Structural kind: state, algebraic...
        pub varKind: Arc<VariableKind::VariableKind>,
        /// values on built-in attributes
        pub attributes: Arc<VariableAttributes::VariableAttributes>,
        /// values on annotations (vendor specific)
        pub annotations: Arc<Annotations::Annotations>,
        /// Pointer (var -> pre) or (pre -> var) if existent.
        pub var_pre: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        /// Pointer (var -> seed) or (seed -> var) if existent.
        pub var_seed: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        /// Pointer (var -> pder) or (pder -> var) if existent.
        pub var_pder: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        /// Pointer (var -> start) or (start -> var) if existent.
        pub var_start: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
        /// record parent if it is part of a record.
        pub parent: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
    }

    impl Default for BackendInfo {
        fn default() -> Self {
            Self {
                varKind: Default::default(),
                attributes: Default::default(),
                annotations: Default::default(),
                var_pre: Default::default(),
                var_seed: Default::default(),
                var_pder: Default::default(),
                var_start: Default::default(),
                parent: Default::default(),
            }
        }
    }

    pub type BACKEND_INFO = BackendInfo;

    pub fn toString(mut backendInfo: Arc<BackendInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = (VariableAttributes::toString(backendInfo.attributes.clone())?).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*VariableKind::toString(backendInfo.varKind.clone())); __mm_s.push_str(&*if (r#str.clone() == literal!("")) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }}); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn map(mut binfo: Arc<BackendInfo>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<BackendInfo> {
        pub type expFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

        let mut binfo: Arc<BackendInfo> = binfo;
        assign_field!(binfo.attributes = VariableAttributes::map(binfo.attributes.clone(), func.clone()));
        binfo
    }

    pub fn getVarKind(mut binfo: Arc<BackendInfo>) -> Arc<VariableKind::VariableKind> {
        let mut varKind: Arc<VariableKind::VariableKind> = binfo.varKind.clone();
        varKind
    }

    pub fn setVarKind(mut binfo: Arc<BackendInfo>, mut varKind: Arc<VariableKind::VariableKind>) -> Arc<BackendInfo> {
        let mut binfo: Arc<BackendInfo> = binfo;
        assign_field!(binfo.varKind = varKind.clone());
        binfo
    }

    pub fn setStateSelect(mut info: Arc<BackendInfo>, mut stateSelect_val: StateSelect, mut overwrite: bool) -> Arc<BackendInfo> {
        let mut info: Arc<BackendInfo> = info;
        assign_field!(info.attributes = VariableAttributes::setStateSelect(info.attributes.clone(), stateSelect_val.clone(), overwrite.clone()));
        info
    }

    pub fn setParent(mut binfo: Arc<BackendInfo>, mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Arc<BackendInfo> {
        let mut binfo: Arc<BackendInfo> = binfo;
        assign_field!(binfo.parent = Some(parent.clone()));
        binfo
    }

    pub type setPartner = fn(Arc<BackendInfo>, Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Result<Arc<BackendInfo>>;

    pub fn setVarPre(mut binfo: Arc<BackendInfo>, mut var_ptr: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Arc<BackendInfo> {
        let mut binfo: Arc<BackendInfo> = binfo;
        assign_field!(binfo.var_pre = var_ptr.clone());
        binfo
    }

    pub fn setVarSeed(mut binfo: Arc<BackendInfo>, mut var_ptr: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Arc<BackendInfo> {
        let mut binfo: Arc<BackendInfo> = binfo;
        assign_field!(binfo.var_seed = var_ptr.clone());
        binfo
    }

    pub fn setVarPDer(mut binfo: Arc<BackendInfo>, mut var_ptr: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Arc<BackendInfo> {
        let mut binfo: Arc<BackendInfo> = binfo;
        assign_field!(binfo.var_pder = var_ptr.clone());
        binfo
    }

    pub fn setVarStart(mut binfo: Arc<BackendInfo>, mut var_ptr: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>) -> Arc<BackendInfo> {
        let mut binfo: Arc<BackendInfo> = binfo;
        assign_field!(binfo.var_start = var_ptr.clone());
        binfo
    }

    pub fn setAttributes(mut binfo: Arc<BackendInfo>, mut attributes: Arc<VariableAttributes::VariableAttributes>, mut annotations: Arc<Annotations::Annotations>) -> Arc<BackendInfo> {
        let mut binfo: Arc<BackendInfo> = binfo;
        assign_field!(
            binfo.attributes = attributes.clone(),
            binfo.annotations = annotations.clone()
        );
        binfo
    }

    pub fn setHideResult(mut binfo: Arc<BackendInfo>, mut hideResult: bool) -> Arc<BackendInfo> {
        let mut binfo: Arc<BackendInfo> = binfo;
        binfo = (::match_deref::match_deref! { match &(binfo.clone()) {
        Deref @ BackendInfo { annotations: anno @ Deref @ Annotations::ANNOTATIONS { .. }, .. } => {
            let mut anno = (*anno).clone();
            assign_field!(anno.hideResult = hideResult.clone());
            assign_field!(binfo.annotations = anno.clone());
            binfo.clone()
        },
        _ => {
            binfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        binfo
    }

    pub fn scalarize(mut binfo: Arc<BackendInfo>, mut length: i32) -> Result<Arc<metamodelica::List<Arc<BackendInfo>>>> {
        let mut binfo_list: Arc<metamodelica::List<Arc<BackendInfo>>> = metamodelica::nil();
        binfo_list = (::match_deref::match_deref! { match &(binfo.varKind.clone()) {
        Deref @ VariableKind::FRONTEND_DUMMY => {
            List::fill(binfo.clone(), length.clone())
        },
        _ => {
            let mut scalar_attributes: Arc<metamodelica::List<Arc<VariableAttributes::VariableAttributes>>> = metamodelica::nil();
            scalar_attributes = VariableAttributes::scalarize(binfo.attributes.clone(), length.clone())?;
            {
        let mut __acc: Arc<metamodelica::List<Arc<BackendInfo>>> = metamodelica::nil();
        for mut attr in (scalar_attributes.clone()).into_iter().cloned() {
            let __x = Arc::new(BackendInfo { varKind: binfo.varKind.clone(), attributes: attr.clone(), annotations: binfo.annotations.clone(), var_pre: binfo.var_pre.clone(), var_seed: binfo.var_seed.clone(), var_pder: binfo.var_pder.clone(), var_start: binfo.var_start.clone(), parent: binfo.parent.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(binfo_list)
    }

}

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<BackendInfo::BackendInfo>
// Expr: Constructor { name: 'NFBackendExtension.BackendInfo.BACKEND_INFO', args: [Constructor { name: 'NFBackendExtension.VariableKind.FRONTEND_DUMMY', args: [], named_args: [], ty: RustUnitVariant, field_names: [] }, Var { name: 'EMPTY_VAR_ATTR_REAL', segments: [CrefSegment { name: 'EMPTY_VAR_ATTR_REAL', subscripts: [] }], ty: RustEnum('NFBackendExtension.VariableAttributes') }, Var { name: 'EMPTY_ANNOTATIONS', segments: [CrefSegment { name: 'EMPTY_ANNOTATIONS', subscripts: [] }], ty: AliasTo('NFBackendExtension.Annotations') }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFBackendExtension.BackendInfo'), field_names: ['varKind', 'attributes', 'annotations', 'var_pre', 'var_seed', 'var_pder', 'var_start', 'parent'] }
pub fn DUMMY_BACKEND_INFO() -> Arc<BackendInfo::BackendInfo> { todo!("non-Sync, non-const-emittable constant DUMMY_BACKEND_INFO — extend codegen") }

pub mod VariableKind {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    pub enum VariableKind {
        TIME,
        ALGEBRAIC,
        STATE {
            /// how often this states was differentiated
            index: i32,
            /// pointer to the derivative
            derivative: Option<Pointer::Pointer<Arc<Variable::NFVariable>>>,
            /// false if it was forced by StateSelect.always or StateSelect.prefer or generated by index reduction
            natural: bool,
        },
        STATE_DER {
            /// Original state
            state: Pointer::Pointer<Arc<Variable::NFVariable>>,
            /// Optional alias state expression. Result of differentiating the state if existant!
            alias: Option<Pointer::Pointer<Arc<Expression::NFExpression>>>,
        },
        DUMMY_DER {
            /// corresponding dummy state
            dummy_state: Pointer::Pointer<Arc<Variable::NFVariable>>,
        },
        DUMMY_STATE {
            /// corresponding dummy derivative
            dummy_der: Pointer::Pointer<Arc<Variable::NFVariable>>,
        },
        DISCRETE,
        DISCRETE_STATE,
        PREVIOUS,
        CLOCK,
        CLOCKED,
        PARAMETER {
            /// if the parameter is resizable, this is the computed optimal size
            resize_value: Option<i32>,
        },
        CONSTANT,
        ITERATOR,
        RECORD {
            children: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>,
            min_var: Variability,
            max_var: Variability,
        },
        START {
            /// Pointer to the corresponding original variable.
            original: Pointer::Pointer<Arc<Variable::NFVariable>>,
        },
        EXTOBJ {
            fullClassName: Arc<Absyn::Path>,
        },
        JAC_VAR,
        JAC_TMP_VAR,
        SEED_VAR,
        OPT_CONSTR,
        OPT_FCONSTR,
        OPT_INPUT_WITH_DER,
        OPT_INPUT_DER,
        OPT_TGRID,
        OPT_LOOP_INPUT {
            replaceCref: Arc<ComponentRef::NFComponentRef>,
        },
        /// algebraic state used by inline solver
        ALG_STATE,
        /// algebraic state old value used by inline solver
        ALG_STATE_OLD,
        RESIDUAL_VAR,
        /// auxiliary variable used for DAEmode
        DAE_AUX_VAR,
        /// used in SIMCODE, iteration variables in algebraic loops
        LOOP_ITERATION,
        /// used in SIMCODE, inner variables of a torn algebraic loop
        LOOP_SOLVED,
        /// Undefined variable type. Only to be used during frontend phase.
        FRONTEND_DUMMY,
    }
    impl Default for VariableKind {
        fn default() -> Self { Self::TIME }
    }
    pub use self::VariableKind::{TIME,ALGEBRAIC,STATE,STATE_DER,DUMMY_DER,DUMMY_STATE,DISCRETE,DISCRETE_STATE,PREVIOUS,CLOCK,CLOCKED,PARAMETER,CONSTANT,ITERATOR,RECORD,START,EXTOBJ,JAC_VAR,JAC_TMP_VAR,SEED_VAR,OPT_CONSTR,OPT_FCONSTR,OPT_INPUT_WITH_DER,OPT_INPUT_DER,OPT_TGRID,OPT_LOOP_INPUT,ALG_STATE,ALG_STATE_OLD,RESIDUAL_VAR,DAE_AUX_VAR,LOOP_ITERATION,LOOP_SOLVED,FRONTEND_DUMMY};
    pub fn toString(mut varKind: Arc<VariableKind>) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(varKind.clone()) {
        Deref @ TIME { .. } => literal!("[TIME]"),
        Deref @ ALGEBRAIC { .. } => literal!("[ALGB]"),
        Deref @ STATE { .. } => literal!("[STAT]"),
        Deref @ STATE_DER { .. } => literal!("[DER-]"),
        Deref @ DUMMY_DER { .. } => literal!("[DDER]"),
        Deref @ DUMMY_STATE { .. } => literal!("[DSTA]"),
        Deref @ DISCRETE { .. } => literal!("[DISC]"),
        Deref @ DISCRETE_STATE { .. } => literal!("[DISS]"),
        Deref @ PREVIOUS { .. } => literal!("[PRE-]"),
        Deref @ CLOCK { .. } => literal!("[CLCK]"),
        Deref @ CLOCKED { .. } => literal!("[CLKD]"),
        Deref @ PARAMETER { .. } => literal!("[PRMT]"),
        Deref @ CONSTANT { .. } => literal!("[CNST]"),
        Deref @ ITERATOR { .. } => literal!("[ITER]"),
        Deref @ RECORD { .. } => literal!("[RECD]"),
        Deref @ START { .. } => literal!("[STRT]"),
        Deref @ EXTOBJ { .. } => literal!("[EXTO]"),
        Deref @ JAC_VAR { .. } => literal!("[JVAR]"),
        Deref @ JAC_TMP_VAR { .. } => literal!("[JTMP]"),
        Deref @ SEED_VAR { .. } => literal!("[SEED]"),
        Deref @ OPT_CONSTR { .. } => literal!("[OPT][CONS]"),
        Deref @ OPT_FCONSTR { .. } => literal!("[OPT][FCON]"),
        Deref @ OPT_INPUT_WITH_DER { .. } => literal!("[OPT][INWD]"),
        Deref @ OPT_INPUT_DER { .. } => literal!("[OPT][INPD]"),
        Deref @ OPT_TGRID { .. } => literal!("[OPT][TGRD]"),
        Deref @ OPT_LOOP_INPUT { .. } => literal!("[OPT][LOOP]"),
        Deref @ ALG_STATE { .. } => literal!("[ASTA]"),
        Deref @ RESIDUAL_VAR { .. } => literal!("[RES-]"),
        Deref @ DAE_AUX_VAR { .. } => literal!("[AUX-]"),
        Deref @ LOOP_ITERATION { .. } => literal!("[LOOP]"),
        Deref @ LOOP_SOLVED { .. } => literal!("[INNR]"),
        Deref @ FRONTEND_DUMMY { .. } => literal!("[DMMY]"),
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[FAIL] ")); __mm_s.push_str(&*literal!("NFBackendExtension.VariableKind.toString")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        r#str
    }

    pub fn isTimeDependent(mut varKind: Arc<VariableKind>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(varKind.clone()) {
        Deref @ PARAMETER { .. } => false,
        Deref @ CONSTANT { .. } => false,
        Deref @ ITERATOR { .. } => false,
        Deref @ START { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn fromType(mut ty: Arc<Type::NFType>, mut makeParam: bool) -> Arc<VariableKind> {
        let mut varKind: Arc<VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
        let mut variability: Variability = Variability::CONSTANT;
        if Type::isRecord(Type::arrayElementType(ty.clone())) {
            variability = if (makeParam.clone()) {Variability::PARAMETER.clone()} else {Variability::CONTINUOUS.clone()};
            varKind = Arc::new(VariableKind::RECORD { children: metamodelica::nil(), min_var: variability.clone(), max_var: variability.clone() });
        } else if makeParam.clone() {
            varKind = Arc::new(VariableKind::PARAMETER { resize_value: None });
        } else if Type::isDiscrete(ty.clone()) {
            varKind = Arc::new(crate::NFBackendExtension::VariableKind::DISCRETE);
        } else {
            varKind = Arc::new(crate::NFBackendExtension::VariableKind::ALGEBRAIC);
        }
        varKind
    }

}

pub mod VariableAttributes {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    pub enum VariableAttributes {
        VAR_ATTR_REAL {
            /// quantity
            quantity: Option<Arc<Expression::NFExpression>>,
            /// SI Unit for actual computation value
            unit: Option<Arc<Expression::NFExpression>>,
            /// SI Unit only for displaying
            displayUnit: Option<Arc<Expression::NFExpression>>,
            /// Lower boundry
            min: Option<Arc<Expression::NFExpression>>,
            /// Upper boundry
            max: Option<Arc<Expression::NFExpression>>,
            /// start value
            start: Option<Arc<Expression::NFExpression>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<Arc<Expression::NFExpression>>,
            /// nominal
            nominal: Option<Arc<Expression::NFExpression>>,
            /// Priority to be selected as a state during index reduction
            stateSelect: Option<StateSelect>,
            /// Priority to be selected as an iteration variable during tearing
            tearingSelect: Option<TearingSelect>,
            /// Attributes from data reconcilliation
            uncertainty: Option<Uncertainty>,
            /// ToDo: ???
            distribution: Option<Arc<Distribution>>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<Arc<Expression::NFExpression>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
            /// where did start=X came from? NONE()|SOME(Expression.STRING binding|type|undefined)
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_INT {
            /// quantity
            quantity: Option<Arc<Expression::NFExpression>>,
            /// Lower boundry
            min: Option<Arc<Expression::NFExpression>>,
            /// Upper boundry
            max: Option<Arc<Expression::NFExpression>>,
            /// start value
            start: Option<Arc<Expression::NFExpression>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<Arc<Expression::NFExpression>>,
            /// Attributes from data reconcilliation
            uncertainty: Option<Uncertainty>,
            /// ToDo: ???
            distribution: Option<Arc<Distribution>>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<Arc<Expression::NFExpression>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
            /// where did start=X came from? NONE()|SOME(Expression.STRING binding|type|undefined)
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_BOOL {
            /// quantity
            quantity: Option<Arc<Expression::NFExpression>>,
            /// start value
            start: Option<Arc<Expression::NFExpression>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<Arc<Expression::NFExpression>>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<Arc<Expression::NFExpression>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
            /// where did start=X came from? NONE()|SOME(Expression.STRING binding|type|undefined)
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_CLOCK {
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
        },
        /// kabdelhak: why does string have quantity/start/fixed?
        VAR_ATTR_STRING {
            /// quantity
            quantity: Option<Arc<Expression::NFExpression>>,
            /// start value
            start: Option<Arc<Expression::NFExpression>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<Arc<Expression::NFExpression>>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<Arc<Expression::NFExpression>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
            /// where did start=X came from? NONE()|SOME(Expression.STRING binding|type|undefined)
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_ENUMERATION {
            /// quantity
            quantity: Option<Arc<Expression::NFExpression>>,
            /// Lower boundry
            min: Option<Arc<Expression::NFExpression>>,
            /// Upper boundry
            max: Option<Arc<Expression::NFExpression>>,
            /// start value
            start: Option<Arc<Expression::NFExpression>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<Arc<Expression::NFExpression>>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<Arc<Expression::NFExpression>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
            /// where did start=X came from? NONE()|SOME(Expression.STRING binding|type|undefined)
            startOrigin: Option<Arc<Expression::NFExpression>>,
        },
        VAR_ATTR_RECORD {
            indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>,
            childrenAttr: metamodelica::Array<Arc<VariableAttributes>>,
        },
    }
    impl Default for VariableAttributes {
        fn default() -> Self {
            Self::VAR_ATTR_CLOCK {
                isProtected: Default::default(),
                finalPrefix: Default::default(),
            }
        }
    }
    pub use self::VariableAttributes::{VAR_ATTR_REAL,VAR_ATTR_INT,VAR_ATTR_BOOL,VAR_ATTR_CLOCK,VAR_ATTR_STRING,VAR_ATTR_ENUMERATION,VAR_ATTR_RECORD};
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    #[repr(i32)]
    pub enum VarType {
        ENUMERATION = 1,
        CLOCK = 2,
        STRING = 3,
    }
    impl PartialOrd for VarType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl Ord for VarType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
    }

    pub fn toString(mut attr: Arc<VariableAttributes>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = ((::match_deref::match_deref! { match &(attr.clone()) {
        Deref @ VAR_ATTR_REAL { .. } => attributesToString(list![(literal!("fixed"), var_field!((*attr).fixed, VariableAttributes::VAR_ATTR_REAL).clone()), (literal!("start"), var_field!((*attr).start, VariableAttributes::VAR_ATTR_REAL).clone()), (literal!("min"), var_field!((*attr).min, VariableAttributes::VAR_ATTR_REAL).clone()), (literal!("max"), var_field!((*attr).max, VariableAttributes::VAR_ATTR_REAL).clone()), (literal!("nominal"), var_field!((*attr).nominal, VariableAttributes::VAR_ATTR_REAL).clone())], var_field!((*attr).stateSelect, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((*attr).tearingSelect, VariableAttributes::VAR_ATTR_REAL).clone())?,
        Deref @ VAR_ATTR_INT { .. } => attributesToString(list![(literal!("fixed"), var_field!((*attr).fixed, VariableAttributes::VAR_ATTR_INT).clone()), (literal!("start"), var_field!((*attr).start, VariableAttributes::VAR_ATTR_INT).clone()), (literal!("min"), var_field!((*attr).min, VariableAttributes::VAR_ATTR_INT).clone()), (literal!("max"), var_field!((*attr).max, VariableAttributes::VAR_ATTR_INT).clone())], None, None)?,
        Deref @ VAR_ATTR_BOOL { .. } => attributesToString(list![(literal!("fixed"), var_field!((*attr).fixed, VariableAttributes::VAR_ATTR_BOOL).clone()), (literal!("start"), var_field!((*attr).start, VariableAttributes::VAR_ATTR_BOOL).clone())], None, None)?,
        Deref @ VAR_ATTR_CLOCK { .. } => literal!(""),
        Deref @ VAR_ATTR_STRING { .. } => attributesToString(list![(literal!("fixed"), var_field!((*attr).fixed, VariableAttributes::VAR_ATTR_STRING).clone()), (literal!("start"), var_field!((*attr).start, VariableAttributes::VAR_ATTR_STRING).clone())], None, None)?,
        Deref @ VAR_ATTR_ENUMERATION { .. } => attributesToString(list![(literal!("fixed"), var_field!((*attr).fixed, VariableAttributes::VAR_ATTR_ENUMERATION).clone()), (literal!("start"), var_field!((*attr).start, VariableAttributes::VAR_ATTR_ENUMERATION).clone()), (literal!("min"), var_field!((*attr).min, VariableAttributes::VAR_ATTR_ENUMERATION).clone()), (literal!("max"), var_field!((*attr).max, VariableAttributes::VAR_ATTR_ENUMERATION).clone())], None, None)?,
        Deref @ VAR_ATTR_RECORD { .. } => List::toString(UnorderedMap::toList(var_field!((*attr).indexMap, VariableAttributes::VAR_ATTR_RECORD).clone()), Arc::new({ let __pe_b1 = var_field!((*attr).childrenAttr, VariableAttributes::VAR_ATTR_RECORD).clone(); move |__pe_a0| recordString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), true, 0)?,
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.toString")); __mm_s.push_str(&*literal!(" failed. Attribute string could not be created.")); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        r#str = (if (literal!("") == r#str.clone()) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }}).clone();
        Ok(r#str)
    }

    pub fn recordString(mut attr_tpl: (ArcStr, i32), mut childrenAttr: metamodelica::Array<Arc<VariableAttributes>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut name: ArcStr = arcstr::literal!("");
        let mut index: i32 = 0;
        (name, index) = attr_tpl.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*toString(childrenAttr.borrow()[(index.clone()-1) as usize].clone())?); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn create(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut ty: Arc<Type::NFType>, mut compAttrs: Arc<Attributes::NFAttributes>, mut children: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut comment: Arc<SCode::Comment>) -> Result<Arc<VariableAttributes>> {
        let mut attributes: Arc<VariableAttributes>;
        let mut is_final: bool = false;
        let mut elTy: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut is_array: bool = false;
        let mut complexTy: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
        is_final = compAttrs.isFinal.clone() || compAttrs.variability.clone() == Variability::STRUCTURAL_PARAMETER.clone();
        attributes = (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::REAL => createReal(attrs.clone(), is_final.clone(), comment.clone())?,
        Deref @ Type::INTEGER => createInt(attrs.clone(), is_final.clone())?,
        Deref @ Type::BOOLEAN => createBool(attrs.clone(), is_final.clone())?,
        Deref @ Type::STRING => createString(attrs.clone(), is_final.clone())?,
        Deref @ Type::ENUMERATION { .. } => createEnum(attrs.clone(), is_final.clone())?,
        Deref @ Type::CLOCK => createClock(is_final.clone()),
        Deref @ Type::COMPLEX { complexTy: complexTy @ Deref @ ComplexType::RECORD { .. }, .. } => createRecord(attrs.clone(), var_field!((**complexTy).indexMap, ComplexType::NFComplexType::RECORD).clone(), children.clone(), is_final.clone())?,
        _ => createReal(attrs.clone(), is_final.clone(), comment.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(attributes)
    }

    pub fn map(mut attributes: Arc<VariableAttributes>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<VariableAttributes> {
        pub type expFunc = fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>>;

        let mut attributes: Arc<VariableAttributes> = attributes;
        attributes = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL;
                quantity = Util::applyOption(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                unit = Util::applyOption(var_field!((*attributes).unit, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                displayUnit = Util::applyOption(var_field!((*attributes).displayUnit, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                min = Util::applyOption(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                max = Util::applyOption(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                nominal = Util::applyOption(var_field!((*attributes).nominal, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                startOrigin = Util::applyOption(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }))
            );
            attributes.clone()
        },
        Deref @ VAR_ATTR_INT { .. } => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT;
                quantity = Util::applyOption(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_INT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                min = Util::applyOption(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_INT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                max = Util::applyOption(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_INT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_INT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_INT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_INT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                startOrigin = Util::applyOption(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_INT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }))
            );
            attributes.clone()
        },
        Deref @ VAR_ATTR_BOOL { .. } => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_BOOL;
                quantity = Util::applyOption(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_BOOL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_BOOL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_BOOL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_BOOL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                startOrigin = Util::applyOption(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_BOOL).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }))
            );
            attributes.clone()
        },
        Deref @ VAR_ATTR_STRING { .. } => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_STRING;
                quantity = Util::applyOption(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_STRING).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_STRING).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_STRING).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_STRING).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                startOrigin = Util::applyOption(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_STRING).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }))
            );
            attributes.clone()
        },
        Deref @ VAR_ATTR_ENUMERATION { .. } => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION;
                quantity = Util::applyOption(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                min = Util::applyOption(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                max = Util::applyOption(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })),
                startOrigin = Util::applyOption(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }))
            );
            attributes.clone()
        },
        Deref @ VAR_ATTR_RECORD { .. } => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_RECORD; childrenAttr = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<VariableAttributes>>> = metamodelica::nil();
        for mut attr in (var_field!((*attributes).childrenAttr, VariableAttributes::VAR_ATTR_RECORD).clone()).borrow().iter() {
            let __x = map(attr.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect()));
            attributes.clone()
        },
        _ => attributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        attributes
    }

    pub fn setFixed(mut attributes: Arc<VariableAttributes>, mut ty: Arc<Type::NFType>, mut b: bool, mut overwrite: bool) -> Result<Arc<VariableAttributes>> {
        let mut attributes: Arc<VariableAttributes> = attributes;
        let mut sizes: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut start: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut iter_range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut binding: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::BOOLEAN { value: b.clone() });
        let mut step: Option<Arc<Expression::NFExpression>> = None;
        let mut iter_name: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
        if Type::isArray(ty.clone()) {
            sizes = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dim in (Type::arrayDims(ty.clone())).into_iter().cloned() {
            let __x = Dimension::size(dim.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            start = Arc::new(Expression::NFExpression::INTEGER { value: 1 });
            step = None;
            for mut stop in &*sizes.clone() {
                let mut stop = stop.clone();
                iter_name = InstNode::newUniqueIterator(Absyn::dummyInfo.clone(), Arc::new(crate::NFType::INTEGER));
                iter_range = Arc::new(Expression::NFExpression::RANGE { ty: Arc::new(crate::NFType::INTEGER), start: start.clone(), step: step.clone(), stop: Arc::new(Expression::NFExpression::INTEGER { value: stop.clone() }) });
                iterators = cons((iter_name.clone(), iter_range.clone()), iterators.clone());
            }
            binding = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty.clone(), var: Expression::variability(binding.clone())?, purity: NFPrefixes::Purity::PURE.clone(), exp: binding.clone(), iters: iterators.clone().reverse() }) });
        }
        attributes = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } if (overwrite.clone() || isNone(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_REAL).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; fixed = Some(binding.clone()));
            attributes.clone()
        },
        Deref @ VAR_ATTR_INT { .. } if (overwrite.clone() || isNone(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_INT).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT; fixed = Some(binding.clone()));
            attributes.clone()
        },
        Deref @ VAR_ATTR_BOOL { .. } if (overwrite.clone() || isNone(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_BOOL).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_BOOL; fixed = Some(binding.clone()));
            attributes.clone()
        },
        Deref @ VAR_ATTR_STRING { .. } if (overwrite.clone() || isNone(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_STRING).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_STRING; fixed = Some(binding.clone()));
            attributes.clone()
        },
        Deref @ VAR_ATTR_ENUMERATION { .. } if (overwrite.clone() || isNone(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_ENUMERATION).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION; fixed = Some(binding.clone()));
            attributes.clone()
        },
        _ => attributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(attributes)
    }

    pub fn isFixed(mut attributes: Arc<VariableAttributes>) -> bool {
        let mut fixed: bool = false;
        fixed = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { fixed: Some(Deref @ Expression::BOOLEAN { value: true }), .. } => true,
        Deref @ VAR_ATTR_INT { fixed: Some(Deref @ Expression::BOOLEAN { value: true }), .. } => true,
        Deref @ VAR_ATTR_BOOL { fixed: Some(Deref @ Expression::BOOLEAN { value: true }), .. } => true,
        Deref @ VAR_ATTR_STRING { fixed: Some(Deref @ Expression::BOOLEAN { value: true }), .. } => true,
        Deref @ VAR_ATTR_ENUMERATION { fixed: Some(Deref @ Expression::BOOLEAN { value: true }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        fixed
    }

    pub fn setStartAttribute(mut attributes: Arc<VariableAttributes>, mut start: Arc<Expression::NFExpression>, mut overwrite: bool) -> Arc<VariableAttributes> {
        let mut attributes: Arc<VariableAttributes> = attributes;
        attributes = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } if (overwrite.clone() || isNone(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_REAL).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; start = Some(start.clone()));
            attributes.clone()
        },
        Deref @ VAR_ATTR_INT { .. } if (overwrite.clone() || isNone(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_INT).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT; start = Some(start.clone()));
            attributes.clone()
        },
        Deref @ VAR_ATTR_BOOL { .. } if (overwrite.clone() || isNone(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_BOOL).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_BOOL; start = Some(start.clone()));
            attributes.clone()
        },
        Deref @ VAR_ATTR_STRING { .. } if (overwrite.clone() || isNone(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_STRING).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_STRING; start = Some(start.clone()));
            attributes.clone()
        },
        Deref @ VAR_ATTR_ENUMERATION { .. } if (overwrite.clone() || isNone(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_ENUMERATION).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION; start = Some(start.clone()));
            attributes.clone()
        },
        _ => attributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        attributes
    }

    pub fn getStartAttribute(mut attributes: Arc<VariableAttributes>) -> Option<Arc<Expression::NFExpression>> {
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        start = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } => var_field!((*attributes).start, VariableAttributes::VAR_ATTR_REAL).clone(),
        Deref @ VAR_ATTR_INT { .. } => var_field!((*attributes).start, VariableAttributes::VAR_ATTR_INT).clone(),
        Deref @ VAR_ATTR_BOOL { .. } => var_field!((*attributes).start, VariableAttributes::VAR_ATTR_BOOL).clone(),
        Deref @ VAR_ATTR_STRING { .. } => var_field!((*attributes).start, VariableAttributes::VAR_ATTR_STRING).clone(),
        Deref @ VAR_ATTR_ENUMERATION { .. } => var_field!((*attributes).start, VariableAttributes::VAR_ATTR_ENUMERATION).clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        start
    }

    pub fn setMin(mut attributes: Arc<VariableAttributes>, mut min_val: Option<Arc<Expression::NFExpression>>, mut overwrite: bool) -> Arc<VariableAttributes> {
        let mut attributes: Arc<VariableAttributes> = attributes;
        attributes = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } if (overwrite.clone() || isNone(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_REAL).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; min = min_val.clone());
            attributes.clone()
        },
        Deref @ VAR_ATTR_INT { .. } if (overwrite.clone() || isNone(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_INT).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT; min = min_val.clone());
            attributes.clone()
        },
        Deref @ VAR_ATTR_ENUMERATION { .. } if (overwrite.clone() || isNone(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_ENUMERATION).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION; min = min_val.clone());
            attributes.clone()
        },
        _ => attributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        attributes
    }

    pub fn setMax(mut attributes: Arc<VariableAttributes>, mut max_val: Option<Arc<Expression::NFExpression>>, mut overwrite: bool) -> Arc<VariableAttributes> {
        let mut attributes: Arc<VariableAttributes> = attributes;
        attributes = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } if (overwrite.clone() || isNone(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_REAL).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; max = max_val.clone());
            attributes.clone()
        },
        Deref @ VAR_ATTR_INT { .. } if (overwrite.clone() || isNone(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_INT).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT; max = max_val.clone());
            attributes.clone()
        },
        Deref @ VAR_ATTR_ENUMERATION { .. } if (overwrite.clone() || isNone(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_ENUMERATION).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION; max = max_val.clone());
            attributes.clone()
        },
        _ => attributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        attributes
    }

    pub fn setStateSelect(mut attributes: Arc<VariableAttributes>, mut stateSelect_val: StateSelect, mut overwrite: bool) -> Arc<VariableAttributes> {
        let mut attributes: Arc<VariableAttributes> = attributes;
        attributes = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } if (overwrite.clone() || isNone(var_field!((*attributes).stateSelect, VariableAttributes::VAR_ATTR_REAL).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; stateSelect = Some(stateSelect_val.clone()));
            attributes.clone()
        },
        _ => attributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        attributes
    }

    pub fn getStateSelect(mut attributes: Arc<VariableAttributes>) -> StateSelect {
        let mut stateSelect: StateSelect = StateSelect::NEVER;
        stateSelect = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { stateSelect: Some(stateSelect), .. } => stateSelect.clone(),
        _ => StateSelect::DEFAULT.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        stateSelect
    }

    pub fn setTearingSelect(mut attributes: Arc<VariableAttributes>, mut tearingSelect_val: TearingSelect, mut overwrite: bool) -> Arc<VariableAttributes> {
        let mut attributes: Arc<VariableAttributes> = attributes;
        attributes = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } if (overwrite.clone() || isNone(var_field!((*attributes).tearingSelect, VariableAttributes::VAR_ATTR_REAL).clone())) => {
            assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; tearingSelect = Some(tearingSelect_val.clone()));
            attributes.clone()
        },
        _ => attributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        attributes
    }

    pub fn getTearingSelect(mut attributes: Arc<VariableAttributes>) -> TearingSelect {
        let mut tearingSelect: TearingSelect = TearingSelect::NEVER;
        tearingSelect = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { tearingSelect: Some(tearingSelect), .. } => tearingSelect.clone(),
        _ => TearingSelect::DEFAULT.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        tearingSelect
    }

    pub fn getNominal(mut attr: Arc<VariableAttributes>) -> Option<Arc<Expression::NFExpression>> {
        let mut nominal: Option<Arc<Expression::NFExpression>> = None;
        nominal = (::match_deref::match_deref! { match &(attr.clone()) {
        Deref @ VAR_ATTR_REAL { .. } => var_field!((*attr).nominal, VariableAttributes::VAR_ATTR_REAL).clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        nominal
    }

    pub fn scalarizeReal(mut quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut unit_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut displayUnit_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut min_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut max_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut start_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut nominal_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut stateSelect: Option<StateSelect>, mut tearingSelect: Option<TearingSelect>, mut uncertainty: Option<Uncertainty>, mut distribution: Option<Arc<Distribution>>, mut binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut isProtected: Option<bool>, mut finalPrefix: Option<bool>, mut startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut length: i32) -> Result<Arc<metamodelica::List<Arc<VariableAttributes>>>> {
        let mut scalar_attributes: Arc<metamodelica::List<Arc<VariableAttributes>>> = metamodelica::nil();
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut unit: Option<Arc<Expression::NFExpression>> = None;
        let mut displayUnit: Option<Arc<Expression::NFExpression>> = None;
        let mut min: Option<Arc<Expression::NFExpression>> = None;
        let mut max: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        let mut nominal: Option<Arc<Expression::NFExpression>> = None;
        let mut binding: Option<Arc<Expression::NFExpression>> = None;
        let mut startOrigin: Option<Arc<Expression::NFExpression>> = None;
        let mut quantity_loc: Arc<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut unit_loc: Arc<ExpressionIterator::NFExpressionIterator> = unit_iter.clone();
        let mut displayUnit_loc: Arc<ExpressionIterator::NFExpressionIterator> = displayUnit_iter.clone();
        let mut min_loc: Arc<ExpressionIterator::NFExpressionIterator> = min_iter.clone();
        let mut max_loc: Arc<ExpressionIterator::NFExpressionIterator> = max_iter.clone();
        let mut start_loc: Arc<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: Arc<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut nominal_loc: Arc<ExpressionIterator::NFExpressionIterator> = nominal_iter.clone();
        let mut binding_loc: Arc<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        let mut startOrigin_loc: Arc<ExpressionIterator::NFExpressionIterator> = startOrigin_iter.clone();
        for mut i in 1..=length.clone() {
            (quantity_loc, quantity) = ExpressionIterator::nextOpt(quantity_loc.clone())?;
            (unit_loc, unit) = ExpressionIterator::nextOpt(unit_loc.clone())?;
            (displayUnit_loc, displayUnit) = ExpressionIterator::nextOpt(displayUnit_loc.clone())?;
            (min_loc, min) = ExpressionIterator::nextOpt(min_loc.clone())?;
            (max_loc, max) = ExpressionIterator::nextOpt(max_loc.clone())?;
            (start_loc, start) = ExpressionIterator::nextOpt(start_loc.clone())?;
            (fixed_loc, fixed) = ExpressionIterator::nextOpt(fixed_loc.clone())?;
            (nominal_loc, nominal) = ExpressionIterator::nextOpt(nominal_loc.clone())?;
            (binding_loc, binding) = ExpressionIterator::nextOpt(binding_loc.clone())?;
            (startOrigin_loc, startOrigin) = ExpressionIterator::nextOpt(startOrigin_loc.clone())?;
            scalar_attributes = cons(Arc::new(VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), nominal: nominal.clone(), stateSelect: stateSelect.clone(), tearingSelect: tearingSelect.clone(), uncertainty: uncertainty.clone(), distribution: distribution.clone(), binding: binding.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() }), scalar_attributes.clone());
        }
        scalar_attributes = scalar_attributes.clone().reverse();
        Ok(scalar_attributes)
    }

    pub fn scalarizeInt(mut quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut min_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut max_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut start_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut uncertainty: Option<Uncertainty>, mut distribution: Option<Arc<Distribution>>, mut binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut isProtected: Option<bool>, mut finalPrefix: Option<bool>, mut startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut length: i32) -> Result<Arc<metamodelica::List<Arc<VariableAttributes>>>> {
        let mut scalar_attributes: Arc<metamodelica::List<Arc<VariableAttributes>>> = metamodelica::nil();
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut min: Option<Arc<Expression::NFExpression>> = None;
        let mut max: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        let mut binding: Option<Arc<Expression::NFExpression>> = None;
        let mut startOrigin: Option<Arc<Expression::NFExpression>> = None;
        let mut quantity_loc: Arc<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut min_loc: Arc<ExpressionIterator::NFExpressionIterator> = min_iter.clone();
        let mut max_loc: Arc<ExpressionIterator::NFExpressionIterator> = max_iter.clone();
        let mut start_loc: Arc<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: Arc<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut binding_loc: Arc<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        let mut startOrigin_loc: Arc<ExpressionIterator::NFExpressionIterator> = startOrigin_iter.clone();
        for mut i in 1..=length.clone() {
            (quantity_loc, quantity) = ExpressionIterator::nextOpt(quantity_loc.clone())?;
            (min_loc, min) = ExpressionIterator::nextOpt(min_loc.clone())?;
            (max_loc, max) = ExpressionIterator::nextOpt(max_loc.clone())?;
            (start_loc, start) = ExpressionIterator::nextOpt(start_loc.clone())?;
            (fixed_loc, fixed) = ExpressionIterator::nextOpt(fixed_loc.clone())?;
            (binding_loc, binding) = ExpressionIterator::nextOpt(binding_loc.clone())?;
            (startOrigin_loc, startOrigin) = ExpressionIterator::nextOpt(startOrigin_loc.clone())?;
            scalar_attributes = cons(Arc::new(VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), uncertainty: uncertainty.clone(), distribution: distribution.clone(), binding: binding.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() }), scalar_attributes.clone());
        }
        scalar_attributes = scalar_attributes.clone().reverse();
        Ok(scalar_attributes)
    }

    pub fn scalarizeBool(mut quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut start_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut isProtected: Option<bool>, mut finalPrefix: Option<bool>, mut startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut length: i32) -> Result<Arc<metamodelica::List<Arc<VariableAttributes>>>> {
        let mut scalar_attributes: Arc<metamodelica::List<Arc<VariableAttributes>>> = metamodelica::nil();
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        let mut binding: Option<Arc<Expression::NFExpression>> = None;
        let mut startOrigin: Option<Arc<Expression::NFExpression>> = None;
        let mut quantity_loc: Arc<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut start_loc: Arc<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: Arc<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut binding_loc: Arc<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        let mut startOrigin_loc: Arc<ExpressionIterator::NFExpressionIterator> = startOrigin_iter.clone();
        for mut i in 1..=length.clone() {
            (quantity_loc, quantity) = ExpressionIterator::nextOpt(quantity_loc.clone())?;
            (start_loc, start) = ExpressionIterator::nextOpt(start_loc.clone())?;
            (fixed_loc, fixed) = ExpressionIterator::nextOpt(fixed_loc.clone())?;
            (binding_loc, binding) = ExpressionIterator::nextOpt(binding_loc.clone())?;
            (startOrigin_loc, startOrigin) = ExpressionIterator::nextOpt(startOrigin_loc.clone())?;
            scalar_attributes = cons(Arc::new(VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), binding: binding.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() }), scalar_attributes.clone());
        }
        scalar_attributes = scalar_attributes.clone().reverse();
        Ok(scalar_attributes)
    }

    pub fn scalarizeClock(mut isProtected: Option<bool>, mut finalPrefix: Option<bool>, mut length: i32) -> Arc<metamodelica::List<Arc<VariableAttributes>>> {
        let mut scalar_attributes: Arc<metamodelica::List<Arc<VariableAttributes>>> = List::fill(Arc::new(VariableAttributes::VAR_ATTR_CLOCK { isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone() }), length.clone());
        scalar_attributes
    }

    pub fn scalarizeString(mut quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut start_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut isProtected: Option<bool>, mut finalPrefix: Option<bool>, mut startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut length: i32) -> Result<Arc<metamodelica::List<Arc<VariableAttributes>>>> {
        let mut scalar_attributes: Arc<metamodelica::List<Arc<VariableAttributes>>> = metamodelica::nil();
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        let mut binding: Option<Arc<Expression::NFExpression>> = None;
        let mut startOrigin: Option<Arc<Expression::NFExpression>> = None;
        let mut quantity_loc: Arc<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut start_loc: Arc<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: Arc<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut binding_loc: Arc<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        let mut startOrigin_loc: Arc<ExpressionIterator::NFExpressionIterator> = startOrigin_iter.clone();
        for mut i in 1..=length.clone() {
            (quantity_loc, quantity) = ExpressionIterator::nextOpt(quantity_loc.clone())?;
            (start_loc, start) = ExpressionIterator::nextOpt(start_loc.clone())?;
            (fixed_loc, fixed) = ExpressionIterator::nextOpt(fixed_loc.clone())?;
            (binding_loc, binding) = ExpressionIterator::nextOpt(binding_loc.clone())?;
            (startOrigin_loc, startOrigin) = ExpressionIterator::nextOpt(startOrigin_loc.clone())?;
            scalar_attributes = cons(Arc::new(VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), binding: binding.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() }), scalar_attributes.clone());
        }
        scalar_attributes = scalar_attributes.clone().reverse();
        Ok(scalar_attributes)
    }

    pub fn scalarizeEnumeration(mut quantity_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut min_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut max_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut start_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut fixed_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut binding_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut isProtected: Option<bool>, mut finalPrefix: Option<bool>, mut startOrigin_iter: Arc<ExpressionIterator::NFExpressionIterator>, mut length: i32) -> Result<Arc<metamodelica::List<Arc<VariableAttributes>>>> {
        let mut scalar_attributes: Arc<metamodelica::List<Arc<VariableAttributes>>> = metamodelica::nil();
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut min: Option<Arc<Expression::NFExpression>> = None;
        let mut max: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        let mut binding: Option<Arc<Expression::NFExpression>> = None;
        let mut startOrigin: Option<Arc<Expression::NFExpression>> = None;
        let mut quantity_loc: Arc<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut min_loc: Arc<ExpressionIterator::NFExpressionIterator> = min_iter.clone();
        let mut max_loc: Arc<ExpressionIterator::NFExpressionIterator> = max_iter.clone();
        let mut start_loc: Arc<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: Arc<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut binding_loc: Arc<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        let mut startOrigin_loc: Arc<ExpressionIterator::NFExpressionIterator> = startOrigin_iter.clone();
        for mut i in 1..=length.clone() {
            (quantity_loc, quantity) = ExpressionIterator::nextOpt(quantity_loc.clone())?;
            (min_loc, min) = ExpressionIterator::nextOpt(min_loc.clone())?;
            (max_loc, max) = ExpressionIterator::nextOpt(max_loc.clone())?;
            (start_loc, start) = ExpressionIterator::nextOpt(start_loc.clone())?;
            (fixed_loc, fixed) = ExpressionIterator::nextOpt(fixed_loc.clone())?;
            (binding_loc, binding) = ExpressionIterator::nextOpt(binding_loc.clone())?;
            (startOrigin_loc, startOrigin) = ExpressionIterator::nextOpt(startOrigin_loc.clone())?;
            scalar_attributes = cons(Arc::new(VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), binding: binding.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: startOrigin.clone() }), scalar_attributes.clone());
        }
        scalar_attributes = scalar_attributes.clone().reverse();
        Ok(scalar_attributes)
    }

    pub fn scalarize(mut attributes: Arc<VariableAttributes>, mut length: i32) -> Result<Arc<metamodelica::List<Arc<VariableAttributes>>>> {
        let mut scalar_attributes: Arc<metamodelica::List<Arc<VariableAttributes>>> = metamodelica::nil();
        scalar_attributes = (::match_deref::match_deref! { match &(attributes.clone()) {
        Deref @ VAR_ATTR_REAL { .. } => scalarizeReal(ExpressionIterator::fromExpOpt(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_REAL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).unit, VariableAttributes::VAR_ATTR_REAL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).displayUnit, VariableAttributes::VAR_ATTR_REAL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_REAL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_REAL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_REAL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_REAL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).nominal, VariableAttributes::VAR_ATTR_REAL).clone())?, var_field!((*attributes).stateSelect, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((*attributes).tearingSelect, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((*attributes).uncertainty, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((*attributes).distribution, VariableAttributes::VAR_ATTR_REAL).clone(), ExpressionIterator::fromExpOpt(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_REAL).clone())?, var_field!((*attributes).isProtected, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((*attributes).finalPrefix, VariableAttributes::VAR_ATTR_REAL).clone(), ExpressionIterator::fromExpOpt(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_REAL).clone())?, length.clone())?,
        Deref @ VAR_ATTR_INT { .. } => scalarizeInt(ExpressionIterator::fromExpOpt(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_INT).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_INT).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_INT).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_INT).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_INT).clone())?, var_field!((*attributes).uncertainty, VariableAttributes::VAR_ATTR_INT).clone(), var_field!((*attributes).distribution, VariableAttributes::VAR_ATTR_INT).clone(), ExpressionIterator::fromExpOpt(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_INT).clone())?, var_field!((*attributes).isProtected, VariableAttributes::VAR_ATTR_INT).clone(), var_field!((*attributes).finalPrefix, VariableAttributes::VAR_ATTR_INT).clone(), ExpressionIterator::fromExpOpt(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_INT).clone())?, length.clone())?,
        Deref @ VAR_ATTR_BOOL { .. } => scalarizeBool(ExpressionIterator::fromExpOpt(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_BOOL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_BOOL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_BOOL).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_BOOL).clone())?, var_field!((*attributes).isProtected, VariableAttributes::VAR_ATTR_BOOL).clone(), var_field!((*attributes).finalPrefix, VariableAttributes::VAR_ATTR_BOOL).clone(), ExpressionIterator::fromExpOpt(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_BOOL).clone())?, length.clone())?,
        Deref @ VAR_ATTR_CLOCK { .. } => scalarizeClock(var_field!((*attributes).isProtected, VariableAttributes::VAR_ATTR_CLOCK).clone(), var_field!((*attributes).finalPrefix, VariableAttributes::VAR_ATTR_CLOCK).clone(), length.clone()),
        Deref @ VAR_ATTR_STRING { .. } => scalarizeString(ExpressionIterator::fromExpOpt(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_STRING).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_STRING).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_STRING).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_STRING).clone())?, var_field!((*attributes).isProtected, VariableAttributes::VAR_ATTR_STRING).clone(), var_field!((*attributes).finalPrefix, VariableAttributes::VAR_ATTR_STRING).clone(), ExpressionIterator::fromExpOpt(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_STRING).clone())?, length.clone())?,
        Deref @ VAR_ATTR_ENUMERATION { .. } => scalarizeEnumeration(ExpressionIterator::fromExpOpt(var_field!((*attributes).quantity, VariableAttributes::VAR_ATTR_ENUMERATION).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_ENUMERATION).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_ENUMERATION).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_ENUMERATION).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_ENUMERATION).clone())?, ExpressionIterator::fromExpOpt(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_ENUMERATION).clone())?, var_field!((*attributes).isProtected, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), var_field!((*attributes).finalPrefix, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), ExpressionIterator::fromExpOpt(var_field!((*attributes).startOrigin, VariableAttributes::VAR_ATTR_ENUMERATION).clone())?, length.clone())?,
        Deref @ VAR_ATTR_RECORD { .. } => list![attributes.clone()],
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.scalarize")); __mm_s.push_str(&*literal!("failed. Not yet handled: ")); __mm_s.push_str(&*toString(attributes.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(scalar_attributes)
    }

    pub fn elemType(mut attr: Arc<VariableAttributes>) -> Result<Arc<Type::NFType>> {
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        ty = (::match_deref::match_deref! { match &(attr.clone()) {
        Deref @ VAR_ATTR_REAL { .. } => Arc::new(crate::NFType::REAL),
        Deref @ VAR_ATTR_INT { .. } => Arc::new(crate::NFType::INTEGER),
        Deref @ VAR_ATTR_BOOL { .. } => Arc::new(crate::NFType::BOOLEAN),
        Deref @ VAR_ATTR_CLOCK { .. } => Arc::new(crate::NFType::CLOCK),
        Deref @ VAR_ATTR_STRING { .. } => Arc::new(crate::NFType::STRING),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.elemType")); __mm_s.push_str(&*literal!(" cannot create type from attributes: ")); __mm_s.push_str(&*toString(attr.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(ty)
    }

    pub fn attributesToString(mut tpl_list: Arc<metamodelica::List<(ArcStr, Option<Arc<Expression::NFExpression>>)>>, mut stateSelect: Option<StateSelect>, mut tearingSelect: Option<TearingSelect>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut buffer: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut name: ArcStr = arcstr::literal!("");
        for mut tpl in &*tpl_list.clone() {
            let mut tpl = tpl.clone();
            buffer = attributeToString(tpl.clone(), buffer.clone())?;
        }
        buffer = stateSelectStringBuffer(stateSelect.clone(), buffer.clone())?;
        buffer = tearingSelectStringBuffer(tearingSelect.clone(), buffer.clone())?;
        buffer = buffer.clone().reverse();
        if !(buffer.clone().is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(buffer.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            buffer = __pa1.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            for mut name in &*buffer.clone() {
                let mut name = name.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            }
        }
        Ok(r#str)
    }

    pub fn attributeToString(mut tpl: (ArcStr, Option<Arc<Expression::NFExpression>>), mut buffer: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
        let mut buffer: Arc<metamodelica::List<ArcStr>> = buffer;
        let mut name: ArcStr = arcstr::literal!("");
        let mut optAttr: Option<Arc<Expression::NFExpression>> = None;
        let mut attr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        (name, optAttr) = tpl.clone();
        if isSome(optAttr.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(optAttr.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            attr = __pa0.clone();
            buffer = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(attr.clone())?); ArcStr::from(__mm_s) }, buffer.clone());
        }
        Ok(buffer)
    }

    pub fn stateSelectString(mut stateSelect: StateSelect) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((match stateSelect.clone() {
        StateSelect::NEVER => literal!("StateSelect = never"),
        StateSelect::AVOID => literal!("StateSelect = avoid"),
        StateSelect::DEFAULT => literal!("StateSelect = default"),
        StateSelect::PREFER => literal!("StateSelect = prefer"),
        StateSelect::ALWAYS => literal!("StateSelect = always"),
        _ => bail!("match: no arm matched"),
    })).clone();
        Ok(r#str)
    }

    pub fn tearingSelectString(mut tearingSelect: TearingSelect) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((match tearingSelect.clone() {
        TearingSelect::NEVER => literal!("TearingSelect = never"),
        TearingSelect::AVOID => literal!("TearingSelect = avoid"),
        TearingSelect::DEFAULT => literal!("TearingSelect = default"),
        TearingSelect::PREFER => literal!("TearingSelect = prefer"),
        TearingSelect::ALWAYS => literal!("TearingSelect = always"),
        _ => bail!("match: no arm matched"),
    })).clone();
        Ok(r#str)
    }

    pub fn stateSelectStringBuffer(mut optStateSelect: Option<StateSelect>, mut buffer: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
        let mut buffer: Arc<metamodelica::List<ArcStr>> = buffer;
        let mut stateSelect: StateSelect = StateSelect::NEVER;
        if isSome(optStateSelect.clone()) {
            let Some(__pa0) = (optStateSelect.clone()) else { bail!("pattern mismatch") };
            stateSelect = __pa0.clone();
            buffer = cons(stateSelectString(stateSelect.clone())?, buffer.clone());
        }
        Ok(buffer)
    }

    pub fn tearingSelectStringBuffer(mut optTearingSelect: Option<TearingSelect>, mut buffer: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
        let mut buffer: Arc<metamodelica::List<ArcStr>> = buffer;
        let mut tearingSelect: TearingSelect = TearingSelect::NEVER;
        if isSome(optTearingSelect.clone()) {
            let Some(__pa0) = (optTearingSelect.clone()) else { bail!("pattern mismatch") };
            tearingSelect = __pa0.clone();
            buffer = cons(tearingSelectString(tearingSelect.clone())?, buffer.clone());
        }
        Ok(buffer)
    }

    fn createReal(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: bool, mut comment: Arc<SCode::Comment>) -> Result<Arc<VariableAttributes>> {
        let mut attributes: Arc<VariableAttributes>;
        let mut name: ArcStr = arcstr::literal!("");
        let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut unit: Option<Arc<Expression::NFExpression>> = None;
        let mut displayUnit: Option<Arc<Expression::NFExpression>> = None;
        let mut min: Option<Arc<Expression::NFExpression>> = None;
        let mut max: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        let mut nominal: Option<Arc<Expression::NFExpression>> = None;
        let mut state_select: Option<StateSelect> = None;
        let mut tearing_select: Option<TearingSelect> = None;
        for mut attr in &*attrs.clone() {
            let mut attr = attr.clone();
            (name, b) = attr.clone();
            let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "displayUnit" => {
            displayUnit = createAttribute(b.clone());
            ()
        },
        Deref @ "fixed" => {
            fixed = createAttribute(b.clone());
            ()
        },
        Deref @ "max" => {
            max = createAttribute(b.clone());
            ()
        },
        Deref @ "min" => {
            min = createAttribute(b.clone());
            ()
        },
        Deref @ "nominal" => {
            nominal = createAttribute(b.clone());
            ()
        },
        Deref @ "quantity" => {
            quantity = createAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = createAttribute(b.clone());
            ()
        },
        Deref @ "stateSelect" => {
            state_select = createStateSelect(b.clone())?;
            ()
        },
        Deref @ "unbounded" => (),
        Deref @ "unit" => {
            unit = createAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createReal")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        tearing_select = createTearingSelect(comment.clone())?;
        attributes = Arc::new(VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), nominal: nominal.clone(), stateSelect: state_select.clone(), tearingSelect: tearing_select.clone(), uncertainty: None, distribution: None, binding: None, isProtected: None, finalPrefix: Some(isFinal.clone()), startOrigin: None });
        Ok(attributes)
    }

    fn createInt(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: bool) -> Result<Arc<VariableAttributes>> {
        let mut attributes: Arc<VariableAttributes>;
        let mut name: ArcStr = arcstr::literal!("");
        let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut min: Option<Arc<Expression::NFExpression>> = None;
        let mut max: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        if attrs.clone().is_empty() && !(isFinal.clone()) {
            attributes = EMPTY_VAR_ATTR_INT().clone();
        } else {
            for mut attr in &*attrs.clone() {
                let mut attr = attr.clone();
                (name, b) = attr.clone();
                let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "quantity" => {
            quantity = createAttribute(b.clone());
            ()
        },
        Deref @ "min" => {
            min = createAttribute(b.clone());
            ()
        },
        Deref @ "max" => {
            max = createAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = createAttribute(b.clone());
            ()
        },
        Deref @ "fixed" => {
            fixed = createAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createInt")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            attributes = Arc::new(VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), uncertainty: None, distribution: None, binding: None, isProtected: None, finalPrefix: Some(isFinal.clone()), startOrigin: None });
        }
        Ok(attributes)
    }

    fn createBool(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: bool) -> Result<Arc<VariableAttributes>> {
        let mut attributes: Arc<VariableAttributes>;
        let mut name: ArcStr = arcstr::literal!("");
        let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        if attrs.clone().is_empty() && !(isFinal.clone()) {
            attributes = EMPTY_VAR_ATTR_BOOL().clone();
        } else {
            for mut attr in &*attrs.clone() {
                let mut attr = attr.clone();
                (name, b) = attr.clone();
                let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "quantity" => {
            quantity = createAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = createAttribute(b.clone());
            ()
        },
        Deref @ "fixed" => {
            fixed = createAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createBool")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            attributes = Arc::new(VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), binding: None, isProtected: None, finalPrefix: Some(isFinal.clone()), startOrigin: None });
        }
        Ok(attributes)
    }

    fn createString(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: bool) -> Result<Arc<VariableAttributes>> {
        let mut attributes: Arc<VariableAttributes>;
        let mut name: ArcStr = arcstr::literal!("");
        let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        if attrs.clone().is_empty() && !(isFinal.clone()) {
            attributes = EMPTY_VAR_ATTR_STRING().clone();
        } else {
            for mut attr in &*attrs.clone() {
                let mut attr = attr.clone();
                (name, b) = attr.clone();
                let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "quantity" => {
            quantity = createAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = createAttribute(b.clone());
            ()
        },
        Deref @ "fixed" => {
            fixed = createAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createString")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            attributes = Arc::new(VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: start.clone(), fixed: fixed.clone(), binding: None, isProtected: None, finalPrefix: Some(isFinal.clone()), startOrigin: None });
        }
        Ok(attributes)
    }

    fn createEnum(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut isFinal: bool) -> Result<Arc<VariableAttributes>> {
        let mut attributes: Arc<VariableAttributes>;
        let mut name: ArcStr = arcstr::literal!("");
        let mut b: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        let mut quantity: Option<Arc<Expression::NFExpression>> = None;
        let mut min: Option<Arc<Expression::NFExpression>> = None;
        let mut max: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut fixed: Option<Arc<Expression::NFExpression>> = None;
        if attrs.clone().is_empty() && !(isFinal.clone()) {
            attributes = EMPTY_VAR_ATTR_REAL().clone();
        } else {
            for mut attr in &*attrs.clone() {
                let mut attr = attr.clone();
                (name, b) = attr.clone();
                let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "fixed" => {
            fixed = createAttribute(b.clone());
            ()
        },
        Deref @ "max" => {
            max = createAttribute(b.clone());
            ()
        },
        Deref @ "min" => {
            min = createAttribute(b.clone());
            ()
        },
        Deref @ "quantity" => {
            quantity = createAttribute(b.clone());
            ()
        },
        Deref @ "start" => {
            start = createAttribute(b.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createEnum")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            attributes = Arc::new(VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: start.clone(), fixed: fixed.clone(), binding: None, isProtected: None, finalPrefix: Some(isFinal.clone()), startOrigin: None });
        }
        Ok(attributes)
    }

    fn createClock(mut isFinal: bool) -> Arc<VariableAttributes> {
        let mut attributes: Arc<VariableAttributes> = Arc::new(VariableAttributes::VAR_ATTR_CLOCK { isProtected: None, finalPrefix: Some(isFinal.clone()) });
        attributes
    }

    fn createRecord(mut attrs: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>>, mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>, mut children: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut isFinal: bool) -> Result<Arc<VariableAttributes>> {
        let mut attributes: Arc<VariableAttributes>;
        let mut childrenAttr: metamodelica::Array<Arc<VariableAttributes>> = arrayCreate((children.clone().len() as i32), EMPTY_VAR_ATTR_REAL().clone());
        let mut index: i32 = 0;
        for mut var in &*children.clone() {
            let mut var = var.clone();
            let _ = (match UnorderedMap::get((ComponentRef::firstName(var.name.clone(), false)?).clone(), indexMap.clone()) {
        Some(mut index) => {
            {
                let __cell0 = create(var.typeAttributes.clone(), var.ty.clone(), var.attributes.clone(), var.children.clone(), var.comment.clone())?;
                childrenAttr.clone().borrow_mut()[(index.clone()-1) as usize] = __cell0;
            }
            ()
        },
        _ => (),
    });
        }
        attributes = Arc::new(VariableAttributes::VAR_ATTR_RECORD { indexMap: indexMap.clone(), childrenAttr: childrenAttr.clone() });
        Ok(attributes)
    }

    fn createAttribute(mut binding: Arc<Binding::NFBinding>) -> Option<Arc<Expression::NFExpression>> {
        let mut attribute: Option<Arc<Expression::NFExpression>> = Some(Binding::getTypedExp(binding.clone()).unwrap());
        attribute
    }

    fn createStateSelect(mut binding: Arc<Binding::NFBinding>) -> Result<Option<StateSelect>> {
        let mut stateSelect: Option<StateSelect> = None;
        let mut exp: Arc<Expression::NFExpression> = Binding::getTypedExp(binding.clone())?;
        let mut name: ArcStr = arcstr::literal!("");
        name = (getStateSelectName(exp.clone())?).clone();
        stateSelect = Some(lookupStateSelectMember((name.clone()).clone())?);
        Ok(stateSelect)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    fn getStateSelectName(mut exp: Arc<Expression::NFExpression>) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut call: Arc<Call::NFCall>;
        let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        name = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ENUM_LITERAL { .. } => var_field!((*exp).name, Expression::NFExpression::ENUM_LITERAL).clone(),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { node, .. }, .. } => InstNode::name(node.clone())?,
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => getStateSelectName(var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone())?,
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: _ }, .. } } if (AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).clone())?, (literal!(".")).clone(), true, false)? == literal!("fill")) => getStateSelectName(arg.clone())?,
        Deref @ Expression::ARRAY { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Arc::new(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg = __pa0.clone();
            rest = __pa1.clone();
            if !(rest.clone().is_empty() || List::all(rest.clone(), Arc::new({ let __pe_b1 = arg.clone(); move |__pe_a0| Expression::isEqual(__pe_a0, __pe_b1.clone()) }))) {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.getStateSelectName")); __mm_s.push_str(&*literal!(" cannot handle array StateSelect with different values yet:")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                bail!("fail");
            }
            getStateSelectName(arg.clone())?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.getStateSelectName")); __mm_s.push_str(&*literal!(" got invalid StateSelect expression ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(name)
    }

    fn lookupStateSelectMember(mut name: ArcStr) -> Result<StateSelect> {
        let mut stateSelect: StateSelect = StateSelect::NEVER;
        stateSelect = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "never" => StateSelect::NEVER.clone(),
        Deref @ "avoid" => StateSelect::AVOID.clone(),
        Deref @ "default" => StateSelect::DEFAULT.clone(),
        Deref @ "prefer" => StateSelect::PREFER.clone(),
        Deref @ "always" => StateSelect::ALWAYS.clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.lookupStateSelectMember")); __mm_s.push_str(&*literal!(" got unknown StateSelect literal ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(stateSelect)
    }

    fn createTearingSelect(mut cmt: Arc<SCode::Comment>) -> Result<Option<TearingSelect>> {
        let mut tearingSelect: Option<TearingSelect> = None;
        let mut opt_anno: Option<Arc<SCode::Annotation>> = None;
        let mut anno: Arc<SCode::Annotation>;
        let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        let mut opt_val: Option<Arc<Absyn::Exp>> = None;
        let mut val: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
        let mut name: ArcStr = arcstr::literal!("");
        let mut info: SourceInfo;
        opt_anno = SCodeUtil::commentAnnotation(cmt.clone());
        if isNone(opt_anno.clone()) {
            return Ok(tearingSelect);
        }
        let __pa0 = ::match_deref::match_deref! { match &(opt_anno.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        anno = __pa0.clone();
        r#mod = SCodeUtil::lookupAnnotation(anno.clone(), (literal!("__OpenModelica_tearingSelect")).clone())?;
        if SCodeUtil::isEmptyMod(r#mod.clone()) {
            r#mod = SCodeUtil::lookupAnnotation(anno.clone(), (literal!("tearingSelect")).clone())?;
            if !(SCodeUtil::isEmptyMod(r#mod.clone())) {
                Error::addSourceMessage(Error::DEPRECATED_EXPRESSION.clone(), list![(literal!("tearingSelect")).clone(), (literal!("__OpenModelica_tearingSelect")).clone()], SCodeUtil::getModifierInfo(r#mod.clone()))?;
            }
        }
        opt_val = SCodeUtil::getModifierBinding(r#mod.clone());
        if isNone(opt_val.clone()) {
            return Ok(tearingSelect);
        }
        let __pa1 = ::match_deref::match_deref! { match &(opt_val.clone()) {
            Some(__pa1) => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        val = __pa1.clone();
        info = SCodeUtil::getModifierInfo(r#mod.clone());
        name = (getTearingSelectName(val.clone(), info.clone())?).clone();
        tearingSelect = lookupTearingSelectMember((name.clone()).clone());
        if isNone(tearingSelect.clone()) {
            Error::addSourceMessage(Error::UNKNOWN_ANNOTATION_VALUE.clone(), list![(Dump::printExpStr(val.clone())?).clone(), (literal!("__OpenModelica_tearingSelect")).clone()], info.clone())?;
        }
        Ok(tearingSelect)
    }

    fn getTearingSelectName(mut exp: Arc<Absyn::Exp>, mut info: SourceInfo) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        name = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name }, subscripts: Deref @ metamodelica::List::Nil, name: Deref @ "TearingSelect" } } => name.clone(),
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name } } => {
            Error::addSourceMessage(Error::DEPRECATED_EXPRESSION.clone(), list![(name.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TearingSelect.")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone()], info.clone())?;
            literal!("")
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(name)
    }

    fn lookupTearingSelectMember(mut name: ArcStr) -> Option<TearingSelect> {
        let mut tearingSelect: Option<TearingSelect> = None;
        tearingSelect = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "never" => Some(TearingSelect::NEVER.clone()),
        Deref @ "avoid" => Some(TearingSelect::AVOID.clone()),
        Deref @ "default" => Some(TearingSelect::DEFAULT.clone()),
        Deref @ "prefer" => Some(TearingSelect::PREFER.clone()),
        Deref @ "always" => Some(TearingSelect::ALWAYS.clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        tearingSelect
    }

}

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<VariableAttributes::VariableAttributes>
// Expr: Constructor { name: 'NFBackendExtension.VariableAttributes.VAR_ATTR_REAL', args: [Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFBackendExtension.VariableAttributes.VAR_ATTR_REAL'), field_names: ['quantity', 'unit', 'displayUnit', 'min', 'max', 'start', 'fixed', 'nominal', 'stateSelect', 'tearingSelect', 'uncertainty', 'distribution', 'binding', 'isProtected', 'finalPrefix', 'startOrigin'] }
pub fn EMPTY_VAR_ATTR_REAL() -> Arc<VariableAttributes::VariableAttributes> { todo!("non-Sync, non-const-emittable constant EMPTY_VAR_ATTR_REAL — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<VariableAttributes::VariableAttributes>
// Expr: Constructor { name: 'NFBackendExtension.VariableAttributes.VAR_ATTR_INT', args: [Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFBackendExtension.VariableAttributes.VAR_ATTR_INT'), field_names: ['quantity', 'min', 'max', 'start', 'fixed', 'uncertainty', 'distribution', 'binding', 'isProtected', 'finalPrefix', 'startOrigin'] }
pub fn EMPTY_VAR_ATTR_INT() -> Arc<VariableAttributes::VariableAttributes> { todo!("non-Sync, non-const-emittable constant EMPTY_VAR_ATTR_INT — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<VariableAttributes::VariableAttributes>
// Expr: Constructor { name: 'NFBackendExtension.VariableAttributes.VAR_ATTR_BOOL', args: [Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFBackendExtension.VariableAttributes.VAR_ATTR_BOOL'), field_names: ['quantity', 'start', 'fixed', 'binding', 'isProtected', 'finalPrefix', 'startOrigin'] }
pub fn EMPTY_VAR_ATTR_BOOL() -> Arc<VariableAttributes::VariableAttributes> { todo!("non-Sync, non-const-emittable constant EMPTY_VAR_ATTR_BOOL — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<VariableAttributes::VariableAttributes>
// Expr: Constructor { name: 'NFBackendExtension.VariableAttributes.VAR_ATTR_CLOCK', args: [Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFBackendExtension.VariableAttributes.VAR_ATTR_CLOCK'), field_names: ['isProtected', 'finalPrefix'] }
pub fn EMPTY_VAR_ATTR_CLOCK() -> Arc<VariableAttributes::VariableAttributes> { todo!("non-Sync, non-const-emittable constant EMPTY_VAR_ATTR_CLOCK — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<VariableAttributes::VariableAttributes>
// Expr: Constructor { name: 'NFBackendExtension.VariableAttributes.VAR_ATTR_STRING', args: [Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFBackendExtension.VariableAttributes.VAR_ATTR_STRING'), field_names: ['quantity', 'start', 'fixed', 'binding', 'isProtected', 'finalPrefix', 'startOrigin'] }
pub fn EMPTY_VAR_ATTR_STRING() -> Arc<VariableAttributes::VariableAttributes> { todo!("non-Sync, non-const-emittable constant EMPTY_VAR_ATTR_STRING — extend codegen") }

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<VariableAttributes::VariableAttributes>
// Expr: Constructor { name: 'NFBackendExtension.VariableAttributes.VAR_ATTR_ENUMERATION', args: [Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }, Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFBackendExtension.VariableAttributes.VAR_ATTR_ENUMERATION'), field_names: ['quantity', 'min', 'max', 'start', 'fixed', 'binding', 'isProtected', 'finalPrefix', 'startOrigin'] }
pub fn EMPTY_VAR_ATTR_ENUMERATION() -> Arc<VariableAttributes::VariableAttributes> { todo!("non-Sync, non-const-emittable constant EMPTY_VAR_ATTR_ENUMERATION — extend codegen") }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum StateSelect {
    NEVER = 1,
    AVOID = 2,
    DEFAULT = 3,
    PREFER = 4,
    ALWAYS = 5,
}
impl PartialOrd for StateSelect {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for StateSelect {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum TearingSelect {
    NEVER = 1,
    AVOID = 2,
    DEFAULT = 3,
    PREFER = 4,
    ALWAYS = 5,
}
impl PartialOrd for TearingSelect {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for TearingSelect {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Uncertainty {
    GIVEN = 1,
    SOUGHT = 2,
    REFINE = 3,
    PROPAGATE = 4,
}
impl PartialOrd for Uncertainty {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Uncertainty {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Distribution {
    pub name: Arc<Expression::NFExpression>,
    pub params: Arc<Expression::NFExpression>,
    pub paramNames: Arc<Expression::NFExpression>,
}

impl Default for Distribution {
    fn default() -> Self {
        Self {
            name: Default::default(),
            params: Default::default(),
            paramNames: Default::default(),
        }
    }
}

pub type DISTRIBUTION = Distribution;


pub mod Annotations {
    use super::*;
    /// all annotations that are vendor specific
    ///      note: doesn't include __OpenModelica_tearingSelect, this is considered a first class attribute
#[derive(Clone, Debug, PartialEq)]
    pub struct Annotations {
        pub hideResult: bool,
        pub resizable: bool,
        pub optimizable: bool,
        pub optimizerExpression: Option<OptimizerExpression>,
    }

    impl Default for Annotations {
        fn default() -> Self {
            Self {
                hideResult: Default::default(),
                resizable: Default::default(),
                optimizable: Default::default(),
                optimizerExpression: Default::default(),
            }
        }
    }

    pub type ANNOTATIONS = Annotations;

    pub fn create(mut comment: Arc<SCode::Comment>, mut attributes: Arc<Attributes::NFAttributes>) -> Arc<Annotations> {
        let mut annotations: Arc<Annotations> = EMPTY_ANNOTATIONS().clone();
        let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        let mut b: bool = false;
        if attributes.isResizable.clone() {
            assign_field!(annotations.resizable = true);
        }
        let _ = (::match_deref::match_deref! { match &(comment.clone()) {
        Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: r#mod @ Deref @ SCode::Mod::MOD { .. } }), .. } => {
            for mut submod in &*var_field!((**r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut submod = submod.clone();
                let _ = (::match_deref::match_deref! { match &(submod.clone()) {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. }, ident: Deref @ "HideResult" } => {
            assign_field!(annotations.hideResult = true);
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "__OpenModelica_resizable" } => {
            assign_field!(annotations.resizable = b.clone());
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "optimizable" } => {
            assign_field!(annotations.optimizable = b.clone());
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "isMayer" } => {
            assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::MAYER.clone()));
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "isLagrange" } => {
            assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::LAGRANGE.clone()));
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "isConstraint" } => {
            assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::PATH_CONSTRAINT.clone()));
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "isInitialConstraint" } => {
            assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::INITIAL_CONSTRAINT.clone()));
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "isFinalConstraint" } => {
            assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::FINAL_CONSTRAINT.clone()));
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "isInitialTime" } => {
            assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::INITIAL_TIME.clone()));
            ()
        },
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: b }), .. }, ident: Deref @ "isFinalTime" } => {
            assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::FINAL_TIME.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        annotations
    }

}

// TODO: how to use Initial or Final state? - better state-pair Real x_0 = x (initialState = true);  -> binding only for initial time / optimizer?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum OptimizerExpression {
    MAYER = 1,
    LAGRANGE = 2,
    PATH_CONSTRAINT = 3,
    INITIAL_CONSTRAINT = 4,
    FINAL_CONSTRAINT = 5,
    INITIAL_TIME = 6,
    FINAL_TIME = 7,
}
impl PartialOrd for OptimizerExpression {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for OptimizerExpression {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<Annotations::Annotations>
// Expr: Constructor { name: 'NFBackendExtension.Annotations.ANNOTATIONS', args: [Lit(Bool(false)), Lit(Bool(false)), Lit(Bool(false)), Call { func: 'NONE', args: [], named_args: [], ty: Option(Unknown), sig_ty: Unknown }], named_args: [], ty: RustStruct('NFBackendExtension.Annotations'), field_names: ['hideResult', 'resizable', 'optimizable', 'optimizerExpression'] }
pub fn EMPTY_ANNOTATIONS() -> Arc<Annotations::Annotations> { todo!("non-Sync, non-const-emittable constant EMPTY_ANNOTATIONS — extend codegen") }

