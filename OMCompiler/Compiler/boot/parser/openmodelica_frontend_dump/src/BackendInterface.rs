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
use openmodelica_util::Global;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BackendInterfaceFunctions {
    pub noRewriteRulesFrontEnd: partialNoRewriteRulesFrontEnd,
    pub rewriteFrontEnd: partialRewriteFrontEnd,
    pub appendLibrary: partialAppendLibrary,
    pub initInstHashTable: partialInitInstHashTable,
}

impl Default for BackendInterfaceFunctions {
    fn default() -> Self {
        Self {
            noRewriteRulesFrontEnd: { let __placeholder: partialNoRewriteRulesFrontEnd = || panic!("default-constructed placeholder fn must not be called"); __placeholder },
            rewriteFrontEnd: { let __placeholder: partialRewriteFrontEnd = |_| panic!("default-constructed placeholder fn must not be called"); __placeholder },
            appendLibrary: { let __placeholder: partialAppendLibrary = |_, _| panic!("default-constructed placeholder fn must not be called"); __placeholder },
            initInstHashTable: { let __placeholder: partialInitInstHashTable = || panic!("default-constructed placeholder fn must not be called"); __placeholder },
        }
    }
}

pub type BACKEND_INTERFACE_FUNCTIONS = BackendInterfaceFunctions;


pub fn initializeBackendInterface(mut inFunctions: BackendInterfaceFunctions) -> () {
    crate::Globals::backendInterface.with(|__root| *__root.borrow_mut() = inFunctions.clone());
    ()
}

pub fn noRewriteRulesFrontEnd() -> bool {
    let mut noRules: bool = false;
    let mut functions: BackendInterfaceFunctions;
    let mut func: partialNoRewriteRulesFrontEnd;
    functions = crate::Globals::backendInterface.with(|__root| __root.borrow().clone());
    func = functions.noRewriteRulesFrontEnd;
    noRules = func().unwrap();
    noRules
}

pub fn rewriteFrontEnd(mut inExp: Arc<Absyn::Exp>) -> (Arc<Absyn::Exp>, bool) {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut isChanged: bool = false;
    let mut functions: BackendInterfaceFunctions;
    let mut func: partialRewriteFrontEnd;
    functions = crate::Globals::backendInterface.with(|__root| __root.borrow().clone());
    func = functions.rewriteFrontEnd;
    (outExp, isChanged) = func(inExp.clone()).unwrap();
    (outExp, isChanged)
}

pub fn appendLibrary(mut modelName: Arc<Absyn::Path>, mut modelicaPath: ArcStr) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program;
    let mut success: bool = false;
    let mut functions: BackendInterfaceFunctions;
    let mut func: partialAppendLibrary;
    functions = crate::Globals::backendInterface.with(|__root| __root.borrow().clone());
    func = functions.appendLibrary;
    (program, success) = func(modelName.clone(), (modelicaPath.clone()).clone()).unwrap();
    (program, success)
}

pub fn initInstHashTable() -> () {
    let mut functions: BackendInterfaceFunctions;
    let mut func: partialInitInstHashTable;
    functions = crate::Globals::backendInterface.with(|__root| __root.borrow().clone());
    func = functions.initInstHashTable;
    func().unwrap();
    ()
}

pub type partialNoRewriteRulesFrontEnd = fn() -> Result<bool>;

pub type partialRewriteFrontEnd = fn(Arc<Absyn::Exp>) -> Result<(Arc<Absyn::Exp>, bool)>;

pub type partialAppendLibrary = fn(Arc<Absyn::Path>, ArcStr) -> Result<(Absyn::Program, bool)>;

pub type partialInitInstHashTable = fn() -> Result<()>;

