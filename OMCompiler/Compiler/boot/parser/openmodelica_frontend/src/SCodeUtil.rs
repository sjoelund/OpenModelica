// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::AbsynUtil;
use crate::SCode;
use openmodelica_ast::Absyn;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn addElementToClass(inElement: Arc<SCode::Element>, inClassDef: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outClassDef: Arc<SCode::Element>;
    let mut cdef: Arc<SCode::ClassDef>;
    let __pa0 = ::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ SCode::CLASS { classDef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cdef = __pa0.clone();
    cdef = addElementToCompositeClassDef(inElement.clone(), cdef.clone())?;
    outClassDef = setClassDef(cdef.clone(), inClassDef.clone())?;
    Ok(outClassDef)
}

pub fn addElementToCompositeClassDef(element: Arc<SCode::Element>, classDef: Arc<SCode::ClassDef>) -> Result<Arc<SCode::ClassDef>> {
    let mut classDef: Arc<SCode::ClassDef> = classDef;
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::PARTS { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = cons(element.clone(), var_field!((*classDef).elementLst, SCode::ClassDef::PARTS).clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(classDef)
}

pub fn algorithmContainReinit(inAlg: Arc<SCode::Statement>) -> bool {
    let mut hasReinit: bool;
    hasReinit = (::match_deref::match_deref! { match &(inAlg.clone()) {
        Deref @ SCode::ALG_REINIT { .. } => true,
        Deref @ SCode::ALG_WHEN_A { branches: tpl_alg, .. } => {
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut algs: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut algs1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut algs2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut algs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            algs_lst = List::map(tpl_alg.clone(), Arc::new(fnptr!(Util::tuple22, _)));
            b = List::applyAndFold(algs_lst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), Arc::new(fnptr!(algorithmsContainReinit, Arc<metamodelica::List<Arc<SCode::Statement>>>)), false);
            b.clone()
        },
        Deref @ SCode::ALG_IF { elseBranch: algs2, elseIfBranch: tpl_alg, trueBranch: algs1, .. } => {
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut algs: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut algs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            b1 = algorithmsContainReinit(algs1.clone());
            algs_lst = List::map(tpl_alg.clone(), Arc::new(fnptr!(Util::tuple22, _)));
            b2 = List::applyAndFold(algs_lst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), Arc::new(fnptr!(algorithmsContainReinit, Arc<metamodelica::List<Arc<SCode::Statement>>>)), b1.clone());
            b3 = algorithmsContainReinit(algs2.clone());
            b = boolOr(b1.clone(), boolOr(b2.clone(), b3.clone()));
            b.clone()
        },
        Deref @ SCode::ALG_FOR { forBody: algs, .. } => {
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut algs1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut algs2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut algs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            let mut tpl_alg: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            b = algorithmsContainReinit(algs.clone());
            b.clone()
        },
        Deref @ SCode::ALG_WHILE { whileBody: algs, .. } => {
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut algs1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut algs2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut algs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            let mut tpl_alg: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            b = algorithmsContainReinit(algs.clone());
            b.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

fn algorithmEqual(alg1: Arc<SCode::AlgorithmSection>, alg2: Arc<SCode::AlgorithmSection>) -> bool {
    let mut equal: bool;
    equal = List::isEqualOnTrue(alg1.statements.clone(), alg2.statements.clone(), Arc::new(statementEqual));
    equal
}

pub fn algorithmsContainReinit(inAlgs: Arc<metamodelica::List<Arc<SCode::Statement>>>) -> bool {
    let mut hasReinit: bool;
    hasReinit = (::match_deref::match_deref! { match &(inAlgs.clone()) {
        _ => {
            let mut b: bool;
            b = List::applyAndFold(inAlgs.clone(), Arc::new(fnptr!(boolOr, bool, bool)), Arc::new(fnptr!(algorithmContainReinit, Arc<SCode::Statement>)), false);
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

pub fn annotationEqual(annotation1: Arc<SCode::Annotation>, annotation2: Arc<SCode::Annotation>) -> bool {
    let mut equal: bool = modEqual(annotation1.modification.clone(), annotation2.modification.clone()).unwrap();
    equal
}

pub fn appendAnnotationToComment(inAnnotation: Arc<SCode::Annotation>, inComment: Arc<SCode::Comment>, check_replace: bool) -> Result<Arc<SCode::Comment>> {
    fn isNotElem(r#mod: Arc<SCode::SubMod>, mods: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> bool {
        let mut b: bool = true;
        for m in &*mods.clone() {
            if r#mod.ident.clone() == m.ident.clone() {
                b = false;
                return b;
            }
        }
        b
    }

    let mut outComment: Arc<SCode::Comment>;
    outComment = (::match_deref::match_deref! { match &((inAnnotation.clone(), inComment.clone())) {
        (_, Deref @ SCode::COMMENT { annotation_: None, comment: cmt }) => Arc::new(SCode::Comment { annotation_: Some(inAnnotation.clone()), comment: cmt.clone() }),
        (Deref @ SCode::ANNOTATION { modification: Deref @ SCode::MOD { subModLst: mods1, .. } }, Deref @ SCode::COMMENT { annotation_: Some(SCode::ANNOTATION { modification: r#mod @ Deref @ SCode::MOD { .. } }), comment: cmt }) => {
            let mut r#mod = (*r#mod).clone();
            if !(check_replace.clone()) {
                r#mod.subModLst = listAppend(mods1.clone(), r#mod.subModLst.clone()); // TODO: unhandled field-assign shape
            } else {
                r#mod.subModLst = listAppend(mods1.clone(), List::filterOnTrue(r#mod.subModLst.clone(), Arc::new({ let __pe_b1 = mods1.clone(); move |__pe_a0| Ok(isNotElem(__pe_a0, __pe_b1.clone())) }))); // TODO: unhandled field-assign shape
            }
            Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: r#mod.clone() })), comment: cmt.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComment)
}

pub fn appendAnnotationToCommentOption(inAnnotation: Arc<SCode::Annotation>, inComment: Option<Arc<SCode::Comment>>, check_replace: bool) -> Result<Option<Arc<SCode::Comment>>> {
    let mut outComment: Option<Arc<SCode::Comment>>;
    outComment = (match inComment.clone() {
        Some(mut comment) => Some(appendAnnotationToComment(inAnnotation.clone(), comment.clone(), check_replace.clone())?),
        _ => Some(Arc::new(SCode::Comment { annotation_: Some(inAnnotation.clone()), comment: None })),
    });
    Ok(outComment)
}

fn arrayDimEqual(iad1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, iad2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<bool> {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (iad1.clone(), iad2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut ad1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut ad2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NOSUB, tail: ad1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NOSUB, tail: ad2 }) => {
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let true = (arrayDimEqual(ad1.clone(), ad2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::SUBSCRIPT { subscript: e1 }, tail: ad1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::SUBSCRIPT { subscript: e2 }, tail: ad2 }) => {
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    let true = (arrayDimEqual(ad1.clone(), ad2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut ad1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut ad2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

fn arraydimOptEqual(adopt1: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, adopt2: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> bool {
    let mut equal: bool;
    equal = (match (adopt1.clone(), adopt2.clone()) {
        (None, None) => true,
        (Some(mut lst1), Some(mut lst2)) => List::isEqualOnTrue(lst1.clone(), lst2.clone(), Arc::new(subscriptEqual)),
        _ => false,
    });
    equal
}

pub fn attrVariability(attr: SCode::Attributes) -> Result<SCode::Variability> {
    let mut var: SCode::Variability;
    var = (match attr.clone() {
        SCode::ATTR { variability: mut v, .. } => v.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(var)
}

pub fn attributesEqual(attr1: SCode::Attributes, attr2: SCode::Attributes) -> Result<bool> {
    let mut equal: bool;
    equal = arrayDimEqual(attr1.arrayDims.clone(), attr2.arrayDims.clone())? && attr1.connectorType.clone() == attr2.connectorType.clone() && parallelismEqual(attr1.parallelism.clone(), attr2.parallelism.clone()) && variabilityEqual(attr1.variability.clone(), attr2.variability.clone()) && AbsynUtil::directionEqual(attr1.direction.clone(), attr2.direction.clone()) && AbsynUtil::isFieldEqual(attr1.isField.clone(), attr2.isField.clone());
    Ok(equal)
}

pub fn boolEach(inBoolEach: bool) -> SCode::Each {
    let mut outEach: SCode::Each;
    outEach = (match inBoolEach.clone() {
        true => crate::SCode::Each::EACH,
        false => crate::SCode::Each::NOT_EACH,
    });
    outEach
}

pub fn boolEncapsulated(inBoolEncapsulated: bool) -> SCode::Encapsulated {
    let mut outEncapsulated: SCode::Encapsulated;
    outEncapsulated = (match inBoolEncapsulated.clone() {
        true => crate::SCode::Encapsulated::ENCAPSULATED,
        false => crate::SCode::Encapsulated::NOT_ENCAPSULATED,
    });
    outEncapsulated
}

pub fn boolFinal(inBoolFinal: bool) -> SCode::Final {
    let mut outFinal: SCode::Final;
    outFinal = if (inBoolFinal.clone()) {crate::SCode::Final::FINAL} else {crate::SCode::Final::NOT_FINAL};
    outFinal
}

pub fn boolFlow(inBoolFlow: bool) -> SCode::ConnectorType {
    let mut outFlow: SCode::ConnectorType;
    outFlow = (match inBoolFlow.clone() {
        true => crate::SCode::ConnectorType::FLOW,
        _ => crate::SCode::ConnectorType::POTENTIAL,
    });
    outFlow
}

pub fn boolPartial(inBoolPartial: bool) -> SCode::Partial {
    let mut outPartial: SCode::Partial;
    outPartial = (match inBoolPartial.clone() {
        true => crate::SCode::Partial::PARTIAL,
        false => crate::SCode::Partial::NOT_PARTIAL,
    });
    outPartial
}

pub fn boolRedeclare(inBoolRedeclare: bool) -> SCode::Redeclare {
    let mut outRedeclare: SCode::Redeclare;
    outRedeclare = (match inBoolRedeclare.clone() {
        true => crate::SCode::Redeclare::REDECLARE,
        false => crate::SCode::Redeclare::NOT_REDECLARE,
    });
    outRedeclare
}

pub fn boolReplaceable(inBoolReplaceable: bool, inOptConstrainClass: Option<Arc<SCode::ConstrainClass>>) -> Result<Arc<SCode::Replaceable>> {
    let mut outReplaceable: Arc<SCode::Replaceable>;
    outReplaceable = (match (inBoolReplaceable.clone(), inOptConstrainClass.clone()) {
        (true, _) => Arc::new(SCode::Replaceable::REPLACEABLE { cc: inOptConstrainClass.clone() }),
        (false, Some(_)) => {
            println!("{}", (literal!("Ignoring constraint class because replaceable prefix is not present!\\n")).clone());
            Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE)
        },
        (false, _) => Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE),
        _ => bail!("match: no arm matched"),
    });
    Ok(outReplaceable)
}

pub fn boolStream(inBoolStream: bool) -> SCode::ConnectorType {
    let mut outStream: SCode::ConnectorType;
    outStream = (match inBoolStream.clone() {
        true => crate::SCode::ConnectorType::STREAM,
        _ => crate::SCode::ConnectorType::POTENTIAL,
    });
    outStream
}

pub fn boolVisibility(inBoolVisibility: bool) -> SCode::Visibility {
    let mut outVisibility: SCode::Visibility;
    outVisibility = (match inBoolVisibility.clone() {
        true => crate::SCode::Visibility::PUBLIC,
        false => crate::SCode::Visibility::PROTECTED,
    });
    outVisibility
}

pub fn checkSameRestriction(inResNew: SCode::Restriction, inResOrig: SCode::Restriction, inInfoNew: SourceInfo, inInfoOrig: SourceInfo) -> (SCode::Restriction, SourceInfo) {
    let mut outRes: SCode::Restriction;
    let mut outInfo: SourceInfo;
    (outRes, outInfo) = (match (inResNew.clone(), inResOrig.clone(), inInfoNew.clone(), inInfoOrig.clone()) {
        (_, _, _, _) => (inResNew.clone(), inInfoNew.clone()),
    });
    (outRes, outInfo)
}

pub fn checkValidEnumLiteral(inLiteral: ArcStr, inInfo: SourceInfo) -> Result<()> {
    if listMember((inLiteral.clone()).clone(), list![(literal!("quantity")).clone(), (literal!("min")).clone(), (literal!("max")).clone(), (literal!("start")).clone(), (literal!("fixed")).clone()]) {
        Error::addSourceMessage(Error::INVALID_ENUM_LITERAL.clone(), list![(inLiteral.clone()).clone()], inInfo.clone())?;
        bail!("fail");
    }
    Ok(())
}

fn classDefEqual(cdef1: Arc<SCode::ClassDef>, cdef2: Arc<SCode::ClassDef>) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((cdef1.clone(), cdef2.clone())) {
        (Deref @ SCode::PARTS { .. }, Deref @ SCode::PARTS { .. }) => List::isEqualOnTrue(var_field!((*cdef1).elementLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).elementLst, SCode::ClassDef::PARTS).clone(), Arc::new(elementEqual)) && List::isEqualOnTrue(var_field!((*cdef1).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).normalEquationLst, SCode::ClassDef::PARTS).clone(), Arc::new(equationEqual)) && List::isEqualOnTrue(var_field!((*cdef1).initialEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).initialEquationLst, SCode::ClassDef::PARTS).clone(), Arc::new(equationEqual)) && List::isEqualOnTrue(var_field!((*cdef1).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), Arc::new(fnptr!(algorithmEqual, Arc<SCode::AlgorithmSection>, Arc<SCode::AlgorithmSection>))) && List::isEqualOnTrue(var_field!((*cdef1).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((*cdef2).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), Arc::new(fnptr!(algorithmEqual, Arc<SCode::AlgorithmSection>, Arc<SCode::AlgorithmSection>))),
        (Deref @ SCode::DERIVED { .. }, Deref @ SCode::DERIVED { .. }) => AbsynUtil::typeSpecEqual(var_field!((*cdef1).typeSpec, SCode::ClassDef::DERIVED).clone(), var_field!((*cdef2).typeSpec, SCode::ClassDef::DERIVED).clone()) && modEqual(var_field!((*cdef1).modifications, SCode::ClassDef::DERIVED).clone(), var_field!((*cdef2).modifications, SCode::ClassDef::DERIVED).clone())? && attributesEqual(var_field!((*cdef1).attributes, SCode::ClassDef::DERIVED).clone(), var_field!((*cdef2).attributes, SCode::ClassDef::DERIVED).clone())?,
        (Deref @ SCode::ENUMERATION { .. }, Deref @ SCode::ENUMERATION { .. }) => List::isEqualOnTrue(var_field!((*cdef1).enumLst, SCode::ClassDef::ENUMERATION).clone(), var_field!((*cdef2).enumLst, SCode::ClassDef::ENUMERATION).clone(), Arc::new(fnptr!(enumEqual, Arc<SCode::Enum>, Arc<SCode::Enum>))),
        (Deref @ SCode::CLASS_EXTENDS { .. }, Deref @ SCode::CLASS_EXTENDS { .. }) => modEqual(var_field!((*cdef1).modifications, SCode::ClassDef::CLASS_EXTENDS).clone(), var_field!((*cdef2).modifications, SCode::ClassDef::CLASS_EXTENDS).clone())? && classDefEqual(var_field!((*cdef1).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), var_field!((*cdef2).composition, SCode::ClassDef::CLASS_EXTENDS).clone())?,
        (Deref @ SCode::PDER { .. }, Deref @ SCode::PDER { .. }) => List::isEqualOnTrue(var_field!((*cdef1).derivedVariables, SCode::ClassDef::PDER).clone(), var_field!((*cdef2).derivedVariables, SCode::ClassDef::PDER).clone(), Arc::new(fnptr!(stringEq, ArcStr, ArcStr))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn classDefHasSections(cdef: Arc<SCode::ClassDef>, checkExternal: bool) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => !(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone().is_empty() && if (checkExternal.clone()) {isNone(var_field!((*cdef).externalDecl, SCode::ClassDef::PARTS).clone())} else {true}),
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => classDefHasSections(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), checkExternal.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn classIsExternalObject(cl: Arc<SCode::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { elementLst: els, .. }, .. } => isExternalObject(els.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn className(inClass: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut outName: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    Ok(outName)
}

pub fn classSetPartial(cls: Arc<SCode::Element>, inPartial: SCode::Partial) -> Result<Arc<SCode::Element>> {
    let mut cls: Arc<SCode::Element> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(cls => SCode::Element::CLASS; partialPrefix = inPartial.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub fn commentAnnotation(cmt: Arc<SCode::Comment>) -> Option<Arc<SCode::Annotation>> {
    let mut ann: Option<Arc<SCode::Annotation>> = cmt.annotation_.clone();
    ann
}

pub fn commentHasBooleanNamedAnnotation(comm: Arc<SCode::Comment>, annotationName: ArcStr) -> Result<bool> {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &((comm.clone(), annotationName.clone())) {
        (Deref @ SCode::COMMENT { annotation_: Some(ann), .. }, _) => hasBooleanNamedAnnotation(ann.clone(), (annotationName.clone()).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

pub fn componentMod(inElement: Arc<SCode::Element>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::COMPONENT { modifications: r#mod, .. } => r#mod.clone(),
        _ => Arc::new(crate::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn componentName(inComponent: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut outName: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(inComponent.clone()) {
        Deref @ SCode::COMPONENT { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    Ok(outName)
}

pub fn componentNames(inClass: Arc<SCode::Element>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>>;
    outStringLst = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { elementLst: elts, .. }, .. } => {
            let mut res: Arc<metamodelica::List<ArcStr>>;
            res = componentNamesFromElts(elts.clone());
            res.clone()
        },
        Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { composition: Deref @ SCode::PARTS { elementLst: elts, .. }, .. }, .. } => {
            let mut res: Arc<metamodelica::List<ArcStr>>;
            res = componentNamesFromElts(elts.clone());
            res.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStringLst
}

pub fn componentNamesFromElts(inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outComponentNames: Arc<metamodelica::List<ArcStr>>;
    outComponentNames = List::filterMap(inElements.clone(), Arc::new(componentName));
    outComponentNames
}

pub fn connectorTypeEqual(inConnectorType1: SCode::ConnectorType, inConnectorType2: SCode::ConnectorType) -> Result<bool> {
    let mut outEqual: bool;
    outEqual = (match (inConnectorType1.clone(), inConnectorType2.clone()) {
        (SCode::POTENTIAL, SCode::POTENTIAL) => true,
        (SCode::FLOW, SCode::FLOW) => true,
        (SCode::STREAM, SCode::STREAM) => true,
        _ => bail!("match: no arm matched"),
    });
    Ok(outEqual)
}

pub fn countParts(inClass: Arc<SCode::Element>) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { elementLst: elts, .. }, .. } => {
                    let mut res: i32;
                    res = (elts.clone().len() as i32);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { composition: Deref @ SCode::PARTS { elementLst: elts, .. }, .. }, .. } => {
                    let mut res: i32;
                    res = (elts.clone().len() as i32);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: i32;
                    let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

pub static dummyInfo: SourceInfo = SourceInfo { fileName: literal!(""), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: 0.0 };

pub fn eachBool(inEach: SCode::Each) -> Result<bool> {
    let mut bEach: bool;
    bEach = (match inEach.clone() {
        SCode::EACH => true,
        SCode::NOT_EACH => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bEach)
}

pub fn eachEqual(each1: SCode::Each, each2: SCode::Each) -> bool {
    let mut equal: bool;
    equal = (match (each1.clone(), each2.clone()) {
        (SCode::NOT_EACH, SCode::NOT_EACH) => true,
        (SCode::EACH, SCode::EACH) => true,
        _ => false,
    });
    equal
}

pub fn elementEqual(element1: Arc<SCode::Element>, element2: Arc<SCode::Element>) -> Result<bool> {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (element1.clone(), element2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::CLASS { .. }, Deref @ SCode::CLASS { .. }) => {
                    Ok(stringEq((var_field!((*element1).name, SCode::Element::CLASS).clone()).clone(), (var_field!((*element2).name, SCode::Element::CLASS).clone()).clone()) && prefixesEqual(var_field!((*element1).prefixes, SCode::Element::CLASS).clone(), var_field!((*element2).prefixes, SCode::Element::CLASS).clone())? && var_field!((*element1).encapsulatedPrefix, SCode::Element::CLASS).clone() == var_field!((*element2).encapsulatedPrefix, SCode::Element::CLASS).clone() && var_field!((*element1).partialPrefix, SCode::Element::CLASS).clone() == var_field!((*element2).partialPrefix, SCode::Element::CLASS).clone() && restrictionEqual(var_field!((*element1).restriction, SCode::Element::CLASS).clone(), var_field!((*element2).restriction, SCode::Element::CLASS).clone()) && classDefEqual(var_field!((*element1).classDef, SCode::Element::CLASS).clone(), var_field!((*element2).classDef, SCode::Element::CLASS).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::COMPONENT { .. }, Deref @ SCode::COMPONENT { .. }) => {
                    Ok(stringEq((var_field!((*element1).name, SCode::Element::COMPONENT).clone()).clone(), (var_field!((*element2).name, SCode::Element::COMPONENT).clone()).clone()) && prefixesEqual(var_field!((*element1).prefixes, SCode::Element::COMPONENT).clone(), var_field!((*element2).prefixes, SCode::Element::COMPONENT).clone())? && attributesEqual(var_field!((*element1).attributes, SCode::Element::COMPONENT).clone(), var_field!((*element2).attributes, SCode::Element::COMPONENT).clone())? && modEqual(var_field!((*element1).modifications, SCode::Element::COMPONENT).clone(), var_field!((*element2).modifications, SCode::Element::COMPONENT).clone())? && AbsynUtil::typeSpecEqual(var_field!((*element1).typeSpec, SCode::Element::COMPONENT).clone(), var_field!((*element2).typeSpec, SCode::Element::COMPONENT).clone()) && var_field!((*element1).condition, SCode::Element::COMPONENT).clone() == var_field!((*element2).condition, SCode::Element::COMPONENT).clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EXTENDS { .. }, Deref @ SCode::EXTENDS { .. }) => {
                    Ok(AbsynUtil::pathEqual(var_field!((*element1).baseClassPath, SCode::Element::EXTENDS).clone(), var_field!((*element2).baseClassPath, SCode::Element::EXTENDS).clone()) && modEqual(var_field!((*element1).modifications, SCode::Element::EXTENDS).clone(), var_field!((*element2).modifications, SCode::Element::EXTENDS).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::IMPORT { .. }, Deref @ SCode::IMPORT { .. }) => {
                    Ok(AbsynUtil::importEqual(var_field!((*element1).imp, SCode::Element::IMPORT).clone(), var_field!((*element2).imp, SCode::Element::IMPORT).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::DEFINEUNIT { .. }, Deref @ SCode::DEFINEUNIT { .. }) => {
                    Ok(stringEq((var_field!((*element1).name, SCode::Element::DEFINEUNIT).clone()).clone(), (var_field!((*element2).name, SCode::Element::DEFINEUNIT).clone()).clone()) && var_field!((*element1).exp, SCode::Element::DEFINEUNIT).clone() == var_field!((*element2).exp, SCode::Element::DEFINEUNIT).clone() && var_field!((*element1).weight, SCode::Element::DEFINEUNIT).clone() == var_field!((*element2).weight, SCode::Element::DEFINEUNIT).clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

pub fn elementInfo(e: Arc<SCode::Element>) -> SourceInfo {
    let mut info: SourceInfo;
    info = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::COMPONENT { info: i, .. } => i.clone(),
        Deref @ SCode::CLASS { info: i, .. } => i.clone(),
        Deref @ SCode::EXTENDS { info: i, .. } => i.clone(),
        Deref @ SCode::IMPORT { info: i, .. } => i.clone(),
        _ => AbsynUtil::dummyInfo.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    info
}

pub fn elementInnerOuter(element: Arc<SCode::Element>) -> Result<Absyn::InnerOuter> {
    let mut io: Absyn::InnerOuter;
    io = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => prefixesInnerOuter(var_field!((*element).prefixes, SCode::Element::CLASS).clone())?,
        Deref @ SCode::Element::COMPONENT { .. } => prefixesInnerOuter(var_field!((*element).prefixes, SCode::Element::COMPONENT).clone())?,
        _ => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(io)
}

pub fn elementIsClass(el: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::CLASS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn elementIsImport(inElement: Arc<SCode::Element>) -> bool {
    let mut outIsImport: bool;
    outIsImport = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::IMPORT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsImport
}

pub fn elementIsProtectedImport(el: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::IMPORT { visibility: SCode::PROTECTED, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn elementIsPublicImport(el: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::IMPORT { visibility: SCode::PUBLIC, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn elementMod(inElement: Arc<SCode::Element>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::COMPONENT { modifications: r#mod, .. } => r#mod.clone(),
        Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { modifications: r#mod, .. }, .. } => r#mod.clone(),
        Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { modifications: r#mod, .. }, .. } => r#mod.clone(),
        Deref @ SCode::EXTENDS { modifications: r#mod, .. } => r#mod.clone(),
        _ => Arc::new(crate::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn elementName(e: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::COMPONENT { name: s, .. } => s.clone(),
        Deref @ SCode::CLASS { name: s, .. } => s.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(s)
}

pub fn elementNameEqual(inElement1: Arc<SCode::Element>, inElement2: Arc<SCode::Element>) -> bool {
    let mut outEqual: bool;
    outEqual = (::match_deref::match_deref! { match &((inElement1.clone(), inElement2.clone())) {
        (Deref @ SCode::CLASS { .. }, Deref @ SCode::CLASS { .. }) => var_field!((*inElement1).name, SCode::Element::CLASS).clone() == var_field!((*inElement2).name, SCode::Element::CLASS).clone(),
        (Deref @ SCode::COMPONENT { .. }, Deref @ SCode::COMPONENT { .. }) => var_field!((*inElement1).name, SCode::Element::COMPONENT).clone() == var_field!((*inElement2).name, SCode::Element::COMPONENT).clone(),
        (Deref @ SCode::DEFINEUNIT { .. }, Deref @ SCode::DEFINEUNIT { .. }) => var_field!((*inElement1).name, SCode::Element::DEFINEUNIT).clone() == var_field!((*inElement2).name, SCode::Element::DEFINEUNIT).clone(),
        (Deref @ SCode::EXTENDS { .. }, Deref @ SCode::EXTENDS { .. }) => AbsynUtil::pathEqual(var_field!((*inElement1).baseClassPath, SCode::Element::EXTENDS).clone(), var_field!((*inElement2).baseClassPath, SCode::Element::EXTENDS).clone()),
        (Deref @ SCode::IMPORT { .. }, Deref @ SCode::IMPORT { .. }) => AbsynUtil::importEqual(var_field!((*inElement1).imp, SCode::Element::IMPORT).clone(), var_field!((*inElement2).imp, SCode::Element::IMPORT).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEqual
}

pub fn elementNameInfo(element: Arc<SCode::Element>) -> Result<(ArcStr, SourceInfo)> {
    let mut name: ArcStr;
    let mut info: SourceInfo;
    (name, info) = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { info, name, .. } => (name.clone(), info.clone()),
        Deref @ SCode::CLASS { info, name, .. } => (name.clone(), info.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok((name, info))
}

pub fn elementNames(elts: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut names: Arc<metamodelica::List<ArcStr>>;
    names = List::fold(elts.clone(), Arc::new(fnptr!(elementNamesWork, Arc<SCode::Element>, Arc<metamodelica::List<ArcStr>>)), metamodelica::nil());
    names
}

fn elementNamesWork(e: Arc<SCode::Element>, acc: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut out: Arc<metamodelica::List<ArcStr>>;
    out = (::match_deref::match_deref! { match &((e.clone(), acc.clone())) {
        (Deref @ SCode::COMPONENT { name: s, .. }, _) => cons(s.clone(), acc.clone()),
        (Deref @ SCode::CLASS { name: s, .. }, _) => cons(s.clone(), acc.clone()),
        _ => acc.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub fn elementPrefixes(inElement: Arc<SCode::Element>) -> Result<Arc<SCode::Prefixes>> {
    let mut outPrefixes: Arc<SCode::Prefixes>;
    outPrefixes = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::CLASS { .. } => var_field!((*inElement).prefixes, SCode::Element::CLASS).clone(),
        Deref @ SCode::COMPONENT { .. } => var_field!((*inElement).prefixes, SCode::Element::COMPONENT).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPrefixes)
}

pub fn elementVisibility(element: Arc<SCode::Element>) -> Result<SCode::Visibility> {
    let mut visibility: SCode::Visibility;
    visibility = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::IMPORT { .. } => var_field!((*element).visibility, SCode::Element::IMPORT).clone(),
        Deref @ SCode::Element::EXTENDS { .. } => var_field!((*element).visibility, SCode::Element::EXTENDS).clone(),
        Deref @ SCode::Element::CLASS { .. } => prefixesVisibility(var_field!((*element).prefixes, SCode::Element::CLASS).clone())?,
        Deref @ SCode::Element::COMPONENT { .. } => prefixesVisibility(var_field!((*element).prefixes, SCode::Element::COMPONENT).clone())?,
        Deref @ SCode::Element::DEFINEUNIT { .. } => var_field!((*element).visibility, SCode::Element::DEFINEUNIT).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(visibility)
}

pub fn emptyModOrEquality(r#mod: Arc<SCode::Mod>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::NOMOD => true,
        Deref @ SCode::MOD { subModLst: Deref @ metamodelica::List::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn encapsulatedBool(inEncapsulated: SCode::Encapsulated) -> Result<bool> {
    let mut bEncapsulated: bool;
    bEncapsulated = (match inEncapsulated.clone() {
        SCode::ENCAPSULATED => true,
        SCode::NOT_ENCAPSULATED => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bEncapsulated)
}

pub fn enumEqual(e1: Arc<SCode::Enum>, e2: Arc<SCode::Enum>) -> bool {
    let mut isEqual: bool = e1.literal.clone() == e2.literal.clone();
    isEqual
}

pub fn enumName(e: Arc<SCode::Enum>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::ENUM { literal: s, .. } => s.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(s)
}

pub fn equationContainReinit(inEq: Arc<SCode::Equation>) -> bool {
    let mut hasReinit: bool;
    hasReinit = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ SCode::EQ_REINIT { .. } => true,
        Deref @ SCode::EQ_WHEN { elseBranches: tpl_el, eEquationLst: eqs, .. } => {
            let mut b: bool;
            let mut eqs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            b = equationsContainReinit(eqs.clone());
            eqs_lst = List::map(tpl_el.clone(), Arc::new(fnptr!(Util::tuple22, _)));
            b = List::applyAndFold(eqs_lst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), Arc::new(fnptr!(equationsContainReinit, Arc<metamodelica::List<Arc<SCode::Equation>>>)), b.clone());
            b.clone()
        },
        Deref @ SCode::EQ_IF { elseBranch: eqs, thenBranch: eqs_lst, .. } => {
            let mut b: bool;
            let mut tpl_el: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            b = equationsContainReinit(eqs.clone());
            b = List::applyAndFold(eqs_lst.clone(), Arc::new(fnptr!(boolOr, bool, bool)), Arc::new(fnptr!(equationsContainReinit, Arc<metamodelica::List<Arc<SCode::Equation>>>)), b.clone());
            b.clone()
        },
        Deref @ SCode::EQ_FOR { eEquationLst: eqs, .. } => {
            let mut b: bool;
            let mut eqs_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut tpl_el: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            b = equationsContainReinit(eqs.clone());
            b.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

fn equationEqual(eq1: Arc<SCode::Equation>, eq2: Arc<SCode::Equation>) -> Result<bool> {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (eq1.clone(), eq2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_IF { elseBranch: fb1, thenBranch: tb1, condition: ifcond1, .. }, Deref @ SCode::EQ_IF { elseBranch: fb2, thenBranch: tb2, condition: ifcond2, .. }) => {
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (equationEqual2(tb1.clone(), tb2.clone())?) else { bail!("pattern mismatch") };
                    let true = (List::isEqualOnTrue(fb1.clone(), fb2.clone(), Arc::new(equationEqual))) else { bail!("pattern mismatch") };
                    let true = (List::isEqualOnTrue(ifcond1.clone(), ifcond2.clone(), Arc::new(AbsynUtil::expEqual))) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_EQUALS { expRight: e12, expLeft: e11, .. }, Deref @ SCode::EQ_EQUALS { expRight: e22, expLeft: e21, .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (AbsynUtil::expEqual(e11.clone(), e21.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e12.clone(), e22.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_PDE { domain: cr1, expRight: e12, expLeft: e11, .. }, Deref @ SCode::EQ_PDE { domain: cr2, expRight: e22, expLeft: e21, .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (AbsynUtil::expEqual(e11.clone(), e21.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e12.clone(), e22.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::crefEqual(cr1.clone(), cr2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_CONNECT { crefRight: cr12, crefLeft: cr11, .. }, Deref @ SCode::EQ_CONNECT { crefRight: cr22, crefLeft: cr21, .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (AbsynUtil::crefEqual(cr11.clone(), cr21.clone())) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::crefEqual(cr12.clone(), cr22.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_FOR { eEquationLst: eql1, range: Some(exp1), index: id1, .. }, Deref @ SCode::EQ_FOR { eEquationLst: eql2, range: Some(exp2), index: id2, .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (List::isEqualOnTrue(eql1.clone(), eql2.clone(), Arc::new(equationEqual))) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(exp1.clone(), exp2.clone())?) else { bail!("pattern mismatch") };
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_FOR { eEquationLst: eql1, range: None, index: id1, .. }, Deref @ SCode::EQ_FOR { eEquationLst: eql2, range: None, index: id2, .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (List::isEqualOnTrue(eql1.clone(), eql2.clone(), Arc::new(equationEqual))) else { bail!("pattern mismatch") };
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_WHEN { eEquationLst: elst1, condition: cond1, .. }, Deref @ SCode::EQ_WHEN { eEquationLst: elst2, condition: cond2, .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (List::isEqualOnTrue(elst1.clone(), elst2.clone(), Arc::new(equationEqual))) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(cond1.clone(), cond2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_ASSERT { message: m1, condition: c1, .. }, Deref @ SCode::EQ_ASSERT { message: m2, condition: c2, .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (AbsynUtil::expEqual(c1.clone(), c2.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(m1.clone(), m2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_REINIT { .. }, Deref @ SCode::EQ_REINIT { .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (AbsynUtil::expEqual(var_field!((*eq1).cref, SCode::Equation::EQ_REINIT).clone(), var_field!((*eq2).cref, SCode::Equation::EQ_REINIT).clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(var_field!((*eq1).expReinit, SCode::Equation::EQ_REINIT).clone(), var_field!((*eq2).expReinit, SCode::Equation::EQ_REINIT).clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::EQ_NORETCALL { exp: e1, .. }, Deref @ SCode::EQ_NORETCALL { exp: e2, .. }) => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut cond1: Arc<Absyn::Exp>;
                    let mut cond2: Arc<Absyn::Exp>;
                    let mut ifcond1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ifcond2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut exp1: Arc<Absyn::Exp>;
                    let mut exp2: Arc<Absyn::Exp>;
                    let mut c1: Arc<Absyn::Exp>;
                    let mut c2: Arc<Absyn::Exp>;
                    let mut m1: Arc<Absyn::Exp>;
                    let mut m2: Arc<Absyn::Exp>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut cr11: Arc<Absyn::ComponentRef>;
                    let mut cr12: Arc<Absyn::ComponentRef>;
                    let mut cr21: Arc<Absyn::ComponentRef>;
                    let mut cr22: Arc<Absyn::ComponentRef>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut fb1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut fb2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut eql2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut elst2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

fn equationEqual2(inTb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>, inTb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>) -> Result<bool> {
    let mut bOut: bool;
    bOut = 'mc: {
        let __mc_input = (inTb1.clone(), inTb2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut tb_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut tb_2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    let mut tb_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut tb_2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    let mut tb_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut tb_2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: tb_1, tail: tb1 }, Deref @ metamodelica::List::Cons { head: tb_2, tail: tb2 }) => {
                    let true = (List::isEqualOnTrue(tb_1.clone(), tb_2.clone(), Arc::new(equationEqual))) else { bail!("pattern mismatch") };
                    let true = (equationEqual2(tb1.clone(), tb2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
                    let mut tb_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut tb_2: Arc<metamodelica::List<Arc<SCode::Equation>>>;
                    let mut tb1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    let mut tb2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(bOut)
}

pub fn equationsContainReinit(inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>) -> bool {
    let mut hasReinit: bool;
    hasReinit = (::match_deref::match_deref! { match &(inEqs.clone()) {
        _ => {
            let mut b: bool;
            b = List::applyAndFold(inEqs.clone(), Arc::new(fnptr!(boolOr, bool, bool)), Arc::new(fnptr!(equationContainReinit, Arc<SCode::Equation>)), false);
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

fn filterComponents(inElements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> (Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<ArcStr>>) {
    let mut outComponents: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut outComponentNames: Arc<metamodelica::List<ArcStr>>;
    (outComponents, outComponentNames) = List::map_2(inElements.clone(), Arc::new(filterComponents2));
    (outComponents, outComponentNames)
}

fn filterComponents2(inElement: Arc<SCode::Element>) -> Result<(Arc<SCode::Element>, ArcStr)> {
    let mut outComponent: Arc<SCode::Element>;
    let mut outName: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::COMPONENT { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    outComponent = inElement.clone();
    Ok((outComponent, outName))
}

pub fn filterGivenSubModNames(submod: Arc<SCode::SubMod>, namesToKeep: Arc<metamodelica::List<ArcStr>>) -> bool {
    let mut keep: bool;
    keep = listMember((submod.ident.clone()).clone(), namesToKeep.clone());
    keep
}

pub fn filterSubMods(r#mod: Arc<SCode::Mod>, filter: Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>) -> Result<bool> + 'static>) -> Arc<SCode::Mod> {
    pub type FilterFunc = fn(Arc<SCode::SubMod>) -> Result<bool>;

    let mut r#mod: Arc<SCode::Mod> = r#mod;
    r#mod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for m in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            if !(filter(m.clone()).unwrap()) { continue; }
            let __x = m.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (::match_deref::match_deref! { match &(r#mod.clone()) {
        SCode::MOD { binding: None, subModLst: Deref @ metamodelica::List::Nil, .. } => Arc::new(crate::SCode::Mod::NOMOD),
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => r#mod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

pub fn finalBool(inFinal: SCode::Final) -> Result<bool> {
    let mut bFinal: bool;
    bFinal = (match inFinal.clone() {
        SCode::FINAL => true,
        SCode::NOT_FINAL => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bFinal)
}

pub fn finalEqual(inFinal1: SCode::Final, inFinal2: SCode::Final) -> bool {
    let mut bFinal: bool;
    bFinal = (match (inFinal1.clone(), inFinal2.clone()) {
        (SCode::FINAL, SCode::FINAL) => true,
        (SCode::NOT_FINAL, SCode::NOT_FINAL) => true,
        _ => false,
    });
    bFinal
}

pub fn findIteratorIndexedCrefsInEquation(inEq: Arc<SCode::Equation>, inIterator: ArcStr, inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>;
    outCrefs = foldEquationsExps(inEq.clone(), Arc::new({ let __pe_b1 = inIterator.clone(); move |__pe_a0, __pe_a2| AbsynUtil::findIteratorIndexedCrefs(__pe_a0, __pe_b1.clone(), __pe_a2) }), inCrefs.clone())?;
    Ok(outCrefs)
}

pub fn findIteratorIndexedCrefsInEquations(inEqs: Arc<metamodelica::List<Arc<SCode::Equation>>>, inIterator: ArcStr, inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>;
    outCrefs = List::fold1(inEqs.clone(), Arc::new(findIteratorIndexedCrefsInEquation), (inIterator.clone()).clone(), inCrefs.clone());
    outCrefs
}

pub fn findIteratorIndexedCrefsInStatement(inStatement: Arc<SCode::Statement>, inIterator: ArcStr, inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>;
    outCrefs = foldStatementsExps(inStatement.clone(), Arc::new({ let __pe_b1 = inIterator.clone(); move |__pe_a0, __pe_a2| AbsynUtil::findIteratorIndexedCrefs(__pe_a0, __pe_b1.clone(), __pe_a2) }), inCrefs.clone())?;
    Ok(outCrefs)
}

pub fn findIteratorIndexedCrefsInStatements(inStatements: Arc<metamodelica::List<Arc<SCode::Statement>>>, inIterator: ArcStr, inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>;
    outCrefs = List::fold1(inStatements.clone(), Arc::new(findIteratorIndexedCrefsInStatement), (inIterator.clone()).clone(), inCrefs.clone());
    outCrefs
}

pub fn flowBool(inConnectorType: SCode::ConnectorType) -> bool {
    let mut outFlow: bool;
    outFlow = (match inConnectorType.clone() {
        SCode::FLOW => true,
        _ => false,
    });
    outFlow
}

pub fn foldEquations<ArgT: Clone + 'static>(inEquation: Arc<SCode::Equation>, inFunc: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<ArgT> + 'static>, inArg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<SCode::Equation>, ArgT) -> Result<ArgT>;

    let mut outArg: ArgT;
    outArg = inFunc(inEquation.clone(), inArg.clone())?;
    outArg = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ SCode::EQ_IF { .. } => {
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = List::foldList(var_field!((*inEquation).thenBranch, SCode::Equation::EQ_IF).clone(), Arc::new({ let __pe_b1 = inFunc.clone(); move |__pe_a0, __pe_a2| foldEquations(__pe_a0, __pe_b1.clone(), __pe_a2) }), outArg.clone());
            List::fold1(var_field!((*inEquation).elseBranch, SCode::Equation::EQ_IF).clone(), Arc::new(foldEquations), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::EQ_FOR { .. } => List::fold1(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_FOR).clone(), Arc::new(foldEquations), inFunc.clone(), outArg.clone()),
        Deref @ SCode::EQ_WHEN { .. } => {
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = List::fold1(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_WHEN).clone(), Arc::new(foldEquations), inFunc.clone(), outArg.clone());
            for branch in &*var_field!((*inEquation).elseBranches, SCode::Equation::EQ_WHEN).clone() {
                (_, eql) = branch.clone();
                outArg = List::fold1(eql.clone(), Arc::new(foldEquations), inFunc.clone(), outArg.clone());
            }
            outArg.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArg)
}

pub fn foldEquationsExps<ArgT: Clone + 'static>(inEquation: Arc<SCode::Equation>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>, inArg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<ArgT>;

    let mut outArg: ArgT = inArg.clone();
    outArg = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ SCode::EQ_IF { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = List::fold(var_field!((*inEquation).condition, SCode::Equation::EQ_IF).clone(), inFunc.clone(), outArg.clone());
            outArg = List::foldList(var_field!((*inEquation).thenBranch, SCode::Equation::EQ_IF).clone(), Arc::new({ let __pe_b1 = inFunc.clone(); move |__pe_a0, __pe_a2| foldEquationsExps(__pe_a0, __pe_b1.clone(), __pe_a2) }), outArg.clone());
            List::fold1(var_field!((*inEquation).elseBranch, SCode::Equation::EQ_IF).clone(), Arc::new(foldEquationsExps), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::EQ_EQUALS { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = inFunc(var_field!((*inEquation).expLeft, SCode::Equation::EQ_EQUALS).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).expRight, SCode::Equation::EQ_EQUALS).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::EQ_PDE { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = inFunc(var_field!((*inEquation).expLeft, SCode::Equation::EQ_PDE).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).expRight, SCode::Equation::EQ_PDE).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::EQ_CONNECT { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = inFunc(Arc::new(Absyn::Exp::CREF { componentRef: var_field!((*inEquation).crefLeft, SCode::Equation::EQ_CONNECT).clone() }), outArg.clone())?;
            outArg = inFunc(Arc::new(Absyn::Exp::CREF { componentRef: var_field!((*inEquation).crefRight, SCode::Equation::EQ_CONNECT).clone() }), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::EQ_FOR { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            if isSome(var_field!((*inEquation).range, SCode::Equation::EQ_FOR).clone()) {
                let Some(__pa0) = (var_field!((*inEquation).range, SCode::Equation::EQ_FOR).clone()) else { bail!("pattern mismatch") };
                exp = __pa0.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
            }
            List::fold1(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_FOR).clone(), Arc::new(foldEquationsExps), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::EQ_WHEN { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = List::fold1(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_WHEN).clone(), Arc::new(foldEquationsExps), inFunc.clone(), outArg.clone());
            for branch in &*var_field!((*inEquation).elseBranches, SCode::Equation::EQ_WHEN).clone() {
                (exp, eql) = branch.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
                outArg = List::fold1(eql.clone(), Arc::new(foldEquationsExps), inFunc.clone(), outArg.clone());
            }
            outArg.clone()
        },
        Deref @ SCode::EQ_ASSERT { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = inFunc(var_field!((*inEquation).condition, SCode::Equation::EQ_ASSERT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).message, SCode::Equation::EQ_ASSERT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).level, SCode::Equation::EQ_ASSERT).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::EQ_TERMINATE { .. } => inFunc(var_field!((*inEquation).message, SCode::Equation::EQ_TERMINATE).clone(), outArg.clone())?,
        Deref @ SCode::EQ_REINIT { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            outArg = inFunc(var_field!((*inEquation).cref, SCode::Equation::EQ_REINIT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inEquation).expReinit, SCode::Equation::EQ_REINIT).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::EQ_NORETCALL { .. } => inFunc(var_field!((*inEquation).exp, SCode::Equation::EQ_NORETCALL).clone(), outArg.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArg)
}

pub fn foldStatementsExps<ArgT: Clone + 'static>(inStatement: Arc<SCode::Statement>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>, inArg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<ArgT>;

    let mut outArg: ArgT = inArg.clone();
    outArg = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ SCode::ALG_ASSIGN { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            outArg = inFunc(var_field!((*inStatement).assignComponent, SCode::Statement::ALG_ASSIGN).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inStatement).value, SCode::Statement::ALG_ASSIGN).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::ALG_IF { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            outArg = inFunc(var_field!((*inStatement).boolExpr, SCode::Statement::ALG_IF).clone(), outArg.clone())?;
            outArg = List::fold1(var_field!((*inStatement).trueBranch, SCode::Statement::ALG_IF).clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone());
            for branch in &*var_field!((*inStatement).elseIfBranch, SCode::Statement::ALG_IF).clone() {
                (exp, stmts) = branch.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
                outArg = List::fold1(stmts.clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone());
            }
            outArg.clone()
        },
        Deref @ SCode::ALG_FOR { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            if isSome(var_field!((*inStatement).range, SCode::Statement::ALG_FOR).clone()) {
                let Some(__pa0) = (var_field!((*inStatement).range, SCode::Statement::ALG_FOR).clone()) else { bail!("pattern mismatch") };
                exp = __pa0.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
            }
            List::fold1(var_field!((*inStatement).forBody, SCode::Statement::ALG_FOR).clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::ALG_PARFOR { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            if isSome(var_field!((*inStatement).range, SCode::Statement::ALG_PARFOR).clone()) {
                let Some(__pa0) = (var_field!((*inStatement).range, SCode::Statement::ALG_PARFOR).clone()) else { bail!("pattern mismatch") };
                exp = __pa0.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
            }
            List::fold1(var_field!((*inStatement).parforBody, SCode::Statement::ALG_PARFOR).clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::ALG_WHILE { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            outArg = inFunc(var_field!((*inStatement).boolExpr, SCode::Statement::ALG_WHILE).clone(), outArg.clone())?;
            List::fold1(var_field!((*inStatement).whileBody, SCode::Statement::ALG_WHILE).clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::ALG_WHEN_A { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            for branch in &*var_field!((*inStatement).branches, SCode::Statement::ALG_WHEN_A).clone() {
                (exp, stmts) = branch.clone();
                outArg = inFunc(exp.clone(), outArg.clone())?;
                outArg = List::fold1(stmts.clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone());
            }
            outArg.clone()
        },
        Deref @ SCode::ALG_ASSERT { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            outArg = inFunc(var_field!((*inStatement).condition, SCode::Statement::ALG_ASSERT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inStatement).message, SCode::Statement::ALG_ASSERT).clone(), outArg.clone())?;
            outArg = inFunc(var_field!((*inStatement).level, SCode::Statement::ALG_ASSERT).clone(), outArg.clone())?;
            outArg.clone()
        },
        Deref @ SCode::ALG_TERMINATE { .. } => inFunc(var_field!((*inStatement).message, SCode::Statement::ALG_TERMINATE).clone(), outArg.clone())?,
        Deref @ SCode::ALG_REINIT { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            outArg = inFunc(var_field!((*inStatement).cref, SCode::Statement::ALG_REINIT).clone(), outArg.clone())?;
            inFunc(var_field!((*inStatement).newValue, SCode::Statement::ALG_REINIT).clone(), outArg.clone())?
        },
        Deref @ SCode::ALG_NORETCALL { .. } => inFunc(var_field!((*inStatement).exp, SCode::Statement::ALG_NORETCALL).clone(), outArg.clone())?,
        Deref @ SCode::ALG_FAILURE { .. } => List::fold1(var_field!((*inStatement).stmts, SCode::Statement::ALG_FAILURE).clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone()),
        Deref @ SCode::ALG_TRY { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            outArg = List::fold1(var_field!((*inStatement).body, SCode::Statement::ALG_TRY).clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone());
            List::fold1(var_field!((*inStatement).elseBody, SCode::Statement::ALG_TRY).clone(), Arc::new(foldStatementsExps), inFunc.clone(), outArg.clone())
        },
        Deref @ SCode::ALG_RETURN { .. } => outArg.clone(),
        Deref @ SCode::ALG_BREAK { .. } => outArg.clone(),
        Deref @ SCode::ALG_CONTINUE { .. } => outArg.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArg)
}

pub fn funcRestrictionEqual(funcRestr1: SCode::FunctionRestriction, funcRestr2: SCode::FunctionRestriction) -> bool {
    let mut equal: bool;
    equal = (match (funcRestr1.clone(), funcRestr2.clone()) {
        (SCode::FR_NORMAL_FUNCTION { .. }, SCode::FR_NORMAL_FUNCTION { .. }) => AbsynUtil::purityEqual(var_field!(funcRestr1.purity, SCode::FunctionRestriction::FR_NORMAL_FUNCTION).clone(), var_field!(funcRestr2.purity, SCode::FunctionRestriction::FR_NORMAL_FUNCTION).clone(), false),
        (SCode::FR_EXTERNAL_FUNCTION { .. }, SCode::FR_EXTERNAL_FUNCTION { .. }) => AbsynUtil::purityEqual(var_field!(funcRestr1.purity, SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION).clone(), var_field!(funcRestr2.purity, SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION).clone(), false),
        (SCode::FR_OPERATOR_FUNCTION, SCode::FR_OPERATOR_FUNCTION) => true,
        (SCode::FR_RECORD_CONSTRUCTOR, SCode::FR_RECORD_CONSTRUCTOR) => true,
        (SCode::FR_PARALLEL_FUNCTION, SCode::FR_PARALLEL_FUNCTION) => true,
        (SCode::FR_KERNEL_FUNCTION, SCode::FR_KERNEL_FUNCTION) => true,
        _ => false,
    });
    equal
}

pub fn getBaseClassPath(inE: Arc<SCode::Element>) -> Result<Arc<Absyn::Path>> {
    let mut outBcPath: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::EXTENDS { baseClassPath: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outBcPath = __pa0.clone();
    Ok(outBcPath)
}

pub fn getClassBody(inClass: Arc<SCode::Element>) -> Result<Arc<SCode::ClassDef>> {
    let mut outCdef: Arc<SCode::ClassDef>;
    outCdef = getClassDef(inClass.clone())?;
    outCdef = (::match_deref::match_deref! { match &(outCdef.clone()) {
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => var_field!((*outCdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(),
        _ => outCdef.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCdef)
}

pub fn getClassComponents(cl: Arc<SCode::Element>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut compElts: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut compNames: Arc<metamodelica::List<ArcStr>>;
    (compElts, compNames) = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { elementLst: elts, .. }, .. } => {
            let mut comps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut names: Arc<metamodelica::List<ArcStr>>;
            (comps, names) = filterComponents(elts.clone());
            (comps.clone(), names.clone())
        },
        Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { composition: Deref @ SCode::PARTS { elementLst: elts, .. }, .. }, .. } => {
            let mut comps: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut names: Arc<metamodelica::List<ArcStr>>;
            (comps, names) = filterComponents(elts.clone());
            (comps.clone(), names.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((compElts, compNames))
}

pub fn getClassDef(inClass: Arc<SCode::Element>) -> Result<Arc<SCode::ClassDef>> {
    let mut outCdef: Arc<SCode::ClassDef>;
    outCdef = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { classDef: outCdef, .. } => outCdef.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCdef)
}

pub fn getClassElements(cl: Arc<SCode::Element>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>;
    elts = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { elementLst: elts, .. }, .. } => elts.clone(),
        Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { composition: Deref @ SCode::PARTS { elementLst: elts, .. }, .. }, .. } => elts.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elts
}

pub fn getClassPartialPrefix(inElement: Arc<SCode::Element>) -> Result<SCode::Partial> {
    let mut outPartial: SCode::Partial;
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::CLASS { partialPrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outPartial = __pa0.clone();
    Ok(outPartial)
}

pub fn getClassRestriction(inElement: Arc<SCode::Element>) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::CLASS { restriction: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outRestriction = __pa0.clone();
    Ok(outRestriction)
}

pub fn getComponentCondition(element: Arc<SCode::Element>) -> Option<Arc<Absyn::Exp>> {
    let mut condition: Option<Arc<Absyn::Exp>>;
    condition = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { .. } => var_field!((*element).condition, SCode::Element::COMPONENT).clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    condition
}

pub fn getComponentMod(inE: Arc<SCode::Element>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::COMPONENT { modifications: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outMod = __pa0.clone();
    Ok(outMod)
}

pub fn getComponentTypeSpec(inE: Arc<SCode::Element>) -> Result<Arc<Absyn::TypeSpec>> {
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::COMPONENT { typeSpec: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outTypeSpec = __pa0.clone();
    Ok(outTypeSpec)
}

pub fn getConstrainedByModifiers(inPrefixes: Arc<SCode::Prefixes>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::PREFIXES { replaceablePrefix: Deref @ SCode::REPLACEABLE { cc: Some(SCode::CONSTRAINCLASS { modifier: m, .. }) }, .. } => m.clone(),
        _ => Arc::new(crate::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn getConstrainingMod(element: Arc<SCode::Element>) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod>;
    r#mod = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::CLASS { prefixes: Deref @ SCode::Prefixes::PREFIXES { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(SCode::CONSTRAINCLASS { modifier: r#mod, .. }) }, .. }, .. } => r#mod.clone(),
        Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { modifications: r#mod, .. }, .. } => r#mod.clone(),
        Deref @ SCode::COMPONENT { prefixes: Deref @ SCode::Prefixes::PREFIXES { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(SCode::CONSTRAINCLASS { modifier: r#mod, .. }) }, .. }, .. } => r#mod.clone(),
        Deref @ SCode::COMPONENT { modifications: r#mod, .. } => r#mod.clone(),
        _ => Arc::new(crate::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

pub fn getDerivedMod(inE: Arc<SCode::Element>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { modifications: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outMod = __pa0.clone();
    Ok(outMod)
}

pub fn getDerivedTypeSpec(inE: Arc<SCode::Element>) -> Result<Arc<Absyn::TypeSpec>> {
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    let __pa0 = ::match_deref::match_deref! { match &(inE.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { typeSpec: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outTypeSpec = __pa0.clone();
    Ok(outTypeSpec)
}

pub fn getElementAnnotation(element: Arc<SCode::Element>, name: ArcStr) -> Option<Arc<SCode::Annotation>> {
    let mut outAnnotation: Option<Arc<SCode::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::EXTENDS { .. } => var_field!((*element).ann, SCode::Element::EXTENDS).clone(),
        Deref @ SCode::CLASS { .. } => var_field!((*element).cmt, SCode::Element::CLASS).annotation_.clone(),
        Deref @ SCode::COMPONENT { .. } => var_field!((*element).comment, SCode::Element::COMPONENT).annotation_.clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAnnotation
}

pub fn getElementClass(el: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element>;
    cl = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::CLASS { .. } => el.clone(),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cl)
}

pub fn getElementComment(inElement: Arc<SCode::Element>) -> Option<Arc<SCode::Comment>> {
    let mut outComment: Option<Arc<SCode::Comment>>;
    outComment = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::COMPONENT { comment: cmt, .. } => Some(cmt.clone()),
        Deref @ SCode::CLASS { cmt, .. } => Some(cmt.clone()),
        Deref @ SCode::EXTENDS { .. } => Some(Arc::new(SCode::Comment { annotation_: var_field!((*inElement).ann, SCode::Element::EXTENDS).clone(), comment: None })),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComment
}

pub fn getElementName(e: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = ((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::COMPONENT { name: s, .. } => s.clone(),
        Deref @ SCode::CLASS { name: s, .. } => s.clone(),
        Deref @ SCode::EXTENDS { baseClassPath: p, .. } => AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(s)
}

pub fn getElementNamed(inIdent: ArcStr, inClass: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &((inIdent.clone(), inClass.clone())) {
        (id, Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { elementLst: elts, .. }, .. }) => {
            let mut elt: Arc<SCode::Element>;
            elt = getElementNamedFromElts((id.clone()).clone(), elts.clone())?;
            elt.clone()
        },
        (id, Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { composition: Deref @ SCode::PARTS { elementLst: elts, .. }, .. }, .. }) => {
            let mut elt: Arc<SCode::Element>;
            elt = getElementNamedFromElts((id.clone()).clone(), elts.clone())?;
            elt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getElementNamedFromElts(inIdent: ArcStr, inElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = 'mc: {
        let __mc_input = (inIdent.clone(), inElementLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: comp @ Deref @ SCode::COMPONENT { name: id1, .. }, tail: _ }) => {
                    let mut elt: Arc<SCode::Element>;
                    let mut cdef: Arc<SCode::Element>;
                    let mut xs: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(comp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: Deref @ SCode::COMPONENT { name: id1, .. }, tail: xs }) => {
                    let mut elt: Arc<SCode::Element>;
                    let mut comp: Arc<SCode::Element>;
                    let mut cdef: Arc<SCode::Element>;
                    let false = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    elt = getElementNamedFromElts((id2.clone()).clone(), xs.clone())?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: Deref @ SCode::CLASS { name: id1, .. }, tail: xs }) => {
                    let mut elt: Arc<SCode::Element>;
                    let mut comp: Arc<SCode::Element>;
                    let mut cdef: Arc<SCode::Element>;
                    let false = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    elt = getElementNamedFromElts((id2.clone()).clone(), xs.clone())?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: Deref @ SCode::EXTENDS { .. }, tail: xs }) => {
                    let mut elt: Arc<SCode::Element>;
                    let mut comp: Arc<SCode::Element>;
                    let mut cdef: Arc<SCode::Element>;
                    let mut id1: ArcStr;
                    elt = getElementNamedFromElts((id2.clone()).clone(), xs.clone())?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: cdef @ Deref @ SCode::CLASS { name: id1, .. }, tail: _ }) => {
                    let mut elt: Arc<SCode::Element>;
                    let mut comp: Arc<SCode::Element>;
                    let mut xs: Arc<metamodelica::List<Arc<SCode::Element>>>;
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(cdef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::List::Cons { head: _, tail: xs }) => {
                    let mut elt: Arc<SCode::Element>;
                    let mut comp: Arc<SCode::Element>;
                    let mut cdef: Arc<SCode::Element>;
                    let mut id1: ArcStr;
                    elt = getElementNamedFromElts((id2.clone()).clone(), xs.clone())?;
                    Ok(elt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElement)
}

pub fn getElementTypePath(element: Arc<SCode::Element>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    path = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { .. } => AbsynUtil::typeSpecPath(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone())?,
        Deref @ SCode::EXTENDS { .. } => var_field!((*element).baseClassPath, SCode::Element::EXTENDS).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(path)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getElementWithId(inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, inId: ArcStr) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &((inProgram.clone(), inId.clone())) {
        (Deref @ metamodelica::List::Cons { head: e @ Deref @ SCode::CLASS { name: n, .. }, tail: _ }, i) if (stringEq((n.clone()).clone(), (i.clone()).clone())) => e.clone(),
        (Deref @ metamodelica::List::Cons { head: e @ Deref @ SCode::COMPONENT { name: n, .. }, tail: _ }, i) if (stringEq((n.clone()).clone(), (i.clone()).clone())) => e.clone(),
        (Deref @ metamodelica::List::Cons { head: e @ Deref @ SCode::EXTENDS { baseClassPath: p, .. }, tail: _ }, i) if (stringEq((AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone(), (i.clone()).clone())) => e.clone(),
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, i) => getElementWithId(rest.clone(), (i.clone()).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getElementWithPath(inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, inPath: Arc<Absyn::Path>) -> Result<Arc<SCode::Element>> {
    let mut outElement: Arc<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &((inProgram.clone(), inPath.clone())) {
        (_, Deref @ Absyn::FULLYQUALIFIED { path: p }) => getElementWithPath(inProgram.clone(), p.clone())?,
        (_, Deref @ Absyn::IDENT { name: i }) => {
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut rest: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut c: Arc<SCode::Element>;
            let mut e: Arc<SCode::Element>;
            let mut p: Arc<Absyn::Path>;
            let mut n: ArcStr;
            e = getElementWithId(inProgram.clone(), (i.clone()).clone())?;
            e.clone()
        },
        (_, Deref @ Absyn::QUALIFIED { name: i, path: p }) => {
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut rest: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut c: Arc<SCode::Element>;
            let mut e: Arc<SCode::Element>;
            let mut n: ArcStr;
            e = getElementWithId(inProgram.clone(), (i.clone()).clone())?;
            sp = getElementsFromElement(inProgram.clone(), e.clone())?;
            e = getElementWithPath(sp.clone(), p.clone())?;
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getElementsFromElement(inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, inElement: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outProgram = (::match_deref::match_deref! { match &((inProgram.clone(), inElement.clone())) {
        (_, Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { elementLst: els, .. }, .. }) => els.clone(),
        (_, Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { composition: Deref @ SCode::PARTS { elementLst: els, .. }, .. }, .. }) => els.clone(),
        (_, Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { typeSpec: Deref @ Absyn::TPATH { path: p, .. }, .. }, .. }) => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut e: Arc<SCode::Element>;
            let mut i: ArcStr;
            e = getElementWithPath(inProgram.clone(), p.clone())?;
            els = getElementsFromElement(inProgram.clone(), e.clone())?;
            els.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outProgram)
}

pub fn getEquationInfo(inEquation: Arc<SCode::Equation>) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ SCode::EQ_IF { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_IF).clone(),
        Deref @ SCode::EQ_EQUALS { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_EQUALS).clone(),
        Deref @ SCode::EQ_PDE { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_PDE).clone(),
        Deref @ SCode::EQ_CONNECT { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_CONNECT).clone(),
        Deref @ SCode::EQ_FOR { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_FOR).clone(),
        Deref @ SCode::EQ_WHEN { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_WHEN).clone(),
        Deref @ SCode::EQ_ASSERT { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_ASSERT).clone(),
        Deref @ SCode::EQ_TERMINATE { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_TERMINATE).clone(),
        Deref @ SCode::EQ_REINIT { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_REINIT).clone(),
        Deref @ SCode::EQ_NORETCALL { .. } => var_field!((*inEquation).info, SCode::Equation::EQ_NORETCALL).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(info)
}

pub fn getEvaluateAnnotation(cmt: Arc<SCode::Comment>) -> Result<Option<bool>> {
    let mut value: Option<bool>;
    let mut ann: Arc<SCode::Annotation>;
    let mut binding: Option<Arc<Absyn::Exp>>;
    value = (::match_deref::match_deref! { match &(cmt.clone()) {
        Deref @ SCode::COMMENT { annotation_: Some(ann), .. } => lookupBooleanAnnotation(ann.clone(), (literal!("Evaluate")).clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub fn getExternalObjectConstructor(inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element>;
    cl = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Cons { head: cl @ Deref @ SCode::CLASS { name: Deref @ "constructor", .. }, tail: _ } => cl.clone(),
        Deref @ metamodelica::List::Cons { head: _, tail: els } => getExternalObjectConstructor(els.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn getExternalObjectDestructor(inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element>;
    cl = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Cons { head: cl @ Deref @ SCode::CLASS { name: Deref @ "destructor", .. }, tail: _ } => cl.clone(),
        Deref @ metamodelica::List::Cons { head: _, tail: els } => getExternalObjectDestructor(els.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn getFunctionRestrictionPurity(restr: SCode::FunctionRestriction) -> Absyn::FunctionPurity {
    let mut purity: Absyn::FunctionPurity;
    purity = (match restr.clone() {
        SCode::FR_NORMAL_FUNCTION { purity: mut purity } => purity.clone(),
        SCode::FR_EXTERNAL_FUNCTION { purity: mut purity } => purity.clone(),
        _ => openmodelica_ast::Absyn::FunctionPurity::NO_PURITY,
    });
    purity
}

pub fn getModifierBinding(inMod: Arc<SCode::Mod>) -> Option<Arc<Absyn::Exp>> {
    let mut outBinding: Option<Arc<Absyn::Exp>>;
    outBinding = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::MOD { .. } => var_field!((*inMod).binding, SCode::Mod::MOD).clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBinding
}

pub fn getModifierInfo(inMod: Arc<SCode::Mod>) -> SourceInfo {
    let mut outInfo: SourceInfo;
    outInfo = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::MOD { info, .. } => info.clone(),
        Deref @ SCode::REDECL { element: el, .. } => elementInfo(el.clone()),
        Deref @ SCode::BREAK_COMPONENT { .. } => var_field!((*inMod).info, SCode::Mod::BREAK_COMPONENT).clone(),
        Deref @ SCode::BREAK_CONNECT { .. } => var_field!((*inMod).info, SCode::Mod::BREAK_CONNECT).clone(),
        _ => AbsynUtil::dummyInfo.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInfo
}

pub fn getStatementInfo(inStatement: Arc<SCode::Statement>) -> Result<SourceInfo> {
    let mut outInfo: SourceInfo;
    outInfo = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ SCode::ALG_ASSIGN { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_ASSIGN).clone(),
        Deref @ SCode::ALG_IF { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_IF).clone(),
        Deref @ SCode::ALG_FOR { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_FOR).clone(),
        Deref @ SCode::ALG_PARFOR { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_PARFOR).clone(),
        Deref @ SCode::ALG_WHILE { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_WHILE).clone(),
        Deref @ SCode::ALG_WHEN_A { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_WHEN_A).clone(),
        Deref @ SCode::ALG_ASSERT { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_ASSERT).clone(),
        Deref @ SCode::ALG_TERMINATE { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_TERMINATE).clone(),
        Deref @ SCode::ALG_REINIT { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_REINIT).clone(),
        Deref @ SCode::ALG_NORETCALL { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_NORETCALL).clone(),
        Deref @ SCode::ALG_RETURN { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_RETURN).clone(),
        Deref @ SCode::ALG_BREAK { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_BREAK).clone(),
        Deref @ SCode::ALG_FAILURE { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_FAILURE).clone(),
        Deref @ SCode::ALG_TRY { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_TRY).clone(),
        Deref @ SCode::ALG_CONTINUE { .. } => var_field!((*inStatement).info, SCode::Statement::ALG_CONTINUE).clone(),
        _ => {
            Error::addInternalError((literal!("SCodeUtil.getStatementInfo failed")).clone(), metamodelica::sourceInfo!())?;
            AbsynUtil::dummyInfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outInfo)
}

pub fn hasBooleanNamedAnnotation(inAnnotation: Arc<SCode::Annotation>, inName: ArcStr) -> Result<bool> {
    let mut outHasEntry: bool;
    let mut binding: Option<Arc<Absyn::Exp>>;
    binding = lookupAnnotationBinding(inAnnotation.clone(), (inName.clone()).clone())?;
    outHasEntry = (match binding.clone() {
        Some(Absyn::BOOL { value: true }) => true,
        _ => false,
    });
    Ok(outHasEntry)
}

pub fn hasBooleanNamedAnnotationFalse(inAnnotation: Arc<SCode::Annotation>, inName: ArcStr) -> Result<bool> {
    let mut outHasEntry: bool;
    let mut binding: Option<Arc<Absyn::Exp>>;
    binding = lookupAnnotationBinding(inAnnotation.clone(), (inName.clone()).clone())?;
    outHasEntry = (match binding.clone() {
        Some(Absyn::BOOL { value: false }) => true,
        _ => false,
    });
    Ok(outHasEntry)
}

pub fn hasBooleanNamedAnnotationInClass(inClass: Arc<SCode::Element>, namedAnnotation: ArcStr) -> Result<bool> {
    let mut hasAnn: bool;
    hasAnn = (::match_deref::match_deref! { match &((inClass.clone(), namedAnnotation.clone())) {
        (Deref @ SCode::CLASS { cmt: Deref @ SCode::COMMENT { annotation_: Some(ann), .. }, .. }, _) => hasBooleanNamedAnnotation(ann.clone(), (namedAnnotation.clone()).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasAnn)
}

pub fn hasBooleanNamedAnnotationInComponent(inComponent: Arc<SCode::Element>, namedAnnotation: ArcStr) -> Result<bool> {
    let mut hasAnn: bool;
    hasAnn = (::match_deref::match_deref! { match &((inComponent.clone(), namedAnnotation.clone())) {
        (Deref @ SCode::COMPONENT { comment: Deref @ SCode::COMMENT { annotation_: Some(ann), .. }, .. }, _) => hasBooleanNamedAnnotation(ann.clone(), (namedAnnotation.clone()).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasAnn)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasExtendsOfExternalObject(inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Nil => false,
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::EXTENDS { baseClassPath: path, .. }, tail: _ } if (AbsynUtil::pathEqual(path.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("ExternalObject")).clone() }))) => true,
        Deref @ metamodelica::List::Cons { head: _, tail: els } => hasExtendsOfExternalObject(els.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasExternalObjectConstructor(inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::CLASS { name: Deref @ "constructor", .. }, tail: _ } => true,
        Deref @ metamodelica::List::Cons { head: _, tail: els } => hasExternalObjectConstructor(els.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn hasExternalObjectDestructor(inEls: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(inEls.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ SCode::CLASS { name: Deref @ "destructor", .. }, tail: _ } => true,
        Deref @ metamodelica::List::Cons { head: _, tail: els } => hasExternalObjectDestructor(els.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn hasNamedExternalCall(name: ArcStr, def: Arc<SCode::ClassDef>) -> bool {
    let mut hasCall: bool;
    hasCall = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { funcName: Some(fn_name), .. }), .. } => fn_name.clone() == name.clone(),
        Deref @ SCode::CLASS_EXTENDS { .. } => hasNamedExternalCall((name.clone()).clone(), var_field!((*def).composition, SCode::ClassDef::CLASS_EXTENDS).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasCall
}

pub fn isArrayComponent(inElement: Arc<SCode::Element>) -> bool {
    let mut outIsArray: bool;
    outIsArray = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::COMPONENT { attributes: SCode::ATTR { arrayDims: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsArray
}

pub fn isBreakComponentSubMod(subMod: Arc<SCode::SubMod>) -> bool {
    let mut isBreak: bool;
    isBreak = (::match_deref::match_deref! { match &(subMod.clone()) {
        Deref @ SCode::NAMEMOD { r#mod: Deref @ SCode::Mod::BREAK_COMPONENT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBreak
}

pub fn isBreakConnectSubMod(subMod: Arc<SCode::SubMod>) -> bool {
    let mut isBreak: bool;
    isBreak = (::match_deref::match_deref! { match &(subMod.clone()) {
        Deref @ SCode::NAMEMOD { r#mod: Deref @ SCode::Mod::BREAK_CONNECT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBreak
}

pub fn isBreakSubMod(subMod: Arc<SCode::SubMod>) -> bool {
    let mut isBreak: bool;
    isBreak = (::match_deref::match_deref! { match &(subMod.r#mod.clone()) {
        Deref @ SCode::Mod::BREAK_COMPONENT { .. } => true,
        Deref @ SCode::Mod::BREAK_CONNECT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBreak
}

pub fn isBuiltinElement(inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut outIsBuiltin: bool;
    outIsBuiltin = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { lang: Some(Deref @ "builtin"), .. }), .. }, .. } => true,
        Deref @ SCode::CLASS { cmt: Deref @ SCode::COMMENT { annotation_: Some(ann), .. }, .. } => hasBooleanNamedAnnotation(ann.clone(), (literal!("__OpenModelica_builtin")).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsBuiltin)
}

pub fn isBuiltinFunction(cl: Arc<SCode::Element>, inVars: Arc<metamodelica::List<ArcStr>>, outVars: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = (::match_deref::match_deref! { match &((cl.clone(), inVars.clone(), outVars.clone())) {
        (Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { lang: Some(Deref @ "builtin"), funcName: None, .. }), .. }, restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_EXTERNAL_FUNCTION { .. } }, name, .. }, _, _) => name.clone(),
        (Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { lang: Some(Deref @ "builtin"), funcName: Some(name), .. }), .. }, restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_EXTERNAL_FUNCTION { .. } }, .. }, _, _) => name.clone(),
        (Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { lang: Some(Deref @ "builtin"), funcName: None, .. }), .. }, restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_PARALLEL_FUNCTION }, name, .. }, _, _) => name.clone(),
        (Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { lang: Some(Deref @ "builtin"), funcName: Some(name), .. }), .. }, restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_PARALLEL_FUNCTION }, .. }, _, _) => name.clone(),
        (Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { args, output_: Some(Absyn::CREF_IDENT { name: outVar2, subscripts: Deref @ metamodelica::List::Nil }), lang: Some(Deref @ "C"), funcName: Some(name), .. }), .. }, restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_EXTERNAL_FUNCTION { .. } }, .. }, _, Deref @ metamodelica::List::Cons { head: outVar1, tail: Deref @ metamodelica::List::Nil }) => {
            let mut argsStr: Arc<metamodelica::List<ArcStr>>;
            let true = (listMember(name.clone(), knownExternalCFunctions.clone())) else { bail!("pattern mismatch") };
            let true = (outVar2.clone() == outVar1.clone()) else { bail!("pattern mismatch") };
            argsStr = List::mapMap(args.clone(), Arc::new(AbsynUtil::expCref), Arc::new(AbsynUtil::crefIdent));
            let true = (argsStr.clone() == inVars.clone()) else { bail!("pattern mismatch") };
            name.clone()
        },
        (Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { lang: Some(Deref @ "C"), funcName: None, .. }), .. }, restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_EXTERNAL_FUNCTION { .. } }, name, .. }, _, _) => {
            let mut outVar1: ArcStr;
            let mut outVar2: ArcStr;
            let mut argsStr: Arc<metamodelica::List<ArcStr>>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let true = (listMember(name.clone(), knownExternalCFunctions.clone())) else { bail!("pattern mismatch") };
            name.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(name)
}

pub fn isClass(inElement: Arc<SCode::Element>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::CLASS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClass
}

pub fn isClassExtends(cls: Arc<SCode::Element>) -> bool {
    let mut isCE: bool;
    isCE = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCE
}

pub fn isClassNamed(inName: ArcStr, inClass: Arc<SCode::Element>) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (::match_deref::match_deref! { match &((inName.clone(), inClass.clone())) {
        (_, Deref @ SCode::CLASS { name, .. }) => stringEq((inName.clone()).clone(), (name.clone()).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

pub fn isClassOrComponent(inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut outIsClassOrComponent: bool;
    outIsClassOrComponent = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::CLASS { .. } => true,
        Deref @ SCode::COMPONENT { .. } => true,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outIsClassOrComponent)
}

pub fn isComponent(elt: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::COMPONENT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isComponentWithDirection(elt: Arc<SCode::Element>, dir1: Absyn::Direction) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((elt.clone(), dir1.clone())) {
        (Deref @ SCode::COMPONENT { attributes: SCode::ATTR { direction: dir2, .. }, .. }, _) => AbsynUtil::directionEqual(dir1.clone(), dir2.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isConnector(inRestriction: SCode::Restriction) -> bool {
    let mut isConnector: bool;
    isConnector = (match inRestriction.clone() {
        SCode::R_CONNECTOR { .. } => true,
        _ => false,
    });
    isConnector
}

pub fn isConstant(inVariability: SCode::Variability) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inVariability.clone() {
        SCode::CONST => true,
        _ => false,
    });
    outBoolean
}

pub fn isDerivedClass(inClass: Arc<SCode::Element>) -> bool {
    let mut isDerived: bool;
    isDerived = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDerived
}

pub fn isDerivedClassDef(inClassDef: Arc<SCode::ClassDef>) -> bool {
    let mut isDerived: bool;
    isDerived = (::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ SCode::DERIVED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDerived
}

pub fn isElementEncapsulated(inElement: Arc<SCode::Element>) -> bool {
    let mut outIsEncapsulated: bool;
    outIsEncapsulated = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::CLASS { encapsulatedPrefix: SCode::ENCAPSULATED, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEncapsulated
}

pub fn isElementExtends(ele: Arc<SCode::Element>) -> bool {
    let mut isExtend: bool;
    isExtend = (::match_deref::match_deref! { match &(ele.clone()) {
        Deref @ SCode::EXTENDS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtend
}

pub fn isElementExtendsOrClassExtends(ele: Arc<SCode::Element>) -> bool {
    let mut isExtend: bool;
    isExtend = (::match_deref::match_deref! { match &(ele.clone()) {
        Deref @ SCode::EXTENDS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtend
}

pub fn isElementNamed(name: ArcStr, element: Arc<SCode::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::CLASS { .. } => var_field!((*element).name, SCode::Element::CLASS).clone() == name.clone(),
        Deref @ SCode::COMPONENT { .. } => var_field!((*element).name, SCode::Element::COMPONENT).clone() == name.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isElementProtected(inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut outIsProtected: bool;
    outIsProtected = !(visibilityBool(elementVisibility(inElement.clone())?)?);
    Ok(outIsProtected)
}

pub fn isElementPublic(inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut outIsPublic: bool;
    outIsPublic = visibilityBool(elementVisibility(inElement.clone())?)?;
    Ok(outIsPublic)
}

pub fn isElementRedeclare(inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut isRedeclare: bool;
    let mut pf: Arc<SCode::Prefixes>;
    pf = elementPrefixes(inElement.clone())?;
    isRedeclare = redeclareBool(prefixesRedeclare(pf.clone())?)?;
    Ok(isRedeclare)
}

pub fn isElementReplaceable(inElement: Arc<SCode::Element>) -> Result<bool> {
    let mut isReplaceable: bool;
    let mut pf: Arc<SCode::Prefixes>;
    pf = elementPrefixes(inElement.clone())?;
    isReplaceable = replaceableBool(prefixesReplaceable(pf.clone())?)?;
    Ok(isReplaceable)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isEmptyClassDef(cdef: Arc<SCode::ClassDef>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::PARTS { .. } => var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone().is_empty() && var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone().is_empty() && isNone(var_field!((*cdef).externalDecl, SCode::ClassDef::PARTS).clone()),
        Deref @ SCode::CLASS_EXTENDS { .. } => isEmptyClassDef(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone()),
        Deref @ SCode::ENUMERATION { .. } => var_field!((*cdef).enumLst, SCode::ClassDef::ENUMERATION).clone().is_empty(),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn isEmptyMod(r#mod: Arc<SCode::Mod>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::NOMOD => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn isEnumeration(el: Arc<SCode::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::CLASS { restriction: SCode::R_ENUMERATION, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isExternalFunctionRestriction(inRestr: SCode::FunctionRestriction) -> bool {
    let mut isExternal: bool;
    isExternal = (match inRestr.clone() {
        SCode::FR_EXTERNAL_FUNCTION { .. } => true,
        _ => false,
    });
    isExternal
}

pub fn isExternalObject(els: Arc<metamodelica::List<Arc<SCode::Element>>>) -> bool {
    let mut res: bool;
    res = if ((els.clone().len() as i32) == 3) {hasExtendsOfExternalObject(els.clone()) && hasExternalObjectDestructor(els.clone()) && hasExternalObjectConstructor(els.clone())} else {false};
    res
}

pub fn isFunction(inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { restriction: SCode::R_FUNCTION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isFunctionOrExtFunctionRestriction(r: SCode::Restriction) -> bool {
    let mut res: bool;
    res = (match r.clone() {
        SCode::R_FUNCTION { functionRestriction: SCode::FR_NORMAL_FUNCTION { .. } } => true,
        SCode::R_FUNCTION { functionRestriction: SCode::FR_EXTERNAL_FUNCTION { .. } } => true,
        _ => false,
    });
    res
}

pub fn isFunctionRestriction(inRestriction: SCode::Restriction) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inRestriction.clone() {
        SCode::R_FUNCTION { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isImport(element: Arc<SCode::Element>) -> bool {
    let mut isImport: bool;
    isImport = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::IMPORT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isImport
}

pub fn isImpureFunctionRestriction(inRestr: SCode::FunctionRestriction) -> bool {
    let mut isExternal: bool;
    isExternal = (match inRestr.clone() {
        SCode::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } => true,
        SCode::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } => true,
        _ => false,
    });
    isExternal
}

pub fn isInitial(inInitial: SCode::Initial) -> bool {
    let mut isIn: bool;
    isIn = (match inInitial.clone() {
        SCode::INITIAL => true,
        _ => false,
    });
    isIn
}

pub fn isInnerComponent(inElement: Arc<SCode::Element>) -> bool {
    let mut outIsInner: bool;
    outIsInner = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::COMPONENT { prefixes: Deref @ SCode::PREFIXES { innerOuter: io, .. }, .. } => AbsynUtil::isInner(io.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsInner
}

pub fn isInstantiableClassRestriction(inRestriction: SCode::Restriction) -> bool {
    let mut outIsInstantiable: bool;
    outIsInstantiable = (match inRestriction.clone() {
        SCode::R_CLASS => true,
        SCode::R_MODEL => true,
        SCode::R_RECORD { .. } => true,
        SCode::R_BLOCK => true,
        SCode::R_CONNECTOR { .. } => true,
        SCode::R_TYPE => true,
        SCode::R_ENUMERATION => true,
        _ => false,
    });
    outIsInstantiable
}

pub fn isNonEmptyAlgorithm(alg: Arc<SCode::AlgorithmSection>) -> bool {
    let mut res: bool = !(alg.statements.clone().is_empty());
    res
}

fn isNotBuiltinClass(inClass: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::PARTS { externalDecl: Some(SCode::EXTERNALDECL { lang: Some(Deref @ "builtin"), .. }), .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isNotComponent(elt: Arc<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::COMPONENT { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isNotElementClassExtends(ele: Arc<SCode::Element>) -> bool {
    let mut isExtend: bool;
    isExtend = (::match_deref::match_deref! { match &(ele.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtend
}

pub fn isOperator(el: Arc<SCode::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::CLASS { restriction: SCode::R_OPERATOR, .. } => true,
        Deref @ SCode::CLASS { restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_OPERATOR_FUNCTION }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isOperatorRecord(inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { restriction: SCode::R_RECORD { isOperator: true }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isOverloadedFunction(inElement: Arc<SCode::Element>) -> bool {
    let mut isOverloaded: bool;
    isOverloaded = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::OVERLOAD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOverloaded
}

pub fn isPackage(inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { restriction: SCode::R_PACKAGE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isParameterOrConst(inVariability: SCode::Variability) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inVariability.clone() {
        SCode::PARAM => true,
        SCode::CONST => true,
        _ => false,
    });
    outBoolean
}

pub fn isPartial(inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { partialPrefix: SCode::PARTIAL, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isPolymorphicTypeVar(cls: Arc<SCode::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { typeSpec: Deref @ Absyn::TCOMPLEX { path: Deref @ Absyn::IDENT { name: Deref @ "polymorphic" }, .. }, .. }, restriction: SCode::R_TYPE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isRecord(inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { restriction: SCode::R_RECORD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isRedeclareElement(element: Arc<SCode::Element>) -> bool {
    let mut isElement: bool;
    isElement = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { prefixes: Deref @ SCode::PREFIXES { redeclarePrefix: SCode::REDECLARE, .. }, .. } => true,
        Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { .. }, .. } => false,
        Deref @ SCode::CLASS { prefixes: Deref @ SCode::PREFIXES { redeclarePrefix: SCode::REDECLARE, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isElement
}

pub fn isRedeclareSubMod(inSubMod: Arc<SCode::SubMod>) -> bool {
    let mut outIsRedeclare: bool;
    outIsRedeclare = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::NAMEMOD { r#mod: Deref @ SCode::REDECL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsRedeclare
}

pub fn isRestrictionImpure(inRestr: SCode::Restriction, hasZeroOutputPreMSL3_2: bool) -> bool {
    let mut isImpure: bool;
    isImpure = (match inRestr.clone() {
        SCode::R_FUNCTION { functionRestriction: SCode::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } } => true,
        SCode::R_FUNCTION { functionRestriction: SCode::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } } => true,
        SCode::R_FUNCTION { functionRestriction: SCode::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::NO_PURITY { .. } } } => !(hasZeroOutputPreMSL3_2.clone()),
        _ => false,
    });
    isImpure
}

pub fn isTypeVar(inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { restriction: SCode::R_TYPE, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isUniontype(inClass: Arc<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ SCode::CLASS { restriction: SCode::R_UNIONTYPE { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isValidPackageElement(inElement: Arc<SCode::Element>) -> bool {
    let mut outIsValid: bool;
    outIsValid = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::COMPONENT { attributes: SCode::ATTR { variability: SCode::CONST, .. }, .. } => true,
        Deref @ SCode::COMPONENT { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsValid
}

pub static knownExternalCFunctions: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("sin")).clone(), (literal!("cos")).clone(), (literal!("tan")).clone(), (literal!("asin")).clone(), (literal!("acos")).clone(), (literal!("atan")).clone(), (literal!("atan2")).clone(), (literal!("sinh")).clone(), (literal!("cosh")).clone(), (literal!("tanh")).clone(), (literal!("exp")).clone(), (literal!("log")).clone(), (literal!("log10")).clone(), (literal!("sqrt")).clone()] });

pub fn lookupAnnotation(ann: Arc<SCode::Annotation>, name: ArcStr) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod>;
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    let mut id: ArcStr;
    r#mod = (::match_deref::match_deref! { match &(ann.clone()) {
        Deref @ SCode::ANNOTATION { modification: Deref @ SCode::MOD { subModLst: submods, .. } } => {
            for sm in submods.clone() /* Unknown type for iterator Unknown */ {
                let SCode::NAMEMOD { ident: __pa0, r#mod: __pa1 } = (sm.clone()) else { bail!("pattern mismatch") };
                id = __pa0.clone();
                r#mod = __pa1.clone();
                if id.clone() == name.clone() {
                    return Ok(r#mod);
                }
            }
            Arc::new(crate::SCode::Mod::NOMOD)
        },
        _ => Arc::new(crate::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn lookupAnnotationBinding(ann: Arc<SCode::Annotation>, name: ArcStr) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut binding: Option<Arc<Absyn::Exp>>;
    binding = getModifierBinding(lookupAnnotation(ann.clone(), (name.clone()).clone())?);
    Ok(binding)
}

pub fn lookupAnnotations(ann: Arc<SCode::Annotation>, name: ArcStr) -> Result<Arc<metamodelica::List<Arc<SCode::Mod>>>> {
    let mut mods: Arc<metamodelica::List<Arc<SCode::Mod>>> = metamodelica::nil();
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    let mut id: ArcStr;
    let mut r#mod: Arc<SCode::Mod>;
    mods = (::match_deref::match_deref! { match &(ann.clone()) {
        Deref @ SCode::ANNOTATION { modification: Deref @ SCode::MOD { subModLst: submods, .. } } => {
            for sm in submods.clone() /* Unknown type for iterator Unknown */ {
                let SCode::NAMEMOD { ident: __pa0, r#mod: __pa1 } = (sm.clone()) else { bail!("pattern mismatch") };
                id = __pa0.clone();
                r#mod = __pa1.clone();
                if id.clone() == name.clone() {
                    mods = cons(r#mod.clone(), mods.clone());
                }
            }
            mods.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(mods)
}

pub fn lookupBooleanAnnotation(ann: Arc<SCode::Annotation>, name: ArcStr) -> Result<Option<bool>> {
    let mut value: Option<bool>;
    let mut binding: Option<Arc<Absyn::Exp>>;
    let mut bval: bool;
    binding = lookupAnnotationBinding(ann.clone(), (name.clone()).clone())?;
    value = (match binding.clone() {
        Some(Absyn::Exp::BOOL { value: mut bval }) => Some(bval.clone()),
        _ => None,
    });
    Ok(value)
}

pub fn lookupBooleanAnnotationMod(r#mod: Arc<SCode::Mod>) -> Option<bool> {
    let mut value: Option<bool>;
    let mut binding: Option<Arc<Absyn::Exp>>;
    let mut bval: bool;
    binding = getModifierBinding(r#mod.clone());
    value = (match binding.clone() {
        Some(Absyn::Exp::BOOL { value: mut bval }) => Some(bval.clone()),
        _ => None,
    });
    value
}

pub fn lookupElementAnnotation(element: Arc<SCode::Element>, name: ArcStr) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod>;
    let mut ann: Option<Arc<SCode::Annotation>>;
    ann = getElementAnnotation(element.clone(), (name.clone()).clone());
    r#mod = if (isSome(ann.clone())) {lookupAnnotation(Util::getOption(ann.clone())?, (name.clone()).clone())?} else {Arc::new(crate::SCode::Mod::NOMOD)};
    Ok(r#mod)
}

pub fn lookupElementAnnotationBinding(element: Arc<SCode::Element>, name: ArcStr) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut binding: Option<Arc<Absyn::Exp>>;
    binding = getModifierBinding(lookupElementAnnotation(element.clone(), (name.clone()).clone())?);
    Ok(binding)
}

pub fn lookupModInMod(name: ArcStr, r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            for m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                if m.ident.clone() == name.clone() {
                    outMod = m.r#mod.clone();
                    return outMod;
                }
            }
            Arc::new(crate::SCode::Mod::NOMOD)
        },
        _ => Arc::new(crate::SCode::Mod::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub fn makeClassPartial(inClass: Arc<SCode::Element>) -> Arc<SCode::Element> {
    let mut outClass: Arc<SCode::Element> = inClass.clone();
    outClass = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ SCode::CLASS { partialPrefix: SCode::NOT_PARTIAL, .. } => {
            assign_variant_field!(outClass => SCode::Element::CLASS; partialPrefix = crate::SCode::Partial::PARTIAL);
            outClass.clone()
        },
        _ => outClass.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outClass
}

pub fn makeElementProtected(element: Arc<SCode::Element>) -> Arc<SCode::Element> {
    let mut element: Arc<SCode::Element> = element;
    let mut prefixes: Arc<SCode::Prefixes>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { prefixes: prefixes @ Deref @ SCode::PREFIXES { visibility: SCode::PUBLIC, .. }, .. } => {
            let mut prefixes = (*prefixes).clone();
            let __owned_variant_visibility_0 = crate::SCode::Visibility::PROTECTED;
            if let SCode::Prefixes::PREFIXES { visibility, .. } = &mut prefixes {
                *visibility = __owned_variant_visibility_0;
            } else { panic!("owned-variant field-assign: value held a different variant than SCode::Prefixes::PREFIXES"); }
            assign_variant_field!(element => SCode::Element::COMPONENT; prefixes = Arc::new(prefixes.clone()));
            ()
        },
        Deref @ SCode::EXTENDS { visibility: SCode::PUBLIC, .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; visibility = crate::SCode::Visibility::PROTECTED);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    element
}

pub fn makeEnumType(inEnum: Arc<SCode::Enum>, inInfo: SourceInfo) -> Result<Arc<SCode::Element>> {
    let mut outEnumType: Arc<SCode::Element>;
    let mut literal: ArcStr;
    let mut comment: Arc<SCode::Comment>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inEnum.clone()) {
        Deref @ SCode::ENUM { comment: __pa0, literal: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comment = __pa0.clone();
    literal = __pa1.clone();
    checkValidEnumLiteral((literal.clone()).clone(), inInfo.clone())?;
    outEnumType = Arc::new(SCode::Element::COMPONENT { name: (literal.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultConstAttr.clone(), typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("EnumType")).clone() }), arrayDim: None }), modifications: Arc::new(crate::SCode::Mod::NOMOD), comment: comment.clone(), condition: None, info: inInfo.clone() });
    Ok(outEnumType)
}

pub fn makeMod(isFinal: bool, isEach: bool, subMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>, binding: Option<Arc<Absyn::Exp>>, comment: Option<ArcStr>, info: SourceInfo) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod>;
    r#mod = Arc::new(SCode::Mod::MOD { finalPrefix: if (isFinal.clone()) {crate::SCode::Final::FINAL} else {crate::SCode::Final::NOT_FINAL}, eachPrefix: if (isEach.clone()) {crate::SCode::Each::EACH} else {crate::SCode::Each::NOT_EACH}, subModLst: subMods.clone(), binding: binding.clone(), comment: comment.clone(), info: info.clone() });
    r#mod
}

pub fn makeSingleAnnotation(name: ArcStr, value: Arc<Absyn::Exp>) -> Arc<SCode::Annotation> {
    let mut ann: Arc<SCode::Annotation>;
    ann = Arc::new(SCode::Annotation { modification: Arc::new(SCode::Mod::MOD { finalPrefix: crate::SCode::Final::NOT_FINAL, eachPrefix: crate::SCode::Each::NOT_EACH, subModLst: list![Arc::new(SCode::SubMod { ident: (name.clone()).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: crate::SCode::Final::NOT_FINAL, eachPrefix: crate::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(value.clone()), comment: None, info: AbsynUtil::dummyInfo.clone() }) })], binding: None, comment: None, info: AbsynUtil::dummyInfo.clone() }) });
    ann
}

pub fn mapAlgorithmStatements(alg: Arc<SCode::AlgorithmSection>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>) -> Arc<SCode::AlgorithmSection> {
    pub type Func = fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>>;

    let mut alg: Arc<SCode::AlgorithmSection> = alg;
    assign_field!(alg.statements = mapStatementsList(alg.statements.clone(), func.clone()));
    alg
}

pub fn mapElement(element: Arc<SCode::Element>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>) -> Arc<SCode::Element> {
    pub type Func = fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>>;

    let mut element: Arc<SCode::Element> = element;
    let mut def: Arc<SCode::ClassDef>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            def = mapElementsClassDef(var_field!((*element).classDef, SCode::Element::CLASS).clone(), func.clone());
            if !(referenceEq(&def.clone(),&var_field!((*element).classDef, SCode::Element::CLASS).clone())) {
                assign_variant_field!(element => SCode::Element::CLASS; classDef = def.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    element = func(element.clone()).unwrap();
    element
}

pub fn mapElements(elements: Arc<metamodelica::List<Arc<SCode::Element>>>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    pub type Func = fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>>;

    let mut elements: Arc<metamodelica::List<Arc<SCode::Element>>> = elements;
    elements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for e in (elements.clone()).into_iter().cloned() {
            let __x = mapElement(e.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    elements
}

pub fn mapElementsClassDef(classDef: Arc<SCode::ClassDef>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>) -> Arc<SCode::ClassDef> {
    pub type Func = fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>>;

    let mut classDef: Arc<SCode::ClassDef> = classDef;
    let mut def: Arc<SCode::ClassDef>;
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for e in (var_field!((*classDef).elementLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = mapElement(e.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            def = mapElementsClassDef(var_field!((*classDef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), func.clone());
            if !(referenceEq(&def.clone(),&var_field!((*classDef).composition, SCode::ClassDef::CLASS_EXTENDS).clone())) {
                assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; composition = def.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    classDef
}

pub fn mapEquationExps(eq: Arc<SCode::Equation>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<Arc<SCode::Equation>> {
    pub type Func = fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>>;

    let mut eq: Arc<SCode::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF; condition = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for e in (var_field!((*eq).condition, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::Equation::EQ_EQUALS { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_EQUALS;
                expLeft = func(var_field!((*eq).expLeft, SCode::Equation::EQ_EQUALS).clone())?,
                expRight = func(var_field!((*eq).expRight, SCode::Equation::EQ_EQUALS).clone())?
            );
            ()
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_PDE;
                expLeft = func(var_field!((*eq).expLeft, SCode::Equation::EQ_PDE).clone())?,
                expRight = func(var_field!((*eq).expRight, SCode::Equation::EQ_PDE).clone())?,
                domain = AbsynUtil::mapCrefExps(var_field!((*eq).domain, SCode::Equation::EQ_PDE).clone(), func.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT;
                crefLeft = AbsynUtil::mapCrefExps(var_field!((*eq).crefLeft, SCode::Equation::EQ_CONNECT).clone(), func.clone()),
                crefRight = AbsynUtil::mapCrefExps(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), func.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            if isSome(var_field!((*eq).range, SCode::Equation::EQ_FOR).clone()) {
                assign_variant_field!(eq => SCode::Equation::EQ_FOR; range = Some(func(Util::getOption(var_field!((*eq).range, SCode::Equation::EQ_FOR).clone())?)?));
            }
            ()
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                condition = func(var_field!((*eq).condition, SCode::Equation::EQ_WHEN).clone())?,
                elseBranches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        for b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = Util::applyTuple21(b.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ SCode::Equation::EQ_ASSERT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_ASSERT;
                condition = func(var_field!((*eq).condition, SCode::Equation::EQ_ASSERT).clone())?,
                message = func(var_field!((*eq).message, SCode::Equation::EQ_ASSERT).clone())?,
                level = func(var_field!((*eq).level, SCode::Equation::EQ_ASSERT).clone())?
            );
            ()
        },
        Deref @ SCode::Equation::EQ_TERMINATE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_TERMINATE; message = func(var_field!((*eq).message, SCode::Equation::EQ_TERMINATE).clone())?);
            ()
        },
        Deref @ SCode::Equation::EQ_REINIT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_REINIT;
                cref = func(var_field!((*eq).cref, SCode::Equation::EQ_REINIT).clone())?,
                expReinit = func(var_field!((*eq).expReinit, SCode::Equation::EQ_REINIT).clone())?
            );
            ()
        },
        Deref @ SCode::Equation::EQ_NORETCALL { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_NORETCALL; exp = func(var_field!((*eq).exp, SCode::Equation::EQ_NORETCALL).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(eq)
}

pub fn mapEquations(eq: Arc<SCode::Equation>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>> + 'static>) -> Arc<SCode::Equation> {
    pub type Func = fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>>;

    let mut eq: Arc<SCode::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                thenBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
        for b in (var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = mapEquationsList(b.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = mapEquationsList(var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone(), func.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR; eEquationLst = mapEquationsList(var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                eEquationLst = mapEquationsList(var_field!((*eq).eEquationLst, SCode::Equation::EQ_WHEN).clone(), func.clone()),
                elseBranches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        for b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(b.clone()), mapEquationsList(Util::tuple22(b.clone()), func.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eq = func(eq.clone()).unwrap();
    eq
}

pub fn mapEquationsList(eql: Arc<metamodelica::List<Arc<SCode::Equation>>>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>> + 'static>) -> Arc<metamodelica::List<Arc<SCode::Equation>>> {
    pub type Func = fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>>;

    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = eql;
    eql = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for e in (eql.clone()).into_iter().cloned() {
            let __x = mapEquations(e.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    eql
}

fn mapFoldBranchExps<ArgT: Clone + 'static>(inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), traverser: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, inArg: ArgT) -> ((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)>;

    let mut outBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>);
    let mut outArg: ArgT;
    let mut arg: ArgT;
    let mut exp: Arc<Absyn::Exp>;
    let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
    (exp, stmts) = inBranch.clone();
    (exp, outArg) = traverser(exp.clone(), inArg.clone()).unwrap();
    outBranch = (exp.clone(), stmts.clone());
    (outBranch, outArg)
}

fn mapFoldBranchStatements<ArgT: Clone + 'static>(branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>, arg: ArgT) -> ((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)>;

    let mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>) = branch;
    let mut arg: ArgT = arg;
    let mut exp: Arc<Absyn::Exp>;
    let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
    (exp, stmts) = branch.clone();
    (stmts, arg) = mapFoldStatementsList(stmts.clone(), traverser.clone(), arg.clone());
    branch = (exp.clone(), stmts.clone());
    (branch, arg)
}

fn mapFoldComponentRefExps<ArgT: Clone + 'static>(inCref: Arc<Absyn::ComponentRef>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, inArg: ArgT) -> Result<(Arc<Absyn::ComponentRef>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)>;

    let mut outCref: Arc<Absyn::ComponentRef>;
    let mut outArg: ArgT;
    (outCref, outArg) = (::match_deref::match_deref! { match &((inCref.clone(), inFunc.clone(), inArg.clone())) {
        (Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cr }, _, _) => {
            let mut name: ArcStr;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut arg: ArgT;
            let mut cr = (*cr).clone();
            (cr, arg) = mapFoldComponentRefExps(cr.clone(), inFunc.clone(), inArg.clone())?;
            (AbsynUtil::crefMakeFullyQualified(cr.clone()), arg.clone())
        },
        (Deref @ Absyn::CREF_QUAL { componentRef: cr, subscripts: subs, name }, _, _) => {
            let mut arg: ArgT;
            let mut cr = (*cr).clone();
            let mut subs = (*subs).clone();
            (cr, arg) = mapFoldComponentRefExps(cr.clone(), inFunc.clone(), inArg.clone())?;
            (subs, arg) = List::map1Fold(subs.clone(), Arc::new(mapFoldSubscriptExps), inFunc.clone(), arg.clone());
            (Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (name.clone()).clone(), subscripts: subs.clone(), componentRef: cr.clone() }), arg.clone())
        },
        (Deref @ Absyn::CREF_IDENT { subscripts: subs, name }, _, _) => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            (subs, arg) = List::map1Fold(subs.clone(), Arc::new(mapFoldSubscriptExps), inFunc.clone(), inArg.clone());
            (Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subs.clone() }), arg.clone())
        },
        (Deref @ Absyn::WILD, _, _) => (inCref.clone(), inArg.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCref, outArg))
}

fn mapFoldElseWhenEquations<ArgT: Clone + 'static>(elseWhen: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>, arg: ArgT) -> ((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)>;

    let mut elseWhen: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>) = elseWhen;
    let mut arg: ArgT = arg;
    let mut exp: Arc<Absyn::Exp>;
    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    (exp, eql) = elseWhen.clone();
    (eql, arg) = mapFoldEquationsList(eql.clone(), traverser.clone(), arg.clone());
    elseWhen = (exp.clone(), eql.clone());
    (elseWhen, arg)
}

fn mapFoldElseWhenExps<ArgT: Clone + 'static>(inElseWhen: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), traverser: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, inArg: ArgT) -> ((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)>;

    let mut outElseWhen: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>);
    let mut outArg: ArgT;
    let mut exp: Arc<Absyn::Exp>;
    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    (exp, eql) = inElseWhen.clone();
    (exp, outArg) = traverser(exp.clone(), inArg.clone()).unwrap();
    outElseWhen = (exp.clone(), eql.clone());
    (outElseWhen, outArg)
}

pub fn mapFoldEquationExps<ArgT: Clone + 'static>(eq: Arc<SCode::Equation>, traverser: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, arg: ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)>;

    let mut eq: Arc<SCode::Equation> = eq;
    let mut arg: ArgT = arg;
    (eq, arg) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::EQ_IF { condition: expl1, thenBranch: then_branch, elseBranch: else_branch, comment, info } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut expl1 = (*expl1).clone();
            (expl1, arg) = AbsynUtil::traverseExpList(expl1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Equation::EQ_IF { condition: expl1.clone(), thenBranch: then_branch.clone(), elseBranch: else_branch.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_EQUALS { expLeft: e1, expRight: e2, comment, info } => {
            let mut e3: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (e2, arg) = traverser(e2.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_EQUALS { expLeft: e1.clone(), expRight: e2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_PDE { expLeft: e1, expRight: e2, domain, comment, info } => {
            let mut e3: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (e2, arg) = traverser(e2.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_PDE { expLeft: e1.clone(), expRight: e2.clone(), domain: domain.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_CONNECT { crefLeft: cr1, crefRight: cr2, comment, info } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut cr1 = (*cr1).clone();
            let mut cr2 = (*cr2).clone();
            (cr1, arg) = mapFoldComponentRefExps(cr1.clone(), traverser.clone(), arg.clone())?;
            (cr2, arg) = mapFoldComponentRefExps(cr2.clone(), traverser.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_CONNECT { crefLeft: cr1.clone(), crefRight: cr2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_FOR { index, range: Some(e1), eEquationLst: eql, comment, info } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_FOR { index: (index.clone()).clone(), range: Some(e1.clone()), eEquationLst: eql.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_WHEN { condition: e1, eEquationLst: eql, elseBranches: else_when, comment, info } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut e1 = (*e1).clone();
            let mut else_when = (*else_when).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (else_when, arg) = List::map1Fold(else_when.clone(), Arc::new(fnptr!(mapFoldElseWhenExps, (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), _, _)), traverser.clone(), arg.clone());
            (Arc::new(SCode::Equation::EQ_WHEN { condition: e1.clone(), eEquationLst: eql.clone(), elseBranches: else_when.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_ASSERT { condition: e1, message: e2, level: e3, comment, info } => {
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut e3 = (*e3).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (e2, arg) = traverser(e2.clone(), arg.clone())?;
            (e3, arg) = traverser(e3.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_TERMINATE { message: e1, comment, info } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_TERMINATE { message: e1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_REINIT { cref: e1, expReinit: e2, comment, info } => {
            let mut e3: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (e2, arg) = traverser(e2.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_REINIT { cref: e1.clone(), expReinit: e2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_NORETCALL { exp: e1, comment, info } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut domain: Arc<Absyn::ComponentRef>;
            let mut index: ArcStr;
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone())?;
            (Arc::new(SCode::Equation::EQ_NORETCALL { exp: e1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        _ => (eq.clone(), arg.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, arg))
}

pub fn mapFoldEquationListExps<ArgT: Clone + 'static, Argument: Clone + 'static>(inEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>, traverser: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, inArg: Argument) -> (Arc<metamodelica::List<Arc<SCode::Equation>>>, Argument) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)>;

    let mut outEquations: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut outArg: Argument;
    (outEquations, outArg) = List::map1Fold(inEquations.clone(), Arc::new(mapFoldEquationExps), traverser.clone(), inArg.clone());
    (outEquations, outArg)
}

pub fn mapFoldEquations<ArgT: Clone + 'static>(eq: Arc<SCode::Equation>, traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>, arg: ArgT) -> (Arc<SCode::Equation>, ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)>;

    let mut eq: Arc<SCode::Equation> = eq;
    let mut arg: ArgT = arg;
    (eq, arg) = traverser(eq.clone(), arg.clone()).unwrap();
    (eq, arg) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::EQ_IF { condition: expl1, thenBranch: then_branch, elseBranch: else_branch, comment, info } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut then_branch = (*then_branch).clone();
            let mut else_branch = (*else_branch).clone();
            (then_branch, arg) = List::mapFold(then_branch.clone(), Arc::new({ let __pe_b1 = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldEquationsList(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            (else_branch, arg) = mapFoldEquationsList(else_branch.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Equation::EQ_IF { condition: expl1.clone(), thenBranch: then_branch.clone(), elseBranch: else_branch.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::EQ_FOR { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut else_when: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut comment: Arc<SCode::Comment>;
            let mut info: SourceInfo;
            (eql, arg) = mapFoldEquationsList(var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone(), traverser.clone(), arg.clone());
            assign_variant_field!(eq => SCode::Equation::EQ_FOR; eEquationLst = eql.clone());
            (eq.clone(), arg.clone())
        },
        Deref @ SCode::EQ_WHEN { condition: e1, eEquationLst: eql, elseBranches: else_when, comment, info } => {
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut then_branch: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql = (*eql).clone();
            let mut else_when = (*else_when).clone();
            (eql, arg) = mapFoldEquationsList(eql.clone(), traverser.clone(), arg.clone());
            (else_when, arg) = List::mapFold(else_when.clone(), Arc::new({ let __pe_b1 = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldElseWhenEquations(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            (Arc::new(SCode::Equation::EQ_WHEN { condition: e1.clone(), eEquationLst: eql.clone(), elseBranches: else_when.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        _ => (eq.clone(), arg.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (eq, arg)
}

pub fn mapFoldEquationsList<ArgT: Clone + 'static>(eql: Arc<metamodelica::List<Arc<SCode::Equation>>>, traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)> + 'static>, arg: ArgT) -> (Arc<metamodelica::List<Arc<SCode::Equation>>>, ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<SCode::Equation>, ArgT) -> Result<(Arc<SCode::Equation>, ArgT)>;

    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = eql;
    let mut arg: ArgT = arg;
    (eql, arg) = List::mapFold(eql.clone(), Arc::new({ let __pe_b1 = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldEquations(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
    (eql, arg)
}

fn mapFoldForIteratorExps<ArgT: Clone + 'static>(inIterator: Arc<Absyn::ForIterator>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, inArg: ArgT) -> Result<(Arc<Absyn::ForIterator>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)>;

    let mut outIterator: Arc<Absyn::ForIterator>;
    let mut outArg: ArgT;
    (outIterator, outArg) = (::match_deref::match_deref! { match &((inIterator.clone(), inFunc.clone(), inArg.clone())) {
        (Deref @ Absyn::ITERATOR { name: ident, guardExp: None, range: None }, _, arg) => (Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: None, range: None }), arg.clone()),
        (Deref @ Absyn::ITERATOR { name: ident, guardExp: None, range: Some(range) }, traverser, arg) => {
            let mut guardExp: Arc<Absyn::Exp>;
            let mut range = (*range).clone();
            let mut arg = (*arg).clone();
            (range, arg) = traverser(range.clone(), arg.clone())?;
            (Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: None, range: Some(range.clone()) }), arg.clone())
        },
        (Deref @ Absyn::ITERATOR { name: ident, guardExp: Some(guardExp), range: Some(range) }, traverser, arg) => {
            let mut guardExp = (*guardExp).clone();
            let mut range = (*range).clone();
            let mut arg = (*arg).clone();
            (guardExp, arg) = traverser(guardExp.clone(), arg.clone())?;
            (range, arg) = traverser(range.clone(), arg.clone())?;
            (Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: Some(guardExp.clone()), range: Some(range.clone()) }), arg.clone())
        },
        (Deref @ Absyn::ITERATOR { name: ident, guardExp: Some(guardExp), range: None }, traverser, arg) => {
            let mut range: Arc<Absyn::Exp>;
            let mut guardExp = (*guardExp).clone();
            let mut arg = (*arg).clone();
            (guardExp, arg) = traverser(guardExp.clone(), arg.clone())?;
            (Arc::new(Absyn::ForIterator { name: (ident.clone()).clone(), guardExp: Some(guardExp.clone()), range: None }), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outIterator, outArg))
}

pub fn mapFoldStatementExps<ArgT: Clone + 'static, Argument: Clone + 'static>(inStatement: Arc<SCode::Statement>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, inArg: ArgT) -> (Arc<SCode::Statement>, ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)>;

    let mut outStatement: Arc<SCode::Statement>;
    let mut outArg: ArgT;
    (outStatement, outArg) = (::match_deref::match_deref! { match &((inStatement.clone(), inFunc.clone(), inArg.clone())) {
        (Deref @ SCode::ALG_ASSIGN { assignComponent: e1, value: e2, comment, info }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut iterator: ArcStr;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (e2, arg) = traverser(e2.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_ASSIGN { assignComponent: e1.clone(), value: e2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_IF { boolExpr: e1, trueBranch: stmts1, elseIfBranch: branches, elseBranch: stmts2, comment, info }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut iterator: ArcStr;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut e1 = (*e1).clone();
            let mut branches = (*branches).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (branches, arg) = List::map1Fold(branches.clone(), Arc::new(fnptr!(mapFoldBranchExps, (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), _, _)), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_IF { boolExpr: e1.clone(), trueBranch: stmts1.clone(), elseIfBranch: branches.clone(), elseBranch: stmts2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_FOR { index: iterator, range: Some(e1), forBody: stmts1, comment, info }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut e1 = (*e1).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_FOR { index: (iterator.clone()).clone(), range: Some(e1.clone()), forBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_PARFOR { index: iterator, range: Some(e1), parforBody: stmts1, comment, info }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut e1 = (*e1).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_PARFOR { index: (iterator.clone()).clone(), range: Some(e1.clone()), parforBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_WHILE { boolExpr: e1, whileBody: stmts1, comment, info }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut iterator: ArcStr;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut e1 = (*e1).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_WHILE { boolExpr: e1.clone(), whileBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_WHEN_A { branches, comment, info }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut iterator: ArcStr;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut branches = (*branches).clone();
            let mut arg = (*arg).clone();
            (branches, arg) = List::map1Fold(branches.clone(), Arc::new(fnptr!(mapFoldBranchExps, (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), _, _)), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_WHEN_A { branches: branches.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_ASSERT { .. }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut iterator: ArcStr;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut comment: Arc<SCode::Comment>;
            let mut info: SourceInfo;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(var_field!((*inStatement).condition, SCode::Statement::ALG_ASSERT).clone(), arg.clone()).unwrap();
            (e2, arg) = traverser(var_field!((*inStatement).message, SCode::Statement::ALG_ASSERT).clone(), arg.clone()).unwrap();
            (e3, arg) = traverser(var_field!((*inStatement).level, SCode::Statement::ALG_ASSERT).clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: var_field!((*inStatement).comment, SCode::Statement::ALG_ASSERT).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_ASSERT).clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_TERMINATE { .. }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut iterator: ArcStr;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut comment: Arc<SCode::Comment>;
            let mut info: SourceInfo;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(var_field!((*inStatement).message, SCode::Statement::ALG_TERMINATE).clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_TERMINATE { message: e1.clone(), comment: var_field!((*inStatement).comment, SCode::Statement::ALG_TERMINATE).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_TERMINATE).clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_REINIT { .. }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut iterator: ArcStr;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut comment: Arc<SCode::Comment>;
            let mut info: SourceInfo;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(var_field!((*inStatement).cref, SCode::Statement::ALG_REINIT).clone(), arg.clone()).unwrap();
            (e2, arg) = traverser(var_field!((*inStatement).newValue, SCode::Statement::ALG_REINIT).clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_REINIT { cref: e1.clone(), newValue: e2.clone(), comment: var_field!((*inStatement).comment, SCode::Statement::ALG_REINIT).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_REINIT).clone() }), arg.clone())
        },
        (Deref @ SCode::ALG_NORETCALL { exp: e1, comment, info }, traverser, arg) => {
            let mut tup: (Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, Argument);
            let mut iterator: ArcStr;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut stmts1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut e1 = (*e1).clone();
            let mut arg = (*arg).clone();
            (e1, arg) = traverser(e1.clone(), arg.clone()).unwrap();
            (Arc::new(SCode::Statement::ALG_NORETCALL { exp: e1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        _ => (inStatement.clone(), inArg.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outStatement, outArg)
}

pub fn mapFoldStatementListExps<ArgT: Clone + 'static, Argument: Clone + 'static>(inStatements: Arc<metamodelica::List<Arc<SCode::Statement>>>, inFunc: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>, inArg: Argument) -> (Arc<metamodelica::List<Arc<SCode::Statement>>>, Argument) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)>;

    let mut outStatements: Arc<metamodelica::List<Arc<SCode::Statement>>>;
    let mut outArg: Argument;
    (outStatements, outArg) = List::map1Fold(inStatements.clone(), Arc::new(fnptr!(mapFoldStatementExps, Arc<SCode::Statement>, _, _)), inFunc.clone(), inArg.clone());
    (outStatements, outArg)
}

pub fn mapFoldStatements<ArgT: Clone + 'static>(stmt: Arc<SCode::Statement>, traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>, arg: ArgT) -> (Arc<SCode::Statement>, ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)>;

    let mut stmt: Arc<SCode::Statement> = stmt;
    let mut arg: ArgT = arg;
    (stmt, arg) = traverser(stmt.clone(), arg.clone()).unwrap();
    (stmt, arg) = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::ALG_IF { boolExpr: e, trueBranch: stmts1, elseIfBranch: branches, elseBranch: stmts2, comment, info } => {
            let mut iter: ArcStr;
            let mut range: Option<Arc<Absyn::Exp>>;
            let mut stmts1 = (*stmts1).clone();
            let mut branches = (*branches).clone();
            let mut stmts2 = (*stmts2).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (branches, arg) = List::mapFold(branches.clone(), Arc::new({ let __pe_b1 = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldBranchStatements(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            (stmts2, arg) = mapFoldStatementsList(stmts2.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_IF { boolExpr: e.clone(), trueBranch: stmts1.clone(), elseIfBranch: branches.clone(), elseBranch: stmts2.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::ALG_FOR { index: iter, range, forBody: stmts1, comment, info } => {
            let mut e: Arc<Absyn::Exp>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_FOR { index: (iter.clone()).clone(), range: range.clone(), forBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::ALG_PARFOR { index: iter, range, parforBody: stmts1, comment, info } => {
            let mut e: Arc<Absyn::Exp>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_PARFOR { index: (iter.clone()).clone(), range: range.clone(), parforBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::ALG_WHILE { boolExpr: e, whileBody: stmts1, comment, info } => {
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter: ArcStr;
            let mut range: Option<Arc<Absyn::Exp>>;
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_WHILE { boolExpr: e.clone(), whileBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::ALG_WHEN_A { branches, comment, info } => {
            let mut e: Arc<Absyn::Exp>;
            let mut stmts1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut iter: ArcStr;
            let mut range: Option<Arc<Absyn::Exp>>;
            let mut branches = (*branches).clone();
            (branches, arg) = List::mapFold(branches.clone(), Arc::new({ let __pe_b1 = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldBranchStatements(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
            (Arc::new(SCode::Statement::ALG_WHEN_A { branches: branches.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ SCode::ALG_FAILURE { stmts: stmts1, comment, info } => {
            let mut e: Arc<Absyn::Exp>;
            let mut stmts2: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter: ArcStr;
            let mut range: Option<Arc<Absyn::Exp>>;
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg.clone());
            (Arc::new(SCode::Statement::ALG_FAILURE { stmts: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg.clone())
        },
        _ => (stmt.clone(), arg.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (stmt, arg)
}

pub fn mapFoldStatementsList<ArgT: Clone + 'static>(statements: Arc<metamodelica::List<Arc<SCode::Statement>>>, traverser: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)> + 'static>, arg: ArgT) -> (Arc<metamodelica::List<Arc<SCode::Statement>>>, ArgT) {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<SCode::Statement>, ArgT) -> Result<(Arc<SCode::Statement>, ArgT)>;

    let mut statements: Arc<metamodelica::List<Arc<SCode::Statement>>> = statements;
    let mut arg: ArgT = arg;
    (statements, arg) = List::mapFold(statements.clone(), Arc::new({ let __pe_b1 = traverser.clone(); move |__pe_a0, __pe_a2| Ok(mapFoldStatements(__pe_a0, __pe_b1.clone(), __pe_a2)) }), arg.clone());
    (statements, arg)
}

fn mapFoldSubscriptExps<ArgT: Clone + 'static>(inSubscript: Arc<Absyn::Subscript>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)> + 'static>, inArg: ArgT) -> Result<(Arc<Absyn::Subscript>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<(Arc<Absyn::Exp>, ArgT)>;

    let mut outSubscript: Arc<Absyn::Subscript>;
    let mut outArg: ArgT;
    (outSubscript, outArg) = (::match_deref::match_deref! { match &((inSubscript.clone(), inFunc.clone(), inArg.clone())) {
        (Deref @ Absyn::SUBSCRIPT { subscript: sub_exp }, traverser, arg) => {
            let mut sub_exp = (*sub_exp).clone();
            let mut arg = (*arg).clone();
            (sub_exp, arg) = traverser(sub_exp.clone(), arg.clone())?;
            (Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: sub_exp.clone() }), arg.clone())
        },
        (Deref @ Absyn::NOSUB, _, _) => (inSubscript.clone(), inArg.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSubscript, outArg))
}

pub fn mapStatementExps(stmt: Arc<SCode::Statement>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<Arc<SCode::Statement>> {
    pub type Func = fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>>;

    let mut stmt: Arc<SCode::Statement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSIGN;
                assignComponent = func(var_field!((*stmt).assignComponent, SCode::Statement::ALG_ASSIGN).clone())?,
                value = func(var_field!((*stmt).value, SCode::Statement::ALG_ASSIGN).clone())?
            );
            ()
        },
        Deref @ SCode::Statement::ALG_IF { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                boolExpr = func(var_field!((*stmt).boolExpr, SCode::Statement::ALG_IF).clone())?,
                elseIfBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(b.clone()))?, Util::tuple22(b.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            if isSome(var_field!((*stmt).range, SCode::Statement::ALG_FOR).clone()) {
                assign_variant_field!(stmt => SCode::Statement::ALG_FOR; range = Some(func(Util::getOption(var_field!((*stmt).range, SCode::Statement::ALG_FOR).clone())?)?));
            }
            ()
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            if isSome(var_field!((*stmt).range, SCode::Statement::ALG_PARFOR).clone()) {
                assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR; range = Some(func(Util::getOption(var_field!((*stmt).range, SCode::Statement::ALG_PARFOR).clone())?)?));
            }
            ()
        },
        Deref @ SCode::Statement::ALG_WHILE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE; boolExpr = func(var_field!((*stmt).boolExpr, SCode::Statement::ALG_WHILE).clone())?);
            ()
        },
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for b in (var_field!((*stmt).branches, SCode::Statement::ALG_WHEN_A).clone()).into_iter().cloned() {
            let __x = (func(Util::tuple21(b.clone()))?, Util::tuple22(b.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSERT;
                condition = func(var_field!((*stmt).condition, SCode::Statement::ALG_ASSERT).clone())?,
                message = func(var_field!((*stmt).message, SCode::Statement::ALG_ASSERT).clone())?,
                level = func(var_field!((*stmt).level, SCode::Statement::ALG_ASSERT).clone())?
            );
            ()
        },
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TERMINATE; message = func(var_field!((*stmt).message, SCode::Statement::ALG_TERMINATE).clone())?);
            ()
        },
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_REINIT;
                cref = func(var_field!((*stmt).cref, SCode::Statement::ALG_REINIT).clone())?,
                newValue = func(var_field!((*stmt).newValue, SCode::Statement::ALG_REINIT).clone())?
            );
            ()
        },
        Deref @ SCode::Statement::ALG_NORETCALL { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_NORETCALL; exp = func(var_field!((*stmt).exp, SCode::Statement::ALG_NORETCALL).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

pub fn mapStatements(stmt: Arc<SCode::Statement>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>) -> Arc<SCode::Statement> {
    pub type Func = fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>>;

    let mut stmt: Arc<SCode::Statement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_IF { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                trueBranch = mapStatementsList(var_field!((*stmt).trueBranch, SCode::Statement::ALG_IF).clone(), func.clone()),
                elseIfBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(b.clone()), mapStatementsList(Util::tuple22(b.clone()), func.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = mapStatementsList(var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone(), func.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FOR; forBody = mapStatementsList(var_field!((*stmt).forBody, SCode::Statement::ALG_FOR).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR; parforBody = mapStatementsList(var_field!((*stmt).parforBody, SCode::Statement::ALG_PARFOR).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_WHILE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE; whileBody = mapStatementsList(var_field!((*stmt).whileBody, SCode::Statement::ALG_WHILE).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A; branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for b in (var_field!((*stmt).branches, SCode::Statement::ALG_WHEN_A).clone()).into_iter().cloned() {
            let __x = (Util::tuple21(b.clone()), mapStatementsList(Util::tuple22(b.clone()), func.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::Statement::ALG_FAILURE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FAILURE; stmts = mapStatementsList(var_field!((*stmt).stmts, SCode::Statement::ALG_FAILURE).clone(), func.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_TRY { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TRY;
                body = mapStatementsList(var_field!((*stmt).body, SCode::Statement::ALG_TRY).clone(), func.clone()),
                elseBody = mapStatementsList(var_field!((*stmt).body, SCode::Statement::ALG_TRY).clone(), func.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stmt = func(stmt.clone()).unwrap();
    stmt
}

pub fn mapStatementsList(statements: Arc<metamodelica::List<Arc<SCode::Statement>>>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>) -> Arc<metamodelica::List<Arc<SCode::Statement>>> {
    pub type Func = fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>>;

    let mut statements: Arc<metamodelica::List<Arc<SCode::Statement>>> = statements;
    statements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (statements.clone()).into_iter().cloned() {
            let __x = mapStatements(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    statements
}

pub fn mergeAttributes(ele: SCode::Attributes, oEle: Option<SCode::Attributes>) -> Result<Option<SCode::Attributes>> {
    let mut outoEle: Option<SCode::Attributes>;
    outoEle = (match (ele.clone(), oEle.clone()) {
        (_, None) => Some(ele.clone()),
        (SCode::ATTR { arrayDims: ref ad1, connectorType: mut ct1, parallelism: mut p1, variability: mut v1, direction: mut d1, isField: mut isf1 }, Some(SCode::ATTR { arrayDims: _, connectorType: mut ct2, parallelism: mut p2, variability: mut v2, direction: mut d2, isField: mut isf2 })) => {
            let mut p: SCode::Parallelism;
            let mut v: SCode::Variability;
            let mut d: Absyn::Direction;
            let mut isf: Absyn::IsField;
            let mut ad2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut ct: SCode::ConnectorType;
            ct = propagateConnectorType(ct1.clone(), ct2.clone());
            p = propagateParallelism(p1.clone(), p2.clone());
            v = propagateVariability(v1.clone(), v2.clone());
            d = propagateDirection(d1.clone(), d2.clone());
            isf = propagateIsField(isf1.clone(), isf2.clone());
            ad = ad1.clone();
            Some(SCode::Attributes { arrayDims: ad.clone(), connectorType: ct.clone(), parallelism: p.clone(), variability: v.clone(), direction: d.clone(), isField: isf.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outoEle)
}

pub fn mergeAttributesFromClass(inAttributes: SCode::Attributes, inClass: Arc<SCode::Element>) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes;
    outAttributes = (::match_deref::match_deref! { match &((inAttributes.clone(), inClass.clone())) {
        (_, Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { attributes: cls_attr, .. }, .. }) => {
            let mut attr: SCode::Attributes;
            let Some(__pa0) = (mergeAttributes(inAttributes.clone(), Some(cls_attr.clone()))?) else { bail!("pattern mismatch") };
            attr = __pa0.clone();
            attr.clone()
        },
        _ => inAttributes.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAttributes)
}

pub fn mergeClassDef(inNew: Arc<SCode::ClassDef>, inOld: Arc<SCode::ClassDef>, inCCModNew: Arc<SCode::Mod>, inCCModOld: Arc<SCode::Mod>) -> Result<Arc<SCode::ClassDef>> {
    let mut outNew: Arc<SCode::ClassDef>;
    outNew = (::match_deref::match_deref! { match &((inNew.clone(), inOld.clone(), inCCModNew.clone(), inCCModOld.clone())) {
        (Deref @ SCode::DERIVED { typeSpec: ts1, modifications: m1, attributes: a1 }, Deref @ SCode::DERIVED { typeSpec: _, modifications: m2, attributes: a2 }, _, _) => {
            let mut n: Arc<SCode::ClassDef>;
            let mut o: Arc<SCode::ClassDef>;
            let mut ts2: Arc<Absyn::TypeSpec>;
            let mut m1 = (*m1).clone();
            let mut m2 = (*m2).clone();
            let mut a2 = (*a2).clone();
            m2 = mergeModifiers(m2.clone(), inCCModOld.clone())?;
            m1 = mergeModifiers(m1.clone(), inCCModNew.clone())?;
            m2 = mergeModifiers(m1.clone(), m2.clone())?;
            a2 = propagateAttributes(a2.clone(), a1.clone(), false)?;
            n = Arc::new(SCode::ClassDef::DERIVED { typeSpec: ts1.clone(), modifications: m2.clone(), attributes: a2.clone() });
            n.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNew)
}

pub fn mergeComponentModifiers(newComp: Arc<SCode::Element>, oldComp: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut newComp: Arc<SCode::Element> = newComp;
    let () = (::match_deref::match_deref! { match &((newComp.clone(), oldComp.clone())) {
        (Deref @ SCode::COMPONENT { .. }, Deref @ SCode::COMPONENT { .. }) => {
            assign_variant_field!(newComp => SCode::Element::COMPONENT; modifications = mergeModifiers(var_field!((*newComp).modifications, SCode::Element::COMPONENT).clone(), var_field!((*oldComp).modifications, SCode::Element::COMPONENT).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newComp)
}

pub fn mergeModifiers(inNewMod: Arc<SCode::Mod>, inOldMod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = 'mc: {
        let __mc_input = (inNewMod.clone(), inOldMod.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::NOMOD) => {
                    let mut f1: SCode::Final;
                    let mut f2: SCode::Final;
                    let mut e1: SCode::Each;
                    let mut e2: SCode::Each;
                    let mut sl1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut sl2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut b1: Option<Arc<Absyn::Exp>>;
                    let mut b2: Option<Arc<Absyn::Exp>>;
                    let mut b: Option<Arc<Absyn::Exp>>;
                    let mut i1: SourceInfo;
                    let mut i2: SourceInfo;
                    let mut m: Arc<SCode::Mod>;
                    let mut cmt: Option<ArcStr>;
                    Ok(inNewMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::NOMOD, _) => {
                    let mut f1: SCode::Final;
                    let mut f2: SCode::Final;
                    let mut e1: SCode::Each;
                    let mut e2: SCode::Each;
                    let mut sl1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut sl2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut b1: Option<Arc<Absyn::Exp>>;
                    let mut b2: Option<Arc<Absyn::Exp>>;
                    let mut b: Option<Arc<Absyn::Exp>>;
                    let mut i1: SourceInfo;
                    let mut i2: SourceInfo;
                    let mut m: Arc<SCode::Mod>;
                    let mut cmt: Option<ArcStr>;
                    Ok(inOldMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::REDECL { .. }, _) => {
                    let mut f1: SCode::Final;
                    let mut f2: SCode::Final;
                    let mut e1: SCode::Each;
                    let mut e2: SCode::Each;
                    let mut sl1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut sl2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut b1: Option<Arc<Absyn::Exp>>;
                    let mut b2: Option<Arc<Absyn::Exp>>;
                    let mut b: Option<Arc<Absyn::Exp>>;
                    let mut i1: SourceInfo;
                    let mut i2: SourceInfo;
                    let mut m: Arc<SCode::Mod>;
                    let mut cmt: Option<ArcStr>;
                    Ok(inNewMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::MOD { finalPrefix: f1, eachPrefix: e1, subModLst: sl1, binding: b1, comment: cmt, info: i1 }, Deref @ SCode::MOD { finalPrefix: f2, eachPrefix: e2, subModLst: sl2, binding: b2, comment: _, .. }) => {
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut b: Option<Arc<Absyn::Exp>>;
                    let mut i2: SourceInfo;
                    let mut m: Arc<SCode::Mod>;
                    b = if (isSome(b1.clone())) {b1.clone()} else {b2.clone()};
                    sl = mergeSubMods(sl1.clone(), sl2.clone())?;
                    if referenceEq(&b.clone(),&b1.clone()) && referenceEq(&sl.clone(),&sl1.clone()) {
                        m = inNewMod.clone();
                    } else if referenceEq(&b.clone(),&b2.clone()) && referenceEq(&sl.clone(),&sl2.clone()) && f1.clone() == f2.clone() && e1.clone() == e2.clone() {
                        m = inOldMod.clone();
                    } else {
                        m = Arc::new(SCode::Mod::MOD { finalPrefix: f1.clone(), eachPrefix: e1.clone(), subModLst: sl.clone(), binding: b.clone(), comment: cmt.clone(), info: i1.clone() });
                    }
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut f1: SCode::Final;
                    let mut f2: SCode::Final;
                    let mut e1: SCode::Each;
                    let mut e2: SCode::Each;
                    let mut sl1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut sl2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut b1: Option<Arc<Absyn::Exp>>;
                    let mut b2: Option<Arc<Absyn::Exp>>;
                    let mut b: Option<Arc<Absyn::Exp>>;
                    let mut i1: SourceInfo;
                    let mut i2: SourceInfo;
                    let mut m: Arc<SCode::Mod>;
                    let mut cmt: Option<ArcStr>;
                    Ok(inNewMod.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMod)
}

pub fn mergeSCodeMods(inModOuter: Arc<SCode::Mod>, inModInner: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &((inModOuter.clone(), inModInner.clone())) {
        (Deref @ SCode::NOMOD, _) => inModInner.clone(),
        (_, Deref @ SCode::NOMOD) => inModOuter.clone(),
        (Deref @ SCode::MOD { .. }, Deref @ SCode::MOD { .. }) => {
            let mut subMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
            let mut binding: Option<Arc<Absyn::Exp>>;
            subMods = listAppend(var_field!((*inModOuter).subModLst, SCode::Mod::MOD).clone(), var_field!((*inModInner).subModLst, SCode::Mod::MOD).clone());
            binding = if (isSome(var_field!((*inModOuter).binding, SCode::Mod::MOD).clone())) {var_field!((*inModOuter).binding, SCode::Mod::MOD).clone()} else {var_field!((*inModInner).binding, SCode::Mod::MOD).clone()};
            Arc::new(SCode::Mod::MOD { finalPrefix: var_field!((*inModOuter).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*inModOuter).eachPrefix, SCode::Mod::MOD).clone(), subModLst: subMods.clone(), binding: binding.clone(), comment: var_field!((*inModOuter).comment, SCode::Mod::MOD).clone(), info: var_field!((*inModOuter).info, SCode::Mod::MOD).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

pub fn mergeSCodeOptAnn(inModOuter: Option<Arc<SCode::Annotation>>, inModInner: Option<Arc<SCode::Annotation>>) -> Result<Option<Arc<SCode::Annotation>>> {
    let mut outMod: Option<Arc<SCode::Annotation>>;
    outMod = (match (inModOuter.clone(), inModInner.clone()) {
        (None, _) => inModInner.clone(),
        (_, None) => inModOuter.clone(),
        (Some(SCode::ANNOTATION { modification: ref mod1 }), Some(SCode::ANNOTATION { modification: ref mod2 })) => {
            let mut r#mod: Arc<SCode::Mod>;
            r#mod = mergeSCodeMods(mod1.clone(), mod2.clone())?;
            Some(Arc::new(SCode::Annotation { modification: r#mod.clone() }))
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outMod)
}

fn mergeSubMods(inNew: Arc<metamodelica::List<Arc<SCode::SubMod>>>, inOld: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    outSubs = 'mc: {
        let __mc_input = (inNew.clone(), inOld.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut rest: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut old: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut s: Arc<SCode::SubMod>;
                    Ok(inOld.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: s, tail: rest }, _) => {
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut old: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    old = removeSub(s.clone(), inOld.clone())?;
                    sl = mergeSubMods(rest.clone(), old.clone())?;
                    Ok(cons(s.clone(), sl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sl: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut rest: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut old: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut s: Arc<SCode::SubMod>;
                    Ok(inNew.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSubs)
}

pub fn mergeWithOriginal(newClass: Arc<SCode::Element>, oldClass: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut newClass: Arc<SCode::Element> = newClass;
    let () = 'mc: {
        let __mc_input = (newClass.clone(), oldClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut prefixes1: Arc<SCode::Prefixes>;
                    let mut prefixes2: Arc<SCode::Prefixes>;
                    let mut cd1: Arc<SCode::ClassDef>;
                    let mut cd2: Arc<SCode::ClassDef>;
                    let mut mCCNew: Arc<SCode::Mod>;
                    let mut mCCOld: Arc<SCode::Mod>;
                    let true = (isFunction(newClass.clone())) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::CLASS { classDef: cd1, prefixes: prefixes1, .. }, Deref @ SCode::CLASS { classDef: cd2, prefixes: prefixes2, .. }) => {
                    let mut mCCNew: Arc<SCode::Mod>;
                    let mut mCCOld: Arc<SCode::Mod>;
                    let mut newClass: Arc<SCode::Element> = newClass.clone();
                    mCCNew = getConstrainedByModifiers(prefixes1.clone());
                    mCCOld = getConstrainedByModifiers(prefixes2.clone());
                    assign_variant_field!(newClass => SCode::Element::CLASS;
                        classDef = mergeClassDef(cd1.clone(), cd2.clone(), mCCNew.clone(), mCCOld.clone())?,
                        prefixes = propagatePrefixes(prefixes1.clone(), prefixes2.clone())?
                    );
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut prefixes1: Arc<SCode::Prefixes>;
                    let mut prefixes2: Arc<SCode::Prefixes>;
                    let mut cd1: Arc<SCode::ClassDef>;
                    let mut cd2: Arc<SCode::ClassDef>;
                    let mut mCCNew: Arc<SCode::Mod>;
                    let mut mCCOld: Arc<SCode::Mod>;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(newClass)
}

pub fn modEqual(mod1: Arc<SCode::Mod>, mod2: Arc<SCode::Mod>) -> Result<bool> {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (mod1.clone(), mod2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::MOD { finalPrefix: f1, eachPrefix: each1, subModLst: submodlst1, binding: Some(e1), comment: _, .. }, Deref @ SCode::MOD { finalPrefix: f2, eachPrefix: each2, subModLst: submodlst2, binding: Some(e2), comment: _, .. }) => {
                    let mut elt1: Arc<SCode::Element>;
                    let mut elt2: Arc<SCode::Element>;
                    let true = (f1.clone() == f2.clone()) else { bail!("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(submodlst1.clone(), submodlst2.clone())?) else { bail!("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::MOD { finalPrefix: f1, eachPrefix: each1, subModLst: submodlst1, binding: None, comment: _, .. }, Deref @ SCode::MOD { finalPrefix: f2, eachPrefix: each2, subModLst: submodlst2, binding: None, comment: _, .. }) => {
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut elt1: Arc<SCode::Element>;
                    let mut elt2: Arc<SCode::Element>;
                    let true = (f1.clone() == f2.clone()) else { bail!("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(submodlst1.clone(), submodlst2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::NOMOD, Deref @ SCode::NOMOD) => {
                    let mut f1: SCode::Final;
                    let mut f2: SCode::Final;
                    let mut each1: SCode::Each;
                    let mut each2: SCode::Each;
                    let mut submodlst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut submodlst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut elt1: Arc<SCode::Element>;
                    let mut elt2: Arc<SCode::Element>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::REDECL { finalPrefix: f1, eachPrefix: each1, element: elt1 }, Deref @ SCode::REDECL { finalPrefix: f2, eachPrefix: each2, element: elt2 }) => {
                    let mut submodlst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut submodlst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let true = (f1.clone() == f2.clone()) else { bail!("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { bail!("pattern mismatch") };
                    let true = (elementEqual(elt1.clone(), elt2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::BREAK_COMPONENT { .. }, Deref @ SCode::BREAK_COMPONENT { .. }) => {
                    let mut f1: SCode::Final;
                    let mut f2: SCode::Final;
                    let mut each1: SCode::Each;
                    let mut each2: SCode::Each;
                    let mut submodlst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut submodlst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut elt1: Arc<SCode::Element>;
                    let mut elt2: Arc<SCode::Element>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::BREAK_CONNECT { .. }, Deref @ SCode::BREAK_CONNECT { .. }) => {
                    let mut f1: SCode::Final;
                    let mut f2: SCode::Final;
                    let mut each1: SCode::Each;
                    let mut each2: SCode::Each;
                    let mut submodlst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut submodlst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut elt1: Arc<SCode::Element>;
                    let mut elt2: Arc<SCode::Element>;
                    Ok(AbsynUtil::crefEqual(var_field!((*mod1).lhs, SCode::Mod::BREAK_CONNECT).clone(), var_field!((*mod2).lhs, SCode::Mod::BREAK_CONNECT).clone()) && AbsynUtil::crefEqual(var_field!((*mod1).rhs, SCode::Mod::BREAK_CONNECT).clone(), var_field!((*mod2).lhs, SCode::Mod::BREAK_CONNECT).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut f1: SCode::Final;
                    let mut f2: SCode::Final;
                    let mut each1: SCode::Each;
                    let mut each2: SCode::Each;
                    let mut submodlst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut submodlst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut elt1: Arc<SCode::Element>;
                    let mut elt2: Arc<SCode::Element>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

pub fn onlyLiteralsInMod(r#mod: Arc<SCode::Mod>) -> Result<bool> {
    let mut onlyLiterals: bool;
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    onlyLiterals = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            if isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
                onlyLiterals = AbsynUtil::onlyLiteralsInExp(Util::getOption(var_field!((*r#mod).binding, SCode::Mod::MOD).clone())?)?;
            } else {
                onlyLiterals = true;
            }
            if onlyLiterals.clone() {
                for m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                    onlyLiterals = onlyLiteralsInMod(m.r#mod.clone())?;
                    if !(onlyLiterals.clone()) {
                        break;
                    }
                }
            }
            onlyLiterals.clone()
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(onlyLiterals)
}

pub fn optCommentAnnotation(cmt: Option<Arc<SCode::Comment>>) -> Option<Arc<SCode::Annotation>> {
    let mut ann: Option<Arc<SCode::Annotation>>;
    ann = (match cmt.clone() {
        Some(SCode::COMMENT { annotation_: mut ann, .. }) => ann.clone(),
        _ => None,
    });
    ann
}

pub fn optCommentHasBooleanNamedAnnotation(comm: Option<Arc<SCode::Comment>>, annotationName: ArcStr) -> Result<bool> {
    let mut outB: bool;
    outB = (match (comm.clone(), annotationName.clone()) {
        (Some(SCode::COMMENT { annotation_: Some(mut ann), .. }), _) => hasBooleanNamedAnnotation(ann.clone(), (annotationName.clone()).clone())?,
        _ => false,
    });
    Ok(outB)
}

pub fn optCommentHasBooleanNamedAnnotationFalse(comm: Option<Arc<SCode::Comment>>, annotationName: ArcStr) -> Result<bool> {
    let mut outB: bool;
    outB = (match (comm.clone(), annotationName.clone()) {
        (Some(SCode::COMMENT { annotation_: Some(mut ann), .. }), _) => hasBooleanNamedAnnotationFalse(ann.clone(), (annotationName.clone()).clone())?,
        _ => false,
    });
    Ok(outB)
}

pub fn parallelismEqual(prl1: SCode::Parallelism, prl2: SCode::Parallelism) -> bool {
    let mut equal: bool;
    equal = (match (prl1.clone(), prl2.clone()) {
        (SCode::PARGLOBAL, SCode::PARGLOBAL) => true,
        (SCode::PARLOCAL, SCode::PARLOCAL) => true,
        (SCode::NON_PARALLEL, SCode::NON_PARALLEL) => true,
        _ => false,
    });
    equal
}

pub fn partialBool(inPartial: SCode::Partial) -> Result<bool> {
    let mut bPartial: bool;
    bPartial = (match inPartial.clone() {
        SCode::PARTIAL => true,
        SCode::NOT_PARTIAL => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bPartial)
}

pub fn potentialBool(inConnectorType: SCode::ConnectorType) -> bool {
    let mut outPotential: bool;
    outPotential = (match inConnectorType.clone() {
        SCode::POTENTIAL => true,
        _ => false,
    });
    outPotential
}

pub fn prefixesEqual(prefixes1: Arc<SCode::Prefixes>, prefixes2: Arc<SCode::Prefixes>) -> Result<bool> {
    let mut equal: bool;
    equal = prefixes1.visibility.clone() == prefixes2.visibility.clone() && prefixes1.redeclarePrefix.clone() == prefixes2.redeclarePrefix.clone() && prefixes1.finalPrefix.clone() == prefixes2.finalPrefix.clone() && AbsynUtil::innerOuterEqual(prefixes1.innerOuter.clone(), prefixes2.innerOuter.clone()) && replaceableEqual(prefixes1.replaceablePrefix.clone(), prefixes2.replaceablePrefix.clone())?;
    Ok(equal)
}

pub fn prefixesFinal(inPrefixes: Arc<SCode::Prefixes>) -> Result<SCode::Final> {
    let mut outFinal: SCode::Final;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::PREFIXES { finalPrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outFinal = __pa0.clone();
    Ok(outFinal)
}

pub fn prefixesInnerOuter(inPrefixes: Arc<SCode::Prefixes>) -> Result<Absyn::InnerOuter> {
    let mut outInnerOuter: Absyn::InnerOuter;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::PREFIXES { innerOuter: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outInnerOuter = __pa0.clone();
    Ok(outInnerOuter)
}

pub fn prefixesRedeclare(inPrefixes: Arc<SCode::Prefixes>) -> Result<SCode::Redeclare> {
    let mut outRedeclare: SCode::Redeclare;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::PREFIXES { redeclarePrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outRedeclare = __pa0.clone();
    Ok(outRedeclare)
}

pub fn prefixesReplaceable(prefixes: Arc<SCode::Prefixes>) -> Result<Arc<SCode::Replaceable>> {
    let mut repl: Arc<SCode::Replaceable>;
    let __pa0 = ::match_deref::match_deref! { match &(prefixes.clone()) {
        Deref @ SCode::PREFIXES { replaceablePrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    repl = __pa0.clone();
    Ok(repl)
}

pub fn prefixesSetInnerOuter(prefixes: Arc<SCode::Prefixes>, innerOuter: Absyn::InnerOuter) -> Arc<SCode::Prefixes> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.innerOuter = innerOuter.clone());
    prefixes
}

pub fn prefixesSetRedeclare(prefixes: Arc<SCode::Prefixes>, inRedeclare: SCode::Redeclare) -> Arc<SCode::Prefixes> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.redeclarePrefix = inRedeclare.clone());
    prefixes
}

pub fn prefixesSetReplaceable(prefixes: Arc<SCode::Prefixes>, inReplaceable: Arc<SCode::Replaceable>) -> Arc<SCode::Prefixes> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.replaceablePrefix = inReplaceable.clone());
    prefixes
}

pub fn prefixesSetVisibility(prefixes: Arc<SCode::Prefixes>, inVisibility: SCode::Visibility) -> Arc<SCode::Prefixes> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.visibility = inVisibility.clone());
    prefixes
}

pub fn prefixesVisibility(inPrefixes: Arc<SCode::Prefixes>) -> Result<SCode::Visibility> {
    let mut outVisibility: SCode::Visibility;
    let __pa0 = ::match_deref::match_deref! { match &(inPrefixes.clone()) {
        Deref @ SCode::PREFIXES { visibility: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outVisibility = __pa0.clone();
    Ok(outVisibility)
}

pub fn prependSubModToMod(subMod: Arc<SCode::SubMod>, r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    r#mod = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::NOMOD => Arc::new(SCode::Mod::MOD { finalPrefix: crate::SCode::Final::NOT_FINAL, eachPrefix: crate::SCode::Each::NOT_EACH, subModLst: list![subMod.clone()], binding: None, comment: None, info: Error::dummyInfo.clone() }),
        Deref @ SCode::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = cons(subMod.clone(), var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()));
            r#mod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(r#mod)
}

pub fn propagateArrayDimensions(inOriginalDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, inNewDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outNewDims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    outNewDims = (::match_deref::match_deref! { match &((inOriginalDims.clone(), inNewDims.clone())) {
        (_, Deref @ metamodelica::List::Nil) => inOriginalDims.clone(),
        _ => inNewDims.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outNewDims
}

pub fn propagateAttributes(inOriginalAttributes: SCode::Attributes, inNewAttributes: SCode::Attributes, inNewTypeIsArray: bool) -> Result<SCode::Attributes> {
    let mut outNewAttributes: SCode::Attributes;
    let mut dims1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    let mut dims2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    let mut ct1: SCode::ConnectorType;
    let mut ct2: SCode::ConnectorType;
    let mut prl1: SCode::Parallelism;
    let mut prl2: SCode::Parallelism;
    let mut var1: SCode::Variability;
    let mut var2: SCode::Variability;
    let mut dir1: Absyn::Direction;
    let mut dir2: Absyn::Direction;
    let mut if1: Absyn::IsField;
    let mut if2: Absyn::IsField;
    let SCode::ATTR { arrayDims: __pa0, connectorType: __pa1, parallelism: __pa2, variability: __pa3, direction: __pa4, isField: __pa5 } = (inOriginalAttributes.clone()) else { bail!("pattern mismatch") };
    dims1 = __pa0.clone();
    ct1 = __pa1.clone();
    prl1 = __pa2.clone();
    var1 = __pa3.clone();
    dir1 = __pa4.clone();
    if1 = __pa5.clone();
    let SCode::ATTR { arrayDims: __pa6, connectorType: __pa7, parallelism: __pa8, variability: __pa9, direction: __pa10, isField: __pa11 } = (inNewAttributes.clone()) else { bail!("pattern mismatch") };
    dims2 = __pa6.clone();
    ct2 = __pa7.clone();
    prl2 = __pa8.clone();
    var2 = __pa9.clone();
    dir2 = __pa10.clone();
    if2 = __pa11.clone();
    if !(inNewTypeIsArray.clone()) {
        dims2 = propagateArrayDimensions(dims1.clone(), dims2.clone());
    }
    ct2 = propagateConnectorType(ct1.clone(), ct2.clone());
    prl2 = propagateParallelism(prl1.clone(), prl2.clone());
    var2 = propagateVariability(var1.clone(), var2.clone());
    dir2 = propagateDirection(dir1.clone(), dir2.clone());
    if2 = propagateIsField(if1.clone(), if2.clone());
    outNewAttributes = SCode::Attributes { arrayDims: dims2.clone(), connectorType: ct2.clone(), parallelism: prl2.clone(), variability: var2.clone(), direction: dir2.clone(), isField: if2.clone() };
    Ok(outNewAttributes)
}

pub fn propagateAttributesClass(originalClass: Arc<SCode::Element>, newClass: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut newClass: Arc<SCode::Element> = newClass;
    let () = (::match_deref::match_deref! { match &((originalClass.clone(), newClass.clone())) {
        (Deref @ SCode::CLASS { .. }, Deref @ SCode::CLASS { .. }) => {
            assign_variant_field!(newClass => SCode::Element::CLASS; prefixes = propagatePrefixes(var_field!((*originalClass).prefixes, SCode::Element::CLASS).clone(), var_field!((*newClass).prefixes, SCode::Element::CLASS).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newClass)
}

pub fn propagateAttributesVar(originalVar: Arc<SCode::Element>, newVar: Arc<SCode::Element>, isNewTypeArray: bool) -> Result<Arc<SCode::Element>> {
    let mut newVar: Arc<SCode::Element> = newVar;
    let () = (::match_deref::match_deref! { match &((originalVar.clone(), newVar.clone())) {
        (Deref @ SCode::COMPONENT { .. }, Deref @ SCode::COMPONENT { .. }) => {
            assign_variant_field!(newVar => SCode::Element::COMPONENT;
                prefixes = propagatePrefixes(var_field!((*originalVar).prefixes, SCode::Element::COMPONENT).clone(), var_field!((*newVar).prefixes, SCode::Element::COMPONENT).clone())?,
                attributes = propagateAttributes(var_field!((*originalVar).attributes, SCode::Element::COMPONENT).clone(), var_field!((*newVar).attributes, SCode::Element::COMPONENT).clone(), isNewTypeArray.clone())?
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newVar)
}

pub fn propagateConnectorType(inOriginalConnectorType: SCode::ConnectorType, inNewConnectorType: SCode::ConnectorType) -> SCode::ConnectorType {
    let mut outNewConnectorType: SCode::ConnectorType;
    outNewConnectorType = (match inNewConnectorType.clone() {
        SCode::POTENTIAL => inOriginalConnectorType.clone(),
        _ => inNewConnectorType.clone(),
    });
    outNewConnectorType
}

pub fn propagateDirection(inOriginalDirection: Absyn::Direction, inNewDirection: Absyn::Direction) -> Absyn::Direction {
    let mut outNewDirection: Absyn::Direction;
    outNewDirection = (match inNewDirection.clone() {
        Absyn::BIDIR => inOriginalDirection.clone(),
        _ => inNewDirection.clone(),
    });
    outNewDirection
}

pub fn propagateIsField(inOriginalIsField: Absyn::IsField, inNewIsField: Absyn::IsField) -> Absyn::IsField {
    let mut outNewIsField: Absyn::IsField;
    outNewIsField = (match inNewIsField.clone() {
        Absyn::NONFIELD => inOriginalIsField.clone(),
        _ => inNewIsField.clone(),
    });
    outNewIsField
}

pub fn propagateParallelism(inOriginalParallelism: SCode::Parallelism, inNewParallelism: SCode::Parallelism) -> SCode::Parallelism {
    let mut outNewParallelism: SCode::Parallelism;
    outNewParallelism = (match inNewParallelism.clone() {
        SCode::NON_PARALLEL => inOriginalParallelism.clone(),
        _ => inNewParallelism.clone(),
    });
    outNewParallelism
}

pub fn propagatePrefixInnerOuter(inOriginalIO: Absyn::InnerOuter, inIO: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut outIO: Absyn::InnerOuter;
    outIO = (match inIO.clone() {
        Absyn::NOT_INNER_OUTER => inOriginalIO.clone(),
        _ => inIO.clone(),
    });
    outIO
}

pub fn propagatePrefixes(originalPrefixes: Arc<SCode::Prefixes>, newPrefixes: Arc<SCode::Prefixes>) -> Result<Arc<SCode::Prefixes>> {
    let mut newPrefixes: Arc<SCode::Prefixes> = newPrefixes;
    let () = (::match_deref::match_deref! { match &((originalPrefixes.clone(), newPrefixes.clone())) {
        (Deref @ SCode::PREFIXES { .. }, Deref @ SCode::PREFIXES { .. }) => {
            assign_variant_field!(newPrefixes => SCode::Prefixes::PREFIXES; innerOuter = propagatePrefixInnerOuter(originalPrefixes.innerOuter.clone(), newPrefixes.innerOuter.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newPrefixes)
}

pub fn propagateVariability(inOriginalVariability: SCode::Variability, inNewVariability: SCode::Variability) -> SCode::Variability {
    let mut outNewVariability: SCode::Variability;
    outNewVariability = (match inNewVariability.clone() {
        SCode::VAR => inOriginalVariability.clone(),
        _ => inNewVariability.clone(),
    });
    outNewVariability
}

pub fn redeclareBool(inRedeclare: SCode::Redeclare) -> Result<bool> {
    let mut bRedeclare: bool;
    bRedeclare = (match inRedeclare.clone() {
        SCode::REDECLARE => true,
        SCode::NOT_REDECLARE => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bRedeclare)
}

pub fn removeAttributeDimensions(attributes: SCode::Attributes) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.arrayDims = metamodelica::nil();
    attributes
}

pub fn removeBuiltinsFromTopScope(inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outProgram = List::filterOnTrue(inProgram.clone(), Arc::new(fnptr!(isNotBuiltinClass, Arc<SCode::Element>)));
    outProgram
}

pub fn removeComponentCondition(element: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; condition = None);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn removeGivenSubModNames(submod: Arc<SCode::SubMod>, namesToRemove: Arc<metamodelica::List<ArcStr>>) -> bool {
    let mut keep: bool;
    keep = !(listMember((submod.ident.clone()).clone(), namesToRemove.clone()));
    keep
}

fn removeSub(inSub: Arc<SCode::SubMod>, inOld: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut outSubs: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    outSubs = 'mc: {
        let __mc_input = (inSub.clone(), inOld.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    let mut rest: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut idxs1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut idxs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut s: Arc<SCode::SubMod>;
                    Ok(inOld.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::NAMEMOD { ident: id1, .. }, Deref @ metamodelica::List::Cons { head: Deref @ SCode::NAMEMOD { ident: id2, .. }, tail: rest }) => {
                    let mut idxs1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut idxs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut s: Arc<SCode::SubMod>;
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(rest.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: s, tail: rest }) => {
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut idxs1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut idxs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut rest = (*rest).clone();
                    rest = removeSub(inSub.clone(), rest.clone())?;
                    Ok(cons(s.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSubs)
}

pub fn renameElement(element: Arc<SCode::Element>, name: ArcStr) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS; name = name.clone());
            ()
        },
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; name = name.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn replaceableBool(inReplaceable: Arc<SCode::Replaceable>) -> Result<bool> {
    let mut bReplaceable: bool;
    bReplaceable = (::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::REPLACEABLE { .. } => true,
        Deref @ SCode::NOT_REPLACEABLE => false,
        _ => bail!("match: no arm matched"),
    } });
    Ok(bReplaceable)
}

pub fn replaceableEqual(r1: Arc<SCode::Replaceable>, r2: Arc<SCode::Replaceable>) -> Result<bool> {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (r1.clone(), r2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::NOT_REPLACEABLE, Deref @ SCode::NOT_REPLACEABLE) => {
                    let mut p1: Arc<Absyn::Path>;
                    let mut p2: Arc<Absyn::Path>;
                    let mut m1: Arc<SCode::Mod>;
                    let mut m2: Arc<SCode::Mod>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::REPLACEABLE { cc: Some(SCode::CONSTRAINCLASS { modifier: m1, constrainingClass: p1, .. }) }, Deref @ SCode::REPLACEABLE { cc: Some(SCode::CONSTRAINCLASS { modifier: m2, constrainingClass: p2, .. }) }) => {
                    let true = (AbsynUtil::pathEqual(p1.clone(), p2.clone())) else { bail!("pattern mismatch") };
                    let true = (modEqual(m1.clone(), m2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::REPLACEABLE { cc: None }, Deref @ SCode::REPLACEABLE { cc: None }) => {
                    let mut p1: Arc<Absyn::Path>;
                    let mut p2: Arc<Absyn::Path>;
                    let mut m1: Arc<SCode::Mod>;
                    let mut m2: Arc<SCode::Mod>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut p1: Arc<Absyn::Path>;
                    let mut p2: Arc<Absyn::Path>;
                    let mut m1: Arc<SCode::Mod>;
                    let mut m2: Arc<SCode::Mod>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

pub fn replaceableOptConstraint(inReplaceable: Arc<SCode::Replaceable>) -> Result<Option<Arc<SCode::ConstrainClass>>> {
    let mut outOptConstrainClass: Option<Arc<SCode::ConstrainClass>>;
    outOptConstrainClass = (::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::REPLACEABLE { cc } => cc.clone(),
        Deref @ SCode::NOT_REPLACEABLE => None,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outOptConstrainClass)
}

pub fn restrictionEqual(restr1: SCode::Restriction, restr2: SCode::Restriction) -> bool {
    let mut equal: bool;
    equal = (match (restr1.clone(), restr2.clone()) {
        (SCode::R_CLASS, SCode::R_CLASS) => true,
        (SCode::R_OPTIMIZATION, SCode::R_OPTIMIZATION) => true,
        (SCode::R_MODEL, SCode::R_MODEL) => true,
        (SCode::R_RECORD { isOperator: true }, SCode::R_RECORD { isOperator: true }) => true,
        (SCode::R_RECORD { isOperator: false }, SCode::R_RECORD { isOperator: false }) => true,
        (SCode::R_BLOCK, SCode::R_BLOCK) => true,
        (SCode::R_CONNECTOR { isExpandable: true }, SCode::R_CONNECTOR { isExpandable: true }) => true,
        (SCode::R_CONNECTOR { isExpandable: false }, SCode::R_CONNECTOR { isExpandable: false }) => true,
        (SCode::R_OPERATOR, SCode::R_OPERATOR) => true,
        (SCode::R_TYPE, SCode::R_TYPE) => true,
        (SCode::R_PACKAGE, SCode::R_PACKAGE) => true,
        (SCode::R_FUNCTION { functionRestriction: mut funcRest1 }, SCode::R_FUNCTION { functionRestriction: mut funcRest2 }) => funcRestrictionEqual(funcRest1.clone(), funcRest2.clone()),
        (SCode::R_ENUMERATION, SCode::R_ENUMERATION) => true,
        (SCode::R_PREDEFINED_INTEGER, SCode::R_PREDEFINED_INTEGER) => true,
        (SCode::R_PREDEFINED_REAL, SCode::R_PREDEFINED_REAL) => true,
        (SCode::R_PREDEFINED_STRING, SCode::R_PREDEFINED_STRING) => true,
        (SCode::R_PREDEFINED_BOOLEAN, SCode::R_PREDEFINED_BOOLEAN) => true,
        (SCode::R_PREDEFINED_CLOCK, SCode::R_PREDEFINED_CLOCK) => true,
        (SCode::R_PREDEFINED_ENUMERATION, SCode::R_PREDEFINED_ENUMERATION) => true,
        (SCode::R_UNIONTYPE { .. }, SCode::R_UNIONTYPE { .. }) => {
        let mut __acc: Option<bool> = None;
        for (t1, t2) in (&(var_field!(restr1.typeVars, SCode::Restriction::R_UNIONTYPE).clone())).into_iter().zip((&(var_field!(restr2.typeVars, SCode::Restriction::R_UNIONTYPE).clone())).into_iter()) {
            let __x = t1.clone() == t2.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction")).unwrap()
    },
        _ => false,
    });
    equal
}

pub fn setAnnotationInComment(name: ArcStr, value: Arc<Absyn::Exp>, cmt: Arc<SCode::Comment>, replace: bool) -> Result<Arc<SCode::Comment>> {
    let mut cmt: Arc<SCode::Comment> = cmt;
    let mut ann: Arc<SCode::Annotation>;
    let mut r#mod: Arc<SCode::Mod>;
    if isNone(cmt.annotation_.clone()) {
        assign_field!(cmt.annotation_ = Some(makeSingleAnnotation((name.clone()).clone(), value.clone())));
        return Ok(cmt);
    } else {
        assign_field!(cmt.annotation_ = Some(setAnnotationValue((name.clone()).clone(), value.clone(), Util::getOption(cmt.annotation_.clone())?, replace.clone())?));
    }
    Ok(cmt)
}

pub fn setAnnotationValue(name: ArcStr, value: Arc<Absyn::Exp>, ann: Arc<SCode::Annotation>, replace: bool) -> Result<Arc<SCode::Annotation>> {
    fn replace_mod(name: ArcStr, value: Arc<Absyn::Exp>, replace: bool, r#mod: Arc<SCode::SubMod>) -> (Arc<SCode::SubMod>, bool) {
        let mut r#mod: Arc<SCode::SubMod> = r#mod;
        let mut found: bool;
        found = r#mod.ident.clone() == name.clone();
        if found.clone() && replace.clone() {
            assign_field!(r#mod.r#mod = setModifierBinding(Some(value.clone()), r#mod.r#mod.clone()));
        }
        (r#mod, found)
    }

    let mut ann: Arc<SCode::Annotation> = ann;
    let mut r#mod: Arc<SCode::Mod>;
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    let mut found: bool;
    let () = (::match_deref::match_deref! { match &(ann.clone()) {
        Deref @ SCode::Annotation::ANNOTATION { modification: r#mod @ Deref @ SCode::Mod::MOD { .. } } => {
            let mut r#mod = (*r#mod).clone();
            (submods, found) = List::findMap(var_field!((**r#mod).subModLst, SCode::Mod::MOD).clone(), Arc::new({ let __pe_b0 = name.clone(); let __pe_b1 = value.clone(); let __pe_b2 = replace.clone(); move |__pe_a3| Ok(replace_mod(__pe_b0.clone(), __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }))?;
            if !(found.clone()) {
                submods = cons(Arc::new(SCode::SubMod { ident: (name.clone()).clone(), r#mod: makeMod(false, false, metamodelica::nil(), Some(value.clone()), None, AbsynUtil::dummyInfo.clone()) }), submods.clone());
            }
            let __owned_variant_subModLst_0 = submods.clone();
            if let SCode::Mod::MOD { subModLst, .. } = &mut r#mod {
                *subModLst = __owned_variant_subModLst_0;
            } else { panic!("owned-variant field-assign: value held a different variant than SCode::Mod::MOD"); }
            assign_variant_field!(ann => SCode::Annotation::ANNOTATION; modification = Arc::new(r#mod.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ann)
}

pub fn setAttributesDirection(attributes: SCode::Attributes, direction: Absyn::Direction) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.direction = direction.clone();
    attributes
}

pub fn setAttributesVariability(attributes: SCode::Attributes, variability: SCode::Variability) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.variability = variability.clone();
    attributes
}

pub fn setBaseClassPath(element: Arc<SCode::Element>, inBcPath: Arc<Absyn::Path>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; baseClassPath = inBcPath.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn setClassDef(classDef: Arc<SCode::ClassDef>, cls: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cls: Arc<SCode::Element> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(cls => SCode::Element::CLASS; classDef = classDef.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

fn setClassDefMod(classDef: Arc<SCode::ClassDef>, inMod: Arc<SCode::Mod>) -> Arc<SCode::ClassDef> {
    let mut classDef: Arc<SCode::ClassDef> = classDef;
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ SCode::DERIVED { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::DERIVED; modifications = inMod.clone());
            ()
        },
        Deref @ SCode::CLASS_EXTENDS { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; modifications = inMod.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    classDef
}

pub fn setClassName(name: ArcStr, cl: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::CLASS { .. } => {
            if name.clone() != var_field!((*cl).name, SCode::Element::CLASS).clone() {
                assign_variant_field!(cl => SCode::Element::CLASS; name = name.clone());
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn setClassPartialPrefix(partialPrefix: SCode::Partial, cl: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::CLASS { .. } => {
            if !(partialPrefix.clone() == var_field!((*cl).partialPrefix, SCode::Element::CLASS).clone()) {
                assign_variant_field!(cl => SCode::Element::CLASS; partialPrefix = partialPrefix.clone());
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn setClassPrefixes(prefixes: Arc<SCode::Prefixes>, cl: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(cl => SCode::Element::CLASS; prefixes = prefixes.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn setClassRestriction(r: SCode::Restriction, cl: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut cl: Arc<SCode::Element> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(cl => SCode::Element::CLASS; restriction = r.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn setComponentMod(element: Arc<SCode::Element>, r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = r#mod.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn setComponentName(element: Arc<SCode::Element>, name: ArcStr) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; name = name.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn setComponentTypeSpec(element: Arc<SCode::Element>, typeSpec: Arc<Absyn::TypeSpec>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; typeSpec = typeSpec.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn setElementMod(element: Arc<SCode::Element>, r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = r#mod.clone());
            ()
        },
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS; classDef = setClassDefMod(var_field!((*element).classDef, SCode::Element::CLASS).clone(), r#mod.clone()));
            ()
        },
        Deref @ SCode::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = r#mod.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn setElementName(e: Arc<SCode::Element>, name: ArcStr) -> Arc<SCode::Element> {
    let mut e: Arc<SCode::Element> = e;
    let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(e => SCode::Element::CLASS; name = name.clone());
            ()
        },
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(e => SCode::Element::COMPONENT; name = name.clone());
            ()
        },
        Deref @ SCode::DEFINEUNIT { .. } => {
            assign_variant_field!(e => SCode::Element::DEFINEUNIT; name = name.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    e
}

pub fn setElementPrefixes(prefixes: Arc<SCode::Prefixes>, element: Arc<SCode::Element>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS; prefixes = prefixes.clone());
            ()
        },
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; prefixes = prefixes.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

pub fn setModifierBinding(binding: Option<Arc<Absyn::Exp>>, r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; binding = binding.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

fn statementEqual(ai1: Arc<SCode::Statement>, ai2: Arc<SCode::Statement>) -> Result<bool> {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (ai1.clone(), ai2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ALG_ASSIGN { value: e1, assignComponent: Deref @ Absyn::CREF { componentRef: cr1 }, .. }, Deref @ SCode::ALG_ASSIGN { value: e2, assignComponent: Deref @ Absyn::CREF { componentRef: cr2 }, .. }) => {
                    let mut alg1: Arc<Absyn::Algorithm>;
                    let mut alg2: Arc<Absyn::Algorithm>;
                    let mut a1: Arc<SCode::Statement>;
                    let mut a2: Arc<SCode::Statement>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut equal: bool;
                    b1 = AbsynUtil::crefEqual(cr1.clone(), cr2.clone());
                    b2 = AbsynUtil::expEqual(e1.clone(), e2.clone())?;
                    equal = boolAnd(b1.clone(), b2.clone());
                    Ok(equal.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ALG_ASSIGN { value: e12, assignComponent: e11 @ Deref @ Absyn::TUPLE { expressions: _ }, .. }, Deref @ SCode::ALG_ASSIGN { value: e22, assignComponent: e21 @ Deref @ Absyn::TUPLE { expressions: _ }, .. }) => {
                    let mut alg1: Arc<Absyn::Algorithm>;
                    let mut alg2: Arc<Absyn::Algorithm>;
                    let mut a1: Arc<SCode::Statement>;
                    let mut a2: Arc<SCode::Statement>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut equal: bool;
                    b1 = AbsynUtil::expEqual(e11.clone(), e21.clone())?;
                    b2 = AbsynUtil::expEqual(e12.clone(), e22.clone())?;
                    equal = boolAnd(b1.clone(), b2.clone());
                    Ok(equal.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (a1, a2) => {
                    let mut alg1: Arc<Absyn::Algorithm>;
                    let mut alg2: Arc<Absyn::Algorithm>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(statementToAlgorithmItem(a1.clone())?) {
                        Deref @ Absyn::ALGORITHMITEM { algorithm_: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    alg1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(statementToAlgorithmItem(a2.clone())?) {
                        Deref @ Absyn::ALGORITHMITEM { algorithm_: __pa1, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    alg2 = __pa1.clone();
                    Ok(alg1.clone() == alg2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut alg1: Arc<Absyn::Algorithm>;
                    let mut alg2: Arc<Absyn::Algorithm>;
                    let mut a1: Arc<SCode::Statement>;
                    let mut a2: Arc<SCode::Statement>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut e11: Arc<Absyn::Exp>;
                    let mut e12: Arc<Absyn::Exp>;
                    let mut e21: Arc<Absyn::Exp>;
                    let mut e22: Arc<Absyn::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

pub fn statementToAlgorithmItem(stmt: Arc<SCode::Statement>) -> Result<Arc<Absyn::AlgorithmItem>> {
    let mut algi: Arc<Absyn::AlgorithmItem>;
    algi = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::ALG_ASSIGN { assignComponent, value, comment: _, info } => Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: assignComponent.clone(), value: value.clone() }), comment: None, info: info.clone() }),
        Deref @ SCode::ALG_IF { boolExpr, trueBranch, elseIfBranch: branches, elseBranch, comment: _, info } => {
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut iterator: ArcStr;
            let mut range: Option<Arc<Absyn::Exp>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut comment: Option<Arc<SCode::Comment>>;
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>>>;
            let mut abranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            algs1 = List::map(trueBranch.clone(), Arc::new(statementToAlgorithmItem));
            conditions = List::map(branches.clone(), Arc::new(fnptr!(Util::tuple21, _)));
            stmtsList = List::map(branches.clone(), Arc::new(fnptr!(Util::tuple22, _)));
            algsLst = List::mapList(stmtsList.clone(), Arc::new(statementToAlgorithmItem));
            abranches = List::zip(conditions.clone(), algsLst.clone());
            algs2 = List::map(elseBranch.clone(), Arc::new(statementToAlgorithmItem));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_IF { ifExp: boolExpr.clone(), trueBranch: algs1.clone(), elseIfAlgorithmBranch: abranches.clone(), elseBranch: algs2.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::ALG_FOR { index: iterator, range, forBody: body, comment: _, info } => {
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut comment: Option<Arc<SCode::Comment>>;
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>>>;
            let mut abranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            algs1 = List::map(body.clone(), Arc::new(statementToAlgorithmItem));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_FOR { iterators: list![Arc::new(Absyn::ForIterator { name: (iterator.clone()).clone(), guardExp: None, range: range.clone() })], forBody: algs1.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::ALG_PARFOR { index: iterator, range, parforBody: body, comment: _, info } => {
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut comment: Option<Arc<SCode::Comment>>;
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>>>;
            let mut abranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            algs1 = List::map(body.clone(), Arc::new(statementToAlgorithmItem));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_PARFOR { iterators: list![Arc::new(Absyn::ForIterator { name: (iterator.clone()).clone(), guardExp: None, range: range.clone() })], parforBody: algs1.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::ALG_WHILE { boolExpr, whileBody: body, comment: _, info } => {
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut iterator: ArcStr;
            let mut range: Option<Arc<Absyn::Exp>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut comment: Option<Arc<SCode::Comment>>;
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>>>;
            let mut abranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            algs1 = List::map(body.clone(), Arc::new(statementToAlgorithmItem));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_WHILE { boolExpr: boolExpr.clone(), whileBody: algs1.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::ALG_WHEN_A { branches, comment: _, info } => {
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut iterator: ArcStr;
            let mut range: Option<Arc<Absyn::Exp>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut comment: Option<Arc<SCode::Comment>>;
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>>>;
            let mut abranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::map(branches.clone(), Arc::new(fnptr!(Util::tuple21, _)))) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            boolExpr = __pa0.clone();
            conditions = __pa1.clone();
            stmtsList = List::map(branches.clone(), Arc::new(fnptr!(Util::tuple22, _)));
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(List::mapList(stmtsList.clone(), Arc::new(statementToAlgorithmItem))) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            algs1 = __pa2.clone();
            algsLst = __pa3.clone();
            abranches = List::zip(conditions.clone(), algsLst.clone());
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_WHEN_A { boolExpr: boolExpr.clone(), whenBody: algs1.clone(), elseWhenAlgorithmBranch: abranches.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::ALG_ASSERT { .. } => Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("assert")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![var_field!((*stmt).condition, SCode::Statement::ALG_ASSERT).clone(), var_field!((*stmt).message, SCode::Statement::ALG_ASSERT).clone(), var_field!((*stmt).level, SCode::Statement::ALG_ASSERT).clone()], argNames: metamodelica::nil() }) }), comment: None, info: var_field!((*stmt).info, SCode::Statement::ALG_ASSERT).clone() }),
        Deref @ SCode::ALG_TERMINATE { .. } => Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("terminate")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![var_field!((*stmt).message, SCode::Statement::ALG_TERMINATE).clone()], argNames: metamodelica::nil() }) }), comment: None, info: var_field!((*stmt).info, SCode::Statement::ALG_TERMINATE).clone() }),
        Deref @ SCode::ALG_REINIT { .. } => Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("reinit")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![var_field!((*stmt).cref, SCode::Statement::ALG_REINIT).clone(), var_field!((*stmt).newValue, SCode::Statement::ALG_REINIT).clone()], argNames: metamodelica::nil() }) }), comment: None, info: var_field!((*stmt).info, SCode::Statement::ALG_REINIT).clone() }),
        Deref @ SCode::ALG_NORETCALL { exp: Deref @ Absyn::CALL { functionArgs, function_: functionCall, .. }, comment: _, info } => Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: functionCall.clone(), functionArgs: functionArgs.clone() }), comment: None, info: info.clone() }),
        Deref @ SCode::ALG_RETURN { comment: _, info } => Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(openmodelica_ast::Absyn::Algorithm::ALG_RETURN), comment: None, info: info.clone() }),
        Deref @ SCode::ALG_BREAK { comment: _, info } => Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(openmodelica_ast::Absyn::Algorithm::ALG_BREAK), comment: None, info: info.clone() }),
        Deref @ SCode::ALG_CONTINUE { comment: _, info } => Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(openmodelica_ast::Absyn::Algorithm::ALG_CONTINUE), comment: None, info: info.clone() }),
        Deref @ SCode::ALG_FAILURE { stmts: body, comment: _, info } => {
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut iterator: ArcStr;
            let mut range: Option<Arc<Absyn::Exp>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut stmtsList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Statement>>>>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut comment: Option<Arc<SCode::Comment>>;
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>>>;
            let mut abranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            algs1 = List::map(body.clone(), Arc::new(statementToAlgorithmItem));
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: Arc::new(Absyn::Algorithm::ALG_FAILURE { equ: algs1.clone() }), comment: None, info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(algi)
}

pub fn streamBool(inStream: SCode::ConnectorType) -> bool {
    let mut bStream: bool;
    bStream = (match inStream.clone() {
        SCode::STREAM => true,
        _ => false,
    });
    bStream
}

pub fn stripAnnotationFromComment(inComment: Option<Arc<SCode::Comment>>) -> Option<Arc<SCode::Comment>> {
    let mut outComment: Option<Arc<SCode::Comment>>;
    outComment = (match inComment.clone() {
        Some(SCode::COMMENT { annotation_: _, comment: mut r#str }) => Some(Arc::new(SCode::Comment { annotation_: None, comment: r#str.clone() })),
        _ => None,
    });
    outComment
}

pub fn stripCommentsFromAlgorithm(alg: Arc<SCode::AlgorithmSection>, stripAnn: bool, stripCmt: bool) -> Result<Arc<SCode::AlgorithmSection>> {
    let mut alg: Arc<SCode::AlgorithmSection> = alg;
    assign_field!(alg.statements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (alg.statements.clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(alg)
}

pub fn stripCommentsFromClassDef(cdef: Arc<SCode::ClassDef>, stripAnn: bool, stripCmt: bool) -> Result<Arc<SCode::ClassDef>> {
    let mut cdef: Arc<SCode::ClassDef> = cdef;
    cdef = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::PARTS { .. } => {
            let mut el: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut ieql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut alg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut ialg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut ext: Option<Arc<SCode::ExternalDecl>>;
            el = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for e in (var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromElement(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            eql = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for eq in (var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(eq.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ieql = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for ieq in (var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(ieq.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            alg = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
        for a in (var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromAlgorithm(a.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ialg = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
        for ia in (var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = stripCommentsFromAlgorithm(ia.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            ext = stripCommentsFromExternalDecl(var_field!((*cdef).externalDecl, SCode::ClassDef::PARTS).clone(), stripAnn.clone(), stripCmt.clone())?;
            Arc::new(SCode::ClassDef::PARTS { elementLst: el.clone(), normalEquationLst: eql.clone(), initialEquationLst: ieql.clone(), normalAlgorithmLst: alg.clone(), initialAlgorithmLst: ialg.clone(), constraintLst: var_field!((*cdef).constraintLst, SCode::ClassDef::PARTS).clone(), clsattrs: var_field!((*cdef).clsattrs, SCode::ClassDef::PARTS).clone(), externalDecl: ext.clone() })
        },
        Deref @ SCode::CLASS_EXTENDS { .. } => {
            let mut el: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut ieql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut alg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut ialg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut ext: Option<Arc<SCode::ExternalDecl>>;
            assign_variant_field!(cdef => SCode::ClassDef::CLASS_EXTENDS;
                modifications = stripCommentsFromMod(var_field!((*cdef).modifications, SCode::ClassDef::CLASS_EXTENDS).clone(), stripAnn.clone(), stripCmt.clone())?,
                composition = stripCommentsFromClassDef(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), stripAnn.clone(), stripCmt.clone())?
            );
            cdef.clone()
        },
        Deref @ SCode::DERIVED { .. } => {
            let mut el: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut ieql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut alg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut ialg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut ext: Option<Arc<SCode::ExternalDecl>>;
            assign_variant_field!(cdef => SCode::ClassDef::DERIVED; modifications = stripCommentsFromMod(var_field!((*cdef).modifications, SCode::ClassDef::DERIVED).clone(), stripAnn.clone(), stripCmt.clone())?);
            cdef.clone()
        },
        Deref @ SCode::ENUMERATION { .. } => {
            let mut el: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut ieql: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut alg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut ialg: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut ext: Option<Arc<SCode::ExternalDecl>>;
            assign_variant_field!(cdef => SCode::ClassDef::ENUMERATION; enumLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Enum>>> = metamodelica::nil();
        for e in (var_field!((*cdef).enumLst, SCode::ClassDef::ENUMERATION).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEnum(e.clone(), stripAnn.clone(), stripCmt.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            cdef.clone()
        },
        _ => cdef.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cdef)
}

pub fn stripCommentsFromComment(cmt: Arc<SCode::Comment>, stripAnn: bool, stripCmt: bool) -> Arc<SCode::Comment> {
    let mut cmt: Arc<SCode::Comment> = cmt;
    if stripAnn.clone() {
        assign_field!(cmt.annotation_ = None);
    }
    if stripCmt.clone() {
        assign_field!(cmt.comment = None);
    }
    cmt
}

pub fn stripCommentsFromElement(element: Arc<SCode::Element>, stripAnn: bool, stripCmt: bool) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::EXTENDS { .. } => {
            if stripAnn.clone() {
                assign_variant_field!(element => SCode::Element::EXTENDS; ann = None);
            }
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = stripCommentsFromMod(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), stripAnn.clone(), stripCmt.clone())?);
            ()
        },
        Deref @ SCode::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS;
                classDef = stripCommentsFromClassDef(var_field!((*element).classDef, SCode::Element::CLASS).clone(), stripAnn.clone(), stripCmt.clone())?,
                cmt = stripCommentsFromComment(var_field!((*element).cmt, SCode::Element::CLASS).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT;
                modifications = stripCommentsFromMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone(), stripAnn.clone(), stripCmt.clone())?,
                comment = stripCommentsFromComment(var_field!((*element).comment, SCode::Element::COMPONENT).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

pub fn stripCommentsFromEnum(r#enum: Arc<SCode::Enum>, stripAnn: bool, stripCmt: bool) -> Arc<SCode::Enum> {
    let mut r#enum: Arc<SCode::Enum> = r#enum;
    assign_field!(r#enum.comment = stripCommentsFromComment(r#enum.comment.clone(), stripAnn.clone(), stripCmt.clone()));
    r#enum
}

pub fn stripCommentsFromEquation(eq: Arc<SCode::Equation>, stripAnn: bool, stripCmt: bool) -> Result<Arc<SCode::Equation>> {
    let mut eq: Arc<SCode::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::EQ_IF { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                thenBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
        for branch in (var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for e in (branch.clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for e in (var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_IF).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::EQ_EQUALS { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_EQUALS; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_EQUALS).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::EQ_PDE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_PDE; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_PDE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_CONNECT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::EQ_FOR { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR;
                eEquationLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for e in (var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_FOR).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::EQ_WHEN { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                eEquationLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for e in (var_field!((*eq).eEquationLst, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        for b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = stripCommentsFromWhenEqBranch(b.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_WHEN).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::EQ_ASSERT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_ASSERT; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_ASSERT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::EQ_TERMINATE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_TERMINATE; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_TERMINATE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::EQ_REINIT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_REINIT; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_REINIT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::EQ_NORETCALL { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_NORETCALL; comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_NORETCALL).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(eq)
}

pub fn stripCommentsFromExternalDecl(extDecl: Option<Arc<SCode::ExternalDecl>>, stripAnn: bool, stripCmt: bool) -> Result<Option<Arc<SCode::ExternalDecl>>> {
    let mut extDecl: Option<Arc<SCode::ExternalDecl>> = extDecl;
    let mut ext_decl: Arc<SCode::ExternalDecl>;
    if isSome(extDecl.clone()) && stripAnn.clone() {
        let Some(__pa0) = (extDecl.clone()) else { bail!("pattern mismatch") };
        ext_decl = __pa0.clone();
        assign_field!(ext_decl.annotation_ = None);
        extDecl = Some(ext_decl.clone());
    }
    Ok(extDecl)
}

pub fn stripCommentsFromMod(r#mod: Arc<SCode::Mod>, stripAnn: bool, stripCmt: bool) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for m in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            let __x = stripCommentsFromSubMod(m.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::REDECL { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::REDECL; element = stripCommentsFromElement(var_field!((*r#mod).element, SCode::Mod::REDECL).clone(), stripAnn.clone(), stripCmt.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn stripCommentsFromProgram(program: Arc<metamodelica::List<Arc<SCode::Element>>>, stripAnnotations: bool, stripComments: bool) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = program;
    program = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for e in (program.clone()).into_iter().cloned() {
            let __x = stripCommentsFromElement(e.clone(), stripAnnotations.clone(), stripComments.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(program)
}

pub fn stripCommentsFromStatement(stmt: Arc<SCode::Statement>, stripAnn: bool, stripCmt: bool) -> Result<Arc<SCode::Statement>> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::ALG_ASSIGN { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSIGN; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSIGN).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::ALG_IF { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                trueBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (var_field!((*stmt).trueBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseIfBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatementBranch(b.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_IF).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::ALG_FOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FOR;
                forBody = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (var_field!((*stmt).forBody, SCode::Statement::ALG_FOR).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_FOR).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::ALG_PARFOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR;
                parforBody = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (var_field!((*stmt).parforBody, SCode::Statement::ALG_PARFOR).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_PARFOR).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::ALG_WHILE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE;
                whileBody = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (var_field!((*stmt).whileBody, SCode::Statement::ALG_WHILE).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHILE).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::ALG_WHEN_A { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A;
                branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for b in (var_field!((*stmt).branches, SCode::Statement::ALG_WHEN_A).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatementBranch(b.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHEN_A).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSERT; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSERT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::ALG_TERMINATE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TERMINATE; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_TERMINATE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::ALG_REINIT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_REINIT; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_REINIT).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::ALG_NORETCALL { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_NORETCALL; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_NORETCALL).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::ALG_RETURN { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_RETURN; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_RETURN).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::ALG_BREAK { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_BREAK; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_BREAK).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::ALG_FAILURE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FAILURE; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_FAILURE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        Deref @ SCode::ALG_TRY { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TRY;
                body = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (var_field!((*stmt).body, SCode::Statement::ALG_TRY).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBody = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (var_field!((*stmt).elseBody, SCode::Statement::ALG_TRY).clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_TRY).clone(), stripAnn.clone(), stripCmt.clone())
            );
            ()
        },
        Deref @ SCode::ALG_CONTINUE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_CONTINUE; comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_CONTINUE).clone(), stripAnn.clone(), stripCmt.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(stmt)
}

pub fn stripCommentsFromStatementBranch(branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>), stripAnn: bool, stripCmt: bool) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)> {
    let mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>) = branch;
    let mut cond: Arc<Absyn::Exp>;
    let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
    (cond, body) = branch.clone();
    body = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for s in (body.clone()).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    branch = (cond.clone(), body.clone());
    Ok(branch)
}

pub fn stripCommentsFromSubMod(submod: Arc<SCode::SubMod>, stripAnn: bool, stripCmt: bool) -> Result<Arc<SCode::SubMod>> {
    let mut submod: Arc<SCode::SubMod> = submod;
    assign_field!(submod.r#mod = stripCommentsFromMod(submod.r#mod.clone(), stripAnn.clone(), stripCmt.clone())?);
    Ok(submod)
}

pub fn stripCommentsFromWhenEqBranch(branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>), stripAnn: bool, stripCmt: bool) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)> {
    let mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>) = branch;
    let mut cond: Arc<Absyn::Exp>;
    let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    (cond, body) = branch.clone();
    body = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for e in (body.clone()).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn.clone(), stripCmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    branch = (cond.clone(), body.clone());
    Ok(branch)
}

pub fn stripSubmod(r#mod: Arc<SCode::Mod>) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = metamodelica::nil());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

fn subModsEqual(inSubModLst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>, inSubModLst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<bool> {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (inSubModLst1.clone(), inSubModLst2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut mod1: Arc<SCode::Mod>;
                    let mut mod2: Arc<SCode::Mod>;
                    let mut ss1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut ss2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut subModLst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut subModLst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::NAMEMOD { ident: id1, r#mod: mod1 }, tail: subModLst1 }, Deref @ metamodelica::List::Cons { head: Deref @ SCode::NAMEMOD { ident: id2, r#mod: mod2 }, tail: subModLst2 }) => {
                    let mut ss1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut ss2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (modEqual(mod1.clone(), mod2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subModsEqual(subModLst1.clone(), subModLst2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut mod1: Arc<SCode::Mod>;
                    let mut mod2: Arc<SCode::Mod>;
                    let mut ss1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut ss2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut subModLst1: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    let mut subModLst2: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

fn subscriptEqual(sub1: Arc<Absyn::Subscript>, sub2: Arc<Absyn::Subscript>) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((sub1.clone(), sub2.clone())) {
        (Deref @ Absyn::NOSUB, Deref @ Absyn::NOSUB) => true,
        (Deref @ Absyn::SUBSCRIPT { subscript: e1 }, Deref @ Absyn::SUBSCRIPT { subscript: e2 }) => AbsynUtil::expEqual(e1.clone(), e2.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(equal)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn subscriptsEqual(inSs1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, inSs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<bool> {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (inSs1.clone(), inSs2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut ss1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut ss2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NOSUB, tail: ss1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NOSUB, tail: ss2 }) => {
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    Ok(subscriptsEqual(ss1.clone(), ss2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::SUBSCRIPT { subscript: e1 }, tail: ss1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::SUBSCRIPT { subscript: e2 }, tail: ss2 }) => {
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { bail!("pattern mismatch") };
                    let true = (subscriptsEqual(ss1.clone(), ss2.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<Absyn::Exp>;
                    let mut e2: Arc<Absyn::Exp>;
                    let mut ss1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    let mut ss2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(equal)
}

pub fn transformPathedElementInClassDef(path: Arc<Absyn::Path>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>, cls: Arc<SCode::ClassDef>) -> Result<(Arc<SCode::ClassDef>, bool)> {
    pub type Func = fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>>;

    let mut cls: Arc<SCode::ClassDef> = cls;
    let mut success: bool;
    let mut elems: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut cdef: Arc<SCode::ClassDef>;
    success = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            (elems, success) = transformPathedElementInProgram(path.clone(), func.clone(), var_field!((*cls).elementLst, SCode::ClassDef::PARTS).clone())?;
            if success.clone() {
                assign_variant_field!(cls => SCode::ClassDef::PARTS; elementLst = elems.clone());
            }
            success.clone()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            (cdef, success) = transformPathedElementInClassDef(path.clone(), func.clone(), var_field!((*cls).composition, SCode::ClassDef::CLASS_EXTENDS).clone())?;
            if success.clone() {
                assign_variant_field!(cls => SCode::ClassDef::CLASS_EXTENDS; composition = cdef.clone());
            }
            success.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cls, success))
}

pub fn transformPathedElementInElement(path: Arc<Absyn::Path>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>, element: Arc<SCode::Element>) -> Result<(Arc<SCode::Element>, bool)> {
    pub type Func = fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>>;

    let mut element: Arc<SCode::Element> = element;
    let mut success: bool;
    let mut cdef: Arc<SCode::ClassDef>;
    success = isElementNamed((AbsynUtil::pathFirstIdent(path.clone())?).clone(), element.clone());
    if success.clone() {
        if AbsynUtil::pathIsIdent(path.clone()) {
            element = func(element.clone())?;
        } else if isClass(element.clone()) {
            (cdef, success) = transformPathedElementInClassDef(AbsynUtil::pathRest(path.clone())?, func.clone(), getClassDef(element.clone())?)?;
            if success.clone() {
                element = setClassDef(cdef.clone(), element.clone())?;
            }
        }
    }
    Ok((element, success))
}

pub fn transformPathedElementInProgram(path: Arc<Absyn::Path>, func: Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>> + 'static>, program: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, bool)> {
    pub type Func = fn(Arc<SCode::Element>) -> Result<Arc<SCode::Element>>;

    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = program;
    let mut success: bool;
    (program, success) = List::findMap(program.clone(), Arc::new({ let __pe_b0 = path.clone(); let __pe_b1 = func.clone(); move |__pe_a2| transformPathedElementInElement(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?;
    Ok((program, success))
}

pub fn variabilityEqual(var1: SCode::Variability, var2: SCode::Variability) -> bool {
    let mut equal: bool;
    equal = (match (var1.clone(), var2.clone()) {
        (SCode::VAR, SCode::VAR) => true,
        (SCode::DISCRETE, SCode::DISCRETE) => true,
        (SCode::PARAM, SCode::PARAM) => true,
        (SCode::CONST, SCode::CONST) => true,
        _ => false,
    });
    equal
}

pub fn variabilityOr(inConst1: SCode::Variability, inConst2: SCode::Variability) -> SCode::Variability {
    let mut outConst: SCode::Variability;
    outConst = (match (inConst1.clone(), inConst2.clone()) {
        (SCode::CONST, _) => crate::SCode::Variability::CONST,
        (_, SCode::CONST) => crate::SCode::Variability::CONST,
        (SCode::PARAM, _) => crate::SCode::Variability::PARAM,
        (_, SCode::PARAM) => crate::SCode::Variability::PARAM,
        (SCode::DISCRETE, _) => crate::SCode::Variability::DISCRETE,
        (_, SCode::DISCRETE) => crate::SCode::Variability::DISCRETE,
        _ => crate::SCode::Variability::VAR,
    });
    outConst
}

pub fn visibilityBool(inVisibility: SCode::Visibility) -> Result<bool> {
    let mut bVisibility: bool;
    bVisibility = (match inVisibility.clone() {
        SCode::PUBLIC => true,
        SCode::PROTECTED => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(bVisibility)
}

pub fn visibilityEqual(inVisibility1: SCode::Visibility, inVisibility2: SCode::Visibility) -> bool {
    let mut outEqual: bool;
    outEqual = (match (inVisibility1.clone(), inVisibility2.clone()) {
        (SCode::PUBLIC, SCode::PUBLIC) => true,
        (SCode::PROTECTED, SCode::PROTECTED) => true,
        _ => false,
    });
    outEqual
}

