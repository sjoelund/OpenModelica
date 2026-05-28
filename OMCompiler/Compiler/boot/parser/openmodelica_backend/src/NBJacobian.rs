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

use crate::NBAdjacency as Adjacency;
use crate::NBAdjacency::Mapping;
use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBDifferentiate::DifferentiationType;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBEquation;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBSolve;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBVariable;
use crate::NBackendDAE as Jacobian;
use crate::SymbolicJacobian;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast_collections::AvlSetPath;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFOperator::MathClassification;
use openmodelica_nf_frontend::NFOperator::SizeClassification;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// Old Backend Import (remove once coloring ins ported)
// Util imports
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum JacobianType {
    ODE = 1,
    DAE = 2,
    LS = 3,
    NLS = 4,
    OPT_LFG = 5,
    OPT_MRF = 6,
    OPT_R0 = 7,
}
impl PartialOrd for JacobianType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for JacobianType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn isDynamic(mut jacType: JacobianType) -> bool {
    let mut b: bool = false;
    b = (match jacType.clone() {
        JacobianType::ODE => true,
        JacobianType::DAE { .. } => true,
        JacobianType::OPT_LFG => true,
        JacobianType::OPT_MRF => true,
        JacobianType::OPT_R0 => true,
        _ => false,
    });
    b
}

pub fn applyToPartitions(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut knowns: Arc<VariablePointers::VariablePointers>, mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>) -> Result<(Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>)> {
    let mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = partitions;
    let mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>> = funcMap;
    partitions = {
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut part in (partitions.clone()).into_iter().cloned() {
            let __x = partJacobian(part.clone(), funcMap.clone(), knowns.clone(), (name.clone()).clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((partitions, funcMap))
}

pub fn nonlinear(mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut comps: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>, mut full: Option<Arc<Adjacency::Matrix::Matrix>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut name: ArcStr, mut init: bool) -> Option<Arc<Jacobian::NBackendDAE>> {
    let mut jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let func: Module::jacobianInterface = if (Flags::isSet(Flags::NLS_ANALYTIC_JACOBIAN.clone()).unwrap()) {jacobianSymbolic.clone()} else {jacobianNumeric.clone()};
    jacobian = func(init=init.clone(), funcMap=funcMap.clone(), full=full.clone(), strongComponents=Some(comps.clone()), equations=equations.clone(), partialCandidates=partialCandidates.clone(), seedCandidates=seedCandidates.clone(), jacType=JacobianType::NLS.clone(), name=(name.clone()).clone()).unwrap();
    jacobian
}

pub fn combine(mut jacobians: Arc<metamodelica::List<Arc<Jacobian::NBackendDAE>>>, mut name: ArcStr) -> Result<Arc<Jacobian::NBackendDAE>> {
    let mut jacobian: Arc<Jacobian::NBackendDAE>;
    let mut jacType: JacobianType = JacobianType::ODE;
    let mut variables: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut unknowns: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut auxiliaryVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut aliasVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut diffVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut dependencies: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut resultVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut tmpVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut seedVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut col_wise_pattern: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    let mut row_wise_pattern: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    let mut seed_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut partial_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut nnz: i32 = 0;
    let mut varData: Arc<VarData::VarData> = Arc::new(VarData::VAR_DATA_EMPTY);
    let mut eqData: Arc<EqData::EqData> = Arc::new(EqData::EQ_DATA_EMPTY);
    let mut sparsityPattern: Arc<SparsityPattern::SparsityPattern> = Arc::new(<SparsityPattern::SparsityPattern as ::std::default::Default>::default());
    let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring> = SparsityColoring::lazy(EMPTY_SPARSITY_PATTERN().clone());
    if List::hasOneElement(jacobians.clone()) {
        jacobian = listHead(jacobians.clone())?;
        jacobian = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ Jacobian::JACOBIAN { .. } => {
            assign_variant_field!(jacobian => Jacobian::NBackendDAE::JACOBIAN; name = name.clone());
            jacobian.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    } else {
        for mut jac in &*jacobians.clone() {
            let mut jac = jac.clone();
            let () = (::match_deref::match_deref! { match &(jac.clone()) {
        Deref @ Jacobian::JACOBIAN { sparsityPattern: tmpPattern, varData: tmpVarData @ Deref @ NBVariable::VarData::VAR_DATA_JAC { .. }, .. } => {
            jacType = var_field!((*jac).jacType, Jacobian::NBackendDAE::JACOBIAN).clone();
            variables = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).variables, VarData::VarData::VAR_DATA_JAC).clone())?, variables.clone());
            unknowns = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).unknowns, VarData::VarData::VAR_DATA_JAC).clone())?, unknowns.clone());
            auxiliaryVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).auxiliaries, VarData::VarData::VAR_DATA_JAC).clone())?, auxiliaryVars.clone());
            aliasVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).aliasVars, VarData::VarData::VAR_DATA_JAC).clone())?, aliasVars.clone());
            diffVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).diffVars, VarData::VarData::VAR_DATA_JAC).clone())?, diffVars.clone());
            dependencies = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).dependencies, VarData::VarData::VAR_DATA_JAC).clone())?, dependencies.clone());
            resultVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).resultVars, VarData::VarData::VAR_DATA_JAC).clone())?, resultVars.clone());
            tmpVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).tmpVars, VarData::VarData::VAR_DATA_JAC).clone())?, tmpVars.clone());
            seedVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).seedVars, VarData::VarData::VAR_DATA_JAC).clone())?, seedVars.clone());
            comps = listAppend(Arc::new(var_field!((*jac).comps, Jacobian::NBackendDAE::JACOBIAN).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), comps.clone());
            col_wise_pattern = listAppend(tmpPattern.col_wise_pattern.clone(), col_wise_pattern.clone());
            row_wise_pattern = listAppend(tmpPattern.row_wise_pattern.clone(), row_wise_pattern.clone());
            seed_vars = listAppend(tmpPattern.seed_vars.clone(), seed_vars.clone());
            partial_vars = listAppend(tmpPattern.partial_vars.clone(), partial_vars.clone());
            nnz = nnz.clone() + tmpPattern.nnz.clone();
            sparsityColoring = SparsityColoring::combine(sparsityColoring.clone(), var_field!((*jac).sparsityColoring, Jacobian::NBackendDAE::JACOBIAN).clone());
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.combine")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*toString(jac.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        varData = Arc::new(VarData::VarData::VAR_DATA_JAC { seedVars: NBVariable::VariablePointers::fromList(seedVars.clone(), false), tmpVars: NBVariable::VariablePointers::fromList(tmpVars.clone(), false), resultVars: NBVariable::VariablePointers::fromList(resultVars.clone(), false), dependencies: NBVariable::VariablePointers::fromList(dependencies.clone(), false), diffVars: NBVariable::VariablePointers::fromList(diffVars.clone(), false), aliasVars: NBVariable::VariablePointers::fromList(aliasVars.clone(), false), auxiliaries: NBVariable::VariablePointers::fromList(auxiliaryVars.clone(), false), unknowns: NBVariable::VariablePointers::fromList(unknowns.clone(), false), variables: NBVariable::VariablePointers::fromList(variables.clone(), false) });
        sparsityPattern = Arc::new(SparsityPattern::SparsityPattern { nnz: nnz.clone(), partial_vars: partial_vars.clone(), seed_vars: seed_vars.clone(), row_wise_pattern: row_wise_pattern.clone(), col_wise_pattern: col_wise_pattern.clone() });
        jacobian = Arc::new(Jacobian::NBackendDAE::JACOBIAN { isAdjoint: name.clone() == literal!("ADJ"), sparsityColoring: sparsityColoring.clone(), sparsityPattern: sparsityPattern.clone(), comps: metamodelica::arrayFromVec(comps.clone().into_iter().cloned().collect()), varData: varData.clone(), jacType: jacType.clone(), name: (name.clone()).clone() });
    }
    Ok(jacobian)
}

pub fn getModule() -> Result<Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>> {
    let mut func: Module::jacobianInterface;
    func = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?) {
        Deref @ "symbolic" => jacobianSymbolic.clone(),
        Deref @ "symbolicadjoint" => jacobianSymbolicAdjoint.clone(),
        Deref @ "numeric" => jacobianNumeric.clone(),
        Deref @ "none" => jacobianNone.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(func)
}

