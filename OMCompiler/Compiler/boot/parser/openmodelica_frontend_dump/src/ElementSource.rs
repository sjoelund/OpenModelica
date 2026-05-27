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

use crate::ExpressionBasics;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

pub fn mergeSources(mut src1: Arc<DAE::ElementSource>, mut src2: Arc<DAE::ElementSource>) -> Result<Arc<DAE::ElementSource>> {
    let mut mergedSrc: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    mergedSrc = (::match_deref::match_deref! { match &((src1.clone(), src2.clone())) {
        (Deref @ DAE::ElementSource { info, partOfLst: partOfLst1, instance: instanceOpt1, connectEquationOptLst: connectEquationOptLst1, typeLst: typeLst1, operations: operations1, comment: comment1 }, Deref @ DAE::ElementSource { info: _, partOfLst: partOfLst2, instance: instanceOpt2, connectEquationOptLst: connectEquationOptLst2, typeLst: typeLst2, operations: operations2, comment: comment2 }) => {
            let mut p: Arc<metamodelica::List<Absyn::Within>> = metamodelica::nil();
            let mut i: Arc<DAE::ComponentPrefix> = Arc::new(DAE::ComponentPrefix::NOCOMPPRE);
            let mut c: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
            let mut t: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut o: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
            let mut comment: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
            p = List::union(partOfLst1.clone(), partOfLst2.clone());
            i = (::match_deref::match_deref! { match &(instanceOpt1.clone()) {
        Deref @ DAE::ComponentPrefix::NOCOMPPRE => instanceOpt2.clone(),
        _ => instanceOpt1.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            c = List::union(connectEquationOptLst1.clone(), connectEquationOptLst2.clone());
            t = List::union(typeLst1.clone(), typeLst2.clone());
            o = listAppend(operations1.clone(), operations2.clone());
            comment = List::union(comment1.clone(), comment2.clone());
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: p.clone(), instance: i.clone(), connectEquationOptLst: c.clone(), typeLst: t.clone(), operations: o.clone(), comment: comment.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(mergedSrc)
}

pub fn addCommentToSource(mut source: Arc<DAE::ElementSource>, mut commentIn: Option<Arc<SCode::Comment>>) -> Arc<DAE::ElementSource> {
    let mut source: Arc<DAE::ElementSource> = source;
    source = (::match_deref::match_deref! { match &((source.clone(), commentIn.clone())) {
        (Deref @ DAE::ElementSource { info: _, partOfLst: _, instance: _, connectEquationOptLst: _, typeLst: _, operations: _, comment: _ }, Some(comment)) => {
            assign_field!(source.comment = cons(comment.clone(), source.comment.clone()));
            source.clone()
        },
        _ => {
            source.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    source
}

pub fn createElementSource(mut fileInfo: SourceInfo, mut partOf: Option<Arc<Absyn::Path>>, mut prefix: DAE::Prefix, mut connectEquation: (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut path: Arc<Absyn::Path>;
    source = Arc::new(DAE::ElementSource { info: fileInfo.clone(), partOfLst: (::match_deref::match_deref! { match &(partOf.clone()) {
        None => metamodelica::nil(),
        Some(path) => list![Absyn::Within::WITHIN { path: path.clone() }],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }), instance: (match prefix.clone() {
        DAE::Prefix::NOPRE => Arc::new(openmodelica_frontend_types::DAE::ComponentPrefix::NOCOMPPRE),
        DAE::Prefix::PREFIX { .. } => var_field!(prefix.compPre, DAE::Prefix::PREFIX).clone(),
        _ => bail!("match: no arm matched"),
    }), connectEquationOptLst: (::match_deref::match_deref! { match &(connectEquation.clone()) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "", .. }, _) => metamodelica::nil(),
        _ => list![connectEquation.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }), typeLst: metamodelica::nil(), operations: metamodelica::nil(), comment: metamodelica::nil() });
    Ok(source)
}

pub fn addAdditionalComment(mut source: Arc<DAE::ElementSource>, mut message: ArcStr) -> Result<Arc<DAE::ElementSource>> {
    let mut outSource: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    outSource = (::match_deref::match_deref! { match &((source.clone(), message.clone())) {
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment }, _) => {
            let mut b: bool = false;
            let mut c: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
            let mut comment = (*comment).clone();
            c = Arc::new(SCode::Comment { annotation_: None, comment: Some((message.clone()).clone()) });
            b = listMember(c.clone(), comment.clone());
            comment = if (b.clone()) {comment.clone()} else {cons(c.clone(), comment.clone())};
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: operations.clone(), comment: comment.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSource)
}

pub fn addAnnotation(mut source: Arc<DAE::ElementSource>, mut comment: Arc<SCode::Comment>) -> Arc<DAE::ElementSource> {
    let mut outSource: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    outSource = (::match_deref::match_deref! { match &((source.clone(), comment.clone())) {
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment: commentLst }, Deref @ SCode::Comment { annotation_: Some(_), .. }) => {
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: operations.clone(), comment: cons(comment.clone(), commentLst.clone()) })
        },
        _ => {
            source.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSource
}

pub fn getComments(mut source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<SCode::Comment>>>> {
    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
    outComments = (::match_deref::match_deref! { match &(source.clone()) {
        Deref @ DAE::ElementSource { comment, .. } => {
            comment.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComments)
}

pub fn getOptComment(mut source: Arc<DAE::ElementSource>) -> Result<Option<Arc<SCode::Comment>>> {
    let mut outComment: Option<Arc<SCode::Comment>> = None;
    if !(source.comment.clone().is_empty()) {
        outComment = Some(List::last(source.comment.clone())?);
    } else {
        outComment = None;
    }
    Ok(outComment)
}

pub fn addSymbolicTransformation(mut source: Arc<DAE::ElementSource>, mut op: Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source.clone(), op.clone())) {
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations: Deref @ metamodelica::List::Cons { head: Deref @ DAE::SymbolicOperation::SUBSTITUTION { substitutions: es1 @ Deref @ metamodelica::List::Cons { head: h1, tail: _ }, source: t1 }, tail: operations }, comment }, Deref @ DAE::SymbolicOperation::SUBSTITUTION { substitutions: es2, source: t2 }) if (ExpressionBasics::expEqual(t2.clone(), h1.clone())?) => {
            let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            es = listAppend(es2.clone(), es1.clone());
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: cons(Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: es.clone(), source: t1.clone() }), operations.clone()), comment: comment.clone() })
        },
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment }, _) => {
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: cons(op.clone(), operations.clone()), comment: comment.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn condAddSymbolicTransformation(mut cond: bool, mut source: Arc<DAE::ElementSource>, mut op: Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(cond.clone()) {
        return Ok(source);
    }
    source = addSymbolicTransformation(source.clone(), op.clone())?;
    Ok(source)
}

pub fn addSymbolicTransformationDeriveLst(mut source: Arc<DAE::ElementSource>, mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((explst1.clone(), explst2.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            source.clone()
        },
        (Deref @ metamodelica::List::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: exp2, tail: rexplst2 }) => {
            let mut op: Arc<DAE::SymbolicOperation>;
            op = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: DAE::crefTime().clone(), before: exp1.clone(), after: exp2.clone() });
            source = addSymbolicTransformation(source.clone(), op.clone())?;
            addSymbolicTransformationDeriveLst(source.clone(), rexplst1.clone(), rexplst2.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationFlattenedEqs(mut source: Arc<DAE::ElementSource>, mut elt: Arc<DAE::Element>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source.clone(), elt.clone())) {
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations: Deref @ metamodelica::List::Cons { head: Deref @ DAE::SymbolicOperation::FLATTEN { scode, dae: None }, tail: operations }, comment }, _) => {
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: cons(Arc::new(DAE::SymbolicOperation::FLATTEN { scode: scode.clone(), dae: Some(elt.clone()) }), operations.clone()), comment: comment.clone() })
        },
        (Deref @ DAE::ElementSource { info, .. }, _) => {
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Tried to add the flattened elements to the list of operations, but did not find the SCode equation")).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationSubstitutionLst(mut add: Arc<metamodelica::List<bool>>, mut source: Arc<DAE::ElementSource>, mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((add.clone(), explst1.clone(), explst2.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            source.clone()
        },
        (Deref @ metamodelica::List::Cons { head: true, tail: brest }, Deref @ metamodelica::List::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: exp2, tail: rexplst2 }) => {
            source = addSymbolicTransformationSubstitution(true, source.clone(), exp1.clone(), exp2.clone())?;
            addSymbolicTransformationSubstitutionLst(brest.clone(), source.clone(), rexplst1.clone(), rexplst2.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: false, tail: brest }, Deref @ metamodelica::List::Cons { head: _, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: _, tail: rexplst2 }) => {
            addSymbolicTransformationSubstitutionLst(brest.clone(), source.clone(), rexplst1.clone(), rexplst2.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationSubstitution(mut add: bool, mut source: Arc<DAE::ElementSource>, mut exp1: Arc<DAE::Exp>, mut exp2: Arc<DAE::Exp>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = condAddSymbolicTransformation(add.clone(), source.clone(), Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: list![exp2.clone()], source: exp1.clone() }))?;
    Ok(source)
}

pub fn addSymbolicTransformationSimplifyLst(mut add: Arc<metamodelica::List<bool>>, mut source: Arc<DAE::ElementSource>, mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((add.clone(), explst1.clone(), explst2.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            source.clone()
        },
        (Deref @ metamodelica::List::Cons { head: true, tail: brest }, Deref @ metamodelica::List::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: exp2, tail: rexplst2 }) => {
            source = addSymbolicTransformation(source.clone(), Arc::new(DAE::SymbolicOperation::SIMPLIFY { before: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: exp1.clone() }), after: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: exp2.clone() }) }))?;
            addSymbolicTransformationSimplifyLst(brest.clone(), source.clone(), rexplst1.clone(), rexplst2.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: false, tail: brest }, Deref @ metamodelica::List::Cons { head: _, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: _, tail: rexplst2 }) => {
            addSymbolicTransformationSimplifyLst(brest.clone(), source.clone(), rexplst1.clone(), rexplst2.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationSimplify(mut add: bool, mut source: Arc<DAE::ElementSource>, mut exp1: Arc<DAE::EquationExp>, mut exp2: Arc<DAE::EquationExp>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = condAddSymbolicTransformation(add.clone(), source.clone(), Arc::new(DAE::SymbolicOperation::SIMPLIFY { before: exp1.clone(), after: exp2.clone() }))?;
    Ok(source)
}

pub fn addSymbolicTransformationSolve(mut add: bool, mut source: Arc<DAE::ElementSource>, mut cr: Arc<DAE::ComponentRef>, mut exp1: Arc<DAE::Exp>, mut exp2: Arc<DAE::Exp>, mut exp: Arc<DAE::Exp>, mut asserts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    let mut op: Arc<DAE::SymbolicOperation>;
    let mut op1: Arc<DAE::SymbolicOperation>;
    let mut op2: Arc<DAE::SymbolicOperation>;
    if !(add.clone() && Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    op1 = Arc::new(DAE::SymbolicOperation::SOLVE { cr: cr.clone(), exp1: exp1.clone(), exp2: exp2.clone(), res: exp.clone(), assertConds: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut ass in (asserts.clone()).into_iter().cloned() {
            let __x = getAssertCond(ass.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
    op2 = Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr.clone(), exp: exp2.clone() });
    op = if (ExpressionBasics::expEqual(exp2.clone(), exp.clone())?) {op2.clone()} else {op1.clone()};
    source = addSymbolicTransformation(source.clone(), op.clone())?;
    Ok(source)
}

pub fn getAssertCond(mut stmt: Arc<DAE::Statement>) -> Result<Arc<DAE::Exp>> {
    let mut cond: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSERT { cond: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cond = __pa0.clone();
    Ok(cond)
}

pub fn getSymbolicTransformations(mut source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> {
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> = metamodelica::nil();
    ops = source.operations.clone();
    ops
}

pub fn getElementSource(mut element: Arc<DAE::Element>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    source = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::VAR { .. } => var_field!((*element).source, DAE::Element::VAR).clone(),
        Deref @ DAE::Element::DEFINE { .. } => var_field!((*element).source, DAE::Element::DEFINE).clone(),
        Deref @ DAE::Element::INITIALDEFINE { .. } => var_field!((*element).source, DAE::Element::INITIALDEFINE).clone(),
        Deref @ DAE::Element::EQUATION { .. } => var_field!((*element).source, DAE::Element::EQUATION).clone(),
        Deref @ DAE::Element::EQUEQUATION { .. } => var_field!((*element).source, DAE::Element::EQUEQUATION).clone(),
        Deref @ DAE::Element::ARRAY_EQUATION { .. } => var_field!((*element).source, DAE::Element::ARRAY_EQUATION).clone(),
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { .. } => var_field!((*element).source, DAE::Element::INITIAL_ARRAY_EQUATION).clone(),
        Deref @ DAE::Element::COMPLEX_EQUATION { .. } => var_field!((*element).source, DAE::Element::COMPLEX_EQUATION).clone(),
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => var_field!((*element).source, DAE::Element::INITIAL_COMPLEX_EQUATION).clone(),
        Deref @ DAE::Element::WHEN_EQUATION { .. } => var_field!((*element).source, DAE::Element::WHEN_EQUATION).clone(),
        Deref @ DAE::Element::IF_EQUATION { .. } => var_field!((*element).source, DAE::Element::IF_EQUATION).clone(),
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => var_field!((*element).source, DAE::Element::INITIAL_IF_EQUATION).clone(),
        Deref @ DAE::Element::INITIALEQUATION { .. } => var_field!((*element).source, DAE::Element::INITIALEQUATION).clone(),
        Deref @ DAE::Element::ALGORITHM { .. } => var_field!((*element).source, DAE::Element::ALGORITHM).clone(),
        Deref @ DAE::Element::INITIALALGORITHM { .. } => var_field!((*element).source, DAE::Element::INITIALALGORITHM).clone(),
        Deref @ DAE::Element::COMP { .. } => var_field!((*element).source, DAE::Element::COMP).clone(),
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => var_field!((*element).source, DAE::Element::EXTOBJECTCLASS).clone(),
        Deref @ DAE::Element::ASSERT { .. } => var_field!((*element).source, DAE::Element::ASSERT).clone(),
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => var_field!((*element).source, DAE::Element::INITIAL_ASSERT).clone(),
        Deref @ DAE::Element::TERMINATE { .. } => var_field!((*element).source, DAE::Element::TERMINATE).clone(),
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => var_field!((*element).source, DAE::Element::INITIAL_TERMINATE).clone(),
        Deref @ DAE::Element::REINIT { .. } => var_field!((*element).source, DAE::Element::REINIT).clone(),
        Deref @ DAE::Element::NORETCALL { .. } => var_field!((*element).source, DAE::Element::NORETCALL).clone(),
        Deref @ DAE::Element::CONSTRAINT { .. } => var_field!((*element).source, DAE::Element::CONSTRAINT).clone(),
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => var_field!((*element).source, DAE::Element::INITIAL_NORETCALL).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ElementSource.getElementSource failed: Element does not have a source")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub fn getStatementSource(mut stmt: Arc<DAE::Statement>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    source = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { .. } => var_field!((*stmt).source, DAE::Statement::STMT_ASSIGN).clone(),
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { .. } => var_field!((*stmt).source, DAE::Statement::STMT_TUPLE_ASSIGN).clone(),
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { .. } => var_field!((*stmt).source, DAE::Statement::STMT_ASSIGN_ARR).clone(),
        Deref @ DAE::Statement::STMT_IF { .. } => var_field!((*stmt).source, DAE::Statement::STMT_IF).clone(),
        Deref @ DAE::Statement::STMT_FOR { .. } => var_field!((*stmt).source, DAE::Statement::STMT_FOR).clone(),
        Deref @ DAE::Statement::STMT_PARFOR { .. } => var_field!((*stmt).source, DAE::Statement::STMT_PARFOR).clone(),
        Deref @ DAE::Statement::STMT_WHILE { .. } => var_field!((*stmt).source, DAE::Statement::STMT_WHILE).clone(),
        Deref @ DAE::Statement::STMT_WHEN { .. } => var_field!((*stmt).source, DAE::Statement::STMT_WHEN).clone(),
        Deref @ DAE::Statement::STMT_ASSERT { .. } => var_field!((*stmt).source, DAE::Statement::STMT_ASSERT).clone(),
        Deref @ DAE::Statement::STMT_TERMINATE { .. } => var_field!((*stmt).source, DAE::Statement::STMT_TERMINATE).clone(),
        Deref @ DAE::Statement::STMT_REINIT { .. } => var_field!((*stmt).source, DAE::Statement::STMT_REINIT).clone(),
        Deref @ DAE::Statement::STMT_NORETCALL { .. } => var_field!((*stmt).source, DAE::Statement::STMT_NORETCALL).clone(),
        Deref @ DAE::Statement::STMT_RETURN { .. } => var_field!((*stmt).source, DAE::Statement::STMT_RETURN).clone(),
        Deref @ DAE::Statement::STMT_BREAK { .. } => var_field!((*stmt).source, DAE::Statement::STMT_BREAK).clone(),
        Deref @ DAE::Statement::STMT_ARRAY_INIT { .. } => var_field!((*stmt).source, DAE::Statement::STMT_ARRAY_INIT).clone(),
        Deref @ DAE::Statement::STMT_FAILURE { .. } => var_field!((*stmt).source, DAE::Statement::STMT_FAILURE).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub use getElementSourceFileInfo as getInfo;

pub fn getElementSourceFileInfo(mut source: Arc<DAE::ElementSource>) -> SourceInfo {
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = source.info.clone();
    info
}

pub fn getElementSourceTypes(mut source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    pathLst = source.typeLst.clone();
    pathLst
}

pub fn getElementSourceInstances(mut source: Arc<DAE::ElementSource>) -> Arc<DAE::ComponentPrefix> {
    let mut instanceOpt: Arc<DAE::ComponentPrefix> = Arc::new(DAE::ComponentPrefix::NOCOMPPRE);
    instanceOpt = source.instance.clone();
    instanceOpt
}

pub fn getElementSourceConnects(mut source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>> {
    let mut connectEquationOptLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>> = metamodelica::nil();
    connectEquationOptLst = source.connectEquationOptLst.clone();
    connectEquationOptLst
}

pub fn getElementSourcePartOfs(mut source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<Absyn::Within>> {
    let mut withinLst: Arc<metamodelica::List<Absyn::Within>> = metamodelica::nil();
    withinLst = source.partOfLst.clone();
    withinLst
}

pub fn addElementSourcePartOf(mut source: Arc<DAE::ElementSource>, mut withinPath: Absyn::Within) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    assign_field!(source.partOfLst = cons(withinPath.clone(), source.partOfLst.clone()));
    Ok(source)
}

pub fn addElementSourcePartOfOpt(mut source: Arc<DAE::ElementSource>, mut classPathOpt: Option<Arc<Absyn::Path>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source.clone(), classPathOpt.clone())) {
        (_, None) => {
            source.clone()
        },
        (_, Some(classPath)) => {
            addElementSourcePartOf(source.clone(), Absyn::Within::WITHIN { path: classPath.clone() })?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addElementSourceFileInfo(mut source: Arc<DAE::ElementSource>, mut fileInfo: SourceInfo) -> Arc<DAE::ElementSource> {
    let mut outSource: Arc<DAE::ElementSource> = source.clone();
    assign_field!(outSource.info = fileInfo.clone());
    outSource
}

pub fn addElementSourceConnect(mut inSource: Arc<DAE::ElementSource>, mut connectEquationOpt: (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<Arc<DAE::ElementSource>> {
    let mut outSource: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    outSource = (::match_deref::match_deref! { match &(inSource.clone()) {
        Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment } => {
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: cons(connectEquationOpt.clone(), connectEquationOptLst.clone()), typeLst: typeLst.clone(), operations: operations.clone(), comment: comment.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSource)
}

pub fn addElementSourceType(mut source: Arc<DAE::ElementSource>, mut classPath: Arc<Absyn::Path>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source.clone(), classPath.clone())) {
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment }, _) => {
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: cons(classPath.clone(), typeLst.clone()), operations: operations.clone(), comment: comment.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addElementSourceInstanceOpt(mut source: Arc<DAE::ElementSource>, mut instanceOpt: Arc<DAE::ComponentPrefix>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    let () = (::match_deref::match_deref! { match &((source.clone(), instanceOpt.clone())) {
        (_, Deref @ DAE::ComponentPrefix::NOCOMPPRE) => (),
        (Deref @ DAE::ElementSource { .. }, _) => {
            assign_field!(source.instance = instanceOpt.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

