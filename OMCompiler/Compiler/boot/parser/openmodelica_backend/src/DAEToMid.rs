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

use crate::HashTableMidVar;
use crate::MidCode;
use crate::MidToMid;
use crate::SimCode;
use crate::SimCodeFunction;
use openmodelica_ast::Absyn;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub fn DAEFunctionsToMid(mut simfuncs: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Arc<metamodelica::List<MidCode::Function>>> {
    let mut midfuncs: Arc<metamodelica::List<MidCode::Function>> = metamodelica::nil();
    midfuncs = {
        let mut __acc: Arc<metamodelica::List<MidCode::Function>> = metamodelica::nil();
        for mut simfunc in (simfuncs.clone()).into_iter().cloned() {
            let __x = DAEFunctionToMid(simfunc.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(midfuncs)
}

#[derive(Clone)]
pub struct State {
    pub locals: DoubleEnded::MutableList<MidCode::Var>,
    pub localBufs: DoubleEnded::MutableList<MidCode::VarBuf>,
    pub localBufPtrs: DoubleEnded::MutableList<MidCode::VarBufPtr>,
    pub blocks: DoubleEnded::MutableList<MidCode::Block>,
    pub stmts: DoubleEnded::MutableList<MidCode::Stmt>,
    pub blockid: Mutable::Mutable<i32>,
    pub continuejumps: Mutable::Mutable<Arc<metamodelica::List<i32>>>,
    pub breakjumps: Mutable::Mutable<Arc<metamodelica::List<i32>>>,
    pub vars: Mutable::Mutable<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, MidCode::Var)>>), i32, (HashTableMidVar::FuncHashCref, HashTableMidVar::FuncCrefEqual, HashTableMidVar::FuncCrefStr, HashTableMidVar::FuncExpStr))>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.locals == other.locals && self.localBufs == other.localBufs && self.localBufPtrs == other.localBufPtrs && self.blocks == other.blocks && self.stmts == other.stmts && self.blockid == other.blockid && self.continuejumps == other.continuejumps && self.breakjumps == other.breakjumps && std::sync::Arc::ptr_eq(&self.vars, &other.vars)
    }
}
impl Eq for State {}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.locals.cmp(&other.locals).then_with(|| self.localBufs.cmp(&other.localBufs).then_with(|| self.localBufPtrs.cmp(&other.localBufPtrs).then_with(|| self.blocks.cmp(&other.blocks).then_with(|| self.stmts.cmp(&other.stmts).then_with(|| self.blockid.cmp(&other.blockid).then_with(|| self.continuejumps.cmp(&other.continuejumps).then_with(|| self.breakjumps.cmp(&other.breakjumps).then_with(|| (std::sync::Arc::as_ptr(&self.vars) as *const ()).cmp(&(std::sync::Arc::as_ptr(&other.vars) as *const ()))))))))))
    }
}
impl std::fmt::Debug for State {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("State");
        __ds.field("locals", &self.locals);
        __ds.field("localBufs", &self.localBufs);
        __ds.field("localBufPtrs", &self.localBufPtrs);
        __ds.field("blocks", &self.blocks);
        __ds.field("stmts", &self.stmts);
        __ds.field("blockid", &self.blockid);
        __ds.field("continuejumps", &self.continuejumps);
        __ds.field("breakjumps", &self.breakjumps);
        __ds.field("vars", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr(&self.vars)));
        __ds.finish()
    }
}

pub type STATE = State;


fn listZip<X: Clone + 'static, Y: Clone + 'static>(mut xs: Arc<metamodelica::List<X>>, mut ys: Arc<metamodelica::List<Y>>) -> Result<Arc<metamodelica::List<(X, Y)>>> {
    let mut zs: Arc<metamodelica::List<(X, Y)>> = metamodelica::nil();
    let mut xs_: Arc<metamodelica::List<X>> = metamodelica::nil();
    let mut ys_: Arc<metamodelica::List<Y>> = metamodelica::nil();
    let mut x: X;
    let mut y: Y;
    zs = (::match_deref::match_deref! { match &((xs.clone(), ys.clone())) {
        (Deref @ metamodelica::List::Nil, _) => metamodelica::nil(),
        (_, Deref @ metamodelica::List::Nil) => metamodelica::nil(),
        (Deref @ metamodelica::List::Cons { head: x, tail: xs_ }, Deref @ metamodelica::List::Cons { head: y, tail: ys_ }) => cons((x.clone(), y.clone()), listZip(xs_.clone(), ys_.clone())?),
        _ => bail!("match: no arm matched"),
    } });
    Ok(zs)
}

