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

use crate::NFAttributes;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Visibility;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn elaborate(mut flatModel: Arc<FlatModel::NFFlatModel>, mut connections: Arc<Connections::NFConnections>) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<Connections::NFConnections>)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut connections: Arc<Connections::NFConnections> = connections;
    let mut expandable_conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = metamodelica::nil();
    let mut undeclared_conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = metamodelica::nil();
    let mut conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut csets: ConnectionSets::Sets = <ConnectionSets::Sets as ::std::default::Default>::default();
    let mut csets_array: metamodelica::Array<Arc<metamodelica::List<Arc<Connector::NFConnector>>>>;
    (expandable_conns, undeclared_conns, conns) = sortConnections(connections.connections.clone())?;
    if expandable_conns.clone().is_empty() && undeclared_conns.clone().is_empty() {
        return Ok((flatModel, connections));
    }
    csets = ConnectionSets::emptySets((expandable_conns.clone().len() as i32) + (undeclared_conns.clone().len() as i32));
    csets = addExpandableConnectorsToSets(expandable_conns.clone(), csets.clone())?;
    (undeclared_conns, csets) = List::mapFold(undeclared_conns.clone(), (std::sync::Arc::new(addUndeclaredConnectorToSets) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connection::NFConnection>, ConnectionSets::Sets) -> Result<(Arc<Connection::NFConnection>, ConnectionSets::Sets)> + 'static>), csets.clone());
    (csets_array, _) = ConnectionSets::extractSets(csets.clone());
    vars = flatModel.variables.clone();
    let __range0 = csets_array.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut set in __range0 {
        vars = elaborateExpandableSet(set.clone(), vars.clone())?;
    }
    conns = List::fold(undeclared_conns.clone(), (std::sync::Arc::new(fnptr!(updateUndeclaredConnection, Arc<Connection::NFConnection>, Arc<metamodelica::List<Arc<Connection::NFConnection>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connection::NFConnection>, Arc<metamodelica::List<Arc<Connection::NFConnection>>>) -> Result<Arc<metamodelica::List<Arc<Connection::NFConnection>>>> + 'static>), conns.clone());
    conns = List::fold(expandable_conns.clone(), (std::sync::Arc::new(updateExpandableConnection) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connection::NFConnection>, Arc<metamodelica::List<Arc<Connection::NFConnection>>>) -> Result<Arc<metamodelica::List<Arc<Connection::NFConnection>>>> + 'static>), conns.clone());
    assign_field!(connections.connections = conns.clone());
    vars = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = updatePotentiallyPresentVariable(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    assign_field!(flatModel.variables = vars.clone());
    Ok((flatModel, connections))
}

fn sortConnections(mut conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>>) -> Result<(Arc<metamodelica::List<Arc<Connection::NFConnection>>>, Arc<metamodelica::List<Arc<Connection::NFConnection>>>, Arc<metamodelica::List<Arc<Connection::NFConnection>>>)> {
    let mut expandableConnections: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = metamodelica::nil();
    let mut undeclaredConnections: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = metamodelica::nil();
    let mut normalConnections: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = metamodelica::nil();
    let mut c1: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut c2: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut err_msg: Option<(ErrorTypes::Message, Arc<metamodelica::List<Arc<Connector::NFConnector>>>)> = None;
    let mut is_undeclared1: bool = false;
    let mut is_undeclared2: bool = false;
    let mut is_expandable1: bool = false;
    let mut is_expandable2: bool = false;
    for mut conn in &*conns.clone() {
        let mut conn = conn.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(conn.clone()) {
            Deref @ Connection::CONNECTION { rhs: __pa0, lhs: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        c2 = __pa0.clone();
        c1 = __pa1.clone();
        is_undeclared1 = Prefixes::ConnectorType::isUndeclared(c1.cty.clone());
        is_undeclared2 = Prefixes::ConnectorType::isUndeclared(c2.cty.clone());
        is_expandable1 = Prefixes::ConnectorType::isExpandable(c1.cty.clone());
        is_expandable2 = Prefixes::ConnectorType::isExpandable(c2.cty.clone());
        if is_expandable1.clone() || is_expandable2.clone() {
            if is_expandable1.clone() && is_expandable2.clone() {
                expandableConnections = cons(conn.clone(), expandableConnections.clone());
            } else {
                Error::addSourceMessageAndFail(Error::EXPANDABLE_NON_EXPANDABLE_CONNECTION.clone(), list![(Connector::toString(if (is_expandable1.clone()) {c1.clone()} else {c2.clone()})).clone(), (Connector::toString(if (is_expandable1.clone()) {c2.clone()} else {c1.clone()})).clone()], Connector::getInfo(c1.clone()))?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
        } else if is_undeclared1.clone() || is_undeclared2.clone() {
            if is_undeclared1.clone() && is_undeclared2.clone() {
                Error::addSourceMessageAndFail(Error::UNDECLARED_CONNECTION.clone(), list![(Connector::toString(c1.clone())).clone(), (Connector::toString(c2.clone())).clone()], Connector::getInfo(c1.clone()))?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            } else {
                undeclaredConnections = cons(conn.clone(), undeclaredConnections.clone());
            }
        } else {
            normalConnections = cons(conn.clone(), normalConnections.clone());
        }
    }
    normalConnections = normalConnections.clone().reverse();
    Ok((expandableConnections, undeclaredConnections, normalConnections))
}

fn addExpandableConnectorsToSets(mut conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>>, mut csets: ConnectionSets::Sets) -> Result<ConnectionSets::Sets> {
    let mut csets: ConnectionSets::Sets = csets;
    let mut c1: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut c2: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    for mut conn in &*conns.clone() {
        let mut conn = conn.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(conn.clone()) {
            Deref @ Connection::CONNECTION { rhs: __pa0, lhs: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        c2 = __pa0.clone();
        c1 = __pa1.clone();
        csets = addConnectionToSets(c1.clone(), c2.clone(), csets.clone())?;
        csets = addNestedExpandableConnectorsToSets(c1.clone(), c2.clone(), csets.clone())?;
    }
    Ok(csets)
}

fn addNestedExpandableConnectorsToSets(mut c1: Arc<Connector::NFConnector>, mut c2: Arc<Connector::NFConnector>, mut csets: ConnectionSets::Sets) -> Result<ConnectionSets::Sets> {
    let mut csets: ConnectionSets::Sets = csets;
    let mut ecl1: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut ecl2: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut oec: Option<Arc<Connector::NFConnector>> = None;
    let mut conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = metamodelica::nil();
    ecl1 = getExpandableConnectorsInConnector(c1.clone())?;
    ecl2 = getExpandableConnectorsInConnector(c2.clone())?;
    if ecl1.clone().is_empty() && ecl2.clone().is_empty() {
        return Ok(csets);
    }
    for mut ec1 in &*ecl1.clone() {
        let mut ec1 = ec1.clone();
        (ecl2, oec) = List::deleteMemberOnTrue(ec1.clone(), ecl2.clone(), (std::sync::Arc::new(fnptr!(Connector::isNodeNameEqual, Arc<Connector::NFConnector>, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Connector::NFConnector>) -> Result<bool> + 'static>))?;
        if isSome(oec.clone()) {
            conns = cons(Arc::new(Connection::NFConnection { lhs: ec1.clone(), rhs: Util::getOption(oec.clone())? }), conns.clone());
        }
    }
    csets = addExpandableConnectorsToSets(conns.clone(), csets.clone())?;
    Ok(csets)
}

fn getExpandableConnectorsInConnector(mut c1: Arc<Connector::NFConnector>) -> Result<Arc<metamodelica::List<Arc<Connector::NFConnector>>>> {
    let mut ecl: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut par_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ecl = (::match_deref::match_deref! { match &(c1.clone()) {
        Deref @ Connector::CONNECTOR { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXPANDABLE_CONNECTOR { expandableConnectors: nodes, .. }, .. }, name: par_name, .. } => {
            ecl = metamodelica::nil();
            for mut n in &*nodes.clone() {
                let mut n = n.clone();
                ty = InstNode::getType(n.clone())?;
                name = ComponentRef::prefixCref(n.clone(), ty.clone(), metamodelica::nil(), par_name.clone());
                ecl = cons(Connector::fromCref(name.clone(), ty.clone(), ElementSource::createElementSource(InstNode::info(n.clone())?, None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?), ecl.clone());
            }
            ecl.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ecl)
}

fn addUndeclaredConnectorToSets(mut conn: Arc<Connection::NFConnection>, mut csets: ConnectionSets::Sets) -> Result<(Arc<Connection::NFConnection>, ConnectionSets::Sets)> {
    let mut conn: Arc<Connection::NFConnection> = conn;
    let mut csets: ConnectionSets::Sets = csets;
    let mut c1: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut c2: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut c: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut ec: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(conn.clone()) {
        Deref @ Connection::CONNECTION { rhs: __pa0, lhs: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    c2 = __pa0.clone();
    c1 = __pa1.clone();
    if Prefixes::ConnectorType::isUndeclared(c1.cty.clone()) {
        if Prefixes::ConnectorType::isVirtual(c1.cty.clone()) {
            c1 = makeVirtualConnector(c1.clone(), c2.clone())?;
            conn = Arc::new(Connection::NFConnection { lhs: c1.clone(), rhs: c2.clone() });
        }
        c = c1.clone();
    } else {
        if Prefixes::ConnectorType::isVirtual(c2.cty.clone()) {
            c2 = makeVirtualConnector(c2.clone(), c1.clone())?;
            conn = Arc::new(Connection::NFConnection { lhs: c1.clone(), rhs: c2.clone() });
        }
        c = c2.clone();
    }
    ec = Arc::new(Connector::NFConnector { name: ComponentRef::rest(c.name.clone())?, ty: c.ty.clone(), face: c.face.clone(), cty: ConnectorType::EXPANDABLE.clone(), source: c.source.clone() });
    csets = addConnectionToSets(c.clone(), ec.clone(), csets.clone())?;
    Ok((conn, csets))
}

fn addConnectionToSets(mut c1: Arc<Connector::NFConnector>, mut c2: Arc<Connector::NFConnector>, mut csets: ConnectionSets::Sets) -> Result<ConnectionSets::Sets> {
    let mut csets: ConnectionSets::Sets = csets;
    csets = ConnectionSets::merge(Connector::setOutside(c1.clone()), Connector::setOutside(c2.clone()), csets.clone())?;
    Ok(csets)
}

fn makeVirtualConnector(mut virtualConnector: Arc<Connector::NFConnector>, mut normalConnector: Arc<Connector::NFConnector>) -> Result<Arc<Connector::NFConnector>> {
    let mut newConnector: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut virtual_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut normal_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    virtual_cref = virtualConnector.name.clone();
    normal_cref = normalConnector.name.clone();
    ty = normalConnector.ty.clone();
    node = ComponentRef::node(normal_cref.clone())?;
    node = InstNode::clone(node.clone())?;
    node = InstNode::rename((ComponentRef::firstName(virtual_cref.clone(), false)?).clone(), node.clone())?;
    node = InstNode::setParent(ComponentRef::node(ComponentRef::rest(virtual_cref.clone())?)?, node.clone())?;
    node = InstNode::componentApply(node.clone(), (std::sync::Arc::new(Component::setType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), ty.clone())?;
    virtual_cref = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), ComponentRef::rest(virtual_cref.clone())?);
    newConnector = Arc::new(Connector::NFConnector { name: virtual_cref.clone(), ty: ty.clone(), face: virtualConnector.face.clone(), cty: virtualConnector.cty.clone(), source: virtualConnector.source.clone() });
    Ok(newConnector)
}

fn elaborateExpandableSet(mut set: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<metamodelica::List<Arc<Variable::NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut exp_set: Arc<UnorderedSet::UnorderedSet<Arc<Connector::NFConnector>>>;
    let mut exp_conns: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut exp_set_lst: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    exp_set = UnorderedSet::new((std::sync::Arc::new(hashConnector) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(Connector::isNodeNameEqual, Arc<Connector::NFConnector>, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>, Arc<Connector::NFConnector>) -> Result<bool> + 'static>), 13);
    for mut c in &*set.clone() {
        let mut c = c.clone();
        if Prefixes::ConnectorType::isExpandable(c.cty.clone()) {
            exp_conns = cons(c.clone(), exp_conns.clone());
        } else if Prefixes::ConnectorType::isUndeclared(c.cty.clone()) {
            UnorderedSet::add(c.clone(), exp_set.clone())?;
            markComponentPresent(ComponentRef::node(Connector::name(c.clone()))?)?;
        }
    }
    exp_set_lst = UnorderedSet::toList(exp_set.clone());
    for mut ec in &*exp_conns.clone() {
        let mut ec = ec.clone();
        vars = augmentExpandableConnector(ec.clone(), exp_set_lst.clone(), vars.clone())?;
    }
    Ok(vars)
}

fn markComponentPresent(mut node: Arc<InstNode::InstNode>) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut cty: i32 = 0;
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    comp = InstNode::component(node.clone())?;
    cty = Component::connectorType(comp.clone());
    if Prefixes::ConnectorType::isPotentiallyPresent(cty.clone()) {
        cty = Prefixes::ConnectorType::setPresent(cty.clone());
        comp = Component::setConnectorType(cty.clone(), comp.clone());
        InstNode::updateComponent(comp.clone(), node.clone())?;
        if Type::isComplex(Component::getType(comp.clone())?) {
            cls = InstNode::getClass(Component::classInstance(comp.clone()))?;
            ClassTree::applyComponents(Class::classTree(cls.clone())?, (std::sync::Arc::new(markComponentPresent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>));
        }
    }
    Ok(())
}

fn augmentExpandableConnector(mut conn: Arc<Connector::NFConnector>, mut expandableSet: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<metamodelica::List<Arc<Variable::NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut exp_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut elem_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut exp_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comp_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut complex_ty: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    exp_name = Connector::name(conn.clone());
    exp_node = ComponentRef::node(exp_name.clone())?;
    if InstNode::isName(exp_node.clone()) {
        Error::addInternalError((literal!("Augmenting a virtual element in an expandable connector is not yet supported.")).clone(), Connector::getInfo(conn.clone()))?;
        bail!("fail");
    }
    cls_node = InstNode::classScope(exp_node.clone());
    cls_node = InstNode::clone(cls_node.clone())?;
    cls = InstNode::getClass(cls_node.clone())?;
    cls_tree = Class::classTree(cls.clone())?;
    for mut c in &*expandableSet.clone() {
        let mut c = c.clone();
        elem_name = Connector::name(c.clone());
        node = ComponentRef::node(elem_name.clone())?;
        match '__try0: {
            (comp_node, _) = unwrap_break_err!(ClassTree::lookupElement((InstNode::name(node.clone())?).clone(), cls_tree.clone()), '__try0);
            Ok::<_, anyhow::Error>((comp_node.clone(),))
        } {
            Ok((__try0_o0,)) => {
                comp_node = __try0_o0;
            }
            Err(_) => {
                comp_node = Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE);
            }
        }
        if InstNode::isEmpty(comp_node.clone()) {
            nodes = cons(node.clone(), nodes.clone());
            ty = c.ty.clone();
            elem_name = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), exp_name.clone());
            vars = createVirtualVariables(elem_name.clone(), ty.clone(), ElementSource::getInfo(c.source.clone()), vars.clone())?;
        } else {
            comp_node = InstNode::resolveInner(comp_node.clone());
            if InstNode::isComponent(comp_node.clone()) {
                markComponentPresent(comp_node.clone())?;
            } else {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpandableConnectors.augmentExpandableConnector")); __mm_s.push_str(&*literal!(" got non-component element")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
        }
    }
    if !(nodes.clone().is_empty()) {
        cls_tree = ClassTree::addElementsToFlatTree(nodes.clone(), cls_tree.clone())?;
        cls = Class::setClassTree(cls_tree.clone(), cls.clone())?;
    }
    complex_ty = Typing::makeConnectorType(cls_tree.clone(), false)?;
    ty = Arc::new(Type::NFType::COMPLEX { cls: cls_node.clone(), complexTy: complex_ty.clone() });
    ty = Type::liftArrayLeftList(ty.clone(), Type::arrayDims(InstNode::getType(exp_node.clone())?));
    cls = Class::setType(ty.clone(), cls.clone())?;
    InstNode::updateClass(cls.clone(), cls_node.clone())?;
    InstNode::componentApply(exp_node.clone(), (std::sync::Arc::new(Component::setType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>, Arc<Component::NFComponent>) -> Result<Arc<Component::NFComponent>> + 'static>), ty.clone())?;
    Ok(vars)
}

fn createVirtualVariables(mut connectorName: Arc<ComponentRef::NFComponentRef>, mut connectorType: Arc<Type::NFType>, mut info: SourceInfo, mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<metamodelica::List<Arc<Variable::NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = vars;
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    if Type::isComplex(connectorType.clone()) {
        let __range0 = Type::complexComponents(connectorType.clone())?.borrow().iter().cloned().collect::<Vec<_>>();
        for mut comp in __range0 {
            ty = InstNode::getType(comp.clone())?;
            name = ComponentRef::prefixCref(comp.clone(), ty.clone(), metamodelica::nil(), connectorName.clone());
            vars = createVirtualVariables(name.clone(), ty.clone(), info.clone(), vars.clone())?;
        }
    } else {
        var = Arc::new(Variable::NFVariable { name: connectorName.clone(), ty: connectorType.clone(), binding: Binding::EMPTY_BINDING().clone(), visibility: Visibility::PUBLIC.clone(), attributes: NFAttributes::AUGMENTED_ATTR().clone(), typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: Arc::new(SCode::Comment { annotation_: None, comment: Some((literal!("virtual variable in expandable connector")).clone()) }), info: info.clone(), backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone() });
        vars = cons(var.clone(), vars.clone());
    }
    Ok(vars)
}

fn updateUndeclaredConnection(mut conn: Arc<Connection::NFConnection>, mut conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>>) -> Arc<metamodelica::List<Arc<Connection::NFConnection>>> {
    let mut conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = conns;
    conns = cons(conn.clone(), conns.clone());
    conns
}

fn updateExpandableConnection(mut conn: Arc<Connection::NFConnection>, mut conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>>) -> Result<Arc<metamodelica::List<Arc<Connection::NFConnection>>>> {
    let mut conns: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = conns;
    let mut c1: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut c2: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mk: MatchKind = MatchKind::EXACT;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(conn.clone()) {
        Deref @ Connection::CONNECTION { rhs: __pa0, lhs: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    c2 = __pa0.clone();
    c1 = __pa1.clone();
    (c1, ty1) = updateExpandableConnector(c1.clone())?;
    (c2, ty2) = updateExpandableConnector(c2.clone())?;
    e1 = Arc::new(Expression::NFExpression::CREF { ty: ty1.clone(), cref: Connector::name(c1.clone()) });
    e2 = Arc::new(Expression::NFExpression::CREF { ty: ty2.clone(), cref: Connector::name(c2.clone()) });
    (_, _, _, mk) = TypeCheck::matchExpressions(e1.clone(), ty1.clone(), e2.clone(), ty2.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
    if TypeCheck::isIncompatibleMatch(mk.clone()) {
        Error::addSourceMessageAndFail(Error::CONNECT_TYPE_MISMATCH.clone(), list![(Expression::toString(e1.clone())?).clone(), (Expression::toString(e2.clone())?).clone()], Connector::getInfo(c1.clone()))?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    conns = cons(Arc::new(Connection::NFConnection { lhs: c1.clone(), rhs: c2.clone() }), conns.clone());
    Ok(conns)
}

fn updateExpandableConnector(mut conn: Arc<Connector::NFConnector>) -> Result<(Arc<Connector::NFConnector>, Arc<Type::NFType>)> {
    let mut conn: Arc<Connector::NFConnector> = conn;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(conn.clone()) {
        Deref @ Connector::CONNECTOR { ty: __pa0, name: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    name = __pa1.clone();
    name = ComponentRef::updateNodeType(name.clone())?;
    ty = Type::setArrayElementType(ty.clone(), Type::arrayElementType(ComponentRef::nodeType(name.clone())?));
    conn = Arc::new(Connector::NFConnector { name: name.clone(), ty: ty.clone(), face: conn.face.clone(), cty: conn.cty.clone(), source: conn.source.clone() });
    Ok((conn, ty))
}

fn updatePotentiallyPresentVariable(mut var: Arc<Variable::NFVariable>) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    if Prefixes::ConnectorType::isPotentiallyPresent(var.attributes.connectorType.clone()) {
        assign_field!(var.attributes = Component::getAttributes(InstNode::component(ComponentRef::node(var.name.clone())?)?));
    }
    Ok(var)
}

fn hashConnector(mut conn: Arc<Connector::NFConnector>) -> Result<i32> {
    let mut res: i32 = 0;
    res = stringHashDjb2((ComponentRef::firstName(conn.name.clone(), false)?).clone());
    Ok(res)
}

