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
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;

pub type UseTable = Arc<UnorderedSet::UnorderedSet<ArcStr>>;

pub fn getTotalModel(mut program: Arc<metamodelica::List<Arc<SCode::Element>>>, mut classPath: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut program: Arc<metamodelica::List<Arc<SCode::Element>>> = program;
    let mut used: UseTable;
    let mut prev_size: i32 = 0;
    used = UnorderedSet::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 13);
    analysePath(classPath.clone(), used.clone())?;
    UnorderedSet::add((literal!("constructor")).clone(), used.clone())?;
    UnorderedSet::add((literal!("destructor")).clone(), used.clone())?;
    while UnorderedSet::size(used.clone()) != prev_size.clone() {
        prev_size = UnorderedSet::size(used.clone());
        analyseProgram(program.clone(), used.clone())?;
    }
    program = saveElements(program.clone(), used.clone())?;
    Ok(program)
}

pub fn analyseProgram(mut program: Arc<metamodelica::List<Arc<SCode::Element>>>, mut used: UseTable) -> Result<()> {
    for mut e in &*program.clone() {
        let mut e = e.clone();
        analyseElement(e.clone(), used.clone())?;
    }
    Ok(())
}

pub fn analyseElements(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut used: UseTable) -> Result<()> {
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        analyseElement(e.clone(), used.clone())?;
    }
    Ok(())
}

