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
use crate::NFClassTree::ClassTree;
use crate::NFInst as Inst;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Error;
use openmodelica_util::ErrorTypes;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFImport {
    UNRESOLVED_IMPORT {
        imp: Absyn::Import,
        scope: Arc<InstNode::InstNode>,
        info: SourceInfo,
    },
    RESOLVED_IMPORT {
        node: Arc<InstNode::InstNode>,
        shortName: ArcStr,
        info: SourceInfo,
    },
    CONFLICTING_IMPORT {
        imp1: Arc<NFImport>,
        imp2: Arc<NFImport>,
    },
}
pub use self::NFImport::{UNRESOLVED_IMPORT,RESOLVED_IMPORT,CONFLICTING_IMPORT};
pub fn name(mut imp: Arc<NFImport>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(imp.clone()) {
        Deref @ UNRESOLVED_IMPORT { .. } => AbsynUtil::importName(var_field!((*imp).imp, NFImport::UNRESOLVED_IMPORT).clone())?,
        Deref @ RESOLVED_IMPORT { .. } => InstNode::name(var_field!((*imp).node, NFImport::RESOLVED_IMPORT).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(name)
}

pub fn info(mut imp: Arc<NFImport>) -> Result<SourceInfo> {
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = (::match_deref::match_deref! { match &(imp.clone()) {
        Deref @ UNRESOLVED_IMPORT { .. } => var_field!((*imp).info, NFImport::UNRESOLVED_IMPORT).clone(),
        Deref @ RESOLVED_IMPORT { .. } => var_field!((*imp).info, NFImport::RESOLVED_IMPORT).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(info)
}

pub fn resolve(mut imp: Arc<NFImport>) -> Result<(Arc<InstNode::InstNode>, bool, Arc<NFImport>)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut changed: bool = false;
    let mut outImport: Arc<NFImport>;
    (outImport, node, changed) = (::match_deref::match_deref! { match &(imp.clone()) {
        Deref @ UNRESOLVED_IMPORT { .. } => {
            (outImport, node) = instQualified(var_field!((*imp).imp, NFImport::UNRESOLVED_IMPORT).clone(), var_field!((*imp).scope, NFImport::UNRESOLVED_IMPORT).clone(), var_field!((*imp).info, NFImport::UNRESOLVED_IMPORT).clone())?;
            (outImport.clone(), node.clone(), true)
        },
        Deref @ RESOLVED_IMPORT { .. } => (imp.clone(), var_field!((*imp).node, NFImport::RESOLVED_IMPORT).clone(), false),
        Deref @ CONFLICTING_IMPORT { .. } => {
            printImportError(var_field!((*imp).imp1, NFImport::CONFLICTING_IMPORT).clone(), var_field!((*imp).imp2, NFImport::CONFLICTING_IMPORT).clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((node, changed, outImport))
}

pub fn resolveList(mut imps: metamodelica::Array<Arc<NFImport>>) -> Arc<metamodelica::List<Arc<NFImport>>> {
    let mut resolvedImps: Arc<metamodelica::List<Arc<NFImport>>> = metamodelica::nil();
    let __range0 = imps.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut imp in __range0 {
        if '__try1: {
            (_, _, imp) = unwrap_break_err!(resolve(imp.clone()), '__try1);
            resolvedImps = (::match_deref::match_deref! { match &(imp.clone()) {
        Deref @ UNRESOLVED_IMPORT { imp: Absyn::Import::UNQUAL_IMPORT { .. }, .. } => unwrap_break_err!(instUnqualified(imp.clone(), resolvedImps.clone()), '__try1),
        _ => cons(imp.clone(), resolvedImps.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    resolvedImps
}

pub fn instQualified(mut imp: Absyn::Import, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<(Arc<NFImport>, Arc<InstNode::InstNode>)> {
    let mut outImport: Arc<NFImport>;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut short_name: ArcStr = arcstr::literal!("");
    node = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => Lookup::lookupImport(var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone(), scope.clone(), info.clone())?,
        Absyn::Import::QUAL_IMPORT { .. } => Lookup::lookupImport(var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(), scope.clone(), info.clone())?,
        _ => bail!("match: no arm matched"),
    });
    short_name = ((match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => var_field!(imp.name, Absyn::Import::NAMED_IMPORT).clone(),
        _ => literal!(""),
    })).clone();
    outImport = Arc::new(NFImport::RESOLVED_IMPORT { node: node.clone(), shortName: (short_name.clone()).clone(), info: info.clone() });
    Ok((outImport, node))
}

pub fn instUnqualified(mut imp: Arc<NFImport>, mut imps: Arc<metamodelica::List<Arc<NFImport>>>) -> Result<Arc<metamodelica::List<Arc<NFImport>>>> {
    let mut imps: Arc<metamodelica::List<Arc<NFImport>>> = imps;
    let mut path: Arc<Absyn::Path>;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(imp.clone()) {
        Deref @ UNRESOLVED_IMPORT { info: __pa0, scope: __pa1, imp: Absyn::Import::UNQUAL_IMPORT { path: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    scope = __pa1.clone();
    path = __pa2.clone();
    node = Lookup::lookupImport(path.clone(), scope.clone(), info.clone())?;
    node = Inst::instPackage(node.clone(), NFInstContext::NO_CONTEXT.clone())?;
    tree = Class::classTree(InstNode::getClass(node.clone())?)?;
    let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ ClassTree::FLAT_TREE { .. } => {
            let __range0 = var_field!((*tree).classes, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut cls in __range0 {
                imps = cons(Arc::new(NFImport::RESOLVED_IMPORT { node: cls.clone(), shortName: (literal!("")).clone(), info: info.clone() }), imps.clone());
            }
            let __range1 = var_field!((*tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range1 {
                imps = cons(Arc::new(NFImport::RESOLVED_IMPORT { node: comp.clone(), shortName: (literal!("")).clone(), info: info.clone() }), imps.clone());
            }
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFImport.instUnqualified")); __mm_s.push_str(&*literal!(" got invalid class tree")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(imps)
}

pub fn printImportError(mut imp1: Arc<NFImport>, mut imp2: Arc<NFImport>) -> Result<()> {
    let mut err_msg: ErrorTypes::Message = <ErrorTypes::Message as ::std::default::Default>::default();
    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), info(imp1.clone())?)?;
    err_msg = (::match_deref::match_deref! { match &(imp2.clone()) {
        Deref @ UNRESOLVED_IMPORT { .. } => Error::MULTIPLE_QUALIFIED_IMPORTS_WITH_SAME_NAME.clone(),
        Deref @ RESOLVED_IMPORT { .. } => Error::IMPORT_SEVERAL_NAMES.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Error::addSourceMessage(err_msg.clone(), list![(name(imp2.clone())?).clone()], info(imp2.clone())?)?;
    Ok(())
}