fn GenTmpVar(mut ty: Arc<DAE::Type>, mut state: State) -> MidCode::Var {
    let mut var: MidCode::Var;
    var = MidCode::Var { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_tmp_")); __mm_s.push_str(&*intString(System::tmpTickIndex(46))); ArcStr::from(__mm_s) }).clone(), ty: ty.clone(), volatile: false };
    DoubleEnded::push_back(state.locals.clone(), var.clone());
    var
}

fn GenTmpVarVolatile(mut ty: Arc<DAE::Type>, mut state: State) -> MidCode::Var {
    let mut var: MidCode::Var;
    var = MidCode::Var { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_tmp_")); __mm_s.push_str(&*intString(System::tmpTickIndex(46))); ArcStr::from(__mm_s) }).clone(), ty: ty.clone(), volatile: true };
    DoubleEnded::push_back(state.locals.clone(), var.clone());
    var
}

fn GenTmpVarBuf(mut state: State) -> MidCode::VarBuf {
    let mut var: MidCode::VarBuf;
    var = MidCode::VarBuf { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_jmpbuf_")); __mm_s.push_str(&*intString(System::tmpTickIndex(47))); ArcStr::from(__mm_s) }).clone() };
    DoubleEnded::push_back(state.localBufs.clone(), var.clone());
    var
}

fn GenTmpVarBufPtr(mut state: State) -> MidCode::VarBufPtr {
    let mut var: MidCode::VarBufPtr;
    var = MidCode::VarBufPtr { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_tmp_")); __mm_s.push_str(&*intString(System::tmpTickIndex(46))); ArcStr::from(__mm_s) }).clone() };
    DoubleEnded::push_back(state.localBufPtrs.clone(), var.clone());
    var
}

fn GenBlockId() -> i32 {
    let mut id: i32 = 0;
    id = System::tmpTickIndex(45);
    id
}

fn ConvertSimCodeVars(mut simcodevar: Arc<SimCodeFunction::Variable::Variable>, mut state: State) -> Result<MidCode::Var> {
    let mut var: MidCode::Var;
    var = (::match_deref::match_deref! { match &(simcodevar.clone()) {
        Deref @ SimCodeFunction::Variable::VARIABLE { name: _, .. } => {
            let mut midcodevar: MidCode::Var;
            midcodevar = CrefToMidVar(var_field!((*simcodevar).name, SimCodeFunction::Variable::Variable::VARIABLE).clone(), state.clone())?;
            let () = (::match_deref::match_deref! { match &(var_field!((*simcodevar).value, SimCodeFunction::Variable::Variable::VARIABLE).clone()) {
        None => {
            ()
        },
        Some(exp) => {
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: midcodevar.clone(), src: ExpToMid(exp.clone(), state.clone())? }, state.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            midcodevar.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

fn GetCrefIndexVar(mut cref: Arc<DAE::ComponentRef>, mut state: State) -> Result<Option<MidCode::Var>> {
    let mut var: Option<MidCode::Var> = None;
    let mut subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    subscripts = ComponentReference::crefLastSubs(cref.clone())?;
    var = (::match_deref::match_deref! { match &(subscripts.clone()) {
        Deref @ metamodelica::List::Nil => {
            None
        },
        Deref @ metamodelica::List::Cons { head: subscript @ Deref @ DAE::Subscript::INDEX { exp: _ }, tail: Deref @ metamodelica::List::Nil } => {
            let mut indexvar: MidCode::Var;
            indexvar = RValueToVar(ExpToMid(var_field!((**subscript).exp, DAE::Subscript::INDEX).clone(), state.clone())?, state.clone())?;
            Some(indexvar.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

fn CrefToMidVar(mut cref: Arc<DAE::ComponentRef>, mut state: State) -> Result<MidCode::Var> {
    let mut var: MidCode::Var;
    let mut ident: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    if !(BaseHashTable::hasKey(cref.clone(), Mutable::access(state.vars.clone()))) {
        (ident, ty) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: ident_, identType: ty_, subscriptLst: _ } => {
            (ident_.clone(), ty_.clone())
        },
        _ => {
            Error::addInternalError((literal!("CrefToMidVar error")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Mutable::update(state.vars.clone(), BaseHashTable::add((cref.clone(), MidCode::Var { name: (ident.clone()).clone(), ty: Types::complicateType(ty.clone())?, volatile: false }), Mutable::access(state.vars.clone()))?);
    }
    var = BaseHashTable::get(cref.clone(), Mutable::access(state.vars.clone()))?;
    Ok(var)
}

fn RValueType(mut rvalue: MidCode::RValue) -> Result<Arc<DAE::Type>> {
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = (match rvalue.clone() {
        MidCode::RValue::VARIABLE { src: _ } => var_field!(rvalue.src, MidCode::RValue::VARIABLE).ty.clone(),
        MidCode::RValue::BINARYOP { op: _, .. } => (match var_field!(rvalue.op, MidCode::RValue::BINARYOP).clone() {
        MidCode::BinaryOp::LESS => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::BinaryOp::LESSEQ => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::BinaryOp::GREATER => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::BinaryOp::GREATEREQ => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::BinaryOp::EQUAL => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::BinaryOp::NEQUAL => DAE::T_BOOL_DEFAULT().clone(),
        _ => var_field!(rvalue.lsrc, MidCode::RValue::BINARYOP).ty.clone(),
    }),
        MidCode::RValue::UNARYOP { op: MidCode::UnaryOp::BOX, src: _ } => Types::boxIfUnboxedType(var_field!(rvalue.src, MidCode::RValue::UNARYOP).ty.clone())?,
        MidCode::RValue::UNARYOP { op: MidCode::UnaryOp::UNBOX, src: _ } => Types::unboxedType(var_field!(rvalue.src, MidCode::RValue::UNARYOP).ty.clone())?,
        MidCode::RValue::UNARYOP { op: _, .. } => var_field!(rvalue.src, MidCode::RValue::UNARYOP).ty.clone(),
        MidCode::RValue::LITERALINTEGER { value: _ } => DAE::T_INTEGER_DEFAULT().clone(),
        MidCode::RValue::LITERALREAL { value: _ } => DAE::T_REAL_DEFAULT().clone(),
        MidCode::RValue::LITERALBOOLEAN { value: _ } => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::RValue::LITERALSTRING { value: _ } => DAE::T_STRING_DEFAULT().clone(),
        MidCode::RValue::LITERALMETATYPE { elements: _, .. } => var_field!(rvalue.ty, MidCode::RValue::LITERALMETATYPE).clone(),
        MidCode::RValue::METAFIELD { src: _, .. } => var_field!(rvalue.ty, MidCode::RValue::METAFIELD).clone(),
        MidCode::RValue::UNIONTYPEVARIANT { src: _ } => DAE::T_INTEGER_DEFAULT().clone(),
        MidCode::RValue::ISCONS { src: _ } => DAE::T_BOOL_DEFAULT().clone(),
        MidCode::RValue::ISSOME { src: _ } => DAE::T_BOOL_DEFAULT().clone(),
        _ => {
            Error::addInternalError((literal!("Could not find the correct type of an RValue.\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(ty)
}

fn RValueToVar(mut rvalue: MidCode::RValue, mut state: State) -> Result<MidCode::Var> {
    let mut var: MidCode::Var;
    var = (match rvalue.clone() {
        MidCode::RValue::VARIABLE { src: _ } => {
            var_field!(rvalue.src, MidCode::RValue::VARIABLE).clone()
        },
        _ => {
            let mut tmpvar: MidCode::Var;
            tmpvar = GenTmpVar(Types::complicateType(RValueType(rvalue.clone())?)?, state.clone());
            DoubleEnded::push_back(state.stmts.clone(), MidCode::Stmt::ASSIGN { dest: tmpvar.clone(), src: rvalue.clone() });
            tmpvar.clone()
        },
    });
    Ok(var)
}

fn DAEFunctionToMid(mut simfunc: Arc<SimCodeFunction::Function::Function>) -> Result<MidCode::Function> {
    let mut midfunc: MidCode::Function;
    let mut state: State;
    let mut inputs: DoubleEnded::MutableList<MidCode::Var>;
    let mut outputs: DoubleEnded::MutableList<MidCode::Var>;
    let mut block_: MidCode::Block;
    let mut path: Arc<Absyn::Path>;
    let mut labelFirst: i32 = 0;
    System::tmpTickReset(47);
    System::tmpTickReset(46);
    System::tmpTickReset(45);
    let () = (::match_deref::match_deref! { match &(simfunc.clone()) {
        Deref @ SimCodeFunction::Function::FUNCTION { name, outVars, functionArguments, variableDeclarations, body, visibility: _, info: _ } => {
            labelFirst = GenBlockId();
            path = name.clone();
            inputs = DoubleEnded::fromList(metamodelica::nil())?;
            outputs = DoubleEnded::fromList(metamodelica::nil())?;
            state = State { locals: DoubleEnded::fromList(metamodelica::nil())?, localBufs: DoubleEnded::fromList(metamodelica::nil())?, localBufPtrs: DoubleEnded::fromList(metamodelica::nil())?, blocks: DoubleEnded::fromList(metamodelica::nil())?, stmts: DoubleEnded::fromList(metamodelica::nil())?, blockid: Mutable::create(labelFirst.clone()), continuejumps: Mutable::create(metamodelica::nil()), breakjumps: Mutable::create(metamodelica::nil()), vars: Mutable::create(HashTableMidVar::emptyHashTable()) };
            for mut simcodeVar in &*variableDeclarations.clone() {
                let mut simcodeVar = simcodeVar.clone();
                DoubleEnded::push_back(state.locals.clone(), ConvertSimCodeVars(simcodeVar.clone(), state.clone())?);
            }
            for mut simcodeVar in &*outVars.clone() {
                let mut simcodeVar = simcodeVar.clone();
                DoubleEnded::push_back(outputs.clone(), ConvertSimCodeVars(simcodeVar.clone(), state.clone())?);
            }
            for mut simcodeVar in &*functionArguments.clone() {
                let mut simcodeVar = simcodeVar.clone();
                DoubleEnded::push_back(inputs.clone(), ConvertSimCodeVars(simcodeVar.clone(), state.clone())?);
            }
            StmtsToMid(body.clone(), state.clone())?;
            ()
        },
        _ => {
            Error::addInternalError((literal!("Unsupported SimCodeFunction.Function type\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail");
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stateTerminate(-1, crate::MidCode::Terminator::RETURN, state.clone());
    midfunc = MidCode::Function { exitId: GenBlockId(), entryId: labelFirst.clone(), body: DoubleEnded::toListAndClear(state.blocks.clone(), metamodelica::nil()), outputs: DoubleEnded::toListAndClear(outputs.clone(), metamodelica::nil()), inputs: DoubleEnded::toListAndClear(inputs.clone(), metamodelica::nil()), localBufPtrs: DoubleEnded::toListAndClear(state.localBufPtrs.clone(), metamodelica::nil()), localBufs: DoubleEnded::toListAndClear(state.localBufs.clone(), metamodelica::nil()), locals: DoubleEnded::toListAndClear(state.locals.clone(), metamodelica::nil()), name: path.clone() };
    midfunc = MidToMid::longJmpGoto(midfunc.clone())?;
    Ok(midfunc)
}

fn StmtsToMid(mut daestmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut state: State) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(daestmts.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: stmt, tail: tail } => {
            let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { type_: _, exp1: exp1 @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, exp, source: _ } => {
            let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut varCref: MidCode::Var;
            cref = ComponentReferenceBasics::crefLastCref(var_field!((**exp1).componentRef, DAE::Exp::CREF).clone())?;
            varCref = CrefToMidVar(cref.clone(), state.clone())?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varCref.clone(), src: ExpToMid(exp.clone(), state.clone())? }, state.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_ASSIGN { type_: _, exp1: exp1 @ Deref @ DAE::Exp::ASUB { exp: _, .. }, exp, source: _ } => {
            let mut varArray: MidCode::Var;
            let mut varIndex: MidCode::Var;
            let mut varValue: MidCode::Var;
            let mut labelNext: i32 = 0;
            varArray = RValueToVar(ExpToMid(var_field!((**exp1).exp, DAE::Exp::ASUB).clone(), state.clone())?, state.clone())?;
            varIndex = (::match_deref::match_deref! { match &(var_field!((**exp1).sub, DAE::Exp::ASUB).clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: indexexp }, tail: Deref @ metamodelica::List::Nil } => {
            RValueToVar(ExpToMid(indexexp.clone(), state.clone())?, state.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
            varValue = RValueToVar(ExpToMid(exp.clone(), state.clone())?, state.clone())?;
            labelNext = GenBlockId();
            stateTerminate(labelNext.clone(), MidCode::Terminator::CALL { func: Arc::new(Absyn::Path::IDENT { name: (literal!("arrayUpdate")).clone() }), builtin: true, inputs: list![varArray.clone(), varIndex.clone(), varValue.clone()], outputs: metamodelica::nil(), next: labelNext.clone() }, state.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_ASSIGN { type_: _, exp1: Deref @ DAE::Exp::PATTERN { pattern }, exp, source: _ } => {
            let mut varRHS: MidCode::Var;
            varRHS = RValueToVar(ExpToMid(exp.clone(), state.clone())?, state.clone())?;
            patternToMidCode(list![(varRHS.clone(), pattern.clone())], 1, state.clone())?;
            ()
        },
        Deref @ DAE::Statement::STMT_ASSIGN { type_: _, .. } => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE.STMT_ASSIGN to Mid conversion failed ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(var_field!((**stmt).exp1, DAE::Statement::STMT_ASSIGN).clone(), 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: _, expExpLst: expLst, exp, source: _ } => {
            let mut exp1: Arc<DAE::Exp>;
            let mut varCref: MidCode::Var;
            let mut outvars: DoubleEnded::MutableList<MidCode::OutVar>;
            outvars = DoubleEnded::fromList(metamodelica::nil())?;
            for mut exp1 in &*expLst.clone() {
                let mut exp1 = exp1.clone();
                let () = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. } => {
            DoubleEnded::push_back(outvars.clone(), crate::MidCode::OutVar::OUT_WILD);
            ()
        },
        Deref @ DAE::Exp::CREF { componentRef: _, .. } => {
            varCref = CrefToMidVar(var_field!((*exp1).componentRef, DAE::Exp::CREF).clone(), state.clone())?;
            DoubleEnded::push_back(outvars.clone(), MidCode::OutVar::OUT_VAR { var: varCref.clone() });
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("outvars convertion failed ")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(exp1.clone(), 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { path: _, .. } => {
            CallToMid(exp.clone(), DoubleEnded::toListAndClear(outvars.clone(), metamodelica::nil()), state.clone())?;
            ()
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { matchType: _, .. } => {
            MatchExpressionToMid(exp.clone(), DoubleEnded::toListAndClear(outvars.clone(), metamodelica::nil()), state.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            ()
        },
        Deref @ DAE::Statement::STMT_IF { exp: _, .. } => {
            IfToMid(var_field!((**stmt).exp, DAE::Statement::STMT_IF).clone(), var_field!((**stmt).statementLst, DAE::Statement::STMT_IF).clone(), var_field!((**stmt).else_, DAE::Statement::STMT_IF).clone(), state.clone())?;
            ()
        },
        Deref @ DAE::Statement::STMT_WHILE { exp: _, .. } => {
            let mut varCondition: MidCode::Var;
            let mut labelBody: i32 = 0;
            let mut labelNext: i32 = 0;
            let mut labelCondition: i32 = 0;
            labelCondition = GenBlockId();
            labelBody = GenBlockId();
            labelNext = GenBlockId();
            Mutable::update(state.continuejumps.clone(), cons(labelCondition.clone(), Mutable::access(state.continuejumps.clone())));
            Mutable::update(state.breakjumps.clone(), cons(labelNext.clone(), Mutable::access(state.breakjumps.clone())));
            stateTerminate(labelCondition.clone(), MidCode::Terminator::GOTO { next: labelCondition.clone() }, state.clone());
            varCondition = RValueToVar(ExpToMid(var_field!((**stmt).exp, DAE::Statement::STMT_WHILE).clone(), state.clone())?, state.clone())?;
            stateTerminate(labelBody.clone(), MidCode::Terminator::BRANCH { condition: varCondition.clone(), onTrue: labelBody.clone(), onFalse: labelNext.clone() }, state.clone());
            StmtsToMid(var_field!((**stmt).statementLst, DAE::Statement::STMT_WHILE).clone(), state.clone())?;
            stateTerminate(labelNext.clone(), MidCode::Terminator::GOTO { next: labelCondition.clone() }, state.clone());
            Mutable::update(state.continuejumps.clone(), listRest(Mutable::access(state.continuejumps.clone()))?);
            Mutable::update(state.breakjumps.clone(), listRest(Mutable::access(state.breakjumps.clone()))?);
            ()
        },
        Deref @ DAE::Statement::STMT_FOR { type_: _, .. } => {
            ForToMid(var_field!((**stmt).type_, DAE::Statement::STMT_FOR).clone(), (var_field!((**stmt).iter, DAE::Statement::STMT_FOR).clone()).clone(), var_field!((**stmt).range, DAE::Statement::STMT_FOR).clone(), var_field!((**stmt).statementLst, DAE::Statement::STMT_FOR).clone(), state.clone())?;
            ()
        },
        Deref @ DAE::Statement::STMT_BREAK { source: _ } => {
            let mut labelNext: i32 = 0;
            labelNext = GenBlockId();
            stateTerminate(labelNext.clone(), MidCode::Terminator::GOTO { next: listHead(Mutable::access(state.breakjumps.clone()))? }, state.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_CONTINUE { source: _ } => {
            let mut labelNext: i32 = 0;
            labelNext = GenBlockId();
            stateTerminate(labelNext.clone(), MidCode::Terminator::GOTO { next: listHead(Mutable::access(state.continuejumps.clone()))? }, state.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_RETURN { source: _ } => {
            let mut labelNext: i32 = 0;
            labelNext = GenBlockId();
            stateTerminate(labelNext.clone(), crate::MidCode::Terminator::RETURN, state.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_NORETCALL { exp: _, .. } => {
            let () = (::match_deref::match_deref! { match &(var_field!((**stmt).exp, DAE::Statement::STMT_NORETCALL).clone()) {
        Deref @ DAE::Exp::CALL { path: _, .. } => {
            CallToMid(var_field!((**stmt).exp, DAE::Statement::STMT_NORETCALL).clone(), metamodelica::nil(), state.clone())?;
            ()
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { matchType: _, .. } => {
            MatchExpressionToMid(var_field!((**stmt).exp, DAE::Statement::STMT_NORETCALL).clone(), metamodelica::nil(), state.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            ()
        },
        Deref @ DAE::Statement::STMT_ASSERT { cond: _, .. } => {
            let mut varCondition: MidCode::Var;
            let mut varMessage: MidCode::Var;
            let mut varLevel: MidCode::Var;
            let mut labelNext: i32 = 0;
            varCondition = RValueToVar(ExpToMid(var_field!((**stmt).cond, DAE::Statement::STMT_ASSERT).clone(), state.clone())?, state.clone())?;
            varMessage = RValueToVar(ExpToMid(var_field!((**stmt).msg, DAE::Statement::STMT_ASSERT).clone(), state.clone())?, state.clone())?;
            varLevel = RValueToVar(ExpToMid(var_field!((**stmt).level, DAE::Statement::STMT_ASSERT).clone(), state.clone())?, state.clone())?;
            labelNext = GenBlockId();
            stateTerminate(labelNext.clone(), MidCode::Terminator::ASSERT { condition: varCondition.clone(), message: varMessage.clone(), level: varLevel.clone(), next: labelNext.clone() }, state.clone());
            ()
        },
        Deref @ DAE::Statement::STMT_TERMINATE { msg: _, .. } => {
            let mut varMessage: MidCode::Var;
            let mut labelNext: i32 = 0;
            varMessage = RValueToVar(ExpToMid(var_field!((**stmt).msg, DAE::Statement::STMT_TERMINATE).clone(), state.clone())?, state.clone())?;
            labelNext = GenBlockId();
            stateTerminate(labelNext.clone(), MidCode::Terminator::TERMINATE { message: varMessage.clone() }, state.clone());
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE.Statement to Mid conversion failed ")); __mm_s.push_str(&*DAEDump::ppStatementStr(stmt.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            StmtsToMid(tail.clone(), state.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn ExpToMid(mut exp: Arc<DAE::Exp>, mut state: State) -> Result<MidCode::RValue> {
    let mut rval: MidCode::RValue;
    rval = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: _ } => {
            MidCode::RValue::LITERALINTEGER { value: var_field!((*exp).integer, DAE::Exp::ICONST).clone() }
        },
        Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. } => {
            MidCode::RValue::LITERALINTEGER { value: var_field!((*exp).index, DAE::Exp::ENUM_LITERAL).clone() }
        },
        Deref @ DAE::Exp::RCONST { real: _ } => {
            MidCode::RValue::LITERALREAL { value: var_field!((*exp).real, DAE::Exp::RCONST).clone() }
        },
        Deref @ DAE::Exp::SCONST { string: _ } => {
            MidCode::RValue::LITERALSTRING { value: (var_field!((*exp).string, DAE::Exp::SCONST).clone()).clone() }
        },
        Deref @ DAE::Exp::SHARED_LITERAL { index: _, .. } => {
            ExpToMid(var_field!((*exp).exp, DAE::Exp::SHARED_LITERAL).clone(), state.clone())?
        },
        Deref @ DAE::Exp::BOX { exp: _ } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(var_field!((*exp).exp, DAE::Exp::BOX).clone(), state.clone())?, state.clone())?;
            MidCode::RValue::UNARYOP { op: crate::MidCode::UnaryOp::BOX, src: varExp.clone() }
        },
        Deref @ DAE::Exp::UNBOX { exp: _, .. } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(var_field!((*exp).exp, DAE::Exp::UNBOX).clone(), state.clone())?, state.clone())?;
            MidCode::RValue::UNARYOP { op: crate::MidCode::UnaryOp::UNBOX, src: varExp.clone() }
        },
        Deref @ DAE::Exp::BCONST { bool: _ } => {
            MidCode::RValue::LITERALBOOLEAN { value: var_field!((*exp).bool, DAE::Exp::BCONST).clone() }
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(exp1) } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
            MidCode::RValue::LITERALMETATYPE { elements: list![varExp.clone()], ty: Types::complicateType(Arc::new(DAE::Type::T_METAOPTION { ty: varExp.ty.clone() }))? }
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            MidCode::RValue::LITERALMETATYPE { elements: metamodelica::nil(), ty: Types::complicateType(DAE::T_NONE_DEFAULT().clone())? }
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expLst } => {
            let mut varExp: MidCode::Var;
            let mut values: DoubleEnded::MutableList<MidCode::Var>;
            values = DoubleEnded::fromList(metamodelica::nil())?;
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                varExp = RValueToVar(ExpToMid(exp.clone(), state.clone())?, state.clone())?;
                DoubleEnded::push_back(values.clone(), varExp.clone());
            }
            MidCode::RValue::LITERALMETATYPE { elements: DoubleEnded::toListAndClear(values.clone(), metamodelica::nil()), ty: Types::complicateType(Expression::r#typeof(exp.clone())?)? }
        },
        Deref @ DAE::Exp::METARECORDCALL { path: _, args: expLst, fieldNames: _, index: _, typeVars: _ } => {
            let mut varExp: MidCode::Var;
            let mut values: DoubleEnded::MutableList<MidCode::Var>;
            values = DoubleEnded::fromList(metamodelica::nil())?;
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                varExp = RValueToVar(ExpToMid(exp.clone(), state.clone())?, state.clone())?;
                DoubleEnded::push_back(values.clone(), varExp.clone());
            }
            MidCode::RValue::LITERALMETATYPE { elements: DoubleEnded::toListAndClear(values.clone(), metamodelica::nil()), ty: Types::complicateType(Expression::r#typeof(exp.clone())?)? }
        },
        Deref @ DAE::Exp::CONS { car: _, .. } => {
            let mut varCar: MidCode::Var;
            let mut varCdr: MidCode::Var;
            varCar = RValueToVar(ExpToMid(var_field!((*exp).car, DAE::Exp::CONS).clone(), state.clone())?, state.clone())?;
            varCdr = RValueToVar(ExpToMid(var_field!((*exp).cdr, DAE::Exp::CONS).clone(), state.clone())?, state.clone())?;
            MidCode::RValue::LITERALMETATYPE { elements: list![varCar.clone(), varCdr.clone()], ty: Types::complicateType(Arc::new(DAE::Type::T_METALIST { ty: varCar.ty.clone() }))? }
        },
        Deref @ DAE::Exp::LIST { valList: expLst } => {
            let mut varCar: MidCode::Var;
            let mut varCdr: MidCode::Var;
            let mut varTmp: MidCode::Var;
            let mut expLst = (*expLst).clone();
            expLst = expLst.clone().reverse();
            varCdr = GenTmpVar(DAE::T_METALIST_DEFAULT().clone(), state.clone());
            DoubleEnded::push_back(state.stmts.clone(), MidCode::Stmt::ASSIGN { dest: varCdr.clone(), src: MidCode::RValue::LITERALMETATYPE { elements: metamodelica::nil(), ty: DAE::T_METALIST_DEFAULT().clone() } });
            for mut exp in &*expLst.clone() {
                let mut exp = exp.clone();
                varCar = RValueToVar(ExpToMid(exp.clone(), state.clone())?, state.clone())?;
                varTmp = GenTmpVar(Arc::new(DAE::Type::T_METALIST { ty: Types::complicateType(varCar.ty.clone())? }), state.clone());
                DoubleEnded::push_back(state.stmts.clone(), MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: MidCode::RValue::LITERALMETATYPE { elements: list![varCar.clone(), varCdr.clone()], ty: Types::complicateType(Arc::new(DAE::Type::T_METALIST { ty: varCar.ty.clone() }))? } });
                varCdr = varTmp.clone();
            }
            MidCode::RValue::VARIABLE { src: varCdr.clone() }
        },
        Deref @ DAE::Exp::CREF { componentRef: cref, ty: _ } => {
            let mut varCref: MidCode::Var;
            let mut varTmp: MidCode::Var;
            let mut labelNext: i32 = 0;
            let mut rvalue: MidCode::RValue;
            varCref = CrefToMidVar(cref.clone(), state.clone())?;
            rvalue = (match GetCrefIndexVar(cref.clone(), state.clone())? {
        None => {
            MidCode::RValue::VARIABLE { src: varCref.clone() }
        },
        Some(mut indexvar) => {
            labelNext = GenBlockId();
            varTmp = GenTmpVar(Types::complicateType(Expression::r#typeof(exp.clone())?)?, state.clone());
            stateTerminate(labelNext.clone(), MidCode::Terminator::CALL { func: Arc::new(Absyn::Path::IDENT { name: (literal!("arrayGet")).clone() }), builtin: true, inputs: list![varCref.clone(), indexvar.clone()], outputs: list![MidCode::OutVar::OUT_VAR { var: varTmp.clone() }], next: labelNext.clone() }, state.clone());
            MidCode::RValue::VARIABLE { src: varTmp.clone() }
        },
    });
            rvalue.clone()
        },
        Deref @ DAE::Exp::ASUB { exp: exp1, sub: subscripts } => {
            let mut varExp: MidCode::Var;
            let mut varExp2: MidCode::Var;
            let mut varTmp: MidCode::Var;
            let mut labelNext: i32 = 0;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            expLst = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subscripts.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            varExp = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
            varExp2 = (::match_deref::match_deref! { match &(expLst.clone()) {
        Deref @ metamodelica::List::Cons { head: indexexp, tail: Deref @ metamodelica::List::Nil } => {
            RValueToVar(ExpToMid(indexexp.clone(), state.clone())?, state.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
            varTmp = GenTmpVar(Types::complicateType(Expression::r#typeof(exp.clone())?)?, state.clone());
            labelNext = GenBlockId();
            stateTerminate(labelNext.clone(), MidCode::Terminator::CALL { func: Arc::new(Absyn::Path::IDENT { name: (literal!("arrayGet")).clone() }), builtin: true, inputs: list![varExp.clone(), varExp2.clone()], outputs: list![MidCode::OutVar::OUT_VAR { var: varTmp.clone() }], next: labelNext.clone() }, state.clone());
            MidCode::RValue::VARIABLE { src: varTmp.clone() }
        },
        Deref @ DAE::Exp::TSUB { exp: exp1 @ Deref @ DAE::Exp::CALL { path: _, expLst: _, attr: callattrs }, ix: 1, ty: _ } => {
            let mut varTmp: MidCode::Var;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut numTailTypes: i32 = 0;
            let mut outvars: Arc<metamodelica::List<MidCode::OutVar>> = metamodelica::nil();
            (ty, numTailTypes) = (::match_deref::match_deref! { match &(callattrs.ty.clone()) {
        Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: actualType, tail: tailTypes }, .. } => {
            (actualType.clone(), (tailTypes.clone().len() as i32))
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            varTmp = GenTmpVar(Types::complicateType(ty.clone())?, state.clone());
            outvars = metamodelica::nil();
            for mut i in 1..=numTailTypes.clone() {
                outvars = cons(crate::MidCode::OutVar::OUT_WILD, outvars.clone());
            }
            outvars = cons(MidCode::OutVar::OUT_VAR { var: varTmp.clone() }, outvars.clone());
            CallToMid(exp1.clone(), outvars.clone(), state.clone())?;
            MidCode::RValue::VARIABLE { src: varTmp.clone() }
        },
        Deref @ DAE::Exp::TSUB { exp: _, .. } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(var_field!((*exp).exp, DAE::Exp::TSUB).clone(), state.clone())?, state.clone())?;
            MidCode::RValue::METAFIELD { src: varExp.clone(), index: var_field!((*exp).ix, DAE::Exp::TSUB).clone(), ty: Types::complicateType(var_field!((*exp).ty, DAE::Exp::TSUB).clone())? }
        },
        Deref @ DAE::Exp::RSUB { exp: _, .. } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(var_field!((*exp).exp, DAE::Exp::RSUB).clone(), state.clone())?, state.clone())?;
            MidCode::RValue::METAFIELD { src: varExp.clone(), index: var_field!((*exp).ix, DAE::Exp::RSUB).clone(), ty: Types::complicateType(var_field!((*exp).ty, DAE::Exp::RSUB).clone())? }
        },
        Deref @ DAE::Exp::CAST { ty: _, exp: exp1 } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
            MidCode::RValue::UNARYOP { op: crate::MidCode::UnaryOp::MOVE, src: varExp.clone() }
        },
        Deref @ DAE::Exp::LUNARY { operator: _, exp: exp1 } => {
            let mut varExp: MidCode::Var;
            varExp = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
            MidCode::RValue::UNARYOP { op: crate::MidCode::UnaryOp::NOT, src: varExp.clone() }
        },
        Deref @ DAE::Exp::LBINARY { exp1, operator, exp2 } => {
            let mut varTmp: MidCode::Var;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut labelElse: i32 = 0;
            let mut labelNext: i32 = 0;
            let mut terminator: MidCode::Terminator = MidCode::Terminator::LONGJMP;
            labelElse = GenBlockId();
            labelNext = GenBlockId();
            ty = (match operator.clone() {
        DAE::Operator::AND { ty: _ } => var_field!(operator.ty, DAE::Operator::AND).clone(),
        DAE::Operator::OR { ty: _ } => var_field!(operator.ty, DAE::Operator::OR).clone(),
        _ => bail!("match: no arm matched"),
    });
            varTmp = GenTmpVar(ty.clone(), state.clone());
            terminator = (match operator.clone() {
        DAE::Operator::AND { ty: _ } => MidCode::Terminator::BRANCH { condition: varTmp.clone(), onTrue: labelElse.clone(), onFalse: labelNext.clone() },
        DAE::Operator::OR { ty: _ } => MidCode::Terminator::BRANCH { condition: varTmp.clone(), onTrue: labelNext.clone(), onFalse: labelElse.clone() },
        _ => bail!("match: no arm matched"),
    });
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: ExpToMid(exp1.clone(), state.clone())? }, state.clone());
            stateTerminate(labelElse.clone(), terminator.clone(), state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: ExpToMid(exp2.clone(), state.clone())? }, state.clone());
            stateTerminate(labelNext.clone(), MidCode::Terminator::GOTO { next: labelNext.clone() }, state.clone());
            MidCode::RValue::VARIABLE { src: varTmp.clone() }
        },
        Deref @ DAE::Exp::UNARY { operator, exp: exp1 } => {
            let mut varExp: MidCode::Var;
            let mut unop: MidCode::UnaryOp = MidCode::UnaryOp::BOX;
            unop = (match operator.clone() {
        DAE::Operator::UMINUS { ty: _ } => crate::MidCode::UnaryOp::UMINUS,
        _ => bail!("match: no arm matched"),
    });
            varExp = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
            MidCode::RValue::UNARYOP { op: unop.clone(), src: varExp.clone() }
        },
        Deref @ DAE::Exp::BINARY { exp1, operator, exp2 } => {
            let mut varExp: MidCode::Var;
            let mut varExp2: MidCode::Var;
            let mut binop: MidCode::BinaryOp = MidCode::BinaryOp::ADD;
            binop = (match operator.clone() {
        DAE::Operator::ADD { ty: _ } => crate::MidCode::BinaryOp::ADD,
        DAE::Operator::SUB { ty: _ } => crate::MidCode::BinaryOp::SUB,
        DAE::Operator::MUL { ty: _ } => crate::MidCode::BinaryOp::MUL,
        DAE::Operator::DIV { ty: _ } => crate::MidCode::BinaryOp::DIV,
        DAE::Operator::POW { ty: _ } => crate::MidCode::BinaryOp::POW,
        _ => bail!("match: no arm matched"),
    });
            varExp = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
            varExp2 = RValueToVar(ExpToMid(exp2.clone(), state.clone())?, state.clone())?;
            MidCode::RValue::BINARYOP { op: binop.clone(), lsrc: varExp.clone(), rsrc: varExp2.clone() }
        },
        Deref @ DAE::Exp::RELATION { exp1, operator, exp2, index: _, optionExpisASUB: _ } => {
            let mut varExp: MidCode::Var;
            let mut varExp2: MidCode::Var;
            let mut binop: MidCode::BinaryOp = MidCode::BinaryOp::ADD;
            binop = (match operator.clone() {
        DAE::Operator::LESS { ty: _ } => crate::MidCode::BinaryOp::LESS,
        DAE::Operator::LESSEQ { ty: _ } => crate::MidCode::BinaryOp::LESSEQ,
        DAE::Operator::GREATER { ty: _ } => crate::MidCode::BinaryOp::GREATER,
        DAE::Operator::GREATEREQ { ty: _ } => crate::MidCode::BinaryOp::GREATEREQ,
        DAE::Operator::EQUAL { ty: _ } => crate::MidCode::BinaryOp::EQUAL,
        DAE::Operator::NEQUAL { ty: _ } => crate::MidCode::BinaryOp::NEQUAL,
        _ => bail!("match: no arm matched"),
    });
            varExp = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
            varExp2 = RValueToVar(ExpToMid(exp2.clone(), state.clone())?, state.clone())?;
            MidCode::RValue::BINARYOP { op: binop.clone(), lsrc: varExp.clone(), rsrc: varExp2.clone() }
        },
        Deref @ DAE::Exp::IFEXP { expCond: exp1, expThen: exp2, expElse: exp3 } => {
            let mut varExp: MidCode::Var;
            let mut varTmp: MidCode::Var;
            let mut labelBody: i32 = 0;
            let mut labelElse: i32 = 0;
            let mut labelNext: i32 = 0;
            labelBody = GenBlockId();
            labelElse = GenBlockId();
            labelNext = GenBlockId();
            varExp = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
            varTmp = GenTmpVar(Types::complicateType(Expression::r#typeof(exp2.clone())?)?, state.clone());
            stateTerminate(labelBody.clone(), MidCode::Terminator::BRANCH { condition: varExp.clone(), onTrue: labelBody.clone(), onFalse: labelElse.clone() }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: ExpToMid(exp2.clone(), state.clone())? }, state.clone());
            stateTerminate(labelElse.clone(), MidCode::Terminator::GOTO { next: labelNext.clone() }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varTmp.clone(), src: ExpToMid(exp3.clone(), state.clone())? }, state.clone());
            stateTerminate(labelNext.clone(), MidCode::Terminator::GOTO { next: labelNext.clone() }, state.clone());
            MidCode::RValue::VARIABLE { src: varTmp.clone() }
        },
        Deref @ DAE::Exp::CALL { path: _, expLst: _, attr: callattrs } => {
            let mut varTmp: MidCode::Var;
            varTmp = GenTmpVar(Types::complicateType(callattrs.ty.clone())?, state.clone());
            CallToMid(exp.clone(), list![MidCode::OutVar::OUT_VAR { var: varTmp.clone() }], state.clone())?;
            MidCode::RValue::VARIABLE { src: varTmp.clone() }
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { et: ty, .. } => {
            let mut varTmp: MidCode::Var;
            varTmp = GenTmpVar(Types::complicateType(ty.clone())?, state.clone());
            let () = (::match_deref::match_deref! { match &(Types::complicateType(ty.clone())?) {
        Deref @ DAE::Type::T_TUPLE { types: _, .. } => {
            Error::addInternalError((literal!("Not supposed to get tuple here.\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            MatchExpressionToMid(exp.clone(), list![MidCode::OutVar::OUT_VAR { var: varTmp.clone() }], state.clone())?;
            MidCode::RValue::VARIABLE { src: varTmp.clone() }
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE.Exp to Mid conversion failed:\n")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(exp.clone(), 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rval)
}

fn CallToMid(mut call: Arc<DAE::Exp>, mut outvars: Arc<metamodelica::List<MidCode::OutVar>>, mut state: State) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ DAE::Exp::CALL { path, expLst, attr: callattr } => {
            let mut labelNext: i32 = 0;
            let mut inputs: DoubleEnded::MutableList<MidCode::Var>;
            let mut var1: MidCode::Var;
            labelNext = GenBlockId();
            inputs = DoubleEnded::fromList(metamodelica::nil())?;
            for mut exp1 in &*expLst.clone() {
                let mut exp1 = exp1.clone();
                var1 = RValueToVar(ExpToMid(exp1.clone(), state.clone())?, state.clone())?;
                DoubleEnded::push_back(inputs.clone(), var1.clone());
            }
            stateTerminate(labelNext.clone(), MidCode::Terminator::CALL { func: path.clone(), builtin: callattr.builtin.clone(), inputs: DoubleEnded::toListAndClear(inputs.clone(), metamodelica::nil()), outputs: outvars.clone(), next: labelNext.clone() }, state.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn ForToMid(mut type_: Arc<DAE::Type>, mut iter: ArcStr, mut range: Arc<DAE::Exp>, mut daestmtLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut state: State) -> Result<()> {
    let mut varCref: MidCode::Var;
    let mut varCondition: MidCode::Var;
    let mut labelCondition: i32 = 0;
    let mut labelStep: i32 = 0;
    let mut labelBody: i32 = 0;
    let mut labelNext: i32 = 0;
    varCref = CrefToMidVar(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (iter.clone()).clone(), identType: type_.clone(), subscriptLst: metamodelica::nil() }), state.clone())?;
    DoubleEnded::push_back(state.locals.clone(), varCref.clone());
    labelCondition = GenBlockId();
    labelStep = GenBlockId();
    labelBody = GenBlockId();
    labelNext = GenBlockId();
    Mutable::update(state.continuejumps.clone(), cons(labelStep.clone(), Mutable::access(state.continuejumps.clone())));
    Mutable::update(state.breakjumps.clone(), cons(labelNext.clone(), Mutable::access(state.breakjumps.clone())));
    varCondition = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state.clone());
    let () = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ DAE::Exp::RANGE { ty: _, start, step, stop } => {
            let mut varFirst: MidCode::Var;
            let mut varIter: MidCode::Var;
            let mut varLast: MidCode::Var;
            let mut varStep: MidCode::Var;
            let mut labelCondition2: i32 = 0;
            let mut rvalueStep: MidCode::RValue;
            labelCondition2 = GenBlockId();
            varFirst = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
            varIter = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
            varLast = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
            varStep = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varFirst.clone(), src: ExpToMid(start.clone(), state.clone())? }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varIter.clone(), src: ExpToMid(start.clone(), state.clone())? }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varLast.clone(), src: ExpToMid(stop.clone(), state.clone())? }, state.clone());
            rvalueStep = (::match_deref::match_deref! { match &(step.clone()) {
        None => {
            MidCode::RValue::LITERALINTEGER { value: 1 }
        },
        Some(stepexp) => {
            ExpToMid(stepexp.clone(), state.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varStep.clone(), src: rvalueStep.clone() }, state.clone());
            stateTerminate(labelCondition.clone(), MidCode::Terminator::GOTO { next: labelCondition.clone() }, state.clone());
            stateTerminate(labelCondition2.clone(), MidCode::Terminator::CALL { func: Arc::new(Absyn::Path::IDENT { name: (literal!("in_range_integer")).clone() }), builtin: true, inputs: list![varIter.clone(), varFirst.clone(), varLast.clone()], outputs: list![MidCode::OutVar::OUT_VAR { var: varCondition.clone() }], next: labelCondition2.clone() }, state.clone());
            stateTerminate(labelBody.clone(), MidCode::Terminator::BRANCH { condition: varCondition.clone(), onTrue: labelBody.clone(), onFalse: labelNext.clone() }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varCref.clone(), src: MidCode::RValue::VARIABLE { src: varIter.clone() } }, state.clone());
            StmtsToMid(daestmtLst.clone(), state.clone())?;
            stateTerminate(labelStep.clone(), MidCode::Terminator::GOTO { next: labelStep.clone() }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varIter.clone(), src: MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::ADD, lsrc: varIter.clone(), rsrc: varStep.clone() } }, state.clone());
            stateTerminate(labelNext.clone(), MidCode::Terminator::GOTO { next: labelCondition.clone() }, state.clone());
            ()
        },
        _ => {
            let mut varRange: MidCode::Var;
            let mut varIter: MidCode::Var;
            let mut varLast: MidCode::Var;
            let mut varStep: MidCode::Var;
            let mut labelBody2: i32 = 0;
            varRange = RValueToVar(ExpToMid(range.clone(), state.clone())?, state.clone())?;
            let () = (::match_deref::match_deref! { match &(varRange.ty.clone()) {
        Deref @ DAE::Type::T_METATYPE { ty: _ } => {
            Error::addInternalError((literal!("metatype error")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        Deref @ DAE::Type::T_METAARRAY { ty: _ } => {
            labelBody2 = GenBlockId();
            varIter = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
            varLast = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
            varStep = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varIter.clone(), src: MidCode::RValue::LITERALINTEGER { value: 1 } }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varStep.clone(), src: MidCode::RValue::LITERALINTEGER { value: 1 } }, state.clone());
            stateTerminate(labelCondition.clone(), MidCode::Terminator::CALL { func: Arc::new(Absyn::Path::IDENT { name: (literal!("arrayLength")).clone() }), builtin: true, inputs: list![varRange.clone()], outputs: list![MidCode::OutVar::OUT_VAR { var: varLast.clone() }], next: labelCondition.clone() }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varCondition.clone(), src: MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::LESSEQ, lsrc: varIter.clone(), rsrc: varLast.clone() } }, state.clone());
            stateTerminate(labelBody.clone(), MidCode::Terminator::BRANCH { condition: varCondition.clone(), onTrue: labelBody.clone(), onFalse: labelNext.clone() }, state.clone());
            stateTerminate(labelBody2.clone(), MidCode::Terminator::CALL { func: Arc::new(Absyn::Path::IDENT { name: (literal!("arrayGet")).clone() }), builtin: true, inputs: list![varRange.clone(), varIter.clone()], outputs: list![MidCode::OutVar::OUT_VAR { var: varCref.clone() }], next: labelBody2.clone() }, state.clone());
            StmtsToMid(daestmtLst.clone(), state.clone())?;
            stateTerminate(labelStep.clone(), MidCode::Terminator::GOTO { next: labelStep.clone() }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varIter.clone(), src: MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::ADD, lsrc: varIter.clone(), rsrc: varStep.clone() } }, state.clone());
            stateTerminate(labelNext.clone(), MidCode::Terminator::GOTO { next: labelCondition.clone() }, state.clone());
            ()
        },
        Deref @ DAE::Type::T_METALIST { ty: _ } => {
            labelBody2 = GenBlockId();
            varIter = varRange.clone();
            stateTerminate(labelCondition.clone(), MidCode::Terminator::GOTO { next: labelCondition.clone() }, state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: varCondition.clone(), src: MidCode::RValue::ISCONS { src: varIter.clone() } }, state.clone());
            stateTerminate(labelBody.clone(), MidCode::Terminator::BRANCH { condition: varCondition.clone(), onTrue: labelBody.clone(), onFalse: labelNext.clone() }, state.clone());
            stateTerminate(labelBody2.clone(), MidCode::Terminator::CALL { func: Arc::new(Absyn::Path::IDENT { name: (literal!("listHead")).clone() }), builtin: true, inputs: list![varIter.clone()], outputs: list![MidCode::OutVar::OUT_VAR { var: varCref.clone() }], next: labelBody2.clone() }, state.clone());
            StmtsToMid(daestmtLst.clone(), state.clone())?;
            stateTerminate(labelStep.clone(), MidCode::Terminator::GOTO { next: labelStep.clone() }, state.clone());
            stateTerminate(labelNext.clone(), MidCode::Terminator::CALL { func: Arc::new(Absyn::Path::IDENT { name: (literal!("listRest")).clone() }), builtin: true, inputs: list![varIter.clone()], outputs: list![MidCode::OutVar::OUT_VAR { var: varIter.clone() }], next: labelCondition.clone() }, state.clone());
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("unknown for type ")); __mm_s.push_str(&*DAEDump::daeTypeStr(varRange.ty.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Mutable::update(state.continuejumps.clone(), listRest(Mutable::access(state.continuejumps.clone()))?);
    Mutable::update(state.breakjumps.clone(), listRest(Mutable::access(state.breakjumps.clone()))?);
    Ok(())
}

fn IfToMid(mut exp: Arc<DAE::Exp>, mut daestmtLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut else_: Arc<DAE::Else>, mut state: State) -> Result<()> {
    let mut labelBody: i32 = 0;
    let mut labelElse: i32 = 0;
    let mut labelNext: i32 = 0;
    let mut var1: MidCode::Var;
    let mut block_: MidCode::Block;
    labelBody = GenBlockId();
    labelElse = GenBlockId();
    labelNext = GenBlockId();
    var1 = RValueToVar(ExpToMid(exp.clone(), state.clone())?, state.clone())?;
    stateTerminate(labelBody.clone(), MidCode::Terminator::BRANCH { condition: var1.clone(), onTrue: labelBody.clone(), onFalse: labelElse.clone() }, state.clone());
    StmtsToMid(daestmtLst.clone(), state.clone())?;
    stateTerminate(labelElse.clone(), MidCode::Terminator::GOTO { next: labelNext.clone() }, state.clone());
    let () = (::match_deref::match_deref! { match &(else_.clone()) {
        Deref @ DAE::Else::NOELSE => {
            ()
        },
        Deref @ DAE::Else::ELSEIF { exp: subexp, statementLst: subdaestmtLst, else_: subelse } => {
            IfToMid(subexp.clone(), subdaestmtLst.clone(), subelse.clone(), state.clone())?;
            ()
        },
        Deref @ DAE::Else::ELSE { statementLst: subdaestmtLst } => {
            StmtsToMid(subdaestmtLst.clone(), state.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    stateTerminate(labelNext.clone(), MidCode::Terminator::GOTO { next: labelNext.clone() }, state.clone());
    Ok(())
}

fn stateGetCurrentLabel(mut state: State) -> i32 {
    let mut label: i32 = 0;
    label = Mutable::access(state.blockid.clone());
    label
}

fn stateSetCurrentLabel(mut label: i32, mut state: State) -> () {
    Mutable::update(state.blockid.clone(), label.clone());
    ()
}

fn stateAddStmt(mut stmt: MidCode::Stmt, mut state: State) -> () {
    DoubleEnded::push_back(state.stmts.clone(), stmt.clone());
    ()
}

fn stateTerminate(mut newLabel: i32, mut terminator: MidCode::Terminator, mut state: State) -> () {
    let mut block_: MidCode::Block;
    block_ = MidCode::Block { id: stateGetCurrentLabel(state.clone()), stmts: DoubleEnded::toListAndClear(state.stmts.clone(), metamodelica::nil()), terminator: terminator.clone() };
    DoubleEnded::push_back(state.blocks.clone(), block_.clone());
    stateSetCurrentLabel(newLabel.clone(), state.clone());
    ()
}

// helper
fn stateAddBailOnFalse(mut var: MidCode::Var, mut labelBail: i32, mut state: State) -> () {
    let mut labelTmp: i32 = 0;
    labelTmp = GenBlockId();
    stateTerminate(labelTmp.clone(), MidCode::Terminator::BRANCH { condition: var.clone(), onTrue: labelTmp.clone(), onFalse: labelBail.clone() }, state.clone());
    ()
}

fn unpackCrefFromExp(mut exp: Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> {
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cref = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => cref.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

//TODO: stuff needs to be volatile for setjmp.
//TODO: could handle match separately from matchcontinue and add more simplifications
/*
The term matchexpression is used to include both matchcontinue and match.
*/
fn MatchExpressionToMid(mut matchexpression: Arc<DAE::Exp>, mut outvars: Arc<metamodelica::List<MidCode::OutVar>>, mut state: State) -> Result<()> {
    let mut labelFin: i32 = 0;
    let mut labelMux: i32 = 0;
    let mut labelInit: i32 = 0;
    let mut labelFail: i32 = 0;
    let mut labelFin2: i32 = 0;
    let mut labelOut: i32 = 0;
    let mut caseLabel: i32 = 0;
    let mut caseLabels: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut muxState: MidCode::Var;
    let mut one: MidCode::Var;
    let mut midvar: MidCode::Var;
    let mut midvar2: MidCode::Var;
    let mut muxOldBuf: MidCode::VarBufPtr;
    let mut muxNewBuf: MidCode::VarBuf;
    let mut outvar: MidCode::OutVar = MidCode::OutVar::OUT_WILD;
    let mut matchContinue: bool = false;
    let mut matchType: DAE::MatchType = DAE::MatchType::MATCHCONTINUE;
    let mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    let mut inputsCref: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut aliases: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut srcVar: MidCode::Var;
    let mut aliasVar: MidCode::Var;
    let mut aliasList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut inputsMidVar: Arc<metamodelica::List<MidCode::Var>> = metamodelica::nil();
    let mut daeExp: Arc<DAE::Exp>;
    let mut caseLabelIterator: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(matchexpression.clone()) {
        Deref @ DAE::Exp::MATCHEXPRESSION { aliases, inputs: inputsCref, cases, matchType, .. } => {
            labelInit = stateGetCurrentLabel(state.clone());
            labelMux = GenBlockId();
            labelFin = GenBlockId();
            matchContinue = (match matchType.clone() {
        DAE::MatchType::MATCHCONTINUE => true,
        DAE::MatchType::MATCH { .. } => false,
        _ => bail!("match: no arm matched"),
    });
            caseLabels = metamodelica::nil();
            for mut i in 1..=(cases.clone().len() as i32) {
                caseLabels = cons(GenBlockId(), caseLabels.clone());
            }
            assert!((inputsCref.clone().len() as i32) == (aliases.clone().len() as i32), "{}", &*(literal!("MatchExpressionToMid: incorrect input: listLength(inputs) != listLength(aliases)")).clone());
            inputsMidVar = metamodelica::nil();
            for mut daeExp_aliasList in &*List::zip(inputsCref.clone(), aliases.clone()) {
                let mut daeExp_aliasList = daeExp_aliasList.clone();
                (daeExp, aliasList) = daeExp_aliasList.clone();
                srcVar = RValueToVar(ExpToMid(daeExp.clone(), state.clone())?, state.clone())?;
                ty = RValueType(MidCode::RValue::VARIABLE { src: srcVar.clone() })?;
                inputsMidVar = cons(srcVar.clone(), inputsMidVar.clone());
                for mut alias in &*aliasList.clone() {
                    let mut alias = alias.clone();
                    aliasVar = MidCode::Var { volatile: false, ty: ty.clone(), name: (alias.clone()).clone() };
                    DoubleEnded::push_back(state.locals.clone(), aliasVar.clone());
                    stateAddStmt(MidCode::Stmt::ASSIGN { dest: aliasVar.clone(), src: MidCode::RValue::VARIABLE { src: srcVar.clone() } }, state.clone());
                }
            }
            muxState = GenTmpVarVolatile(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: muxState.clone(), src: MidCode::RValue::LITERALINTEGER { value: 0 } }, state.clone());
            if matchContinue.clone() {
                muxOldBuf = GenTmpVarBufPtr(state.clone());
                muxNewBuf = GenTmpVarBuf(state.clone());
                stateTerminate(labelMux.clone(), MidCode::Terminator::PUSHJMP { old_buf: muxOldBuf.clone(), new_buf: muxNewBuf.clone(), next: labelMux.clone() }, state.clone());
            } else {
                stateTerminate(labelMux.clone(), MidCode::Terminator::GOTO { next: labelMux.clone() }, state.clone());
            }
            if matchContinue.clone() {
                one = GenTmpVar(DAE::T_INTEGER_DEFAULT().clone(), state.clone());
                stateAddStmt(MidCode::Stmt::ASSIGN { dest: one.clone(), src: MidCode::RValue::LITERALINTEGER { value: 1 } }, state.clone());
                stateAddStmt(MidCode::Stmt::ASSIGN { dest: muxState.clone(), src: MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::ADD, lsrc: muxState.clone(), rsrc: one.clone() } }, state.clone());
                stateTerminate(labelFin.clone(), MidCode::Terminator::SWITCH { condition: muxState.clone(), cases: List::zip(List::intRange((cases.clone().len() as i32) + 1), listAppend(caseLabels.clone(), list![labelFin.clone()])) }, state.clone());
            } else {
                stateTerminate(labelFin.clone(), MidCode::Terminator::GOTO { next: if (!(caseLabels.clone().is_empty())) {listHead(caseLabels.clone())?} else {labelFin.clone()} }, state.clone());
            }
            labelFail = GenBlockId();
            labelFin2 = GenBlockId();
            labelOut = GenBlockId();
            if matchContinue.clone() {
                stateTerminate(labelFin2.clone(), MidCode::Terminator::POPJMP { old_buf: muxOldBuf.clone(), next: labelFin2.clone() }, state.clone());
            } else {
                stateTerminate(labelFin2.clone(), MidCode::Terminator::GOTO { next: labelFin2.clone() }, state.clone());
            }
            midvar = RValueToVar(MidCode::RValue::LITERALINTEGER { value: (cases.clone().len() as i32) + 1 }, state.clone())?;
            midvar2 = RValueToVar(MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::EQUAL, lsrc: muxState.clone(), rsrc: midvar.clone() }, state.clone())?;
            stateTerminate(labelFail.clone(), MidCode::Terminator::BRANCH { condition: midvar2.clone(), onTrue: labelFail.clone(), onFalse: labelOut.clone() }, state.clone());
            stateTerminate(labelOut.clone(), crate::MidCode::Terminator::LONGJMP, state.clone());
            caseLabelIterator = caseLabels.clone();
            while !(caseLabelIterator.clone().is_empty()) {
                caseLabel = listHead(caseLabelIterator.clone())?;
                caseLabelIterator = listRest(caseLabelIterator.clone())?;
                stateSetCurrentLabel(caseLabel.clone(), state.clone());
                let () = (::match_deref::match_deref! { match &(cases.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { result: caseResult, patternGuard, body: daeBody, patterns, .. }, tail: cases } => {
            if matchContinue.clone() {
                patternToMidCode(List::zip(inputsMidVar.clone(), patterns.clone()), labelMux.clone(), state.clone())?;
            } else {
                patternToMidCode(List::zip(inputsMidVar.clone(), patterns.clone()), if (!(caseLabelIterator.clone().is_empty())) {listHead(caseLabelIterator.clone())?} else {labelFail.clone()}, state.clone())?;
            }
            let () = (::match_deref::match_deref! { match &(patternGuard.clone()) {
        None => (),
        Some(daeExp) => {
            midvar = RValueToVar(ExpToMid(daeExp.clone(), state.clone())?, state.clone())?;
            if matchContinue.clone() {
                stateAddBailOnFalse(midvar.clone(), labelMux.clone(), state.clone());
            } else {
                stateAddBailOnFalse(midvar.clone(), if (!(caseLabelIterator.clone().is_empty())) {listHead(caseLabelIterator.clone())?} else {labelFail.clone()}, state.clone());
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            StmtsToMid(daeBody.clone(), state.clone())?;
            let () = (::match_deref::match_deref! { match &((caseResult.clone(), outvars.clone())) {
        (Some(Deref @ DAE::Exp::TUPLE { PR: expList }), _) => {
            for mut outvarDaeExp in &*listZip(outvars.clone(), expList.clone())? {
                let mut outvarDaeExp = outvarDaeExp.clone();
                (outvar, daeExp) = outvarDaeExp.clone();
                let () = (match outvar.clone() {
        MidCode::OutVar::OUT_VAR { var: mut var } => {
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: var.clone(), src: ExpToMid(daeExp.clone(), state.clone())? }, state.clone());
            ()
        },
        MidCode::OutVar::OUT_WILD => {
            ()
        },
        _ => bail!("match: no arm matched"),
    });
            }
            ()
        },
        (Some(daeExp @ Deref @ DAE::Exp::CALL { path: _, .. }), _) => {
            CallToMid(daeExp.clone(), outvars.clone(), state.clone())?;
            ()
        },
        (Some(daeExp @ Deref @ DAE::Exp::MATCHEXPRESSION { matchType: _, .. }), _) => {
            MatchExpressionToMid(daeExp.clone(), outvars.clone(), state.clone())?;
            ()
        },
        (Some(daeExp), Deref @ metamodelica::List::Cons { head: MidCode::OutVar::OUT_VAR { var: midvar }, tail: Deref @ metamodelica::List::Nil }) => {
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: midvar.clone(), src: ExpToMid(daeExp.clone(), state.clone())? }, state.clone());
            ()
        },
        (Some(daeExp), _) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Match expression output to Mid conversion failed:\n")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(daeExp.clone(), 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            ()
        },
        (None, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (None, _) => {
            Error::addInternalError((literal!("case fail")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
            stateTerminate(labelOut.clone(), MidCode::Terminator::GOTO { next: labelFin.clone() }, state.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn patternToMidCode(mut matches: Arc<metamodelica::List<(MidCode::Var, Arc<DAE::Pattern>)>>, mut labelNoMatch: i32, mut state: State) -> Result<metamodelica::Array<Arc<metamodelica::List<MidCode::Stmt>>>> {
    let mut assignBlock: metamodelica::Array<Arc<metamodelica::List<MidCode::Stmt>>>;
    assignBlock = arrayCreate(1, metamodelica::nil());
    patternToMidCode2(state.clone(), matches.clone(), labelNoMatch.clone(), assignBlock.clone())?;
    let __range0 = &*assignBlock.clone().borrow()[(1-1) as usize].clone().reverse();
    for mut stmt in __range0 {
        let mut stmt = stmt.clone();
        stateAddStmt(stmt.clone(), state.clone());
    }
    Ok(assignBlock)
}

fn patternToMidCode2(mut state: State, mut matches: Arc<metamodelica::List<(MidCode::Var, Arc<DAE::Pattern>)>>, mut labelNoMatch: i32, mut assignBlock: metamodelica::Array<Arc<metamodelica::List<MidCode::Stmt>>>) -> Result<()> {
    let mut name: Arc<Absyn::Path>;
    let mut index: i32 = 0;
    let mut morePatterns: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
    let mut iterator: Arc<metamodelica::List<Arc<DAE::Pattern>>> = metamodelica::nil();
    let mut fields: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    let mut typeVars: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut knownSingleton: bool = false;
    let mut fieldNr: i32 = 0;
    let () = (::match_deref::match_deref! { match &(matches.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: (_, Deref @ DAE::Pattern::PAT_WILD), tail: restMatches } => {
            patternToMidCode2(state.clone(), restMatches.clone(), labelNoMatch.clone(), assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_AS { pat: pattern, ty: None, id, .. }), tail: restMatches } => {
            let mut midvar: MidCode::Var;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            ty = RValueType(MidCode::RValue::VARIABLE { src: scrutinee.clone() })?;
            midvar = MidCode::Var { name: (id.clone()).clone(), ty: ty.clone(), volatile: false };
            {let _arr = assignBlock.clone(); let _val = cons(MidCode::Stmt::ASSIGN { dest: midvar.clone(), src: MidCode::RValue::VARIABLE { src: scrutinee.clone() } }, assignBlock.clone().borrow()[(1-1) as usize].clone()); _arr.borrow_mut()[(1-1) as usize] = _val; _arr};
            patternToMidCode2(state.clone(), cons((scrutinee.clone(), pattern.clone()), restMatches.clone()), labelNoMatch.clone(), assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_AS { pat: pattern, ty: Some(ty), id, .. }), tail: restMatches } => {
            let mut midvar: MidCode::Var;
            midvar = MidCode::Var { name: (id.clone()).clone(), ty: ty.clone(), volatile: false };
            {let _arr = assignBlock.clone(); let _val = cons(MidCode::Stmt::ASSIGN { dest: midvar.clone(), src: MidCode::RValue::UNARYOP { op: crate::MidCode::UnaryOp::UNBOX, src: scrutinee.clone() } }, assignBlock.clone().borrow()[(1-1) as usize].clone()); _arr.borrow_mut()[(1-1) as usize] = _val; _arr};
            patternToMidCode2(state.clone(), cons((scrutinee.clone(), pattern.clone()), restMatches.clone()), labelNoMatch.clone(), assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_CONSTANT { exp, ty: optType }), tail: restMatches } => {
            let mut ok: MidCode::Var;
            let mut scrutineeCompareVar: MidCode::Var;
            let mut patCompareVar: MidCode::Var;
            let mut bool: bool = false;
            let mut integer: i32 = 0;
            let mut real: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut string: ArcStr = arcstr::literal!("");
            let mut scrutinee = (*scrutinee).clone();
            let mut exp = (*exp).clone();
            exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::SHARED_LITERAL { exp, .. } => exp.clone(),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            scrutinee = (::match_deref::match_deref! { match &(optType.clone()) {
        None => scrutinee.clone(),
        Some(_) => RValueToVar(MidCode::RValue::UNARYOP { op: crate::MidCode::UnaryOp::UNBOX, src: scrutinee.clone() }, state.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::BCONST { bool } => {
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: bool.clone() }, state.clone())?;
            ()
        },
        Deref @ DAE::Exp::ICONST { integer } => {
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALINTEGER { value: integer.clone() }, state.clone())?;
            ()
        },
        Deref @ DAE::Exp::RCONST { real } => {
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALREAL { value: real.clone() }, state.clone())?;
            ()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { index: integer, .. } => {
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALINTEGER { value: integer.clone() }, state.clone())?;
            ()
        },
        Deref @ DAE::Exp::LIST { valList: Deref @ metamodelica::List::Nil } => {
            scrutineeCompareVar = RValueToVar(MidCode::RValue::ISCONS { src: scrutinee.clone() }, state.clone())?;
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: false }, state.clone())?;
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            scrutineeCompareVar = RValueToVar(MidCode::RValue::ISSOME { src: scrutinee.clone() }, state.clone())?;
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: false }, state.clone())?;
            ()
        },
        Deref @ DAE::Exp::SCONST { string } => {
            scrutineeCompareVar = scrutinee.clone();
            patCompareVar = RValueToVar(MidCode::RValue::LITERALSTRING { value: (string.clone()).clone() }, state.clone())?;
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE.Exp to Mid conversion failed for pattern constant. Exp:")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(exp.clone(), 0)?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ok = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: ok.clone(), src: MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::EQUAL, lsrc: scrutineeCompareVar.clone(), rsrc: patCompareVar.clone() } }, state.clone());
            stateAddBailOnFalse(ok.clone(), labelNoMatch.clone(), state.clone());
            patternToMidCode2(state.clone(), restMatches.clone(), labelNoMatch.clone(), assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: morePatterns }), tail: restMatches } => {
            let mut moreMatches: Arc<metamodelica::List<(MidCode::Var, Arc<DAE::Pattern>)>> = metamodelica::nil();
            let mut listTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut midvar: MidCode::Var;
            listTypes = (::match_deref::match_deref! { match &(scrutinee.ty.clone()) {
        Deref @ DAE::Type::T_METATUPLE { types: listTypes } => listTypes.clone(),
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Wrong type of midvar in tuple pattern: ")); __mm_s.push_str(&*DAEDump::daeTypeStr(scrutinee.ty.clone())?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            moreMatches = metamodelica::nil();
            iterator = morePatterns.clone();
            fieldNr = 0;
            while !(iterator.clone().is_empty()) {
                midvar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: fieldNr.clone(), ty: listHead(listTypes.clone())? }, state.clone())?;
                moreMatches = cons((midvar.clone(), listHead(iterator.clone())?), moreMatches.clone());
                fieldNr = fieldNr.clone() + 1;
                iterator = listRest(iterator.clone())?;
                listTypes = listRest(listTypes.clone())?;
            }
            moreMatches = moreMatches.clone().reverse();
            patternToMidCode2(state.clone(), listAppend(moreMatches.clone(), restMatches.clone()), labelNoMatch.clone(), assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_SOME { pat: pattern }), tail: restMatches } => {
            let mut ok: MidCode::Var;
            let mut midvar: MidCode::Var;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut scrutineeCompareVar: MidCode::Var;
            let mut patCompareVar: MidCode::Var;
            ok = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state.clone());
            scrutineeCompareVar = RValueToVar(MidCode::RValue::ISSOME { src: scrutinee.clone() }, state.clone())?;
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: true }, state.clone())?;
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: ok.clone(), src: MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::EQUAL, lsrc: scrutineeCompareVar.clone(), rsrc: patCompareVar.clone() } }, state.clone());
            stateAddBailOnFalse(ok.clone(), labelNoMatch.clone(), state.clone());
            ty = (::match_deref::match_deref! { match &(scrutinee.ty.clone()) {
        Deref @ DAE::Type::T_METAOPTION { ty } => ty.clone(),
        _ => {
            Error::addInternalError((literal!("Wrong type of midvar in option pattern.\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            midvar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: 0, ty: ty.clone() }, state.clone())?;
            patternToMidCode2(state.clone(), cons((midvar.clone(), pattern.clone()), restMatches.clone()), labelNoMatch.clone(), assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_CONS { tail: restPattern, head: headPattern }), tail: restMatches } => {
            let mut ok: MidCode::Var;
            let mut headVar: MidCode::Var;
            let mut restVar: MidCode::Var;
            let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut scrutineeCompareVar: MidCode::Var;
            let mut patCompareVar: MidCode::Var;
            scrutineeCompareVar = RValueToVar(MidCode::RValue::ISCONS { src: scrutinee.clone() }, state.clone())?;
            patCompareVar = RValueToVar(MidCode::RValue::LITERALBOOLEAN { value: true }, state.clone())?;
            ok = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state.clone());
            stateAddStmt(MidCode::Stmt::ASSIGN { dest: ok.clone(), src: MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::EQUAL, lsrc: scrutineeCompareVar.clone(), rsrc: patCompareVar.clone() } }, state.clone());
            stateAddBailOnFalse(ok.clone(), labelNoMatch.clone(), state.clone());
            ty = (::match_deref::match_deref! { match &(scrutinee.ty.clone()) {
        Deref @ DAE::Type::T_METALIST { ty: Deref @ DAE::Type::T_UNKNOWN } => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Found list of unknown in cons pattern: ")); __mm_s.push_str(&*DAEDump::daeTypeStr(scrutinee.ty.clone())?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        Deref @ DAE::Type::T_METALIST { ty } => ty.clone(),
        _ => {
            Error::addInternalError((literal!("Wrong type of midvar in option pattern.\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            headVar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: 0, ty: ty.clone() }, state.clone())?;
            restVar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: 1, ty: scrutinee.ty.clone() }, state.clone())?;
            patternToMidCode2(state.clone(), cons((headVar.clone(), headPattern.clone()), cons((restVar.clone(), restPattern.clone()), restMatches.clone())), labelNoMatch.clone(), assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (scrutinee, Deref @ DAE::Pattern::PAT_CALL { name, index, patterns: morePatterns, fields, typeVars, knownSingleton }), tail: restMatches } => {
            let mut moreMatches: Arc<metamodelica::List<(MidCode::Var, Arc<DAE::Pattern>)>> = metamodelica::nil();
            let mut listTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut ok: MidCode::Var;
            let mut midvar: MidCode::Var;
            let mut scrutineeCompareVar: MidCode::Var;
            let mut patCompareVar: MidCode::Var;
            if !(knownSingleton.clone()) {
                ok = GenTmpVar(DAE::T_BOOL_DEFAULT().clone(), state.clone());
                scrutineeCompareVar = RValueToVar(MidCode::RValue::UNIONTYPEVARIANT { src: scrutinee.clone() }, state.clone())?;
                patCompareVar = RValueToVar(MidCode::RValue::LITERALINTEGER { value: index.clone() }, state.clone())?;
                stateAddStmt(MidCode::Stmt::ASSIGN { dest: ok.clone(), src: MidCode::RValue::BINARYOP { op: crate::MidCode::BinaryOp::EQUAL, lsrc: scrutineeCompareVar.clone(), rsrc: patCompareVar.clone() } }, state.clone());
                stateAddBailOnFalse(ok.clone(), labelNoMatch.clone(), state.clone());
            }
            listTypes = {
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut v in (fields.clone()).into_iter().cloned() {
            let __x = v.ty.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            moreMatches = metamodelica::nil();
            iterator = morePatterns.clone();
            fieldNr = 1;
            while !(iterator.clone().is_empty()) {
                midvar = RValueToVar(MidCode::RValue::METAFIELD { src: scrutinee.clone(), index: fieldNr.clone(), ty: listHead(listTypes.clone())? }, state.clone())?;
                moreMatches = cons((midvar.clone(), listHead(iterator.clone())?), moreMatches.clone());
                fieldNr = fieldNr.clone() + 1;
                iterator = listRest(iterator.clone())?;
                listTypes = listRest(listTypes.clone())?;
            }
            moreMatches = moreMatches.clone().reverse();
            patternToMidCode2(state.clone(), listAppend(moreMatches.clone(), restMatches.clone()), labelNoMatch.clone(), assignBlock.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (_, Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { .. }), tail: _ } => {
            Error::addInternalError((literal!("DAE.Pattern to Mid conversion failed. Unimplemented pattern: PAT_AS_FUNC_PTR.\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        Deref @ metamodelica::List::Cons { head: (_, Deref @ DAE::Pattern::PAT_CALL_TUPLE { .. }), tail: _ } => {
            Error::addInternalError((literal!("DAE.Pattern to Mid conversion failed. Unimplemented pattern: PAT_CALL_TUPLE.\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        Deref @ metamodelica::List::Cons { head: (_, Deref @ DAE::Pattern::PAT_CALL_NAMED { .. }), tail: _ } => {
            Error::addInternalError((literal!("DAE.Pattern to Mid conversion failed. Unimplemented pattern: PAT_CALL_NAMED.\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => {
            Error::addInternalError((literal!("DAE.Pattern to Mid conversion failed\n")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

