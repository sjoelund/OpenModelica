// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::Algorithm;
use crate::DAE;
use crate::Expression;
use crate::SCode;
use openmodelica_ast::Absyn;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

pub fn addAdditionalComment(source: Arc<DAE::ElementSource>, message: ArcStr) -> Result<Arc<DAE::ElementSource>> {
    let mut outSource: Arc<DAE::ElementSource>;
    outSource = (::match_deref::match_deref! { match &((source.clone(), message.clone())) {
        (Deref @ DAE::SOURCE { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment }, _) => {
            let mut b: bool;
            let mut c: Arc<SCode::Comment>;
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

pub fn addAnnotation(source: Arc<DAE::ElementSource>, comment: Arc<SCode::Comment>) -> Arc<DAE::ElementSource> {
    let mut outSource: Arc<DAE::ElementSource>;
    outSource = (::match_deref::match_deref! { match &((source.clone(), comment.clone())) {
        (Deref @ DAE::SOURCE { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment: commentLst }, Deref @ SCode::COMMENT { annotation_: Some(_), .. }) => Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: operations.clone(), comment: cons(comment.clone(), commentLst.clone()) }),
        _ => source.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSource
}

pub fn addCommentToSource(source: Arc<DAE::ElementSource>, commentIn: Option<Arc<SCode::Comment>>) -> Arc<DAE::ElementSource> {
    let mut source: Arc<DAE::ElementSource> = source;
    source = (::match_deref::match_deref! { match &((source.clone(), commentIn.clone())) {
        (Deref @ DAE::SOURCE { info: _, partOfLst: _, instance: _, connectEquationOptLst: _, typeLst: _, operations: _, comment: _ }, Some(comment)) => {
            let mut info: SourceInfo;
            let mut partOfLst1: Arc<metamodelica::List<Absyn::Within>>;
            let mut instanceOpt1: Arc<DAE::ComponentPrefix>;
            let mut connectEquationOptLst1: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>;
            let mut typeLst1: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut operations1: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut comment1: Arc<metamodelica::List<Arc<SCode::Comment>>>;
            let mut comment2: Arc<metamodelica::List<Arc<SCode::Comment>>>;
            assign_variant_field!(source => DAE::ElementSource::SOURCE; comment = cons(comment.clone(), source.comment.clone()));
            source.clone()
        },
        _ => source.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    source
}

pub fn addElementSourceConnect(inSource: Arc<DAE::ElementSource>, connectEquationOpt: (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<Arc<DAE::ElementSource>> {
    let mut outSource: Arc<DAE::ElementSource>;
    outSource = (::match_deref::match_deref! { match &(inSource.clone()) {
        Deref @ DAE::SOURCE { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment } => Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: cons(connectEquationOpt.clone(), connectEquationOptLst.clone()), typeLst: typeLst.clone(), operations: operations.clone(), comment: comment.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSource)
}

pub fn addElementSourceFileInfo(source: Arc<DAE::ElementSource>, fileInfo: SourceInfo) -> Arc<DAE::ElementSource> {
    let mut outSource: Arc<DAE::ElementSource> = source.clone();
    assign_field!(outSource.info = fileInfo.clone());
    outSource
}

pub fn addElementSourceInstanceOpt(source: Arc<DAE::ElementSource>, instanceOpt: Arc<DAE::ComponentPrefix>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    let () = (::match_deref::match_deref! { match &((source.clone(), instanceOpt.clone())) {
        (_, Deref @ DAE::NOCOMPPRE) => (),
        (Deref @ DAE::SOURCE { .. }, _) => {
            assign_variant_field!(source => DAE::ElementSource::SOURCE; instance = instanceOpt.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addElementSourcePartOf(source: Arc<DAE::ElementSource>, withinPath: Absyn::Within) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    assign_field!(source.partOfLst = cons(withinPath.clone(), source.partOfLst.clone()));
    Ok(source)
}

pub fn addElementSourcePartOfOpt(source: Arc<DAE::ElementSource>, classPathOpt: Option<Arc<Absyn::Path>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source.clone(), classPathOpt.clone())) {
        (_, None) => source.clone(),
        (_, Some(classPath)) => addElementSourcePartOf(source.clone(), Absyn::Within::WITHIN { path: classPath.clone() })?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addElementSourceType(source: Arc<DAE::ElementSource>, classPath: Arc<Absyn::Path>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source.clone(), classPath.clone())) {
        (Deref @ DAE::SOURCE { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment }, _) => Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: cons(classPath.clone(), typeLst.clone()), operations: operations.clone(), comment: comment.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformation(source: Arc<DAE::ElementSource>, op: Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source.clone(), op.clone())) {
        (Deref @ DAE::SOURCE { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations: Deref @ metamodelica::List::Cons { head: Deref @ DAE::SUBSTITUTION { substitutions: es1 @ Deref @ metamodelica::List::Cons { head: h1, tail: _ }, source: t1 }, tail: operations }, comment }, Deref @ DAE::SUBSTITUTION { substitutions: es2, source: t2 }) if (Expression::expEqual(t2.clone(), h1.clone())?) => {
            let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            es = listAppend(es2.clone(), es1.clone());
            Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: cons(Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: es.clone(), source: t1.clone() }), operations.clone()), comment: comment.clone() })
        },
        (Deref @ DAE::SOURCE { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment }, _) => Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: cons(op.clone(), operations.clone()), comment: comment.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationDeriveLst(source: Arc<DAE::ElementSource>, explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((explst1.clone(), explst2.clone())) {
        (Deref @ metamodelica::List::Nil, _) => source.clone(),
        (Deref @ metamodelica::List::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: exp2, tail: rexplst2 }) => {
            let mut op: Arc<DAE::SymbolicOperation>;
            op = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: DAE::crefTime.clone(), before: exp1.clone(), after: exp2.clone() });
            source = addSymbolicTransformation(source.clone(), op.clone())?;
            addSymbolicTransformationDeriveLst(source.clone(), rexplst1.clone(), rexplst2.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationFlattenedEqs(source: Arc<DAE::ElementSource>, elt: Arc<DAE::Element>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source.clone(), elt.clone())) {
        (Deref @ DAE::SOURCE { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations: Deref @ metamodelica::List::Cons { head: Deref @ DAE::FLATTEN { scode, dae: None }, tail: operations }, comment }, _) => Arc::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: cons(Arc::new(DAE::SymbolicOperation::FLATTEN { scode: scode.clone(), dae: Some(elt.clone()) }), operations.clone()), comment: comment.clone() }),
        (Deref @ DAE::SOURCE { info, .. }, _) => {
            let mut typeLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut partOfLst: Arc<metamodelica::List<Absyn::Within>>;
            let mut instanceOpt: Arc<DAE::ComponentPrefix>;
            let mut connectEquationOptLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>;
            let mut operations: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut h1: Arc<DAE::Exp>;
            let mut t1: Arc<DAE::Exp>;
            let mut t2: Arc<DAE::Exp>;
            let mut comment: Arc<metamodelica::List<Arc<SCode::Comment>>>;
            let mut scode: Arc<SCode::Equation>;
            let mut elts: Arc<metamodelica::List<Arc<DAE::Element>>>;
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Tried to add the flattened elements to the list of operations, but did not find the SCode equation")).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationSimplify(add: bool, source: Arc<DAE::ElementSource>, exp1: Arc<DAE::EquationExp>, exp2: Arc<DAE::EquationExp>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = condAddSymbolicTransformation(add.clone(), source.clone(), Arc::new(DAE::SymbolicOperation::SIMPLIFY { before: exp1.clone(), after: exp2.clone() }))?;
    Ok(source)
}

pub fn addSymbolicTransformationSimplifyLst(add: Arc<metamodelica::List<bool>>, source: Arc<DAE::ElementSource>, explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((add.clone(), explst1.clone(), explst2.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => source.clone(),
        (Deref @ metamodelica::List::Cons { head: true, tail: brest }, Deref @ metamodelica::List::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: exp2, tail: rexplst2 }) => {
            source = addSymbolicTransformation(source.clone(), Arc::new(DAE::SymbolicOperation::SIMPLIFY { before: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: exp1.clone() }), after: Arc::new(DAE::EquationExp::PARTIAL_EQUATION { exp: exp2.clone() }) }))?;
            addSymbolicTransformationSimplifyLst(brest.clone(), source.clone(), rexplst1.clone(), rexplst2.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: false, tail: brest }, Deref @ metamodelica::List::Cons { head: _, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: _, tail: rexplst2 }) => addSymbolicTransformationSimplifyLst(brest.clone(), source.clone(), rexplst1.clone(), rexplst2.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationSolve(add: bool, source: Arc<DAE::ElementSource>, cr: Arc<DAE::ComponentRef>, exp1: Arc<DAE::Exp>, exp2: Arc<DAE::Exp>, exp: Arc<DAE::Exp>, asserts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    let mut op: Arc<DAE::SymbolicOperation>;
    let mut op1: Arc<DAE::SymbolicOperation>;
    let mut op2: Arc<DAE::SymbolicOperation>;
    if !(add.clone() && Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    op1 = Arc::new(DAE::SymbolicOperation::SOLVE { cr: cr.clone(), exp1: exp1.clone(), exp2: exp2.clone(), res: exp.clone(), assertConds: {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for ass in (asserts.clone()).into_iter().cloned() {
            let __x = Algorithm::getAssertCond(ass.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
    op2 = Arc::new(DAE::SymbolicOperation::SOLVED { cr: cr.clone(), exp: exp2.clone() });
    op = if (Expression::expEqual(exp2.clone(), exp.clone())?) {op2.clone()} else {op1.clone()};
    source = addSymbolicTransformation(source.clone(), op.clone())?;
    Ok(source)
}

pub fn addSymbolicTransformationSubstitution(add: bool, source: Arc<DAE::ElementSource>, exp1: Arc<DAE::Exp>, exp2: Arc<DAE::Exp>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = condAddSymbolicTransformation(add.clone(), source.clone(), Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: list![exp2.clone()], source: exp1.clone() }))?;
    Ok(source)
}

pub fn addSymbolicTransformationSubstitutionLst(add: Arc<metamodelica::List<bool>>, source: Arc<DAE::ElementSource>, explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>, explst2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((add.clone(), explst1.clone(), explst2.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => source.clone(),
        (Deref @ metamodelica::List::Cons { head: true, tail: brest }, Deref @ metamodelica::List::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: exp2, tail: rexplst2 }) => {
            source = addSymbolicTransformationSubstitution(true, source.clone(), exp1.clone(), exp2.clone())?;
            addSymbolicTransformationSubstitutionLst(brest.clone(), source.clone(), rexplst1.clone(), rexplst2.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: false, tail: brest }, Deref @ metamodelica::List::Cons { head: _, tail: rexplst1 }, Deref @ metamodelica::List::Cons { head: _, tail: rexplst2 }) => addSymbolicTransformationSubstitutionLst(brest.clone(), source.clone(), rexplst1.clone(), rexplst2.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn condAddSymbolicTransformation(cond: bool, source: Arc<DAE::ElementSource>, op: Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource> = source;
    if !(cond.clone()) {
        return Ok(source);
    }
    source = addSymbolicTransformation(source.clone(), op.clone())?;
    Ok(source)
}

pub fn createElementSource(fileInfo: SourceInfo, partOf: Option<Arc<Absyn::Path>>, prefix: DAE::Prefix, connectEquation: (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource>;
    let mut path: Arc<Absyn::Path>;
    source = Arc::new(DAE::ElementSource { info: fileInfo.clone(), partOfLst: (match partOf.clone() {
        None => metamodelica::nil(),
        Some(mut path) => list![Absyn::Within::WITHIN { path: Arc::new(path.clone()) }],
    }), instance: (match prefix.clone() {
        DAE::NOPRE => Arc::new(crate::DAE::ComponentPrefix::NOCOMPPRE),
        DAE::PREFIX { .. } => var_field!(prefix.compPre, DAE::Prefix::PREFIX).clone(),
        _ => bail!("match: no arm matched"),
    }), connectEquationOptLst: (::match_deref::match_deref! { match &(connectEquation.clone()) {
        (Deref @ DAE::CREF_IDENT { ident: Deref @ "", .. }, _) => metamodelica::nil(),
        _ => list![connectEquation.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }), typeLst: metamodelica::nil(), operations: metamodelica::nil(), comment: metamodelica::nil() });
    Ok(source)
}

pub fn getComments(source: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<Arc<SCode::Comment>>>> {
    let mut outComments: Arc<metamodelica::List<Arc<SCode::Comment>>>;
    outComments = (::match_deref::match_deref! { match &(source.clone()) {
        Deref @ DAE::SOURCE { comment, .. } => comment.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComments)
}

pub fn getElementSource(element: Arc<DAE::Element>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource>;
    source = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::VAR { .. } => var_field!((*element).source, DAE::Element::VAR).clone(),
        Deref @ DAE::DEFINE { .. } => var_field!((*element).source, DAE::Element::DEFINE).clone(),
        Deref @ DAE::INITIALDEFINE { .. } => var_field!((*element).source, DAE::Element::INITIALDEFINE).clone(),
        Deref @ DAE::EQUATION { .. } => var_field!((*element).source, DAE::Element::EQUATION).clone(),
        Deref @ DAE::EQUEQUATION { .. } => var_field!((*element).source, DAE::Element::EQUEQUATION).clone(),
        Deref @ DAE::ARRAY_EQUATION { .. } => var_field!((*element).source, DAE::Element::ARRAY_EQUATION).clone(),
        Deref @ DAE::INITIAL_ARRAY_EQUATION { .. } => var_field!((*element).source, DAE::Element::INITIAL_ARRAY_EQUATION).clone(),
        Deref @ DAE::COMPLEX_EQUATION { .. } => var_field!((*element).source, DAE::Element::COMPLEX_EQUATION).clone(),
        Deref @ DAE::INITIAL_COMPLEX_EQUATION { .. } => var_field!((*element).source, DAE::Element::INITIAL_COMPLEX_EQUATION).clone(),
        Deref @ DAE::WHEN_EQUATION { .. } => var_field!((*element).source, DAE::Element::WHEN_EQUATION).clone(),
        Deref @ DAE::IF_EQUATION { .. } => var_field!((*element).source, DAE::Element::IF_EQUATION).clone(),
        Deref @ DAE::INITIAL_IF_EQUATION { .. } => var_field!((*element).source, DAE::Element::INITIAL_IF_EQUATION).clone(),
        Deref @ DAE::INITIALEQUATION { .. } => var_field!((*element).source, DAE::Element::INITIALEQUATION).clone(),
        Deref @ DAE::ALGORITHM { .. } => var_field!((*element).source, DAE::Element::ALGORITHM).clone(),
        Deref @ DAE::INITIALALGORITHM { .. } => var_field!((*element).source, DAE::Element::INITIALALGORITHM).clone(),
        Deref @ DAE::COMP { .. } => var_field!((*element).source, DAE::Element::COMP).clone(),
        Deref @ DAE::EXTOBJECTCLASS { .. } => var_field!((*element).source, DAE::Element::EXTOBJECTCLASS).clone(),
        Deref @ DAE::ASSERT { .. } => var_field!((*element).source, DAE::Element::ASSERT).clone(),
        Deref @ DAE::INITIAL_ASSERT { .. } => var_field!((*element).source, DAE::Element::INITIAL_ASSERT).clone(),
        Deref @ DAE::TERMINATE { .. } => var_field!((*element).source, DAE::Element::TERMINATE).clone(),
        Deref @ DAE::INITIAL_TERMINATE { .. } => var_field!((*element).source, DAE::Element::INITIAL_TERMINATE).clone(),
        Deref @ DAE::REINIT { .. } => var_field!((*element).source, DAE::Element::REINIT).clone(),
        Deref @ DAE::NORETCALL { .. } => var_field!((*element).source, DAE::Element::NORETCALL).clone(),
        Deref @ DAE::CONSTRAINT { .. } => var_field!((*element).source, DAE::Element::CONSTRAINT).clone(),
        Deref @ DAE::INITIAL_NORETCALL { .. } => var_field!((*element).source, DAE::Element::INITIAL_NORETCALL).clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ElementSource.getElementSource failed: Element does not have a source")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub fn getElementSourceConnects(source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>> {
    let mut connectEquationOptLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>;
    connectEquationOptLst = source.connectEquationOptLst.clone();
    connectEquationOptLst
}

pub fn getElementSourceFileInfo(source: Arc<DAE::ElementSource>) -> SourceInfo {
    let mut info: SourceInfo;
    info = source.info.clone();
    info
}

pub fn getElementSourceInstances(source: Arc<DAE::ElementSource>) -> Arc<DAE::ComponentPrefix> {
    let mut instanceOpt: Arc<DAE::ComponentPrefix>;
    instanceOpt = source.instance.clone();
    instanceOpt
}

pub fn getElementSourcePartOfs(source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<Absyn::Within>> {
    let mut withinLst: Arc<metamodelica::List<Absyn::Within>>;
    withinLst = source.partOfLst.clone();
    withinLst
}

pub fn getElementSourceTypes(source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
    pathLst = source.typeLst.clone();
    pathLst
}

pub use getElementSourceFileInfo as getInfo;

pub fn getOptComment(source: Arc<DAE::ElementSource>) -> Result<Option<Arc<SCode::Comment>>> {
    let mut outComment: Option<Arc<SCode::Comment>>;
    if !(source.comment.clone().is_empty()) {
        outComment = Some(List::last(source.comment.clone())?);
    } else {
        outComment = None;
    }
    Ok(outComment)
}

pub fn getStatementSource(stmt: Arc<DAE::Statement>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource>;
    source = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::STMT_ASSIGN { .. } => var_field!((*stmt).source, DAE::Statement::STMT_ASSIGN).clone(),
        Deref @ DAE::STMT_TUPLE_ASSIGN { .. } => var_field!((*stmt).source, DAE::Statement::STMT_TUPLE_ASSIGN).clone(),
        Deref @ DAE::STMT_ASSIGN_ARR { .. } => var_field!((*stmt).source, DAE::Statement::STMT_ASSIGN_ARR).clone(),
        Deref @ DAE::STMT_IF { .. } => var_field!((*stmt).source, DAE::Statement::STMT_IF).clone(),
        Deref @ DAE::STMT_FOR { .. } => var_field!((*stmt).source, DAE::Statement::STMT_FOR).clone(),
        Deref @ DAE::STMT_PARFOR { .. } => var_field!((*stmt).source, DAE::Statement::STMT_PARFOR).clone(),
        Deref @ DAE::STMT_WHILE { .. } => var_field!((*stmt).source, DAE::Statement::STMT_WHILE).clone(),
        Deref @ DAE::STMT_WHEN { .. } => var_field!((*stmt).source, DAE::Statement::STMT_WHEN).clone(),
        Deref @ DAE::STMT_ASSERT { .. } => var_field!((*stmt).source, DAE::Statement::STMT_ASSERT).clone(),
        Deref @ DAE::STMT_TERMINATE { .. } => var_field!((*stmt).source, DAE::Statement::STMT_TERMINATE).clone(),
        Deref @ DAE::STMT_REINIT { .. } => var_field!((*stmt).source, DAE::Statement::STMT_REINIT).clone(),
        Deref @ DAE::STMT_NORETCALL { .. } => var_field!((*stmt).source, DAE::Statement::STMT_NORETCALL).clone(),
        Deref @ DAE::STMT_RETURN { .. } => var_field!((*stmt).source, DAE::Statement::STMT_RETURN).clone(),
        Deref @ DAE::STMT_BREAK { .. } => var_field!((*stmt).source, DAE::Statement::STMT_BREAK).clone(),
        Deref @ DAE::STMT_ARRAY_INIT { .. } => var_field!((*stmt).source, DAE::Statement::STMT_ARRAY_INIT).clone(),
        Deref @ DAE::STMT_FAILURE { .. } => var_field!((*stmt).source, DAE::Statement::STMT_FAILURE).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(source)
}

pub fn getSymbolicTransformations(source: Arc<DAE::ElementSource>) -> Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>> {
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
    ops = source.operations.clone();
    ops
}

pub fn mergeSources(src1: Arc<DAE::ElementSource>, src2: Arc<DAE::ElementSource>) -> Result<Arc<DAE::ElementSource>> {
    let mut mergedSrc: Arc<DAE::ElementSource>;
    mergedSrc = (::match_deref::match_deref! { match &((src1.clone(), src2.clone())) {
        (Deref @ DAE::SOURCE { info, partOfLst: partOfLst1, instance: instanceOpt1, connectEquationOptLst: connectEquationOptLst1, typeLst: typeLst1, operations: operations1, comment: comment1 }, Deref @ DAE::SOURCE { info: _, partOfLst: partOfLst2, instance: instanceOpt2, connectEquationOptLst: connectEquationOptLst2, typeLst: typeLst2, operations: operations2, comment: comment2 }) => {
            let mut p: Arc<metamodelica::List<Absyn::Within>>;
            let mut i: Arc<DAE::ComponentPrefix>;
            let mut c: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>;
            let mut t: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut o: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
            let mut comment: Arc<metamodelica::List<Arc<SCode::Comment>>>;
            p = List::union(partOfLst1.clone(), partOfLst2.clone());
            i = (::match_deref::match_deref! { match &(instanceOpt1.clone()) {
        Deref @ DAE::NOCOMPPRE => instanceOpt2.clone(),
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

