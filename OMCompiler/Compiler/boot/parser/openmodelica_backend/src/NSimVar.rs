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

use crate::BackendDAE as OldBackendDAE;
use crate::NBEquation::Equation;
use crate::NBEvents::Condition;
use crate::NBEvents::EventInfo;
use crate::NBPartition::Partition;
use crate::NBSlice as Slice;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use crate::NSimCode as SimCode;
use crate::NSimCode::SimCodeIndices;
use crate::SimCode as OldSimCode;
use crate::SimCodeVar as OldSimCodeVar;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Old Backend imports
// Backend imports
// Old Simcode imports
// SimCode imports
// Util imports
pub mod SimVar {
    use super::*;
    /// Information about a variable in a Modelica model.
#[derive(Clone, Debug, PartialEq)]
    pub struct SimVar {
        pub name: Arc<ComponentRef::NFComponentRef>,
        pub varKind: Arc<VariableKind::VariableKind>,
        pub comment: ArcStr,
        pub unit: ArcStr,
        pub displayUnit: ArcStr,
        pub index: i32,
        pub min: Option<Arc<Expression::NFExpression>>,
        pub max: Option<Arc<Expression::NFExpression>>,
        pub start: Option<Arc<Expression::NFExpression>>,
        pub nominal: Option<Arc<Expression::NFExpression>>,
        pub isFixed: bool,
        pub type_: Arc<Type::NFType>,
        pub isDiscrete: bool,
        /// the name of the array if this variable is the first in that array
        pub arrayCref: Option<Arc<ComponentRef::NFComponentRef>>,
        pub aliasvar: Arc<Alias::Alias>,
        pub info: SourceInfo,
        pub causality: Option<Causality>,
        /// valueReference
        pub variable_index: Option<i32>,
        /// index of variable in modelDescription.xml
        pub fmi_index: Option<i32>,
        pub numArrayElement: Arc<metamodelica::List<ArcStr>>,
        pub isValueChangeable: bool,
        pub isProtected: bool,
        pub hideResult: bool,
        pub isEncrypted: bool,
        pub inputIndex: Option<metamodelica::Array<i32>>,
        /// if the varibale is a jacobian var, this is the corresponding matrix
        pub matrixName: Option<ArcStr>,
        /// FMI-2.0 variabilty attribute
        pub variability: Option<Variability>,
        /// FMI-2.0 initial attribute
        pub initial_: Option<Initial>,
        /// variables will only be exported to the modelDescription.xml if this attribute is SOME(cref) and this cref is only used in ModelDescription.xml for FMI-2.0 export
        pub exportVar: Option<Arc<ComponentRef::NFComponentRef>>,
    }

    pub type SIMVAR = SimVar;

