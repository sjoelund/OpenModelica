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

use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnection as Connection;
use crate::NFConnector as Connector;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::ConnectorType;
use crate::NFType as Type;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub struct NFConnections {
    pub connections: Arc<metamodelica::List<Arc<Connection::NFConnection>>>,
    pub flows: Arc<metamodelica::List<Arc<Connector::NFConnector>>>,
    pub broken: BrokenEdges,
}

impl Default for NFConnections {
    fn default() -> Self {
        Self {
            connections: Default::default(),
            flows: Default::default(),
            broken: Default::default(),
        }
    }
}

pub type CONNECTIONS = NFConnections;

#[derive(Clone, Debug, PartialEq)]
pub struct BrokenEdge {
    pub lhs: Arc<ComponentRef::NFComponentRef>,
    pub rhs: Arc<ComponentRef::NFComponentRef>,
    pub source: Arc<DAE::ElementSource>,
    pub brokenEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
}

pub type BROKEN_EDGE = BrokenEdge;


pub type BrokenEdges = Arc<metamodelica::List<BrokenEdge>>;

pub fn new() -> Arc<NFConnections> {
    let mut conns: Arc<NFConnections> = Arc::new(NFConnections { connections: metamodelica::nil(), flows: metamodelica::nil(), broken: metamodelica::nil() });
    conns
}

pub fn fromConnectionList(mut connl: Arc<metamodelica::List<Arc<Connection::NFConnection>>>) -> Arc<NFConnections> {
    let mut conns: Arc<NFConnections>;
    conns = Arc::new(NFConnections { connections: connl.clone(), flows: metamodelica::nil(), broken: metamodelica::nil() });
    conns
}

pub fn addConnection(mut conn: Arc<Connection::NFConnection>, mut conns: Arc<NFConnections>) -> Arc<NFConnections> {
    let mut conns: Arc<NFConnections> = conns;
    assign_field!(conns.connections = cons(conn.clone(), conns.connections.clone()));
    conns
}

pub fn addFlow(mut conn: Arc<Connector::NFConnector>, mut conns: Arc<NFConnections>) -> Arc<NFConnections> {
    let mut conns: Arc<NFConnections> = conns;
    assign_field!(conns.flows = cons(conn.clone(), conns.flows.clone()));
    conns
}

pub fn addBroken(mut broken: BrokenEdges, mut conns: Arc<NFConnections>) -> Arc<NFConnections> {
    let mut conns: Arc<NFConnections> = conns;
    assign_field!(conns.broken = broken.clone());
    conns
}