pub fn analyseElement(mut element: Arc<SCode::Element>, mut used: UseTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::IMPORT { .. } => {
            analyseImport(var_field!((*element).imp, SCode::Element::IMPORT).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            analysePath(var_field!((*element).baseClassPath, SCode::Element::EXTENDS).clone(), used.clone())?;
            analyseMod(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Element::CLASS { .. } if (UnorderedSet::contains((var_field!((*element).name, SCode::Element::CLASS).clone()).clone(), used.clone())?) => {
            if SCodeUtil::isOperatorRecord(element.clone()) {
                analyseOperatorRecord(element.clone(), used.clone())?;
            }
            analyseClassDef(var_field!((*element).classDef, SCode::Element::CLASS).clone(), used.clone())?;
            analysePrefixes(var_field!((*element).prefixes, SCode::Element::CLASS).clone(), used.clone())?;
            analyseComment(var_field!((*element).cmt, SCode::Element::CLASS).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            analysePrefixes(var_field!((*element).prefixes, SCode::Element::COMPONENT).clone(), used.clone())?;
            analyseAttributes(var_field!((*element).attributes, SCode::Element::COMPONENT).clone(), used.clone())?;
            analyseTypeSpec(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone(), used.clone())?;
            analyseMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone(), used.clone())?;
            analyseExpOpt(var_field!((*element).condition, SCode::Element::COMPONENT).clone(), used.clone())?;
            analyseComment(var_field!((*element).comment, SCode::Element::COMPONENT).clone(), used.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn analyseImport(mut imp: Absyn::Import, mut used: UseTable) -> Result<()> {
    analysePath(AbsynUtil::importPath(imp.clone())?, used.clone())?;
    Ok(())
}

pub fn analyseClassDef(mut def: Arc<SCode::ClassDef>, mut used: UseTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            analyseElements(var_field!((*def).elementLst, SCode::ClassDef::PARTS).clone(), used.clone())?;
            analyseEquations(var_field!((*def).normalEquationLst, SCode::ClassDef::PARTS).clone(), used.clone())?;
            analyseEquations(var_field!((*def).initialEquationLst, SCode::ClassDef::PARTS).clone(), used.clone())?;
            analyseAlgorithms(var_field!((*def).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), used.clone())?;
            analyseAlgorithms(var_field!((*def).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), used.clone())?;
            if isSome(var_field!((*def).externalDecl, SCode::ClassDef::PARTS).clone()) {
                analyseExternalDecl(Util::getOption(var_field!((*def).externalDecl, SCode::ClassDef::PARTS).clone())?, used.clone())?;
            }
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            analyseMod(var_field!((*def).modifications, SCode::ClassDef::CLASS_EXTENDS).clone(), used.clone())?;
            analyseClassDef(var_field!((*def).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::ClassDef::DERIVED { .. } => {
            analyseTypeSpec(var_field!((*def).typeSpec, SCode::ClassDef::DERIVED).clone(), used.clone())?;
            analyseMod(var_field!((*def).modifications, SCode::ClassDef::DERIVED).clone(), used.clone())?;
            analyseAttributes(var_field!((*def).attributes, SCode::ClassDef::DERIVED).clone(), used.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn analyseExternalDecl(mut extDecl: Arc<SCode::ExternalDecl>, mut used: UseTable) -> Result<()> {
    if isSome(extDecl.annotation_.clone()) {
        analyseAnnotation(Util::getOption(extDecl.annotation_.clone())?, used.clone())?;
    }
    Ok(())
}

pub fn analyseOperatorRecord(mut element: Arc<SCode::Element>, mut used: UseTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            UnorderedSet::add((var_field!((*element).name, SCode::Element::CLASS).clone()).clone(), used.clone())?;
            for mut e in &*SCodeUtil::getClassElements(element.clone()) {
                let mut e = e.clone();
                analyseOperatorRecord(e.clone(), used.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn analyseAttributes(mut attributes: SCode::Attributes, mut used: UseTable) -> Result<()> {
    analyseDims(attributes.arrayDims.clone(), used.clone())?;
    Ok(())
}

pub fn analysePrefixes(mut prefixes: Arc<SCode::Prefixes>, mut used: UseTable) -> Result<()> {
    analyseReplaceable(prefixes.replaceablePrefix.clone(), used.clone())?;
    Ok(())
}

pub fn analyseReplaceable(mut repl: Arc<SCode::Replaceable>, mut used: UseTable) -> Result<()> {
    let mut cc: Arc<SCode::ConstrainClass> = Arc::new(<SCode::ConstrainClass as ::std::default::Default>::default());
    let () = (::match_deref::match_deref! { match &(repl.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(cc) } => {
            analyseConstrainClass(cc.clone(), used.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn analyseConstrainClass(mut cc: Arc<SCode::ConstrainClass>, mut used: UseTable) -> Result<()> {
    analysePath(cc.constrainingClass.clone(), used.clone())?;
    analyseMod(cc.modifier.clone(), used.clone())?;
    analyseComment(cc.comment.clone(), used.clone())?;
    Ok(())
}

pub fn analyseMod(mut r#mod: Arc<SCode::Mod>, mut used: UseTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            for mut s in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut s = s.clone();
                analyseMod(s.r#mod.clone(), used.clone())?;
            }
            analyseExpOpt(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Mod::REDECL { .. } => {
            analyseElement(var_field!((*r#mod).element, SCode::Mod::REDECL).clone(), used.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn analyseTypeSpec(mut ty: Arc<Absyn::TypeSpec>, mut used: UseTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { .. } => {
            analysePath(var_field!((*ty).path, Absyn::TypeSpec::TPATH).clone(), used.clone())?;
            if isSome(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone()) {
                analyseDims(Util::getOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone())?, used.clone())?;
            }
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { .. } => {
            analysePath(var_field!((*ty).path, Absyn::TypeSpec::TCOMPLEX).clone(), used.clone())?;
            for mut t in &*var_field!((*ty).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone() {
                let mut t = t.clone();
                analyseTypeSpec(t.clone(), used.clone())?;
            }
            if isSome(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone()) {
                analyseDims(Util::getOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone())?, used.clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn analysePath(mut path: Arc<Absyn::Path>, mut used: UseTable) -> Result<()> {
    for mut i in &*AbsynUtil::pathToStringList(path.clone())? {
        let mut i = i.clone();
        UnorderedSet::add((i.clone()).clone(), used.clone())?;
    }
    Ok(())
}

pub fn analyseEquations(mut eqs: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut used: UseTable) -> Result<()> {
    for mut e in &*eqs.clone() {
        let mut e = e.clone();
        analyseEquation(e.clone(), used.clone())?;
    }
    Ok(())
}

pub fn analyseEquation(mut eq: Arc<SCode::Equation>, mut used: UseTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { .. } => {
            analyseExpList(var_field!((*eq).condition, SCode::Equation::EQ_IF).clone(), used.clone())?;
            for mut b in &*var_field!((*eq).thenBranch, SCode::Equation::EQ_IF).clone() {
                let mut b = b.clone();
                analyseEquations(b.clone(), used.clone())?;
            }
            analyseEquations(var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone(), used.clone())?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_IF).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_EQUALS { .. } => {
            analyseExp(var_field!((*eq).expLeft, SCode::Equation::EQ_EQUALS).clone(), used.clone())?;
            analyseExp(var_field!((*eq).expRight, SCode::Equation::EQ_EQUALS).clone(), used.clone())?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_EQUALS).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_PDE { .. } => {
            analyseExp(var_field!((*eq).expLeft, SCode::Equation::EQ_PDE).clone(), used.clone())?;
            analyseExp(var_field!((*eq).expRight, SCode::Equation::EQ_PDE).clone(), used.clone())?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_PDE).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_CONNECT { .. } => {
            analyseCref(var_field!((*eq).crefLeft, SCode::Equation::EQ_CONNECT).clone(), used.clone(), true)?;
            analyseCref(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), used.clone(), true)?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_CONNECT).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_FOR { .. } => {
            analyseExpOpt(var_field!((*eq).range, SCode::Equation::EQ_FOR).clone(), used.clone())?;
            analyseEquations(var_field!((*eq).eEquationLst, SCode::Equation::EQ_FOR).clone(), used.clone())?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_FOR).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_WHEN { .. } => {
            analyseExp(var_field!((*eq).condition, SCode::Equation::EQ_WHEN).clone(), used.clone())?;
            analyseEquations(var_field!((*eq).eEquationLst, SCode::Equation::EQ_WHEN).clone(), used.clone())?;
            for mut b in &*var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone() {
                let mut b = b.clone();
                analyseExp(Util::tuple21(b.clone()), used.clone())?;
                analyseEquations(Util::tuple22(b.clone()), used.clone())?;
            }
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_WHEN).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_ASSERT { .. } => {
            analyseExp(var_field!((*eq).condition, SCode::Equation::EQ_ASSERT).clone(), used.clone())?;
            analyseExp(var_field!((*eq).message, SCode::Equation::EQ_ASSERT).clone(), used.clone())?;
            analyseExp(var_field!((*eq).level, SCode::Equation::EQ_ASSERT).clone(), used.clone())?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_ASSERT).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_TERMINATE { .. } => {
            analyseExp(var_field!((*eq).message, SCode::Equation::EQ_TERMINATE).clone(), used.clone())?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_TERMINATE).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_REINIT { .. } => {
            analyseExp(var_field!((*eq).cref, SCode::Equation::EQ_REINIT).clone(), used.clone())?;
            analyseExp(var_field!((*eq).expReinit, SCode::Equation::EQ_REINIT).clone(), used.clone())?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_REINIT).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Equation::EQ_NORETCALL { .. } => {
            analyseExp(var_field!((*eq).exp, SCode::Equation::EQ_NORETCALL).clone(), used.clone())?;
            analyseComment(var_field!((*eq).comment, SCode::Equation::EQ_NORETCALL).clone(), used.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn analyseAlgorithms(mut algs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut used: UseTable) -> Result<()> {
    for mut a in &*algs.clone() {
        let mut a = a.clone();
        analyseAlgorithm(a.clone(), used.clone())?;
    }
    Ok(())
}

pub fn analyseAlgorithm(mut alg: Arc<SCode::AlgorithmSection>, mut used: UseTable) -> Result<()> {
    analyseStatements(alg.statements.clone(), used.clone())?;
    Ok(())
}

pub fn analyseStatements(mut stmts: Arc<metamodelica::List<Arc<SCode::Statement>>>, mut used: UseTable) -> Result<()> {
    for mut s in &*stmts.clone() {
        let mut s = s.clone();
        analyseStatement(s.clone(), used.clone())?;
    }
    Ok(())
}

pub fn analyseStatement(mut stmt: Arc<SCode::Statement>, mut used: UseTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { .. } => {
            analyseExp(var_field!((*stmt).assignComponent, SCode::Statement::ALG_ASSIGN).clone(), used.clone())?;
            analyseExp(var_field!((*stmt).value, SCode::Statement::ALG_ASSIGN).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSIGN).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_IF { .. } => {
            analyseExp(var_field!((*stmt).boolExpr, SCode::Statement::ALG_IF).clone(), used.clone())?;
            analyseStatements(var_field!((*stmt).trueBranch, SCode::Statement::ALG_IF).clone(), used.clone())?;
            for mut b in &*var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone() {
                let mut b = b.clone();
                analyseExp(Util::tuple21(b.clone()), used.clone())?;
                analyseStatements(Util::tuple22(b.clone()), used.clone())?;
            }
            analyseStatements(var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_IF).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_FOR { .. } => {
            analyseExpOpt(var_field!((*stmt).range, SCode::Statement::ALG_FOR).clone(), used.clone())?;
            analyseStatements(var_field!((*stmt).forBody, SCode::Statement::ALG_FOR).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_FOR).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_PARFOR { .. } => {
            analyseExpOpt(var_field!((*stmt).range, SCode::Statement::ALG_PARFOR).clone(), used.clone())?;
            analyseStatements(var_field!((*stmt).parforBody, SCode::Statement::ALG_PARFOR).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_PARFOR).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_WHILE { .. } => {
            analyseExp(var_field!((*stmt).boolExpr, SCode::Statement::ALG_WHILE).clone(), used.clone())?;
            analyseStatements(var_field!((*stmt).whileBody, SCode::Statement::ALG_WHILE).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHILE).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_WHEN_A { .. } => {
            for mut b in &*var_field!((*stmt).branches, SCode::Statement::ALG_WHEN_A).clone() {
                let mut b = b.clone();
                analyseExp(Util::tuple21(b.clone()), used.clone())?;
                analyseStatements(Util::tuple22(b.clone()), used.clone())?;
            }
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHEN_A).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            analyseExp(var_field!((*stmt).condition, SCode::Statement::ALG_ASSERT).clone(), used.clone())?;
            analyseExp(var_field!((*stmt).message, SCode::Statement::ALG_ASSERT).clone(), used.clone())?;
            analyseExp(var_field!((*stmt).level, SCode::Statement::ALG_ASSERT).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_ASSERT).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => {
            analyseExp(var_field!((*stmt).message, SCode::Statement::ALG_TERMINATE).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_TERMINATE).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            analyseExp(var_field!((*stmt).cref, SCode::Statement::ALG_REINIT).clone(), used.clone())?;
            analyseExp(var_field!((*stmt).newValue, SCode::Statement::ALG_REINIT).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_REINIT).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_NORETCALL { .. } => {
            analyseExp(var_field!((*stmt).exp, SCode::Statement::ALG_NORETCALL).clone(), used.clone())?;
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_NORETCALL).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_RETURN { .. } => {
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_RETURN).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_BREAK { .. } => {
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_BREAK).clone(), used.clone())?;
            ()
        },
        Deref @ SCode::Statement::ALG_CONTINUE { .. } => {
            analyseComment(var_field!((*stmt).comment, SCode::Statement::ALG_CONTINUE).clone(), used.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub use analyseSubscripts as analyseDims;

pub fn analyseSubscripts(mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut used: UseTable) -> Result<()> {
    for mut s in &*subs.clone() {
        let mut s = s.clone();
        analyseSubscript(s.clone(), used.clone())?;
    }
    Ok(())
}

pub fn analyseSubscript(mut sub: Arc<Absyn::Subscript>, mut used: UseTable) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            analyseExp(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone(), used.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn analyseExpOpt(mut exp: Option<Arc<Absyn::Exp>>, mut used: UseTable) -> Result<()> {
    if isSome(exp.clone()) {
        analyseExp(Util::getOption(exp.clone())?, used.clone())?;
    }
    Ok(())
}

pub fn analyseExpList(mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut used: UseTable) -> Result<()> {
    for mut e in &*expl.clone() {
        let mut e = e.clone();
        analyseExp(e.clone(), used.clone())?;
    }
    Ok(())
}

pub fn analyseExp(mut exp: Arc<Absyn::Exp>, mut used: UseTable) -> Result<()> {
    AbsynUtil::traverseExp(exp.clone(), (std::sync::Arc::new(analyseExpTraverse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<UnorderedSet::UnorderedSet<ArcStr>>) -> Result<(Arc<Absyn::Exp>, Arc<UnorderedSet::UnorderedSet<ArcStr>>)> + 'static>), used.clone())?;
    Ok(())
}

pub fn analyseExpTraverse(mut exp: Arc<Absyn::Exp>, mut used: UseTable) -> Result<(Arc<Absyn::Exp>, UseTable)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut used: UseTable = used;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => {
            analyseCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), used.clone(), true)?;
            ()
        },
        Deref @ Absyn::Exp::CALL { .. } => {
            analyseCref(var_field!((*exp).function_, Absyn::Exp::CALL).clone(), used.clone(), true)?;
            ()
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { .. } => {
            analyseCref(var_field!((*exp).function_, Absyn::Exp::PARTEVALFUNCTION).clone(), used.clone(), true)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, used))
}

pub fn analyseCref(mut cref: Arc<Absyn::ComponentRef>, mut used: UseTable, mut includeLast: bool) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
            analyseCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), used.clone(), includeLast.clone())?;
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            UnorderedSet::add((var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), used.clone())?;
            analyseSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), used.clone())?;
            analyseCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), used.clone(), includeLast.clone())?;
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            if includeLast.clone() {
                UnorderedSet::add((var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), used.clone())?;
            }
            analyseSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), used.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn analyseComment(mut comment: Arc<SCode::Comment>, mut used: UseTable) -> Result<()> {
    if isSome(comment.annotation_.clone()) {
        analyseAnnotation(Util::getOption(comment.annotation_.clone())?, used.clone())?;
    }
    Ok(())
}

pub fn analyseAnnotation(mut ann: Arc<SCode::Annotation>, mut used: UseTable) -> Result<()> {
    analyseMod(ann.modification.clone(), used.clone())?;
    Ok(())
}

pub fn saveElements(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut used: UseTable) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        outElements = saveElement(e.clone(), used.clone(), outElements.clone())?;
    }
    outElements = outElements.clone().reverse();
    Ok(outElements)
}

pub fn saveElement(mut element: Arc<SCode::Element>, mut used: UseTable, mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut elements: Arc<metamodelica::List<Arc<SCode::Element>>> = elements;
    let mut elem: Arc<SCode::Element> = element.clone();
    let mut is_empty: bool = false;
    elements = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ SCode::Element::CLASS { .. } if (UnorderedSet::contains((var_field!((*elem).name, SCode::Element::CLASS).clone()).clone(), used.clone())?) => {
            assign_variant_field!(elem => SCode::Element::CLASS; classDef = saveClassDef(var_field!((*elem).classDef, SCode::Element::CLASS).clone(), used.clone())?);
            cons(elem.clone(), elements.clone())
        },
        Deref @ SCode::Element::CLASS { .. } => elements.clone(),
        Deref @ SCode::Element::EXTENDS { .. } if (AbsynUtil::pathContains(var_field!((*elem).baseClassPath, SCode::Element::EXTENDS).clone(), (literal!("Icons")).clone())?) => elements.clone(),
        _ => cons(element.clone(), elements.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

pub fn saveClassDef(mut def: Arc<SCode::ClassDef>, mut used: UseTable) -> Result<Arc<SCode::ClassDef>> {
    let mut def: Arc<SCode::ClassDef> = def;
    let () = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::ClassDef::PARTS { .. } => {
            assign_variant_field!(def => SCode::ClassDef::PARTS; elementLst = saveElements(var_field!((*def).elementLst, SCode::ClassDef::PARTS).clone(), used.clone())?);
            ()
        },
        Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(def => SCode::ClassDef::CLASS_EXTENDS; composition = saveClassDef(var_field!((*def).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), used.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(def)
}