    pub fn toString(mut var: Arc<SimVar>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var.index.clone())); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*VariableKind::toString(var.varKind.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(size(var.clone()))); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*Type::toString(var.type_.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ComponentRef::toString(var.name.clone())?); ArcStr::from(__mm_s) }).clone();
        if isSome(var.start.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(Util::getOption(var.start.clone())?)?); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn listToString(mut var_lst: Arc<metamodelica::List<Arc<SimVar>>>, mut r#str: ArcStr, mut printAlias: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        if !(var_lst.clone().is_empty()) {
            r#str = (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((var_lst.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())).clone();
            for mut var in &*var_lst.clone() {
                let mut var = var.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toString(var.clone(), (literal!("  ")).clone())?); ArcStr::from(__mm_s) }).clone();
                r#str = (if (printAlias.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*Alias::toString(var.aliasvar.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }}).clone();
            }
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = (literal!("")).clone();
        }
        Ok(r#str)
    }

    pub fn create(mut var: Arc<Variable::NFVariable>, mut uniqueIndex: i32, mut typeIndex: i32, mut alias: Arc<Alias::Alias>) -> Result<Arc<SimVar>> {
        let mut simVar: Arc<SimVar>;
        simVar = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { .. } => {
            let mut varKind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
            let mut comment: ArcStr = arcstr::literal!("");
            let mut unit: ArcStr = arcstr::literal!("");
            let mut displayUnit: ArcStr = arcstr::literal!("");
            let mut min: Option<Arc<Expression::NFExpression>> = None;
            let mut max: Option<Arc<Expression::NFExpression>> = None;
            let mut start: Option<Arc<Expression::NFExpression>> = None;
            let mut nominal: Option<Arc<Expression::NFExpression>> = None;
            let mut isFixed: bool = false;
            let mut isDiscrete: bool = false;
            let mut isProtected: bool = false;
            let mut isValueChangeable: bool = false;
            let mut causality: Causality = Causality::NONE;
            let mut result: Arc<SimVar>;
            comment = (parseComment(var.comment.clone())).clone();
            (varKind, unit, displayUnit, min, max, start, nominal, isFixed, isDiscrete, isProtected) = parseAttributes(var.backendinfo.clone())?;
            (start, isValueChangeable, causality) = parseBinding(start.clone(), var.clone());
            result = Arc::new(SimVar { exportVar: None, initial_: None, variability: None, matrixName: None, inputIndex: None, isEncrypted: Variable::isEncrypted(var.clone())?, hideResult: var.backendinfo.annotations.hideResult.clone(), isProtected: isProtected.clone(), isValueChangeable: isValueChangeable.clone(), numArrayElement: metamodelica::nil(), fmi_index: Some(typeIndex.clone()), variable_index: Some(uniqueIndex.clone()), causality: Some(causality.clone()), info: var.info.clone(), aliasvar: alias.clone(), arrayCref: ComponentRef::getArrayCrefOpt(var.name.clone())?, isDiscrete: isDiscrete.clone(), type_: var.ty.clone(), isFixed: isFixed.clone(), nominal: nominal.clone(), start: start.clone(), max: max.clone(), min: min.clone(), index: typeIndex.clone(), displayUnit: (displayUnit.clone()).clone(), unit: (unit.clone()).clone(), comment: (comment.clone()).clone(), varKind: varKind.clone(), name: var.name.clone() });
            result.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVar.create")); __mm_s.push_str(&*literal!(" failed for variable ")); __mm_s.push_str(&*ComponentRef::toString(var.name.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(simVar)
    }

    pub fn traverseCreate(mut var: Arc<Variable::NFVariable>, mut acc: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar>>>>, mut indices_ptr: Pointer::Pointer<SimCodeIndices>, mut varType: VarType) -> Result<Arc<Variable::NFVariable>> {
        let mut var: Arc<Variable::NFVariable> = var;
        let mut simCodeIndices: SimCodeIndices = Pointer::access(indices_ptr.clone());
        let _ = (match varType.clone() {
        VarType::SIMULATION => {
            Pointer::update(acc.clone(), cons(create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.realVarIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(acc.clone())));
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            simCodeIndices.realVarIndex = simCodeIndices.realVarIndex.clone() + 1;
            ()
        },
        VarType::PARAMETER { .. } => {
            Pointer::update(acc.clone(), cons(create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.realParamIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(acc.clone())));
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            simCodeIndices.realParamIndex = simCodeIndices.realParamIndex.clone() + 1;
            ()
        },
        VarType::ALIAS { .. } => {
            Pointer::update(acc.clone(), cons(create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.realAliasIndex.clone(), Alias::fromBinding(var.binding.clone())?)?, Pointer::access(acc.clone())));
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            simCodeIndices.realAliasIndex = simCodeIndices.realAliasIndex.clone() + 1;
            ()
        },
        VarType::RESIDUAL { .. } => {
            Pointer::update(acc.clone(), cons(create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.residualIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(acc.clone())));
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            simCodeIndices.residualIndex = simCodeIndices.residualIndex.clone() + 1;
            ()
        },
        VarType::EXTERNAL_OBJECT { .. } => {
            Pointer::update(acc.clone(), cons(create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.extObjIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(acc.clone())));
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            simCodeIndices.extObjIndex = simCodeIndices.extObjIndex.clone() + 1;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVar.traverseCreate")); __mm_s.push_str(&*literal!(" failed for variable ")); __mm_s.push_str(&*ComponentRef::toString(var.name.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
        Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
        Ok(var)
    }

    pub fn createFromResidualComponent(mut comp: Arc<StrongComponent::NBStrongComponent>, mut acc: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar>>>>, mut indices_ptr: Pointer::Pointer<SimCodeIndices>, mut varType: VarType) -> Result<Arc<StrongComponent::NBStrongComponent>> {
        let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
        let _ = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } if (Equation::isResidual(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone())) => {
            traverseCreate(Pointer::access(Equation::getResidualVar(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone())?), acc.clone(), indices_ptr.clone(), varType.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(comp)
    }

    pub fn size(mut var: Arc<SimVar>) -> i32 {
        let mut s: i32 = Type::sizeOf(var.type_.clone(), false).unwrap();
        s
    }

    pub fn getName(mut var: Arc<SimVar>) -> Arc<ComponentRef::NFComponentRef> {
        let mut name: Arc<ComponentRef::NFComponentRef> = var.name.clone();
        name
    }

    pub fn getIndex(mut cref: Arc<ComponentRef::NFComponentRef>, mut sim_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar>>>) -> Result<i32> {
        let mut index: i32 = 0;
        let mut var: Arc<SimVar>;
        match '__try0: {
            var = unwrap_break_err!(UnorderedMap::getSafe(cref.clone(), sim_map.clone(), metamodelica::sourceInfo!()), '__try0);
            index = var.index.clone();
            Ok::<_, anyhow::Error>((index.clone(), var.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                index = __try0_o0;
                var = __try0_o1;
            }
            Err(_) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVar.getIndex")); __mm_s.push_str(&*literal!(" failed to get index for cref: ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
        }
        Ok(index)
    }

    pub fn shiftIndex(mut var: Arc<SimVar>, mut shift: i32) -> Result<Arc<SimVar>> {
        let mut var: Arc<SimVar> = var;
        assign_field!(var.index = var.index.clone() + shift.clone());
        if isSome(var.fmi_index.clone()) {
            assign_field!(var.fmi_index = Some(Util::getOption(var.fmi_index.clone())? + shift.clone()));
        }
        Ok(var)
    }

    pub fn convert(mut simVar: Arc<SimVar>) -> Result<OldSimCodeVar::SimVar> {
        let mut oldSimVar: OldSimCodeVar::SimVar;
        oldSimVar = OldSimCodeVar::SimVar { relativeQuantity: false, exportVar: Util::applyOption(simVar.exportVar.clone(), Arc::new(ComponentRef::toDAE)), initial_: None, variability: None, matrixName: simVar.matrixName.clone(), initNonlinear: false, inputIndex: simVar.inputIndex.clone(), isEncrypted: simVar.isEncrypted.clone(), hideResult: Some(simVar.hideResult.clone()), isProtected: simVar.isProtected.clone(), isValueChangeable: simVar.isValueChangeable.clone(), numArrayElement: simVar.numArrayElement.clone(), fmi_index: simVar.fmi_index.clone(), variable_index: simVar.variable_index.clone(), causality: None, source: DAE::emptyElementSource.clone(), aliasvar: Alias::convert(simVar.aliasvar.clone())?, arrayCref: Util::applyOption(simVar.arrayCref.clone(), Arc::new(ComponentRef::toDAE)), isDiscrete: simVar.isDiscrete.clone(), type_: Type::toDAE(simVar.type_.clone(), true)?, isFixed: simVar.isFixed.clone(), nominalValue: Util::applyOption(simVar.nominal.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| Expression::toDAE(__pe_a0, __pe_b1.clone()) })), initialValue: Util::applyOption(simVar.start.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| Expression::toDAE(__pe_a0, __pe_b1.clone()) })), maxValue: Util::applyOption(simVar.max.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| Expression::toDAE(__pe_a0, __pe_b1.clone()) })), minValue: Util::applyOption(simVar.min.clone(), Arc::new({ let __pe_b1 = false; move |__pe_a0| Expression::toDAE(__pe_a0, __pe_b1.clone()) })), index: simVar.index.clone(), displayUnit: (simVar.displayUnit.clone()).clone(), unit: (simVar.unit.clone()).clone(), comment: (simVar.comment.clone()).clone(), varKind: convertVarKind(simVar.varKind.clone())?, name: ComponentRef::toDAE(simVar.name.clone())? };
        Ok(oldSimVar)
    }

    pub fn convertList(mut simVar_lst: Arc<metamodelica::List<Arc<SimVar>>>) -> Arc<metamodelica::List<OldSimCodeVar::SimVar>> {
        let mut oldSimVar_lst: Arc<metamodelica::List<OldSimCodeVar::SimVar>> = {
        let mut __acc: Arc<metamodelica::List<OldSimCodeVar::SimVar>> = metamodelica::nil();
        for mut simVar in (simVar_lst.clone()).into_iter().cloned() {
            let __x = convert(simVar.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        oldSimVar_lst
    }

    pub fn convertTpl(mut tpl: (Arc<SimVar>, bool)) -> Result<(OldSimCodeVar::SimVar, bool)> {
        let mut oldTpl: (OldSimCodeVar::SimVar, bool);
        let mut var: Arc<SimVar>;
        let mut b: bool = false;
        (var, b) = tpl.clone();
        oldTpl = (convert(var.clone())?, b.clone());
        Ok(oldTpl)
    }

    fn parseAttributes(mut backendInfo: Arc<BackendInfo::BackendInfo>) -> Result<(Arc<VariableKind::VariableKind>, ArcStr, ArcStr, Option<Arc<Expression::NFExpression>>, Option<Arc<Expression::NFExpression>>, Option<Arc<Expression::NFExpression>>, Option<Arc<Expression::NFExpression>>, bool, bool, bool)> {
        let mut varKind: Arc<VariableKind::VariableKind> = Arc::new(VariableKind::ALGEBRAIC);
        let mut unit: ArcStr = literal!("");
        let mut displayUnit: ArcStr = literal!("");
        let mut min: Option<Arc<Expression::NFExpression>> = None;
        let mut max: Option<Arc<Expression::NFExpression>> = None;
        let mut start: Option<Arc<Expression::NFExpression>> = None;
        let mut nominal: Option<Arc<Expression::NFExpression>> = None;
        let mut isFixed: bool = false;
        let mut isDiscrete: bool = false;
        let mut isProtected: bool = false;
        let () = (::match_deref::match_deref! { match &(backendInfo.clone()) {
        Deref @ BackendInfo::BACKEND_INFO { attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_REAL { .. }, varKind, .. } => {
            unit = (Util::applyOptionOrDefault(var_field!((**varAttr).unit, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new(fnptr!(Expression::stringValue, Arc<Expression::NFExpression>)), (literal!("")).clone())).clone();
            displayUnit = (Util::applyOptionOrDefault(var_field!((**varAttr).displayUnit, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new(fnptr!(Expression::stringValue, Arc<Expression::NFExpression>)), (literal!("")).clone())).clone();
            min = var_field!((**varAttr).min, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone();
            max = var_field!((**varAttr).max, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone();
            start = var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone();
            nominal = var_field!((**varAttr).nominal, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone();
            isFixed = Util::applyOptionOrDefault(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), Arc::new(fnptr!(Expression::booleanValue, Arc<Expression::NFExpression>)), false);
            isDiscrete = (::match_deref::match_deref! { match &(varKind.clone()) {
        Deref @ VariableKind::DISCRETE => true,
        Deref @ VariableKind::DISCRETE_STATE => true,
        Deref @ VariableKind::PREVIOUS => true,
        Deref @ VariableKind::PARAMETER { .. } => true,
        Deref @ VariableKind::CONSTANT => true,
        Deref @ VariableKind::START { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), false);
            ()
        },
        Deref @ BackendInfo::BACKEND_INFO { attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_INT { .. }, varKind, .. } => {
            min = var_field!((**varAttr).min, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone();
            max = var_field!((**varAttr).max, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone();
            start = var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone();
            isFixed = Util::applyOptionOrDefault(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), Arc::new(fnptr!(Expression::booleanValue, Arc<Expression::NFExpression>)), false);
            isDiscrete = true;
            isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), false);
            ()
        },
        Deref @ BackendInfo::BACKEND_INFO { attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_BOOL { .. }, varKind, .. } => {
            start = var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone();
            isFixed = Util::applyOptionOrDefault(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), Arc::new(fnptr!(Expression::booleanValue, Arc<Expression::NFExpression>)), false);
            isDiscrete = true;
            isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), false);
            ()
        },
        Deref @ BackendInfo::BACKEND_INFO { attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_CLOCK { .. }, varKind, .. } => {
            isDiscrete = true;
            isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_CLOCK).clone(), false);
            ()
        },
        Deref @ BackendInfo::BACKEND_INFO { attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_STRING { .. }, varKind, .. } => {
            start = var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_STRING).clone();
            isFixed = Util::applyOptionOrDefault(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_STRING).clone(), Arc::new(fnptr!(Expression::booleanValue, Arc<Expression::NFExpression>)), false);
            isDiscrete = true;
            isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_STRING).clone(), false);
            ()
        },
        Deref @ BackendInfo::BACKEND_INFO { attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_ENUMERATION { .. }, varKind, .. } => {
            min = var_field!((**varAttr).min, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone();
            max = var_field!((**varAttr).max, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone();
            start = var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone();
            isFixed = Util::applyOptionOrDefault(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), Arc::new(fnptr!(Expression::booleanValue, Arc<Expression::NFExpression>)), false);
            isDiscrete = true;
            isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), false);
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVar.parseAttributes")); __mm_s.push_str(&*literal!(" failed because the BackendInfo could not be parsed:\n")); __mm_s.push_str(&*BackendInfo::toString(backendInfo.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((varKind, unit, displayUnit, min, max, start, nominal, isFixed, isDiscrete, isProtected))
    }

    fn parseComment(mut absynComment: Arc<SCode::Comment>) -> ArcStr {
        let mut commentStr: ArcStr = arcstr::literal!("");
        commentStr = ((::match_deref::match_deref! { match &(absynComment.clone()) {
        Deref @ SCode::Comment { comment: Some(commentStr), .. } => commentStr.clone(),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        commentStr
    }

    fn parseBinding(mut start: Option<Arc<Expression::NFExpression>>, mut var: Arc<Variable::NFVariable>) -> (Option<Arc<Expression::NFExpression>>, bool, Causality) {
        let mut start: Option<Arc<Expression::NFExpression>> = start;
        let mut isValueChangeable: bool = false;
        let mut causality: Causality = Causality::NONE;
        (start, isValueChangeable, causality) = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { .. }, .. }, binding: Deref @ Binding::TYPED_BINDING { bindingExp, variability: Prefixes::Variability::CONSTANT, .. }, .. } => {
            (Some(bindingExp.clone()), true, Causality::PARAMETER.clone())
        },
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { .. }, .. }, binding: Deref @ Binding::FLAT_BINDING { bindingExp, variability: Prefixes::Variability::CONSTANT, .. }, .. } => {
            (Some(bindingExp.clone()), true, Causality::PARAMETER.clone())
        },
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { .. }, .. }, .. } => {
            (start.clone(), false, Causality::CALCULATED_PARAMETER.clone())
        },
        _ => {
            (start.clone(), false, Causality::LOCAL.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (start, isValueChangeable, causality)
    }

    fn convertVarKind(mut varKind: Arc<VariableKind::VariableKind>) -> Result<OldBackendDAE::VarKind> {
        let mut oldVarKind: OldBackendDAE::VarKind = OldBackendDAE::VarKind::ALG_STATE;
        oldVarKind = (::match_deref::match_deref! { match &(varKind.clone()) {
        Deref @ VariableKind::ALGEBRAIC => {
            crate::BackendDAE::VarKind::VARIABLE
        },
        Deref @ VariableKind::STATE { .. } => {
            let mut var: Arc<Variable::NFVariable>;
            let mut oldCrefOpt: Option<Arc<DAE::ComponentRef>> = None;
            if isSome(var_field!((*varKind).derivative, VariableKind::VariableKind::STATE).clone()) {
                var = Pointer::access(Util::getOption(var_field!((*varKind).derivative, VariableKind::VariableKind::STATE).clone())?);
                oldCrefOpt = Some(ComponentRef::toDAE(var.name.clone())?);
            } else {
                oldCrefOpt = None;
            }
            OldBackendDAE::VarKind::STATE { index: var_field!((*varKind).index, VariableKind::VariableKind::STATE).clone(), derName: oldCrefOpt.clone(), natural: var_field!((*varKind).natural, VariableKind::VariableKind::STATE).clone() }
        },
        Deref @ VariableKind::STATE_DER { .. } => {
            crate::BackendDAE::VarKind::STATE_DER
        },
        Deref @ VariableKind::DUMMY_DER { .. } => {
            crate::BackendDAE::VarKind::DUMMY_DER
        },
        Deref @ VariableKind::DUMMY_STATE { .. } => {
            crate::BackendDAE::VarKind::DUMMY_STATE
        },
        Deref @ VariableKind::DISCRETE => {
            crate::BackendDAE::VarKind::DISCRETE
        },
        Deref @ VariableKind::DISCRETE_STATE => {
            crate::BackendDAE::VarKind::DISCRETE
        },
        Deref @ VariableKind::CLOCKED => {
            crate::BackendDAE::VarKind::DISCRETE
        },
        Deref @ VariableKind::PREVIOUS => {
            crate::BackendDAE::VarKind::DISCRETE
        },
        Deref @ VariableKind::PARAMETER { .. } => {
            crate::BackendDAE::VarKind::PARAM
        },
        Deref @ VariableKind::CONSTANT => {
            crate::BackendDAE::VarKind::CONST
        },
        Deref @ VariableKind::START { .. } => {
            crate::BackendDAE::VarKind::VARIABLE
        },
        Deref @ VariableKind::EXTOBJ { .. } => {
            OldBackendDAE::VarKind::EXTOBJ { fullClassName: var_field!((*varKind).fullClassName, VariableKind::VariableKind::EXTOBJ).clone() }
        },
        Deref @ VariableKind::JAC_VAR => {
            crate::BackendDAE::VarKind::JAC_VAR
        },
        Deref @ VariableKind::JAC_TMP_VAR => {
            crate::BackendDAE::VarKind::JAC_TMP_VAR
        },
        Deref @ VariableKind::SEED_VAR => {
            crate::BackendDAE::VarKind::SEED_VAR
        },
        Deref @ VariableKind::OPT_CONSTR => {
            crate::BackendDAE::VarKind::OPT_CONSTR
        },
        Deref @ VariableKind::OPT_FCONSTR => {
            crate::BackendDAE::VarKind::OPT_FCONSTR
        },
        Deref @ VariableKind::OPT_INPUT_WITH_DER => {
            crate::BackendDAE::VarKind::OPT_INPUT_WITH_DER
        },
        Deref @ VariableKind::OPT_INPUT_DER => {
            crate::BackendDAE::VarKind::OPT_INPUT_DER
        },
        Deref @ VariableKind::OPT_TGRID => {
            crate::BackendDAE::VarKind::OPT_TGRID
        },
        Deref @ VariableKind::OPT_LOOP_INPUT { .. } => {
            OldBackendDAE::VarKind::OPT_LOOP_INPUT { replaceExp: ComponentRef::toDAE(var_field!((*varKind).replaceCref, VariableKind::VariableKind::OPT_LOOP_INPUT).clone())? }
        },
        Deref @ VariableKind::ALG_STATE => {
            crate::BackendDAE::VarKind::ALG_STATE
        },
        Deref @ VariableKind::ALG_STATE_OLD => {
            crate::BackendDAE::VarKind::ALG_STATE_OLD
        },
        Deref @ VariableKind::RESIDUAL_VAR => {
            crate::BackendDAE::VarKind::DAE_RESIDUAL_VAR
        },
        Deref @ VariableKind::DAE_AUX_VAR => {
            crate::BackendDAE::VarKind::DAE_AUX_VAR
        },
        Deref @ VariableKind::LOOP_ITERATION => {
            crate::BackendDAE::VarKind::LOOP_ITERATION
        },
        Deref @ VariableKind::LOOP_SOLVED => {
            crate::BackendDAE::VarKind::LOOP_SOLVED
        },
        Deref @ VariableKind::FRONTEND_DUMMY => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVar.convertVarKind")); __mm_s.push_str(&*literal!(" failed because of wrong VariableKind FRONTEND_DUMMY(). This should not exist after frontend.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVar.convertVarKind")); __mm_s.push_str(&*literal!(" failed because of unhandled VariableKind ")); __mm_s.push_str(&*VariableKind::toString(varKind.clone())); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldVarKind)
    }

}

pub mod Alias {
    use super::*;
    #[derive(Clone, Debug, PartialEq)]
    pub enum Alias {
        NO_ALIAS,
        /// General alias expression with a coefficent.
        ///      var := gain * alias + offset
        ALIAS {
            /// The name of the alias variable.
            alias: Arc<ComponentRef::NFComponentRef>,
            /// = 1 for regular alias.
            gain: metamodelica::Real,
            /// = 0 for regular alias.
            offset: metamodelica::Real,
        },
    }
    impl Default for Alias {
        fn default() -> Self { Self::NO_ALIAS }
    }
    pub use self::Alias::{NO_ALIAS,ALIAS};
    pub fn fromBinding(mut binding: Arc<Binding::NFBinding>) -> Result<Arc<Alias>> {
        let mut alias: Arc<Alias> = Arc::new(Alias::NO_ALIAS);
        alias = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { .. } => getAlias(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone())?,
        Deref @ Binding::FLAT_BINDING { .. } => getAlias(var_field!((*binding).bindingExp, Binding::NFBinding::FLAT_BINDING).clone())?,
        _ => Arc::new(crate::NSimVar::Alias::NO_ALIAS),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(alias)
    }

    pub fn toString(mut alias: Arc<Alias>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((::match_deref::match_deref! { match &(alias.clone()) {
        Deref @ NO_ALIAS { .. } => {
            literal!("(no alias)")
        },
        Deref @ ALIAS { .. } => {
            let mut gainStr: ArcStr = arcstr::literal!("");
            let mut offsetStr: ArcStr = arcstr::literal!("");
            gainStr = (if (var_field!((*alias).gain, Alias::ALIAS).clone() == metamodelica::OrderedFloat(1.0_f64)) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*realString(var_field!((*alias).gain, Alias::ALIAS).clone())); __mm_s.push_str(&*literal!("*")); ArcStr::from(__mm_s) }}).clone();
            offsetStr = (if (var_field!((*alias).offset, Alias::ALIAS).clone() == metamodelica::OrderedFloat(0.0_f64)) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("+")); __mm_s.push_str(&*realString(var_field!((*alias).offset, Alias::ALIAS).clone())); ArcStr::from(__mm_s) }}).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(bound alias: ")); __mm_s.push_str(&*gainStr.clone()); __mm_s.push_str(&*ComponentRef::toString(var_field!((*alias).alias, Alias::ALIAS).clone())?); __mm_s.push_str(&*offsetStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(r#str)
    }

    pub fn convert(mut alias: Arc<Alias>) -> Result<OldSimCodeVar::AliasVariable> {
        let mut oldAlias: OldSimCodeVar::AliasVariable = OldSimCodeVar::AliasVariable::NOALIAS;
        oldAlias = (::match_deref::match_deref! { match &(alias.clone()) {
        Deref @ NO_ALIAS { .. } => crate::SimCodeVar::AliasVariable::NOALIAS,
        Deref @ ALIAS { .. } if (realEq(var_field!((*alias).gain, Alias::ALIAS).clone(), metamodelica::OrderedFloat(1.0_f64)) && realEq(var_field!((*alias).offset, Alias::ALIAS).clone(), metamodelica::OrderedFloat(0.0_f64))) => OldSimCodeVar::AliasVariable::ALIAS { varName: ComponentRef::toDAE(var_field!((*alias).alias, Alias::ALIAS).clone())? },
        Deref @ ALIAS { .. } if (realEq(var_field!((*alias).gain, Alias::ALIAS).clone(), metamodelica::OrderedFloat(-1.0_f64)) && realEq(var_field!((*alias).offset, Alias::ALIAS).clone(), metamodelica::OrderedFloat(0.0_f64))) => OldSimCodeVar::AliasVariable::NEGATEDALIAS { varName: ComponentRef::toDAE(var_field!((*alias).alias, Alias::ALIAS).clone())? },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.Alias.convert")); __mm_s.push_str(&*literal!(" failed because of unknown Alias type.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldAlias)
    }

    fn getAlias(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Alias>> {
        let mut alias: Arc<Alias> = Arc::new(Alias::NO_ALIAS);
        alias = (::match_deref::match_deref! { match &(SimplifyExp::simplify(exp.clone(), false)?) {
        e @ Deref @ Expression::CREF { .. } => {
            Arc::new(Alias::ALIAS { alias: var_field!((**e).cref, Expression::NFExpression::CREF).clone(), gain: metamodelica::OrderedFloat(1.0_f64), offset: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ Expression::UNARY { exp: e @ Deref @ Expression::CREF { .. }, .. } => {
            Arc::new(Alias::ALIAS { alias: var_field!((**e).cref, Expression::NFExpression::CREF).clone(), gain: metamodelica::OrderedFloat(-1.0_f64), offset: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ Expression::LUNARY { exp: e @ Deref @ Expression::CREF { .. }, .. } => {
            Arc::new(Alias::ALIAS { alias: var_field!((**e).cref, Expression::NFExpression::CREF).clone(), gain: metamodelica::OrderedFloat(-1.0_f64), offset: metamodelica::OrderedFloat(0.0_f64) })
        },
        _ => {
            Arc::new(crate::NSimVar::Alias::NO_ALIAS)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(alias)
    }

    fn getGainAlias(mut e1: Arc<Expression::NFExpression>, mut e2: Arc<Expression::NFExpression>) -> Result<(Arc<ComponentRef::NFComponentRef>, metamodelica::Real)> {
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut gain: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        (cref, gain) = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (Deref @ Expression::CREF { .. }, _) if (Expression::isConstNumber(e2.clone())) => (var_field!((*e1).cref, Expression::NFExpression::CREF).clone(), Expression::realValue(e2.clone())?),
        (_, Deref @ Expression::CREF { .. }) if (Expression::isConstNumber(e1.clone())) => (var_field!((*e2).cref, Expression::NFExpression::CREF).clone(), Expression::realValue(e1.clone())?),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.Alias.getGainAlias")); __mm_s.push_str(&*literal!(" cannot generate gain alias from Expressions: {")); __mm_s.push_str(&*Expression::toString(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(e2.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((cref, gain))
    }

    fn getOffsetAlias(mut e1: Arc<Expression::NFExpression>, mut e2: Arc<Expression::NFExpression>) -> Result<(Arc<ComponentRef::NFComponentRef>, metamodelica::Real, metamodelica::Real)> {
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut gain: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut offset: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        (cref, gain, offset) = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (Deref @ Expression::CREF { .. }, _) if (Expression::isConstNumber(e2.clone())) => {
            (var_field!((*e1).cref, Expression::NFExpression::CREF).clone(), metamodelica::OrderedFloat(0.0_f64), Expression::realValue(e2.clone())?)
        },
        (_, Deref @ Expression::CREF { .. }) if (Expression::isConstNumber(e1.clone())) => {
            (var_field!((*e2).cref, Expression::NFExpression::CREF).clone(), metamodelica::OrderedFloat(0.0_f64), Expression::realValue(e1.clone())?)
        },
        (Deref @ Expression::MULTARY { inv_arguments: Deref @ metamodelica::List::Nil, arguments: Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } }, .. }, _) if (Operator::getMathClassification(var_field!((*e1).operator, Expression::NFExpression::MULTARY).clone())? == Operator::MathClassification::MULTIPLICATION.clone() && Expression::isConstNumber(e2.clone())) => {
            (cref, gain) = getGainAlias(arg1.clone(), arg2.clone())?;
            (cref.clone(), gain.clone(), Expression::realValue(e2.clone())?)
        },
        (_, Deref @ Expression::MULTARY { inv_arguments: Deref @ metamodelica::List::Nil, arguments: Deref @ metamodelica::List::Cons { head: arg1, tail: Deref @ metamodelica::List::Cons { head: arg2, tail: Deref @ metamodelica::List::Nil } }, .. }) if (Operator::getMathClassification(var_field!((*e2).operator, Expression::NFExpression::MULTARY).clone())? == Operator::MathClassification::MULTIPLICATION.clone() && Expression::isConstNumber(e1.clone())) => {
            (cref, gain) = getGainAlias(arg1.clone(), arg2.clone())?;
            (cref.clone(), gain.clone(), Expression::realValue(e1.clone())?)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.Alias.getOffsetAlias")); __mm_s.push_str(&*literal!(" cannot generate offset alias from Expressions: {")); __mm_s.push_str(&*Expression::toString(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(e2.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((cref, gain, offset))
    }

}

// kabdelhak: i don't like "CALCULATED_PARAMETER", is there a better way to describe it?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Causality {
    NONE = 1,
    OUTPUT = 2,
    INPUT = 3,
    LOCAL = 4,
    PARAMETER = 5,
    CALCULATED_PARAMETER = 6,
}
impl PartialOrd for Causality {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Causality {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

// kabdelhak: where is the difference between approx and calculated?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Initial {
    NONE = 1,
    EXACT = 2,
    APPROX = 3,
    CALCULATED = 4,
}
impl PartialOrd for Initial {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Initial {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

// kabdelhak: i don't like "TUNABLE" -> just "VARIABLE"?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Variability {
    CONSTANT = 1,
    FIXED = 2,
    TUNABLE = 3,
    DISCRETE = 4,
    CONTINUOUS = 5,
}
impl PartialOrd for Variability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Variability {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub mod SimVars {
    use super::*;
    /// Container for metadata about variables in a Modelica model.
#[derive(Clone, Debug, PartialEq)]
    pub struct SimVars {
        pub stateVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub derivativeVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub algVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub discreteAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub intAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub boolAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub stringAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub enumAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub inputVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub outputVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub aliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub intAliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub boolAliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub stringAliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub enumAliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub paramVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub intParamVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub boolParamVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub stringParamVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub enumParamVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub extObjVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub constVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub intConstVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub boolConstVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub stringConstVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub enumConstVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub residualVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub jacobianVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub seedVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub realOptimizeConstraintsVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub realOptimizeFinalConstraintsVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        /// variable used to calculate sensitivities for parameters nSensitivitityParameters + nRealParam*nStates
        pub sensitivityVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub dataReconSetcVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub dataReconinputVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub dataReconSetBVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
    }

    pub type SIMVARS = SimVars;

    pub fn toString(mut vars: Arc<SimVars>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = (StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SimVars ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.stateVars.clone(), (literal!("States")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.derivativeVars.clone(), (literal!("Derivatives")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.algVars.clone(), (literal!("Algebraic Variables")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.discreteAlgVars.clone(), (literal!("Discrete Algebraic Variables")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.intAlgVars.clone(), (literal!("Integer Algebraic Variables")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.boolAlgVars.clone(), (literal!("Boolean Algebraic Variables")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.paramVars.clone(), (literal!("Real Parameters")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.intParamVars.clone(), (literal!("Integer Parameters")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.boolParamVars.clone(), (literal!("Boolean Parameters")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.residualVars.clone(), (literal!("Residual Variables")).clone(), false)?); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::listToString(vars.aliasVars.clone(), (literal!("Real Alias")).clone(), true)?); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn create(mut varData: Arc<BVariable::VarData::VarData>, mut residual_vars: Arc<VariablePointers::VariablePointers>, mut simCodeIndices: SimCodeIndices) -> Result<(Arc<SimVars>, SimCodeIndices)> {
        let mut simVars: Arc<SimVars>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut stateVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut derivativeVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut algVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut nonTrivialAlias: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut discreteAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut intAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut boolAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut stringAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut enumAlgVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut discreteAlgVars2: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut intAlgVars2: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut boolAlgVars2: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut stringAlgVars2: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut enumAlgVars2: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut discreteAlgVars3: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut intAlgVars3: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut boolAlgVars3: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut stringAlgVars3: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut enumAlgVars3: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut inputVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut outputVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut aliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut intAliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut boolAliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut stringAliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut enumAliasVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut paramVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut intParamVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut boolParamVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut stringParamVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut enumParamVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut paramVarsR: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut intParamVarsR: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut boolParamVarsR: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut stringParamVarsR: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut enumParamVarsR: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut constVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut intConstVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut boolConstVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut stringConstVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut enumConstVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut extObjVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut residualVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut jacobianVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut seedVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut realOptimizeConstraintsVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut realOptimizeFinalConstraintsVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut sensitivityVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut dataReconSetcVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut dataReconinputVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut dataReconSetBVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let () = (::match_deref::match_deref! { match &(varData.clone()) {
        Deref @ BVariable::VarData::VAR_DATA_SIM { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).states, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            stateVars = __pa0.clone();
            simCodeIndices = __pa1.clone();
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).derivatives, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil }, __pa4) => (__pa3.clone(), __pa4.clone()),
                _ => bail!("pattern mismatch"),
            } };
            derivativeVars = __pa3.clone();
            simCodeIndices = __pa4.clone();
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).algebraics, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil }, __pa7) => (__pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            algVars = __pa6.clone();
            simCodeIndices = __pa7.clone();
            let (__pa9, __pa10) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).top_level_inputs, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa9, tail: Deref @ metamodelica::List::Nil }, __pa10) => (__pa9.clone(), __pa10.clone()),
                _ => bail!("pattern mismatch"),
            } };
            inputVars = __pa9.clone();
            simCodeIndices = __pa10.clone();
            let (__pa12, __pa13) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).nonTrivialAlias, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Nil }, __pa13) => (__pa12.clone(), __pa13.clone()),
                _ => bail!("pattern mismatch"),
            } };
            nonTrivialAlias = __pa12.clone();
            simCodeIndices = __pa13.clone();
            let (__pa15, __pa16, __pa17, __pa18, __pa19, __pa20) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).discretes, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::TYPE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa15, tail: Deref @ metamodelica::List::Cons { head: __pa16, tail: Deref @ metamodelica::List::Cons { head: __pa17, tail: Deref @ metamodelica::List::Cons { head: __pa18, tail: Deref @ metamodelica::List::Cons { head: __pa19, tail: Deref @ metamodelica::List::Nil } } } } }, __pa20) => (__pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone()),
                _ => bail!("pattern mismatch"),
            } };
            discreteAlgVars = __pa15.clone();
            intAlgVars = __pa16.clone();
            boolAlgVars = __pa17.clone();
            stringAlgVars = __pa18.clone();
            enumAlgVars = __pa19.clone();
            simCodeIndices = __pa20.clone();
            let (__pa22, __pa23, __pa24, __pa25, __pa26, __pa27) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).discrete_states, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::TYPE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa22, tail: Deref @ metamodelica::List::Cons { head: __pa23, tail: Deref @ metamodelica::List::Cons { head: __pa24, tail: Deref @ metamodelica::List::Cons { head: __pa25, tail: Deref @ metamodelica::List::Cons { head: __pa26, tail: Deref @ metamodelica::List::Nil } } } } }, __pa27) => (__pa22.clone(), __pa23.clone(), __pa24.clone(), __pa25.clone(), __pa26.clone(), __pa27.clone()),
                _ => bail!("pattern mismatch"),
            } };
            discreteAlgVars2 = __pa22.clone();
            intAlgVars2 = __pa23.clone();
            boolAlgVars2 = __pa24.clone();
            stringAlgVars2 = __pa25.clone();
            enumAlgVars2 = __pa26.clone();
            simCodeIndices = __pa27.clone();
            let (__pa29, __pa30, __pa31, __pa32, __pa33, __pa34) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).clocked_states, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::TYPE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa29, tail: Deref @ metamodelica::List::Cons { head: __pa30, tail: Deref @ metamodelica::List::Cons { head: __pa31, tail: Deref @ metamodelica::List::Cons { head: __pa32, tail: Deref @ metamodelica::List::Cons { head: __pa33, tail: Deref @ metamodelica::List::Nil } } } } }, __pa34) => (__pa29.clone(), __pa30.clone(), __pa31.clone(), __pa32.clone(), __pa33.clone(), __pa34.clone()),
                _ => bail!("pattern mismatch"),
            } };
            discreteAlgVars3 = __pa29.clone();
            intAlgVars3 = __pa30.clone();
            boolAlgVars3 = __pa31.clone();
            stringAlgVars3 = __pa32.clone();
            enumAlgVars3 = __pa33.clone();
            simCodeIndices = __pa34.clone();
            let (__pa36, __pa37, __pa38, __pa39, __pa40, __pa41) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).aliasVars, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::TYPE.clone(), VarType::ALIAS.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa36, tail: Deref @ metamodelica::List::Cons { head: __pa37, tail: Deref @ metamodelica::List::Cons { head: __pa38, tail: Deref @ metamodelica::List::Cons { head: __pa39, tail: Deref @ metamodelica::List::Cons { head: __pa40, tail: Deref @ metamodelica::List::Nil } } } } }, __pa41) => (__pa36.clone(), __pa37.clone(), __pa38.clone(), __pa39.clone(), __pa40.clone(), __pa41.clone()),
                _ => bail!("pattern mismatch"),
            } };
            aliasVars = __pa36.clone();
            intAliasVars = __pa37.clone();
            boolAliasVars = __pa38.clone();
            stringAliasVars = __pa39.clone();
            enumAliasVars = __pa40.clone();
            simCodeIndices = __pa41.clone();
            let (__pa43, __pa44, __pa45, __pa46, __pa47, __pa48) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).parameters, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::TYPE.clone(), VarType::PARAMETER.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa43, tail: Deref @ metamodelica::List::Cons { head: __pa44, tail: Deref @ metamodelica::List::Cons { head: __pa45, tail: Deref @ metamodelica::List::Cons { head: __pa46, tail: Deref @ metamodelica::List::Cons { head: __pa47, tail: Deref @ metamodelica::List::Nil } } } } }, __pa48) => (__pa43.clone(), __pa44.clone(), __pa45.clone(), __pa46.clone(), __pa47.clone(), __pa48.clone()),
                _ => bail!("pattern mismatch"),
            } };
            paramVars = __pa43.clone();
            intParamVars = __pa44.clone();
            boolParamVars = __pa45.clone();
            stringParamVars = __pa46.clone();
            enumParamVars = __pa47.clone();
            simCodeIndices = __pa48.clone();
            let (__pa50, __pa51, __pa52, __pa53, __pa54, __pa55) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).resizables, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::TYPE.clone(), VarType::PARAMETER.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa50, tail: Deref @ metamodelica::List::Cons { head: __pa51, tail: Deref @ metamodelica::List::Cons { head: __pa52, tail: Deref @ metamodelica::List::Cons { head: __pa53, tail: Deref @ metamodelica::List::Cons { head: __pa54, tail: Deref @ metamodelica::List::Nil } } } } }, __pa55) => (__pa50.clone(), __pa51.clone(), __pa52.clone(), __pa53.clone(), __pa54.clone(), __pa55.clone()),
                _ => bail!("pattern mismatch"),
            } };
            paramVarsR = __pa50.clone();
            intParamVarsR = __pa51.clone();
            boolParamVarsR = __pa52.clone();
            stringParamVarsR = __pa53.clone();
            enumParamVarsR = __pa54.clone();
            simCodeIndices = __pa55.clone();
            let (__pa57, __pa58, __pa59, __pa60, __pa61, __pa62) = ::match_deref::match_deref! { match &(createSimVarLists(var_field!((*varData).constants, BVariable::VarData::VarData::VAR_DATA_SIM).clone(), simCodeIndices.clone(), SplitType::TYPE.clone(), VarType::SIMULATION.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa57, tail: Deref @ metamodelica::List::Cons { head: __pa58, tail: Deref @ metamodelica::List::Cons { head: __pa59, tail: Deref @ metamodelica::List::Cons { head: __pa60, tail: Deref @ metamodelica::List::Cons { head: __pa61, tail: Deref @ metamodelica::List::Nil } } } } }, __pa62) => (__pa57.clone(), __pa58.clone(), __pa59.clone(), __pa60.clone(), __pa61.clone(), __pa62.clone()),
                _ => bail!("pattern mismatch"),
            } };
            constVars = __pa57.clone();
            intConstVars = __pa58.clone();
            boolConstVars = __pa59.clone();
            stringConstVars = __pa60.clone();
            enumConstVars = __pa61.clone();
            simCodeIndices = __pa62.clone();
            let (__pa64, __pa65) = ::match_deref::match_deref! { match &(createSimVarLists(residual_vars.clone(), simCodeIndices.clone(), SplitType::NONE.clone(), VarType::RESIDUAL.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa64, tail: Deref @ metamodelica::List::Nil }, __pa65) => (__pa64.clone(), __pa65.clone()),
                _ => bail!("pattern mismatch"),
            } };
            residualVars = __pa64.clone();
            simCodeIndices = __pa65.clone();
            ()
        },
        Deref @ BVariable::VarData::VAR_DATA_JAC { .. } => (),
        Deref @ BVariable::VarData::VAR_DATA_HES { .. } => (),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVars.create")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        simVars = Arc::new(SimVars { dataReconSetBVars: dataReconSetBVars.clone(), dataReconinputVars: dataReconinputVars.clone(), dataReconSetcVars: dataReconSetcVars.clone(), sensitivityVars: sensitivityVars.clone(), realOptimizeFinalConstraintsVars: realOptimizeFinalConstraintsVars.clone(), realOptimizeConstraintsVars: realOptimizeConstraintsVars.clone(), seedVars: seedVars.clone(), jacobianVars: jacobianVars.clone(), residualVars: residualVars.clone(), enumConstVars: enumConstVars.clone(), stringConstVars: stringConstVars.clone(), boolConstVars: boolConstVars.clone(), intConstVars: intConstVars.clone(), constVars: constVars.clone(), extObjVars: extObjVars.clone(), enumParamVars: List::flatten(list![enumParamVars.clone(), enumParamVarsR.clone()]), stringParamVars: List::flatten(list![stringParamVars.clone(), stringParamVarsR.clone()]), boolParamVars: List::flatten(list![boolParamVars.clone(), boolParamVarsR.clone()]), intParamVars: List::flatten(list![intParamVars.clone(), intParamVarsR.clone()]), paramVars: List::flatten(list![paramVars.clone(), paramVarsR.clone()]), enumAliasVars: enumAliasVars.clone(), stringAliasVars: stringAliasVars.clone(), boolAliasVars: boolAliasVars.clone(), intAliasVars: intAliasVars.clone(), aliasVars: aliasVars.clone(), outputVars: outputVars.clone(), inputVars: inputVars.clone(), enumAlgVars: List::flatten(list![enumAlgVars.clone(), enumAlgVars2.clone(), enumAlgVars3.clone()]), stringAlgVars: List::flatten(list![stringAlgVars.clone(), stringAlgVars2.clone(), stringAlgVars3.clone()]), boolAlgVars: List::flatten(list![boolAlgVars.clone(), boolAlgVars2.clone(), boolAlgVars3.clone()]), intAlgVars: List::flatten(list![intAlgVars.clone(), intAlgVars2.clone(), intAlgVars3.clone()]), discreteAlgVars: List::flatten(list![discreteAlgVars.clone(), discreteAlgVars2.clone(), discreteAlgVars3.clone()]), algVars: List::flatten(list![algVars.clone(), inputVars.clone(), nonTrivialAlias.clone()]), derivativeVars: derivativeVars.clone(), stateVars: stateVars.clone() });
        assign_field!(
            simVars.intAlgVars = listAppend(simVars.intAlgVars.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        for mut v in (simVars.enumAlgVars.clone()).into_iter().cloned() {
            let __x = SimVar::shiftIndex(v.clone(), simCodeIndices.integerVarIndex.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
            simVars.intAliasVars = listAppend(simVars.intAliasVars.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        for mut v in (simVars.enumAliasVars.clone()).into_iter().cloned() {
            let __x = SimVar::shiftIndex(v.clone(), simCodeIndices.integerAliasIndex.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
            simVars.intParamVars = listAppend(simVars.intParamVars.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        for mut v in (simVars.enumParamVars.clone()).into_iter().cloned() {
            let __x = SimVar::shiftIndex(v.clone(), simCodeIndices.integerParamIndex.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
            simVars.intConstVars = listAppend(simVars.intConstVars.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        for mut v in (simVars.enumConstVars.clone()).into_iter().cloned() {
            let __x = SimVar::shiftIndex(v.clone(), simCodeIndices.integerVarIndex.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        );
        Ok((simVars, simCodeIndices))
    }

    pub fn addSeedAndJacobianVars(mut vars: Arc<SimVars>, mut hash_tpl: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>)>>) -> Arc<SimVars> {
        let mut vars: Arc<SimVars> = vars;
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut var: Arc<SimVar::SimVar>;
        let mut seed_vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut jacobian_vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        for mut tpl in &*hash_tpl.clone() {
            let mut tpl = tpl.clone();
            (cref, var) = tpl.clone();
            if BVariable::checkCref(cref.clone(), fnptr!(BVariable::isSeed, Pointer::Pointer<Arc<Variable::NFVariable>>), metamodelica::sourceInfo!()) {
                seed_vars = cons(var.clone(), seed_vars.clone());
            } else {
                jacobian_vars = cons(var.clone(), jacobian_vars.clone());
            }
        }
        assign_field!(
            vars.seedVars = listAppend(seed_vars.clone(), vars.seedVars.clone()),
            vars.jacobianVars = listAppend(jacobian_vars.clone(), vars.jacobianVars.clone())
        );
        vars
    }

    pub fn size(mut simVars: Arc<SimVars>) -> i32 {
        let mut size: i32 = (simVars.stateVars.clone().len() as i32) + (simVars.derivativeVars.clone().len() as i32) + (simVars.algVars.clone().len() as i32) + (simVars.discreteAlgVars.clone().len() as i32) + (simVars.intAlgVars.clone().len() as i32) + (simVars.boolAlgVars.clone().len() as i32) + (simVars.inputVars.clone().len() as i32) + (simVars.outputVars.clone().len() as i32) + (simVars.aliasVars.clone().len() as i32) + (simVars.intAliasVars.clone().len() as i32) + (simVars.boolAliasVars.clone().len() as i32) + (simVars.paramVars.clone().len() as i32) + (simVars.intParamVars.clone().len() as i32) + (simVars.boolParamVars.clone().len() as i32) + (simVars.stringAlgVars.clone().len() as i32) + (simVars.stringParamVars.clone().len() as i32) + (simVars.stringAliasVars.clone().len() as i32) + (simVars.extObjVars.clone().len() as i32) + (simVars.constVars.clone().len() as i32) + (simVars.intConstVars.clone().len() as i32) + (simVars.boolConstVars.clone().len() as i32) + (simVars.stringConstVars.clone().len() as i32) + (simVars.stringAlgVars.clone().len() as i32) + (simVars.jacobianVars.clone().len() as i32) + (simVars.seedVars.clone().len() as i32) + (simVars.realOptimizeConstraintsVars.clone().len() as i32) + (simVars.realOptimizeFinalConstraintsVars.clone().len() as i32) + (simVars.sensitivityVars.clone().len() as i32) + (simVars.dataReconSetcVars.clone().len() as i32) + (simVars.dataReconinputVars.clone().len() as i32) + (simVars.dataReconSetBVars.clone().len() as i32);
        size
    }

    pub fn convert(mut simVars: Arc<SimVars>) -> OldSimCodeVar::SimVars {
        let mut oldSimVars: OldSimCodeVar::SimVars;
        oldSimVars = OldSimCodeVar::SimVars { dataReconSetBVars: SimVar::convertList(simVars.dataReconSetBVars.clone()), dataReconinputVars: SimVar::convertList(simVars.dataReconinputVars.clone()), dataReconSetcVars: SimVar::convertList(simVars.dataReconSetcVars.clone()), sensitivityVars: SimVar::convertList(simVars.sensitivityVars.clone()), realOptimizeFinalConstraintsVars: SimVar::convertList(simVars.realOptimizeFinalConstraintsVars.clone()), realOptimizeConstraintsVars: SimVar::convertList(simVars.realOptimizeConstraintsVars.clone()), seedVars: SimVar::convertList(simVars.seedVars.clone()), jacobianVars: SimVar::convertList(simVars.jacobianVars.clone()), stringConstVars: SimVar::convertList(simVars.stringConstVars.clone()), boolConstVars: SimVar::convertList(simVars.boolConstVars.clone()), intConstVars: SimVar::convertList(simVars.intConstVars.clone()), constVars: SimVar::convertList(simVars.constVars.clone()), extObjVars: SimVar::convertList(simVars.extObjVars.clone()), stringAliasVars: SimVar::convertList(simVars.stringAliasVars.clone()), stringParamVars: SimVar::convertList(simVars.stringParamVars.clone()), stringAlgVars: SimVar::convertList(simVars.stringAlgVars.clone()), boolParamVars: SimVar::convertList(simVars.boolParamVars.clone()), intParamVars: SimVar::convertList(simVars.intParamVars.clone()), paramVars: SimVar::convertList(simVars.paramVars.clone()), boolAliasVars: SimVar::convertList(simVars.boolAliasVars.clone()), intAliasVars: SimVar::convertList(simVars.intAliasVars.clone()), aliasVars: SimVar::convertList(simVars.aliasVars.clone()), outputVars: SimVar::convertList(simVars.outputVars.clone()), inputVars: SimVar::convertList(simVars.inputVars.clone()), boolAlgVars: SimVar::convertList(simVars.boolAlgVars.clone()), intAlgVars: SimVar::convertList(simVars.intAlgVars.clone()), discreteAlgVars: SimVar::convertList(simVars.discreteAlgVars.clone()), algVars: SimVar::convertList(simVars.algVars.clone()), derivativeVars: SimVar::convertList(simVars.derivativeVars.clone()), stateVars: SimVar::convertList(simVars.stateVars.clone()) };
        oldSimVars
    }

    pub fn createSimVarLists(mut vars: Arc<VariablePointers::VariablePointers>, mut simCodeIndices: SimCodeIndices, mut splitType: SplitType, mut varType: VarType) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>>, SimCodeIndices)> {
        let mut simVars: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut sim_vars: Arc<VariablePointers::VariablePointers> = if (Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())?) {BVariable::VariablePointers::scalarize(vars.clone())?} else {vars.clone()};
        let mut acc: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut real_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut int_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut bool_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut string_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut enum_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut indices_ptr: Pointer::Pointer<SimCodeIndices> = Pointer::create(simCodeIndices.clone());
        if splitType.clone() == SplitType::NONE.clone() {
            BVariable::VariablePointers::map(sim_vars.clone(), Arc::new({ let __pe_b1 = acc.clone(); let __pe_b2 = indices_ptr.clone(); let __pe_b3 = varType.clone(); move |__pe_a0| SimVar::traverseCreate(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            simVars = list![Pointer::access(acc.clone()).reverse()];
            simCodeIndices = Pointer::access(indices_ptr.clone());
        } else if splitType.clone() == SplitType::TYPE.clone() {
            BVariable::VariablePointers::map(sim_vars.clone(), Arc::new({ let __pe_b1 = real_lst.clone(); let __pe_b2 = int_lst.clone(); let __pe_b3 = bool_lst.clone(); let __pe_b4 = string_lst.clone(); let __pe_b5 = enum_lst.clone(); let __pe_b6 = indices_ptr.clone(); let __pe_b7 = varType.clone(); move |__pe_a0| splitByType(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone()) }))?;
            simVars = list![Pointer::access(real_lst.clone()).reverse(), Pointer::access(int_lst.clone()).reverse(), Pointer::access(bool_lst.clone()).reverse(), Pointer::access(string_lst.clone()).reverse(), Pointer::access(enum_lst.clone()).reverse()];
            simCodeIndices = Pointer::access(indices_ptr.clone());
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVars.createSimVarLists")); __mm_s.push_str(&*literal!(" failed because of invalid splitType.")); ArcStr::from(__mm_s) }).clone()])?;
        }
        Ok((simVars, simCodeIndices))
    }

    pub fn splitByType(mut var: Arc<Variable::NFVariable>, mut real_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>, mut int_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>, mut bool_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>, mut string_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>, mut enum_lst: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>, mut indices_ptr: Pointer::Pointer<SimCodeIndices>, mut varType: VarType) -> Result<Arc<Variable::NFVariable>> {
        let mut var: Arc<Variable::NFVariable> = var;
        let mut simCodeIndices: SimCodeIndices = Pointer::access(indices_ptr.clone());
        let () = (::match_deref::match_deref! { match &((Type::arrayElementType(var.ty.clone()), varType.clone())) {
        (Deref @ Type::REAL, VarType::SIMULATION) => {
            Pointer::update(real_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.realVarIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(real_lst.clone())));
            simCodeIndices.realVarIndex = simCodeIndices.realVarIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::INTEGER, VarType::SIMULATION) => {
            Pointer::update(int_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.integerVarIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(int_lst.clone())));
            simCodeIndices.integerVarIndex = simCodeIndices.integerVarIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::BOOLEAN, VarType::SIMULATION) => {
            Pointer::update(bool_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.booleanVarIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(bool_lst.clone())));
            simCodeIndices.booleanVarIndex = simCodeIndices.booleanVarIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::STRING, VarType::SIMULATION) => {
            Pointer::update(string_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.stringVarIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(string_lst.clone())));
            simCodeIndices.stringVarIndex = simCodeIndices.stringVarIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::ENUMERATION { .. }, VarType::SIMULATION) => {
            Pointer::update(enum_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.enumerationVarIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(enum_lst.clone())));
            simCodeIndices.enumerationVarIndex = simCodeIndices.enumerationVarIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::REAL, VarType::PARAMETER { .. }) => {
            Pointer::update(real_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.realParamIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(real_lst.clone())));
            simCodeIndices.realParamIndex = simCodeIndices.realParamIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::INTEGER, VarType::PARAMETER { .. }) => {
            Pointer::update(int_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.integerParamIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(int_lst.clone())));
            simCodeIndices.integerParamIndex = simCodeIndices.integerParamIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::BOOLEAN, VarType::PARAMETER { .. }) => {
            Pointer::update(bool_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.booleanParamIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(bool_lst.clone())));
            simCodeIndices.booleanParamIndex = simCodeIndices.booleanParamIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::STRING, VarType::PARAMETER { .. }) => {
            Pointer::update(string_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.stringParamIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(string_lst.clone())));
            simCodeIndices.stringParamIndex = simCodeIndices.stringParamIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::ENUMERATION { .. }, VarType::PARAMETER { .. }) => {
            Pointer::update(enum_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.enumerationParamIndex.clone(), Arc::new(crate::NSimVar::Alias::NO_ALIAS))?, Pointer::access(enum_lst.clone())));
            simCodeIndices.enumerationParamIndex = simCodeIndices.enumerationParamIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::REAL, VarType::ALIAS { .. }) => {
            Pointer::update(real_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.realAliasIndex.clone(), Alias::fromBinding(var.binding.clone())?)?, Pointer::access(real_lst.clone())));
            simCodeIndices.realAliasIndex = simCodeIndices.realAliasIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::INTEGER, VarType::ALIAS { .. }) => {
            Pointer::update(int_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.integerAliasIndex.clone(), Alias::fromBinding(var.binding.clone())?)?, Pointer::access(int_lst.clone())));
            simCodeIndices.integerAliasIndex = simCodeIndices.integerAliasIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::BOOLEAN, VarType::ALIAS { .. }) => {
            Pointer::update(bool_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.booleanAliasIndex.clone(), Alias::fromBinding(var.binding.clone())?)?, Pointer::access(bool_lst.clone())));
            simCodeIndices.booleanAliasIndex = simCodeIndices.booleanAliasIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::STRING, VarType::ALIAS { .. }) => {
            Pointer::update(string_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.stringAliasIndex.clone(), Alias::fromBinding(var.binding.clone())?)?, Pointer::access(string_lst.clone())));
            simCodeIndices.stringAliasIndex = simCodeIndices.stringAliasIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::ENUMERATION { .. }, VarType::ALIAS { .. }) => {
            Pointer::update(enum_lst.clone(), cons(SimVar::create(var.clone(), simCodeIndices.uniqueIndex.clone(), simCodeIndices.enumerationAliasIndex.clone(), Alias::fromBinding(var.binding.clone())?)?, Pointer::access(enum_lst.clone())));
            simCodeIndices.enumerationAliasIndex = simCodeIndices.enumerationAliasIndex.clone() + 1;
            simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
            Pointer::update(indices_ptr.clone(), simCodeIndices.clone());
            ()
        },
        (Deref @ Type::CLOCK, _) => (),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVars.splitByType")); __mm_s.push_str(&*literal!(" failed because of unhandled Variable ")); __mm_s.push_str(&*ComponentRef::toString(var.name.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(var)
    }

    pub fn getPartitionVars(mut partition: Arc<Partition::Partition>, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> {
        let mut part_vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        part_vars = (match partition.strongComponents.clone() {
        Some(mut comps) => {
            let mut result: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>> = metamodelica::nil();
            let __range0 = 1..=(comps.clone().borrow().len() as i32);
            for mut i in __range0 {
                result = cons(getStrongComponentVars(comps.borrow()[(i.clone()-1) as usize].clone(), simcode_map.clone())?, result.clone());
            }
            List::flatten(result.clone())
        },
        _ => {
            let mut result: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>> = metamodelica::nil();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVars.getPartitionVars")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*Partition::toString(partition.clone(), 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
        Ok(part_vars)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getStrongComponentVars(mut comp: Arc<StrongComponent::NBStrongComponent>, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> {
        let mut part_vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        part_vars = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => getVars(var_field!((*comp).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone(), simcode_map.clone())?,
        Deref @ StrongComponent::MULTI_COMPONENT { .. } => List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>> = metamodelica::nil();
        for mut v in (var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone()).into_iter().cloned() {
            let __x = getVars(Slice::getT(v.clone()), simcode_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ StrongComponent::SLICED_COMPONENT { .. } => getVars(Slice::getT(var_field!((*comp).var, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone()), simcode_map.clone())?,
        Deref @ StrongComponent::RESIZABLE_COMPONENT { .. } => getVars(Slice::getT(var_field!((*comp).var, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone()), simcode_map.clone())?,
        Deref @ StrongComponent::GENERIC_COMPONENT { .. } => getVars(BVariable::getVarPointer(var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::GENERIC_COMPONENT).clone(), metamodelica::sourceInfo!())?, simcode_map.clone())?,
        Deref @ StrongComponent::ENTWINED_COMPONENT { .. } => List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>> = metamodelica::nil();
        for mut c in (var_field!((*comp).entwined_slices, StrongComponent::NBStrongComponent::ENTWINED_COMPONENT).clone()).into_iter().cloned() {
            let __x = getStrongComponentVars(c.clone(), simcode_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ StrongComponent::ALGEBRAIC_LOOP { .. } => List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimVar::SimVar>>>>> = metamodelica::nil();
        for mut v in (var_field!((*comp).strict, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).iteration_vars.clone()).into_iter().cloned() {
            let __x = getVars(Slice::getT(v.clone()), simcode_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        Deref @ StrongComponent::ALIAS { .. } => getStrongComponentVars(var_field!((*comp).original, StrongComponent::NBStrongComponent::ALIAS).clone(), simcode_map.clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVars.getStrongComponentVars")); __mm_s.push_str(&*literal!(" failed with unknown reason for\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(part_vars)
    }

    fn getVars(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> {
        let mut vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        if Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())? {
            vars = {
        let mut __acc: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        for mut v in (BVariable::VariablePointers::scalarizeList(list![var.clone()])?).into_iter() {
            let __x = UnorderedMap::getSafe(BVariable::getVarName(v.clone()), simcode_map.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        } else {
            vars = list![UnorderedMap::getSafe(BVariable::getVarName(var.clone()), simcode_map.clone(), metamodelica::sourceInfo!())?];
        }
        Ok(vars)
    }

}

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: Arc<SimVars::SimVars>
// Expr: Constructor { name: 'NSimVar.SimVars.SIMVARS', args: [Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }, Array { elems: [], ty: List(Unknown) }], named_args: [], ty: RustStruct('NSimVar.SimVars'), field_names: ['stateVars', 'derivativeVars', 'algVars', 'discreteAlgVars', 'intAlgVars', 'boolAlgVars', 'stringAlgVars', 'enumAlgVars', 'inputVars', 'outputVars', 'aliasVars', 'intAliasVars', 'boolAliasVars', 'stringAliasVars', 'enumAliasVars', 'paramVars', 'intParamVars', 'boolParamVars', 'stringParamVars', 'enumParamVars', 'extObjVars', 'constVars', 'intConstVars', 'boolConstVars', 'stringConstVars', 'enumConstVars', 'residualVars', 'jacobianVars', 'seedVars', 'realOptimizeConstraintsVars', 'realOptimizeFinalConstraintsVars', 'sensitivityVars', 'dataReconSetcVars', 'dataReconinputVars', 'dataReconSetBVars'] }
pub fn emptySimVars() -> Arc<SimVars::SimVars> { todo!("non-Sync, non-const-emittable constant emptySimVars — extend codegen") }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SplitType {
    NONE = 1,
    TYPE = 2,
}
impl PartialOrd for SplitType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SplitType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum VarType {
    SIMULATION = 1,
    PARAMETER = 2,
    ALIAS = 3,
    RESIDUAL = 4,
    EXTERNAL_OBJECT = 5,
}
impl PartialOrd for VarType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for VarType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

// ToDo: PRE, OLD, RELATIONS...
pub mod VarInfo {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct VarInfo {
        pub numZeroCrossings: i32,
        pub numTimeEvents: i32,
        pub numRelations: i32,
        pub numMathEventFunctions: i32,
        pub numStateVars: i32,
        pub numAlgVars: i32,
        pub numDiscreteReal: i32,
        pub numIntAlgVars: i32,
        pub numBoolAlgVars: i32,
        pub numAlgAliasVars: i32,
        pub numIntAliasVars: i32,
        pub numBoolAliasVars: i32,
        pub numParams: i32,
        pub numIntParams: i32,
        pub numBoolParams: i32,
        pub numOutVars: i32,
        pub numInVars: i32,
        pub numExternalObjects: i32,
        pub numStringAlgVars: i32,
        pub numStringParamVars: i32,
        pub numStringAliasVars: i32,
        pub numEquations: i32,
        pub numLinearSystems: i32,
        pub numNonLinearSystems: i32,
        pub numMixedSystems: i32,
        pub numStateSets: i32,
        pub numJacobians: i32,
        pub numOptimizeConstraints: i32,
        pub numOptimizeFinalConstraints: i32,
        pub numSensitivityParameters: i32,
        pub numSetcVars: i32,
        pub numDataReconVars: i32,
        pub numRealIntputVars: i32,
        pub numSetbVars: i32,
        pub numRelatedBoundaryConditions: i32,
    }

    pub type VAR_INFO = VarInfo;

    pub fn create(mut vars: Arc<SimVars::SimVars>, mut eventInfo: Arc<EventInfo::EventInfo>, mut simCodeIndices: SimCodeIndices) -> Arc<VarInfo> {
        let mut varInfo: Arc<VarInfo>;
        varInfo = Arc::new(VarInfo { numRelatedBoundaryConditions: 0, numSetbVars: 0, numRealIntputVars: 0, numDataReconVars: 0, numSetcVars: 0, numSensitivityParameters: 0, numOptimizeFinalConstraints: 0, numOptimizeConstraints: 0, numJacobians: simCodeIndices.nonlinearSystemIndex.clone() + 5, numStateSets: 0, numMixedSystems: 0, numNonLinearSystems: simCodeIndices.nonlinearSystemIndex.clone(), numLinearSystems: simCodeIndices.linearSystemIndex.clone(), numEquations: simCodeIndices.equationIndex.clone(), numStringAliasVars: (vars.stringAliasVars.clone().len() as i32), numStringParamVars: (vars.stringParamVars.clone().len() as i32), numStringAlgVars: (vars.stringAlgVars.clone().len() as i32), numExternalObjects: (vars.extObjVars.clone().len() as i32), numInVars: (vars.inputVars.clone().len() as i32), numOutVars: (vars.outputVars.clone().len() as i32), numBoolParams: (vars.boolParamVars.clone().len() as i32), numIntParams: (vars.intParamVars.clone().len() as i32), numParams: (vars.paramVars.clone().len() as i32), numBoolAliasVars: (vars.boolAliasVars.clone().len() as i32), numIntAliasVars: (vars.intAliasVars.clone().len() as i32), numAlgAliasVars: (vars.aliasVars.clone().len() as i32), numBoolAlgVars: (vars.boolAlgVars.clone().len() as i32), numIntAlgVars: (vars.intAlgVars.clone().len() as i32), numDiscreteReal: (vars.discreteAlgVars.clone().len() as i32), numAlgVars: (vars.algVars.clone().len() as i32), numStateVars: (vars.stateVars.clone().len() as i32), numMathEventFunctions: eventInfo.numberMathEvents.clone(), numRelations: {
        let mut __acc: i32 = 0;
        for mut cond in (UnorderedMap::keyList(eventInfo.state_map.clone())).into_iter().cloned() {
            let __x = Condition::size(cond.clone());
            __acc += __x;
        }
        __acc
    }, numTimeEvents: UnorderedSet::size(eventInfo.time_set.clone()), numZeroCrossings: {
        let mut __acc: i32 = 0;
        for mut cond in (UnorderedMap::keyList(eventInfo.state_map.clone())).into_iter().cloned() {
            let __x = Condition::size(cond.clone());
            __acc += __x;
        }
        __acc
    } });
        varInfo
    }

    pub fn convert(mut varInfo: Arc<VarInfo>) -> OldSimCode::VarInfo {
        let mut oldVarInfo: OldSimCode::VarInfo;
        oldVarInfo = OldSimCode::VarInfo { numRelatedBoundaryConditions: varInfo.numRelatedBoundaryConditions.clone(), numSetbVars: varInfo.numSetbVars.clone(), numRealInputVars: varInfo.numRealIntputVars.clone(), numDataReconVars: varInfo.numDataReconVars.clone(), numSetcVars: varInfo.numSetcVars.clone(), numSensitivityParameters: varInfo.numSensitivityParameters.clone(), numOptimizeFinalConstraints: varInfo.numOptimizeFinalConstraints.clone(), numOptimizeConstraints: varInfo.numOptimizeConstraints.clone(), numJacobians: varInfo.numJacobians.clone(), numStateSets: varInfo.numStateSets.clone(), numMixedSystems: varInfo.numMixedSystems.clone(), numNonLinearSystems: varInfo.numNonLinearSystems.clone(), numLinearSystems: varInfo.numLinearSystems.clone(), numEquations: varInfo.numEquations.clone(), numStringAliasVars: varInfo.numStringAliasVars.clone(), numStringParamVars: varInfo.numStringParamVars.clone(), numStringAlgVars: varInfo.numStringAlgVars.clone(), numExternalObjects: varInfo.numExternalObjects.clone(), numInVars: varInfo.numInVars.clone(), numOutVars: varInfo.numOutVars.clone(), numBoolParams: varInfo.numBoolParams.clone(), numIntParams: varInfo.numIntParams.clone(), numParams: varInfo.numParams.clone(), numBoolAliasVars: varInfo.numBoolAliasVars.clone(), numIntAliasVars: varInfo.numIntAliasVars.clone(), numAlgAliasVars: varInfo.numAlgAliasVars.clone(), numBoolAlgVars: varInfo.numBoolAlgVars.clone(), numIntAlgVars: varInfo.numIntAlgVars.clone(), numDiscreteReal: varInfo.numDiscreteReal.clone(), numAlgVars: varInfo.numAlgVars.clone(), numStateVars: varInfo.numStateVars.clone(), numMathEventFunctions: varInfo.numMathEventFunctions.clone(), numRelations: varInfo.numRelations.clone(), numTimeEvents: varInfo.numTimeEvents.clone(), numZeroCrossings: varInfo.numZeroCrossings.clone() };
        oldVarInfo
    }

}

pub mod ExtObjInfo {
    use super::*;
#[derive(Clone, Debug, PartialEq)]
    pub struct ExtObjInfo {
        pub objects: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub aliases: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>,
    }

    pub type EXT_OBJ_INFO = ExtObjInfo;

    pub fn toString(mut info: Arc<ExtObjInfo>) -> ArcStr {
        let mut r#str: ArcStr = SimVar::listToString(info.objects.clone(), (literal!("External Objects")).clone(), false).unwrap();
        r#str
    }

    pub fn create(mut external_objects: Arc<VariablePointers::VariablePointers>, mut vars: Arc<SimVars::SimVars>, mut simCodeIndices: SimCodeIndices) -> Result<(Arc<ExtObjInfo>, Arc<SimVars::SimVars>, SimCodeIndices)> {
        let mut info: Arc<ExtObjInfo>;
        let mut vars: Arc<SimVars::SimVars> = vars;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut indices_ptr: Pointer::Pointer<SimCodeIndices> = Pointer::create(simCodeIndices.clone());
        let mut acc: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut varType: VarType = VarType::EXTERNAL_OBJECT.clone();
        let mut var_lst: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        BVariable::VariablePointers::map(external_objects.clone(), Arc::new({ let __pe_b1 = acc.clone(); let __pe_b2 = indices_ptr.clone(); let __pe_b3 = varType.clone(); move |__pe_a0| SimVar::traverseCreate(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
        simCodeIndices = Pointer::access(indices_ptr.clone());
        var_lst = Pointer::access(acc.clone()).reverse();
        assign_field!(vars.extObjVars = var_lst.clone());
        info = Arc::new(ExtObjInfo { objects: var_lst.clone(), aliases: metamodelica::nil() });
        Ok((info, vars, simCodeIndices))
    }

    pub fn convert(mut info: Arc<ExtObjInfo>) -> OldSimCode::ExtObjInfo {
        let mut oldInfo: OldSimCode::ExtObjInfo;
        oldInfo = OldSimCode::ExtObjInfo { vars: SimVar::convertList(info.objects.clone()), aliases: metamodelica::nil() };
        oldInfo
    }

}

