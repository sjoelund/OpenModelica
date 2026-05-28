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

use crate::BackendDAE;
use crate::BackendDAEUtil;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::VisualXMLTpl;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_program_util::ProgramUtil;
use openmodelica_susan::Tpl;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

//----------------------------
//  Visualization types
//----------------------------
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Visualization {
    SHAPE {
        ident: Arc<DAE::ComponentRef>,
        shapeType: Arc<DAE::Exp>,
        T: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>,
        r: metamodelica::Array<Arc<DAE::Exp>>,
        r_shape: metamodelica::Array<Arc<DAE::Exp>>,
        lengthDir: metamodelica::Array<Arc<DAE::Exp>>,
        widthDir: metamodelica::Array<Arc<DAE::Exp>>,
        length: Arc<DAE::Exp>,
        width: Arc<DAE::Exp>,
        height: Arc<DAE::Exp>,
        extra: Arc<DAE::Exp>,
        color: metamodelica::Array<Arc<DAE::Exp>>,
        specularCoeff: Arc<DAE::Exp>,
    },
    VECTOR {
        ident: Arc<DAE::ComponentRef>,
        T: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>,
        r: metamodelica::Array<Arc<DAE::Exp>>,
        coordinates: metamodelica::Array<Arc<DAE::Exp>>,
        color: metamodelica::Array<Arc<DAE::Exp>>,
        specularCoeff: Arc<DAE::Exp>,
        quantity: Arc<DAE::Exp>,
        headAtOrigin: Arc<DAE::Exp>,
        twoHeadedArrow: Arc<DAE::Exp>,
    },
    SURFACE {
        ident: Arc<DAE::ComponentRef>,
        T: metamodelica::Array<Arc<metamodelica::List<Arc<DAE::Exp>>>>,
        r_0: metamodelica::Array<Arc<DAE::Exp>>,
        nu: Arc<DAE::Exp>,
        nv: Arc<DAE::Exp>,
        wireframe: Arc<DAE::Exp>,
        multiColored: Arc<DAE::Exp>,
        color: metamodelica::Array<Arc<DAE::Exp>>,
        specularCoeff: Arc<DAE::Exp>,
        transparency: Arc<DAE::Exp>,
    },
}
pub use self::Visualization::{SHAPE,VECTOR,SURFACE};

