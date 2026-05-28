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
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFFunctionInverse as FunctionInverse;
use crate::NFInline as Inline;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Visibility;
use crate::NFScalarize as Scalarize;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTyping as Typing;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NFFlatModel {
    pub name: Arc<Absyn::Path>,
    pub variables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>,
    pub equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
    pub initialEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
    pub algorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>,
    pub initialAlgorithms: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>,
    pub source: Arc<ElementSource>,
}

impl Default for NFFlatModel {
    fn default() -> Self {
        Self {
            name: Default::default(),
            variables: Default::default(),
            equations: Default::default(),
            initialEquations: Default::default(),
            algorithms: Default::default(),
            initialAlgorithms: Default::default(),
            source: Default::default(),
        }
    }
}

pub type FLAT_MODEL = NFFlatModel;

pub type TypeMap = Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>;

pub fn mapExp(mut flatModel: Arc<NFFlatModel>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Arc<NFFlatModel> {
    pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut flatModel: Arc<NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = Variable::mapExpShallow(v.clone(), r#fn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        flatModel.equations = Equation::mapExpList(flatModel.equations.clone(), r#fn.clone()),
        flatModel.initialEquations = Equation::mapExpList(flatModel.initialEquations.clone(), r#fn.clone()),
        flatModel.algorithms = Algorithm::mapExpList(flatModel.algorithms.clone(), r#fn.clone()),
        flatModel.initialAlgorithms = Algorithm::mapExpList(flatModel.initialAlgorithms.clone(), r#fn.clone())
    );
    flatModel
}

pub fn mapEquations(mut flatModel: Arc<NFFlatModel>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> + 'static>) -> Arc<NFFlatModel> {
    pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> + 'static>;

    let mut flatModel: Arc<NFFlatModel> = flatModel;
    assign_field!(
        flatModel.equations = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (flatModel.equations.clone()).into_iter().cloned() {
            let __x = Equation::map(eq.clone(), r#fn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        flatModel.initialEquations = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (flatModel.initialEquations.clone()).into_iter().cloned() {
            let __x = Equation::map(eq.clone(), r#fn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
    );
    flatModel
}

pub fn mapAlgorithms(mut flatModel: Arc<NFFlatModel>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>) -> Arc<NFFlatModel> {
    pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>;

    let mut flatModel: Arc<NFFlatModel> = flatModel;
    assign_field!(
        flatModel.algorithms = ({
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut alg in (flatModel.algorithms.clone()).into_iter().cloned() {
            let __x = r#fn(alg.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        flatModel.initialAlgorithms = ({
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut alg in (flatModel.initialAlgorithms.clone()).into_iter().cloned() {
            let __x = r#fn(alg.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
    );
    flatModel
}

pub fn fullName(mut flatModel: Arc<NFFlatModel>) -> ArcStr {
    let mut name: ArcStr = AbsynUtil::pathString(flatModel.name.clone(), (literal!(".")).clone(), true, false).unwrap();
    name
}

pub fn className(mut flatModel: Arc<NFFlatModel>) -> ArcStr {
    let mut name: ArcStr = AbsynUtil::pathLastIdent(flatModel.name.clone()).unwrap();
    name
}

pub fn toString(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>, mut printBindingTypes: bool) -> ArcStr {
    let mut r#str: ArcStr = IOStream::string(toStream(flatModel.clone(), functions.clone(), printBindingTypes.clone()).unwrap()).unwrap();
    r#str
}

pub fn printString(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>, mut printBindingTypes: bool) -> Result<()> {
    let mut s: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
    s = toStream(flatModel.clone(), functions.clone(), printBindingTypes.clone())?;
    IOStream::print(s.clone(), IOStream::stdOutput.clone())?;
    Ok(())
}

pub fn toStream(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>, mut printBindingTypes: bool) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
    s = IOStream::create((literal!("NFFlatModel.toStream")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = appendStream(flatModel.clone(), functions.clone(), printBindingTypes.clone(), s.clone())?;
    Ok(s)
}

pub fn appendStream(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>, mut printBindingTypes: bool, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut name: ArcStr = className(flatModel.clone());
    for mut r#fn in &*Flatten::FunctionTreeImpl::listValues(functions.clone(), metamodelica::nil()) {
        let mut r#fn = r#fn.clone();
        s = Function::toStream(r#fn.clone(), (literal!("")).clone(), s.clone())?;
        s = IOStream::append(s.clone(), (literal!(";\n\n")).clone())?;
    }
    s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("class ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    for mut v in &*flatModel.variables.clone() {
        let mut v = v.clone();
        s = Variable::toStream(v.clone(), (literal!("  ")).clone(), printBindingTypes.clone(), s.clone())?;
        s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
    }
    if !(flatModel.initialEquations.clone().is_empty()) {
        s = IOStream::append(s.clone(), (literal!("initial equation\n")).clone())?;
        s = Equation::toStreamList(flatModel.initialEquations.clone(), (literal!("  ")).clone(), s.clone())?;
    }
    if !(flatModel.equations.clone().is_empty()) {
        s = IOStream::append(s.clone(), (literal!("equation\n")).clone())?;
        s = Equation::toStreamList(flatModel.equations.clone(), (literal!("  ")).clone(), s.clone())?;
    }
    for mut alg in &*flatModel.initialAlgorithms.clone() {
        let mut alg = alg.clone();
        if !(alg.statements.clone().is_empty()) {
            s = IOStream::append(s.clone(), (literal!("initial algorithm\n")).clone())?;
            s = Statement::toStreamList(alg.statements.clone(), (literal!("  ")).clone(), s.clone())?;
        }
    }
    for mut alg in &*flatModel.algorithms.clone() {
        let mut alg = alg.clone();
        if !(alg.statements.clone().is_empty()) {
            s = IOStream::append(s.clone(), (literal!("algorithm\n")).clone())?;
            s = Statement::toStreamList(alg.statements.clone(), (literal!("  ")).clone(), s.clone())?;
        }
    }
    s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("end ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())?;
    Ok(s)
}

pub fn toFlatString(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>, mut printBindingTypes: bool) -> ArcStr {
    let mut r#str: ArcStr = IOStream::string(toFlatStream(flatModel.clone(), functions.clone(), printBindingTypes.clone()).unwrap()).unwrap();
    r#str
}

pub fn printFlatString(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>, mut printBindingTypes: bool) -> Result<()> {
    let mut s: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
    s = toFlatStream(flatModel.clone(), functions.clone(), printBindingTypes.clone())?;
    IOStream::print(s.clone(), IOStream::stdOutput.clone())?;
    Ok(())
}

pub fn toFlatStream(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>, mut printBindingTypes: bool) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
    s = IOStream::create((className(flatModel.clone())).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = appendFlatStream(flatModel.clone(), functions.clone(), printBindingTypes.clone(), s.clone())?;
    Ok(s)
}

pub fn appendFlatStream(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<Flatten::FunctionTreeImpl::Tree>, mut printBindingTypes: bool, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut flat_model: Arc<NFFlatModel> = flatModel.clone();
    let mut name: ArcStr = Util::makeQuotedIdentifier((className(flatModel.clone())).clone())?;
    let mut format: BaseModelica::OutputFormat = <BaseModelica::OutputFormat as ::std::default::Default>::default();
    let mut scalarize: bool = false;
    let mut funcs: Arc<metamodelica::List<Arc<Function::Function>>> = Flatten::FunctionTreeImpl::listValues(functions.clone(), metamodelica::nil());
    format = BaseModelica::formatFromFlags()?;
    scalarize = Flags::isConfigFlagSet(Flags::BASE_MODELICA_OPTIONS.clone(), (literal!("scalarize")).clone())?;
    if Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("protected") || Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("encrypted") {
        flat_model = obfuscate(flat_model.clone())?;
    }
    if BaseModelica::inlineFunctions() {
        (flat_model, funcs) = inlineFunctions(flat_model.clone());
    }
    if scalarize.clone() {
        assign_field!(
            flat_model.variables = Scalarize::scalarizeVariables(flat_model.variables.clone(), true)?,
            flat_model.equations = Equation::splitRecordEquations(flat_model.equations.clone())?,
            flat_model.equations = Scalarize::scalarizeEquations(flat_model.equations.clone(), true)?,
            flat_model.initialEquations = Equation::splitRecordEquations(flat_model.initialEquations.clone())?,
            flat_model.initialEquations = Scalarize::scalarizeEquations(flat_model.initialEquations.clone(), true)?,
            flat_model.algorithms = ({
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (flat_model.algorithms.clone()).into_iter().cloned() {
            let __x = Flatten::unrollForStatementsInAlg(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
            flat_model.initialAlgorithms = ({
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (flat_model.initialAlgorithms.clone()).into_iter().cloned() {
            let __x = Flatten::unrollForStatementsInAlg(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        );
        flat_model = mapExp(flat_model.clone(), (std::sync::Arc::new(ExpandExp::expandCallArgs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>));
    } else {
        assign_field!(
            flat_model.variables = reconstructRecordInstances(flat_model.variables.clone())?,
            flat_model.variables = List::filterOnFalse(flat_model.variables.clone(), (std::sync::Arc::new(fnptr!(Variable::isEmptyArray, Arc<Variable::NFVariable>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<bool> + 'static>))
        );
    }
    if format.moveBindings.clone() {
        flat_model = moveBindings(flat_model.clone())?;
    }
    s = IOStream::append(s.clone(), (literal!("//! base 0.1.0\n")).clone())?;
    s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("package ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    for mut r#fn in &*funcs.clone() {
        let mut r#fn = r#fn.clone();
        if !(Function::isDefaultRecordConstructor(r#fn.clone()) || Function::isExternalObjectConstructorOrDestructor(r#fn.clone())?) {
            s = Function::toFlatStream(r#fn.clone(), BaseModelica::defaultFormat.clone(), (literal!("  ")).clone(), s.clone(), (literal!("")).clone())?;
            s = IOStream::append(s.clone(), (literal!(";\n\n")).clone())?;
        }
    }
    for mut ty in &*collectFlatTypes(flat_model.clone(), funcs.clone())? {
        let mut ty = ty.clone();
        s = Type::toFlatDeclarationStream(ty.clone(), format.clone(), (literal!("  ")).clone(), s.clone())?;
        s = IOStream::append(s.clone(), (literal!(";\n\n")).clone())?;
    }
    s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  model ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone())?;
    s = FlatModelicaUtil::appendElementSourceCommentString(flat_model.source.clone(), s.clone())?;
    s = IOStream::append(s.clone(), (literal!("\n")).clone())?;
    for mut v in &*flat_model.variables.clone() {
        let mut v = v.clone();
        s = Variable::toFlatStream(v.clone(), format.clone(), (literal!("    ")).clone(), printBindingTypes.clone(), s.clone())?;
        s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
    }
    if !(flat_model.initialEquations.clone().is_empty()) {
        s = IOStream::append(s.clone(), (literal!("  initial equation\n")).clone())?;
        s = Equation::toFlatStreamList(flat_model.initialEquations.clone(), format.clone(), (literal!("    ")).clone(), s.clone())?;
    }
    if !(flat_model.equations.clone().is_empty()) {
        s = IOStream::append(s.clone(), (literal!("  equation\n")).clone())?;
        s = Equation::toFlatStreamList(flat_model.equations.clone(), format.clone(), (literal!("    ")).clone(), s.clone())?;
    }
    for mut alg in &*flat_model.initialAlgorithms.clone() {
        let mut alg = alg.clone();
        if !(alg.statements.clone().is_empty()) {
            s = IOStream::append(s.clone(), (literal!("  initial algorithm\n")).clone())?;
            s = Statement::toFlatStreamList(alg.statements.clone(), format.clone(), (literal!("    ")).clone(), s.clone())?;
        }
    }
    for mut alg in &*flat_model.algorithms.clone() {
        let mut alg = alg.clone();
        if !(alg.statements.clone().is_empty()) {
            s = IOStream::append(s.clone(), (literal!("  algorithm\n")).clone())?;
            s = Statement::toFlatStreamList(alg.statements.clone(), format.clone(), (literal!("    ")).clone(), s.clone())?;
        }
    }
    s = FlatModelicaUtil::appendElementSourceCommentAnnotation(flat_model.source.clone(), FlatModelicaUtil::ElementType::ROOT_CLASS.clone(), (literal!("    ")).clone(), (literal!(";\n")).clone(), s.clone())?;
    s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  end ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())?;
    s = IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("end ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())?;
    Ok(s)
}

pub fn inlineFunctions(mut flatModel: Arc<NFFlatModel>) -> (Arc<NFFlatModel>, Arc<metamodelica::List<Arc<Function::Function>>>) {
    let mut flatModel: Arc<NFFlatModel> = flatModel;
    let mut remainingFuncs: Arc<metamodelica::List<Arc<Function::Function>>> = metamodelica::nil();
    let mut funcs: Arc<UnorderedSet::UnorderedSet<Arc<Function::Function>>>;
    funcs = UnorderedSet::new((std::sync::Arc::new(fnptr!(Function::nameHash, Arc<Function::Function>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(Function::nameEqual, Arc<Function::Function>, Arc<Function::Function>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>, Arc<Function::Function>) -> Result<bool> + 'static>), 13);
    flatModel = mapExp(flatModel.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = Arc::new({ let __pe_b1 = funcs.clone(); move |__pe_a0| inlineFunctions_traverser(__pe_a0, __pe_b1.clone()) }); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }));
    remainingFuncs = UnorderedSet::toList(funcs.clone());
    (flatModel, remainingFuncs)
}

pub fn inlineFunctions_traverser(mut exp: Arc<Expression::NFExpression>, mut funcs: Arc<UnorderedSet::UnorderedSet<Arc<Function::Function>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut r#fn: Arc<Function::Function> = Arc::new(<Function::Function as ::std::default::Default>::default());
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } => {
            r#fn = Call::typedFunction(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?;
            if Function::isBuiltin(r#fn.clone()) {
                outExp = exp.clone();
            } else {
                outExp = Inline::inlineCallExp(exp.clone(), true)?;
                if referenceEq(&exp.clone(),&outExp.clone()) {
                    collectFunction(r#fn.clone(), funcs.clone())?;
                } else {
                    Expression::apply(outExp.clone(), Arc::new({ let __pe_b1 = funcs.clone(); move |__pe_a0| collectFunctions(__pe_a0, __pe_b1.clone()) }))?;
                }
            }
            outExp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn collectFunctions(mut exp: Arc<Expression::NFExpression>, mut funcs: Arc<UnorderedSet::UnorderedSet<Arc<Function::Function>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } => {
            collectFunction(Call::typedFunction(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?, funcs.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn collectFunction(mut r#fn: Arc<Function::Function>, mut funcs: Arc<UnorderedSet::UnorderedSet<Arc<Function::Function>>>) -> Result<()> {
    if !(Function::isBuiltin(r#fn.clone())) {
        UnorderedSet::add(r#fn.clone(), funcs.clone())?;
        for mut fn_der in &*r#fn.derivatives.clone() {
            let mut fn_der = fn_der.clone();
            for mut der_fn in &*Function::getCachedFuncs(fn_der.derivativeFn.clone())? {
                let mut der_fn = der_fn.clone();
                UnorderedSet::add(der_fn.clone(), funcs.clone())?;
            }
        }
        let __range0 = r#fn.inverses.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut fn_inv in __range0 {
            UnorderedSet::add(FunctionInverse::getFunction(fn_inv.clone())?, funcs.clone())?;
        }
    }
    Ok(())
}

pub fn collectFlatTypes(mut flatModel: Arc<NFFlatModel>, mut functions: Arc<metamodelica::List<Arc<Function::Function>>>) -> Result<Arc<metamodelica::List<Arc<Type::NFType>>>> {
    let mut outTypes: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
    let mut types: TypeMap;
    types = UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1);
    List::map1_0(flatModel.variables.clone(), (std::sync::Arc::new(collectVariableFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
    List::map1_0(flatModel.equations.clone(), (std::sync::Arc::new(collectEquationFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
    List::map1_0(flatModel.initialEquations.clone(), (std::sync::Arc::new(collectEquationFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
    List::map1_0(flatModel.algorithms.clone(), (std::sync::Arc::new(collectAlgorithmFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
    List::map1_0(flatModel.initialAlgorithms.clone(), (std::sync::Arc::new(collectAlgorithmFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
    List::map1_0(functions.clone(), (std::sync::Arc::new(collectFunctionFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
    outTypes = UnorderedMap::valueList(types.clone());
    outTypes = ({
        let mut __acc: Arc<metamodelica::List<Arc<Type::NFType>>> = metamodelica::nil();
        for mut ty in (outTypes.clone()).into_iter().cloned() {
            let __x = typeFlatType(ty.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outTypes)
}

pub fn collectVariableFlatTypes(mut var: Arc<Variable::NFVariable>, mut types: TypeMap) -> Result<()> {
    collectFlatType(var.ty.clone(), types.clone())?;
    collectBindingFlatTypes(var.binding.clone(), types.clone())?;
    for mut attr in &*var.typeAttributes.clone() {
        let mut attr = attr.clone();
        collectBindingFlatTypes(Util::tuple22(attr.clone()), types.clone())?;
    }
    Ok(())
}

pub fn collectFlatType(mut ty: Arc<Type::NFType>, mut types: TypeMap) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ENUMERATION { .. } if (!(Type::isBuiltinEnumeration(ty.clone()))) => {
            UnorderedMap::tryAdd(var_field!((*ty).typePath, Type::NFType::ENUMERATION).clone(), ty.clone(), types.clone())?;
            ()
        },
        Deref @ Type::ARRAY { .. } => {
            Dimension::foldExpList(var_field!((*ty).dimensions, Type::NFType::ARRAY).clone(), (std::sync::Arc::new(collectExpFlatTypes_traverse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>> + 'static>), types.clone())?;
            collectFlatType(var_field!((*ty).elementType, Type::NFType::ARRAY).clone(), types.clone())?;
            ()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => {
            UnorderedMap::tryAdd(InstNode::scopePath(var_field!((*ty).cls, Type::NFType::COMPLEX).clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, ty.clone(), types.clone())?;
            ()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => {
            UnorderedMap::tryAdd(InstNode::scopePath(var_field!((*ty).cls, Type::NFType::COMPLEX).clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, ty.clone(), types.clone())?;
            ()
        },
        Deref @ Type::FUNCTION { fnType: Type::FunctionType::FUNCTIONAL_PARAMETER, .. } => {
            UnorderedMap::tryAdd(InstNode::scopePath(var_field!((*ty).r#fn, Type::NFType::FUNCTION).node.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, ty.clone(), types.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn collectBindingFlatTypes(mut binding: Arc<Binding::NFBinding>, mut types: TypeMap) -> Result<()> {
    if Binding::isExplicitlyBound(binding.clone()) {
        collectExpFlatTypes(Binding::getTypedExp(binding.clone())?, types.clone())?;
    }
    Ok(())
}

pub fn collectEquationFlatTypes(mut eq: Arc<Equation::NFEquation>, mut types: TypeMap) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            collectExpFlatTypes(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), types.clone())?;
            collectExpFlatTypes(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), types.clone())?;
            collectFlatType(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone(), types.clone())?;
            ()
        },
        Deref @ Equation::FOR { .. } => {
            if isSome(var_field!((*eq).range, Equation::NFEquation::FOR).clone()) {
                collectExpFlatTypes(Util::getOption(var_field!((*eq).range, Equation::NFEquation::FOR).clone())?, types.clone())?;
            }
            List::map1_0(var_field!((*eq).body, Equation::NFEquation::FOR).clone(), (std::sync::Arc::new(collectEquationFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
            ()
        },
        Deref @ Equation::IF { .. } => {
            List::map1_0(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), (std::sync::Arc::new(collectEqBranchFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Branch::Branch>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
            ()
        },
        Deref @ Equation::WHEN { .. } => {
            List::map1_0(var_field!((*eq).branches, Equation::NFEquation::WHEN).clone(), (std::sync::Arc::new(collectEqBranchFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Branch::Branch>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
            ()
        },
        Deref @ Equation::ASSERT { .. } => {
            collectExpFlatTypes(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), types.clone())?;
            collectExpFlatTypes(var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), types.clone())?;
            collectExpFlatTypes(var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), types.clone())?;
            ()
        },
        Deref @ Equation::TERMINATE { .. } => {
            collectExpFlatTypes(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), types.clone())?;
            ()
        },
        Deref @ Equation::REINIT { .. } => {
            collectExpFlatTypes(var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), types.clone())?;
            ()
        },
        Deref @ Equation::NORETCALL { .. } => {
            collectExpFlatTypes(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), types.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn collectEqBranchFlatTypes(mut branch: Arc<Equation::Branch::Branch>, mut types: TypeMap) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } => {
            collectExpFlatTypes(var_field!((*branch).condition, Equation::Branch::Branch::BRANCH).clone(), types.clone())?;
            List::map1_0(var_field!((*branch).body, Equation::Branch::Branch::BRANCH).clone(), (std::sync::Arc::new(collectEquationFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn collectAlgorithmFlatTypes(mut alg: Arc<Algorithm::NFAlgorithm>, mut types: TypeMap) -> Result<()> {
    collectStatementsFlatTypes(alg.statements.clone(), types.clone())?;
    Ok(())
}

pub fn collectStatementsFlatTypes(mut statements: Arc<metamodelica::List<Arc<Statement::NFStatement>>>, mut types: TypeMap) -> Result<()> {
    for mut s in &*statements.clone() {
        let mut s = s.clone();
        collectStatementFlatTypes(s.clone(), types.clone())?;
    }
    Ok(())
}

pub fn collectStatementFlatTypes(mut stmt: Arc<Statement::NFStatement>, mut types: TypeMap) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            collectExpFlatTypes(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), types.clone())?;
            collectExpFlatTypes(var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), types.clone())?;
            collectFlatType(var_field!((*stmt).ty, Statement::NFStatement::ASSIGNMENT).clone(), types.clone())?;
            ()
        },
        Deref @ Statement::FOR { .. } => {
            collectStatementsFlatTypes(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), types.clone())?;
            collectExpFlatTypes(Util::getOption(var_field!((*stmt).range, Statement::NFStatement::FOR).clone())?, types.clone())?;
            ()
        },
        Deref @ Statement::IF { .. } => {
            List::map1_0(var_field!((*stmt).branches, Statement::NFStatement::IF).clone(), (std::sync::Arc::new(collectStmtBranchFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
            ()
        },
        Deref @ Statement::WHEN { .. } => {
            List::map1_0(var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone(), (std::sync::Arc::new(collectStmtBranchFlatTypes) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<()> + 'static>), types.clone());
            ()
        },
        Deref @ Statement::ASSERT { .. } => {
            collectExpFlatTypes(var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), types.clone())?;
            collectExpFlatTypes(var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), types.clone())?;
            collectExpFlatTypes(var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), types.clone())?;
            ()
        },
        Deref @ Statement::TERMINATE { .. } => {
            collectExpFlatTypes(var_field!((*stmt).message, Statement::NFStatement::TERMINATE).clone(), types.clone())?;
            ()
        },
        Deref @ Statement::REINIT { .. } => {
            collectExpFlatTypes(var_field!((*stmt).cref, Statement::NFStatement::REINIT).clone(), types.clone())?;
            collectExpFlatTypes(var_field!((*stmt).reinitExp, Statement::NFStatement::REINIT).clone(), types.clone())?;
            ()
        },
        Deref @ Statement::NORETCALL { .. } => {
            collectExpFlatTypes(var_field!((*stmt).exp, Statement::NFStatement::NORETCALL).clone(), types.clone())?;
            ()
        },
        Deref @ Statement::WHILE { .. } => {
            collectExpFlatTypes(var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), types.clone())?;
            collectStatementsFlatTypes(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone(), types.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn collectStmtBranchFlatTypes(mut branch: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), mut types: TypeMap) -> Result<()> {
    collectExpFlatTypes(Util::tuple21(branch.clone()), types.clone())?;
    collectStatementsFlatTypes(Util::tuple22(branch.clone()), types.clone())?;
    Ok(())
}

pub fn collectExpFlatTypes(mut exp: Arc<Expression::NFExpression>, mut types: TypeMap) -> Result<()> {
    Expression::fold(exp.clone(), (std::sync::Arc::new(collectExpFlatTypes_traverse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Type::NFType>>>> + 'static>), types.clone())?;
    Ok(())
}

pub fn collectExpFlatTypes_traverse(mut exp: Arc<Expression::NFExpression>, mut types: TypeMap) -> Result<TypeMap> {
    let mut types: TypeMap = types;
    collectFlatType(Expression::typeOf(exp.clone()), types.clone())?;
    Ok(types)
}

pub fn collectFunctionFlatTypes(mut r#fn: Arc<Function::Function>, mut types: TypeMap) -> Result<()> {
    ClassTree::applyComponents(Class::classTree(InstNode::getClass(r#fn.node.clone())?)?, Arc::new({ let __pe_b1 = types.clone(); move |__pe_a0| collectComponentFlatTypes(__pe_a0, __pe_b1.clone()) }));
    if !(Function::isExternal(r#fn.clone())) {
        collectStatementsFlatTypes(Function::getBody(r#fn.clone()), types.clone())?;
    }
    Ok(())
}

pub fn collectComponentFlatTypes(mut component: Arc<InstNode::InstNode>, mut types: TypeMap) -> Result<()> {
    let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    comp = InstNode::component(component.clone())?;
    collectFlatType(Component::getType(comp.clone())?, types.clone())?;
    collectBindingFlatTypes(Component::getBinding(comp.clone()), types.clone())?;
    Ok(())
}

pub fn reconstructRecordInstances(mut variables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<metamodelica::List<Arc<Variable::NFVariable>>>> {
    let mut outVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut rest_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = variables.clone();
    let mut record_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut parent_cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut parent_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut field_count: i32 = 0;
    while !(rest_vars.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_vars.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        var = __pa0.clone();
        rest_vars = __pa1.clone();
        parent_cr = ComponentRef::rest(var.name.clone())?;
        if !(ComponentRef::isEmpty(parent_cr.clone())) {
            parent_ty = ComponentRef::nodeType(parent_cr.clone())?;
            if Type::isRecord(parent_ty.clone()) {
                field_count = (Type::recordFields(parent_ty.clone()).len() as i32);
                (record_vars, rest_vars) = List::split(rest_vars.clone(), field_count.clone() - 1)?;
                record_vars = cons(var.clone(), record_vars.clone());
                var = reconstructRecordInstance(parent_cr.clone(), record_vars.clone())?;
            }
        }
        outVariables = cons(var.clone(), outVariables.clone());
    }
    outVariables = outVariables.clone().reverse();
    Ok(outVariables)
}

pub fn reconstructRecordInstance(mut recordName: Arc<ComponentRef::NFComponentRef>, mut variables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<Variable::NFVariable>> {
    let mut recordVar: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
    let mut record_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut record_comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
    let mut record_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut field_exps: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut record_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut record_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    record_node = ComponentRef::node(recordName.clone())?;
    record_comp = InstNode::component(record_node.clone())?;
    record_ty = ComponentRef::nodeType(recordName.clone())?;
    field_exps = metamodelica::nil();
    for mut v in &*variables.clone() {
        let mut v = v.clone();
        if Binding::hasExp(v.binding.clone()) {
            field_exps = cons(Binding::getExp(v.binding.clone())?, field_exps.clone());
        } else {
            field_exps = metamodelica::nil();
            break;
        }
    }
    if field_exps.clone().is_empty() {
        record_binding = Binding::EMPTY_BINDING().clone();
    } else {
        field_exps = field_exps.clone().reverse();
        record_exp = Expression::makeRecord(InstNode::scopePath(InstNode::classScope(record_node.clone()), InstNode::ScopeType::RELATIVE.clone(), false)?, record_ty.clone(), field_exps.clone());
        record_binding = Binding::makeFlat(record_exp.clone(), Component::variability(record_comp.clone()), Binding::Source::GENERATED.clone(), Binding::NO_CONFIDENCE.clone());
    }
    recordVar = Arc::new(Variable::NFVariable { name: recordName.clone(), ty: record_ty.clone(), binding: record_binding.clone(), visibility: InstNode::visibility(record_node.clone()), attributes: Component::getAttributes(record_comp.clone()), typeAttributes: metamodelica::nil(), children: variables.clone(), comment: Component::comment(record_comp.clone())?, info: InstNode::info(record_node.clone())?, backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone() });
    Ok(recordVar)
}

pub fn typeFlatType(mut ty: Arc<Type::NFType>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = ty;
    let () = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => {
            Typing::typeBindings(var_field!((*ty).cls, Type::NFType::COMPLEX).clone(), InstContext::CLASS.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub type ObfuscationMap = Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, ArcStr>>;

pub fn obfuscate(mut flatModel: Arc<NFFlatModel>) -> Result<Arc<NFFlatModel>> {
    let mut flatModel: Arc<NFFlatModel> = flatModel;
    let mut obfuscation_map: ObfuscationMap;
    let mut only_encrypted: bool = false;
    only_encrypted = Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("encrypted");
    obfuscation_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(InstNode::hash, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), 1);
    for mut v in &*flatModel.variables.clone() {
        let mut v = v.clone();
        addObfuscatedVariable(v.clone(), only_encrypted.clone(), obfuscation_map.clone())?;
    }
    assign_field!(flatModel.variables = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = obfuscateVariable(v.clone(), obfuscation_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    flatModel = mapEquations(flatModel.clone(), Arc::new({ let __pe_b1 = obfuscation_map.clone(); move |__pe_a0| obfuscateEquation(__pe_a0, __pe_b1.clone()) }));
    flatModel = mapAlgorithms(flatModel.clone(), Arc::new({ let __pe_b1 = obfuscation_map.clone(); move |__pe_a0| obfuscateAlgorithm(__pe_a0, __pe_b1.clone()) }));
    Ok(flatModel)
}

pub fn addObfuscatedVariable(mut var: Arc<Variable::NFVariable>, mut onlyEncrypted: bool, mut obfuscationMap: ObfuscationMap) -> Result<()> {
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    if Variable::isProtected(var.clone()) && (!(onlyEncrypted.clone()) || Variable::isEncrypted(var.clone())?) {
        nodes = ComponentRef::nodes(var.name.clone(), metamodelica::nil());
        nodes = List::trim(nodes.clone(), (std::sync::Arc::new(fnptr!(InstNode::isPublic, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<bool> + 'static>))?;
        for mut node in &*nodes.clone() {
            let mut node = node.clone();
            UnorderedMap::tryAdd(node.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("n")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", UnorderedMap::size(obfuscationMap.clone()) + 1))); ArcStr::from(__mm_s) }).clone(), obfuscationMap.clone())?;
        }
    }
    Ok(())
}

pub fn obfuscateVariable(mut var: Arc<Variable::NFVariable>, mut obfuscationMap: ObfuscationMap) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    assign_field!(
        var.name = obfuscateCref(var.name.clone(), obfuscationMap.clone())?.0,
        var.comment = obfuscateComment(var.comment.clone(), ComponentRef::node(var.name.clone())?, obfuscationMap.clone(), !(Variable::isAccessible(var.clone())?))
    );
    var = Variable::mapExpShallow(var.clone(), Arc::new({ let __pe_b1 = obfuscationMap.clone(); move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone()) }));
    Ok(var)
}

pub fn obfuscateCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut obfuscationMap: ObfuscationMap) -> Result<(Arc<ComponentRef::NFComponentRef>, bool)> {
    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut insideRecord: bool = false;
    let mut name: Option<ArcStr> = None;
    let mut rest_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { .. } => {
            (rest_cref, insideRecord) = obfuscateCref(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), obfuscationMap.clone())?;
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = rest_cref.clone());
            if !(insideRecord.clone()) {
                name = UnorderedMap::get(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone(), obfuscationMap.clone());
                if isSome(name.clone()) {
                    assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; node = InstNode::rename((Util::getOption(name.clone())?).clone(), var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone())?);
                }
            }
            insideRecord = InstNode::isRecord(var_field!((*cref).node, ComponentRef::NFComponentRef::CREF).clone());
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; subscripts = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone()).into_iter().cloned() {
            let __x = Subscript::mapShallowExp(s.clone(), Arc::new({ let __pe_b1 = obfuscationMap.clone(); move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cref, insideRecord))
}

pub fn obfuscateExp(mut exp: Arc<Expression::NFExpression>, mut obfuscationMap: ObfuscationMap) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp.clone(), Arc::new({ let __pe_b1 = obfuscationMap.clone(); move |__pe_a0| obfuscateExp_impl(__pe_a0, __pe_b1.clone()) }))?;
    Ok(exp)
}

pub fn obfuscateExpOpt(mut exp: Option<Arc<Expression::NFExpression>>, mut obfuscationMap: ObfuscationMap) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut exp: Option<Arc<Expression::NFExpression>> = exp;
    if isSome(exp.clone()) {
        exp = Some(obfuscateExp(Util::getOption(exp.clone())?, obfuscationMap.clone())?);
    }
    Ok(exp)
}

pub fn obfuscateExp_impl(mut exp: Arc<Expression::NFExpression>, mut obfuscationMap: ObfuscationMap) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = obfuscateCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), obfuscationMap.clone())?.0);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn obfuscateEquation(mut eq: Arc<Equation::NFEquation>, mut obfuscationMap: ObfuscationMap) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    eq = Equation::setSource(obfuscateSource(Equation::source(eq.clone())?, Equation::scope(eq.clone())?, obfuscationMap.clone()), eq.clone())?;
    eq = Equation::mapExpShallow(eq.clone(), Arc::new({ let __pe_b1 = obfuscationMap.clone(); move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone()) }));
    Ok(eq)
}

pub fn obfuscateAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut obfuscationMap: ObfuscationMap) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(
        alg.source = obfuscateSource(alg.source.clone(), alg.scope.clone(), obfuscationMap.clone()),
        alg.inputs = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut e in (alg.inputs.clone()).into_iter().cloned() {
            let __x = (obfuscateCref(e.clone(), obfuscationMap.clone())?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        alg.outputs = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut e in (alg.outputs.clone()).into_iter().cloned() {
            let __x = (obfuscateCref(e.clone(), obfuscationMap.clone())?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        alg.statements = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (alg.statements.clone()).into_iter().cloned() {
            let __x = Statement::map(s.clone(), Arc::new({ let __pe_b1 = alg.scope.clone(); let __pe_b2 = obfuscationMap.clone(); move |__pe_a0| obfuscateStatement(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
    );
    Ok(alg)
}

pub fn obfuscateStatement(mut stmt: Arc<Statement::NFStatement>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement> = stmt;
    stmt = Statement::setSource(obfuscateSource(Statement::source(stmt.clone())?, scope.clone(), obfuscationMap.clone()), stmt.clone())?;
    stmt = Statement::mapExpShallow(stmt.clone(), Arc::new({ let __pe_b1 = obfuscationMap.clone(); move |__pe_a0| obfuscateExp(__pe_a0, __pe_b1.clone()) }));
    Ok(stmt)
}

pub fn obfuscateSource(mut source: Arc<ElementSource>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Arc<ElementSource> {
    let mut source: Arc<ElementSource> = source;
    assign_field!(source.comment = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
        for mut c in (source.comment.clone()).into_iter().cloned() {
            let __x = obfuscateComment(c.clone(), scope.clone(), obfuscationMap.clone(), true);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    source
}

pub fn obfuscateCommentOpt(mut comment: Option<Arc<SCode::Comment>>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap, mut stripComment: bool) -> Option<Arc<SCode::Comment>> {
    let mut comment: Option<Arc<SCode::Comment>> = comment;
    comment = Util::applyOption(comment.clone(), Arc::new({ let __pe_b1 = scope.clone(); let __pe_b2 = obfuscationMap.clone(); let __pe_b3 = stripComment.clone(); move |__pe_a0| Ok(obfuscateComment(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())) }));
    comment
}

pub fn obfuscateComment(mut comment: Arc<SCode::Comment>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap, mut stripComment: bool) -> Arc<SCode::Comment> {
    let mut comment: Arc<SCode::Comment> = comment;
    assign_field!(comment.annotation_ = obfuscateAnnotationOpt(comment.annotation_.clone(), scope.clone(), obfuscationMap.clone()));
    if stripComment.clone() {
        assign_field!(comment.comment = None);
    }
    comment
}

pub fn obfuscateAnnotationOpt(mut ann: Option<Arc<SCode::Annotation>>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Option<Arc<SCode::Annotation>> {
    let mut ann: Option<Arc<SCode::Annotation>> = ann;
    ann = Util::applyOption(ann.clone(), Arc::new({ let __pe_b1 = scope.clone(); let __pe_b2 = obfuscationMap.clone(); move |__pe_a0| Ok(obfuscateAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }));
    ann
}

pub fn obfuscateAnnotation(mut ann: Arc<SCode::Annotation>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Arc<SCode::Annotation> {
    let mut ann: Arc<SCode::Annotation> = ann;
    assign_field!(ann.modification = obfuscateAnnotationMod(ann.modification.clone(), scope.clone(), obfuscationMap.clone()));
    ann
}

pub fn obfuscateAnnotationMod(mut r#mod: Arc<SCode::Mod>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Arc<SCode::Mod> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD;
                subModLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut s in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            if !(isAllowedAnnotation(s.clone())) { continue; }
            let __x = obfuscateAnnotationSubMod(s.clone(), scope.clone(), obfuscationMap.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                binding = obfuscateAbsynExpOpt(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), scope.clone(), obfuscationMap.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
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
        _ => !(StringUtil::startsWith((r#mod.ident.clone()).clone(), (literal!("__")).clone())),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    allowed
}

pub fn obfuscateAnnotationSubMod(mut r#mod: Arc<SCode::SubMod>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Arc<SCode::SubMod> {
    let mut r#mod: Arc<SCode::SubMod> = r#mod;
    assign_field!(r#mod.r#mod = obfuscateAnnotationMod(r#mod.r#mod.clone(), scope.clone(), obfuscationMap.clone()));
    r#mod
}

pub fn obfuscateAbsynExpOpt(mut exp: Option<Arc<Absyn::Exp>>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Option<Arc<Absyn::Exp>> {
    let mut exp: Option<Arc<Absyn::Exp>> = exp;
    exp = Util::applyOption(exp.clone(), Arc::new({ let __pe_b1 = scope.clone(); let __pe_b2 = obfuscationMap.clone(); move |__pe_a0| obfuscateAbsynExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }));
    exp
}

pub fn obfuscateAbsynExp(mut exp: Arc<Absyn::Exp>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(exp.clone(), Arc::new({ let __pe_b1 = scope.clone(); move |__pe_a0, __pe_a2| Ok(obfuscateAbsynExpTraverse(__pe_a0, __pe_b1.clone(), __pe_a2)) }), obfuscationMap.clone())?;
    Ok(exp)
}

pub fn obfuscateAbsynExpTraverse(mut exp: Arc<Absyn::Exp>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> (Arc<Absyn::Exp>, ObfuscationMap) {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut obfuscationMap: ObfuscationMap = obfuscationMap;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = obfuscateAbsynCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), scope.clone(), obfuscationMap.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, obfuscationMap)
}

pub fn obfuscateAbsynCref(mut cref: Arc<Absyn::ComponentRef>, mut scope: Arc<InstNode::InstNode>, mut obfuscationMap: ObfuscationMap) -> Arc<Absyn::ComponentRef> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut inst_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    ErrorExt::setCheckpoint((literal!("NFFlatModel.obfuscateAbsynCref")).clone());
    match '__try0: {
        (inst_cref, _, _) = unwrap_break_err!(Lookup::lookupCref(cref.clone(), scope.clone(), InstContext::RELAXED.clone()), '__try0);
        nodes = ({
        let mut __acc: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        for mut c in (ComponentRef::toListReverse(inst_cref.clone(), false, metamodelica::nil())).into_iter().cloned() {
            let __x = unwrap_break_err!(ComponentRef::node(c.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        cref = unwrap_break_err!(obfuscateAbsynCref2(cref.clone(), nodes.clone(), obfuscationMap.clone()), '__try0);
        Ok::<_, anyhow::Error>((cref.clone(), inst_cref.clone(), nodes.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            cref = __try0_o0;
            inst_cref = __try0_o1;
            nodes = __try0_o2;
        }
        Err(_) => {
            panic!("try/else: outputs not set in else branch");
        }
    }
    ErrorExt::rollBack((literal!("NFFlatModel.obfuscateAbsynCref")).clone());
    cref
}

pub fn obfuscateAbsynCref2(mut cref: Arc<Absyn::ComponentRef>, mut nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut obfuscationMap: ObfuscationMap) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut rest_nodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &((cref.clone(), nodes.clone())) {
        (Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }, _) => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = obfuscateAbsynCref2(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), nodes.clone(), obfuscationMap.clone())?);
            ()
        },
        (Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, Deref @ metamodelica::List::Cons { head: node, tail: rest_nodes }) if (InstNode::name(node.clone())? == var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()) => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL;
                name = UnorderedMap::getOrDefault(node.clone(), obfuscationMap.clone(), (var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone()),
                componentRef = obfuscateAbsynCref2(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), rest_nodes.clone(), obfuscationMap.clone())?
            );
            ()
        },
        (Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, Deref @ metamodelica::List::Cons { head: node, tail: _ }) if (InstNode::name(node.clone())? == var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()) => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; name = UnorderedMap::getOrDefault(node.clone(), obfuscationMap.clone(), (var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn hasArrayConnections(mut flatModel: Arc<NFFlatModel>, mut minSize: i32) -> Result<bool> {
    let mut hasArrays: bool = false;
    for mut eq in &*flatModel.equations.clone() {
        let mut eq = eq.clone();
        if Equation::contains(eq.clone(), (std::sync::Arc::new(fnptr!(Equation::isConnect, Arc<Equation::NFEquation>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>)) && Equation::sizeOf(eq.clone())? >= minSize.clone() {
            hasArrays = true;
            return Ok(hasArrays.clone());
        }
    }
    Ok(hasArrays)
}

pub fn removeNonTopLevelDirections(mut flatModel: Arc<NFFlatModel>) -> Result<Arc<NFFlatModel>> {
    let mut flatModel: Arc<NFFlatModel> = flatModel;
    if Flags::getConfigBool(Flags::USE_LOCAL_DIRECTION.clone())? {
        return Ok(flatModel.clone());
    }
    assign_field!(flatModel.variables = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = Variable::removeNonTopLevelDirection(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    Ok(flatModel)
}

pub fn moveBindings(mut flatModel: Arc<NFFlatModel>) -> Result<Arc<NFFlatModel>> {
    let mut flatModel: Arc<NFFlatModel> = flatModel;
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    for mut var in &*flatModel.variables.clone() {
        let mut var = var.clone();
        (var, eqs) = Variable::moveBinding(var.clone(), eqs.clone())?;
        vars = cons(var.clone(), vars.clone());
    }
    if !(eqs.clone().is_empty()) {
        assign_field!(
            flatModel.variables = vars.clone().reverse(),
            flatModel.equations = listAppend(eqs.clone().reverse(), flatModel.equations.clone())
        );
    }
    Ok(flatModel)
}


