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

use crate::BaseModelica;
use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectBreakTree;
use crate::NFDimension as Dimension;
use crate::NFEquation;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFFunctionDerivative as FunctionDerivative;
use crate::NFFunctionInverse as FunctionInverse;
use crate::NFInst as Inst;
use crate::NFInst::InstSettings;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFLookupState::LookupState;
use crate::NFModifier::Modifier;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::*;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::InstBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

pub type NamedArg = (ArcStr, Arc<Expression::NFExpression>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypedArg {
    pub name: Option<ArcStr>,
    pub value: Arc<Expression::NFExpression>,
    pub ty: Arc<Type::NFType>,
    pub var: Prefixes::Variability,
    pub purity: Prefixes::Purity,
}

impl Default for TypedArg {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
            ty: Default::default(),
            var: Default::default(),
            purity: Default::default(),
        }
    }
}

pub type TYPED_ARG = TypedArg;


/// Determines which type of argument a slot accepts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SlotType {
    /// Only accepts positional arguments.
    POSITIONAL = 1,
    /// Only accepts named argument.
    NAMED = 2,
    /// Accepts both positional and named arguments.
    GENERIC = 3,
}
impl PartialOrd for SlotType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SlotType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for SlotType {
    fn default() -> Self { Self::POSITIONAL }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SlotEvalStatus {
    NOT_EVALUATED = 1,
    EVALUATING = 2,
    EVALUATED = 3,
}
impl PartialOrd for SlotEvalStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SlotEvalStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for SlotEvalStatus {
    fn default() -> Self { Self::NOT_EVALUATED }
}

pub mod Slot {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Slot {
        pub node: Arc<InstNode::InstNode>,
        pub ty: SlotType,
        pub default: Option<Arc<Expression::NFExpression>>,
        pub arg: Option<Arc<TypedArg>>,
        pub index: i32,
        pub evalStatus: SlotEvalStatus,
    }

    impl Default for Slot {
        fn default() -> Self {
            Self {
                node: Default::default(),
                ty: Default::default(),
                default: Default::default(),
                arg: Default::default(),
                index: Default::default(),
                evalStatus: Default::default(),
            }
        }
    }

    pub type SLOT = Slot;

    pub fn positional(mut slot: Arc<Slot>) -> bool {
        let mut pos: bool = false;
        pos = (match slot.ty.clone() {
        SlotType::POSITIONAL => true,
        SlotType::GENERIC => true,
        _ => false,
    });
        pos
    }

    pub fn named(mut slot: Arc<Slot>) -> bool {
        let mut pos: bool = false;
        pos = (match slot.ty.clone() {
        SlotType::NAMED => true,
        SlotType::GENERIC => true,
        _ => false,
    });
        pos
    }

    pub fn name(mut slot: Arc<Slot>) -> ArcStr {
        let mut name: ArcStr = InstNode::name(slot.node.clone()).unwrap();
        name
    }

