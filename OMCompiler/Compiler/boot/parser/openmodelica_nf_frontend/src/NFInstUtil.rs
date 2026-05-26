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

use crate::BaseModelica;
use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFlatten;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn dumpFlatModelDebug(mut stage: ArcStr, mut flatModel: Arc<FlatModel::NFFlatModel>, mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree>) -> Result<()> {
    let mut flat_model: Arc<FlatModel::NFFlatModel> = flatModel.clone();
    if Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("protected") || Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("encrypted") {
        flat_model = FlatModel::obfuscate(flat_model.clone())?;
    }
    if Flags::isConfigFlagSet(Flags::DUMP_FLAT_MODEL.clone(), (stage.clone()).clone())? || Flags::getConfigStringList(Flags::DUMP_FLAT_MODEL.clone())?.is_empty() {
        flat_model = combineSubscripts(flatModel.clone())?;
        println!("{}", (literal!("########################################\n")).clone());
        println!("{}", (stage.clone()).clone());
        println!("{}", (literal!("\n########################################\n\n")).clone());
        if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
            FlatModel::printFlatString(flat_model.clone(), functions.clone(), false)?;
        } else {
            FlatModel::printString(flat_model.clone(), functions.clone(), false)?;
        }
        println!("{}", (literal!("\n")).clone());
    }
    Ok(())
}

pub fn combineSubscripts(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    if Flags::isSet(Flags::COMBINE_SUBSCRIPTS.clone())? {
        flatModel = FlatModel::mapExp(flatModel.clone(), Arc::new(combineSubscriptsExp));
    }
    Ok(flatModel)
}

pub fn combineSubscriptsExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    fn traverser(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = ComponentRef::combineSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), Arc::new(traverser))?;
    Ok(exp)
}

pub fn printStructuralParameters(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<()> {
    let mut params: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    if Flags::isSet(Flags::PRINT_STRUCTURAL.clone())? {
        params = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            if !(Variable::isStructural(v.clone())) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        if !(params.clone().is_empty()) {
            names = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (params.clone()).into_iter().cloned() {
            let __x = ComponentRef::toString(v.name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            Error::addMessage(Error::NOTIFY_FRONTEND_STRUCTURAL_PARAMETERS.clone(), list![stringDelimitList(names.clone(), (literal!(", ")).clone())])?;
        }
    }
    Ok(())
}

pub fn dumpFlatModel(mut flatModel: Arc<FlatModel::NFFlatModel>, mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut flat_model: Arc<FlatModel::NFFlatModel>;
    flat_model = combineSubscripts(flatModel.clone())?;
    r#str = (FlatModel::toFlatString(flat_model.clone(), functions.clone(), false)).clone();
    Ok(r#str)
}

pub fn replaceEmptyArrays(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Arc<FlatModel::NFFlatModel> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    flatModel = FlatModel::mapExp(flatModel.clone(), Arc::new(replaceEmptyArraysExp));
    flatModel
}

pub fn replaceEmptyArraysExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    fn traverser(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
        let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref, .. } if (ComponentRef::isEmptyArray(cref.clone())) => {
            let mut cref = (*cref).clone();
            if ComponentRef::hasSubscripts(cref.clone()) {
                cref = ComponentRef::fillSubscripts(cref.clone());
                cref = ComponentRef::replaceWholeSubscripts(cref.clone())?;
                subs = ComponentRef::subscriptsAllFlat(cref.clone());
                cref = ComponentRef::stripSubscriptsAll(cref.clone());
                ty = ComponentRef::getSubscriptedType(cref.clone(), false)?;
            } else {
                subs = metamodelica::nil();
                ty = var_field!((*exp).ty, Expression::NFExpression::CREF).clone();
            }
            outExp = Expression::makeDefaultValue(ty.clone(), None, None)?;
            if !(subs.clone().is_empty()) {
                outExp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: outExp.clone(), subscripts: subs.clone(), ty: var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), split: false });
            }
            outExp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outExp)
    }

    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), Arc::new(traverser))?;
    Ok(exp)
}

