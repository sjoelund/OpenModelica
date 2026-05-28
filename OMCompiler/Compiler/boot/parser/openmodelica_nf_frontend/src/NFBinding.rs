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

use crate::BaseModelica;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;
use openmodelica_util_datatypes_basic::Mutable;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFBinding {
    UNBOUND,
    RAW_BINDING {
        bindingExp: Arc<Absyn::Exp>,
        scope: Arc<InstNode::InstNode>,
        subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>,
        eachType: EachType,
        source: Source,
        confidence: i32,
        info: SourceInfo,
    },
    UNTYPED_BINDING {
        bindingExp: Arc<Expression::NFExpression>,
        isProcessing: bool,
        scope: Arc<InstNode::InstNode>,
        eachType: EachType,
        source: Source,
        confidence: i32,
        info: SourceInfo,
    },
    TYPED_BINDING {
        bindingExp: Arc<Expression::NFExpression>,
        bindingType: Arc<Type::NFType>,
        variability: Variability,
        purity: Purity,
        eachType: EachType,
        evalState: Mutable::Mutable<EvalState>,
        isFlattened: bool,
        source: Source,
        confidence: i32,
        info: SourceInfo,
    },
    FLAT_BINDING {
        bindingExp: Arc<Expression::NFExpression>,
        variability: Variability,
        source: Source,
        confidence: i32,
    },
    /// Used by the constant evaluation for generated bindings (e.g. record
    ///     bindings constructed from the record fields) that should be discarded
    ///     during flattening.
    CEVAL_BINDING {
        bindingExp: Arc<Expression::NFExpression>,
    },
    INVALID_BINDING {
        binding: Arc<NFBinding>,
        errors: Arc<metamodelica::List<ErrorTypes::TotalMessage>>,
    },
    WILD,
}
impl Default for NFBinding {
    fn default() -> Self { Self::UNBOUND }
}
pub use self::NFBinding::{UNBOUND,RAW_BINDING,UNTYPED_BINDING,TYPED_BINDING,FLAT_BINDING,CEVAL_BINDING,INVALID_BINDING,WILD};
thread_local! { static __EMPTY_BINDING_TLS: Arc<NFBinding> = Arc::new(crate::NFBinding::UNBOUND); }
pub fn EMPTY_BINDING() -> Arc<NFBinding> { __EMPTY_BINDING_TLS.with(|__t| __t.clone()) }

