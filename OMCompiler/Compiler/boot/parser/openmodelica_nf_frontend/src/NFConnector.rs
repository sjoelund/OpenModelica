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

use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Variability;
use crate::NFRestriction as Restriction;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NFConnector {
    pub name: Arc<ComponentRef::NFComponentRef>,
    pub ty: Arc<Type::NFType>,
    pub face: Face,
    pub cty: i32,
    pub source: Arc<DAE::ElementSource>,
}

impl Default for NFConnector {
    fn default() -> Self {
        Self {
            name: Default::default(),
            ty: Default::default(),
            face: Default::default(),
            cty: Default::default(),
            source: Default::default(),
        }
    }
}

pub type CONNECTOR = NFConnector;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Face {
    INSIDE = 1,
    OUTSIDE = 2,
}
impl PartialOrd for Face {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Face {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for Face {
    fn default() -> Self { Self::INSIDE }
}

pub fn fromCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut source: Arc<DAE::ElementSource>) -> Arc<NFConnector> {
    let mut conn: Arc<NFConnector> = fromFacedCref(cref.clone(), ty.clone(), crefFace(cref.clone()).unwrap(), source.clone()).unwrap();
    conn
}

pub fn fromFacedCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut face: Face, mut source: Arc<DAE::ElementSource>) -> Result<Arc<NFConnector>> {
    let mut conn: Arc<NFConnector> = Arc::new(<NFConnector as ::std::default::Default>::default());
    let mut node: Arc<InstNode::InstNode> = ComponentRef::node(cref.clone())?;
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut cty: i32 = 0;
    let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    if InstNode::isComponent(node.clone()) {
        comp = InstNode::component(node.clone())?;
        res = Class::restriction(InstNode::getClass(Component::classInstance(comp.clone()))?);
        cty = Component::connectorType(comp.clone());
    } else {
        cty = intBitOr(ConnectorType::VIRTUAL.clone(), ConnectorType::POTENTIAL.clone());
    }
    conn = Arc::new(NFConnector { name: ComponentRef::simplifySubscripts(cref.clone(), false)?, ty: ty.clone(), face: face.clone(), cty: cty.clone(), source: source.clone() });
    Ok(conn)
}

