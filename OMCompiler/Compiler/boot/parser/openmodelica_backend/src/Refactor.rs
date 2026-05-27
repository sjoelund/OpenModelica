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

use crate::Interactive;
use crate::InteractiveUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend::FCore;
use openmodelica_frontend::Inst;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// stringReal
pub fn refactorGraphicalAnnotation(mut wholeAST: Absyn::Program, mut classToRefactor: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut changedClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    changedClass = (::match_deref::match_deref! { match &((wholeAST.clone(), classToRefactor.clone())) {
        (_, _) => {
            let mut c: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            c = refactorGraphAnnInClass(classToRefactor.clone(), wholeAST.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }))?;
            c.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(changedClass)
}

fn refactorGraphAnnInClass(mut inClass: Arc<Absyn::Class>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = 'mc: {
        let __mc_input = (inClass.clone(), inProgram.clone(), classPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (outClass @ Deref @ Absyn::Class { body: d, name: n, .. }, p, Deref @ Absyn::Path::IDENT { name: Deref @ "" }) => {
                    let mut resultClassDef: Arc<Absyn::ClassDef>;
                    let mut cPath: Arc<Absyn::Path>;
                    let mut env: Interactive::GraphicEnvCache;
                    let mut outClass = (*outClass).clone();
                    cPath = Arc::new(Absyn::Path::IDENT { name: (n.clone()).clone() });
                    env = Interactive::getClassEnv(p.clone(), cPath.clone())?;
                    resultClassDef = refactorGraphAnnInClassDef(d.clone(), p.clone(), cPath.clone(), env.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (outClass @ Deref @ Absyn::Class { body: d, name: n, .. }, p, cPath) => {
                    let mut env: Interactive::GraphicEnvCache;
                    let mut cPath = (*cPath).clone();
                    cPath = AbsynUtil::joinPaths(cPath.clone(), Arc::new(Absyn::Path::IDENT { name: (n.clone()).clone() }))?;
                    env = Interactive::getClassEnv(p.clone(), cPath.clone())?;
                    let _ = refactorGraphAnnInClassDef(d.clone(), p.clone(), cPath.clone(), env.clone())?;
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

fn refactorGraphAnnInClassDef(mut inDef: Arc<Absyn::ClassDef>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ClassDef>> {
    let mut outDef: Arc<Absyn::ClassDef>;
    outDef = 'mc: {
        let __mc_input = (inDef.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: cp, classAttrs, typeVars }, p, cPath, env) => {
                    let mut resultPart: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    resultPart = refactorGraphAnnInClassParts(cp.clone(), p.clone(), cPath.clone(), env.clone())?;
                    Ok(Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: resultPart.clone(), ann: ann.clone(), comment: cmt.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::DERIVED { comment: Some(Deref @ Absyn::Comment { comment: cmt, annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annList }) }), arguments: args, attributes: attrs, typeSpec: ts }, p, _, _) => {
                    let mut resAnnList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    resAnnList = transformClassAnnList(annList.clone(), list![(literal!("Class")).clone()], metamodelica::nil(), p.clone())?;
                    Ok(Arc::new(Absyn::ClassDef::DERIVED { typeSpec: ts.clone(), attributes: attrs.clone(), arguments: args.clone(), comment: Some(Arc::new(Absyn::Comment { annotation_: Some(Arc::new(Absyn::Annotation { elementArgs: resAnnList.clone() })), comment: cmt.clone() })) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inDef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outDef)
}

fn refactorGraphAnnInClassParts(mut inParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut env: Interactive::GraphicEnvCache) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outParts = (::match_deref::match_deref! { match &((inParts.clone(), inProgram.clone(), classPath.clone(), env.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: firstPart, tail: restParts }, p, cPath, _) => {
            let mut resParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut resultPart: Arc<Absyn::ClassPart>;
            resultPart = refactorGraphAnnInClassPart(firstPart.clone(), p.clone(), cPath.clone(), env.clone())?;
            resParts = refactorGraphAnnInClassParts(restParts.clone(), p.clone(), cPath.clone(), env.clone())?;
            cons(resultPart.clone(), resParts.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outParts)
}

fn refactorGraphAnnInClassPart(mut inPart: Arc<Absyn::ClassPart>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ClassPart>> {
    let mut outPart: Arc<Absyn::ClassPart>;
    outPart = 'mc: {
        let __mc_input = (inPart.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::PUBLIC { contents: elContent }, p, cPath, env) => {
                    let mut resultElContent: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    resultElContent = refactorGraphAnnInContentList(elContent.clone(), (std::sync::Arc::new(refactorGraphAnnInElItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementItem>, Absyn::Program, Arc<Absyn::Path>, Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ElementItem>> + 'static>), p.clone(), cPath.clone(), env.clone())?;
                    Ok(Arc::new(Absyn::ClassPart::PUBLIC { contents: resultElContent.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::PROTECTED { contents: elContent }, p, cPath, env) => {
                    let mut resultElContent: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    resultElContent = refactorGraphAnnInContentList(elContent.clone(), (std::sync::Arc::new(refactorGraphAnnInElItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementItem>, Absyn::Program, Arc<Absyn::Path>, Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ElementItem>> + 'static>), p.clone(), cPath.clone(), env.clone())?;
                    Ok(Arc::new(Absyn::ClassPart::PROTECTED { contents: resultElContent.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::EQUATIONS { contents: eqContent }, p, cPath, env) => {
                    let mut resultEqContent: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    resultEqContent = refactorGraphAnnInContentList(eqContent.clone(), (std::sync::Arc::new(refactorGraphAnnInEqItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>, Absyn::Program, Arc<Absyn::Path>, Interactive::GraphicEnvCache) -> Result<Arc<Absyn::EquationItem>> + 'static>), p.clone(), cPath.clone(), env.clone())?;
                    Ok(Arc::new(Absyn::ClassPart::EQUATIONS { contents: resultEqContent.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::ALGORITHMS { contents: algContent }, p, cPath, env) => {
                    let mut resultAlgContent: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    resultAlgContent = refactorGraphAnnInContentList(algContent.clone(), (std::sync::Arc::new(refactorGraphAnnInAlgItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>, Absyn::Program, Arc<Absyn::Path>, Interactive::GraphicEnvCache) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>), p.clone(), cPath.clone(), env.clone())?;
                    Ok(Arc::new(Absyn::ClassPart::ALGORITHMS { contents: resultAlgContent.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqContent }, p, cPath, env) => {
                    let mut resultEqContent: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    resultEqContent = refactorGraphAnnInContentList(eqContent.clone(), (std::sync::Arc::new(refactorGraphAnnInEqItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::EquationItem>, Absyn::Program, Arc<Absyn::Path>, Interactive::GraphicEnvCache) -> Result<Arc<Absyn::EquationItem>> + 'static>), p.clone(), cPath.clone(), env.clone())?;
                    Ok(Arc::new(Absyn::ClassPart::INITIALEQUATIONS { contents: resultEqContent.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algContent }, p, cPath, env) => {
                    let mut resultAlgContent: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
                    resultAlgContent = refactorGraphAnnInContentList(algContent.clone(), (std::sync::Arc::new(refactorGraphAnnInAlgItem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::AlgorithmItem>, Absyn::Program, Arc<Absyn::Path>, Interactive::GraphicEnvCache) -> Result<Arc<Absyn::AlgorithmItem>> + 'static>), p.clone(), cPath.clone(), env.clone())?;
                    Ok(Arc::new(Absyn::ClassPart::INITIALALGORITHMS { contents: resultAlgContent.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inPart.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPart)
}

fn refactorGraphAnnInContentList<contentType: Clone + 'static>(mut inList: Arc<metamodelica::List<contentType>>, mut refactorGraphAnnInItem: Arc<dyn ::std::ops::Fn(contentType, Absyn::Program, Arc<Absyn::Path>, Interactive::GraphicEnvCache) -> Result<contentType> + 'static>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<metamodelica::List<contentType>>> {
    pub type refactorGraphAnnInContent<contentType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(contentType, Absyn::Program, Arc<Absyn::Path>, Interactive::GraphicEnvCache) -> Result<contentType> + 'static>;

    let mut outList: Arc<metamodelica::List<contentType>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &((inList.clone(), refactorGraphAnnInItem.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: firstItem, tail: restList }, _, p, cPath, env) => {
            let mut resList: Arc<metamodelica::List<contentType>> = metamodelica::nil();
            let mut resultItem: contentType;
            resultItem = refactorGraphAnnInItem(firstItem.clone(), p.clone(), cPath.clone(), env.clone())?;
            resList = refactorGraphAnnInContentList(restList.clone(), refactorGraphAnnInItem.clone(), p.clone(), cPath.clone(), env.clone())?;
            cons(resultItem.clone(), resList.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

fn refactorGraphAnnInElItem(mut inItem: Arc<Absyn::ElementItem>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ElementItem>> {
    let mut outItem: Arc<Absyn::ElementItem>;
    outItem = (::match_deref::match_deref! { match &((inItem.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone())) {
        (Deref @ Absyn::ElementItem::ELEMENTITEM { element: el }, p, cPath, env) => {
            let mut resultElement: Arc<Absyn::Element>;
            resultElement = refactorGraphAnnInElement(el.clone(), p.clone(), cPath.clone(), env.clone())?;
            Arc::new(Absyn::ElementItem::ELEMENTITEM { element: resultElement.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outItem)
}

fn refactorGraphAnnInEqItem(mut inItem: Arc<Absyn::EquationItem>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::EquationItem>> {
    let mut outItem: Arc<Absyn::EquationItem>;
    outItem = 'mc: {
        let __mc_input = (inItem.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::EquationItem::EQUATIONITEM { comment: Some(Deref @ Absyn::Comment { comment: com, annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annList }) }), info, equation_: e }, p, _, _) => {
                    let mut annList = (*annList).clone();
                    annList = transformConnectAnnList(annList.clone(), list![(literal!("Connect")).clone()], metamodelica::nil(), p.clone())?;
                    Ok(Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: e.clone(), comment: Some(Arc::new(Absyn::Comment { annotation_: Some(Arc::new(Absyn::Annotation { elementArgs: annList.clone() })), comment: com.clone() })), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inItem.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outItem)
}

fn refactorGraphAnnInAlgItem(mut inItem: Arc<Absyn::AlgorithmItem>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::AlgorithmItem>> {
    let mut outItem: Arc<Absyn::AlgorithmItem>;
    outItem = 'mc: {
        let __mc_input = (inItem.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annList }), comment: com }), info, algorithm_: alg }, _, _, _) => {
                    Ok(Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg.clone(), comment: Some(Arc::new(Absyn::Comment { annotation_: Some(Arc::new(Absyn::Annotation { elementArgs: annList.clone() })), comment: com.clone() })), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inItem.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outItem)
}

fn refactorGraphAnnInElement(mut inElement: Arc<Absyn::Element>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::Element>> {
    let mut outElement: Arc<Absyn::Element>;
    outElement = (::match_deref::match_deref! { match &((inElement.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone())) {
        (Deref @ Absyn::Element::ELEMENT { constrainClass: cc, info: i, specification: es, innerOuter: io, redeclareKeywords: rdk, finalPrefix: f }, p, cPath, env) => {
            let mut resultSpec: Arc<Absyn::ElementSpec>;
            let mut cc = (*cc).clone();
            cc = refactorConstrainClass(cc.clone(), p.clone(), cPath.clone(), env.clone())?;
            resultSpec = refactorGraphAnnInElSpec(es.clone(), p.clone(), cPath.clone(), env.clone())?;
            Arc::new(Absyn::Element::ELEMENT { finalPrefix: f.clone(), redeclareKeywords: rdk.clone(), innerOuter: io.clone(), specification: resultSpec.clone(), info: i.clone(), constrainClass: cc.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

fn refactorConstrainClass(mut inCC: Option<Arc<Absyn::ConstrainClass>>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Option<Arc<Absyn::ConstrainClass>>> {
    let mut outCC: Option<Arc<Absyn::ConstrainClass>> = None;
    outCC = (::match_deref::match_deref! { match &((inCC.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone())) {
        (Some(Deref @ Absyn::ConstrainClass { comment: com, elementSpec: es }), p, cPath, env) => {
            let mut resultSpec: Arc<Absyn::ElementSpec>;
            resultSpec = refactorGraphAnnInElSpec(es.clone(), p.clone(), cPath.clone(), env.clone())?;
            Some(Arc::new(Absyn::ConstrainClass { elementSpec: resultSpec.clone(), comment: com.clone() }))
        },
        (None, _, _, _) => {
            None
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCC)
}

fn refactorGraphAnnInElSpec(mut inSpec: Arc<Absyn::ElementSpec>, mut inProgram: Absyn::Program, mut classPath: Arc<Absyn::Path>, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ElementSpec>> {
    let mut outSpec: Arc<Absyn::ElementSpec>;
    outSpec = 'mc: {
        let __mc_input = (inSpec.clone(), inProgram.clone(), classPath.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ElementSpec::CLASSDEF { class_: cl, replaceable_: r }, p, cPath, _) => {
                    let mut cl1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    cl1 = refactorGraphAnnInClass(cl.clone(), p.clone(), cPath.clone())?;
                    Ok(Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: r.clone(), class_: cl1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ElementSpec::COMPONENTS { attributes: at, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: z }, components: Deref @ metamodelica::List::Cons { head: firstComp, tail: restCompList } }, p, cPath, env) => {
                    let mut resultComp: Arc<Absyn::ComponentItem>;
                    let mut resCompList: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut at = (*at).clone();
                    let mut path = (*path).clone();
                    let mut z = (*z).clone();
                    resultComp = refactorGraphAnnInComponentItem(firstComp.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(refactorGraphAnnInElSpec(Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: at.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: z.clone() }), components: restCompList.clone() }), p.clone(), cPath.clone(), env.clone())?) {
                        Deref @ Absyn::ElementSpec::COMPONENTS { attributes: __pa0, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa1, arrayDim: __pa2 }, components: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    at = __pa0.clone();
                    path = __pa1.clone();
                    z = __pa2.clone();
                    resCompList = __pa3.clone();
                    Ok(Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: at.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: z.clone() }), components: cons(resultComp.clone(), resCompList.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inSpec.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSpec)
}

fn refactorGraphAnnInComponentItem(mut inCom: Arc<Absyn::ComponentItem>, mut classPath: Arc<Absyn::Path>, mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ComponentItem>> {
    let mut outCom: Arc<Absyn::ComponentItem>;
    outCom = 'mc: {
        let __mc_input = (inCom.clone(), classPath.clone(), inPath.clone(), inProgram.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentItem { comment: Some(Deref @ Absyn::Comment { comment: r#str, annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annList }) }), condition: con, component: comp }, cPath, path, p, env) => {
                    let mut annList = (*annList).clone();
                    annList = transformComponentAnnList(annList.clone(), list![(literal!("Component")).clone()], metamodelica::nil(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    Ok(Arc::new(Absyn::ComponentItem { component: comp.clone(), condition: con.clone(), comment: Some(Arc::new(Absyn::Comment { annotation_: Some(Arc::new(Absyn::Annotation { elementArgs: annList.clone() })), comment: r#str.clone() })) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCom.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCom)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn transformComponentAnnList(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inCon: Context, mut resultList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut classPath: Arc<Absyn::Path>, mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outArgs = 'mc: {
        let __mc_input = (inArgs.clone(), inCon.clone(), resultList.clone(), classPath.clone(), inPath.clone(), inProgram.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, res, _, _, _, _) => {
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x1, tail: Deref @ metamodelica::List::Cons { head: y1, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x2, tail: Deref @ metamodelica::List::Cons { head: y2, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Component", tail: _ }, res, cPath, path, p, env) => {
                    let mut trans: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut iconTrans: Arc<Absyn::ElementArg>;
                    let mut diagramTrans: Arc<Absyn::ElementArg>;
                    let mut rot: Option<metamodelica::Real> = None;
                    let mut res = (*res).clone();
                    let Absyn::R_CONNECTOR { .. } = (getRestrictionFromPath(cPath.clone(), path.clone(), p.clone(), env.clone())?) else { bail!("pattern mismatch") };
                    rot = getRotationDegree(listAppend(res.clone(), rest.clone()))?;
                    iconTrans = getIconTransformation(x1.clone(), y1.clone(), x2.clone(), y2.clone(), rot.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    diagramTrans = getDiagramTransformation(x1.clone(), y1.clone(), x2.clone(), y2.clone(), rot.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    trans = list![diagramTrans.clone(), iconTrans.clone()];
                    res = transformComponentAnnList(rest.clone(), context.clone(), res.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    res = list![Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Placement")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: trans.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: com.clone(), info: info.clone() })];
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x1, tail: Deref @ metamodelica::List::Cons { head: y1, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x2, tail: Deref @ metamodelica::List::Cons { head: y2, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Component", tail: _ }, res, cPath, path, p, env) => {
                    let mut diagramTrans: Arc<Absyn::ElementArg>;
                    let mut rot: Option<metamodelica::Real> = None;
                    let mut res = (*res).clone();
                    rot = getRotationDegree(listAppend(res.clone(), rest.clone()))?;
                    diagramTrans = getDiagramTransformation(x1.clone(), y1.clone(), x2.clone(), y2.clone(), rot.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    res = transformComponentAnnList(rest.clone(), context.clone(), res.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    res = list![Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Placement")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![diagramTrans.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: com.clone(), info: info.clone() })];
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: arg, tail: rest }, context, res, cPath, path, p, env) => {
                    let mut res = (*res).clone();
                    res = cons(arg.clone(), res.clone());
                    res = transformComponentAnnList(rest.clone(), context.clone(), res.clone(), cPath.clone(), path.clone(), p.clone(), env.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArgs)
}

fn getRestrictionFromPath(mut classPath: Arc<Absyn::Path>, mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Absyn::Restriction> {
    let mut outRestriction: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    outRestriction = 'mc: {
        let __mc_input = (classPath.clone(), inPath.clone(), inProgram.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cPath, path, p, _) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut fullPath: Arc<Absyn::Path>;
                    let mut restriction: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
                    fullPath = fixPaths(cPath.clone(), path.clone())?;
                    cdef = InteractiveUtil::getPathedClassInProgram(fullPath.clone(), p.clone(), false, false)?;
                    restriction = getRestrictionInClass(cdef.clone())?;
                    Ok(restriction.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, path, p, env) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut fullPath: Arc<Absyn::Path>;
                    let mut restriction: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
                    (_, fullPath) = Interactive::mkFullyQual(env.clone(), path.clone(), false)?;
                    cdef = InteractiveUtil::getPathedClassInProgram(fullPath.clone(), p.clone(), false, false)?;
                    restriction = getRestrictionInClass(cdef.clone())?;
                    Ok(restriction.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outRestriction)
}

fn getRestrictionInClass(mut inClass: Arc<Absyn::Class>) -> Result<Absyn::Restriction> {
    let mut outRestriction: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    outRestriction = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { restriction, .. } => {
            restriction.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRestriction)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getRotationDegree(mut inList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Option<metamodelica::Real>> {
    let mut degrees: Option<metamodelica::Real> = None;
    degrees = 'mc: {
        let __mc_input = inList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: ex, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "rotation" }, .. }, tail: _ } => {
                    let mut rot: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    rot = getValueFromExp(ex.clone())?;
                    Ok(Some(rot.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut res: Option<metamodelica::Real> = None;
                    res = getRotationDegree(rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(degrees)
}

fn getIconTransformation(mut ax1: Arc<Absyn::Exp>, mut ay1: Arc<Absyn::Exp>, mut ax2: Arc<Absyn::Exp>, mut ay2: Arc<Absyn::Exp>, mut inRotation: Option<metamodelica::Real>, mut classPath: Arc<Absyn::Path>, mut inPath: Arc<Absyn::Path>, mut inProg: Absyn::Program, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ElementArg>> {
    let mut iconTrans: Arc<Absyn::ElementArg>;
    iconTrans = (::match_deref::match_deref! { match &((ax1.clone(), ay1.clone(), ax2.clone(), ay2.clone(), inRotation.clone(), classPath.clone(), inPath.clone(), inProg.clone(), inClassEnv.clone())) {
        (x1, y1, x2, y2, None, cPath, path, p, env) => {
            let mut rcx1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcy1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcx2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcy2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rax1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ray1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rax2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ray2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut scale: Arc<Absyn::ElementArg>;
            let mut aspectRatio: Arc<Absyn::ElementArg>;
            let mut x: Arc<Absyn::ElementArg>;
            let mut y: Arc<Absyn::ElementArg>;
            let mut flipHorizontal: Arc<Absyn::ElementArg>;
            let mut flipVertical: Arc<Absyn::ElementArg>;
            let mut x1 = (*x1).clone();
            let mut y1 = (*y1).clone();
            let mut x2 = (*x2).clone();
            let mut y2 = (*y2).clone();
            rax1 = getValueFromExp(x1.clone())?;
            ray1 = getValueFromExp(y1.clone())?;
            rax2 = getValueFromExp(x2.clone())?;
            ray2 = getValueFromExp(y2.clone())?;
            (x1, y1, x2, y2) = getCoordsInPath(cPath.clone(), path.clone(), p.clone(), list![(literal!("Icon")).clone()], env.clone())?;
            rcx1 = getValueFromExp(x1.clone())?;
            rcy1 = getValueFromExp(y1.clone())?;
            rcx2 = getValueFromExp(x2.clone())?;
            rcy2 = getValueFromExp(y2.clone())?;
            aspectRatio = getAspectRatioAnn(rax1.clone(), rax2.clone(), ray1.clone(), ray2.clone(), rcx1.clone(), rcy1.clone(), rcx2.clone(), rcy2.clone());
            x = getXYAnn(rax1.clone(), rax2.clone(), (literal!("x")).clone());
            y = getXYAnn(ray1.clone(), ray2.clone(), (literal!("y")).clone());
            scale = getScaleAnn(rax1.clone(), rax2.clone(), rcx1.clone(), rcx2.clone());
            flipHorizontal = getFlipAnn(rax1.clone(), rax2.clone(), (literal!("flipHorizontal")).clone());
            flipVertical = getFlipAnn(ray1.clone(), ray2.clone(), (literal!("flipVertical")).clone());
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("iconTransformation")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![x.clone(), y.clone(), scale.clone(), aspectRatio.clone(), flipHorizontal.clone(), flipVertical.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() })
        },
        (x1, y1, x2, y2, Some(rot), cPath, path, p, env) => {
            let mut rcx1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcy1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcx2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcy2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rax1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ray1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rax2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ray2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut scale: Arc<Absyn::ElementArg>;
            let mut aspectRatio: Arc<Absyn::ElementArg>;
            let mut x: Arc<Absyn::ElementArg>;
            let mut y: Arc<Absyn::ElementArg>;
            let mut flipHorizontal: Arc<Absyn::ElementArg>;
            let mut flipVertical: Arc<Absyn::ElementArg>;
            let mut rotation: Arc<Absyn::ElementArg>;
            let mut x1 = (*x1).clone();
            let mut y1 = (*y1).clone();
            let mut x2 = (*x2).clone();
            let mut y2 = (*y2).clone();
            rax1 = getValueFromExp(x1.clone())?;
            ray1 = getValueFromExp(y1.clone())?;
            rax2 = getValueFromExp(x2.clone())?;
            ray2 = getValueFromExp(y2.clone())?;
            (x1, y1, x2, y2) = getCoordsInPath(cPath.clone(), path.clone(), p.clone(), list![(literal!("Icon")).clone()], env.clone())?;
            rcx1 = getValueFromExp(x1.clone())?;
            rcy1 = getValueFromExp(y1.clone())?;
            rcx2 = getValueFromExp(x2.clone())?;
            rcy2 = getValueFromExp(y2.clone())?;
            aspectRatio = getAspectRatioAnn(rax1.clone(), rax2.clone(), ray1.clone(), ray2.clone(), rcx1.clone(), rcy1.clone(), rcx2.clone(), rcy2.clone());
            x = getXYAnn(rax1.clone(), rax2.clone(), (literal!("x")).clone());
            y = getXYAnn(ray1.clone(), ray2.clone(), (literal!("y")).clone());
            scale = getScaleAnn(rax1.clone(), rax2.clone(), rcx1.clone(), rcx2.clone());
            flipHorizontal = getFlipAnn(rax1.clone(), rax2.clone(), (literal!("flipHorizontal")).clone());
            flipVertical = getFlipAnn(ray1.clone(), ray2.clone(), (literal!("flipVertical")).clone());
            rotation = getRotationAnn(rot.clone());
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("iconTransformation")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![x.clone(), y.clone(), scale.clone(), aspectRatio.clone(), flipHorizontal.clone(), flipVertical.clone(), rotation.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(iconTrans)
}

fn getDiagramTransformation(mut ax1: Arc<Absyn::Exp>, mut ay1: Arc<Absyn::Exp>, mut ax2: Arc<Absyn::Exp>, mut ay2: Arc<Absyn::Exp>, mut inRotation: Option<metamodelica::Real>, mut classPath: Arc<Absyn::Path>, mut inPath: Arc<Absyn::Path>, mut inProg: Absyn::Program, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<Arc<Absyn::ElementArg>> {
    let mut trans: Arc<Absyn::ElementArg>;
    trans = (::match_deref::match_deref! { match &((ax1.clone(), ay1.clone(), ax2.clone(), ay2.clone(), inRotation.clone(), classPath.clone(), inPath.clone(), inProg.clone(), inClassEnv.clone())) {
        (x1, y1, x2, y2, None, cPath, path, p, env) => {
            let mut rcx1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcy1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcx2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcy2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rax1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ray1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rax2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ray2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut scale: Arc<Absyn::ElementArg>;
            let mut aspectRatio: Arc<Absyn::ElementArg>;
            let mut x: Arc<Absyn::ElementArg>;
            let mut y: Arc<Absyn::ElementArg>;
            let mut flipHorizontal: Arc<Absyn::ElementArg>;
            let mut flipVertical: Arc<Absyn::ElementArg>;
            let mut x1 = (*x1).clone();
            let mut y1 = (*y1).clone();
            let mut x2 = (*x2).clone();
            let mut y2 = (*y2).clone();
            rax1 = getValueFromExp(x1.clone())?;
            ray1 = getValueFromExp(y1.clone())?;
            rax2 = getValueFromExp(x2.clone())?;
            ray2 = getValueFromExp(y2.clone())?;
            (x1, y1, x2, y2) = getCoordsInPath(cPath.clone(), path.clone(), p.clone(), list![(literal!("Diagram")).clone()], env.clone())?;
            rcx1 = getValueFromExp(x1.clone())?;
            rcy1 = getValueFromExp(y1.clone())?;
            rcx2 = getValueFromExp(x2.clone())?;
            rcy2 = getValueFromExp(y2.clone())?;
            aspectRatio = getAspectRatioAnn(rax1.clone(), rax2.clone(), ray1.clone(), ray2.clone(), rcx1.clone(), rcy1.clone(), rcx2.clone(), rcy2.clone());
            x = getXYAnn(rax1.clone(), rax2.clone(), (literal!("x")).clone());
            y = getXYAnn(ray1.clone(), ray2.clone(), (literal!("y")).clone());
            scale = getScaleAnn(rax1.clone(), rax2.clone(), rcx1.clone(), rcx2.clone());
            flipHorizontal = getFlipAnn(rax1.clone(), rax2.clone(), (literal!("flipHorizontal")).clone());
            flipVertical = getFlipAnn(ray1.clone(), ray2.clone(), (literal!("flipVertical")).clone());
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("transformation")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![x.clone(), y.clone(), scale.clone(), aspectRatio.clone(), flipHorizontal.clone(), flipVertical.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() })
        },
        (x1, y1, x2, y2, Some(rot), cPath, path, p, env) => {
            let mut rcx1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcy1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcx2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rcy2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rax1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ray1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rax2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ray2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut scale: Arc<Absyn::ElementArg>;
            let mut aspectRatio: Arc<Absyn::ElementArg>;
            let mut x: Arc<Absyn::ElementArg>;
            let mut y: Arc<Absyn::ElementArg>;
            let mut flipHorizontal: Arc<Absyn::ElementArg>;
            let mut flipVertical: Arc<Absyn::ElementArg>;
            let mut rotation: Arc<Absyn::ElementArg>;
            let mut x1 = (*x1).clone();
            let mut y1 = (*y1).clone();
            let mut x2 = (*x2).clone();
            let mut y2 = (*y2).clone();
            rax1 = getValueFromExp(x1.clone())?;
            ray1 = getValueFromExp(y1.clone())?;
            rax2 = getValueFromExp(x2.clone())?;
            ray2 = getValueFromExp(y2.clone())?;
            (x1, y1, x2, y2) = getCoordsInPath(cPath.clone(), path.clone(), p.clone(), list![(literal!("Diagram")).clone()], env.clone())?;
            rcx1 = getValueFromExp(x1.clone())?;
            rcy1 = getValueFromExp(y1.clone())?;
            rcx2 = getValueFromExp(x2.clone())?;
            rcy2 = getValueFromExp(y2.clone())?;
            aspectRatio = getAspectRatioAnn(rax1.clone(), rax2.clone(), ray1.clone(), ray2.clone(), rcx1.clone(), rcy1.clone(), rcx2.clone(), rcy2.clone());
            x = getXYAnn(rax1.clone(), rax2.clone(), (literal!("x")).clone());
            y = getXYAnn(ray1.clone(), ray2.clone(), (literal!("y")).clone());
            scale = getScaleAnn(rax1.clone(), rax2.clone(), rcx1.clone(), rcx2.clone());
            flipHorizontal = getFlipAnn(rax1.clone(), rax2.clone(), (literal!("flipHorizontal")).clone());
            flipVertical = getFlipAnn(ray1.clone(), ray2.clone(), (literal!("flipVertical")).clone());
            rotation = getRotationAnn(rot.clone());
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("transformation")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![x.clone(), y.clone(), scale.clone(), aspectRatio.clone(), flipHorizontal.clone(), flipVertical.clone(), rotation.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(trans)
}

fn getAspectRatioAnn(mut x1: metamodelica::Real, mut x2: metamodelica::Real, mut y1: metamodelica::Real, mut y2: metamodelica::Real, mut cx1: metamodelica::Real, mut cy1: metamodelica::Real, mut cx2: metamodelica::Real, mut cy2: metamodelica::Real) -> Arc<Absyn::ElementArg> {
    let mut aspectRatio: Arc<Absyn::ElementArg>;
    aspectRatio = (match (x1.clone(), x2.clone(), y1.clone(), y2.clone(), cx1.clone(), cy1.clone(), cx2.clone(), cy2.clone()) {
        (mut rx1, mut rx2, mut ry1, mut ry2, mut crx1, mut cry1, mut crx2, mut cry2) => {
            let mut aspect: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut s: ArcStr = arcstr::literal!("");
            aspect = realAbs(ry2.clone() - ry1.clone()) * realAbs(cry2.clone() - cry1.clone()) / (realAbs(rx2.clone() - rx1.clone()) * realAbs(crx2.clone() - crx1.clone()));
            s = (realString(aspect.clone())).clone();
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("aspectRatio")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::REAL { value: (s.clone()).clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() })
        },
    });
    aspectRatio
}

fn getXYAnn(mut val1: metamodelica::Real, mut val2: metamodelica::Real, mut name: ArcStr) -> Arc<Absyn::ElementArg> {
    let mut res: Arc<Absyn::ElementArg>;
    res = (match (val1.clone(), val2.clone(), name.clone()) {
        (mut x1, mut x2, mut n) => {
            let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut s: ArcStr = arcstr::literal!("");
            value = (x1.clone() + x2.clone()) / metamodelica::OrderedFloat(2.0_f64);
            s = (realString(value.clone())).clone();
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (n.clone()).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::REAL { value: (s.clone()).clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() })
        },
    });
    res
}

fn getScaleAnn(mut ax1: metamodelica::Real, mut ax2: metamodelica::Real, mut cx1: metamodelica::Real, mut cx2: metamodelica::Real) -> Arc<Absyn::ElementArg> {
    let mut scale: Arc<Absyn::ElementArg>;
    scale = (match (ax1.clone(), ax2.clone(), cx1.clone(), cx2.clone()) {
        (mut arx1, mut arx2, mut crx1, mut crx2) => {
            let mut scaleFac: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut s: ArcStr = arcstr::literal!("");
            scaleFac = realAbs(arx1.clone() - arx2.clone()) / realAbs(crx1.clone() - crx2.clone());
            s = (realString(scaleFac.clone())).clone();
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("scale")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::REAL { value: (s.clone()).clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() })
        },
    });
    scale
}

fn getFlipAnn(mut val1: metamodelica::Real, mut val2: metamodelica::Real, mut name: ArcStr) -> Arc<Absyn::ElementArg> {
    let mut flip: Arc<Absyn::ElementArg>;
    let mut value: bool = false;
    value = val1.clone() > val2.clone();
    flip = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::BOOL { value: value.clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
    flip
}

fn getRotationAnn(mut rot: metamodelica::Real) -> Arc<Absyn::ElementArg> {
    let mut rotation: Arc<Absyn::ElementArg>;
    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut s: ArcStr = arcstr::literal!("");
    r = rot.clone() * metamodelica::OrderedFloat(-1.0_f64);
    s = (realString(r.clone())).clone();
    rotation = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("rotation")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::REAL { value: (s.clone()).clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
    rotation
}

fn getCoordsInPath(mut classPath: Arc<Absyn::Path>, mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut contextToGetCoordsFrom: Context, mut inClassEnv: Interactive::GraphicEnvCache) -> Result<(Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>)> {
    let mut posX1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut posY1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut posX2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut posY2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (posX1, posY1, posX2, posY2) = 'mc: {
        let __mc_input = (classPath.clone(), inPath.clone(), inProgram.clone(), contextToGetCoordsFrom.clone(), inClassEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cPath, path, p, context, _) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut x1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut y1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut x2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut y2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut fullPath: Arc<Absyn::Path>;
                    fullPath = fixPaths(cPath.clone(), path.clone())?;
                    cdef = InteractiveUtil::getPathedClassInProgram(fullPath.clone(), p.clone(), false, false)?;
                    (x1, y1, x2, y2) = getCoordsInClass(cdef.clone(), context.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, path, p, context, env) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut x1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut y1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut x2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut y2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut fullPath: Arc<Absyn::Path>;
                    (_, fullPath) = Interactive::mkFullyQual(env.clone(), path.clone(), false)?;
                    cdef = InteractiveUtil::getPathedClassInProgram(fullPath.clone(), p.clone(), false, false)?;
                    (x1, y1, x2, y2) = getCoordsInClass(cdef.clone(), context.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((posX1, posY1, posX2, posY2))
}

fn getCoordsInClass(mut inClass: Arc<Absyn::Class>, mut contextToGetCoordsFrom: Context) -> Result<(Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>)> {
    let mut x1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut x2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (x1, y1, x2, y2) = (::match_deref::match_deref! { match &((inClass.clone(), contextToGetCoordsFrom.clone())) {
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { ann, .. }, .. }, context) => {
            let mut annlst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            annlst = List::flatten(List::map(ann.clone(), (std::sync::Arc::new(AbsynUtil::annotationToElementArgs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Annotation>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> + 'static>)));
            (x1, y1, x2, y2) = getCoordsInAnnList(annlst.clone(), context.clone())?;
            (x1.clone(), y1.clone(), x2.clone(), y2.clone())
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annlst }), .. }), .. }, .. }, context) => {
            (x1, y1, x2, y2) = getCoordsInAnnList(annlst.clone(), context.clone())?;
            (x1.clone(), y1.clone(), x2.clone(), y2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((x1, y1, x2, y2))
}

fn getCoordsInAnnList(mut inAnns: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut contextToGetCoordsFrom: Context) -> Result<(Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>)> {
    let mut x1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut x2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (x1, y1, x2, y2) = 'mc: {
        let __mc_input = (inAnns.clone(), contextToGetCoordsFrom.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((Arc::new(Absyn::Exp::REAL { value: (literal!("-100.0")).clone() }), Arc::new(Absyn::Exp::REAL { value: (literal!("-100.0")).clone() }), Arc::new(Absyn::Exp::REAL { value: (literal!("100.0")).clone() }), Arc::new(Absyn::Exp::REAL { value: (literal!("100.0")).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Coordsys" }, .. }, tail: _ }, _) => {
                    let mut y1: Arc<Absyn::Exp> = y1.clone();
                    let mut y2: Arc<Absyn::Exp> = y2.clone();
                    let mut x1: Arc<Absyn::Exp> = x1.clone();
                    let mut x2: Arc<Absyn::Exp> = x2.clone();
                    (x1, y1, x2, y2) = getCoordsFromCoordSysArgs(args.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Icon" }, .. }, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ "Icon", tail: _ }) => {
                    let mut x1: Arc<Absyn::Exp> = x1.clone();
                    let mut y1: Arc<Absyn::Exp> = y1.clone();
                    let mut y2: Arc<Absyn::Exp> = y2.clone();
                    let mut x2: Arc<Absyn::Exp> = x2.clone();
                    (x1, y1, x2, y2) = getCoordsFromLayerArgs(args.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Diagram" }, .. }, tail: _ }, Deref @ metamodelica::List::Cons { head: Deref @ "Diagram", tail: _ }) => {
                    let mut y1: Arc<Absyn::Exp> = y1.clone();
                    let mut x2: Arc<Absyn::Exp> = x2.clone();
                    let mut x1: Arc<Absyn::Exp> = x1.clone();
                    let mut y2: Arc<Absyn::Exp> = y2.clone();
                    (x1, y1, x2, y2) = getCoordsFromLayerArgs(args.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, context) => {
                    let mut y1: Arc<Absyn::Exp> = y1.clone();
                    let mut x1: Arc<Absyn::Exp> = x1.clone();
                    let mut y2: Arc<Absyn::Exp> = y2.clone();
                    let mut x2: Arc<Absyn::Exp> = x2.clone();
                    (x1, y1, x2, y2) = getCoordsInAnnList(rest.clone(), context.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((x1, y1, x2, y2))
}

fn getCoordsFromCoordSysArgs(mut inAnns: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>)> {
    let mut x1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut x2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (x1, y1, x2, y2) = (::match_deref::match_deref! { match &(inAnns.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x1, tail: Deref @ metamodelica::List::Cons { head: y1, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x2, tail: Deref @ metamodelica::List::Cons { head: y2, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, .. }, tail: _ } => {
            (x1.clone(), y1.clone(), x2.clone(), y2.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            (x1, y1, x2, y2) = getCoordsFromCoordSysArgs(rest.clone())?;
            (x1.clone(), y1.clone(), x2.clone(), y2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((x1, y1, x2, y2))
}

fn getExtentModification(mut elementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>)> {
    let mut x1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut x2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (x1, y1, x2, y2) = (::match_deref::match_deref! { match &(elementArgLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Cons { head: x1, tail: Deref @ metamodelica::List::Cons { head: y1, tail: Deref @ metamodelica::List::Nil } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Cons { head: x2, tail: Deref @ metamodelica::List::Cons { head: y2, tail: Deref @ metamodelica::List::Nil } } }, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, .. }, tail: _ } => {
            (x1.clone(), y1.clone(), x2.clone(), y2.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            (x1, y1, x2, y2) = getExtentModification(rest.clone())?;
            (x1.clone(), y1.clone(), x2.clone(), y2.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((x1, y1, x2, y2))
}

fn getCoordsFromLayerArgs(mut inAnns: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>, Arc<Absyn::Exp>)> {
    let mut x1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut x2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut y2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    (x1, y1, x2, y2) = 'mc: {
        let __mc_input = inAnns.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "coordinateSystem" }, .. }, tail: _ } => {
                    let mut x2: Arc<Absyn::Exp> = x2.clone();
                    let mut y2: Arc<Absyn::Exp> = y2.clone();
                    let mut y1: Arc<Absyn::Exp> = y1.clone();
                    let mut x1: Arc<Absyn::Exp> = x1.clone();
                    (x1, y1, x2, y2) = getExtentModification(args.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut x2: Arc<Absyn::Exp> = x2.clone();
                    let mut x1: Arc<Absyn::Exp> = x1.clone();
                    let mut y2: Arc<Absyn::Exp> = y2.clone();
                    let mut y1: Arc<Absyn::Exp> = y1.clone();
                    (x1, y1, x2, y2) = getCoordsFromLayerArgs(rest.clone())?;
                    Ok((x1.clone(), y1.clone(), x2.clone(), y2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((x1, y1, x2, y2))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn transformConnectAnnList(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inCon: Context, mut resultList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outArgs = 'mc: {
        let __mc_input = (inArgs.clone(), inCon.clone(), resultList.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, res, _) => {
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: expMatrix }, info }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "points" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Connect", tail: _ }, res, p) => {
                    let mut expLst: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut context = (*context).clone();
                    let mut res = (*res).clone();
                    context = addContext(context.clone(), (literal!("Line")).clone());
                    expLst = List::map(expMatrix.clone(), (std::sync::Arc::new(fnptr!(matrixToArray, Arc<metamodelica::List<Arc<Absyn::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<Absyn::Exp>> + 'static>));
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(list![Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Line")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("points")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: expLst.clone() }), info: info.clone() }) })), comment: None, info: mod_info.clone() }), res.clone()), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: com.clone(), info: mod_info.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: expMatrix }, info }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "points" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut expLst: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut res = (*res).clone();
                    expLst = List::map(expMatrix.clone(), (std::sync::Arc::new(fnptr!(matrixToArray, Arc<metamodelica::List<Arc<Absyn::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<Absyn::Exp>> + 'static>));
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("points")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: expLst.clone() }), info: info.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "style" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Connect", tail: _ }, res, p) => {
                    let mut args = (*args).clone();
                    let mut rest = (*rest).clone();
                    let mut context = (*context).clone();
                    let mut res = (*res).clone();
                    context = addContext(context.clone(), (literal!("Line")).clone());
                    args = cleanStyleAttrs(args.clone(), metamodelica::nil(), context.clone())?;
                    rest = listAppend(args.clone(), rest.clone());
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(list![Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Line")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: res.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "style" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut args = (*args).clone();
                    let mut rest = (*rest).clone();
                    let mut res = (*res).clone();
                    args = cleanStyleAttrs(args.clone(), metamodelica::nil(), context.clone())?;
                    rest = listAppend(args.clone(), rest.clone());
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, info }, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut color1: i32 = 0;
                    let mut color2: i32 = 0;
                    let mut color3: i32 = 0;
                    let mut res = (*res).clone();
                    (color1, color2, color3) = getMappedColor(x.clone());
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("color")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: color1.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color2.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color3.clone() })] }), info: info.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut val: ArcStr = arcstr::literal!("");
                    let mut res = (*res).clone();
                    val = ((patternMapList.clone()).get(x.clone() + 1)?).clone();
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("pattern")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("LinePattern")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (val.clone()).clone(), subscripts: metamodelica::nil() }) }) }), info: Absyn::dummyInfo.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut thick: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut res = (*res).clone();
                    thick = (thicknessMapList.clone()).get(x.clone())?;
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    s = (realString(thick.clone())).clone();
                    Ok(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("thickness")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::REAL { value: (s.clone()).clone() }), info: Absyn::dummyInfo.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut res = (*res).clone();
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("smooth")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "arrow" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }, res, p) => {
                    let mut val1: ArcStr = arcstr::literal!("");
                    let mut val2: ArcStr = arcstr::literal!("");
                    let mut arrows: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res = (*res).clone();
                    arrows = (arrowMapList.clone()).get(x.clone() + 1)?;
                    val1 = ((arrows.clone()).get(1)?).clone();
                    val2 = ((arrows.clone()).get(2)?).clone();
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("arrow")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("Arrow")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (val1.clone()).clone(), subscripts: metamodelica::nil() }) }) }), Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("Arrow")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (val2.clone()).clone(), subscripts: metamodelica::nil() }) }) })] }), info: Absyn::dummyInfo.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: arg, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformConnectAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(arg.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArgs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn transformClassAnnList(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inCon: Context, mut resultList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outArgs = 'mc: {
        let __mc_input = (inArgs.clone(), inCon.clone(), resultList.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, res, _) => {
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Icon" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Class", tail: c }, res, p) => {
                    let mut argRes: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut coord: Arc<Absyn::ElementArg>;
                    let mut c = (*c).clone();
                    let mut res = (*res).clone();
                    c = addContext(context.clone(), (literal!("Layer")).clone());
                    argRes = transAnnLstToCalls(args.clone(), c.clone())?;
                    coord = getCoordSysAnn(listAppend(res.clone(), rest.clone()), p.clone())?;
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Icon")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![coord.clone(), Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("graphics")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: argRes.clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: mod_info.clone() })], eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Diagram" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Class", tail: c }, res, p) => {
                    let mut argRes: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut coord: Arc<Absyn::ElementArg>;
                    let mut c = (*c).clone();
                    let mut res = (*res).clone();
                    c = addContext(context.clone(), (literal!("Layer")).clone());
                    argRes = transAnnLstToCalls(args.clone(), c.clone())?;
                    coord = getCoordSysAnn(listAppend(res.clone(), rest.clone()), p.clone())?;
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Diagram")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![coord.clone(), Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("graphics")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: argRes.clone() }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: mod_info.clone() })], eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Coordsys" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    let true = (isLayerAnnInList(listAppend(res.clone(), rest.clone()))) else { bail!("pattern mismatch") };
                    res = cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Coordsys")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone());
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    (res, _) = List::deleteMemberOnTrue(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Coordsys")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Coordsys" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context, res, p) => {
                    let mut coord: Arc<Absyn::ElementArg>;
                    let mut res = (*res).clone();
                    res = cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Coordsys")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone());
                    coord = getCoordSysAnn(listAppend(res.clone(), rest.clone()), p.clone())?;
                    res = cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("Diagram")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![coord.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: mod_info.clone() }), cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("Icon")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![coord.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: mod_info.clone() }), res.clone()));
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    (res, _) = List::deleteMemberOnTrue(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Coordsys")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: mod_info.clone() }), res.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info: mod_info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x1, tail: Deref @ metamodelica::List::Cons { head: y1, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x2, tail: Deref @ metamodelica::List::Cons { head: y2, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Nil } } }, info }, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, eachPrefix: e, finalPrefix: fi }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Coordsys", tail: _ }, res, p) => {
                    let mut res = (*res).clone();
                    res = cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("extent")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::ARRAY { arrayExp: list![x1.clone(), y1.clone()] }), Arc::new(Absyn::Exp::ARRAY { arrayExp: list![x2.clone(), y2.clone()] })] }), info: info.clone() }) })), comment: com.clone(), info: mod_info.clone() }), res.clone());
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "grid" }, .. }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "component" }, .. }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Window" }, .. }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Terminal" }, .. }, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: arg, tail: rest }, context, res, p) => {
                    let mut res = (*res).clone();
                    res = transformClassAnnList(rest.clone(), context.clone(), res.clone(), p.clone())?;
                    Ok(cons(arg.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArgs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isLayerAnnInList(mut inList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Diagram" }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Icon" }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut res: bool = false;
            res = isLayerAnnInList(rest.clone());
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getCoordSysAnn(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inProgram: Absyn::Program) -> Result<Arc<Absyn::ElementArg>> {
    let mut coordSys: Arc<Absyn::ElementArg>;
    coordSys = 'mc: {
        let __mc_input = (inArgs.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("coordinateSystem")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("extent")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: -100 }), Arc::new(Absyn::Exp::INTEGER { value: -100 })] }), Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: 100 }), Arc::new(Absyn::Exp::INTEGER { value: 100 })] })] }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() })], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info, comment: com, modification: Some(Deref @ Absyn::Modification { eqMod, elementArgLst: args }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Coordsys" }, eachPrefix: e, finalPrefix: fi }, tail: _ }, p) => {
                    let mut args = (*args).clone();
                    args = transformClassAnnList(args.clone(), cons((literal!("Coordsys")).clone(), metamodelica::nil()), metamodelica::nil(), p.clone())?;
                    Ok(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("coordinateSystem")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: eqMod.clone() })), comment: com.clone(), info: info.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, p) => {
                    let mut res: Arc<Absyn::ElementArg>;
                    res = getCoordSysAnn(rest.clone(), p.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(coordSys)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn transAnnLstToCalls(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inCon: Context) -> Result<Arc<metamodelica::List<Arc<Absyn::Exp>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    outArgs = 'mc: {
        let __mc_input = (inArgs.clone(), inCon.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "Line" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Layer", tail: c }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut argRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut c = (*c).clone();
                    c = addContext(context.clone(), (literal!("Line")).clone());
                    argRes = transAnnLstToNamedArgs(args.clone(), c.clone())?;
                    let metamodelica::List::Nil = (List::select1(argRes.clone(), (std::sync::Arc::new(nameArgWithName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::NamedArg>, ArcStr) -> Result<bool> + 'static>), (literal!("color")).clone())) else { bail!("pattern mismatch") };
                    restRes = transAnnLstToCalls(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("Line")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: cons(Arc::new(Absyn::NamedArg { argName: (literal!("color")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: 0 }), Arc::new(Absyn::Exp::INTEGER { value: 0 }), Arc::new(Absyn::Exp::INTEGER { value: 255 })] }) }), argRes.clone()) }), typeVars: metamodelica::nil() }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: n }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Layer", tail: c }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut argRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut c = (*c).clone();
                    c = addContext(context.clone(), (n.clone()).clone());
                    let true = (isLinebasedGraphic(c.clone())) else { bail!("pattern mismatch") };
                    argRes = transAnnLstToNamedArgs(args.clone(), c.clone())?;
                    let metamodelica::List::Nil = (List::select1(argRes.clone(), (std::sync::Arc::new(nameArgWithName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::NamedArg>, ArcStr) -> Result<bool> + 'static>), (literal!("lineColor")).clone())) else { bail!("pattern mismatch") };
                    restRes = transAnnLstToCalls(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (n.clone()).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: cons(Arc::new(Absyn::NamedArg { argName: (literal!("lineColor")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: 0 }), Arc::new(Absyn::Exp::INTEGER { value: 0 }), Arc::new(Absyn::Exp::INTEGER { value: 255 })] }) }), argRes.clone()) }), typeVars: metamodelica::nil() }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: n }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Layer", tail: c }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut argRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut c = (*c).clone();
                    c = addContext(context.clone(), (n.clone()).clone());
                    argRes = transAnnLstToNamedArgs(args.clone(), c.clone())?;
                    restRes = transAnnLstToCalls(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (n.clone()).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: argRes.clone() }), typeVars: metamodelica::nil() }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, context) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    res = transAnnLstToCalls(rest.clone(), context.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArgs)
}

fn nameArgWithName(mut narg: Arc<Absyn::NamedArg>, mut argName: ArcStr) -> Result<bool> {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &((narg.clone(), argName.clone())) {
        (Deref @ Absyn::NamedArg { argName: name, argValue: _ }, _) => {
            res = name.clone() == argName.clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn transAnnLstToNamedArgs(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inCon: Context) -> Result<Arc<metamodelica::List<Arc<Absyn::NamedArg>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    outArgs = 'mc: {
        let __mc_input = (inArgs.clone(), inCon.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x1, tail: Deref @ metamodelica::List::Cons { head: y1, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: x2, tail: Deref @ metamodelica::List::Cons { head: y2, tail: Deref @ metamodelica::List::Nil } }, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "extent" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("extent")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::ARRAY { arrayExp: list![x1.clone(), y1.clone()] }), Arc::new(Absyn::Exp::ARRAY { arrayExp: list![x2.clone(), y2.clone()] })] }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "style" }, .. }, tail: rest }, context) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut argRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut args = (*args).clone();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    args = cleanStyleAttrs(args.clone(), metamodelica::nil(), context.clone())?;
                    argRes = transAnnLstToNamedArgs(args.clone(), context.clone())?;
                    res = listAppend(argRes.clone(), restRes.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Text", tail: _ }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut color1: i32 = 0;
                    let mut color2: i32 = 0;
                    let mut color3: i32 = 0;
                    (color1, color2, color3) = getMappedColor(x.clone());
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("fillColor")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: color1.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color2.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color3.clone() })] }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut color1: i32 = 0;
                    let mut color2: i32 = 0;
                    let mut color3: i32 = 0;
                    (color1, color2, color3) = getMappedColor(x.clone());
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("color")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: color1.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color2.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color3.clone() })] }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut color1: i32 = 0;
                    let mut color2: i32 = 0;
                    let mut color3: i32 = 0;
                    (color1, color2, color3) = getMappedColor(x.clone());
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("lineColor")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: color1.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color2.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color3.clone() })] }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut color1: i32 = 0;
                    let mut color2: i32 = 0;
                    let mut color3: i32 = 0;
                    (color1, color2, color3) = getMappedColor(x.clone());
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("fillColor")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: color1.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color2.clone() }), Arc::new(Absyn::Exp::INTEGER { value: color3.clone() })] }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut val: ArcStr = arcstr::literal!("");
                    val = ((patternMapList.clone()).get(x.clone() + 1)?).clone();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("pattern")).clone(), argValue: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("LinePattern")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (val.clone()).clone(), subscripts: metamodelica::nil() }) }) }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut val: ArcStr = arcstr::literal!("");
                    val = ((fillPatternMapList.clone()).get(x.clone() + 1)?).clone();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("fillPattern")).clone(), argValue: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("FillPattern")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (val.clone()).clone(), subscripts: metamodelica::nil() }) }) }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut thick: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    thick = (thicknessMapList.clone()).get(x.clone())?;
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    s = (realString(thick.clone())).clone();
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("thickness")).clone(), argValue: Arc::new(Absyn::Exp::REAL { value: (s.clone()).clone() }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut thick: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    thick = (thicknessMapList.clone()).get(x.clone())?;
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    s = (realString(thick.clone())).clone();
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("lineThickness")).clone(), argValue: Arc::new(Absyn::Exp::REAL { value: (s.clone()).clone() }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut val: ArcStr = arcstr::literal!("");
                    val = ((gradientMapList.clone()).get(x.clone() + 1)?).clone();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("fillPattern")).clone(), argValue: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("FillPattern")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (val.clone()).clone(), subscripts: metamodelica::nil() }) }) }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("smooth")).clone(), argValue: exp.clone() }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: x }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "arrow" }, .. }, tail: rest }, context) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut arrows: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut val1: ArcStr = arcstr::literal!("");
                    let mut val2: ArcStr = arcstr::literal!("");
                    arrows = (arrowMapList.clone()).get(x.clone() + 1)?;
                    val1 = ((arrows.clone()).get(1)?).clone();
                    val2 = ((arrows.clone()).get(2)?).clone();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("arrow")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("Arrow")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (val1.clone()).clone(), subscripts: metamodelica::nil() }) }) }), Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("Arrow")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (val2.clone()).clone(), subscripts: metamodelica::nil() }) }) })] }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "textStyle" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Text", tail: _ }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("textStyle")).clone(), argValue: exp.clone() }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "font" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Text", tail: _ }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("font")).clone(), argValue: exp.clone() }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "string" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Text", tail: _ }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("textString")).clone(), argValue: exp.clone() }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "name" }, .. }, tail: rest }, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Bitmap", tail: _ }) => {
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("fileName")).clone(), argValue: exp.clone() }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::MATRIX { matrix: expMatrix }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "points" }, .. }, tail: rest }, context) => {
                    let mut expLst: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut restRes: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    expLst = List::map(expMatrix.clone(), (std::sync::Arc::new(fnptr!(matrixToArray, Arc<metamodelica::List<Arc<Absyn::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Arc<Absyn::Exp>> + 'static>));
                    restRes = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(cons(Arc::new(Absyn::NamedArg { argName: (literal!("points")).clone(), argValue: Arc::new(Absyn::Exp::ARRAY { arrayExp: expLst.clone() }) }), restRes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, context) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    res = transAnnLstToNamedArgs(rest.clone(), context.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArgs)
}

fn cleanStyleAttrs(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut resultList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inCon: Context) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outArgs = 'mc: {
        let __mc_input = (inArgs.clone(), resultList.clone(), inCon.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, context) => {
                    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = outArgs.clone();
                    let true = (isLinebasedGraphic(context.clone())) else { bail!("pattern mismatch") };
                    let metamodelica::List::Nil = (List::select(inArgs.clone(), (std::sync::Arc::new(fnptr!(isLineColorModifier, Arc<Absyn::ElementArg>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    outArgs = cleanStyleAttrs2(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("lineColor")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: 0 }), Arc::new(Absyn::Exp::INTEGER { value: 0 }), Arc::new(Absyn::Exp::INTEGER { value: 255 })] }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), inArgs.clone()), resultList.clone(), context.clone())?;
                    Ok(outArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, context) => {
                    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = outArgs.clone();
                    let true = (isLineGraphic(context.clone())) else { bail!("pattern mismatch") };
                    let metamodelica::List::Nil = (List::select(inArgs.clone(), (std::sync::Arc::new(fnptr!(isLineColorModifier, Arc<Absyn::ElementArg>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    outArgs = cleanStyleAttrs2(cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("color")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: 0 }), Arc::new(Absyn::Exp::INTEGER { value: 0 }), Arc::new(Absyn::Exp::INTEGER { value: 255 })] }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), inArgs.clone()), resultList.clone(), context.clone())?;
                    Ok(outArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = outArgs.clone();
                    outArgs = cleanStyleAttrs2(inArgs.clone(), resultList.clone(), inCon.clone())?;
                    Ok(outArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outArgs)
}

fn isLineColorModifier(mut arg: Arc<Absyn::ElementArg>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: _, eqMod: _ }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn isStyleModifier(mut arg: Arc<Absyn::ElementArg>) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "style" }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn isLinebasedGraphic(mut context: Context) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(context.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ "Rectangle", tail: _ } => true,
        Deref @ metamodelica::List::Cons { head: Deref @ "Ellipse", tail: _ } => true,
        Deref @ metamodelica::List::Cons { head: Deref @ "Polygon", tail: _ } => true,
        Deref @ metamodelica::List::Cons { head: Deref @ "Text", tail: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn isLineGraphic(mut context: Context) -> bool {
    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(context.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn cleanStyleAttrs2(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inResultList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inCon: Context) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outArgs = (::match_deref::match_deref! { match &((inArgs.clone(), inResultList.clone(), inCon.clone())) {
        (Deref @ metamodelica::List::Nil, resultList, _) => {
            resultList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, .. }, tail: rest }, resultList, context) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Rectangle", tail: _ }) if (!(isGradientInList(listAppend(rest.clone(), resultList.clone()))) && !(isFillPatternInList(listAppend(rest.clone(), resultList.clone())))) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = insertFillPatternInList(resultList.clone());
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Ellipse", tail: _ }) if (!(isGradientInList(listAppend(rest.clone(), resultList.clone()))) && !(isFillPatternInList(listAppend(rest.clone(), resultList.clone())))) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = insertFillPatternInList(resultList.clone());
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Polygon", tail: _ }) if (!(isGradientInList(listAppend(rest.clone(), resultList.clone()))) && !(isFillPatternInList(listAppend(rest.clone(), resultList.clone())))) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = insertFillPatternInList(resultList.clone());
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Rectangle", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Ellipse", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Polygon", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Rectangle", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Ellipse", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Polygon", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Rectangle", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Ellipse", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Polygon", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Bitmap", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, .. }, tail: rest }, resultList, context) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::INTEGER { value: 0 }, .. }, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, .. }, tail: rest }, resultList, context) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Rectangle", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut rest = (*rest).clone();
            let mut resultList = (*resultList).clone();
            rest = removeFillPatternInList(rest.clone());
            resultList = removeFillPatternInList(resultList.clone());
            rest = setDefaultLineInList(rest.clone());
            resultList = setDefaultLineInList(resultList.clone());
            (rest, resultList) = setDefaultFillColor(rest.clone(), resultList.clone())?;
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Ellipse", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut rest = (*rest).clone();
            let mut resultList = (*resultList).clone();
            rest = removeFillPatternInList(rest.clone());
            resultList = removeFillPatternInList(resultList.clone());
            rest = setDefaultLineInList(rest.clone());
            resultList = setDefaultLineInList(resultList.clone());
            (rest, resultList) = setDefaultFillColor(rest.clone(), resultList.clone())?;
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Polygon", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "arrow" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Line", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "textStyle" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Text", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "font" }, .. }, tail: rest }, resultList, context @ Deref @ metamodelica::List::Cons { head: Deref @ "Text", tail: _ }) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            let mut resultList = (*resultList).clone();
            resultList = List::appendElt(arg.clone(), resultList.clone());
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, resultList, context) => {
            let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            outList = cleanStyleAttrs2(rest.clone(), resultList.clone(), context.clone())?;
            outList.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArgs)
}

