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

use openmodelica_ast::Absyn;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub type Mapping = Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>;

pub type Builtins = Arc<UnorderedMap::UnorderedMap<ArcStr, ElementType>>;

// Most builtin elements are not reserved keywords and can be shadowed by
// user elements. To try and avoid issues when we have e.g. a component named
// abs we keep track of what types of builtin elements we have and what type
// of element we're looking for with this enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum ElementType {
    TYPE = 1,
    FUNCTION = 2,
    TYPE_AND_FUNCTION = 3,
    OTHER = 4,
}
impl PartialOrd for ElementType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ElementType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub mapping: Mapping,
    pub builtins: Builtins,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            mapping: Default::default(),
            builtins: Default::default(),
        }
    }
}

pub type ENV = Env;


pub fn obfuscateProgram(mut program: Arc<metamodelica::List<Arc<SCode::Element>>>, mut classPath: Arc<Absyn::Path>, mut classComment: Arc<SCode::Comment>) -> Result<(Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<Absyn::Path>, Arc<SCode::Comment>, ArcStr, Mapping)> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = program;
    let mut classPath: Arc<Absyn::Path> = classPath;
    let mut classComment: Arc<SCode::Comment> = classComment;
    let mut mapStr: ArcStr = arcstr::literal!("");
    let mut mapping: Mapping;
    let mut builtins: Builtins;
    let mut env: Env;
    mapping = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), fnptr!(stringEqual, ArcStr, ArcStr), 1);
    builtins = makeBuiltins()?;
    env = Env { mapping: mapping.clone(), builtins: builtins.clone() };
    program = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (program.clone()).into_iter().cloned() {
            let __x = obfuscateElement(e.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    classPath = obfuscatePath(classPath.clone(), env.clone(), ElementType::TYPE.clone())?;
    classComment = obfuscateComment(classComment.clone(), env.clone());
    mapStr = (UnorderedMap::toJSON(env.mapping.clone(), fnptr!(Util::id, _), fnptr!(Util::id, _))?).clone();
    Ok((program, classPath, classComment, mapStr, mapping))
}

pub fn makeBuiltins() -> Result<Builtins> {
    let mut builtins: Builtins;
    let mut builtin_scode: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut etype: ElementType = ElementType::TYPE;
    builtins = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), fnptr!(stringEqual, ArcStr, ArcStr), 1);
    (_, builtin_scode) = FBuiltin::getInitialFunctions()?;
    for mut b in &*builtin_scode.clone() {
        let mut b = b.clone();
        etype = if (SCodeUtil::isFunction(b.clone())) {ElementType::FUNCTION.clone()} else {ElementType::TYPE.clone()};
        UnorderedMap::add((SCodeUtil::elementName(b.clone())?).clone(), etype.clone(), builtins.clone())?;
    }
    UnorderedMap::add((literal!("Boolean")).clone(), ElementType::TYPE.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("Clock")).clone(), ElementType::TYPE.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("Real")).clone(), ElementType::TYPE.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("Integer")).clone(), ElementType::TYPE_AND_FUNCTION.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("String")).clone(), ElementType::TYPE_AND_FUNCTION.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("displayUnit")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("fixed")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("max")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("min")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("nominal")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("quantity")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("start")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("stateSelect")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("time")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("unbounded")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("uncertain")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("unit")).clone(), ElementType::OTHER.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("constructor")).clone(), ElementType::FUNCTION.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("destructor")).clone(), ElementType::FUNCTION.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("$array")).clone(), ElementType::FUNCTION.clone(), builtins.clone())?;
    UnorderedMap::add((literal!("equalityConstraint")).clone(), ElementType::FUNCTION.clone(), builtins.clone())?;
    Ok(builtins)
}

