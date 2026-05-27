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

use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFExpression as Expression;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub type Table = Arc<UnorderedMap::UnorderedMap<ArcStr, i32>>;

pub fn emptyCardinalityTable(mut size: i32) -> Table {
    let mut table: Table;
    table = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), size.clone());
    table
}

pub fn fromConnections(mut conns: Arc<Connections::NFConnections>) -> Result<Table> {
    let mut table: Table;
    if System::getUsesCardinality() {
        table = emptyCardinalityTable(std::cmp::max(1, Util::nextPrime((conns.connections.clone().len() as i32))));
        for mut conn in &*conns.connections.clone() {
            let mut conn = conn.clone();
            addConnector(conn.lhs.clone(), table.clone())?;
            addConnector(conn.rhs.clone(), table.clone())?;
        }
    } else {
        table = emptyCardinalityTable(1);
    }
    Ok(table)
}

pub fn addConnector(mut conn: Arc<Connector::NFConnector>, mut table: Table) -> Result<()> {
    fn update(mut count: Option<i32>) -> i32 {
        let mut outCount: i32 = 0;
        outCount = (match count.clone() {
        Some(mut outCount) => outCount.clone() + 1,
        _ => 1,
    });
        outCount
    }

    let mut conn_str: ArcStr = arcstr::literal!("");
    conn_str = (Connector::toString(conn.clone())).clone();
    UnorderedMap::addUpdate((conn_str.clone()).clone(), (std::sync::Arc::new(fnptr!(update, Option<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(Option<i32>) -> Result<i32> + 'static>), table.clone())?;
    Ok(())
}

pub fn evaluateCardinality(mut arg: Arc<Expression::NFExpression>, mut table: Table) -> Result<Arc<Expression::NFExpression>> {
    let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut count: i32 = 0;
    count = UnorderedMap::getOrDefault((Expression::toString(arg.clone())?).clone(), table.clone(), 0);
    res = Arc::new(Expression::NFExpression::INTEGER { value: count.clone() });
    Ok(res)
}

pub fn print(mut table: Table) -> () {
    for mut e in &*UnorderedMap::toList(table.clone()) {
        let mut e = e.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Util::tuple21(e.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", Util::tuple22(e.clone())))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    ()
}