    pub fn hasNode(mut node: Arc<InstNode::InstNode>, mut slot: Arc<Slot>) -> bool {
        let mut hasNode: bool = InstNode::refEqual(node.clone(), slot.node.clone());
        hasNode
    }

}

pub mod FunctionMatchKind {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FunctionMatchKind {
        /// Exact match.
        EXACT,
        /// Matched by casting one or more arguments. e.g. Integer to Real
        CAST,
        /// Matched with a generic type on one or more arguments e.g. function F<T> input T i; end F; F(1)
        GENERIC,
        /// Matched by vectorization
        VECTORIZED {
            vectDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>,
            vectorizedArgs: Arc<metamodelica::List<i32>>,
            baseMatch: Arc<FunctionMatchKind>,
        },
        NOT_COMPATIBLE,
    }
    pub use self::FunctionMatchKind::{EXACT,CAST,GENERIC,VECTORIZED,NOT_COMPATIBLE};
    pub fn isValid(mut mk: Arc<FunctionMatchKind>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(mk.clone()) {
        Deref @ NOT_COMPATIBLE { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isExact(mut mk: Arc<FunctionMatchKind>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(mk.clone()) {
        Deref @ EXACT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isVectorized(mut mk: Arc<FunctionMatchKind>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(mk.clone()) {
        Deref @ VECTORIZED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn isExactVectorized(mut mk: Arc<FunctionMatchKind>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(mk.clone()) {
        Deref @ VECTORIZED { baseMatch: Deref @ EXACT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

}

thread_local! { static __EXACT_MATCH_TLS: Arc<FunctionMatchKind::FunctionMatchKind> = Arc::new(crate::NFFunction::FunctionMatchKind::EXACT); }
pub fn EXACT_MATCH() -> Arc<FunctionMatchKind::FunctionMatchKind> { __EXACT_MATCH_TLS.with(|__t| __t.clone()) }

thread_local! { static __CAST_MATCH_TLS: Arc<FunctionMatchKind::FunctionMatchKind> = Arc::new(crate::NFFunction::FunctionMatchKind::CAST); }
pub fn CAST_MATCH() -> Arc<FunctionMatchKind::FunctionMatchKind> { __CAST_MATCH_TLS.with(|__t| __t.clone()) }

thread_local! { static __GENERIC_MATCH_TLS: Arc<FunctionMatchKind::FunctionMatchKind> = Arc::new(crate::NFFunction::FunctionMatchKind::GENERIC); }
pub fn GENERIC_MATCH() -> Arc<FunctionMatchKind::FunctionMatchKind> { __GENERIC_MATCH_TLS.with(|__t| __t.clone()) }

thread_local! { static __NO_MATCH_TLS: Arc<FunctionMatchKind::FunctionMatchKind> = Arc::new(crate::NFFunction::FunctionMatchKind::NOT_COMPATIBLE); }
pub fn NO_MATCH() -> Arc<FunctionMatchKind::FunctionMatchKind> { __NO_MATCH_TLS.with(|__t| __t.clone()) }

pub mod MatchedFunction {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MatchedFunction {
        pub func: Arc<Function::Function>,
        pub args: Arc<metamodelica::List<Arc<TypedArg>>>,
        pub mk: Arc<FunctionMatchKind::FunctionMatchKind>,
    }

    pub type MATCHED_FUNC = MatchedFunction;

    pub fn getExactMatches(mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction>>>) -> Arc<metamodelica::List<Arc<MatchedFunction>>> {
        let mut outFuncs: Arc<metamodelica::List<Arc<MatchedFunction>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<MatchedFunction>>> = metamodelica::nil();
        for mut mf in (matchedFunctions.clone()).into_iter().cloned() {
            if !(FunctionMatchKind::isExact(mf.mk.clone())) { continue; }
            let __x = mf.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        outFuncs
    }

    pub fn getExactVectorizedMatches(mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction>>>) -> Arc<metamodelica::List<Arc<MatchedFunction>>> {
        let mut outFuncs: Arc<metamodelica::List<Arc<MatchedFunction>>> = {
        let mut __acc: Arc<metamodelica::List<Arc<MatchedFunction>>> = metamodelica::nil();
        for mut mf in (matchedFunctions.clone()).into_iter().cloned() {
            if !(FunctionMatchKind::isExactVectorized(mf.mk.clone())) { continue; }
            let __x = mf.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        outFuncs
    }

    pub fn isVectorized(mut mf: Arc<MatchedFunction>) -> bool {
        let mut b: bool = FunctionMatchKind::isVectorized(mf.mk.clone());
        b
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum FunctionStatus {
    /// A builtin function.
    BUILTIN = 1,
    /// The initial status.
    INITIAL = 2,
    /// Constants in the function has been evaluated by EvalConstants.
    EVALUATED = 3,
    /// The function has been simplified by SimplifyModel.
    SIMPLIFIED = 4,
    /// The function has been added to the function tree.
    COLLECTED = 5,
}
impl PartialOrd for FunctionStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for FunctionStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for FunctionStatus {
    fn default() -> Self { Self::BUILTIN }
}

pub mod Function {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Function {
        pub path: Arc<Absyn::Path>,
        pub node: Arc<InstNode::InstNode>,
        pub inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
        pub outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
        pub locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
        pub interfaceDiffInfo: Option<Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>>,
        pub slots: Arc<metamodelica::List<Arc<Slot::Slot>>>,
        pub returnType: Arc<Type::NFType>,
        pub attributes: DAE::FunctionAttributes,
        pub derivatives: Arc<metamodelica::List<Arc<FunctionDerivative::NFFunctionDerivative>>>,
        pub derivedInputs: Arc<metamodelica::List<i32>>,
        pub inverses: metamodelica::Array<Arc<FunctionInverse::NFFunctionInverse>>,
        pub status: Pointer::Pointer<FunctionStatus>,
        /// Used during function evaluation to limit recursion.
        pub callCounter: Pointer::Pointer<i32>,
    }

    impl Default for Function {
        fn default() -> Self {
            Self {
                path: Default::default(),
                node: Default::default(),
                inputs: Default::default(),
                outputs: Default::default(),
                locals: Default::default(),
                interfaceDiffInfo: Default::default(),
                slots: Default::default(),
                returnType: Default::default(),
                attributes: Default::default(),
                derivatives: Default::default(),
                derivedInputs: Default::default(),
                inverses: Default::default(),
                status: Default::default(),
                callCounter: Default::default(),
            }
        }
    }

    pub type FUNCTION = Function;

    pub fn new(mut path: Arc<Absyn::Path>, mut node: Arc<InstNode::InstNode>, mut comments: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Result<Arc<Function>> {
        let mut r#fn: Arc<Function> = Arc::new(<Function as ::std::default::Default>::default());
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
        let mut attr: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
        let mut status: FunctionStatus = FunctionStatus::BUILTIN;
        (inputs, outputs, locals) = collectParams(node.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
        attr = makeAttributes(node.clone(), inputs.clone(), outputs.clone(), comments.clone())?;
        status = if (isBuiltinAttr(attr.clone())) {FunctionStatus::COLLECTED.clone()} else {FunctionStatus::INITIAL.clone()};
        r#fn = Arc::new(Function { path: path.clone(), node: node.clone(), inputs: inputs.clone(), outputs: outputs.clone(), locals: locals.clone(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: Arc::new(crate::NFType::UNKNOWN), attributes: attr.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: Pointer::create(status.clone()), callCounter: Pointer::create(0) });
        Ok(r#fn)
    }

    pub fn lookupFunctionSimple(mut functionName: ArcStr, mut scope: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut functionRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut found_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
        let mut functionPath: Arc<Absyn::Path>;
        let mut prefix: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        (functionRef, found_scope) = Lookup::lookupFunctionNameSilent(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (functionName.clone()).clone(), subscripts: metamodelica::nil() }), scope.clone(), context.clone())?;
        prefix = ComponentRef::fromNodeList(InstNode::scopeList(found_scope.clone(), false, metamodelica::nil())?)?;
        functionRef = ComponentRef::append(functionRef.clone(), prefix.clone())?;
        Ok(functionRef)
    }

    pub fn lookupFunction(mut functionName: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut functionRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut found_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut state: Arc<LookupState::LookupState> = Arc::new(LookupState::BEGIN);
        let mut functionPath: Arc<Absyn::Path>;
        let mut prefix: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut is_class: bool = false;
        if '__try0: {
            functionPath = unwrap_break_err!(AbsynUtil::crefToPath(functionName.clone()), '__try0);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            Error::addSourceMessageAndFail(Error::SUBSCRIPTED_FUNCTION_CALL.clone(), list![(Dump::printComponentRefStr(functionName.clone())?).clone()], info.clone())?;
        }
        (functionRef, found_scope) = Lookup::lookupFunctionName(functionName.clone(), scope.clone(), context.clone(), info.clone())?;
        is_class = InstNode::isClass(ComponentRef::node(functionRef.clone())?);
        prefix = ComponentRef::fromNodeList(InstNode::scopeList(found_scope.clone(), is_class.clone(), metamodelica::nil())?)?;
        functionRef = ComponentRef::append(functionRef.clone(), prefix.clone())?;
        Ok(functionRef)
    }

    pub fn instFunction(mut functionName: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, bool)> {
        let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut fn_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut specialBuiltin: bool = false;
        let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
        fn_ref = lookupFunction(functionName.clone(), scope.clone(), context.clone(), info.clone())?;
        (fn_ref, fn_node, specialBuiltin) = instFunctionRef(fn_ref.clone(), context.clone(), info.clone())?;
        Ok((fn_ref, fn_node, specialBuiltin))
    }

    pub fn instFunctionRef(mut fn_ref: Arc<ComponentRef::NFComponentRef>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>, bool)> {
        let mut fn_ref: Arc<ComponentRef::NFComponentRef> = fn_ref;
        let mut fn_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut specialBuiltin: bool = false;
        let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
        let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        fn_node = InstNode::classScope(ComponentRef::node(fn_ref.clone())?);
        cache = InstNode::getFuncCache(fn_node.clone())?;
        (fn_node, specialBuiltin) = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::FUNCTION { .. } => (fn_node.clone(), var_field!((*cache).specialBuiltin, CachedData::CachedData::FUNCTION).clone()),
        _ => {
            parent = if (InstNode::isRedeclare(ComponentRef::node(fn_ref.clone())?)? || ComponentRef::isSimple(fn_ref.clone())) {Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE)} else {ComponentRef::node(ComponentRef::rest(fn_ref.clone())?)?};
            if !(InstNode::isComponent(parent.clone())) {
                parent = Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE);
            }
            instFunction2(ComponentRef::toPath(fn_ref.clone())?, fn_node.clone(), context.clone(), info.clone(), parent.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((fn_ref, fn_node, specialBuiltin))
    }

    pub fn instFunctionNode(mut node: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<InstNode::InstNode>> {
        let mut node: Arc<InstNode::InstNode> = node;
        let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
        cache = InstNode::getFuncCache(node.clone())?;
        let () = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::FUNCTION { .. } => (),
        _ => {
            (node, _) = instFunction2(InstNode::fullPath(node.clone(), false), node.clone(), context.clone(), info.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn instFunction2(mut fnPath: Arc<Absyn::Path>, mut fnNode: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo, mut parent: Arc<InstNode::InstNode>) -> Result<(Arc<InstNode::InstNode>, bool)> {
        let mut fnNode: Arc<InstNode::InstNode> = fnNode;
        let mut specialBuiltin: bool = false;
        let mut def: Arc<SCode::Element> = InstNode::definition(fnNode.clone())?;
        (fnNode, specialBuiltin) = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::Element::CLASS { .. } if (SCodeUtil::isOperatorRecord(def.clone())) => {
            (fnNode, _) = instFunction3(fnNode.clone(), context.clone(), info.clone())?;
            fnNode = OperatorOverloading::instConstructor(fnPath.clone(), fnNode.clone(), context.clone(), info.clone())?;
            (fnNode.clone(), false)
        },
        Deref @ SCode::Element::CLASS { .. } if (SCodeUtil::isRecord(def.clone())) => {
            (fnNode, _) = instFunction3(fnNode.clone(), context.clone(), info.clone())?;
            fnNode = Record::instDefaultConstructor(fnPath.clone(), fnNode.clone(), context.clone(), info.clone())?;
            (fnNode.clone(), false)
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { .. }, restriction: SCode::Restriction::R_OPERATOR, .. } => {
            (fnNode, _) = instFunction3(fnNode.clone(), context.clone(), info.clone())?;
            fnNode = OperatorOverloading::instOperatorFunctions(fnNode.clone(), context.clone(), info.clone())?;
            (fnNode.clone(), false)
        },
        Deref @ SCode::Element::CLASS { classDef: cdef @ Deref @ SCode::ClassDef::OVERLOAD { .. }, .. } => {
            let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            for mut p in &*var_field!((**cdef).pathLst, SCode::ClassDef::OVERLOAD).clone() {
                let mut p = p.clone();
                cr = AbsynUtil::pathToCref(p.clone())?;
                (_, node, specialBuiltin) = instFunction(cr.clone(), fnNode.clone(), context.clone(), info.clone())?;
                for mut f in &*getCachedFuncs(node.clone())? {
                    let mut f = f.clone();
                    fnNode = InstNode::cacheAddFunc(fnNode.clone(), f.clone(), specialBuiltin.clone())?;
                }
            }
            (fnNode.clone(), false)
        },
        Deref @ SCode::Element::CLASS { .. } if (InstNode::isEnumerationType(fnNode.clone())) => {
            let mut r#fn: Arc<Function> = Arc::new(<Function as ::std::default::Default>::default());
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            let mut cmts: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
            node = makeEnumConversionOp(fnNode.clone())?;
            node = InstNode::makeRootClass(node.clone(), parent.clone(), None);
            (node, cmts) = instFunction3(node.clone(), context.clone(), info.clone())?;
            r#fn = new(fnPath.clone(), node.clone(), cmts.clone())?;
            fnNode = InstNode::cacheAddFunc(fnNode.clone(), r#fn.clone(), false)?;
            (fnNode.clone(), false)
        },
        Deref @ SCode::Element::CLASS { .. } => {
            let mut r#fn: Arc<Function> = Arc::new(<Function as ::std::default::Default>::default());
            let mut cmts: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
            if SCodeUtil::isOperator(def.clone()) {
                OperatorOverloading::checkOperatorRestrictions(fnNode.clone())?;
            }
            fnNode = InstNode::makeRootClass(fnNode.clone(), parent.clone(), None);
            (fnNode, cmts) = instFunction3(fnNode.clone(), context.clone(), info.clone())?;
            r#fn = new(fnPath.clone(), fnNode.clone(), cmts.clone())?;
            specialBuiltin = isSpecialBuiltin(r#fn.clone())?;
            assign_field!(
                r#fn.derivatives = FunctionDerivative::instDerivatives(fnNode.clone(), r#fn.clone())?,
                r#fn.inverses = FunctionInverse::instInverses(fnNode.clone(), r#fn.clone())?,
                r#fn.derivedInputs = instPartialDerivedVars(var_field!((*def).classDef, SCode::Element::CLASS).clone(), r#fn.inputs.clone(), r#fn.clone(), context.clone(), info.clone())?
            );
            fnNode = InstNode::cacheAddFunc(fnNode.clone(), r#fn.clone(), specialBuiltin.clone())?;
            (fnNode.clone(), specialBuiltin.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok((fnNode, specialBuiltin))
    }

    pub fn instFunction3(mut fnNode: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<SCode::Comment>>>)> {
        let mut fnNode: Arc<InstNode::InstNode> = fnNode;
        let mut cmts: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
        let mut def: Arc<SCode::Element>;
        let mut numError: i32 = Error::getNumErrorMessages();
        let mut fn_context: i32 = InstContext::set(context.clone(), InstContext::FUNCTION.clone());
        if let Ok(__iflet0) = Inst::instantiate(fnNode.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), fn_context.clone(), true) {
            fnNode = __iflet0;
        } else {
            let true = (Error::getNumErrorMessages() == numError.clone()) else { bail!("pattern mismatch") };
            def = InstNode::definition(fnNode.clone())?;
            Error::addSourceMessage(Error::UNKNOWN_ERROR_INST_FUNCTION.clone(), list![(SCodeDump::unparseElementStr(def.clone(), SCodeDump::defaultOptions.clone())?).clone()], SCodeUtil::elementInfo(def.clone()))?;
            bail!("fail");
        }
        cmts = InstNode::getComments(fnNode.clone(), metamodelica::nil());
        InstNode::cacheInitFunc(fnNode.clone())?;
        Inst::instExpressions(fnNode.clone(), fnNode.clone(), Arc::new(crate::NFSections::EMPTY), NFConnectBreakTree::new(), context.clone(), Inst::InstSettings::create())?;
        Ok((fnNode, cmts))
    }

    pub fn makeEnumConversionOp(mut enumNode: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
        let mut fnNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut def: Arc<SCode::ClassDef>;
        let mut fn_def: Arc<SCode::ClassDef>;
        let mut elem: Arc<SCode::Element>;
        let mut fn_elem: Arc<SCode::Element>;
        let mut params: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        let mut info: SourceInfo = InstNode::info(enumNode.clone())?;
        let mut enum_name: ArcStr = InstNode::name(enumNode.clone())?;
        elem = InstNode::definition(InstNode::resolveInner(Class::lastBaseClass(enumNode.clone())))?;
        fn_def = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::CLASS { classDef: def @ Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => {
            params = list![Arc::new(SCode::Element::COMPONENT { name: (literal!("index")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultInputAttr.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Integer")).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: info.clone() }), Arc::new(SCode::Element::COMPONENT { name: (literal!("value")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultOutputAttr.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (enum_name.clone()).clone() }), arrayDim: None }), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: info.clone() })];
            stmts = list![Arc::new(SCode::Statement::ALG_ASSERT { condition: Arc::new(Absyn::Exp::LBINARY { exp1: Arc::new(Absyn::Exp::RELATION { exp1: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("index")).clone(), subscripts: metamodelica::nil() }) }), op: openmodelica_ast::Absyn::Operator::GREATEREQ, exp2: Arc::new(Absyn::Exp::INTEGER { value: 1 }) }), op: openmodelica_ast::Absyn::Operator::AND, exp2: Arc::new(Absyn::Exp::RELATION { exp1: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("index")).clone(), subscripts: metamodelica::nil() }) }), op: openmodelica_ast::Absyn::Operator::LESSEQ, exp2: Arc::new(Absyn::Exp::INTEGER { value: (var_field!((**def).enumLst, SCode::ClassDef::ENUMERATION).clone().len() as i32) }) }) }), message: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::STRING { value: (literal!("Enumeration index '")).clone() }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: Arc::new(Absyn::Exp::BINARY { exp1: Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("String")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("index")).clone(), subscripts: metamodelica::nil() }) })], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: Arc::new(Absyn::Exp::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("' out of bounds in call to ")); __mm_s.push_str(&*enum_name.clone()); __mm_s.push_str(&*literal!("()")); ArcStr::from(__mm_s) }).clone() }) }) }), level: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("AssertionLevel")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("error")).clone(), subscripts: metamodelica::nil() }) }) }), comment: SCode::noComment.clone(), info: info.clone() }), Arc::new(SCode::Statement::ALG_ASSIGN { assignComponent: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("value")).clone(), subscripts: metamodelica::nil() }) }), value: Arc::new(Absyn::Exp::SUBSCRIPTED_EXP { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((**def).enumLst, SCode::ClassDef::ENUMERATION).clone()).into_iter().cloned() {
            let __x = Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (enum_name.clone()).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (e.literal.clone()).clone(), subscripts: metamodelica::nil() }) }) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } }), subscripts: list![Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("index")).clone(), subscripts: metamodelica::nil() }) }) })] }), comment: SCode::noComment.clone(), info: info.clone() })];
            Arc::new(SCode::ClassDef::PARTS { elementLst: params.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: list![Arc::new(SCode::AlgorithmSection { statements: stmts.clone() })], initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None })
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        fn_elem = Arc::new(SCode::Element::CLASS { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*enum_name.clone()); ArcStr::from(__mm_s) }).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: openmodelica_ast::Absyn::FunctionPurity::PURE } }, classDef: fn_def.clone(), cmt: Arc::new(SCode::Comment { annotation_: None, comment: Some(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Automatically generated conversion operator for ")); __mm_s.push_str(&*enum_name.clone()); ArcStr::from(__mm_s) }).clone()) }), info: info.clone() });
        fnNode = InstNode::new(fn_elem.clone(), InstNode::parentScope(enumNode.clone(), true)?)?;
        Ok(fnNode)
    }

    pub fn getCachedFuncs(mut inNode: Arc<InstNode::InstNode>) -> Result<Arc<metamodelica::List<Arc<Function>>>> {
        let mut outFuncs: Arc<metamodelica::List<Arc<Function>>> = metamodelica::nil();
        let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
        cache = InstNode::getFuncCache(InstNode::classScope(inNode.clone()))?;
        outFuncs = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::FUNCTION { .. } => var_field!((*cache).funcs, CachedData::CachedData::FUNCTION).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outFuncs)
    }

    pub fn mapCachedFuncs(mut inNode: Arc<InstNode::InstNode>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<Function>) -> Result<Arc<Function>> + 'static>) -> Result<()> {
        pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Function>) -> Result<Arc<Function>> + 'static>;

        let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
        cls_node = InstNode::classScope(inNode.clone());
        cache = InstNode::getFuncCache(cls_node.clone())?;
        cache = (::match_deref::match_deref! { match &(cache.clone()) {
        Deref @ CachedData::FUNCTION { .. } => {
            assign_variant_field!(cache => CachedData::CachedData::FUNCTION; funcs = {
        let mut __acc: Arc<metamodelica::List<Arc<Function>>> = metamodelica::nil();
        for mut r#fn in (var_field!((*cache).funcs, CachedData::CachedData::FUNCTION).clone()).into_iter().cloned() {
            let __x = mapFn(r#fn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            cache.clone()
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        InstNode::setFuncCache(cls_node.clone(), cache.clone())?;
        Ok(())
    }

    pub fn isEvaluated(mut r#fn: Arc<Function>) -> bool {
        let mut evaluated: bool = false;
        evaluated = (match Pointer::access(r#fn.status.clone()) {
        FunctionStatus::BUILTIN => true,
        FunctionStatus::EVALUATED => true,
        _ => false,
    });
        evaluated
    }

    pub fn markEvaluated(mut r#fn: Arc<Function>) -> () {
        if Pointer::access(r#fn.status.clone()) != FunctionStatus::BUILTIN.clone() {
            Pointer::update(r#fn.status.clone(), FunctionStatus::EVALUATED.clone());
        }
        ()
    }

    pub fn isSimplified(mut r#fn: Arc<Function>) -> bool {
        let mut simplified: bool = false;
        simplified = (match Pointer::access(r#fn.status.clone()) {
        FunctionStatus::BUILTIN => true,
        FunctionStatus::SIMPLIFIED => true,
        _ => false,
    });
        simplified
    }

    pub fn markSimplified(mut r#fn: Arc<Function>) -> () {
        if Pointer::access(r#fn.status.clone()) != FunctionStatus::BUILTIN.clone() {
            Pointer::update(r#fn.status.clone(), FunctionStatus::SIMPLIFIED.clone());
        }
        ()
    }

    pub fn isCollected(mut r#fn: Arc<Function>) -> bool {
        let mut collected: bool = false;
        collected = (match Pointer::access(r#fn.status.clone()) {
        FunctionStatus::BUILTIN => true,
        FunctionStatus::COLLECTED => true,
        _ => false,
    });
        collected
    }

    pub fn collect(mut r#fn: Arc<Function>) -> () {
        if Pointer::access(r#fn.status.clone()) != FunctionStatus::BUILTIN.clone() {
            Pointer::update(r#fn.status.clone(), FunctionStatus::COLLECTED.clone());
        }
        ()
    }

    pub fn name(mut r#fn: Arc<Function>) -> Arc<Absyn::Path> {
        let mut path: Arc<Absyn::Path> = r#fn.path.clone();
        path
    }

    pub fn setName(mut name: Arc<Absyn::Path>, mut r#fn: Arc<Function>) -> Arc<Function> {
        let mut r#fn: Arc<Function> = r#fn;
        assign_field!(r#fn.path = name.clone());
        r#fn
    }

    pub fn nameConsiderBuiltin(mut r#fn: Arc<Function>) -> Result<Arc<Absyn::Path>> {
        let mut path: Arc<Absyn::Path>;
        path = (match r#fn.attributes.isBuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some(mut name), .. } => {
            Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() })
        },
        DAE::FunctionBuiltin::FUNCTION_BUILTIN { .. } => {
            AbsynUtil::pathLast(r#fn.path.clone())?
        },
        _ => {
            r#fn.path.clone()
        },
    });
        Ok(path)
    }

    pub fn nameEqual(mut fn1: Arc<Function>, mut fn2: Arc<Function>) -> bool {
        let mut equal: bool = AbsynUtil::pathEqual(name(fn1.clone()), name(fn2.clone()));
        equal
    }

    pub fn nameHash(mut r#fn: Arc<Function>) -> i32 {
        let mut hash: i32 = AbsynUtil::pathHash(name(r#fn.clone())).unwrap();
        hash
    }

    pub fn signatureString(mut r#fn: Arc<Function>, mut printTypes: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut fn_name: Arc<Absyn::Path>;
        let mut input_str: ArcStr = arcstr::literal!("");
        let mut output_str: ArcStr = arcstr::literal!("");
        let mut var_s: ArcStr = arcstr::literal!("");
        let mut inputs_strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = r#fn.inputs.clone();
        let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut def_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        for mut s in &*r#fn.slots.clone() {
            let mut s = s.clone();
            input_str = (literal!("")).clone();
            c = InstNode::component(listHead(inputs.clone())?)?;
            inputs = listRest(inputs.clone())?;
            if isSome(s.default.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(s.default.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                def_exp = __pa0.clone();
                input_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(def_exp.clone())?); ArcStr::from(__mm_s) }).clone();
            }
            input_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Slot::name(s.clone())); __mm_s.push_str(&*input_str.clone()); ArcStr::from(__mm_s) }).clone();
            input_str = ((match s.ty.clone() {
        SlotType::POSITIONAL => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*input_str.clone()); ArcStr::from(__mm_s) },
        _ => input_str.clone(),
    })).clone();
            if printTypes.clone() && Component::isTyped(c.clone()) {
                ty = Component::getType(c.clone())?;
                var_s = (Prefixes::unparseVariability(Component::variability(c.clone()), ty.clone())).clone();
                input_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_s.clone()); __mm_s.push_str(&*Type::toString(ty.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*input_str.clone()); ArcStr::from(__mm_s) }).clone();
            }
            inputs_strl = cons((input_str.clone()).clone(), inputs_strl.clone());
        }
        input_str = stringDelimitList(inputs_strl.clone().reverse(), (literal!(", ")).clone());
        output_str = (if (printTypes.clone() && isTyped(r#fn.clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" => ")); __mm_s.push_str(&*Type::toString(r#fn.returnType.clone())?); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
        fn_name = nameConsiderBuiltin(r#fn.clone())?;
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(fn_name.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*input_str.clone()); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*output_str.clone()); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn candidateFuncListString(mut fns: Arc<metamodelica::List<Arc<Function>>>) -> ArcStr {
        let mut s: ArcStr = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut r#fn in (fns.clone()).into_iter().cloned() {
            let __x = signatureString(r#fn.clone(), true).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!("\n  ")).clone());
        s
    }

    pub fn callString(mut r#fn: Arc<Function>, mut posArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut namedArgs: Arc<metamodelica::List<(ArcStr, Arc<Expression::NFExpression>)>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (posArgs.clone()).into_iter().cloned() {
            let __x = Expression::toString(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
        if !(namedArgs.clone().is_empty()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut arg in (namedArgs.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*Util::tuple21(arg.clone())); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(Util::tuple22(arg.clone()))?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(r#fn.path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn typeString(mut r#fn: Arc<Function>) -> Result<ArcStr> {
        fn param_str(mut p: Arc<InstNode::InstNode>) -> ArcStr {
            let mut s: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*Type::toString(InstNode::getType(p.clone()).unwrap()).unwrap()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*InstNode::name(p.clone()).unwrap()); ArcStr::from(__mm_s) };
            s
        }

        let mut r#str: ArcStr = arcstr::literal!("");
        let mut inputs: ArcStr = arcstr::literal!("");
        let mut outputs: ArcStr = arcstr::literal!("");
        inputs = (List::toString(r#fn.inputs.clone(), (std::sync::Arc::new(fnptr!(param_str, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), true, 0)?).clone();
        outputs = (List::toString(r#fn.outputs.clone(), (std::sync::Arc::new(fnptr!(param_str, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), true, 0)?).clone();
        if r#fn.outputs.clone().is_empty() || (r#fn.outputs.clone().len() as i32) > 1 {
            outputs = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*outputs.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        }
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(name(r#fn.clone()), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("<function>(")); __mm_s.push_str(&*inputs.clone()); __mm_s.push_str(&*literal!(") => ")); __mm_s.push_str(&*outputs.clone()); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn toStream(mut r#fn: Arc<Function>, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        let mut fn_name: ArcStr = arcstr::literal!("");
        let mut cmt: Option<Arc<SCode::Comment>> = None;
        if isDefaultRecordConstructor(r#fn.clone()) {
            s = Record::toDeclarationStream(r#fn.node.clone(), (indent.clone()).clone(), s.clone())?;
        } else if isPartialDerivative(r#fn.clone()) {
            fn_name = (AbsynUtil::pathString(r#fn.path.clone(), (literal!(".")).clone(), true, false)?).clone();
            cmt = SCodeUtil::getElementComment(InstNode::definition(r#fn.node.clone())?);
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("function ")).clone())?;
            s = IOStream::append(s.clone(), (fn_name.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!(" = der(")).clone())?;
            s = IOStream::append(s.clone(), (AbsynUtil::pathString(getDerivedFunctionName(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone())?;
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            s = IOStream::append(s.clone(), stringDelimitList(getDerivedInputNames(r#fn.clone())?, (literal!(", ")).clone()))?;
            s = IOStream::append(s.clone(), (DAEDumpTypes::dumpCommentAnnotationStr(cmt.clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
        } else {
            fn_name = (AbsynUtil::pathString(r#fn.path.clone(), (literal!(".")).clone(), true, false)?).clone();
            cmt = SCodeUtil::getElementComment(InstNode::definition(r#fn.node.clone())?);
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            if InstNode::isPartial(r#fn.node.clone()) {
                s = IOStream::append(s.clone(), (literal!("partial ")).clone())?;
            }
            s = IOStream::append(s.clone(), (literal!("function ")).clone())?;
            s = IOStream::append(s.clone(), (fn_name.clone()).clone())?;
            s = IOStream::append(s.clone(), (DAEDumpTypes::dumpCommentStr(cmt.clone())).clone())?;
            s = IOStream::append(s.clone(), (literal!("\n")).clone())?;
            for mut i in &*r#fn.inputs.clone() {
                let mut i = i.clone();
                s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?;
                s = IOStream::append(s.clone(), (InstNode::toString(i.clone())?).clone())?;
                s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
            }
            for mut o in &*r#fn.outputs.clone() {
                let mut o = o.clone();
                s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?;
                s = IOStream::append(s.clone(), (InstNode::toString(o.clone())?).clone())?;
                s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
            }
            if !(r#fn.locals.clone().is_empty()) {
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                s = IOStream::append(s.clone(), (literal!("protected\n")).clone())?;
                for mut l in &*r#fn.locals.clone() {
                    let mut l = l.clone();
                    s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?;
                    s = IOStream::append(s.clone(), (InstNode::toString(l.clone())?).clone())?;
                    s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
                }
            }
            s = Sections::toStream(InstNode::getSections(r#fn.node.clone())?, (indent.clone()).clone(), s.clone())?;
            s = IOStream::append(s.clone(), (DAEDumpTypes::dumpClassAnnotationStr(cmt.clone())?).clone())?;
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("end ")).clone())?;
            s = IOStream::append(s.clone(), (fn_name.clone()).clone())?;
        }
        Ok(s)
    }

    pub fn toFlatStream(mut r#fn: Arc<Function>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream, mut overrideName: ArcStr) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        let mut fn_name: ArcStr = arcstr::literal!("");
        let mut fn_body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
        let mut annMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        if isDefaultRecordConstructor(r#fn.clone()) {
            s = Record::toFlatDeclarationStream(r#fn.node.clone(), format.clone(), (indent.clone()).clone(), s.clone())?;
        } else if isPartialDerivative(r#fn.clone()) {
            fn_name = (if (stringEmpty((overrideName.clone()).clone())) {Util::makeQuotedIdentifier((AbsynUtil::pathString(r#fn.path.clone(), (literal!(".")).clone(), true, false)?).clone())?} else {overrideName.clone()}).clone();
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("function ")).clone())?;
            s = IOStream::append(s.clone(), (fn_name.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!(" = der(")).clone())?;
            s = IOStream::append(s.clone(), (Util::makeQuotedIdentifier((AbsynUtil::pathString(getDerivedFunctionName(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone())?).clone())?;
            s = IOStream::append(s.clone(), (literal!(", ")).clone())?;
            s = IOStream::append(s.clone(), stringDelimitList(getDerivedInputNames(r#fn.clone())?, (literal!(", ")).clone()))?;
            s = FlatModelicaUtil::appendCommentOpt(SCodeUtil::getElementComment(InstNode::definition(r#fn.node.clone())?), FlatModelicaUtil::ElementType::FUNCTION.clone(), s.clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
        } else {
            cmt = Util::getOptionOrDefault(SCodeUtil::getElementComment(InstNode::definition(r#fn.node.clone())?), Arc::new(SCode::Comment { annotation_: None, comment: None }));
            fn_name = (if (stringEmpty((overrideName.clone()).clone())) {Util::makeQuotedIdentifier((AbsynUtil::pathString(r#fn.path.clone(), (literal!(".")).clone(), true, false)?).clone())?} else {overrideName.clone()}).clone();
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            if InstNode::isPartial(r#fn.node.clone()) {
                s = IOStream::append(s.clone(), (literal!("partial ")).clone())?;
            }
            s = IOStream::append(s.clone(), (literal!("function ")).clone())?;
            s = IOStream::append(s.clone(), (fn_name.clone()).clone())?;
            s = FlatModelicaUtil::appendCommentString(cmt.clone(), s.clone())?;
            s = IOStream::append(s.clone(), (literal!("\n")).clone())?;
            for mut i in &*r#fn.inputs.clone() {
                let mut i = i.clone();
                s = IOStream::append(s.clone(), (InstNode::toFlatString(i.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?).clone())?;
                s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
            }
            for mut o in &*r#fn.outputs.clone() {
                let mut o = o.clone();
                s = IOStream::append(s.clone(), (InstNode::toFlatString(o.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?).clone())?;
                s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
            }
            if !(r#fn.locals.clone().is_empty()) {
                for mut l in &*r#fn.locals.clone() {
                    let mut l = l.clone();
                    s = IOStream::append(s.clone(), (InstNode::toFlatString(l.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?).clone())?;
                    s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
                }
            }
            s = Sections::toFlatStream(InstNode::getSections(r#fn.node.clone())?, r#fn.path.clone(), format.clone(), (indent.clone()).clone(), s.clone())?;
            if isSome(cmt.annotation_.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(cmt.annotation_.clone()) {
                    Some(Deref @ SCode::Annotation { modification: __pa0 }) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                annMod = __pa0.clone();
            } else {
                annMod = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD);
            }
            annMod = SCodeUtil::filterSubMods(annMod.clone(), Arc::new({ let __pe_b1 = list![(literal!("derivative")).clone(), (literal!("inverse")).clone()]; move |__pe_a0| Ok(SCodeUtil::removeGivenSubModNames(__pe_a0, __pe_b1.clone())) }));
            for mut derivative in &*r#fn.derivatives.clone().reverse() {
                let mut derivative = derivative.clone();
                annMod = SCodeUtil::prependSubModToMod(FunctionDerivative::toSubMod(derivative.clone())?, annMod.clone())?;
            }
            let __range1 = (1..=(r#fn.inverses.clone().borrow().len() as i32)).rev();
            for mut i in __range1 {
                annMod = SCodeUtil::prependSubModToMod(FunctionInverse::toSubMod(r#fn.inverses.borrow()[(i.clone()-1) as usize].clone())?, annMod.clone())?;
            }
            if !(SCodeUtil::emptyModOrEquality(annMod.clone())) {
                cmt = Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: annMod.clone() })), comment: None });
                s = FlatModelicaUtil::appendCommentAnnotation(cmt.clone(), FlatModelicaUtil::ElementType::FUNCTION.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), (literal!(";\n")).clone(), s.clone())?;
            }
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("end ")).clone())?;
            s = IOStream::append(s.clone(), (fn_name.clone()).clone())?;
        }
        Ok(s)
    }

    pub fn toFlatString(mut r#fn: Arc<Function>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut s: IOStream::IOStream;
        s = IOStream::create((literal!("NFFunction.Function.toFlatString")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
        s = toFlatStream(r#fn.clone(), format.clone(), (indent.clone()).clone(), s.clone(), (literal!("")).clone())?;
        r#str = (IOStream::string(s.clone())?).clone();
        IOStream::delete(s.clone())?;
        Ok(r#str)
    }

    pub fn instance(mut r#fn: Arc<Function>) -> Arc<InstNode::InstNode> {
        let mut node: Arc<InstNode::InstNode> = r#fn.node.clone();
        node
    }

    pub fn returnType(mut r#fn: Arc<Function>) -> Arc<Type::NFType> {
        let mut ty: Arc<Type::NFType> = r#fn.returnType.clone();
        ty
    }

    pub fn setReturnType(mut ty: Arc<Type::NFType>, mut r#fn: Arc<Function>) -> Arc<Function> {
        let mut r#fn: Arc<Function> = r#fn;
        assign_field!(r#fn.returnType = ty.clone());
        r#fn
    }

    pub fn getSlots(mut r#fn: Arc<Function>) -> Arc<metamodelica::List<Arc<Slot::Slot>>> {
        let mut slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = r#fn.slots.clone();
        slots
    }

    pub fn fillArgs(mut posArgs: Arc<metamodelica::List<Arc<TypedArg>>>, mut namedArgs: Arc<metamodelica::List<Arc<TypedArg>>>, mut r#fn: Arc<Function>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<TypedArg>>>, bool)> {
        let mut args: Arc<metamodelica::List<Arc<TypedArg>>> = posArgs.clone();
        let mut matching: bool = false;
        let mut slot: Arc<Slot::Slot> = Arc::new(<Slot::Slot as ::std::default::Default>::default());
        let mut slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
        let mut remaining_slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
        let mut filled_named_args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
        let mut slots_arr: metamodelica::Array<Arc<Slot::Slot>>;
        let mut pos_arg_count: i32 = 0;
        let mut slot_count: i32 = 0;
        let mut index: i32 = 1;
        slots = r#fn.slots.clone();
        pos_arg_count = (posArgs.clone().len() as i32);
        slot_count = (slots.clone().len() as i32);
        if pos_arg_count.clone() > slot_count.clone() {
            matching = false;
            return Ok((args, matching));
        } else if pos_arg_count.clone() == slot_count.clone() && namedArgs.clone().is_empty() && List::all(slots.clone(), (std::sync::Arc::new(fnptr!(Slot::positional, Arc<Slot::Slot>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Slot::Slot>) -> Result<bool> + 'static>)) {
            matching = true;
            return Ok((args, matching));
        }
        slots_arr = metamodelica::arrayFromVec(slots.clone().into_iter().cloned().collect());
        for mut arg in &*args.clone() {
            let mut arg = arg.clone();
            slot = slots_arr.borrow()[(index.clone()-1) as usize].clone();
            if !(Slot::positional(slot.clone())) {
                matching = false;
                return Ok((args, matching));
            }
            assign_field!(slot.arg = Some(arg.clone()));
            {let _arr = slots_arr.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = slot.clone(); _arr};
            index = index.clone() + 1;
        }
        for mut narg in &*namedArgs.clone() {
            let mut narg = narg.clone();
            (slots_arr, matching) = fillNamedArg(narg.clone(), slots_arr.clone(), r#fn.clone(), info.clone())?;
            if !(matching.clone()) {
                return Ok((args, matching));
            }
        }
        (args, matching) = collectArgs(slots_arr.clone(), context.clone(), info.clone())?;
        Ok((args, matching))
    }

    pub fn fillNamedArg(mut arg: Arc<TypedArg>, mut slots: metamodelica::Array<Arc<Slot::Slot>>, mut r#fn: Arc<Function>, mut info: SourceInfo) -> Result<(metamodelica::Array<Arc<Slot::Slot>>, bool)> {
        let mut slots: metamodelica::Array<Arc<Slot::Slot>> = slots;
        let mut matching: bool = true;
        let mut s: Arc<Slot::Slot> = Arc::new(<Slot::Slot as ::std::default::Default>::default());
        let mut arg_name: ArcStr = arcstr::literal!("");
        let __range0 = (1..=(slots.clone().borrow().len() as i32)).rev();
        for mut i in __range0 {
            s = slots.borrow()[(i.clone()-1) as usize].clone();
            let Some(__pa1) = (arg.name.clone()) else { bail!("pattern mismatch") };
            arg_name = __pa1.clone();
            if Slot::name(s.clone()) == arg_name.clone() {
                if !(Slot::named(s.clone())) {
                    matching = false;
                } else if isNone(s.arg.clone()) {
                    assign_field!(s.arg = Some(arg.clone()));
                    {
                        let __cell2 = s.clone();
                        slots.clone().borrow_mut()[(i.clone()-1) as usize] = __cell2;
                    }
                } else {
                    Error::addSourceMessage(Error::FUNCTION_SLOT_ALREADY_FILLED.clone(), list![(arg_name.clone()).clone(), (literal!("")).clone()], info.clone())?;
                    matching = false;
                }
                return Ok((slots, matching));
            }
        }
        matching = false;
        for mut s in &*r#fn.slots.clone() {
            let mut s = s.clone();
            if arg_name.clone() == Slot::name(s.clone()) {
                Error::addSourceMessage(Error::FUNCTION_SLOT_ALREADY_FILLED.clone(), list![(arg_name.clone()).clone(), (literal!("")).clone()], info.clone())?;
                return Ok((slots, matching));
            }
            Error::addSourceMessage(Error::NO_SUCH_INPUT_PARAMETER.clone(), list![(InstNode::name(instance(r#fn.clone()))?).clone(), (arg_name.clone()).clone()], info.clone())?;
        }
        Ok((slots, matching))
    }

    pub fn collectArgs(mut slots: metamodelica::Array<Arc<Slot::Slot>>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<Arc<TypedArg>>>, bool)> {
        let mut args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
        let mut matching: bool = true;
        let mut default: Option<Arc<Expression::NFExpression>> = None;
        let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut arg: Option<Arc<TypedArg>> = None;
        let mut a: Arc<TypedArg> = Arc::new(<TypedArg as ::std::default::Default>::default());
        let __range0 = slots.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut s in __range0 {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(s.clone()) {
                Deref @ Slot::SLOT { arg: __pa1, default: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg = __pa1.clone();
            default = __pa2.clone();
            args = 'mc: {
        let __mc_input = arg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(a) => {
                    Ok(cons(a.clone(), args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cons(fillDefaultSlot(s.clone(), slots.clone(), context.clone(), info.clone())?, args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut matching: bool = matching.clone();
                    matching = false;
                    Ok(args.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        }
        args = args.clone().reverse();
        Ok((args, matching))
    }

    pub fn fillDefaultSlot(mut slot: Arc<Slot::Slot>, mut slots: metamodelica::Array<Arc<Slot::Slot>>, mut context: i32, mut info: SourceInfo) -> Result<Arc<TypedArg>> {
        let mut outArg: Arc<TypedArg> = Arc::new(<TypedArg as ::std::default::Default>::default());
        outArg = (::match_deref::match_deref! { match &(slot.clone()) {
        Deref @ Slot::SLOT { arg: Some(outArg), .. } => outArg.clone(),
        Deref @ Slot::SLOT { default: Some(_), .. } => fillDefaultSlot2(slot.clone(), slots.clone(), context.clone(), info.clone())?,
        _ => {
            Error::addSourceMessage(Error::UNFILLED_SLOT.clone(), list![(Slot::name(slot.clone())).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outArg)
    }

    pub fn fillDefaultSlot2(mut slot: Arc<Slot::Slot>, mut slots: metamodelica::Array<Arc<Slot::Slot>>, mut context: i32, mut info: SourceInfo) -> Result<Arc<TypedArg>> {
        let mut outArg: Arc<TypedArg> = Arc::new(<TypedArg as ::std::default::Default>::default());
        outArg = (match slot.evalStatus.clone() {
        SlotEvalStatus::EVALUATED => {
            Util::getOption(slot.arg.clone())?
        },
        SlotEvalStatus::EVALUATING => {
            Error::addSourceMessage(Error::CYCLIC_DEFAULT_VALUE.clone(), list![(Slot::name(slot.clone())).clone()], info.clone())?;
            bail!("fail")
        },
        SlotEvalStatus::NOT_EVALUATED => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
            let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
            let mut pur: Prefixes::Purity = Prefixes::Purity::PURE;
            assign_field!(slot.evalStatus = SlotEvalStatus::EVALUATING.clone());
            {let _arr = slots.clone(); _arr.borrow_mut()[(slot.index.clone()-1) as usize] = slot.clone(); _arr};
            exp = evaluateSlotExp(Util::getOption(slot.default.clone())?, slots.clone(), context.clone(), info.clone())?;
            (exp, ty, var, pur) = Typing::typeExp(exp.clone(), context.clone(), info.clone(), false)?;
            outArg = Arc::new(TypedArg { name: None, value: exp.clone(), ty: ty.clone(), var: var.clone(), purity: pur.clone() });
            assign_field!(
                slot.arg = Some(outArg.clone()),
                slot.evalStatus = SlotEvalStatus::EVALUATED.clone()
            );
            {let _arr = slots.clone(); _arr.borrow_mut()[(slot.index.clone()-1) as usize] = slot.clone(); _arr};
            outArg.clone()
        },
        _ => bail!("match: no arm matched"),
    });
        Ok(outArg)
    }

    pub fn evaluateSlotExp(mut exp: Arc<Expression::NFExpression>, mut slots: metamodelica::Array<Arc<Slot::Slot>>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
        let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        outExp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = slots.clone(); let __pe_b2 = context.clone(); let __pe_b3 = info.clone(); move |__pe_a0| evaluateSlotExp_traverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
        Ok(outExp)
    }

    pub fn evaluateSlotExp_traverser(mut exp: Arc<Expression::NFExpression>, mut slots: metamodelica::Array<Arc<Slot::Slot>>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
        let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => evaluateSlotCref(exp.clone(), slots.clone(), context.clone(), info.clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outExp)
    }

    pub fn evaluateSlotCref(mut crefExp: Arc<Expression::NFExpression>, mut slots: metamodelica::Array<Arc<Slot::Slot>>, mut context: i32, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
        let mut crefExp: Arc<Expression::NFExpression> = crefExp;
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut cref_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut cref_parts: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut slot: Option<Arc<Slot::Slot>> = None;
        let mut arg: Arc<TypedArg> = Arc::new(<TypedArg as ::std::default::Default>::default());
        let mut cref_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefExp.clone()) {
            Deref @ Expression::CREF { ty: __pa0, cref: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cref_ty = __pa0.clone();
        cref = __pa1.clone();
        if !(ComponentRef::isCref(cref.clone())) {
            return Ok(crefExp);
        }
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ComponentRef::toListReverse(cref.clone(), true, metamodelica::nil())) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cref = __pa2.clone();
        cref_parts = __pa3.clone();
        cref_node = ComponentRef::node(cref.clone())?;
        slot = lookupSlotInArray(cref_node.clone(), slots.clone());
        if isSome(slot.clone()) {
            arg = fillDefaultSlot(Util::getOption(slot.clone())?, slots.clone(), context.clone(), info.clone())?;
            crefExp = arg.value.clone();
            crefExp = applyCrefSubs(cref.clone(), crefExp.clone())?;
            for mut cr in &*cref_parts.clone() {
                let mut cr = cr.clone();
                crefExp = Expression::recordElement((ComponentRef::firstName(cr.clone(), false)?).clone(), crefExp.clone())?;
                crefExp = applyCrefSubs(cref.clone(), crefExp.clone())?;
            }
            if Type::isKnown(cref_ty.clone()) {
                (crefExp, _, _) = TypeCheck::matchTypes(Expression::typeOf(crefExp.clone()), cref_ty.clone(), crefExp.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
            }
        }
        Ok(crefExp)
    }

    pub fn applyCrefSubs(mut cref: Arc<ComponentRef::NFComponentRef>, mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        subs = ComponentRef::getSubscripts(cref.clone());
        if subs.clone().is_empty() {
            return Ok(exp);
        }
        match '__try0: {
            exp = unwrap_break_err!(Expression::applySubscripts(subs.clone(), exp.clone(), false), '__try0);
            Ok::<_, anyhow::Error>((exp.clone(),))
        } {
            Ok((__try0_o0,)) => {
                exp = __try0_o0;
            }
            Err(_) => {
                exp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: exp.clone(), subscripts: subs.clone(), ty: ComponentRef::getSubscriptedType(cref.clone(), false)?, split: false });
            }
        }
        Ok(exp)
    }

    pub fn lookupSlotInArray(mut node: Arc<InstNode::InstNode>, mut slots: metamodelica::Array<Arc<Slot::Slot>>) -> Option<Arc<Slot::Slot>> {
        let mut outSlot: Option<Arc<Slot::Slot>> = None;
        let mut slot: Arc<Slot::Slot> = Arc::new(<Slot::Slot as ::std::default::Default>::default());
        match '__try0: {
            (slot, _) = unwrap_break_err!(Array::getMemberOnTrue(node.clone(), slots.clone(), (std::sync::Arc::new(fnptr!(Slot::hasNode, Arc<InstNode::InstNode>, Arc<Slot::Slot>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<Slot::Slot>) -> Result<bool> + 'static>)), '__try0);
            outSlot = Some(slot.clone());
            Ok::<_, anyhow::Error>((outSlot.clone(),))
        } {
            Ok((__try0_o0,)) => {
                outSlot = __try0_o0;
            }
            Err(_) => {
                outSlot = None;
            }
        }
        outSlot
    }

    pub fn matchArgs(mut func: Arc<Function>, mut args: Arc<metamodelica::List<Arc<TypedArg>>>, mut info: SourceInfo, mut vectorize: bool) -> Result<(Arc<metamodelica::List<Arc<TypedArg>>>, Arc<FunctionMatchKind::FunctionMatchKind>)> {
        let mut args: Arc<metamodelica::List<Arc<TypedArg>>> = args;
        let mut funcMatchKind: Arc<FunctionMatchKind::FunctionMatchKind> = EXACT_MATCH().clone();
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = func.inputs.clone();
        let mut input_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut arg_idx: i32 = 1;
        let mut checked_args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
        let mut arg_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut arg_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut input_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut arg_var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
        let mut mk: MatchKind = MatchKind::EXACT;
        let mut vect_arg: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
        let mut vect_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut matched: bool = false;
        let mut vectorized_args: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut arg in &*args.clone() {
            let mut arg = arg.clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(arg.clone()) {
                Deref @ TypedArg { var: __pa0, ty: __pa1, value: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg_var = __pa0.clone();
            arg_ty = __pa1.clone();
            arg_exp = __pa2.clone();
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inputs.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            input_node = __pa3.clone();
            inputs = __pa4.clone();
            comp = InstNode::component(input_node.clone())?;
            if arg_var.clone() > Component::variability(comp.clone()) {
                Error::addSourceMessage(Error::FUNCTION_SLOT_VARIABILITY.clone(), list![(InstNode::name(input_node.clone())?).clone(), (Expression::toString(arg_exp.clone())?).clone(), (AbsynUtil::pathString(name(func.clone()), (literal!(".")).clone(), true, false)?).clone(), (Prefixes::variabilityString(arg_var.clone())?).clone(), (Prefixes::variabilityString(Component::variability(comp.clone()))?).clone()], info.clone())?;
                funcMatchKind = NO_MATCH().clone();
                return Ok((args, funcMatchKind));
            }
            input_ty = Component::getType(comp.clone())?;
            (arg_exp, ty, mk) = TypeCheck::matchTypes(arg_ty.clone(), input_ty.clone(), arg_exp.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
            matched = TypeCheck::isValidArgumentMatch(mk.clone());
            if !(matched.clone()) && vectorize.clone() {
                (arg_exp, ty, vect_arg, vect_dims, mk) = matchArgVectorized(arg_exp.clone(), arg_ty.clone(), input_ty.clone(), vect_arg.clone(), vect_dims.clone(), info.clone())?;
                vectorized_args = cons(arg_idx.clone(), vectorized_args.clone());
                matched = TypeCheck::isValidArgumentMatch(mk.clone());
            }
            if !(matched.clone()) {
                Error::addSourceMessage(Error::ARG_TYPE_MISMATCH.clone(), list![(intString(arg_idx.clone())).clone(), (AbsynUtil::pathString(func.path.clone(), (literal!(".")).clone(), true, false)?).clone(), (InstNode::name(input_node.clone())?).clone(), (Expression::toString(arg_exp.clone())?).clone(), (Type::toString(arg_ty.clone())?).clone(), (Type::toString(input_ty.clone())?).clone()], info.clone())?;
                funcMatchKind = NO_MATCH().clone();
                return Ok((args, funcMatchKind));
            }
            if TypeCheck::isCastMatch(mk.clone()) {
                funcMatchKind = CAST_MATCH().clone();
            } else if TypeCheck::isGenericMatch(mk.clone()) {
                funcMatchKind = GENERIC_MATCH().clone();
            }
            checked_args = cons(Arc::new(TypedArg { name: arg.name.clone(), value: arg_exp.clone(), ty: ty.clone(), var: arg_var.clone(), purity: arg.purity.clone() }), checked_args.clone());
            arg_idx = arg_idx.clone() + 1;
        }
        if !(vectorized_args.clone().is_empty()) {
            funcMatchKind = Arc::new(FunctionMatchKind::FunctionMatchKind::VECTORIZED { vectDims: vect_dims.clone(), vectorizedArgs: vectorized_args.clone().reverse(), baseMatch: funcMatchKind.clone() });
        }
        args = checked_args.clone().reverse();
        Ok((args, funcMatchKind))
    }

    pub fn matchArgVectorized(mut argExp: Arc<Expression::NFExpression>, mut argTy: Arc<Type::NFType>, mut inputTy: Arc<Type::NFType>, mut vectArg: Arc<Expression::NFExpression>, mut vectDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, MatchKind)> {
        let mut argExp: Arc<Expression::NFExpression> = argExp;
        let mut argTy: Arc<Type::NFType> = argTy;
        let mut vectArg: Arc<Expression::NFExpression> = vectArg;
        let mut vectDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = vectDims;
        let mut matchKind: MatchKind = MatchKind::EXACT;
        let mut arg_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut input_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut vect_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut rest_dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut rest_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut mk: MatchKind = MatchKind::EXACT;
        let mut vect_dims_count: i32 = 0;
        arg_dims = Type::arrayDims(argTy.clone());
        input_dims = Type::arrayDims(inputTy.clone());
        vect_dims_count = (arg_dims.clone().len() as i32) - (input_dims.clone().len() as i32);
        if vect_dims_count.clone() < 1 {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            return Ok((argExp, argTy, vectArg, vectDims, matchKind));
        }
        (vect_dims, rest_dims) = List::split(arg_dims.clone(), vect_dims_count.clone())?;
        if vectDims.clone().is_empty() {
            vectDims = fillUnknownVectorizedDims(vect_dims.clone(), argExp.clone());
            vectArg = argExp.clone();
        } else if !(List::isEqualOnTrue(vectDims.clone(), vect_dims.clone(), (std::sync::Arc::new(Dimension::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>, Arc<Dimension::NFDimension>) -> Result<bool> + 'static>))) {
            Error::addSourceMessage(Error::VECTORIZE_CALL_DIM_MISMATCH.clone(), list![(literal!("")).clone(), (Expression::toString(vectArg.clone())?).clone(), (literal!("")).clone(), (Expression::toString(argExp.clone())?).clone(), (Dimension::toStringList(vectDims.clone(), true)?).clone(), (Dimension::toStringList(vect_dims.clone(), true)?).clone()], info.clone())?;
        }
        rest_ty = Type::liftArrayLeftList(Type::arrayElementType(argTy.clone()), rest_dims.clone());
        (argExp, argTy, matchKind) = TypeCheck::matchTypes(rest_ty.clone(), inputTy.clone(), argExp.clone(), TypeCheck::DEFAULT_OPTIONS.clone())?;
        Ok((argExp, argTy, vectArg, vectDims, matchKind))
    }

    pub fn fillUnknownVectorizedDims(mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut argExp: Arc<Expression::NFExpression>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
        let mut outDims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        let mut i: i32 = 1;
        for mut dim in &*dims.clone() {
            let mut dim = dim.clone();
            if Dimension::isUnknown(dim.clone()) {
                dim = Arc::new(Dimension::NFDimension::EXP { exp: Arc::new(Expression::NFExpression::SIZE { exp: argExp.clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: i.clone() })) }), var: Variability::CONTINUOUS.clone() });
            }
            outDims = cons(dim.clone(), outDims.clone());
            i = i.clone() + 1;
        }
        outDims = outDims.clone().reverse();
        outDims
    }

    pub fn matchFunction(mut func: Arc<Function>, mut args: Arc<metamodelica::List<Arc<TypedArg>>>, mut named_args: Arc<metamodelica::List<Arc<TypedArg>>>, mut context: i32, mut info: SourceInfo, mut vectorize: bool) -> Result<(Arc<metamodelica::List<Arc<TypedArg>>>, Arc<FunctionMatchKind::FunctionMatchKind>)> {
        let mut out_args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
        let mut matchKind: Arc<FunctionMatchKind::FunctionMatchKind> = NO_MATCH().clone();
        let mut slot_matched: bool = false;
        (out_args, slot_matched) = fillArgs(args.clone(), named_args.clone(), func.clone(), context.clone(), info.clone())?;
        if slot_matched.clone() {
            (out_args, matchKind) = matchArgs(func.clone(), out_args.clone(), info.clone(), vectorize.clone())?;
        }
        Ok((out_args, matchKind))
    }

    pub fn matchFunctions(mut funcs: Arc<metamodelica::List<Arc<Function>>>, mut args: Arc<metamodelica::List<Arc<TypedArg>>>, mut named_args: Arc<metamodelica::List<Arc<TypedArg>>>, mut context: i32, mut info: SourceInfo, mut vectorize: bool) -> Result<Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>>> {
        let mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
        let mut m_args: Arc<metamodelica::List<Arc<TypedArg>>> = metamodelica::nil();
        let mut matchKind: Arc<FunctionMatchKind::FunctionMatchKind> = Arc::new(FunctionMatchKind::CAST);
        let mut matched: bool = false;
        matchedFunctions = metamodelica::nil();
        for mut func in &*funcs.clone() {
            let mut func = func.clone();
            (m_args, matchKind) = matchFunction(func.clone(), args.clone(), named_args.clone(), context.clone(), info.clone(), vectorize.clone())?;
            if FunctionMatchKind::isValid(matchKind.clone()) {
                matchedFunctions = cons(Arc::new(MatchedFunction::MatchedFunction { func: func.clone(), args: m_args.clone(), mk: matchKind.clone() }), matchedFunctions.clone());
            }
        }
        Ok(matchedFunctions)
    }

    pub fn matchFunctionsSilent(mut funcs: Arc<metamodelica::List<Arc<Function>>>, mut args: Arc<metamodelica::List<Arc<TypedArg>>>, mut named_args: Arc<metamodelica::List<Arc<TypedArg>>>, mut context: i32, mut info: SourceInfo, mut vectorize: bool) -> Result<Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>>> {
        let mut matchedFunctions: Arc<metamodelica::List<Arc<MatchedFunction::MatchedFunction>>> = metamodelica::nil();
        ErrorExt::setCheckpoint((literal!("NFFunction:matchFunctions")).clone());
        matchedFunctions = matchFunctions(funcs.clone(), args.clone(), named_args.clone(), context.clone(), info.clone(), vectorize.clone())?;
        ErrorExt::rollBack((literal!("NFFunction:matchFunctions")).clone());
        Ok(matchedFunctions)
    }

    pub fn isTyped(mut r#fn: Arc<Function>) -> bool {
        let mut isTyped: bool = false;
        isTyped = (::match_deref::match_deref! { match &(r#fn.returnType.clone()) {
        Deref @ Type::UNKNOWN => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isTyped
    }

    pub fn typeRefCache(mut functionRef: Arc<ComponentRef::NFComponentRef>, mut context: i32) -> Result<Arc<metamodelica::List<Arc<Function>>>> {
        let mut functions: Arc<metamodelica::List<Arc<Function>>> = metamodelica::nil();
        functions = (::match_deref::match_deref! { match &(functionRef.clone()) {
        Deref @ ComponentRef::CREF { .. } => typeNodeCache(var_field!((*functionRef).node, ComponentRef::NFComponentRef::CREF).clone(), context.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.typeRefCache")); __mm_s.push_str(&*literal!(" got invalid function call reference")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(functions)
    }

    pub fn typeNodeCache(mut functionNode: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<metamodelica::List<Arc<Function>>>> {
        let mut functions: Arc<metamodelica::List<Arc<Function>>> = metamodelica::nil();
        let mut fn_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut typed: bool = false;
        let mut special: bool = false;
        let mut name: ArcStr = arcstr::literal!("");
        fn_node = InstNode::classScope(functionNode.clone());
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(InstNode::getFuncCache(fn_node.clone())?) {
            Deref @ CachedData::FUNCTION { funcs: __pa0, typed: __pa1, specialBuiltin: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        functions = __pa0.clone();
        typed = __pa1.clone();
        special = __pa2.clone();
        if !(typed.clone()) {
            functions = {
        let mut __acc: Arc<metamodelica::List<Arc<Function>>> = metamodelica::nil();
        for mut f in (functions.clone()).into_iter().cloned() {
            let __x = typeFunctionSignature(f.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            InstNode::setFuncCache(fn_node.clone(), Arc::new(CachedData::CachedData::FUNCTION { funcs: functions.clone(), typed: true, specialBuiltin: special.clone() }))?;
            functions = {
        let mut __acc: Arc<metamodelica::List<Arc<Function>>> = metamodelica::nil();
        for mut f in (functions.clone()).into_iter().cloned() {
            let __x = typeFunctionBody(f.clone(), context.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            InstNode::setFuncCache(fn_node.clone(), Arc::new(CachedData::CachedData::FUNCTION { funcs: functions.clone(), typed: true, specialBuiltin: special.clone() }))?;
        }
        Ok(functions)
    }

    pub fn getRefCache(mut fnRef: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<metamodelica::List<Arc<Function>>>> {
        let mut functions: Arc<metamodelica::List<Arc<Function>>> = metamodelica::nil();
        let mut fn_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        fn_node = InstNode::classScope(ComponentRef::node(fnRef.clone())?);
        let __pa0 = ::match_deref::match_deref! { match &(InstNode::getFuncCache(fn_node.clone())?) {
            Deref @ CachedData::FUNCTION { funcs: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        functions = __pa0.clone();
        Ok(functions)
    }

    pub fn typeFunction(mut r#fn: Arc<Function>, mut context: i32) -> Result<Arc<Function>> {
        let mut r#fn: Arc<Function> = r#fn;
        r#fn = typeFunctionSignature(r#fn.clone(), context.clone())?;
        r#fn = typeFunctionBody(r#fn.clone(), context.clone())?;
        Ok(r#fn)
    }

    pub fn typeFunctionSignature(mut r#fn: Arc<Function>, mut context: i32) -> Result<Arc<Function>> {
        let mut r#fn: Arc<Function> = r#fn;
        let mut attr: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
        let mut node: Arc<InstNode::InstNode> = r#fn.node.clone();
        let mut fn_context: i32 = 0;
        if !(isTyped(r#fn.clone())) {
            fn_context = InstContext::set(context.clone(), InstContext::FUNCTION.clone());
            assign_field!(r#fn.slots = makeSlots(r#fn.inputs.clone())?);
            Typing::typeClassType(node.clone(), Binding::EMPTY_BINDING().clone(), fn_context.clone(), node.clone())?;
            Typing::typeComponents(node.clone(), fn_context.clone(), isPartialDerivative(r#fn.clone()))?;
            if InstNode::isPartial(node.clone()) {
                ClassTree::applyComponents(Class::classTree(InstNode::getClass(node.clone())?)?, (std::sync::Arc::new(boxFunctionParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>));
            }
            checkParamTypes(r#fn.clone())?;
            checkPartialDerivativeTypes(r#fn.clone())?;
            assign_field!(r#fn.returnType = makeReturnType(r#fn.clone())?);
        }
        Ok(r#fn)
    }

    pub fn typeFunctionBody(mut r#fn: Arc<Function>, mut context: i32) -> Result<Arc<Function>> {
        let mut r#fn: Arc<Function> = r#fn;
        let mut pure: bool = false;
        let mut attr: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
        let mut fn_context: i32 = 0;
        fn_context = InstContext::set(context.clone(), InstContext::FUNCTION.clone());
        for mut c in &*r#fn.inputs.clone() {
            let mut c = c.clone();
            Typing::typeComponentBinding(c.clone(), fn_context.clone(), true)?;
        }
        for mut c in &*r#fn.outputs.clone() {
            let mut c = c.clone();
            Typing::typeComponentBinding(c.clone(), fn_context.clone(), true)?;
        }
        for mut c in &*r#fn.locals.clone() {
            let mut c = c.clone();
            Typing::typeComponentBinding(c.clone(), fn_context.clone(), true)?;
        }
        Typing::typeFunctionSections(r#fn.node.clone(), fn_context.clone())?;
        for mut fn_der in &*r#fn.derivatives.clone() {
            let mut fn_der = fn_der.clone();
            FunctionDerivative::typeDerivative(fn_der.clone())?;
        }
        Array::mapNoCopy(r#fn.inverses.clone(), (std::sync::Arc::new(FunctionInverse::typeInverse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<FunctionInverse::NFFunctionInverse>) -> Result<Arc<FunctionInverse::NFFunctionInverse>> + 'static>));
        if !(isImpure(r#fn.clone())) {
            pure = foldExp(r#fn.clone(), Arc::new({ let __pe_b1 = r#fn.clone(); move |__pe_a0, __pe_a2| checkPureCall(__pe_a0, __pe_b1.clone(), __pe_a2) }), true, true, true)?;
            if !(pure.clone()) {
                attr = r#fn.attributes.clone();
                attr.purity = DAE::Purity::IMPURE.clone();
                assign_field!(r#fn.attributes = attr.clone());
            }
        }
        if !(InstContext::inRelaxed(fn_context.clone())) {
            checkUseBeforeAssign(r#fn.clone())?;
        }
        assign_field!(r#fn.locals = sortLocals(r#fn.locals.clone(), InstNode::info(r#fn.node.clone())?)?);
        Ok(r#fn)
    }

    pub fn checkPureCall(mut exp: Arc<Expression::NFExpression>, mut r#fn: Arc<Function>, mut pure: bool) -> Result<bool> {
        let mut pure: bool = pure;
        if !(pure.clone()) {
            return Ok(pure);
        }
        if Expression::isImpureCall(exp.clone())? {
            pure = false;
            if Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())? {
                Error::addSourceMessage(Error::PURE_FUNCTION_WITH_IMPURE_CALLS.clone(), list![(AbsynUtil::pathString(name(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone(), (Expression::getName(exp.clone())?).clone()], InstNode::info(r#fn.node.clone())?)?;
            }
        }
        Ok(pure)
    }

    pub fn boxFunctionParameter(mut component: Arc<InstNode::InstNode>) -> Result<()> {
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        comp = InstNode::component(component.clone())?;
        comp = Component::setType(Type::r#box(Component::getType(comp.clone())?), comp.clone())?;
        InstNode::updateComponent(comp.clone(), component.clone())?;
        Ok(())
    }

    pub fn typePartialApplication(mut exp: Arc<Expression::NFExpression>, mut context: i32, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<Type::NFType>, Prefixes::Variability, Prefixes::Purity)> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut variability: Prefixes::Variability = Prefixes::Variability::CONSTANT;
        let mut purity: Prefixes::Purity = Prefixes::Purity::PURE;
        let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut ty_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut arg_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut rest_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut arg_name: ArcStr = arcstr::literal!("");
        let mut arg_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut arg_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut arg_var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
        let mut arg_pur: Prefixes::Purity = Prefixes::Purity::PURE;
        let mut r#fn: Arc<Function> = Arc::new(<Function as ::std::default::Default>::default());
        let mut next_context: i32 = InstContext::set(context.clone(), InstContext::SUBEXPRESSION.clone());
        let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
        let mut slots_arr: metamodelica::Array<Arc<Slot::Slot>>;
        let mut ty_arg: Arc<TypedArg> = Arc::new(<TypedArg as ::std::default::Default>::default());
        let mut mk: MatchKind = MatchKind::EXACT;
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { argNames: __pa0, args: __pa1, r#fn: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg_names = __pa0.clone();
        args = __pa1.clone();
        fn_ref = __pa2.clone();
        let __pa3 = ::match_deref::match_deref! { match &(typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: _ } => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        r#fn = __pa3.clone();
        slots_arr = metamodelica::arrayFromVec(r#fn.slots.clone().into_iter().cloned().collect());
        purity = if (isImpure(r#fn.clone())) {Purity::IMPURE.clone()} else {Purity::PURE.clone()};
        variability = Variability::CONSTANT.clone();
        for mut arg in &*args.clone() {
            let mut arg = arg.clone();
            (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg.clone(), next_context.clone(), info.clone(), false)?;
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(arg_names.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            arg_name = __pa4.clone();
            arg_names = __pa5.clone();
            ty_arg = Arc::new(TypedArg { name: Some((arg_name.clone()).clone()), value: arg.clone(), ty: arg_ty.clone(), var: arg_var.clone(), purity: arg_pur.clone() });
            let (__pa6, true) = (fillNamedArg(ty_arg.clone(), slots_arr.clone(), r#fn.clone(), info.clone())?) else { bail!("pattern mismatch") };
            slots_arr = __pa6.clone();
            variability = Prefixes::variabilityMax(variability.clone(), arg_var.clone());
            purity = Prefixes::purityMin(purity.clone(), arg_pur.clone());
        }
        exp = makePartialApplicationFromSlots(slots_arr.clone(), r#fn.clone(), fn_ref.clone(), info.clone())?;
        ty = Expression::typeOf(exp.clone());
        Ok((exp, ty, variability, purity))
    }

    pub fn makePartialApplicationFromSlots(mut slotsArray: metamodelica::Array<Arc<Slot::Slot>>, mut r#fn: Arc<Function>, mut fnRef: Arc<ComponentRef::NFComponentRef>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
        let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
        let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut arg_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut ty_arg: Arc<TypedArg> = Arc::new(<TypedArg as ::std::default::Default>::default());
        let mut arg: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut mk: MatchKind = MatchKind::EXACT;
        let mut fn_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let __range0 = slotsArray.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut slot in __range0 {
            if isSome(slot.arg.clone()) {
                let __pa1 = ::match_deref::match_deref! { match &(slot.arg.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                ty_arg = __pa1.clone();
                (arg, _, mk) = TypeCheck::matchTypes(ty_arg.ty.clone(), InstNode::getType(slot.node.clone())?, ty_arg.value.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
                if TypeCheck::isIncompatibleMatch(mk.clone()) {
                    Error::addSourceMessage(Error::NAMED_ARG_TYPE_MISMATCH.clone(), list![(AbsynUtil::pathString(name(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone(), (Util::getOption(ty_arg.name.clone())?).clone(), (Expression::toString(ty_arg.value.clone())?).clone(), (Type::toString(ty_arg.ty.clone())?).clone(), (Type::toString(InstNode::getType(slot.node.clone())?)?).clone()], info.clone())?;
                    bail!("fail");
                }
                args = cons(Expression::r#box(arg.clone()), args.clone());
                arg_names = cons((Util::getOption(ty_arg.name.clone())?).clone(), arg_names.clone());
            } else {
                inputs = cons(slot.node.clone(), inputs.clone());
                slots = cons(slot.clone(), slots.clone());
            }
        }
        assign_field!(
            r#fn.inputs = inputs.clone().reverse(),
            r#fn.slots = slots.clone().reverse()
        );
        fn_ty = Arc::new(Type::NFType::FUNCTION { r#fn: r#fn.clone(), fnType: Type::FunctionType::FUNCTIONAL_VARIABLE.clone() });
        args = args.clone().reverse();
        arg_names = arg_names.clone().reverse();
        outExp = Arc::new(Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION { r#fn: fnRef.clone(), args: args.clone(), argNames: arg_names.clone(), ty: fn_ty.clone() });
        Ok(outExp)
    }

    pub fn isBuiltin(mut r#fn: Arc<Function>) -> bool {
        let mut isBuiltin: bool = isBuiltinAttr(r#fn.attributes.clone());
        isBuiltin
    }

    pub fn isBuiltinAttr(mut attrs: DAE::FunctionAttributes) -> bool {
        let mut isBuiltin: bool = false;
        isBuiltin = (match attrs.isBuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN { .. } => false,
        _ => true,
    });
        isBuiltin
    }

    pub fn isSpecialBuiltin(mut r#fn: Arc<Function>) -> Result<bool> {
        let mut special: bool = false;
        let mut path: Arc<Absyn::Path>;
        if !(isBuiltin(r#fn.clone())) {
            special = false;
        } else {
            path = nameConsiderBuiltin(r#fn.clone())?;
            if !(AbsynUtil::pathIsIdent(path.clone())) {
                special = false;
            } else {
                special = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(path.clone())?) {
        Deref @ "array" => true,
        Deref @ "actualStream" => true,
        Deref @ "backSample" => true,
        Deref @ "branch" => true,
        Deref @ "cardinality" => true,
        Deref @ "cat" => true,
        Deref @ "change" => true,
        Deref @ "der" => true,
        Deref @ "edge" => true,
        Deref @ "fill" => true,
        Deref @ "getInstanceName" => true,
        Deref @ "initial" => true,
        Deref @ "inStream" => true,
        Deref @ "isRoot" => true,
        Deref @ "matrix" => true,
        Deref @ "max" => true,
        Deref @ "min" => true,
        Deref @ "ndims" => true,
        Deref @ "noEvent" => true,
        Deref @ "ones" => true,
        Deref @ "potentialRoot" => true,
        Deref @ "pre" => true,
        Deref @ "promote" => true,
        Deref @ "pure" => true,
        Deref @ "root" => true,
        Deref @ "rooted" => true,
        Deref @ "uniqueRoot" => true,
        Deref @ "uniqueRootIndices" => true,
        Deref @ "scalar" => true,
        Deref @ "size" => true,
        Deref @ "shiftSample" => true,
        Deref @ "smooth" => true,
        Deref @ "subSample" => true,
        Deref @ "superSample" => true,
        Deref @ "symmetric" => true,
        Deref @ "terminal" => true,
        Deref @ "transpose" => true,
        Deref @ "vector" => true,
        Deref @ "zeros" => true,
        Deref @ "sample" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
        }
        Ok(special)
    }

    pub fn isSubscriptableBuiltin(mut r#fn: Arc<Function>) -> Result<bool> {
        let mut scalarBuiltin: bool = false;
        if !(isBuiltin(r#fn.clone())) {
            scalarBuiltin = false;
        } else {
            scalarBuiltin = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(nameConsiderBuiltin(r#fn.clone())?)?) {
        Deref @ "change" => true,
        Deref @ "der" => true,
        Deref @ "pre" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(scalarBuiltin)
    }

    pub fn isImpure(mut r#fn: Arc<Function>) -> bool {
        let mut isImpure: bool = r#fn.attributes.purity.clone() == DAE::Purity::IMPURE.clone();
        isImpure
    }

    pub fn isFunctionPointer(mut r#fn: Arc<Function>) -> bool {
        let mut isPointer: bool = r#fn.attributes.isFunctionPointer.clone();
        isPointer
    }

    pub fn setFunctionPointer(mut isPointer: bool, mut r#fn: Arc<Function>) -> Arc<Function> {
        let mut r#fn: Arc<Function> = r#fn;
        let mut attr: DAE::FunctionAttributes = r#fn.attributes.clone();
        attr.isFunctionPointer = isPointer.clone();
        assign_field!(r#fn.attributes = attr.clone());
        r#fn
    }

    pub fn isExternal(mut r#fn: Arc<Function>) -> bool {
        let mut isExternal: bool = !(InstNode::isEmpty(r#fn.node.clone())) && Class::isExternalFunction(InstNode::getClass(r#fn.node.clone()).unwrap()).unwrap();
        isExternal
    }

    pub fn isExternalObjectConstructorOrDestructor(mut r#fn: Arc<Function>) -> Result<bool> {
        let mut isExternal: bool = false;
        let mut path: Arc<Absyn::Path>;
        let mut lastIdent: ArcStr = arcstr::literal!("");
        path = name(r#fn.clone());
        lastIdent = (AbsynUtil::pathLastIdent(path.clone())?).clone();
        isExternal = false;
        if lastIdent.clone() == literal!("constructor") {
            isExternal = Type::isExternalObject(r#fn.returnType.clone());
        } else if lastIdent.clone() == literal!("destructor") {
            if (r#fn.inputs.clone().len() as i32) == 1 {
                isExternal = Type::isExternalObject(Component::getType(InstNode::component(listHead(r#fn.inputs.clone())?)?)?);
            }
        }
        Ok(isExternal)
    }

    pub fn isPartialDerivative(mut r#fn: Arc<Function>) -> bool {
        let mut res: bool = !(r#fn.derivedInputs.clone().is_empty());
        res
    }

    pub fn getDerivedInputNames(mut r#fn: Arc<Function>) -> Result<Arc<metamodelica::List<ArcStr>>> {
        let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in &*r#fn.derivedInputs.clone() {
            let mut i = i.clone();
            names = cons((InstNode::name((r#fn.inputs.clone()).get(i.clone())?)?).clone(), names.clone());
        }
        names = names.clone().reverse();
        Ok(names)
    }

    pub fn getDerivedFunctionName(mut r#fn: Arc<Function>) -> Arc<Absyn::Path> {
        let mut name: Arc<Absyn::Path> = InstNode::fullPath(Class::lastBaseClass(r#fn.node.clone()), true);
        name
    }

    pub fn inlineBuiltin(mut r#fn: Arc<Function>) -> DAE::InlineType {
        let mut inlineType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
        inlineType = (match r#fn.attributes.isBuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR { .. } => openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE,
        _ => r#fn.attributes.inline.clone(),
    });
        inlineType
    }

    pub fn isDefaultRecordConstructor(mut r#fn: Arc<Function>) -> bool {
        let mut isConstructor: bool = Restriction::isRecordConstructor(InstNode::restriction(r#fn.node.clone()));
        isConstructor
    }

    pub fn isNonDefaultRecordConstructor(mut r#fn: Arc<Function>) -> bool {
        // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
        // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
        pub fn isNonDefaultRecordConstructorPath(mut path: Arc<Absyn::Path>) -> bool {
            let mut b: bool = false;
            b = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::QUALIFIED { name: Deref @ "'constructor'", .. } => true,
        Deref @ Absyn::Path::QUALIFIED { .. } => isNonDefaultRecordConstructorPath(var_field!((*path).path, Absyn::Path::QUALIFIED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            b
        }

        let mut b: bool = isNonDefaultRecordConstructorPath(r#fn.path.clone());
        b
    }

    pub fn toDAE(mut r#fn: Arc<Function>, mut def: DAE::FunctionDefinition) -> Result<DAE::Function> {
        let mut daeFn: DAE::Function;
        let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
        let mut par: bool = false;
        let mut impr: bool = false;
        let mut ity: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
        let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
        let mut defs: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
        let mut unused_inputs: Arc<metamodelica::List<i32>> = metamodelica::nil();
        vis = openmodelica_frontend_types::SCode::Visibility::PUBLIC;
        par = false;
        impr = r#fn.attributes.purity.clone() == DAE::Purity::IMPURE.clone();
        ity = r#fn.attributes.inline.clone();
        ty = makeDAEType(r#fn.clone(), false)?;
        unused_inputs = analyseUnusedParameters(r#fn.clone())?;
        defs = {
        let mut __acc: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
        for mut fn_inv in (r#fn.inverses.clone()).borrow().iter() {
            let __x = FunctionInverse::toDAE(fn_inv.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        defs = listAppend({
        let mut __acc: Arc<metamodelica::List<DAE::FunctionDefinition>> = metamodelica::nil();
        for mut fn_der in (r#fn.derivatives.clone()).into_iter().cloned() {
            let __x = FunctionDerivative::toDAE(fn_der.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, defs.clone());
        defs = cons(def.clone(), defs.clone());
        daeFn = DAE::Function::FUNCTION { path: r#fn.path.clone(), functions: defs.clone(), type_: ty.clone(), visibility: vis.clone(), partialPrefix: par.clone(), isImpure: impr.clone(), inlineType: ity.clone(), unusedInputs: unused_inputs.clone(), source: ElementSource::createElementSource(InstNode::info(r#fn.node.clone())?, None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?, comment: SCodeUtil::getElementComment(InstNode::definition(r#fn.node.clone())?) };
        Ok(daeFn)
    }

    pub fn makeDAEType(mut r#fn: Arc<Function>, mut boxTypes: bool) -> Result<Arc<DAE::Type>> {
        let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
        let mut params: Arc<metamodelica::List<Arc<DAE::FuncArg>>> = metamodelica::nil();
        let mut pname: ArcStr = arcstr::literal!("");
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut ptype: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
        let mut pconst: DAE::Const = DAE::Const::C_CONST;
        let mut ppar: DAE::VarParallelism = DAE::VarParallelism::NON_PARALLEL;
        let mut pdefault: Option<Arc<DAE::Exp>> = None;
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        for mut param in &*r#fn.inputs.clone() {
            let mut param = param.clone();
            comp = InstNode::component(param.clone())?;
            pname = (InstNode::name(param.clone())?).clone();
            ty = Component::getType(comp.clone())?;
            ptype = Type::toDAE(if (boxTypes.clone()) {Type::r#box(ty.clone())} else {ty.clone()}, true)?;
            pconst = Prefixes::variabilityToDAEConst(Component::variability(comp.clone()));
            ppar = Prefixes::parallelismToDAE(Component::parallelism(comp.clone()))?;
            pdefault = Util::applyOption(Binding::typedExp(Component::getBinding(comp.clone())), Arc::new({ let __pe_b1 = false; move |__pe_a0| Expression::toDAE(__pe_a0, __pe_b1.clone()) }));
            params = cons(Arc::new(DAE::FuncArg { name: (pname.clone()).clone(), ty: ptype.clone(), r#const: pconst.clone(), par: ppar.clone(), defaultBinding: pdefault.clone() }), params.clone());
        }
        params = params.clone().reverse();
        ty = if (isDefaultRecordConstructor(r#fn.clone())) {InstNode::getType(r#fn.node.clone())?} else {r#fn.returnType.clone()};
        ty = if (boxTypes.clone()) {Type::r#box(ty.clone())} else {ty.clone()};
        outType = Arc::new(DAE::Type::T_FUNCTION { funcArg: params.clone(), funcResultType: Type::toDAE(ty.clone(), true)?, functionAttributes: r#fn.attributes.clone(), path: r#fn.path.clone() });
        Ok(outType)
    }

    pub fn getSingleBodyExp(mut r#fn: Arc<Function>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        body = getBody(r#fn.clone());
        exp = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ metamodelica::List::Cons { head: stmt @ Deref @ Statement::ASSIGNMENT { .. }, tail: Deref @ metamodelica::List::Nil } => {
            var_field!((**stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getSingleBodyExp")); __mm_s.push_str(&*literal!(" failed because the body of the function is not a single assignment:\n")); __mm_s.push_str(&*List::toString(body.clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Statement::toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    pub fn getBody(mut r#fn: Arc<Function>) -> Arc<metamodelica::List<Arc<Statement::NFStatement>>> {
        let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = getBody2(r#fn.node.clone()).unwrap();
        body
    }

    pub fn hasUnboxArgs(mut r#fn: Arc<Function>) -> bool {
        let mut res: bool = false;
        res = (match r#fn.attributes.clone() {
        DAE::FunctionAttributes { isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { unboxArgs: mut res, .. }, .. } => res.clone(),
        _ => false,
    });
        res
    }

    pub fn hasUnboxArgsAnnotation(mut cmt: Arc<SCode::Comment>) -> bool {
        let mut res: bool = SCodeUtil::commentHasBooleanNamedAnnotation(cmt.clone(), (literal!("__OpenModelica_UnboxArguments")).clone()).unwrap();
        res
    }

    pub fn hasOptionalArgument(mut component: Arc<SCode::Element>) -> bool {
        let mut res: bool = SCodeUtil::hasBooleanNamedAnnotationInComponent(component.clone(), (literal!("__OpenModelica_optionalArgument")).clone()).unwrap();
        res
    }

    pub fn mapExp(mut r#fn: Arc<Function>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut mapFnFields: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut mapParameters: bool, mut mapBody: bool) -> Result<Arc<Function>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

        let mut r#fn: Arc<Function> = r#fn;
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        let mut ctree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        let mut binding2: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        cls = InstNode::getClass(r#fn.node.clone())?;
        if mapParameters.clone() {
            ctree = Class::classTree(cls.clone())?;
            ClassTree::applyComponents(ctree.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = mapFn.clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = mapFnFields.clone(); move |__pe_a0| mapExpParameter(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            assign_field!(r#fn.returnType = makeReturnType(r#fn.clone())?);
        }
        if mapBody.clone() {
            sections = Sections::mapExp(Class::getSections(cls.clone())?, mapFn.clone());
            cls = Class::setSections(sections.clone(), cls.clone())?;
            InstNode::updateClass(cls.clone(), r#fn.node.clone())?;
        }
        Ok(r#fn)
    }

    pub fn mapExpParameter(mut node: Arc<InstNode::InstNode>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut mapFnFields: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<()> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        let mut binding2: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut dirty: bool = false;
        comp = InstNode::component(node.clone())?;
        binding = Component::getBinding(comp.clone());
        binding2 = Binding::mapExpShallow(binding.clone(), mapFn.clone());
        if !(referenceEq(&binding.clone(),&binding2.clone())) {
            comp = Component::setBinding(binding2.clone(), comp.clone())?;
            dirty = true;
        }
        let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT { .. } => {
            ty = Type::mapDims(var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = mapFn.clone(); move |__pe_a0| Dimension::mapExp(__pe_a0, __pe_b1.clone()) }));
            if !(referenceEq(&ty.clone(),&var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone())) {
                assign_variant_field!(comp => Component::NFComponent::COMPONENT; ty = ty.clone());
                dirty = true;
            }
            cls = InstNode::getClass(var_field!((*comp).classInst, Component::NFComponent::COMPONENT).clone())?;
            ClassTree::applyComponents(Class::classTree(cls.clone())?, Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = mapFnFields.clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = mapFnFields.clone(); move |__pe_a0| mapExpParameter(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if dirty.clone() {
            InstNode::updateComponent(comp.clone(), node.clone())?;
        }
        Ok(())
    }

    pub fn mapBody(mut r#fn: Arc<Function>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>) -> Result<Arc<Function>> {
        pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>;

        let mut r#fn: Arc<Function> = r#fn;
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
        cls = InstNode::getClass(r#fn.node.clone())?;
        sections = Sections::map(Class::getSections(cls.clone())?, (std::sync::Arc::new(fnptr!(Sections::eqId, Arc<NFEquation::NFEquation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFEquation::NFEquation>) -> Result<Arc<NFEquation::NFEquation>> + 'static>), mapFn.clone(), (std::sync::Arc::new(fnptr!(Sections::eqId, Arc<NFEquation::NFEquation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFEquation::NFEquation>) -> Result<Arc<NFEquation::NFEquation>> + 'static>), mapFn.clone());
        cls = Class::setSections(sections.clone(), cls.clone())?;
        InstNode::updateClass(cls.clone(), r#fn.node.clone())?;
        Ok(r#fn)
    }

    pub fn foldExp<ArgT: Clone + 'static>(mut r#fn: Arc<Function>, mut foldFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT, mut mapParameters: bool, mut mapBody: bool) -> Result<ArgT> {
        pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

        let mut arg: ArgT = arg;
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        cls = InstNode::getClass(r#fn.node.clone())?;
        if mapParameters.clone() {
            arg = ClassTree::foldComponents(Class::classTree(cls.clone())?, Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, _) -> Result<_> + 'static> = foldFn.clone(); move |__pe_a0, __pe_a2| foldExpParameter(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
        }
        if mapBody.clone() {
            arg = Sections::foldExp(Class::getSections(cls.clone())?, foldFn.clone(), arg.clone())?;
        }
        Ok(arg)
    }

    pub fn foldExpParameter<ArgT: Clone + 'static>(mut node: Arc<InstNode::InstNode>, mut foldFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
        pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

        let mut arg: ArgT = arg;
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        comp = InstNode::component(node.clone())?;
        arg = Binding::foldExp(Component::getBinding(comp.clone()), foldFn.clone(), arg.clone())?;
        let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT { .. } => {
            arg = Type::foldDims(var_field!((*comp).ty, Component::NFComponent::COMPONENT).clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, _) -> Result<_> + 'static> = foldFn.clone(); move |__pe_a0, __pe_a2| Dimension::foldExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
            cls = InstNode::getClass(var_field!((*comp).classInst, Component::NFComponent::COMPONENT).clone())?;
            arg = ClassTree::foldComponents(Class::classTree(cls.clone())?, Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, _) -> Result<_> + 'static> = foldFn.clone(); move |__pe_a0, __pe_a2| foldExpParameter(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(arg)
    }

    pub fn isPartial(mut r#fn: Arc<Function>) -> bool {
        let mut isPartial: bool = InstNode::isPartial(r#fn.node.clone());
        isPartial
    }

    pub fn getLocalArguments(mut r#fn: Arc<Function>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
        let mut localArgs: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        for mut l in &*r#fn.locals.clone() {
            let mut l = l.clone();
            if InstNode::isComponent(l.clone()) {
                binding = Component::getBinding(InstNode::component(l.clone())?);
                Error::assertion(Binding::hasExp(binding.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getLocalArguments")); __mm_s.push_str(&*literal!(" got local component without binding")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                localArgs = cons(Binding::getExp(binding.clone())?, localArgs.clone());
            }
        }
        localArgs = localArgs.clone().reverse();
        Ok(localArgs)
    }

    fn collectParams(mut node: Arc<InstNode::InstNode>, mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<(Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>)> {
        let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = inputs;
        let mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = outputs;
        let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = locals;
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut n: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut check_vis: bool = false;
        Error::assertion(InstNode::isClass(node.clone()) || InstNode::isComponent(node.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.collectParams")); __mm_s.push_str(&*literal!(" got non-class/non-component node")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        cls = InstNode::getClass(node.clone())?;
        let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: comps, .. }, .. } => {
            let __range0 = (1..=(comps.clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                n = comps.borrow()[(i.clone()-1) as usize].clone();
                check_vis = !(Flags::getConfigBool(Flags::BASE_MODELICA.clone())?);
                let () = (match paramDirection(n.clone(), check_vis.clone())? {
        Prefixes::Direction::INPUT => {
            inputs = cons(n.clone(), inputs.clone());
            ()
        },
        Prefixes::Direction::OUTPUT => {
            outputs = cons(n.clone(), outputs.clone());
            ()
        },
        Prefixes::Direction::NONE => {
            locals = cons(n.clone(), locals.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
            }
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { .. } => {
            (inputs, outputs, locals) = collectParams(var_field!((*cls).baseClass, Class::NFClass::EXPANDED_DERIVED).clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.collectParams")); __mm_s.push_str(&*literal!(" got non-instantiated function ")); __mm_s.push_str(&*AbsynUtil::pathString(InstNode::scopePath(node.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((inputs, outputs, locals))
    }

    fn paramDirection(mut component: Arc<InstNode::InstNode>, mut checkVisibility: bool) -> Result<Prefixes::Direction> {
        let mut direction: Prefixes::Direction = Prefixes::Direction::NONE;
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut cty: i32 = 0;
        let mut io: Prefixes::InnerOuter = Prefixes::InnerOuter::NOT_INNER_OUTER;
        let mut vis: Prefixes::Visibility = Prefixes::Visibility::PUBLIC;
        let mut var: Prefixes::Variability = Prefixes::Variability::CONSTANT;
        comp = InstNode::component(InstNode::resolveOuter(component.clone()))?;
        io = Component::innerOuter(comp.clone())?;
        if io.clone() != InnerOuter::NOT_INNER_OUTER.clone() {
            Error::addSourceMessage(Error::INNER_OUTER_FORMAL_PARAMETER.clone(), list![(Prefixes::innerOuterString(io.clone())).clone(), (InstNode::name(component.clone())?).clone()], InstNode::info(InstNode::resolveOuter(component.clone()))?)?;
            bail!("fail");
        }
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Component::getAttributes(comp.clone())) {
            Deref @ Attributes::ATTRIBUTES { variability: __pa0, direction: __pa1, connectorType: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        var = __pa0.clone();
        direction = __pa1.clone();
        cty = __pa2.clone();
        if Prefixes::ConnectorType::isFlowOrStream(cty.clone()) {
            Error::addSourceMessage(Error::INNER_OUTER_FORMAL_PARAMETER.clone(), list![(Prefixes::ConnectorType::toString(cty.clone())).clone(), (InstNode::name(component.clone())?).clone()], InstNode::info(component.clone())?)?;
            bail!("fail");
        }
        if checkVisibility.clone() {
            vis = InstNode::visibility(component.clone());
            if direction.clone() != Direction::NONE.clone() {
                if vis.clone() == Visibility::PROTECTED.clone() {
                    Error::addSourceMessage(Error::PROTECTED_FORMAL_FUNCTION_VAR.clone(), list![(InstNode::name(component.clone())?).clone()], InstNode::info(component.clone())?)?;
                    bail!("fail");
                }
            } else if vis.clone() == Visibility::PUBLIC.clone() {
                Error::addSourceMessageAsError(Error::NON_FORMAL_PUBLIC_FUNCTION_VAR.clone(), list![(InstNode::name(component.clone())?).clone()], InstNode::info(component.clone())?)?;
                bail!("fail");
            }
        }
        Ok(direction)
    }

    fn makeSlots(mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<Slot::Slot>>>> {
        let mut slots: Arc<metamodelica::List<Arc<Slot::Slot>>> = metamodelica::nil();
        let mut index: i32 = 1;
        for mut i in &*inputs.clone() {
            let mut i = i.clone();
            slots = cons(makeSlot(i.clone(), index.clone())?, slots.clone());
            index = index.clone() + 1;
        }
        slots = slots.clone().reverse();
        Ok(slots)
    }

    fn makeSlot(mut component: Arc<InstNode::InstNode>, mut index: i32) -> Result<Arc<Slot::Slot>> {
        let mut slot: Arc<Slot::Slot> = Arc::new(<Slot::Slot as ::std::default::Default>::default());
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut default: Option<Arc<Expression::NFExpression>> = None;
        let mut name: ArcStr = arcstr::literal!("");
        match '__try0: {
            comp = unwrap_break_err!(InstNode::component(component.clone()), '__try0);
            default = Binding::getExpOpt(Component::getImplicitBinding(comp.clone(), InstNode::instanceParent(component.clone())));
            name = (unwrap_break_err!(InstNode::name(component.clone()), '__try0)).clone();
            if StringUtil::startsWith((name.clone()).clone(), (literal!("$in_")).clone()) {
                name = unwrap_break_err!(substring((name.clone()).clone(), 5, ((name.clone()).clone().len() as i32)), '__try0);
            }
            slot = Arc::new(Slot::Slot { node: component.clone(), ty: SlotType::GENERIC.clone(), default: default.clone(), arg: None, index: index.clone(), evalStatus: SlotEvalStatus::NOT_EVALUATED.clone() });
            Ok::<_, anyhow::Error>((comp.clone(), default.clone(), name.clone(), slot.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
                comp = __try0_o0;
                default = __try0_o1;
                name = __try0_o2;
                slot = __try0_o3;
            }
            Err(_) => {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.makeSlot")); __mm_s.push_str(&*literal!(" got invalid component")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                bail!("try/else: outputs not set in else branch");
            }
        }
        Ok(slot)
    }

    fn hasOMPure(mut cmt: Arc<SCode::Comment>) -> bool {
        let mut res: bool = !(SCodeUtil::commentHasBooleanNamedAnnotation(cmt.clone(), (literal!("__OpenModelica_Impure")).clone()).unwrap());
        res
    }

    fn getBuiltinPtr(mut cmt: Arc<SCode::Comment>) -> DAE::FunctionBuiltin {
        let mut builtin: DAE::FunctionBuiltin = if (SCodeUtil::commentHasBooleanNamedAnnotation(cmt.clone(), (literal!("__OpenModelica_BuiltinPtr")).clone()).unwrap()) {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR} else {openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN};
        builtin
    }

    fn mergeFunctionAnnotations(mut comments: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Result<Arc<SCode::Comment>> {
        let mut outComment: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
        let mut comment: Option<ArcStr> = None;
        let mut r#mod: Arc<SCode::Mod> = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD);
        let mut mod2: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        for mut cmt in &*comments.clone() {
            let mut cmt = cmt.clone();
            if isNone(comment.clone()) {
                comment = cmt.comment.clone();
            }
            r#mod = (::match_deref::match_deref! { match &(cmt.clone()) {
        Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: mod2 }), .. } => SCodeUtil::mergeModifiers(mod2.clone(), r#mod.clone())?,
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        outComment = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::NOMOD => Arc::new(SCode::Comment { annotation_: None, comment: comment.clone() }),
        _ => Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: r#mod.clone() })), comment: comment.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outComment)
    }

    fn makeAttributes(mut node: Arc<InstNode::InstNode>, mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut outputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut comments: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Result<DAE::FunctionAttributes> {
        let mut attr: DAE::FunctionAttributes = <DAE::FunctionAttributes as ::std::default::Default>::default();
        let mut def: Arc<SCode::Element>;
        let mut params: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut res: SCode::Restriction = SCode::Restriction::R_BLOCK;
        let mut fres: SCode::FunctionRestriction = SCode::FunctionRestriction::FR_KERNEL_FUNCTION;
        let mut is_partial: bool = false;
        let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
        let mut purity: DAE::Purity = DAE::Purity::PURE;
        def = InstNode::classDefinition(Class::lastBaseClass(node.clone()))?;
        res = SCodeUtil::getClassRestriction(def.clone())?;
        Error::assertion(SCodeUtil::isFunctionRestriction(res.clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.makeAttributes")); __mm_s.push_str(&*literal!(" got non-function restriction")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        let SCode::Restriction::R_FUNCTION { functionRestriction: __pa0 } = (res.clone()) else { bail!("pattern mismatch") };
        fres = __pa0.clone();
        is_partial = InstNode::isPartial(node.clone());
        cmt = mergeFunctionAnnotations(comments.clone())?;
        purity = InstBasics::getFunctionRestrictionPurity(SCodeUtil::getFunctionRestrictionPurity(fres.clone()), cmt.clone(), true)?;
        attr = 'mc: {
        let __mc_input = fres.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut has_unbox_args: bool = false;
            let mut name: ArcStr = arcstr::literal!("");
            let mut in_params: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut out_params: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut inline_ty: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
            let mut generateEvents: bool = false;
            in_params = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (inputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(i.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            out_params = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut o in (outputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(o.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            name = (SCodeUtil::isBuiltinFunction(def.clone(), in_params.clone(), out_params.clone())?).clone();
            inline_ty = InstBasics::commentIsInlineFunc(cmt.clone())?;
            generateEvents = InstBasics::commentGenerateEvents(cmt.clone());
            has_unbox_args = hasUnboxArgsAnnotation(cmt.clone());
            Ok(DAE::FunctionAttributes { inline: inline_ty.clone(), generateEvents: generateEvents.clone(), purity: purity.clone(), isFunctionPointer: is_partial.clone(), isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some((name.clone()).clone()), unboxArgs: has_unbox_args.clone() }, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut has_unbox_args: bool = false;
            let mut name: ArcStr = arcstr::literal!("");
            let mut in_params: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut out_params: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut inline_ty: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
            let mut generateEvents: bool = false;
            in_params = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut i in (inputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(i.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            out_params = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut o in (outputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(o.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            name = (SCodeUtil::isBuiltinFunction(def.clone(), in_params.clone(), out_params.clone())?).clone();
            inline_ty = InstBasics::commentIsInlineFunc(cmt.clone())?;
            generateEvents = InstBasics::commentGenerateEvents(cmt.clone());
            has_unbox_args = hasUnboxArgsAnnotation(cmt.clone());
            Ok(DAE::FunctionAttributes { inline: inline_ty.clone(), generateEvents: generateEvents.clone(), purity: purity.clone(), isFunctionPointer: is_partial.clone(), isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some((name.clone()).clone()), unboxArgs: has_unbox_args.clone() }, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut inline_ty: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
            let mut generateEvents: bool = false;
            inline_ty = InstBasics::commentIsInlineFunc(cmt.clone())?;
            generateEvents = InstBasics::commentGenerateEvents(cmt.clone());
            Ok(DAE::FunctionAttributes { inline: inline_ty.clone(), generateEvents: generateEvents.clone(), purity: purity.clone(), isFunctionPointer: is_partial.clone(), isBuiltin: getBuiltinPtr(cmt.clone()), functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_PARALLEL_FUNCTION })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. } = __mc_input.clone() else { bail!("nomatch") };
            Ok(DAE::FunctionAttributes { inline: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, generateEvents: false, purity: purity.clone(), isFunctionPointer: is_partial.clone(), isBuiltin: openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_KERNEL_FUNCTION })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut inline_ty: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
            let mut generateEvents: bool = false;
            let mut purity: DAE::Purity = purity.clone();
            inline_ty = InstBasics::commentIsInlineFunc(cmt.clone())?;
            generateEvents = InstBasics::commentGenerateEvents(cmt.clone());
            if purity.clone() == DAE::Purity::UNDEFINED.clone() && Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())? {
                purity = if (SCodeUtil::isExternalFunctionRestriction(fres.clone())) {DAE::Purity::IMPURE.clone()} else {DAE::Purity::PURE.clone()};
            }
            if SCodeUtil::hasNamedExternalCall((literal!("ModelicaError")).clone(), SCodeUtil::getClassDef(def.clone())?) {
                purity = DAE::Purity::PURE.clone();
            }
            Ok(DAE::FunctionAttributes { inline: inline_ty.clone(), generateEvents: generateEvents.clone(), purity: purity.clone(), isFunctionPointer: is_partial.clone(), isBuiltin: getBuiltinPtr(cmt.clone()), functionParallelism: openmodelica_frontend_types::DAE::FunctionParallelism::FP_NON_PARALLEL })
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        Ok(attr)
    }

    fn checkParamTypes(mut r#fn: Arc<Function>) -> Result<()> {
        checkParamTypes2(r#fn.inputs.clone())?;
        checkParamTypes2(r#fn.outputs.clone())?;
        checkParamTypes2(r#fn.locals.clone())?;
        Ok(())
    }

    fn checkParamTypes2(mut params: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<()> {
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        for mut p in &*params.clone() {
            let mut p = p.clone();
            ty = InstNode::getType(p.clone())?;
            if !(isValidParamType(ty.clone())?) {
                Error::addSourceMessage(Error::INVALID_FUNCTION_VAR_TYPE.clone(), list![(Type::toString(ty.clone())?).clone(), (InstNode::name(p.clone())?).clone()], InstNode::info(p.clone())?)?;
                bail!("fail");
            }
        }
        Ok(())
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    fn isValidParamType(mut ty: Arc<Type::NFType>) -> Result<bool> {
        let mut isValid: bool = false;
        isValid = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::INTEGER => true,
        Deref @ Type::REAL => true,
        Deref @ Type::STRING => true,
        Deref @ Type::BOOLEAN => true,
        Deref @ Type::CLOCK => true,
        Deref @ Type::ENUMERATION { .. } => true,
        Deref @ Type::POLYMORPHIC { .. } => true,
        Deref @ Type::ARRAY { .. } => isValidParamType(var_field!((*ty).elementType, Type::NFType::ARRAY).clone())?,
        Deref @ Type::COMPLEX { .. } => isValidParamState(var_field!((*ty).cls, Type::NFType::COMPLEX).clone())?,
        Deref @ Type::FUNCTION { .. } => true,
        Deref @ Type::METABOXED { .. } => isValidParamType(var_field!((*ty).ty, Type::NFType::METABOXED).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isValid)
    }

    fn isValidParamState(mut cls: Arc<InstNode::InstNode>) -> Result<bool> {
        let mut isValid: bool = false;
        isValid = (::match_deref::match_deref! { match &(Class::restriction(InstNode::getClass(cls.clone())?)) {
        Deref @ Restriction::RECORD { .. } => true,
        Deref @ Restriction::TYPE => true,
        Deref @ Restriction::OPERATOR => true,
        Deref @ Restriction::FUNCTION => true,
        Deref @ Restriction::EXTERNAL_OBJECT => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isValid)
    }

    fn checkPartialDerivativeTypes(mut r#fn: Arc<Function>) -> Result<()> {
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        for mut i in &*r#fn.derivedInputs.clone() {
            let mut i = i.clone();
            node = (r#fn.inputs.clone()).get(i.clone())?;
            ty = InstNode::getType(node.clone())?;
            if !(Type::isReal(ty.clone()) && Type::isScalar(ty.clone())) {
                Error::addSourceMessage(Error::PARTIAL_DERIVATIVE_INPUT_INVALID_TYPE.clone(), list![(InstNode::name(node.clone())?).clone(), (AbsynUtil::pathString(getDerivedFunctionName(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone()], InstNode::info(r#fn.node.clone())?)?;
                bail!("fail");
            }
        }
        Ok(())
    }

    pub fn makeReturnType(mut r#fn: Arc<Function>) -> Result<Arc<Type::NFType>> {
        let mut returnType: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut ret_tyl: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        ret_tyl = {
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut o in (r#fn.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::getType(o.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        returnType = (::match_deref::match_deref! { match &(ret_tyl.clone()) {
        Deref @ metamodelica::List::Nil => Arc::new(crate::NFType::NORETCALL),
        Deref @ metamodelica::List::Cons { head: returnType, tail: Deref @ metamodelica::List::Nil } => returnType.clone(),
        _ => Arc::new(Type::NFType::TUPLE { types: ret_tyl.clone(), names: None }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(returnType)
    }

    pub fn getBody2(mut node: Arc<InstNode::InstNode>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
        let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        let mut fn_body: Arc<Algorithm::NFAlgorithm>;
        body = (::match_deref::match_deref! { match &(InstNode::getSections(node.clone())?) {
        Deref @ Sections::SECTIONS { algorithms: Deref @ metamodelica::List::Nil, .. } => metamodelica::nil(),
        Deref @ Sections::SECTIONS { algorithms: Deref @ metamodelica::List::Cons { head: fn_body, tail: Deref @ metamodelica::List::Nil }, .. } => fn_body.statements.clone(),
        Deref @ Sections::EMPTY => metamodelica::nil(),
        Deref @ Sections::EXTERNAL { .. } => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getBody2")); __mm_s.push_str(&*literal!(" got function with external section (not algorithm section)")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        Deref @ Sections::SECTIONS { .. } => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getBody2")); __mm_s.push_str(&*literal!(" got function with multiple algorithm sections")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunction.Function.getBody2")); __mm_s.push_str(&*literal!(" got unknown sections")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(body)
    }

    pub fn hasSingleOrEmptyBody(mut r#fn: Arc<Function>) -> bool {
        let mut b: bool = false;
        let mut algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        if '__try0: {
            if isBuiltin(r#fn.clone()) {
                return b;
            }
            b = (::match_deref::match_deref! { match &(unwrap_break_err!(InstNode::getSections(r#fn.node.clone()), '__try0)) {
        Deref @ Sections::SECTIONS { algorithms, .. } => (algorithms.clone().len() as i32) < 2,
        Deref @ Sections::EMPTY => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
        b
    }

    pub fn analyseUnusedParameters(mut r#fn: Arc<Function>) -> Result<Arc<metamodelica::List<i32>>> {
        let mut unusedInputs: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut index: i32 = 0;
        inputs = foldExp(r#fn.clone(), (std::sync::Arc::new(analyseUnusedParametersExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> + 'static>), r#fn.inputs.clone(), true, true)?;
        for mut i in &*inputs.clone() {
            let mut i = i.clone();
            index = List::positionOnTrue(r#fn.inputs.clone(), Arc::new({ let __pe_b0 = i.clone(); move |__pe_a1| Ok(InstNode::refEqual(__pe_b0.clone(), __pe_a1)) }));
            unusedInputs = cons(index.clone(), unusedInputs.clone());
        }
        Ok(unusedInputs)
    }

    pub fn analyseUnusedParametersExp(mut exp: Arc<Expression::NFExpression>, mut params: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> {
        let mut params: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = params;
        if !(params.clone().is_empty()) {
            params = Expression::fold(exp.clone(), (std::sync::Arc::new(analyseUnusedParametersExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> + 'static>), params.clone())?;
        }
        Ok(params)
    }

    pub fn analyseUnusedParametersExp2(mut exp: Arc<Expression::NFExpression>, mut params: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> {
        let mut params: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = params;
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            (params, _) = List::deleteMemberOnTrue(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), params.clone(), (std::sync::Arc::new(fnptr!(ComponentRef::containsNode, Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>))?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(params)
    }

    pub fn sortLocals(mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut info: SourceInfo) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> {
        let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = locals;
        let mut locals_set: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>;
        let mut dep_graph: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>)>> = metamodelica::nil();
        let mut cycles: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>)>> = metamodelica::nil();
        let mut cycles_str: ArcStr = arcstr::literal!("");
        locals_set = UnorderedSet::fromList(locals.clone(), (std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>))?;
        dep_graph = Graph::buildGraph(locals.clone(), (std::sync::Arc::new(getLocalDependencies) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> + 'static>), locals_set.clone());
        (locals, cycles) = Graph::topologicalSort(dep_graph.clone(), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>))?;
        if !(cycles.clone().is_empty()) {
            cycles_str = stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut cycle in (Graph::findCycles(cycles.clone(), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>))?).into_iter().cloned() {
            let __x = List::toString(cycle.clone(), (std::sync::Arc::new(InstNode::name) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone());
            Error::addSourceMessage(Error::CYCLIC_FUNCTION_COMPONENTS.clone(), list![(cycles_str.clone()).clone()], info.clone())?;
            bail!("fail");
        }
        Ok(locals)
    }

    pub fn getLocalDependencies(mut node: Arc<InstNode::InstNode>, mut locals: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> {
        let mut dependencies: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut deps: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>;
        deps = UnorderedSet::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 1);
        deps = getLocalDependencies2(node.clone(), locals.clone(), deps.clone())?;
        UnorderedSet::remove(node.clone(), deps.clone())?;
        deps = Type::foldDims(InstNode::getType(node.clone())?, Arc::new({ let __pe_b1 = locals.clone(); move |__pe_a0, __pe_a2| getLocalDependenciesDim(__pe_a0, __pe_b1.clone(), __pe_a2) }), deps.clone());
        dependencies = UnorderedSet::toList(deps.clone());
        Ok(dependencies)
    }

    pub fn getLocalDependencies2(mut node: Arc<InstNode::InstNode>, mut locals: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>, mut dependencies: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>> {
        let mut dependencies: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>> = dependencies;
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
        comp = InstNode::component(node.clone())?;
        binding = Component::getBinding(comp.clone());
        if Binding::hasExp(binding.clone()) {
            dependencies = getLocalDependenciesExp(Binding::getExp(binding.clone())?, locals.clone(), dependencies.clone())?;
        } else if Type::isRecord(Component::getType(comp.clone())?) {
            dependencies = ClassTree::foldComponents(Class::classTree(InstNode::getClass(node.clone())?)?, Arc::new({ let __pe_b1 = locals.clone(); move |__pe_a0, __pe_a2| getLocalDependencies2(__pe_a0, __pe_b1.clone(), __pe_a2) }), dependencies.clone());
        }
        Ok(dependencies)
    }

    pub fn getLocalDependenciesExp(mut exp: Arc<Expression::NFExpression>, mut locals: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>, mut deps: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>> {
        let mut deps: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>> = deps;
        deps = Expression::fold(exp.clone(), Arc::new({ let __pe_b1 = locals.clone(); move |__pe_a0, __pe_a2| getLocalDependenciesExp2(__pe_a0, __pe_b1.clone(), __pe_a2) }), deps.clone())?;
        Ok(deps)
    }

    pub fn getLocalDependenciesExp2(mut exp: Arc<Expression::NFExpression>, mut locals: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>, mut deps: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>> {
        let mut deps: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>> = deps;
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut cr_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            cr = ComponentRef::last(var_field!((*exp).cref, Expression::NFExpression::CREF).clone());
            if ComponentRef::isCref(cr.clone()) {
                cr_node = ComponentRef::node(cr.clone())?;
                if UnorderedSet::contains(cr_node.clone(), locals.clone())? {
                    UnorderedSet::add(cr_node.clone(), deps.clone())?;
                }
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(deps)
    }

    pub fn getLocalDependenciesDim(mut dim: Arc<Dimension::NFDimension>, mut locals: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>, mut deps: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>> {
        let mut deps: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>> = deps;
        deps = Dimension::foldExp(dim.clone(), Arc::new({ let __pe_b1 = locals.clone(); move |__pe_a0, __pe_a2| getLocalDependenciesExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), deps.clone())?;
        Ok(deps)
    }

    pub fn getDerivative(mut original: Arc<Function>, mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>) -> Result<Option<Arc<Function>>> {
        let mut derivative: Option<Arc<Function>> = None;
        let mut derivatives: Arc<metamodelica::List<Arc<FunctionDerivative::NFFunctionDerivative>>> = metamodelica::nil();
        let mut perfect_fit: bool = false;
        for mut func in &*original.derivatives.clone() {
            let mut func = func.clone();
            if FunctionDerivative::perfectFit(func.clone(), interface_map.clone())? {
                derivative = Some(listHead(getCachedFuncs(func.derivativeFn.clone())?)?);
                return Ok(derivative);
            }
        }
        for mut key in &*UnorderedMap::keyList(interface_map.clone()) {
            let mut key = key.clone();
            UnorderedMap::add((key.clone()).clone(), true, interface_map.clone())?;
        }
        Ok(derivative)
    }

    pub fn checkUseBeforeAssign(mut r#fn: Arc<Function>) -> Result<()> {
        let mut unassigned: Arc<Vector::Vector<Arc<InstNode::InstNode>>>;
        let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        let mut parent: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut sources: Arc<metamodelica::List<SourceInfo>> = metamodelica::nil();
        if isExternal(r#fn.clone()) || isBuiltin(r#fn.clone()) {
            return Ok(());
        }
        unassigned = Vector::new(0);
        addUnassignedComponents(unassigned.clone(), r#fn.outputs.clone())?;
        addUnassignedComponents(unassigned.clone(), r#fn.locals.clone())?;
        body = getBody(r#fn.clone());
        checkUseBeforeAssign2(unassigned.clone(), body.clone())?;
        for mut var in &*Vector::toList(unassigned.clone()) {
            let mut var = var.clone();
            if InstNode::isOutput(var.clone()) {
                parent = InstNode::parent(var.clone());
                sources = list![InstNode::info(var.clone())?];
                if InstNode::isBaseClass(parent.clone()) {
                    sources = cons(InstNode::info(InstNode::getDerivedNode(parent.clone(), true))?, sources.clone());
                }
                Error::addMultiSourceMessage(Error::UNASSIGNED_FUNCTION_OUTPUT.clone(), list![(InstNode::name(var.clone())?).clone()], sources.clone())?;
            }
        }
        Ok(())
    }

    pub fn addUnassignedComponents(mut unassigned: Arc<Vector::Vector<Arc<InstNode::InstNode>>>, mut variables: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<()> {
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        for mut var in &*variables.clone() {
            let mut var = var.clone();
            ty = InstNode::getType(var.clone())?;
            if Type::isScalarBuiltin(ty.clone()) && !(Component::hasBinding(InstNode::component(var.clone())?, Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?) {
                Vector::push(unassigned.clone(), var.clone());
            }
        }
        Ok(())
    }

    pub fn checkUseBeforeAssign2(mut unassigned: Arc<Vector::Vector<Arc<InstNode::InstNode>>>, mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<()> {
        let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
        for mut stmt in &*statements.clone() {
            let mut stmt = stmt.clone();
            info = Statement::info(stmt.clone());
            let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            checkUseBeforeAssignExp(unassigned.clone(), var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), info.clone())?;
            markAssignedOutput(unassigned.clone(), var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone())?;
            ()
        },
        Deref @ Statement::FOR { .. } => {
            if isSome(var_field!((*stmt).range, Statement::NFStatement::FOR).clone()) {
                checkUseBeforeAssignExp(unassigned.clone(), Util::getOption(var_field!((*stmt).range, Statement::NFStatement::FOR).clone())?, info.clone())?;
            }
            checkUseBeforeAssign2(unassigned.clone(), var_field!((*stmt).body, Statement::NFStatement::FOR).clone())?;
            ()
        },
        Deref @ Statement::IF { .. } => {
            checkUseBeforeAssignIf(unassigned.clone(), var_field!((*stmt).branches, Statement::NFStatement::IF).clone(), info.clone())?;
            ()
        },
        Deref @ Statement::ASSERT { .. } => {
            checkUseBeforeAssignExp(unassigned.clone(), var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), info.clone())?;
            checkUseBeforeAssignExp(unassigned.clone(), var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), info.clone())?;
            checkUseBeforeAssignExp(unassigned.clone(), var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), info.clone())?;
            ()
        },
        Deref @ Statement::WHILE { .. } => {
            checkUseBeforeAssignExp(unassigned.clone(), var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), info.clone())?;
            checkUseBeforeAssign2(unassigned.clone(), var_field!((*stmt).body, Statement::NFStatement::WHILE).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(())
    }

    pub fn markAssignedOutput(mut unassigned: Arc<Vector::Vector<Arc<InstNode::InstNode>>>, mut assignedExp: Arc<Expression::NFExpression>) -> Result<()> {
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut index: i32 = 0;
        let () = (::match_deref::match_deref! { match &(assignedExp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isCref(var_field!((*assignedExp).cref, Expression::NFExpression::CREF).clone())) => {
            node = ComponentRef::node(ComponentRef::last(var_field!((*assignedExp).cref, Expression::NFExpression::CREF).clone()))?;
            (_, index) = Vector::find(unassigned.clone(), Arc::new({ let __pe_b0 = node.clone(); move |__pe_a1| Ok(InstNode::refEqual(__pe_b0.clone(), __pe_a1)) }));
            if index.clone() > 0 {
                Vector::remove(unassigned.clone(), index.clone())?;
            }
            ()
        },
        Deref @ Expression::TUPLE { .. } => {
            for mut e in &*var_field!((*assignedExp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut e = e.clone();
                markAssignedOutput(unassigned.clone(), e.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn checkUseBeforeAssignIf(mut unassigned: Arc<Vector::Vector<Arc<InstNode::InstNode>>>, mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>>, mut info: SourceInfo) -> Result<()> {
        let mut unassigned_branch: Arc<Vector::Vector<Arc<InstNode::InstNode>>>;
        let mut assigned: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut index: i32 = 0;
        for mut b in &*branches.clone() {
            let mut b = b.clone();
            checkUseBeforeAssignExp(unassigned.clone(), Util::tuple21(b.clone()), info.clone())?;
        }
        for mut b in &*branches.clone() {
            let mut b = b.clone();
            unassigned_branch = Vector::copy(unassigned.clone());
            checkUseBeforeAssign2(unassigned_branch.clone(), Util::tuple22(b.clone()))?;
            if Vector::size(unassigned.clone()) != Vector::size(unassigned_branch.clone()) {
                assigned = listAppend(List::setDifferenceOnTrue(Vector::toList(unassigned.clone()), Vector::toList(unassigned_branch.clone()), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>))?, assigned.clone());
            }
        }
        if !(assigned.clone().is_empty()) {
            assigned = List::uniqueOnTrue(assigned.clone(), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>));
            for mut a in &*assigned.clone() {
                let mut a = a.clone();
                (_, index) = Vector::find(unassigned.clone(), Arc::new({ let __pe_b0 = a.clone(); move |__pe_a1| Ok(InstNode::refEqual(__pe_b0.clone(), __pe_a1)) }));
                if index.clone() > 0 {
                    Vector::remove(unassigned.clone(), index.clone())?;
                }
            }
        }
        Ok(())
    }

    pub fn checkUseBeforeAssignExp(mut unassigned: Arc<Vector::Vector<Arc<InstNode::InstNode>>>, mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<()> {
        Expression::apply(exp.clone(), Arc::new({ let __pe_b0 = unassigned.clone(); let __pe_b2 = info.clone(); move |__pe_a1| checkUseBeforeAssignExp_traverse(__pe_b0.clone(), __pe_a1, __pe_b2.clone()) }))?;
        Ok(())
    }

    pub fn checkUseBeforeAssignExp_traverse(mut unassigned: Arc<Vector::Vector<Arc<InstNode::InstNode>>>, mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<()> {
        let mut index: i32 = 0;
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) => {
            node = ComponentRef::node(ComponentRef::last(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))?;
            (_, index) = Vector::find(unassigned.clone(), Arc::new({ let __pe_b0 = node.clone(); move |__pe_a1| Ok(InstNode::refEqual(__pe_b0.clone(), __pe_a1)) }));
            if index.clone() > 0 {
                Vector::remove(unassigned.clone(), index.clone())?;
                Error::addSourceMessage(Error::WARNING_DEF_USE.clone(), list![(InstNode::name(node.clone())?).clone()], info.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn instPartialDerivedVars(mut classDef: Arc<SCode::ClassDef>, mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut r#fn: Arc<Function>, mut context: i32, mut info: SourceInfo) -> Result<Arc<metamodelica::List<i32>>> {
        let mut derivedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut index: i32 = 0;
        let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::ClassDef::PDER { .. } => {
            for mut var in &*var_field!((*classDef).derivedVariables, SCode::ClassDef::PDER).clone() {
                let mut var = var.clone();
                index = List::positionOnTrue(inputs.clone(), Arc::new({ let __pe_b1 = (var.clone()).clone(); move |__pe_a0| Ok(InstNode::isNamed(__pe_a0, __pe_b1.clone())) }));
                if index.clone() < 1 {
                    Error::addSourceMessage(Error::PARTIAL_DERIVATIVE_INPUT_NOT_FOUND.clone(), list![(var.clone()).clone(), (AbsynUtil::pathString(getDerivedFunctionName(r#fn.clone()), (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
                    bail!("fail");
                }
                derivedVars = cons(index.clone(), derivedVars.clone());
            }
            derivedVars = derivedVars.clone().reverse();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(derivedVars)
    }

}

