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

use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::Frame;
use crate::NBEquation::FrameLocation;
use crate::NBMatching as Matching;
use crate::NBVariable as BVariable;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::MMath;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// backend imports
// Util imports
// old imports
pub mod Rational {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Rational {
        pub n: i32,
        pub d: i32,
    }

    pub type RATIONAL = Rational;

    pub fn toString(mut r: Arc<Rational>) -> ArcStr {
        let mut r#str: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(r.n.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(r.d.clone())); ArcStr::from(__mm_s) };
        r#str
    }

    pub fn normalize(mut r: Arc<Rational>) -> Arc<Rational> {
        let mut r: Arc<Rational> = r;
        if r.n.clone() == 0 {
            assign_field!(r.d = 1);
        }
        r
    }

    pub fn add(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> Arc<Rational> {
        let mut r: Arc<Rational> = finalize(r1.n.clone() * r2.d.clone() + r2.n.clone() * r1.d.clone(), r1.d.clone() * r2.d.clone());
        r
    }

    pub fn multiply(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> Arc<Rational> {
        let mut r: Arc<Rational> = finalize(r1.n.clone() * r2.n.clone(), r1.d.clone() * r2.d.clone());
        r
    }

    pub fn isEqual(mut r1: Arc<Rational>, mut r2: Arc<Rational>) -> bool {
        let mut b: bool = r1.n.clone() == r2.n.clone() && r1.d.clone() == r2.d.clone();
        b
    }

    pub fn convert(mut r: Arc<Rational>) -> MMath::Rational {
        let mut oldR: MMath::Rational = MMath::Rational { nom: r.n.clone(), denom: r.d.clone() };
        oldR
    }

    fn finalize(mut i1: i32, mut i2: i32) -> Arc<Rational> {
        let mut r: Arc<Rational>;
        let mut d: i32 = intGcd(i1.clone(), i2.clone());
        r = normalize(Arc::new(Rational { n: intDiv(i1.clone(), d.clone()), d: intDiv(i2.clone(), d.clone()) }));
        r
    }

    #[tailcall::tailcall]
    fn intGcd(mut i1: i32, mut i2: i32) -> i32 {
        if (i2.clone() == 0) {i1.clone()} else {tailcall::call!{ intGcd(i2.clone(), intMod(i1.clone(), i2.clone())) }}
    }

}

pub fn findTrueIndices(mut arr: metamodelica::Array<bool>) -> Arc<metamodelica::List<i32>> {
    let mut indices: Arc<metamodelica::List<i32>> = {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in ((1..=(arr.clone().borrow().len() as i32)).rev()).into_iter() {
            if !(arr.borrow()[(i.clone()-1) as usize].clone()) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    indices
}

pub fn countElem(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> i32 {
    let mut count: i32 = {
        let mut __acc: i32 = 0;
        for mut lst in (m.clone()).borrow().iter() {
            let __x = (lst.clone().len() as i32);
            __acc += __x;
        }
        __acc
    };
    count
}

pub fn indexTplGt<T: Clone + 'static>(mut tpl1: (i32, T), mut tpl2: (i32, T)) -> bool {
    let mut gt: bool = false;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    (i1, _) = tpl1.clone();
    (i2, _) = tpl2.clone();
    gt = i1.clone() > i2.clone();
    gt
}

pub fn noNameHashEq(mut eq: Arc<Equation::Equation>, mut r#mod: i32) -> Result<i32> {
    let mut hash: i32 = 0;
    hash = noNameHashExp(BEquation::Equation::getResidualExp(eq.clone(), true)?, r#mod.clone())?;
    Ok(hash)
}

pub fn noNameHashExp(mut exp: Arc<Expression::NFExpression>, mut r#mod: i32) -> Result<i32> {
    let mut hash: i32 = 0;
    hash = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            var_field!((*exp).value, Expression::NFExpression::INTEGER).clone()
        },
        Deref @ Expression::REAL { .. } => {
            ((var_field!((*exp).value, Expression::NFExpression::REAL).clone()).0 as i32)
        },
        Deref @ Expression::STRING { .. } => {
            stringHashDjb2Mod((var_field!((*exp).value, Expression::NFExpression::STRING).clone()).clone(), r#mod.clone())
        },
        Deref @ Expression::BOOLEAN { .. } => {
            Util::boolInt(var_field!((*exp).value, Expression::NFExpression::BOOLEAN).clone())
        },
        Deref @ Expression::ENUM_LITERAL { .. } => {
            var_field!((*exp).index, Expression::NFExpression::ENUM_LITERAL).clone()
        },
        Deref @ Expression::CLKCONST { .. } => {
            0
        },
        Deref @ Expression::CREF { .. } => {
            let mut var: Arc<Variable::NFVariable> = Arc::new(<Variable::NFVariable as ::std::default::Default>::default());
            var = BVariable::getVar(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?;
            stringHashDjb2Mod((BackendInfo::toString(var.backendinfo.clone())?).clone(), r#mod.clone())
        },
        Deref @ Expression::TYPENAME { .. } => {
            1
        },
        Deref @ Expression::ARRAY { .. } => {
            let __range0 = var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut elem in __range0 {
                hash = hash.clone() + noNameHashExp(elem.clone(), r#mod.clone())?;
            }
            hash = hash.clone() + Util::boolInt(var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone());
            hash.clone()
        },
        Deref @ Expression::MATRIX { .. } => {
            for mut lst in &*var_field!((*exp).elements, Expression::NFExpression::MATRIX).clone() {
                let mut lst = lst.clone();
                for mut elem in &*lst.clone() {
                    let mut elem = elem.clone();
                    hash = hash.clone() + noNameHashExp(elem.clone(), r#mod.clone())?;
                }
            }
            hash.clone()
        },
        Deref @ Expression::RANGE { .. } => {
            if isSome(var_field!((*exp).step, Expression::NFExpression::RANGE).clone()) {
                hash = noNameHashExp(Util::getOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone())?, r#mod.clone())?;
            }
            hash.clone() + noNameHashExp(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), r#mod.clone())? + noNameHashExp(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), r#mod.clone())?
        },
        Deref @ Expression::TUPLE { .. } => {
            for mut elem in &*var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone() {
                let mut elem = elem.clone();
                hash = hash.clone() + noNameHashExp(elem.clone(), r#mod.clone())?;
            }
            hash.clone()
        },
        Deref @ Expression::RECORD { .. } => {
            for mut elem in &*var_field!((*exp).elements, Expression::NFExpression::RECORD).clone() {
                let mut elem = elem.clone();
                hash = hash.clone() + noNameHashExp(elem.clone(), r#mod.clone())?;
            }
            hash.clone()
        },
        Deref @ Expression::CALL { .. } => {
            2
        },
        Deref @ Expression::SIZE { .. } => {
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                hash = noNameHashExp(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?, r#mod.clone())?;
            }
            hash.clone() + noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone(), r#mod.clone())?
        },
        Deref @ Expression::END => {
            stringHashDjb2Mod((literal!("end")).clone(), r#mod.clone())
        },
        Deref @ Expression::BINARY { .. } => {
            let mut hash1: i32 = 0;
            let mut hash2: i32 = 0;
            hash1 = noNameHashExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), r#mod.clone())?;
            hash2 = noNameHashExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), r#mod.clone())?;
            hash = (match Operator::classify(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone())? {
        (Operator::MathClassification::ADDITION, _) => hash1.clone() + hash2.clone(),
        (Operator::MathClassification::SUBTRACTION, _) => hash1.clone() - hash2.clone(),
        (Operator::MathClassification::MULTIPLICATION, _) => hash1.clone() * hash2.clone(),
        (Operator::MathClassification::DIVISION, _) => ((metamodelica::OrderedFloat((hash1.clone() / hash2.clone()) as f64)).0 as i32),
        (Operator::MathClassification::POWER, _) => (((metamodelica::OrderedFloat((hash1.clone()) as f64)).powf(metamodelica::OrderedFloat((hash2.clone()) as f64))).0 as i32),
        (Operator::MathClassification::LOGICAL, _) => -(hash1.clone() + hash2.clone()),
        (Operator::MathClassification::RELATION, _) => hash2.clone() - hash1.clone(),
        _ => hash2.clone() - hash1.clone(),
    });
            hash.clone()
        },
        Deref @ Expression::UNARY { .. } => {
            -(noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), r#mod.clone())?)
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut hash1: i32 = 0;
            let mut hash2: i32 = 0;
            hash1 = noNameHashExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), r#mod.clone())?;
            hash2 = noNameHashExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), r#mod.clone())?;
            hash = (match var_field!((*exp).operator, Expression::NFExpression::LBINARY).op.clone() {
        Operator::Op::AND => hash1.clone() + hash2.clone(),
        Operator::Op::OR => hash1.clone() - hash2.clone(),
        _ => hash2.clone() - hash1.clone(),
    });
            hash.clone()
        },
        Deref @ Expression::LUNARY { .. } => {
            -(noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), r#mod.clone())?)
        },
        Deref @ Expression::RELATION { .. } => {
            let mut hash1: i32 = 0;
            let mut hash2: i32 = 0;
            hash1 = noNameHashExp(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), r#mod.clone())?;
            hash2 = noNameHashExp(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), r#mod.clone())?;
            hash = (match var_field!((*exp).operator, Expression::NFExpression::RELATION).op.clone() {
        Operator::Op::LESS => hash1.clone() + hash2.clone(),
        Operator::Op::LESSEQ => -(hash1.clone() + hash2.clone()),
        Operator::Op::GREATER => hash1.clone() - hash2.clone(),
        Operator::Op::GREATEREQ => hash2.clone() - hash1.clone(),
        Operator::Op::EQUAL => hash1.clone() * hash2.clone(),
        Operator::Op::NEQUAL => (((metamodelica::OrderedFloat((hash1.clone()) as f64)).powf(metamodelica::OrderedFloat((hash2.clone()) as f64))).0 as i32),
        _ => hash2.clone() - hash1.clone(),
    });
            hash.clone()
        },
        Deref @ Expression::IF { .. } => {
            noNameHashExp(var_field!((*exp).condition, Expression::NFExpression::IF).clone(), r#mod.clone())? + noNameHashExp(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), r#mod.clone())? + noNameHashExp(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), r#mod.clone())?
        },
        Deref @ Expression::CAST { .. } => {
            noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), r#mod.clone())?
        },
        Deref @ Expression::BOX { .. } => {
            noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), r#mod.clone())?
        },
        Deref @ Expression::UNBOX { .. } => {
            noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), r#mod.clone())?
        },
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            noNameHashExp(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), r#mod.clone())?
        },
        Deref @ Expression::TUPLE_ELEMENT { .. } => {
            noNameHashExp(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone(), r#mod.clone())? + var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone()
        },
        Deref @ Expression::RECORD_ELEMENT { .. } => {
            noNameHashExp(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone(), r#mod.clone())? + var_field!((*exp).index, Expression::NFExpression::RECORD_ELEMENT).clone()
        },
        Deref @ Expression::MUTABLE { .. } => {
            noNameHashExp(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone()), r#mod.clone())?
        },
        Deref @ Expression::EMPTY { .. } => {
            stringHashDjb2Mod((literal!("empty")).clone(), r#mod.clone())
        },
        Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. } => {
            for mut arg in &*var_field!((*exp).args, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone() {
                let mut arg = arg.clone();
                hash = hash.clone() + noNameHashExp(arg.clone(), r#mod.clone())?;
            }
            hash.clone()
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hash = intMod(intAbs(hash.clone()), r#mod.clone());
    Ok(hash)
}

pub fn isOnlyTimeDependent(mut exp: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool = false;
    b = Expression::fold(exp.clone(), (std::sync::Arc::new(fnptr!(isOnlyTimeDependentFold, Arc<Expression::NFExpression>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<bool> + 'static>), true)?;
    Ok(b)
}

pub fn isOnlyTimeDependentFold(mut exp: Arc<Expression::NFExpression>, mut b: bool) -> bool {
    let mut b: bool = b;
    if b.clone() {
        b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => ComponentRef::isTime(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()) || BVariable::checkCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(fnptr!(BVariable::isParamOrConst, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!()),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    b
}

pub fn isContinuous(mut exp: Arc<Expression::NFExpression>, mut init: bool) -> Result<bool> {
    let mut b: bool = false;
    b = Expression::fold(exp.clone(), Arc::new({ let __pe_b1 = init.clone(); move |__pe_a0, __pe_a2| Ok(isContinuousFold(__pe_a0, __pe_b1.clone(), __pe_a2)) }), true)?;
    Ok(b)
}

pub fn isContinuousFold(mut exp: Arc<Expression::NFExpression>, mut init: bool, mut b: bool) -> bool {
    let mut b: bool = b;
    if b.clone() {
        b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => BVariable::checkCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), Arc::new({ let __pe_b1 = init.clone(); move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone()) }), metamodelica::sourceInfo!()),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    b
}

pub fn getLocalSystem(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>, mut eqn_indices: Arc<metamodelica::List<i32>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<Matching::NBMatching>, metamodelica::Array<i32>)> {
    let mut m_loc: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut matching_loc: Arc<Matching::NBMatching> = Arc::new(<Matching::NBMatching as ::std::default::Default>::default());
    let mut map_back: metamodelica::Array<i32>;
    let N: i32 = (eqn_indices.clone().len() as i32);
    let mut var_to_eqn: metamodelica::Array<i32> = arrayCreate(N.clone(), -1);
    let mut eqn_to_var: metamodelica::Array<i32> = arrayCreate(N.clone(), -1);
    let mut var_loc: Arc<UnorderedMap::UnorderedMap<i32, i32>> = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), N.clone());
    let mut j: i32 = 1;
    map_back = arrayCreate(N.clone(), -1);
    for mut i in &*eqn_indices.clone() {
        let mut i = i.clone();
        {
            let __cell0 = i.clone();
            map_back.clone().borrow_mut()[(j.clone()-1) as usize] = __cell0;
        }
        UnorderedMap::addUnique(matching.eqn_to_var[(i.clone()-1) as usize].clone(), j.clone(), var_loc.clone())?;
        {
            let __cell1 = j.clone();
            eqn_to_var.clone().borrow_mut()[(j.clone()-1) as usize] = __cell1;
        }
        {
            let __cell2 = j.clone();
            var_to_eqn.clone().borrow_mut()[(j.clone()-1) as usize] = __cell2;
        }
        j = j.clone() + 1;
    }
    matching_loc = Arc::new(Matching::NBMatching { var_to_eqn: var_to_eqn.clone(), eqn_to_var: eqn_to_var.clone() });
    m_loc = arrayCreate(N.clone(), metamodelica::nil());
    for mut j in 1..=N.clone() {
        {
            let __cell3 = UnorderedMap::getList(m.borrow()[(map_back.borrow()[(j.clone()-1) as usize].clone()-1) as usize].clone(), var_loc.clone())?;
            m_loc.clone().borrow_mut()[(j.clone()-1) as usize] = __cell3;
        }
    }
    Ok((m_loc, matching_loc, map_back))
}

pub fn makeFDerString(mut r#str: ArcStr, mut i_opt: Option<i32>) -> ArcStr {
    let mut r#str: ArcStr = r#str;
    let mut i: ArcStr = if (Util::isSome(i_opt.clone())) {intString(Util::getOption(i_opt.clone()).unwrap())} else {literal!("")};
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BVariable::FUNCTION_DERIVATIVE_STR)); __mm_s.push_str(&*i.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
    r#str
}