pub fn expandSlicedCrefs(mut flatModel: Arc<FlatModel::NFFlatModel>, mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree>) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<NFFlatten::FunctionTreeImpl::Tree>)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree> = functions;
    if Flags::isSet(Flags::COMBINE_SUBSCRIPTS.clone())? || !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        return Ok((flatModel, functions));
    }
    assign_field!(flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = Variable::mapExpShallow(v.clone(), expandSlicedCrefsExp);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    flatModel = FlatModel::mapEquations(flatModel.clone(), Arc::new(expandSlicedCrefsEq));
    flatModel = FlatModel::mapAlgorithms(flatModel.clone(), Arc::new(fnptr!(expandSlicedCrefsAlg, Arc<Algorithm::NFAlgorithm>)));
    functions = NFFlatten::FunctionTreeImpl::map(functions.clone(), Arc::new(expandSlicedCrefsFunction));
    Ok((flatModel, functions))
}

pub fn addTrailingWholeIndices(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::hasImplicitTrailingIndex(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = ComponentRef::fillSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()));
            exp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn expandSlicedCrefsExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isSliced(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) => expandSlicedCrefsExp2(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn expandSlicedCrefsExp2(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>> = metamodelica::nil();
    (cr, iterators) = ComponentRef::iterate(cref.clone())?;
    outExp = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty.clone(), var: ComponentRef::variability(cref.clone()), purity: ComponentRef::purity(cref.clone()), exp: Expression::fromCref(cr.clone(), false)?, iters: iterators.clone() }) });
    Ok(outExp)
}

pub fn expandSlicedCrefsEq(mut eq: Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eq2: Arc<Equation::NFEquation>;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { rhs: e1, .. } => {
            let mut e1 = (*e1).clone();
            e1 = Expression::map(e1.clone(), Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)))?;
            e2 = Expression::map(e1.clone(), Arc::new(expandSlicedCrefsExp))?;
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(eq => Equation::NFEquation::EQUALITY; rhs = e2.clone());
            }
            eq.clone()
        },
        _ => {
            eq2 = Equation::mapExpShallow(eq.clone(), { let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) });
            Equation::mapExpShallow(eq2.clone(), { let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new(expandSlicedCrefsExp); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub fn expandSlicedCrefsAlg(mut alg: Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = {
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (alg.statements.clone()).into_iter().cloned() {
            let __x = Statement::map(s.clone(), Arc::new(expandSlicedCrefsStmt));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    alg
}

pub fn expandSlicedCrefsStmt(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement> = stmt;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { rhs: e1, .. } => {
            let mut e1 = (*e1).clone();
            assign_variant_field!(stmt => Statement::NFStatement::ASSIGNMENT; lhs = Expression::map(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)))?);
            e1 = Expression::map(e1.clone(), Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)))?;
            e2 = Expression::map(e1.clone(), Arc::new(expandSlicedCrefsExp))?;
            if !(referenceEq(&e1.clone(),&e2.clone())) {
                assign_variant_field!(stmt => Statement::NFStatement::ASSIGNMENT; rhs = e2.clone());
            }
            stmt.clone()
        },
        _ => {
            let mut stmt2: Arc<Statement::NFStatement>;
            stmt2 = Statement::mapExpShallow(stmt.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }));
            Statement::mapExpShallow(stmt2.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new(expandSlicedCrefsExp); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

pub fn expandSlicedCrefsFunction(mut fnPath: Arc<Absyn::Path>, mut r#fn: Arc<Function::Function>) -> Result<Arc<Function::Function>> {
    let mut r#fn: Arc<Function::Function> = r#fn;
    r#fn = Function::mapExp(r#fn.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new(expandSlicedCrefsExp); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new(expandSlicedCrefsExp); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }), true, false)?;
    r#fn = Function::mapBody(r#fn.clone(), Arc::new(fnptr!(expandSlicedCrefsAlg, Arc<Algorithm::NFAlgorithm>)))?;
    Ok(r#fn)
}

pub type MergeNameMap = Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>;

pub fn makeMergeNameMap() -> MergeNameMap {
    let mut nameMap: MergeNameMap = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), fnptr!(stringEq, ArcStr, ArcStr), 1);
    nameMap
}

pub fn mergeScalars(mut node: Arc<InstNode::InstNode>, mut classPath: Arc<Absyn::Path>, mut isRootClass: bool, mut nameMap: MergeNameMap) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut elem: Arc<SCode::Element>;
    if !(Flags::isSet(Flags::MERGE_COMPONENTS.clone())?) {
        return Ok(node);
    }
    elem = InstNode::definition(node.clone())?;
    elem = mergeScalars2(elem.clone(), classPath.clone(), isRootClass.clone(), nameMap.clone())?;
    node = InstNode::setDefinition(elem.clone(), node.clone())?;
    execStat((literal!("NFInstUtil.mergeScalars")).clone())?;
    Ok(node)
}