pub fn toString(mut jacobian: Arc<Jacobian::NBackendDAE>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = (toString(jacobian.clone(), (r#str.clone()).clone())?).clone();
    Ok(r#str)
}

pub fn jacobianTypeString(mut jacType: JacobianType) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match jacType.clone() {
        JacobianType::ODE => literal!("[ODE]"),
        JacobianType::DAE { .. } => literal!("[DAE]"),
        JacobianType::LS => literal!("[LS-]"),
        JacobianType::NLS => literal!("[NLS]"),
        JacobianType::OPT_LFG => literal!("[OPT-LFG]"),
        JacobianType::OPT_MRF => literal!("[OPT-MRF]"),
        JacobianType::OPT_R0 => literal!("[OPT-R0]"),
        _ => literal!("[ERR]"),
    })).clone();
    r#str
}

// necessary as wrapping value type for UnorderedMap
pub type CrefLst = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

/// partial_vars, {seed_vars}
pub type SparsityPatternCol = (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>);

/// seed_vars, {partial_vars}
pub type SparsityPatternRow = (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>);

pub mod SparsityPattern {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SparsityPattern {
        /// colum-wise sparsity pattern
        pub col_wise_pattern: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>,
        /// row-wise sparsity pattern
        pub row_wise_pattern: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>,
        /// independent variables solved here ($SEED)
        pub seed_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
        /// LHS variables of the jacobian ($pDER)
        pub partial_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
        /// number of nonzero elements
        pub nnz: i32,
    }

    impl Default for SparsityPattern {
        fn default() -> Self {
            Self {
                col_wise_pattern: Default::default(),
                row_wise_pattern: Default::default(),
                seed_vars: Default::default(),
                partial_vars: Default::default(),
                nnz: Default::default(),
            }
        }
    }

    pub type SPARSITY_PATTERN = SparsityPattern;

    pub fn toString(mut pattern: Arc<SparsityPattern>) -> Result<ArcStr> {
        let mut r#str: ArcStr = StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Sparsity Pattern (nnz: ")); __mm_s.push_str(&*intString(pattern.nnz.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone());
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut colEmpty: bool = pattern.col_wise_pattern.clone().is_empty();
        let mut rowEmpty: bool = pattern.row_wise_pattern.clone().is_empty();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("### Seeds (col vars) ###")).clone())); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(pattern.seed_vars.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("### Partials (row vars) ###")).clone())); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*List::toString(pattern.partial_vars.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        if !(colEmpty.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("### Columns ###")).clone())); ArcStr::from(__mm_s) }).clone();
            for mut col in &*pattern.col_wise_pattern.clone() {
                let mut col = col.clone();
                (cref, dependencies) = col.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(")\t affects:\t")); __mm_s.push_str(&*ComponentRef::listToString(dependencies.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
        }
        if !(rowEmpty.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("##### Rows #####")).clone())); ArcStr::from(__mm_s) }).clone();
            for mut row in &*pattern.row_wise_pattern.clone() {
                let mut row = row.clone();
                (cref, dependencies) = row.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(")\t depends on:\t")); __mm_s.push_str(&*ComponentRef::listToString(dependencies.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
        }
        Ok(r#str)
    }

    pub fn lazy(mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, mut jacType: JacobianType) -> Result<(Arc<SparsityPattern>, Arc<SparsityColoring::SparsityColoring>)> {
        let mut sparsityPattern: Arc<SparsityPattern> = Arc::new(<SparsityPattern as ::std::default::Default>::default());
        let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring> = Arc::new(<SparsityColoring::SparsityColoring as ::std::default::Default>::default());
        let mut seed_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut partial_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut cols: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        let mut rows: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        let mut nnz: i32 = 0;
        seed_vars = NBVariable::VariablePointers::getScalarVarNames(seedCandidates.clone(), false)?;
        partial_vars = NBVariable::VariablePointers::getScalarVarNames(partialCandidates.clone(), false)?;
        cols = {
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        for mut s in (seed_vars.clone()).into_iter().cloned() {
            let __x = (s.clone(), partial_vars.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        rows = {
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        for mut p in (partial_vars.clone()).into_iter().cloned() {
            let __x = (p.clone(), seed_vars.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        nnz = (partial_vars.clone().len() as i32) * (seed_vars.clone().len() as i32);
        sparsityPattern = Arc::new(SparsityPattern { col_wise_pattern: cols.clone(), row_wise_pattern: rows.clone(), seed_vars: seed_vars.clone(), partial_vars: partial_vars.clone(), nnz: nnz.clone() });
        sparsityColoring = SparsityColoring::lazy(sparsityPattern.clone());
        Ok((sparsityPattern, sparsityColoring))
    }

    pub fn adjacencyMapToString(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<ArcStr> {
        let mut s: ArcStr = arcstr::literal!("");
        let mut keys: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut k: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut neighs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut lines: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        keys = UnorderedMap::keyList(map.clone());
        for mut k in &*keys.clone() {
            let mut k = k.clone();
            neighs = UnorderedMap::getOrFail(k.clone(), map.clone());
            lines = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*ComponentRef::toString(k.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*ComponentRef::listToString(neighs.clone())); ArcStr::from(__mm_s) }).clone(), lines.clone());
        }
        lines = lines.clone().reverse();
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Adjacency map (")); __mm_s.push_str(&*intString((keys.clone().len() as i32))); __mm_s.push_str(&*literal!(" keys):\n")); __mm_s.push_str(&*stringDelimitList(lines.clone(), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
        Ok(s)
    }

    pub fn create(mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, mut jacType: JacobianType) -> Result<(Arc<SparsityPattern>, Arc<SparsityColoring::SparsityColoring>)> {
        let mut sparsityPattern: Arc<SparsityPattern> = Arc::new(<SparsityPattern as ::std::default::Default>::default());
        let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring> = Arc::new(<SparsityColoring::SparsityColoring as ::std::default::Default>::default());
        let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>;
        (sparsityPattern, map) = ({
        let mut row_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut col_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut cols: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        let mut rows: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        let mut nnz: i32 = 0;
        (match strongComponents.clone() {
        Some(mut comps) if (comps.clone().borrow().is_empty()) => {
            (EMPTY_SPARSITY_PATTERN().clone(), UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))
        },
        Some(mut comps) => {
            let mut seed_mapping: Arc<Mapping::Mapping> = Arc::new(<Mapping::Mapping as ::std::default::Default>::default());
            let mut partial_mapping: Arc<Mapping::Mapping> = Arc::new(<Mapping::Mapping as ::std::default::Default>::default());
            let mut seed_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut seed_vars_array: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut partial_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut partial_vars_array: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut tmp: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
            let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            seed_mapping = Adjacency::Mapping::create(NBEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), seedCandidates.clone())?;
            partial_mapping = Adjacency::Mapping::create(NBEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), partialCandidates.clone())?;
            partial_vars = NBVariable::VariablePointers::getScalarVarNames(partialCandidates.clone(), false)?;
            seed_vars = NBVariable::VariablePointers::getScalarVarNames(seedCandidates.clone(), false)?;
            seed_vars_array = NBVariable::VariablePointers::getVarNames(seedCandidates.clone())?;
            partial_vars_array = NBVariable::VariablePointers::getVarNames(partialCandidates.clone())?;
            map = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime((seed_vars.clone().len() as i32) + (partial_vars.clone().len() as i32)));
            set = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime((seed_vars_array.clone().len() as i32)));
            for mut cref in &*seed_vars.clone() {
                let mut cref = cref.clone();
                UnorderedMap::add(cref.clone(), metamodelica::nil(), map.clone())?;
            }
            for mut cref in &*partial_vars.clone() {
                let mut cref = cref.clone();
                UnorderedMap::add(cref.clone(), metamodelica::nil(), map.clone())?;
            }
            for mut cref in &*seed_vars_array.clone() {
                let mut cref = cref.clone();
                UnorderedSet::add(cref.clone(), set.clone())?;
            }
            for mut cref in &*partial_vars_array.clone() {
                let mut cref = cref.clone();
                UnorderedSet::add(cref.clone(), set.clone())?;
            }
            let __range0 = 1..=(comps.clone().borrow().len() as i32);
            for mut i in __range0 {
                if !(StrongComponent::isDiscrete(comps.borrow()[(i.clone()-1) as usize].clone())?) {
                    StrongComponent::collectCrefs(comps.borrow()[(i.clone()-1) as usize].clone(), seedCandidates.clone(), partialCandidates.clone(), seed_mapping.clone(), partial_mapping.clone(), map.clone(), set.clone(), jacType.clone())?;
                }
            }
            for mut cref in &*partial_vars.clone().reverse() {
                let mut cref = cref.clone();
                if jacType.clone() == JacobianType::NLS.clone() || isRowInJacobian(cref.clone(), jacType.clone()) {
                    if UnorderedMap::contains(cref.clone(), map.clone()) {
                        tmp = UnorderedSet::unique_list(UnorderedMap::getOrFail(cref.clone(), map.clone()), (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>));
                        rows = cons((cref.clone(), tmp.clone()), rows.clone());
                        row_vars = cons(cref.clone(), row_vars.clone());
                        for mut dep in &*tmp.clone() {
                            let mut dep = dep.clone();
                            UnorderedMap::add(dep.clone(), cons(cref.clone(), UnorderedMap::getSafe(dep.clone(), map.clone(), metamodelica::sourceInfo!())?), map.clone())?;
                        }
                    }
                }
            }
            for mut cref in &*seed_vars.clone().reverse() {
                let mut cref = cref.clone();
                if jacType.clone() == JacobianType::NLS.clone() || BVariable::checkCref(cref.clone(), (std::sync::Arc::new(BVariable.isState) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!())? || (jacType.clone() == JacobianType::OPT_LFG.clone() || jacType.clone() == JacobianType::OPT_MRF.clone() || jacType.clone() == JacobianType::OPT_R0.clone()) {
                    tmp = UnorderedSet::unique_list(UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!())?, (std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>));
                    cols = cons((cref.clone(), tmp.clone()), cols.clone());
                    col_vars = cons(cref.clone(), col_vars.clone());
                }
            }
            for mut col in &*cols.clone() {
                let mut col = col.clone();
                (_, tmp) = col.clone();
                nnz = nnz.clone() + (tmp.clone().len() as i32);
            }
            (Arc::new(SparsityPattern { col_wise_pattern: cols.clone(), row_wise_pattern: rows.clone(), seed_vars: col_vars.clone(), partial_vars: row_vars.clone(), nnz: nnz.clone() }), map.clone())
        },
        None => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.SparsityPattern.create")); __mm_s.push_str(&*literal!(" failed because of missing strong components.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.SparsityPattern.create")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    })
    });
        sparsityColoring = SparsityColoring::PartialD2ColoringAlgC(sparsityPattern.clone(), jacType.clone())?;
        if Flags::isSet(Flags::DUMP_SPARSE.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(sparsityPattern.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SparsityColoring::toString(sparsityColoring.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok((sparsityPattern, sparsityColoring))
    }

    pub fn createEmpty() -> (Arc<SparsityPattern>, Arc<SparsityColoring::SparsityColoring>) {
        let mut sparsityPattern: Arc<SparsityPattern> = EMPTY_SPARSITY_PATTERN().clone();
        let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring> = EMPTY_SPARSITY_COLORING().clone();
        (sparsityPattern, sparsityColoring)
    }

}

thread_local! { static __EMPTY_SPARSITY_PATTERN_TLS: Arc<SparsityPattern::SparsityPattern> = Arc::new(SparsityPattern::SparsityPattern { col_wise_pattern: metamodelica::nil(), row_wise_pattern: metamodelica::nil(), seed_vars: metamodelica::nil(), partial_vars: metamodelica::nil(), nnz: 0 }); }
pub fn EMPTY_SPARSITY_PATTERN() -> Arc<SparsityPattern::SparsityPattern> { __EMPTY_SPARSITY_PATTERN_TLS.with(|__t| __t.clone()) }

thread_local! { static __EMPTY_SPARSITY_COLORING_TLS: Arc<SparsityColoring::SparsityColoring> = Arc::new(SparsityColoring::SparsityColoring { cols: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), rows: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()) }); }
pub fn EMPTY_SPARSITY_COLORING() -> Arc<SparsityColoring::SparsityColoring> { __EMPTY_SPARSITY_COLORING_TLS.with(|__t| __t.clone()) }

/// seed variable lists belonging to the same color
pub type SparsityColoringCol = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

/// partial variable lists for each color (multiples allowed!)
pub type SparsityColoringRow = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

pub mod SparsityColoring {
    use super::*;
    /// column wise coloring with extra row sparsity information
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SparsityColoring {
        pub cols: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>,
        pub rows: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>,
    }

    impl Default for SparsityColoring {
        fn default() -> Self {
            Self {
                cols: Default::default(),
                rows: Default::default(),
            }
        }
    }

    pub type SPARSITY_COLORING = SparsityColoring;

    pub fn toString(mut sparsityColoring: Arc<SparsityColoring>) -> ArcStr {
        let mut r#str: ArcStr = StringUtil::headline_2((literal!("Sparsity Coloring")).clone());
        let mut empty: bool = (sparsityColoring.cols.clone().borrow().len() as i32) == 0;
        if empty.clone() {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n<empty sparsity pattern>\n")); ArcStr::from(__mm_s) }).clone();
        }
        let __range0 = 1..=(sparsityColoring.cols.clone().borrow().len() as i32);
        for mut i in __range0 {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("Column Color (")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*literal!("  - Column: ")); __mm_s.push_str(&*ComponentRef::listToString(sparsityColoring.cols.borrow()[(i.clone()-1) as usize].clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        let __range1 = 1..=(sparsityColoring.rows.clone().borrow().len() as i32);
        for mut i in __range1 {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("Row Color (")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*literal!("  - Row:    ")); __mm_s.push_str(&*ComponentRef::listToString(sparsityColoring.rows.borrow()[(i.clone()-1) as usize].clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        r#str
    }

    pub fn lazy(mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>) -> Arc<SparsityColoring> {
        let mut sparsityColoring: Arc<SparsityColoring> = Arc::new(<SparsityColoring as ::std::default::Default>::default());
        let mut cols: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut rows: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        cols = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut cref in (sparsityPattern.seed_vars.clone()).into_iter().cloned() {
            let __x = list![cref.clone()];
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
        rows = arrayCreate((cols.clone().borrow().len() as i32), sparsityPattern.partial_vars.clone());
        sparsityColoring = Arc::new(SparsityColoring { cols: cols.clone(), rows: rows.clone() });
        sparsityColoring
    }

    pub fn PartialD2ColoringAlgC(mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>, mut jacType: JacobianType) -> Result<Arc<SparsityColoring>> {
        fn getIndices(mut cref: Arc<ComponentRef::NFComponentRef>, mut seed_indices: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut partial_indices: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut rows: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
            let mut indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
            if UnorderedMap::contains(cref.clone(), seed_indices.clone()) {
                indices = list![UnorderedMap::getSafe(cref.clone(), seed_indices.clone(), metamodelica::sourceInfo!())?];
            } else if UnorderedMap::contains(cref.clone(), partial_indices.clone()) {
                indices = rows.borrow()[(UnorderedMap::getSafe(cref.clone(), partial_indices.clone(), metamodelica::sourceInfo!())?-1) as usize].clone();
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.SparsityColoring.PartialD2ColoringAlgC.getIndices")); __mm_s.push_str(&*literal!(" failed because cref ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(" is neither a seed nor a partial candidate!")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            Ok(indices)
        }

        let mut sparsityColoring: Arc<SparsityColoring> = Arc::new(<SparsityColoring as ::std::default::Default>::default());
        let mut seeds: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut partials: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut seed_indices: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        let mut partial_indices: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        let mut sizeCols: i32 = 0;
        let mut sizeRows: i32 = 0;
        let mut idx_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut deps: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut cols: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut rows: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut colored_cols: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut colored_rows: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut cref_colored_cols: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut cref_colored_rows: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        seeds = metamodelica::arrayFromVec(sparsityPattern.seed_vars.clone().into_iter().cloned().collect());
        if jacType.clone() == JacobianType::NLS.clone() {
            partials = metamodelica::arrayFromVec(sparsityPattern.partial_vars.clone().into_iter().cloned().collect());
        } else {
            partials = metamodelica::arrayFromVec({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut cref in (sparsityPattern.partial_vars.clone()).into_iter().cloned() {
            if !(isRowInJacobian(cref.clone(), jacType.clone())) { continue; }
            let __x = cref.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }.into_iter().cloned().collect());
        }
        sizeCols = (seeds.clone().borrow().len() as i32);
        sizeRows = (partials.clone().borrow().len() as i32);
        seed_indices = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime(sizeCols.clone()));
        partial_indices = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime(sizeRows.clone()));
        for mut i in 1..=sizeCols.clone() {
            UnorderedMap::add(seeds.borrow()[(i.clone()-1) as usize].clone(), i.clone(), seed_indices.clone())?;
        }
        for mut i in 1..=sizeRows.clone() {
            UnorderedMap::add(partials.borrow()[(i.clone()-1) as usize].clone(), i.clone(), partial_indices.clone())?;
        }
        cols = arrayCreate(sizeCols.clone(), metamodelica::nil());
        rows = arrayCreate(sizeRows.clone(), metamodelica::nil());
        for mut tpl in &*sparsityPattern.col_wise_pattern.clone() {
            let mut tpl = tpl.clone();
            (idx_cref, deps) = tpl.clone();
            {
                let __cell0 = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dep in (deps.clone()).into_iter().cloned() {
            let __x = UnorderedMap::getSafe(dep.clone(), partial_indices.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                cols.clone().borrow_mut()[(UnorderedMap::getSafe(idx_cref.clone(), seed_indices.clone(), metamodelica::sourceInfo!())?-1) as usize] = __cell0;
            }
        }
        for mut tpl in &*sparsityPattern.row_wise_pattern.clone() {
            let mut tpl = tpl.clone();
            (idx_cref, deps) = tpl.clone();
            {
                let __cell1 = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dep in (deps.clone()).into_iter().cloned() {
            let __x = getIndices(dep.clone(), seed_indices.clone(), partial_indices.clone(), rows.clone())?;
            __acc = __x.append(&__acc);
        }
        __acc
    };
                rows.clone().borrow_mut()[(UnorderedMap::getSafe(idx_cref.clone(), partial_indices.clone(), metamodelica::sourceInfo!())?-1) as usize] = __cell1;
            }
        }
        colored_cols = SymbolicJacobian::createColoring(rows.clone(), cols.clone(), sizeCols.clone(), sizeRows.clone())?;
        cref_colored_cols = arrayCreate((colored_cols.clone().borrow().len() as i32), metamodelica::nil());
        let __range2 = 1..=(colored_cols.clone().borrow().len() as i32);
        for mut i in __range2 {
            {
                let __cell3 = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut idx in (colored_cols.borrow()[(i.clone()-1) as usize].clone()).into_iter().cloned() {
            let __x = seeds.borrow()[(idx.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                cref_colored_cols.clone().borrow_mut()[(i.clone()-1) as usize] = __cell3;
            }
        }
        colored_rows = SymbolicJacobian::createColoring(cols.clone(), rows.clone(), sizeRows.clone(), sizeCols.clone())?;
        cref_colored_rows = arrayCreate((colored_rows.clone().borrow().len() as i32), metamodelica::nil());
        let __range4 = 1..=(colored_rows.clone().borrow().len() as i32);
        for mut i in __range4 {
            {
                let __cell5 = {
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut idx in (colored_rows.borrow()[(i.clone()-1) as usize].clone()).into_iter().cloned() {
            let __x = partials.borrow()[(idx.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                cref_colored_rows.clone().borrow_mut()[(i.clone()-1) as usize] = __cell5;
            }
        }
        sparsityColoring = Arc::new(SparsityColoring { cols: cref_colored_cols.clone(), rows: cref_colored_rows.clone() });
        Ok(sparsityColoring)
    }

    pub fn PartialD2ColoringAlgColumnAndRow(mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<Arc<SparsityColoring>> {
        let mut sparsityColoring: Arc<SparsityColoring> = Arc::new(<SparsityColoring as ::std::default::Default>::default());
        let mut seed_nodes: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut partial_nodes: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut col_groups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        let mut row_groups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        let mut nCols: i32 = 0;
        let mut nRows: i32 = 0;
        let mut pad: i32 = 0;
        let mut k: i32 = 0;
        let mut cols_arr: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut rows_arr: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        seed_nodes = metamodelica::arrayFromVec(sparsityPattern.seed_vars.clone().into_iter().cloned().collect());
        partial_nodes = metamodelica::arrayFromVec(sparsityPattern.partial_vars.clone().into_iter().cloned().collect());
        col_groups = GreedyPartialD2Color(seed_nodes.clone(), map.clone())?;
        row_groups = GreedyPartialD2Color(partial_nodes.clone(), map.clone())?;
        cols_arr = metamodelica::arrayFromVec(col_groups.clone().into_iter().cloned().collect());
        rows_arr = metamodelica::arrayFromVec(row_groups.clone().into_iter().cloned().collect());
        sparsityColoring = Arc::new(SparsityColoring { cols: cols_arr.clone(), rows: rows_arr.clone() });
        Ok(sparsityColoring)
    }

    pub fn GreedyPartialD2Color(mut nodes: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>> {
        let mut groups_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        let mut index_lookup: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        let mut coloring: metamodelica::Array<i32>;
        let mut forbidden_colors: metamodelica::Array<i32>;
        let mut color_exists: metamodelica::Array<bool>;
        let mut groups: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut i: i32 = 0;
        let mut color: i32 = 0;
        let mut n: i32 = (nodes.clone().borrow().len() as i32);
        let mut node: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut mid: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut neigh: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        index_lookup = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime(n.clone()));
        for mut i in 1..=n.clone() {
            UnorderedMap::add(nodes.borrow()[(i.clone()-1) as usize].clone(), i.clone(), index_lookup.clone())?;
        }
        coloring = arrayCreate(n.clone(), 0);
        forbidden_colors = arrayCreate(n.clone(), 0);
        color_exists = arrayCreate(n.clone(), false);
        groups = arrayCreate(n.clone(), metamodelica::nil());
        for mut i in 1..=n.clone() {
            node = nodes.borrow()[(i.clone()-1) as usize].clone();
            for mut mid in &*UnorderedMap::getSafe(node.clone(), map.clone(), metamodelica::sourceInfo!())? {
                let mut mid = mid.clone();
                for mut neigh in &*UnorderedMap::getSafe(mid.clone(), map.clone(), metamodelica::sourceInfo!())? {
                    let mut neigh = neigh.clone();
                    color = coloring.borrow()[(UnorderedMap::getSafe(neigh.clone(), index_lookup.clone(), metamodelica::sourceInfo!())?-1) as usize].clone();
                    if color.clone() > 0 {
                        {
                            let __cell0 = i.clone();
                            forbidden_colors.clone().borrow_mut()[(color.clone()-1) as usize] = __cell0;
                        }
                    }
                }
            }
            color = 1;
            while forbidden_colors.borrow()[(color.clone()-1) as usize].clone() == i.clone() {
                color = color.clone() + 1;
            }
            {
                let __cell1 = color.clone();
                coloring.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
            }
            {
                let __cell2 = true;
                color_exists.clone().borrow_mut()[(color.clone()-1) as usize] = __cell2;
            }
            {
                let __cell3 = cons(node.clone(), groups.borrow()[(color.clone()-1) as usize].clone());
                groups.clone().borrow_mut()[(color.clone()-1) as usize] = __cell3;
            }
        }
        groups_lst = metamodelica::nil();
        let __range4 = (1..=(color_exists.clone().borrow().len() as i32)).rev();
        for mut i in __range4 {
            if color_exists.borrow()[(i.clone()-1) as usize].clone() {
                groups_lst = cons(groups.borrow()[(i.clone()-1) as usize].clone(), groups_lst.clone());
            }
        }
        Ok(groups_lst)
    }

    pub fn PartialD2ColoringAlg(mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<Arc<SparsityColoring>> {
        let mut sparsityColoring: Arc<SparsityColoring> = Arc::new(<SparsityColoring as ::std::default::Default>::default());
        let mut cref_lookup: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut index_lookup: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        let mut color_exists: metamodelica::Array<bool>;
        let mut coloring: metamodelica::Array<i32>;
        let mut forbidden_colors: metamodelica::Array<i32>;
        let mut col_coloring: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut row_coloring: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut color: i32 = 0;
        let mut cols_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        let mut rows_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        cref_lookup = metamodelica::arrayFromVec(sparsityPattern.seed_vars.clone().into_iter().cloned().collect());
        index_lookup = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime((sparsityPattern.seed_vars.clone().len() as i32)));
        let __range0 = 1..=(cref_lookup.clone().borrow().len() as i32);
        for mut i in __range0 {
            UnorderedMap::add(cref_lookup.borrow()[(i.clone()-1) as usize].clone(), i.clone(), index_lookup.clone())?;
        }
        coloring = arrayCreate((cref_lookup.clone().borrow().len() as i32), 0);
        forbidden_colors = arrayCreate((cref_lookup.clone().borrow().len() as i32), 0);
        color_exists = arrayCreate((cref_lookup.clone().borrow().len() as i32), false);
        col_coloring = arrayCreate((cref_lookup.clone().borrow().len() as i32), metamodelica::nil());
        row_coloring = arrayCreate((cref_lookup.clone().borrow().len() as i32), metamodelica::nil());
        let __range1 = 1..=(cref_lookup.clone().borrow().len() as i32);
        for mut i in __range1 {
            let __range2 = &*UnorderedMap::getSafe(cref_lookup.borrow()[(i.clone()-1) as usize].clone(), map.clone(), metamodelica::sourceInfo!())?;
            for mut row_var in __range2 {
                let mut row_var = row_var.clone();
                for mut col_var in &*UnorderedMap::getSafe(row_var.clone(), map.clone(), metamodelica::sourceInfo!())? {
                    let mut col_var = col_var.clone();
                    color = coloring.borrow()[(UnorderedMap::getSafe(col_var.clone(), index_lookup.clone(), metamodelica::sourceInfo!())?-1) as usize].clone();
                    if color.clone() > 0 {
                        {
                            let __cell3 = i.clone();
                            forbidden_colors.clone().borrow_mut()[(color.clone()-1) as usize] = __cell3;
                        }
                    }
                }
            }
            color = 1;
            while forbidden_colors.borrow()[(color.clone()-1) as usize].clone() == i.clone() {
                color = color.clone() + 1;
            }
            {
                let __cell4 = color.clone();
                coloring.clone().borrow_mut()[(i.clone()-1) as usize] = __cell4;
            }
            {
                let __cell5 = listAppend(row_coloring.borrow()[(color.clone()-1) as usize].clone(), UnorderedMap::getSafe(cref_lookup.borrow()[(i.clone()-1) as usize].clone(), map.clone(), metamodelica::sourceInfo!())?);
                row_coloring.clone().borrow_mut()[(color.clone()-1) as usize] = __cell5;
            }
            {
                let __cell6 = true;
                color_exists.clone().borrow_mut()[(color.clone()-1) as usize] = __cell6;
            }
        }
        let __range7 = 1..=(coloring.clone().borrow().len() as i32);
        for mut i in __range7 {
            {
                let __cell8 = cons(cref_lookup.borrow()[(i.clone()-1) as usize].clone(), col_coloring.borrow()[(coloring.borrow()[(i.clone()-1) as usize].clone()-1) as usize].clone());
                col_coloring.clone().borrow_mut()[(coloring.borrow()[(i.clone()-1) as usize].clone()-1) as usize] = __cell8;
            }
        }
        let __range9 = (1..=(color_exists.clone().borrow().len() as i32)).rev();
        for mut i in __range9 {
            if color_exists.borrow()[(i.clone()-1) as usize].clone() {
                cols_lst = cons(col_coloring.borrow()[(i.clone()-1) as usize].clone(), cols_lst.clone());
                rows_lst = cons(row_coloring.borrow()[(i.clone()-1) as usize].clone(), rows_lst.clone());
            }
        }
        sparsityColoring = Arc::new(SparsityColoring { cols: metamodelica::arrayFromVec(cols_lst.clone().into_iter().cloned().collect()), rows: metamodelica::arrayFromVec(rows_lst.clone().into_iter().cloned().collect()) });
        Ok(sparsityColoring)
    }

    pub fn combine(mut coloring1: Arc<SparsityColoring>, mut coloring2: Arc<SparsityColoring>) -> Arc<SparsityColoring> {
        let mut coloring_out: Arc<SparsityColoring> = Arc::new(<SparsityColoring as ::std::default::Default>::default());
        let mut smaller_coloring: Arc<SparsityColoring> = Arc::new(<SparsityColoring as ::std::default::Default>::default());
        (coloring_out, smaller_coloring) = if ((coloring2.cols.clone().borrow().len() as i32) > (coloring1.cols.clone().borrow().len() as i32)) {(coloring2.clone(), coloring1.clone())} else {(coloring1.clone(), coloring2.clone())};
        let __range0 = 1..=(smaller_coloring.cols.clone().borrow().len() as i32);
        for mut i in __range0 {
            {
                let __cell1 = listAppend(coloring_out.cols.borrow()[(i.clone()-1) as usize].clone(), smaller_coloring.cols.borrow()[(i.clone()-1) as usize].clone());
                coloring_out.cols.clone().borrow_mut()[(i.clone()-1) as usize] = __cell1;
            }
            {
                let __cell2 = listAppend(coloring_out.rows.borrow()[(i.clone()-1) as usize].clone(), smaller_coloring.rows.borrow()[(i.clone()-1) as usize].clone());
                coloring_out.rows.clone().borrow_mut()[(i.clone()-1) as usize] = __cell2;
            }
        }
        coloring_out
    }

}

// ToDo: all the DAEMode stuff is probably incorrect!
fn isRowInJacobian(mut cref: Arc<ComponentRef::NFComponentRef>, mut jacType: JacobianType) -> bool {
    let mut b: bool = false;
    b = BVariable::checkCref(cref.clone(), (std::sync::Arc::new(BVariable.isResidual) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!()).unwrap() || BVariable::checkCref(cref.clone(), (std::sync::Arc::new(BVariable.isStateDerivative) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!()).unwrap() && jacType.clone() != JacobianType::OPT_MRF.clone() && jacType.clone() != JacobianType::OPT_R0.clone() || jacType.clone() == JacobianType::OPT_LFG.clone() && BVariable::checkCref(cref.clone(), (std::sync::Arc::new(BVariable.isLagrangeOrPathConstraint) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!()).unwrap() || jacType.clone() == JacobianType::OPT_MRF.clone() && BVariable::checkCref(cref.clone(), (std::sync::Arc::new(BVariable.isMayerOrFinalConstraint) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!()).unwrap() || jacType.clone() == JacobianType::OPT_R0.clone() && BVariable::checkCref(cref.clone(), (std::sync::Arc::new(BVariable.isInitialConstraint) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!()).unwrap();
    b
}

// TODO: refactor with map
fn getOptimizableVars(mut variables: Arc<VariablePointers::VariablePointers>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut optimizable_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(variables.clone()).unwrap() {
        let mut var_ptr = var_ptr.clone();
        if BVariable::isOptimizable(var_ptr.clone()).unwrap() {
            optimizable_vars = cons(var_ptr.clone(), optimizable_vars.clone());
        }
    }
    optimizable_vars
}

fn getLagrangePathEquations(mut part: Arc<Partition::Partition::Partition>, mut variables: Arc<VariablePointers::VariablePointers>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut out: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(variables.clone()).unwrap() {
        let mut var_ptr = var_ptr.clone();
        if BVariable::isLagrangeOrPathConstraint(var_ptr.clone()).unwrap() {
            out = cons(var_ptr.clone(), out.clone());
        }
    }
    out
}

fn getMayerFinalEquations(mut part: Arc<Partition::Partition::Partition>, mut variables: Arc<VariablePointers::VariablePointers>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut out: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(variables.clone()).unwrap() {
        let mut var_ptr = var_ptr.clone();
        if BVariable::isMayerOrFinalConstraint(var_ptr.clone()).unwrap() {
            out = cons(var_ptr.clone(), out.clone());
        }
    }
    out
}

fn getInitialEquations(mut part: Arc<Partition::Partition::Partition>, mut variables: Arc<VariablePointers::VariablePointers>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut out: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(variables.clone()).unwrap() {
        let mut var_ptr = var_ptr.clone();
        if BVariable::isInitialConstraint(var_ptr.clone()).unwrap() {
            out = cons(var_ptr.clone(), out.clone());
        }
    }
    out
}

fn isLfgVariable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut out: bool = false;
    out = !(BVariable::isFinalTime(var_ptr.clone()).unwrap() || BVariable::isInitialTime(var_ptr.clone()).unwrap());
    out
}

fn isMrfVariable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut out: bool = false;
    out = !(BVariable::isInitialTime(var_ptr.clone()).unwrap());
    out
}

fn isR0Variable(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>) -> bool {
    let mut out: bool = false;
    out = !(BVariable::isFinalTime(var_ptr.clone()).unwrap());
    out
}

fn getSeedCandidatesDynamicOptimization(mut part: Arc<Partition::Partition::Partition>, mut all_knowns: Arc<VariablePointers::VariablePointers>, mut filter: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut unknowns: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut derivative_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut unknown_states: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    unknowns = getOptimizableVars(all_knowns.clone());
    derivative_vars = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (NBVariable::VariablePointers::toList(part.unknowns.clone())?).into_iter().cloned() {
            if !(BVariable::isStateDerivative(var.clone())?) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    unknown_states = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut var in (derivative_vars.clone()).into_iter().cloned() {
            let __x = Util::getOption((BVariable::getVarState(var.clone())?).0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    unknowns = listAppend(unknown_states.clone(), unknowns.clone());
    unknowns = List::filterOnTrue(unknowns.clone(), filter.clone());
    Ok(unknowns)
}

fn getLfgPartialCandidates(mut part: Arc<Partition::Partition::Partition>, mut all_knowns: Arc<VariablePointers::VariablePointers>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut partialCandidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    partialCandidates = NBVariable::VariablePointers::toList(part.unknowns.clone())?;
    partialCandidates = listAppend(getLagrangePathEquations(part.clone(), all_knowns.clone()), partialCandidates.clone());
    Ok(partialCandidates)
}

fn getMrfPartialCandidates(mut part: Arc<Partition::Partition::Partition>, mut all_knowns: Arc<VariablePointers::VariablePointers>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut partialCandidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    partialCandidates = NBVariable::VariablePointers::toList(part.unknowns.clone())?;
    partialCandidates = listAppend(getMayerFinalEquations(part.clone(), all_knowns.clone()), partialCandidates.clone());
    Ok(partialCandidates)
}

fn getR0PartialCandidates(mut part: Arc<Partition::Partition::Partition>, mut all_knowns: Arc<VariablePointers::VariablePointers>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut partialCandidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    partialCandidates = NBVariable::VariablePointers::toList(part.unknowns.clone())?;
    partialCandidates = listAppend(getInitialEquations(part.clone(), all_knowns.clone()), partialCandidates.clone());
    Ok(partialCandidates)
}

// before this is ever called, we should check if the variable / annotation pairs are even valid: e.g. path constraint with final time or so!
fn partJacobianDynamicOptimization(mut part: Arc<Partition::Partition::Partition>, mut all_knowns: Arc<VariablePointers::VariablePointers>, mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Option<Arc<Jacobian::NBackendDAE>>, Option<Arc<Jacobian::NBackendDAE>>, Option<Arc<Jacobian::NBackendDAE>>)> {
    let mut LFG_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut MRF_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut R0_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut kind: Partition::Kind = Partition::Partition::getKind(part.clone());
    let mut init: bool = Partition::kindIsInitial(kind.clone());
    let mut seedCandidates: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut partialCandidates: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    partialCandidates = NBVariable::VariablePointers::fromList(getLfgPartialCandidates(part.clone(), all_knowns.clone())?, part.unknowns.scalarized.clone());
    seedCandidates = NBVariable::VariablePointers::fromList(getSeedCandidatesDynamicOptimization(part.clone(), all_knowns.clone(), (std::sync::Arc::new(fnptr!(isLfgVariable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?, partialCandidates.scalarized.clone());
    LFG_jacobian = func((name.clone()).clone(), JacobianType::OPT_LFG.clone(), seedCandidates.clone(), partialCandidates.clone(), part.equations.clone(), part.strongComponents.clone(), part.adjacencyMatrix.clone(), funcMap.clone(), init.clone())?;
    partialCandidates = NBVariable::VariablePointers::fromList(getMrfPartialCandidates(part.clone(), all_knowns.clone())?, part.unknowns.scalarized.clone());
    seedCandidates = NBVariable::VariablePointers::fromList(getSeedCandidatesDynamicOptimization(part.clone(), all_knowns.clone(), (std::sync::Arc::new(fnptr!(isMrfVariable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?, partialCandidates.scalarized.clone());
    MRF_jacobian = func((name.clone()).clone(), JacobianType::OPT_MRF.clone(), seedCandidates.clone(), partialCandidates.clone(), part.equations.clone(), part.strongComponents.clone(), part.adjacencyMatrix.clone(), funcMap.clone(), init.clone())?;
    partialCandidates = NBVariable::VariablePointers::fromList(getR0PartialCandidates(part.clone(), all_knowns.clone())?, part.unknowns.scalarized.clone());
    seedCandidates = NBVariable::VariablePointers::fromList(getSeedCandidatesDynamicOptimization(part.clone(), all_knowns.clone(), (std::sync::Arc::new(fnptr!(isR0Variable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?, partialCandidates.scalarized.clone());
    R0_jacobian = func((name.clone()).clone(), JacobianType::OPT_R0.clone(), seedCandidates.clone(), partialCandidates.clone(), part.equations.clone(), part.strongComponents.clone(), part.adjacencyMatrix.clone(), funcMap.clone(), init.clone())?;
    Ok((LFG_jacobian, MRF_jacobian, R0_jacobian))
}

fn partJacobian(mut part: Arc<Partition::Partition::Partition>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut knowns: Arc<VariablePointers::VariablePointers>, mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>) -> Result<Arc<Partition::Partition::Partition>> {
    let mut part: Arc<Partition::Partition::Partition> = part;
    let mut jacType: JacobianType = JacobianType::ODE;
    let mut unknowns: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut derivative_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut state_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut seedCandidates: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut partialCandidates: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut LFG_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut MRF_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut R0_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut kind: Partition::Kind = Partition::Partition::getKind(part.clone());
    let mut updated: bool = false;
    assign_field!(part.strongComponents = (match part.strongComponents.clone() {
        Some(mut comps) => {
            let mut tmp: Arc<StrongComponent::NBStrongComponent>;
            let __range0 = 1..=(comps.clone().borrow().len() as i32);
            for mut i in __range0 {
                (tmp, updated) = compJacobian(comps.borrow()[(i.clone()-1) as usize].clone(), part.adjacencyMatrix.clone(), funcMap.clone(), kind.clone())?;
                if updated.clone() {
                    {let _arr = comps.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = tmp.clone(); _arr};
                }
            }
            Some(comps.clone())
        },
        _ => {
            part.strongComponents.clone()
        },
    }));
    if Partition::Partition::isODEorDAE(part.clone()) {
        partialCandidates = part.unknowns.clone();
        unknowns = if (Partition::Partition::getKind(part.clone()) == Partition::Kind::DAE.clone()) {Util::getOption(part.daeUnknowns.clone())?} else {part.unknowns.clone()};
        jacType = if (Partition::Partition::getKind(part.clone()) == Partition::Kind::DAE.clone()) {JacobianType::DAE.clone()} else {JacobianType::ODE.clone()};
        derivative_vars = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (NBVariable::VariablePointers::toList(unknowns.clone())?).into_iter().cloned() {
            if !(BVariable::isStateDerivative(var.clone())?) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        state_vars = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut var in (derivative_vars.clone()).into_iter().cloned() {
            let __x = Util::getOption((BVariable::getVarState(var.clone())?).0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        seedCandidates = NBVariable::VariablePointers::fromList(state_vars.clone(), partialCandidates.scalarized.clone());
        jacobian = func((name.clone()).clone(), jacType.clone(), seedCandidates.clone(), partialCandidates.clone(), part.equations.clone(), part.strongComponents.clone(), part.adjacencyMatrix.clone(), funcMap.clone(), Partition::kindIsInitial(kind.clone()))?;
        if Flags::getConfigBool(Flags::MOO_DYNAMIC_OPTIMIZATION.clone())? {
            (LFG_jacobian, MRF_jacobian, R0_jacobian) = partJacobianDynamicOptimization(part.clone(), knowns.clone(), (name.clone()).clone(), func.clone(), funcMap.clone())?;
        }
        if Util::isSome(jacobian.clone()) {
            if BackendDAE::getIsAdjoint(Util::getOption(jacobian.clone())?)? {
                assign_field!(part.association = Arc::new(Partition::Association::Association::CONTINUOUS { kind: kind.clone(), jacobian: None, jacobianAdjoint: jacobian.clone(), LFG_jacobian: LFG_jacobian.clone(), MRF_jacobian: MRF_jacobian.clone(), R0_jacobian: R0_jacobian.clone() }));
            } else {
                assign_field!(part.association = Arc::new(Partition::Association::Association::CONTINUOUS { kind: kind.clone(), jacobian: jacobian.clone(), jacobianAdjoint: None, LFG_jacobian: LFG_jacobian.clone(), MRF_jacobian: MRF_jacobian.clone(), R0_jacobian: R0_jacobian.clone() }));
            }
        } else {
            assign_field!(part.association = Arc::new(Partition::Association::Association::CONTINUOUS { kind: kind.clone(), jacobian: None, jacobianAdjoint: None, LFG_jacobian: LFG_jacobian.clone(), MRF_jacobian: MRF_jacobian.clone(), R0_jacobian: R0_jacobian.clone() }));
        }
        if Flags::isSet(Flags::JAC_DUMP.clone())? {
            println!("{}", (Partition::Partition::toString(part.clone(), 2)?).clone());
        }
    }
    Ok(part)
}

fn compJacobian(mut comp: Arc<StrongComponent::NBStrongComponent>, mut full: Option<Arc<Adjacency::Matrix::Matrix>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, bool)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut updated: bool = false;
    let mut strict: Arc<Tearing::NBTearing> = Arc::new(<Tearing::NBTearing as ::std::default::Default>::default());
    let mut residual_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut seed_candidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut residual_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut inner_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let init: bool = Partition::kindIsInitial(kind.clone());
    (comp, updated) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::ALGEBRAIC_LOOP { strict, .. } => {
            let mut strict = (*strict).clone();
            residual_comps = {
        let mut __acc: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = StrongComponent::fromSolvedEquationSlice(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            seed_candidates = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            residual_vars = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = NBEquation::Equation::getResidualVar(Slice::getT(eqn.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            inner_vars = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut comp in (strict.innerEquations.clone()).borrow().iter() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (StrongComponent::getVariables(comp.clone())?).into_iter().cloned() {
            if !(BVariable::isContinuous(var.clone(), init.clone())?) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = __x.append(&__acc);
        }
        __acc
    };
            assign_field!(strict.jac = nonlinear(NBVariable::VariablePointers::fromList(seed_candidates.clone(), false), NBVariable::VariablePointers::fromList(listAppend(residual_vars.clone(), inner_vars.clone()), false), NBEquation::EquationPointers::fromList({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), Array::appendList(strict.innerEquations.clone(), residual_comps.clone())?, full.clone(), funcMap.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Partition::Partition::kindToString(kind.clone())?); __mm_s.push_str(&*if (var_field!((*comp).linear, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone()) {literal!("_LS_JAC_")} else {literal!("_NLS_JAC_")}); __mm_s.push_str(&*intString(var_field!((*comp).idx, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone())); ArcStr::from(__mm_s) }).clone(), Partition::kindIsInitial(kind.clone())));
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            if Flags::isSet(Flags::JAC_DUMP.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (comp.clone(), true)
        },
        _ => (comp.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, updated))
}

fn sizeClassificationFromType(mut ty: Arc<Type::NFType>) -> SizeClassification {
    let mut sc: SizeClassification = SizeClassification::SCALAR;
    sc = (match Type::dimensionCount(ty.clone()) {
        0 => SizeClassification::SCALAR.clone(),
        1 => SizeClassification::ELEMENT_WISE.clone(),
        2 => SizeClassification::MATRIX.clone(),
        _ => SizeClassification::ELEMENT_WISE.clone(),
    });
    sc
}

// Helper: build addition (or single term) expression from a list of terms for a given LHS cref.
fn buildAdjointRhs(mut lhsCref: Arc<ComponentRef::NFComponentRef>, mut terms: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut vty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut sc: SizeClassification = SizeClassification::SCALAR;
    let mut addOp: Arc<Operator::NFOperator> = Arc::new(<Operator::NFOperator as ::std::default::Default>::default());
    vty = ComponentRef::getComponentType(lhsCref.clone());
    if terms.clone().is_empty() {
        rhs = Expression::makeZero(vty.clone())?;
        return Ok(rhs);
    }
    if List::hasOneElement(terms.clone()) {
        rhs = listHead(terms.clone())?;
        return Ok(rhs);
    }
    sc = sizeClassificationFromType(vty.clone());
    addOp = Operator::fromClassification((MathClassification::ADDITION.clone(), sc.clone()), vty.clone())?;
    rhs = SimplifyExp::simplify(Arc::new(Expression::NFExpression::MULTARY { arguments: terms.clone(), inv_arguments: metamodelica::nil(), operator: addOp.clone() }), false)?;
    rhs = Expression::map(rhs.clone(), (std::sync::Arc::new(Expression::repairOperator) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(rhs)
}

// Helper: run reverse-mode on a residual expression with a given seed (current_grad),
// accumulating into the provided adjoint_map. Returns updated DifferentiationArguments.
fn accumulateAdjointForResidual(mut residual: Arc<Expression::NFExpression>, mut seed: Arc<Expression::NFExpression>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut funcMapIn: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut scalarized: bool, mut adjoint_map_in: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>) -> Result<Arc<DifferentiationArguments::DifferentiationArguments>> {
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    diffArguments = Arc::new(DifferentiationArguments::DifferentiationArguments { collectAdjoints: true, current_grad: seed.clone(), adjoint_map: Some(adjoint_map_in.clone()), scalarized: scalarized.clone(), funcMap: funcMapIn.clone(), diffType: Differentiate::DifferentiationType::JACOBIAN.clone(), diff_map: Some(diff_map.clone()), new_vars: metamodelica::nil(), diffCref: Arc::new(openmodelica_nf_frontend::NFComponentRef::EMPTY) });
    (_, diffArguments) = Differentiate::differentiateExpression(residual.clone(), diffArguments.clone())?;
    Ok(diffArguments)
}

// Reusable builder for a SINGLE_COMPONENT adjoint assignment (tmp or result var).
fn makeAdjointComponent(mut lhsKey: Arc<ComponentRef::NFComponentRef>, mut adjoint_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>, mut contextName: ArcStr, mut eqIndex: i32) -> Result<Arc<StrongComponent::NBStrongComponent>> {
    let mut diffed_comp: Arc<StrongComponent::NBStrongComponent>;
    let mut terms: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut rhsExpr: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eqPtr: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut eq: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    let mut lhsVarPtr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    terms = UnorderedMap::getOrFail(lhsKey.clone(), adjoint_map.clone());
    rhsExpr = buildAdjointRhs(lhsKey.clone(), terms.clone())?;
    eqPtr = NBEquation::Equation::makeAssignment(Expression::fromCref(lhsKey.clone(), false)?, rhsExpr.clone(), Pointer::create(eqIndex.clone()), (contextName.clone()).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), NBEquation::EquationAttributes::default(NBEquation::EquationKind::CONTINUOUS.clone(), false)?)?;
    lhsVarPtr = BVariable::getVarPointer(lhsKey.clone(), metamodelica::sourceInfo!())?;
    eq = Pointer::access(eqPtr.clone());
    diffed_comp = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ NBEquation::Equation::SCALAR_EQUATION { .. } => {
            if !(ComponentRef::subscriptsAllFlat(lhsKey.clone()).is_empty()) {
                diffed_comp = Arc::new(StrongComponent::NBStrongComponent::SLICED_COMPONENT { status: NBSolve::Status::EXPLICIT.clone(), eqn: Arc::new(Slice::NBSlice { t: eqPtr.clone(), indices: metamodelica::nil() }), var: Arc::new(Slice::NBSlice { t: lhsVarPtr.clone(), indices: metamodelica::nil() }), var_cref: lhsKey.clone() });
            } else {
                diffed_comp = Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { status: NBSolve::Status::EXPLICIT.clone(), eqn: eqPtr.clone(), var: lhsVarPtr.clone() });
            }
            diffed_comp.clone()
        },
        Deref @ NBEquation::Equation::ARRAY_EQUATION { .. } => Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { status: NBSolve::Status::EXPLICIT.clone(), eqn: eqPtr.clone(), var: lhsVarPtr.clone() }),
        Deref @ NBEquation::Equation::RECORD_EQUATION { .. } => Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { status: NBSolve::Status::EXPLICIT.clone(), eqn: eqPtr.clone(), var: lhsVarPtr.clone() }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.makeAdjointComponent")); __mm_s.push_str(&*literal!(" cannot create adjoint strong component for equation ")); __mm_s.push_str(&*NBEquation::Equation::toString(eq.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(diffed_comp)
}

fn addEntryToLPAMap(mut vptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut loop_product_adjoint_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>) -> Result<()> {
    let mut mappedSeed: Option<Arc<ComponentRef::NFComponentRef>> = None;
    mappedSeed = UnorderedMap::get(BVariable::getVarName(vptr.clone())?, diff_map.clone());
    if isSome(mappedSeed.clone()) {
        UnorderedMap::tryAdd(Util::getOption(mappedSeed.clone())?, metamodelica::nil(), loop_product_adjoint_map.clone())?;
    }
    Ok(())
}

// Build a filtered diff map for a given variable list.
// For each variable pointer v in 'vars', if there exists a mapping
//   base = BVariable.getVarName(v) -> mapped in 'globalDiffMap'
// then add (base -> mapped) to the returned map.
fn populateDiffMap(mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut globalDiffMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>> {
    let mut outMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;
    let mut baseCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut o_mappedCref: Option<Arc<ComponentRef::NFComponentRef>> = None;
    outMap = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime((vars.clone().len() as i32)));
    for mut vp in &*vars.clone() {
        let mut vp = vp.clone();
        baseCref = BVariable::getVarName(vp.clone())?;
        o_mappedCref = UnorderedMap::get(baseCref.clone(), globalDiffMap.clone());
        if isSome(o_mappedCref.clone()) {
            UnorderedMap::add(baseCref.clone(), Util::getOption(o_mappedCref.clone())?, outMap.clone())?;
        }
    }
    Ok(outMap)
}

// Flattened across all components: preserve component order and in-component order
fn getAllAlgVars(mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut c in &*comps.clone() {
        let mut c = c.clone();
        for mut v in &*{
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut v in (StrongComponent::getVariables(c.clone()).unwrap()).into_iter().cloned() {
            if !(BVariable::isAlgebraic(v.clone()).unwrap()) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } {
            let mut v = v.clone();
            vars = cons(v.clone(), vars.clone());
        }
    }
    vars
}

fn getTmpFilterFunction(mut jacType: JacobianType) -> Result<Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>> {
    let mut func: NBVariable::checkVar;
    func = (match jacType.clone() {
        JacobianType::ODE => (std::sync::Arc::new(BVariable.isStateDerivative) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::DAE { .. } => (std::sync::Arc::new(BVariable.isResidual) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::LS => (std::sync::Arc::new(BVariable.isResidual) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::NLS => (std::sync::Arc::new(BVariable.isResidual) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::OPT_LFG => (std::sync::Arc::new(BVariable.isStateDerivative) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::OPT_MRF => (std::sync::Arc::new(BVariable.isStateDerivative) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::OPT_R0 => (std::sync::Arc::new(BVariable.isStateDerivative) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.getTmpFilterFunction")); __mm_s.push_str(&*literal!(" failed because jacobian type is not known: ")); __mm_s.push_str(&*jacobianTypeString(jacType.clone())); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
    Ok(func)
}

fn makeVarTraverse(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut name: ArcStr, mut vars_ptr: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut makeVar: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>, mut init: bool) -> Result<()> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>;

    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut diff: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut parent_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut diff_parent_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut diff_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut diff_parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
    if BVariable::isContinuous(var_ptr.clone(), init.clone())? {
        (diff, diff_ptr) = makeVar(var.name.clone(), (name.clone()).clone())?;
        Pointer::update(vars_ptr.clone(), cons(diff_ptr.clone(), Pointer::access(vars_ptr.clone())));
        UnorderedMap::add(var.name.clone(), diff.clone(), map.clone())?;
        let _ = (match BVariable::getParent(var_ptr.clone())? {
        Some(mut parent) => {
            parent_name = BVariable::getVarName(parent.clone())?;
            diff_parent = (::match_deref::match_deref! { match &(UnorderedMap::get(parent_name.clone(), map.clone())) {
        Some(diff_parent_name) => BVariable::getVarPointer(diff_parent_name.clone(), metamodelica::sourceInfo!())?,
        _ => {
            (diff_parent_name, _) = makeVar(parent_name.clone(), (name.clone()).clone())?;
            UnorderedMap::add(parent_name.clone(), diff_parent_name.clone(), map.clone())?;
            BVariable::getVarPointer(diff_parent_name.clone(), metamodelica::sourceInfo!())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            BVariable::addRecordChild(diff_parent.clone(), diff_ptr.clone())?;
            diff_ptr = BVariable::setParent(diff_ptr.clone(), diff_parent.clone())?;
            ()
        },
        _ => (),
    });
    }
    Ok(())
}

fn adjointMapToString(mut adjoint_map: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>>) -> Result<ArcStr> {
    fn valueToString(mut elst: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<ArcStr> {
        let mut vstr: ArcStr = arcstr::literal!("");
        vstr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (elst.clone()).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        Ok(vstr)
    }

    let mut r#str: ArcStr = arcstr::literal!("");
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>;
    if Util::isNone(adjoint_map.clone()) {
        r#str = (literal!("{}")).clone();
        return Ok(r#str);
    }
    let __pa0 = ::match_deref::match_deref! { match &(adjoint_map.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    map = __pa0.clone();
    r#str = (UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(valueToString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<ArcStr> + 'static>), (literal!("\n  ")).clone(), (literal!(" -> ")).clone())?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{\n  ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n}")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn diffMapToString(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = (UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n  ")).clone(), (literal!(" -> ")).clone())?).clone();
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{\n  ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\n}")); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

fn makeLinearAlgebraicLoop(mut itVarPtrs: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut resEqnPtrs: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut jac: Option<Arc<Jacobian::NBackendDAE>>, mut mixed: bool, mut homotopy: bool) -> Result<Arc<StrongComponent::NBStrongComponent>> {
    let mut comp: Arc<StrongComponent::NBStrongComponent>;
    let mut m1: i32 = (itVarPtrs.clone().len() as i32);
    let mut m2: i32 = (resEqnPtrs.clone().len() as i32);
    let mut itVars_s: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut res_s: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut tearingSet: Arc<Tearing::NBTearing> = Arc::new(<Tearing::NBTearing as ::std::default::Default>::default());
    if m1.clone() != m2.clone() {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("makeLinearAlgebraicLoop: |vars| != |eqns|")).clone()])?;
        bail!("fail");
    }
    itVars_s = {
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice>>> = metamodelica::nil();
        for mut vp in (itVarPtrs.clone()).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: vp.clone(), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    res_s = {
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice>>> = metamodelica::nil();
        for mut ep in (resEqnPtrs.clone()).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: ep.clone(), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    tearingSet = Arc::new(Tearing::NBTearing { jac: jac.clone(), innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), residual_eqns: res_s.clone(), iteration_vars: itVars_s.clone() });
    comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { status: NBSolve::Status::IMPLICIT.clone(), homotopy: homotopy.clone(), mixed: mixed.clone(), linear: true, casual: None, strict: tearingSet.clone(), idx: -1 });
    Ok(comp)
}

