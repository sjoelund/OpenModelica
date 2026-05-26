// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::AbsynUtil;
use crate::BaseModelica;
use crate::DAE;
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
use crate::SCode;
use crate::SCodeDump;
use crate::SCodeUtil;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::IOStream;
use openmodelica_util::List;
use openmodelica_util::Pointer;
use openmodelica_util::UnorderedMap;

pub mod CachedData {
    use super::*;
    pub enum CachedData {
        NO_CACHE,
        PACKAGE {
            instance: Arc<InstNode::InstNode>,
            state: PackageCacheState,
        },
        FUNCTION {
            funcs: metamodelica::List<Arc<Function::Function>>,
            typed: bool,
            specialBuiltin: bool,
        },
    }
    pub use CachedData::*;
    pub fn empty() -> Vec<Arc<CachedData>> {
        todo!()
    }

    pub fn initFunc(caches: Vec<Arc<CachedData>>) -> () {
        todo!()
    }

    pub fn addFunc(r#fn: Arc<Function::Function>, specialBuiltin: bool, caches: Vec<Arc<CachedData>>) -> () {
        todo!()
    }

    pub fn getFuncCache(in_caches: Vec<Arc<CachedData>>) -> Arc<CachedData> {
        todo!()
    }

    pub fn setFuncCache(in_caches: Vec<Arc<CachedData>>, in_cache: Arc<CachedData>) -> () {
        todo!()
    }

    pub fn getPackageCache(in_caches: Vec<Arc<CachedData>>) -> Arc<CachedData> {
        todo!()
    }

    pub fn setPackageCache(in_caches: Vec<Arc<CachedData>>, in_cache: Arc<CachedData>) -> Vec<Arc<CachedData>> {
        todo!()
    }

    pub fn clearPackageCache(in_caches: Vec<Arc<CachedData>>) -> Vec<Arc<CachedData>> {
        todo!()
    }

}

