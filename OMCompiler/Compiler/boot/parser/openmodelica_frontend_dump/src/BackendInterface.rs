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

#[derive(Clone)]
pub struct BackendInterfaceFunctions {
    pub noRewriteRulesFrontEnd: partialNoRewriteRulesFrontEnd,
    pub rewriteFrontEnd: partialRewriteFrontEnd,
    pub appendLibrary: partialAppendLibrary,
    pub initInstHashTable: partialInitInstHashTable,
}

impl PartialEq for BackendInterfaceFunctions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq((&self.noRewriteRulesFrontEnd), (&other.noRewriteRulesFrontEnd)) && std::sync::Arc::ptr_eq((&self.rewriteFrontEnd), (&other.rewriteFrontEnd)) && std::sync::Arc::ptr_eq((&self.appendLibrary), (&other.appendLibrary)) && std::sync::Arc::ptr_eq((&self.initInstHashTable), (&other.initInstHashTable))
    }
}
impl Eq for BackendInterfaceFunctions {}
impl PartialOrd for BackendInterfaceFunctions {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for BackendInterfaceFunctions {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (std::sync::Arc::as_ptr((&self.noRewriteRulesFrontEnd)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.noRewriteRulesFrontEnd)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.rewriteFrontEnd)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.rewriteFrontEnd)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.appendLibrary)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.appendLibrary)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.initInstHashTable)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.initInstHashTable)) as *const ())))))
    }
}
impl std::hash::Hash for BackendInterfaceFunctions {
    fn hash<__H: std::hash::Hasher>(&self, __state: &mut __H) {
        (std::sync::Arc::as_ptr((&self.noRewriteRulesFrontEnd)) as *const ()).hash(__state);
        (std::sync::Arc::as_ptr((&self.rewriteFrontEnd)) as *const ()).hash(__state);
        (std::sync::Arc::as_ptr((&self.appendLibrary)) as *const ()).hash(__state);
        (std::sync::Arc::as_ptr((&self.initInstHashTable)) as *const ()).hash(__state);
    }
}
impl std::fmt::Debug for BackendInterfaceFunctions {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("BackendInterfaceFunctions");
        __ds.field("noRewriteRulesFrontEnd", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.noRewriteRulesFrontEnd))));
        __ds.field("rewriteFrontEnd", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.rewriteFrontEnd))));
        __ds.field("appendLibrary", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.appendLibrary))));
        __ds.field("initInstHashTable", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.initInstHashTable))));
        __ds.finish()
    }
}

impl Default for BackendInterfaceFunctions {
    fn default() -> Self {
        Self {
            noRewriteRulesFrontEnd: { let __placeholder: partialNoRewriteRulesFrontEnd = std::sync::Arc::new(|| panic!("default-constructed placeholder fn must not be called")); __placeholder },
            rewriteFrontEnd: { let __placeholder: partialRewriteFrontEnd = std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called")); __placeholder },
            appendLibrary: { let __placeholder: partialAppendLibrary = std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called")); __placeholder },
            initInstHashTable: { let __placeholder: partialInitInstHashTable = std::sync::Arc::new(|| panic!("default-constructed placeholder fn must not be called")); __placeholder },
        }
    }
}

pub type BACKEND_INTERFACE_FUNCTIONS = BackendInterfaceFunctions;


pub fn initializeBackendInterface(mut inFunctions: BackendInterfaceFunctions) -> () {
    { let __v = inFunctions.clone(); crate::Globals::backendInterface.with(|__root| *__root.borrow_mut() = __v) };
    ()
}

pub fn noRewriteRulesFrontEnd() -> bool {
    let mut noRules: bool = false;
    let mut functions: BackendInterfaceFunctions = <BackendInterfaceFunctions as ::std::default::Default>::default();
    let mut func: partialNoRewriteRulesFrontEnd;
    functions = crate::Globals::backendInterface.with(|__root| __root.borrow().clone());
    func = functions.noRewriteRulesFrontEnd.clone();
    noRules = func().unwrap();
    noRules
}

pub fn rewriteFrontEnd(mut inExp: Arc<Absyn::Exp>) -> (Arc<Absyn::Exp>, bool) {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut isChanged: bool = false;
    let mut functions: BackendInterfaceFunctions = <BackendInterfaceFunctions as ::std::default::Default>::default();
    let mut func: partialRewriteFrontEnd;
    functions = crate::Globals::backendInterface.with(|__root| __root.borrow().clone());
    func = functions.rewriteFrontEnd.clone();
    (outExp, isChanged) = func(inExp.clone()).unwrap();
    (outExp, isChanged)
}

pub fn appendLibrary(mut modelName: Arc<Absyn::Path>, mut modelicaPath: ArcStr) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut success: bool = false;
    let mut functions: BackendInterfaceFunctions = <BackendInterfaceFunctions as ::std::default::Default>::default();
    let mut func: partialAppendLibrary;
    functions = crate::Globals::backendInterface.with(|__root| __root.borrow().clone());
    func = functions.appendLibrary.clone();
    (program, success) = func(modelName.clone(), (modelicaPath.clone()).clone()).unwrap();
    (program, success)
}

pub fn initInstHashTable() -> () {
    let mut functions: BackendInterfaceFunctions = <BackendInterfaceFunctions as ::std::default::Default>::default();
    let mut func: partialInitInstHashTable;
    functions = crate::Globals::backendInterface.with(|__root| __root.borrow().clone());
    func = functions.initInstHashTable.clone();
    func().unwrap();
    ()
}

pub type partialNoRewriteRulesFrontEnd = std::sync::Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>;

pub type partialRewriteFrontEnd = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<(Arc<Absyn::Exp>, bool)> + 'static>;

pub type partialAppendLibrary = std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, ArcStr) -> Result<(Absyn::Program, bool)> + 'static>;

pub type partialInitInstHashTable = std::sync::Arc<dyn ::std::ops::Fn() -> Result<()> + 'static>;