pub fn obfuscateElement(mut element: Arc<SCode::Element>, mut env: Env) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::IMPORT { .. } => {
            assign_variant_field!(element => SCode::Element::IMPORT; imp = obfuscateImport(var_field!((*element).imp, SCode::Element::IMPORT).clone(), env.clone())?);
            ()
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS;
                baseClassPath = obfuscatePath(var_field!((*element).baseClassPath, SCode::Element::EXTENDS).clone(), env.clone(), ElementType::TYPE.clone())?,
                modifications = obfuscateMod(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), env.clone())?,
                ann = obfuscateAnnotationOpt(var_field!((*element).ann, SCode::Element::EXTENDS).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Element::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS;
                name = obfuscateIdentifier((var_field!((*element).name, SCode::Element::CLASS).clone()).clone(), env.clone(), ElementType::TYPE_AND_FUNCTION.clone())?.0,
                prefixes = obfuscatePrefixes(var_field!((*element).prefixes, SCode::Element::CLASS).clone(), env.clone())?,
                classDef = obfuscateClassDef(var_field!((*element).classDef, SCode::Element::CLASS).clone(), env.clone())?,
                cmt = obfuscateComment(var_field!((*element).cmt, SCode::Element::CLASS).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT;
                name = obfuscateIdentifier((var_field!((*element).name, SCode::Element::COMPONENT).clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0,
                prefixes = obfuscatePrefixes(var_field!((*element).prefixes, SCode::Element::COMPONENT).clone(), env.clone())?,
                attributes = obfuscateAttributes(var_field!((*element).attributes, SCode::Element::COMPONENT).clone(), env.clone())?,
                typeSpec = obfuscateTypeSpec(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone(), env.clone())?,
                modifications = obfuscateMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*element).comment, SCode::Element::COMPONENT).clone(), env.clone()),
                condition = obfuscateExpOpt(var_field!((*element).condition, SCode::Element::COMPONENT).clone(), env.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

pub fn obfuscateImport(mut imp: Absyn::Import, mut env: Env) -> Result<Absyn::Import> {
    let mut imp: Absyn::Import = imp;
    let () = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => {
            let __owned_variant_name_0 = obfuscateIdentifier((var_field!(imp.name, Absyn::Import::NAMED_IMPORT).clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0;
            let __owned_variant_path_1 = obfuscatePath(var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone(), env.clone(), ElementType::TYPE.clone())?;
            if let Absyn::Import::NAMED_IMPORT { name, path, .. } = &mut imp {
                *name = __owned_variant_name_0;
                *path = __owned_variant_path_1;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::NAMED_IMPORT"); }
            ()
        },
        Absyn::Import::QUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = obfuscatePath(var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(), env.clone(), ElementType::TYPE.clone())?;
            if let Absyn::Import::QUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::QUAL_IMPORT"); }
            ()
        },
        Absyn::Import::UNQUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = obfuscatePath(var_field!(imp.path, Absyn::Import::UNQUAL_IMPORT).clone(), env.clone(), ElementType::TYPE.clone())?;
            if let Absyn::Import::UNQUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::UNQUAL_IMPORT"); }
            ()
        },
        Absyn::Import::GROUP_IMPORT { .. } => {
            let __owned_variant_prefix_0 = obfuscatePath(var_field!(imp.prefix, Absyn::Import::GROUP_IMPORT).clone(), env.clone(), ElementType::TYPE.clone())?;
            let __owned_variant_groups_1 = {
        let mut __acc: Arc<metamodelica::List<Absyn::GroupImport>> = metamodelica::nil();
        for mut g in (var_field!(imp.groups, Absyn::Import::GROUP_IMPORT).clone()).into_iter().cloned() {
            let __x = obfuscateGroupImport(g.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            if let Absyn::Import::GROUP_IMPORT { prefix, groups, .. } = &mut imp {
                *prefix = __owned_variant_prefix_0;
                *groups = __owned_variant_groups_1;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::GROUP_IMPORT"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(imp)
}

pub fn obfuscateGroupImport(mut imp: Absyn::GroupImport, mut env: Env) -> Result<Absyn::GroupImport> {
    let mut imp: Absyn::GroupImport = imp;
    let () = (match imp.clone() {
        Absyn::GroupImport::GROUP_IMPORT_NAME { .. } => {
            let __owned_variant_name_0 = obfuscateIdentifier((var_field!(imp.name, Absyn::GroupImport::GROUP_IMPORT_NAME).clone()).clone(), env.clone(), ElementType::TYPE.clone())?.0;
            if let Absyn::GroupImport::GROUP_IMPORT_NAME { name, .. } = &mut imp {
                *name = __owned_variant_name_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::GroupImport::GROUP_IMPORT_NAME"); }
            ()
        },
        Absyn::GroupImport::GROUP_IMPORT_RENAME { .. } => {
            let __owned_variant_rename_0 = obfuscateIdentifier((var_field!(imp.rename, Absyn::GroupImport::GROUP_IMPORT_RENAME).clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0;
            let __owned_variant_name_1 = obfuscateIdentifier((var_field!(imp.name, Absyn::GroupImport::GROUP_IMPORT_RENAME).clone()).clone(), env.clone(), ElementType::TYPE.clone())?.0;
            if let Absyn::GroupImport::GROUP_IMPORT_RENAME { rename, name, .. } = &mut imp {
                *rename = __owned_variant_rename_0;
                *name = __owned_variant_name_1;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::GroupImport::GROUP_IMPORT_RENAME"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(imp)
}

pub fn obfuscateClassDef(mut cdef: Arc<SCode::ClassDef>, mut env: Env) -> Result<Arc<SCode::ClassDef>> {
    let mut cdef: Arc<SCode::ClassDef> = cdef;
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::PARTS;
                elementLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = obfuscateElement(e.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                normalEquationLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut e in (var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = obfuscateEquation(e.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                initialEquationLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut e in (var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = obfuscateEquation(e.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                normalAlgorithmLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = obfuscateAlgorithm(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                initialAlgorithmLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = obfuscateAlgorithm(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                externalDecl = Util::applyOption(var_field!((*cdef).externalDecl, SCode::ClassDef::PARTS).clone(), Arc::new({ let __pe_b1 = env.clone(); move |__pe_a0| obfuscateExternalDecl(__pe_a0, __pe_b1.clone()) }))
            );
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::CLASS_EXTENDS;
                modifications = obfuscateMod(var_field!((*cdef).modifications, SCode::ClassDef::CLASS_EXTENDS).clone(), env.clone())?,
                composition = obfuscateClassDef(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), env.clone())?
            );
            ()
        },
        Deref @ SCode::ClassDef::DERIVED { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::DERIVED;
                typeSpec = obfuscateTypeSpec(var_field!((*cdef).typeSpec, SCode::ClassDef::DERIVED).clone(), env.clone())?,
                modifications = obfuscateMod(var_field!((*cdef).modifications, SCode::ClassDef::DERIVED).clone(), env.clone())?,
                attributes = obfuscateAttributes(var_field!((*cdef).attributes, SCode::ClassDef::DERIVED).clone(), env.clone())?
            );
            ()
        },
        Deref @ SCode::ClassDef::ENUMERATION { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::ENUMERATION; enumLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Enum>>> = metamodelica::nil();
        for mut e in (var_field!((*cdef).enumLst, SCode::ClassDef::ENUMERATION).clone()).into_iter().cloned() {
            let __x = obfuscateEnum(e.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::ClassDef::OVERLOAD { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::OVERLOAD; pathLst = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut p in (var_field!((*cdef).pathLst, SCode::ClassDef::OVERLOAD).clone()).into_iter().cloned() {
            let __x = obfuscatePath(p.clone(), env.clone(), ElementType::TYPE.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ SCode::ClassDef::PDER { .. } => {
            assign_variant_field!(cdef => SCode::ClassDef::PDER;
                functionPath = obfuscatePath(var_field!((*cdef).functionPath, SCode::ClassDef::PDER).clone(), env.clone(), ElementType::FUNCTION.clone())?,
                derivedVariables = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (var_field!((*cdef).derivedVariables, SCode::ClassDef::PDER).clone()).into_iter().cloned() {
            let __x = (obfuscateIdentifier((v.clone()).clone(), env.clone(), ElementType::OTHER.clone())?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cdef)
}

pub fn obfuscateTypeSpec(mut ty: Arc<Absyn::TypeSpec>, mut env: Env) -> Result<Arc<Absyn::TypeSpec>> {
    let mut ty: Arc<Absyn::TypeSpec> = ty;
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { .. } => {
            assign_variant_field!(ty => Absyn::TypeSpec::TPATH;
                path = obfuscatePath(var_field!((*ty).path, Absyn::TypeSpec::TPATH).clone(), env.clone(), ElementType::TYPE.clone())?,
                arrayDim = obfuscateArrayDimsOpt(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone(), env.clone())
            );
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { .. } => {
            assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX;
                path = obfuscatePath(var_field!((*ty).path, Absyn::TypeSpec::TCOMPLEX).clone(), env.clone(), ElementType::TYPE.clone())?,
                typeSpecs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>> = metamodelica::nil();
        for mut t in (var_field!((*ty).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone()).into_iter().cloned() {
            let __x = obfuscateTypeSpec(t.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                arrayDim = obfuscateArrayDimsOpt(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone(), env.clone())
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ty)
}

pub fn obfuscateEnum(mut r#enum: Arc<SCode::Enum>, mut env: Env) -> Result<Arc<SCode::Enum>> {
    let mut r#enum: Arc<SCode::Enum> = r#enum;
    assign_field!(
        r#enum.literal = obfuscateIdentifier((r#enum.literal.clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0,
        r#enum.comment = obfuscateComment(r#enum.comment.clone(), env.clone())
    );
    Ok(r#enum)
}

pub fn obfuscatePrefixes(mut prefixes: Arc<SCode::Prefixes>, mut env: Env) -> Result<Arc<SCode::Prefixes>> {
    let mut prefixes: Arc<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.replaceablePrefix = obfuscateReplaceable(prefixes.replaceablePrefix.clone(), env.clone())?);
    Ok(prefixes)
}

pub fn obfuscateReplaceable(mut repl: Arc<SCode::Replaceable>, mut env: Env) -> Result<Arc<SCode::Replaceable>> {
    let mut repl: Arc<SCode::Replaceable> = repl;
    let mut cc: Arc<SCode::ConstrainClass>;
    let () = (::match_deref::match_deref! { match &(repl.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) } => {
            let mut cc = (*cc).clone();
            assign_field!(
                cc.constrainingClass = obfuscatePath(cc.constrainingClass.clone(), env.clone(), ElementType::OTHER.clone())?,
                cc.modifier = obfuscateMod(cc.modifier.clone(), env.clone())?,
                cc.comment = obfuscateComment(cc.comment.clone(), env.clone())
            );
            assign_variant_field!(repl => SCode::Replaceable::REPLACEABLE; cc = Some(cc.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(repl)
}

pub fn obfuscateAttributes(mut attributes: SCode::Attributes, mut env: Env) -> Result<SCode::Attributes> {
    let mut attributes: SCode::Attributes = attributes;
    attributes.arrayDims = obfuscateArrayDims(attributes.arrayDims.clone(), env.clone())?;
    Ok(attributes)
}

pub fn obfuscateMod(mut r#mod: Arc<SCode::Mod>, mut env: Env) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD;
                subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut s in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            let __x = obfuscateSubMod(s.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                binding = obfuscateExpOpt(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Mod::REDECL { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::REDECL; element = obfuscateElement(var_field!((*r#mod).element, SCode::Mod::REDECL).clone(), env.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn obfuscateSubMod(mut r#mod: Arc<SCode::SubMod>, mut env: Env) -> Result<Arc<SCode::SubMod>> {
    let mut r#mod: Arc<SCode::SubMod> = r#mod;
    assign_field!(
        r#mod.ident = obfuscateIdentifier((r#mod.ident.clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0,
        r#mod.r#mod = obfuscateMod(r#mod.r#mod.clone(), env.clone())?
    );
    Ok(r#mod)
}

pub fn obfuscatePath(mut path: Arc<Absyn::Path>, mut env: Env, mut etype: ElementType) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path> = path;
    let mut name: ArcStr = arcstr::literal!("");
    let () = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            (name, _) = obfuscateIdentifier((var_field!((*path).name, Absyn::Path::IDENT).clone()).clone(), env.clone(), etype.clone())?;
            if referenceEq(&name.clone(),&var_field!((*path).name, Absyn::Path::IDENT).clone()) {
                return Ok(path);
            }
            assign_variant_field!(path => Absyn::Path::IDENT; name = name.clone());
            ()
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            (name, _) = obfuscateIdentifier((var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone(), env.clone(), etype.clone())?;
            if referenceEq(&name.clone(),&var_field!((*path).name, Absyn::Path::QUALIFIED).clone()) {
                return Ok(path);
            }
            assign_variant_field!(path => Absyn::Path::QUALIFIED;
                name = name.clone(),
                path = obfuscatePath(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), env.clone(), etype.clone())?
            );
            ()
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
            assign_variant_field!(path => Absyn::Path::FULLYQUALIFIED; path = obfuscatePath(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), env.clone(), etype.clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(path)
}

pub fn obfuscateIdentifier(mut id: ArcStr, mut env: Env, mut etype: ElementType) -> Result<(ArcStr, ElementType)> {
    let mut outId: ArcStr = arcstr::literal!("");
    let mut foundType: ElementType = ElementType::TYPE;
    let mut builtins: Builtins = env.builtins.clone();
    let mut mapping: Mapping = env.mapping.clone();
    let mut opt_ety: Option<ElementType> = None;
    opt_ety = UnorderedMap::get((id.clone()).clone(), builtins.clone());
    if isSome(opt_ety.clone()) {
        let Some(__pa0) = (opt_ety.clone()) else { bail!("pattern mismatch") };
        foundType = __pa0.clone();
        if isBuiltinInContext(etype.clone(), foundType.clone()) {
            outId = (id.clone()).clone();
            return Ok((outId, foundType));
        }
    } else {
        foundType = ElementType::OTHER.clone();
    }
    outId = (UnorderedMap::addUpdate((id.clone()).clone(), Arc::new({ let __pe_b1 = UnorderedMap::size(mapping.clone()); move |__pe_a0| makeId(__pe_a0, __pe_b1.clone()) }), mapping.clone())?).clone();
    Ok((outId, foundType))
}

pub fn isBuiltinInContext(mut expectedType: ElementType, mut actualType: ElementType) -> bool {
    let mut res: bool = false;
    res = (match (expectedType.clone(), actualType.clone()) {
        (ElementType::TYPE { .. }, ElementType::TYPE { .. }) => true,
        (ElementType::TYPE { .. }, ElementType::TYPE_AND_FUNCTION) => true,
        (ElementType::FUNCTION { .. }, ElementType::FUNCTION { .. }) => true,
        (ElementType::FUNCTION { .. }, ElementType::TYPE_AND_FUNCTION) => true,
        (ElementType::TYPE_AND_FUNCTION, ElementType::TYPE { .. }) => true,
        (ElementType::TYPE_AND_FUNCTION, ElementType::FUNCTION { .. }) => true,
        (ElementType::TYPE_AND_FUNCTION, ElementType::TYPE_AND_FUNCTION) => true,
        (_, ElementType::TYPE { .. }) => true,
        (_, ElementType::OTHER) => true,
        _ => false,
    });
    res
}

pub fn makeId(mut oldId: Option<ArcStr>, mut index: i32) -> Result<ArcStr> {
    let mut id: ArcStr = arcstr::literal!("");
    if isSome(oldId.clone()) {
        let Some(__pa0) = (oldId.clone()) else { bail!("pattern mismatch") };
        id = __pa0.clone();
    } else {
        id = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); ArcStr::from(__mm_s) }).clone();
    }
    Ok(id)
}

pub fn obfuscateComment(mut comment: Arc<SCode::Comment>, mut env: Env) -> Arc<SCode::Comment> {
    let mut comment: Arc<SCode::Comment> = comment;
    assign_field!(
        comment.annotation_ = obfuscateAnnotationOpt(comment.annotation_.clone(), env.clone()),
        comment.comment = None
    );
    comment
}

pub fn obfuscateAnnotationOpt(mut ann: Option<Arc<SCode::Annotation>>, mut env: Env) -> Option<Arc<SCode::Annotation>> {
    let mut ann: Option<Arc<SCode::Annotation>> = ann;
    ann = Util::applyOption(ann.clone(), Arc::new({ let __pe_b1 = env.clone(); move |__pe_a0| obfuscateAnnotation(__pe_a0, __pe_b1.clone()) }));
    ann
}

pub fn obfuscateAnnotation(mut ann: Arc<SCode::Annotation>, mut env: Env) -> Result<Arc<SCode::Annotation>> {
    let mut ann: Arc<SCode::Annotation> = ann;
    assign_field!(ann.modification = obfuscateAnnotationMod(ann.modification.clone(), env.clone(), false, true)?);
    Ok(ann)
}

pub fn obfuscateAnnotationMod(mut r#mod: Arc<SCode::Mod>, mut env: Env, mut obfuscateName: bool, mut obfuscateBinding: bool) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut s in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            if !(isAllowedAnnotation(s.clone())) { continue; }
            let __x = obfuscateAnnotationSubMod(s.clone(), env.clone(), obfuscateName.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if obfuscateBinding.clone() {
                assign_variant_field!(r#mod => SCode::Mod::MOD; binding = obfuscateExpOpt(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), env.clone()));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn isAllowedAnnotation(mut r#mod: Arc<SCode::SubMod>) -> bool {
    let mut allowed: bool = false;
    allowed = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "Icon" => false,
        Deref @ "Diagram" => false,
        Deref @ "Dialog" => false,
        Deref @ "IconMap" => false,
        Deref @ "DiagramMap" => false,
        Deref @ "Placement" => false,
        Deref @ "Text" => false,
        Deref @ "Line" => false,
        Deref @ "defaultComponentName" => false,
        Deref @ "defaultComponentPrefixes" => false,
        Deref @ "missingInnerMessage" => false,
        Deref @ "obsolete" => false,
        Deref @ "unassignedMessage" => false,
        Deref @ "Protection" => false,
        Deref @ "Authorization" => false,
        _ => StringUtil::startsWith((r#mod.ident.clone()).clone(), (literal!("__OpenModelica")).clone()) || !(StringUtil::startsWith((r#mod.ident.clone()).clone(), (literal!("__")).clone())),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    allowed
}

pub fn obfuscateAnnotationSubMod(mut r#mod: Arc<SCode::SubMod>, mut env: Env, mut obfuscateName: bool) -> Result<Arc<SCode::SubMod>> {
    let mut r#mod: Arc<SCode::SubMod> = r#mod;
    let mut obfuscate_name: bool = false;
    let mut obfuscate_binding: bool = false;
    if obfuscateName.clone() {
        assign_field!(r#mod.ident = obfuscateIdentifier((r#mod.ident.clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0);
    }
    obfuscate_name = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "inverse" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    obfuscate_binding = (::match_deref::match_deref! { match &(r#mod.ident.clone()) {
        Deref @ "__OpenModelica_tearingSelect" => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    assign_field!(r#mod.r#mod = obfuscateAnnotationMod(r#mod.r#mod.clone(), env.clone(), obfuscate_name.clone(), obfuscate_binding.clone())?);
    Ok(r#mod)
}

pub fn obfuscateExpOpt(mut exp: Option<Arc<Absyn::Exp>>, mut env: Env) -> Option<Arc<Absyn::Exp>> {
    let mut exp: Option<Arc<Absyn::Exp>> = exp;
    exp = Util::applyOption(exp.clone(), Arc::new({ let __pe_b1 = env.clone(); move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone()) }));
    exp
}

pub fn obfuscateExp(mut exp: Arc<Absyn::Exp>, mut env: Env) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(exp.clone(), Arc::new(obfuscateExpTraverse), env.clone())?;
    Ok(exp)
}

pub fn obfuscateExpTraverse(mut exp: Arc<Absyn::Exp>, mut env: Env) -> Result<(Arc<Absyn::Exp>, Env)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut env: Env = env;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = obfuscateCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), env.clone(), ElementType::OTHER.clone(), false)?);
            ()
        },
        Deref @ Absyn::Exp::CALL { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CALL;
                functionArgs = obfuscateFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::CALL).clone(), var_field!((*exp).function_, Absyn::Exp::CALL).clone(), env.clone())?,
                function_ = obfuscateCref(var_field!((*exp).function_, Absyn::Exp::CALL).clone(), env.clone(), ElementType::FUNCTION.clone(), false)?
            );
            ()
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { .. } => {
            assign_variant_field!(exp => Absyn::Exp::PARTEVALFUNCTION;
                functionArgs = obfuscateFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::PARTEVALFUNCTION).clone(), var_field!((*exp).function_, Absyn::Exp::PARTEVALFUNCTION).clone(), env.clone())?,
                function_ = obfuscateCref(var_field!((*exp).function_, Absyn::Exp::PARTEVALFUNCTION).clone(), env.clone(), ElementType::OTHER.clone(), false)?
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, env))
}

pub fn obfuscateCref(mut cref: Arc<Absyn::ComponentRef>, mut env: Env, mut etype: ElementType, mut obfuscateSubs: bool) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut name: ArcStr = arcstr::literal!("");
    let mut ety: ElementType = ElementType::TYPE;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            (name, _) = obfuscateIdentifier((var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), env.clone(), etype.clone())?;
            if referenceEq(&name.clone(),&var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()) {
                return Ok(cref);
            }
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; name = name.clone());
            if obfuscateSubs.clone() {
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = obfuscateSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), env.clone())?);
            }
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            (name, ety) = obfuscateIdentifier((var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), env.clone(), etype.clone())?;
            if !(referenceEq(&name.clone(),&var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone())) {
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; name = name.clone());
            }
            if ety.clone() == ElementType::OTHER.clone() {
                if obfuscateSubs.clone() {
                    assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; subscripts = obfuscateSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), env.clone())?);
                }
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; componentRef = obfuscateCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), env.clone(), etype.clone(), obfuscateSubs.clone())?);
            }
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = obfuscateCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), env.clone(), etype.clone(), obfuscateSubs.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn obfuscateSubscripts(mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut env: Env) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = subs;
    subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = obfuscateSubscript(s.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(subs)
}

pub fn obfuscateSubscript(mut sub: Arc<Absyn::Subscript>, mut env: Env) -> Result<Arc<Absyn::Subscript>> {
    let mut sub: Arc<Absyn::Subscript> = sub;
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = obfuscateExp(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone(), env.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sub)
}

pub fn obfuscateFunctionArgs(mut args: Arc<Absyn::FunctionArgs>, mut fnName: Arc<Absyn::ComponentRef>, mut env: Env) -> Result<Arc<Absyn::FunctionArgs>> {
    let mut args: Arc<Absyn::FunctionArgs> = args;
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. } if (!(var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone().is_empty()) && !(isBuiltinCall(fnName.clone(), env.clone())?)) => {
            assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS; argNames = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
        for mut a in (var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone()).into_iter().cloned() {
            let __x = obfuscateNamedArg(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. } => {
            assign_variant_field!(args => Absyn::FunctionArgs::FOR_ITER_FARG; iterators = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
        for mut i in (var_field!((*args).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone()).into_iter().cloned() {
            let __x = obfuscateForIterator(i.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(args)
}

pub fn obfuscateNamedArg(mut arg: Arc<Absyn::NamedArg>, mut env: Env) -> Result<Arc<Absyn::NamedArg>> {
    let mut arg: Arc<Absyn::NamedArg> = arg;
    assign_field!(arg.argName = obfuscateIdentifier((arg.argName.clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0);
    Ok(arg)
}

pub fn obfuscateForIterator(mut iterator: Arc<Absyn::ForIterator>, mut env: Env) -> Result<Arc<Absyn::ForIterator>> {
    let mut iterator: Arc<Absyn::ForIterator> = iterator;
    assign_field!(iterator.name = obfuscateIdentifier((iterator.name.clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0);
    Ok(iterator)
}

pub fn obfuscateArrayDimsOpt(mut dims: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut env: Env) -> Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut dims: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> = dims;
    dims = Util::applyOption(dims.clone(), Arc::new(todo!("PARTEVALFUNCTION of obfuscateArrayDims: function signature not resolved")));
    dims
}

pub use obfuscateSubscripts as obfuscateArrayDims;

pub fn obfuscateExternalDecl(mut extDecl: Arc<SCode::ExternalDecl>, mut env: Env) -> Result<Arc<SCode::ExternalDecl>> {
    let mut extDecl: Arc<SCode::ExternalDecl> = extDecl;
    assign_field!(
        extDecl.args = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut a in (extDecl.args.clone()).into_iter().cloned() {
            let __x = obfuscateExp(a.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        extDecl.output_ = Util::applyOption(extDecl.output_.clone(), Arc::new({ let __pe_b1 = env.clone(); let __pe_b2 = ElementType::OTHER.clone(); let __pe_b3 = true; move |__pe_a0| obfuscateCref(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) })),
        extDecl.annotation_ = obfuscateAnnotationOpt(extDecl.annotation_.clone(), env.clone())
    );
    Ok(extDecl)
}

pub fn obfuscateEquations(mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut env: Env) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = eql;
    eql = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Equation>>> = metamodelica::nil();
        for mut eq in (eql.clone()).into_iter().cloned() {
            let __x = obfuscateEquation(eq.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(eql)
}

pub fn obfuscateEquation(mut eq: Arc<SCode::Equation>, mut env: Env) -> Result<Arc<SCode::Equation>> {
    let mut eq: Arc<SCode::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                condition = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).condition, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = obfuscateExp(e.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                thenBranch = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = obfuscateEquations(e.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = obfuscateEquations(var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_IF).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_EQUALS { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_EQUALS;
                expLeft = obfuscateExp(var_field!((*eq).expLeft, SCode::Equation::EQ_EQUALS).clone(), env.clone())?,
                expRight = obfuscateExp(var_field!((*eq).expRight, SCode::Equation::EQ_EQUALS).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_EQUALS).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_PDE;
                expLeft = obfuscateExp(var_field!((*eq).expLeft, SCode::Equation::EQ_PDE).clone(), env.clone())?,
                expRight = obfuscateExp(var_field!((*eq).expRight, SCode::Equation::EQ_PDE).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_PDE).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT;
                crefLeft = obfuscateCref(var_field!((*eq).crefLeft, SCode::Equation::EQ_CONNECT).clone(), env.clone(), ElementType::OTHER.clone(), true)?,
                crefRight = obfuscateCref(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), env.clone(), ElementType::OTHER.clone(), true)?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_CONNECT).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR;
                index = obfuscateIdentifier((var_field!((*eq).index, SCode::Equation::EQ_FOR).clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0,
                range = obfuscateExpOpt(var_field!((*eq).range, SCode::Equation::EQ_FOR).clone(), env.clone()),
                eEquationLst = obfuscateEquations(var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_FOR).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                condition = obfuscateExp(var_field!((*eq).condition, SCode::Equation::EQ_WHEN).clone(), env.clone())?,
                eEquationLst = obfuscateEquations(var_field!((*eq).eEquationLst, SCode::Equation::EQ_WHEN).clone(), env.clone())?,
                elseBranches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
            let __x = (obfuscateExp(Util::tuple21(b.clone()), env.clone())?, obfuscateEquations(Util::tuple22(b.clone()), env.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_WHEN).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_ASSERT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_ASSERT;
                condition = obfuscateExp(var_field!((*eq).condition, SCode::Equation::EQ_ASSERT).clone(), env.clone())?,
                message = obfuscateMessage(var_field!((*eq).message, SCode::Equation::EQ_ASSERT).clone(), (literal!("assert")).clone())?,
                level = obfuscateExp(var_field!((*eq).level, SCode::Equation::EQ_ASSERT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_ASSERT).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_TERMINATE { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_TERMINATE;
                message = obfuscateMessage(var_field!((*eq).message, SCode::Equation::EQ_TERMINATE).clone(), (literal!("terminate")).clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_TERMINATE).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_REINIT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_REINIT;
                cref = obfuscateExp(var_field!((*eq).cref, SCode::Equation::EQ_REINIT).clone(), env.clone())?,
                expReinit = obfuscateExp(var_field!((*eq).expReinit, SCode::Equation::EQ_REINIT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_REINIT).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Equation::EQ_NORETCALL { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_NORETCALL;
                exp = obfuscateExp(var_field!((*eq).exp, SCode::Equation::EQ_NORETCALL).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*eq).comment, SCode::Equation::EQ_NORETCALL).clone(), env.clone())
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(eq)
}

pub fn obfuscateMessage(mut message: Arc<Absyn::Exp>, mut fnName: ArcStr) -> Result<Arc<Absyn::Exp>> {
    let mut message: Arc<Absyn::Exp> = message;
    let mut msg_str: ArcStr = arcstr::literal!("");
    msg_str = ((::match_deref::match_deref! { match &(message.clone()) {
        Deref @ Absyn::Exp::STRING { .. } => var_field!((*message).value, Absyn::Exp::STRING).clone(),
        _ => Dump::printExpStr(message.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    msg_str = ArcStr::from(::std::format!("{}", stringHashDjb2((msg_str.clone()).clone())));
    msg_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fnName.clone()); __mm_s.push_str(&*literal!(" message ")); __mm_s.push_str(&*msg_str.clone()); ArcStr::from(__mm_s) }).clone();
    message = Arc::new(Absyn::Exp::STRING { value: (msg_str.clone()).clone() });
    Ok(message)
}

pub fn obfuscateAlgorithm(mut alg: Arc<SCode::AlgorithmSection>, mut env: Env) -> Result<Arc<SCode::AlgorithmSection>> {
    let mut alg: Arc<SCode::AlgorithmSection> = alg;
    assign_field!(alg.statements = obfuscateStatements(alg.statements.clone(), env.clone())?);
    Ok(alg)
}

pub fn obfuscateStatements(mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut env: Env) -> Result<Arc<metamodelica::List<Arc<SCode::Statement>>>> {
    let mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>> = stmts;
    stmts = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Statement>>> = metamodelica::nil();
        for mut s in (stmts.clone()).into_iter().cloned() {
            let __x = obfuscateStatement(s.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(stmts)
}

pub fn obfuscateStatement(mut stmt: Arc<SCode::Statement>, mut env: Env) -> Result<Arc<SCode::Statement>> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSIGN;
                assignComponent = obfuscateExp(var_field!((*stmt).assignComponent, SCode::Statement::ALG_ASSIGN).clone(), env.clone())?,
                value = obfuscateExp(var_field!((*stmt).value, SCode::Statement::ALG_ASSIGN).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSIGN).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_IF { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                boolExpr = obfuscateExp(var_field!((*stmt).boolExpr, SCode::Statement::ALG_IF).clone(), env.clone())?,
                trueBranch = obfuscateStatements(var_field!((*stmt).trueBranch, SCode::Statement::ALG_IF).clone(), env.clone())?,
                elseIfBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
            let __x = (obfuscateExp(Util::tuple21(b.clone()), env.clone())?, obfuscateStatements(Util::tuple22(b.clone()), env.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = obfuscateStatements(var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_IF).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FOR;
                index = obfuscateIdentifier((var_field!((*stmt).index, SCode::Statement::ALG_FOR).clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0,
                range = obfuscateExpOpt(var_field!((*stmt).range, SCode::Statement::ALG_FOR).clone(), env.clone()),
                forBody = obfuscateStatements(var_field!((*stmt).forBody, SCode::Statement::ALG_FOR).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_FOR).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR;
                index = obfuscateIdentifier((var_field!((*stmt).index, SCode::Statement::ALG_PARFOR).clone()).clone(), env.clone(), ElementType::OTHER.clone())?.0,
                range = obfuscateExpOpt(var_field!((*stmt).range, SCode::Statement::ALG_PARFOR).clone(), env.clone()),
                parforBody = obfuscateStatements(var_field!((*stmt).parforBody, SCode::Statement::ALG_PARFOR).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_PARFOR).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_WHILE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE;
                boolExpr = obfuscateExp(var_field!((*stmt).boolExpr, SCode::Statement::ALG_WHILE).clone(), env.clone())?,
                whileBody = obfuscateStatements(var_field!((*stmt).whileBody, SCode::Statement::ALG_WHILE).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHILE).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A;
                branches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, SCode::Statement::ALG_WHEN_A).clone()).into_iter().cloned() {
            let __x = (obfuscateExp(Util::tuple21(b.clone()), env.clone())?, obfuscateStatements(Util::tuple22(b.clone()), env.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHEN_A).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSERT;
                condition = obfuscateExp(var_field!((*stmt).condition, SCode::Statement::ALG_ASSERT).clone(), env.clone())?,
                message = obfuscateMessage(var_field!((*stmt).message, SCode::Statement::ALG_ASSERT).clone(), (literal!("assert")).clone())?,
                level = obfuscateExp(var_field!((*stmt).level, SCode::Statement::ALG_ASSERT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSERT).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TERMINATE;
                message = obfuscateMessage(var_field!((*stmt).message, SCode::Statement::ALG_TERMINATE).clone(), (literal!("terminate")).clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_TERMINATE).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_REINIT;
                cref = obfuscateExp(var_field!((*stmt).cref, SCode::Statement::ALG_REINIT).clone(), env.clone())?,
                newValue = obfuscateExp(var_field!((*stmt).newValue, SCode::Statement::ALG_REINIT).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_REINIT).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_NORETCALL { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_NORETCALL;
                exp = obfuscateExp(var_field!((*stmt).exp, SCode::Statement::ALG_NORETCALL).clone(), env.clone())?,
                comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_NORETCALL).clone(), env.clone())
            );
            ()
        },
        Deref @ SCode::Statement::ALG_RETURN { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_RETURN; comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_RETURN).clone(), env.clone()));
            ()
        },
        Deref @ SCode::Statement::ALG_BREAK { .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_BREAK; comment = obfuscateComment(var_field!((*stmt).comment, SCode::Statement::ALG_BREAK).clone(), env.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

pub fn isBuiltinCall(mut callName: Arc<Absyn::ComponentRef>, mut env: Env) -> Result<bool> {
    let mut res: bool = false;
    let mut name: ArcStr = arcstr::literal!("");
    let mut ety: ElementType = ElementType::TYPE;
    name = (AbsynUtil::crefFirstIdent(callName.clone())?).clone();
    ety = UnorderedMap::getOrDefault((name.clone()).clone(), env.builtins.clone(), ElementType::OTHER.clone());
    res = ety.clone() == ElementType::FUNCTION.clone() || ety.clone() == ElementType::TYPE_AND_FUNCTION.clone();
    Ok(res)
}