fn insertFillPatternInList(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementArg>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outArgs = (::match_deref::match_deref! { match &(inArgs.clone()) {
        lst => {
            let mut lst = (*lst).clone();
            lst = cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("fillPattern")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::INTEGER { value: 1 }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), lst.clone());
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArgs
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isGradientInList(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inArgs.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "gradient" }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut res: bool = false;
            res = isGradientInList(rest.clone());
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isFillPatternInList(mut inArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> bool {
    let mut result: bool = false;
    result = (::match_deref::match_deref! { match &(inArgs.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut res: bool = false;
            res = isFillPatternInList(rest.clone());
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn removeFillPatternInList(mut inList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementArg>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillPattern" }, .. }, tail: rest } => {
            rest.clone()
        },
        Deref @ metamodelica::List::Cons { head: arg, tail: rest } => {
            let mut lst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            lst = removeFillPatternInList(rest.clone());
            cons(arg.clone(), lst.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outList
}

fn setDefaultFillColor(mut oldList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut transformedList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>)> {
    let mut oList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut tList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    (oList, tList) = 'mc: {
        let __mc_input = (oldList.clone(), transformedList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (oLst, tLst) => {
                    let mut tLst = (*tLst).clone();
                    let false = (isFillColorInList(listAppend(oLst.clone(), tLst.clone()))) else { bail!("pattern mismatch") };
                    tLst = cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (literal!("fillColor")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::INTEGER { value: 3 }), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), tLst.clone());
                    Ok((oLst.clone(), tLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((oldList.clone(), transformedList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oList, tList))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isFillColorInList(mut inList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fillColor" }, .. }, tail: _ } => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            isFillColorInList(rest.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn setDefaultLineInList(mut inList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementArg>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inList.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "thickness" }, .. }, tail: rest } => {
            let mut lst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            lst = setDefaultLineInList(rest.clone());
            lst.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "pattern" }, .. }, tail: rest } => {
            let mut lst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            lst = setDefaultLineInList(rest.clone());
            lst.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { info, comment: com, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: Deref @ "color" }, eachPrefix: e, finalPrefix: fi }, tail: rest } => {
            let mut lst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            lst = setDefaultLineInList(rest.clone());
            cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fi.clone(), eachPrefix: e.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("color")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(Absyn::Exp::INTEGER { value: 0 }), info: Absyn::dummyInfo.clone() }) })), comment: com.clone(), info: info.clone() }), lst.clone())
        },
        Deref @ metamodelica::List::Cons { head: arg, tail: rest } => {
            let mut lst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
            lst = setDefaultLineInList(rest.clone());
            cons(arg.clone(), lst.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outList
}

fn getMappedColor(mut inColor: i32) -> (i32, i32, i32) {
    let mut color1: i32 = 0;
    let mut color2: i32 = 0;
    let mut color3: i32 = 0;
    (color1, color2, color3) = (match inColor.clone() {
        mut color => {
            let mut rcol: rgbColor = metamodelica::nil();
            rcol = (colorMapList.clone()).get(color.clone() + 1).unwrap();
            color1 = (rcol.clone()).get(1).unwrap();
            color2 = (rcol.clone()).get(2).unwrap();
            color3 = (rcol.clone()).get(3).unwrap();
            (color1.clone(), color2.clone(), color3.clone())
        },
    });
    (color1, color2, color3)
}

fn matrixToArray(mut inLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Arc<Absyn::Exp> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    outExp = Arc::new(Absyn::Exp::ARRAY { arrayExp: inLst.clone() });
    outExp
}

/*
protected function getValueFromIntExp

  input Absyn.Exp intExpr;
  output Integer value;
algorithm
  value := match(intExpr)
    local
      Integer val;
    case(Absyn.INTEGER(value = val))
      then val;

    case(Absyn.UNARY(exp = Absyn.INTEGER(value = val)))
      then (-val);
  end match;
end getValueFromIntExp;

protected function getValueFromRealExp
  input Absyn.Exp realExpr;
  output Real value;
algorithm
  value := match(realExpr)
    local
      Real val;
    case(Absyn.REAL(value = val))
      then val;
    case(Absyn.UNARY(exp = Absyn.REAL(value = val)))
      then -val;
  end match;
end getValueFromRealExp;  */
fn getValueFromExp(mut expr: Arc<Absyn::Exp>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    value = (::match_deref::match_deref! { match &(expr.clone()) {
        Deref @ Absyn::Exp::REAL { value: realVal } => {
            stringReal((realVal.clone()).clone())?
        },
        Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::REAL { value: realVal }, .. } => {
            -(stringReal((realVal.clone()).clone())?)
        },
        Deref @ Absyn::Exp::INTEGER { value: intVal } => {
            intReal(intVal.clone())
        },
        Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::INTEGER { value: intVal }, .. } => {
            -(intReal(intVal.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

fn addContext(mut inList: Arc<metamodelica::List<ArcStr>>, mut newCon: ArcStr) -> Arc<metamodelica::List<ArcStr>> {
    let mut outList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &((inList.clone(), newCon.clone())) {
        (strLst, r#str) => {
            cons((r#str.clone()).clone(), strLst.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outList
}

pub type Context = Arc<metamodelica::List<ArcStr>>;

pub type rgbColor = Arc<metamodelica::List<i32>>;

pub type rgbColorMapList = Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;

pub static colorMapList: std::sync::LazyLock<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> = std::sync::LazyLock::new(|| { list![list![0, 0, 0], list![255, 0, 0], list![0, 255, 0], list![0, 0, 255], list![0, 255, 255], list![255, 0, 255], list![255, 255, 0], list![255, 255, 255], list![192, 192, 192], list![160, 160, 160], list![128, 128, 128], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![235, 235, 235], list![240, 255, 255], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![0, 0, 0], list![255, 0, 0], list![191, 0, 0], list![255, 127, 127], list![223, 159, 159], list![255, 127, 0], list![191, 95, 0], list![255, 191, 127], list![223, 191, 159], list![255, 255, 0], list![191, 191, 0], list![255, 255, 127], list![223, 223, 159], list![127, 255, 0], list![95, 191, 0], list![191, 255, 127], list![191, 223, 159], list![0, 255, 0], list![0, 191, 0], list![127, 255, 127], list![159, 223, 159], list![0, 255, 127], list![0, 191, 95], list![127, 255, 191], list![159, 223, 191], list![0, 255, 255], list![0, 191, 191], list![127, 255, 255], list![159, 223, 223], list![0, 127, 255], list![0, 95, 191], list![127, 191, 255], list![159, 191, 223], list![0, 0, 255], list![0, 0, 191], list![127, 127, 255], list![159, 159, 223], list![127, 0, 255], list![95, 0, 191], list![191, 127, 255], list![191, 159, 223], list![255, 0, 255], list![191, 0, 191], list![255, 127, 255], list![223, 159, 223], list![255, 0, 127], list![191, 0, 95], list![255, 127, 191], list![223, 159, 191]] });

pub const None: &'static str = "None";

pub const Solid: &'static str = "Solid";

pub const Horizontal: &'static str = "Horizontal";

pub const Vertical: &'static str = "Vertical";

pub const Cross: &'static str = "Cross";

pub const Forward: &'static str = "Forward";

pub const Backward: &'static str = "Backward";

pub const CrossDiag: &'static str = "CrossDiag";

pub const HorizontalCylinder: &'static str = "HorizontalCylinder";

pub const VerticalCylinder: &'static str = "VerticalCylinder";

pub const Sphere: &'static str = "Sphere";

pub const Dash: &'static str = "Dash";

pub const Dot: &'static str = "Dot";

pub const DashDot: &'static str = "DashDot";

pub const DashDotDot: &'static str = "DashDotDot";

pub const Filled: &'static str = "Filled";

pub const Half: &'static str = "Half";

pub static fillPatternMapList: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(arcstr::literal!(None)).clone(), (arcstr::literal!(Solid)).clone(), (arcstr::literal!(None)).clone(), (arcstr::literal!(None)).clone(), (arcstr::literal!(None)).clone(), (arcstr::literal!(Horizontal)).clone(), (arcstr::literal!(Vertical)).clone(), (arcstr::literal!(Forward)).clone(), (arcstr::literal!(Backward)).clone(), (arcstr::literal!(Cross)).clone(), (arcstr::literal!(CrossDiag)).clone()] });

pub static gradientMapList: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(arcstr::literal!(None)).clone(), (arcstr::literal!(VerticalCylinder)).clone(), (arcstr::literal!(HorizontalCylinder)).clone(), (arcstr::literal!(Sphere)).clone()] });

pub static patternMapList: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(arcstr::literal!(None)).clone(), (arcstr::literal!(Solid)).clone(), (arcstr::literal!(Dash)).clone(), (arcstr::literal!(Dot)).clone(), (arcstr::literal!(DashDot)).clone(), (arcstr::literal!(DashDotDot)).clone()] });

pub static thicknessMapList: std::sync::LazyLock<Arc<metamodelica::List<metamodelica::Real>>> = std::sync::LazyLock::new(|| { list![metamodelica::OrderedFloat(0.25_f64), metamodelica::OrderedFloat(0.5_f64), metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(1.0_f64)] });

pub static arrowMapList: std::sync::LazyLock<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> = std::sync::LazyLock::new(|| { list![list![(arcstr::literal!(None)).clone(), (arcstr::literal!(None)).clone()], list![(arcstr::literal!(None)).clone(), (arcstr::literal!(Filled)).clone()], list![(arcstr::literal!(Filled)).clone(), (arcstr::literal!(None)).clone()], list![(arcstr::literal!(Filled)).clone(), (arcstr::literal!(Filled)).clone()], list![(arcstr::literal!(None)).clone(), (arcstr::literal!(Half)).clone()]] });

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fixPaths(mut inPath1: Arc<Absyn::Path>, mut inPath2: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = (inPath1.clone(), inPath2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ip1, ip2) => {
                    let mut p1: Arc<Absyn::Path>;
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut out: Arc<Absyn::Path>;
                    str1 = (AbsynUtil::pathLastIdent(ip1.clone())?).clone();
                    str2 = (AbsynUtil::pathFirstIdent(ip2.clone())?).clone();
                    let false = (stringEq((str1.clone()).clone(), (str2.clone()).clone())) else { bail!("pattern mismatch") };
                    p1 = AbsynUtil::stripLast(ip1.clone())?;
                    out = fixPaths(p1.clone(), ip2.clone())?;
                    Ok(out.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ip1, ip2) => {
                    let mut p1: Arc<Absyn::Path>;
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut out: Arc<Absyn::Path>;
                    str1 = (AbsynUtil::pathLastIdent(ip1.clone())?).clone();
                    str2 = (AbsynUtil::pathFirstIdent(ip2.clone())?).clone();
                    let true = (stringEq((str1.clone()).clone(), (str2.clone()).clone())) else { bail!("pattern mismatch") };
                    p1 = AbsynUtil::stripLast(ip1.clone())?;
                    out = AbsynUtil::joinPaths(p1.clone(), ip2.clone())?;
                    Ok(out.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inPath2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