pub mod InstNode {
    use super::*;
    pub enum InstNode {
        CLASS_NODE {
            name: String,
            definition: Arc<SCode::Element>,
            visibility: Visibility,
            cls: Pointer::Pointer<Arc<Class::NFClass>>,
            caches: Vec<Arc<CachedData::CachedData>>,
            parentScope: Arc<InstNode>,
            nodeType: Arc<InstNodeType>,
        },
        COMPONENT_NODE {
            name: String,
            definition: Option<Arc<SCode::Element>>,
            visibility: Visibility,
            component: Pointer::Pointer<Arc<Component::NFComponent>>,
            parent: Arc<InstNode>,
            nodeType: Arc<InstNodeType>,
        },
        INNER_OUTER_NODE {
            innerNode: Arc<InstNode>,
            outerNode: Arc<InstNode>,
        },
        REF_NODE {
            index: i32,
        },
        NAME_NODE {
            name: String,
        },
        IMPLICIT_SCOPE {
            parentScope: Arc<InstNode>,
            locals: metamodelica::List<Arc<InstNode>>,
        },
        ITERATOR_NODE {
            exp: Arc<Expression::NFExpression>,
        },
        VAR_NODE {
            name: String,
            varPointer: Pointer::Pointer<Arc<Variable::NFVariable>>,
        },
        EMPTY_NODE,
    }
    pub use InstNode::*;
    pub fn new(definition: Arc<SCode::Element>, parent: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn newClass(definition: Arc<SCode::Element>, parent: Arc<InstNode>, nodeType: Arc<InstNodeType>) -> Arc<InstNode> {
        todo!()
    }

    pub fn newComponent(definition: Arc<SCode::Element>, parent: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn newExtends(definition: Arc<SCode::Element>, parent: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn newIterator(name: String, ty: Arc<Type::NFType>, info: SourceInfo) -> Arc<InstNode> {
        todo!()
    }

    pub fn newUniqueIterator(info: SourceInfo, ty: Arc<Type::NFType>) -> Arc<InstNode> {
        todo!()
    }

    pub fn newIndexedIterator(index: i32, name: String, info: SourceInfo, ty: Arc<Type::NFType>) -> Arc<InstNode> {
        todo!()
    }

    pub fn fromComponent(name: String, component: Arc<Component::NFComponent>, parent: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn isClass(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isBaseClass(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isUserdefinedClass(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isDerivedClass(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn makeRootClass(node: Arc<InstNode>, parent: Arc<InstNode>, context: Option<Arc<Absyn::Path>>) -> Arc<InstNode> {
        todo!()
    }

    pub fn isRootClass(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn rootClassContext(node: Arc<InstNode>) -> Option<Arc<Absyn::Path>> {
        todo!()
    }

    pub fn isFunction(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isComponent(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isRef(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isEmpty(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isImplicit(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isName(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isConnector(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isExpandableConnector(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn hasParentExpandableConnector(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isOperator(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn name(node: Arc<InstNode>) -> String {
        todo!()
    }

    pub fn isNamed(node: Arc<InstNode>, name: String) -> bool {
        todo!()
    }

    pub fn className(node: Arc<InstNode>) -> String {
        todo!()
    }

    pub fn scopeName(node: Arc<InstNode>) -> String {
        todo!()
    }

    pub fn typeName(node: Arc<InstNode>) -> String {
        todo!()
    }

    pub fn rename(name: String, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn parent(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn explicitParent(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn classParent(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn instanceParent(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn rootParent(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn rootTypeParent(nodeType: Arc<InstNodeType>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn parentScope(node: Arc<InstNode>, ignoreRedeclare: bool) -> Arc<InstNode> {
        todo!()
    }

    pub fn enclosingScopePath(node: Arc<InstNode>, ignoreRedeclare: bool, ignoreBaseClass: bool) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn enclosingScopeList(node: Arc<InstNode>, ignoreRedeclare: bool, ignoreBaseClass: bool) -> metamodelica::List<Arc<InstNode>> {
        todo!()
    }

    pub fn enclosingScope(node: Arc<InstNode>, ignoreRedeclare: bool, ignoreBaseClass: bool) -> Arc<InstNode> {
        todo!()
    }

    pub fn classScope(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn libraryScope(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn topScope(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn annotationScope(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn isTopScope(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn topComponent(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn setParent(parent: Arc<InstNode>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn setOrphanParent(parent: Arc<InstNode>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn getClass(node: Arc<InstNode>) -> Arc<Class::NFClass> {
        todo!()
    }

    pub fn getDerivedClass(node: Arc<InstNode>) -> Arc<Class::NFClass> {
        todo!()
    }

    pub fn getDerivedNode(node: Arc<InstNode>, recursive: bool) -> Arc<InstNode> {
        todo!()
    }

    pub fn getDerivedNode2(node: Arc<InstNode>, ty: Arc<InstNodeType>, recursive: bool) -> Arc<InstNode> {
        todo!()
    }

    pub fn updateClass(cls: Arc<Class::NFClass>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn component(node: Arc<InstNode>) -> Arc<Component::NFComponent> {
        todo!()
    }

    pub fn updateComponent(component: Arc<Component::NFComponent>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn replaceComponent(component: Arc<Component::NFComponent>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn replaceClass(cls: Arc<Class::NFClass>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn nodeType(node: Arc<InstNode>) -> Arc<InstNodeType> {
        todo!()
    }

    pub fn derivedNodeType(node: Arc<InstNode>) -> Arc<InstNodeType> {
        todo!()
    }

    pub fn setNodeType(nodeType: Arc<InstNodeType>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn definition(node: Arc<InstNode>) -> Arc<SCode::Element> {
        todo!()
    }

    pub fn classDefinition(node: Arc<InstNode>) -> Arc<SCode::Element> {
        todo!()
    }

    pub fn extendsDefinition(node: Arc<InstNode>) -> Option<Arc<SCode::Element>> {
        todo!()
    }

    pub fn setDefinition(definition: Arc<SCode::Element>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn setComponentDirection(direction: NFPrefixes::Direction, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn info(node: Arc<InstNode>) -> SourceInfo {
        todo!()
    }

    pub fn getType(node: Arc<InstNode>) -> Arc<Type::NFType> {
        todo!()
    }

    pub fn classApply<ArgT>(node: Arc<InstNode>, func: fn(ArgT, Arc<Class::NFClass>) -> Arc<Class::NFClass>, arg: ArgT) -> Arc<InstNode> {
        todo!()
    }

    pub fn componentApply<ArgT>(node: Arc<InstNode>, func: fn(ArgT, Arc<Component::NFComponent>) -> Arc<Component::NFComponent>, arg: ArgT) -> Arc<InstNode> {
        todo!()
    }

    pub fn scopeList(node: Arc<InstNode>, includeRoot: bool, accumScopes: metamodelica::List<Arc<InstNode>>) -> metamodelica::List<Arc<InstNode>> {
        todo!()
    }

    pub fn scopeListClass(clsNode: Arc<InstNode>, ty: Arc<InstNodeType>, includeRoot: bool, accumScopes: metamodelica::List<Arc<InstNode>>) -> metamodelica::List<Arc<InstNode>> {
        todo!()
    }

    pub fn getAnnotation(name: String, node: Arc<InstNode>) -> (Arc<SCode::Mod>, Arc<InstNode>) {
        todo!()
    }

    pub enum ScopeType {
        RELATIVE,
        INCLUDING_ROOT,
        FULL,
    }

    pub fn rootPath(node: Arc<InstNode>, ignoreBaseClass: bool) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn fullPath(node: Arc<InstNode>, ignoreBaseClass: bool) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn scopePath(node: Arc<InstNode>, scopeType: ScopeType, ignoreBaseClass: bool) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn scopePath2(node: Arc<InstNode>, scopeType: ScopeType, accumPath: Arc<Absyn::Path>) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn scopePathClass(node: Arc<InstNode>, ty: Arc<InstNodeType>, scopeType: ScopeType, accumPath: Arc<Absyn::Path>) -> Arc<Absyn::Path> {
        todo!()
    }

    pub fn isInput(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isOutput(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isInner(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isOuter(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isOnlyOuter(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isInnerOuterNode(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isGeneratedInner(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn resolveInner(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn resolveOuter(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn cacheInitFunc(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn cacheAddFunc(node: Arc<InstNode>, r#fn: Arc<Function::Function>, specialBuiltin: bool) -> Arc<InstNode> {
        todo!()
    }

    pub fn newFuncCache(node: Arc<InstNode>, in_func_cache: Arc<CachedData::CachedData>) -> Arc<InstNode> {
        todo!()
    }

    pub fn getFuncCache(inNode: Arc<InstNode>) -> Arc<CachedData::CachedData> {
        todo!()
    }

    pub fn setFuncCache(node: Arc<InstNode>, in_func_cache: Arc<CachedData::CachedData>) -> Arc<InstNode> {
        todo!()
    }

    pub fn getPackageCache(inNode: Arc<InstNode>) -> Arc<CachedData::CachedData> {
        todo!()
    }

    pub fn setPackageCache(node: Arc<InstNode>, packageNode: Arc<InstNode>, state: PackageCacheState) -> Arc<InstNode> {
        todo!()
    }

    pub fn clearPackageCache(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn openImplicitScope(scope: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn explicitScope(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn addIterator(iterator: Arc<InstNode>, scope: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn refEqual(node1: Arc<InstNode>, node2: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn refCompare(node1: Arc<InstNode>, node2: Arc<InstNode>) -> i32 {
        todo!()
    }

    pub fn nameEqual(node1: Arc<InstNode>, node2: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isSame(node1: Arc<InstNode>, node2: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn checkIdentical(node1: Arc<InstNode>, node2: Arc<InstNode>) -> () {
        todo!()
    }

    pub fn toString(node: Arc<InstNode>) -> String {
        todo!()
    }

    pub fn toFlatString(node: Arc<InstNode>, format: BaseModelica::OutputFormat, indent: String) -> String {
        todo!()
    }

    pub fn toFlatStream(node: Arc<InstNode>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
        todo!()
    }

    pub fn isRedeclare(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isRedeclared(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn getRedeclaredNode(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn isReplaceable(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isProtectedBaseClass(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn visibility(node: Arc<InstNode>) -> Visibility {
        todo!()
    }

    pub fn isProtected(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isPublic(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn protectClass(cls: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn protectComponent(comp: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn protect(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn isEncapsulated(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn getModifier(node: Arc<InstNode>) -> Arc<Modifier::Modifier> {
        todo!()
    }

    pub fn mergeModifier(r#mod: Arc<Modifier::Modifier>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn setModifier(r#mod: Arc<Modifier::Modifier>, node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn toPartialDAEType(clsNode: Arc<InstNode>) -> Arc<DAE::Type> {
        todo!()
    }

    pub fn stripDAETypeVars(ty: Arc<DAE::Type>) -> Arc<DAE::Type> {
        todo!()
    }

    pub fn toFullDAEType(clsNode: Arc<InstNode>) -> Arc<DAE::Type> {
        todo!()
    }

    pub fn isBuiltin(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isBuiltinNodeType(nodeType: Arc<InstNodeType>) -> bool {
        todo!()
    }

    pub fn isPartial(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn clone(node: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn cloneComponent(component: Arc<InstNode>, newParent: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn getComments(node: Arc<InstNode>, accumCmts: metamodelica::List<Arc<SCode::Comment>>) -> metamodelica::List<Arc<SCode::Comment>> {
        todo!()
    }

    pub fn copyInstancePtr(srcNode: Arc<InstNode>, dstNode: Arc<InstNode>) -> Arc<InstNode> {
        todo!()
    }

    pub fn isRecord(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isModel(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isEnumerationType(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn hasBinding(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn getBindingExpOpt(node: Arc<InstNode>) -> Option<Arc<Expression::NFExpression>> {
        todo!()
    }

    pub fn getSections(node: Arc<InstNode>) -> Arc<Sections::NFSections> {
        todo!()
    }

    pub fn hash(node: Arc<InstNode>) -> i32 {
        todo!()
    }

    pub fn hashContinue(node: Arc<InstNode>, hash: i32) -> i32 {
        todo!()
    }

    pub fn dimensionCount(node: Arc<InstNode>) -> i32 {
        todo!()
    }

    pub fn isClockType(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn restriction(node: Arc<InstNode>) -> Arc<Restriction::NFRestriction> {
        todo!()
    }

    pub fn isExtends(node: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn isDiscreteClass(clsNode: Arc<InstNode>) -> bool {
        todo!()
    }

    pub fn clearGeneratedInners(node: Arc<InstNode>) -> () {
        todo!()
    }

    pub fn getAccessLevel(node: Arc<InstNode>) -> Option<AccessLevel> {
        todo!()
    }

}

pub enum InstNodeType {
    NORMAL_CLASS,
    BASE_CLASS {
        parent: Arc<InstNode::InstNode>,
        definition: Arc<SCode::Element>,
        ty: Arc<InstNodeType>,
    },
    DERIVED_CLASS {
        ty: Arc<InstNodeType>,
    },
    BUILTIN_CLASS,
    TOP_SCOPE {
        annotationScope: Arc<InstNode::InstNode>,
        generatedInners: UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, String>,
    },
    ROOT_CLASS {
        parent: Arc<InstNode::InstNode>,
        context: Option<Arc<Absyn::Path>>,
    },
    NORMAL_COMP,
    REDECLARED_COMP {
        parent: Arc<InstNode::InstNode>,
    },
    REDECLARED_CLASS {
        parent: Arc<InstNode::InstNode>,
        originalType: Arc<InstNodeType>,
        originalNode: Option<Arc<InstNode::InstNode>>,
    },
    GENERATED_INNER,
    IMPLICIT_SCOPE,
}
pub use InstNodeType::*;

pub enum PackageCacheState {
    NOT_INITIALIZED,
    PROCESSING,
    EXPANDED,
    PARTIALLY_INSTANTIATED,
    INSTANTIATED,
}

