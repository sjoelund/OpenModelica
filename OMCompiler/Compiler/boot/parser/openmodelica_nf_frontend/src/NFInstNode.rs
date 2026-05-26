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
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFConvertDAE as ConvertDAE;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFModifier::Modifier;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::AccessLevel;
use crate::NFPrefixes::Visibility;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

#[derive(Clone, Debug, PartialEq)]
pub enum InstNodeType {
    /// An element with no specific characteristics.
    NORMAL_CLASS,
    /// A base class extended by another class.
    BASE_CLASS {
        parent: Arc<InstNode::InstNode>,
        /// The extends clause definition.
        definition: Arc<SCode::Element>,
        /// The original node type before the class was extended.
        ty: Arc<InstNodeType>,
    },
    /// A short class definition.
    DERIVED_CLASS {
        /// The base node type not considering that it's a derived class.
        ty: Arc<InstNodeType>,
    },
    /// A builtin element.
    BUILTIN_CLASS,
    /// The unnamed class containing all the top-level classes.
    TOP_SCOPE {
        annotationScope: Arc<InstNode::InstNode>,
        generatedInners: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<InstNode::InstNode>>>,
    },
    /// The root of the instance tree, i.e. the class that the instantiation starts from.
    ROOT_CLASS {
        /// The parent of the class, e.g. when instantiating a function
        ///                     in a component where the component is the parent.
        parent: Arc<InstNode::InstNode>,
        /// Used by getModelInstance to add context to instances.
        context: Option<Arc<Absyn::Path>>,
    },
    NORMAL_COMP,
    REDECLARED_COMP {
        /// The parent of the replaced component
        parent: Arc<InstNode::InstNode>,
    },
    REDECLARED_CLASS {
        parent: Arc<InstNode::InstNode>,
        originalType: Arc<InstNodeType>,
        originalNode: Option<Arc<InstNode::InstNode>>,
    },
    /// A generated inner element due to a missing outer.
    GENERATED_INNER,
    /// An implicit scope that's ignored when e.g. constructing a scope path. Not
    ///     used by implicit scope nodes since those have no node type (they're
    ///     implicitly implicit), but by e.g. the annotation scope.
    IMPLICIT_SCOPE,
}
impl Default for InstNodeType {
    fn default() -> Self { Self::NORMAL_CLASS }
}
pub use self::InstNodeType::{NORMAL_CLASS,BASE_CLASS,DERIVED_CLASS,BUILTIN_CLASS,TOP_SCOPE,ROOT_CLASS,NORMAL_COMP,REDECLARED_COMP,REDECLARED_CLASS,GENERATED_INNER,IMPLICIT_SCOPE};

pub const NUMBER_OF_CACHES: i32 = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum PackageCacheState {
    NOT_INITIALIZED = 1,
    PROCESSING = 2,
    EXPANDED = 3,
    PARTIALLY_INSTANTIATED = 4,
    INSTANTIATED = 5,
}
impl PartialOrd for PackageCacheState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for PackageCacheState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub mod CachedData {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    pub enum CachedData {
        NO_CACHE,
        PACKAGE {
            instance: Arc<InstNode::InstNode>,
            state: PackageCacheState,
        },
        FUNCTION {
            funcs: Arc<metamodelica::List<Arc<Function::Function>>>,
            typed: bool,
            specialBuiltin: bool,
        },
    }
    impl Default for CachedData {
        fn default() -> Self { Self::NO_CACHE }
    }
    pub use self::CachedData::{NO_CACHE,PACKAGE,FUNCTION};
    pub fn empty() -> metamodelica::Array<Arc<CachedData>> {
        let mut cache: metamodelica::Array<Arc<CachedData>> = arrayCreate(NUMBER_OF_CACHES.clone(), Arc::new(crate::NFInstNode::CachedData::NO_CACHE));
        cache
    }

    pub fn initFunc(mut caches: metamodelica::Array<Arc<CachedData>>) -> Result<()> {
        let mut func_cache: Arc<CachedData> = Arc::new(CachedData::NO_CACHE);
        func_cache = getFuncCache(caches.clone());
        func_cache = (::match_deref::match_deref! { match &(func_cache.clone()) {
        Deref @ NO_CACHE { .. } => Arc::new(CachedData::FUNCTION { funcs: metamodelica::nil(), typed: false, specialBuiltin: false }),
        Deref @ FUNCTION { .. } => func_cache.clone(),
        _ => bail!("match: no arm matched"),
    } });
        setFuncCache(caches.clone(), func_cache.clone())?;
        Ok(())
    }

