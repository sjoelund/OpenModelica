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

use openmodelica_ast::Absyn;
use openmodelica_frontend::CevalFunction;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::Inst;
use openmodelica_frontend::Lookup;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::AvlTreeStringString;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::List;

/// file:        SymbolTable.mo
///  package:     SymbolTable
///  description: Thread-local, mutable symbol table. Set this at the start
///               of any interactive call or in Main.
#[derive(Clone, Debug, PartialEq)]
pub struct SymbolTable {
    /// ast ; The ast
    pub ast: Absyn::Program,
    /// the explodedAst is invalidated every time the program is updated
    pub explodedAst: Option<Arc<metamodelica::List<Arc<SCode::Element>>>>,
    /// List of variables with values
    pub vars: /* ? */,
    pub cachedAsts: Arc<Vector::Vector<Absyn::Program>>,
    pub cacheIndex: i32,
}

pub type SYMBOLTABLE = SymbolTable;

pub const AST_CACHE_MAX_SIZE: i32 = 1000;

pub fn reset() -> Result<()> {
    crate::Globals::symbolTable.with(|__root| *__root.borrow_mut() = Arc::new(SymbolTable { cacheIndex: 0, cachedAsts: Vector::new(0), vars: metamodelica::nil(), explodedAst: None, ast: Absyn::Program { classes: metamodelica::nil(), within_: openmodelica_ast::Absyn::Within::TOP } }));
    updateUriMapping(metamodelica::nil())?;
    Ok(())
}

pub fn update(mut table: Arc<SymbolTable>) -> () {
    crate::Globals::symbolTable.with(|__root| *__root.borrow_mut() = table.clone());
    ()
}

pub fn get() -> Arc<SymbolTable> {
    let mut table: Arc<SymbolTable>;
    table = crate::Globals::symbolTable.with(|__root| __root.borrow().clone());
    table
}

pub fn getAbsyn() -> Absyn::Program {
    let mut ast: Absyn::Program;
    let mut table: Arc<SymbolTable>;
    table = get();
    ast = table.ast.clone();
    ast
}

pub fn setAbsyn(mut ast: Absyn::Program) -> Result<()> {
    let mut table: Arc<SymbolTable>;
    table = get();
    if referenceEq(&table.ast.clone(),&ast.clone()) {
        return Ok(());
    }
    assign_field!(table.ast = ast.clone());
    updateUriMapping(ast.classes.clone())?;
    if isSome(table.explodedAst.clone()) {
        assign_field!(table.explodedAst = None);
    }
    update(table.clone());
    Ok(())
}