//-------------------------
// dump visualization xml
//-------------------------
pub fn visualizationInfoXML(mut daeIn: Arc<BackendDAE::BackendDAE>, mut fileName: ArcStr, mut program: Absyn::Program) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut daeOut: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut eqs0: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut aliasVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut allVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut aliasVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut visuals: Arc<metamodelica::List<Visualization>> = metamodelica::nil();
    let mut allVisuals: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(daeIn.clone()) {
        Deref @ DAE { shared: __pa0, eqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    shared = __pa0.clone();
    eqs0 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { aliasVars: __pa2, globalKnownVars: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    aliasVars = __pa2.clone();
    globalKnownVars = __pa3.clone();
    eqs = List::map(eqs0.clone(), (std::sync::Arc::new(BackendDAEUtil::copyEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>));
    eqs = List::map(eqs.clone(), (std::sync::Arc::new(fnptr!(setBindingForProtectedVars, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>));
    globalKnownVarLst = BackendVariable::varList(globalKnownVars.clone())?;
    aliasVarLst = BackendVariable::varList(aliasVars.clone())?;
    allVarLst = List::flatten(List::mapMap(eqs.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::daeVars, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<BackendDAE::Variables> + 'static>), (std::sync::Arc::new(BackendVariable::varList) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>)));
    (globalKnownVarLst, allVisuals) = List::fold(globalKnownVarLst.clone(), (std::sync::Arc::new(isVisualizationVarFold) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>)> + 'static>), (metamodelica::nil(), metamodelica::nil()));
    (allVarLst, allVisuals) = List::fold(allVarLst.clone(), (std::sync::Arc::new(isVisualizationVarFold) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>)> + 'static>), (metamodelica::nil(), allVisuals.clone()));
    (aliasVarLst, allVisuals) = List::fold(aliasVarLst.clone(), (std::sync::Arc::new(isVisualizationVarFold) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>)> + 'static>), (metamodelica::nil(), allVisuals.clone()));
    allVarLst = listAppend(globalKnownVarLst.clone(), listAppend(allVarLst.clone(), aliasVarLst.clone()));
    (visuals, _, _) = List::mapFold2(allVisuals.clone(), (std::sync::Arc::new(fillVisualizationObjects) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, ArcStr), Arc<metamodelica::List<BackendDAE::Var>>, Absyn::Program) -> Result<(Visualization, Arc<metamodelica::List<BackendDAE::Var>>, Absyn::Program)> + 'static>), allVarLst.clone(), program.clone());
    visuals = List::map2(visuals.clone(), (std::sync::Arc::new(replaceVisualBinding) as std::sync::Arc<dyn ::std::ops::Fn(Visualization, BackendDAE::Variables, Absyn::Program) -> Result<Visualization> + 'static>), globalKnownVars.clone(), program.clone());
    dumpVis(metamodelica::arrayFromVec(visuals.clone().into_iter().cloned().collect()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!("_visual.xml")); ArcStr::from(__mm_s) }).clone())?;
    (globalKnownVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), (std::sync::Arc::new(setVisVarsPublic) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArcStr) -> Result<(BackendDAE::Var, ArcStr)> + 'static>), (literal!("")).clone())?;
    (aliasVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(aliasVars.clone(), (std::sync::Arc::new(setVisVarsPublic) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, ArcStr) -> Result<(BackendDAE::Var, ArcStr)> + 'static>), (literal!("")).clone())?;
    daeOut = BackendDAE::DAE(shared=shared.clone(), eqs=eqs.clone())?;
    Ok(daeOut)
}

fn replaceVisualBinding(mut vis: Visualization, mut varArray: BackendDAE::Variables, mut program: Absyn::Program) -> Result<Visualization> {
    let mut vis: Visualization = vis;
    let () = 'mc: {
        let __mc_input = vis.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Visualization::SHAPE { shapeType: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. } => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_shapeType_0 = getConstCrefBinding(cr.clone(), varArray.clone())?;
                    if let Visualization::SHAPE { shapeType, .. } = &mut vis {
                        *shapeType = __owned_variant_shapeType_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Visualization::SHAPE { shapeType: Deref @ DAE::Exp::SCONST { string: s }, .. } => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_shapeType_0 = Arc::new(DAE::Exp::SCONST { string: (getFullCADFilePath((s.clone()).clone(), program.clone())?).clone() });
                    if let Visualization::SHAPE { shapeType, .. } = &mut vis {
                        *shapeType = __owned_variant_shapeType_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(vis)
}

fn getConstCrefBinding(mut cr: Arc<DAE::ComponentRef>, mut vars: BackendDAE::Variables) -> Result<Arc<DAE::Exp>> {
    let mut eOut: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendVariable::getVar(cr.clone(), vars.clone()), '__try0)) {
            (Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil }, _) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        var = __pa1.clone();
        e = unwrap_break_err!(BackendVariable::varBindExp(var.clone()), '__try0);
        eOut = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((unwrap_break_err!(Expression::isConst(e.clone()), '__try0))) { bail!("guard") }
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: _, .. } => {
                    Ok(unwrap_break_err!(getConstCrefBinding(Expression::expCref(e.clone())?, vars.clone()), '__try0))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    unwrap_break_err!(Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The binding expression ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!(" of the visualization type component ")); __mm_s.push_str(&*ComponentReference::crefStr(cr.clone())?); __mm_s.push_str(&*literal!("  cannot be evaluated. Please specify a visualization type (CAD files are specified as modelica://packagename/filename.stl)")); ArcStr::from(__mm_s) }).clone()), '__try0);
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        Ok::<_, anyhow::Error>((e.clone(), eOut.clone(), var.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            e = __try0_o0;
            eOut = __try0_o1;
            var = __try0_o2;
        }
        Err(_) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("VisualXMl.getConstCrefBinding failed for ")); __mm_s.push_str(&*ComponentReference::crefStr(cr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok(eOut)
}

fn setVisVarsPublic(mut inVar: BackendDAE::Var, mut dummyArgIn: ArcStr) -> Result<(BackendDAE::Var, ArcStr)> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut dummyArgOut: ArcStr = dummyArgIn.clone();
    if isVisualizationVar(inVar.clone())? {
        outVar = makeVarPublicHideResultFalse(inVar.clone())?;
    }
    Ok((outVar, dummyArgOut))
}

fn makeVarPublicHideResultFalse(mut inVar: BackendDAE::Var) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut vals: Option<Arc<DAE::VariableAttributes>> = None;
    vals = inVar.values.clone();
    vals = DAEUtil::setProtectedAttr(vals.clone(), false)?;
    outVar = BackendVariable::setVarAttributes(inVar.clone(), vals.clone());
    outVar = BackendVariable::setHideResult(outVar.clone(), Some(Arc::new(DAE::Exp::BCONST { bool: false })));
    Ok(outVar)
}

fn setBindingForProtectedVars(mut eqSysIn: Arc<BackendDAE::EqSystem>) -> Arc<BackendDAE::EqSystem> {
    let mut eqSysOut: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut ass1: metamodelica::Array<i32>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    if '__try0: {
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eqSysIn.clone()) {
            Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa1, .. }, orderedVars: __pa2, orderedEqs: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        ass1 = __pa1.clone();
        vars = __pa2.clone();
        eqs = __pa3.clone();
        unwrap_break_err!(BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(setBindingForProtectedVars1) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, metamodelica::Array<i32>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (i32, metamodelica::Array<i32>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (1, ass1.clone(), eqs.clone())), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    eqSysOut = eqSysIn.clone();
    eqSysOut
}

fn setBindingForProtectedVars1(mut varIn: BackendDAE::Var, mut tplIn: (i32, metamodelica::Array<i32>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (i32, metamodelica::Array<i32>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut varOut: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut tplOut: (i32, metamodelica::Array<i32>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    (varOut, tplOut) = 'mc: {
        let __mc_input = (varIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { values: Some(_), bindExp: None, .. }, (idx, ass1, eqs)) => {
                    if !((BackendVariable::isProtectedVar(varIn.clone()) && isVisualizationVar(varIn.clone())?)) { bail!("guard") }
                    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut exp1: Arc<DAE::Exp>;
                    let mut exp2: Arc<DAE::Exp>;
                    eq = BackendEquation::get(eqs.clone(), ass1.clone().borrow()[(idx.clone()-1) as usize].clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eq.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, exp: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp2 = __pa0.clone();
                    exp1 = __pa1.clone();
                    (exp1, _) = ExpressionSolve::solve(exp1.clone(), exp2.clone(), BackendVariable::varExp(varIn.clone())?, None)?;
                    var = BackendVariable::setBindExp(varIn.clone(), Some(exp1.clone()));
                    var = makeVarPublicHideResultFalse(var.clone())?;
                    Ok((var.clone(), (idx.clone() + 1, ass1.clone(), eqs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (idx, ass1, eqs)) => {
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    if BackendVariable::isProtectedVar(varIn.clone()) && isVisualizationVar(varIn.clone())? {
                        var = makeVarPublicHideResultFalse(varIn.clone())?;
                    } else {
                        var = varIn.clone();
                    }
                    Ok((var.clone(), (idx.clone() + 1, ass1.clone(), eqs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((varOut, tplOut))
}

fn fillVisualizationObjects(mut visVar: (Arc<DAE::ComponentRef>, ArcStr), mut allVarsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut programIn: Absyn::Program) -> Result<(Visualization, Arc<metamodelica::List<BackendDAE::Var>>, Absyn::Program)> {
    let mut visOut: Visualization;
    let mut allVarsOut: Arc<metamodelica::List<BackendDAE::Var>> = allVarsIn.clone();
    let mut programOut: Absyn::Program = programIn.clone();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut vis_name: ArcStr = arcstr::literal!("");
    let mut vis: Visualization;
    match '__try0: {
        (cref, vis_name) = visVar.clone();
        vis = unwrap_break_err!(newVisualizer(cref.clone(), (vis_name.clone()).clone()), '__try0);
        (_, visOut) = List::fold2(allVarsIn.clone(), (std::sync::Arc::new(fillVisualizationObjects1) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool, Absyn::Program, (Arc<metamodelica::List<BackendDAE::Var>>, Visualization)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Visualization)> + 'static>), true, programIn.clone(), (metamodelica::nil(), vis.clone()));
        Ok::<_, anyhow::Error>((cref.clone(), vis.clone(), visOut.clone(), vis_name.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cref = __try0_o0;
            vis = __try0_o1;
            visOut = __try0_o2;
            vis_name = __try0_o3;
        }
        Err(_) => {
            println!("{}", (literal!("fillVisualizationObjects failed! - not yet supported type")).clone());
            bail!("fail");
        }
    }
    Ok((visOut, allVarsOut, programOut))
}

fn newVisualizer(mut cref: Arc<DAE::ComponentRef>, mut visualizerName: ArcStr) -> Result<Visualization> {
    let mut vis: Visualization;
    vis = (::match_deref::match_deref! { match &(visualizerName.clone()) {
        Deref @ "Shape" => Visualization::SHAPE { ident: cref.clone(), shapeType: Arc::new(DAE::Exp::SCONST { string: (literal!("DUMMY")).clone() }), T: arrayCreate(3, list![Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })]), r: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), r_shape: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), lengthDir: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), widthDir: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), length: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), width: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), height: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), extra: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), color: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), specularCoeff: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }) },
        Deref @ "Vector" => Visualization::VECTOR { ident: cref.clone(), T: arrayCreate(3, list![Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })]), r: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), coordinates: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), color: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), specularCoeff: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), quantity: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), headAtOrigin: Arc::new(DAE::Exp::BCONST { bool: false }), twoHeadedArrow: Arc::new(DAE::Exp::BCONST { bool: false }) },
        Deref @ "Surface" => Visualization::SURFACE { ident: cref.clone(), T: arrayCreate(3, list![Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })]), r_0: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), nu: Arc::new(DAE::Exp::ICONST { integer: -1 }), nv: Arc::new(DAE::Exp::ICONST { integer: -1 }), wireframe: Arc::new(DAE::Exp::BCONST { bool: false }), multiColored: Arc::new(DAE::Exp::BCONST { bool: false }), color: arrayCreate(3, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) })), specularCoeff: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }), transparency: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((-1) as f64) }) },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("VisualXML.newVisualizer")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*visualizerName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vis)
}

fn makeCrefQualFromString(mut s: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut crefOut: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut sLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    sLst = Util::stringSplitAtChar((s.clone()).clone(), (literal!(".")).clone())?;
    crefs = List::map2(sLst.clone(), (std::sync::Arc::new(fnptr!(ComponentReferenceBasics::makeCrefIdent, ArcStr, Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Subscript>>>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::Type>, Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::ComponentRef>> + 'static>), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cref = __pa0.clone();
    crefs = __pa1.clone();
    crefOut = List::foldr(crefs.clone(), (std::sync::Arc::new(ComponentReference::joinCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), cref.clone());
    Ok(crefOut)
}

fn splitCrefAfter(mut crefIn: Arc<DAE::ComponentRef>, mut crefCut: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::ComponentRef>, bool)> {
    let mut crefOut: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut wasCut: bool = false;
    (crefOut, wasCut) = 'mc: {
        let __mc_input = (crefIn.clone(), crefCut.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: crefIn1, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: crefCut1, .. }) => {
                    let true = (ComponentReferenceBasics::crefFirstCrefEqual(crefIn.clone(), crefCut.clone())?) else { bail!("pattern mismatch") };
                    Ok(splitCrefAfter(crefIn1.clone(), crefCut1.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: crefIn1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, .. }) => {
                    let true = (ComponentReferenceBasics::crefFirstCrefEqual(crefIn.clone(), crefCut.clone())?) else { bail!("pattern mismatch") };
                    Ok((crefIn1.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: crefIn1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, .. }) => {
                    let true = (!(ComponentReferenceBasics::crefFirstCrefEqual(crefIn.clone(), crefCut.clone())?)) else { bail!("pattern mismatch") };
                    Ok((crefIn1.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((crefCut.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((crefOut, wasCut))
}

fn fillVisualizationObjects1(mut varIn: BackendDAE::Var, mut storeProtectedCrefs: bool, mut program: Absyn::Program, mut tplIn: (Arc<metamodelica::List<BackendDAE::Var>>, Visualization)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Visualization)> {
    let mut tplOut: (Arc<metamodelica::List<BackendDAE::Var>>, Visualization);
    tplOut = 'mc: {
        let __mc_input = (varIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varName: cref, .. }, (vars, vis @ Visualization::SHAPE { ident, .. })) => {
                    let mut cref1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut filled_vis: Visualization;
                    let __pa0 = ::match_deref::match_deref! { match &(splitCrefAfter(cref.clone(), ident.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cref1 = __pa0.clone();
                    filled_vis = fillShapeObject(cref1.clone(), varIn.clone(), storeProtectedCrefs.clone(), program.clone(), vis.clone())?;
                    Ok((vars.clone(), filled_vis.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varName: cref, .. }, (vars, vis @ Visualization::VECTOR { ident, .. })) => {
                    let mut cref1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut filled_vis: Visualization;
                    let __pa0 = ::match_deref::match_deref! { match &(splitCrefAfter(cref.clone(), ident.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cref1 = __pa0.clone();
                    filled_vis = fillVectorObject(cref1.clone(), varIn.clone(), storeProtectedCrefs.clone(), program.clone(), vis.clone())?;
                    Ok((vars.clone(), filled_vis.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { varName: cref, .. }, (vars, vis @ Visualization::SURFACE { ident, .. })) => {
                    let mut cref1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut filled_vis: Visualization;
                    let __pa0 = ::match_deref::match_deref! { match &(splitCrefAfter(cref.clone(), ident.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cref1 = __pa0.clone();
                    filled_vis = fillSurfaceObject(cref1.clone(), varIn.clone(), storeProtectedCrefs.clone(), program.clone(), vis.clone())?;
                    Ok((vars.clone(), filled_vis.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut vis: Visualization;
                    (vars, vis) = tplIn.clone();
                    Ok((cons(varIn.clone(), vars.clone()), vis.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplOut)
}

fn getFullCADFilePath(mut sIn: ArcStr, mut program: Absyn::Program) -> Result<ArcStr> {
    let mut sOut: ArcStr = sIn.clone();
    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    chars = stringListStringChar((sIn.clone()).clone());
    if (chars.clone().len() as i32) > 11 && stringEqual(stringDelimitList(List::firstN(chars.clone(), 11)?, (literal!("")).clone()), (literal!("modelica://")).clone()) {
        sOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("file://")); __mm_s.push_str(&*ProgramUtil::getFullPathFromUri(program.clone(), (sIn.clone()).clone(), true)?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(sOut)
}

fn fillShapeObject(mut cref: Arc<DAE::ComponentRef>, mut var: BackendDAE::Var, mut storeProtectedCrefs: bool, mut program: Absyn::Program, mut vis: Visualization) -> Result<Visualization> {
    let mut vis: Visualization = vis;
    let () = 'mc: {
        let __mc_input = (cref.clone(), vis.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "shapeType", .. }, Visualization::SHAPE { .. }) => {
                    let mut bind: Option<Arc<DAE::Exp>> = None;
                    let mut vis: Visualization = vis.clone();
                    let BackendDAE::VAR { bindExp: __pa0, .. } = (var.clone()) else { bail!("pattern mismatch") };
                    bind = __pa0.clone();
                    if isSome(bind.clone()) {
                        let __owned_variant_shapeType_0 = Util::getOption(bind.clone())?;
                        if let Visualization::SHAPE { shapeType, .. } = &mut vis {
                            *shapeType = __owned_variant_shapeType_0;
                        } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos1 } }, tail: Deref @ metamodelica::List::Nil } }, ident: Deref @ "T", .. }, ident: Deref @ "R", .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut T0: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    T0 = var_field!(vis.T, Visualization::SHAPE).clone().borrow()[(pos.clone()-1) as usize].clone();
                    T0 = List::replaceAt(exp.clone(), pos1.clone(), T0.clone())?;
                    {let _arr = var_field!(vis.T, Visualization::SHAPE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = T0.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "r", .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.r, Visualization::SHAPE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "r_shape", .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.r_shape, Visualization::SHAPE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "lengthDirection", .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.lengthDir, Visualization::SHAPE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "widthDirection", .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.widthDir, Visualization::SHAPE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "length", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_length_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SHAPE { length, .. } = &mut vis {
                        *length = __owned_variant_length_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "width", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_width_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SHAPE { width, .. } = &mut vis {
                        *width = __owned_variant_width_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "height", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_height_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SHAPE { height, .. } = &mut vis {
                        *height = __owned_variant_height_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "extra", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_extra_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SHAPE { extra, .. } = &mut vis {
                        *extra = __owned_variant_extra_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "color", .. }, Visualization::SHAPE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.color, Visualization::SHAPE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "specularCoefficient", .. }, Visualization::SHAPE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_specularCoeff_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SHAPE { specularCoeff, .. } = &mut vis {
                        *specularCoeff = __owned_variant_specularCoeff_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SHAPE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(vis)
}

fn fillVectorObject(mut cref: Arc<DAE::ComponentRef>, mut var: BackendDAE::Var, mut storeProtectedCrefs: bool, mut program: Absyn::Program, mut vis: Visualization) -> Result<Visualization> {
    let mut vis: Visualization = vis;
    let () = 'mc: {
        let __mc_input = (cref.clone(), vis.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos1 } }, tail: Deref @ metamodelica::List::Nil } }, ident: Deref @ "T", .. }, ident: Deref @ "R", .. }, Visualization::VECTOR { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut T0: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    T0 = var_field!(vis.T, Visualization::VECTOR).clone().borrow()[(pos.clone()-1) as usize].clone();
                    T0 = List::replaceAt(exp.clone(), pos1.clone(), T0.clone())?;
                    {let _arr = var_field!(vis.T, Visualization::VECTOR).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = T0.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "r", .. }, Visualization::VECTOR { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.r, Visualization::VECTOR).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "coordinates", .. }, Visualization::VECTOR { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.coordinates, Visualization::VECTOR).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "color", .. }, Visualization::VECTOR { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.color, Visualization::VECTOR).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "specularCoefficient", .. }, Visualization::VECTOR { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_specularCoeff_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::VECTOR { specularCoeff, .. } = &mut vis {
                        *specularCoeff = __owned_variant_specularCoeff_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "quantity", .. }, Visualization::VECTOR { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_quantity_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::VECTOR { quantity, .. } = &mut vis {
                        *quantity = __owned_variant_quantity_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "headAtOrigin", .. }, Visualization::VECTOR { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_headAtOrigin_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::VECTOR { headAtOrigin, .. } = &mut vis {
                        *headAtOrigin = __owned_variant_headAtOrigin_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "twoHeadedArrow", .. }, Visualization::VECTOR { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_twoHeadedArrow_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::VECTOR { twoHeadedArrow, .. } = &mut vis {
                        *twoHeadedArrow = __owned_variant_twoHeadedArrow_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::VECTOR"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(vis)
}

fn fillSurfaceObject(mut cref: Arc<DAE::ComponentRef>, mut var: BackendDAE::Var, mut storeProtectedCrefs: bool, mut program: Absyn::Program, mut vis: Visualization) -> Result<Visualization> {
    let mut vis: Visualization = vis;
    let () = 'mc: {
        let __mc_input = (cref.clone(), vis.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos1 } }, tail: Deref @ metamodelica::List::Nil } }, ident: Deref @ "T", .. }, ident: Deref @ "R", .. }, Visualization::SURFACE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut T0: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    T0 = var_field!(vis.T, Visualization::SURFACE).clone().borrow()[(pos.clone()-1) as usize].clone();
                    T0 = List::replaceAt(exp.clone(), pos1.clone(), T0.clone())?;
                    {let _arr = var_field!(vis.T, Visualization::SURFACE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = T0.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "r_0", .. }, Visualization::SURFACE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.r_0, Visualization::SURFACE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "nu", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_nu_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SURFACE { nu, .. } = &mut vis {
                        *nu = __owned_variant_nu_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "nv", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_nv_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SURFACE { nv, .. } = &mut vis {
                        *nv = __owned_variant_nv_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "wireframe", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_wireframe_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SURFACE { wireframe, .. } = &mut vis {
                        *wireframe = __owned_variant_wireframe_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "multiColored", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_multiColored_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SURFACE { multiColored, .. } = &mut vis {
                        *multiColored = __owned_variant_multiColored_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: pos } }, tail: Deref @ metamodelica::List::Nil }, ident: Deref @ "color", .. }, Visualization::SURFACE { .. }) => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    {let _arr = var_field!(vis.color, Visualization::SURFACE).clone(); _arr.borrow_mut()[(pos.clone()-1) as usize] = exp.clone(); _arr};
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "specularCoefficient", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_specularCoeff_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SURFACE { specularCoeff, .. } = &mut vis {
                        *specularCoeff = __owned_variant_specularCoeff_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "transparency", .. }, Visualization::SURFACE { .. }) => {
                    let mut vis: Visualization = vis.clone();
                    let __owned_variant_transparency_0 = getVariableBinding(var.clone(), storeProtectedCrefs.clone())?;
                    if let Visualization::SURFACE { transparency, .. } = &mut vis {
                        *transparency = __owned_variant_transparency_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Visualization::SURFACE"); }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(vis)
}

fn getVariableBinding(mut var: BackendDAE::Var, mut storeProtectedCrefs: bool) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut binding: Option<Arc<DAE::Exp>> = None;
    let BackendDAE::VAR { bindExp: __pa0, .. } = (var.clone()) else { bail!("pattern mismatch") };
    binding = __pa0.clone();
    if isSome(binding.clone()) {
        let __pa1 = ::match_deref::match_deref! { match &(binding.clone()) {
            Some(__pa1) => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa1.clone();
        if !(Expression::isConstValue(exp.clone())?) && storeProtectedCrefs.clone() {
            exp = BackendVariable::varExp(var.clone())?;
        }
    } else {
        exp = BackendVariable::varExp(var.clone())?;
    }
    Ok(exp)
}

fn printVisualization(mut vis: Visualization) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    s = ((match vis.clone() {
        Visualization::SHAPE { extra: mut extra, height: mut height, width: mut width, length: mut length, T: mut T, widthDir: mut widthDir, lengthDir: mut lengthDir, r: mut r, color: mut color, shapeType: mut shapeType, ident: mut ident, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SHAPE ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ident.clone())?); __mm_s.push_str(&*literal!(" '")); __mm_s.push_str(&*ExpressionBasics::printExpStr(shapeType.clone())?); __mm_s.push_str(&*literal!("'\n r{")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (r.clone()).borrow().iter() {
            let __x = ExpressionDump::dumpExpStr(e.clone(), 0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!("\nlD{")); __mm_s.push_str(&*stringDelimitList(List::mapArray(lengthDir.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!(" wD{")); __mm_s.push_str(&*stringDelimitList(List::mapArray(widthDir.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!("\ncolor(")); __mm_s.push_str(&*stringDelimitList(List::mapArray(color.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*literal!(" w: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(width.clone())?); __mm_s.push_str(&*literal!(" h: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(height.clone())?); __mm_s.push_str(&*literal!(" l: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(length.clone())?); __mm_s.push_str(&*literal!("\nT {")); __mm_s.push_str(&*stringDelimitList(List::map(List::flatten(Arc::new(T.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!("\nextra{")); __mm_s.push_str(&*ExpressionBasics::printExpStr(extra.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!("-")
        },
    })).clone();
    Ok(s)
}

fn isVisualizationVar(mut var: BackendDAE::Var) -> Result<bool> {
    let mut isVisVar: bool = false;
    isVisVar = 'mc: {
        let __mc_input = var.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::Var { source: mut source, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut obj: ArcStr = arcstr::literal!("");
            let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            paths = ElementSource::getElementSourceTypes(source.clone());
            (obj, _) = hasVisPath(paths.clone(), 1)?;
            Ok(Util::stringNotEqual((obj.clone()).clone(), (literal!("")).clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isVisVar)
}

fn isVisualizationVarFold(mut var: BackendDAE::Var, mut tplIn: (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>)) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>)> {
    let mut tplOut: (Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, ArcStr)>>);
    tplOut = 'mc: {
        let __mc_input = (var.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { source, varName, .. }, (varLst, crefs)) => {
                    let mut idx: i32 = 0;
                    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
                    let mut obj: ArcStr = arcstr::literal!("");
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut crefs = (*crefs).clone();
                    paths = ElementSource::getElementSourceTypes(source.clone());
                    (obj, idx) = hasVisPath(paths.clone(), 1)?;
                    let true = (Util::stringNotEqual((obj.clone()).clone(), (literal!("")).clone())) else { bail!("pattern mismatch") };
                    cref = ComponentReference::firstNCrefs(varName.clone(), idx.clone() - 1)?;
                    crefs = List::unique(cons((cref.clone(), obj.clone()), crefs.clone()));
                    Ok((cons(var.clone(), varLst.clone()), crefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tplIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplOut)
}

fn hasVisPath(mut pathsIn: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut numIn: i32) -> Result<(ArcStr, i32)> {
    let mut visPath: ArcStr = arcstr::literal!("");
    let mut numOut: i32 = 0;
    (visPath, numOut) = 'mc: {
        let __mc_input = pathsIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((literal!(""), -1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Path::FULLYQUALIFIED { path }, tail: rest } => {
                    Ok(hasVisPath(cons(path.clone(), rest.clone()), numIn.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name }, name: Deref @ "Advanced" }, name: Deref @ "Visualizers" }, name: Deref @ "MultiBody" }, name: Deref @ "Mechanics" }, name: Deref @ "Modelica" }, tail: _ } => {
                    if !(((::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "Shape" => true,
        Deref @ "Vector" => true,
        Deref @ "Surface" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }))) { bail!("guard") }
                    Ok((name.clone(), numIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(hasVisPath(rest.clone(), numIn.clone() + 1)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((visPath, numOut))
}

fn dumpVis(mut visIn: metamodelica::Array<Visualization>, mut iFileName: ArcStr) -> Result<()> {
    println!("{}", (literal!("")).clone());
    Tpl::tplNoret2((std::sync::Arc::new(VisualXMLTpl::dumpVisXML) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, metamodelica::Array<Visualization>, ArcStr) -> Result<Tpl::Text> + 'static>), visIn.clone(), (iFileName.clone()).clone())?;
    Ok(())
}