    pub fn addFunc(mut r#fn: Arc<Function::Function>, mut specialBuiltin: bool, mut caches: metamodelica::Array<Arc<CachedData>>) -> Result<()> {
        let mut func_cache: Arc<CachedData> = Arc::new(CachedData::NO_CACHE);
        func_cache = getFuncCache(caches.clone());
        func_cache = (::match_deref::match_deref! { match &(func_cache.clone()) {
        Deref @ NO_CACHE { .. } => Arc::new(CachedData::FUNCTION { funcs: list![r#fn.clone()], typed: false, specialBuiltin: specialBuiltin.clone() }),
        Deref @ FUNCTION { .. } => Arc::new(CachedData::FUNCTION { funcs: listAppend(var_field!((*func_cache).funcs, CachedData::FUNCTION).clone(), list![r#fn.clone()]), typed: false, specialBuiltin: var_field!((*func_cache).specialBuiltin, CachedData::FUNCTION).clone() || specialBuiltin.clone() }),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.CachedData.addFunc")); __mm_s.push_str(&*literal!(": Invalid cache for function")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        setFuncCache(caches.clone(), func_cache.clone())?;
        Ok(())
    }

    pub fn getFuncCache(mut in_caches: metamodelica::Array<Arc<CachedData>>) -> Arc<CachedData> {
        let mut out_cache: Arc<CachedData> = in_caches.clone().borrow()[(1-1) as usize].clone();
        out_cache
    }

    pub fn setFuncCache(mut in_caches: metamodelica::Array<Arc<CachedData>>, mut in_cache: Arc<CachedData>) -> Result<()> {
        {let _arr = in_caches.clone(); _arr.borrow_mut()[(1-1) as usize] = in_cache.clone(); _arr};
        Ok(())
    }

    pub fn getPackageCache(mut in_caches: metamodelica::Array<Arc<CachedData>>) -> Arc<CachedData> {
        let mut out_cache: Arc<CachedData> = in_caches.clone().borrow()[(2-1) as usize].clone();
        out_cache
    }

    pub fn setPackageCache(mut in_caches: metamodelica::Array<Arc<CachedData>>, mut in_cache: Arc<CachedData>) -> metamodelica::Array<Arc<CachedData>> {
        let mut out_caches: metamodelica::Array<Arc<CachedData>> = {let _arr = in_caches.clone(); _arr.borrow_mut()[(2-1) as usize] = in_cache.clone(); _arr};
        out_caches
    }

    pub fn clearPackageCache(mut in_caches: metamodelica::Array<Arc<CachedData>>) -> metamodelica::Array<Arc<CachedData>> {
        let mut out_caches: metamodelica::Array<Arc<CachedData>> = {let _arr = in_caches.clone(); _arr.borrow_mut()[(2-1) as usize] = Arc::new(crate::NFInstNode::CachedData::NO_CACHE); _arr};
        out_caches
    }

}

pub mod InstNode {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    pub enum InstNode {
        CLASS_NODE {
            name: ArcStr,
            definition: Arc<SCode::Element>,
            visibility: Visibility,
            cls: Pointer::Pointer<Arc<Class::NFClass>>,
            caches: metamodelica::Array<Arc<CachedData::CachedData>>,
            parentScope: Arc<InstNode>,
            nodeType: Arc<InstNodeType>,
        },
        COMPONENT_NODE {
            name: ArcStr,
            definition: Option<Arc<SCode::Element>>,
            visibility: Visibility,
            component: Pointer::Pointer<Arc<Component::NFComponent>>,
            /// The instance that this component is part of.
            parent: Arc<InstNode>,
            nodeType: Arc<InstNodeType>,
        },
        /// A node representing an outer element, with a reference to the corresponding inner.
        INNER_OUTER_NODE {
            innerNode: Arc<InstNode>,
            outerNode: Arc<InstNode>,
        },
        REF_NODE {
            index: i32,
        },
        NAME_NODE {
            name: ArcStr,
        },
        IMPLICIT_SCOPE {
            parentScope: Arc<InstNode>,
            locals: Arc<metamodelica::List<Arc<InstNode>>>,
        },
        ITERATOR_NODE {
            exp: Arc<Expression::NFExpression>,
        },
        /// This is an extension for better use in the backend. Not used in the Frontend.
        ///    NOTE: Map and traversal functions are not allowed to follow the variable
        ///    pointer, it would create cyclic behaviour! Var->cref->pointer->Var
        VAR_NODE {
            name: ArcStr,
            varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>,
        },
        EMPTY_NODE,
    }
    impl Default for InstNode {
        fn default() -> Self { Self::EMPTY_NODE }
    }
    pub use self::InstNode::{CLASS_NODE,COMPONENT_NODE,INNER_OUTER_NODE,REF_NODE,NAME_NODE,IMPLICIT_SCOPE,ITERATOR_NODE,VAR_NODE,EMPTY_NODE};
    pub fn new(mut definition: Arc<SCode::Element>, mut parent: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        node = (::match_deref::match_deref! { match &(definition.clone()) {
        Deref @ SCode::Element::CLASS { .. } => newClass(definition.clone(), parent.clone(), Arc::new(crate::NFInstNode::InstNodeType::NORMAL_CLASS))?,
        Deref @ SCode::Element::COMPONENT { .. } => newComponent(definition.clone(), parent.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn newClass(mut definition: Arc<SCode::Element>, mut parent: Arc<InstNode>, mut nodeType: Arc<InstNodeType>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut name: ArcStr = arcstr::literal!("");
        let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(definition.clone()) {
            Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { visibility: __pa0, .. }, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vis = __pa0.clone();
        name = __pa1.clone();
        node = Arc::new(InstNode::CLASS_NODE { name: (name.clone()).clone(), definition: definition.clone(), visibility: Prefixes::visibilityFromSCode(vis.clone()), cls: Pointer::create(Arc::new(crate::NFClass::NOT_INSTANTIATED)), caches: CachedData::empty(), parentScope: parent.clone(), nodeType: nodeType.clone() });
        Ok(node)
    }

    pub fn newComponent(mut definition: Arc<SCode::Element>, mut parent: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut name: ArcStr = arcstr::literal!("");
        let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(definition.clone()) {
            Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { visibility: __pa0, .. }, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vis = __pa0.clone();
        name = __pa1.clone();
        node = Arc::new(InstNode::COMPONENT_NODE { name: (name.clone()).clone(), definition: Some(definition.clone()), visibility: Prefixes::visibilityFromSCode(vis.clone()), component: Pointer::create(Component::new(definition.clone())), parent: parent.clone(), nodeType: Arc::new(crate::NFInstNode::InstNodeType::NORMAL_COMP) });
        Ok(node)
    }

    pub fn newExtends(mut definition: Arc<SCode::Element>, mut parent: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut base_path: Arc<Absyn::Path>;
        let mut name: ArcStr = arcstr::literal!("");
        let mut vis: SCode::Visibility = SCode::Visibility::PROTECTED;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(definition.clone()) {
            Deref @ SCode::Element::EXTENDS { visibility: __pa0, baseClassPath: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vis = __pa0.clone();
        base_path = __pa1.clone();
        name = (AbsynUtil::pathLastIdent(base_path.clone())?).clone();
        node = Arc::new(InstNode::CLASS_NODE { name: (name.clone()).clone(), definition: definition.clone(), visibility: Prefixes::visibilityFromSCode(vis.clone()), cls: Pointer::create(Arc::new(crate::NFClass::NOT_INSTANTIATED)), caches: CachedData::empty(), parentScope: parent.clone(), nodeType: Arc::new(InstNodeType::BASE_CLASS { parent: parent.clone(), definition: definition.clone(), ty: nodeType(parent.clone())? }) });
        Ok(node)
    }

    pub fn newIterator(mut name: ArcStr, mut ty: Arc<Type::NFType>, mut info: SourceInfo) -> Arc<InstNode> {
        let mut iterator: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        iterator = fromComponent((name.clone()).clone(), Component::newIterator(ty.clone(), info.clone()), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
        iterator
    }

    pub fn newUniqueIterator(mut info: SourceInfo, mut ty: Arc<Type::NFType>) -> Arc<InstNode> {
        let mut iterator: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        iterator = newIterator(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$i")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", System::tmpTickIndex(Global::iteratorIndex.clone())))); ArcStr::from(__mm_s) }).clone(), ty.clone(), info.clone());
        iterator
    }

    pub fn newIndexedIterator(mut index: i32, mut name: ArcStr, mut info: SourceInfo, mut ty: Arc<Type::NFType>) -> Arc<InstNode> {
        let mut iterator: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        iterator = newIterator(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); ArcStr::from(__mm_s) }).clone(), ty.clone(), info.clone());
        iterator
    }

    pub fn fromComponent(mut name: ArcStr, mut component: Arc<Component::NFComponent>, mut parent: Arc<InstNode>) -> Arc<InstNode> {
        let mut node: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        node = Arc::new(InstNode::COMPONENT_NODE { name: (name.clone()).clone(), definition: None, visibility: Visibility::PUBLIC.clone(), component: Pointer::create(component.clone()), parent: parent.clone(), nodeType: Arc::new(crate::NFInstNode::InstNodeType::NORMAL_COMP) });
        node
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isClass(mut node: Arc<InstNode>) -> bool {
        let mut isClass: bool = false;
        isClass = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => true,
        Deref @ INNER_OUTER_NODE { .. } => self::isClass(var_field!((*node).innerNode, InstNode::INNER_OUTER_NODE).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isClass
    }

    pub fn isBaseClass(mut node: Arc<InstNode>) -> bool {
        let mut isBaseClass: bool = false;
        isBaseClass = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::BASE_CLASS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isBaseClass
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isUserdefinedClass(mut node: Arc<InstNode>) -> bool {
        let mut isUserdefined: bool = false;
        let mut ty: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
        isUserdefined = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: ty, .. } => (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ InstNodeType::NORMAL_CLASS => true,
        Deref @ InstNodeType::BASE_CLASS { .. } => true,
        Deref @ InstNodeType::DERIVED_CLASS { .. } => true,
        Deref @ InstNodeType::REDECLARED_CLASS { .. } => isUserdefinedClass(var_field!((**ty).parent, InstNodeType::REDECLARED_CLASS).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isUserdefined
    }

    pub fn isDerivedClass(mut node: Arc<InstNode>) -> bool {
        let mut isDerived: bool = false;
        isDerived = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::DERIVED_CLASS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isDerived
    }

    pub fn makeRootClass(mut node: Arc<InstNode>, mut parent: Arc<InstNode>, mut context: Option<Arc<Absyn::Path>>) -> Arc<InstNode> {
        let mut node: Arc<InstNode> = node;
        node = setNodeType(Arc::new(InstNodeType::ROOT_CLASS { parent: parent.clone(), context: context.clone() }), node.clone());
        node
    }

    pub fn isRootClass(mut node: Arc<InstNode>) -> bool {
        let mut res: bool = false;
        res = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::ROOT_CLASS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    pub fn rootClassContext(mut node: Arc<InstNode>) -> Option<Arc<Absyn::Path>> {
        let mut context: Option<Arc<Absyn::Path>> = None;
        context = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::ROOT_CLASS { context, .. }, .. } => context.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        context
    }

    pub fn isFunction(mut node: Arc<InstNode>) -> Result<bool> {
        let mut isFunc: bool = false;
        isFunc = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Class::isFunction(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone())),
        Deref @ COMPONENT_NODE { .. } => Class::isFunction(getClass(node.clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isFunc)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isComponent(mut node: Arc<InstNode>) -> bool {
        let mut isComponent: bool = false;
        isComponent = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => true,
        Deref @ INNER_OUTER_NODE { .. } => self::isComponent(var_field!((*node).innerNode, InstNode::INNER_OUTER_NODE).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isComponent
    }

    pub fn isRef(mut node: Arc<InstNode>) -> bool {
        let mut isRef: bool = false;
        isRef = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ REF_NODE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isRef
    }

    pub fn isEmpty(mut node: Arc<InstNode>) -> bool {
        let mut isEmpty: bool = false;
        isEmpty = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ EMPTY_NODE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isEmpty
    }

    pub fn isImplicit(mut node: Arc<InstNode>) -> bool {
        let mut isImplicit: bool = false;
        isImplicit = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ IMPLICIT_SCOPE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isImplicit
    }

    pub fn isName(mut node: Arc<InstNode>) -> bool {
        let mut isName: bool = false;
        isName = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ NAME_NODE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isName
    }

    pub fn isConnector(mut node: Arc<InstNode>) -> Result<bool> {
        let mut isConnector: bool = false;
        isConnector = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::isConnector(component(node.clone())?),
        Deref @ NAME_NODE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isConnector)
    }

    pub fn isExpandableConnector(mut node: Arc<InstNode>) -> Result<bool> {
        let mut isConnector: bool = false;
        isConnector = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::isExpandableConnector(component(node.clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isConnector)
    }

    pub fn hasParentExpandableConnector(mut node: Arc<InstNode>) -> Result<bool> {
        let mut b: bool = isExpandableConnector(node.clone())?;
        let mut p: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        p = node.clone();
        while !(isEmpty(p.clone())) {
            p = parent(p.clone());
            b = boolOr(b.clone(), isExpandableConnector(p.clone())?);
            if b.clone() {
                break;
            }
        }
        Ok(b)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isOperator(mut node: Arc<InstNode>) -> bool {
        let mut op: bool = false;
        op = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => SCodeUtil::isOperator(var_field!((*node).definition, InstNode::CLASS_NODE).clone()),
        Deref @ INNER_OUTER_NODE { .. } => isOperator(var_field!((*node).innerNode, InstNode::INNER_OUTER_NODE).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        op
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn name(mut node: Arc<InstNode>) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        name = ((::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => var_field!((*node).name, InstNode::CLASS_NODE).clone(),
        Deref @ COMPONENT_NODE { .. } => var_field!((*node).name, InstNode::COMPONENT_NODE).clone(),
        Deref @ INNER_OUTER_NODE { .. } => self::name(var_field!((*node).innerNode, InstNode::INNER_OUTER_NODE).clone())?,
        Deref @ VAR_NODE { .. } => var_field!((*node).name, InstNode::VAR_NODE).clone(),
        Deref @ REF_NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$REF[")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", var_field!((*node).index, InstNode::REF_NODE).clone()))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) },
        Deref @ NAME_NODE { .. } => var_field!((*node).name, InstNode::NAME_NODE).clone(),
        Deref @ IMPLICIT_SCOPE { .. } => literal!("$IMPLICIT"),
        Deref @ ITERATOR_NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$ITERATOR(")); __mm_s.push_str(&*Expression::toString(var_field!((*node).exp, InstNode::ITERATOR_NODE).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ EMPTY_NODE { .. } => literal!("$EMPTY"),
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(name)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isNamed(mut node: Arc<InstNode>, mut name: ArcStr) -> bool {
        let mut res: bool = false;
        res = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => var_field!((*node).name, InstNode::CLASS_NODE).clone() == name.clone(),
        Deref @ COMPONENT_NODE { .. } => var_field!((*node).name, InstNode::COMPONENT_NODE).clone() == name.clone(),
        Deref @ INNER_OUTER_NODE { .. } => isNamed(var_field!((*node).innerNode, InstNode::INNER_OUTER_NODE).clone(), (name.clone()).clone()),
        Deref @ VAR_NODE { .. } => var_field!((*node).name, InstNode::VAR_NODE).clone() == name.clone(),
        Deref @ NAME_NODE { .. } => var_field!((*node).name, InstNode::NAME_NODE).clone() == name.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    pub fn className(mut node: Arc<InstNode>) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        let __pa0 = ::match_deref::match_deref! { match &(node.clone()) {
            Deref @ CLASS_NODE { name: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa0.clone();
        Ok(name)
    }

    pub fn scopeName(mut node: Arc<InstNode>) -> ArcStr {
        let mut outName: ArcStr = name(classScope(explicitScope(node.clone()))).unwrap();
        outName
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn typeName(mut node: Arc<InstNode>) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        name = ((::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => literal!("class"),
        Deref @ COMPONENT_NODE { .. } => literal!("component"),
        Deref @ INNER_OUTER_NODE { .. } => typeName(var_field!((*node).innerNode, InstNode::INNER_OUTER_NODE).clone())?,
        Deref @ REF_NODE { .. } => literal!("ref node"),
        Deref @ NAME_NODE { .. } => literal!("name node"),
        Deref @ IMPLICIT_SCOPE { .. } => literal!("implicit scope"),
        Deref @ EMPTY_NODE { .. } => literal!("empty node"),
        Deref @ VAR_NODE { .. } => literal!("var node"),
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(name)
    }

    pub fn rename(mut name: ArcStr, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            assign_variant_field!(node => InstNode::CLASS_NODE; name = name.clone());
            ()
        },
        Deref @ COMPONENT_NODE { .. } => {
            assign_variant_field!(node => InstNode::COMPONENT_NODE; name = name.clone());
            ()
        },
        Deref @ NAME_NODE { .. } => {
            assign_variant_field!(node => InstNode::NAME_NODE; name = name.clone());
            ()
        },
        Deref @ VAR_NODE { .. } => {
            assign_variant_field!(node => InstNode::VAR_NODE; name = name.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn parent(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut parent: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        parent = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => var_field!((*node).parentScope, InstNode::CLASS_NODE).clone(),
        Deref @ COMPONENT_NODE { .. } => var_field!((*node).parent, InstNode::COMPONENT_NODE).clone(),
        Deref @ IMPLICIT_SCOPE { .. } => var_field!((*node).parentScope, InstNode::IMPLICIT_SCOPE).clone(),
        _ => Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        parent
    }

    pub fn explicitParent(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut parentNode: Arc<InstNode> = explicitScope(parent(node.clone()));
        parentNode
    }

    pub fn classParent(mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut parent: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let __pa0 = ::match_deref::match_deref! { match &(node.clone()) {
            Deref @ CLASS_NODE { parentScope: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        parent = __pa0.clone();
        Ok(parent)
    }

    pub fn instanceParent(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut parent: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        parent = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => getDerivedNode(self::parent(getDerivedNode(node.clone(), true)), true),
        Deref @ COMPONENT_NODE { nodeType: Deref @ InstNodeType::REDECLARED_COMP { parent }, .. } => getDerivedNode(parent.clone(), true),
        Deref @ COMPONENT_NODE { .. } => getDerivedNode(self::parent(getDerivedNode(node.clone(), true)), true),
        Deref @ IMPLICIT_SCOPE { .. } => getDerivedNode(self::parent(getDerivedNode(node.clone(), true)), true),
        _ => Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        parent
    }

    pub fn rootParent(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut parent: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        parent = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => rootTypeParent(var_field!((*node).nodeType, InstNode::CLASS_NODE).clone(), node.clone()),
        _ => self::parent(node.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        parent
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn rootTypeParent(mut nodeType: Arc<InstNodeType>, mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut parent: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        parent = (::match_deref::match_deref! { match &(nodeType.clone()) {
        Deref @ InstNodeType::ROOT_CLASS { .. } if (!(isEmpty(var_field!((*nodeType).parent, InstNodeType::ROOT_CLASS).clone()))) => var_field!((*nodeType).parent, InstNodeType::ROOT_CLASS).clone(),
        Deref @ InstNodeType::DERIVED_CLASS { .. } => rootTypeParent(var_field!((*nodeType).ty, InstNodeType::DERIVED_CLASS).clone(), node.clone()),
        _ => self::parent(node.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        parent
    }

    pub fn parentScope(mut node: Arc<InstNode>, mut ignoreRedeclare: bool) -> Result<Arc<InstNode>> {
        let mut scope: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut orig_node: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        scope = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::DERIVED_CLASS { .. }, .. } => {
            scope = Class::lastBaseClass(node.clone());
            if (isBuiltin(scope.clone())) {topScope(var_field!((*node).parentScope, InstNode::CLASS_NODE).clone())} else if (referenceEq(&node.clone(),&scope.clone())) {var_field!((*node).parentScope, InstNode::CLASS_NODE).clone()} else {parentScope(scope.clone(), false)?}
        },
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { originalNode: Some(orig_node), .. }, .. } if (ignoreRedeclare.clone()) => parentScope(orig_node.clone(), false)?,
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { parent: scope, .. }, .. } if (ignoreRedeclare.clone()) => scope.clone(),
        Deref @ CLASS_NODE { .. } => var_field!((*node).parentScope, InstNode::CLASS_NODE).clone(),
        Deref @ COMPONENT_NODE { .. } => parentScope(Component::classInstance(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())), false)?,
        Deref @ IMPLICIT_SCOPE { .. } => var_field!((*node).parentScope, InstNode::IMPLICIT_SCOPE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(scope)
    }

    pub fn enclosingScopePath(mut node: Arc<InstNode>, mut ignoreRedeclare: bool, mut ignoreBaseClass: bool) -> Result<Arc<Absyn::Path>> {
        let mut path: Arc<Absyn::Path>;
        path = AbsynUtil::stringListPath({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (enclosingScopeList(node.clone(), ignoreRedeclare.clone(), ignoreBaseClass.clone())?).into_iter().cloned() {
            let __x = name(n.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        Ok(path)
    }

    pub fn enclosingScopeList(mut node: Arc<InstNode>, mut ignoreRedeclare: bool, mut ignoreBaseClass: bool) -> Result<Arc<metamodelica::List<Arc<InstNode>>>> {
        let mut res: Arc<metamodelica::List<Arc<InstNode>>> = metamodelica::nil();
        let mut scope: Arc<InstNode> = node.clone();
        while !(isTopScope(scope.clone())) {
            res = cons(scope.clone(), res.clone());
            scope = enclosingScope(scope.clone(), ignoreRedeclare.clone(), ignoreBaseClass.clone())?;
            if isEmpty(scope.clone()) {
                break;
            }
            scope = classScope(scope.clone());
        }
        Ok(res)
    }

    pub fn enclosingScope(mut node: Arc<InstNode>, mut ignoreRedeclare: bool, mut ignoreBaseClass: bool) -> Result<Arc<InstNode>> {
        let mut scope: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut it: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
        let mut orig_node: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        scope = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { originalNode: Some(orig_node), .. }, .. } if (ignoreRedeclare.clone()) => enclosingScope(orig_node.clone(), ignoreRedeclare.clone(), ignoreBaseClass.clone())?,
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { parent: scope, .. }, .. } if (ignoreRedeclare.clone()) => scope.clone(),
        Deref @ CLASS_NODE { .. } => if (ignoreBaseClass.clone()) {getDerivedNode(var_field!((*node).parentScope, InstNode::CLASS_NODE).clone(), true)} else {var_field!((*node).parentScope, InstNode::CLASS_NODE).clone()},
        Deref @ COMPONENT_NODE { .. } => enclosingScope(classScope(node.clone()), ignoreRedeclare.clone(), ignoreBaseClass.clone())?,
        Deref @ IMPLICIT_SCOPE { .. } => var_field!((*node).parentScope, InstNode::IMPLICIT_SCOPE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(scope)
    }

    pub fn classScope(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut scope: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        scope = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::classInstance(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())),
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        scope
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn libraryScope(mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut lib: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        lib = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { parentScope: Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::TOP_SCOPE { .. }, .. }, .. } => node.clone(),
        _ => libraryScope(parentScope(node.clone(), false)?)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(lib)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn topScope(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut topScope: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        topScope = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::TOP_SCOPE { .. }, .. } => node.clone(),
        _ => self::topScope(parent(node.clone())),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        topScope
    }

    pub fn annotationScope(mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut annScope: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        annScope = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::TOP_SCOPE { annotationScope: annScope, .. }, .. } => annScope.clone(),
        _ => annotationScope(parentScope(node.clone(), false)?)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(annScope)
    }

    pub fn isTopScope(mut node: Arc<InstNode>) -> bool {
        let mut res: bool = false;
        res = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::TOP_SCOPE { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn topComponent(mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut topComponent: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        topComponent = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { parent: Deref @ EMPTY_NODE { .. }, .. } => node.clone(),
        Deref @ COMPONENT_NODE { .. } => self::topComponent(var_field!((*node).parent, InstNode::COMPONENT_NODE).clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(topComponent)
    }

    pub fn setParent(mut parent: Arc<InstNode>, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            assign_variant_field!(node => InstNode::CLASS_NODE; parentScope = parent.clone());
            ()
        },
        Deref @ COMPONENT_NODE { .. } => {
            assign_variant_field!(node => InstNode::COMPONENT_NODE; parent = parent.clone());
            ()
        },
        Deref @ IMPLICIT_SCOPE { .. } => {
            assign_variant_field!(node => InstNode::IMPLICIT_SCOPE; parentScope = parent.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn setOrphanParent(mut parent: Arc<InstNode>, mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { parentScope: Deref @ EMPTY_NODE { .. }, .. } => {
            assign_variant_field!(node => InstNode::CLASS_NODE; parentScope = parent.clone());
            ()
        },
        Deref @ COMPONENT_NODE { parent: Deref @ EMPTY_NODE { .. }, .. } => {
            assign_variant_field!(node => InstNode::COMPONENT_NODE; parent = parent.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        node
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getClass(mut node: Arc<InstNode>) -> Result<Arc<Class::NFClass>> {
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        cls = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()),
        Deref @ COMPONENT_NODE { .. } => getClass(Component::classInstance(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())))?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(cls)
    }

    pub fn getDerivedClass(mut node: Arc<InstNode>) -> Result<Arc<Class::NFClass>> {
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        cls = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => getClass(getDerivedNode(node.clone(), true))?,
        Deref @ COMPONENT_NODE { .. } => getClass(getDerivedNode(Component::classInstance(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())), true))?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(cls)
    }

    pub fn getDerivedNode(mut node: Arc<InstNode>, mut recursive: bool) -> Arc<InstNode> {
        let mut derived: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        derived = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => getDerivedNode2(node.clone(), var_field!((*node).nodeType, InstNode::CLASS_NODE).clone(), recursive.clone()),
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        derived
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getDerivedNode2(mut node: Arc<InstNode>, mut ty: Arc<InstNodeType>, mut recursive: bool) -> Arc<InstNode> {
        let mut derived: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        derived = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ InstNodeType::BASE_CLASS { .. } => if (recursive.clone()) {getDerivedNode(var_field!((*ty).parent, InstNodeType::BASE_CLASS).clone(), true)} else {var_field!((*ty).parent, InstNodeType::BASE_CLASS).clone()},
        Deref @ InstNodeType::DERIVED_CLASS { .. } => getDerivedNode2(node.clone(), var_field!((*ty).ty, InstNodeType::DERIVED_CLASS).clone(), recursive.clone()),
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        derived
    }

    pub fn updateClass(mut cls: Arc<Class::NFClass>, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        node = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            Pointer::update(var_field!((*node).cls, InstNode::CLASS_NODE).clone(), cls.clone());
            node.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn component(mut node: Arc<InstNode>) -> Result<Arc<Component::NFComponent>> {
        let mut component: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        component = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()),
        Deref @ VAR_NODE { .. } => Arc::new(crate::NFComponent::WILD),
        Deref @ NAME_NODE { .. } => Arc::new(crate::NFComponent::WILD),
        _ => bail!("match: no arm matched"),
    } });
        Ok(component)
    }

    pub fn updateComponent(mut component: Arc<Component::NFComponent>, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        node = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => {
            Pointer::update(var_field!((*node).component, InstNode::COMPONENT_NODE).clone(), component.clone());
            node.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn replaceComponent(mut component: Arc<Component::NFComponent>, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => {
            assign_variant_field!(node => InstNode::COMPONENT_NODE; component = Pointer::create(component.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn replaceClass(mut cls: Arc<Class::NFClass>, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            assign_variant_field!(node => InstNode::CLASS_NODE; cls = Pointer::create(cls.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn nodeType(mut node: Arc<InstNode>) -> Result<Arc<InstNodeType>> {
        let mut nodeType: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
        nodeType = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => var_field!((*node).nodeType, InstNode::CLASS_NODE).clone(),
        Deref @ COMPONENT_NODE { .. } => var_field!((*node).nodeType, InstNode::COMPONENT_NODE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(nodeType)
    }

    pub fn derivedNodeType(mut node: Arc<InstNode>) -> Result<Arc<InstNodeType>> {
        let mut ty: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
        ty = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::DERIVED_CLASS { ty }, .. } => ty.clone(),
        _ => nodeType(node.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(ty)
    }

    pub fn setNodeType(mut nodeType: Arc<InstNodeType>, mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            assign_variant_field!(node => InstNode::CLASS_NODE; nodeType = nodeType.clone());
            ()
        },
        Deref @ COMPONENT_NODE { .. } => {
            assign_variant_field!(node => InstNode::COMPONENT_NODE; nodeType = nodeType.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        node
    }

    pub fn definition(mut node: Arc<InstNode>) -> Result<Arc<SCode::Element>> {
        let mut definition: Arc<SCode::Element>;
        definition = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => var_field!((*node).definition, InstNode::CLASS_NODE).clone(),
        Deref @ COMPONENT_NODE { definition: Some(definition), .. } => definition.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.definition")); __mm_s.push_str(&*literal!(" failed for non class/component node: ")); __mm_s.push_str(&*toString(node.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(definition)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn classDefinition(mut node: Arc<InstNode>) -> Result<Arc<SCode::Element>> {
        let mut definition: Arc<SCode::Element>;
        definition = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => var_field!((*node).definition, InstNode::CLASS_NODE).clone(),
        Deref @ COMPONENT_NODE { .. } => classDefinition(Component::classInstance(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())))?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.classDefinition")); __mm_s.push_str(&*literal!(" failed for non class/component node: ")); __mm_s.push_str(&*toString(node.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(definition)
    }

    pub fn extendsDefinition(mut node: Arc<InstNode>) -> Result<Option<Arc<SCode::Element>>> {
        let mut definition: Option<Arc<SCode::Element>> = None;
        let mut ty: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
        ty = derivedNodeType(node.clone())?;
        definition = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ InstNodeType::BASE_CLASS { .. } => Some(var_field!((*ty).definition, InstNodeType::BASE_CLASS).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(definition)
    }

    pub fn setDefinition(mut definition: Arc<SCode::Element>, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            assign_variant_field!(node => InstNode::CLASS_NODE; definition = definition.clone());
            ()
        },
        Deref @ COMPONENT_NODE { .. } => {
            assign_variant_field!(node => InstNode::COMPONENT_NODE; definition = Some(definition.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn setComponentDirection(mut direction: Prefixes::Direction, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        node = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => {
            assign_variant_field!(node => InstNode::COMPONENT_NODE; component = Pointer::create(Component::setDirection(direction.clone(), Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))));
            node.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.setComponentDirection")); __mm_s.push_str(&*literal!(" failed for non component node: ")); __mm_s.push_str(&*toString(node.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn info(mut node: Arc<InstNode>) -> Result<SourceInfo> {
        let mut info: SourceInfo;
        info = 'mc: {
        let __mc_input = node.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ CLASS_NODE { nodeType: ty @ Deref @ InstNodeType::BASE_CLASS { .. }, .. } => {
                    Ok(SCodeUtil::elementInfo(var_field!((**ty).definition, InstNodeType::BASE_CLASS).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ CLASS_NODE { .. } => {
                    Ok(SCodeUtil::elementInfo(var_field!((*node).definition, InstNode::CLASS_NODE).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ COMPONENT_NODE { .. } => {
                    Ok(Component::info(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ COMPONENT_NODE { .. } => {
                    Ok(self::info(var_field!((*node).parent, InstNode::COMPONENT_NODE).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Absyn::dummyInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        Ok(info)
    }

    pub fn getType(mut node: Arc<InstNode>) -> Result<Arc<Type::NFType>> {
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        let mut var: Arc<Variable::NFVariable>;
        ty = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Class::getType(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()), node.clone())?,
        Deref @ COMPONENT_NODE { .. } => Component::getType(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))?,
        Deref @ VAR_NODE { .. } => {
            var = Pointer::access(var_field!((*node).varPointer, InstNode::VAR_NODE).clone());
            var.ty.clone()
        },
        Deref @ NAME_NODE { .. } => Arc::new(crate::NFType::UNKNOWN),
        _ => bail!("match: no arm matched"),
    } });
        Ok(ty)
    }

    pub fn classApply<ArgT: Clone + 'static>(mut node: Arc<InstNode>, mut func: Arc<dyn ::std::ops::Fn(ArgT, Arc<Class::NFClass>) -> Result<Arc<Class::NFClass>> + 'static>, mut arg: ArgT) -> Result<Arc<InstNode>> {
        pub type FuncType<ArgT: Clone> = fn(ArgT, Arc<Class::NFClass>) -> Result<Arc<Class::NFClass>>;

        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            Pointer::update(var_field!((*node).cls, InstNode::CLASS_NODE).clone(), func(arg.clone(), Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()))?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    pub fn componentApply<ArgT: Clone + 'static>(mut node: Arc<InstNode>, mut func: Arc<dyn ::std::ops::Fn(ArgT, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>, mut arg: ArgT) -> Result<Arc<InstNode>> {
        pub type FuncType<ArgT: Clone> = fn(ArgT, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>>;

        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => {
            Pointer::update(var_field!((*node).component, InstNode::COMPONENT_NODE).clone(), func(arg.clone(), Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(node)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn scopeList(mut node: Arc<InstNode>, mut includeRoot: bool, mut accumScopes: Arc<metamodelica::List<Arc<InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode>>>> {
        let mut scopes: Arc<metamodelica::List<Arc<InstNode>>> = metamodelica::nil();
        scopes = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            scopeListClass(node.clone(), var_field!((*node).nodeType, InstNode::CLASS_NODE).clone(), includeRoot.clone(), accumScopes.clone())?
        },
        Deref @ COMPONENT_NODE { parent: Deref @ EMPTY_NODE { .. }, .. } => {
            accumScopes.clone()
        },
        Deref @ COMPONENT_NODE { nodeType: Deref @ InstNodeType::REDECLARED_COMP { parent }, .. } => {
            scopeList(parent.clone(), includeRoot.clone(), cons(node.clone(), accumScopes.clone()))?
        },
        Deref @ COMPONENT_NODE { .. } => {
            scopeList(var_field!((*node).parent, InstNode::COMPONENT_NODE).clone(), includeRoot.clone(), cons(node.clone(), accumScopes.clone()))?
        },
        Deref @ IMPLICIT_SCOPE { .. } => {
            scopeList(var_field!((*node).parentScope, InstNode::IMPLICIT_SCOPE).clone(), includeRoot.clone(), accumScopes.clone())?
        },
        _ => {
            accumScopes.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(scopes)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn scopeListClass(mut clsNode: Arc<InstNode>, mut ty: Arc<InstNodeType>, mut includeRoot: bool, mut accumScopes: Arc<metamodelica::List<Arc<InstNode>>>) -> Result<Arc<metamodelica::List<Arc<InstNode>>>> {
        let mut scopes: Arc<metamodelica::List<Arc<InstNode>>> = metamodelica::nil();
        scopes = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ InstNodeType::NORMAL_CLASS => scopeList(parent(clsNode.clone()), includeRoot.clone(), cons(clsNode.clone(), accumScopes.clone()))?,
        Deref @ InstNodeType::BASE_CLASS { .. } => scopeList(var_field!((*ty).parent, InstNodeType::BASE_CLASS).clone(), includeRoot.clone(), accumScopes.clone())?,
        Deref @ InstNodeType::DERIVED_CLASS { .. } => scopeListClass(clsNode.clone(), var_field!((*ty).ty, InstNodeType::DERIVED_CLASS).clone(), includeRoot.clone(), accumScopes.clone())?,
        Deref @ InstNodeType::BUILTIN_CLASS => cons(clsNode.clone(), accumScopes.clone()),
        Deref @ InstNodeType::TOP_SCOPE { .. } => accumScopes.clone(),
        Deref @ InstNodeType::ROOT_CLASS { .. } => if (includeRoot.clone()) {scopeList(parent(clsNode.clone()), includeRoot.clone(), cons(clsNode.clone(), accumScopes.clone()))?} else {accumScopes.clone()},
        Deref @ InstNodeType::REDECLARED_CLASS { .. } => scopeList(var_field!((*ty).parent, InstNodeType::REDECLARED_CLASS).clone(), includeRoot.clone(), cons(getDerivedNode(clsNode.clone(), true), accumScopes.clone()))?,
        Deref @ InstNodeType::IMPLICIT_SCOPE => scopeList(parent(clsNode.clone()), includeRoot.clone(), accumScopes.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.scopeListClass")); __mm_s.push_str(&*literal!(" got unknown node type")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(scopes)
    }

    pub fn getAnnotation(mut name: ArcStr, mut node: Arc<InstNode>) -> Result<(Arc<SCode::Mod>, Arc<InstNode>)> {
        let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        let mut scope: Arc<InstNode> = node.clone();
        let mut ann: Option<Arc<SCode::Annotation>> = None;
        while isComponent(scope.clone()) {
            ann = SCodeUtil::commentAnnotation(Component::comment(component(scope.clone())?)?);
            if isSome(ann.clone()) {
                r#mod = SCodeUtil::lookupAnnotation(Util::getOption(ann.clone())?, (name.clone()).clone())?;
                if !(SCodeUtil::isEmptyMod(r#mod.clone())) {
                    scope = instanceParent(scope.clone());
                    return Ok((r#mod, scope));
                }
            }
            scope = instanceParent(scope.clone());
        }
        r#mod = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD);
        Ok((r#mod, scope))
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    #[repr(i32)]
    pub enum ScopeType {
        /// Stops at a root class and doesn't include the root
        RELATIVE = 1,
        /// Stops at a root class and includes the root
        INCLUDING_ROOT = 2,
        /// Stops at the top scope
        FULL = 3,
    }
    impl PartialOrd for ScopeType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl Ord for ScopeType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
    }

    pub fn rootPath(mut node: Arc<InstNode>, mut ignoreBaseClass: bool) -> Arc<Absyn::Path> {
        let mut path: Arc<Absyn::Path> = scopePath(node.clone(), ScopeType::INCLUDING_ROOT.clone(), ignoreBaseClass.clone()).unwrap();
        path
    }

    pub fn fullPath(mut node: Arc<InstNode>, mut ignoreBaseClass: bool) -> Arc<Absyn::Path> {
        let mut path: Arc<Absyn::Path> = scopePath(node.clone(), ScopeType::FULL.clone(), ignoreBaseClass.clone()).unwrap();
        path
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn scopePath(mut node: Arc<InstNode>, mut scopeType: ScopeType, mut ignoreBaseClass: bool) -> Result<Arc<Absyn::Path>> {
        let mut path: Arc<Absyn::Path>;
        path = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: it, .. } => {
            (::match_deref::match_deref! { match &(it.clone()) {
        Deref @ InstNodeType::BASE_CLASS { .. } if (!(ignoreBaseClass.clone())) => scopePath(var_field!((**it).parent, InstNodeType::BASE_CLASS).clone(), scopeType.clone(), false)?,
        _ => scopePath2(var_field!((*node).parentScope, InstNode::CLASS_NODE).clone(), scopeType.clone(), Arc::new(Absyn::Path::IDENT { name: (var_field!((*node).name, InstNode::CLASS_NODE).clone()).clone() }))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ COMPONENT_NODE { .. } => {
            scopePath2(var_field!((*node).parent, InstNode::COMPONENT_NODE).clone(), scopeType.clone(), Arc::new(Absyn::Path::IDENT { name: (var_field!((*node).name, InstNode::COMPONENT_NODE).clone()).clone() }))?
        },
        Deref @ IMPLICIT_SCOPE { .. } => {
            scopePath(var_field!((*node).parentScope, InstNode::IMPLICIT_SCOPE).clone(), scopeType.clone(), false)?
        },
        _ => {
            Arc::new(Absyn::Path::IDENT { name: (name(node.clone())?).clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(path)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn scopePath2(mut node: Arc<InstNode>, mut scopeType: ScopeType, mut accumPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
        let mut path: Arc<Absyn::Path>;
        path = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => scopePathClass(node.clone(), var_field!((*node).nodeType, InstNode::CLASS_NODE).clone(), scopeType.clone(), accumPath.clone())?,
        Deref @ COMPONENT_NODE { .. } => scopePath2(var_field!((*node).parent, InstNode::COMPONENT_NODE).clone(), scopeType.clone(), Arc::new(Absyn::Path::QUALIFIED { name: (var_field!((*node).name, InstNode::COMPONENT_NODE).clone()).clone(), path: accumPath.clone() }))?,
        _ => accumPath.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(path)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn scopePathClass(mut node: Arc<InstNode>, mut ty: Arc<InstNodeType>, mut scopeType: ScopeType, mut accumPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
        let mut path: Arc<Absyn::Path>;
        path = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ InstNodeType::NORMAL_CLASS => scopePath2(classParent(node.clone())?, scopeType.clone(), Arc::new(Absyn::Path::QUALIFIED { name: (className(node.clone())?).clone(), path: accumPath.clone() }))?,
        Deref @ InstNodeType::BASE_CLASS { .. } => scopePath2(var_field!((*ty).parent, InstNodeType::BASE_CLASS).clone(), scopeType.clone(), accumPath.clone())?,
        Deref @ InstNodeType::DERIVED_CLASS { .. } => scopePathClass(node.clone(), var_field!((*ty).ty, InstNodeType::DERIVED_CLASS).clone(), scopeType.clone(), accumPath.clone())?,
        Deref @ InstNodeType::BUILTIN_CLASS => Arc::new(Absyn::Path::QUALIFIED { name: (className(node.clone())?).clone(), path: accumPath.clone() }),
        Deref @ InstNodeType::TOP_SCOPE { .. } => accumPath.clone(),
        Deref @ InstNodeType::ROOT_CLASS { .. } => if (scopeType.clone() == ScopeType::FULL.clone()) {scopePath2(classParent(node.clone())?, scopeType.clone(), Arc::new(Absyn::Path::QUALIFIED { name: (className(node.clone())?).clone(), path: accumPath.clone() }))?} else if (scopeType.clone() == ScopeType::INCLUDING_ROOT.clone()) {Arc::new(Absyn::Path::QUALIFIED { name: (className(node.clone())?).clone(), path: accumPath.clone() })} else {accumPath.clone()},
        Deref @ InstNodeType::REDECLARED_CLASS { .. } => scopePath2(var_field!((*ty).parent, InstNodeType::REDECLARED_CLASS).clone(), scopeType.clone(), Arc::new(Absyn::Path::QUALIFIED { name: (className(node.clone())?).clone(), path: accumPath.clone() }))?,
        Deref @ InstNodeType::IMPLICIT_SCOPE => scopePath2(classParent(node.clone())?, scopeType.clone(), accumPath.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.scopePathClass")); __mm_s.push_str(&*literal!(" got unknown node type")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(path)
    }

    pub fn isInput(mut node: Arc<InstNode>) -> bool {
        let mut isInput: bool = false;
        isInput = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::isInput(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isInput
    }

    pub fn isOutput(mut node: Arc<InstNode>) -> bool {
        let mut isOutput: bool = false;
        isOutput = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::isOutput(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isOutput
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isInner(mut node: Arc<InstNode>) -> Result<bool> {
        let mut isInner: bool = false;
        isInner = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::isInner(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())),
        Deref @ CLASS_NODE { .. } => AbsynUtil::isInner(SCodeUtil::prefixesInnerOuter(SCodeUtil::elementPrefixes(var_field!((*node).definition, InstNode::CLASS_NODE).clone())?)?),
        Deref @ INNER_OUTER_NODE { .. } => self::isInner(var_field!((*node).outerNode, InstNode::INNER_OUTER_NODE).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isInner)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isOuter(mut node: Arc<InstNode>) -> Result<bool> {
        let mut isOuter: bool = false;
        isOuter = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::isOuter(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())),
        Deref @ CLASS_NODE { .. } => AbsynUtil::isOuter(SCodeUtil::prefixesInnerOuter(SCodeUtil::elementPrefixes(var_field!((*node).definition, InstNode::CLASS_NODE).clone())?)?),
        Deref @ INNER_OUTER_NODE { .. } => self::isOuter(var_field!((*node).outerNode, InstNode::INNER_OUTER_NODE).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isOuter)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isOnlyOuter(mut node: Arc<InstNode>) -> Result<bool> {
        let mut isOuter: bool = false;
        isOuter = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::isOnlyOuter(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())),
        Deref @ CLASS_NODE { .. } => AbsynUtil::isOnlyOuter(SCodeUtil::prefixesInnerOuter(SCodeUtil::elementPrefixes(var_field!((*node).definition, InstNode::CLASS_NODE).clone())?)?),
        Deref @ INNER_OUTER_NODE { .. } => isOnlyOuter(var_field!((*node).outerNode, InstNode::INNER_OUTER_NODE).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isOuter)
    }

    pub fn isInnerOuterNode(mut node: Arc<InstNode>) -> bool {
        let mut isIO: bool = false;
        isIO = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ INNER_OUTER_NODE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isIO
    }

    pub fn isGeneratedInner(mut node: Arc<InstNode>) -> bool {
        let mut isInner: bool = false;
        isInner = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::GENERATED_INNER, .. } => true,
        Deref @ COMPONENT_NODE { nodeType: Deref @ InstNodeType::GENERATED_INNER, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isInner
    }

    pub fn resolveInner(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut innerNode: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        innerNode = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ INNER_OUTER_NODE { .. } => var_field!((*node).innerNode, InstNode::INNER_OUTER_NODE).clone(),
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        innerNode
    }

    pub fn resolveOuter(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut outerNode: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        outerNode = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ INNER_OUTER_NODE { .. } => var_field!((*node).outerNode, InstNode::INNER_OUTER_NODE).clone(),
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outerNode
    }

    pub fn cacheInitFunc(mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            CachedData::initFunc(var_field!((*node).caches, InstNode::CLASS_NODE).clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.cacheInitFunc")); __mm_s.push_str(&*literal!(" got node without cache")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn cacheAddFunc(mut node: Arc<InstNode>, mut r#fn: Arc<Function::Function>, mut specialBuiltin: bool) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            CachedData::addFunc(r#fn.clone(), specialBuiltin.clone(), var_field!((*node).caches, InstNode::CLASS_NODE).clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.cacheAddFunc")); __mm_s.push_str(&*literal!(" got node without cache")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn newFuncCache(mut node: Arc<InstNode>, mut in_func_cache: Arc<CachedData::CachedData>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            assign_variant_field!(node => InstNode::CLASS_NODE; caches = arrayCreate(1, in_func_cache.clone()));
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.newFuncCache")); __mm_s.push_str(&*literal!(" got node without cache")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn getFuncCache(mut inNode: Arc<InstNode>) -> Result<Arc<CachedData::CachedData>> {
        let mut func_cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
        func_cache = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ CLASS_NODE { .. } => CachedData::getFuncCache(var_field!((*inNode).caches, InstNode::CLASS_NODE).clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.getFuncCache")); __mm_s.push_str(&*literal!(" got node without cache")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(func_cache)
    }

    pub fn setFuncCache(mut node: Arc<InstNode>, mut in_func_cache: Arc<CachedData::CachedData>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            CachedData::setFuncCache(var_field!((*node).caches, InstNode::CLASS_NODE).clone(), in_func_cache.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.setFuncCache")); __mm_s.push_str(&*literal!(" got node without cache")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn getPackageCache(mut inNode: Arc<InstNode>) -> Result<Arc<CachedData::CachedData>> {
        let mut pack_cache: Arc<CachedData::CachedData> = Arc::new(CachedData::NO_CACHE);
        pack_cache = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ CLASS_NODE { .. } => CachedData::getPackageCache(var_field!((*inNode).caches, InstNode::CLASS_NODE).clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.getPackageCache")); __mm_s.push_str(&*literal!(" got node without cache")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(pack_cache)
    }

    pub fn setPackageCache(mut node: Arc<InstNode>, mut packageNode: Arc<InstNode>, mut state: PackageCacheState) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            CachedData::setPackageCache(var_field!((*node).caches, InstNode::CLASS_NODE).clone(), Arc::new(CachedData::CachedData::PACKAGE { instance: packageNode.clone(), state: state.clone() }));
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.setPackageCache")); __mm_s.push_str(&*literal!(" got node without cache")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn clearPackageCache(mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            CachedData::clearPackageCache(var_field!((*node).caches, InstNode::CLASS_NODE).clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.clearPackageCache")); __mm_s.push_str(&*literal!(" got node without cache")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn openImplicitScope(mut scope: Arc<InstNode>) -> Arc<InstNode> {
        let mut scope: Arc<InstNode> = scope;
        scope = (::match_deref::match_deref! { match &(scope.clone()) {
        Deref @ IMPLICIT_SCOPE { .. } => scope.clone(),
        _ => Arc::new(InstNode::IMPLICIT_SCOPE { parentScope: scope.clone(), locals: metamodelica::nil() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        scope
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn explicitScope(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut scope: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        scope = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ IMPLICIT_SCOPE { .. } => explicitScope(var_field!((*node).parentScope, InstNode::IMPLICIT_SCOPE).clone()),
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        scope
    }

    pub fn addIterator(mut iterator: Arc<InstNode>, mut scope: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut scope: Arc<InstNode> = scope;
        scope = (::match_deref::match_deref! { match &(scope.clone()) {
        Deref @ IMPLICIT_SCOPE { .. } => Arc::new(InstNode::IMPLICIT_SCOPE { parentScope: scope.clone(), locals: cons(iterator.clone(), var_field!((*scope).locals, InstNode::IMPLICIT_SCOPE).clone()) }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(scope)
    }

    pub fn refEqual(mut node1: Arc<InstNode>, mut node2: Arc<InstNode>) -> bool {
        let mut refEqual: bool = false;
        refEqual = (::match_deref::match_deref! { match &((node1.clone(), node2.clone())) {
        (Deref @ CLASS_NODE { .. }, Deref @ CLASS_NODE { .. }) => referenceEq(&Pointer::access(var_field!((*node1).cls, InstNode::CLASS_NODE).clone()),&Pointer::access(var_field!((*node2).cls, InstNode::CLASS_NODE).clone())),
        (Deref @ COMPONENT_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => referenceEq(&Pointer::access(var_field!((*node1).component, InstNode::COMPONENT_NODE).clone()),&Pointer::access(var_field!((*node2).component, InstNode::COMPONENT_NODE).clone())),
        (Deref @ VAR_NODE { .. }, Deref @ VAR_NODE { .. }) => referenceEq(&Pointer::access(var_field!((*node1).varPointer, InstNode::VAR_NODE).clone()),&Pointer::access(var_field!((*node2).varPointer, InstNode::VAR_NODE).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        refEqual
    }

    pub fn refCompare(mut node1: Arc<InstNode>, mut node2: Arc<InstNode>) -> Result<i32> {
        let mut res: i32 = 0;
        res = (::match_deref::match_deref! { match &((node1.clone(), node2.clone())) {
        (Deref @ CLASS_NODE { .. }, Deref @ CLASS_NODE { .. }) => Util::referenceCompare(Pointer::access(var_field!((*node1).cls, InstNode::CLASS_NODE).clone()), Pointer::access(var_field!((*node2).cls, InstNode::CLASS_NODE).clone())),
        (Deref @ COMPONENT_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => Util::referenceCompare(Pointer::access(var_field!((*node1).component, InstNode::COMPONENT_NODE).clone()), Pointer::access(var_field!((*node2).component, InstNode::COMPONENT_NODE).clone())),
        (Deref @ CLASS_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => Util::referenceCompare(Pointer::access(var_field!((*node1).cls, InstNode::CLASS_NODE).clone()), Pointer::access(var_field!((*node2).component, InstNode::COMPONENT_NODE).clone())),
        (Deref @ COMPONENT_NODE { .. }, Deref @ CLASS_NODE { .. }) => Util::referenceCompare(Pointer::access(var_field!((*node1).component, InstNode::COMPONENT_NODE).clone()), Pointer::access(var_field!((*node2).cls, InstNode::CLASS_NODE).clone())),
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    pub fn nameEqual(mut node1: Arc<InstNode>, mut node2: Arc<InstNode>) -> bool {
        let mut equal: bool = name(node1.clone()).unwrap() == name(node2.clone()).unwrap();
        equal
    }

    pub fn isSame(mut node1: Arc<InstNode>, mut node2: Arc<InstNode>) -> bool {
        let mut same: bool = false;
        let mut n1: Arc<InstNode> = resolveOuter(node1.clone());
        let mut n2: Arc<InstNode> = resolveOuter(node2.clone());
        if referenceEq(&n1.clone(),&n2.clone()) {
            same = true;
            return same;
        }
        match '__try0: {
            same = referenceEq(&unwrap_break_err!(definition(node1.clone()), '__try0),&unwrap_break_err!(definition(node2.clone()), '__try0));
            Ok::<_, anyhow::Error>((same.clone(),))
        } {
            Ok((__try0_o0,)) => {
                same = __try0_o0;
            }
            Err(_) => {
                same = false;
            }
        }
        same
    }

    pub fn checkIdentical(mut node1: Arc<InstNode>, mut node2: Arc<InstNode>) -> Result<()> {
        let mut n1: Arc<InstNode> = resolveOuter(node1.clone());
        let mut n2: Arc<InstNode> = resolveOuter(node2.clone());
        if referenceEq(&n1.clone(),&n2.clone()) {
            return Ok(());
        }
        let () = 'mc: {
        let __mc_input = (n1.clone(), n2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ CLASS_NODE { .. }, Deref @ CLASS_NODE { .. }) => {
                    if !((Class::isIdentical(getClass(n1.clone())?, getClass(n2.clone())?))) { bail!("guard") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ COMPONENT_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => {
                    if !((Component::isIdentical(component(n1.clone())?, component(n2.clone())?)?)) { bail!("guard") }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMultiSourceMessage(Error::DUPLICATE_ELEMENTS_NOT_IDENTICAL.clone(), list![(toString(n1.clone())?).clone(), (toString(n2.clone())?).clone()], list![info(n1.clone())?, info(n2.clone())?])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        Ok(())
    }

    pub fn toString(mut node: Arc<InstNode>) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        name = ((::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::toString((var_field!((*node).name, InstNode::COMPONENT_NODE).clone()).clone(), Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))?,
        Deref @ CLASS_NODE { .. } => SCodeDump::unparseElementStr(var_field!((*node).definition, InstNode::CLASS_NODE).clone(), SCodeDump::defaultOptions.clone())?,
        _ => self::name(node.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(name)
    }

    pub fn toFlatString(mut node: Arc<InstNode>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        name = ((::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::toFlatString((var_field!((*node).name, InstNode::COMPONENT_NODE).clone()).clone(), Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()), format.clone(), (indent.clone()).clone())?,
        Deref @ CLASS_NODE { .. } => Class::toFlatString(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()), node.clone(), format.clone(), (indent.clone()).clone())?,
        _ => self::name(node.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(name)
    }

    pub fn toFlatStream(mut node: Arc<InstNode>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        s = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::toFlatStream((var_field!((*node).name, InstNode::COMPONENT_NODE).clone()).clone(), Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()), format.clone(), (indent.clone()).clone(), s.clone())?,
        Deref @ CLASS_NODE { .. } => Class::toFlatStream(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()), node.clone(), format.clone(), (indent.clone()).clone(), s.clone())?,
        _ => IOStream::append(s.clone(), (toFlatString(node.clone(), format.clone(), (indent.clone()).clone())?).clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(s)
    }

    pub fn isRedeclare(mut node: Arc<InstNode>) -> Result<bool> {
        let mut isRedeclare: bool = false;
        isRedeclare = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => SCodeUtil::isElementRedeclare(definition(node.clone())?)?,
        Deref @ COMPONENT_NODE { .. } => Component::isRedeclare(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isRedeclare)
    }

    pub fn isRedeclared(mut node: Arc<InstNode>) -> Result<bool> {
        let mut redeclared: bool = false;
        redeclared = (::match_deref::match_deref! { match &(nodeType(node.clone())?) {
        Deref @ InstNodeType::REDECLARED_COMP { .. } => true,
        Deref @ InstNodeType::REDECLARED_CLASS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(redeclared)
    }

    pub fn getRedeclaredNode(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut outNode: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        outNode = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { originalNode: Some(outNode), .. }, .. } => outNode.clone(),
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outNode
    }

    pub fn isReplaceable(mut node: Arc<InstNode>) -> Result<bool> {
        let mut repl: bool = false;
        let mut elem: Arc<SCode::Element>;
        repl = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => SCodeUtil::isElementReplaceable(var_field!((*node).definition, InstNode::CLASS_NODE).clone())?,
        Deref @ COMPONENT_NODE { definition: Some(elem), .. } => SCodeUtil::isElementReplaceable(elem.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(repl)
    }

    pub fn isProtectedBaseClass(mut node: Arc<InstNode>) -> bool {
        let mut isProtected: bool = false;
        isProtected = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::BASE_CLASS { definition: Deref @ SCode::Element::EXTENDS { visibility: SCode::Visibility::PROTECTED { .. }, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isProtected
    }

    pub fn visibility(mut node: Arc<InstNode>) -> Visibility {
        let mut vis: Visibility = Visibility::PUBLIC;
        vis = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => var_field!((*node).visibility, InstNode::CLASS_NODE).clone(),
        Deref @ COMPONENT_NODE { .. } => var_field!((*node).visibility, InstNode::COMPONENT_NODE).clone(),
        _ => Visibility::PUBLIC.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        vis
    }

    pub fn isProtected(mut node: Arc<InstNode>) -> bool {
        let mut isProtected: bool = false;
        isProtected = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { visibility: Prefixes::Visibility::PROTECTED { .. }, .. } => true,
        Deref @ COMPONENT_NODE { visibility: Prefixes::Visibility::PROTECTED { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isProtected
    }

    pub fn isPublic(mut node: Arc<InstNode>) -> bool {
        let mut isPublic: bool = !(isProtected(node.clone()));
        isPublic
    }

    pub fn protectClass(mut cls: Arc<InstNode>) -> Arc<InstNode> {
        let mut cls: Arc<InstNode> = cls;
        let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ CLASS_NODE { visibility: Prefixes::Visibility::PUBLIC { .. }, .. } => {
            assign_variant_field!(cls => InstNode::CLASS_NODE; visibility = Visibility::PROTECTED.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        cls
    }

    pub fn protectComponent(mut comp: Arc<InstNode>) -> Arc<InstNode> {
        let mut comp: Arc<InstNode> = comp;
        let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ COMPONENT_NODE { visibility: Prefixes::Visibility::PUBLIC { .. }, .. } => {
            assign_variant_field!(comp => InstNode::COMPONENT_NODE; visibility = Visibility::PROTECTED.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        comp
    }

    pub fn protect(mut node: Arc<InstNode>) -> Arc<InstNode> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { visibility: Prefixes::Visibility::PUBLIC { .. }, .. } => {
            assign_variant_field!(node => InstNode::COMPONENT_NODE; visibility = Visibility::PROTECTED.clone());
            ()
        },
        Deref @ CLASS_NODE { visibility: Prefixes::Visibility::PUBLIC { .. }, .. } => {
            assign_variant_field!(node => InstNode::CLASS_NODE; visibility = Visibility::PROTECTED.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        node
    }

    pub fn isEncapsulated(mut node: Arc<InstNode>) -> Result<bool> {
        let mut enc: bool = false;
        enc = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Class::isEncapsulated(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone())),
        Deref @ COMPONENT_NODE { .. } => Class::isEncapsulated(getClass(node.clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(enc)
    }

    pub fn getModifier(mut node: Arc<InstNode>) -> Arc<Modifier::Modifier> {
        let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
        r#mod = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Class::getModifier(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone())),
        Deref @ COMPONENT_NODE { .. } => Component::getModifier(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())),
        _ => Arc::new(crate::NFModifier::Modifier::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        r#mod
    }

    pub fn mergeModifier(mut r#mod: Arc<Modifier::Modifier>, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            Pointer::update(var_field!((*node).cls, InstNode::CLASS_NODE).clone(), Class::mergeModifier(r#mod.clone(), Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()))?);
            ()
        },
        Deref @ COMPONENT_NODE { .. } => {
            Pointer::update(var_field!((*node).component, InstNode::COMPONENT_NODE).clone(), Component::mergeModifier(r#mod.clone(), Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn setModifier(mut r#mod: Arc<Modifier::Modifier>, mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            Pointer::update(var_field!((*node).cls, InstNode::CLASS_NODE).clone(), Class::setModifier(r#mod.clone(), Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()))?);
            ()
        },
        Deref @ COMPONENT_NODE { .. } => {
            Pointer::update(var_field!((*node).component, InstNode::COMPONENT_NODE).clone(), Component::mergeModifier(r#mod.clone(), Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn toPartialDAEType(mut clsNode: Arc<InstNode>) -> Result<Arc<DAE::Type>> {
        let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
        outType = (::match_deref::match_deref! { match &(clsNode.clone()) {
        Deref @ CLASS_NODE { .. } => {
            let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            let mut state: ClassInf::State;
            let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
            cls = Pointer::access(var_field!((*clsNode).cls, InstNode::CLASS_NODE).clone());
            (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::DAE_TYPE { .. } => stripDAETypeVars(var_field!((*cls).ty, Class::NFClass::DAE_TYPE).clone()),
        _ => {
            res = Class::restriction(cls.clone());
            state = Restriction::toDAE(res.clone(), fullPath(clsNode.clone(), false));
            Arc::new(DAE::Type::T_COMPLEX { complexClassType: state.clone(), varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: Restriction::isExternalRecord(res.clone()) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(outType)
    }

    pub fn stripDAETypeVars(mut ty: Arc<DAE::Type>) -> Arc<DAE::Type> {
        let mut ty: Arc<DAE::Type> = ty;
        let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_COMPLEX { .. } => {
            assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = metamodelica::nil());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ty
    }

    pub fn toFullDAEType(mut clsNode: Arc<InstNode>) -> Result<Arc<DAE::Type>> {
        let mut outType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
        outType = (::match_deref::match_deref! { match &(clsNode.clone()) {
        Deref @ CLASS_NODE { .. } => {
            let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            let mut vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut state: ClassInf::State;
            let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
            cls = Pointer::access(var_field!((*clsNode).cls, InstNode::CLASS_NODE).clone());
            (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::DAE_TYPE { .. } => var_field!((*cls).ty, Class::NFClass::DAE_TYPE).clone(),
        _ => {
            res = Class::restriction(cls.clone());
            state = Restriction::toDAE(res.clone(), fullPath(clsNode.clone(), false));
            vars = ConvertDAE::makeTypeVars(clsNode.clone())?;
            outType = Arc::new(DAE::Type::T_COMPLEX { complexClassType: state.clone(), varLst: vars.clone(), equalityConstraint: None, usedExternally: Restriction::isExternalRecord(res.clone()) });
            Pointer::update(var_field!((*clsNode).cls, InstNode::CLASS_NODE).clone(), Arc::new(Class::NFClass::DAE_TYPE { ty: outType.clone() }));
            outType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(outType)
    }

    pub fn isBuiltin(mut node: Arc<InstNode>) -> bool {
        let mut isBuiltin: bool = false;
        isBuiltin = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => isBuiltinNodeType(var_field!((*node).nodeType, InstNode::CLASS_NODE).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isBuiltin
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isBuiltinNodeType(mut nodeType: Arc<InstNodeType>) -> bool {
        let mut isBuiltin: bool = false;
        isBuiltin = (::match_deref::match_deref! { match &(nodeType.clone()) {
        Deref @ InstNodeType::BUILTIN_CLASS => true,
        Deref @ InstNodeType::BASE_CLASS { .. } => isBuiltinNodeType(var_field!((*nodeType).ty, InstNodeType::BASE_CLASS).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isBuiltin
    }

    pub fn isPartial(mut node: Arc<InstNode>) -> bool {
        let mut isPartial: bool = false;
        isPartial = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Class::isPartial(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone())),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isPartial
    }

    pub fn clone(mut node: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut node: Arc<InstNode> = node;
        let () = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => {
            let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
            cls = Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone());
            cls = Class::classTreeApply(cls.clone(), Arc::new(fnptr!(ClassTree::clone, Arc<ClassTree::ClassTree>)));
            assign_variant_field!(node => InstNode::CLASS_NODE;
                cls = Pointer::create(cls.clone()),
                caches = CachedData::empty()
            );
            ()
        },
        Deref @ COMPONENT_NODE { .. } => {
            let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
            comp = Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone());
            comp = Component::setClassInstance(clone(Component::classInstance(comp.clone()))?, comp.clone())?;
            assign_variant_field!(node => InstNode::COMPONENT_NODE; component = Pointer::create(comp.clone()));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    pub fn cloneComponent(mut component: Arc<InstNode>, mut newParent: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut outComponent: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        outComponent = (::match_deref::match_deref! { match &(component.clone()) {
        Deref @ COMPONENT_NODE { .. } => Arc::new(InstNode::COMPONENT_NODE { name: (var_field!((*component).name, InstNode::COMPONENT_NODE).clone()).clone(), definition: var_field!((*component).definition, InstNode::COMPONENT_NODE).clone(), visibility: var_field!((*component).visibility, InstNode::COMPONENT_NODE).clone(), component: Pointer::create(Pointer::access(var_field!((*component).component, InstNode::COMPONENT_NODE).clone())), parent: newParent.clone(), nodeType: var_field!((*component).nodeType, InstNode::COMPONENT_NODE).clone() }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(outComponent)
    }

    pub fn getComments(mut node: Arc<InstNode>, mut accumCmts: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Arc<metamodelica::List<Arc<SCode::Comment>>> {
        let mut cmts: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
        cmts = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { definition: Deref @ SCode::Element::CLASS { cmt, .. }, .. } => {
            cons(cmt.clone(), Class::getDerivedComments(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()), accumCmts.clone()))
        },
        _ => {
            accumCmts.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        cmts
    }

    pub fn copyInstancePtr(mut srcNode: Arc<InstNode>, mut dstNode: Arc<InstNode>) -> Result<Arc<InstNode>> {
        let mut dstNode: Arc<InstNode> = dstNode;
        let () = (::match_deref::match_deref! { match &((srcNode.clone(), dstNode.clone())) {
        (Deref @ COMPONENT_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => {
            assign_variant_field!(dstNode => InstNode::COMPONENT_NODE; component = var_field!((*srcNode).component, InstNode::COMPONENT_NODE).clone());
            ()
        },
        (Deref @ CLASS_NODE { .. }, Deref @ CLASS_NODE { .. }) => {
            assign_variant_field!(dstNode => InstNode::CLASS_NODE; cls = var_field!((*srcNode).cls, InstNode::CLASS_NODE).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(dstNode)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isRecord(mut node: Arc<InstNode>) -> bool {
        let mut isRec: bool = false;
        isRec = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Restriction::isRecord(Class::restriction(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()))),
        Deref @ COMPONENT_NODE { .. } => isRecord(Component::classInstance(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isRec
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isModel(mut node: Arc<InstNode>) -> bool {
        let mut isModel: bool = false;
        isModel = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Restriction::isModel(Class::restriction(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone()))),
        Deref @ COMPONENT_NODE { .. } => self::isModel(Component::classInstance(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isModel
    }

    pub fn isEnumerationType(mut node: Arc<InstNode>) -> bool {
        let mut isEnum: bool = isClass(node.clone()) && Class::isEnumeration(getClass(resolveInner(node.clone())).unwrap()).unwrap();
        isEnum
    }

    pub fn hasBinding(mut node: Arc<InstNode>) -> Result<bool> {
        let mut hasBinding: bool = false;
        hasBinding = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::hasBinding(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))? || self::hasBinding(instanceParent(node.clone()))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(hasBinding)
    }

    pub fn getBindingExpOpt(mut node: Arc<InstNode>) -> Option<Arc<Expression::NFExpression>> {
        let mut binding_exp: Option<Arc<Expression::NFExpression>> = None;
        binding_exp = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => {
            let mut scope: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
            scope = instanceParent(node.clone());
            match '__try0: {
                binding_exp = Binding::getExpOpt(Component::getImplicitBinding(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()), scope.clone()));
                Ok::<_, anyhow::Error>((binding_exp.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    binding_exp = __try0_o0;
                }
                Err(_) => {
                    binding_exp = getBindingExpOpt(scope.clone());
                }
            }
            binding_exp.clone()
        },
        Deref @ VAR_NODE { .. } => {
            let mut var: Arc<Variable::NFVariable>;
            var = Pointer::access(var_field!((*node).varPointer, InstNode::VAR_NODE).clone());
            Binding::getExpOpt(var.binding.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        binding_exp
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getSections(mut node: Arc<InstNode>) -> Result<Arc<Sections::NFSections>> {
        let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
        let mut cls: Arc<Class::NFClass> = getClass(node.clone())?;
        sections = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::INSTANCED_CLASS { .. } => var_field!((*cls).sections, Class::NFClass::INSTANCED_CLASS).clone(),
        Deref @ Class::TYPED_DERIVED { .. } => getSections(var_field!((*cls).baseClass, Class::NFClass::TYPED_DERIVED).clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.getSections")); __mm_s.push_str(&*literal!(" did not get an instanced class")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(sections)
    }

    pub fn hash(mut node: Arc<InstNode>) -> i32 {
        let mut hash: i32 = stringHashDjb2((name(node.clone()).unwrap()).clone());
        hash
    }

    pub fn hashContinue(mut node: Arc<InstNode>, mut hash: i32) -> Result<i32> {
        let mut hash: i32 = hash;
        hash = stringHashDjb2Continue((name(node.clone())?).clone(), hash.clone());
        Ok(hash)
    }

    pub fn dimensionCount(mut node: Arc<InstNode>) -> i32 {
        let mut count: i32 = 0;
        count = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ COMPONENT_NODE { .. } => Component::dimensionCount(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone())),
        Deref @ CLASS_NODE { .. } => Class::dimensionCount(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone())),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        count
    }

    pub fn isClockType(mut node: Arc<InstNode>) -> bool {
        let mut clock: bool = false;
        clock = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::BUILTIN_CLASS, name: Deref @ "Clock", .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        clock
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn restriction(mut node: Arc<InstNode>) -> Arc<Restriction::NFRestriction> {
        let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
        res = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { .. } => Class::restriction(Pointer::access(var_field!((*node).cls, InstNode::CLASS_NODE).clone())),
        Deref @ COMPONENT_NODE { .. } => restriction(Component::classInstance(Pointer::access(var_field!((*node).component, InstNode::COMPONENT_NODE).clone()))),
        Deref @ INNER_OUTER_NODE { .. } => restriction(var_field!((*node).innerNode, InstNode::INNER_OUTER_NODE).clone()),
        _ => Arc::new(crate::NFRestriction::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    pub fn isExtends(mut node: Arc<InstNode>) -> bool {
        let mut res: bool = false;
        res = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ CLASS_NODE { definition: Deref @ SCode::Element::EXTENDS { .. }, .. } => true,
        Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::BASE_CLASS { definition: Deref @ SCode::Element::EXTENDS { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        res
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn isDiscreteClass(mut clsNode: Arc<InstNode>) -> Result<bool> {
        let mut isDiscrete: bool = false;
        let mut base_node: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        let mut exts: metamodelica::Array<Arc<InstNode>>;
        base_node = Class::lastBaseClass(clsNode.clone());
        cls = getClass(base_node.clone())?;
        isDiscrete = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { restriction: Deref @ Restriction::TYPE, .. } => {
            exts = ClassTree::getExtends(var_field!((*cls).elements, Class::NFClass::EXPANDED_CLASS).clone());
            if ((exts.clone().borrow().len() as i32) == 1) {isDiscreteClass(exts.borrow()[(1-1) as usize].clone())?} else {false}
        },
        _ => Type::isDiscrete(Class::getType(cls.clone(), base_node.clone())?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isDiscrete)
    }

    pub fn clearGeneratedInners(mut node: Arc<InstNode>) -> Result<()> {
        let mut top: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut inners: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<InstNode>>>;
        let __pa0 = ::match_deref::match_deref! { match &(nodeType(topScope(node.clone()))?) {
            Deref @ InstNodeType::TOP_SCOPE { generatedInners: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        inners = __pa0.clone();
        UnorderedMap::clear(inners.clone());
        Ok(())
    }

    pub fn getAccessLevel(mut node: Arc<InstNode>) -> Result<Option<AccessLevel>> {
        let mut access: Option<AccessLevel> = None;
        let mut scope: Arc<InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut access_mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        let mut access_exp: Option<Arc<Absyn::Exp>> = None;
        scope = classScope(parent(resolveInner(node.clone())));
        while isClass(scope.clone()) {
            access_mod = SCodeUtil::lookupElementAnnotation(definition(scope.clone())?, (literal!("Protection")).clone())?;
            access_mod = SCodeUtil::lookupModInMod((literal!("access")).clone(), access_mod.clone());
            access_exp = SCodeUtil::getModifierBinding(access_mod.clone());
            if isSome(access_exp.clone()) {
                access = Prefixes::accessLevelFromAbsyn(Util::getOption(access_exp.clone())?);
                if isSome(access.clone()) {
                    return Ok(access);
                }
            }
            scope = parent(scope.clone());
        }
        Ok(access)
    }

}