pub const NO_CONFIDENCE: i32 = 99999;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum EachType {
    NOT_EACH = 1,
    EACH = 2,
}
impl PartialOrd for EachType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for EachType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum EvalState {
    NOT_EVALUATED = 1,
    EVALUATING = 2,
    EVALUATED = 3,
}
impl PartialOrd for EvalState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for EvalState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Source {
    /// The binding comes from a binding equation.
    BINDING = 1,
    /// The binding comes from a type.
    TYPE = 2,
    /// The binding comes from a modifier.
    MODIFIER = 3,
    /// The binding was generated by the frontend.
    GENERATED = 4,
}
impl PartialOrd for Source {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Source {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn fromAbsyn(mut bindingExp: Option<Arc<Absyn::Exp>>, mut eachPrefix: bool, mut fromType: bool, mut scope: Arc<InstNode::InstNode>, mut instanceLevel: i32, mut info: SourceInfo) -> Arc<NFBinding> {
    let mut binding: Arc<NFBinding> = Arc::new(NFBinding::UNBOUND);
    binding = (::match_deref::match_deref! { match &(bindingExp.clone()) {
        Some(exp) => {
            let mut each_ty: EachType = EachType::NOT_EACH;
            let mut source: Source = Source::BINDING;
            each_ty = if (eachPrefix.clone()) {EachType::EACH.clone()} else {EachType::NOT_EACH.clone()};
            source = if (fromType.clone()) {Source::TYPE.clone()} else {Source::BINDING.clone()};
            Arc::new(NFBinding::RAW_BINDING { bindingExp: exp.clone(), scope: scope.clone(), subs: metamodelica::nil(), eachType: each_ty.clone(), source: source.clone(), confidence: instanceLevel.clone(), info: info.clone() })
        },
        _ => {
            EMPTY_BINDING().clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

pub fn isBound(mut binding: Arc<NFBinding>) -> bool {
    let mut isBound: bool = false;
    isBound = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNBOUND { .. } => false,
        Deref @ INVALID_BINDING { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBound
}

pub fn isExplicitlyBound(mut binding: Arc<NFBinding>) -> bool {
    let mut isBound: bool = false;
    isBound = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNBOUND { .. } => false,
        Deref @ CEVAL_BINDING { .. } => false,
        Deref @ INVALID_BINDING { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBound
}

pub fn isUnbound(mut binding: Arc<NFBinding>) -> bool {
    let mut isUnbound: bool = false;
    isUnbound = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNBOUND { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isUnbound
}

pub fn isInvalid(mut binding: Arc<NFBinding>) -> bool {
    let mut isInvalid: bool = false;
    isInvalid = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ INVALID_BINDING { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isInvalid
}

pub fn typedExp(mut binding: Arc<NFBinding>) -> Option<Arc<Expression::NFExpression>> {
    let mut exp: Option<Arc<Expression::NFExpression>> = None;
    exp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => Some(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone()),
        Deref @ FLAT_BINDING { .. } => Some(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn getUntypedExp(mut binding: Arc<NFBinding>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let __pa0 = ::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { bindingExp: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    Ok(exp)
}

pub fn getTypedExp(mut binding: Arc<NFBinding>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone(),
        Deref @ FLAT_BINDING { .. } => var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub fn setTypedExp(mut exp: Arc<Expression::NFExpression>, mut binding: Arc<NFBinding>) -> Result<Arc<NFBinding>> {
    let mut binding: Arc<NFBinding> = binding;
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = exp.clone());
            ()
        },
        Deref @ FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = exp.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(binding)
}

pub fn hasExp(mut binding: Arc<NFBinding>) -> bool {
    let mut hasExp: bool = false;
    hasExp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => true,
        Deref @ TYPED_BINDING { .. } => true,
        Deref @ FLAT_BINDING { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasExp
}

pub fn getExp(mut binding: Arc<NFBinding>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => var_field!((*binding).bindingExp, NFBinding::UNTYPED_BINDING).clone(),
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone(),
        Deref @ FLAT_BINDING { .. } => var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub fn getExpOpt(mut binding: Arc<NFBinding>) -> Option<Arc<Expression::NFExpression>> {
    let mut exp: Option<Arc<Expression::NFExpression>> = None;
    exp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => Some(var_field!((*binding).bindingExp, NFBinding::UNTYPED_BINDING).clone()),
        Deref @ TYPED_BINDING { .. } => Some(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone()),
        Deref @ FLAT_BINDING { .. } => Some(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn setExp(mut exp: Arc<Expression::NFExpression>, mut binding: Arc<NFBinding>) -> Result<Arc<NFBinding>> {
    let mut binding: Arc<NFBinding> = binding;
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; bindingExp = exp.clone());
            ()
        },
        Deref @ TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = exp.clone());
            ()
        },
        Deref @ FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = exp.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(binding)
}

pub fn isRecordExp(mut binding: Arc<NFBinding>) -> bool {
    let mut isRecordExp: bool = false;
    isRecordExp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => Expression::isRecord(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRecordExp
}

pub fn isCrefExp(mut binding: Arc<NFBinding>) -> bool {
    let mut isCref: bool = false;
    isCref = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => Expression::isCref(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCref
}

pub fn recordFieldBinding(mut fieldNode: Arc<InstNode::InstNode>, mut recordBinding: Arc<NFBinding>) -> Result<Arc<NFBinding>> {
    let mut fieldBinding: Arc<NFBinding> = recordBinding.clone();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut purity: Purity = Purity::PURE;
    let mut field_name: ArcStr = InstNode::name(fieldNode.clone())?;
    fieldBinding = (::match_deref::match_deref! { match &(fieldBinding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => {
            assign_variant_field!(fieldBinding => NFBinding::UNTYPED_BINDING; bindingExp = Expression::recordElement((field_name.clone()).clone(), var_field!((*fieldBinding).bindingExp, NFBinding::UNTYPED_BINDING).clone())?);
            fieldBinding.clone()
        },
        Deref @ TYPED_BINDING { .. } => {
            exp = Expression::recordElement((field_name.clone()).clone(), var_field!((*fieldBinding).bindingExp, NFBinding::TYPED_BINDING).clone())?;
            ty = Expression::typeOf(exp.clone());
            purity = Expression::purity(exp.clone())?;
            var = Expression::variability(exp.clone())?;
            Arc::new(NFBinding::TYPED_BINDING { bindingExp: exp.clone(), bindingType: ty.clone(), variability: var.clone(), purity: purity.clone(), eachType: var_field!((*fieldBinding).eachType, NFBinding::TYPED_BINDING).clone(), evalState: var_field!((*fieldBinding).evalState, NFBinding::TYPED_BINDING).clone(), isFlattened: var_field!((*fieldBinding).isFlattened, NFBinding::TYPED_BINDING).clone(), source: var_field!((*fieldBinding).source, NFBinding::TYPED_BINDING).clone(), confidence: var_field!((*fieldBinding).confidence, NFBinding::TYPED_BINDING).clone(), info: var_field!((*fieldBinding).info, NFBinding::TYPED_BINDING).clone() })
        },
        Deref @ FLAT_BINDING { .. } => {
            exp = Expression::recordElement((field_name.clone()).clone(), var_field!((*fieldBinding).bindingExp, NFBinding::FLAT_BINDING).clone())?;
            var = Expression::variability(exp.clone())?;
            Arc::new(NFBinding::FLAT_BINDING { bindingExp: exp.clone(), variability: var.clone(), source: var_field!((*fieldBinding).source, NFBinding::FLAT_BINDING).clone(), confidence: var_field!((*fieldBinding).confidence, NFBinding::FLAT_BINDING).clone() })
        },
        Deref @ CEVAL_BINDING { .. } => {
            assign_variant_field!(fieldBinding => NFBinding::CEVAL_BINDING; bindingExp = Expression::recordElement((field_name.clone()).clone(), var_field!((*fieldBinding).bindingExp, NFBinding::CEVAL_BINDING).clone())?);
            fieldBinding.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(fieldBinding)
}

pub fn variability(mut binding: Arc<NFBinding>) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT;
    var = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).variability, NFBinding::TYPED_BINDING).clone(),
        Deref @ FLAT_BINDING { .. } => var_field!((*binding).variability, NFBinding::FLAT_BINDING).clone(),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBinding.variability")); __mm_s.push_str(&*literal!(" got unknown binding")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var)
}

pub fn setVariability(mut var: Variability, mut binding: Arc<NFBinding>) -> Arc<NFBinding> {
    let mut binding: Arc<NFBinding> = binding;
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; variability = var.clone());
            ()
        },
        Deref @ FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; variability = var.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

pub fn purity(mut binding: Arc<NFBinding>) -> Purity {
    let mut purity: Purity = Purity::PURE;
    purity = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).purity, NFBinding::TYPED_BINDING).clone(),
        _ => Purity::PURE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    purity
}

pub fn getInfo(mut binding: Arc<NFBinding>) -> SourceInfo {
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { .. } => var_field!((*binding).info, NFBinding::RAW_BINDING).clone(),
        Deref @ UNTYPED_BINDING { .. } => var_field!((*binding).info, NFBinding::UNTYPED_BINDING).clone(),
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).info, NFBinding::TYPED_BINDING).clone(),
        _ => Absyn::dummyInfo.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    info
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getType(mut binding: Arc<NFBinding>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNBOUND { .. } => Arc::new(crate::NFType::UNKNOWN),
        Deref @ RAW_BINDING { .. } => Arc::new(crate::NFType::UNKNOWN),
        Deref @ UNTYPED_BINDING { .. } => Arc::new(crate::NFType::UNKNOWN),
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).bindingType, NFBinding::TYPED_BINDING).clone(),
        Deref @ FLAT_BINDING { .. } => Expression::typeOf(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone()),
        Deref @ CEVAL_BINDING { .. } => Expression::typeOf(var_field!((*binding).bindingExp, NFBinding::CEVAL_BINDING).clone()),
        Deref @ INVALID_BINDING { .. } => getType(var_field!((*binding).binding, NFBinding::INVALID_BINDING).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(ty)
}

pub fn isEach(mut binding: Arc<NFBinding>) -> bool {
    let mut isEach: bool = false;
    isEach = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { .. } => var_field!((*binding).eachType, NFBinding::RAW_BINDING).clone() == EachType::EACH.clone(),
        Deref @ UNTYPED_BINDING { .. } => var_field!((*binding).eachType, NFBinding::UNTYPED_BINDING).clone() == EachType::EACH.clone(),
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).eachType, NFBinding::TYPED_BINDING).clone() == EachType::EACH.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEach
}

pub fn isTyped(mut binding: Arc<NFBinding>) -> bool {
    let mut isTyped: bool = false;
    isTyped = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => true,
        Deref @ FLAT_BINDING { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTyped
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toString(mut binding: Arc<NFBinding>, mut prefix: ArcStr) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNBOUND { .. } => literal!(""),
        Deref @ RAW_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Dump::printExpStr(var_field!((*binding).bindingExp, NFBinding::RAW_BINDING).clone())?); ArcStr::from(__mm_s) },
        Deref @ UNTYPED_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Expression::toString(var_field!((*binding).bindingExp, NFBinding::UNTYPED_BINDING).clone())?); ArcStr::from(__mm_s) },
        Deref @ TYPED_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Expression::toString(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone())?); ArcStr::from(__mm_s) },
        Deref @ FLAT_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Expression::toString(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone())?); ArcStr::from(__mm_s) },
        Deref @ CEVAL_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Expression::toString(var_field!((*binding).bindingExp, NFBinding::CEVAL_BINDING).clone())?); ArcStr::from(__mm_s) },
        Deref @ INVALID_BINDING { .. } => toString(var_field!((*binding).binding, NFBinding::INVALID_BINDING).clone(), (prefix.clone()).clone())?,
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(string)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toFlatString(mut binding: Arc<NFBinding>, mut format: BaseModelica::OutputFormat, mut prefix: ArcStr) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNBOUND { .. } => literal!(""),
        Deref @ RAW_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Dump::printExpStr(var_field!((*binding).bindingExp, NFBinding::RAW_BINDING).clone())?); ArcStr::from(__mm_s) },
        Deref @ UNTYPED_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Expression::toFlatString(var_field!((*binding).bindingExp, NFBinding::UNTYPED_BINDING).clone(), format.clone())?); ArcStr::from(__mm_s) },
        Deref @ TYPED_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Expression::toFlatString(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone(), format.clone())?); ArcStr::from(__mm_s) },
        Deref @ FLAT_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Expression::toFlatString(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone(), format.clone())?); ArcStr::from(__mm_s) },
        Deref @ CEVAL_BINDING { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*Expression::toFlatString(var_field!((*binding).bindingExp, NFBinding::CEVAL_BINDING).clone(), format.clone())?); ArcStr::from(__mm_s) },
        Deref @ INVALID_BINDING { .. } => toFlatString(var_field!((*binding).binding, NFBinding::INVALID_BINDING).clone(), format.clone(), (prefix.clone()).clone())?,
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(string)
}