pub fn mergeScalars2(mut cls: Arc<SCode::Element>, mut classPath: Arc<Absyn::Path>, mut isRootClass: bool, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Element>> {
    let mut cls: Arc<SCode::Element> = cls;
    let mut cdef: Arc<SCode::ClassDef>;
    let mut elems: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { classDef: cdef @ Deref @ SCode::ClassDef::PARTS { .. }, .. } => {
            let mut cdef = (*cdef).clone();
            elems = mergeScalars3(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?;
            elems = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (elems.clone()).into_iter().cloned() {
            let __x = mergeScalarsElement(e.clone(), nameMap.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            assign_variant_field!(cdef => SCode::ClassDef::PARTS;
                elementLst = elems.clone(),
                normalEquationLst = mergeScalarsEql(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), nameMap.clone()),
                initialEquationLst = mergeScalarsEql(var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), nameMap.clone()),
                normalAlgorithmLst = mergeScalarsAlgs(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), nameMap.clone()),
                initialAlgorithmLst = mergeScalarsAlgs(var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())
            );
            assign_variant_field!(cls => SCode::Element::CLASS; classDef = cdef.clone());
            if isRootClass.clone() {
                System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(classPath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("_merged_table.json")); ArcStr::from(__mm_s) }).clone(), (UnorderedMap::toJSON(nameMap.clone(), fnptr!(Util::id, _), Dump::printComponentRefStr)?).clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub fn mergeScalars3(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut nameMap: MergeNameMap) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut mergeable: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Element>>>>> = metamodelica::nil();
    let mut merged_e: Arc<SCode::Element>;
    let mut i: i32 = UnorderedMap::size(nameMap.clone()) + 1;
    let mut prefix: ArcStr = arcstr::literal!("");
    (mergeable, outElements) = makeMergeMap(elements.clone())?;
    for mut el in &*mergeable.clone() {
        let mut el = el.clone();
        prefix = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(SCodeUtil::getElementTypePath(listHead(el.clone())?)?)?); ArcStr::from(__mm_s) }).clone();
        merged_e = mergeComponents(el.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", i.clone()))); ArcStr::from(__mm_s) }).clone(), nameMap.clone())?;
        i = i.clone() + 1;
        outElements = cons(merged_e.clone(), outElements.clone());
    }
    outElements = outElements.clone().reverse();
    Ok(outElements)
}

pub fn makeMergeMap(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Element>>>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    fn append_merge(mut oldValue: Option<Arc<metamodelica::List<Arc<SCode::Element>>>>, mut elem: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
        let mut newValue: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        if isSome(oldValue.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(oldValue.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            newValue = __pa0.clone();
        } else {
            newValue = metamodelica::nil();
        }
        newValue = cons(elem.clone(), newValue.clone());
        Ok(newValue)
    }

    let mut mergeable: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Element>>>>> = metamodelica::nil();
    let mut unmergeable: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut merge_map: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<metamodelica::List<Arc<SCode::Element>>>>>;
    let mut grouped_elems: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Element>>>>> = metamodelica::nil();
    merge_map = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), fnptr!(stringEq, ArcStr, ArcStr), 1);
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } if (isMergeableComponent(e.clone())) => {
            UnorderedMap::addUpdate((getComponentSignature(e.clone())?).clone(), Arc::new({ let __pe_b1 = e.clone(); move |__pe_a0| append_merge(__pe_a0, __pe_b1.clone()) }), merge_map.clone())?;
            ()
        },
        _ => {
            unmergeable = cons(e.clone(), unmergeable.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    grouped_elems = UnorderedMap::valueList(merge_map.clone());
    for mut el in &*grouped_elems.clone() {
        let mut el = el.clone();
        if (el.clone().len() as i32) == 1 {
            unmergeable = cons(listHead(el.clone())?, unmergeable.clone());
        } else {
            mergeable = cons(el.clone().reverse(), mergeable.clone());
        }
    }
    Ok((mergeable, unmergeable))
}

pub fn isMergeableComponent(mut element: Arc<SCode::Element>) -> bool {
    let mut isMergeable: bool = false;
    isMergeable = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { condition: None, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, redeclarePrefix: SCode::Redeclare::NOT_REDECLARE { .. }, .. }, attributes: SCode::Attributes { arrayDims: Deref @ metamodelica::List::Nil, .. }, .. } => isMergeableType(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone()) && isMergeableMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isMergeable
}