pub fn setAbsynElement(mut ast: Absyn::Program, mut element: Arc<Absyn::Element>, mut path: Arc<Absyn::Path>) -> Result<()> {
    fn update_element(mut oldElement: Arc<SCode::Element>, mut newElement: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
        let mut newElement: Arc<SCode::Element> = newElement;
        if SCodeUtil::isElementProtected(oldElement.clone())? {
            newElement = SCodeUtil::makeElementProtected(newElement.clone());
        }
        Ok(newElement)
    }

    let mut table: Arc<SymbolTable>;
    let mut scode_elem: Arc<SCode::Element>;
    let mut scode_elems: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut scode_prog: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    table = get();
    if referenceEq(&table.ast.clone(),&ast.clone()) {
        return Ok(());
    }
    assign_field!(table.ast = ast.clone());
    updateUriMapping(ast.classes.clone())?;
    if isSome(table.explodedAst.clone()) {
        scode_elems = AbsynToSCode::translateElement(element.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC)?;
        if (scode_elems.clone().len() as i32) > 1 {
            let __pa0 = ::match_deref::match_deref! { match &(List::findOption(scode_elems.clone(), Arc::new({ let __pe_b0 = (AbsynUtil::pathLastIdent(path.clone())?).clone(); move |__pe_a1| Ok(SCodeUtil::isElementNamed(__pe_b0.clone(), __pe_a1)) }))) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            scode_elem = __pa0.clone();
        } else {
            scode_elem = listHead(scode_elems.clone())?;
        }
        let __pa1 = ::match_deref::match_deref! { match &(table.explodedAst.clone()) {
            Some(__pa1) => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        scode_prog = __pa1.clone();
        let __pa2 = ::match_deref::match_deref! { match &(SCodeUtil::transformPathedElementInProgram(path.clone(), Arc::new({ let __pe_b1 = scode_elem.clone(); move |__pe_a0| update_element(__pe_a0, __pe_b1.clone()) }), scode_prog.clone())?) {
            (__pa2, true) => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        scode_prog = __pa2.clone();
        assign_field!(table.explodedAst = Some(scode_prog.clone()));
    }
    update(table.clone());
    Ok(())
}

pub fn setAbsynClass(mut ast: Absyn::Program, mut cls: Arc<Absyn::Class>, mut path: Arc<Absyn::Path>) -> Result<()> {
    fn update_element(mut oldElement: Arc<SCode::Element>, mut newElement: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
        let mut newElement: Arc<SCode::Element> = newElement;
        newElement = SCodeUtil::setElementPrefixes(SCodeUtil::elementPrefixes(oldElement.clone())?, newElement.clone())?;
        Ok(newElement)
    }

    let mut table: Arc<SymbolTable>;
    let mut scode_elem: Arc<SCode::Element>;
    let mut scode_prog: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    table = get();
    if referenceEq(&table.ast.clone(),&ast.clone()) {
        return Ok(());
    }
    assign_field!(table.ast = ast.clone());
    updateUriMapping(ast.classes.clone())?;
    if isSome(table.explodedAst.clone()) {
        scode_elem = AbsynToSCode::translateClass(cls.clone())?;
        let __pa0 = ::match_deref::match_deref! { match &(table.explodedAst.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        scode_prog = __pa0.clone();
        let __pa1 = ::match_deref::match_deref! { match &(SCodeUtil::transformPathedElementInProgram(path.clone(), Arc::new({ let __pe_b1 = scode_elem.clone(); move |__pe_a0| update_element(__pe_a0, __pe_b1.clone()) }), scode_prog.clone())?) {
            (__pa1, true) => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        scode_prog = __pa1.clone();
        assign_field!(table.explodedAst = Some(scode_prog.clone()));
    }
    update(table.clone());
    Ok(())
}

pub fn getSCode() -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut ast: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut table: Arc<SymbolTable>;
    table = get();
    if isNone(table.explodedAst.clone()) {
        ast = AbsynToSCode::translateAbsyn2SCode(table.ast.clone())?;
        assign_field!(table.explodedAst = Some(ast.clone()));
        update(table.clone());
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(table.explodedAst.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        ast = __pa0.clone();
    }
    Ok(ast)
}

pub fn setSCode(mut ast: Option<Arc<metamodelica::List<Arc<SCode::Element>>>>) -> () {
    let mut table: Arc<SymbolTable>;
    table = get();
    if referenceEq(&table.explodedAst.clone(),&ast.clone()) {
        return ();
    }
    assign_field!(table.explodedAst = ast.clone());
    update(table.clone());
    ()
}

pub fn clearSCode() -> () {
    let mut table: Arc<SymbolTable>;
    table = get();
    if isSome(table.explodedAst.clone()) {
        assign_field!(table.explodedAst = None);
        update(table.clone());
    }
    ()
}

pub fn clearProgram() -> Result<()> {
    let mut table: Arc<SymbolTable>;
    table = get();
    reset()?;
    setVars(table.vars.clone());
    Ok(())
}

pub fn addVars(mut inCref: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inEnv: FCore::Graph) -> Result<()> {
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    crefs = inCref.clone();
    vals = inValues.clone();
    while !(crefs.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cr = __pa0.clone();
        crefs = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(vals.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        v = __pa2.clone();
        vals = __pa3.clone();
        addVar(cr.clone(), v.clone(), inEnv.clone())?;
    }
    Ok(())
}

pub fn appendVar(mut inIdent: ArcStr, mut inValue: Arc<Values::Value>, mut inType: Arc<DAE::Type>) -> () {
    let mut table: Arc<SymbolTable>;
    table = get();
    assign_field!(table.vars = cons(InteractiveTypes::IVAR((inIdent.clone()).clone(), inValue.clone(), inType.clone()).unwrap(), table.vars.clone()));
    update(table.clone());
    ()
}

pub fn deleteVarFirstEntry(mut inIdent: ArcStr) -> Result<()> {
    let mut table: Arc<SymbolTable>;
    table = get();
    assign_field!(table.vars = List::deleteMemberOnTrue((inIdent.clone()).clone(), table.vars.clone(), Arc::new(isVarNamed.clone()))?.0);
    update(table.clone());
    Ok(())
}

pub fn storeAST() -> Result<i32> {
    let mut id: i32 = 0;
    let mut table: Arc<SymbolTable>;
    let mut index: i32 = 0;
    table = get();
    id = table.cacheIndex.clone() + 1;
    if id.clone() < 0 {
        id = 1;
    }
    assign_field!(table.cacheIndex = id.clone());
    update(table.clone());
    if Vector::size(table.cachedAsts.clone()) >= AST_CACHE_MAX_SIZE.clone() {
        Vector::update(table.cachedAsts.clone(), intMod(id.clone() - 1, AST_CACHE_MAX_SIZE.clone()) + 1, getAbsyn())?;
    } else {
        Vector::push(table.cachedAsts.clone(), getAbsyn());
    }
    Ok(id)
}

pub fn restoreAST(mut id: i32) -> Result<bool> {
    let mut success: bool = false;
    let mut table: Arc<SymbolTable>;
    table = get();
    success = id.clone() <= table.cacheIndex.clone() && id.clone() > table.cacheIndex.clone() - AST_CACHE_MAX_SIZE.clone() && id.clone() > 0;
    if success.clone() {
        setAbsyn(Vector::get(table.cachedAsts.clone(), intMod(id.clone() - 1, AST_CACHE_MAX_SIZE.clone()) + 1)?)?;
    }
    Ok(success)
}

pub fn buildEnv() -> Result<FCore::Graph> {
    let mut env: FCore::Graph;
    let mut table: Arc<SymbolTable>;
    table = get();
    (_, env) = Inst::makeEnvFromProgram(getSCode()?)?;
    env = addVarsToEnv(table.vars.clone().reverse(), env.clone());
    Ok(env)
}

fn updateUriMapping(mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>>) -> Result<()> {
    let mut tree: Arc<AvlTreeStringString::Tree> = Arc::new(AvlTreeStringString::Tree::EMPTY);
    let mut name: ArcStr = arcstr::literal!("");
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut dir: ArcStr = arcstr::literal!("");
    let mut b: bool = false;
    let mut namesAndDirs: metamodelica::Array<ArcStr>;
    let mut infos: Arc<metamodelica::List<SourceInfo>> = metamodelica::nil();
    tree = Arc::new(openmodelica_util::AvlTreeStringString::Tree::EMPTY);
    for mut cl in &*classes.clone() {
        let mut cl = cl.clone();
        let _ = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::Class { info: SourceInfo { fileName: Deref @ "<interactive>", .. }, .. } => (),
        Deref @ Absyn::Class { info: SourceInfo { fileName, .. }, name, .. } => {
            let mut fileName = (*fileName).clone();
            dir = (System::dirname((fileName.clone()).clone())).clone();
            fileName = (System::basename((fileName.clone()).clone())).clone();
            b = stringEq((fileName.clone()).clone(), (literal!("ModelicaBuiltin.mo")).clone()) || stringEq((fileName.clone()).clone(), (literal!("MetaModelicaBuiltin.mo")).clone()) || stringEq((dir.clone()).clone(), (literal!(".")).clone());
            if !(b.clone()) {
                if AvlTreeStringString::hasKey(tree.clone(), (name.clone()).clone())? {
                    infos = {
        let mut __acc: Arc<metamodelica::List<SourceInfo>> = metamodelica::nil();
        for mut cl in (classes.clone()).into_iter().cloned() {
            let __x = cl.info.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    Error::addMultiSourceMessage(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone(), list![(name.clone()).clone()], infos.clone())?;
                }
                tree = AvlTreeStringString::add(tree.clone(), (name.clone()).clone(), (dir.clone()).clone(), AvlTreeStringString::addConflictDefault)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    namesAndDirs = metamodelica::arrayFromVec(List::thread(AvlTreeStringString::listValues(tree.clone(), metamodelica::nil()), AvlTreeStringString::listKeys(tree.clone(), metamodelica::nil()), metamodelica::nil())?.into_iter().cloned().collect());
    System::updateUriMapping(namesAndDirs.clone());
    Ok(())
}


