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

use crate::NFConnector as Connector;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NFConnection {
    pub lhs: Arc<Connector::NFConnector>,
    pub rhs: Arc<Connector::NFConnector>,
}

pub type CONNECTION = NFConnection;

pub fn split(mut conn: Arc<NFConnection>) -> Result<Arc<metamodelica::List<Arc<NFConnection>>>> {
    let mut conns: Arc<metamodelica::List<Arc<NFConnection>>> = metamodelica::nil();
    let mut cls: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut crs: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut cr: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    cls = Connector::split(conn.lhs.clone())?;
    crs = Connector::split(conn.rhs.clone())?;
    checkBalance(cls.clone(), crs.clone(), conn.clone())?;
    for mut cl in &*cls.clone() {
        let mut cl = cl.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cr = __pa0.clone();
        crs = __pa1.clone();
        if !(Connector::isDeleted(cl.clone()) || Connector::isDeleted(cr.clone())) {
            conns = cons(Arc::new(NFConnection { lhs: cl.clone(), rhs: cr.clone() }), conns.clone());
        }
    }
    conns = conns.clone().reverse();
    Ok(conns)
}

pub fn scalarize(mut conn: Arc<NFConnection>) -> Result<Arc<metamodelica::List<Arc<NFConnection>>>> {
    let mut conns: Arc<metamodelica::List<Arc<NFConnection>>> = metamodelica::nil();
    let mut cls: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut crs: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut cr: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    if !(Connector::isArray(conn.lhs.clone())) {
        conns = list![conn.clone()];
        return Ok(conns);
    }
    cls = Connector::scalarize(conn.lhs.clone())?;
    crs = Connector::scalarize(conn.rhs.clone())?;
    checkBalance(cls.clone(), crs.clone(), conn.clone())?;
    for mut cl in &*cls.clone() {
        let mut cl = cl.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cr = __pa0.clone();
        crs = __pa1.clone();
        conns = cons(Arc::new(NFConnection { lhs: cl.clone(), rhs: cr.clone() }), conns.clone());
    }
    conns = conns.clone().reverse();
    Ok(conns)
}

pub fn scalarizePrefix(mut conn: Arc<NFConnection>) -> Result<Arc<metamodelica::List<Arc<NFConnection>>>> {
    let mut conns: Arc<metamodelica::List<Arc<NFConnection>>> = metamodelica::nil();
    let mut cls: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut crs: Arc<metamodelica::List<Arc<Connector::NFConnector>>> = metamodelica::nil();
    let mut cr: Arc<Connector::NFConnector> = Arc::new(<Connector::NFConnector as ::std::default::Default>::default());
    if !(Connector::isArray(conn.lhs.clone())) {
        conns = list![conn.clone()];
        return Ok(conns);
    }
    cls = Connector::scalarizePrefix(conn.lhs.clone())?;
    crs = Connector::scalarizePrefix(conn.rhs.clone())?;
    checkBalance(cls.clone(), crs.clone(), conn.clone())?;
    for mut cl in &*cls.clone() {
        let mut cl = cl.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cr = __pa0.clone();
        crs = __pa1.clone();
        conns = cons(Arc::new(NFConnection { lhs: cl.clone(), rhs: cr.clone() }), conns.clone());
    }
    conns = conns.clone().reverse();
    Ok(conns)
}

pub fn toString(mut conn: Arc<NFConnection>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*Connector::toString(conn.lhs.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Connector::toString(conn.rhs.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn checkBalance(mut leftConnectors: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut rightConnectors: Arc<metamodelica::List<Arc<Connector::NFConnector>>>, mut conn: Arc<NFConnection>) -> Result<()> {
    if (leftConnectors.clone().len() as i32) != (rightConnectors.clone().len() as i32) {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFConnection.checkBalance")); __mm_s.push_str(&*literal!(" got unbalanced connection ")); __mm_s.push_str(&*toString(conn.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*List::toString(leftConnectors.clone(), (std::sync::Arc::new(fnptr!(Connector::toString, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>) -> Result<ArcStr> + 'static>), (literal!("\n  lhs: ")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*List::toString(rightConnectors.clone(), (std::sync::Arc::new(fnptr!(Connector::toString, Arc<Connector::NFConnector>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Connector::NFConnector>) -> Result<ArcStr> + 'static>), (literal!("\n  rhs: ")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    Ok(())
}


