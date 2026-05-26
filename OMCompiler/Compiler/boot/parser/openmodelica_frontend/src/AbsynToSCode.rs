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
use crate::Dump;
use crate::InstHashTable;
use crate::MetaUtil;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub static ASSERTION_LEVEL_ERROR: std::sync::LazyLock<Arc<Absyn::Exp>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (literal!("AssertionLevel")).clone(), subscripts: metamodelica::nil(), componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("error")).clone(), subscripts: metamodelica::nil() }) }) }) }) });

fn checkTypeSpec(ts: Arc<Absyn::TypeSpec>, info: SourceInfo) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((ts.clone(), info.clone())) {
        (Deref @ Absyn::TPATH { .. }, _) => (),
        (Deref @ Absyn::TCOMPLEX { typeSpecs: Deref @ metamodelica::List::Cons { head: ts2, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::IDENT { name: Deref @ "tuple" }, .. }, _) => {
            let mut tss: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>;
            let mut r#str: ArcStr;
            r#str = (AbsynUtil::typeSpecString(ts.clone())?).clone();
            Error::addSourceMessage(Error::TCOMPLEX_TUPLE_ONE_NAME.clone(), list![(r#str.clone()).clone()], info.clone())?;
            checkTypeSpec(ts2.clone(), info.clone())?;
            ()
        },
        (Deref @ Absyn::TCOMPLEX { typeSpecs: tss @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, path: Deref @ Absyn::IDENT { name: Deref @ "tuple" }, .. }, _) => {
            let mut ts2: Arc<Absyn::TypeSpec>;
            let mut r#str: ArcStr;
            List::map1_0(tss.clone(), Arc::new(checkTypeSpec), info.clone());
            ()
        },
        (Deref @ Absyn::TCOMPLEX { typeSpecs: Deref @ metamodelica::List::Cons { head: ts2, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut tss: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>;
            let mut r#str: ArcStr;
            checkTypeSpec(ts2.clone(), info.clone())?;
            ()
        },
        (Deref @ Absyn::TCOMPLEX { typeSpecs: tss, .. }, _) => {
            let mut ts2: Arc<Absyn::TypeSpec>;
            let mut r#str: ArcStr;
            if listMember(var_field!((*ts).path, Absyn::TypeSpec::TCOMPLEX).clone(), list![Arc::new(Absyn::Path::IDENT { name: (literal!("list")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("List")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("array")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("Array")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("polymorphic")).clone() }), Arc::new(Absyn::Path::IDENT { name: (literal!("Option")).clone() })]) {
                r#str = (AbsynUtil::typeSpecString(ts.clone())?).clone();
                Error::addSourceMessage(Error::TCOMPLEX_MULTIPLE_NAMES.clone(), list![(r#str.clone()).clone()], info.clone())?;
                List::map1_0(tss.clone(), Arc::new(checkTypeSpec), info.clone());
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn containsExternalFuncDecl(inClass: Arc<Absyn::Class>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::CLASS { body: Deref @ Absyn::PARTS { classParts: parts, .. }, .. } => List::any(parts.clone(), Arc::new(fnptr!(AbsynUtil::isExternalPart, Arc<Absyn::ClassPart>))),
        Deref @ Absyn::CLASS { body: Deref @ Absyn::CLASS_EXTENDS { parts, .. }, .. } => List::any(parts.clone(), Arc::new(fnptr!(AbsynUtil::isExternalPart, Arc<Absyn::ClassPart>))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn getInfoAnnotationOrDefault(comment: Arc<SCode::Comment>, default: SourceInfo) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = (::match_deref::match_deref! { match &((comment.clone(), default.clone())) {
        (Deref @ SCode::COMMENT { annotation_: Some(SCode::ANNOTATION { modification: Deref @ SCode::MOD { subModLst: lst, .. } }), .. }, _) => getInfoAnnotationOrDefault2(lst.clone(), default.clone())?,
        _ => default.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(info)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getInfoAnnotationOrDefault2(lst: Arc<metamodelica::List<Arc<SCode::SubMod>>>, default: SourceInfo) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = (::match_deref::match_deref! { match &((lst.clone(), default.clone())) {
        (Deref @ metamodelica::List::Nil, _) => default.clone(),
        (Deref @ metamodelica::List::Cons { head: Deref @ SCode::NAMEMOD { r#mod: Deref @ SCode::MOD { binding: Some(Absyn::TUPLE { expressions: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::STRING { value: fileName }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::INTEGER { value: line }, tail: Deref @ metamodelica::List::Nil } } }), .. }, ident: Deref @ "__OpenModelica_FileInfo" }, tail: _ }, _) => SourceInfo { fileName: (fileName.clone()).clone(), isReadOnly: false, lineNumberStart: line.clone(), columnNumberStart: 0, lineNumberEnd: line.clone(), columnNumberEnd: 0, lastModification: 0.0 },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => getInfoAnnotationOrDefault2(rest.clone(), default.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(info)
}

pub fn getListofQualOperatorFuncsfromOperator(inOperator: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
    outNames = (::match_deref::match_deref! { match &(inOperator.clone()) {
        Deref @ SCode::CLASS { name: opername, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::R_OPERATOR, classDef: Deref @ SCode::PARTS { elementLst: els, .. }, cmt: _, info: _ } => {
            let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            names = List::map1(els.clone(), Arc::new(getOperatorQualName), (opername.clone()).clone());
            names.clone()
        },
        Deref @ SCode::CLASS { name: opername, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_OPERATOR_FUNCTION }, classDef: _, cmt: _, info: _ } => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            names = list![Arc::new(Absyn::Path::IDENT { name: (opername.clone()).clone() })];
            names.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNames)
}

pub fn getOperatorGivenName(inOperatorFunction: Arc<SCode::Element>) -> Result<Arc<Absyn::Path>> {
    let mut outName: Arc<Absyn::Path>;
    outName = (::match_deref::match_deref! { match &(inOperatorFunction.clone()) {
        Deref @ SCode::CLASS { name, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::R_FUNCTION { functionRestriction: SCode::FR_OPERATOR_FUNCTION }, classDef: _, cmt: _, info: _ } => Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outName)
}

pub fn getOperatorQualName(inOperatorFunction: Arc<SCode::Element>, operName: ArcStr) -> Result<Arc<Absyn::Path>> {
    let mut outName: Arc<Absyn::Path>;
    outName = (::match_deref::match_deref! { match &((inOperatorFunction.clone(), operName.clone())) {
        (Deref @ SCode::CLASS { name, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::R_FUNCTION { functionRestriction: _ }, classDef: _, cmt: _, info: _ }, opname) => AbsynUtil::joinPaths(Arc::new(Absyn::Path::IDENT { name: (opname.clone()).clone() }), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outName)
}

fn makeTypeVarElement(r#str: ArcStr, info: SourceInfo) -> Arc<SCode::Element> {
    let mut elt: Arc<SCode::Element>;
    let mut cd: Arc<SCode::ClassDef>;
    let mut ts: Arc<Absyn::TypeSpec>;
    ts = Arc::new(Absyn::TypeSpec::TCOMPLEX { path: Arc::new(Absyn::Path::IDENT { name: (literal!("polymorphic")).clone() }), typeSpecs: list![Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Any")).clone() }), arrayDim: None })], arrayDim: None });
    cd = Arc::new(SCode::ClassDef::DERIVED { typeSpec: ts.clone(), modifications: Arc::new(crate::SCode::Mod::NOMOD), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: crate::SCode::ConnectorType::POTENTIAL, parallelism: crate::SCode::Parallelism::NON_PARALLEL, variability: crate::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD } });
    elt = Arc::new(SCode::Element::CLASS { name: (r#str.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: crate::SCode::Visibility::PUBLIC, redeclarePrefix: crate::SCode::Redeclare::NOT_REDECLARE, finalPrefix: crate::SCode::Final::FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE) }), encapsulatedPrefix: crate::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: crate::SCode::Partial::NOT_PARTIAL, restriction: crate::SCode::Restriction::R_TYPE, classDef: cd.clone(), cmt: SCode::noComment.clone(), info: info.clone() });
    elt
}

fn setHasInnerOuterDefinitionsHandler(io: Absyn::InnerOuter) -> () {
    let _ = (match io.clone() {
        Absyn::NOT_INNER_OUTER => (),
        _ => {
            System::setHasInnerOuterDefinitions(true);
            ()
        },
    });
    ()
}

fn setHasStreamConnectorsHandler(streamPrefix: bool) -> () {
    let _ = (match streamPrefix.clone() {
        false => (),
        true => {
            System::setHasStreamConnectors(true);
            ()
        },
    });
    ()
}

pub fn translateAbsyn2SCode(inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outProgram: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outProgram = (match inProgram.clone() {
        _ => {
            let mut spInitial: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut inClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>;
            let mut initialClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>;
            InstHashTable::init()?;
            let Absyn::PROGRAM { classes: __pa0, .. } = (MetaUtil::createMetaClassesInProgram(inProgram.clone())?) else { bail!("pattern mismatch") };
            inClasses = __pa0.clone();
            System::setHasInnerOuterDefinitions(false);
            System::setHasExpandableConnectors(false);
            System::setHasOverconstrainedConnectors(false);
            System::setHasStreamConnectors(false);
            sp = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for c in (inClasses.clone()).into_iter().cloned() {
            let __x = translateClass(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            sp.clone()
        },
    });
    Ok(outProgram)
}

fn translateAlgBranches(inBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>> {
    let mut outBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
    let mut condition: Arc<Absyn::Exp>;
    let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
    outBranches = {
        let mut __acc: Arc<metamodelica::List<(_, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for branch in (inBranches.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(branch.clone()) {
        (condition, body) => (condition.clone(), translateClassdefAlgorithmitems(body.clone())?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outBranches)
}

fn translateAlternativeExternalAnnotation(decl: Option<Arc<SCode::ExternalDecl>>, comment: Arc<SCode::Comment>, info: SourceInfo) -> Result<Option<Arc<SCode::ExternalDecl>>> {
    fn whitelist_mod(submod: Arc<SCode::SubMod>) -> bool {
        let mut keep: bool;
        keep = (::match_deref::match_deref! { match &(submod.ident.clone()) {
        Deref @ "Library" => true,
        Deref @ "Include" => true,
        Deref @ "LibraryDirectory" => true,
        Deref @ "SourceDirectory" => true,
        Deref @ "License" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        keep
    }

    let mut outDecl: Option<Arc<SCode::ExternalDecl>>;
    let mut ext_decl: Arc<SCode::ExternalDecl>;
    let mut ann: Arc<SCode::Annotation>;
    outDecl = (::match_deref::match_deref! { match &((decl.clone(), comment.clone())) {
        (Some(ext_decl @ SCode::EXTERNALDECL { annotation_: None, .. }), Deref @ SCode::COMMENT { annotation_: Some(ann), .. }) => {
            let mut ext_decl = (*ext_decl).clone();
            let mut ann = (*ann).clone();
            ann.modification = SCodeUtil::filterSubMods(ann.modification.clone(), Arc::new(fnptr!(whitelist_mod, Arc<SCode::SubMod>))); // TODO: unhandled field-assign shape
            if !(SCodeUtil::isEmptyMod(ann.modification.clone())) {
                if Config::languageStandardAtLeast(Config::LanguageStandard::_3_3.clone())? {
                    Error::addSourceMessage(Error::MISPLACED_EXTERNAL_ANNOTATION.clone(), metamodelica::nil(), info.clone())?;
                }
                assign_field!(ext_decl.annotation_ = Some(ann.clone()));
            }
            Some(ext_decl.clone())
        },
        _ => decl.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDecl)
}

pub fn translateAnnotation(inAnnotation: Arc<Absyn::Annotation>) -> Result<Option<Arc<SCode::Annotation>>> {
    let mut outAnnotation: Option<Arc<SCode::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match &(inAnnotation.clone()) {
        Deref @ Absyn::ANNOTATION { elementArgs: Deref @ metamodelica::List::Nil } => None,
        Deref @ Absyn::ANNOTATION { elementArgs: args } => {
            let mut m: Arc<SCode::Mod>;
            m = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), crate::SCode::Final::NOT_FINAL, crate::SCode::Each::NOT_EACH, None, AbsynUtil::dummyInfo.clone(), true)?;
            if (SCodeUtil::isEmptyMod(m.clone())) {None} else {Some(Arc::new(SCode::Annotation { modification: m.clone() }))}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAnnotation)
}

pub fn translateAnnotationOpt(absynAnnotation: Option<Arc<Absyn::Annotation>>) -> Result<Option<Arc<SCode::Annotation>>> {
    let mut scodeAnnotation: Option<Arc<SCode::Annotation>>;
    scodeAnnotation = (match absynAnnotation.clone() {
        Some(mut ann) => translateAnnotation(ann.clone())?,
        _ => None,
    });
    Ok(scodeAnnotation)
}

fn translateArgs(args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, keepEmpty: bool) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
    let mut subMods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let mut smod: Arc<SCode::Mod>;
    let mut elem: Arc<SCode::Element>;
    let mut sub: Arc<SCode::SubMod>;
    let mut cr1: Arc<Absyn::ComponentRef>;
    let mut cr2: Arc<Absyn::ComponentRef>;
    let mut name: ArcStr;
    let mut s: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    for arg in &*args.clone() {
        subMods = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::MODIFICATION { .. } => {
            smod = translateMod(var_field!(arg.modification, Absyn::ElementArg::MODIFICATION).clone(), SCodeUtil::boolFinal(var_field!(arg.finalPrefix, Absyn::ElementArg::MODIFICATION).clone()), translateEach(var_field!(arg.eachPrefix, Absyn::ElementArg::MODIFICATION).clone())?, var_field!(arg.comment, Absyn::ElementArg::MODIFICATION).clone(), var_field!(arg.info, Absyn::ElementArg::MODIFICATION).clone(), false)?;
            if !(SCodeUtil::isEmptyMod(smod.clone())) || keepEmpty.clone() {
                sub = translateSub(var_field!(arg.path, Absyn::ElementArg::MODIFICATION).clone(), smod.clone(), var_field!(arg.info, Absyn::ElementArg::MODIFICATION).clone())?;
                subMods = cons(sub.clone(), subMods.clone());
            }
            subMods.clone()
        },
        Deref @ Absyn::REDECLARATION { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(translateElementspec(var_field!(arg.constrainClass, Absyn::ElementArg::REDECLARATION).clone(), var_field!(arg.finalPrefix, Absyn::ElementArg::REDECLARATION).clone(), openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, Some(var_field!(arg.redeclareKeywords, Absyn::ElementArg::REDECLARATION).clone()), crate::SCode::Visibility::PUBLIC, var_field!(arg.elementSpec, Absyn::ElementArg::REDECLARATION).clone(), var_field!(arg.info, Absyn::ElementArg::REDECLARATION).clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elem = __pa0.clone();
            sub = Arc::new(SCode::SubMod { ident: (AbsynUtil::elementSpecName(var_field!(arg.elementSpec, Absyn::ElementArg::REDECLARATION).clone())?).clone(), r#mod: Arc::new(SCode::Mod::REDECL { finalPrefix: SCodeUtil::boolFinal(var_field!(arg.finalPrefix, Absyn::ElementArg::REDECLARATION).clone()), eachPrefix: translateEach(var_field!(arg.eachPrefix, Absyn::ElementArg::REDECLARATION).clone())?, element: elem.clone() }) });
            cons(sub.clone(), subMods.clone())
        },
        Deref @ Absyn::ELEMENTARGCOMMENT { .. } => subMods.clone(),
        Deref @ Absyn::INHERITANCEBREAK { cnct: Deref @ Absyn::EQ_CONNECT { connector2: Deref @ Absyn::ComponentRef::CREF_IDENT { name, .. }, connector1: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "break", .. } }, .. } => cons(Arc::new(SCode::SubMod { ident: name.clone(), r#mod: Arc::new(SCode::Mod::BREAK_COMPONENT { info: var_field!(arg.info, Absyn::ElementArg::INHERITANCEBREAK).clone() }) }), subMods.clone()),
        Deref @ Absyn::INHERITANCEBREAK { cnct: Deref @ Absyn::EQ_CONNECT { connector2: cr2, connector1: cr1 }, .. } => cons(Arc::new(SCode::SubMod { ident: (literal!("")).clone(), r#mod: Arc::new(SCode::Mod::BREAK_CONNECT { lhs: Arc::new(cr1.clone()), rhs: Arc::new(cr2.clone()), info: var_field!(arg.info, Absyn::ElementArg::INHERITANCEBREAK).clone() }) }), subMods.clone()),
        _ => bail!("match: no arm matched"),
    } });
    }
    subMods = subMods.clone().reverse();
    Ok(subMods)
}

fn translateAttributes(inEA: Absyn::ElementAttributes, extraArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<SCode::Attributes> {
    let mut outA: SCode::Attributes;
    outA = (::match_deref::match_deref! { match &((inEA.clone(), extraArrayDim.clone())) {
        (Absyn::ATTR { flowPrefix: f, streamPrefix: s, parallelism: p, variability: v, direction: dir, isField: fi, arrayDim: adim }, extraADim) => {
            let mut ct: SCode::ConnectorType;
            let mut sp: SCode::Parallelism;
            let mut sv: SCode::Variability;
            let mut adim = (*adim).clone();
            ct = translateConnectorType(f.clone(), s.clone())?;
            sv = translateVariability(v.clone())?;
            sp = translateParallelism(p.clone())?;
            adim = listAppend(extraADim.clone(), adim.clone());
            SCode::Attributes { arrayDims: adim.clone(), connectorType: ct.clone(), parallelism: sp.clone(), variability: sv.clone(), direction: dir.clone(), isField: fi.clone() }
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outA)
}

pub fn translateClass(inClass: Arc<Absyn::Class>) -> Result<Arc<SCode::Element>> {
    let mut outClass: Arc<SCode::Element>;
    outClass = translateClass2(inClass.clone(), Error::getNumMessages())?;
    Ok(outClass)
}

fn translateClass2(inClass: Arc<Absyn::Class>, inNumMessages: i32) -> Result<Arc<SCode::Element>> {
    let mut outClass: Arc<SCode::Element>;
    outClass = 'mc: {
        let __mc_input = (inClass.clone(), inNumMessages.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c @ Deref @ Absyn::CLASS { info: file_info, body: d, restriction: r, encapsulatedPrefix: e, finalPrefix: f, partialPrefix: p, name: n, .. }, _) => {
                    let mut d_1: Arc<SCode::ClassDef>;
                    let mut r_1: SCode::Restriction;
                    let mut scodeClass: Arc<SCode::Element>;
                    let mut sFin: SCode::Final;
                    let mut sEnc: SCode::Encapsulated;
                    let mut sPar: SCode::Partial;
                    let mut cmt: Arc<SCode::Comment>;
                    r_1 = translateRestriction(c.clone(), r.clone())?;
                    (d_1, cmt) = translateClassdef(d.clone(), file_info.clone(), r_1.clone())?;
                    sFin = SCodeUtil::boolFinal(f.clone());
                    sEnc = SCodeUtil::boolEncapsulated(e.clone());
                    sPar = SCodeUtil::boolPartial(p.clone());
                    scodeClass = Arc::new(SCode::Element::CLASS { name: (n.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: crate::SCode::Visibility::PUBLIC, redeclarePrefix: crate::SCode::Redeclare::NOT_REDECLARE, finalPrefix: sFin.clone(), innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE) }), encapsulatedPrefix: sEnc.clone(), partialPrefix: sPar.clone(), restriction: r_1.clone(), classDef: d_1.clone(), cmt: cmt.clone(), info: file_info.clone() });
                    Ok(scodeClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::CLASS { info: file_info, name: n, .. }, _) => {
                    let mut d_1: Arc<SCode::ClassDef>;
                    let mut r_1: SCode::Restriction;
                    let mut c: Arc<Absyn::Class>;
                    let mut p: bool;
                    let mut f: bool;
                    let mut e: bool;
                    let mut r: Absyn::Restriction;
                    let mut d: Arc<Absyn::ClassDef>;
                    let mut scodeClass: Arc<SCode::Element>;
                    let mut sFin: SCode::Final;
                    let mut sEnc: SCode::Encapsulated;
                    let mut sPar: SCode::Partial;
                    let mut cmt: Arc<SCode::Comment>;
                    let mut n = (*n).clone();
                    let true = (intEq(Error::getNumMessages(), inNumMessages.clone())) else { bail!("pattern mismatch") };
                    n = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AbsynToSCode.translateClass2 failed: ")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(n.clone()).clone()], file_info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

fn translateClassdef(inClassDef: Arc<Absyn::ClassDef>, info: SourceInfo, re: SCode::Restriction) -> Result<(Arc<SCode::ClassDef>, Arc<SCode::Comment>)> {
    let mut outClassDef: Arc<SCode::ClassDef>;
    let mut outComment: Arc<SCode::Comment>;
    (outClassDef, outComment) = (::match_deref::match_deref! { match &((inClassDef.clone(), info.clone())) {
        (Deref @ Absyn::DERIVED { comment: cmt, arguments: a, attributes: attr, typeSpec: t }, _) => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut cmod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmtString: Option<ArcStr>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut lst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut path: Arc<Absyn::Path>;
            let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut scodeAttr: SCode::Attributes;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            checkTypeSpec(t.clone(), info.clone())?;
            r#mod = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: a.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), crate::SCode::Final::NOT_FINAL, crate::SCode::Each::NOT_EACH, None, info.clone(), false)?;
            scodeAttr = translateAttributes(attr.clone(), metamodelica::nil())?;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::DERIVED { typeSpec: t.clone(), modifications: r#mod.clone(), attributes: scodeAttr.clone() }), scodeCmt.clone())
        },
        (Deref @ Absyn::PARTS { comment: cmtString, ann, classParts: parts, classAttrs, typeVars }, _) => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut attr: Absyn::ElementAttributes;
            let mut a: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmt: Option<Arc<Absyn::Comment>>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut lst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut path: Arc<Absyn::Path>;
            let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut scodeAttr: SCode::Attributes;
            let mut typeVars = (*typeVars).clone();
            typeVars = (match re.clone() {
        SCode::R_METARECORD { .. } => List::union(typeVars.clone(), var_field!(re.typeVars, SCode::Restriction::R_METARECORD).clone()),
        SCode::R_UNIONTYPE { .. } => List::union(typeVars.clone(), var_field!(re.typeVars, SCode::Restriction::R_UNIONTYPE).clone()),
        _ => typeVars.clone(),
    });
            tvels = List::map1(typeVars.clone(), Arc::new(fnptr!(makeTypeVarElement, ArcStr, SourceInfo)), info.clone());
            els = translateClassdefElements(parts.clone())?;
            els = listAppend(tvels.clone(), els.clone());
            eqs = translateClassdefEquations(parts.clone())?;
            initeqs = translateClassdefInitialequations(parts.clone())?;
            als = translateClassdefAlgorithms(parts.clone())?;
            initals = translateClassdefInitialalgorithms(parts.clone())?;
            cos = translateClassdefConstraints(parts.clone())?;
            scodeCmt = translateCommentList(ann.clone(), cmtString.clone())?;
            decl = translateClassdefExternaldecls(parts.clone())?;
            decl = translateAlternativeExternalAnnotation(decl.clone(), scodeCmt.clone(), info.clone())?;
            (Arc::new(SCode::ClassDef::PARTS { elementLst: els.clone(), normalEquationLst: eqs.clone(), initialEquationLst: initeqs.clone(), normalAlgorithmLst: als.clone(), initialAlgorithmLst: initals.clone(), constraintLst: cos.clone(), clsattrs: classAttrs.clone(), externalDecl: decl.clone() }), scodeCmt.clone())
        },
        (Deref @ Absyn::ENUMERATION { enumLiterals: Deref @ Absyn::ENUMLITERALS { enumLiterals: lst }, comment: cmt }, _) => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut attr: Absyn::ElementAttributes;
            let mut a: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmtString: Option<ArcStr>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut path: Arc<Absyn::Path>;
            let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut scodeAttr: SCode::Attributes;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            lst_1 = translateEnumlist(lst.clone())?;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::ENUMERATION { enumLst: lst_1.clone() }), scodeCmt.clone())
        },
        (Deref @ Absyn::ENUMERATION { enumLiterals: Deref @ Absyn::ENUM_COLON, comment: cmt }, _) => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut attr: Absyn::ElementAttributes;
            let mut a: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmtString: Option<ArcStr>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut lst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut path: Arc<Absyn::Path>;
            let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut scodeAttr: SCode::Attributes;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::ENUMERATION { enumLst: metamodelica::nil() }), scodeCmt.clone())
        },
        (Deref @ Absyn::OVERLOAD { functionNames: pathLst, comment: cmt }, _) => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut attr: Absyn::ElementAttributes;
            let mut a: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmtString: Option<ArcStr>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut lst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut path: Arc<Absyn::Path>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut scodeAttr: SCode::Attributes;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::OVERLOAD { pathLst: pathLst.clone() }), scodeCmt.clone())
        },
        (Deref @ Absyn::CLASS_EXTENDS { parts, comment: cmtString, ann, modifications: cmod, .. }, _) => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut attr: Absyn::ElementAttributes;
            let mut a: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmt: Option<Arc<Absyn::Comment>>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut lst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut path: Arc<Absyn::Path>;
            let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut scodeAttr: SCode::Attributes;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            els = translateClassdefElements(parts.clone())?;
            eqs = translateClassdefEquations(parts.clone())?;
            initeqs = translateClassdefInitialequations(parts.clone())?;
            als = translateClassdefAlgorithms(parts.clone())?;
            initals = translateClassdefInitialalgorithms(parts.clone())?;
            cos = translateClassdefConstraints(parts.clone())?;
            r#mod = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: cmod.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), crate::SCode::Final::NOT_FINAL, crate::SCode::Each::NOT_EACH, None, AbsynUtil::dummyInfo.clone(), false)?;
            scodeCmt = translateCommentList(ann.clone(), cmtString.clone())?;
            decl = translateClassdefExternaldecls(parts.clone())?;
            decl = translateAlternativeExternalAnnotation(decl.clone(), scodeCmt.clone(), info.clone())?;
            (Arc::new(SCode::ClassDef::CLASS_EXTENDS { modifications: r#mod.clone(), composition: Arc::new(SCode::ClassDef::PARTS { elementLst: els.clone(), normalEquationLst: eqs.clone(), initialEquationLst: initeqs.clone(), normalAlgorithmLst: als.clone(), initialAlgorithmLst: initals.clone(), constraintLst: cos.clone(), clsattrs: metamodelica::nil(), externalDecl: decl.clone() }) }), scodeCmt.clone())
        },
        (Deref @ Absyn::PDER { comment: cmt, vars, functionName: path }, _) => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut attr: Absyn::ElementAttributes;
            let mut a: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmtString: Option<ArcStr>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut lst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut scodeAttr: SCode::Attributes;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            scodeCmt = translateComment(cmt.clone())?;
            (Arc::new(SCode::ClassDef::PDER { functionPath: path.clone(), derivedVariables: vars.clone() }), scodeCmt.clone())
        },
        _ => {
            let mut r#mod: Arc<SCode::Mod>;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut attr: Absyn::ElementAttributes;
            let mut a: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut cmt: Option<Arc<Absyn::Comment>>;
            let mut cmtString: Option<ArcStr>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut tvels: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut initeqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut initals: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut decl: Option<Arc<SCode::ExternalDecl>>;
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut lst_1: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut lst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            let mut scodeCmt: Arc<SCode::Comment>;
            let mut path: Arc<Absyn::Path>;
            let mut pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut scodeAttr: SCode::Attributes;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("AbsynToSCode.translateClassdef failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDef, outComment))
}

fn translateClassdefAlgorithmItem(inAlgorithm: Arc<Absyn::AlgorithmItem>) -> Result<Arc<SCode::Statement>> {
    let mut outStatement: Arc<SCode::Statement>;
    let mut absynComment: Option<Arc<Absyn::Comment>>;
    let mut comment: Arc<SCode::Comment>;
    let mut info: SourceInfo;
    let mut alg: Arc<Absyn::Algorithm>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inAlgorithm.clone()) {
        Deref @ Absyn::ALGORITHMITEM { info: __pa0, comment: __pa1, algorithm_: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    absynComment = __pa1.clone();
    alg = __pa2.clone();
    (comment, info) = translateCommentWithLineInfoChanges(absynComment.clone(), info.clone())?;
    outStatement = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::ALG_ASSIGN { .. } => Arc::new(SCode::Statement::ALG_ASSIGN { assignComponent: var_field!((*alg).assignComponent, Absyn::Algorithm::ALG_ASSIGN).clone(), value: var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), comment: comment.clone(), info: info.clone() }),
        Deref @ Absyn::ALG_IF { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut stmt: Arc<SCode::Statement>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).trueBranch, Absyn::Algorithm::ALG_IF).clone())?;
            else_body = translateClassdefAlgorithmitems(var_field!((*alg).elseBranch, Absyn::Algorithm::ALG_IF).clone())?;
            branches = translateAlgBranches(var_field!((*alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone())?;
            Arc::new(SCode::Statement::ALG_IF { boolExpr: var_field!((*alg).ifExp, Absyn::Algorithm::ALG_IF).clone(), trueBranch: body.clone(), elseIfBranch: branches.clone(), elseBranch: else_body.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ Absyn::ALG_FOR { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut stmt: Arc<SCode::Statement>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone())?;
            for i in &*var_field!((*alg).iterators, Absyn::Algorithm::ALG_FOR).clone().reverse() {
                (iter_name, iter_range) = translateIterator(i.clone(), info.clone())?;
                body = list![Arc::new(SCode::Statement::ALG_FOR { index: (iter_name.clone()).clone(), range: iter_range.clone(), forBody: body.clone(), comment: comment.clone(), info: info.clone() })];
            }
            listHead(body.clone())?
        },
        Deref @ Absyn::ALG_PARFOR { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut stmt: Arc<SCode::Statement>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).parforBody, Absyn::Algorithm::ALG_PARFOR).clone())?;
            for i in &*var_field!((*alg).iterators, Absyn::Algorithm::ALG_PARFOR).clone().reverse() {
                (iter_name, iter_range) = translateIterator(i.clone(), info.clone())?;
                body = list![Arc::new(SCode::Statement::ALG_PARFOR { index: (iter_name.clone()).clone(), range: iter_range.clone(), parforBody: body.clone(), comment: comment.clone(), info: info.clone() })];
            }
            listHead(body.clone())?
        },
        Deref @ Absyn::ALG_WHILE { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut stmt: Arc<SCode::Statement>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone())?;
            Arc::new(SCode::Statement::ALG_WHILE { boolExpr: var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), whileBody: body.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ Absyn::ALG_WHEN_A { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut stmt: Arc<SCode::Statement>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cr: Arc<Absyn::ComponentRef>;
            branches = translateAlgBranches(cons((var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHEN_A).clone(), var_field!((*alg).whenBody, Absyn::Algorithm::ALG_WHEN_A).clone()), var_field!((*alg).elseWhenAlgorithmBranch, Absyn::Algorithm::ALG_WHEN_A).clone()))?;
            Arc::new(SCode::Statement::ALG_WHEN_A { branches: branches.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ Absyn::ALG_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, functionCall: Deref @ Absyn::CREF_IDENT { name: Deref @ "assert", .. } } => Arc::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: ASSERTION_LEVEL_ERROR.clone(), comment: comment.clone(), info: info.clone() }),
        Deref @ Absyn::ALG_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Nil } } } }, functionCall: Deref @ Absyn::CREF_IDENT { name: Deref @ "assert", .. } } => Arc::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment.clone(), info: info.clone() }),
        Deref @ Absyn::ALG_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NAMEDARG { argName: Deref @ "level", argValue: e3 }, tail: Deref @ metamodelica::List::Nil }, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, functionCall: Deref @ Absyn::CREF_IDENT { name: Deref @ "assert", .. } } => Arc::new(SCode::Statement::ALG_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment.clone(), info: info.clone() }),
        Deref @ Absyn::ALG_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, functionCall: Deref @ Absyn::CREF_IDENT { name: Deref @ "terminate", .. } } => Arc::new(SCode::Statement::ALG_TERMINATE { message: e1.clone(), comment: comment.clone(), info: info.clone() }),
        Deref @ Absyn::ALG_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, functionCall: Deref @ Absyn::CREF_IDENT { name: Deref @ "reinit", .. } } => Arc::new(SCode::Statement::ALG_REINIT { cref: e1.clone(), newValue: e2.clone(), comment: comment.clone(), info: info.clone() }),
        Deref @ Absyn::ALG_NORETCALL { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut stmt: Arc<SCode::Statement>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cr: Arc<Absyn::ComponentRef>;
            e1 = Arc::new(Absyn::Exp::CALL { function_: var_field!((*alg).functionCall, Absyn::Algorithm::ALG_NORETCALL).clone(), functionArgs: var_field!((*alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), typeVars: metamodelica::nil() });
            Arc::new(SCode::Statement::ALG_NORETCALL { exp: e1.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ Absyn::ALG_FAILURE { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut stmt: Arc<SCode::Statement>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).equ, Absyn::Algorithm::ALG_FAILURE).clone())?;
            Arc::new(SCode::Statement::ALG_FAILURE { stmts: body.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ Absyn::ALG_TRY { .. } => {
            let mut body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut else_body: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut stmt: Arc<SCode::Statement>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateClassdefAlgorithmitems(var_field!((*alg).body, Absyn::Algorithm::ALG_TRY).clone())?;
            else_body = translateClassdefAlgorithmitems(var_field!((*alg).elseBody, Absyn::Algorithm::ALG_TRY).clone())?;
            Arc::new(SCode::Statement::ALG_TRY { body: body.clone(), elseBody: else_body.clone(), comment: comment.clone(), info: info.clone() })
        },
        Deref @ Absyn::ALG_RETURN => Arc::new(SCode::Statement::ALG_RETURN { comment: comment.clone(), info: info.clone() }),
        Deref @ Absyn::ALG_BREAK => Arc::new(SCode::Statement::ALG_BREAK { comment: comment.clone(), info: info.clone() }),
        Deref @ Absyn::ALG_CONTINUE => Arc::new(SCode::Statement::ALG_CONTINUE { comment: comment.clone(), info: info.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub fn translateClassdefAlgorithmitems(inStatements: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Statement>>>> {
    let mut outStatements: Arc<metamodelica::List<Arc<SCode::Statement>>>;
    outStatements = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for stmt in (inStatements.clone()).into_iter().cloned() {
            if !(AbsynUtil::isAlgorithmItem(stmt.clone())) { continue; }
            let __x = translateClassdefAlgorithmItem(stmt.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outStatements)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateClassdefAlgorithms(inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>> {
    let mut outAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
    outAlgorithmLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => metamodelica::nil(),
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ALGORITHMS { contents: al }, tail: rest } => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut als_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut al_1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut cp: Arc<Absyn::ClassPart>;
            al_1 = translateClassdefAlgorithmitems(al.clone())?;
            als = translateClassdefAlgorithms(rest.clone())?;
            als_1 = cons(Arc::new(SCode::AlgorithmSection { statements: al_1.clone() }), als.clone());
            als_1.clone()
        },
        Deref @ metamodelica::List::Cons { head: cp, tail: rest } => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut als_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut al_1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut al: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            als = translateClassdefAlgorithms(rest.clone())?;
            als.clone()
        },
        _ => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut als_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut al_1: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut al: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut rest: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut cp: Arc<Absyn::ClassPart>;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- AbsynToSCode.translateClassdefAlgorithms failed\\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAlgorithmLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateClassdefConstraints(inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<SCode::ConstraintSection>>> {
    let mut outConstraintLst: Arc<metamodelica::List<SCode::ConstraintSection>>;
    outConstraintLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => metamodelica::nil(),
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::CONSTRAINTS { contents: consts }, tail: rest } => {
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut cos_1: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut cp: Arc<Absyn::ClassPart>;
            cos = translateClassdefConstraints(rest.clone())?;
            cos_1 = cons(SCode::ConstraintSection { constraints: consts.clone() }, cos.clone());
            cos_1.clone()
        },
        Deref @ metamodelica::List::Cons { head: cp, tail: rest } => {
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut cos_1: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut consts: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            cos = translateClassdefConstraints(rest.clone())?;
            cos.clone()
        },
        _ => {
            let mut cos: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut cos_1: Arc<metamodelica::List<SCode::ConstraintSection>>;
            let mut consts: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut rest: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut cp: Arc<Absyn::ClassPart>;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- AbsynToSCode.translateClassdefConstraints failed\\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outConstraintLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn translateClassdefElements(inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outElementLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => metamodelica::nil(),
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::PUBLIC { contents: es }, tail: rest } => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut es_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut els_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            es_1 = translateEitemlist(es.clone(), crate::SCode::Visibility::PUBLIC)?;
            els = translateClassdefElements(rest.clone())?;
            els = listAppend(es_1.clone(), els.clone());
            els.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::PROTECTED { contents: es }, tail: rest } => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut es_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut els_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            es_1 = translateEitemlist(es.clone(), crate::SCode::Visibility::PROTECTED)?;
            els = translateClassdefElements(rest.clone())?;
            els = listAppend(es_1.clone(), els.clone());
            els.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => translateClassdefElements(rest.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElementLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateClassdefEquations(inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    outEquationLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => metamodelica::nil(),
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EQUATIONS { contents: eql }, tail: rest } => {
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eqs_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            eql_1 = translateEquations(eql.clone(), false)?;
            eqs = translateClassdefEquations(rest.clone())?;
            eqs_1 = listAppend(eqs.clone(), eql_1.clone());
            eqs_1.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eqs_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            eqs = translateClassdefEquations(rest.clone())?;
            eqs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEquationLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateClassdefExternaldecls(inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Option<Arc<SCode::ExternalDecl>>> {
    let mut outAbsynExternalDeclOption: Option<Arc<SCode::ExternalDecl>>;
    outAbsynExternalDeclOption = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EXTERNAL { externalDecl: Deref @ Absyn::EXTERNALDECL { funcName: fn_name, lang, output_, args, annotation_: aann }, .. }, tail: _ } => {
            let mut res: Option<Arc<SCode::ExternalDecl>>;
            let mut rest: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut sann: Option<Arc<SCode::Annotation>>;
            sann = translateAnnotationOpt(aann.clone())?;
            Some(Arc::new(SCode::ExternalDecl { funcName: fn_name.clone(), lang: lang.clone(), output_: output_.clone(), args: args.clone(), annotation_: sann.clone() }))
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut res: Option<Arc<SCode::ExternalDecl>>;
            let mut fn_name: Option<ArcStr>;
            let mut lang: Option<ArcStr>;
            let mut output_: Option<Arc<Absyn::ComponentRef>>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut aann: Option<Arc<Absyn::Annotation>>;
            let mut sann: Option<Arc<SCode::Annotation>>;
            res = translateClassdefExternaldecls(rest.clone())?;
            res.clone()
        },
        Deref @ metamodelica::List::Nil => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAbsynExternalDeclOption)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateClassdefInitialalgorithms(inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>> {
    let mut outAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
    outAlgorithmLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => metamodelica::nil(),
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::INITIALALGORITHMS { contents: al }, tail: rest } => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut als_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            stmts = translateClassdefAlgorithmitems(al.clone())?;
            als = translateClassdefInitialalgorithms(rest.clone())?;
            als_1 = cons(Arc::new(SCode::AlgorithmSection { statements: stmts.clone() }), als.clone());
            als_1.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut als: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut als_1: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>;
            let mut al: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            als = translateClassdefInitialalgorithms(rest.clone())?;
            als.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAlgorithmLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateClassdefInitialequations(inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    outEquationLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => metamodelica::nil(),
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::INITIALEQUATIONS { contents: eql }, tail: rest } => {
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eqs_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            eql_1 = translateEquations(eql.clone(), true)?;
            eqs = translateClassdefInitialequations(rest.clone())?;
            eqs_1 = listAppend(eqs.clone(), eql_1.clone());
            eqs_1.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            let mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eqs_1: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut eql: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            eqs = translateClassdefInitialequations(rest.clone())?;
            eqs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEquationLst)
}

fn translateComment(inComment: Option<Arc<Absyn::Comment>>) -> Result<Arc<SCode::Comment>> {
    let mut outComment: Arc<SCode::Comment>;
    outComment = (match inComment.clone() {
        None => SCode::noComment.clone(),
        Some(Absyn::COMMENT { annotation_: mut absann, comment: mut ostr }) => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut ostr = ostr.clone();
            ann = translateAnnotationOpt(absann.clone())?;
            ostr = Util::applyOption(ostr.clone(), Arc::new(fnptr!(System::unescapedString, ArcStr)));
            Arc::new(SCode::Comment { annotation_: ann.clone(), comment: ostr.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outComment)
}

fn translateCommentList(inAnns: Arc<metamodelica::List<Arc<Absyn::Annotation>>>, inString: Option<ArcStr>) -> Result<Arc<SCode::Comment>> {
    let mut outComment: Arc<SCode::Comment>;
    outComment = (::match_deref::match_deref! { match &((inAnns.clone(), inString.clone())) {
        (Deref @ metamodelica::List::Nil, _) => Arc::new(SCode::Comment { annotation_: None, comment: inString.clone() }),
        (Deref @ metamodelica::List::Cons { head: absann, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut anns: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut ostr: Option<ArcStr>;
            ann = translateAnnotation(absann.clone())?;
            ostr = Util::applyOption(inString.clone(), Arc::new(fnptr!(System::unescapedString, ArcStr)));
            Arc::new(SCode::Comment { annotation_: ann.clone(), comment: ostr.clone() })
        },
        (Deref @ metamodelica::List::Cons { head: absann, tail: anns }, _) => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut ostr: Option<ArcStr>;
            let mut absann = (*absann).clone();
            absann = AbsynUtil::mergeAnnotationsList(absann.clone(), anns.clone())?;
            ann = translateAnnotation(absann.clone())?;
            ostr = Util::applyOption(inString.clone(), Arc::new(fnptr!(System::unescapedString, ArcStr)));
            Arc::new(SCode::Comment { annotation_: ann.clone(), comment: ostr.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComment)
}

fn translateCommentSeparate(inComment: Option<Arc<Absyn::Comment>>) -> Result<(Option<Arc<SCode::Annotation>>, Option<ArcStr>)> {
    let mut outAnn: Option<Arc<SCode::Annotation>>;
    let mut outStr: Option<ArcStr>;
    (outAnn, outStr) = (match inComment.clone() {
        None => (None, None),
        Some(Absyn::COMMENT { annotation_: None, comment: None }) => (None, None),
        Some(Absyn::COMMENT { annotation_: None, comment: Some(mut r#str) }) => (None, Some((r#str.clone()).clone())),
        Some(Absyn::COMMENT { annotation_: Some(mut absann), comment: None }) => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut r#str: ArcStr;
            ann = translateAnnotation(absann.clone())?;
            (ann.clone(), None)
        },
        Some(Absyn::COMMENT { annotation_: Some(mut absann), comment: Some(mut r#str) }) => {
            let mut ann: Option<Arc<SCode::Annotation>>;
            ann = translateAnnotation(absann.clone())?;
            (ann.clone(), Some((r#str.clone()).clone()))
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outAnn, outStr))
}

fn translateCommentWithLineInfoChanges(inComment: Option<Arc<Absyn::Comment>>, inInfo: SourceInfo) -> Result<(Arc<SCode::Comment>, SourceInfo)> {
    let mut outComment: Arc<SCode::Comment>;
    let mut outInfo: SourceInfo;
    outComment = translateComment(inComment.clone())?;
    outInfo = getInfoAnnotationOrDefault(outComment.clone(), inInfo.clone())?;
    Ok((outComment, outInfo))
}

fn translateConnectorType(inFlow: bool, inStream: bool) -> Result<SCode::ConnectorType> {
    let mut outType: SCode::ConnectorType;
    outType = (match (inFlow.clone(), inStream.clone()) {
        (false, false) => crate::SCode::ConnectorType::POTENTIAL,
        (true, false) => crate::SCode::ConnectorType::FLOW,
        (false, true) => crate::SCode::ConnectorType::STREAM,
        (true, true) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("AbsynToSCode.translateConnectorType got both flow and stream prefix.")).clone()])?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outType)
}

fn translateConstrainClass(inConstrainClass: Option<Arc<Absyn::ConstrainClass>>) -> Result<Option<Arc<SCode::ConstrainClass>>> {
    let mut outConstrainClass: Option<Arc<SCode::ConstrainClass>>;
    outConstrainClass = (::match_deref::match_deref! { match &(inConstrainClass.clone()) {
        Some(Absyn::CONSTRAINCLASS { comment: cmt, elementSpec: Deref @ Absyn::EXTENDS { elementArg: eltargs, path: cc_path, .. } }) => {
            let mut cc_cmt: Arc<SCode::Comment>;
            let mut r#mod: Arc<Absyn::Modification>;
            let mut cc_mod: Arc<SCode::Mod>;
            r#mod = Arc::new(Absyn::Modification { elementArgLst: eltargs.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) });
            cc_mod = translateMod(Some(r#mod.clone()), crate::SCode::Final::NOT_FINAL, crate::SCode::Each::NOT_EACH, None, AbsynUtil::dummyInfo.clone(), false)?;
            cc_cmt = translateComment(cmt.clone())?;
            Some(Arc::new(SCode::ConstrainClass { constrainingClass: cc_path.clone(), modifier: cc_mod.clone(), comment: cc_cmt.clone() }))
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outConstrainClass)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateDefineunitParam(inArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, inArg: ArcStr) -> Result<Option<ArcStr>> {
    let mut expOpt: Option<ArcStr>;
    expOpt = (::match_deref::match_deref! { match &((inArgs.clone(), inArg.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NAMEDARG { argName: name, argValue: Deref @ Absyn::STRING { value: r#str } }, tail: _ }, arg) if (name.clone() == arg.clone()) => Some((r#str.clone()).clone()),
        (Deref @ metamodelica::List::Nil, _) => None,
        (Deref @ metamodelica::List::Cons { head: _, tail: args }, arg) => translateDefineunitParam(args.clone(), (arg.clone()).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(expOpt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateDefineunitParam2(inArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, inArg: ArcStr) -> Result<Option<f64>> {
    let mut weightOpt: Option<f64>;
    weightOpt = (::match_deref::match_deref! { match &((inArgs.clone(), inArg.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NAMEDARG { argName: name, argValue: Deref @ Absyn::REAL { value: s } }, tail: _ }, arg) if (name.clone() == arg.clone()) => {
            let mut r: f64;
            let mut args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            r = stringReal((s.clone()).clone())?;
            Some(r.clone())
        },
        (Deref @ metamodelica::List::Nil, _) => None,
        (Deref @ metamodelica::List::Cons { head: _, tail: args }, arg) => translateDefineunitParam2(args.clone(), (arg.clone()).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(weightOpt)
}

fn translateEach(inAEach: Absyn::Each) -> Result<SCode::Each> {
    let mut outSEach: SCode::Each;
    outSEach = (match inAEach.clone() {
        Absyn::EACH => crate::SCode::Each::EACH,
        Absyn::NON_EACH => crate::SCode::Each::NOT_EACH,
        _ => bail!("match: no arm matched"),
    });
    Ok(outSEach)
}

pub fn translateEitemlist(inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, inVisibility: SCode::Visibility) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut l: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut es: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = inAbsynElementItemLst.clone();
    let mut ei: Arc<Absyn::ElementItem>;
    let mut vis: SCode::Visibility;
    let mut e: Arc<Absyn::Element>;
    for ei in &*es.clone() {
        let _ = (::match_deref::match_deref! { match &(ei.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: e } => {
            let mut e_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            e_1 = translateElement(e.clone(), inVisibility.clone())?;
            l = List::append_reverse(e_1.clone(), l.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outElementLst = Dangerous::listReverseInPlace(l.clone());
    Ok(outElementLst)
}

pub fn translateElement(inElement: Arc<Absyn::Element>, inVisibility: SCode::Visibility) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outElementLst = (::match_deref::match_deref! { match &((inElement.clone(), inVisibility.clone())) {
        (Deref @ Absyn::ELEMENT { info, specification: s, redeclareKeywords: repl, innerOuter: io, finalPrefix: f, constrainClass: cc }, vis) => {
            let mut es: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut expOpt: Option<ArcStr>;
            let mut weightOpt: Option<f64>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut name: ArcStr;
            es = translateElementspec(cc.clone(), f.clone(), io.clone(), repl.clone(), vis.clone(), s.clone(), info.clone())?;
            es.clone()
        },
        (Deref @ Absyn::DEFINEUNIT { name, args, info }, vis) => {
            let mut es: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut f: bool;
            let mut repl: Option<Absyn::RedeclareKeywords>;
            let mut s: Arc<Absyn::ElementSpec>;
            let mut io: Absyn::InnerOuter;
            let mut cc: Option<Arc<Absyn::ConstrainClass>>;
            let mut expOpt: Option<ArcStr>;
            let mut weightOpt: Option<f64>;
            expOpt = translateDefineunitParam(args.clone(), (literal!("exp")).clone())?;
            weightOpt = translateDefineunitParam2(args.clone(), (literal!("weight")).clone())?;
            list![Arc::new(SCode::Element::DEFINEUNIT { name: (name.clone()).clone(), visibility: vis.clone(), exp: expOpt.clone(), weight: weightOpt.clone(), info: info.clone() })]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElementLst)
}

fn translateElementAddinfo(elem: Arc<SCode::Element>, nfo: SourceInfo) -> Arc<SCode::Element> {
    let mut oelem: Arc<SCode::Element>;
    oelem = (::match_deref::match_deref! { match &((elem.clone(), nfo.clone())) {
        (Deref @ SCode::COMPONENT { name: a1, prefixes: p, attributes: a6, typeSpec: a7, modifications: a8, comment: a10, condition: a11, info: _ }, _) => Arc::new(SCode::Element::COMPONENT { name: (a1.clone()).clone(), prefixes: p.clone(), attributes: a6.clone(), typeSpec: a7.clone(), modifications: a8.clone(), comment: a10.clone(), condition: a11.clone(), info: nfo.clone() }),
        _ => elem.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oelem
}

fn translateElementspec(cc: Option<Arc<Absyn::ConstrainClass>>, finalPrefix: bool, io: Absyn::InnerOuter, inRedeclareKeywords: Option<Absyn::RedeclareKeywords>, inVisibility: SCode::Visibility, inElementSpec4: Arc<Absyn::ElementSpec>, inInfo: SourceInfo) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outElementLst = (::match_deref::match_deref! { match &((cc.clone(), finalPrefix.clone(), io.clone(), inRedeclareKeywords.clone(), inVisibility.clone(), inElementSpec4.clone(), inInfo.clone())) {
        (_, _, _, repl, vis, Deref @ Absyn::CLASSDEF { class_: Deref @ Absyn::CLASS { info: i, body: de, restriction: Absyn::R_OPERATOR, encapsulatedPrefix: e, partialPrefix: pa, name: n, .. }, replaceable_: rp }, _) => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut fi: bool;
            let mut repl_1: bool;
            let mut fl: bool;
            let mut st: bool;
            let mut redecl: bool;
            let mut cl: Arc<Absyn::Class>;
            let mut re: Absyn::Restriction;
            let mut r#mod: Arc<SCode::Mod>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut d: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut attr: Absyn::ElementAttributes;
            let mut di: Absyn::Direction;
            let mut isf: Absyn::IsField;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut m: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut cmt: Arc<SCode::Comment>;
            let mut xs: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut imp: Absyn::Import;
            let mut cond: Option<Arc<Absyn::Exp>>;
            let mut path: Arc<Absyn::Path>;
            let mut absann: Arc<Absyn::Annotation>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut variability: Absyn::Variability;
            let mut parallelism: Absyn::Parallelism;
            let mut info: SourceInfo;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: Arc<SCode::Prefixes>;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            (de_1, cmt) = translateOperatorDef(de.clone(), (n.clone()).clone(), i.clone())?;
            (_, redecl) = translateRedeclarekeywords(repl.clone());
            sRed = SCodeUtil::boolRedeclare(redecl.clone());
            sFin = SCodeUtil::boolFinal(finalPrefix.clone());
            scc = translateConstrainClass(cc.clone())?;
            sRep = if (rp.clone()) {Arc::new(SCode::Replaceable::REPLACEABLE { cc: scc.clone() })} else {Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE)};
            sEnc = SCodeUtil::boolEncapsulated(e.clone());
            sPar = SCodeUtil::boolPartial(pa.clone());
            cls = Arc::new(SCode::Element::CLASS { name: (n.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: sRed.clone(), finalPrefix: sFin.clone(), innerOuter: io.clone(), replaceablePrefix: sRep.clone() }), encapsulatedPrefix: sEnc.clone(), partialPrefix: sPar.clone(), restriction: crate::SCode::Restriction::R_OPERATOR, classDef: de_1.clone(), cmt: cmt.clone(), info: i.clone() });
            list![cls.clone()]
        },
        (_, _, _, repl, vis, Deref @ Absyn::CLASSDEF { class_: cl @ Deref @ Absyn::CLASS { info: i, body: de, restriction: re, encapsulatedPrefix: e, partialPrefix: pa, name: n, .. }, replaceable_: rp }, _) => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut fi: bool;
            let mut repl_1: bool;
            let mut fl: bool;
            let mut st: bool;
            let mut redecl: bool;
            let mut r#mod: Arc<SCode::Mod>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut d: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut attr: Absyn::ElementAttributes;
            let mut di: Absyn::Direction;
            let mut isf: Absyn::IsField;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut m: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut cmt: Arc<SCode::Comment>;
            let mut xs: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut imp: Absyn::Import;
            let mut cond: Option<Arc<Absyn::Exp>>;
            let mut path: Arc<Absyn::Path>;
            let mut absann: Arc<Absyn::Annotation>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut variability: Absyn::Variability;
            let mut parallelism: Absyn::Parallelism;
            let mut info: SourceInfo;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: Arc<SCode::Prefixes>;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            re_1 = translateRestriction(cl.clone(), re.clone())?;
            (de_1, cmt) = translateClassdef(de.clone(), i.clone(), re_1.clone())?;
            (_, redecl) = translateRedeclarekeywords(repl.clone());
            sRed = SCodeUtil::boolRedeclare(redecl.clone());
            sFin = SCodeUtil::boolFinal(finalPrefix.clone());
            scc = translateConstrainClass(cc.clone())?;
            sRep = if (rp.clone()) {Arc::new(SCode::Replaceable::REPLACEABLE { cc: scc.clone() })} else {Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE)};
            sEnc = SCodeUtil::boolEncapsulated(e.clone());
            sPar = SCodeUtil::boolPartial(pa.clone());
            cls = Arc::new(SCode::Element::CLASS { name: (n.clone()).clone(), prefixes: Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: sRed.clone(), finalPrefix: sFin.clone(), innerOuter: io.clone(), replaceablePrefix: sRep.clone() }), encapsulatedPrefix: sEnc.clone(), partialPrefix: sPar.clone(), restriction: re_1.clone(), classDef: de_1.clone(), cmt: cmt.clone(), info: i.clone() });
            list![cls.clone()]
        },
        (_, _, _, _, vis, Deref @ Absyn::EXTENDS { annotationOpt: None, elementArg: args, path }, info) => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut rp: bool;
            let mut pa: bool;
            let mut fi: bool;
            let mut e: bool;
            let mut repl_1: bool;
            let mut fl: bool;
            let mut st: bool;
            let mut redecl: bool;
            let mut repl: Option<Absyn::RedeclareKeywords>;
            let mut cl: Arc<Absyn::Class>;
            let mut n: ArcStr;
            let mut re: Absyn::Restriction;
            let mut de: Arc<Absyn::ClassDef>;
            let mut r#mod: Arc<SCode::Mod>;
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut d: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut attr: Absyn::ElementAttributes;
            let mut di: Absyn::Direction;
            let mut isf: Absyn::IsField;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut m: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut cmt: Arc<SCode::Comment>;
            let mut xs: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut imp: Absyn::Import;
            let mut cond: Option<Arc<Absyn::Exp>>;
            let mut absann: Arc<Absyn::Annotation>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut variability: Absyn::Variability;
            let mut parallelism: Absyn::Parallelism;
            let mut i: SourceInfo;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: Arc<SCode::Prefixes>;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            r#mod = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), crate::SCode::Final::NOT_FINAL, crate::SCode::Each::NOT_EACH, None, AbsynUtil::dummyInfo.clone(), false)?;
            list![Arc::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: vis.clone(), modifications: r#mod.clone(), ann: None, info: info.clone() })]
        },
        (_, _, _, _, vis, Deref @ Absyn::EXTENDS { annotationOpt: Some(absann), elementArg: args, path }, info) => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut rp: bool;
            let mut pa: bool;
            let mut fi: bool;
            let mut e: bool;
            let mut repl_1: bool;
            let mut fl: bool;
            let mut st: bool;
            let mut redecl: bool;
            let mut repl: Option<Absyn::RedeclareKeywords>;
            let mut cl: Arc<Absyn::Class>;
            let mut n: ArcStr;
            let mut re: Absyn::Restriction;
            let mut de: Arc<Absyn::ClassDef>;
            let mut r#mod: Arc<SCode::Mod>;
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut d: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut attr: Absyn::ElementAttributes;
            let mut di: Absyn::Direction;
            let mut isf: Absyn::IsField;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut m: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut cmt: Arc<SCode::Comment>;
            let mut xs: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut imp: Absyn::Import;
            let mut cond: Option<Arc<Absyn::Exp>>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut variability: Absyn::Variability;
            let mut parallelism: Absyn::Parallelism;
            let mut i: SourceInfo;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: Arc<SCode::Prefixes>;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            r#mod = translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), crate::SCode::Final::NOT_FINAL, crate::SCode::Each::NOT_EACH, None, AbsynUtil::dummyInfo.clone(), false)?;
            ann = translateAnnotation(absann.clone())?;
            list![Arc::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: vis.clone(), modifications: r#mod.clone(), ann: ann.clone(), info: info.clone() })]
        },
        (_, _, _, _, _, Deref @ Absyn::COMPONENTS { components: Deref @ metamodelica::List::Nil, .. }, _) => metamodelica::nil(),
        (_, _, _, repl, vis, Deref @ Absyn::COMPONENTS { typeSpec: t, attributes: Absyn::ATTR { arrayDim: ad, isField: isf, direction: di, variability, parallelism, streamPrefix: st, flowPrefix: fl }, .. }, info) => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut rp: bool;
            let mut pa: bool;
            let mut fi: bool;
            let mut e: bool;
            let mut repl_1: bool;
            let mut redecl: bool;
            let mut cl: Arc<Absyn::Class>;
            let mut n: ArcStr;
            let mut re: Absyn::Restriction;
            let mut de: Arc<Absyn::ClassDef>;
            let mut r#mod: Arc<SCode::Mod>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut d: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut attr: Absyn::ElementAttributes;
            let mut m: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut cmt: Arc<SCode::Comment>;
            let mut xs: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut imp: Absyn::Import;
            let mut cond: Option<Arc<Absyn::Exp>>;
            let mut path: Arc<Absyn::Path>;
            let mut absann: Arc<Absyn::Annotation>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut i: SourceInfo;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: Arc<SCode::Prefixes>;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            let mut info = (*info).clone();
            xs_1 = metamodelica::nil();
            for comp in &*var_field!((*inElementSpec4).components, Absyn::ElementSpec::COMPONENTS).clone() {
                let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ Absyn::COMPONENTITEM { condition: __pa0, comment: __pa1, component: Absyn::COMPONENT { modification: __pa2, arrayDim: __pa3, name: __pa4 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                cond = __pa0.clone();
                comment = __pa1.clone();
                m = __pa2.clone();
                d = __pa3.clone();
                n = __pa4.clone();
                checkTypeSpec(t.clone(), info.clone())?;
                setHasInnerOuterDefinitionsHandler(io.clone());
                setHasStreamConnectorsHandler(st.clone());
                r#mod = translateMod(m.clone(), crate::SCode::Final::NOT_FINAL, crate::SCode::Each::NOT_EACH, None, info.clone(), false)?;
                prl1 = translateParallelism(parallelism.clone())?;
                var1 = translateVariability(variability.clone())?;
                tot_dim = listAppend(d.clone(), ad.clone());
                (repl_1, redecl) = translateRedeclarekeywords(repl.clone());
                (cmt, info) = translateCommentWithLineInfoChanges(comment.clone(), info.clone())?;
                sFin = SCodeUtil::boolFinal(finalPrefix.clone());
                sRed = SCodeUtil::boolRedeclare(redecl.clone());
                scc = translateConstrainClass(cc.clone())?;
                sRep = if (repl_1.clone()) {Arc::new(SCode::Replaceable::REPLACEABLE { cc: scc.clone() })} else {Arc::new(crate::SCode::Replaceable::NOT_REPLACEABLE)};
                ct = translateConnectorType(fl.clone(), st.clone())?;
                prefixes = Arc::new(SCode::Prefixes { visibility: vis.clone(), redeclarePrefix: sRed.clone(), finalPrefix: sFin.clone(), innerOuter: io.clone(), replaceablePrefix: sRep.clone() });
                xs_1 = (match di.clone() {
        Absyn::INPUT_OUTPUT if (!(Flags::isSet(Flags::SKIP_INPUT_OUTPUT_SYNTACTIC_SUGAR.clone())?)) => {
            let mut attr1: SCode::Attributes;
            let mut attr2: SCode::Attributes;
            let mut mod2: Arc<SCode::Mod>;
            let mut inName: ArcStr;
            inName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$in_")); __mm_s.push_str(&*n.clone()); ArcStr::from(__mm_s) }).clone();
            attr1 = SCode::Attributes { arrayDims: tot_dim.clone(), connectorType: ct.clone(), parallelism: prl1.clone(), variability: var1.clone(), direction: openmodelica_ast::Absyn::Direction::INPUT, isField: isf.clone() };
            attr2 = SCode::Attributes { arrayDims: tot_dim.clone(), connectorType: ct.clone(), parallelism: prl1.clone(), variability: var1.clone(), direction: openmodelica_ast::Absyn::Direction::OUTPUT, isField: isf.clone() };
            mod2 = Arc::new(SCode::Mod::MOD { finalPrefix: crate::SCode::Final::FINAL, eachPrefix: crate::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (inName.clone()).clone(), subscripts: metamodelica::nil() }) })), comment: None, info: info.clone() });
            cons(Arc::new(SCode::Element::COMPONENT { name: (n.clone()).clone(), prefixes: prefixes.clone(), attributes: attr2.clone(), typeSpec: t.clone(), modifications: mod2.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() }), cons(Arc::new(SCode::Element::COMPONENT { name: (inName.clone()).clone(), prefixes: prefixes.clone(), attributes: attr1.clone(), typeSpec: t.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() }), xs_1.clone()))
        },
        _ => cons(Arc::new(SCode::Element::COMPONENT { name: (n.clone()).clone(), prefixes: prefixes.clone(), attributes: SCode::Attributes { arrayDims: tot_dim.clone(), connectorType: ct.clone(), parallelism: prl1.clone(), variability: var1.clone(), direction: di.clone(), isField: isf.clone() }, typeSpec: t.clone(), modifications: r#mod.clone(), comment: cmt.clone(), condition: cond.clone(), info: info.clone() }), xs_1.clone()),
    });
            }
            xs_1 = Dangerous::listReverseInPlace(xs_1.clone());
            xs_1.clone()
        },
        (_, _, _, _, vis, Deref @ Absyn::IMPORT { info, import_: imp, .. }, _) => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut rp: bool;
            let mut pa: bool;
            let mut fi: bool;
            let mut e: bool;
            let mut repl_1: bool;
            let mut fl: bool;
            let mut st: bool;
            let mut redecl: bool;
            let mut repl: Option<Absyn::RedeclareKeywords>;
            let mut cl: Arc<Absyn::Class>;
            let mut n: ArcStr;
            let mut re: Absyn::Restriction;
            let mut de: Arc<Absyn::ClassDef>;
            let mut r#mod: Arc<SCode::Mod>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut d: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut attr: Absyn::ElementAttributes;
            let mut di: Absyn::Direction;
            let mut isf: Absyn::IsField;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut m: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut cmt: Arc<SCode::Comment>;
            let mut xs: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut cond: Option<Arc<Absyn::Exp>>;
            let mut path: Arc<Absyn::Path>;
            let mut absann: Arc<Absyn::Annotation>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut variability: Absyn::Variability;
            let mut parallelism: Absyn::Parallelism;
            let mut i: SourceInfo;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: Arc<SCode::Prefixes>;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            xs_1 = translateImports(imp.clone(), vis.clone(), info.clone());
            xs_1.clone()
        },
        _ => {
            let mut de_1: Arc<SCode::ClassDef>;
            let mut re_1: SCode::Restriction;
            let mut rp: bool;
            let mut pa: bool;
            let mut fi: bool;
            let mut e: bool;
            let mut repl_1: bool;
            let mut fl: bool;
            let mut st: bool;
            let mut redecl: bool;
            let mut repl: Option<Absyn::RedeclareKeywords>;
            let mut cl: Arc<Absyn::Class>;
            let mut n: ArcStr;
            let mut re: Absyn::Restriction;
            let mut de: Arc<Absyn::ClassDef>;
            let mut r#mod: Arc<SCode::Mod>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut xs_1: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut prl1: SCode::Parallelism;
            let mut var1: SCode::Variability;
            let mut tot_dim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut ad: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut d: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut attr: Absyn::ElementAttributes;
            let mut di: Absyn::Direction;
            let mut isf: Absyn::IsField;
            let mut t: Arc<Absyn::TypeSpec>;
            let mut m: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut cmt: Arc<SCode::Comment>;
            let mut xs: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut imp: Absyn::Import;
            let mut cond: Option<Arc<Absyn::Exp>>;
            let mut path: Arc<Absyn::Path>;
            let mut absann: Arc<Absyn::Annotation>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            let mut variability: Absyn::Variability;
            let mut parallelism: Absyn::Parallelism;
            let mut i: SourceInfo;
            let mut info: SourceInfo;
            let mut cls: Arc<SCode::Element>;
            let mut sRed: SCode::Redeclare;
            let mut sFin: SCode::Final;
            let mut sRep: Arc<SCode::Replaceable>;
            let mut sEnc: SCode::Encapsulated;
            let mut sPar: SCode::Partial;
            let mut vis: SCode::Visibility;
            let mut ct: SCode::ConnectorType;
            let mut prefixes: Arc<SCode::Prefixes>;
            let mut scc: Option<Arc<SCode::ConstrainClass>>;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("AbsynToSCode.translateElementspec failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElementLst)
}

fn translateEnumlist(inAbsynEnumLiteralLst: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Enum>>>> {
    let mut outEnumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
    outEnumLst = (::match_deref::match_deref! { match &(inAbsynEnumLiteralLst.clone()) {
        Deref @ metamodelica::List::Nil => metamodelica::nil(),
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ENUMLITERAL { literal: id, comment: cmtOpt }, tail: rest } => {
            let mut res: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut cmt: Arc<SCode::Comment>;
            cmt = translateComment(cmtOpt.clone())?;
            res = translateEnumlist(rest.clone())?;
            cons(Arc::new(SCode::Enum { literal: (id.clone()).clone(), comment: cmt.clone() }), res.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEnumLst)
}

fn translateEqBranch(inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), inIsInitial: bool) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)> {
    let mut outCondition: Arc<Absyn::Exp>;
    let mut outBody: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    let mut body: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    (outCondition, body) = inBranch.clone();
    outBody = translateEquations(body.clone(), inIsInitial.clone())?;
    Ok((outCondition, outBody))
}

fn translateEquation(inEquation: Arc<Absyn::Equation>, inComment: Arc<SCode::Comment>, inInfo: SourceInfo, inIsInitial: bool) -> Result<Arc<SCode::Equation>> {
    let mut outEquation: Arc<SCode::Equation>;
    outEquation = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ Absyn::EQ_IF { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut abody: Arc<metamodelica::List<Arc<Absyn::Equation>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut eq: Arc<SCode::Equation>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut bodies: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateEquations(var_field!((*inEquation).equationTrueItems, Absyn::Equation::EQ_IF).clone(), inIsInitial.clone())?;
            (conditions, bodies) = List::map1_2(var_field!((*inEquation).elseIfBranches, Absyn::Equation::EQ_IF).clone(), Arc::new(translateEqBranch), inIsInitial.clone());
            conditions = cons(var_field!((*inEquation).ifExp, Absyn::Equation::EQ_IF).clone(), conditions.clone());
            else_branch = translateEquations(var_field!((*inEquation).equationElseItems, Absyn::Equation::EQ_IF).clone(), inIsInitial.clone())?;
            Arc::new(SCode::Equation::EQ_IF { condition: conditions.clone(), thenBranch: cons(body.clone(), bodies.clone()), elseBranch: else_branch.clone(), comment: inComment.clone(), info: inInfo.clone() })
        },
        Deref @ Absyn::EQ_WHEN_E { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut abody: Arc<metamodelica::List<Arc<Absyn::Equation>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut eq: Arc<SCode::Equation>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut bodies: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateEquations(var_field!((*inEquation).whenEquations, Absyn::Equation::EQ_WHEN_E).clone(), inIsInitial.clone())?;
            (conditions, bodies) = List::map1_2(var_field!((*inEquation).elseWhenEquations, Absyn::Equation::EQ_WHEN_E).clone(), Arc::new(translateEqBranch), inIsInitial.clone());
            branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        for (c, b) in (&(conditions.clone())).into_iter().zip((&(bodies.clone())).into_iter()) {
            let __x = (c.clone(), b.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Arc::new(SCode::Equation::EQ_WHEN { condition: var_field!((*inEquation).whenExp, Absyn::Equation::EQ_WHEN_E).clone(), eEquationLst: body.clone(), elseBranches: branches.clone(), comment: inComment.clone(), info: inInfo.clone() })
        },
        Deref @ Absyn::EQ_EQUALS { .. } => Arc::new(SCode::Equation::EQ_EQUALS { expLeft: var_field!((*inEquation).leftSide, Absyn::Equation::EQ_EQUALS).clone(), expRight: var_field!((*inEquation).rightSide, Absyn::Equation::EQ_EQUALS).clone(), comment: inComment.clone(), info: inInfo.clone() }),
        Deref @ Absyn::EQ_PDE { .. } => Arc::new(SCode::Equation::EQ_PDE { expLeft: var_field!((*inEquation).leftSide, Absyn::Equation::EQ_PDE).clone(), expRight: var_field!((*inEquation).rightSide, Absyn::Equation::EQ_PDE).clone(), domain: var_field!((*inEquation).domain, Absyn::Equation::EQ_PDE).clone(), comment: inComment.clone(), info: inInfo.clone() }),
        Deref @ Absyn::EQ_CONNECT { .. } => Arc::new(SCode::Equation::EQ_CONNECT { crefLeft: var_field!((*inEquation).connector1, Absyn::Equation::EQ_CONNECT).clone(), crefRight: var_field!((*inEquation).connector2, Absyn::Equation::EQ_CONNECT).clone(), comment: inComment.clone(), info: inInfo.clone() }),
        Deref @ Absyn::EQ_FOR { .. } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut abody: Arc<metamodelica::List<Arc<Absyn::Equation>>>;
            let mut else_branch: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut body: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>;
            let mut iter_name: ArcStr;
            let mut iter_range: Option<Arc<Absyn::Exp>>;
            let mut eq: Arc<SCode::Equation>;
            let mut conditions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut bodies: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>;
            let mut cr: Arc<Absyn::ComponentRef>;
            body = translateEquations(var_field!((*inEquation).forEquations, Absyn::Equation::EQ_FOR).clone(), inIsInitial.clone())?;
            for i in &*var_field!((*inEquation).iterators, Absyn::Equation::EQ_FOR).clone().reverse() {
                (iter_name, iter_range) = translateIterator(i.clone(), inInfo.clone())?;
                body = list![Arc::new(SCode::Equation::EQ_FOR { index: (iter_name.clone()).clone(), range: iter_range.clone(), eEquationLst: body.clone(), comment: inComment.clone(), info: inInfo.clone() })];
            }
            listHead(body.clone())?
        },
        Deref @ Absyn::EQ_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, functionName: Deref @ Absyn::CREF_IDENT { name: Deref @ "assert", .. } } => Arc::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: ASSERTION_LEVEL_ERROR.clone(), comment: inComment.clone(), info: inInfo.clone() }),
        Deref @ Absyn::EQ_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Nil } } } }, functionName: Deref @ Absyn::CREF_IDENT { name: Deref @ "assert", .. } } => Arc::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: inComment.clone(), info: inInfo.clone() }),
        Deref @ Absyn::EQ_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NAMEDARG { argName: Deref @ "level", argValue: e3 }, tail: Deref @ metamodelica::List::Nil }, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, functionName: Deref @ Absyn::CREF_IDENT { name: Deref @ "assert", .. } } => Arc::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: inComment.clone(), info: inInfo.clone() }),
        Deref @ Absyn::EQ_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, functionName: Deref @ Absyn::CREF_IDENT { name: Deref @ "terminate", .. } } => Arc::new(SCode::Equation::EQ_TERMINATE { message: e1.clone(), comment: inComment.clone(), info: inInfo.clone() }),
        Deref @ Absyn::EQ_NORETCALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, functionName: Deref @ Absyn::CREF_IDENT { name: Deref @ "reinit", .. } } => Arc::new(SCode::Equation::EQ_REINIT { cref: e1.clone(), expReinit: e2.clone(), comment: inComment.clone(), info: inInfo.clone() }),
        Deref @ Absyn::EQ_NORETCALL { .. } => Arc::new(SCode::Equation::EQ_NORETCALL { exp: Arc::new(Absyn::Exp::CALL { function_: var_field!((*inEquation).functionName, Absyn::Equation::EQ_NORETCALL).clone(), functionArgs: var_field!((*inEquation).functionArgs, Absyn::Equation::EQ_NORETCALL).clone(), typeVars: metamodelica::nil() }), comment: inComment.clone(), info: inInfo.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEquation)
}

fn translateEquations(inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, inIsInitial: bool) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut outEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
    outEquationLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for eq in (inAbsynEquationItemLst.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EQUATIONITEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EQUATIONITEM { .. } => {
            let mut com: Arc<SCode::Comment>;
            let mut info: SourceInfo;
            (com, info) = translateCommentWithLineInfoChanges(var_field!(eq.comment, Absyn::EquationItem::EQUATIONITEM).clone(), var_field!(eq.info, Absyn::EquationItem::EQUATIONITEM).clone())?;
            translateEquation(var_field!(eq.equation_, Absyn::EquationItem::EQUATIONITEM).clone(), com.clone(), info.clone(), inIsInitial.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outEquationLst)
}

fn translateGroupImport(gimp: Absyn::GroupImport, prefix: Arc<Absyn::Path>, visibility: SCode::Visibility, info: SourceInfo) -> Result<Arc<SCode::Element>> {
    let mut elt: Arc<SCode::Element>;
    elt = (::match_deref::match_deref! { match &((gimp.clone(), prefix.clone(), visibility.clone(), info.clone())) {
        (Absyn::GROUP_IMPORT_NAME { name }, _, vis, _) => {
            let mut rename: ArcStr;
            let mut path: Arc<Absyn::Path>;
            path = AbsynUtil::joinPaths(prefix.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
            Arc::new(SCode::Element::IMPORT { imp: Absyn::Import::QUAL_IMPORT { path: path.clone() }, visibility: vis.clone(), info: info.clone() })
        },
        (Absyn::GROUP_IMPORT_RENAME { name, rename }, _, vis, _) => {
            let mut path: Arc<Absyn::Path>;
            path = AbsynUtil::joinPaths(prefix.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
            Arc::new(SCode::Element::IMPORT { imp: Absyn::Import::NAMED_IMPORT { name: (rename.clone()).clone(), path: path.clone() }, visibility: vis.clone(), info: info.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(elt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn translateImports(imp: Absyn::Import, visibility: SCode::Visibility, info: SourceInfo) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>>;
    elts = (::match_deref::match_deref! { match &((imp.clone(), visibility.clone(), info.clone())) {
        (Absyn::NAMED_IMPORT { name, path: Deref @ Absyn::FULLYQUALIFIED { path: p } }, _, _) => translateImports(Absyn::Import::NAMED_IMPORT { name: (name.clone()).clone(), path: p.clone() }, visibility.clone(), info.clone()),
        (Absyn::QUAL_IMPORT { path: Deref @ Absyn::FULLYQUALIFIED { path: p } }, _, _) => translateImports(Absyn::Import::QUAL_IMPORT { path: p.clone() }, visibility.clone(), info.clone()),
        (Absyn::UNQUAL_IMPORT { path: Deref @ Absyn::FULLYQUALIFIED { path: p } }, _, _) => translateImports(Absyn::Import::UNQUAL_IMPORT { path: p.clone() }, visibility.clone(), info.clone()),
        (Absyn::GROUP_IMPORT { groups, prefix: p }, _, _) => List::map3(groups.clone(), Arc::new(translateGroupImport), p.clone(), visibility.clone(), info.clone()),
        _ => list![Arc::new(SCode::Element::IMPORT { imp: imp.clone(), visibility: visibility.clone(), info: info.clone() })],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elts
}

fn translateIterator(inIterator: Arc<Absyn::ForIterator>, inInfo: SourceInfo) -> Result<(ArcStr, Option<Arc<Absyn::Exp>>)> {
    let mut outName: ArcStr;
    let mut outRange: Option<Arc<Absyn::Exp>>;
    let mut guard_exp: Option<Arc<Absyn::Exp>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inIterator.clone()) {
        Deref @ Absyn::ITERATOR { range: __pa0, guardExp: __pa1, name: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outRange = __pa0.clone();
    guard_exp = __pa1.clone();
    outName = __pa2.clone();
    if isSome(guard_exp.clone()) {
        Error::addSourceMessageAndFail(Error::INTERNAL_ERROR.clone(), list![(literal!("For loops with guards not yet implemented")).clone()], inInfo.clone())?;
    }
    Ok((outName, outRange))
}

pub fn translateMod(inMod: Option<Arc<Absyn::Modification>>, finalPrefix: SCode::Final, eachPrefix: SCode::Each, comment: Option<ArcStr>, info: SourceInfo, keepEmpty: bool) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod>;
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut eqmod: Arc<Absyn::EqMod>;
    let mut subs: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    let mut binding: Option<Arc<Absyn::Exp>>;
    (args, eqmod) = (match inMod.clone() {
        Some(Absyn::CLASSMOD { eqMod: ref eqmod, elementArgLst: ref args }) => (args.clone(), eqmod.clone()),
        _ => (metamodelica::nil(), Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD)),
    });
    subs = if (args.clone().is_empty()) {metamodelica::nil()} else {translateArgs(args.clone(), keepEmpty.clone())?};
    binding = (::match_deref::match_deref! { match &(eqmod.clone()) {
        Deref @ Absyn::EQMOD { .. } => Some(var_field!((*eqmod).exp, Absyn::EqMod::EQMOD).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod = (::match_deref::match_deref! { match &((subs.clone(), binding.clone(), finalPrefix.clone(), eachPrefix.clone())) {
        (Deref @ metamodelica::List::Nil, None, SCode::NOT_FINAL, SCode::NOT_EACH) => Arc::new(crate::SCode::Mod::NOMOD),
        _ => Arc::new(SCode::Mod::MOD { finalPrefix: finalPrefix.clone(), eachPrefix: eachPrefix.clone(), subModLst: subs.clone(), binding: binding.clone(), comment: comment.clone(), info: info.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

pub fn translateOperatorDef(inClassDef: Arc<Absyn::ClassDef>, operatorName: ArcStr, info: SourceInfo) -> Result<(Arc<SCode::ClassDef>, Arc<SCode::Comment>)> {
    let mut outOperDef: Arc<SCode::ClassDef>;
    let mut cmt: Arc<SCode::Comment>;
    (outOperDef, cmt) = (::match_deref::match_deref! { match &((inClassDef.clone(), operatorName.clone(), info.clone())) {
        (Deref @ Absyn::PARTS { comment: cmtString, ann: aann, classParts: parts, .. }, _, _) => {
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut scodeCmt: Option<Arc<SCode::Comment>>;
            let mut opName: ArcStr;
            let mut ann: Option<Arc<SCode::Annotation>>;
            els = translateClassdefElements(parts.clone())?;
            cmt = translateCommentList(aann.clone(), cmtString.clone())?;
            (Arc::new(SCode::ClassDef::PARTS { elementLst: els.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None }), cmt.clone())
        },
        _ => {
            let mut cmtString: Option<ArcStr>;
            let mut els: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut anns: Arc<metamodelica::List<Arc<SCode::Annotation>>>;
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut scodeCmt: Option<Arc<SCode::Comment>>;
            let mut opName: ArcStr;
            let mut aann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut ann: Option<Arc<SCode::Annotation>>;
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Could not translate operator to SCode because it is not using class parts.")).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outOperDef, cmt))
}

fn translateParallelism(inParallelism: Absyn::Parallelism) -> Result<SCode::Parallelism> {
    let mut outParallelism: SCode::Parallelism;
    outParallelism = (match inParallelism.clone() {
        Absyn::PARGLOBAL => crate::SCode::Parallelism::PARGLOBAL,
        Absyn::PARLOCAL => crate::SCode::Parallelism::PARLOCAL,
        Absyn::NON_PARALLEL => crate::SCode::Parallelism::NON_PARALLEL,
        _ => bail!("match: no arm matched"),
    });
    Ok(outParallelism)
}

fn translateRedeclarekeywords(inRedeclKeywords: Option<Absyn::RedeclareKeywords>) -> (bool, bool) {
    let mut outIsReplaceable: bool;
    let mut outIsRedeclared: bool;
    (outIsReplaceable, outIsRedeclared) = (match inRedeclKeywords.clone() {
        Some(Absyn::REDECLARE) => (false, true),
        Some(Absyn::REPLACEABLE) => (true, false),
        Some(Absyn::REDECLARE_REPLACEABLE) => (true, true),
        _ => (false, false),
    });
    (outIsReplaceable, outIsRedeclared)
}

pub fn translateRestriction(inClass: Arc<Absyn::Class>, inRestriction: Absyn::Restriction) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    outRestriction = (::match_deref::match_deref! { match &((inClass.clone(), inRestriction.clone())) {
        (d, Absyn::R_FUNCTION { functionRestriction: Absyn::FR_NORMAL_FUNCTION { purity } }) => if (containsExternalFuncDecl(d.clone())) {SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: purity.clone() } }} else {SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: purity.clone() } }},
        (_, Absyn::R_FUNCTION { functionRestriction: Absyn::FR_OPERATOR_FUNCTION }) => SCode::Restriction::R_FUNCTION { functionRestriction: crate::SCode::FunctionRestriction::FR_OPERATOR_FUNCTION },
        (_, Absyn::R_FUNCTION { functionRestriction: Absyn::FR_PARALLEL_FUNCTION }) => SCode::Restriction::R_FUNCTION { functionRestriction: crate::SCode::FunctionRestriction::FR_PARALLEL_FUNCTION },
        (_, Absyn::R_FUNCTION { functionRestriction: Absyn::FR_KERNEL_FUNCTION }) => SCode::Restriction::R_FUNCTION { functionRestriction: crate::SCode::FunctionRestriction::FR_KERNEL_FUNCTION },
        (_, Absyn::R_CLASS) => crate::SCode::Restriction::R_CLASS,
        (_, Absyn::R_OPTIMIZATION) => crate::SCode::Restriction::R_OPTIMIZATION,
        (_, Absyn::R_MODEL) => crate::SCode::Restriction::R_MODEL,
        (_, Absyn::R_RECORD) => SCode::Restriction::R_RECORD { isOperator: false },
        (_, Absyn::R_OPERATOR_RECORD) => SCode::Restriction::R_RECORD { isOperator: true },
        (_, Absyn::R_BLOCK) => crate::SCode::Restriction::R_BLOCK,
        (_, Absyn::R_CONNECTOR) => SCode::Restriction::R_CONNECTOR { isExpandable: false },
        (_, Absyn::R_EXP_CONNECTOR) => {
            let mut d: Arc<Absyn::Class>;
            let mut name: Arc<Absyn::Path>;
            let mut index: i32;
            let mut singleton: bool;
            let mut isImpure: bool;
            let mut moved: bool;
            let mut purity: Absyn::FunctionPurity;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            System::setHasExpandableConnectors(true);
            SCode::Restriction::R_CONNECTOR { isExpandable: true }
        },
        (_, Absyn::R_OPERATOR) => crate::SCode::Restriction::R_OPERATOR,
        (_, Absyn::R_TYPE) => crate::SCode::Restriction::R_TYPE,
        (_, Absyn::R_PACKAGE) => crate::SCode::Restriction::R_PACKAGE,
        (_, Absyn::R_ENUMERATION) => crate::SCode::Restriction::R_ENUMERATION,
        (_, Absyn::R_PREDEFINED_INTEGER) => crate::SCode::Restriction::R_PREDEFINED_INTEGER,
        (_, Absyn::R_PREDEFINED_REAL) => crate::SCode::Restriction::R_PREDEFINED_REAL,
        (_, Absyn::R_PREDEFINED_STRING) => crate::SCode::Restriction::R_PREDEFINED_STRING,
        (_, Absyn::R_PREDEFINED_BOOLEAN) => crate::SCode::Restriction::R_PREDEFINED_BOOLEAN,
        (_, Absyn::R_PREDEFINED_CLOCK) => crate::SCode::Restriction::R_PREDEFINED_CLOCK,
        (_, Absyn::R_PREDEFINED_ENUMERATION) => crate::SCode::Restriction::R_PREDEFINED_ENUMERATION,
        (_, Absyn::R_METARECORD { name, index, singleton, moved, typeVars }) => SCode::Restriction::R_METARECORD { name: name.clone(), index: index.clone(), singleton: singleton.clone(), moved: moved.clone(), typeVars: typeVars.clone() },
        (Deref @ Absyn::CLASS { body: Deref @ Absyn::PARTS { typeVars, .. }, .. }, Absyn::R_UNIONTYPE) => SCode::Restriction::R_UNIONTYPE { typeVars: typeVars.clone() },
        (_, Absyn::R_UNIONTYPE) => SCode::Restriction::R_UNIONTYPE { typeVars: metamodelica::nil() },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outRestriction)
}

fn translateSub(inPath: Arc<Absyn::Path>, inMod: Arc<SCode::Mod>, info: SourceInfo) -> Result<Arc<SCode::SubMod>> {
    let mut outSubMod: Arc<SCode::SubMod>;
    outSubMod = (::match_deref::match_deref! { match &((inPath.clone(), inMod.clone(), info.clone())) {
        (Deref @ Absyn::IDENT { name: i }, r#mod, _) => Arc::new(SCode::SubMod { ident: (i.clone()).clone(), r#mod: r#mod.clone() }),
        (Deref @ Absyn::QUALIFIED { path, name: i }, r#mod, _) => {
            let mut sub: Arc<SCode::SubMod>;
            let mut r#mod = (*r#mod).clone();
            sub = translateSub(path.clone(), r#mod.clone(), info.clone())?;
            r#mod = Arc::new(SCode::Mod::MOD { finalPrefix: crate::SCode::Final::NOT_FINAL, eachPrefix: crate::SCode::Each::NOT_EACH, subModLst: list![sub.clone()], binding: None, comment: None, info: info.clone() });
            Arc::new(SCode::SubMod { ident: (i.clone()).clone(), r#mod: r#mod.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubMod)
}

fn translateVariability(inVariability: Absyn::Variability) -> Result<SCode::Variability> {
    let mut outVariability: SCode::Variability;
    outVariability = (match inVariability.clone() {
        Absyn::VAR => crate::SCode::Variability::VAR,
        Absyn::DISCRETE => crate::SCode::Variability::DISCRETE,
        Absyn::PARAM => crate::SCode::Variability::PARAM,
        Absyn::CONST => crate::SCode::Variability::CONST,
        _ => bail!("match: no arm matched"),
    });
    Ok(outVariability)
}

