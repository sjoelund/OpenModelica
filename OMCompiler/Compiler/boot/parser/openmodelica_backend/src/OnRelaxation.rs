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

use crate::AdjacencyMatrix;
use crate::BackendDAE;
use crate::BackendDAEEXT;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Differentiate;
use crate::DumpGraphML;
use crate::Matching;
use crate::Sorting;
use crate::SymbolicJacobian;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

/*
 * relaxation from gausian elemination
 *
 */
pub fn relaxSystem(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(relaxSystem0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    Ok(outDAE)
}

fn relaxSystem0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outChanged: bool = false;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut b: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    (osyst, outShared, b2) = relaxSystem1(isyst.clone(), inShared.clone(), comps.clone())?;
    outChanged = inChanged.clone() || b2.clone();
    Ok((osyst, outShared, outChanged))
}

fn relaxSystem1(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outRunMatching: bool = false;
    (osyst, oshared, outRunMatching) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone(), inComps.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil) => {
                    Ok((isyst.clone(), ishared.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, shared @ Deref @ BackendDAE::Shared { functionTree: funcs, .. }, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_LINEAR, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, vars: vindx, eqns: eindex, .. }, tail: comps }) => {
                    let mut eorphans: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vorphans: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut otherorphans: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut roots: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut constraints: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut constraintresidual: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut subsyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut vec2: metamodelica::Array<i32>;
                    let mut rowmarks: metamodelica::Array<i32>;
                    let mut colummarks: metamodelica::Array<i32>;
                    let mut mapIncRowEqn: metamodelica::Array<i32>;
                    let mut orowmarks: metamodelica::Array<i32>;
                    let mut ocolummarks: metamodelica::Array<i32>;
                    let mut size: i32 = 0;
                    let mut mark: i32 = 0;
                    let mut esize: i32 = 0;
                    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut tvars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut teqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut m1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mc: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mct: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut beqs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>;
                    let mut crefexps: metamodelica::Array<Arc<DAE::Exp>>;
                    let mut crefexplst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut vorphansarray1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut ass22: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut vec1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut neweqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut shared = (*shared).clone();
                    let mut jac = (*jac).clone();
                    println!("{}", (literal!("try to relax\n")).clone());
                    Util::profilerinit()?;
                    Util::profilerstart2()?;
                    Util::profilerstart1()?;
                    size = (vindx.clone().len() as i32);
                    esize = (eindex.clone().len() as i32);
                    ass1 = arrayCreate(size.clone(), -1);
                    ass2 = arrayCreate(size.clone(), -1);
                    eqn_lst = BackendEquation::getList(eindex.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()));
                    eqns = BackendEquation::listEquation(eqn_lst.clone())?;
                    var_lst = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), BackendVariable::daeVars(isyst.clone()));
                    vars = BackendVariable::listVar1(var_lst.clone());
                    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (subsyst, m, mt, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(subsyst.clone(), crate::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                    (_, ass1, ass2) = List::fold1(eqn_lst.clone(), (std::sync::Arc::new(vectorMatching) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> + 'static>), vars.clone(), (1, ass1.clone(), ass2.clone()));
                    (_, ass1, ass2) = List::fold1(eqn_lst.clone(), (std::sync::Arc::new(aliasMatching) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> + 'static>), vars.clone(), (1, ass1.clone(), ass2.clone()));
                    m1 = arrayCreate(size.clone(), metamodelica::nil());
                    transformJacToAdjacencyMatrix2(jac.clone(), m1.clone(), mapIncRowEqn.clone(), eqns.clone(), ass1.clone(), ass2.clone(), (std::sync::Arc::new(fnptr!(isConstOneMinusOne, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
                    Matching::matchingExternalsetAdjacencyMatrix(size.clone(), size.clone(), m1.clone());
                    let true = (BackendDAEEXT::setAssignment(size.clone(), size.clone(), ass2.clone(), ass1.clone())) else { bail!("pattern mismatch") };
                    BackendDAEEXT::matching(size.clone(), size.clone(), 5, -1, metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    m1 = arrayCreate(size.clone(), metamodelica::nil());
                    transformJacToAdjacencyMatrix1(jac.clone(), m1.clone(), ass1.clone(), ass2.clone(), (std::sync::Arc::new(fnptr!(isConstOneMinusOne, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
                    Matching::matchingExternalsetAdjacencyMatrix(size.clone(), size.clone(), m1.clone());
                    let true = (BackendDAEEXT::setAssignment(size.clone(), size.clone(), ass2.clone(), ass1.clone())) else { bail!("pattern mismatch") };
                    BackendDAEEXT::matching(size.clone(), size.clone(), 1, -1, metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
                    unassigned = Matching::getUnassigned(size.clone(), ass2.clone(), metamodelica::nil());
                    colummarks = arrayCreate(size.clone(), -1);
                    onefreeMatchingBFS(unassigned.clone(), m.clone(), mt.clone(), size.clone(), ass1.clone(), ass2.clone(), colummarks.clone(), 1, metamodelica::nil())?;
                    Util::profilerstop1()?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Matching  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vorphans = getOrphans(1, size.clone(), ass1.clone(), metamodelica::nil())?;
                    eorphans = getOrphans(1, size.clone(), ass2.clone(), metamodelica::nil())?;
                    ass1 = BackendDAETransform::varAssignmentNonScalar(ass1.clone(), mapIncRowEqn.clone());
                    ass22 = BackendDAETransform::eqnAssignmentNonScalar(mapEqnIncRow.clone(), ass2.clone())?;
                    eorphans = List::uniqueIntN(List::map1r(eorphans.clone(), Arc::new(arrayGet.clone()), mapIncRowEqn.clone()), (mapIncRowEqn.clone().borrow().len() as i32))?;
                    (subsyst, m, mt) = BackendDAEUtil::getAdjacencyMatrix(subsyst.clone(), crate::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                    rowmarks = arrayCreate(size.clone(), -1);
                    colummarks = arrayCreate(size.clone(), -1);
                    orowmarks = arrayCreate(size.clone(), -1);
                    ocolummarks = arrayCreate(size.clone(), -1);
                    vorphansarray1 = arrayCreate(size.clone(), metamodelica::nil());
                    mc = arrayCreate(esize.clone(), metamodelica::nil());
                    mct = arrayCreate(size.clone(), metamodelica::nil());
                    mc = Array::copy(m.clone(), mc.clone())?;
                    mct = Array::copy(mt.clone(), mct.clone())?;
                    mark = 1;
                    (mark, constraintresidual) = generateCliquesResidual(eorphans.clone(), ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vars.clone(), metamodelica::nil())?;
                    (mark, roots, constraints) = prepairOrphansOrder(vorphans.clone(), ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vorphansarray1.clone(), vars.clone(), metamodelica::nil(), metamodelica::nil())?;
                    mark = prepairOrphansOrder2(vorphans.clone(), ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vorphansarray1.clone())?;
                    Util::profilerstop1()?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Identifikation  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vorphansarray1 = arrayCreate(size.clone(), metamodelica::nil());
                    List::map2_0(roots.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
                    List::map2_0(constraints.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
                    otherorphans = List::select2(vorphans.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone());
                    mark = getOrphansOrderEdvanced(otherorphans.clone(), ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vorphansarray1.clone())?;
                    List::map2_0(otherorphans.clone(), (std::sync::Arc::new(removeRootConnections) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), vorphansarray1.clone(), roots.clone());
                    mark = getConstraintesOrphansOrderEdvanced(constraints.clone(), ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vorphansarray1.clone())?;
                    (vorphans, mark) = getOrphansOrderEdvanced3(roots.clone(), otherorphans.clone(), constraints.clone(), vorphans.clone(), vorphansarray1.clone(), mark.clone(), rowmarks.clone())?;
                    Util::profilerstop1()?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Reihenfolge  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    List::map2_0(constraints.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
                    otherorphans = List::select2(vorphans.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone());
                    List::map2_0(constraintresidual.clone(), (std::sync::Arc::new(doAssign) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), ass22.clone(), list![-1]);
                    mark = getOrphansPairs(otherorphans.clone(), ass1.clone(), ass22.clone(), m.clone(), mt.clone(), mark.clone() + 1, rowmarks.clone(), colummarks.clone())?;
                    List::map2_0(constraintresidual.clone(), (std::sync::Arc::new(doAssign) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), ass22.clone(), metamodelica::nil());
                    mark = getOrphansPairsConstraints(constraints.clone(), ass1.clone(), ass22.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqns.clone())?;
                    Util::profilerstop1()?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Paarung  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    vec1 = arrayCreate(esize.clone(), metamodelica::nil());
                    vec2 = arrayCreate(esize.clone(), -1);
                    orowmarks = List::fold1(vorphans.clone(), (std::sync::Arc::new(markOrphans) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), 1, orowmarks.clone());
                    ocolummarks = List::fold1(eorphans.clone(), (std::sync::Arc::new(markOrphans) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), 1, ocolummarks.clone());
                    mark = getIndexesForEqnsAdvanced(vorphans.clone(), 1, m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass22.clone(), vec1.clone(), vec2.clone(), arrayCreate(esize.clone(), false), vars.clone(), eqns.clone(), shared.clone(), size.clone())?;
                    (_, _, _, eqns, vars) = Array::fold(vec2.clone(), (std::sync::Arc::new(getEqnsinOrder) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)> + 'static>), (eqns.clone(), vars.clone(), ass22.clone(), BackendEquation::listEquation(metamodelica::nil())?, BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())));
                    Util::profilerstop1()?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Indizierung  time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    subsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
                    (subsyst, m, _) = BackendDAEUtil::getAdjacencyMatrix(subsyst.clone(), crate::BackendDAE::IndexType::ABSOLUTE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(ishared.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(SymbolicJacobian::calculateJacobian(vars.clone(), eqns.clone(), m.clone(), true, ishared.clone())?) {
                        (Some(__pa0), _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    jac = __pa0.clone();
                    (beqs, _) = BackendDAEUtil::getEqnSysRhs(eqns.clone(), vars.clone(), Some(funcs.clone()))?;
                    beqs = beqs.clone().reverse();
                    matrix = arrayCreate(size.clone(), metamodelica::nil());
                    transformJacToMatrix(jac.clone(), 1, 1, size.clone(), beqs.clone(), matrix.clone())?;
                    (tvars, teqns) = gaussElimination(1, size.clone(), matrix.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()), BackendEquation::listEquation(metamodelica::nil())?, (1, 1))?;
                    eqn_lst = BackendEquation::equationList(teqns.clone());
                    var_lst = BackendVariable::varList(tvars.clone())?;
                    syst = List::fold(eqn_lst.clone(), (std::sync::Arc::new(BackendEquation::equationAddDAE) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), isyst.clone());
                    syst = List::fold(var_lst.clone(), (std::sync::Arc::new(BackendVariable::addVarDAE) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), syst.clone());
                    crefexplst = List::map(BackendVariable::varList(vars.clone())?, (std::sync::Arc::new(makeCrefExps) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::Exp>> + 'static>));
                    crefexps = metamodelica::arrayFromVec(crefexplst.clone().into_iter().cloned().collect());
                    neweqns = makeGausElimination(1, size.clone(), matrix.clone(), crefexps.clone(), metamodelica::nil())?;
                    Util::profilerstop1()?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Gaus Elimination time: ")); __mm_s.push_str(&*realString(Util::profilertime1())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    syst = replaceEquationsAddNew(eindex.clone(), neweqns.clone(), syst.clone())?;
                    Util::profilerstop2()?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Gesamt  time: ")); __mm_s.push_str(&*realString(Util::profilertime2())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Util::profilerreset1();
                    Util::profilerstart1()?;
                    println!("{}", (literal!("Ok system relaxed\n")).clone());
                    (syst, shared, _) = relaxSystem1(syst.clone(), shared.clone(), comps.clone())?;
                    Ok((syst.clone(), shared.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: _, tail: comps }) => {
                    let mut b: bool = false;
                    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
                    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
                    (syst, shared, b) = relaxSystem1(isyst.clone(), ishared.clone(), comps.clone())?;
                    Ok((syst.clone(), shared.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outRunMatching))
}

fn removeRootConnections(mut orphan: i32, mut orphansarray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut roots: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (orphan.clone(), orphansarray.clone(), roots.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    lst = orphansarray.borrow()[(orphan.clone()-1) as usize].clone();
                    let true = (intGt((lst.clone().len() as i32), 1)) else { bail!("pattern mismatch") };
                    lst = List::fold1(roots.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst.clone());
                    {let _arr = orphansarray.clone(); _arr.borrow_mut()[(orphan.clone()-1) as usize] = lst.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn replaceFinalParameter(mut itpl: (Arc<DAE::Exp>, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> {
    let mut outTpl: (Arc<DAE::Exp>, BackendDAE::Variables);
    let mut e: Arc<DAE::Exp>;
    let mut knvars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut b: bool = false;
    (e, knvars) = itpl.clone();
    let (__pa0, (__pa1, __pa2)) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(traverserExpreplaceFinalParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool))> + 'static>), (knvars.clone(), false))?;
    e = __pa0.clone();
    knvars = __pa1.clone();
    b = __pa2.clone();
    (e, _) = ExpressionSimplify::condsimplify(b.clone(), e.clone())?;
    outTpl = (e.clone(), knvars.clone());
    Ok(outTpl)
}

fn traverserExpreplaceFinalParameter(mut inExp: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, bool);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), tpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (knvars, _)) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), knvars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    v = __pa0.clone();
                    let true = (BackendVariable::isFinalVar(v.clone())) else { bail!("pattern mismatch") };
                    e1 = BackendVariable::varBindExpStartValue(v.clone())?;
                    Ok((e1.clone(), (knvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outTpl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn replaceEquationsAddNew(mut inEqnIndxes: Arc<metamodelica::List<i32>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    outEqSystem = (::match_deref::match_deref! { match &((inEqnIndxes.clone(), inEqns.clone(), inEqSystem.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            BackendEquation::equationsAddDAE(inEqns.clone(), inEqSystem.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: index, tail: indices }, Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, Deref @ BackendDAE::EqSystem { orderedEqs, .. }) => {
            let mut eqSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
            eqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem.clone(), BackendEquation::setAtIndex(orderedEqs.clone(), index.clone(), eqn.clone())?);
            replaceEquationsAddNew(indices.clone(), eqns.clone(), eqSystem.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEqSystem)
}

fn dumpVar(mut id: i32, mut vars: BackendDAE::Variables) -> Result<()> {
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    v = BackendVariable::getVarAt(vars.clone(), id.clone())?;
    println!("{}", (ComponentReferenceBasics::printComponentRefStr(BackendVariable::varCref(v.clone())?)?).clone());
    println!("{}", (literal!("\n")).clone());
    Ok(())
}

fn transposeOrphanVec(mut c: i32, mut vec3: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inId: i32) -> Result<i32> {
    let mut outId: i32 = 0;
    outId = 'mc: {
        let __mc_input = (c.clone(), vec3.clone(), inId.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let true = (intGt(c.clone(), 0)) else { bail!("pattern mismatch") };
            lst = vec3.borrow()[(c.clone()-1) as usize].clone();
            {let _arr = vec3.clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = cons(inId.clone(), lst.clone()); _arr};
            Ok(inId.clone() + 1)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inId.clone() + 1)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outId)
}

fn markOrphans(mut o: i32, mut mark: i32, mut rowmark: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut orowmark: metamodelica::Array<i32>;
    orowmark = {let _arr = rowmark.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = mark.clone(); _arr};
    Ok(orowmark)
}

fn generateCliquesResidual(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut vars: BackendDAE::Variables, mut iconstraints: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut omark: i32 = 0;
    let mut oconstraints: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (omark, oconstraints) = 'mc: {
        let __mc_input = (inOrphans.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vars.clone(), iconstraints.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _) => {
                    Ok((mark.clone() + 2, iconstraints.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: rest }, _, _, _, _, _, _, _, _, _) => {
                    let mut constraints: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut partner: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut foundflow: bool = false;
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut omark: i32 = omark.clone();
                    let false = (intEq(colummarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = mark.clone(); _arr};
                    rlst = m.borrow()[(o.clone()-1) as usize].clone();
                    elst = List::select1(List::flatten(List::map1r(rlst.clone(), Arc::new(arrayGet.clone()), mt.clone())), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    partner = List::select1(elst.clone(), (std::sync::Arc::new(fnptr!(isResOrphan, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<bool> + 'static>), ass2.clone());
                    partner = List::uniqueIntN(List::removeOnTrue(o.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), partner.clone()), (colummarks.clone().borrow().len() as i32))?;
                    List::map2_0(partner.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), colummarks.clone(), mark.clone());
                    vlst = List::map1r(rlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                    blst = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isFlowVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
                    foundflow = List::any(blst.clone(), std::sync::Arc::new(fnptr!(Util::id, _)));
                    rlst = selectNonFlows(rlst.clone(), blst.clone())?;
                    foundflow = generateCliquesResidual1(rlst.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), foundflow.clone(), vars.clone())?;
                    generateCliquesResidual2(rlst.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone() + 1, rowmarks.clone(), colummarks.clone(), cons(o.clone(), partner.clone()))?;
                    constraints = if (!(foundflow.clone())) {listAppend(cons(o.clone(), partner.clone()), iconstraints.clone())} else {iconstraints.clone()};
                    (omark, constraints) = generateCliquesResidual(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vars.clone(), constraints.clone())?;
                    Ok((omark.clone(), constraints.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _) => {
                    let mut constraints: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut omark: i32 = omark.clone();
                    (omark, constraints) = generateCliquesResidual(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), vars.clone(), iconstraints.clone())?;
                    Ok((omark.clone(), constraints.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((omark, oconstraints))
}

fn generateCliquesResidual1(mut rows: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut ifoundFlow: bool, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut ofoundFlow: bool = ifoundFlow.clone();
    let mut e: i32 = 0;
    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut rlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut b1: bool = false;
    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    for mut r in &*rows.clone() {
        let mut r = r.clone();
        if !(intEq(rowmarks.borrow()[(r.clone()-1) as usize].clone(), mark.clone())) {
            next = List::select1(mt.borrow()[(r.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(isNoResOrphan, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<bool> + 'static>), ass2.clone());
            next = List::select2(next.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), colummarks.clone(), mark.clone());
            next = List::removeOnTrue(ass1.borrow()[(r.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone());
            if next.clone().is_empty() {
                {let _arr = rowmarks.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = mark.clone(); _arr};
                e = ass1.borrow()[(r.clone()-1) as usize].clone();
                {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                rlst = ass2.borrow()[(e.clone()-1) as usize].clone();
                next = List::fold1(rlst.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), m.borrow()[(e.clone()-1) as usize].clone());
                vlst = List::map1r(next.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                blst = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isFlowVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
                b1 = List::any(blst.clone(), std::sync::Arc::new(fnptr!(Util::id, _)));
                next = selectNonFlows(next.clone(), blst.clone())?;
                ofoundFlow = generateCliquesResidual1(next.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), b1.clone() || ofoundFlow.clone(), vars.clone())?;
            }
        }
    }
    Ok(ofoundFlow)
}

fn selectNonFlows(mut rows: Arc<metamodelica::List<i32>>, mut flowFlag: Arc<metamodelica::List<bool>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut brest: Arc<metamodelica::List<bool>> = flowFlag.clone();
    let mut b: bool = false;
    for mut r in &*rows.clone() {
        let mut r = r.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(brest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        b = __pa0.clone();
        brest = __pa1.clone();
        if !(b.clone()) {
            oAcc = cons(r.clone(), oAcc.clone());
        }
    }
    Ok(oAcc)
}

fn generateCliquesResidual2(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphan: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((eqns.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphan.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _) if (!(intEq(rowmarks.borrow()[(r.clone()-1) as usize].clone(), mark.clone()))) => {
            let mut e: i32 = 0;
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut lst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            e = ass1.borrow()[(r.clone()-1) as usize].clone();
            rlst = ass2.borrow()[(e.clone()-1) as usize].clone();
            lst = List::fold1(rlst.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), m.borrow()[(e.clone()-1) as usize].clone());
            let __pa0 @ metamodelica::List::Cons { head: _, tail: _ } = (List::select2(lst.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone() - 1)) else { bail!("pattern mismatch") };
            lst1 = __pa0.clone();
            List::map4_0(lst1.clone(), (std::sync::Arc::new(generateResidualClique) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, i32) -> Result<()> + 'static>), m.clone(), mt.clone(), orphan.clone(), e.clone());
            List::map2_0(rlst.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
            lst = List::select2(lst.clone(), (std::sync::Arc::new(fnptr!(marked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone() - 1);
            {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
            generateCliquesResidual2(lst.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphan.clone())?;
            generateCliquesResidual2(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphan.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _) => {
            generateCliquesResidual2(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphan.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn prepairOrphansOrder(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vars: BackendDAE::Variables, mut iroots: Arc<metamodelica::List<i32>>, mut iconstraints: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut omark: i32 = 0;
    let mut oroots: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oconstraints: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (omark, oroots, oconstraints) = (::match_deref::match_deref! { match &((inOrphans.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone(), vars.clone(), iroots.clone(), iconstraints.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _) => {
            (mark.clone(), iroots.clone(), iconstraints.clone())
        },
        (Deref @ metamodelica::List::Cons { head: o, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) if (!(intEq(rowmarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone()))) => {
            let mut roots: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut constraints: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut foundflow: bool = false;
            let mut constr: bool = false;
            let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            {let _arr = rowmarks.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = mark.clone(); _arr};
            elst = mt.borrow()[(o.clone()-1) as usize].clone();
            rlst = List::flatten(List::map1r(elst.clone(), Arc::new(arrayGet.clone()), ass2.clone()));
            vlst = List::map1r(rlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
            constr = List::all(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isFlowVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
            constraints = List::consOnTrue(constr.clone(), o.clone(), iconstraints.clone());
            foundflow = prepairOrphansOrder1(mt.borrow()[(o.clone()-1) as usize].clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), list![o.clone()], false, vars.clone())?;
            roots = List::consOnTrue(foundflow.clone() && !(constr.clone()), o.clone(), iroots.clone());
            (omark, roots, constraints) = prepairOrphansOrder(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone() + 1, rowmarks.clone(), colummarks.clone(), orphans.clone(), vars.clone(), roots.clone(), constraints.clone())?;
            (omark.clone(), roots.clone(), constraints.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut roots: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut constraints: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (omark, roots, constraints) = prepairOrphansOrder(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone(), vars.clone(), iroots.clone(), iconstraints.clone())?;
            (omark.clone(), roots.clone(), constraints.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((omark, oroots, oconstraints))
}

fn prepairOrphansOrder1(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut preorphan: i32, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut prer: Arc<metamodelica::List<i32>>, mut ifoundFlow: bool, mut vars: BackendDAE::Variables) -> Result<bool> {
    let mut ofoundFlow: bool = ifoundFlow.clone();
    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut b1: bool = false;
    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    for mut e in &*eqns.clone() {
        let mut e = e.clone();
        if !(intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) {
            next = List::select1(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(isNoOrphan, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), ass1.clone());
            next = List::select2(next.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone());
            next = List::fold1(ass2.borrow()[(e.clone()-1) as usize].clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone());
            if next.clone().is_empty() {
                {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                r = ass2.borrow()[(e.clone()-1) as usize].clone();
                List::map2_0(r.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
                elst = List::select1(List::map1r(r.clone(), Arc::new(arrayGet.clone()), ass1.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                next = List::flatten(List::map1r(r.clone(), Arc::new(arrayGet.clone()), mt.clone()));
                next = List::fold1(elst.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone());
                List::map2_0(r.clone(), (std::sync::Arc::new(addPreOrphan) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), preorphan.clone(), orphans.clone());
                vlst = List::map1r(r.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
                b1 = List::any(vlst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isFlowVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>));
                ofoundFlow = prepairOrphansOrder1(next.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), r.clone(), b1.clone() || ofoundFlow.clone(), vars.clone())?;
            }
        }
    }
    Ok(ofoundFlow)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn prepairOrphansOrder2(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut omark: i32 = 0;
    omark = 'mc: {
        let __mc_input = (inOrphans.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _) => {
                    Ok(imark.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: rest }, _, _, _, _, _, _, _, _) => {
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut partner: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(rowmarks.borrow()[(o.clone()-1) as usize].clone(), imark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = rowmarks.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = imark.clone(); _arr};
                    elst = List::select1(mt.borrow()[(o.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    rlst = List::select1(List::flatten(List::map1r(elst.clone(), Arc::new(arrayGet.clone()), m.clone())), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    partner = List::select1(rlst.clone(), (std::sync::Arc::new(fnptr!(isOrphan, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), ass1.clone());
                    partner = List::unique(partner.clone());
                    List::map2_0(partner.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), imark.clone());
                    prepairOrphansOrder3(mt.borrow()[(o.clone()-1) as usize].clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark.clone(), rowmarks.clone(), colummarks.clone(), o.clone(), partner.clone(), orphans.clone(), list![o.clone()])?;
                    Ok(prepairOrphansOrder2(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _) => {
                    Ok(prepairOrphansOrder2(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), imark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(omark)
}

fn prepairOrphansOrder3(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut preorphan: i32, mut partner: Arc<metamodelica::List<i32>>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut prer: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (eqns.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), partner.clone(), orphans.clone(), prer.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    r = ass2.borrow()[(e.clone()-1) as usize].clone();
                    lst = List::unique(List::flatten(List::map1r(r.clone(), Arc::new(arrayGet.clone()), orphans.clone())));
                    let true = (listMember(preorphan.clone(), lst.clone())) else { bail!("pattern mismatch") };
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                    List::map2_0(r.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
                    elst = List::select1(List::map1r(r.clone(), Arc::new(arrayGet.clone()), ass1.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    next = List::flatten(List::map1r(r.clone(), Arc::new(arrayGet.clone()), mt.clone()));
                    next = List::fold1(elst.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), next.clone());
                    prepairOrphansOrder3(next.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), partner.clone(), orphans.clone(), r.clone())?;
                    prepairOrphansOrder3(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), partner.clone(), orphans.clone(), prer.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) => {
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    List::map4_0(prer.clone(), (std::sync::Arc::new(generateClique) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, i32) -> Result<()> + 'static>), m.clone(), mt.clone(), partner.clone(), e.clone());
                    prepairOrphansOrder3(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), partner.clone(), orphans.clone(), prer.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) => {
                    prepairOrphansOrder3(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), partner.clone(), orphans.clone(), prer.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn generateClique(mut r: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut orphans: Arc<metamodelica::List<i32>>, mut e: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((r.clone(), m.clone(), mt.clone(), orphans.clone(), e.clone())) {
        (_, _, _, Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (_, _, _, Deref @ metamodelica::List::Cons { head: orphan, tail: rest }, _) => {
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            lst = mt.borrow()[(r.clone()-1) as usize].clone();
            lst = List::removeOnTrue(e.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst.clone());
            {let _arr = mt.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = lst.clone(); _arr};
            lst = mt.borrow()[(orphan.clone()-1) as usize].clone();
            lst = List::unique(cons(e.clone(), lst.clone()));
            {let _arr = mt.clone(); _arr.borrow_mut()[(orphan.clone()-1) as usize] = lst.clone(); _arr};
            lst = m.borrow()[(e.clone()-1) as usize].clone();
            lst = List::removeOnTrue(r.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst.clone());
            lst = List::unique(cons(orphan.clone(), lst.clone()));
            {let _arr = m.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = lst.clone(); _arr};
            generateClique(r.clone(), m.clone(), mt.clone(), rest.clone(), e.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn generateResidualClique(mut r: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut orphans: Arc<metamodelica::List<i32>>, mut e: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((r.clone(), m.clone(), mt.clone(), orphans.clone(), e.clone())) {
        (_, _, _, Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (_, _, _, Deref @ metamodelica::List::Cons { head: orphan, tail: rest }, _) => {
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            lst = m.borrow()[(e.clone()-1) as usize].clone();
            lst = List::removeOnTrue(r.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst.clone());
            {let _arr = m.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = lst.clone(); _arr};
            lst = m.borrow()[(orphan.clone()-1) as usize].clone();
            lst = List::unique(cons(r.clone(), lst.clone()));
            {let _arr = m.clone(); _arr.borrow_mut()[(orphan.clone()-1) as usize] = lst.clone(); _arr};
            lst = mt.borrow()[(r.clone()-1) as usize].clone();
            lst = List::removeOnTrue(e.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), lst.clone());
            lst = List::unique(cons(orphan.clone(), lst.clone()));
            {let _arr = mt.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = lst.clone(); _arr};
            generateResidualClique(r.clone(), m.clone(), mt.clone(), rest.clone(), e.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getOrphansOrderEdvanced(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mc: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mct: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut omark: i32 = 0;
    omark = 'mc: {
        let __mc_input = (inOrphans.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(mark.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    let false = (intEq(rowmarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = rowmarks.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = mark.clone(); _arr};
                    getOrphansOrderEdvanced1(mct.borrow()[(o.clone()-1) as usize].clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), metamodelica::nil())?;
                    Ok(getOrphansOrderEdvanced(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone() + 1, rowmarks.clone(), colummarks.clone(), orphans.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(getOrphansOrderEdvanced(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(omark)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasOrphanAdvanced(mut rows: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oAcc = (::match_deref::match_deref! { match &((rows.clone(), ass1.clone(), iAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            iAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _) => {
            if (!(intGt(ass1.borrow()[(r.clone()-1) as usize].clone(), 0))) {hasOrphanAdvanced(rest.clone(), ass1.clone(), cons(r.clone(), iAcc.clone()))?} else {hasOrphanAdvanced(rest.clone(), ass1.clone(), iAcc.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oAcc)
}

fn addPreOrphan(mut orphan: i32, mut preorphan: i32, mut arr: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut olst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    olst = arr.borrow()[(orphan.clone()-1) as usize].clone();
    olst = List::unionElt(preorphan.clone(), olst.clone());
    let _ = {let _arr = arr.clone(); _arr.borrow_mut()[(orphan.clone()-1) as usize] = olst.clone(); _arr};
    Ok(())
}

fn addPreOrphans(mut orphan: i32, mut preorphans: Arc<metamodelica::List<i32>>, mut arr: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut olst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let _ = (::match_deref::match_deref! { match &((orphan.clone(), preorphans.clone(), arr.clone())) {
        (_, Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (_, Deref @ metamodelica::List::Cons { head: o, tail: rest }, _) => {
            addPreOrphan(orphan.clone(), o.clone(), arr.clone())?;
            addPreOrphans(orphan.clone(), rest.clone(), arr.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn getOrphansOrderEdvanced1(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut preorphan: i32, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nextQueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (eqns.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), nextQueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _) => {
                    getOrphansOrderEdvanced1(nextQueue.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: _ }, _, _, _, _, _, _, _, _, _, _) => {
                    let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut olst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    r = List::removeOnTrue(preorphan.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), m.borrow()[(e.clone()-1) as usize].clone());
                    olst = hasOrphanAdvanced(r.clone(), ass1.clone(), metamodelica::nil())?;
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                    addPreOrphans(preorphan.clone(), olst.clone(), orphans.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    r = List::removeOnTrue(preorphan.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), m.borrow()[(e.clone()-1) as usize].clone());
                    r1 = List::select1(ass2.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    r = List::fold1(r1.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone());
                    elst = List::select1(List::map1r(r.clone(), Arc::new(arrayGet.clone()), ass1.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    next = listAppend(nextQueue.clone(), elst.clone());
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                    getOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    getOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), nextQueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getConstraintesOrphansOrderEdvanced(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mc: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mct: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut omark: i32 = 0;
    omark = 'mc: {
        let __mc_input = (inOrphans.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(mark.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    let false = (intEq(rowmarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = rowmarks.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = mark.clone(); _arr};
                    getConstraintesOrphansOrderEdvanced1(mct.borrow()[(o.clone()-1) as usize].clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), o.clone(), orphans.clone(), metamodelica::nil())?;
                    Ok(getConstraintesOrphansOrderEdvanced(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone() + 1, rowmarks.clone(), colummarks.clone(), orphans.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(getConstraintesOrphansOrderEdvanced(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mc.clone(), mct.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphans.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(omark)
}

fn getConstraintesOrphansOrderEdvanced1(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut preorphan: i32, mut orphans: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nextQueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (eqns.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), nextQueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _) => {
                    getConstraintesOrphansOrderEdvanced1(nextQueue.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut olst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    r = List::removeOnTrue(preorphan.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), m.borrow()[(e.clone()-1) as usize].clone());
                    olst = hasOrphanAdvanced(r.clone(), ass1.clone(), metamodelica::nil())?;
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                    addPreOrphans(preorphan.clone(), olst.clone(), orphans.clone())?;
                    r1 = ass2.borrow()[(e.clone()-1) as usize].clone();
                    r = List::fold1(r1.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone());
                    elst = List::select1(List::map1r(r.clone(), Arc::new(arrayGet.clone()), ass1.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    next = listAppend(nextQueue.clone(), elst.clone());
                    getConstraintesOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut r1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    r = List::removeOnTrue(preorphan.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), m.borrow()[(e.clone()-1) as usize].clone());
                    r1 = ass2.borrow()[(e.clone()-1) as usize].clone();
                    r = List::fold1(r1.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), r.clone());
                    elst = List::select1(List::map1r(r.clone(), Arc::new(arrayGet.clone()), ass1.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    next = listAppend(nextQueue.clone(), elst.clone());
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                    getConstraintesOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    getConstraintesOrphansOrderEdvanced1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), preorphan.clone(), orphans.clone(), nextQueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn mergeOrphanParents(mut links: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oAcc = 'mc: {
        let __mc_input = (links.clone(), m.clone(), iAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: l, tail: rest }, _, _) => {
                    ::match_deref::match_deref! { match &(m.borrow()[(l.clone()-1) as usize].clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(mergeOrphanParents(rest.clone(), m.clone(), iAcc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: l, tail: rest }, _, _) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    lst = m.borrow()[(l.clone()-1) as usize].clone();
                    Ok(mergeOrphanParents(rest.clone(), m.clone(), listAppend(lst.clone(), iAcc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oAcc)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getLinkPosition(mut orphans: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut ochilds: Arc<metamodelica::List<i32>> = metamodelica::nil();
    ochilds = 'mc: {
        let __mc_input = (orphans.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), iAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: rest }, _, _, _, _, _) => {
                    let mut childs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(rowmarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = rowmarks.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = mark.clone(); _arr};
                    childs = getLinkPosition1(m.borrow()[(o.clone()-1) as usize].clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), o.clone(), iAcc.clone())?;
                    Ok(getLinkPosition(rest.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), childs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _) => {
                    Ok(getLinkPosition(rest.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), iAcc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(ochilds)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getLinkPosition1(mut orphans: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut preorphan: i32, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut childs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    childs = 'mc: {
        let __mc_input = (orphans.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), preorphan.clone(), iAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
                    Ok(cons(preorphan.clone(), iAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: Deref @ metamodelica::List::Nil }, _, _, _, _, _, _) => {
                    let false = (intEq(rowmarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = rowmarks.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = mark.clone(); _arr};
                    Ok(getLinkPosition1(m.borrow()[(o.clone()-1) as usize].clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), o.clone(), iAcc.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: Deref @ metamodelica::List::Nil }, _, _, _, _, _, _) => {
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let true = (intEq(rowmarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    lst = listAppend(mt.borrow()[(0-1) as usize].clone(), iAcc.clone());
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _) => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error in getLinkPosition1! Found Orphan with more than one parents ")); __mm_s.push_str(&*stringDelimitList(List::map(orphans.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(childs)
}

fn getOrphansOrderEdvanced5(mut linklst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imark: i32, mut rowmarks: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, i32)> {
    let mut oAcc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut omark: i32 = 0;
    (oAcc, omark) = (::match_deref::match_deref! { match &((linklst.clone(), m.clone(), mt.clone(), imark.clone(), rowmarks.clone(), iAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            (iAcc.clone().reverse(), imark.clone())
        },
        (Deref @ metamodelica::List::Cons { head: links, tail: rest }, _, _, _, _, _) => {
            let mut mark: i32 = 0;
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut childs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            lst = mergeOrphanParents(links.clone(), m.clone(), metamodelica::nil())?;
            childs = getLinkPosition(lst.clone(), m.clone(), mt.clone(), imark.clone(), rowmarks.clone(), metamodelica::nil())?;
            (acc, mark) = getOrphansOrderEdvanced5(rest.clone(), m.clone(), mt.clone(), imark.clone() + 1, rowmarks.clone(), cons(childs.clone(), iAcc.clone()))?;
            (acc.clone(), mark.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((oAcc, omark))
}

fn getOrphansOrderEdvanced6(mut linklst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut childslst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((linklst.clone(), childslst.clone(), m.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: links, tail: rest }, Deref @ metamodelica::List::Cons { head: childs, tail: acc }, _) => {
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            lst = List::unique(List::flatten(List::map1r(childs.clone(), Arc::new(arrayGet.clone()), m.clone())));
            List::map2_0(links.clone(), (std::sync::Arc::new(doAssign) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), m.clone(), lst.clone());
            List::map2_0(childs.clone(), (std::sync::Arc::new(doAssign) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<()> + 'static>), m.clone(), links.clone());
            getOrphansOrderEdvanced6(rest.clone(), acc.clone(), m.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn getOrphansOrderEdvanced4(mut linklst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imark: i32, mut rowmarks: metamodelica::Array<i32>, mut iorder: Arc<metamodelica::List<i32>>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<i32> {
    let mut omark: i32 = 0;
    let mut childs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (childs, omark) = getOrphansOrderEdvanced5(linklst.clone(), m.clone(), mt.clone(), imark.clone(), rowmarks.clone(), metamodelica::nil())?;
    getOrphansOrderEdvanced6(linklst.clone(), childs.clone(), m.clone())?;
    Ok(omark)
}

fn getInvMap(mut orphan: i32, mut invmap: metamodelica::Array<i32>, mut index: i32) -> Result<i32> {
    let mut oindex: i32 = 0;
    let _ = {let _arr = invmap.clone(); _arr.borrow_mut()[(orphan.clone()-1) as usize] = index.clone(); _arr};
    oindex = index.clone() + 1;
    Ok(oindex)
}

fn getOrphansAdjacencyMatrix(mut orphans: Arc<metamodelica::List<i32>>, mut invmap: metamodelica::Array<i32>, mut vorphansarray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut addself: bool) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut outMT: metamodelica::Array<Arc<metamodelica::List<i32>>> = mT.clone();
    let mut m: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut am: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut amT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    for mut o in &*orphans.clone() {
        let mut o = o.clone();
        lst = List::map1r(vorphansarray.borrow()[(o.clone()-1) as usize].clone(), Arc::new(arrayGet.clone()), invmap.clone());
        i = invmap.borrow()[(o.clone()-1) as usize].clone();
        lst = List::consOnTrue(addself.clone(), i.clone(), lst.clone());
        outMT = List::fold1(lst.clone(), (std::sync::Arc::new(Array::consToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), i.clone(), outMT.clone());
        m = cons(lst.clone(), m.clone());
    }
    outM = List::listArrayReverse(m.clone())?;
    outMT = mT.clone();
    Ok((outM, outMT))
}

fn getOrder(mut comp: Arc<metamodelica::List<i32>>, mut inorder: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) {
    let mut outorder: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>);
    outorder = (::match_deref::match_deref! { match &((comp.clone(), inorder.clone())) {
        (Deref @ metamodelica::List::Cons { head: o, tail: Deref @ metamodelica::List::Nil }, (order, links)) => {
            (cons(o.clone(), order.clone()), links.clone())
        },
        (_, (order, links)) => {
            (order.clone(), cons(comp.clone(), links.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outorder
}

fn getOrphansOrderEdvanced3(mut roots: Arc<metamodelica::List<i32>>, mut otherorphans: Arc<metamodelica::List<i32>>, mut constraints: Arc<metamodelica::List<i32>>, mut vorphans: Arc<metamodelica::List<i32>>, mut vorphansarray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut sortvorphans: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut omark: i32 = 0;
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut leafs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut childlist: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut size: i32 = 0;
    let mut map: metamodelica::Array<i32>;
    let mut ass: metamodelica::Array<i32>;
    let mut invmap: metamodelica::Array<i32>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut range: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut links: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut linkslst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    map = metamodelica::arrayFromVec(vorphans.clone().into_iter().cloned().collect());
    size = (map.clone().borrow().len() as i32);
    invmap = arrayCreate((vorphansarray.clone().borrow().len() as i32), 0);
    let _ = List::fold1(vorphans.clone(), (std::sync::Arc::new(getInvMap) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<i32> + 'static>), invmap.clone(), 1);
    range = List::intRange(size.clone());
    (m, mt) = getOrphansAdjacencyMatrix(vorphans.clone(), invmap.clone(), vorphansarray.clone(), arrayCreate(size.clone(), metamodelica::nil()), true)?;
    ass = metamodelica::arrayFromVec(range.clone().into_iter().cloned().collect());
    comps = Sorting::TarjanTransposed(mt.clone(), ass.clone())?;
    (order, linkslst) = List::fold(comps.clone(), (std::sync::Arc::new(fnptr!(getOrder, Arc<metamodelica::List<i32>>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> + 'static>), (metamodelica::nil(), metamodelica::nil()));
    (m, mt) = getOrphansAdjacencyMatrix(vorphans.clone(), invmap.clone(), vorphansarray.clone(), arrayCreate(size.clone(), metamodelica::nil()), false)?;
    reduceOrphancMatrix(comps.clone().reverse(), m.clone())?;
    omark = getOrphansOrderEdvanced4(linkslst.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), order.clone(), metamodelica::nil())?;
    mt = AdjacencyMatrix::transposeAdjacencyMatrix(m.clone(), (mt.clone().borrow().len() as i32))?;
    comps = Sorting::TarjanTransposed(mt.clone(), ass.clone())?;
    sortvorphans = List::flattenReverse(comps.clone());
    sortvorphans = List::map1r(sortvorphans.clone(), Arc::new(arrayGet.clone()), map.clone());
    Ok((sortvorphans, omark))
}

fn reduceOrphancMatrix(mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((comps.clone(), m.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, tail: rest }, _) => {
            reduceOrphancMatrix(rest.clone(), m.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: comp, tail: rest }, _) => {
            reduceOrphancMatrix1(comp.clone(), comp.clone(), m.clone())?;
            reduceOrphancMatrix(rest.clone(), m.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn reduceOrphancMatrix1(mut comps: Arc<metamodelica::List<i32>>, mut comps1: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((comps.clone(), comps1.clone(), m.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _, _) => {
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            lst = m.borrow()[(c.clone()-1) as usize].clone();
            lst = List::setDifference(lst.clone(), comps1.clone())?;
            {let _arr = m.clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = lst.clone().reverse(); _arr};
            reduceOrphancMatrix1(rest.clone(), comps1.clone(), m.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasResidualOrphan1(mut eqns: Arc<metamodelica::List<i32>>, mut ass: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqnsarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut Orphan: i32 = 0;
    Orphan = 'mc: {
        let __mc_input = (eqns.clone(), ass.clone(), eqnsarr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: _ }, _, _) => {
                    let mut len: i32 = 0;
                    let mut size: i32 = 0;
                    len = (ass.borrow()[(e.clone()-1) as usize].clone().len() as i32);
                    size = BackendEquation::equationSize(BackendEquation::get(eqnsarr.clone(), e.clone()))?;
                    let true = (intLt(len.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _) => {
                    Ok(hasResidualOrphan1(rest.clone(), ass.clone(), eqnsarr.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(Orphan)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasResidualOrphan(mut eqns: Arc<metamodelica::List<i32>>, mut ass: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut Orphan: i32 = 0;
    Orphan = 'mc: {
        let __mc_input = (eqns.clone(), ass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: _ }, _) => {
                    ::match_deref::match_deref! { match &(ass.borrow()[(e.clone()-1) as usize].clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    Ok(hasResidualOrphan(rest.clone(), ass.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(Orphan)
}

fn makeCrefExps(mut v: BackendDAE::Var) -> Result<Arc<DAE::Exp>> {
    let mut e: Arc<DAE::Exp>;
    e = Expression::crefExp(BackendVariable::varCref(v.clone())?)?;
    Ok(e)
}

fn makeGausEliminationRow(mut lst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, mut size: i32, mut vars: metamodelica::Array<Arc<DAE::Exp>>, mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outExp1: Arc<DAE::Exp>;
    (outExp, outExp1) = 'mc: {
        let __mc_input = (lst.clone(), size.clone(), vars.clone(), inExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok((inExp.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, e), tail: _ }, _, _, _) => {
                    let true = (intGt(c.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok((inExp.clone(), e.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (c, e), tail: rest }, _, _, _) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut b: Arc<DAE::Exp>;
                    e1 = Expression::expMul(e.clone(), vars.borrow()[(c.clone()-1) as usize].clone())?;
                    e1 = Expression::expAdd(e1.clone(), inExp.clone())?;
                    (e1, b) = makeGausEliminationRow(rest.clone(), size.clone(), vars.clone(), e1.clone())?;
                    Ok((e1.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outExp1))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeGausElimination(mut row: i32, mut size: i32, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>, mut vars: metamodelica::Array<Arc<DAE::Exp>>, mut iAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut oAcc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    oAcc = 'mc: {
        let __mc_input = (row.clone(), size.clone(), matrix.clone(), vars.clone(), iAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let true = (intGt(row.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok(iAcc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut b: Arc<DAE::Exp>;
                    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    (e, b) = makeGausEliminationRow(matrix.borrow()[(row.clone()-1) as usize].clone(), size.clone(), vars.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: b.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
                    Ok(makeGausElimination(row.clone() + 1, size.clone(), matrix.clone(), vars.clone(), cons(eqn.clone(), iAcc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oAcc)
}

fn dumpMatrix(mut row: i32, mut size: i32, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (row.clone(), size.clone(), matrix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(row.clone(), size.clone())) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _) = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(row.clone())); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone());
            BackendDump::debuglst(matrix.borrow()[(row.clone()-1) as usize].clone(), (std::sync::Arc::new(dumpMatrix1) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<DAE::Exp>)) -> Result<ArcStr> + 'static>), (literal!(", ")).clone(), (literal!("\n")).clone());
            dumpMatrix(row.clone() + 1, size.clone(), matrix.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpMatrix1(mut inTpl: (i32, Arc<DAE::Exp>)) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    let mut c: i32 = 0;
    let mut e: Arc<DAE::Exp>;
    let mut cs: ArcStr = arcstr::literal!("");
    let mut es: ArcStr = arcstr::literal!("");
    (c, e) = inTpl.clone();
    cs = (intString(c.clone())).clone();
    es = (ExpressionBasics::printExpStr(e.clone())?).clone();
    s = stringAppendList(list![(cs.clone()).clone(), (literal!(":")).clone(), (es.clone()).clone()]);
    Ok(s)
}

fn addRows(mut inA: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, mut inB: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, mut col: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inTpl: (i32, i32), mut inElst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>) -> Result<(Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (i32, i32))> {
    let mut outElst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outTpl: (i32, i32);
    (outElst, outVars, outEqns, outTpl) = 'mc: {
        let __mc_input = (inA.clone(), inB.clone(), col.clone(), inVars.clone(), inEqns.clone(), inTpl.clone(), inElst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok((inElst.clone().reverse(), inVars.clone(), inEqns.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
                    Ok((List::append_reverse(inElst.clone(), inB.clone()), inVars.clone(), inEqns.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    Ok((List::append_reverse(inElst.clone(), inA.clone()), inVars.clone(), inEqns.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, _), tail: resta }, Deref @ metamodelica::List::Cons { head: (cb, _), tail: restb }, _, _, _, _, _) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intEq(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(ca.clone(), col.clone())) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(resta.clone(), restb.clone(), col.clone(), inVars.clone(), inEqns.clone(), inTpl.clone(), inElst.clone())?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, ea), tail: resta }, Deref @ metamodelica::List::Cons { head: (cb, eb), tail: restb }, _, _, _, _, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intEq(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    e = Expression::expAdd(ea.clone(), eb.clone())?;
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    (vars, eqns, e, tpl) = makeDummyVar(inTpl.clone(), e.clone(), inVars.clone(), inEqns.clone())?;
                    (elst, vars, eqns, tpl) = addRows(resta.clone(), restb.clone(), col.clone(), vars.clone(), eqns.clone(), tpl.clone(), cons((ca.clone(), e.clone()), inElst.clone()))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, _), tail: _ }, Deref @ metamodelica::List::Cons { head: (cb, _), tail: restb }, _, _, _, _, _) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intGt(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(cb.clone(), col.clone())) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(inA.clone(), restb.clone(), col.clone(), inVars.clone(), inEqns.clone(), inTpl.clone(), inElst.clone())?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, _), tail: _ }, Deref @ metamodelica::List::Cons { head: (cb, eb), tail: restb }, _, _, _, _, _) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intGt(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(inA.clone(), restb.clone(), col.clone(), inVars.clone(), inEqns.clone(), inTpl.clone(), cons((cb.clone(), eb.clone()), inElst.clone()))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, _), tail: resta }, Deref @ metamodelica::List::Cons { head: (cb, _), tail: _ }, _, _, _, _, _) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intLt(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(ca.clone(), col.clone())) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(resta.clone(), inB.clone(), col.clone(), inVars.clone(), inEqns.clone(), inTpl.clone(), inElst.clone())?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ca, ea), tail: resta }, Deref @ metamodelica::List::Cons { head: (cb, _), tail: _ }, _, _, _, _, _) => {
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    let true = (intLt(ca.clone(), cb.clone())) else { bail!("pattern mismatch") };
                    (elst, vars, eqns, tpl) = addRows(resta.clone(), inB.clone(), col.clone(), inVars.clone(), inEqns.clone(), inTpl.clone(), cons((ca.clone(), ea.clone()), inElst.clone()))?;
                    Ok((elst.clone(), vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElst, outVars, outEqns, outTpl))
}

fn mulRow(mut inTpl: (i32, Arc<DAE::Exp>), mut e1: Arc<DAE::Exp>) -> Result<(i32, Arc<DAE::Exp>)> {
    let mut outTpl: (i32, Arc<DAE::Exp>);
    let mut e: Arc<DAE::Exp>;
    let mut c: i32 = 0;
    (c, e) = inTpl.clone();
    e = Expression::negate(Expression::expMul(e.clone(), e1.clone())?)?;
    outTpl = (c.clone(), e.clone());
    Ok(outTpl)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeFromCol(mut i: i32, mut inTpl: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>, mut inAcc: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>) -> Result<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>> {
    let mut outAcc: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
    outAcc = (::match_deref::match_deref! { match &((i.clone(), inTpl.clone(), inAcc.clone())) {
        (_, Deref @ metamodelica::List::Nil, _) => {
            inAcc.clone().reverse()
        },
        (_, Deref @ metamodelica::List::Cons { head: (c, _), tail: rest }, _) if (intEq(i.clone(), c.clone())) => {
            listAppend(inAcc.clone().reverse(), rest.clone())
        },
        (_, Deref @ metamodelica::List::Cons { head: (c, e), tail: rest }, _) => {
            removeFromCol(i.clone(), rest.clone(), cons((c.clone(), e.clone()), inAcc.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAcc)
}

fn makeDummyVar(mut inTpl: (i32, i32), mut e: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<DAE::Exp>, (i32, i32))> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (i32, i32);
    (outVars, outEqns, outExp, outTpl) = 'mc: {
        let __mc_input = (inTpl.clone(), e.clone(), inVars.clone(), inEqns.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { .. }, _, _) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CREF { .. }, .. }, _, _) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::RCONST { .. }, _, _) => {
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let true = (Expression::isConst(e.clone())?) else { bail!("pattern mismatch") };
                    Ok((inVars.clone(), inEqns.clone(), e.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((a, b), _, _, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut sa: ArcStr = arcstr::literal!("");
                    let mut sb: ArcStr = arcstr::literal!("");
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut cexp: Arc<DAE::Exp>;
                    sa = (intString(a.clone())).clone();
                    sb = (intString(b.clone())).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent(stringAppendList(list![(literal!("$tmp")).clone(), (sa.clone()).clone(), (literal!("_")).clone(), (sb.clone()).clone()]), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
                    cexp = Expression::crefExp(cr.clone())?;
                    eqns = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: cexp.clone(), scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), inEqns.clone())?;
                    v = BackendDAE::Var { varName: cr.clone(), varKind: crate::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: Arc::new(openmodelica_frontend_types::DAE::ConnectorType::NON_CONNECTOR), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    vars = BackendVariable::addVar(v.clone(), inVars.clone())?;
                    Ok((vars.clone(), eqns.clone(), cexp.clone(), (a.clone(), b.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVars, outEqns, outExp, outTpl))
}

fn gaussElimination1(mut col: i32, mut row: i32, mut size: i32, mut ce: Arc<DAE::Exp>, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inTpl: (i32, i32)) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (i32, i32))> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outTpl: (i32, i32);
    (outVars, outEqns, outTpl) = 'mc: {
        let __mc_input = (col.clone(), row.clone(), size.clone(), ce.clone(), matrix.clone(), inVars.clone(), inEqns.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let true = (intGt(row.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok((inVars.clone(), inEqns.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut e: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut cexp: Arc<DAE::Exp>;
                    let mut elst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let mut tpl: (i32, i32);
                    let __pa0 = ::match_deref::match_deref! { match &(diagonalEntry(col.clone(), matrix.borrow()[(row.clone()-1) as usize].clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    e1 = Expression::expDiv(e.clone(), ce.clone())?;
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (vars, eqns, cexp, tpl) = makeDummyVar(inTpl.clone(), e1.clone(), inVars.clone(), inEqns.clone())?;
                    elst = matrix.borrow()[(col.clone()-1) as usize].clone();
                    elst = List::map1(elst.clone(), (std::sync::Arc::new(mulRow) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<DAE::Exp>), Arc<DAE::Exp>) -> Result<(i32, Arc<DAE::Exp>)> + 'static>), cexp.clone());
                    (elst, vars, eqns, tpl) = addRows(matrix.borrow()[(row.clone()-1) as usize].clone(), elst.clone(), col.clone(), vars.clone(), eqns.clone(), tpl.clone(), metamodelica::nil())?;
                    {let _arr = matrix.clone(); _arr.borrow_mut()[(row.clone()-1) as usize] = elst.clone(); _arr};
                    (vars, eqns, tpl) = gaussElimination1(col.clone(), row.clone() + 1, size.clone(), ce.clone(), matrix.clone(), vars.clone(), eqns.clone(), tpl.clone())?;
                    Ok((vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut tpl: (i32, i32);
                    (vars, eqns, tpl) = gaussElimination1(col.clone(), row.clone() + 1, size.clone(), ce.clone(), matrix.clone(), inVars.clone(), inEqns.clone(), inTpl.clone())?;
                    Ok((vars.clone(), eqns.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVars, outEqns, outTpl))
}

fn gaussElimination(mut col: i32, mut size: i32, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inTpl: (i32, i32)) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut outVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    (outVars, outEqns) = 'mc: {
        let __mc_input = (col.clone(), size.clone(), matrix.clone(), inVars.clone(), inEqns.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let true = (intGt(col.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok((inVars.clone(), inEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut e: Arc<DAE::Exp>;
                    let mut tpl: (i32, i32);
                    let __pa0 = ::match_deref::match_deref! { match &(diagonalEntry(col.clone(), matrix.borrow()[(col.clone()-1) as usize].clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    (vars, eqns, tpl) = gaussElimination1(col.clone(), col.clone() + 1, size.clone(), e.clone(), matrix.clone(), inVars.clone(), inEqns.clone(), inTpl.clone())?;
                    (vars, eqns) = gaussElimination(col.clone() + 1, size.clone(), matrix.clone(), vars.clone(), eqns.clone(), tpl.clone())?;
                    Ok((vars.clone(), eqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    ::match_deref::match_deref! { match &(diagonalEntry(col.clone(), matrix.borrow()[(col.clone()-1) as usize].clone())?) {
                        None => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("gaussElimination failt because of non diagonal Entry for col ")); __mm_s.push_str(&*intString(col.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVars, outEqns))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn diagonalEntry(mut col: i32, mut row: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut oe: Option<Arc<DAE::Exp>> = None;
    oe = (::match_deref::match_deref! { match &((col.clone(), row.clone())) {
        (_, Deref @ metamodelica::List::Cons { head: (r, e), tail: rest }) => {
            if (intEq(r.clone(), col.clone()) && !(Expression::isZero(e.clone()))) {Some(e.clone())} else {if (intGt(r.clone(), col.clone())) {None} else {diagonalEntry(col.clone(), rest.clone())?}}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oe)
}

fn isConstOneMinusOne(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut b: bool = false;
    b = Expression::isConstOne(inExp.clone()) || Expression::isConstMinusOne(inExp.clone());
    b
}

fn transformJacToAdjacencyMatrix2(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mapIncRowEqn: metamodelica::Array<i32>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let _ = (::match_deref::match_deref! { match &((jac.clone(), m.clone(), mapIncRowEqn.clone(), eqns.clone(), ass1.clone(), ass2.clone(), func.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest }, _, _, _, _, _, _) => {
            let mut i: i32 = 0;
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            i = mapIncRowEqn.borrow()[(r.clone()-1) as usize].clone();
            eqn = BackendEquation::get(eqns.clone(), i.clone());
            b1 = BackendEquation::isArrayEquation(eqn.clone());
            b = func(e.clone())?;
            lst = List::consOnTrue(b.clone() && b1.clone(), c.clone(), m.borrow()[(r.clone()-1) as usize].clone());
            {let _arr = m.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = lst.clone(); _arr};
            transformJacToAdjacencyMatrix2(rest.clone(), m.clone(), mapIncRowEqn.clone(), eqns.clone(), ass1.clone(), ass2.clone(), func.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToAdjacencyMatrix1(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let _ = (::match_deref::match_deref! { match &((jac.clone(), m.clone(), ass1.clone(), ass2.clone(), func.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest }, _, _, _, _) => {
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            b1 = intLt(ass1.borrow()[(c.clone()-1) as usize].clone(), 1);
            b = func(e.clone())?;
            lst = List::consOnTrue(b.clone() && b1.clone(), c.clone(), m.borrow()[(r.clone()-1) as usize].clone());
            {let _arr = m.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = lst.clone(); _arr};
            transformJacToAdjacencyMatrix1(rest.clone(), m.clone(), ass1.clone(), ass2.clone(), func.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToAdjacencyMatrix(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>) -> Result<()> {
    pub type CompareFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>;

    let _ = (::match_deref::match_deref! { match &((jac.clone(), m.clone(), mT.clone(), func.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            transformJacToAdjacencyMatrix(jac.clone(), m.clone(), mT.clone(), func.clone())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest }, _, _, _) => {
            let mut b: bool = false;
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut lst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            b = func(e.clone())?;
            lst = List::consOnTrue(b.clone(), c.clone(), m.borrow()[(r.clone()-1) as usize].clone());
            lst1 = List::consOnTrue(b.clone(), r.clone(), mT.borrow()[(c.clone()-1) as usize].clone());
            {let _arr = m.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = lst.clone(); _arr};
            {let _arr = mT.clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = lst1.clone(); _arr};
            transformJacToAdjacencyMatrix(rest.clone(), m.clone(), mT.clone(), func.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn transformJacToMatrix(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut row: i32, mut col: i32, mut size: i32, mut b: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut matrix: metamodelica::Array<Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (jac.clone(), row.clone(), col.clone(), size.clone(), b.clone(), matrix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let true = (intGt(row.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut be: Arc<DAE::Exp>;
                    let mut b1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut lst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let true = (intGt(col.clone(), size.clone())) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(b.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    be = __pa0.clone();
                    b1 = __pa1.clone();
                    lst = matrix.borrow()[(row.clone()-1) as usize].clone();
                    lst = List::consOnTrue(!(Expression::isZero(be.clone())), (col.clone(), be.clone()), lst.clone());
                    lst = lst.clone().reverse();
                    {let _arr = matrix.clone(); _arr.borrow_mut()[(row.clone()-1) as usize] = lst.clone(); _arr};
                    transformJacToMatrix(jac.clone(), row.clone() + 1, 1, size.clone(), b1.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    transformJacToMatrix(jac.clone(), row.clone(), col.clone() + 1, size.clone(), b.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest }, _, _, _, _, _) => {
                    let mut lst: Arc<metamodelica::List<(i32, Arc<DAE::Exp>)>> = metamodelica::nil();
                    let true = (intEq(r.clone(), row.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(c.clone(), col.clone())) else { bail!("pattern mismatch") };
                    lst = matrix.borrow()[(r.clone()-1) as usize].clone();
                    lst = cons((c.clone(), e.clone()), lst.clone());
                    {let _arr = matrix.clone(); _arr.borrow_mut()[(row.clone()-1) as usize] = lst.clone(); _arr};
                    transformJacToMatrix(rest.clone(), row.clone(), col.clone() + 1, size.clone(), b.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (r, c, _), tail: _ }, _, _, _, _, _) => {
                    let true = (intEq(r.clone(), row.clone())) else { bail!("pattern mismatch") };
                    let true = (intLt(col.clone(), c.clone())) else { bail!("pattern mismatch") };
                    transformJacToMatrix(jac.clone(), row.clone(), col.clone() + 1, size.clone(), b.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (r, _, _), tail: _ }, _, _, _, _, _) => {
                    let true = (intGe(r.clone(), row.clone())) else { bail!("pattern mismatch") };
                    transformJacToMatrix(jac.clone(), row.clone(), col.clone() + 1, size.clone(), b.clone(), matrix.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpJacMatrix(mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut row: i32, mut col: i32, mut size: i32, mut vars: BackendDAE::Variables) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (jac.clone(), row.clone(), col.clone(), size.clone(), vars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let true = (intGt(row.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let true = (intGt(col.clone(), size.clone())) else { bail!("pattern mismatch") };
                    v = BackendVariable::getVarAt(vars.clone(), row.clone())?;
                    cr = BackendVariable::varCref(v.clone())?;
                    println!("{}", (literal!(";... % ")).clone());
                    println!("{}", (intString(row.clone())).clone());
                    println!("{}", (literal!(" ")).clone());
                    println!("{}", (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone());
                    println!("{}", (literal!("\n")).clone());
                    dumpJacMatrix(jac.clone(), row.clone() + 1, 1, size.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    println!("{}", (literal!("0, ")).clone());
                    dumpJacMatrix(jac.clone(), row.clone(), col.clone() + 1, size.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (r, c, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: rest }, _, _, _, _) => {
                    let mut estr: ArcStr = arcstr::literal!("");
                    let true = (intEq(r.clone(), row.clone())) else { bail!("pattern mismatch") };
                    let true = (intEq(c.clone(), col.clone())) else { bail!("pattern mismatch") };
                    estr = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    println!("{}", (estr.clone()).clone());
                    println!("{}", (literal!(", ")).clone());
                    dumpJacMatrix(rest.clone(), row.clone(), col.clone() + 1, size.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (r, c, _), tail: _ }, _, _, _, _) => {
                    let true = (intEq(r.clone(), row.clone())) else { bail!("pattern mismatch") };
                    let true = (intLt(col.clone(), c.clone())) else { bail!("pattern mismatch") };
                    println!("{}", (literal!("0, ")).clone());
                    dumpJacMatrix(jac.clone(), row.clone(), col.clone() + 1, size.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (r, _, _), tail: _ }, _, _, _, _) => {
                    let false = (intEq(r.clone(), row.clone())) else { bail!("pattern mismatch") };
                    println!("{}", (literal!("0, ")).clone());
                    dumpJacMatrix(jac.clone(), row.clone(), col.clone() + 1, size.clone(), vars.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getEqnsinOrder(mut indx: i32, mut inTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)> {
    let mut outTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables);
    let mut e: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqnssort: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut varssort: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut vindxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (eqns, vars, ass2, eqnssort, varssort) = inTpl.clone();
    e = BackendEquation::get(eqns.clone(), indx.clone());
    eqnssort = BackendEquation::add(e.clone(), eqnssort.clone())?;
    vindxs = ass2.borrow()[(indx.clone()-1) as usize].clone();
    vlst = List::map1r(vindxs.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone());
    vlst = sortVarsforOrder(e.clone(), vlst.clone(), vindxs.clone(), vars.clone())?;
    varssort = BackendVariable::addVars(vlst.clone(), varssort.clone());
    outTpl = (eqns.clone(), vars.clone(), ass2.clone(), eqnssort.clone(), varssort.clone());
    Ok(outTpl)
}

fn sortVarsforOrder(mut inEqn: Arc<BackendDAE::Equation>, mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut vindxs: Arc<metamodelica::List<i32>>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = 'mc: {
        let __mc_input = (inEqn.clone(), inVarLst.clone(), vindxs.clone(), vars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, .. }, _, _, _) => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    elst = Expression::flattenArrayExpToList(e1.clone())?;
                    crlst = List::map(elst.clone(), (std::sync::Arc::new(Expression::expCrefNegCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    vlst = sortVarsforOrder1(crlst.clone(), 1, inVarLst.clone(), vindxs.clone(), arrayCreate((vindxs.clone().len() as i32), None), vars.clone())?;
                    Ok(vlst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e1, .. }, _, _, _) => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    elst = Expression::flattenArrayExpToList(e1.clone())?;
                    crlst = List::map(elst.clone(), (std::sync::Arc::new(Expression::expCrefNegCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    vlst = sortVarsforOrder1(crlst.clone(), 1, inVarLst.clone(), vindxs.clone(), arrayCreate((vindxs.clone().len() as i32), None), vars.clone())?;
                    Ok(vlst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    vlst = List::sort(inVarLst.clone(), (std::sync::Arc::new(BackendVariable::varSortFunc) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Var) -> Result<bool> + 'static>))?;
                    Ok(vlst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn sortVarsforOrder1(mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut index: i32, mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut vindxs: Arc<metamodelica::List<i32>>, mut vararray: metamodelica::Array<Option<BackendDAE::Var>>, mut vars: BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = 'mc: {
        let __mc_input = (crlst.clone(), index.clone(), inVarLst.clone(), vindxs.clone(), vararray.clone(), vars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    vlst = List::sort(inVarLst.clone(), (std::sync::Arc::new(BackendVariable::varSortFunc) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Var) -> Result<bool> + 'static>))?;
                    vlst = sortVarsforOrder2(1, vlst.clone(), vararray.clone(), metamodelica::nil())?;
                    Ok(vlst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr, tail: rest }, _, _, _, _, _) => {
                    let mut i: i32 = 0;
                    let mut p: i32 = 0;
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    (v, i) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    p = List::position(i.clone(), vindxs.clone())?;
                    ilst = listDelete(vindxs.clone(), p.clone())?;
                    vlst = listDelete(inVarLst.clone(), p.clone())?;
                    {let _arr = vararray.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = Some(v.clone()); _arr};
                    Ok(sortVarsforOrder1(rest.clone(), index.clone() + 1, vlst.clone(), ilst.clone(), vararray.clone(), vars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _) => {
                    Ok(sortVarsforOrder1(rest.clone(), index.clone() + 1, inVarLst.clone(), vindxs.clone(), vararray.clone(), vars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn sortVarsforOrder2(mut index: i32, mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut vararray: metamodelica::Array<Option<BackendDAE::Var>>, mut iAcc: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> {
    let mut outVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    outVarLst = 'mc: {
        let __mc_input = (index.clone(), inVarLst.clone(), vararray.clone(), iAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let true = (intGt(index.clone(), (vararray.clone().borrow().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(iAcc.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let Some(__pa0) = (vararray.borrow()[(index.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    v = __pa0.clone();
                    Ok(sortVarsforOrder2(index.clone() + 1, inVarLst.clone(), vararray.clone(), cons(v.clone(), iAcc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: v, tail: vlst }, _, _) => {
                    Ok(sortVarsforOrder2(index.clone() + 1, vlst.clone(), vararray.clone(), cons(v.clone(), iAcc.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVarLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getOrphansPairs(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>) -> Result<i32> {
    let mut omark: i32 = 0;
    omark = 'mc: {
        let __mc_input = (inOrphans.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _) => {
                    Ok(mark.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: rest }, _, _, _, _, _, _, _) => {
                    let false = (intEq(rowmarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    getOrphansPairs1(list![o.clone()], ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), o.clone(), metamodelica::nil())?;
                    Ok(getOrphansPairs(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone() + 1, rowmarks.clone(), colummarks.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _) => {
                    Ok(getOrphansPairs(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(omark)
}

fn getOrphansPairs1(mut rows: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orphan: i32, mut nextQueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (rows.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphan.clone(), nextQueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _) => {
                    getOrphansPairs1(nextQueue.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphan.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: _ }, _, _, _, _, _, _, _, _, _) => {
                    let mut o: i32 = 0;
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(rowmarks.borrow()[(r.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    elst = List::select1(mt.borrow()[(r.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    o = hasResidualOrphan(elst.clone(), ass2.clone())?;
                    {let _arr = ass1.clone(); _arr.borrow_mut()[(orphan.clone()-1) as usize] = o.clone(); _arr};
                    {let _arr = ass2.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = list![orphan.clone()]; _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _) => {
                    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(rowmarks.borrow()[(r.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    elst = List::select1(mt.borrow()[(r.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    next = List::select1(List::flatten(List::map1r(elst.clone(), Arc::new(arrayGet.clone()), ass2.clone())), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    next = listAppend(nextQueue.clone(), next.clone());
                    {let _arr = rowmarks.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = mark.clone(); _arr};
                    getOrphansPairs1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphan.clone(), next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _) => {
                    getOrphansPairs1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orphan.clone(), nextQueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getOrphansPairsConstraints(mut inOrphans: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<i32> {
    let mut omark: i32 = 0;
    omark = 'mc: {
        let __mc_input = (inOrphans.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqns.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _) => {
                    Ok(mark.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: o, tail: rest }, _, _, _, _, _, _, _, _) => {
                    let false = (intEq(colummarks.borrow()[(o.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = mark.clone(); _arr};
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getOrphansPairsConstraints Process Orphan ")); __mm_s.push_str(&*intString(o.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    getOrphansPairsConstraints1(mt.borrow()[(o.clone()-1) as usize].clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqns.clone(), o.clone(), metamodelica::nil())?;
                    Ok(getOrphansPairsConstraints(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone() + 1, rowmarks.clone(), colummarks.clone(), eqns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _) => {
                    Ok(getOrphansPairsConstraints(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqns.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(omark)
}

fn getOrphansPairsConstraints1(mut eqns: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut eqnsarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut orphan: i32, mut nextQueue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (eqns.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan.clone(), nextQueue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _) => {
                    getOrphansPairsConstraints1(nextQueue.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan.clone(), metamodelica::nil())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: _ }, _, _, _, _, _, _, _, _, _, _) => {
                    let mut o: i32 = 0;
                    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut ass2lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    rlst = List::select1(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    rlst = List::fold1(ass2.borrow()[(e.clone()-1) as usize].clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rlst.clone());
                    next = List::select1(List::flatten(List::map1r(rlst.clone(), Arc::new(arrayGet.clone()), mt.clone())), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    o = hasResidualOrphan1(next.clone(), ass2.clone(), eqnsarr.clone())?;
                    {let _arr = ass1.clone(); _arr.borrow_mut()[(orphan.clone()-1) as usize] = o.clone(); _arr};
                    ass2lst = ass2.borrow()[(o.clone()-1) as usize].clone();
                    ass2lst = cons(orphan.clone(), ass2lst.clone());
                    {let _arr = ass2.clone(); _arr.borrow_mut()[(o.clone()-1) as usize] = ass2lst.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    let mut next: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    rlst = List::select1(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    lst = List::select1(List::map1r(rlst.clone(), Arc::new(arrayGet.clone()), ass1.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    rlst = List::fold1(lst.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rlst.clone());
                    next = List::select1(List::map1r(rlst.clone(), Arc::new(arrayGet.clone()), ass1.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0);
                    next = listAppend(nextQueue.clone(), next.clone());
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                    getOrphansPairsConstraints1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan.clone(), next.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _) => {
                    getOrphansPairsConstraints1(rest.clone(), ass1.clone(), ass2.clone(), m.clone(), mt.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), eqnsarr.clone(), orphan.clone(), nextQueue.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getIndexesForEqnsAdvanced(mut orphans: Arc<metamodelica::List<i32>>, mut index: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut imark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orowmarks: metamodelica::Array<i32>, mut ocolummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vec1: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vec2: metamodelica::Array<i32>, mut queuemark: metamodelica::Array<bool>, mut vars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>, mut size: i32) -> Result<i32> {
    let mut outMark: i32 = 0;
    outMark = 'mc: {
        let __mc_input = (orphans.clone(), index.clone(), m.clone(), mT.clone(), imark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone(), queuemark.clone(), vars.clone(), eqns.clone(), shared.clone(), size.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(imark.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: vorphan, tail: rest }, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut eorphan: i32 = 0;
                    let mut index1: i32 = 0;
                    let mut mark: i32 = 0;
                    let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut queue: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rqueue: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut bvars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut beqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vorphans: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vorphanseqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut queuelst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let true = (intEq(orowmarks.borrow()[(vorphan.clone()-1) as usize].clone(), 1)) else { bail!("pattern mismatch") };
                    eorphan = ass1.borrow()[(vorphan.clone()-1) as usize].clone();
                    vorphans = ass2.borrow()[(eorphan.clone()-1) as usize].clone();
                    rows = List::select(m.borrow()[(eorphan.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
                    rows = List::fold1(ass2.borrow()[(eorphan.clone()-1) as usize].clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rows.clone());
                    let _ = getIndexSubGraph(rows.clone(), vorphans.clone(), m.clone(), mT.clone(), imark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false)?;
                    vorphanseqns = List::unique(List::flatten(List::map1r(vorphans.clone(), Arc::new(arrayGet.clone()), mT.clone())));
                    queuelst = getIndexQueque(vorphanseqns.clone(), m.clone(), mT.clone(), imark.clone(), rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                    queue = List::flatten(queuelst.clone());
                    mark = imark.clone() + 2;
                    (index1, queue, rqueue) = List::fold1(queue.clone(), (std::sync::Arc::new(setIndexQueue) as std::sync::Arc<dyn ::std::ops::Fn(i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<bool>, metamodelica::Array<i32>, i32), (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), (vec1.clone(), vec2.clone(), ass2.clone(), queuemark.clone(), colummarks.clone(), mark.clone()), (index.clone(), metamodelica::nil(), metamodelica::nil()));
                    {let _arr = vec1.clone(); _arr.borrow_mut()[(index1.clone()-1) as usize] = vorphans.clone(); _arr};
                    {let _arr = vec2.clone(); _arr.borrow_mut()[(index1.clone()-1) as usize] = eorphan.clone(); _arr};
                    {let _arr = queuemark.clone(); _arr.borrow_mut()[(eorphan.clone()-1) as usize] = true; _arr};
                    mark = mark.clone() + 1;
                    List::map2_0(rqueue.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
                    List::map2_0(queue.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), colummarks.clone(), mark.clone());
                    bvars = getBorderElements(queue.clone(), m.clone(), mark.clone(), rowmarks.clone(), metamodelica::nil())?;
                    bvars = List::fold1(vorphans.clone(), std::sync::Arc::new(fnptr!(List::removeOnTrue, _, _, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), bvars.clone());
                    beqns = getBorderElements(rqueue.clone(), mT.clone(), mark.clone(), colummarks.clone(), metamodelica::nil())?;
                    beqns = List::removeOnTrue(eorphan.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), beqns.clone());
                    lst = List::select2(m.borrow()[(eorphan.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), rowmarks.clone(), mark.clone());
                    lst = listAppend(vorphans.clone(), listAppend(lst.clone(), bvars.clone()));
                    {let _arr = m.clone(); _arr.borrow_mut()[(eorphan.clone()-1) as usize] = lst.clone(); _arr};
                    lst = List::select2(vorphanseqns.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), colummarks.clone(), mark.clone());
                    lst = listAppend(cons(eorphan.clone(), lst.clone()), beqns.clone());
                    {let _arr = mT.clone(); _arr.borrow_mut()[(vorphan.clone()-1) as usize] = lst.clone(); _arr};
                    setBoarderElemts(bvars.clone(), mT.clone(), mark.clone(), colummarks.clone(), eorphan.clone())?;
                    setBoarderElemts(beqns.clone(), m.clone(), mark.clone(), rowmarks.clone(), vorphan.clone())?;
                    let _ = List::fold1(vorphans.clone(), (std::sync::Arc::new(markOrphans) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), -1, orowmarks.clone());
                    {let _arr = ocolummarks.clone(); _arr.borrow_mut()[(eorphan.clone()-1) as usize] = -1; _arr};
                    vorphans = List::removeOnTrue(vorphan.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), vorphans.clone());
                    let _ = List::fold1(vorphans.clone(), (std::sync::Arc::new(markOrphans) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), -1, orowmarks.clone());
                    let _ = List::fold1r(vorphans.clone(), Arc::new(arrayUpdate.clone()), metamodelica::nil(), mT.clone());
                    Ok(getIndexesForEqnsAdvanced(rest.clone(), index1.clone() + 1, m.clone(), mT.clone(), mark.clone() + 2, rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone(), queuemark.clone(), vars.clone(), eqns.clone(), shared.clone(), size.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(getIndexesForEqnsAdvanced(rest.clone(), index.clone(), m.clone(), mT.clone(), imark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone(), queuemark.clone(), vars.clone(), eqns.clone(), shared.clone(), size.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMark)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getBorderElements(mut elements: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut arr: metamodelica::Array<i32>, mut iAcc: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAcc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oAcc = (::match_deref::match_deref! { match &((elements.clone(), m.clone(), mark.clone(), arr.clone(), iAcc.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            iAcc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: elem, tail: rest }, _, _, _, _) => {
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut lst1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (lst, lst1) = List::split2OnTrue(m.borrow()[(elem.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), arr.clone(), mark.clone());
            {let _arr = m.clone(); _arr.borrow_mut()[(elem.clone()-1) as usize] = lst1.clone(); _arr};
            lst = List::select2(lst.clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), arr.clone(), mark.clone() + 1);
            List::map2_0(lst.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), arr.clone(), mark.clone() + 1);
            lst = getBorderElements(rest.clone(), m.clone(), mark.clone(), arr.clone(), listAppend(lst.clone(), iAcc.clone()))?;
            lst.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oAcc)
}

fn setBoarderElemts(mut elements: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut arr: metamodelica::Array<i32>, mut orphan: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((elements.clone(), m.clone(), mark.clone(), arr.clone(), orphan.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: elem, tail: rest }, _, _, _, _) => {
            let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            lst = List::select2(m.borrow()[(elem.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(unmarked, i32, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), arr.clone(), mark.clone());
            {let _arr = m.clone(); _arr.borrow_mut()[(elem.clone()-1) as usize] = cons(orphan.clone(), lst.clone()); _arr};
            setBoarderElemts(rest.clone(), m.clone(), mark.clone(), arr.clone(), orphan.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn setIndexQueue(mut col: i32, mut tpl: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<bool>, metamodelica::Array<i32>, i32), mut itpl: (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut otpl: (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    otpl = 'mc: {
        let __mc_input = (col.clone(), tpl.clone(), itpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (vec1, vec2, ass2, queuemark, colummark, mark), (index, elst, rlst)) => {
                    let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    r = ass2.borrow()[(col.clone()-1) as usize].clone();
                    let false = (queuemark.borrow()[(col.clone()-1) as usize].clone()) else { bail!("pattern mismatch") };
                    {let _arr = vec1.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = r.clone(); _arr};
                    {let _arr = vec2.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = col.clone(); _arr};
                    {let _arr = queuemark.clone(); _arr.borrow_mut()[(col.clone()-1) as usize] = true; _arr};
                    {let _arr = colummark.clone(); _arr.borrow_mut()[(col.clone()-1) as usize] = mark.clone(); _arr};
                    Ok((index.clone() + 1, cons(col.clone(), elst.clone()), listAppend(r.clone(), rlst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, _, ass2, _, colummark, mark), (index, elst, rlst)) => {
                    let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    r = ass2.borrow()[(col.clone()-1) as usize].clone();
                    let false = (intEq(colummark.borrow()[(col.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    {let _arr = colummark.clone(); _arr.borrow_mut()[(col.clone()-1) as usize] = mark.clone(); _arr};
                    Ok((index.clone(), cons(col.clone(), elst.clone()), listAppend(r.clone(), rlst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(itpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(otpl)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getIndexQueque(mut colums: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vec2: metamodelica::Array<i32>, mut queuemark: metamodelica::Array<bool>, mut nextqueue: Arc<metamodelica::List<i32>>, mut iqueue: Arc<metamodelica::List<i32>>, mut iqueue1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut oqueue: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    oqueue = (::match_deref::match_deref! { match &((colums.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), nextqueue.clone(), iqueue.clone(), iqueue1.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil, _, _) => {
            iqueue1.clone()
        },
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut queue: Arc<metamodelica::List<i32>> = metamodelica::nil();
            queue = List::unique(iqueue.clone());
            getIndexQueque(nextqueue.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), metamodelica::nil(), metamodelica::nil(), cons(queue.clone(), iqueue1.clone()))?
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _, _, _, _, _, _, _, _, _, _, _, _) => {
            let mut queue: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut r: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut queue1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut colums1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut b1: bool = false;
            let mut b2: bool = false;
            r = ass2.borrow()[(c.clone()-1) as usize].clone();
            (colums1, b2) = getIndexQueque1(r.clone(), c.clone(), mT.clone(), mark.clone(), rowmarks.clone());
            b1 = !(colums.clone().is_empty());
            queue = if (b1.clone()) {List::unionOnTrue(colums1.clone(), nextqueue.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))} else {nextqueue.clone()};
            queue1 = List::consOnTrue(b2.clone(), c.clone(), iqueue.clone());
            getIndexQueque(rest.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec2.clone(), queuemark.clone(), queue.clone(), queue1.clone(), iqueue1.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oqueue)
}

fn getIndexQueque1(mut rows: Arc<metamodelica::List<i32>>, mut c: i32, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>) -> (Arc<metamodelica::List<i32>>, bool) {
    let mut ocolums: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ob: bool = false;
    let mut colums: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut r in &*rows.clone() {
        let mut r = r.clone();
        if intEq(rowmarks.borrow()[(r.clone()-1) as usize].clone(), mark.clone()) {
            ob = true;
            colums = List::select(mT.borrow()[(r.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
            colums = List::removeOnTrue(c.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), colums.clone());
            ocolums = listAppend(colums.clone(), ocolums.clone());
        }
    }
    ocolums = List::unique(ocolums.clone());
    (ocolums, ob)
}

fn unmarked(mut indx: i32, mut markarray: metamodelica::Array<i32>, mut mark: i32) -> bool {
    let mut b: bool = false;
    b = intNe(markarray.borrow()[(indx.clone()-1) as usize].clone(), mark.clone());
    b
}

fn marked(mut indx: i32, mut markarray: metamodelica::Array<i32>, mut mark: i32) -> bool {
    let mut b: bool = false;
    b = intEq(markarray.borrow()[(indx.clone()-1) as usize].clone(), mark.clone());
    b
}

fn isOrphan(mut indx: i32, mut ass: metamodelica::Array<i32>) -> bool {
    let mut b: bool = false;
    b = intLt(ass.borrow()[(indx.clone()-1) as usize].clone(), 1);
    b
}

fn isNoOrphan(mut indx: i32, mut ass: metamodelica::Array<i32>) -> bool {
    let mut b: bool = false;
    b = intGt(ass.borrow()[(indx.clone()-1) as usize].clone(), 0);
    b
}

fn isResOrphan(mut indx: i32, mut ass: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> bool {
    let mut b: bool = false;
    b = ass.borrow()[(indx.clone()-1) as usize].clone().is_empty();
    b
}

fn isNoResOrphan(mut indx: i32, mut ass: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> bool {
    let mut b: bool = false;
    b = !(ass.borrow()[(indx.clone()-1) as usize].clone().is_empty());
    b
}

fn doAssign(mut index: i32, mut arr: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut assign: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = {let _arr = arr.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = assign.clone(); _arr};
    Ok(())
}

fn doMark(mut index: i32, mut arr: metamodelica::Array<i32>, mut mark: i32) -> Result<()> {
    let _ = {let _arr = arr.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = mark.clone(); _arr};
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getIndexSubGraph(mut rows: Arc<metamodelica::List<i32>>, mut vorphan: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>, mut colummarks: metamodelica::Array<i32>, mut orowmarks: metamodelica::Array<i32>, mut ocolummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ifound: bool) -> Result<bool> {
    let mut found: bool = false;
    found = 'mc: {
        let __mc_input = (rows.clone(), vorphan.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), ifound.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(ifound.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) => {
                    let true = (listMember(r.clone(), vorphan.clone())) else { bail!("pattern mismatch") };
                    let _ = getIndexSubGraph(rest.clone(), vorphan.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false)?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut e: i32 = 0;
                    let false = (listMember(r.clone(), vorphan.clone())) else { bail!("pattern mismatch") };
                    let false = (intEq(orowmarks.borrow()[(r.clone()-1) as usize].clone(), 1)) else { bail!("pattern mismatch") };
                    let true = (intEq(rowmarks.borrow()[(r.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    e = ass1.borrow()[(r.clone()-1) as usize].clone();
                    List::map2_0(ass2.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
                    Ok(getIndexSubGraph(rest.clone(), vorphan.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut e: i32 = 0;
                    let mut nextrows: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut b: bool = false;
                    let false = (listMember(r.clone(), vorphan.clone())) else { bail!("pattern mismatch") };
                    let false = (intEq(orowmarks.borrow()[(r.clone()-1) as usize].clone(), 1)) else { bail!("pattern mismatch") };
                    e = ass1.borrow()[(r.clone()-1) as usize].clone();
                    let false = (intEq(ocolummarks.borrow()[(e.clone()-1) as usize].clone(), 1)) else { bail!("pattern mismatch") };
                    let false = (intEq(colummarks.borrow()[(e.clone()-1) as usize].clone(), mark.clone())) else { bail!("pattern mismatch") };
                    nextrows = List::select(m.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(Util::intPositive, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>));
                    nextrows = List::setDifferenceOnTrue(nextrows.clone(), ass2.borrow()[(e.clone()-1) as usize].clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    {let _arr = colummarks.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = mark.clone(); _arr};
                    b = getIndexSubGraph(nextrows.clone(), vorphan.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), false)?;
                    markIndexSubgraph(b.clone(), ass2.borrow()[(e.clone()-1) as usize].clone(), mark.clone(), rowmarks.clone())?;
                    Ok(getIndexSubGraph(rest.clone(), vorphan.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), b.clone() || ifound.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _, _, _, _, _, _, _, _, _) => {
                    Ok(getIndexSubGraph(rest.clone(), vorphan.clone(), m.clone(), mT.clone(), mark.clone(), rowmarks.clone(), colummarks.clone(), orowmarks.clone(), ocolummarks.clone(), ass1.clone(), ass2.clone(), ifound.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(found)
}

fn markIndexSubgraph(mut b: bool, mut r: Arc<metamodelica::List<i32>>, mut mark: i32, mut rowmarks: metamodelica::Array<i32>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((b.clone(), r.clone(), mark.clone(), rowmarks.clone())) {
        (false, _, _, _) => (),
        (true, _, _, _) => {
            List::map2_0(r.clone(), (std::sync::Arc::new(doMark) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, i32) -> Result<()> + 'static>), rowmarks.clone(), mark.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn getIndexesForEqnsRest(mut i: i32, mut size: i32, mut id: i32, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut vec1: metamodelica::Array<i32>, mut vec2: metamodelica::Array<i32>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (i.clone(), size.clone(), id.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(i.clone(), size.clone())) else { bail!("pattern mismatch") };
            let true = (intEq(mark.clone(), colummarks.borrow()[(i.clone()-1) as usize].clone())) else { bail!("pattern mismatch") };
            getIndexesForEqnsRest(i.clone() + 1, size.clone(), id.clone(), mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(i.clone(), size.clone())) else { bail!("pattern mismatch") };
            {let _arr = vec1.clone(); let _val = ass2.borrow()[(i.clone()-1) as usize].clone(); _arr.borrow_mut()[(id.clone()-1) as usize] = _val; _arr};
            {let _arr = vec2.clone(); _arr.borrow_mut()[(id.clone()-1) as usize] = i.clone(); _arr};
            getIndexesForEqnsRest(i.clone() + 1, size.clone(), id.clone() + 1, mark.clone(), colummarks.clone(), ass1.clone(), ass2.clone(), vec1.clone(), vec2.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn markIndexdColums(mut i: i32, mut size: i32, mut mark: i32, mut colummarks: metamodelica::Array<i32>, mut vec2: metamodelica::Array<i32>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (i.clone(), size.clone(), mark.clone(), colummarks.clone(), vec2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(i.clone(), size.clone())) else { bail!("pattern mismatch") };
            let true = (intGt(vec2.borrow()[(i.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
            {let _arr = colummarks.clone(); _arr.borrow_mut()[(vec2.borrow()[(i.clone()-1) as usize].clone()-1) as usize] = mark.clone(); _arr};
            markIndexdColums(i.clone() + 1, size.clone(), mark.clone(), colummarks.clone(), vec2.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, _, _, _) = __mc_input.clone() else { bail!("nomatch") };
            let false = (intGt(i.clone(), size.clone())) else { bail!("pattern mismatch") };
            markIndexdColums(i.clone() + 1, size.clone(), mark.clone(), colummarks.clone(), vec2.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getOrphans(mut indx: i32, mut size: i32, mut ass: metamodelica::Array<i32>, mut inOrphans: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outOrphans: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outOrphans = 'mc: {
        let __mc_input = (indx.clone(), size.clone(), ass.clone(), inOrphans.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let true = (intGt(indx.clone(), size.clone())) else { bail!("pattern mismatch") };
                    Ok(inOrphans.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut orphans: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    orphans = List::consOnTrue(intLt(ass.borrow()[(indx.clone()-1) as usize].clone(), 1), indx.clone(), inOrphans.clone());
                    Ok(getOrphans(indx.clone() + 1, size.clone(), ass.clone(), orphans.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outOrphans)
}

fn expHasCref(mut inExp: Arc<DAE::Exp>, mut cr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut isthere: bool = false;
    let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    set = HashSet::emptyHashSet();
    set = addCrefandParentsToSet(cr.clone(), set.clone(), None)?;
    let (_, (_, __pa0)) = Expression::traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(expHasCreftraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool))> + 'static>), (set.clone(), false))?;
    isthere = __pa0.clone();
    Ok(isthere)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn addCrefandParentsToSet(mut inCref: Arc<DAE::ComponentRef>, mut ihs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), mut oprecr: Option<Arc<DAE::ComponentRef>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr))> {
    let mut ohs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    ohs = (::match_deref::match_deref! { match &((inCref.clone(), ihs.clone(), oprecr.clone())) {
        (cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _, None) => {
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            crlst = ComponentReference::expandCref(cr.clone(), true)?;
            set = List::fold(cons(cr.clone(), crlst.clone()), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ihs.clone());
            set.clone()
        },
        (cr @ Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _, Some(precr)) => {
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            crlst = ComponentReference::expandCref(cr.clone(), true)?;
            crlst = List::map1r(cons(cr.clone(), crlst.clone()), (std::sync::Arc::new(ComponentReference::joinCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), precr.clone());
            set = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ihs.clone());
            set.clone()
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: subcr, subscriptLst, identType: ty, ident }, _, None) => {
            let mut idcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            idcr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
            set = BaseHashSet::add(idcr.clone(), ihs.clone())?;
            idcr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), subscriptLst.clone());
            set = BaseHashSet::add(idcr.clone(), set.clone())?;
            addCrefandParentsToSet(subcr.clone(), set.clone(), Some(idcr.clone()))?
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: subcr, subscriptLst, identType: ty, ident }, _, Some(precr)) => {
            let mut idcr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut precr = (*precr).clone();
            idcr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), metamodelica::nil());
            idcr = ComponentReference::joinCrefs(precr.clone(), idcr.clone())?;
            set = BaseHashSet::add(idcr.clone(), ihs.clone())?;
            idcr = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), ty.clone(), subscriptLst.clone());
            precr = ComponentReference::joinCrefs(precr.clone(), idcr.clone())?;
            set = BaseHashSet::add(precr.clone(), ihs.clone())?;
            addCrefandParentsToSet(subcr.clone(), set.clone(), Some(precr.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ohs)
}

fn expHasCreftraverser(mut e: Arc<DAE::Exp>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = false;
    let mut outTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (e.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (set, false)) => {
                    let mut b: bool = false;
                    b = BaseHashSet::has(cr.clone(), set.clone())?;
                    Ok((e.clone(), !(b.clone()), (set.clone(), b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (set, b)) => {
                    Ok((e.clone(), !(b.clone()), (set.clone(), b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, cont, outTpl))
}

fn assignLst(mut vlst: Arc<metamodelica::List<i32>>, mut e: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((vlst.clone(), e.clone(), ass1.clone(), ass2.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: v, tail: rest }, _, _, _) => {
            {let _arr = ass1.clone(); _arr.borrow_mut()[(v.clone()-1) as usize] = e.clone(); _arr};
            {let _arr = ass2.clone(); _arr.borrow_mut()[(e.clone()-1) as usize] = v.clone(); _arr};
            assignLst(rest.clone(), e.clone() + 1, ass1.clone(), ass2.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn unassignedLst(mut vlst: Arc<metamodelica::List<i32>>, mut ass1: metamodelica::Array<i32>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((vlst.clone(), ass1.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: v, tail: rest }, _) => {
            let false = (intGt(ass1.borrow()[(v.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
            unassignedLst(rest.clone(), ass1.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn onefreeMatchingBFS(mut queue: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut size: i32, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut columark: metamodelica::Array<i32>, mut mark: i32, mut nextQeue: Arc<metamodelica::List<i32>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((queue.clone(), m.clone(), mt.clone(), size.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone(), nextQeue.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, _, _, _) => {
            onefreeMatchingBFS(nextQeue.clone(), m.clone(), mt.clone(), size.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone(), metamodelica::nil())?;
            ()
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: rest }, _, _, _, _, _, _, _, _) => {
            let mut newqueue: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut rows: Arc<metamodelica::List<i32>> = metamodelica::nil();
            rows = List::removeOnTrue(ass1.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), m.borrow()[(c.clone()-1) as usize].clone());
            newqueue = onefreeMatchingBFS1(rows.clone(), c.clone(), mt.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone(), nextQeue.clone())?;
            onefreeMatchingBFS(rest.clone(), m.clone(), mt.clone(), size.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone(), newqueue.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn isAssignedSaveEnhanced(mut ass: metamodelica::Array<i32>, mut inTpl: i32) -> bool {
    let mut outB: bool = false;
    outB = if (intGt(inTpl.clone(), 0)) {intGt(ass.borrow()[(inTpl.clone()-1) as usize].clone(), 0)} else {true};
    outB
}

fn onefreeMatchingBFS1(mut rows: Arc<metamodelica::List<i32>>, mut c: i32, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>, mut columark: metamodelica::Array<i32>, mut mark: i32, mut inNextQeue: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outNextQeue: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outNextQeue = 'mc: {
        let __mc_input = (rows.clone(), c.clone(), mt.clone(), ass1.clone(), ass2.clone(), columark.clone(), mark.clone(), inNextQeue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: r, tail: Deref @ metamodelica::List::Nil }, _, _, _, _, _, _, _) => {
                    let mut vareqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    {let _arr = ass1.clone(); _arr.borrow_mut()[(r.clone()-1) as usize] = c.clone(); _arr};
                    {let _arr = ass2.clone(); _arr.borrow_mut()[(c.clone()-1) as usize] = r.clone(); _arr};
                    vareqns = List::removeOnTrue(ass2.clone(), (std::sync::Arc::new(fnptr!(isAssignedSaveEnhanced, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<i32>, i32) -> Result<bool> + 'static>), mt.borrow()[(r.clone()-1) as usize].clone());
                    Ok(listAppend(inNextQeue.clone(), vareqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNextQeue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNextQeue)
}

fn vectorMatching(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), vars.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e2, left: e1, dimSize: ds, .. }, _, _) => {
                    let mut size: i32 = 0;
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1);
                    tpl = vectorMatching1(e1.clone(), e2.clone(), size.clone(), vars.clone(), inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e1, left: e2, dimSize: ds, .. }, _, _) => {
                    let mut size: i32 = 0;
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    size = List::fold(ds.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1);
                    tpl = vectorMatching1(e2.clone(), e1.clone(), size.clone(), vars.clone(), inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e2, left: e1, size, .. }, _, _) => {
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    tpl = vectorMatching1(e1.clone(), e2.clone(), size.clone(), vars.clone(), inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e1, left: e2, size, .. }, _, _) => {
                    let mut tpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
                    tpl = vectorMatching1(e2.clone(), e1.clone(), size.clone(), vars.clone(), inTpl.clone())?;
                    Ok(tpl.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    let mut size: i32 = 0;
                    size = BackendEquation::equationSize(eqn.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn vectorMatching1(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>, mut size: i32, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (e1.clone(), e2.clone(), size.clone(), vars.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _, _, _, (id, vec1, vec2)) => {
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (expHasCref(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let true = (intEq(size.clone(), (ilst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _, _, (id, vec1, vec2)) => {
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (expHasCref(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let true = (intEq(size.clone(), (ilst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, _, _, _, (id, vec1, vec2)) => {
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (expHasCref(e2.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let true = (intEq(size.clone(), (ilst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, _, _, (id, vec1, vec2)) => {
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let false = (expHasCref(e1.clone(), cr.clone())?) else { bail!("pattern mismatch") };
                    (_, ilst) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    let true = (intEq(size.clone(), (ilst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, (id, vec1, vec2)) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crnosubs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crlst1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    elst = Expression::flattenArrayExpToList(e1.clone())?;
                    crlst = List::map(elst.clone(), (std::sync::Arc::new(Expression::expCrefNegCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    crlst = List::uniqueOnTrue(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>));
                    let true = (intEq(size.clone(), (crlst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crlst.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    crlst1 = __pa1.clone();
                    let true = (List::all(crlst1.clone(), Arc::new({ let __pe_b1 = cr.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqualWithoutLastSubs(__pe_a0, __pe_b1.clone()) }))) else { bail!("pattern mismatch") };
                    set = HashSet::emptyHashSet();
                    crnosubs = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    set = addCrefandParentsToSet(crnosubs.clone(), set.clone(), None)?;
                    set = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), set.clone());
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(expHasCreftraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool))> + 'static>), (set.clone(), false))?) {
                        (_, (_, false)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (_, ilst) = BackendVariable::getVarLst(crlst.clone(), vars.clone());
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, (id, vec1, vec2)) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crnosubs: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut crlst1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut ilst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut set: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    elst = Expression::flattenArrayExpToList(e2.clone())?;
                    crlst = List::map(elst.clone(), (std::sync::Arc::new(Expression::expCrefNegCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>));
                    crlst = List::uniqueOnTrue(crlst.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqualNoStringCompare) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>));
                    let true = (intEq(size.clone(), (crlst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crlst.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = __pa0.clone();
                    crlst1 = __pa1.clone();
                    let true = (List::all(crlst1.clone(), Arc::new({ let __pe_b1 = cr.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqualWithoutLastSubs(__pe_a0, __pe_b1.clone()) }))) else { bail!("pattern mismatch") };
                    set = HashSet::emptyHashSet();
                    crnosubs = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    set = addCrefandParentsToSet(crnosubs.clone(), set.clone(), None)?;
                    set = List::fold(crlst.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), set.clone());
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(expHasCreftraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool)) -> Result<(Arc<DAE::Exp>, bool, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), bool))> + 'static>), (set.clone(), false))?) {
                        (_, (_, false)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (_, ilst) = BackendVariable::getVarLst(crlst.clone(), vars.clone());
                    unassignedLst(ilst.clone(), vec1.clone())?;
                    assignLst(ilst.clone(), id.clone(), vec1.clone(), vec2.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn aliasMatching(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), vars.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, .. }, _, (id, vec1, vec2)) => {
                    let mut i: i32 = 0;
                    let mut i1: i32 = 0;
                    let mut i2: i32 = 0;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(vec2.borrow()[(id.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
                    (_, i1) = BackendVariable::getVarSingle(cr1.clone(), vars.clone())?;
                    (_, i2) = BackendVariable::getVarSingle(cr2.clone(), vars.clone())?;
                    i = aliasMatching1(i1.clone(), i2.clone(), intGt(vec1.borrow()[(i1.clone()-1) as usize].clone(), 0), intGt(vec1.borrow()[(i2.clone()-1) as usize].clone(), 0))?;
                    vec1 = {let _arr = vec1.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = id.clone(); _arr};
                    vec2 = {let _arr = vec2.clone(); _arr.borrow_mut()[(id.clone()-1) as usize] = i.clone(); _arr};
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    let mut size: i32 = 0;
                    size = BackendEquation::equationSize(eqn.clone())?;
                    Ok((id.clone() + size.clone(), vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn aliasMatching1(mut i1: i32, mut i2: i32, mut b1: bool, mut b2: bool) -> Result<i32> {
    let mut i: i32 = 0;
    i = (match (i1.clone(), i2.clone(), b1.clone(), b2.clone()) {
        (_, _, false, true) => i1.clone(),
        (_, _, true, false) => i2.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(i)
}

fn naturalMatching(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), vars.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, _, (id, vec1, vec2)) => {
                    let mut i: i32 = 0;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(vec2.borrow()[(id.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    let false = (intGt(vec1.borrow()[(i.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = {let _arr = vec1.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = id.clone(); _arr};
                    vec2 = {let _arr = vec2.clone(); _arr.borrow_mut()[(id.clone()-1) as usize] = i.clone(); _arr};
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn naturalMatching1(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), vars.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, _, (id, vec1, vec2)) => {
                    let mut i: i32 = 0;
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(vec2.borrow()[(id.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    let false = (intGt(vec1.borrow()[(i.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
                    vec1 = {let _arr = vec1.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = id.clone(); _arr};
                    vec2 = {let _arr = vec2.clone(); _arr.borrow_mut()[(id.clone()-1) as usize] = i.clone(); _arr};
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn naturalMatching2(mut eqn: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut inTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut outTpl: (i32, metamodelica::Array<i32>, metamodelica::Array<i32>);
    outTpl = 'mc: {
        let __mc_input = (eqn.clone(), vars.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { scalar: e2, exp: e1, .. }, _, (id, vec1, vec2)) => {
                    let mut i: i32 = 0;
                    let mut e: Arc<DAE::Exp>;
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut vec1 = (*vec1).clone();
                    let mut vec2 = (*vec2).clone();
                    let false = (intGt(vec2.borrow()[(id.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
                    e = Expression::expSub(e1.clone(), e2.clone())?;
                    vlst = BackendEquation::equationVars(eqn.clone(), vars.clone())?;
                    (_, i) = getConstOneVariable(vlst.clone(), e.clone(), vec1.clone(), vars.clone())?;
                    vec1 = {let _arr = vec1.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = id.clone(); _arr};
                    vec2 = {let _arr = vec2.clone(); _arr.borrow_mut()[(id.clone()-1) as usize] = i.clone(); _arr};
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, (id, vec1, vec2)) => {
                    Ok((id.clone() + 1, vec1.clone(), vec2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn getConstOneVariable(mut vlst: Arc<metamodelica::List<BackendDAE::Var>>, mut e: Arc<DAE::Exp>, mut vec1: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<(Arc<DAE::ComponentRef>, i32)> {
    let mut outCr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut i: i32 = 0;
    (outCr, i) = 'mc: {
        let __mc_input = (vlst.clone(), e.clone(), vec1.clone(), vars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: v, tail: _ }, _, _, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut i: i32 = i.clone();
                    cr = BackendVariable::varCref(v.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    let false = (intGt(vec1.borrow()[(i.clone()-1) as usize].clone(), 0)) else { bail!("pattern mismatch") };
                    e1 = Differentiate::differentiateExpSolve(e.clone(), cr.clone(), None)?;
                    (e2, _) = ExpressionSimplify::simplify(e1.clone())?;
                    let true = (Expression::isConstOne(e2.clone()) || Expression::isConstMinusOne(e2.clone())) else { bail!("pattern mismatch") };
                    Ok((cr.clone(), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _, _, _) => {
                    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut i: i32 = i.clone();
                    (cr, i) = getConstOneVariable(rest.clone(), e.clone(), vec1.clone(), vars.clone())?;
                    Ok((cr.clone(), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCr, i))
}

