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
use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::SlicingStatus;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBPartition as Partition;
use crate::NBPartitioning as Partitioning;
use crate::NBPartitioning::BClock;
use crate::NBPartitioning::ClockedInfo;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBStrongComponent::AliasInfo;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimCode as SimCode;
use crate::NSimCode::Identifier;
use crate::NSimCode::SimCodeIndices;
use crate::NSimGenericCall::SimIterator;
use crate::NSimJacobian::SimJacobian;
use crate::NSimPartition as SimPartition;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use crate::NSimVar::VarType;
use crate::SimCode as OldSimCode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFScalarize as Scalarize;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// old backend imports
// Backend imports
// Old SimCode imports
// SimCode imports
// Util imports
pub mod Block {
    use super::*;
    /// A single blck from BLT transformation.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Block {
        /// Single residual equation of the form
        ///      0 = exp
        RESIDUAL {
            index: i32,
            res_index: i32,
            exp: Arc<Expression::NFExpression>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// Single residual array equation of the form
        ///      0 = exp. Structurally equal to RESIDUAL, but the destinction is important
        ///      for code generation.
        ARRAY_RESIDUAL {
            index: i32,
            res_index: i32,
            exp: Arc<Expression::NFExpression>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// for-loop residual equation of the form
        ///      for {i in 1:n, j in 1:m, ...} loop
        ///        0 = exp;
        ///      end for;
        FOR_RESIDUAL {
            index: i32,
            res_index: i32,
            iterators: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)>>,
            exp: Arc<Expression::NFExpression>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// a generic residual calling a for loop body function with an index list.
        GENERIC_RESIDUAL {
            index: i32,
            res_index: i32,
            scal_indices: Arc<metamodelica::List<i32>>,
            iterators: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)>>,
            exp: Arc<Expression::NFExpression>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// Simple assignment or solved inner equation of (casual) tearing set
        ///      (Dynamic Tearing) with constraints on the solvability
        ///      lhs := rhs
        SIMPLE_ASSIGN {
            index: i32,
            /// left hand side of equation
            lhs: Arc<ComponentRef::NFComponentRef>,
            rhs: Arc<Expression::NFExpression>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// Array assignment where the left hand side can be an array constructor.
        ///      {a, b, ...} := rhs
        ARRAY_ASSIGN {
            index: i32,
            lhs: Arc<Expression::NFExpression>,
            rhs: Arc<Expression::NFExpression>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// a resizable assignment calling a for loop body function.
        RESIZABLE_ASSIGN {
            index: i32,
            call_index: i32,
            iters: Arc<metamodelica::List<Arc<SimIterator::SimIterator>>>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// a generic assignment calling a for loop body function with an index list.
        GENERIC_ASSIGN {
            index: i32,
            call_index: i32,
            scal_indices: Arc<metamodelica::List<i32>>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// entwined generic assignments calling for loop body functions with an index list and a call order.
        ENTWINED_ASSIGN {
            index: i32,
            call_order: Arc<metamodelica::List<i32>>,
            single_calls: Arc<metamodelica::List<Arc<Block>>>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// Simple alias assignment pointing to the alias equation.
        ///      - alias of will be -1 at the point of creation and computed afterwards
        ALIAS {
            index: i32,
            /// backend alias info
            aliasInfo: Arc<AliasInfo::AliasInfo>,
            /// final alias index
            aliasOf: i32,
            isDiscrete: bool,
        },
        /// An algorithm section.
        ALGORITHM {
            index: i32,
            stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// An algorithm section that had to be inverted.
        INVERSE_ALGORITHM {
            index: i32,
            stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>,
            /// this is a subset of output crefs of the original algorithm, which are already known
            knownOutputs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
            insideNonLinearSystem: bool,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// An if section.
        IF {
            index: i32,
            branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Block>>>)>>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// A when section.
        WHEN {
            index: i32,
            /// true, if top-level branch with initial()
            initialCall: bool,
            conditions: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
            when_stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>,
            else_when: Option<Arc<Block>>,
            source: Arc<DAE::ElementSource>,
            attr: Arc<EquationAttributes::EquationAttributes>,
        },
        /// Linear algebraic loop.
        LINEAR {
            system: Arc<LinearSystem::LinearSystem>,
            alternativeTearing: Option<Arc<LinearSystem::LinearSystem>>,
        },
        /// Nonlinear algebraic loop.
        NONLINEAR {
            system: Arc<NonlinearSystem::NonlinearSystem>,
            alternativeTearing: Option<Arc<NonlinearSystem::NonlinearSystem>>,
        },
        /// Hybrid system containing both continuous and discrete equations.
        HYBRID {
            index: i32,
            continuous: Arc<Block>,
            discreteVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
            discreteEqs: Arc<metamodelica::List<Arc<Block>>>,
            indexHybridSystem: i32,
        },
    }
    impl Default for Block {
        fn default() -> Self {
            Self::LINEAR {
                system: Default::default(),
                alternativeTearing: Default::default(),
            }
        }
    }
    pub use self::Block::{RESIDUAL,ARRAY_RESIDUAL,FOR_RESIDUAL,GENERIC_RESIDUAL,SIMPLE_ASSIGN,ARRAY_ASSIGN,RESIZABLE_ASSIGN,GENERIC_ASSIGN,ENTWINED_ASSIGN,ALIAS,ALGORITHM,INVERSE_ALGORITHM,IF,WHEN,LINEAR,NONLINEAR,HYBRID};
    pub fn toString(mut blck: Arc<Block>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = ((::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ RESIDUAL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::RESIDUAL).clone())); __mm_s.push_str(&*literal!(") 0 = ")); __mm_s.push_str(&*Expression::toString(var_field!((*blck).exp, Block::RESIDUAL).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ ARRAY_RESIDUAL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::ARRAY_RESIDUAL).clone())); __mm_s.push_str(&*literal!(") 0 = ")); __mm_s.push_str(&*Expression::toString(var_field!((*blck).exp, Block::ARRAY_RESIDUAL).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ FOR_RESIDUAL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::FOR_RESIDUAL).clone())); __mm_s.push_str(&*literal!(") For-Loop-Residual:\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("for ")); __mm_s.push_str(&*List::toString(var_field!((*blck).iterators, Block::FOR_RESIDUAL).clone(), (std::sync::Arc::new(forTplStr) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!(" loop\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  0 = ")); __mm_s.push_str(&*Expression::toString(var_field!((*blck).exp, Block::FOR_RESIDUAL).clone())?); __mm_s.push_str(&*literal!(";\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("end for;\n")); ArcStr::from(__mm_s) },
        Deref @ GENERIC_RESIDUAL { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::GENERIC_RESIDUAL).clone())); __mm_s.push_str(&*literal!(") Generic For-Loop-Residual:\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(var_field!((*blck).scal_indices, Block::GENERIC_RESIDUAL).clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("slice")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 10)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("for ")); __mm_s.push_str(&*List::toString(var_field!((*blck).iterators, Block::GENERIC_RESIDUAL).clone(), (std::sync::Arc::new(forTplStr) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!(" loop\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  0 = ")); __mm_s.push_str(&*Expression::toString(var_field!((*blck).exp, Block::GENERIC_RESIDUAL).clone())?); __mm_s.push_str(&*literal!(";\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("end for;\n")); ArcStr::from(__mm_s) },
        Deref @ SIMPLE_ASSIGN { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::SIMPLE_ASSIGN).clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*blck).lhs, Block::SIMPLE_ASSIGN).clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*Expression::toString(var_field!((*blck).rhs, Block::SIMPLE_ASSIGN).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ ARRAY_ASSIGN { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::ARRAY_ASSIGN).clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*Expression::toString(var_field!((*blck).lhs, Block::ARRAY_ASSIGN).clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*Expression::toString(var_field!((*blck).rhs, Block::ARRAY_ASSIGN).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ RESIZABLE_ASSIGN { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::RESIZABLE_ASSIGN).clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*literal!("resizable call [index  ")); __mm_s.push_str(&*intString(var_field!((*blck).call_index, Block::RESIZABLE_ASSIGN).clone())); __mm_s.push_str(&*literal!("]\n")); ArcStr::from(__mm_s) },
        Deref @ GENERIC_ASSIGN { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::GENERIC_ASSIGN).clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*literal!("single generic call [index  ")); __mm_s.push_str(&*intString(var_field!((*blck).call_index, Block::GENERIC_ASSIGN).clone())); __mm_s.push_str(&*literal!("] ")); __mm_s.push_str(&*List::toString(var_field!((*blck).scal_indices, Block::GENERIC_ASSIGN).clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 10)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ ENTWINED_ASSIGN { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(var_field!((*blck).single_calls, Block::ENTWINED_ASSIGN).clone(), Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| toString(__pe_a0, __pe_b1.clone()) }), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### entwined call (")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::ENTWINED_ASSIGN).clone())); __mm_s.push_str(&*literal!(") ###")); ArcStr::from(__mm_s) }).clone(), (literal!("\n    ")).clone(), (literal!("    ")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) },
        Deref @ ALIAS { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::ALIAS).clone())); __mm_s.push_str(&*literal!(") Alias of ")); __mm_s.push_str(&*intString(var_field!((*blck).aliasOf, Block::ALIAS).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ ALGORITHM { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::ALGORITHM).clone())); __mm_s.push_str(&*literal!(") Algorithm\n")); __mm_s.push_str(&*Statement::toStringList(var_field!((*blck).stmts, Block::ALGORITHM).clone(), (r#str.clone()).clone())?); ArcStr::from(__mm_s) },
        Deref @ INVERSE_ALGORITHM { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::INVERSE_ALGORITHM).clone())); __mm_s.push_str(&*literal!(") Inverse Algorithm\n")); __mm_s.push_str(&*Statement::toStringList(var_field!((*blck).stmts, Block::INVERSE_ALGORITHM).clone(), (r#str.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        Deref @ IF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::IF).clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*List::toString(var_field!((*blck).branches, Block::IF).clone(), Arc::new({ let __pe_b1 = (r#str.clone()).clone(); move |__pe_a0| ifTplStr(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (r#str.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("else ")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("end if;\n")); ArcStr::from(__mm_s) }).clone(), true, 0)?); ArcStr::from(__mm_s) },
        Deref @ WHEN { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::WHEN).clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*whenString(var_field!((*blck).conditions, Block::WHEN).clone(), var_field!((*blck).when_stmts, Block::WHEN).clone(), var_field!((*blck).else_when, Block::WHEN).clone(), (r#str.clone()).clone())?); ArcStr::from(__mm_s) },
        Deref @ LINEAR { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).system, Block::LINEAR).index.clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*LinearSystem::toString(var_field!((*blck).system, Block::LINEAR).clone(), (r#str.clone()).clone())?); ArcStr::from(__mm_s) },
        Deref @ NONLINEAR { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).system, Block::NONLINEAR).index.clone())); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*NonlinearSystem::toString(var_field!((*blck).system, Block::NONLINEAR).clone(), (r#str.clone()).clone())?); ArcStr::from(__mm_s) },
        Deref @ HYBRID { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(var_field!((*blck).index, Block::HYBRID).clone())); __mm_s.push_str(&*literal!(") Hybrid\n")); ArcStr::from(__mm_s) },
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.toString")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub fn forTplStr(mut tpl: (Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>)) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        (name, range) = tpl.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::toString(name.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*Expression::toString(range.clone())?); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn ifTplStr(mut tpl: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Block>>>), mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut condition: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut blcks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        (condition, blcks) = tpl.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*Expression::toString(condition.clone())?); __mm_s.push_str(&*literal!(" then\n  ")); __mm_s.push_str(&*List::toString(blcks.clone(), Arc::new({ let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(); move |__pe_a0| toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn getIndex(mut blck: Arc<Block>) -> Result<i32> {
        let mut index: i32 = 0;
        index = (::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ RESIDUAL { .. } => var_field!((*blck).index, Block::RESIDUAL).clone(),
        Deref @ ARRAY_RESIDUAL { .. } => var_field!((*blck).index, Block::ARRAY_RESIDUAL).clone(),
        Deref @ FOR_RESIDUAL { .. } => var_field!((*blck).index, Block::FOR_RESIDUAL).clone(),
        Deref @ SIMPLE_ASSIGN { .. } => var_field!((*blck).index, Block::SIMPLE_ASSIGN).clone(),
        Deref @ ARRAY_ASSIGN { .. } => var_field!((*blck).index, Block::ARRAY_ASSIGN).clone(),
        Deref @ RESIZABLE_ASSIGN { .. } => var_field!((*blck).index, Block::RESIZABLE_ASSIGN).clone(),
        Deref @ GENERIC_ASSIGN { .. } => var_field!((*blck).index, Block::GENERIC_ASSIGN).clone(),
        Deref @ ENTWINED_ASSIGN { .. } => var_field!((*blck).index, Block::ENTWINED_ASSIGN).clone(),
        Deref @ ALIAS { .. } => var_field!((*blck).index, Block::ALIAS).clone(),
        Deref @ ALGORITHM { .. } => var_field!((*blck).index, Block::ALGORITHM).clone(),
        Deref @ INVERSE_ALGORITHM { .. } => var_field!((*blck).index, Block::INVERSE_ALGORITHM).clone(),
        Deref @ IF { .. } => var_field!((*blck).index, Block::IF).clone(),
        Deref @ WHEN { .. } => var_field!((*blck).index, Block::WHEN).clone(),
        Deref @ LINEAR { .. } => var_field!((*blck).system, Block::LINEAR).index.clone(),
        Deref @ NONLINEAR { .. } => var_field!((*blck).system, Block::NONLINEAR).index.clone(),
        Deref @ HYBRID { .. } => var_field!((*blck).index, Block::HYBRID).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.getIndex")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*toString(blck.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(index)
    }

    pub fn isDiscrete(mut blck: Arc<Block>) -> bool {
        let mut b: bool = false;
        b = (::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ RESIDUAL { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ ARRAY_RESIDUAL { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ FOR_RESIDUAL { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ SIMPLE_ASSIGN { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ ARRAY_ASSIGN { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ RESIZABLE_ASSIGN { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ GENERIC_ASSIGN { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ ENTWINED_ASSIGN { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ ALIAS { .. } => {
            var_field!((*blck).isDiscrete, Block::ALIAS).clone()
        },
        Deref @ ALGORITHM { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ INVERSE_ALGORITHM { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ IF { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        Deref @ WHEN { attr, .. } => {
            attr.kind.clone() == EquationKind::DISCRETE.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub fn filterWhen(mut blcks: Arc<metamodelica::List<Arc<Block>>>, mut out_blcks: Arc<metamodelica::List<Arc<Block>>>, mut new_blcks: Arc<metamodelica::List<Arc<Block>>>, mut indices: SimCodeIndices) -> Result<(Arc<metamodelica::List<Arc<Block>>>, Arc<metamodelica::List<Arc<Block>>>, SimCodeIndices)> {
        let mut out_blcks: Arc<metamodelica::List<Arc<Block>>> = out_blcks;
        let mut new_blcks: Arc<metamodelica::List<Arc<Block>>> = new_blcks;
        let mut indices: SimCodeIndices = indices;
        let mut blck: Arc<Block>;
        let mut new_blck: Arc<Block>;
        let mut rest: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        (out_blcks, new_blcks, indices) = (::match_deref::match_deref! { match &(blcks.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ WHEN { .. }, tail: rest } => filterWhen(rest.clone(), out_blcks.clone(), new_blcks.clone(), indices.clone())?,
        Deref @ metamodelica::List::Cons { head: blck @ Deref @ ALGORITHM { .. }, tail: rest } => {
            stmts = Statement::filterDiscrete(var_field!((**blck).stmts, Block::ALGORITHM).clone(), metamodelica::nil());
            if stmts.clone().is_empty() {
                (out_blcks, new_blcks, indices) = filterWhen(rest.clone(), out_blcks.clone(), new_blcks.clone(), indices.clone())?;
            } else if List::compareLength(stmts.clone(), var_field!((**blck).stmts, Block::ALGORITHM).clone())? != 0 {
                new_blck = Arc::new(Block::ALGORITHM { index: indices.equationIndex.clone(), stmts: stmts.clone(), attr: var_field!((**blck).attr, Block::ALGORITHM).clone() });
                indices.equationIndex = indices.equationIndex.clone() + 1;
                (out_blcks, new_blcks, indices) = filterWhen(rest.clone(), cons(new_blck.clone(), out_blcks.clone()), cons(new_blck.clone(), new_blcks.clone()), indices.clone())?;
            } else {
                (out_blcks, new_blcks, indices) = filterWhen(rest.clone(), cons(blck.clone(), out_blcks.clone()), new_blcks.clone(), indices.clone())?;
            }
            (out_blcks.clone(), new_blcks.clone(), indices.clone())
        },
        Deref @ metamodelica::List::Cons { head: blck, tail: rest } => filterWhen(rest.clone(), cons(blck.clone(), out_blcks.clone()), new_blcks.clone(), indices.clone())?,
        _ => (out_blcks.clone(), new_blcks.clone(), indices.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((out_blcks, new_blcks, indices))
    }

    pub fn map(mut blck: Arc<Block>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Block>> {
        pub type expFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

        let mut blck: Arc<Block> = blck;
        blck = (::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ RESIDUAL { .. } => {
            assign_variant_field!(blck => Block::RESIDUAL; exp = Expression::map(var_field!((*blck).exp, Block::RESIDUAL).clone(), func.clone())?);
            blck.clone()
        },
        Deref @ SIMPLE_ASSIGN { .. } => {
            assign_variant_field!(blck => Block::SIMPLE_ASSIGN; rhs = Expression::map(var_field!((*blck).rhs, Block::SIMPLE_ASSIGN).clone(), func.clone())?);
            blck.clone()
        },
        _ => blck.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(blck)
    }

    pub fn listToString(mut blcks: Arc<metamodelica::List<Arc<Block>>>, mut r#str: ArcStr, mut header: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut indent: ArcStr = r#str.clone();
        r#str = (if (header.clone() != literal!("")) {StringUtil::headline_3((header.clone()).clone())} else {literal!("")}).clone();
        for mut blck in &*blcks.clone() {
            let mut blck = blck.clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*toString(blck.clone(), (indent.clone()).clone())?); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub fn createBlocks(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut all_blcks: Arc<metamodelica::List<Arc<Block>>>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>>, Arc<metamodelica::List<Arc<Block>>>, SimCodeIndices)> {
        let mut blcks: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>> = metamodelica::nil();
        let mut all_blcks: Arc<metamodelica::List<Arc<Block>>> = all_blcks;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut tmp: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        for mut partition in &*partitions.clone() {
            let mut partition = partition.clone();
            (tmp, simCodeIndices) = fromPartition(partition.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
            blcks = cons(tmp.clone(), blcks.clone());
            all_blcks = listAppend(tmp.clone(), all_blcks.clone());
        }
        blcks = blcks.clone().reverse();
        Ok((blcks, all_blcks, simCodeIndices))
    }

    pub fn createDiscreteBlocks(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut blcks: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>>, mut all_blcks: Arc<metamodelica::List<Arc<Block>>>, mut event_dependencies: Arc<metamodelica::List<Arc<Block>>>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>>, Arc<metamodelica::List<Arc<Block>>>, Arc<metamodelica::List<Arc<Block>>>, SimCodeIndices)> {
        let mut blcks: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>> = blcks;
        let mut all_blcks: Arc<metamodelica::List<Arc<Block>>> = all_blcks;
        let mut event_dependencies: Arc<metamodelica::List<Arc<Block>>> = event_dependencies;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut tmp: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut new_blcks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        for mut partition in &*partitions.clone() {
            let mut partition = partition.clone();
            (tmp, simCodeIndices) = fromPartition(partition.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
            all_blcks = listAppend(tmp.clone(), all_blcks.clone());
            (tmp, new_blcks, simCodeIndices) = filterWhen(tmp.clone().reverse(), metamodelica::nil(), metamodelica::nil(), simCodeIndices.clone())?;
            all_blcks = listAppend(new_blcks.clone(), all_blcks.clone());
            blcks = cons(tmp.clone(), blcks.clone());
            tmp = ({
        let mut __acc: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        for mut blck in (tmp.clone()).into_iter().cloned() {
            if !(!(isDiscrete(blck.clone()))) { continue; }
            let __x = blck.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            event_dependencies = listAppend(tmp.clone(), event_dependencies.clone());
        }
        blcks = blcks.clone().reverse();
        Ok((blcks, all_blcks, event_dependencies, simCodeIndices))
    }

    pub fn createInitialBlocks(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<metamodelica::List<Arc<Block>>>, SimCodeIndices)> {
        let mut blcks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut tmp: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut tmp_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>> = metamodelica::nil();
        for mut partition in &*partitions.clone() {
            let mut partition = partition.clone();
            (tmp, simCodeIndices) = fromPartition(partition.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
            tmp_lst = cons(tmp.clone(), tmp_lst.clone());
        }
        blcks = List::flatten(tmp_lst.clone());
        Ok((blcks, simCodeIndices))
    }

    pub fn createDAEModeBlocks(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>>, Arc<metamodelica::List<Arc<SimVar::SimVar>>>, SimCodeIndices)> {
        let mut blcks: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>> = metamodelica::nil();
        let mut vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut indices_ptr: Pointer::Pointer<SimCodeIndices>;
        let mut vars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut tmp: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        for mut partition in &*partitions.clone().reverse() {
            let mut partition = partition.clone();
            indices_ptr = Pointer::create(simCodeIndices.clone());
            Partition::Partition::mapStrongComponents(partition.clone(), Arc::new({ let __pe_b1 = vars_ptr.clone(); let __pe_b2 = indices_ptr.clone(); let __pe_b3 = VarType::RESIDUAL.clone(); move |__pe_a0| SimVar::createFromResidualComponent(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
            (tmp, simCodeIndices) = fromPartition(partition.clone(), Pointer::access(indices_ptr.clone()), simcode_map.clone(), equation_map.clone())?;
            blcks = cons(tmp.clone(), blcks.clone());
        }
        vars = Pointer::access(vars_ptr.clone()).reverse();
        Ok((blcks, vars, simCodeIndices))
    }

    pub fn createClockedBlocks(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>, mut info: Arc<ClockedInfo::ClockedInfo>) -> Result<(Arc<metamodelica::List<Arc<SimPartition::NSimPartition>>>, Arc<metamodelica::List<Arc<Block>>>, SimCodeIndices)> {
        let mut baseParts: Arc<metamodelica::List<Arc<SimPartition::NSimPartition>>> = metamodelica::nil();
        let mut eventClocks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut clock_collector: Arc<UnorderedMap::UnorderedMap<Arc<BClock::BClock>, Arc<metamodelica::List<Arc<SimPartition::NSimPartition>>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(BClock::hash, Arc<BClock::BClock>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(BClock::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BClock::BClock>, Arc<BClock::BClock>) -> Result<bool> + 'static>), 1);
        let mut blcks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>> = metamodelica::nil();
        let mut clock: Arc<BClock::BClock>;
        let mut subClock: Arc<BClock::BClock>;
        let mut baseClock: Arc<BClock::BClock>;
        let mut holdEvents: bool = false;
        let mut baseClock_opt: Option<Arc<BClock::BClock>> = None;
        let mut subPart: Arc<SimPartition::NSimPartition>;
        for mut c in &*UnorderedMap::valueList(info.baseClocks.clone()) {
            let mut c = c.clone();
            UnorderedMap::add(c.clone(), metamodelica::nil(), clock_collector.clone())?;
        }
        for mut partition in &*partitions.clone().reverse() {
            let mut partition = partition.clone();
            (blcks, simCodeIndices) = fromPartition(partition.clone(), simCodeIndices.clone(), simcode_map.clone(), equation_map.clone())?;
            vars = SimVars::getPartitionVars(partition.clone(), simcode_map.clone())?;
            (clock, baseClock_opt, holdEvents) = Partition::Partition::getClocks(partition.clone())?;
            if isSome(baseClock_opt.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(baseClock_opt.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                baseClock = __pa0.clone();
                subClock = clock.clone();
            } else {
                baseClock = clock.clone();
                subClock = Partitioning::DEFAULT_SUB_CLOCK().clone();
            }
            subPart = SimPartition::createSubPartition(subClock.clone(), blcks.clone(), vars.clone(), holdEvents.clone());
            UnorderedMap::add(baseClock.clone(), cons(subPart.clone(), UnorderedMap::getSafe(baseClock.clone(), clock_collector.clone(), metamodelica::sourceInfo!())?), clock_collector.clone())?;
        }
        (baseParts, eventClocks, simCodeIndices) = SimPartition::createBasePartitions(clock_collector.clone(), simCodeIndices.clone());
        Ok((baseParts, eventClocks, simCodeIndices))
    }

    pub fn createNoReturnBlocks(mut equations: Arc<EquationPointers::EquationPointers>, mut simCodeIndices: SimCodeIndices, mut kind: Partition::Kind, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<metamodelica::List<Arc<Block>>>, SimCodeIndices)> {
        let mut blcks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
        let mut tmp: Arc<Block>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i.clone(), equations.eqArr.clone()) {
                eqn = Pointer::access(ExpandableArray::get(i.clone(), equations.eqArr.clone())?);
                (tmp, simCodeIndices) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            createEquation(BVariable::getVar(cref.clone(), metamodelica::sourceInfo!())?, eqn.clone(), Solve::Status::EXPLICIT.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            createEquation(BVariable::getVar(cref.clone(), metamodelica::sourceInfo!())?, eqn.clone(), Solve::Status::EXPLICIT.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            createEquation(BVariable::getVar(cref.clone(), metamodelica::sourceInfo!())?, eqn.clone(), Solve::Status::EXPLICIT.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?
        },
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            createEquation(BVariable::DUMMY_VARIABLE().clone(), eqn.clone(), Solve::Status::EXPLICIT.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?
        },
        Deref @ BEquation::Equation::ALGORITHM { .. } => {
            createAlgorithm(eqn.clone(), simCodeIndices.clone(), equation_map.clone())?
        },
        Deref @ BEquation::Equation::FOR_EQUATION { .. } => {
            createAlgorithm(eqn.clone(), simCodeIndices.clone(), equation_map.clone())?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createNoReturnBlocks")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                blcks = cons(tmp.clone(), blcks.clone());
            }
        }
        Ok((blcks, simCodeIndices))
    }

    pub fn fromPartition(mut partition: Arc<Partition::Partition::Partition>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<metamodelica::List<Arc<Block>>>, SimCodeIndices)> {
        let mut blcks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        blcks = ({
        let mut result: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        (match partition.strongComponents.clone() {
        Some(mut comps) => {
            let mut kind: Partition::Kind = Partition::Kind::ODE;
            let mut tmp: Arc<Block>;
            let mut index: i32 = 0;
            let mut alias_index: i32 = 0;
            kind = Partition::Partition::getKind(partition.clone());
            let __range0 = (1..=(comps.clone().borrow().len() as i32)).rev();
            for mut i in __range0 {
                (tmp, simCodeIndices, index) = fromStrongComponent(comps.borrow()[(i.clone()-1) as usize].clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
                alias_index = (::match_deref::match_deref! { match &(comps.borrow()[(i.clone()-1) as usize].clone()) {
        Deref @ StrongComponent::ALIAS { aliasInfo, .. } => {
            UnorderedMap::getOrDefault(aliasInfo.clone(), simCodeIndices.alias_map.clone(), -1)
        },
        _ => {
            index.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                UnorderedMap::add(Arc::new(AliasInfo::AliasInfo { kind: kind.clone(), partitionIndex: partition.index.clone(), componentIndex: i.clone() }), alias_index.clone(), simCodeIndices.alias_map.clone())?;
                result = cons(tmp.clone(), result.clone());
            }
            result.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.fromPartition")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*Partition::Partition::toString(partition.clone(), 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    })
    });
        Ok((blcks, simCodeIndices))
    }

    pub fn fromStrongComponent(mut comp: Arc<StrongComponent::NBStrongComponent>, mut simCodeIndices: SimCodeIndices, mut kind: Partition::Kind, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<Block>, SimCodeIndices, i32)> {
        let mut blck: Arc<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut index: i32 = 0;
        (blck, index) = ({
        let mut eqns: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut residual_index: i32 = 0;
        let mut single_calls: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        let mut call_order: Arc<metamodelica::List<i32>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => {
            let mut tmp: Arc<Block>;
            (tmp, simCodeIndices) = createEquation(Pointer::access(var_field!((*comp).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), Pointer::access(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), var_field!((*comp).status, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
            (tmp.clone(), getIndex(tmp.clone())?)
        },
        Deref @ StrongComponent::MULTI_COMPONENT { .. } => {
            let mut tmp: Arc<Block>;
            (tmp, simCodeIndices) = createEquation(BVariable::DUMMY_VARIABLE().clone(), Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone())), var_field!((*comp).status, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
            (tmp.clone(), getIndex(tmp.clone())?)
        },
        Deref @ StrongComponent::SLICED_COMPONENT { .. } if (BEquation::Equation::isForEquation(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone()))) => {
            let mut tmp: Arc<Block>;
            (tmp, simCodeIndices) = createAlgorithm(Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone())), simCodeIndices.clone(), equation_map.clone())?;
            (tmp.clone(), getIndex(tmp.clone())?)
        },
        Deref @ StrongComponent::SLICED_COMPONENT { .. } => {
            let mut tmp: Arc<Block>;
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            eqn = Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone()));
            (tmp, simCodeIndices) = createEquation(Variable::fromCref(var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone())?, eqn.clone(), var_field!((*comp).status, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
            (tmp.clone(), getIndex(tmp.clone())?)
        },
        Deref @ StrongComponent::RESIZABLE_COMPONENT { .. } if (BEquation::Equation::isForEquation(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone()))) => {
            let mut tmp: Arc<Block>;
            let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut generic_call_index: i32 = 0;
            let mut ident: Arc<Identifier::Identifier> = Arc::new(<Identifier::Identifier as ::std::default::Default>::default());
            let mut iters: Arc<metamodelica::List<Arc<SimIterator::SimIterator>>> = metamodelica::nil();
            eqn_ptr = Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone());
            eqn = Pointer::access(eqn_ptr.clone());
            ident = Arc::new(Identifier::Identifier { eqn: eqn_ptr.clone(), var_cref: var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone(), resizable: true });
            iters = SimIterator::fromIterator(BEquation::Equation::getForIterator(eqn.clone()))?;
            generic_call_index = UnorderedMap::tryAdd(ident.clone(), UnorderedMap::size(simCodeIndices.generic_call_map.clone()), simCodeIndices.generic_call_map.clone())?;
            tmp = Arc::new(Block::RESIZABLE_ASSIGN { index: simCodeIndices.equationIndex.clone(), call_index: generic_call_index.clone(), iters: iters.clone(), source: BEquation::Equation::getSource(eqn.clone()), attr: BEquation::Equation::getAttributes(eqn.clone()) });
            UnorderedMap::add(BEquation::Equation::getEqnName(eqn_ptr.clone())?, tmp.clone(), equation_map.clone())?;
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            (tmp.clone(), getIndex(tmp.clone())?)
        },
        Deref @ StrongComponent::GENERIC_COMPONENT { .. } => {
            let mut tmp: Arc<Block>;
            let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
            let mut generic_call_index: i32 = 0;
            let mut ident: Arc<Identifier::Identifier> = Arc::new(<Identifier::Identifier as ::std::default::Default>::default());
            eqn_ptr = Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::GENERIC_COMPONENT).clone());
            eqn = Pointer::access(eqn_ptr.clone());
            ident = Arc::new(Identifier::Identifier { eqn: eqn_ptr.clone(), var_cref: var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::GENERIC_COMPONENT).clone(), resizable: false });
            generic_call_index = UnorderedMap::tryAdd(ident.clone(), UnorderedMap::size(simCodeIndices.generic_call_map.clone()), simCodeIndices.generic_call_map.clone())?;
            tmp = Arc::new(Block::GENERIC_ASSIGN { index: simCodeIndices.equationIndex.clone(), call_index: generic_call_index.clone(), scal_indices: var_field!((*comp).eqn, StrongComponent::NBStrongComponent::GENERIC_COMPONENT).indices.clone(), source: BEquation::Equation::getSource(eqn.clone()), attr: BEquation::Equation::getAttributes(eqn.clone()) });
            UnorderedMap::add(BEquation::Equation::getEqnName(eqn_ptr.clone())?, tmp.clone(), equation_map.clone())?;
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            (tmp.clone(), getIndex(tmp.clone())?)
        },
        Deref @ StrongComponent::ENTWINED_COMPONENT { .. } => {
            let mut tmp: Arc<Block>;
            let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut single_call: Arc<Block>;
            let mut entwined_index_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
            entwined_index_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            for mut slice in &*var_field!((*comp).entwined_slices, StrongComponent::NBStrongComponent::ENTWINED_COMPONENT).clone() {
                let mut slice = slice.clone();
                (single_call, simCodeIndices, _) = fromStrongComponent(slice.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
                UnorderedMap::add(getGenericEquationName(slice.clone())?, getGenericAssignIndex(single_call.clone())?, entwined_index_map.clone())?;
                single_calls = cons(single_call.clone(), single_calls.clone());
            }
            for mut tpl in &*var_field!((*comp).entwined_tpl_lst, StrongComponent::NBStrongComponent::ENTWINED_COMPONENT).clone().reverse() {
                let mut tpl = tpl.clone();
                (eqn_ptr, _) = tpl.clone();
                call_order = cons(UnorderedMap::getSafe(BEquation::Equation::getEqnName(eqn_ptr.clone())?, entwined_index_map.clone(), metamodelica::sourceInfo!())?, call_order.clone());
            }
            tmp = Arc::new(Block::ENTWINED_ASSIGN { index: simCodeIndices.equationIndex.clone(), call_order: call_order.clone(), single_calls: single_calls.clone(), source: DAE::emptyElementSource().clone(), attr: BEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None) });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            (tmp.clone(), getIndex(tmp.clone())?)
        },
        Deref @ StrongComponent::ALGEBRAIC_LOOP { strict, .. } => {
            let mut system: Arc<NonlinearSystem::NonlinearSystem> = Arc::new(<NonlinearSystem::NonlinearSystem as ::std::default::Default>::default());
            let mut tmp: Arc<Block>;
            let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            let mut jacobian: Option<Arc<SimJacobian::SimJacobian>> = None;
            let __range0 = 1..=(strict.innerEquations.clone().borrow().len() as i32);
            for mut i in __range0 {
                (tmp, simCodeIndices, _) = fromStrongComponent(strict.innerEquations.borrow()[(i.clone()-1) as usize].clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
                eqns = cons(tmp.clone(), eqns.clone());
            }
            for mut slice in &*strict.residual_eqns.clone() {
                let mut slice = slice.clone();
                (tmp, simCodeIndices, residual_index) = createResidual(slice.clone(), simCodeIndices.clone(), residual_index.clone(), equation_map.clone())?;
                eqns = cons(tmp.clone(), eqns.clone());
            }
            for mut slice in &*strict.iteration_vars.clone() {
                let mut slice = slice.clone();
                var = Pointer::access(Slice::getT(slice.clone()));
                if Variable::size(var.clone(), false) > 1 {
                    for mut scal_var in &*Scalarize::scalarizeBackendVariable(var.clone(), slice.indices.clone())? {
                        let mut scal_var = scal_var.clone();
                        crefs = cons(scal_var.name.clone(), crefs.clone());
                    }
                } else {
                    crefs = cons(var.name.clone(), crefs.clone());
                }
            }
            if false && isSome(strict.jac.clone()) {
                (jacobian, simCodeIndices) = SimJacobian::create(Util::getOption(strict.jac.clone())?, simCodeIndices.clone(), simcode_map.clone())?;
            } else {
                jacobian = None;
            }
            system = Arc::new(NonlinearSystem::NonlinearSystem { torn: true, mixed: var_field!((*comp).mixed, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone(), homotopy: var_field!((*comp).homotopy, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone(), jacobian: Pointer::create(jacobian.clone()), size: (crefs.clone().len() as i32), indexSystem: simCodeIndices.nonlinearSystemIndex.clone(), crefs: crefs.clone().reverse(), blcks: eqns.clone().reverse(), index: simCodeIndices.equationIndex.clone() });
            simCodeIndices.nonlinearSystemIndex = simCodeIndices.nonlinearSystemIndex.clone() + 1;
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            (Arc::new(Block::NONLINEAR { system: system.clone(), alternativeTearing: None }), system.index.clone())
        },
        Deref @ StrongComponent::ALIAS { .. } => {
            let mut tmp: Arc<Block>;
            let mut aliasOf: i32 = 0;
            aliasOf = UnorderedMap::getOrDefault(var_field!((*comp).aliasInfo, StrongComponent::NBStrongComponent::ALIAS).clone(), simCodeIndices.alias_map.clone(), -1);
            tmp = Arc::new(Block::ALIAS { index: simCodeIndices.equationIndex.clone(), aliasInfo: var_field!((*comp).aliasInfo, StrongComponent::NBStrongComponent::ALIAS).clone(), aliasOf: aliasOf.clone(), isDiscrete: StrongComponent::isDiscrete(comp.clone())? && !(StrongComponent::isAlgebraicLoop(var_field!((*comp).original, StrongComponent::NBStrongComponent::ALIAS).clone())) });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            (tmp.clone(), getIndex(tmp.clone())?)
        },
        Deref @ StrongComponent::ENTWINED_COMPONENT { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.fromStrongComponent")); __mm_s.push_str(&*literal!(" failed because entwined equations have to be resolved beforehand in Solve.solve(). Failed for:\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.fromStrongComponent")); __mm_s.push_str(&*literal!(" failed with unknown reason for\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        Ok((blck, simCodeIndices, index))
    }

    pub fn createResidual(mut slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, mut simCodeIndices: SimCodeIndices, mut res_idx: i32, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<Block>, SimCodeIndices, i32)> {
        let mut blck: Arc<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut res_idx: i32 = res_idx;
        let mut eqn: Arc<Equation::Equation> = Pointer::access(Slice::getT(slice.clone()));
        blck = (::match_deref::match_deref! { match &((eqn.clone(), slice.indices.clone())) {
        (Deref @ BEquation::Equation::SCALAR_EQUATION { .. }, Deref @ metamodelica::List::Nil) => {
            let mut tmp: Arc<Block>;
            tmp = Arc::new(Block::RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx.clone(), exp: var_field!((*eqn).rhs, Equation::Equation::SCALAR_EQUATION).clone(), source: var_field!((*eqn).source, Equation::Equation::SCALAR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::SCALAR_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            res_idx = res_idx.clone() + 1;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::ARRAY_EQUATION { .. }, Deref @ metamodelica::List::Nil) => {
            let mut tmp: Arc<Block>;
            tmp = Arc::new(Block::ARRAY_RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx.clone(), exp: var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), source: var_field!((*eqn).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            res_idx = res_idx.clone() + BEquation::Equation::size(Slice::getT(slice.clone()), false)?;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::IF_EQUATION { .. }, _) => {
            let mut tmp: Arc<Block>;
            (tmp, simCodeIndices, res_idx) = createResidual(Arc::new(Slice::NBSlice { t: Pointer::create(BEquation::IfEquationBody::inline(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), eqn.clone())?), indices: slice.indices.clone() }), simCodeIndices.clone(), res_idx.clone(), equation_map.clone())?;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ metamodelica::List::Nil) => {
            let mut tmp: Arc<Block>;
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            (names, ranges, _) = BEquation::Iterator::getFrames(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            tmp = Arc::new(Block::FOR_RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx.clone(), iterators: List::zip(names.clone(), ranges.clone()), exp: Util::getOption(BEquation::Equation::getRHS(eqn.clone())?)?, source: var_field!((*eqn).source, Equation::Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::FOR_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            res_idx = res_idx.clone() + BEquation::Equation::size(Slice::getT(slice.clone()), false)?;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut tmp: Arc<Block>;
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
            (names, ranges, _) = BEquation::Iterator::getFrames(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            tmp = Arc::new(Block::GENERIC_RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx.clone(), scal_indices: slice.indices.clone(), iterators: List::zip(names.clone(), ranges.clone()), exp: Util::getOption(BEquation::Equation::getRHS(eqn.clone())?)?, source: var_field!((*eqn).source, Equation::Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::FOR_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            res_idx = res_idx.clone() + (slice.indices.clone().len() as i32);
            tmp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createResidual")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*Slice::toString(slice.clone(), Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        UnorderedMap::add(BEquation::Equation::getEqnName(Pointer::create(eqn.clone()))?, blck.clone(), equation_map.clone())?;
        Ok((blck, simCodeIndices, res_idx))
    }

    pub fn createEquation(mut var: Arc<Variable::NFVariable>, mut eqn: Arc<Equation::Equation>, mut status: Solve::Status, mut simCodeIndices: SimCodeIndices, mut kind: Partition::Kind, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<Block>, SimCodeIndices)> {
        let mut blck: Arc<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        blck = (::match_deref::match_deref! { match &((eqn.clone(), status.clone())) {
        (Deref @ BEquation::Equation::SCALAR_EQUATION { .. }, Solve::Status::EXPLICIT) => {
            let mut tmp: Arc<Block>;
            tmp = Arc::new(Block::SIMPLE_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: var.name.clone(), rhs: var_field!((*eqn).rhs, Equation::Equation::SCALAR_EQUATION).clone(), source: var_field!((*eqn).source, Equation::Equation::SCALAR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::SCALAR_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::ARRAY_EQUATION { .. }, Solve::Status::EXPLICIT) => {
            let mut tmp: Arc<Block>;
            tmp = Arc::new(Block::ARRAY_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: var_field!((*eqn).lhs, Equation::Equation::ARRAY_EQUATION).clone(), rhs: var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), source: var_field!((*eqn).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::RECORD_EQUATION { .. }, Solve::Status::EXPLICIT) => {
            let mut tmp: Arc<Block>;
            (tmp, simCodeIndices) = createAlgorithm(eqn.clone(), simCodeIndices.clone(), equation_map.clone())?;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::WHEN_EQUATION { .. }, Solve::Status::EXPLICIT) => {
            let mut tmp: Arc<Block>;
            (tmp, simCodeIndices) = createWhenBody(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone(), var_field!((*eqn).source, Equation::Equation::WHEN_EQUATION).clone(), var_field!((*eqn).attr, Equation::Equation::WHEN_EQUATION).clone(), simCodeIndices.clone())?;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::IF_EQUATION { .. }, Solve::Status::EXPLICIT) => {
            let mut tmp: Arc<Block>;
            let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Block>>>)>> = metamodelica::nil();
            (branches, simCodeIndices) = createIfBody(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), metamodelica::nil(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
            tmp = Arc::new(Block::IF { index: simCodeIndices.equationIndex.clone(), branches: branches.clone().reverse(), source: var_field!((*eqn).source, Equation::Equation::IF_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::IF_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            tmp.clone()
        },
        (Deref @ BEquation::Equation::ALGORITHM { .. }, Solve::Status::EXPLICIT) => {
            let mut tmp: Arc<Block>;
            tmp = Arc::new(Block::ALGORITHM { index: simCodeIndices.equationIndex.clone(), stmts: var_field!((*eqn).alg, Equation::Equation::ALGORITHM).statements.clone(), attr: var_field!((*eqn).attr, Equation::Equation::ALGORITHM).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            tmp.clone()
        },
        (_, Solve::Status::IMPLICIT) => {
            let mut tmp: Arc<Block>;
            (tmp, simCodeIndices) = createImplicitEquation(var.clone(), eqn.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
            tmp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createEquation")); __mm_s.push_str(&*literal!(" failed with status ")); __mm_s.push_str(&*Solve::statusString(status.clone())); __mm_s.push_str(&*literal!(" for\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        UnorderedMap::add(BEquation::Equation::getEqnName(Pointer::create(eqn.clone()))?, blck.clone(), equation_map.clone())?;
        Ok((blck, simCodeIndices))
    }

    pub fn createImplicitEquation(mut var: Arc<Variable::NFVariable>, mut eqn: Arc<Equation::Equation>, mut simCodeIndices: SimCodeIndices, mut kind: Partition::Kind, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<Block>, SimCodeIndices)> {
        let mut blck: Arc<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut comp: Arc<StrongComponent::NBStrongComponent>;
        let mut index: i32 = 0;
        (comp, index) = Tearing::implicit(Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { var: Pointer::create(var.clone()), eqn: Pointer::create(eqn.clone()), status: Solve::Status::IMPLICIT.clone() }), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1), simCodeIndices.implicitIndex.clone(), kind.clone())?;
        simCodeIndices.implicitIndex = index.clone();
        (blck, simCodeIndices, _) = fromStrongComponent(comp.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
        Ok((blck, simCodeIndices))
    }

    pub fn createWhenBody(mut body: Arc<WhenEquationBody::WhenEquationBody>, mut source: Arc<DAE::ElementSource>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut simCodeIndices: SimCodeIndices) -> Result<(Arc<Block>, SimCodeIndices)> {
        let mut blck: Arc<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut conditions: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut when_stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>> = metamodelica::nil();
        let mut else_when: Option<Arc<WhenEquationBody::WhenEquationBody>> = None;
        let mut tmp: Arc<Block>;
        let mut else_when_block: Option<Arc<Block>> = None;
        let mut index: i32 = simCodeIndices.equationIndex.clone();
        simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
        (conditions, when_stmts, else_when) = BEquation::WhenEquationBody::getBodyAttributes(body.clone())?;
        if isSome(else_when.clone()) {
            (tmp, simCodeIndices) = createWhenBody(Util::getOption(else_when.clone())?, source.clone(), attr.clone(), simCodeIndices.clone())?;
            else_when_block = Some(tmp.clone());
        } else {
            else_when_block = None;
        }
        blck = Arc::new(Block::WHEN { index: index.clone(), initialCall: false, conditions: conditions.clone(), when_stmts: when_stmts.clone(), else_when: else_when_block.clone(), source: source.clone(), attr: attr.clone() });
        Ok((blck, simCodeIndices))
    }

    pub fn createIfBody(mut body: Arc<IfEquationBody::IfEquationBody>, mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Block>>>)>>, mut simCodeIndices: SimCodeIndices, mut kind: Partition::Kind, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Block>>>)>>, SimCodeIndices)> {
        let mut branches: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Block>>>)>> = branches;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        let mut blck: Arc<Block>;
        let mut blcks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        comps = ({
        let mut __acc: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        for mut eqn in (body.then_eqns.clone()).into_iter().cloned() {
            let __x = StrongComponent::fromSolvedEquationSlice(Arc::new(Slice::NBSlice { t: eqn.clone(), indices: metamodelica::nil() }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        for mut comp in &*comps.clone().reverse() {
            let mut comp = comp.clone();
            (blck, simCodeIndices, _) = fromStrongComponent(comp.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
            blcks = cons(blck.clone(), blcks.clone());
        }
        branches = cons((body.condition.clone(), blcks.clone()), branches.clone());
        if isSome(body.else_if.clone()) {
            (branches, simCodeIndices) = createIfBody(Util::getOption(body.else_if.clone())?, branches.clone(), simCodeIndices.clone(), kind.clone(), simcode_map.clone(), equation_map.clone())?;
        }
        Ok((branches, simCodeIndices))
    }

    pub fn createAlgorithm(mut eqn: Arc<Equation::Equation>, mut indices: SimCodeIndices, mut equation_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Block>>>) -> Result<(Arc<Block>, SimCodeIndices)> {
        let mut blck: Arc<Block>;
        let mut indices: SimCodeIndices = indices;
        let mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        stmts = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::ALGORITHM { .. } => var_field!((*eqn).alg, Equation::Equation::ALGORITHM).statements.clone(),
        _ => BEquation::Equation::toStatement(eqn.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        blck = Arc::new(Block::ALGORITHM { index: indices.equationIndex.clone(), stmts: stmts.clone(), attr: BEquation::Equation::getAttributes(eqn.clone()) });
        indices.equationIndex = indices.equationIndex.clone() + 1;
        UnorderedMap::add(BEquation::Equation::getEqnName(Pointer::create(eqn.clone()))?, blck.clone(), equation_map.clone())?;
        Ok((blck, indices))
    }

    pub fn createAssignment(mut eqn: Arc<Equation::Equation>, mut simCodeIndices: SimCodeIndices) -> Result<(Arc<Block>, SimCodeIndices)> {
        let mut blck: Arc<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        blck = (::match_deref::match_deref! { match &(eqn.clone()) {
        qual @ Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            let mut tmp: Arc<Block>;
            tmp = Arc::new(Block::SIMPLE_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: cref.clone(), rhs: var_field!((**qual).rhs, Equation::Equation::SCALAR_EQUATION).clone(), source: var_field!((**qual).source, Equation::Equation::SCALAR_EQUATION).clone(), attr: var_field!((**qual).attr, Equation::Equation::SCALAR_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            tmp.clone()
        },
        qual @ Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            let mut tmp: Arc<Block>;
            tmp = Arc::new(Block::SIMPLE_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: cref.clone(), rhs: var_field!((**qual).rhs, Equation::Equation::ARRAY_EQUATION).clone(), source: var_field!((**qual).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: var_field!((**qual).attr, Equation::Equation::ARRAY_EQUATION).clone() });
            simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
            tmp.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createAssignment")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((blck, simCodeIndices))
    }

    pub fn collectAlgebraicLoops(mut blcks: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>>, mut linearLoops: Arc<metamodelica::List<Arc<Block>>>, mut nonlinearLoops: Arc<metamodelica::List<Arc<Block>>>, mut jacobians: Arc<metamodelica::List<Arc<SimJacobian::SimJacobian>>>, mut simCodeIndices: SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<Block>>>, Arc<metamodelica::List<Arc<Block>>>, Arc<metamodelica::List<Arc<SimJacobian::SimJacobian>>>, SimCodeIndices)> {
        let mut linearLoops: Arc<metamodelica::List<Arc<Block>>> = linearLoops;
        let mut nonlinearLoops: Arc<metamodelica::List<Arc<Block>>> = nonlinearLoops;
        let mut jacobians: Arc<metamodelica::List<Arc<SimJacobian::SimJacobian>>> = jacobians;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        for mut blck_lst in &*blcks.clone() {
            let mut blck_lst = blck_lst.clone();
            for mut blck in &*blck_lst.clone() {
                let mut blck = blck.clone();
                (linearLoops, nonlinearLoops) = (::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ LINEAR { .. } => {
            (cons(blck.clone(), linearLoops.clone()), nonlinearLoops.clone())
        },
        Deref @ NONLINEAR { .. } => {
            let mut jacobian: Option<Arc<SimJacobian::SimJacobian>> = None;
            jacobian = NonlinearSystem::getJacobian(var_field!((*blck).system, Block::NONLINEAR).clone());
            if isSome(jacobian.clone()) {
                jacobians = cons(Util::getOption(jacobian.clone())?, jacobians.clone());
            }
            assign_variant_field!(blck => Block::NONLINEAR; system = NonlinearSystem::setJacobian(var_field!((*blck).system, Block::NONLINEAR).clone(), jacobian.clone()));
            (linearLoops.clone(), cons(blck.clone(), nonlinearLoops.clone()))
        },
        _ => {
            (linearLoops.clone(), nonlinearLoops.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
        }
        Ok((linearLoops, nonlinearLoops, jacobians, simCodeIndices))
    }

    pub fn convert(mut blck: Arc<Block>) -> Result<Arc<OldSimCode::SimEqSystem>> {
        let mut oldBlck: Arc<OldSimCode::SimEqSystem>;
        oldBlck = ({
        let mut old_iterators: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>> = metamodelica::nil();
        let mut oldBranches: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>>)>> = metamodelica::nil();
        let mut else_branch: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ RESIDUAL { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_RESIDUAL { index: var_field!((*blck).index, Block::RESIDUAL).clone(), res_index: var_field!((*blck).res_index, Block::RESIDUAL).clone(), exp: Expression::toDAE(var_field!((*blck).exp, Block::RESIDUAL).clone(), false)?, source: var_field!((*blck).source, Block::RESIDUAL).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::RESIDUAL).clone())? })
        },
        Deref @ ARRAY_RESIDUAL { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_RESIDUAL { index: var_field!((*blck).index, Block::ARRAY_RESIDUAL).clone(), res_index: var_field!((*blck).res_index, Block::ARRAY_RESIDUAL).clone(), exp: Expression::toDAE(var_field!((*blck).exp, Block::ARRAY_RESIDUAL).clone(), false)?, source: var_field!((*blck).source, Block::ARRAY_RESIDUAL).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::ARRAY_RESIDUAL).clone())? })
        },
        Deref @ FOR_RESIDUAL { .. } => {
            let mut iter: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            for mut iterator in &*var_field!((*blck).iterators, Block::FOR_RESIDUAL).clone().reverse() {
                let mut iterator = iterator.clone();
                (iter, range) = iterator.clone();
                old_iterators = cons((ComponentRef::toDAE(iter.clone())?, Expression::toDAE(range.clone(), false)?), old_iterators.clone());
            }
            Arc::new(OldSimCode::SimEqSystem::SES_FOR_RESIDUAL { index: var_field!((*blck).index, Block::FOR_RESIDUAL).clone(), res_index: var_field!((*blck).res_index, Block::FOR_RESIDUAL).clone(), iterators: old_iterators.clone(), exp: Expression::toDAE(var_field!((*blck).exp, Block::FOR_RESIDUAL).clone(), false)?, source: var_field!((*blck).source, Block::FOR_RESIDUAL).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::FOR_RESIDUAL).clone())? })
        },
        Deref @ GENERIC_RESIDUAL { .. } => {
            let mut iter: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            for mut iterator in &*var_field!((*blck).iterators, Block::GENERIC_RESIDUAL).clone().reverse() {
                let mut iterator = iterator.clone();
                (iter, range) = iterator.clone();
                old_iterators = cons((ComponentRef::toDAE(iter.clone())?, Expression::toDAE(range.clone(), false)?), old_iterators.clone());
            }
            Arc::new(OldSimCode::SimEqSystem::SES_GENERIC_RESIDUAL { index: var_field!((*blck).index, Block::GENERIC_RESIDUAL).clone(), res_index: var_field!((*blck).res_index, Block::GENERIC_RESIDUAL).clone(), scal_indices: var_field!((*blck).scal_indices, Block::GENERIC_RESIDUAL).clone(), iterators: old_iterators.clone(), exp: Expression::toDAE(var_field!((*blck).exp, Block::GENERIC_RESIDUAL).clone(), false)?, source: var_field!((*blck).source, Block::GENERIC_RESIDUAL).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::GENERIC_RESIDUAL).clone())? })
        },
        Deref @ SIMPLE_ASSIGN { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: var_field!((*blck).index, Block::SIMPLE_ASSIGN).clone(), cref: ComponentRef::toDAE(var_field!((*blck).lhs, Block::SIMPLE_ASSIGN).clone())?, exp: Expression::toDAE(var_field!((*blck).rhs, Block::SIMPLE_ASSIGN).clone(), false)?, source: var_field!((*blck).source, Block::SIMPLE_ASSIGN).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::SIMPLE_ASSIGN).clone())? })
        },
        Deref @ ARRAY_ASSIGN { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: var_field!((*blck).index, Block::ARRAY_ASSIGN).clone(), lhs: Expression::toDAE(var_field!((*blck).lhs, Block::ARRAY_ASSIGN).clone(), false)?, exp: Expression::toDAE(var_field!((*blck).rhs, Block::ARRAY_ASSIGN).clone(), false)?, source: var_field!((*blck).source, Block::ARRAY_ASSIGN).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::ARRAY_ASSIGN).clone())? })
        },
        Deref @ RESIZABLE_ASSIGN { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { index: var_field!((*blck).index, Block::RESIZABLE_ASSIGN).clone(), call_index: var_field!((*blck).call_index, Block::RESIZABLE_ASSIGN).clone(), iters: ({
        let mut __acc: Arc<metamodelica::List<OldBackendDAE::SimIterator>> = metamodelica::nil();
        for mut it in (var_field!((*blck).iters, Block::RESIZABLE_ASSIGN).clone()).into_iter().cloned() {
            let __x = SimIterator::convert(it.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), source: var_field!((*blck).source, Block::RESIZABLE_ASSIGN).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::RESIZABLE_ASSIGN).clone())? })
        },
        Deref @ GENERIC_ASSIGN { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_GENERIC_ASSIGN { index: var_field!((*blck).index, Block::GENERIC_ASSIGN).clone(), call_index: var_field!((*blck).call_index, Block::GENERIC_ASSIGN).clone(), scal_indices: var_field!((*blck).scal_indices, Block::GENERIC_ASSIGN).clone(), source: var_field!((*blck).source, Block::GENERIC_ASSIGN).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::GENERIC_ASSIGN).clone())? })
        },
        Deref @ ENTWINED_ASSIGN { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_ENTWINED_ASSIGN { index: var_field!((*blck).index, Block::ENTWINED_ASSIGN).clone(), call_order: var_field!((*blck).call_order, Block::ENTWINED_ASSIGN).clone(), single_calls: ({
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut single_call in (var_field!((*blck).single_calls, Block::ENTWINED_ASSIGN).clone()).into_iter().cloned() {
            let __x = convert(single_call.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), source: var_field!((*blck).source, Block::ENTWINED_ASSIGN).clone(), eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::ENTWINED_ASSIGN).clone())? })
        },
        Deref @ IF { .. } => {
            let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut blcks: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
            for mut branch in &*var_field!((*blck).branches, Block::IF).clone() {
                let mut branch = branch.clone();
                (exp, blcks) = branch.clone();
                if Expression::isEnd(exp.clone()) {
                    if else_branch.clone().is_empty() {
                        else_branch = ({
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck_ in (blcks.clone()).into_iter().cloned() {
            let __x = convert(blck_.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    } else {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert")); __mm_s.push_str(&*literal!(" failed because there is\n                  at least two non-conditional branches in:\n")); __mm_s.push_str(&*toString(blck.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                        bail!("fail");
                    }
                } else if else_branch.clone().is_empty() {
                    oldBranches = cons((Expression::toDAE(exp.clone(), false)?, ({
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck_ in (blcks.clone()).into_iter().cloned() {
            let __x = convert(blck_.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), oldBranches.clone());
                } else {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert")); __mm_s.push_str(&*literal!(" failed because there is a\n                conditional branch after a non-conditional branch in:\n")); __mm_s.push_str(&*toString(blck.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                }
            }
            if else_branch.clone().is_empty() {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert")); __mm_s.push_str(&*literal!(" failed because there ")); __mm_s.push_str(&*literal!("is no non-conditional branch in:\n")); __mm_s.push_str(&*toString(blck.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            Arc::new(OldSimCode::SimEqSystem::SES_IFEQUATION { eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::IF).clone())?, source: var_field!((*blck).source, Block::IF).clone(), elsebranch: else_branch.clone(), ifbranches: oldBranches.clone().reverse(), index: var_field!((*blck).index, Block::IF).clone() })
        },
        Deref @ WHEN { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_WHEN { eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::WHEN).clone())?, source: var_field!((*blck).source, Block::WHEN).clone(), elseWhen: Util::applyOption(var_field!((*blck).else_when, Block::WHEN).clone(), (std::sync::Arc::new(convert) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Block>) -> Result<Arc<OldSimCode::SimEqSystem>> + 'static>)), whenStmtLst: ({
        let mut __acc: Arc<metamodelica::List<OldBackendDAE::WhenOperator>> = metamodelica::nil();
        for mut stmt in (var_field!((*blck).when_stmts, Block::WHEN).clone()).into_iter().cloned() {
            let __x = BEquation::WhenStatement::convert(stmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), initialCall: var_field!((*blck).initialCall, Block::WHEN).clone(), conditions: ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut cr in (var_field!((*blck).conditions, Block::WHEN).clone()).into_iter().cloned() {
            let __x = ComponentRef::toDAE(cr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), index: var_field!((*blck).index, Block::WHEN).clone() })
        },
        Deref @ NONLINEAR { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_NONLINEAR { nlSystem: NonlinearSystem::convert(var_field!((*blck).system, Block::NONLINEAR).clone())?, alternativeTearing: None, eqAttr: BEquation::EquationAttributes::convert(BEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None))? })
        },
        Deref @ ALGORITHM { .. } => {
            Arc::new(OldSimCode::SimEqSystem::SES_ALGORITHM { index: var_field!((*blck).index, Block::ALGORITHM).clone(), statements: ConvertDAE::convertStatements(var_field!((*blck).stmts, Block::ALGORITHM).clone())?, eqAttr: BEquation::EquationAttributes::convert(var_field!((*blck).attr, Block::ALGORITHM).clone())? })
        },
        Deref @ ALIAS { .. } if (var_field!((*blck).aliasOf, Block::ALIAS).clone() > 0) => {
            Arc::new(OldSimCode::SimEqSystem::SES_ALIAS { index: var_field!((*blck).index, Block::ALIAS).clone(), aliasOf: var_field!((*blck).aliasOf, Block::ALIAS).clone() })
        },
        Deref @ ALIAS { .. } if (var_field!((*blck).aliasOf, Block::ALIAS).clone() == -1) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert")); __mm_s.push_str(&*literal!(" failed for following alias block because the index has not been updated:\n")); __mm_s.push_str(&*toString(blck.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*toString(blck.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        Ok(oldBlck)
    }

    pub fn convertList(mut blck_lst: Arc<metamodelica::List<Arc<Block>>>) -> Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> {
        let mut oldBlck_lst: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = ({
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck in (blck_lst.clone()).into_iter().cloned() {
            let __x = convert(blck.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        oldBlck_lst
    }

    pub fn convertListList(mut blck_lst_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Block>>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>>>> {
        let mut oldBlck_lst_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>>>> = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>>>> = metamodelica::nil();
        for mut blck_lst in (blck_lst_lst.clone()).into_iter().cloned() {
            let __x = convertList(blck_lst.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        oldBlck_lst_lst
    }

    pub fn fixIndices(mut blcks: Arc<metamodelica::List<Arc<Block>>>, mut acc: Arc<metamodelica::List<Arc<Block>>>, mut indices: SimCodeIndices) -> Result<(Arc<metamodelica::List<Arc<Block>>>, SimCodeIndices)> {
        let mut acc: Arc<metamodelica::List<Arc<Block>>> = acc;
        let mut indices: SimCodeIndices = indices;
        (acc, indices) = (::match_deref::match_deref! { match &(blcks.clone()) {
        Deref @ metamodelica::List::Cons { head: blck, tail: rest } => {
            let mut blck = (*blck).clone();
            (blck, indices) = fixIndex(blck.clone(), indices.clone())?;
            fixIndices(rest.clone(), cons(blck.clone(), acc.clone()), indices.clone())?
        },
        _ => {
            (acc.clone(), indices.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((acc, indices))
    }

    pub fn fixIndex(mut blck: Arc<Block>, mut indices: SimCodeIndices) -> Result<(Arc<Block>, SimCodeIndices)> {
        let mut blck: Arc<Block> = blck;
        let mut indices: SimCodeIndices = indices;
        blck = (::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ RESIDUAL { .. } => {
            assign_variant_field!(blck => Block::RESIDUAL; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ ARRAY_RESIDUAL { .. } => {
            assign_variant_field!(blck => Block::ARRAY_RESIDUAL; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ SIMPLE_ASSIGN { .. } => {
            assign_variant_field!(blck => Block::SIMPLE_ASSIGN; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ ARRAY_ASSIGN { .. } => {
            assign_variant_field!(blck => Block::ARRAY_ASSIGN; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ RESIZABLE_ASSIGN { .. } => {
            assign_variant_field!(blck => Block::RESIZABLE_ASSIGN; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ GENERIC_ASSIGN { .. } => {
            assign_variant_field!(blck => Block::GENERIC_ASSIGN; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ ALIAS { .. } => {
            assign_variant_field!(blck => Block::ALIAS; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ ALGORITHM { .. } => {
            assign_variant_field!(blck => Block::ALGORITHM; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ INVERSE_ALGORITHM { .. } => {
            assign_variant_field!(blck => Block::INVERSE_ALGORITHM; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(blck => Block::IF; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            blck.clone()
        },
        Deref @ WHEN { .. } => {
            let mut tmp: Arc<Block>;
            assign_variant_field!(blck => Block::WHEN; index = indices.equationIndex.clone());
            indices.equationIndex = indices.equationIndex.clone() + 1;
            if isSome(var_field!((*blck).else_when, Block::WHEN).clone()) {
                (tmp, indices) = fixIndex(Util::getOption(var_field!((*blck).else_when, Block::WHEN).clone())?, indices.clone())?;
                assign_variant_field!(blck => Block::WHEN; else_when = Some(tmp.clone()));
            }
            blck.clone()
        },
        Deref @ LINEAR { .. } => {
            blck.clone()
        },
        Deref @ NONLINEAR { .. } => {
            blck.clone()
        },
        Deref @ HYBRID { .. } => {
            let mut tmp: Arc<Block>;
            let mut tmp_lst: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
            (tmp, indices) = fixIndex(var_field!((*blck).continuous, Block::HYBRID).clone(), indices.clone())?;
            (tmp_lst, indices) = fixIndices(var_field!((*blck).discreteEqs, Block::HYBRID).clone(), metamodelica::nil(), indices.clone())?;
            assign_variant_field!(blck => Block::HYBRID;
                continuous = tmp.clone(),
                discreteEqs = tmp_lst.clone()
            );
            blck.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.fixIndex")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*toString(blck.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((blck, indices))
    }

    pub fn collectEntwinedEquations(mut blck: Arc<Block>) -> Arc<metamodelica::List<Arc<Block>>> {
        let mut lst: Arc<metamodelica::List<Arc<Block>>> = metamodelica::nil();
        lst = (::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ ENTWINED_ASSIGN { .. } => var_field!((*blck).single_calls, Block::ENTWINED_ASSIGN).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    fn whenString(mut conditions: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut when_stmts: Arc<metamodelica::List<Arc<WhenStatement::WhenStatement>>>, mut else_when: Option<Arc<Block>>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut indent: ArcStr = r#str.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("when ")); __mm_s.push_str(&*List::toString(conditions.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*List::toString(when_stmts.clone(), Arc::new({ let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("\t")); ArcStr::from(__mm_s) }).clone(); move |__pe_a0| WhenStatement::toString(__pe_a0, __pe_b1.clone()) }), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        if isSome(else_when.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("else")); __mm_s.push_str(&*toString(Util::getOption(else_when.clone())?, (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone();
        } else {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("end when;\n")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    fn getGenericAssignIndex(mut blck: Arc<Block>) -> Result<i32> {
        let mut index: i32 = 0;
        index = (::match_deref::match_deref! { match &(blck.clone()) {
        Deref @ RESIZABLE_ASSIGN { .. } => var_field!((*blck).call_index, Block::RESIZABLE_ASSIGN).clone(),
        Deref @ GENERIC_ASSIGN { .. } => var_field!((*blck).call_index, Block::GENERIC_ASSIGN).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.getGenericAssignIndex")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*toString(blck.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(index)
    }

    fn getGenericEquationName(mut comp: Arc<StrongComponent::NBStrongComponent>) -> Result<Arc<ComponentRef::NFComponentRef>> {
        let mut name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        name = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::GENERIC_COMPONENT { .. } => BEquation::Equation::getEqnName(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::GENERIC_COMPONENT).clone()))?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.getGenericEquationName")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(name)
    }

}

pub mod LinearSystem {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LinearSystem {
        pub index: i32,
        pub mixed: bool,
        pub torn: bool,
        pub vars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        pub beqs: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
        pub simJac: Arc<metamodelica::List<(i32, i32, Arc<Block::Block>)>>,
        pub residual: Arc<metamodelica::List<Arc<Block::Block>>>,
        pub jacobian: Option<Arc<SimJacobian::SimJacobian>>,
        pub sources: Arc<metamodelica::List<Arc<DAE::ElementSource>>>,
        pub indexSystem: i32,
        /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
        pub size: i32,
        /// if TRUE then this system is part of a jacobian matrix
        pub partOfJac: bool,
    }

    impl Default for LinearSystem {
        fn default() -> Self {
            Self {
                index: Default::default(),
                mixed: Default::default(),
                torn: Default::default(),
                vars: Default::default(),
                beqs: Default::default(),
                simJac: Default::default(),
                residual: Default::default(),
                jacobian: Default::default(),
                sources: Default::default(),
                indexSystem: Default::default(),
                size: Default::default(),
                partOfJac: Default::default(),
            }
        }
    }

    pub type LINEAR_SYSTEM = LinearSystem;

    pub fn toString(mut system: Arc<LinearSystem>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Linear System (size = ")); __mm_s.push_str(&*intString(system.size.clone())); __mm_s.push_str(&*literal!(", jacobian = ")); __mm_s.push_str(&*boolString(system.partOfJac.clone())); __mm_s.push_str(&*literal!(", mixed = ")); __mm_s.push_str(&*boolString(system.mixed.clone())); __mm_s.push_str(&*literal!(", torn = ")); __mm_s.push_str(&*boolString(system.torn.clone())); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*Block::listToString(system.residual.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("--")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

}

pub mod NonlinearSystem {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct NonlinearSystem {
        pub index: i32,
        /// equations
        pub blcks: Arc<metamodelica::List<Arc<Block::Block>>>,
        /// iteration variables
        pub crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
        pub indexSystem: i32,
        /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
        pub size: i32,
        pub jacobian: Pointer::Pointer<Option<Arc<SimJacobian::SimJacobian>>>,
        pub homotopy: bool,
        pub mixed: bool,
        pub torn: bool,
    }

    impl Default for NonlinearSystem {
        fn default() -> Self {
            Self {
                index: Default::default(),
                blcks: Default::default(),
                crefs: Default::default(),
                indexSystem: Default::default(),
                size: Default::default(),
                jacobian: Default::default(),
                homotopy: Default::default(),
                mixed: Default::default(),
                torn: Default::default(),
            }
        }
    }

    pub type NONLINEAR_SYSTEM = NonlinearSystem;

    pub fn getJacobian(mut syst: Arc<NonlinearSystem>) -> Option<Arc<SimJacobian::SimJacobian>> {
        let mut jacobian: Option<Arc<SimJacobian::SimJacobian>> = Pointer::access(syst.jacobian.clone());
        jacobian
    }

    pub fn setJacobian(mut syst: Arc<NonlinearSystem>, mut jacobian: Option<Arc<SimJacobian::SimJacobian>>) -> Arc<NonlinearSystem> {
        let mut syst: Arc<NonlinearSystem> = syst;
        Pointer::update(syst.jacobian.clone(), jacobian.clone());
        syst
    }

    pub fn toString(mut system: Arc<NonlinearSystem>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Nonlinear System (size = ")); __mm_s.push_str(&*intString(system.size.clone())); __mm_s.push_str(&*literal!(", homotopy = ")); __mm_s.push_str(&*boolString(system.homotopy.clone())); __mm_s.push_str(&*literal!(", mixed = ")); __mm_s.push_str(&*boolString(system.mixed.clone())); __mm_s.push_str(&*literal!(", torn = ")); __mm_s.push_str(&*boolString(system.torn.clone())); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("--")); __mm_s.push_str(&*List::toString(system.crefs.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("Iteration Vars:")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 10)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Block::listToString(system.blcks.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("--")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn convert(mut system: Arc<NonlinearSystem>) -> Result<Arc<OldSimCode::NonlinearSystem>> {
        let mut oldSystem: Arc<OldSimCode::NonlinearSystem> = Arc::new(<OldSimCode::NonlinearSystem as ::std::default::Default>::default());
        let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut cref in &*system.crefs.clone() {
            let mut cref = cref.clone();
            crefs = cons(ComponentRef::toDAE(cref.clone())?, crefs.clone());
        }
        oldSystem = Arc::new(OldSimCode::NonlinearSystem { clockIndex: None, tornSystem: system.torn.clone(), mixedSystem: system.mixed.clone(), homotopySupport: system.homotopy.clone(), jacobianMatrix: Util::applyOption(Pointer::access(system.jacobian.clone()), (std::sync::Arc::new(SimJacobian::convert) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimJacobian::SimJacobian>) -> Result<Arc<OldSimCode::JacobianMatrix>> + 'static>)), nUnknowns: system.size.clone(), indexNonLinearSystem: system.indexSystem.clone(), crefs: crefs.clone().reverse(), eqs: Block::convertList(system.blcks.clone()), index: system.index.clone() });
        Ok(oldSystem)
    }

}