pub fn collectConnections(mut flatModel: Arc<FlatModel::NFFlatModel>, mut isDeleted: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<NFConnections>)> {
    pub type IsDeleted = fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool>;

    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut conns: Arc<NFConnections> = new();
    let mut lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut source: Arc<DAE::ElementSource>;
    let mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut ty1: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    for mut eq in &*flatModel.equations.clone() {
        let mut eq = eq.clone();
        eql = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::CONNECT { source, rhs: Deref @ Expression::CREF { cref: rhs, ty: ty2 }, lhs: Deref @ Expression::CREF { cref: lhs, ty: ty1 }, .. } => {
            let mut rhs = (*rhs).clone();
            let mut lhs = (*lhs).clone();
            lhs = ComponentRef::evaluateSubscripts(lhs.clone())?;
            rhs = ComponentRef::evaluateSubscripts(rhs.clone())?;
            assign_field!(conns.connections = makeConnections(lhs.clone(), ty1.clone(), rhs.clone(), ty2.clone(), source.clone(), isDeleted.clone(), conns.connections.clone())?);
            eql.clone()
        },
        _ => cons(eq.clone(), eql.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    assign_field!(flatModel.equations = eql.clone().reverse());
    Ok((flatModel, conns))
}

pub fn collectFlows(mut flatModel: Arc<FlatModel::NFFlatModel>, mut conns: Arc<NFConnections>) -> Result<Arc<NFConnections>> {
    let mut conns: Arc<NFConnections> = conns;
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut c: Arc<Connector::NFConnector>;
    let mut src: Arc<DAE::ElementSource>;
    for mut var in &*flatModel.variables.clone() {
        let mut var = var.clone();
        comp = InstNode::component(ComponentRef::node(var.name.clone())?)?;
        if Component::isFlow(comp.clone()) {
            src = ElementSource::createElementSource(Component::info(comp.clone())?, None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?;
            c = Connector::fromFacedCref(var.name.clone(), var.ty.clone(), Connector::Face::INSIDE.clone(), src.clone())?;
            conns = addFlow(c.clone(), conns.clone());
            if ConnectorType::isAugmented(var.attributes.connectorType.clone()) {
                c = Connector::fromFacedCref(var.name.clone(), var.ty.clone(), Connector::Face::OUTSIDE.clone(), src.clone())?;
                conns = addFlow(c.clone(), conns.clone());
            }
        }
    }
    Ok(conns)
}

pub fn makeConnections(mut lhsCref: Arc<ComponentRef::NFComponentRef>, mut lhsType: Arc<Type::NFType>, mut rhsCref: Arc<ComponentRef::NFComponentRef>, mut rhsType: Arc<Type::NFType>, mut source: Arc<DAE::ElementSource>, mut isDeleted: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>, mut connections: Arc<metamodelica::List<Arc<Connection::NFConnection>>>) -> Result<Arc<metamodelica::List<Arc<Connection::NFConnection>>>> {
    pub type IsDeleted = fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool>;

    let mut connections: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = connections;
    let mut cl1: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut cl2: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut c2: Arc<Connector::NFConnector>;
    if isDeleted(lhsCref.clone())? || isDeleted(rhsCref.clone())? {
        return Ok(connections);
    }
    if InstNode::isName(ComponentRef::node(lhsCref.clone())?) || InstNode::isName(ComponentRef::node(rhsCref.clone())?) {
        cl1 = list![Connector::fromCref(lhsCref.clone(), lhsType.clone(), source.clone())];
        cl2 = list![Connector::fromCref(rhsCref.clone(), rhsType.clone(), source.clone())];
    } else {
        cl1 = makeConnectors(lhsCref.clone(), lhsType.clone(), source.clone())?;
        cl2 = makeConnectors(rhsCref.clone(), rhsType.clone(), source.clone())?;
    }
    for mut c1 in &*cl1.clone() {
        let mut c1 = c1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cl2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        c2 = __pa0.clone();
        cl2 = __pa1.clone();
        if !(isDeleted(c1.name.clone())? || isDeleted(c2.name.clone())?) {
            connections = cons(Arc::new(Connection::NFConnection { lhs: c1.clone(), rhs: c2.clone() }), connections.clone());
        }
    }
    Ok(connections)
}

pub fn makeConnectors(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<Connector::NFConnector>>>> {
    let mut connectors: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut cref_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut expanded: bool = false;
    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        connectors = list![Connector::fromCref(cref.clone(), ComponentRef::getSubscriptedType(cref.clone(), false)?, source.clone())];
        return Ok(connectors);
    }
    cref_exp = Arc::new(Expression::NFExpression::CREF { ty: ComponentRef::getSubscriptedType(cref.clone(), false)?, cref: cref.clone() });
    (cref_exp, expanded) = ExpandExp::expand(cref_exp.clone(), false, false)?;
    if expanded.clone() {
        connectors = Connector::fromExp(cref_exp.clone(), source.clone(), metamodelica::nil())?;
    } else {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConnections.makeConnectors")); __mm_s.push_str(&*literal!(" failed to expand connector `")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), ElementSource::getInfo(source.clone()))?;
    }
    Ok(connectors)
}

pub fn split(mut conns: Arc<NFConnections>) -> Arc<NFConnections> {
    let mut conns: Arc<NFConnections> = conns;
    assign_field!(
        conns.flows = List::mapFlat(conns.flows.clone(), Arc::new(Connector::split)),
        conns.connections = List::mapFlat(conns.connections.clone(), Arc::new(Connection::split))
    );
    conns
}

pub fn connectCount(mut conn: Arc<Connector::NFConnector>, mut connectCounts: Arc<UnorderedMap::UnorderedMap<Arc<Connector::NFConnector>, i32>>) -> i32 {
    let mut count: i32 = 0;
    count = UnorderedMap::getOrDefault(conn.clone(), connectCounts.clone(), 0);
    count
}

pub fn scalarize(mut conns: Arc<NFConnections>, mut keepSingleConnectedArrays: bool) -> Result<Arc<NFConnections>> {
    let mut conns: Arc<NFConnections> = conns;
    let mut connect_counts: Arc<UnorderedMap::UnorderedMap<Arc<Connector::NFConnector>, i32>>;
    let mut flows: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut connections: Arc<metamodelica::List<Arc<Connection::NFConnection>>> = metamodelica::nil();
    let mut count: i32 = 0;
    if keepSingleConnectedArrays.clone() {
        connect_counts = analyseArrayConnections(conns.clone())?;
        for mut f in &*conns.flows.clone() {
            let mut f = f.clone();
            count = connectCount(f.clone(), connect_counts.clone());
            if count.clone() == 0 {
                flows = cons(f.clone(), flows.clone());
            } else if count.clone() > 1 || count.clone() == -1 {
                flows = listAppend(Connector::scalarize(f.clone())?, flows.clone());
            }
        }
        for mut c in &*conns.connections.clone() {
            let mut c = c.clone();
            if !(ConnectorType::isStream(c.lhs.cty.clone())) && connectCount(c.lhs.clone(), connect_counts.clone()) == 1 && connectCount(c.rhs.clone(), connect_counts.clone()) == 1 {
                connections = cons(c.clone(), connections.clone());
            } else {
                connections = listAppend(Connection::scalarize(c.clone())?, connections.clone());
            }
        }
        assign_field!(
            conns.flows = flows.clone().reverse(),
            conns.connections = connections.clone().reverse()
        );
    } else {
        assign_field!(
            conns.flows = List::mapFlat(conns.flows.clone(), Arc::new(Connector::scalarize)),
            conns.connections = List::mapFlat(conns.connections.clone(), Arc::new(Connection::scalarize))
        );
    }
    Ok(conns)
}

pub fn analyseArrayConnections(mut conns: Arc<NFConnections>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<Connector::NFConnector>, i32>>> {
    let mut connectCounts: Arc<UnorderedMap::UnorderedMap<Arc<Connector::NFConnector>, i32>>;
    connectCounts = UnorderedMap::new(fnptr!(Connector::hashNoSubs, Arc<Connector::NFConnector>), fnptr!(Connector::isEqualNoSubs, Arc<Connector::NFConnector>, Arc<Connector::NFConnector>), (conns.connections.clone().len() as i32));
    for mut conn in &*conns.connections.clone() {
        let mut conn = conn.clone();
        analyseArrayConnector(conn.lhs.clone(), connectCounts.clone())?;
        analyseArrayConnector(conn.rhs.clone(), connectCounts.clone())?;
    }
    Ok(connectCounts)
}

pub fn analyseArrayConnector(mut conn: Arc<Connector::NFConnector>, mut connectCounts: Arc<UnorderedMap::UnorderedMap<Arc<Connector::NFConnector>, i32>>) -> Result<()> {
    fn update(mut count: Option<i32>) -> i32 {
        let mut outCount: i32 = 0;
        outCount = (match count.clone() {
        Some(mut outCount) => if (outCount.clone() >= 0) {outCount.clone() + 1} else {-1},
        _ => 1,
    });
        outCount
    }

    if Connector::isArray(conn.clone()) {
        UnorderedMap::addUpdate(conn.clone(), Arc::new(fnptr!(update, Option<i32>)), connectCounts.clone())?;
    } else if ComponentRef::hasSubscripts(conn.name.clone()) {
        UnorderedMap::add(conn.clone(), -1, connectCounts.clone())?;
    }
    Ok(())
}

pub fn toString(mut conns: Arc<NFConnections>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    strl = cons(literal!("FLOWS:"), strl.clone());
    for mut f in &*conns.flows.clone() {
        let mut f = f.clone();
        strl = cons(Connector::toString(f.clone()), strl.clone());
    }
    strl = cons(literal!("\nCONNECTIONS:"), strl.clone());
    for mut c in &*conns.connections.clone() {
        let mut c = c.clone();
        strl = cons(Connection::toString(c.clone()), strl.clone());
    }
    strl = strl.clone().reverse();
    r#str = stringDelimitList(strl.clone(), (literal!("\n")).clone());
    r#str
}

pub fn toStringList(mut conns: Arc<NFConnections>) -> Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> {
    let mut strl: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    strl = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut c in (conns.connections.clone()).into_iter().cloned() {
            let __x = list![(Connector::toString(c.lhs.clone())).clone(), (Connector::toString(c.rhs.clone())).clone()];
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    strl
}


