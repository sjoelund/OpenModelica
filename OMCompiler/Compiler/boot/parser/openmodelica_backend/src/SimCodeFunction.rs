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

use crate::CodegenCFunctions;
use crate::CodegenMidToC;
use crate::DAEToMid;
use crate::HashTableCrefSimVar;
use crate::MidCode;
use crate::SimCodeFunctionUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::HashTableStringToPath;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_susan::Tpl;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util_datatypes_basic::List;

// public imports
// private imports
/// Root data structure containing information required for templates to
///  generate C functions for Modelica/MetaModelica functions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionCode {
    pub name: ArcStr,
    /// This function is special; the 'in'-function should be generated for it
    pub mainFunction: Option<Arc<Function::Function>>,
    pub functions: Arc<metamodelica::List<Arc<Function::Function>>>,
    /// shared literals
    pub literals: Arc<metamodelica::List<Arc<DAE::Exp>>>,
    pub externalFunctionIncludes: Arc<metamodelica::List<ArcStr>>,
    pub makefileParams: MakefileParams,
    pub extraRecordDecls: Arc<metamodelica::List<RecordDeclaration>>,
}

impl Default for FunctionCode {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mainFunction: Default::default(),
            functions: Default::default(),
            literals: Default::default(),
            externalFunctionIncludes: Default::default(),
            makefileParams: Default::default(),
            extraRecordDecls: Default::default(),
        }
    }
}

pub type FUNCTIONCODE = FunctionCode;