pub fn isMergeableMod(mut r#mod: Arc<SCode::Mod>) -> bool {
    let mut mergeable: bool = false;
    mergeable = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH { .. }, .. } => {
            for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut m = m.clone();
                if !(isMergeableMod(m.r#mod.clone())) {
                    mergeable = false;
                    return mergeable;
                }
            }
            true
        },
        Deref @ SCode::Mod::NOMOD => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    mergeable
}

pub fn isMergeableType(mut ty: Arc<Absyn::TypeSpec>) -> bool {
    let mut mergeable: bool = false;
    mergeable = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { arrayDim: None, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    mergeable
}

pub fn getComponentSignature(mut element: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut signature: ArcStr = arcstr::literal!("");
    let mut prefs: Arc<SCode::Prefixes>;
    let mut attrs: SCode::Attributes;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { modifications: __pa0, typeSpec: __pa1, attributes: __pa2, prefixes: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r#mod = __pa0.clone();
    ty = __pa1.clone();
    attrs = __pa2.clone();
    prefs = __pa3.clone();
    signature = stringAppendList(list![(SCodeDump::visibilityStr(prefs.visibility.clone())?).clone(), (SCodeDump::finalStr(prefs.finalPrefix.clone())?).clone(), (SCodeDump::connectorTypeStr(attrs.connectorType.clone())?).clone(), (SCodeDump::variabilityString(attrs.variability.clone())?).clone(), (Dump::unparseDirectionSymbolStr(attrs.direction.clone())?).clone(), (Dump::unparseTypeSpec(ty.clone())?).clone(), (getModSignature(r#mod.clone(), (literal!("")).clone())?).clone()]);
    Ok(signature)
}

pub fn getModSignature(mut r#mod: Arc<SCode::Mod>, mut name: ArcStr) -> Result<ArcStr> {
    fn sub_mod_lt(mut m1: Arc<SCode::SubMod>, mut m2: Arc<SCode::SubMod>) -> bool {
        let mut res: bool = m1.ident.clone() < m2.ident.clone();
        res
    }

    let mut signature: ArcStr = arcstr::literal!("");
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut has_binding: bool = false;
    let mut has_submods: bool = false;
    signature = ((::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            has_binding = isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone());
            has_submods = !(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone().is_empty());
            if has_binding.clone() {
                strl = cons(literal!("="), strl.clone());
            }
            if has_submods.clone() {
                strl = cons(literal!(")"), strl.clone());
                for mut m in &*List::sort(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), Arc::new(fnptr!(sub_mod_lt, Arc<SCode::SubMod>, Arc<SCode::SubMod>)))? {
                    let mut m = m.clone();
                    strl = cons(literal!(","), strl.clone());
                    strl = cons(getModSignature(m.r#mod.clone(), (m.ident.clone()).clone())?, strl.clone());
                }
                strl = cons(literal!("("), strl.clone());
            }
            if has_binding.clone() || has_submods.clone() {
                strl = cons(name.clone(), strl.clone());
            }
            if SCodeUtil::finalBool(var_field!((*r#mod).finalPrefix, SCode::Mod::MOD).clone())? {
                strl = cons(literal!("final "), strl.clone());
            }
            if SCodeUtil::eachBool(var_field!((*r#mod).eachPrefix, SCode::Mod::MOD).clone())? {
                strl = cons(literal!("each "), strl.clone());
            }
            stringAppendList(strl.clone())
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(signature)
}

pub fn mergeComponents(mut components: Arc<metamodelica::List<Arc<SCode::Element>>>, mut prefix: ArcStr, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Element>> {
    let mut mergedComponent: Arc<SCode::Element>;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut prefs: Arc<SCode::Prefixes>;
    let mut attrs: SCode::Attributes;
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut i: i32 = 1;
    let mut name: ArcStr = arcstr::literal!("");
    let mut cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut mods: Arc<metamodelica::List<Arc<SCode::Mod>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(listHead(components.clone())?) {
        Deref @ SCode::Element::COMPONENT { attributes: __pa0, prefixes: __pa1, typeSpec: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    attrs = __pa0.clone();
    prefs = __pa1.clone();
    ty = __pa2.clone();
    attrs.arrayDims = list![AbsynUtil::makeIntegerSubscript((components.clone().len() as i32))];
    mods = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Mod>>> = metamodelica::nil();
        for mut c in (components.clone()).into_iter().cloned() {
            let __x = SCodeUtil::componentMod(c.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    r#mod = mergeMods(mods.clone())?;
    mergedComponent = Arc::new(SCode::Element::COMPONENT { name: (prefix.clone()).clone(), prefixes: prefs.clone(), attributes: attrs.clone(), typeSpec: ty.clone(), modifications: r#mod.clone(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
    for mut c in &*components.clone() {
        let mut c = c.clone();
        let __pa3 = ::match_deref::match_deref! { match &(c.clone()) {
            Deref @ SCode::Element::COMPONENT { name: __pa3, .. } => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa3.clone();
        cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (prefix.clone()).clone(), subscripts: list![AbsynUtil::makeIntegerSubscript(i.clone())] });
        i = i.clone() + 1;
        UnorderedMap::addUnique((name.clone()).clone(), cref.clone(), nameMap.clone())?;
    }
    Ok(mergedComponent)
}

pub fn mergeMods(mut mods: Arc<metamodelica::List<Arc<SCode::Mod>>>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut bindings: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
    let mut binding_map: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Absyn::Exp>>>;
    if mods.clone().is_empty() {
        r#mod = Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD);
        return Ok(r#mod);
    }
    r#mod = listHead(mods.clone())?;
    names = getModNames(r#mod.clone(), metamodelica::nil(), metamodelica::nil())?;
    bindings = List::fill(metamodelica::nil(), (names.clone().len() as i32));
    for mut m in &*mods.clone().reverse() {
        let mut m = m.clone();
        bindings = getModBindings(m.clone(), names.clone(), bindings.clone())?;
    }
    binding_map = UnorderedMap::fromLists(names.clone(), {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut b in (bindings.clone()).into_iter().cloned() {
            let __x = Arc::new(Absyn::Exp::ARRAY { arrayExp: b.clone() });
            __acc = cons(__x, __acc);
        }
        __acc
    }, AbsynUtil::pathHash, fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>))?;
    r#mod = mergeMods2(r#mod.clone(), binding_map.clone(), metamodelica::nil())?;
    Ok(r#mod)
}

pub fn getModNames(mut r#mod: Arc<SCode::Mod>, mut name: Arc<metamodelica::List<ArcStr>>, mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>> = names;
    names = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            if isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
                names = cons(makeModPath(name.clone())?, names.clone());
            }
            for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut m = m.clone();
                names = getModNames(m.r#mod.clone(), cons(m.ident.clone(), name.clone()), names.clone())?;
            }
            names.clone()
        },
        _ => names.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(names)
}

pub fn makeModPath(mut name: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    if name.clone().is_empty() {
        path = Arc::new(Absyn::Path::IDENT { name: (literal!("$")).clone() });
    } else {
        path = AbsynUtil::stringListPathReversed(name.clone())?;
    }
    Ok(path)
}

pub fn mergeMods2(mut r#mod: Arc<SCode::Mod>, mut bindingMap: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Absyn::Exp>>>, mut name: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let mut new_binding: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            if isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
                new_binding = UnorderedMap::getOrFail(makeModPath(name.clone())?, bindingMap.clone());
                assign_variant_field!(r#mod => SCode::Mod::MOD; binding = Some(new_binding.clone()));
            }
            if !(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone().is_empty()) {
                for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                    let mut m = m.clone();
                    assign_field!(m.r#mod = mergeMods2(m.r#mod.clone(), bindingMap.clone(), cons(m.ident.clone(), name.clone()))?);
                    submods = cons(m.clone(), submods.clone());
                }
                assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = submods.clone().reverse());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub fn getModBindings(mut r#mod: Arc<SCode::Mod>, mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut bindings: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>> {
    let mut bindings: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = bindings;
    let mut mod_bindings: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    for mut name in &*names.clone() {
        let mut name = name.clone();
        mod_bindings = cons(lookupModBinding(name.clone(), r#mod.clone())?, mod_bindings.clone());
    }
    bindings = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for (e1, e2) in (&(mod_bindings.clone())).into_iter().zip((&(bindings.clone())).into_iter()) {
            let __x = cons(e1.clone(), e2.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(bindings)
}

pub fn lookupModBinding(mut name: Arc<Absyn::Path>, mut r#mod: Arc<SCode::Mod>) -> Result<Arc<Absyn::Exp>> {
    let mut binding: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut m: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let __pa0 = ::match_deref::match_deref! { match &(lookupMod(name.clone(), r#mod.clone())?) {
        Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    binding = __pa0.clone();
    Ok(binding)
}

pub fn lookupMod(mut name: Arc<Absyn::Path>, mut r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => if (var_field!((*name).name, Absyn::Path::IDENT).clone() == literal!("$")) {r#mod.clone()} else {SCodeUtil::lookupModInMod((var_field!((*name).name, Absyn::Path::IDENT).clone()).clone(), r#mod.clone())},
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            outMod = SCodeUtil::lookupModInMod((var_field!((*name).name, Absyn::Path::QUALIFIED).clone()).clone(), r#mod.clone());
            lookupMod(var_field!((*name).path, Absyn::Path::QUALIFIED).clone(), outMod.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

pub fn mergeScalarsElement(mut element: Arc<SCode::Element>, mut nameMap: MergeNameMap) -> Arc<SCode::Element> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = mergeScalarsMod(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), nameMap.clone()));
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = mergeScalarsMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone(), nameMap.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    element
}

pub fn mergeScalarsEql(mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut nameMap: MergeNameMap) -> Arc<metamodelica::List<Arc<SCode::Equation>>> {
    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = eql;
    eql = SCodeUtil::mapEquationsList(eql.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsEq(__pe_a0, __pe_b1.clone()) }));
    eql
}

pub fn mergeScalarsEq(mut eq: Arc<SCode::Equation>, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Equation>> {
    let mut eq: Arc<SCode::Equation> = eq;
    eq = SCodeUtil::mapEquationExps(eq.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone()) }))?;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT;
                crefLeft = mergeScalarsCref(var_field!((*eq).crefLeft, SCode::Equation::EQ_CONNECT).clone(), nameMap.clone())?,
                crefRight = mergeScalarsCref(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), nameMap.clone())?
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub fn mergeScalarsMod(mut r#mod: Arc<SCode::Mod>, mut nameMap: MergeNameMap) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD;
                binding = Util::applyOption(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone()) })),
                subModLst = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut m in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            let __x = mergeScalarsSubMod(m.clone(), nameMap.clone());
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
    r#mod
}

pub fn mergeScalarsSubMod(mut r#mod: Arc<SCode::SubMod>, mut nameMap: MergeNameMap) -> Arc<SCode::SubMod> {
    let mut r#mod: Arc<SCode::SubMod> = r#mod;
    assign_field!(r#mod.r#mod = mergeScalarsMod(r#mod.r#mod.clone(), nameMap.clone()));
    r#mod
}

pub fn mergeScalarsExps(mut exp: Arc<Absyn::Exp>, mut nameMap: MergeNameMap) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(exp.clone(), Arc::new(mergeScalarsExp), nameMap.clone())?;
    Ok(exp)
}

pub fn mergeScalarsExp(mut exp: Arc<Absyn::Exp>, mut nameMap: MergeNameMap) -> Result<(Arc<Absyn::Exp>, MergeNameMap)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut nameMap: MergeNameMap = nameMap;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } if (!(AbsynUtil::crefIsWild(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone()))) => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = mergeScalarsCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), nameMap.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, nameMap))
}

pub fn mergeScalarsCref(mut cref: Arc<Absyn::ComponentRef>, mut nameMap: MergeNameMap) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut repl_ocr: Option<Arc<Absyn::ComponentRef>> = None;
    let mut repl_cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
    repl_ocr = UnorderedMap::get((AbsynUtil::crefFirstIdent(cref.clone())?).clone(), nameMap.clone());
    if isSome(repl_ocr.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(repl_ocr.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        repl_cr = __pa0.clone();
        subs = AbsynUtil::crefFirstSubs(cref.clone())?;
        if !(subs.clone().is_empty()) {
            subs = listAppend(AbsynUtil::crefFirstSubs(repl_cr.clone())?, subs.clone());
            repl_cr = AbsynUtil::crefSetLastSubs(repl_cr.clone(), subs.clone())?;
        }
        cref = AbsynUtil::crefReplaceFirst(cref.clone(), repl_cr.clone())?;
    }
    Ok(cref)
}

pub fn mergeScalarsAlgs(mut algs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut nameMap: MergeNameMap) -> Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> {
    let mut algs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = algs;
    algs = {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
        for mut a in (algs.clone()).into_iter().cloned() {
            let __x = SCodeUtil::mapAlgorithmStatements(a.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsStmt(__pe_a0, __pe_b1.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    algs
}

pub fn mergeScalarsStmt(mut stmt: Arc<SCode::Statement>, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Statement>> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    stmt = SCodeUtil::mapStatementExps(stmt.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone()) }))?;
    Ok(stmt)
}

pub fn mergeScalarsComponentBindings(mut node: Arc<InstNode::InstNode>, mut nameMap: MergeNameMap) -> Result<()> {
    let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
    let mut cls_tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    cls = InstNode::getClass(node.clone())?;
    cls_tree = Class::classTree(cls.clone())?;
    ClassTree::applyComponents(cls_tree.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsComponentBinding(__pe_a0, __pe_b1.clone()) }));
    cls = Class::setClassTree(cls_tree.clone(), cls.clone())?;
    InstNode::updateClass(cls.clone(), node.clone())?;
    Ok(())
}

pub fn mergeScalarsComponentBinding(mut node: Arc<InstNode::InstNode>, mut nameMap: MergeNameMap) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    if !(InstNode::isComponent(node.clone())) {
        return Ok(());
    }
    comp = InstNode::component(node.clone())?;
    let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT_DEF { .. } => {
            assign_variant_field!(comp => Component::NFComponent::COMPONENT_DEF; definition = mergeScalarsElement(var_field!((*comp).definition, Component::NFComponent::COMPONENT_DEF).clone(), nameMap.clone()));
            InstNode::updateComponent(comp.clone(), node.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn createExtractorModel(mut flatModel: Arc<FlatModel::NFFlatModel>, mut funcs: Arc<NFFlatten::FunctionTreeImpl::Tree>) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<NFFlatten::FunctionTreeImpl::Tree>)> {
    let mut extractorModel: Arc<FlatModel::NFFlatModel> = flatModel.clone();
    let mut outFuncs: Arc<NFFlatten::FunctionTreeImpl::Tree> = funcs.clone();
    let mut top_level_connectors: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut flows: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut inputs: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut fn_template: Arc<Function::Function>;
    let mut index: i32 = 0;
    let mut eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut eq: Arc<Equation::NFEquation>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    (top_level_connectors, flows, inputs) = collectExtractorModelVariables(flatModel.variables.clone())?;
    fn_template = createExtractorModelDummyFn(top_level_connectors.clone())?;
    args = {
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut c in (top_level_connectors.clone()).into_iter().cloned() {
            let __x = Expression::fromCref(Variable::name(c.clone()), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    for mut f in &*flows.clone() {
        let mut f = f.clone();
        (eq, outFuncs, index) = createExtractorModelDummyEq(f.clone(), (literal!("flow")).clone(), fn_template.clone(), args.clone(), outFuncs.clone(), index.clone())?;
        eqs = cons(eq.clone(), eqs.clone());
    }
    for mut i in &*inputs.clone() {
        let mut i = i.clone();
        (eq, outFuncs, index) = createExtractorModelDummyEq(i.clone(), (literal!("input")).clone(), fn_template.clone(), args.clone(), outFuncs.clone(), index.clone())?;
        eqs = cons(eq.clone(), eqs.clone());
    }
    eqs = eqs.clone().reverse();
    assign_field!(extractorModel.equations = listAppend(extractorModel.equations.clone(), eqs.clone()));
    Ok((extractorModel, outFuncs))
}

pub fn collectExtractorModelVariables(mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut topLevelConnectorVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut flowVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut inputVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut top_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    for mut var in &*vars.clone().reverse() {
        let mut var = var.clone();
        if !(ComponentRef::isSimple(var.name.clone())) {
            top_node = ComponentRef::node(ComponentRef::last(var.name.clone()))?;
            if InstNode::isConnector(top_node.clone())? && InstNode::isPublic(top_node.clone()) {
                topLevelConnectorVars = cons(var.clone(), topLevelConnectorVars.clone());
                if Variable::isFlow(var.clone()) {
                    flowVars = cons(var.clone(), flowVars.clone());
                } else if Variable::isInput(var.clone()) {
                    inputVars = cons(var.clone(), flowVars.clone());
                }
            }
        }
    }
    Ok((topLevelConnectorVars, flowVars, inputVars))
}

pub static REAL_TYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() }), arrayDim: None }) });

pub fn createExtractorModelDummyFn(mut connectors: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<Function::Function>> {
    let mut r#fn: Arc<Function::Function>;
    let mut cdef: Arc<SCode::ClassDef>;
    let mut output_param: Arc<SCode::Element>;
    let mut elem: Arc<SCode::Element>;
    let mut fn_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut params: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut output_binding: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut cmt: Arc<SCode::Comment>;
    output_binding = SCodeUtil::makeMod(false, false, metamodelica::nil(), Some(Arc::new(Absyn::Exp::INTEGER { value: 0 })), None, Absyn::dummyInfo.clone());
    output_param = Arc::new(SCode::Element::COMPONENT { name: (literal!("dummy")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultOutputAttr.clone(), typeSpec: REAL_TYPE_SPEC.clone(), modifications: output_binding.clone(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
    params = listAppend({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (connectors.clone()).into_iter().cloned() {
            let __x = createExtractorModelDummyFnInput(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, list![output_param.clone()]);
    cdef = Arc::new(SCode::ClassDef::PARTS { elementLst: params.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None });
    cmt = Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: SCodeUtil::makeMod(false, false, list![Arc::new(SCode::SubMod { ident: (literal!("Inline")).clone(), r#mod: SCodeUtil::makeMod(false, false, metamodelica::nil(), Some(Arc::new(Absyn::Exp::BOOL { value: false })), None, Absyn::dummyInfo.clone()) })], None, None, Absyn::dummyInfo.clone()) })), comment: None });
    elem = Arc::new(SCode::Element::CLASS { name: (literal!("dummy")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: openmodelica_ast::Absyn::FunctionPurity::PURE } }, classDef: cdef.clone(), cmt: cmt.clone(), info: Absyn::dummyInfo.clone() });
    fn_node = InstNode::new(elem.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?;
    fn_node = Function::instFunctionNode(fn_node.clone(), NFInstContext::FUNCTION.clone(), Absyn::dummyInfo.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(fn_node.clone(), NFInstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa0.clone();
    Ok(r#fn)
}

pub fn createExtractorModelDummyFnInput(mut var: Arc<Variable::NFVariable>) -> Result<Arc<SCode::Element>> {
    let mut inputElem: Arc<SCode::Element>;
    inputElem = Arc::new(SCode::Element::COMPONENT { name: (ComponentRef::toFlatString(var.name.clone(), BaseModelica::defaultFormat.clone())?).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultInputAttr.clone(), typeSpec: REAL_TYPE_SPEC.clone(), modifications: Arc::new(openmodelica_frontend_types::SCode::Mod::NOMOD), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
    Ok(inputElem)
}

pub fn createExtractorModelDummyEq(mut var: Arc<Variable::NFVariable>, mut varType: ArcStr, mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut funcs: Arc<NFFlatten::FunctionTreeImpl::Tree>, mut index: i32) -> Result<(Arc<Equation::NFEquation>, Arc<NFFlatten::FunctionTreeImpl::Tree>, i32)> {
    let mut eq: Arc<Equation::NFEquation>;
    let mut funcs: Arc<NFFlatten::FunctionTreeImpl::Tree> = funcs;
    let mut index: i32 = index;
    let mut indexed_fn: Arc<Function::Function>;
    let mut fn_name: Arc<Absyn::Path>;
    let mut src: Arc<DAE::ElementSource> = DAE::emptyElementSource.clone();
    let mut var_name: ArcStr = arcstr::literal!("");
    while true {
        index = index.clone() + 1;
        fn_name = Arc::new(Absyn::Path::IDENT { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("f")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); ArcStr::from(__mm_s) }).clone() });
        if !(NFFlatten::FunctionTreeImpl::hasKey(funcs.clone(), fn_name.clone())?) {
            break;
        }
    }
    indexed_fn = Function::setName(fn_name.clone(), r#fn.clone());
    var_name = (ComponentRef::toString(Variable::name(var.clone()))?).clone();
    src = ElementSource::addCommentToSource(src.clone(), Some(Arc::new(SCode::Comment { annotation_: None, comment: Some(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dummy equation for ")); __mm_s.push_str(&*var_name.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*varType.clone()); __mm_s.push_str(&*literal!(" variable")); ArcStr::from(__mm_s) }).clone()) })));
    eq = Equation::makeEquality(Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((0) as f64) }), Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(indexed_fn.clone(), args.clone(), Variability::CONTINUOUS.clone(), Purity::PURE.clone(), indexed_fn.returnType.clone()) }), Arc::new(crate::NFType::REAL), src.clone(), r#fn.node.clone(), Equation::ScalarizeMode::NO_PREFERENCE.clone());
    funcs = NFFlatten::FunctionTreeImpl::add(funcs.clone(), fn_name.clone(), indexed_fn.clone(), NFFlatten::FunctionTreeImpl::addConflictDefault)?;
    Ok((eq, funcs, index))
}