pub fn fromExp(mut exp: Arc<Expression::NFExpression>, mut source: Arc<DAE::ElementSource>, mut conns: Arc<metamodelica::List<Arc<NFConnector>>>) -> Result<Arc<metamodelica::List<Arc<NFConnector>>>> {
    let mut conns: Arc<metamodelica::List<Arc<NFConnector>>> = conns;
    conns = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => cons(fromCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), source.clone()), conns.clone()),
        Deref @ Expression::ARRAY { .. } => {
            let __range0 = (1..=(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                conns = fromExp(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow()[(i.clone()-1) as usize].clone(), source.clone(), conns.clone())?;
            }
            conns.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConnector.fromExp")); __mm_s.push_str(&*literal!(" got unknown expression ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(conns)
}

pub fn getType(mut conn: Arc<NFConnector>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType> = conn.ty.clone();
    ty
}

pub fn getInfo(mut conn: Arc<NFConnector>) -> SourceInfo {
    let mut info: SourceInfo = conn.source.info.clone();
    info
}

pub fn variability(mut conn: Arc<NFConnector>) -> Variability {
    let mut var: Variability = Component::variability(InstNode::component(ComponentRef::node(conn.name.clone()).unwrap()).unwrap());
    var
}

pub fn isEqual(mut conn1: Arc<NFConnector>, mut conn2: Arc<NFConnector>) -> bool {
    let mut isEqual: bool = ComponentRef::isEqual(conn1.name.clone(), conn2.name.clone()).unwrap() && conn1.face.clone() == conn2.face.clone();
    isEqual
}

pub fn isEqualNoSubs(mut conn1: Arc<NFConnector>, mut conn2: Arc<NFConnector>) -> bool {
    let mut isEqual: bool = ComponentRef::isEqualStrip(conn1.name.clone(), conn2.name.clone()).unwrap() && conn1.face.clone() == conn2.face.clone();
    isEqual
}

pub fn isPrefix(mut conn1: Arc<NFConnector>, mut conn2: Arc<NFConnector>) -> bool {
    let mut isPrefix: bool = ComponentRef::isPrefix(conn1.name.clone(), conn2.name.clone()).unwrap();
    isPrefix
}

pub fn isNodeNameEqual(mut conn1: Arc<NFConnector>, mut conn2: Arc<NFConnector>) -> bool {
    let mut isEqual: bool = InstNode::name(ComponentRef::node(conn1.name.clone()).unwrap()).unwrap() == InstNode::name(ComponentRef::node(conn2.name.clone()).unwrap()).unwrap();
    isEqual
}

pub fn isOutside(mut conn: Arc<NFConnector>) -> bool {
    let mut isOutside: bool = false;
    let mut f: Face = conn.face.clone();
    isOutside = f.clone() == Face::OUTSIDE.clone();
    isOutside
}

pub fn isInside(mut conn: Arc<NFConnector>) -> bool {
    let mut isInside: bool = false;
    let mut f: Face = conn.face.clone();
    isInside = f.clone() == Face::INSIDE.clone();
    isInside
}

pub fn setOutside(mut conn: Arc<NFConnector>) -> Arc<NFConnector> {
    let mut conn: Arc<NFConnector> = conn;
    if conn.face.clone() != Face::OUTSIDE.clone() {
        assign_field!(conn.face = Face::OUTSIDE.clone());
    }
    conn
}

pub fn isDeleted(mut conn: Arc<NFConnector>) -> bool {
    let mut isDeleted: bool = ComponentRef::isDeleted(conn.name.clone()).unwrap();
    isDeleted
}

pub fn isExpandable(mut conn: Arc<NFConnector>) -> bool {
    let mut isExpandable: bool = ConnectorType::isExpandable(conn.cty.clone());
    isExpandable
}

pub fn isArray(mut conn: Arc<NFConnector>) -> bool {
    let mut isArray: bool = Type::isArray(conn.ty.clone());
    isArray
}

pub fn name(mut conn: Arc<NFConnector>) -> Arc<ComponentRef::NFComponentRef> {
    let mut name: Arc<ComponentRef::NFComponentRef> = conn.name.clone();
    name
}

pub fn toString(mut conn: Arc<NFConnector>) -> ArcStr {
    let mut r#str: ArcStr = ComponentRef::toString(conn.name.clone()).unwrap();
    r#str
}

pub fn faceString(mut conn: Arc<NFConnector>) -> ArcStr {
    let mut r#str: ArcStr = if (conn.face.clone() == Face::INSIDE.clone()) {literal!("inside")} else {literal!("outside")};
    r#str
}

pub fn hash(mut conn: Arc<NFConnector>) -> i32 {
    let mut hash: i32 = ComponentRef::hash(conn.name.clone());
    hash
}

pub fn hashNoSubs(mut conn: Arc<NFConnector>) -> i32 {
    let mut hash: i32 = 0;
    hash = ComponentRef::hashStrip(conn.name.clone());
    hash
}

pub fn split(mut conn: Arc<NFConnector>) -> Result<Arc<metamodelica::List<Arc<NFConnector>>>> {
    let mut connl: Arc<metamodelica::List<Arc<NFConnector>>> = metamodelica::nil();
    connl = splitImpl(conn.name.clone(), conn.ty.clone(), conn.face.clone(), conn.source.clone(), conn.cty.clone(), metamodelica::nil(), metamodelica::nil())?;
    connl = connl.clone().reverse();
    Ok(connl)
}

pub fn scalarize(mut conn: Arc<NFConnector>) -> Result<Arc<metamodelica::List<Arc<NFConnector>>>> {
    let mut connl: Arc<metamodelica::List<Arc<NFConnector>>> = metamodelica::nil();
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut face: Face = Face::INSIDE;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut cty: i32 = 0;
    let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(conn.clone()) {
        Deref @ NFConnector { name: __pa0, ty: __pa1, face: __pa2, cty: __pa3, source: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    face = __pa2.clone();
    cty = __pa3.clone();
    source = __pa4.clone();
    names = ComponentRef::scalarizeAll(name.clone(), false)?;
    ty = Type::arrayElementType(ty.clone());
    for mut n in &*names.clone() {
        let mut n = n.clone();
        connl = cons(Arc::new(NFConnector { name: n.clone(), ty: ty.clone(), face: face.clone(), cty: cty.clone(), source: source.clone() }), connl.clone());
    }
    Ok(connl)
}

pub fn scalarizePrefix(mut conn: Arc<NFConnector>) -> Result<Arc<metamodelica::List<Arc<NFConnector>>>> {
    let mut connl: Arc<metamodelica::List<Arc<NFConnector>>> = metamodelica::nil();
    let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut prefix: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut face: Face = Face::INSIDE;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut cty: i32 = 0;
    let mut prefixes: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(conn.clone()) {
        Deref @ NFConnector { name: __pa0, ty: __pa1, face: __pa2, cty: __pa3, source: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    ty = __pa1.clone();
    face = __pa2.clone();
    cty = __pa3.clone();
    source = __pa4.clone();
    prefix = ComponentRef::rest(name.clone())?;
    if ComponentRef::isEmpty(prefix.clone()) {
        connl = list![conn.clone()];
        return Ok(connl.clone());
    }
    prefixes = ComponentRef::scalarizeAll(prefix.clone(), false)?;
    ty = ComponentRef::getSubscriptedType(ComponentRef::first(name.clone()), false)?;
    for mut p in &*prefixes.clone() {
        let mut p = p.clone();
        name = ComponentRef::prepend(p.clone(), name.clone())?;
        connl = cons(Arc::new(NFConnector { name: name.clone(), ty: ty.clone(), face: face.clone(), cty: cty.clone(), source: source.clone() }), connl.clone());
    }
    connl = connl.clone().reverse();
    Ok(connl)
}

pub fn addSubscripts(mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut conn: Arc<NFConnector>) -> Result<Arc<NFConnector>> {
    let mut conn: Arc<NFConnector> = conn;
    assign_field!(
        conn.name = ComponentRef::mergeSubscripts(subscripts.clone(), conn.name.clone(), true, false, false)?,
        conn.ty = Type::subscript(conn.ty.clone(), subscripts.clone(), true)?
    );
    Ok(conn)
}

fn crefFace(mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<Face> {
    let mut face: Face = Face::INSIDE;
    face = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { restCref: Deref @ ComponentRef::EMPTY, .. } => Face::OUTSIDE.clone(),
        _ => if (InstNode::isConnector(ComponentRef::node(ComponentRef::firstNonScope(cref.clone()))?)?) {Face::OUTSIDE.clone()} else {Face::INSIDE.clone()},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(face)
}

fn splitImpl(mut name: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut face: Face, mut source: Arc<DAE::ElementSource>, mut cty: i32, mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut conns: Arc<metamodelica::List<Arc<NFConnector>>>) -> Result<Arc<metamodelica::List<Arc<NFConnector>>>> {
    let mut conns: Arc<metamodelica::List<Arc<NFConnector>>> = conns;
    let mut ct: Arc<ComplexType::NFComplexType> = Arc::new(ComplexType::CLASS);
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    conns = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::COMPLEX { complexTy: ct @ Deref @ ComplexType::CONNECTOR { .. }, .. } => {
            conns = splitImpl2(name.clone(), face.clone(), source.clone(), var_field!((**ct).potentials, ComplexType::NFComplexType::CONNECTOR).clone(), dims.clone(), conns.clone())?;
            conns = splitImpl2(name.clone(), face.clone(), source.clone(), var_field!((**ct).flows, ComplexType::NFComplexType::CONNECTOR).clone(), dims.clone(), conns.clone())?;
            conns = splitImpl2(name.clone(), face.clone(), source.clone(), var_field!((**ct).streams, ComplexType::NFComplexType::CONNECTOR).clone(), dims.clone(), conns.clone())?;
            conns.clone()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => cons(Arc::new(NFConnector { name: name.clone(), ty: Type::liftArrayLeftList(ty.clone(), dims.clone()), face: face.clone(), cty: cty.clone(), source: source.clone() }), conns.clone()),
        Deref @ Type::COMPLEX { .. } => {
            tree = Class::classTree(InstNode::getClass(var_field!((*ty).cls, Type::NFType::COMPLEX).clone())?)?;
            conns = splitImpl2(name.clone(), face.clone(), source.clone(), Arc::new(ClassTree::getComponents(tree.clone())?.borrow().iter().cloned().collect::<metamodelica::List<_>>()), dims.clone(), conns.clone())?;
            conns.clone()
        },
        Deref @ Type::ARRAY { .. } => splitImpl(name.clone(), var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), face.clone(), source.clone(), cty.clone(), listAppend(dims.clone(), var_field!((*ty).dimensions, Type::NFType::ARRAY).clone()), conns.clone())?,
        _ => cons(Arc::new(NFConnector { name: name.clone(), ty: Type::liftArrayLeftList(ty.clone(), dims.clone()), face: face.clone(), cty: cty.clone(), source: source.clone() }), conns.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(conns)
}

fn splitImpl2(mut name: Arc<ComponentRef::NFComponentRef>, mut face: Face, mut source: Arc<DAE::ElementSource>, mut comps: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>>, mut conns: Arc<metamodelica::List<Arc<NFConnector>>>) -> Result<Arc<metamodelica::List<Arc<NFConnector>>>> {
    let mut conns: Arc<metamodelica::List<Arc<NFConnector>>> = conns;
    let mut c: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut cty: i32 = 0;
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        c = InstNode::component(comp.clone())?;
        ty = Component::getType(c.clone())?;
        cty = Component::connectorType(c.clone());
        if !(ConnectorType::isPotentiallyPresent(cty.clone())) {
            cref = ComponentRef::append(ComponentRef::fromNode(comp.clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone()), name.clone())?;
            conns = splitImpl(cref.clone(), ty.clone(), face.clone(), source.clone(), cty.clone(), dims.clone(), conns.clone())?;
        }
    }
    Ok(conns)
}


