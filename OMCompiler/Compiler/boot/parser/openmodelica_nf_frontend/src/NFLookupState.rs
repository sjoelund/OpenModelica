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
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFRestriction as Restriction;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;

pub mod LookupStateName {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum LookupStateName {
        PATH {
            path: Arc<Absyn::Path>,
        },
        CREF {
            cref: Arc<Absyn::ComponentRef>,
        },
    }
    pub use self::LookupStateName::{PATH,CREF};
    pub fn toString(mut name: Arc<LookupStateName>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(name.clone()) {
        Deref @ PATH { .. } => AbsynUtil::pathString(var_field!((*name).path, LookupStateName::PATH).clone(), (literal!(".")).clone(), true, false)?,
        Deref @ CREF { .. } => Dump::printComponentRefStr(var_field!((*name).cref, LookupStateName::CREF).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(r#str)
    }

    pub fn firstIdent(mut name: Arc<LookupStateName>) -> Result<ArcStr> {
        let mut id: ArcStr = arcstr::literal!("");
        id = ((::match_deref::match_deref! { match &(name.clone()) {
        Deref @ PATH { .. } => AbsynUtil::pathFirstIdent(var_field!((*name).path, LookupStateName::PATH).clone())?,
        Deref @ CREF { .. } => AbsynUtil::crefFirstIdent(var_field!((*name).cref, LookupStateName::CREF).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(id)
    }

    pub fn secondIdent(mut name: Arc<LookupStateName>) -> Result<ArcStr> {
        let mut id: ArcStr = arcstr::literal!("");
        id = ((::match_deref::match_deref! { match &(name.clone()) {
        Deref @ PATH { .. } => AbsynUtil::pathSecondIdent(var_field!((*name).path, LookupStateName::PATH).clone())?,
        Deref @ CREF { .. } => AbsynUtil::crefSecondIdent(var_field!((*name).cref, LookupStateName::CREF).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(id)
    }

}

pub mod LookupState {
    use super::*;
    /// LookupState is used by the lookup to keep track of what state it's in so that
    ///  the rules for composite name lookup can be enforced.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum LookupState {
        /// The start state.
        BEGIN,
        /// A component.
        COMP,
        /// A component found in a class.
        CLASS_COMP,
        /// A class found in component.
        COMP_CLASS,
        /// A function found in component.
        COMP_FUNC,
        /// A package.
        PACKAGE,
        /// A class.
        CLASS,
        /// A function.
        FUNC,
        /// A predefined component.
        PREDEF_COMP,
        /// A predefined class.
        PREDEF_CLASS,
        IMPORT,
        /// A partial class.
        PARTIAL_CLASS,
        /// A nonconstant found in a context where a constant is required.
        NON_CONSTANT,
        /// A nonencapsulated element found in a context where encapsulated is required.
        NON_ENCAPSULATED,
        /// An error occured during lookup.
        ERROR {
            errorState: Arc<LookupState>,
        },
    }
    pub use self::LookupState::{BEGIN,COMP,CLASS_COMP,COMP_CLASS,COMP_FUNC,PACKAGE,CLASS,FUNC,PREDEF_COMP,PREDEF_CLASS,IMPORT,PARTIAL_CLASS,NON_CONSTANT,NON_ENCAPSULATED,ERROR};
    pub fn assertClass(mut endState: Arc<LookupState>, mut node: Arc<InstNode::InstNode>, mut name: Arc<Absyn::Path>, mut context: i32, mut info: SourceInfo) -> Result<()> {
        assertState(endState.clone(), Arc::new(crate::NFLookupState::LookupState::CLASS), node.clone(), Arc::new(LookupStateName::LookupStateName::PATH { path: name.clone() }), context.clone(), info.clone())?;
        Ok(())
    }

    pub fn assertFunction(mut endState: Arc<LookupState>, mut node: Arc<InstNode::InstNode>, mut name: Arc<Absyn::ComponentRef>, mut context: i32, mut info: SourceInfo) -> Result<()> {
        assertState(endState.clone(), Arc::new(crate::NFLookupState::LookupState::FUNC), node.clone(), Arc::new(LookupStateName::LookupStateName::CREF { cref: name.clone() }), context.clone(), info.clone())?;
        Ok(())
    }

    pub fn assertComponent(mut endState: Arc<LookupState>, mut node: Arc<InstNode::InstNode>, mut name: Arc<Absyn::ComponentRef>, mut context: i32, mut info: SourceInfo) -> Result<()> {
        assertState(endState.clone(), Arc::new(crate::NFLookupState::LookupState::COMP), node.clone(), Arc::new(LookupStateName::LookupStateName::CREF { cref: name.clone() }), context.clone(), info.clone())?;
        Ok(())
    }

    pub fn assertImport(mut endState: Arc<LookupState>, mut node: Arc<InstNode::InstNode>, mut name: Arc<Absyn::Path>, mut info: SourceInfo) -> Result<()> {
        assertState(endState.clone(), Arc::new(crate::NFLookupState::LookupState::IMPORT), node.clone(), Arc::new(LookupStateName::LookupStateName::PATH { path: name.clone() }), InstContext::NO_CONTEXT.clone(), info.clone())?;
        Ok(())
    }

    pub fn isCallableType(mut node: Arc<InstNode::InstNode>) -> Result<bool> {
        let mut callable: bool = false;
        let mut n: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        if !(InstNode::isClass(node.clone())) {
            callable = false;
            return Ok(callable.clone());
        }
        n = InstNode::resolveInner(node.clone());
        Inst::expand(n.clone(), InstContext::NO_CONTEXT.clone())?;
        callable = (::match_deref::match_deref! { match &(InstNode::restriction(n.clone())) {
        Deref @ Restriction::RECORD { .. } => true,
        Deref @ Restriction::OPERATOR => true,
        Deref @ Restriction::ENUMERATION => true,
        Deref @ Restriction::TYPE if (InstNode::isEnumerationType(n.clone())) => true,
        _ => InstNode::isClockType(n.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(callable)
    }

    pub fn isCallableComponent(mut node: Arc<InstNode::InstNode>) -> Result<bool> {
        let mut callable: bool = false;
        callable = Class::isFunction(InstNode::getClass(node.clone())?);
        Ok(callable)
    }

    pub fn isFunction(mut state: Arc<LookupState>, mut node: Arc<InstNode::InstNode>) -> Result<bool> {
        let mut isFunction: bool = false;
        isFunction = (::match_deref::match_deref! { match &(state.clone()) {
        Deref @ FUNC { .. } => true,
        Deref @ COMP_FUNC { .. } => true,
        Deref @ CLASS { .. } => isCallableType(node.clone())?,
        Deref @ COMP { .. } => isCallableComponent(node.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(isFunction)
    }

    pub fn isClass(mut state: Arc<LookupState>) -> bool {
        let mut isClass: bool = false;
        isClass = (::match_deref::match_deref! { match &(state.clone()) {
        Deref @ COMP_CLASS { .. } => true,
        Deref @ CLASS { .. } => true,
        Deref @ PREDEF_CLASS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isClass
    }

    pub fn assertState(mut endState: Arc<LookupState>, mut expectedState: Arc<LookupState>, mut node: Arc<InstNode::InstNode>, mut name: Arc<LookupStateName::LookupStateName>, mut context: i32, mut info: SourceInfo) -> Result<()> {
        let () = (::match_deref::match_deref! { match &((endState.clone(), expectedState.clone())) {
        (Deref @ COMP { .. }, Deref @ COMP { .. }) => {
            ()
        },
        (Deref @ CLASS_COMP { .. }, Deref @ COMP { .. }) => {
            ()
        },
        (Deref @ PREDEF_COMP { .. }, Deref @ COMP { .. }) => {
            ()
        },
        (Deref @ FUNC { .. }, Deref @ COMP { .. }) => {
            ()
        },
        (Deref @ COMP_FUNC { .. }, Deref @ COMP { .. }) => {
            ()
        },
        (Deref @ PACKAGE { .. }, Deref @ CLASS { .. }) => {
            ()
        },
        (Deref @ CLASS { .. }, Deref @ CLASS { .. }) => {
            ()
        },
        (Deref @ PREDEF_CLASS { .. }, Deref @ CLASS { .. }) => {
            ()
        },
        (Deref @ FUNC { .. }, Deref @ CLASS { .. }) => {
            ()
        },
        (Deref @ FUNC { .. }, Deref @ FUNC { .. }) => {
            ()
        },
        (Deref @ COMP_FUNC { .. }, Deref @ FUNC { .. }) => {
            ()
        },
        (Deref @ CLASS { .. }, Deref @ FUNC { .. }) if (isCallableType(node.clone())?) => {
            ()
        },
        (Deref @ COMP { .. }, Deref @ FUNC { .. }) if (isCallableComponent(node.clone())?) => {
            ()
        },
        (Deref @ COMP_CLASS { .. }, Deref @ FUNC { .. }) => {
            printFoundWrongTypeError(endState.clone(), expectedState.clone(), name.clone(), info.clone())?;
            bail!("fail")
        },
        (Deref @ COMP_FUNC { .. }, _) => {
            let mut name_str: ArcStr = arcstr::literal!("");
            name_str = (LookupStateName::toString(name.clone())?).clone();
            Error::addSourceMessage(Error::FOUND_FUNC_NAME_VIA_COMP_NONCALL.clone(), list![(name_str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ COMP_CLASS { .. }, _) => {
            Error::addSourceMessage(Error::FOUND_CLASS_NAME_VIA_COMPONENT.clone(), list![(LookupStateName::toString(name.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ ERROR { errorState: Deref @ COMP_FUNC { .. } }, Deref @ FUNC { .. }) => {
            let mut name_str: ArcStr = arcstr::literal!("");
            let mut info2: SourceInfo = <SourceInfo as ::std::default::Default>::default();
            name_str = (InstNode::name(node.clone())?).clone();
            info2 = InstNode::info(node.clone())?;
            Error::addSourceMessage(Error::NON_CLASS_IN_COMP_FUNC_NAME.clone(), list![(name_str.clone()).clone()], info2.clone())?;
            bail!("fail")
        },
        (Deref @ ERROR { errorState: Deref @ COMP_FUNC { .. } }, Deref @ COMP { .. }) => {
            let mut name_str: ArcStr = arcstr::literal!("");
            name_str = (InstNode::name(node.clone())?).clone();
            Error::addSourceMessage(Error::UNEXPECTED_COMPONENT_IN_COMPOSITE_NAME.clone(), list![(name_str.clone()).clone(), (LookupStateName::toString(name.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ ERROR { errorState: Deref @ COMP_FUNC { .. } }, _) => {
            let mut name_str: ArcStr = arcstr::literal!("");
            name_str = (InstNode::name(node.clone())?).clone();
            Error::addSourceMessage(Error::LOOKUP_CLASS_VIA_COMP_COMP.clone(), list![(name_str.clone()).clone(), (LookupStateName::toString(name.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ ERROR { errorState: Deref @ CLASS_COMP { .. } }, Deref @ COMP { .. }) => {
            let mut name_str: ArcStr = arcstr::literal!("");
            name_str = (InstNode::name(node.clone())?).clone();
            Error::addSourceMessage(Error::CLASS_IN_COMPOSITE_COMP_NAME.clone(), list![(name_str.clone()).clone(), (LookupStateName::toString(name.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ ERROR { errorState: Deref @ CLASS_COMP { .. } }, _) => {
            let mut name_str: ArcStr = arcstr::literal!("");
            name_str = (InstNode::name(node.clone())?).clone();
            Error::addSourceMessage(Error::LOOKUP_CLASS_VIA_COMP_COMP.clone(), list![(name_str.clone()).clone(), (LookupStateName::toString(name.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ ERROR { errorState: Deref @ IMPORT { .. } }, _) => {
            let mut name_str: ArcStr = arcstr::literal!("");
            name_str = (InstNode::name(node.clone())?).clone();
            Error::addSourceMessage(Error::IMPORT_IN_COMPOSITE_NAME.clone(), list![(name_str.clone()).clone(), (LookupStateName::toString(name.clone())?).clone()], info.clone())?;
            bail!("fail")
        },
        (Deref @ ERROR { errorState: Deref @ PARTIAL_CLASS { .. } }, _) => {
            let mut node2: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            if !(InstContext::inRelaxed(context.clone()) || InstContext::inRedeclared(context.clone())) {
                node2 = listHead(InstNode::scopeList(node.clone(), false, metamodelica::nil())?)?;
                if InstNode::isComponent(node2.clone()) {
                    Error::addMultiSourceMessage(Error::USE_OF_PARTIAL_CLASS.clone(), list![(InstNode::name(node2.clone())?).clone(), (InstNode::name(node.clone())?).clone(), (AbsynUtil::pathString(Class::constrainingClassPath(node.clone())?, (literal!(".")).clone(), true, false)?).clone()], list![InstNode::info(node.clone())?, InstNode::info(node2.clone())?])?;
                } else {
                    Error::addSourceMessage(Error::LOOKUP_IN_PARTIAL_CLASS.clone(), list![(InstNode::name(node.clone())?).clone()], info.clone())?;
                }
                bail!("fail");
            }
            ()
        },
        (Deref @ ERROR { errorState: Deref @ NON_CONSTANT { .. } }, _) => {
            Error::addMultiSourceMessage(Error::NON_CONSTANT_IN_ENCLOSING_SCOPE.clone(), list![(InstNode::name(node.clone())?).clone()], list![InstNode::info(node.clone())?, info.clone()])?;
            bail!("fail")
        },
        (Deref @ ERROR { errorState: Deref @ NON_ENCAPSULATED { .. } }, _) => {
            Error::addMultiSourceMessage(Error::NON_ENCAPSULATED_CLASS_ACCESS.clone(), list![(InstNode::name(InstNode::parent(node.clone()))?).clone(), (InstNode::name(node.clone())?).clone()], list![InstNode::info(node.clone())?, info.clone()])?;
            bail!("fail")
        },
        (_, Deref @ IMPORT { .. }) => {
            ()
        },
        _ => {
            printFoundWrongTypeError(endState.clone(), expectedState.clone(), name.clone(), info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn isError(mut state: Arc<LookupState>) -> bool {
        let mut isError: bool = false;
        isError = (::match_deref::match_deref! { match &(state.clone()) {
        Deref @ ERROR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isError
    }

    pub fn lookupStateString(mut state: Arc<LookupState>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(state.clone()) {
        Deref @ BEGIN { .. } => literal!("<begin>"),
        Deref @ COMP { .. } => System::gettext((literal!("component")).clone()),
        Deref @ CLASS_COMP { .. } => System::gettext((literal!("component")).clone()),
        Deref @ COMP_CLASS { .. } => System::gettext((literal!("class")).clone()),
        Deref @ COMP_FUNC { .. } => System::gettext((literal!("function")).clone()),
        Deref @ PACKAGE { .. } => System::gettext((literal!("package")).clone()),
        Deref @ CLASS { .. } => System::gettext((literal!("class")).clone()),
        Deref @ FUNC { .. } => System::gettext((literal!("function")).clone()),
        Deref @ PREDEF_COMP { .. } => System::gettext((literal!("component")).clone()),
        Deref @ PREDEF_CLASS { .. } => System::gettext((literal!("class")).clone()),
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(r#str)
    }

    pub fn printFoundWrongTypeError(mut foundState: Arc<LookupState>, mut expectedState: Arc<LookupState>, mut name: Arc<LookupStateName::LookupStateName>, mut info: SourceInfo) -> Result<()> {
        let mut name_str: ArcStr = arcstr::literal!("");
        let mut found_str: ArcStr = arcstr::literal!("");
        let mut expected_str: ArcStr = arcstr::literal!("");
        name_str = (LookupStateName::toString(name.clone())?).clone();
        found_str = (lookupStateString(foundState.clone())?).clone();
        expected_str = (lookupStateString(expectedState.clone())?).clone();
        Error::addSourceMessage(Error::LOOKUP_FOUND_WRONG_TYPE.clone(), list![(name_str.clone()).clone(), (expected_str.clone()).clone(), (found_str.clone()).clone()], info.clone())?;
        Ok(())
    }

    pub fn next(mut node: Arc<InstNode::InstNode>, mut currentState: Arc<LookupState>, mut context: i32, mut checkAccessViolations: bool) -> Result<Arc<LookupState>> {
        let mut nextState: Arc<LookupState> = Arc::new(LookupState::BEGIN);
        let mut entry_ty: Arc<LookupState> = Arc::new(LookupState::BEGIN);
        if checkAccessViolations.clone() && !(InstContext::inInstanceAPI(context.clone())) {
            checkProtection(node.clone(), currentState.clone())?;
        }
        entry_ty = nodeState(node.clone())?;
        nextState = next2(entry_ty.clone(), currentState.clone(), node.clone())?;
        Ok(nextState)
    }

    pub fn checkProtection(mut node: Arc<InstNode::InstNode>, mut currentState: Arc<LookupState>) -> Result<()> {
        let () = (::match_deref::match_deref! { match &(currentState.clone()) {
        Deref @ BEGIN { .. } => (),
        _ => {
            if InstNode::isProtected(node.clone()) && !(Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), (literal!("protectedAccess")).clone())?) {
                Error::addSourceMessage(Error::PROTECTED_ACCESS.clone(), list![(InstNode::name(node.clone())?).clone()], InstNode::info(node.clone())?)?;
                bail!("fail");
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

    pub fn nodeState(mut node: Arc<InstNode::InstNode>) -> Result<Arc<LookupState>> {
        let mut state: Arc<LookupState> = Arc::new(LookupState::BEGIN);
        if InstNode::isComponent(node.clone()) || InstNode::isName(node.clone()) || InstNode::isEmpty(node.clone()) {
            state = Arc::new(crate::NFLookupState::LookupState::COMP);
        } else {
            state = elementState(InstNode::definition(node.clone())?)?;
        }
        Ok(state)
    }

    pub fn elementState(mut element: Arc<SCode::Element>) -> Result<Arc<LookupState>> {
        let mut state: Arc<LookupState> = Arc::new(LookupState::BEGIN);
        state = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE, .. } => Arc::new(crate::NFLookupState::LookupState::PACKAGE),
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { .. }, .. } => Arc::new(crate::NFLookupState::LookupState::FUNC),
        Deref @ SCode::Element::CLASS { .. } => Arc::new(crate::NFLookupState::LookupState::CLASS),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookupState.LookupState.elementState")); __mm_s.push_str(&*literal!(" got unknown element.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(state)
    }

    pub fn next2(mut elementState: Arc<LookupState>, mut currentState: Arc<LookupState>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<LookupState>> {
        let mut nextState: Arc<LookupState> = Arc::new(LookupState::BEGIN);
        nextState = (::match_deref::match_deref! { match &((elementState.clone(), currentState.clone())) {
        (_, Deref @ BEGIN { .. }) => elementState.clone(),
        (Deref @ COMP { .. }, Deref @ COMP { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP),
        (Deref @ FUNC { .. }, Deref @ COMP { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP_FUNC),
        (_, Deref @ COMP { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP_CLASS),
        (Deref @ COMP { .. }, Deref @ CLASS_COMP { .. }) => Arc::new(crate::NFLookupState::LookupState::CLASS_COMP),
        (Deref @ CLASS_COMP { .. }, Deref @ CLASS_COMP { .. }) => Arc::new(crate::NFLookupState::LookupState::CLASS_COMP),
        (Deref @ COMP { .. }, Deref @ PACKAGE { .. }) => Arc::new(crate::NFLookupState::LookupState::CLASS_COMP),
        (_, Deref @ PACKAGE { .. }) => elementState.clone(),
        (Deref @ COMP { .. }, Deref @ CLASS { .. }) => Arc::new(crate::NFLookupState::LookupState::CLASS_COMP),
        (_, Deref @ CLASS { .. }) => elementState.clone(),
        (Deref @ COMP { .. }, Deref @ FUNC { .. }) => Arc::new(crate::NFLookupState::LookupState::CLASS_COMP),
        (_, Deref @ FUNC { .. }) => elementState.clone(),
        (Deref @ FUNC { .. }, Deref @ COMP_CLASS { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP_FUNC),
        (Deref @ CLASS { .. }, Deref @ COMP_CLASS { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP_CLASS),
        (Deref @ PACKAGE { .. }, Deref @ COMP_CLASS { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP_CLASS),
        (Deref @ FUNC { .. }, Deref @ COMP_FUNC { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP_FUNC),
        (Deref @ CLASS { .. }, Deref @ COMP_FUNC { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP_CLASS),
        (Deref @ PACKAGE { .. }, Deref @ COMP_FUNC { .. }) => Arc::new(crate::NFLookupState::LookupState::COMP_CLASS),
        (Deref @ COMP { .. }, _) => Arc::new(LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::COMP_FUNC) }),
        (_, Deref @ CLASS_COMP { .. }) => Arc::new(LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::CLASS_COMP) }),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFLookupState.LookupState.next2")); __mm_s.push_str(&*literal!(" failed on unknown transition for element ")); __mm_s.push_str(&*InstNode::name(node.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(nextState)
    }

    pub fn checkCrefVariability(mut cref: Arc<ComponentRef::NFComponentRef>, mut inEnclosingScope: bool, mut context: i32, mut state: Arc<LookupState>) -> Result<Arc<LookupState>> {
        let mut state: Arc<LookupState> = state;
        if isError(state.clone()) {
            return Ok(state.clone());
        }
        if inEnclosingScope.clone() && !(InstContext::inRelaxed(context.clone())) && isNonConstantComponent(ComponentRef::node(cref.clone())?)? {
            state = Arc::new(LookupState::ERROR { errorState: Arc::new(crate::NFLookupState::LookupState::NON_CONSTANT) });
        }
        Ok(state)
    }

    pub fn isNonConstantComponent(mut node: Arc<InstNode::InstNode>) -> Result<bool> {
        let mut res: bool = false;
        res = InstNode::isComponent(node.clone()) && !(Component::isConst(InstNode::component(node.clone())?));
        Ok(res)
    }

}