pub fn toDebugString(mut binding: Arc<NFBinding>) -> ArcStr {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ WILD { .. } => literal!("WILD"),
        Deref @ UNBOUND { .. } => literal!("UNBOUND"),
        Deref @ RAW_BINDING { .. } => literal!("RAW_BINDING"),
        Deref @ UNTYPED_BINDING { .. } => literal!("UNTYPED_BINDING"),
        Deref @ TYPED_BINDING { .. } => literal!("TYPED_BINDING"),
        Deref @ FLAT_BINDING { .. } => literal!("FLAT_BINDING"),
        Deref @ CEVAL_BINDING { .. } => literal!("CEVAL_BINDING"),
        Deref @ INVALID_BINDING { .. } => literal!("INVALID_BINDING"),
        _ => literal!("UNKNOWN"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    string
}

pub fn isEqual(mut binding1: Arc<NFBinding>, mut binding2: Arc<NFBinding>) -> Result<bool> {
    let mut equal: bool = false;
    equal = (::match_deref::match_deref! { match &((binding1.clone(), binding2.clone())) {
        (Deref @ UNBOUND { .. }, Deref @ UNBOUND { .. }) => true,
        (Deref @ RAW_BINDING { .. }, Deref @ RAW_BINDING { .. }) => AbsynUtil::expEqual(var_field!((*binding1).bindingExp, NFBinding::RAW_BINDING).clone(), var_field!((*binding2).bindingExp, NFBinding::RAW_BINDING).clone())?,
        (Deref @ UNTYPED_BINDING { .. }, Deref @ UNTYPED_BINDING { .. }) => Expression::isEqual(var_field!((*binding1).bindingExp, NFBinding::UNTYPED_BINDING).clone(), var_field!((*binding2).bindingExp, NFBinding::UNTYPED_BINDING).clone())?,
        (Deref @ TYPED_BINDING { .. }, Deref @ TYPED_BINDING { .. }) => Expression::isEqual(var_field!((*binding1).bindingExp, NFBinding::TYPED_BINDING).clone(), var_field!((*binding2).bindingExp, NFBinding::TYPED_BINDING).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn toDAE(mut binding: Arc<NFBinding>) -> Result<Arc<DAE::Binding>> {
    let mut outBinding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    outBinding = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ WILD { .. } => Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND),
        Deref @ UNBOUND { .. } => Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND),
        Deref @ TYPED_BINDING { .. } => makeDAEBinding(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone(), var_field!((*binding).variability, NFBinding::TYPED_BINDING).clone())?,
        Deref @ FLAT_BINDING { .. } => makeDAEBinding(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone(), var_field!((*binding).variability, NFBinding::FLAT_BINDING).clone())?,
        Deref @ CEVAL_BINDING { .. } => Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND),
        Deref @ INVALID_BINDING { .. } => {
            Error::addTotalMessages(var_field!((*binding).errors, NFBinding::INVALID_BINDING).clone())?;
            bail!("fail")
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBinding.toDAE")); __mm_s.push_str(&*literal!(" got untyped binding")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinding)
}

pub fn makeDAEBinding(mut exp: Arc<Expression::NFExpression>, mut var: Variability) -> Result<Arc<DAE::Binding>> {
    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    binding = Arc::new(DAE::Binding::EQBOUND { exp: Expression::toDAE(exp.clone(), false)?, evaluatedExp: None, constant_: NFPrefixes::variabilityToDAEConst(var.clone()), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE });
    Ok(binding)
}

pub fn toDAEExp(mut binding: Arc<NFBinding>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut bindingExp: Option<Arc<DAE::Exp>> = None;
    bindingExp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNBOUND { .. } => None,
        Deref @ TYPED_BINDING { .. } => Some(Expression::toDAE(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone(), false)?),
        Deref @ FLAT_BINDING { .. } => Some(Expression::toDAE(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone(), false)?),
        Deref @ CEVAL_BINDING { .. } => None,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBinding.toDAEExp")); __mm_s.push_str(&*literal!(" got untyped binding")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bindingExp)
}

pub fn applyExp(mut binding: Arc<NFBinding>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => {
            Expression::apply(var_field!((*binding).bindingExp, NFBinding::UNTYPED_BINDING).clone(), r#fn.clone())?;
            ()
        },
        Deref @ TYPED_BINDING { .. } => {
            Expression::apply(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone(), r#fn.clone())?;
            ()
        },
        Deref @ FLAT_BINDING { .. } => {
            Expression::apply(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone(), r#fn.clone())?;
            ()
        },
        Deref @ CEVAL_BINDING { .. } => {
            Expression::apply(var_field!((*binding).bindingExp, NFBinding::CEVAL_BINDING).clone(), r#fn.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn applyExpShallow(mut binding: Arc<NFBinding>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> () {
    pub type ApplyFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => {
            r#fn(var_field!((*binding).bindingExp, NFBinding::UNTYPED_BINDING).clone()).unwrap();
            ()
        },
        Deref @ TYPED_BINDING { .. } => {
            r#fn(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone()).unwrap();
            ()
        },
        Deref @ FLAT_BINDING { .. } => {
            r#fn(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone()).unwrap();
            ()
        },
        Deref @ CEVAL_BINDING { .. } => {
            r#fn(var_field!((*binding).bindingExp, NFBinding::CEVAL_BINDING).clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub fn mapExp(mut binding: Arc<NFBinding>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFBinding>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut binding: Arc<NFBinding> = binding;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { bindingExp: e1, .. } => {
            e2 = Expression::map(e1.clone(), mapFn.clone())?;
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; bindingExp = e2.clone());
            }
            ()
        },
        Deref @ TYPED_BINDING { bindingExp: e1, .. } => {
            e2 = Expression::map(e1.clone(), mapFn.clone())?;
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = e2.clone());
            }
            ()
        },
        Deref @ FLAT_BINDING { bindingExp: e1, .. } => {
            e2 = Expression::map(e1.clone(), mapFn.clone())?;
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = e2.clone());
            }
            ()
        },
        Deref @ CEVAL_BINDING { bindingExp: e1 } => {
            e2 = Expression::map(e1.clone(), mapFn.clone())?;
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(binding => NFBinding::CEVAL_BINDING; bindingExp = e2.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

pub fn mapExpShallow(mut binding: Arc<NFBinding>, mut mapFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFBinding> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut binding: Arc<NFBinding> = binding;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { bindingExp: e1, .. } => {
            e2 = mapFn(e1.clone()).unwrap();
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; bindingExp = e2.clone());
            }
            ()
        },
        Deref @ TYPED_BINDING { bindingExp: e1, .. } => {
            e2 = mapFn(e1.clone()).unwrap();
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = e2.clone());
            }
            ()
        },
        Deref @ FLAT_BINDING { bindingExp: e1, .. } => {
            e2 = mapFn(e1.clone()).unwrap();
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = e2.clone());
            }
            ()
        },
        Deref @ CEVAL_BINDING { bindingExp: e1 } => {
            e2 = mapFn(e1.clone()).unwrap();
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(binding => NFBinding::CEVAL_BINDING; bindingExp = e2.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

pub fn foldExp<ArgT: Clone + 'static>(mut binding: Arc<NFBinding>, mut foldFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    arg = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => Expression::fold(var_field!((*binding).bindingExp, NFBinding::UNTYPED_BINDING).clone(), foldFn.clone(), arg.clone())?,
        Deref @ TYPED_BINDING { .. } => Expression::fold(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone(), foldFn.clone(), arg.clone())?,
        Deref @ FLAT_BINDING { .. } => Expression::fold(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone(), foldFn.clone(), arg.clone())?,
        Deref @ CEVAL_BINDING { .. } => Expression::fold(var_field!((*binding).bindingExp, NFBinding::CEVAL_BINDING).clone(), foldFn.clone(), arg.clone())?,
        _ => arg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

pub fn containsExp(mut binding: Arc<NFBinding>, mut predFn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ UNTYPED_BINDING { .. } => Expression::contains(var_field!((*binding).bindingExp, NFBinding::UNTYPED_BINDING).clone(), predFn.clone())?,
        Deref @ TYPED_BINDING { .. } => Expression::contains(var_field!((*binding).bindingExp, NFBinding::TYPED_BINDING).clone(), predFn.clone())?,
        Deref @ FLAT_BINDING { .. } => Expression::contains(var_field!((*binding).bindingExp, NFBinding::FLAT_BINDING).clone(), predFn.clone())?,
        Deref @ CEVAL_BINDING { .. } => Expression::contains(var_field!((*binding).bindingExp, NFBinding::CEVAL_BINDING).clone(), predFn.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn update(mut binding: Arc<NFBinding>, mut exp: Arc<Expression::NFExpression>) -> Result<Arc<NFBinding>> {
    let mut binding: Arc<NFBinding> = binding;
    binding = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ WILD { .. } => Arc::new(NFBinding::TYPED_BINDING { info: metamodelica::sourceInfo!(), confidence: NO_CONFIDENCE.clone(), source: Source::BINDING.clone(), isFlattened: true, evalState: if (Expression::isConstNumber(exp.clone())) {Mutable::create(EvalState::EVALUATED.clone())} else {Mutable::create(EvalState::NOT_EVALUATED.clone())}, eachType: EachType::NOT_EACH.clone(), purity: Expression::purity(exp.clone())?, variability: Expression::variability(exp.clone())?, bindingType: Expression::typeOf(exp.clone()), bindingExp: exp.clone() }),
        Deref @ UNBOUND { .. } => Arc::new(NFBinding::TYPED_BINDING { info: metamodelica::sourceInfo!(), confidence: NO_CONFIDENCE.clone(), source: Source::BINDING.clone(), isFlattened: true, evalState: if (Expression::isConstNumber(exp.clone())) {Mutable::create(EvalState::EVALUATED.clone())} else {Mutable::create(EvalState::NOT_EVALUATED.clone())}, eachType: EachType::NOT_EACH.clone(), purity: Expression::purity(exp.clone())?, variability: Expression::variability(exp.clone())?, bindingType: Expression::typeOf(exp.clone()), bindingExp: exp.clone() }),
        Deref @ UNTYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; bindingExp = exp.clone());
            binding.clone()
        },
        Deref @ TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; bindingExp = exp.clone());
            binding.clone()
        },
        Deref @ FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; bindingExp = exp.clone());
            binding.clone()
        },
        Deref @ CEVAL_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::CEVAL_BINDING; bindingExp = exp.clone());
            binding.clone()
        },
        Deref @ INVALID_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::INVALID_BINDING; binding = update(var_field!((*binding).binding, NFBinding::INVALID_BINDING).clone(), exp.clone())?);
            binding.clone()
        },
        Deref @ RAW_BINDING { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBinding.update")); __mm_s.push_str(&*literal!(" failed because a raw binding cannot be updated.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBinding.update")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

pub fn setAttr(mut ty_attr: Arc<metamodelica::List<(ArcStr, Arc<NFBinding>)>>, mut attr_name: ArcStr, mut attr_value: Arc<NFBinding>) -> Arc<metamodelica::List<(ArcStr, Arc<NFBinding>)>> {
    let mut ty_attr: Arc<metamodelica::List<(ArcStr, Arc<NFBinding>)>> = ty_attr;
    ty_attr = (::match_deref::match_deref! { match &(ty_attr.clone()) {
        Deref @ metamodelica::List::Cons { head: (name, _), tail: rest } if (name.clone() == attr_name.clone()) => {
            cons((attr_name.clone(), attr_value.clone()), rest.clone())
        },
        Deref @ metamodelica::List::Cons { head: at, tail: rest } => {
            cons(at.clone(), setAttr(rest.clone(), (attr_name.clone()).clone(), attr_value.clone()))
        },
        Deref @ metamodelica::List::Nil => {
            list![(attr_name.clone(), attr_value.clone())]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty_attr
}

pub fn propagate(mut binding: Arc<NFBinding>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Arc<NFBinding> {
    let mut binding: Arc<NFBinding> = binding;
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::RAW_BINDING;
                subs = listAppend(var_field!((*binding).subs, NFBinding::RAW_BINDING).clone(), subs.clone()),
                source = Source::MODIFIER.clone()
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

pub fn unpropagate(mut binding: Arc<NFBinding>, mut node: Arc<InstNode::InstNode>) -> Arc<NFBinding> {
    let mut binding: Arc<NFBinding> = binding;
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::RAW_BINDING; subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*binding).subs, NFBinding::RAW_BINDING).clone()).into_iter().cloned() {
            if !(!(Subscript::isSplitFromOrigin(s.clone(), node.clone()))) { continue; }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

pub fn source(mut binding: Arc<NFBinding>) -> Source {
    let mut source: Source = Source::BINDING;
    source = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { .. } => var_field!((*binding).source, NFBinding::RAW_BINDING).clone(),
        Deref @ UNTYPED_BINDING { .. } => var_field!((*binding).source, NFBinding::UNTYPED_BINDING).clone(),
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).source, NFBinding::TYPED_BINDING).clone(),
        Deref @ FLAT_BINDING { .. } => var_field!((*binding).source, NFBinding::FLAT_BINDING).clone(),
        _ => Source::BINDING.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    source
}

pub fn setSource(mut source: Source, mut binding: Arc<NFBinding>) -> Arc<NFBinding> {
    let mut binding: Arc<NFBinding> = binding;
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::RAW_BINDING; source = source.clone());
            ()
        },
        Deref @ UNTYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::UNTYPED_BINDING; source = source.clone());
            ()
        },
        Deref @ TYPED_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::TYPED_BINDING; source = source.clone());
            ()
        },
        Deref @ FLAT_BINDING { .. } => {
            assign_variant_field!(binding => NFBinding::FLAT_BINDING; source = source.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

pub fn makeUntyped(mut exp: Arc<Expression::NFExpression>, mut scope: Arc<InstNode::InstNode>, mut eachType: EachType, mut source: Source, mut info: SourceInfo, mut confidence: i32) -> Arc<NFBinding> {
    let mut binding: Arc<NFBinding> = Arc::new(NFBinding::UNBOUND);
    binding = Arc::new(NFBinding::UNTYPED_BINDING { bindingExp: exp.clone(), isProcessing: false, scope: scope.clone(), eachType: eachType.clone(), source: source.clone(), confidence: confidence.clone(), info: info.clone() });
    binding
}

pub fn makeTyped(mut exp: Arc<Expression::NFExpression>, mut eachType: EachType, mut source: Source, mut info: SourceInfo, mut state: EvalState, mut confidence: i32) -> Result<Arc<NFBinding>> {
    let mut binding: Arc<NFBinding> = Arc::new(NFBinding::UNBOUND);
    binding = Arc::new(NFBinding::TYPED_BINDING { bindingExp: exp.clone(), bindingType: Expression::typeOf(exp.clone()), variability: Expression::variability(exp.clone())?, purity: Expression::purity(exp.clone())?, eachType: eachType.clone(), evalState: Mutable::create(state.clone()), isFlattened: false, source: source.clone(), confidence: confidence.clone(), info: info.clone() });
    Ok(binding)
}

pub fn makeFlat(mut exp: Arc<Expression::NFExpression>, mut var: Variability, mut source: Source, mut confidence: i32) -> Arc<NFBinding> {
    let mut binding: Arc<NFBinding> = Arc::new(NFBinding::UNBOUND);
    binding = Arc::new(NFBinding::FLAT_BINDING { bindingExp: exp.clone(), variability: var.clone(), source: source.clone(), confidence: confidence.clone() });
    binding
}

pub fn isEvaluated(mut binding: Arc<NFBinding>) -> bool {
    let mut evaluated: bool = false;
    evaluated = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ TYPED_BINDING { .. } => Mutable::access(var_field!((*binding).evalState, NFBinding::TYPED_BINDING).clone()) == EvalState::EVALUATED.clone(),
        Deref @ CEVAL_BINDING { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    evaluated
}

pub fn hasTypeOrigin(mut binding: Arc<NFBinding>) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { .. } if (!(var_field!((*binding).subs, NFBinding::RAW_BINDING).clone().is_empty())) => Subscript::isSplitClassProxy(listHead(var_field!((*binding).subs, NFBinding::RAW_BINDING).clone())?),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn expandEach(mut binding: Arc<NFBinding>, mut node: Arc<InstNode::InstNode>) -> Result<Arc<NFBinding>> {
    let mut binding: Arc<NFBinding> = binding;
    let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let mut node_exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let size_name: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("size")).clone(), subscripts: metamodelica::nil() });
    let fill_name: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("fill")).clone(), subscripts: metamodelica::nil() });
    let () = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { eachType: EachType::EACH, .. } => {
            node_exp = Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (InstNode::name(node.clone())?).clone(), subscripts: metamodelica::nil() }) });
            args = metamodelica::nil();
            for mut i in (1..=InstNode::dimensionCount(node.clone())).rev() {
                args = cons(AbsynUtil::makeCall(size_name.clone(), list![node_exp.clone(), Arc::new(Absyn::Exp::INTEGER { value: i.clone() })], metamodelica::nil()), args.clone());
            }
            args = cons(var_field!((*binding).bindingExp, NFBinding::RAW_BINDING).clone(), args.clone());
            assign_variant_field!(binding => NFBinding::RAW_BINDING; bindingExp = AbsynUtil::makeCall(fill_name.clone(), args.clone(), metamodelica::nil()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(binding)
}

pub fn isClockOrSampleFunction(mut binding: Arc<NFBinding>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(getExpOpt(binding.clone())) {
        Some(exp) => {
            Expression::isClockOrSampleFunction(exp.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn confidence(mut binding: Arc<NFBinding>) -> i32 {
    let mut confidence: i32 = 0;
    confidence = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ RAW_BINDING { .. } => var_field!((*binding).confidence, NFBinding::RAW_BINDING).clone(),
        Deref @ UNTYPED_BINDING { .. } => var_field!((*binding).confidence, NFBinding::UNTYPED_BINDING).clone(),
        Deref @ TYPED_BINDING { .. } => var_field!((*binding).confidence, NFBinding::TYPED_BINDING).clone(),
        Deref @ FLAT_BINDING { .. } => var_field!((*binding).confidence, NFBinding::FLAT_BINDING).clone(),
        _ => NO_CONFIDENCE.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    confidence
}

pub fn actualConfidence(mut binding: Arc<NFBinding>) -> Result<i32> {
    let mut conf: i32 = NO_CONFIDENCE.clone();
    let mut b: Arc<NFBinding> = binding.clone();
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    while hasExp(b.clone()) {
        conf = std::cmp::min(conf.clone(), confidence(b.clone()));
        exp = getExp(b.clone())?;
        b = EMPTY_BINDING().clone();
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) => {
            node = InstNode::resolveInner(ComponentRef::node(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?);
            if InstNode::isComponent(node.clone()) {
                comp = InstNode::component(node.clone())?;
                if Component::variability(comp.clone()) < Variability::DISCRETE.clone() {
                    b = Component::getBinding(comp.clone());
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(conf)
}


