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
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponent::ComponentState;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectBreakTree;
use crate::NFDimension as Dimension;
use crate::NFEvalConstants as EvalConstants;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionStatus;
use crate::NFInst as Inst;
use crate::NFInst::InstSettings;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFLookup as Lookup;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

pub mod Field {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Field {
        INPUT {
            name: ArcStr,
        },
        LOCAL {
            name: ArcStr,
        },
    }
    pub use self::Field::{INPUT,LOCAL};
    pub fn isInput(mut field: Arc<Field>) -> bool {
        let mut isInput: bool = false;
        isInput = (::match_deref::match_deref! { match &(field.clone()) {
        Deref @ INPUT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isInput
    }

    pub fn name(mut field: Arc<Field>) -> Result<ArcStr> {
        let mut name: ArcStr = arcstr::literal!("");
        name = ((::match_deref::match_deref! { match &(field.clone()) {
        Deref @ INPUT { .. } => var_field!((*field).name, Field::INPUT).clone(),
        Deref @ LOCAL { .. } => var_field!((*field).name, Field::LOCAL).clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(name)
    }

}

pub fn instRecord(mut node: Arc<InstNode::InstNode>, mut context: i32) -> Result<Arc<InstNode::InstNode>> {
    let mut recordNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut next_context: i32 = 0;
    match '__try0: {
        (recordNode, _) = unwrap_break_err!(Lookup::lookupLocalSimpleName((InstNode::name(node.clone())?).clone(), InstNode::classScope(InstNode::parent(node.clone()))), '__try0);
        let true = (referenceEq(&unwrap_break_err!(InstNode::definition(node.clone()), '__try0),&unwrap_break_err!(InstNode::definition(recordNode.clone()), '__try0))) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        Ok::<_, anyhow::Error>((recordNode.clone(),))
    } {
        Ok((__try0_o0,)) => {
            recordNode = __try0_o0;
        }
        Err(_) => {
            recordNode = InstNode::replaceClass(Arc::new(crate::NFClass::NOT_INSTANTIATED), node.clone())?;
        }
    }
    next_context = InstContext::set(context.clone(), InstContext::RELAXED.clone());
    next_context = InstContext::set(next_context.clone(), InstContext::FUNCTION.clone());
    recordNode = InstNode::makeRootClass(recordNode.clone(), InstNode::parent(node.clone()), None);
    recordNode = Inst::instantiate(recordNode.clone(), Arc::new(crate::NFModifier::Modifier::NOMOD), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), next_context.clone(), false)?;
    Inst::instExpressions(recordNode.clone(), recordNode.clone(), Arc::new(crate::NFSections::EMPTY), NFConnectBreakTree::new(), next_context.clone(), Inst::InstSettings::create())?;
    Ok(recordNode)
}

pub fn instDefaultConstructor(mut path: Arc<Absyn::Path>, mut node: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut all_params: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut sorted_locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut attr: DAE::FunctionAttributes;
    let mut status: Pointer::Pointer<FunctionStatus>;
    let mut ctor_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut out_rec: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut out_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut ctor_cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    ctor_node = instRecord(node.clone(), context.clone())?;
    (inputs, locals, all_params) = collectRecordParams(ctor_node.clone())?;
    out_comp = Arc::new(Component::NFComponent::COMPONENT { classInst: ctor_node.clone(), ty: Arc::new(Type::NFType::UNTYPED { typeNode: node.clone(), dimensions: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()) }), binding: Binding::EMPTY_BINDING().clone(), condition: Binding::EMPTY_BINDING().clone(), attributes: Attributes::OUTPUT_ATTR().clone(), comment: SCode::noComment.clone(), state: ComponentState::FullyInstantiated.clone(), info: Absyn::dummyInfo.clone() });
    out_rec = InstNode::fromComponent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$out")); __mm_s.push_str(&*InstNode::name(ctor_node.clone())?); ArcStr::from(__mm_s) }).clone(), out_comp.clone(), ctor_node.clone());
    ctor_cls = Class::makeRecordConstructor(all_params.clone(), out_rec.clone())?;
    ctor_node = InstNode::replaceClass(ctor_cls.clone(), ctor_node.clone())?;
    InstNode::classApply(ctor_node.clone(), Arc::new(Class::setType), Arc::new(Type::NFType::COMPLEX { cls: ctor_node.clone(), complexTy: Arc::new(crate::NFComplexType::CLASS) }))?;
    attr = DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone();
    status = Pointer::create(FunctionStatus::INITIAL.clone());
    InstNode::cacheAddFunc(node.clone(), Arc::new(Function::Function { path: path.clone(), node: ctor_node.clone(), inputs: inputs.clone(), outputs: list![out_rec.clone()], locals: locals.clone(), interfaceDiffInfo: None, slots: metamodelica::nil(), returnType: Arc::new(crate::NFType::UNKNOWN), attributes: attr.clone(), derivatives: metamodelica::nil(), derivedInputs: metamodelica::nil(), inverses: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), status: status.clone(), callCounter: Pointer::create(0) }), false)?;
    Ok(node)
}

pub fn checkLocalFieldOrder(mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut recNode: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<()> {
    let mut locals_set: Arc<UnorderedSet::UnorderedSet<Arc<InstNode::InstNode>>>;
    let mut locs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut deps: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut loc: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if (locals.clone().len() as i32) <= 1 {
        return Ok(());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(locals.clone().reverse()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    loc = __pa0.clone();
    locs = __pa1.clone();
    locals_set = UnorderedSet::fromList(list![loc.clone()], fnptr!(InstNode::hash, Arc<InstNode::InstNode>), fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>))?;
    for mut l in &*locs.clone() {
        let mut l = l.clone();
        deps = Function::getLocalDependencies(l.clone(), locals_set.clone())?;
        if !(deps.clone().is_empty()) {
            Error::addSourceMessage(Error::UNSUPPORTED_RECORD_REORDERING.clone(), list![(InstNode::name(recNode.clone())?).clone()], info.clone())?;
            bail!("fail");
        }
        UnorderedSet::add(l.clone(), locals_set.clone())?;
    }
    Ok(())
}

pub fn collectRecordParams(mut recNode: Arc<InstNode::InstNode>) -> Result<(Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>)> {
    let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut allParams: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut comp: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut pcomps: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    tree = Class::classTree(InstNode::getClass(recNode.clone())?)?;
    let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ClassTree::FLAT_TREE { components: comps, .. } => {
            let __range0 = (1..=(comps.clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                comp = comps.borrow()[(i.clone()-1) as usize].clone();
                (inputs, locals) = collectRecordParam(comp.clone(), inputs.clone(), locals.clone())?;
                allParams = cons(comp.clone(), allParams.clone());
            }
            ()
        },
        Deref @ ClassTree::INSTANTIATED_TREE { components: pcomps, .. } => {
            let __range0 = (1..=(pcomps.clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                comp = Mutable::access(pcomps.borrow()[(i.clone()-1) as usize].clone());
                (inputs, locals) = collectRecordParam(comp.clone(), inputs.clone(), locals.clone())?;
                allParams = cons(comp.clone(), allParams.clone());
            }
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFRecord.collectRecordParams")); __mm_s.push_str(&*literal!(" got non-instantiated function")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((inputs, locals, allParams))
}

pub fn collectRecordParam(mut component: Arc<InstNode::InstNode>, mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Result<(Arc<metamodelica::List<Arc<InstNode::InstNode>>>, Arc<metamodelica::List<Arc<InstNode::InstNode>>>)> {
    let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = inputs;
    let mut locals: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = locals;
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut comp_node: Arc<InstNode::InstNode> = InstNode::resolveInner(component.clone());
    if InstNode::isProtected(comp_node.clone()) {
        locals = cons(comp_node.clone(), locals.clone());
        return Ok((inputs, locals));
    }
    comp = InstNode::component(comp_node.clone())?;
    if Component::isFinal(comp.clone())? {
        setFieldDirection(comp_node.clone(), Direction::NONE.clone())?;
        locals = cons(comp_node.clone(), locals.clone());
    } else {
        setFieldDirection(comp_node.clone(), Direction::INPUT.clone())?;
        InstNode::componentApply(comp_node.clone(), Arc::new(fnptr!(Component::setVariability, Variability, Arc<Component::NFComponent>)), Variability::CONTINUOUS.clone())?;
        inputs = cons(comp_node.clone(), inputs.clone());
    }
    Ok((inputs, locals))
}

pub fn setFieldDirection(mut field: Arc<InstNode::InstNode>, mut direction: Direction) -> Result<()> {
    InstNode::componentApply(field.clone(), Arc::new(fnptr!(Component::setDirection, Direction, Arc<Component::NFComponent>)), direction.clone())?;
    Ok(())
}

pub fn collectRecordFields(mut recNode: Arc<InstNode::InstNode>) -> Result<(metamodelica::Array<Arc<Field::Field>>, Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>)> {
    let mut fields: metamodelica::Array<Arc<Field::Field>>;
    let mut indexMap: Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>;
    let mut field_lst: Arc<metamodelica::List<Arc<Field::Field>>> = metamodelica::nil();
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    tree = Class::classTree(InstNode::getClass(recNode.clone())?)?;
    field_lst = ClassTree::foldComponents(tree.clone(), Arc::new(collectRecordField), metamodelica::nil());
    fields = metamodelica::arrayFromVec(field_lst.clone().reverse().into_iter().cloned().collect());
    indexMap = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), fnptr!(stringEq, ArcStr, ArcStr), (fields.clone().borrow().len() as i32));
    Type::updateRecordFieldsIndexMap(fields.clone(), indexMap.clone())?;
    Ok((fields, indexMap))
}

pub fn collectRecordField(mut component: Arc<InstNode::InstNode>, mut fields: Arc<metamodelica::List<Arc<Field::Field>>>) -> Result<Arc<metamodelica::List<Arc<Field::Field>>>> {
    let mut fields: Arc<metamodelica::List<Arc<Field::Field>>> = fields;
    let mut comp_node: Arc<InstNode::InstNode> = InstNode::resolveInner(component.clone());
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    if InstNode::isProtected(comp_node.clone()) {
        fields = cons(Arc::new(Field::Field::LOCAL { name: (InstNode::name(comp_node.clone())?).clone() }), fields.clone());
    } else {
        comp = InstNode::component(comp_node.clone())?;
        if Component::isFinal(comp.clone())? {
            fields = cons(Arc::new(Field::Field::LOCAL { name: (InstNode::name(comp_node.clone())?).clone() }), fields.clone());
        } else if !(Component::isOutput(comp.clone())) {
            fields = cons(Arc::new(Field::Field::INPUT { name: (InstNode::name(comp_node.clone())?).clone() }), fields.clone());
        }
    }
    Ok(fields)
}

pub fn fieldsToDAE(mut fields: Arc<metamodelica::List<Arc<Field::Field>>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut fieldNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut field in &*fields.clone() {
        let mut field = field.clone();
        let () = (::match_deref::match_deref! { match &(field.clone()) {
        Deref @ Field::INPUT { .. } => {
            fieldNames = cons(var_field!((*field).name, Field::Field::INPUT).clone(), fieldNames.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    fieldNames
}

pub fn foldInputFields<T: Clone + 'static, ArgT: Clone + 'static>(mut fields: Arc<metamodelica::List<Arc<Field::Field>>>, mut args: Arc<metamodelica::List<T>>, mut func: Arc<dyn ::std::ops::Fn(T, ArgT) -> Result<ArgT> + 'static>, mut foldArg: ArgT) -> Result<ArgT> {
    pub type FuncT<T: Clone, ArgT: Clone> = fn(T, ArgT) -> Result<ArgT>;

    let mut foldArg: ArgT = foldArg;
    let mut arg: T;
    let mut rest_args: Arc<metamodelica::List<T>> = args.clone();
    for mut field in &*fields.clone() {
        let mut field = field.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        if Field::isInput(field.clone()) {
            foldArg = func(arg.clone(), foldArg.clone())?;
        }
    }
    Ok(foldArg)
}

pub fn toDeclarationStream(mut recordNode: Arc<InstNode::InstNode>, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    node = getDeclarationNode(recordNode.clone(), false)?;
    s = IOStream::append(s.clone(), (indent.clone()).clone())?;
    s = IOStream::append(s.clone(), (InstNode::toString(node.clone())?).clone())?;
    Ok(s)
}

pub fn toFlatDeclarationStream(mut recordNode: Arc<InstNode::InstNode>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    node = getDeclarationNode(recordNode.clone(), true)?;
    s = IOStream::append(s.clone(), (InstNode::toFlatString(node.clone(), format.clone(), (indent.clone()).clone())?).clone())?;
    Ok(s)
}

pub fn getDeclarationNode(mut recordNode: Arc<InstNode::InstNode>, mut evaluate: bool) -> Result<Arc<InstNode::InstNode>> {
    let mut declNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut node_ty: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
    node_ty = InstNode::nodeType(recordNode.clone())?;
    declNode = instRecord(recordNode.clone(), InstContext::NO_CONTEXT.clone())?;
    Typing::typeClass(declNode.clone(), InstContext::RELAXED.clone())?;
    declNode = InstNode::setNodeType(node_ty.clone(), declNode.clone());
    if evaluate.clone() {
        EvalConstants::evaluateRecordDeclaration(declNode.clone())?;
    }
    Ok(declNode)
}

