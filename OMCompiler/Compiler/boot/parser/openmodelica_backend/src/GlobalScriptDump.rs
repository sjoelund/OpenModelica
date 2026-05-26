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

use crate::SymbolTable;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_util_datatypes_basic::List;

pub fn printIstmtsStr(mut inStatements: GlobalScript::Statements) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inStatements.clone() {
        GlobalScript::Statements { interactiveStmtLst: ref stmts, .. } => {
            stringDelimitList(List::map(stmts.clone(), Arc::new(printIstmtStr)), (literal!("; ")).clone())
        },
        _ => {
            literal!("printIstmtsStr: unknown")
        },
    })).clone();
    outString
}

pub fn printIstmtStr(mut inStatement: GlobalScript::Statement) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inStatement.clone() {
        GlobalScript::Statement::IALG { algItem: ref alg } => {
            Dump::unparseAlgorithmStr(alg.clone())?
        },
        GlobalScript::Statement::IEXP { exp: ref expr, .. } => {
            Dump::printExpStr(expr.clone())?
        },
        _ => {
            literal!("printIstmtStr: unknown")
        },
    })).clone();
    Ok(outString)
}

pub fn printAST(mut pr: Absyn::Program) -> Result<()> {
    let mut s: ArcStr = literal!("");
    let mut class_: Arc<Absyn::Class>;
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut within_: Absyn::Within = Absyn::Within::TOP;
    let Absyn::PROGRAM { classes: __pa0, within_: __pa1 } = (pr.clone()) else { bail!("pattern mismatch") };
    classes = __pa0.clone();
    within_ = __pa1.clone();
    for mut class_ in &*classes.clone() {
        let mut class_ = class_.clone();
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*classString(class_.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    println!("{}", (s.clone()).clone());
    Ok(())
}

pub fn printGlobalScript(mut st: Arc<SymbolTable::SymbolTable>) -> Result<()> {
    println!("{}", (literal!("AST\n")).clone());
    printAST(st.ast.clone())?;
    Ok(())
}

fn classString(mut cl: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    let mut id: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::Class { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    id = __pa0.clone();
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*AbsynUtil::classFilename(cl.clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

