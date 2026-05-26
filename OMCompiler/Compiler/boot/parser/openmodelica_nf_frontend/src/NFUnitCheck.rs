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

use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFUnit as Unit;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

//import DAE;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Functionargs {
    pub name: ArcStr,
    pub invars: Arc<metamodelica::List<ArcStr>>,
    pub outvars: Arc<metamodelica::List<ArcStr>>,
    pub inunits: Arc<metamodelica::List<ArcStr>>,
    pub outunits: Arc<metamodelica::List<ArcStr>>,
}

impl Default for Functionargs {
    fn default() -> Self {
        Self {
            name: Default::default(),
            invars: Default::default(),
            outvars: Default::default(),
            inunits: Default::default(),
            outunits: Default::default(),
        }
    }
}

pub type FUNCTIONUNITS = Functionargs;


pub type FunctionUnitCache = Arc<UnorderedMap::UnorderedMap<ArcStr, Functionargs>>;

pub fn checkUnits(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut htCr2U1: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>;
    let mut htCr2U2: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>;
    let mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>;
    let mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>;
    let mut fn_cache: FunctionUnitCache;
    if !(Flags::getConfigBool(Flags::UNIT_CHECKING.clone())? || Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) {
        return Ok(flatModel);
    }
    match '__try0: {
        htCr2U1 = Unit::newCrefUnitTable(Util::nextPrime(((metamodelica::OrderedFloat((10) as f64) + metamodelica::OrderedFloat(1.4_f64) * metamodelica::OrderedFloat(((flatModel.variables.clone().len() as i32)) as f64)).0.floor() as i32)));
        htS2U = unwrap_break_err!(Unit::getKnownUnits(), '__try0);
        htU2S = unwrap_break_err!(Unit::getKnownUnitsInverse(), '__try0);
        fn_cache = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), fnptr!(stringEq, ArcStr, ArcStr), 1);
        for mut v in &*flatModel.variables.clone() {
            let mut v = v.clone();
            unwrap_break_err!(convertUnitStringToUnit(v.clone(), htCr2U1.clone(), htS2U.clone(), htU2S.clone()), '__try0);
        }
        htCr2U2 = UnorderedMap::copy(htCr2U1.clone());
        htCr2U2 = unwrap_break_err!(checkModelConsistency(flatModel.variables.clone(), flatModel.equations.clone(), flatModel.initialEquations.clone(), htCr2U2.clone(), htS2U.clone(), htU2S.clone(), fn_cache.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_UNIT.clone()), '__try0) {
            println!("{}", (unwrap_break_err!(UnorderedMap::toString(htCr2U2.clone(), ComponentRef::toString, Unit::unit2string, (literal!("\n")).clone(), (literal!(", ")).clone()), '__try0)).clone());
            println!("{}", (literal!("\n######## UnitCheck COMPLETED ########\n")).clone());
        }
        unwrap_break_err!(notification(htCr2U1.clone(), htCr2U2.clone(), htU2S.clone()), '__try0);
        flatModel = updateModel(flatModel.clone(), htCr2U2.clone(), htU2S.clone());
        Ok::<_, anyhow::Error>((flatModel.clone(), fn_cache.clone(), htCr2U1.clone(), htCr2U2.clone(), htS2U.clone(), htU2S.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            flatModel = __try0_o0;
            fn_cache = __try0_o1;
            htCr2U1 = __try0_o2;
            htCr2U2 = __try0_o3;
            htS2U = __try0_o4;
            htU2S = __try0_o5;
        }
        Err(_) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFUnitCheck.checkUnits")); __mm_s.push_str(&*literal!(": unit check module failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    execStat((literal!("NFUnitCheck.checkUnits")).clone())?;
    Ok(flatModel)
}

fn updateModel(mut flatModel: Arc<FlatModel::NFFlatModel>, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Arc<FlatModel::NFFlatModel> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    assign_field!(flatModel.variables = {
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = updateVariable(v.clone(), htCr2U.clone(), htU2S.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    flatModel
}

fn updateVariable(mut var: Arc<Variable::NFVariable>, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Arc<Variable::NFVariable> {
    let mut var: Arc<Variable::NFVariable> = var;
    let mut name: ArcStr = arcstr::literal!("");
    let mut unit_str: ArcStr = arcstr::literal!("");
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut unit_idx: i32 = 0;
    let mut unit: Unit::Unit;
    if Type::isReal(var.ty.clone()) {
        for mut attr in &*var.typeAttributes.clone() {
            let mut attr = attr.clone();
            (name, binding) = attr.clone();
            unit_idx = unit_idx.clone() + 1;
            if name.clone() == literal!("unit") {
                if Binding::isBound(binding.clone()) {
                    return var;
                } else {
                    assign_field!(var.typeAttributes = listDelete(var.typeAttributes.clone(), unit_idx.clone()).unwrap());
                    break;
                }
            }
        }
        if '__try0: {
            unit = UnorderedMap::getOrFail(var.name.clone(), htCr2U.clone());
            if Unit::isUnit(unit.clone()) {
                unit_str = (unwrap_break_err!(Unit::unitString(unit.clone(), htU2S.clone()), '__try0)).clone();
                binding = Binding::makeFlat(Arc::new(Expression::NFExpression::STRING { value: (unit_str.clone()).clone() }), Variability::CONSTANT.clone(), Binding::Source::GENERATED.clone());
                assign_field!(var.typeAttributes = cons((literal!("unit"), binding.clone()), var.typeAttributes.clone()));
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    var
}

fn notification(mut inHtCr2U1: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut inHtCr2U2: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut inHtU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut lt1: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Unit::Unit)>> = metamodelica::nil();
    lt1 = UnorderedMap::toList(inHtCr2U1.clone());
    r#str = (notification2(lt1.clone(), inHtCr2U2.clone(), inHtU2S.clone())?).clone();
    if Flags::isSet(Flags::DUMP_UNIT.clone())? && r#str.clone() != literal!("") {
        Error::addCompilerNotification((r#str.clone()).clone())?;
    }
    Ok(())
}

fn notification2(mut inLt1: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Unit::Unit)>>, mut inHtCr2U2: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut inHtU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    let mut cr1: Arc<ComponentRef::NFComponentRef> = Arc::new(crate::NFComponentRef::EMPTY);
    let mut factor: metamodelica::Real = metamodelica::OrderedFloat((0) as f64);
    let mut s: i32 = 0;
    let mut m: i32 = 0;
    let mut g: i32 = 0;
    let mut A: i32 = 0;
    let mut K: i32 = 0;
    let mut mol: i32 = 0;
    let mut cd: i32 = 0;
    outS = stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t1 in (inLt1.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(t1.clone()) {
        (cr1, Unit::Unit::MASTER { .. }) => {
            let mut b: bool = false;
            b = false;
            if '__try0: {
                let Unit::UNIT { factor: __pa1, mol: __pa2, cd: __pa3, m: __pa4, s: __pa5, A: __pa6, K: __pa7, g: __pa8 } = (UnorderedMap::getOrFail(ComponentRef::stripSubscripts(cr1.clone()), inHtCr2U2.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                s = __pa1.clone();
                m = __pa2.clone();
                g = __pa3.clone();
                A = __pa4.clone();
                K = __pa5.clone();
                mol = __pa6.clone();
                cd = __pa7.clone();
                factor = metamodelica::OrderedFloat((__pa8.clone()) as f64);
                b = true;
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
            b.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*ComponentRef::toString(cr1.clone())?); __mm_s.push_str(&*literal!("\" has the Unit \"")); __mm_s.push_str(&*Unit::unitString(Unit::Unit::UNIT { s: s.clone(), m: m.clone(), g: g.clone(), A: A.clone(), K: K.clone(), mol: mol.clone(), cd: cd.clone(), factor: factor.clone() }, inHtU2S.clone())?); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outS)
}

fn checkModelConsistency(mut variables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut initialEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut fnCache: FunctionUnitCache) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>> {
    let mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>> = htCr2U;
    let mut dump_eq_unit: bool = Flags::isSet(Flags::DUMP_EQ_UNIT_STRUCT.clone())?;
    for mut v in &*variables.clone() {
        let mut v = v.clone();
        foldBindingExp(v.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), dump_eq_unit.clone())?;
        for mut c in &*v.children.clone() {
            let mut c = c.clone();
            foldBindingExp(c.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), dump_eq_unit.clone())?;
        }
    }
    for mut eq in &*equations.clone() {
        let mut eq = eq.clone();
        foldEquation(eq.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), dump_eq_unit.clone())?;
    }
    for mut ieq in &*initialEquations.clone() {
        let mut ieq = ieq.clone();
        foldEquation(ieq.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), dump_eq_unit.clone())?;
    }
    Ok(htCr2U)
}

fn foldBindingExp(mut var: Arc<Variable::NFVariable>, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut fnCache: FunctionUnitCache, mut dumpEqInitStruct: bool) -> Result<()> {
    let mut binding_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eq: Arc<Equation::NFEquation>;
    if Type::isReal(var.ty.clone()) && Binding::isBound(var.binding.clone()) {
        binding_exp = Binding::getTypedExp(var.binding.clone())?;
        eq = Equation::makeEquality(Expression::fromCref(var.name.clone(), false)?, binding_exp.clone(), var.ty.clone(), ElementSource::createElementSource(var.info.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref.clone(), DAE::emptyCref.clone()))?, Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), Equation::ScalarizeMode::NO_PREFERENCE.clone());
        foldEquation(eq.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), dumpEqInitStruct.clone())?;
    }
    Ok(())
}

fn foldEquation(mut eq: Arc<Equation::NFEquation>, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut fnCache: FunctionUnitCache, mut dumpEqInitStruct: bool) -> Result<()> {
    let mut inconsistent_units: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
    inconsistent_units = foldEquation2(eq.clone(), dumpEqInitStruct.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
    for mut u in &*inconsistent_units.clone() {
        let mut u = u.clone();
        Errorfunction(u.clone(), eq.clone(), htU2S.clone())?;
    }
    Ok(())
}

fn foldEquation2(mut eq: Arc<Equation::NFEquation>, mut dumpEqInitStruct: bool, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut fnCache: FunctionUnitCache) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>>> {
    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
    inconsistentUnits = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { rhs: rhs @ Deref @ Expression::CALL { .. }, lhs: lhs @ Deref @ Expression::TUPLE { .. }, .. } if (!(Function::isBuiltin(Call::typedFunction(var_field!((**rhs).call, Expression::NFExpression::CALL).clone())?))) => {
            let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
            let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
            let mut fn_name: ArcStr = arcstr::literal!("");
            let mut out_vars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut out_units: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            fn_name = (AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(Call::functionName(var_field!((**rhs).call, Expression::NFExpression::CALL).clone())?), (literal!(".")).clone(), true, false)?).clone();
            (_, out_vars, _, out_units) = getCallUnits((fn_name.clone()).clone(), var_field!((**rhs).call, Expression::NFExpression::CALL).clone(), fnCache.clone())?;
            icu1 = foldCallArg1(var_field!((**lhs).elements, Expression::NFExpression::TUPLE).clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, out_units.clone(), out_vars.clone(), (fn_name.clone()).clone())?;
            (_, icu2) = insertUnitInEquation(rhs.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
            List::append_reverse(icu1.clone(), icu2.clone())
        },
        Deref @ Equation::EQUALITY { rhs: rhs @ Deref @ Expression::CALL { .. }, .. } if (!(Function::isBuiltin(Call::typedFunction(var_field!((**rhs).call, Expression::NFExpression::CALL).clone())?))) => {
            let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
            let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
            let mut fn_name: ArcStr = arcstr::literal!("");
            let mut formal_args: ArcStr = arcstr::literal!("");
            let mut formal_var: ArcStr = arcstr::literal!("");
            let mut out_vars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut out_units: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut unit1: Unit::Unit;
            let mut unit2: Unit::Unit;
            let mut b: bool = false;
            fn_name = (AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(Call::functionName(var_field!((**rhs).call, Expression::NFExpression::CALL).clone())?), (literal!(".")).clone(), true, false)?).clone();
            (_, out_vars, _, out_units) = getCallUnits((fn_name.clone()).clone(), var_field!((**rhs).call, Expression::NFExpression::CALL).clone(), fnCache.clone())?;
            (unit1, _) = insertUnitInEquation(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
            formal_args = (listHead(out_units.clone())?).clone();
            formal_var = (listHead(out_vars.clone())?).clone();
            unit2 = if (formal_args.clone() == literal!("NONE")) {Unit::Unit::MASTER { varList: metamodelica::nil() }} else {Unit::parseUnitString((formal_args.clone()).clone(), htS2U.clone(), Equation::info(eq.clone()))?};
            (b, _) = unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone());
            if b.clone() {
                icu1 = metamodelica::nil();
            } else {
                icu1 = list![list![(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), unit1.clone()), (makeNewCref((formal_var.clone()).clone(), (fn_name.clone()).clone()), unit2.clone())]];
            }
            (_, icu2) = insertUnitInEquation(rhs.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
            List::append_reverse(icu1.clone(), icu2.clone())
        },
        Deref @ Equation::EQUALITY { .. } => {
            let mut temp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            temp = Arc::new(Expression::NFExpression::BINARY { exp1: var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), operator: Operator::makeSub(Arc::new(crate::NFType::REAL)), exp2: var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone() });
            if dumpEqInitStruct.clone() {
                println!("{}", (Expression::toString(temp.clone())?).clone());
                println!("{}", (literal!("--------------------\n")).clone());
            }
            (_, inconsistentUnits) = insertUnitInEquation(temp.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
            inconsistentUnits.clone()
        },
        Deref @ Equation::WHEN { branches: Deref @ metamodelica::List::Cons { head: Deref @ Equation::Branch::BRANCH { body: eql, .. }, tail: _ }, .. } => {
            let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
            inconsistentUnits = metamodelica::nil();
            for mut e in &*eql.clone() {
                let mut e = e.clone();
                icu1 = foldEquation2(e.clone(), dumpEqInitStruct.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                inconsistentUnits = List::append_reverse(icu1.clone(), inconsistentUnits.clone());
            }
            inconsistentUnits.clone()
        },
        Deref @ Equation::NORETCALL { .. } => {
            (_, inconsistentUnits) = insertUnitInEquation(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
            inconsistentUnits.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(inconsistentUnits)
}

fn makeNewCref(mut paramName: ArcStr, mut fnName: ArcStr) -> Arc<Expression::NFExpression> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outExp = Arc::new(Expression::NFExpression::CREF { ty: Arc::new(crate::NFType::UNKNOWN), cref: ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: (paramName.clone()).clone() }), Arc::new(crate::NFType::UNKNOWN), metamodelica::nil(), ComponentRef::fromNode(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fnName.clone()); __mm_s.push_str(&*literal!("()")); ArcStr::from(__mm_s) }).clone() }), Arc::new(crate::NFType::UNKNOWN), metamodelica::nil(), ComponentRef::Origin::CREF.clone())) });
    outExp
}

fn insertUnitInEquation(mut eq: Arc<Expression::NFExpression>, mut unit: Unit::Unit, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut fnCache: FunctionUnitCache) -> Result<(Unit::Unit, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>>)> {
    use crate::NFOperator::Op;
    let mut unit: Unit::Unit = unit;
    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
    (unit, inconsistentUnits) = 'mc: {
        let __mc_input = eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa0.clone();
                    icu2 = __pa1.clone();
                    (unit1, icu1) = insertUnitInEquation(exp1.clone(), unit2.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    let (true, __pa2) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())) else { bail!("pattern mismatch") };
                    op_unit = __pa2.clone();
                    Ok((op_unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    (unit1, icu2) = insertUnitInEquation(exp1.clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    (unit2, icu1) = insertUnitInEquation(exp2.clone(), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    let (true, __pa0) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())) else { bail!("pattern mismatch") };
                    op_unit = __pa0.clone();
                    Ok((op_unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa0.clone();
                    icu2 = __pa1.clone();
                    (unit1, icu1) = insertUnitInEquation(exp1.clone(), unit2.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    let (false, _) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())) else { bail!("pattern mismatch") };
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, cons(list![(exp1.clone(), unit1.clone()), (exp2.clone(), unit2.clone())], List::append_reverse(icu1.clone(), icu2.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    (unit1, icu2) = insertUnitInEquation(exp1.clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    (unit2, icu1) = insertUnitInEquation(exp2.clone(), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    let (false, _) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())) else { bail!("pattern mismatch") };
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, cons(list![(exp1.clone(), unit1.clone()), (exp2.clone(), unit2.clone())], List::append_reverse(icu1.clone(), icu2.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa0.clone();
                    icu2 = __pa1.clone();
                    (unit1, icu1) = insertUnitInEquation(exp1.clone(), unit2.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    let (true, __pa2) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())) else { bail!("pattern mismatch") };
                    op_unit = __pa2.clone();
                    Ok((op_unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    (unit1, icu2) = insertUnitInEquation(exp1.clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    (unit2, icu1) = insertUnitInEquation(exp2.clone(), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    let (true, __pa0) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())) else { bail!("pattern mismatch") };
                    op_unit = __pa0.clone();
                    Ok((op_unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa0.clone();
                    icu2 = __pa1.clone();
                    (unit1, icu1) = insertUnitInEquation(exp1.clone(), unit2.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    let (false, _) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())) else { bail!("pattern mismatch") };
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, cons(list![(exp1.clone(), unit1.clone()), (exp2.clone(), unit2.clone())], List::append_reverse(icu1.clone(), icu2.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    (unit1, icu2) = insertUnitInEquation(exp1.clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    (unit2, icu1) = insertUnitInEquation(exp2.clone(), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    let (false, _) = (unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone())) else { bail!("pattern mismatch") };
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, cons(list![(exp1.clone(), unit1.clone()), (exp2.clone(), unit2.clone())], List::append_reverse(icu1.clone(), icu2.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit1 = __pa0.clone();
                    icu1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa2 @ Unit::Unit::UNIT { .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa2.clone();
                    icu2 = __pa3.clone();
                    op_unit = Unit::unitMul(unit1.clone(), unit2.clone())?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((op_unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    if !((Unit::isMaster(unit.clone()))) { bail!("guard") }
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { .. }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::UNIT { .. }, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu2 = __pa1.clone();
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    if !((Unit::isUnit(unit.clone()))) { bail!("guard") }
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vars = __pa0.clone();
                    icu1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa2 @ Unit::Unit::UNIT { .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa2.clone();
                    icu2 = __pa3.clone();
                    op_unit = Unit::unitDiv(unit.clone(), unit2.clone())?;
                    List::map2_0(vars.clone(), Arc::new(updateHtCr2U), op_unit.clone(), htCr2U.clone());
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    if !((Unit::isMaster(unit.clone()))) { bail!("guard") }
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::UNIT { .. }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { .. }, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu2 = __pa1.clone();
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    if !((Unit::isUnit(unit.clone()))) { bail!("guard") }
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa0.clone();
                    icu1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa2 }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vars = __pa2.clone();
                    icu2 = __pa3.clone();
                    op_unit = Unit::unitDiv(unit.clone(), unit2.clone())?;
                    List::map2_0(vars.clone(), Arc::new(updateHtCr2U), op_unit.clone(), htCr2U.clone());
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2 } => {
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { .. }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { .. }, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu2 = __pa1.clone();
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit1 = __pa0.clone();
                    icu1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa2 @ Unit::Unit::UNIT { .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa2.clone();
                    icu2 = __pa3.clone();
                    op_unit = Unit::unitDiv(unit1.clone(), unit2.clone())?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((op_unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    if !((Unit::isMaster(unit.clone()))) { bail!("guard") }
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = inconsistentUnits.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { .. }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::UNIT { .. }, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu2 = __pa1.clone();
                    inconsistentUnits = List::append_reverse(icu1.clone(), icu2.clone());
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    if !((Unit::isUnit(unit.clone()))) { bail!("guard") }
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vars = __pa0.clone();
                    icu1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa2 @ Unit::Unit::UNIT { .. }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa2.clone();
                    icu2 = __pa3.clone();
                    op_unit = Unit::unitMul(unit.clone(), unit2.clone())?;
                    List::map2_0(vars.clone(), Arc::new(updateHtCr2U), op_unit.clone(), htCr2U.clone());
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    if !((Unit::isMaster(unit.clone()))) { bail!("guard") }
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::UNIT { .. }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { .. }, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu2 = __pa1.clone();
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    if !((Unit::isUnit(unit.clone()))) { bail!("guard") }
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit2 = __pa0.clone();
                    icu1 = __pa1.clone();
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa2 }, __pa3) => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vars = __pa2.clone();
                    icu2 = __pa3.clone();
                    op_unit = Unit::unitDiv(unit2.clone(), unit.clone())?;
                    List::map2_0(vars.clone(), Arc::new(updateHtCr2U), op_unit.clone(), htCr2U.clone());
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::DIV, .. }, exp2 } => {
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { .. }, __pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(insertUnitInEquation(exp2.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { .. }, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    icu2 = __pa1.clone();
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, List::append_reverse(icu1.clone(), icu2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::POW, .. }, exp2: exp2 @ Deref @ Expression::REAL { .. } } => {
                    let mut unit1: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut i: i32 = 0;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (__pa0 @ Unit::Unit::UNIT { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    unit1 = __pa0.clone();
                    icu1 = __pa1.clone();
                    i = ((exp2.value.clone()).0 as i32);
                    let true = (realEq(exp2.value.clone(), metamodelica::OrderedFloat((i.clone()) as f64))) else { bail!("pattern mismatch") };
                    op_unit = Unit::unitPow(unit1.clone(), i.clone())?;
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((op_unit.clone(), icu1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::POW, .. }, exp2: exp2 @ Deref @ Expression::REAL { .. } } => {
                    if !((Unit::isUnit(unit.clone()))) { bail!("guard") }
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?) {
                        (Unit::Unit::MASTER { varList: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    vars = __pa0.clone();
                    icu1 = __pa1.clone();
                    op_unit = Unit::unitRoot(unit.clone(), exp2.value.clone())?;
                    List::map2_0(vars.clone(), Arc::new(updateHtCr2U), op_unit.clone(), htCr2U.clone());
                    insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    Ok((unit.clone(), icu1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BINARY { exp1, operator: Deref @ Operator::OPERATOR { op: Operator::Op::POW, .. }, exp2: Deref @ Expression::REAL { .. } } => {
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    (_, icu1) = insertUnitInEquation(exp1.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, icu1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::CALL { .. } => {
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    (op_unit, icu1) = insertUnitInEquationCall(var_field!((*eq).call, Expression::NFExpression::CALL).clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    Ok((op_unit.clone(), icu1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::IF { .. } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = inconsistentUnits.clone();
                    (unit1, icu1) = insertUnitInEquation(var_field!((*eq).trueBranch, Expression::NFExpression::IF).clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    (unit2, icu2) = insertUnitInEquation(var_field!((*eq).falseBranch, Expression::NFExpression::IF).clone(), unit1.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    (b, op_unit) = unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone());
                    inconsistentUnits = List::append_reverse(icu1.clone(), icu2.clone());
                    if !(b.clone()) {
                        inconsistentUnits = cons(list![(var_field!((*eq).trueBranch, Expression::NFExpression::IF).clone(), unit1.clone()), (var_field!((*eq).falseBranch, Expression::NFExpression::IF).clone(), unit2.clone())], inconsistentUnits.clone());
                        op_unit = Unit::Unit::MASTER { varList: metamodelica::nil() };
                    }
                    Ok((op_unit.clone(), inconsistentUnits.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::RELATION { .. } => {
                    let mut unit1: Unit::Unit;
                    let mut unit2: Unit::Unit;
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut icu2: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = inconsistentUnits.clone();
                    (unit1, icu1) = insertUnitInEquation(var_field!((*eq).exp1, Expression::NFExpression::RELATION).clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    (unit2, icu2) = insertUnitInEquation(var_field!((*eq).exp2, Expression::NFExpression::RELATION).clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    (b, op_unit) = unitTypesEqual(unit1.clone(), unit2.clone(), htCr2U.clone());
                    inconsistentUnits = List::append_reverse(icu1.clone(), icu2.clone());
                    if !(b.clone()) {
                        inconsistentUnits = cons(list![(var_field!((*eq).exp1, Expression::NFExpression::RELATION).clone(), unit1.clone()), (var_field!((*eq).exp2, Expression::NFExpression::RELATION).clone(), unit2.clone())], inconsistentUnits.clone());
                        op_unit = Unit::Unit::MASTER { varList: metamodelica::nil() };
                    }
                    Ok((op_unit.clone(), inconsistentUnits.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::UNARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::UMINUS, .. }, .. } => {
                    let mut op_unit: Unit::Unit;
                    let mut icu1: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
                    (op_unit, icu1) = insertUnitInEquation(var_field!((*eq).exp, Expression::NFExpression::UNARY).clone(), unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    Ok((op_unit.clone(), icu1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::CREF { .. } => {
                    if !((ComponentRef::isTime(var_field!((*eq).cref, Expression::NFExpression::CREF).clone()))) { bail!("guard") }
                    let mut op_unit: Unit::Unit;
                    op_unit = Unit::SECOND().clone();
                    addUnit2HtS2U((literal!("time")).clone(), op_unit.clone(), htS2U.clone())?;
                    addUnit2HtU2S((literal!("time")).clone(), op_unit.clone(), htU2S.clone())?;
                    Ok((op_unit.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::CREF { ty: Deref @ Type::REAL, .. } => {
                    Ok((UnorderedMap::getOrFail(ComponentRef::stripSubscripts(var_field!((*eq).cref, Expression::NFExpression::CREF).clone()), htCr2U.clone()), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((unit, inconsistentUnits))
}

fn insertUnitInEquationCall(mut call: Arc<Call::NFCall>, mut unit: Unit::Unit, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut fnCache: FunctionUnitCache) -> Result<(Unit::Unit, Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>>)> {
    let mut unit: Unit::Unit = unit;
    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
    let mut fn_path: Arc<Absyn::Path>;
    let mut fn_name: ArcStr = arcstr::literal!("");
    let mut call_args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut op_unit: Unit::Unit;
    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut var_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut unit_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    fn_path = Call::functionName(call.clone())?;
    call_args = Call::arguments(call.clone())?;
    (unit, inconsistentUnits) = 'mc: {
        let __mc_input = fn_path.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: Deref @ "pre" } => {
                    let mut op_unit: Unit::Unit;
                    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = inconsistentUnits.clone();
                    (op_unit, inconsistentUnits) = insertUnitInEquation(listHead(call_args.clone())?, unit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, inconsistentUnits.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: Deref @ "der" } => {
                    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = vars.clone();
                    let mut op_unit: Unit::Unit;
                    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = inconsistentUnits.clone();
                    (op_unit, inconsistentUnits) = insertUnitInEquation(listHead(call_args.clone())?, Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    if Unit::isUnit(op_unit.clone()) {
                        op_unit = Unit::unitDiv(op_unit.clone(), Unit::SECOND().clone())?;
                        insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    } else if Unit::isUnit(unit.clone()) {
                        let Unit::MASTER { varList: __pa0 } = (op_unit.clone()) else { bail!("pattern mismatch") };
                        vars = __pa0.clone();
                        op_unit = Unit::unitMul(unit.clone(), Unit::SECOND().clone())?;
                        List::map2_0(vars.clone(), Arc::new(updateHtCr2U), op_unit.clone(), htCr2U.clone());
                        insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    } else {
                        op_unit = Unit::Unit::MASTER { varList: metamodelica::nil() };
                    }
                    Ok((op_unit.clone(), inconsistentUnits.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" } => {
                    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = inconsistentUnits.clone();
                    let mut vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = vars.clone();
                    let mut op_unit: Unit::Unit;
                    (op_unit, inconsistentUnits) = insertUnitInEquation(listHead(call_args.clone())?, Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    if Unit::isUnit(op_unit.clone()) {
                        op_unit = Unit::unitRoot(op_unit.clone(), metamodelica::OrderedFloat(2.0_f64))?;
                        insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                    } else if Unit::isUnit(unit.clone()) {
                        let Unit::MASTER { varList: __pa0 } = (op_unit.clone()) else { bail!("pattern mismatch") };
                        vars = __pa0.clone();
                        op_unit = Unit::unitPow(unit.clone(), 2)?;
                        List::map2_0(vars.clone(), Arc::new(updateHtCr2U), op_unit.clone(), htCr2U.clone());
                        insertUnitString(op_unit.clone(), htS2U.clone(), htU2S.clone())?;
                        op_unit = unit.clone();
                    } else {
                        op_unit = Unit::Unit::MASTER { varList: metamodelica::nil() };
                    }
                    Ok((op_unit.clone(), inconsistentUnits.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { .. } => {
                    if !((Function::isBuiltin(Call::typedFunction(call.clone())?))) { bail!("guard") }
                    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = inconsistentUnits.clone();
                    inconsistentUnits = foldCallArg(call_args.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, inconsistentUnits.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut unit_names: Arc<metamodelica::List<ArcStr>> = unit_names.clone();
                    let mut var_names: Arc<metamodelica::List<ArcStr>> = var_names.clone();
                    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = inconsistentUnits.clone();
                    let mut fn_name: ArcStr = fn_name.clone();
                    fn_name = (AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(fn_path.clone()), (literal!(".")).clone(), true, false)?).clone();
                    (var_names, _, unit_names, _) = getCallUnits((fn_name.clone()).clone(), call.clone(), fnCache.clone())?;
                    inconsistentUnits = foldCallArg1(call_args.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone(), unit.clone(), unit_names.clone(), var_names.clone(), (fn_name.clone()).clone())?;
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, inconsistentUnits.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((Unit::Unit::MASTER { varList: metamodelica::nil() }, metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((unit, inconsistentUnits))
}

fn insertUnitString(mut unit: Unit::Unit, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Result<()> {
    let mut unit_str: ArcStr = arcstr::literal!("");
    unit_str = (Unit::unitString(unit.clone(), htU2S.clone())?).clone();
    addUnit2HtS2U((unit_str.clone()).clone(), unit.clone(), htS2U.clone())?;
    addUnit2HtU2S((unit_str.clone()).clone(), unit.clone(), htU2S.clone())?;
    Ok(())
}

fn getCallUnits(mut fnName: ArcStr, mut call: Arc<Call::NFCall>, mut fnCache: FunctionUnitCache) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut inputVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outputVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut inputUnits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outputUnits: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut opt_args: Option<Functionargs> = None;
    let mut args: Functionargs;
    opt_args = UnorderedMap::get((fnName.clone()).clone(), fnCache.clone());
    if isSome(opt_args.clone()) {
        let Some(__pa0) = (opt_args.clone()) else { bail!("pattern mismatch") };
        args = __pa0.clone();
    } else {
        args = parseFunctionUnits((fnName.clone()).clone(), Call::typedFunction(call.clone())?)?;
        UnorderedMap::addUnique((fnName.clone()).clone(), args.clone(), fnCache.clone())?;
    }
    let Functionargs { name: _, invars: __pa1, outvars: __pa2, inunits: __pa3, outunits: __pa4 } = (args.clone()) else { bail!("pattern mismatch") };
    inputVars = __pa1.clone();
    outputVars = __pa2.clone();
    inputUnits = __pa3.clone();
    outputUnits = __pa4.clone();
    Ok((inputVars, outputVars, inputUnits, outputUnits))
}

fn parseFunctionUnits(mut funcName: ArcStr, mut func: Arc<Function::Function>) -> Result<Functionargs> {
    let mut outArgs: Functionargs;
    let mut fn_name: ArcStr = arcstr::literal!("");
    let mut in_units: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut out_units: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut in_args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut out_args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    in_units = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (func.inputs.clone()).into_iter().cloned() {
            let __x = Component::getUnitAttribute(InstNode::component(p.clone())?, (literal!("NONE")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    out_units = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (func.outputs.clone()).into_iter().cloned() {
            let __x = Component::getUnitAttribute(InstNode::component(p.clone())?, (literal!("NONE")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    in_args = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (func.inputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(p.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    out_args = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (func.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(p.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outArgs = Functionargs { name: (funcName.clone()).clone(), invars: in_args.clone(), outvars: out_args.clone(), inunits: in_units.clone(), outunits: out_units.clone() };
    Ok(outArgs)
}

fn unitTypesEqual(mut unit1: Unit::Unit, mut unit2: Unit::Unit, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>) -> (bool, Unit::Unit) {
    let mut isEqual: bool = false;
    let mut outUnit: Unit::Unit;
    (isEqual, outUnit) = (match (unit1.clone(), unit2.clone()) {
        (Unit::Unit::UNIT { .. }, Unit::Unit::UNIT { .. }) => {
            (Unit::isEqual(unit1.clone(), unit2.clone()), unit1.clone())
        },
        (Unit::Unit::UNIT { .. }, Unit::Unit::MASTER { varList: ref vars2 }) => {
            List::map2_0(vars2.clone(), Arc::new(updateHtCr2U), unit1.clone(), htCr2U.clone());
            (true, unit1.clone())
        },
        (Unit::Unit::MASTER { varList: ref vars1 }, Unit::Unit::UNIT { .. }) => {
            List::map2_0(vars1.clone(), Arc::new(updateHtCr2U), unit2.clone(), htCr2U.clone());
            (true, unit2.clone())
        },
        (Unit::Unit::MASTER { varList: ref vars1 }, Unit::Unit::MASTER { varList: ref vars2 }) => {
            (true, Unit::Unit::MASTER { varList: List::append_reverse(vars1.clone(), vars2.clone()) })
        },
        (Unit::Unit::UNKNOWN { unit: mut s1 }, Unit::Unit::UNKNOWN { unit: mut s2 }) => {
            (s1.clone() == s2.clone(), unit1.clone())
        },
        (Unit::Unit::UNKNOWN { .. }, _) => {
            (true, unit1.clone())
        },
        (_, Unit::Unit::UNKNOWN { .. }) => {
            (true, unit2.clone())
        },
        _ => {
            (false, unit1.clone())
        },
    });
    (isEqual, outUnit)
}

fn updateHtCr2U(mut cref: Arc<ComponentRef::NFComponentRef>, mut unit: Unit::Unit, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>) -> Result<()> {
    UnorderedMap::tryAdd(Unit::UPDATECREF().clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone())?;
    UnorderedMap::add(cref.clone(), unit.clone(), htCr2U.clone())?;
    Ok(())
}

fn Errorfunction(mut inexpList: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>, mut inEq: Arc<Equation::NFEquation>, mut inHtU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inexpList.clone(), inEq.clone(), inHtU2S.clone())) {
        (expList, _, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut info: SourceInfo;
            info = Equation::info(inEq.clone());
            s = (Equation::toString(inEq.clone(), (literal!("")).clone())?).clone();
            s1 = (Errorfunction2(expList.clone(), inHtU2S.clone())?).clone();
            s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following equation is INCONSISTENT due to specified unit information: ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::COMPILER_WARNING.clone(), list![(s2.clone()).clone()], info.clone())?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The units of following sub-expressions need to be equal:\n")); __mm_s.push_str(&*s1.clone()); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn Errorfunction2(mut inexpList: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>, mut inHtU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Result<ArcStr> {
    let mut outS: ArcStr = arcstr::literal!("");
    outS = ((::match_deref::match_deref! { match &((inexpList.clone(), inHtU2S.clone())) {
        (Deref @ metamodelica::List::Cons { head: (exp, ut), tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            s = (Expression::toString(exp.clone())?).clone();
            s1 = (Unit::unitString(ut.clone(), inHtU2S.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- sub-expression \"")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\" has unit \"")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (exp, ut), tail: expList }, _) => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            s = (Expression::toString(exp.clone())?).clone();
            s1 = (Unit::unitString(ut.clone(), inHtU2S.clone())?).clone();
            s2 = (Errorfunction2(expList.clone(), inHtU2S.clone())?).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- sub-expression \"")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\" has unit \"")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\"\n")); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outS)
}

fn foldCallArg(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut fnCache: FunctionUnitCache) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>>> {
    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
    let mut icu: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
    for mut exp in &*args.clone() {
        let mut exp = exp.clone();
        (_, icu) = insertUnitInEquation(exp.clone(), Unit::Unit::MASTER { varList: metamodelica::nil() }, htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
        inconsistentUnits = List::append_reverse(icu.clone(), inconsistentUnits.clone());
    }
    inconsistentUnits = inconsistentUnits.clone().reverse();
    Ok(inconsistentUnits)
}

fn foldCallArg1(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut fnCache: FunctionUnitCache, mut inUnit: Unit::Unit, mut units: Arc<metamodelica::List<ArcStr>>, mut vars: Arc<metamodelica::List<ArcStr>>, mut fnName: ArcStr) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>>> {
    let mut inconsistentUnits: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
    let mut unit: ArcStr = arcstr::literal!("");
    let mut var: ArcStr = arcstr::literal!("");
    let mut rest_units: Arc<metamodelica::List<ArcStr>> = units.clone();
    let mut rest_vars: Arc<metamodelica::List<ArcStr>> = vars.clone();
    let mut op_unit: Unit::Unit;
    let mut op_unit2: Unit::Unit;
    let mut icu: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<Expression::NFExpression>, Unit::Unit)>>>> = metamodelica::nil();
    let mut temp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut b: bool = false;
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_vars.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        var = __pa0.clone();
        rest_vars = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_units.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        unit = __pa2.clone();
        rest_units = __pa3.clone();
        (op_unit, icu) = insertUnitInEquation(arg.clone(), inUnit.clone(), htCr2U.clone(), htS2U.clone(), htU2S.clone(), fnCache.clone())?;
        if unit.clone() == literal!("NONE") {
            op_unit2 = Unit::Unit::MASTER { varList: metamodelica::nil() };
        } else {
            op_unit2 = Unit::parseUnitString((unit.clone()).clone(), htS2U.clone(), Absyn::dummyInfo.clone())?;
        }
        (b, op_unit) = unitTypesEqual(op_unit.clone(), op_unit2.clone(), htCr2U.clone());
        if b.clone() {
            icu = metamodelica::nil();
        } else {
            temp = makeNewCref((var.clone()).clone(), (fnName.clone()).clone());
            icu = list![list![(arg.clone(), op_unit.clone()), (temp.clone(), op_unit2.clone())]];
        }
        inconsistentUnits = List::append_reverse(icu.clone(), inconsistentUnits.clone());
    }
    Ok(inconsistentUnits)
}

fn addUnit2HtS2U(mut name: ArcStr, mut unit: Unit::Unit, mut inHtS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>) -> Result<()> {
    UnorderedMap::add((name.clone()).clone(), unit.clone(), inHtS2U.clone())?;
    Ok(())
}

fn addUnit2HtU2S(mut name: ArcStr, mut unit: Unit::Unit, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Result<()> {
    UnorderedMap::tryAdd(unit.clone(), (name.clone()).clone(), htU2S.clone())?;
    Ok(())
}

fn convertUnitStringToUnit(mut var: Arc<Variable::NFVariable>, mut htCr2U: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Unit::Unit>>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>) -> Result<()> {
    let mut unit_binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut unit_exp: Option<Arc<Expression::NFExpression>> = None;
    let mut unit_string: ArcStr = arcstr::literal!("");
    let mut unit: Unit::Unit;
    unit_binding = Variable::lookupTypeAttribute((literal!("unit")).clone(), var.clone());
    unit_exp = Binding::typedExp(unit_binding.clone());
    unit_string = (if (isSome(unit_exp.clone())) {getUnitStringFromExp(Util::getOption(unit_exp.clone())?)?} else {literal!("")}).clone();
    if stringEmpty((unit_string.clone()).clone()) {
        UnorderedMap::add(var.name.clone(), Unit::Unit::MASTER { varList: list![var.name.clone()] }, htCr2U.clone())?;
        addUnit2HtS2U((literal!("-")).clone(), Unit::Unit::MASTER { varList: list![var.name.clone()] }, htS2U.clone())?;
        addUnit2HtU2S((literal!("-")).clone(), Unit::Unit::MASTER { varList: list![var.name.clone()] }, htU2S.clone())?;
    } else {
        unit = parse((unit_string.clone()).clone(), var.name.clone(), htS2U.clone(), htU2S.clone(), var.info.clone())?;
        UnorderedMap::add(var.name.clone(), unit.clone(), htCr2U.clone())?;
    }
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getUnitStringFromExp(mut unitExp: Arc<Expression::NFExpression>) -> Result<ArcStr> {
    let mut unitString: ArcStr = arcstr::literal!("");
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    unitString = ((::match_deref::match_deref! { match &(unitExp.clone()) {
        Deref @ Expression::STRING { .. } => var_field!((*unitExp).value, Expression::NFExpression::STRING).clone(),
        Deref @ Expression::ARRAY { literal: true, .. } if (Expression::isLiteral(unitExp.clone()) && !(Type::isEmptyArray(Expression::typeOf(unitExp.clone())))) => getUnitStringFromExp(Expression::arrayFirstScalar(unitExp.clone())?)?,
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: exp, tail: _ }, .. } } if (Call::isNamed(var_field!((*unitExp).call, Expression::NFExpression::CALL).clone(), (literal!("fill")).clone())?) => getUnitStringFromExp(exp.clone())?,
        _ if (!(Expression::isLiteral(unitExp.clone()))) => {
            exp = Ceval::tryEvalExp(unitExp.clone(), Ceval::noTarget().clone());
            if (Expression::isLiteral(exp.clone())) {getUnitStringFromExp(exp.clone())?} else {literal!("")}
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(unitString)
}

fn parse(mut unitString: ArcStr, mut cref: Arc<ComponentRef::NFComponentRef>, mut htS2U: Arc<UnorderedMap::UnorderedMap<ArcStr, Unit::Unit>>, mut htU2S: Arc<UnorderedMap::UnorderedMap<Unit::Unit, ArcStr>>, mut info: SourceInfo) -> Result<Unit::Unit> {
    let mut unit: Unit::Unit;
    if stringEmpty((unitString.clone()).clone()) {
        unit = Unit::Unit::MASTER { varList: list![cref.clone()] };
        return Ok(unit);
    }
    match '__try0: {
        unit = UnorderedMap::getOrFail((unitString.clone()).clone(), htS2U.clone());
        Ok::<_, anyhow::Error>((unit.clone(),))
    } {
        Ok((__try0_o0,)) => {
            unit = __try0_o0;
        }
        Err(_) => {
            match '__try1: {
                unit = unwrap_break_err!(Unit::parseUnitString((unitString.clone()).clone(), htS2U.clone(), info.clone()), '__try1);
                Ok::<_, anyhow::Error>((unit.clone(),))
            } {
                Ok((__try1_o0,)) => {
                    unit = __try1_o0;
                }
                Err(_) => {
                    unit = Unit::Unit::UNKNOWN { unit: (unitString.clone()).clone() };
                }
            }
            addUnit2HtS2U((unitString.clone()).clone(), unit.clone(), htS2U.clone())?;
            addUnit2HtU2S((unitString.clone()).clone(), unit.clone(), htU2S.clone())?;
        }
    }
    Ok(unit)
}