// TODO: I believe some of these fields can be removed. Check to see what is
//       used in templates.
pub mod Function {
    use super::*;
    /// Represents a Modelica, MetaModelica or external function.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Function {
        FUNCTION {
            name: Arc<Absyn::Path>,
            outVars: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            functionArguments: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            variableDeclarations: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            body: Arc<metamodelica::List<Arc<DAE::Statement>>>,
            visibility: SCode::Visibility,
            info: SourceInfo,
        },
        PARALLEL_FUNCTION {
            name: Arc<Absyn::Path>,
            outVars: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            functionArguments: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            variableDeclarations: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            body: Arc<metamodelica::List<Arc<DAE::Statement>>>,
            info: SourceInfo,
        },
        KERNEL_FUNCTION {
            name: Arc<Absyn::Path>,
            outVars: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            functionArguments: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            variableDeclarations: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            body: Arc<metamodelica::List<Arc<DAE::Statement>>>,
            info: SourceInfo,
        },
        EXTERNAL_FUNCTION {
            name: Arc<Absyn::Path>,
            extName: ArcStr,
            funArgs: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            extArgs: Arc<metamodelica::List<Arc<SimExtArg::SimExtArg>>>,
            extReturn: Arc<SimExtArg::SimExtArg>,
            inVars: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            outVars: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            biVars: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            /// this one is needed so that we know if we should generate the external function prototype or not
            includes: Arc<metamodelica::List<ArcStr>>,
            /// need this one for C#
            libs: Arc<metamodelica::List<ArcStr>>,
            /// C or Fortran
            language: ArcStr,
            visibility: SCode::Visibility,
            info: SourceInfo,
            dynamicLoad: bool,
        },
        RECORD_CONSTRUCTOR {
            name: Arc<Absyn::Path>,
            funArgs: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            locals: Arc<metamodelica::List<Arc<Variable::Variable>>>,
            visibility: SCode::Visibility,
            info: SourceInfo,
        },
    }
    pub use self::Function::{FUNCTION,PARALLEL_FUNCTION,KERNEL_FUNCTION,EXTERNAL_FUNCTION,RECORD_CONSTRUCTOR};
    pub fn toString(mut func: Arc<Function>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = ((::match_deref::match_deref! { match &(func.clone()) {
        Deref @ FUNCTION { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("name: ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*func).name, Function::FUNCTION).clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FUNCTION(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ PARALLEL_FUNCTION { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("name: ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*func).name, Function::PARALLEL_FUNCTION).clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PARALLEL_FUNCTION(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ KERNEL_FUNCTION { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("name: ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*func).name, Function::KERNEL_FUNCTION).clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("KERNEL_FUNCTION(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ EXTERNAL_FUNCTION { .. } => {
            let mut tmp: ArcStr = literal!("");
            let mut ls: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            tmp = (literal!("\n")).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  name: ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*func).name, Function::EXTERNAL_FUNCTION).clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(",\n")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  extName: ")); __mm_s.push_str(&*var_field!((*func).extName, Function::EXTERNAL_FUNCTION).clone()); __mm_s.push_str(&*literal!(",\n")); ArcStr::from(__mm_s) }).clone();
            ls = List::map(var_field!((*func).funArgs, Function::EXTERNAL_FUNCTION).clone(), Arc::new(Variable::toString));
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  funArgs: {")); __mm_s.push_str(&*stringDelimitList(ls.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("},\n")); ArcStr::from(__mm_s) }).clone();
            ls = List::map(var_field!((*func).extArgs, Function::EXTERNAL_FUNCTION).clone(), Arc::new(SimExtArg::toString));
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  extArgs: {")); __mm_s.push_str(&*stringDelimitList(ls.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("},\n")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  extReturn: ")); __mm_s.push_str(&*SimExtArg::toString(var_field!((*func).extReturn, Function::EXTERNAL_FUNCTION).clone())?); __mm_s.push_str(&*literal!(",\n")); ArcStr::from(__mm_s) }).clone();
            ls = List::map(var_field!((*func).inVars, Function::EXTERNAL_FUNCTION).clone(), Arc::new(Variable::toString));
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  inVars: {")); __mm_s.push_str(&*stringDelimitList(ls.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("},\n")); ArcStr::from(__mm_s) }).clone();
            ls = List::map(var_field!((*func).outVars, Function::EXTERNAL_FUNCTION).clone(), Arc::new(Variable::toString));
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  outVars: {")); __mm_s.push_str(&*stringDelimitList(ls.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("},\n")); ArcStr::from(__mm_s) }).clone();
            ls = List::map(var_field!((*func).biVars, Function::EXTERNAL_FUNCTION).clone(), Arc::new(Variable::toString));
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  biVars: {")); __mm_s.push_str(&*stringDelimitList(ls.clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("},\n")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  includes: {")); __mm_s.push_str(&*stringDelimitList(var_field!((*func).includes, Function::EXTERNAL_FUNCTION).clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("},\n")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  libs: {")); __mm_s.push_str(&*stringDelimitList(var_field!((*func).libs, Function::EXTERNAL_FUNCTION).clone(), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("},\n")); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("  language: ")); __mm_s.push_str(&*var_field!((*func).language, Function::EXTERNAL_FUNCTION).clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EXTERNAL_FUNCTION(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ RECORD_CONSTRUCTOR { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("name: ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*func).name, Function::RECORD_CONSTRUCTOR).clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("RECORD_CONSTRUCTOR(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            let mut tmp: ArcStr = literal!("");
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SimCodeFunction.Function.toString")); __mm_s.push_str(&*literal!(" failed for an unknown reason.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordDeclaration {
    RECORD_DECL_FULL {
        /// struct (record) name ? encoded
        name: ArcStr,
        /// alias of struct (record) name ? encoded. Code generators can generate an aliasing typedef using this, and avoid problems when casting a record from one type to another (*(othertype*)(&var)), which only works if you have a lhs value.
        aliasName: Option<ArcStr>,
        /// definition path
        defPath: Arc<Absyn::Path>,
        /// only name and type
        variables: Arc<metamodelica::List<Arc<Variable::Variable>>>,
        /// If the record is passed to an external function at any point, we need to generate conversion functions for it (for instance to convert 'modelica_integer' to 'int')
        usedExternally: bool,
    },
    RECORD_DECL_ADD_CONSTRCTOR {
        /// A unique name for the new constor. e.g. R_1_3() if it needs the 1st an 3rd members as inputs
        ctor_name: ArcStr,
        /// The record's name
        name: ArcStr,
        /// The members with the ones that need outisde binding marked. e.g 1st and 3rd elements will have bind_from_outside=true
        variables: Arc<metamodelica::List<Arc<Variable::Variable>>>,
    },
    RECORD_DECL_DEF {
        /// definition path .. encoded?
        path: Arc<Absyn::Path>,
        fieldNames: Arc<metamodelica::List<ArcStr>>,
    },
}
impl Default for RecordDeclaration {
    fn default() -> Self {
        Self::RECORD_DECL_ADD_CONSTRCTOR {
            ctor_name: Default::default(),
            name: Default::default(),
            variables: Default::default(),
        }
    }
}
pub use self::RecordDeclaration::{RECORD_DECL_FULL,RECORD_DECL_ADD_CONSTRCTOR,RECORD_DECL_DEF};

/// Platform specific parameters used when generating makefiles.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MakefileParams {
    pub ccompiler: ArcStr,
    pub cxxcompiler: ArcStr,
    pub linker: ArcStr,
    pub exeext: ArcStr,
    pub dllext: ArcStr,
    pub omhome: ArcStr,
    pub cflags: ArcStr,
    pub ldflags: ArcStr,
    /// Libraries that are required by the runtime library
    pub runtimelibs: ArcStr,
    pub includes: Arc<metamodelica::List<ArcStr>>,
    pub libs: Arc<metamodelica::List<ArcStr>>,
    pub libPaths: Arc<metamodelica::List<ArcStr>>,
    pub platform: ArcStr,
    pub compileDir: ArcStr,
}

impl Default for MakefileParams {
    fn default() -> Self {
        Self {
            ccompiler: Default::default(),
            cxxcompiler: Default::default(),
            linker: Default::default(),
            exeext: Default::default(),
            dllext: Default::default(),
            omhome: Default::default(),
            cflags: Default::default(),
            ldflags: Default::default(),
            runtimelibs: Default::default(),
            includes: Default::default(),
            libs: Default::default(),
            libPaths: Default::default(),
            platform: Default::default(),
            compileDir: Default::default(),
        }
    }
}

pub type MAKEFILE_PARAMS = MakefileParams;


pub mod SimExtArg {
    use super::*;
    /// Information about an argument to an external function.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum SimExtArg {
        SIMEXTARG {
            cref: Arc<DAE::ComponentRef>,
            isInput: bool,
            /// > 0 if output
            outputIndex: i32,
            isArray: bool,
            /// avoid double allocation
            hasBinding: bool,
            type_: Arc<DAE::Type>,
        },
        SIMEXTARGEXP {
            exp: Arc<DAE::Exp>,
            type_: Arc<DAE::Type>,
        },
        SIMEXTARGSIZE {
            cref: Arc<DAE::ComponentRef>,
            isInput: bool,
            /// > 0 if output
            outputIndex: i32,
            type_: Arc<DAE::Type>,
            exp: Arc<DAE::Exp>,
        },
        SIMNOEXTARG,
    }
    impl Default for SimExtArg {
        fn default() -> Self { Self::SIMNOEXTARG }
    }
    pub use self::SimExtArg::{SIMEXTARG,SIMEXTARGEXP,SIMEXTARGSIZE,SIMNOEXTARG};
    pub fn toString(mut simExtArg: Arc<SimExtArg>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = ((::match_deref::match_deref! { match &(simExtArg.clone()) {
        Deref @ SIMEXTARG { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("cref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!((*simExtArg).cref, SimExtArg::SIMEXTARG).clone())?); ArcStr::from(__mm_s) }).clone();
            tmp = (if (var_field!((*simExtArg).isInput, SimExtArg::SIMEXTARG).clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", isInput: true")); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", isInput: false")); ArcStr::from(__mm_s) }}).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", outputIndex: ")); __mm_s.push_str(&*intString(var_field!((*simExtArg).outputIndex, SimExtArg::SIMEXTARG).clone())); ArcStr::from(__mm_s) }).clone();
            tmp = (if (var_field!((*simExtArg).isArray, SimExtArg::SIMEXTARG).clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", isArray: true")); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", isArray: false")); ArcStr::from(__mm_s) }}).clone();
            tmp = (if (var_field!((*simExtArg).hasBinding, SimExtArg::SIMEXTARG).clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", hasBinding: true")); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", hasBinding: false")); ArcStr::from(__mm_s) }}).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", type: ")); __mm_s.push_str(&*TypesDump::unparseType(var_field!((*simExtArg).type_, SimExtArg::SIMEXTARG).clone())?); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SIMEXTARG(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ SIMEXTARGEXP { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!((*simExtArg).exp, SimExtArg::SIMEXTARGEXP).clone())?); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", type: ")); __mm_s.push_str(&*TypesDump::unparseType(var_field!((*simExtArg).type_, SimExtArg::SIMEXTARGEXP).clone())?); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SIMEXTARGEXP(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ SIMEXTARGSIZE { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("cref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!((*simExtArg).cref, SimExtArg::SIMEXTARGSIZE).clone())?); ArcStr::from(__mm_s) }).clone();
            tmp = (if (var_field!((*simExtArg).isInput, SimExtArg::SIMEXTARGSIZE).clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", isInput: true")); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", isInput: false")); ArcStr::from(__mm_s) }}).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", outputIndex: ")); __mm_s.push_str(&*intString(var_field!((*simExtArg).outputIndex, SimExtArg::SIMEXTARGSIZE).clone())); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", type: ")); __mm_s.push_str(&*TypesDump::unparseType(var_field!((*simExtArg).type_, SimExtArg::SIMEXTARGSIZE).clone())?); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!((*simExtArg).exp, SimExtArg::SIMEXTARGSIZE).clone())?); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SIMEXTARGSIZE(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ SIMNOEXTARG { .. } => {
            let mut tmp: ArcStr = literal!("");
            literal!("SIMNOEXTARG()")
        },
        _ => {
            let mut tmp: ArcStr = literal!("");
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SimCodeFunction.SimExtArg.toString")); __mm_s.push_str(&*literal!(" failed for an unknown reason.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

}

pub mod Variable {
    use super::*;
    /// A variable represents a name, a type and a possible default value
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Variable {
        VARIABLE {
            name: Arc<DAE::ComponentRef>,
            ty: Arc<DAE::Type>,
            /// default value
            value: Option<Arc<DAE::Exp>>,
            instDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>,
            parallelism: DAE::VarParallelism,
            kind: DAE::VarKind,
            bind_from_outside: bool,
        },
        FUNCTION_PTR {
            name: ArcStr,
            tys: Arc<metamodelica::List<Arc<DAE::Type>>>,
            args: Arc<metamodelica::List<Arc<Variable>>>,
            /// default value
            defaultValue: Option<Arc<DAE::Exp>>,
        },
    }
    impl Default for Variable {
        fn default() -> Self {
            Self::FUNCTION_PTR {
                name: Default::default(),
                tys: Default::default(),
                args: Default::default(),
                defaultValue: Default::default(),
            }
        }
    }
    pub use self::Variable::{VARIABLE,FUNCTION_PTR};
    pub fn toString(mut variable: Arc<Variable>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = ((::match_deref::match_deref! { match &(variable.clone()) {
        Deref @ VARIABLE { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("name: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!((*variable).name, Variable::VARIABLE).clone())?); ArcStr::from(__mm_s) }).clone();
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(", type: ")); __mm_s.push_str(&*TypesDump::unparseType(var_field!((*variable).ty, Variable::VARIABLE).clone())?); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("VARIABLE(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ FUNCTION_PTR { .. } => {
            let mut tmp: ArcStr = literal!("");
            tmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!("name: ")); __mm_s.push_str(&*var_field!((*variable).name, Variable::FUNCTION_PTR).clone()); ArcStr::from(__mm_s) }).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FUNCTION_PTR(")); __mm_s.push_str(&*tmp.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            let mut tmp: ArcStr = literal!("");
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SimCodeFunction.Variable.toString")); __mm_s.push_str(&*literal!(" failed for an unknown reason.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

}

/// Constants of this type defined below are used by templates to be able to
///  generate different code depending on the context it is generated in.
#[derive(Clone, Debug, PartialEq)]
pub enum Context {
    SIMULATION_CONTEXT {
        genDiscrete: bool,
    },
    FUNCTION_CONTEXT {
        cref_prefix: ArcStr,
        is_parallel: bool,
    },
    ALGLOOP_CONTEXT {
        genInitialisation: bool,
        genJacobian: bool,
    },
    JACOBIAN_CONTEXT {
        name: ArcStr,
        jacHT: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr))>,
    },
    OTHER_CONTEXT,
    ZEROCROSSINGS_CONTEXT,
    OPTIMIZATION_CONTEXT,
    FMI_CONTEXT,
    DAE_MODE_CONTEXT,
    OMSI_CONTEXT {
        /// used to get local SimVars and corresponding value references
        hashTable: Option<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr))>,
    },
}
impl Default for Context {
    fn default() -> Self { Self::OTHER_CONTEXT }
}
pub use self::Context::{SIMULATION_CONTEXT,FUNCTION_CONTEXT,ALGLOOP_CONTEXT,JACOBIAN_CONTEXT,OTHER_CONTEXT,ZEROCROSSINGS_CONTEXT,OPTIMIZATION_CONTEXT,FMI_CONTEXT,DAE_MODE_CONTEXT,OMSI_CONTEXT};

pub const fn contextSimulationNonDiscrete() -> Context { Context::SIMULATION_CONTEXT { genDiscrete: false } }

pub const fn contextSimulationDiscrete() -> Context { Context::SIMULATION_CONTEXT { genDiscrete: true } }

pub const fn contextFunction() -> Context { Context::FUNCTION_CONTEXT { cref_prefix: literal!(""), is_parallel: false } }

pub const fn contextJacobian() -> Context { Context::JACOBIAN_CONTEXT { name: literal!(""), jacHT: None } }

pub const fn contextAlgloopJacobian() -> Context { Context::ALGLOOP_CONTEXT { genInitialisation: false, genJacobian: true } }

pub const fn contextAlgloopInitialisation() -> Context { Context::ALGLOOP_CONTEXT { genInitialisation: true, genJacobian: false } }

pub const fn contextAlgloop() -> Context { Context::ALGLOOP_CONTEXT { genInitialisation: false, genJacobian: false } }

pub const fn contextOther() -> Context { crate::SimCodeFunction::Context::OTHER_CONTEXT }

pub const fn contextParallelFunction() -> Context { Context::FUNCTION_CONTEXT { cref_prefix: literal!(""), is_parallel: true } }

pub const fn contextZeroCross() -> Context { crate::SimCodeFunction::Context::ZEROCROSSINGS_CONTEXT }

pub const fn contextOptimization() -> Context { crate::SimCodeFunction::Context::OPTIMIZATION_CONTEXT }

pub const fn contextFMI() -> Context { crate::SimCodeFunction::Context::FMI_CONTEXT }

pub const fn contextDAEmode() -> Context { crate::SimCodeFunction::Context::DAE_MODE_CONTEXT }

pub const fn contextOMSI() -> Context { Context::OMSI_CONTEXT { hashTable: None } }

pub static listExpLength1: std::sync::LazyLock<Arc<metamodelica::List<Arc<DAE::Exp>>>> = std::sync::LazyLock::new(|| { list![Arc::new(DAE::Exp::ICONST { integer: 0 })] });

pub static boxedRecordOutVars: std::sync::LazyLock<Arc<metamodelica::List<Arc<Variable::Variable>>>> = std::sync::LazyLock::new(|| { cons(Arc::new(Variable::Variable::VARIABLE { name: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("")).clone(), identType: DAE::T_COMPLEX_DEFAULT_RECORD.clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_COMPLEX_DEFAULT_RECORD.clone(), value: None, instDims: metamodelica::nil(), parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE, bind_from_outside: false }), metamodelica::nil()) });

pub fn translateFunctions(mut program: Absyn::Program, mut name: ArcStr, mut optMainFunction: Option<DAE::Function>, mut idaeElements: Arc<metamodelica::List<DAE::Function>>, mut metarecordTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inIncludes: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    crate::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = None);
    let _ = (::match_deref::match_deref! { match &((program.clone(), name.clone(), optMainFunction.clone(), idaeElements.clone(), metarecordTypes.clone(), inIncludes.clone())) {
        (_, _, Some(daeMainFunction), daeElements, _, includes) => {
            let mut mainFunction: Arc<Function::Function>;
            let mut fns: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut makefileParams: MakefileParams;
            let mut fnCode: FunctionCode;
            let mut extraRecordDecls: Arc<metamodelica::List<RecordDeclaration>> = metamodelica::nil();
            let mut literals: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut midCode: Tpl::Text;
            let mut midfuncs: Arc<metamodelica::List<MidCode::Function>> = metamodelica::nil();
            let mut daeElements = (*daeElements).clone();
            let mut includes = (*includes).clone();
            (daeElements, literals) = SimCodeFunctionUtil::findLiterals(cons(daeMainFunction.clone(), daeElements.clone()));
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(SimCodeFunctionUtil::elaborateFunctions(program.clone(), daeElements.clone(), metarecordTypes.clone(), literals.clone(), includes.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 }, __pa2, __pa3, __pa4, __pa5, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            mainFunction = __pa0.clone();
            fns = __pa1.clone();
            extraRecordDecls = __pa2.clone();
            includes = __pa3.clone();
            includeDirs = __pa4.clone();
            libs = __pa5.clone();
            libPaths = __pa6.clone();
            SimCodeFunctionUtil::checkValidMainFunction((name.clone()).clone(), mainFunction.clone())?;
            makefileParams = SimCodeFunctionUtil::createMakefileParams(includeDirs.clone(), libs.clone(), libPaths.clone(), true, false)?;
            fnCode = FunctionCode { name: (name.clone()).clone(), mainFunction: Some(mainFunction.clone()), functions: fns.clone(), literals: literals.clone(), externalFunctionIncludes: includes.clone(), makefileParams: makefileParams.clone(), extraRecordDecls: extraRecordDecls.clone() };
            if Config::simCodeTarget()? == literal!("MidC") {
                let _ = Tpl::tplString(Arc::new(CodegenCFunctions::translateFunctionHeaderFiles), fnCode.clone())?;
                midfuncs = DAEToMid::DAEFunctionsToMid(cons(mainFunction.clone(), fns.clone()))?;
                midCode = Tpl::tplCallWithFailError(Arc::new(CodegenMidToC::genProgram), MidCode::Program { name: (name.clone()).clone(), functions: midfuncs.clone() }, Tpl::emptyTxt.clone())?;
                let _ = Tpl::textFileConvertLines(midCode.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".c")); ArcStr::from(__mm_s) }).clone())?;
            } else {
                let _ = Tpl::tplString(Arc::new(CodegenCFunctions::translateFunctions), fnCode.clone())?;
            }
            ()
        },
        (_, _, None, daeElements, _, includes) => {
            let mut fns: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut makefileParams: MakefileParams;
            let mut fnCode: FunctionCode;
            let mut extraRecordDecls: Arc<metamodelica::List<RecordDeclaration>> = metamodelica::nil();
            let mut literals: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut midCode: Tpl::Text;
            let mut midfuncs: Arc<metamodelica::List<MidCode::Function>> = metamodelica::nil();
            let mut daeElements = (*daeElements).clone();
            let mut includes = (*includes).clone();
            (daeElements, literals) = SimCodeFunctionUtil::findLiterals(daeElements.clone());
            (fns, extraRecordDecls, includes, includeDirs, libs, libPaths) = SimCodeFunctionUtil::elaborateFunctions(program.clone(), daeElements.clone(), metarecordTypes.clone(), literals.clone(), includes.clone())?;
            makefileParams = SimCodeFunctionUtil::createMakefileParams(includeDirs.clone(), libs.clone(), libPaths.clone(), true, false)?;
            fns = removeThreadDataFunction(fns.clone(), metamodelica::nil())?;
            extraRecordDecls = removeThreadDataRecord(extraRecordDecls.clone(), metamodelica::nil())?;
            fnCode = FunctionCode { name: (name.clone()).clone(), mainFunction: None, functions: fns.clone(), literals: literals.clone(), externalFunctionIncludes: includes.clone(), makefileParams: makefileParams.clone(), extraRecordDecls: extraRecordDecls.clone() };
            if Config::simCodeTarget()? == literal!("MidC") {
                let _ = Tpl::tplString(Arc::new(CodegenCFunctions::translateFunctionHeaderFiles), fnCode.clone())?;
                midfuncs = DAEToMid::DAEFunctionsToMid(fns.clone())?;
                midCode = Tpl::tplCallWithFailError(Arc::new(CodegenMidToC::genProgram), MidCode::Program { name: (name.clone()).clone(), functions: midfuncs.clone() }, Tpl::emptyTxt.clone())?;
                let _ = Tpl::textFileConvertLines(midCode.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".c")); ArcStr::from(__mm_s) }).clone())?;
            } else {
                let _ = Tpl::tplString(Arc::new(CodegenCFunctions::translateFunctions), fnCode.clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeThreadDataRecord(mut inRecs: Arc<metamodelica::List<RecordDeclaration>>, mut inAcc: Arc<metamodelica::List<RecordDeclaration>>) -> Result<Arc<metamodelica::List<RecordDeclaration>>> {
    let mut outRecs: Arc<metamodelica::List<RecordDeclaration>> = metamodelica::nil();
    outRecs = (::match_deref::match_deref! { match &((inRecs.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            inAcc.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: RecordDeclaration::RECORD_DECL_FULL { name: Deref @ "OpenModelica_threadData_ThreadData", .. }, tail: rest }, _) => {
            let mut acc: Arc<metamodelica::List<RecordDeclaration>> = metamodelica::nil();
            acc = removeThreadDataRecord(rest.clone(), inAcc.clone())?;
            acc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: RecordDeclaration::RECORD_DECL_DEF { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "threadData", path: Deref @ Absyn::Path::IDENT { name: Deref @ "ThreadData" } } }, .. }, tail: rest }, _) => {
            let mut acc: Arc<metamodelica::List<RecordDeclaration>> = metamodelica::nil();
            acc = removeThreadDataRecord(rest.clone(), inAcc.clone())?;
            acc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _) => {
            let mut acc: Arc<metamodelica::List<RecordDeclaration>> = metamodelica::nil();
            acc = removeThreadDataRecord(rest.clone(), cons(r.clone(), inAcc.clone()))?;
            acc.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRecs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeThreadDataFunction(mut inFuncs: Arc<metamodelica::List<Arc<Function::Function>>>, mut inAcc: Arc<metamodelica::List<Arc<Function::Function>>>) -> Result<Arc<metamodelica::List<Arc<Function::Function>>>> {
    let mut outFuncs: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    outFuncs = (::match_deref::match_deref! { match &((inFuncs.clone(), inAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            inAcc.clone().reverse()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Function::RECORD_CONSTRUCTOR { name: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "threadData", path: Deref @ Absyn::Path::IDENT { name: Deref @ "ThreadData" } } } }, .. }, tail: rest }, _) => {
            let mut acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
            acc = removeThreadDataFunction(rest.clone(), inAcc.clone())?;
            acc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: f, tail: rest }, _) => {
            let mut acc: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
            acc = removeThreadDataFunction(rest.clone(), cons(f.clone(), inAcc.clone()))?;
            acc.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFuncs)
}

