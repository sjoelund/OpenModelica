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

use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFRecord as Record;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_util::Error;

pub fn instConstructor(mut path: Arc<Absyn::Path>, mut recordNode: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<InstNode::InstNode>> {
    let mut recordNode: Arc<InstNode::InstNode> = recordNode;
    let mut ctor_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ctor_path: Arc<Absyn::Path>;
    let mut ctor_overloaded: bool = false;
    let mut ctor_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    match '__try0: {
        ctor_ref = unwrap_break_err!(Function::lookupFunctionSimple((literal!("'constructor'")).clone(), recordNode.clone(), context.clone()), '__try0);
        ctor_overloaded = true;
        Ok::<_, anyhow::Error>((ctor_overloaded.clone(),))
    } {
        Ok((__try0_o0,)) => {
            ctor_overloaded = __try0_o0;
        }
        Err(_) => {
            ctor_overloaded = false;
        }
    }
    if ctor_overloaded.clone() {
        (_, ctor_node, _) = Function::instFunctionRef(ctor_ref.clone(), context.clone(), info.clone())?;
        ctor_path = InstNode::fullPath(ctor_node.clone(), false);
        for mut f in &*Function::getCachedFuncs(ctor_node.clone())? {
            let mut f = f.clone();
            checkOperatorConstructorOutput(f.clone(), Class::lastBaseClass(recordNode.clone()), ctor_path.clone(), info.clone())?;
            recordNode = InstNode::cacheAddFunc(recordNode.clone(), f.clone(), false)?;
        }
    }
    recordNode = Record::instDefaultConstructor(path.clone(), recordNode.clone(), context.clone(), info.clone())?;
    Ok(recordNode)
}

pub fn instOperatorFunctions(mut node: Arc<InstNode::InstNode>, mut context: i32, mut info: SourceInfo) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut mclss: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut allfuncs: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut funcs: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    checkOperatorRestrictions(node.clone())?;
    tree = Class::classTree(InstNode::getClass(node.clone())?)?;
    let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ClassTree::FLAT_TREE { classes: mclss, .. } => {
            let __range0 = mclss.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut op in __range0 {
                Function::instFunctionNode(op.clone(), context.clone(), info.clone())?;
                funcs = Function::getCachedFuncs(op.clone())?;
                allfuncs = listAppend(funcs.clone(), allfuncs.clone());
            }
            for mut f in &*allfuncs.clone() {
                let mut f = f.clone();
                node = InstNode::cacheAddFunc(node.clone(), f.clone(), false)?;
            }
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperatorOverloading.instOperatorFunctions")); __mm_s.push_str(&*literal!(" got non-instantiated function")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(node)
}

pub fn checkOperatorRestrictions(mut operatorNode: Arc<InstNode::InstNode>) -> Result<()> {
    if !(SCodeUtil::isElementEncapsulated(InstNode::definition(operatorNode.clone())?)) {
        Error::addSourceMessage(Error::OPERATOR_NOT_ENCAPSULATED.clone(), list![(AbsynUtil::pathString(InstNode::fullPath(operatorNode.clone(), false), (literal!(".")).clone(), true, false)?).clone()], InstNode::info(operatorNode.clone())?)?;
        bail!("fail");
    }
    Ok(())
}

pub fn lookupOperatorFunctionsInType(mut operatorName: ArcStr, mut ty: Arc<Type::NFType>) -> Result<Arc<metamodelica::List<Arc<Function::Function>>>> {
    let mut functions: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut fn_ref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut is_defined: bool = false;
    functions = (::match_deref::match_deref! { match &(Type::arrayElementType(ty.clone())) {
        Deref @ Type::COMPLEX { cls: node, .. } => {
            match '__try0: {
                fn_ref = unwrap_break_err!(Function::lookupFunctionSimple((operatorName.clone()).clone(), node.clone(), InstContext::NO_CONTEXT.clone()), '__try0);
                is_defined = true;
                Ok::<_, anyhow::Error>((is_defined.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    is_defined = __try0_o0;
                }
                Err(_) => {
                    is_defined = false;
                }
            }
            if is_defined.clone() {
                (fn_ref, _, _) = Function::instFunctionRef(fn_ref.clone(), InstContext::NO_CONTEXT.clone(), InstNode::info(node.clone())?)?;
                functions = Function::typeRefCache(fn_ref.clone(), InstContext::FUNCTION.clone())?;
            } else {
                functions = metamodelica::nil();
            }
            functions.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(functions)
}

pub fn patchOperatorRecordConstructorBinding(mut r#fn: Arc<Function::Function>) -> Result<Arc<Function::Function>> {
    let mut r#fn: Arc<Function::Function> = r#fn;
    let mut output_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut output_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut output_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    if (r#fn.outputs.clone().len() as i32) != 1 {
        return Ok(r#fn.clone());
    }
    output_node = listHead(r#fn.outputs.clone())?;
    output_comp = InstNode::component(output_node.clone())?;
    output_binding = Component::getBinding(output_comp.clone());
    if !(Binding::isBound(output_binding.clone())) {
        return Ok(r#fn.clone());
    }
    output_binding = Binding::mapExp(output_binding.clone(), Arc::new({ let __pe_b1 = r#fn.clone(); move |__pe_a0| Ok(patchOperatorRecordConstructorBinding_traverser(__pe_a0, __pe_b1.clone())) }))?;
    output_comp = Component::setBinding(output_binding.clone(), output_comp.clone())?;
    output_node = InstNode::updateComponent(output_comp.clone(), output_node.clone())?;
    Ok(r#fn)
}

fn checkOperatorConstructorOutput(mut r#fn: Arc<Function::Function>, mut recordNode: Arc<InstNode::InstNode>, mut path: Arc<Absyn::Path>, mut info: SourceInfo) -> Result<()> {
    let mut output_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut output_ty: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if (r#fn.outputs.clone().len() as i32) != 1 {
        Error::addSourceMessage(Error::OPERATOR_OVERLOADING_ONE_OUTPUT_ERROR.clone(), list![(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone()], info.clone())?;
        bail!("fail");
    }
    output_node = listHead(r#fn.outputs.clone())?;
    output_ty = InstNode::classScope(output_node.clone());
    if !(InstNode::isSame(output_ty.clone(), recordNode.clone())) {
        Error::addSourceMessage(Error::OPERATOR_OVERLOADING_INVALID_OUTPUT_TYPE.clone(), list![(InstNode::name(output_node.clone())?).clone(), (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), (InstNode::name(recordNode.clone())?).clone(), (InstNode::name(output_ty.clone())?).clone()], info.clone())?;
        bail!("fail");
    }
    Ok(())
}

fn patchOperatorRecordConstructorBinding_traverser(mut exp: Arc<Expression::NFExpression>, mut constructorFn: Arc<Function::Function>) -> Arc<Expression::NFExpression> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: args, ty, r#fn, .. } } if (referenceEq(&constructorFn.node.clone(),&r#fn.node.clone())) => Expression::makeRecord(Function::name(constructorFn.clone()), ty.clone(), args.clone()),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

