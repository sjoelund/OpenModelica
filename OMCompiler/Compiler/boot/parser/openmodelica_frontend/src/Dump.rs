// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::AbsynDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::File::Escape;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Print;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub struct DumpOptions {
    pub fileName: ArcStr,
}

pub type DUMPOPTIONS = DumpOptions;


pub fn boolUnparseFileFromInfo(info: SourceInfo, options: DumpOptions) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((options.clone(), info.clone())) {
        (DUMPOPTIONS { fileName: Deref @ "" }, _) => true,
        (DUMPOPTIONS { .. }, SourceInfo { .. }) => options.fileName.clone() == info.fileName.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

pub static defaultDumpOptions: DumpOptions = DumpOptions { fileName: literal!("") };

pub fn directionSymbol(inDirection: Absyn::Direction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inDirection.clone() {
        Absyn::BIDIR => literal!(""),
        Absyn::INPUT => literal!("input"),
        Absyn::OUTPUT => literal!("output"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn equationName(eq: Arc<Absyn::Equation>) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = ((::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EQ_IF { .. } => literal!("if"),
        Deref @ Absyn::EQ_EQUALS { .. } => literal!("equals"),
        Deref @ Absyn::EQ_PDE { .. } => literal!("pde"),
        Deref @ Absyn::EQ_CONNECT { .. } => literal!("connect"),
        Deref @ Absyn::EQ_WHEN_E { .. } => literal!("when"),
        Deref @ Absyn::EQ_NORETCALL { .. } => literal!("function call"),
        Deref @ Absyn::EQ_FAILURE { .. } => literal!("failure"),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(name)
}

pub fn expPriority(inExp: Arc<Absyn::Exp>, inLhs: bool) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (::match_deref::match_deref! { match &((inExp.clone(), inLhs.clone())) {
        (Deref @ Absyn::BINARY { op, .. }, false) => priorityBinopRhs(op.clone())?,
        (Deref @ Absyn::BINARY { op, .. }, true) => priorityBinopLhs(op.clone())?,
        (Deref @ Absyn::UNARY { .. }, _) => 4,
        (Deref @ Absyn::LBINARY { op, .. }, _) => priorityLBinop(op.clone())?,
        (Deref @ Absyn::LUNARY { .. }, _) => 7,
        (Deref @ Absyn::RELATION { .. }, _) => 6,
        (Deref @ Absyn::RANGE { .. }, _) => 10,
        (Deref @ Absyn::IFEXP { .. }, _) => 11,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPriority)
}

pub fn getAstAsCorbaString(program: Absyn::Program) -> Result<()> {
    let _ = (match program.clone() {
        Absyn::PROGRAM { within_: mut within_, classes: mut classes } => {
            Print::printBuf((literal!("record Absyn.PROGRAM\\nclasses = ")).clone())?;
            printListAsCorbaString(classes.clone(), Arc::new(printClassAsCorbaString), (literal!(",\\n")).clone())?;
            Print::printBuf((literal!(",\\nwithin_ = ")).clone())?;
            printWithinAsCorbaString(within_.clone())?;
            Print::printBuf((literal!("\\nend Absyn.PROGRAM;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn isAssociativeExp(inExp: Arc<Absyn::Exp>) -> bool {
    let mut outIsAssociative: bool;
    outIsAssociative = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::BINARY { op, .. } => isAssociativeOp(op.clone()),
        Deref @ Absyn::LBINARY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsAssociative
}

fn isAssociativeOp(inOperator: Absyn::Operator) -> bool {
    let mut outIsAssociative: bool;
    outIsAssociative = (match inOperator.clone() {
        Absyn::ADD => true,
        Absyn::ADD_EW => true,
        Absyn::MUL_EW => true,
        _ => false,
    });
    outIsAssociative
}

fn isNonAssociativeExp(exp: Arc<Absyn::Exp>) -> bool {
    let mut isNonAssociative: bool;
    isNonAssociative = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::BINARY { .. } => isNonAssociativeOp(var_field!((*exp).op, Absyn::Exp::BINARY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNonAssociative
}

fn isNonAssociativeOp(operator: Absyn::Operator) -> bool {
    let mut isNonAssociative: bool;
    isNonAssociative = (match operator.clone() {
        Absyn::POW => true,
        Absyn::POW_EW => true,
        _ => false,
    });
    isNonAssociative
}

pub fn opSymbol(inOperator: Absyn::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inOperator.clone() {
        Absyn::ADD => literal!(" + "),
        Absyn::SUB => literal!(" - "),
        Absyn::MUL => literal!(" * "),
        Absyn::DIV => literal!(" / "),
        Absyn::POW => literal!(" ^ "),
        Absyn::UMINUS => literal!("-"),
        Absyn::UPLUS => literal!("+"),
        Absyn::ADD_EW => literal!(" .+ "),
        Absyn::SUB_EW => literal!(" .- "),
        Absyn::MUL_EW => literal!(" .* "),
        Absyn::DIV_EW => literal!(" ./ "),
        Absyn::POW_EW => literal!(" .^ "),
        Absyn::UMINUS_EW => literal!(" .-"),
        Absyn::UPLUS_EW => literal!(" .+"),
        Absyn::AND => literal!(" and "),
        Absyn::OR => literal!(" or "),
        Absyn::NOT => literal!("not "),
        Absyn::LESS => literal!(" < "),
        Absyn::LESSEQ => literal!(" <= "),
        Absyn::GREATER => literal!(" > "),
        Absyn::GREATEREQ => literal!(" >= "),
        Absyn::EQUAL => literal!(" == "),
        Absyn::NEQUAL => literal!(" <> "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn opSymbolCompact(inOperator: Absyn::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inOperator.clone() {
        Absyn::ADD => literal!("+"),
        Absyn::SUB => literal!("-"),
        Absyn::MUL => literal!("*"),
        Absyn::DIV => literal!("/"),
        Absyn::POW => literal!("^"),
        Absyn::UMINUS => literal!("-"),
        Absyn::UPLUS => literal!("+"),
        Absyn::ADD_EW => literal!("+"),
        Absyn::SUB_EW => literal!("-"),
        Absyn::MUL_EW => literal!("*"),
        Absyn::DIV_EW => literal!("/"),
        Absyn::POW_EW => literal!("^"),
        Absyn::UMINUS_EW => literal!("-"),
        Absyn::AND => literal!("and"),
        Absyn::OR => literal!("or"),
        Absyn::NOT => literal!("not"),
        Absyn::LESS => literal!("<"),
        Absyn::LESSEQ => literal!("<="),
        Absyn::GREATER => literal!(">"),
        Absyn::GREATEREQ => literal!(">="),
        Absyn::EQUAL => literal!("=="),
        Absyn::NEQUAL => literal!("<>"),
        _ => bail!("fail"),
    })).clone();
    Ok(outString)
}

fn printAlgorithmAsCorbaString(alg: Arc<Absyn::Algorithm>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::ALG_ASSIGN { assignComponent, value } => {
            let mut ifExp: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_ASSIGN assignComponent = ")).clone())?;
            printExpAsCorbaString(assignComponent.clone())?;
            Print::printBuf((literal!(", value = ")).clone())?;
            printExpAsCorbaString(value.clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_ASSIGN;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_IF { ifExp, trueBranch, elseIfAlgorithmBranch, elseBranch } => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_IF ifExp = ")).clone())?;
            printExpAsCorbaString(ifExp.clone())?;
            Print::printBuf((literal!(", trueBranch = ")).clone())?;
            printListAsCorbaString(trueBranch.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseIfAlgorithmBranch = ")).clone())?;
            printListAsCorbaString(elseIfAlgorithmBranch.clone(), Arc::new(printAlgorithmBranchAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseBranch = ")).clone())?;
            printListAsCorbaString(elseBranch.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_IF;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_FOR { iterators, forBody } => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_FOR iterators = ")).clone())?;
            printListAsCorbaString(iterators.clone(), Arc::new(printForIteratorAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", forBody = ")).clone())?;
            printListAsCorbaString(forBody.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_FOR;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_PARFOR { iterators, parforBody: forBody } => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_PARFOR iterators = ")).clone())?;
            printListAsCorbaString(iterators.clone(), Arc::new(printForIteratorAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", parforBody = ")).clone())?;
            printListAsCorbaString(forBody.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_PARFOR;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_WHILE { boolExpr, whileBody } => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_WHILE boolExpr = ")).clone())?;
            printExpAsCorbaString(boolExpr.clone())?;
            Print::printBuf((literal!(", whileBody = ")).clone())?;
            printListAsCorbaString(whileBody.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_WHILE;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_WHEN_A { boolExpr, whenBody, elseWhenAlgorithmBranch } => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_WHEN_A boolExpr = ")).clone())?;
            printExpAsCorbaString(boolExpr.clone())?;
            Print::printBuf((literal!(", whenBody = ")).clone())?;
            printListAsCorbaString(whenBody.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseWhenAlgorithmBranch = ")).clone())?;
            printListAsCorbaString(elseWhenAlgorithmBranch.clone(), Arc::new(printAlgorithmBranchAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_WHEN_A;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_NORETCALL { functionCall, functionArgs } => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            Print::printBuf((literal!("record Absyn.ALG_NORETCALL functionCall = ")).clone())?;
            printComponentRefAsCorbaString(functionCall.clone())?;
            Print::printBuf((literal!(", functionArgs = ")).clone())?;
            printFunctionArgsAsCorbaString(functionArgs.clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_NORETCALL;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_RETURN => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_RETURN end Absyn.ALG_RETURN;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_BREAK => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_BREAK end Absyn.ALG_BREAK;")).clone())?;
            ()
        },
        Deref @ Absyn::ALG_FAILURE { equ: body } => {
            let mut assignComponent: Arc<Absyn::Exp>;
            let mut value: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut boolExpr: Arc<Absyn::Exp>;
            let mut elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut tryBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut catchBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut functionCall: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.ALG_FAILURE body = ")).clone())?;
            printListAsCorbaString(body.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_FAILURE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printAlgorithmBranchAsCorbaString(inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)) -> Result<()> {
    printTupleAsCorbaString(inBranch.clone(), Arc::new(printExpAsCorbaString), Arc::new(printAlgorithmItemListAsCorbaString))?;
    Ok(())
}

fn printAlgorithmItemAsCorbaString(el: Arc<Absyn::AlgorithmItem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::ALGORITHMITEM { algorithm_, comment, info } => {
            let mut annotation_: Arc<Absyn::Annotation>;
            Print::printBuf((literal!("\\nrecord Absyn.ALGORITHMITEM algorithm_ = ")).clone())?;
            printAlgorithmAsCorbaString(algorithm_.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.ALGORITHMITEM;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printAlgorithmItemListAsCorbaString(inLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<()> {
    printListAsCorbaString(inLst.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
    Ok(())
}

fn printAnnotationAsCorbaString(annotation_: Arc<Absyn::Annotation>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(annotation_.clone()) {
        Deref @ Absyn::ANNOTATION { elementArgs } => {
            Print::printBuf((literal!("record Absyn.ANNOTATION elementArgs = ")).clone())?;
            printListAsCorbaString(elementArgs.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ANNOTATION;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printArrayDimAsCorbaString(arrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<()> {
    printListAsCorbaString(arrayDim.clone(), Arc::new(printSubscriptAsCorbaString), (literal!(",")).clone())?;
    Ok(())
}

pub fn printArraydimStr(s: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (printSubscriptsStr(s.clone())?).clone();
    Ok(r#str)
}

fn printCaseAsCorbaString(case_: Arc<Absyn::Case>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(case_.clone()) {
        Deref @ Absyn::CASE { pattern, patternGuard, patternInfo, localDecls, classPart, result, resultInfo, comment, info } => {
            Print::printBuf((literal!("record Absyn.CASE pattern = ")).clone())?;
            printExpAsCorbaString(pattern.clone())?;
            Print::printBuf((literal!(", patternGuard = ")).clone())?;
            printOption(patternGuard.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!(", patternInfo = ")).clone())?;
            printInfoAsCorbaString(patternInfo.clone())?;
            Print::printBuf((literal!(", localDecls = ")).clone())?;
            printListAsCorbaString(localDecls.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", classPart = ")).clone())?;
            printClassPartAsCorbaString(classPart.clone())?;
            Print::printBuf((literal!(", result = ")).clone())?;
            printExpAsCorbaString(result.clone())?;
            Print::printBuf((literal!(", resultInfo = ")).clone())?;
            printInfoAsCorbaString(resultInfo.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.CASE;")).clone())?;
            ()
        },
        Deref @ Absyn::ELSE { localDecls, classPart, result, resultInfo, comment, info } => {
            let mut pattern: Arc<Absyn::Exp>;
            let mut patternGuard: Option<Arc<Absyn::Exp>>;
            let mut patternInfo: SourceInfo;
            Print::printBuf((literal!("record Absyn.ELSE localDecls = ")).clone())?;
            printListAsCorbaString(localDecls.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", classPart = ")).clone())?;
            printClassPartAsCorbaString(classPart.clone())?;
            Print::printBuf((literal!(", result = ")).clone())?;
            printExpAsCorbaString(result.clone())?;
            Print::printBuf((literal!(", resultInfo = ")).clone())?;
            printInfoAsCorbaString(resultInfo.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.ELSE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printClassAsCorbaString(cl: Arc<Absyn::Class>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::CLASS { name, partialPrefix, finalPrefix, encapsulatedPrefix, restriction, body, commentsBeforeClass: _, commentsBeforeEnd: _, commentsAfterEnd: _, info } => {
            Print::printBuf((literal!("record Absyn.CLASS name = \\\"")).clone())?;
            Print::printBuf((name.clone()).clone())?;
            Print::printBuf((literal!("\\\", partialPrefix = ")).clone())?;
            Print::printBuf((boolString(partialPrefix.clone())).clone())?;
            Print::printBuf((literal!(", finalPrefix = ")).clone())?;
            Print::printBuf((boolString(finalPrefix.clone())).clone())?;
            Print::printBuf((literal!(", encapsulatedPrefix = ")).clone())?;
            Print::printBuf((boolString(encapsulatedPrefix.clone())).clone())?;
            Print::printBuf((literal!(", restriction = ")).clone())?;
            printRestrictionAsCorbaString(restriction.clone())?;
            Print::printBuf((literal!(", body = ")).clone())?;
            printClassDefAsCorbaString(body.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.CLASS;")).clone())?;
            ()
        },
        _ => {
            let mut name: ArcStr;
            let mut partialPrefix: bool;
            let mut finalPrefix: bool;
            let mut encapsulatedPrefix: bool;
            let mut restriction: Absyn::Restriction;
            let mut body: Arc<Absyn::ClassDef>;
            let mut info: SourceInfo;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printClassAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printClassDefAsCorbaString(classDef: Arc<Absyn::ClassDef>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ Absyn::PARTS { typeVars, classAttrs: _, classParts, ann, comment: optString } => {
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut attributes: Absyn::ElementAttributes;
            let mut arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut enumLiterals: Arc<Absyn::EnumDef>;
            let mut functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut baseClassName: ArcStr;
            let mut functionName: Arc<Absyn::Path>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            Print::printBuf((literal!("record Absyn.PARTS typeVars = {")).clone())?;
            Print::printBuf(stringDelimitList(typeVars.clone(), (literal!(",")).clone()))?;
            Print::printBuf((literal!("}, classParts = ")).clone())?;
            printListAsCorbaString(classParts.clone(), Arc::new(printClassPartAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", ann = ")).clone())?;
            printListAsCorbaString(ann.clone(), Arc::new(printAnnotationAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(optString.clone())?;
            Print::printBuf((literal!(" end Absyn.PARTS;")).clone())?;
            ()
        },
        Deref @ Absyn::DERIVED { typeSpec, attributes, arguments, comment } => {
            let mut classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut optString: Option<ArcStr>;
            let mut modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut enumLiterals: Arc<Absyn::EnumDef>;
            let mut functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut baseClassName: ArcStr;
            let mut functionName: Arc<Absyn::Path>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            Print::printBuf((literal!("record Absyn.DERIVED typeSpec = ")).clone())?;
            printTypeSpecAsCorbaString(typeSpec.clone())?;
            Print::printBuf((literal!(", attributes = ")).clone())?;
            printElementAttributesAsCorbaString(attributes.clone())?;
            Print::printBuf((literal!(", arguments = ")).clone())?;
            printListAsCorbaString(arguments.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.DERIVED;")).clone())?;
            ()
        },
        Deref @ Absyn::ENUMERATION { enumLiterals, comment } => {
            let mut classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut optString: Option<ArcStr>;
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut attributes: Absyn::ElementAttributes;
            let mut arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut baseClassName: ArcStr;
            let mut functionName: Arc<Absyn::Path>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            Print::printBuf((literal!("record Absyn.ENUMERATION enumLiterals = ")).clone())?;
            printEnumDefAsCorbaString(enumLiterals.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.ENUMERATION;")).clone())?;
            ()
        },
        Deref @ Absyn::OVERLOAD { functionNames, comment } => {
            let mut classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut optString: Option<ArcStr>;
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut attributes: Absyn::ElementAttributes;
            let mut arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut enumLiterals: Arc<Absyn::EnumDef>;
            let mut baseClassName: ArcStr;
            let mut functionName: Arc<Absyn::Path>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            Print::printBuf((literal!("record Absyn.OVERLOAD functionNames = ")).clone())?;
            printListAsCorbaString(functionNames.clone(), Arc::new(printPathAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.OVERLOAD;")).clone())?;
            ()
        },
        Deref @ Absyn::CLASS_EXTENDS { baseClassName, modifications, comment: optString, parts: classParts, ann } => {
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut attributes: Absyn::ElementAttributes;
            let mut arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut enumLiterals: Arc<Absyn::EnumDef>;
            let mut functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut functionName: Arc<Absyn::Path>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            Print::printBuf((literal!("record Absyn.CLASS_EXTENDS baseClassName = \\\"")).clone())?;
            Print::printBuf((baseClassName.clone()).clone())?;
            Print::printBuf((literal!("\\\", modifications = ")).clone())?;
            printListAsCorbaString(modifications.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(optString.clone())?;
            Print::printBuf((literal!(", parts = ")).clone())?;
            printListAsCorbaString(classParts.clone(), Arc::new(printClassPartAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", ann = ")).clone())?;
            printListAsCorbaString(ann.clone(), Arc::new(printAnnotationAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!("end Absyn.CLASS_EXTENDS;")).clone())?;
            ()
        },
        Deref @ Absyn::PDER { functionName, vars, comment } => {
            let mut classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut optString: Option<ArcStr>;
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut attributes: Absyn::ElementAttributes;
            let mut arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut enumLiterals: Arc<Absyn::EnumDef>;
            let mut functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut baseClassName: ArcStr;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            Print::printBuf((literal!("record Absyn.PDER functionName = ")).clone())?;
            printPathAsCorbaString(functionName.clone())?;
            Print::printBuf((literal!(", vars = ")).clone())?;
            printListAsCorbaString(vars.clone(), Arc::new(printStringAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.PDER;")).clone())?;
            ()
        },
        _ => {
            let mut classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut optString: Option<ArcStr>;
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut attributes: Absyn::ElementAttributes;
            let mut arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut enumLiterals: Arc<Absyn::EnumDef>;
            let mut functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut baseClassName: ArcStr;
            let mut functionName: Arc<Absyn::Path>;
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut vars: Arc<metamodelica::List<ArcStr>>;
            let mut classAttrs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printClassDefAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printClassPartAsCorbaString(classPart: Arc<Absyn::ClassPart>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(classPart.clone()) {
        Deref @ Absyn::PUBLIC { contents } => {
            let mut eqContents: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algContents: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut externalDecl: Arc<Absyn::ExternalDecl>;
            let mut annotation_: Option<Arc<Absyn::Annotation>>;
            Print::printBuf((literal!("\\nrecord Absyn.PUBLIC contents = ")).clone())?;
            printListAsCorbaString(contents.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.PUBLIC;")).clone())?;
            ()
        },
        Deref @ Absyn::PROTECTED { contents } => {
            let mut eqContents: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algContents: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut externalDecl: Arc<Absyn::ExternalDecl>;
            let mut annotation_: Option<Arc<Absyn::Annotation>>;
            Print::printBuf((literal!("\\nrecord Absyn.PROTECTED contents = ")).clone())?;
            printListAsCorbaString(contents.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.PROTECTED;")).clone())?;
            ()
        },
        Deref @ Absyn::EQUATIONS { contents: eqContents } => {
            let mut contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut algContents: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut externalDecl: Arc<Absyn::ExternalDecl>;
            let mut annotation_: Option<Arc<Absyn::Annotation>>;
            Print::printBuf((literal!("\\nrecord Absyn.EQUATIONS contents = ")).clone())?;
            printListAsCorbaString(eqContents.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.EQUATIONS;")).clone())?;
            ()
        },
        Deref @ Absyn::INITIALEQUATIONS { contents: eqContents } => {
            let mut contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut algContents: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut externalDecl: Arc<Absyn::ExternalDecl>;
            let mut annotation_: Option<Arc<Absyn::Annotation>>;
            Print::printBuf((literal!("\\nrecord Absyn.INITIALEQUATIONS contents = ")).clone())?;
            printListAsCorbaString(eqContents.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.INITIALEQUATIONS;")).clone())?;
            ()
        },
        Deref @ Absyn::ALGORITHMS { contents: algContents } => {
            let mut contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut eqContents: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut externalDecl: Arc<Absyn::ExternalDecl>;
            let mut annotation_: Option<Arc<Absyn::Annotation>>;
            Print::printBuf((literal!("\\nrecord Absyn.ALGORITHMS contents = ")).clone())?;
            printListAsCorbaString(algContents.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALGORITHMS;")).clone())?;
            ()
        },
        Deref @ Absyn::INITIALALGORITHMS { contents: algContents } => {
            let mut contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut eqContents: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut externalDecl: Arc<Absyn::ExternalDecl>;
            let mut annotation_: Option<Arc<Absyn::Annotation>>;
            Print::printBuf((literal!("\\nrecord Absyn.INITIALALGORITHMS contents = ")).clone())?;
            printListAsCorbaString(algContents.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.INITIALALGORITHMS;")).clone())?;
            ()
        },
        Deref @ Absyn::EXTERNAL { externalDecl, annotation_ } => {
            let mut contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut eqContents: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algContents: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            Print::printBuf((literal!("\\nrecord Absyn.EXTERNAL externalDecl = ")).clone())?;
            printExternalDeclAsCorbaString(externalDecl.clone())?;
            Print::printBuf((literal!(", annotation_ = ")).clone())?;
            printOption(annotation_.clone(), Arc::new(printAnnotationAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.EXTERNAL;")).clone())?;
            ()
        },
        _ => {
            let mut contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut eqContents: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algContents: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut externalDecl: Arc<Absyn::ExternalDecl>;
            let mut annotation_: Option<Arc<Absyn::Annotation>>;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printClassPartAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printCodeAsCorbaString(code: Arc<Absyn::CodeNode>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(code.clone()) {
        Deref @ Absyn::C_TYPENAME { path } => {
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut boolean: bool;
            let mut equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut element: Arc<Absyn::Element>;
            let mut exp: Arc<Absyn::Exp>;
            let mut modification: Arc<Absyn::Modification>;
            Print::printBuf((literal!("record Absyn.C_TYPENAME path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.C_TYPENAME;")).clone())?;
            ()
        },
        Deref @ Absyn::C_VARIABLENAME { componentRef } => {
            let mut path: Arc<Absyn::Path>;
            let mut boolean: bool;
            let mut equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut element: Arc<Absyn::Element>;
            let mut exp: Arc<Absyn::Exp>;
            let mut modification: Arc<Absyn::Modification>;
            Print::printBuf((literal!("record Absyn.C_VARIABLENAME componentRef = ")).clone())?;
            printComponentRefAsCorbaString(componentRef.clone())?;
            Print::printBuf((literal!(" end Absyn.C_VARIABLENAME;")).clone())?;
            ()
        },
        Deref @ Absyn::C_EQUATIONSECTION { boolean, equationItemLst } => {
            let mut path: Arc<Absyn::Path>;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut element: Arc<Absyn::Element>;
            let mut exp: Arc<Absyn::Exp>;
            let mut modification: Arc<Absyn::Modification>;
            Print::printBuf((literal!("record Absyn.C_EQUATIONSECTION boolean = ")).clone())?;
            Print::printBuf((boolString(boolean.clone())).clone())?;
            Print::printBuf((literal!(", equationItemLst = ")).clone())?;
            printListAsCorbaString(equationItemLst.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.C_EQUATIONSECTION;")).clone())?;
            ()
        },
        Deref @ Absyn::C_ALGORITHMSECTION { boolean, algorithmItemLst } => {
            let mut path: Arc<Absyn::Path>;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut element: Arc<Absyn::Element>;
            let mut exp: Arc<Absyn::Exp>;
            let mut modification: Arc<Absyn::Modification>;
            Print::printBuf((literal!("record Absyn.C_ALGORITHMSECTION boolean = ")).clone())?;
            Print::printBuf((boolString(boolean.clone())).clone())?;
            Print::printBuf((literal!(", algorithmItemLst = ")).clone())?;
            printListAsCorbaString(algorithmItemLst.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.C_ALGORITHMSECTION;")).clone())?;
            ()
        },
        Deref @ Absyn::C_ELEMENT { element } => {
            let mut path: Arc<Absyn::Path>;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut boolean: bool;
            let mut equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut exp: Arc<Absyn::Exp>;
            let mut modification: Arc<Absyn::Modification>;
            Print::printBuf((literal!("record Absyn.C_ELEMENT element = ")).clone())?;
            printElementAsCorbaString(element.clone())?;
            Print::printBuf((literal!(" end Absyn.C_ELEMENT;")).clone())?;
            ()
        },
        Deref @ Absyn::C_EXPRESSION { exp } => {
            let mut path: Arc<Absyn::Path>;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut boolean: bool;
            let mut equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut element: Arc<Absyn::Element>;
            let mut modification: Arc<Absyn::Modification>;
            Print::printBuf((literal!("record Absyn.C_EXPRESSION exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(" end Absyn.C_EXPRESSION;")).clone())?;
            ()
        },
        Deref @ Absyn::C_MODIFICATION { modification } => {
            let mut path: Arc<Absyn::Path>;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut boolean: bool;
            let mut equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut element: Arc<Absyn::Element>;
            let mut exp: Arc<Absyn::Exp>;
            Print::printBuf((literal!("record Absyn.C_MODIFICATION modification = ")).clone())?;
            printModificationAsCorbaString(modification.clone())?;
            Print::printBuf((literal!(" end Absyn.C_MODIFICATION;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printCodeStr(inCode: Arc<Absyn::CodeNode>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpCodeNode), inCode.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

fn printCommentAsCorbaString(inComment: Arc<Absyn::Comment>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inComment.clone()) {
        Deref @ Absyn::COMMENT { annotation_, comment } => {
            Print::printBuf((literal!("record Absyn.COMMENT annotation_ = ")).clone())?;
            printOption(annotation_.clone(), Arc::new(printAnnotationAsCorbaString))?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(" end Absyn.COMMENT;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printComponentAsCorbaString(component: Absyn::Component) -> Result<()> {
    let _ = (match component.clone() {
        Absyn::COMPONENT { name: mut name, arrayDim: mut arrayDim, modification: mut modification } => {
            Print::printBuf((literal!("record Absyn.COMPONENT name = \\\"")).clone())?;
            Print::printBuf((name.clone()).clone())?;
            Print::printBuf((literal!("\\\", arrayDim = ")).clone())?;
            printArrayDimAsCorbaString(arrayDim.clone())?;
            Print::printBuf((literal!(", modification = ")).clone())?;
            printOption(modification.clone(), Arc::new(printModificationAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.COMPONENT;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printComponentItemAsCorbaString(componentItem: Arc<Absyn::ComponentItem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(componentItem.clone()) {
        Deref @ Absyn::COMPONENTITEM { component, condition, comment } => {
            Print::printBuf((literal!("record Absyn.COMPONENTITEM component = ")).clone())?;
            printComponentAsCorbaString(component.clone())?;
            Print::printBuf((literal!(", condition = ")).clone())?;
            printOption(condition.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.COMPONENTITEM;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printComponentRefAsCorbaString(cref: Arc<Absyn::ComponentRef>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_QUAL { componentRef: p, subscripts, name: s } => {
            Print::printBuf((literal!("record Absyn.CREF_QUAL name = \\\"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\\\", subscripts = ")).clone())?;
            printListAsCorbaString(subscripts.clone(), Arc::new(printSubscriptAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", componentRef = ")).clone())?;
            printComponentRefAsCorbaString(p.clone())?;
            Print::printBuf((literal!(" end Absyn.CREF_QUAL;")).clone())?;
            ()
        },
        Deref @ Absyn::CREF_IDENT { subscripts, name: s } => {
            let mut p: Arc<Absyn::ComponentRef>;
            Print::printBuf((literal!("record Absyn.CREF_IDENT name = \\\"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\\\", subscripts = ")).clone())?;
            printListAsCorbaString(subscripts.clone(), Arc::new(printSubscriptAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.CREF_IDENT;")).clone())?;
            ()
        },
        Deref @ Absyn::ALLWILD => {
            let mut s: ArcStr;
            let mut p: Arc<Absyn::ComponentRef>;
            let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            Print::printBuf((literal!("record Absyn.ALLWILD end Absyn.ALLWILD;")).clone())?;
            ()
        },
        Deref @ Absyn::WILD => {
            let mut s: ArcStr;
            let mut p: Arc<Absyn::ComponentRef>;
            let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            Print::printBuf((literal!("record Absyn.WILD end Absyn.WILD;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printComponentRefStr(inComponentRef: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::CREF_IDENT { subscripts: subs, name: s } => {
            let mut subsstr: ArcStr;
            let mut s_1: ArcStr;
            let mut crs: ArcStr;
            let mut s_2: ArcStr;
            let mut s_3: ArcStr;
            let mut cr: Arc<Absyn::ComponentRef>;
            subsstr = (printSubscriptsStr(subs.clone())?).clone();
            s_1 = (stringAppend((s.clone()).clone(), (subsstr.clone()).clone())).clone();
            s_1.clone()
        },
        Deref @ Absyn::CREF_QUAL { componentRef: cr, subscripts: subs, name: s } => {
            let mut subsstr: ArcStr;
            let mut s_1: ArcStr;
            let mut crs: ArcStr;
            let mut s_2: ArcStr;
            let mut s_3: ArcStr;
            crs = (printComponentRefStr(cr.clone())?).clone();
            subsstr = (printSubscriptsStr(subs.clone())?).clone();
            s_1 = (stringAppend((s.clone()).clone(), (subsstr.clone()).clone())).clone();
            s_2 = (stringAppend((s_1.clone()).clone(), (literal!(".")).clone())).clone();
            s_3 = (stringAppend((s_2.clone()).clone(), (crs.clone()).clone())).clone();
            s_3.clone()
        },
        Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cr } => {
            let mut subsstr: ArcStr;
            let mut s_1: ArcStr;
            let mut s: ArcStr;
            let mut crs: ArcStr;
            let mut s_2: ArcStr;
            let mut s_3: ArcStr;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            crs = (printComponentRefStr(cr.clone())?).clone();
            s_3 = (stringAppend((literal!(".")).clone(), (crs.clone()).clone())).clone();
            s_3.clone()
        },
        Deref @ Absyn::ALLWILD => literal!("__"),
        Deref @ Absyn::WILD => if (Config::acceptMetaModelicaGrammar()?) {literal!("_")} else {literal!("")},
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn printConstrainClassAsCorbaString(constrainClass: Arc<Absyn::ConstrainClass>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(constrainClass.clone()) {
        Deref @ Absyn::CONSTRAINCLASS { elementSpec, comment } => {
            Print::printBuf((literal!("record Absyn.CONSTRAINCLASS elementSpec = ")).clone())?;
            printElementSpecAsCorbaString(elementSpec.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.CONSTRAINCLASS;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printDirectionAsCorbaString(dir: Absyn::Direction) -> Result<()> {
    let _ = (match dir.clone() {
        Absyn::INPUT => {
            Print::printBuf((literal!("record Absyn.INPUT end Absyn.INPUT;")).clone())?;
            ()
        },
        Absyn::OUTPUT => {
            Print::printBuf((literal!("record Absyn.OUTPUT end Absyn.OUTPUT;")).clone())?;
            ()
        },
        Absyn::BIDIR => {
            Print::printBuf((literal!("record Absyn.BIDIR end Absyn.BIDIR;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printEachAsCorbaString(each_: Absyn::Each) -> Result<()> {
    let _ = (match each_.clone() {
        Absyn::EACH => {
            Print::printBuf((literal!("record Absyn.EACH end Absyn.EACH;")).clone())?;
            ()
        },
        Absyn::NON_EACH => {
            Print::printBuf((literal!("record Absyn.NON_EACH end Absyn.NON_EACH;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printElementArgAsCorbaString(arg: Arc<Absyn::ElementArg>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::MODIFICATION { finalPrefix, eachPrefix, path: p, modification, comment, info } => {
            let mut redeclareKeywords: Absyn::RedeclareKeywords;
            let mut elementSpec: Arc<Absyn::ElementSpec>;
            let mut constrainClass: Option<Arc<Absyn::ConstrainClass>>;
            Print::printBuf((literal!("record Absyn.MODIFICATION finalPrefix = ")).clone())?;
            Print::printBuf((boolString(finalPrefix.clone())).clone())?;
            Print::printBuf((literal!(", eachPrefix = ")).clone())?;
            printEachAsCorbaString(eachPrefix.clone())?;
            Print::printBuf((literal!(", path = ")).clone())?;
            printPathAsCorbaString(p.clone())?;
            Print::printBuf((literal!(", modification = ")).clone())?;
            printOption(modification.clone(), Arc::new(printModificationAsCorbaString))?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.MODIFICATION;")).clone())?;
            ()
        },
        Deref @ Absyn::REDECLARATION { finalPrefix, redeclareKeywords, eachPrefix, elementSpec, constrainClass, info } => {
            let mut modification: Option<Arc<Absyn::Modification>>;
            let mut comment: Option<ArcStr>;
            let mut p: Arc<Absyn::Path>;
            Print::printBuf((literal!("record Absyn.REDECLARATION finalPrefix = ")).clone())?;
            Print::printBuf((boolString(finalPrefix.clone())).clone())?;
            Print::printBuf((literal!(", redeclareKeywords = ")).clone())?;
            printRedeclareKeywordsAsCorbaString(redeclareKeywords.clone())?;
            Print::printBuf((literal!(", eachPrefix = ")).clone())?;
            printEachAsCorbaString(eachPrefix.clone())?;
            Print::printBuf((literal!(", elementSpec = ")).clone())?;
            printElementSpecAsCorbaString(elementSpec.clone())?;
            Print::printBuf((literal!(", constrainClass = ")).clone())?;
            printOption(constrainClass.clone(), Arc::new(printConstrainClassAsCorbaString))?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.REDECLARATION;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printElementAsCorbaString(el: Arc<Absyn::Element>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::ELEMENT { finalPrefix, redeclareKeywords, innerOuter, specification, info, constrainClass } => {
            let mut name: ArcStr;
            let mut string: ArcStr;
            let mut args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut optName: Option<ArcStr>;
            Print::printBuf((literal!("\\nrecord Absyn.ELEMENT finalPrefix = ")).clone())?;
            Print::printBuf((boolString(finalPrefix.clone())).clone())?;
            Print::printBuf((literal!(",redeclareKeywords = ")).clone())?;
            printOption(redeclareKeywords.clone(), Arc::new(printRedeclareKeywordsAsCorbaString))?;
            Print::printBuf((literal!(",innerOuter = ")).clone())?;
            printInnerOuterAsCorbaString(innerOuter.clone())?;
            Print::printBuf((literal!(",specification = ")).clone())?;
            printElementSpecAsCorbaString(specification.clone())?;
            Print::printBuf((literal!(",info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(",constrainClass = ")).clone())?;
            printOption(constrainClass.clone(), Arc::new(printConstrainClassAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.ELEMENT;")).clone())?;
            ()
        },
        Deref @ Absyn::DEFINEUNIT { name, args, .. } => {
            let mut finalPrefix: bool;
            let mut redeclareKeywords: Option<Absyn::RedeclareKeywords>;
            let mut innerOuter: Absyn::InnerOuter;
            let mut string: ArcStr;
            let mut specification: Arc<Absyn::ElementSpec>;
            let mut info: SourceInfo;
            let mut constrainClass: Option<Arc<Absyn::ConstrainClass>>;
            let mut optName: Option<ArcStr>;
            Print::printBuf((literal!("\\nrecord Absyn.DEFINEUNIT name = \\\"")).clone())?;
            Print::printBuf((name.clone()).clone())?;
            Print::printBuf((literal!("\\\", args = ")).clone())?;
            printListAsCorbaString(args.clone(), Arc::new(printNamedArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.DEFINEUNIT;")).clone())?;
            ()
        },
        Deref @ Absyn::TEXT { optName, string, info } => {
            let mut finalPrefix: bool;
            let mut redeclareKeywords: Option<Absyn::RedeclareKeywords>;
            let mut innerOuter: Absyn::InnerOuter;
            let mut name: ArcStr;
            let mut specification: Arc<Absyn::ElementSpec>;
            let mut constrainClass: Option<Arc<Absyn::ConstrainClass>>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            Print::printBuf((literal!("\\nrecord Absyn.TEXT optName = ")).clone())?;
            printStringCommentOption(optName.clone())?;
            Print::printBuf((literal!(", string = \\\"")).clone())?;
            Print::printBuf((string.clone()).clone())?;
            Print::printBuf((literal!("\\\", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.TEXT;")).clone())?;
            ()
        },
        _ => {
            let mut finalPrefix: bool;
            let mut redeclareKeywords: Option<Absyn::RedeclareKeywords>;
            let mut innerOuter: Absyn::InnerOuter;
            let mut name: ArcStr;
            let mut string: ArcStr;
            let mut specification: Arc<Absyn::ElementSpec>;
            let mut info: SourceInfo;
            let mut constrainClass: Option<Arc<Absyn::ConstrainClass>>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut optName: Option<ArcStr>;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printElementAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printElementAttributesAsCorbaString(attr: Absyn::ElementAttributes) -> Result<()> {
    let _ = (match attr.clone() {
        Absyn::ATTR { flowPrefix: mut flowPrefix, streamPrefix: mut streamPrefix, parallelism: mut parallelism, variability: mut variability, direction: mut direction, isField: mut isField, arrayDim: mut arrayDim } => {
            Print::printBuf((literal!("record Absyn.ATTR flowPrefix = ")).clone())?;
            Print::printBuf((boolString(flowPrefix.clone())).clone())?;
            Print::printBuf((literal!(", streamPrefix = ")).clone())?;
            Print::printBuf((boolString(streamPrefix.clone())).clone())?;
            Print::printBuf((literal!(", parallelism = ")).clone())?;
            printParallelismAsCorbaString(parallelism.clone())?;
            Print::printBuf((literal!(", variability = ")).clone())?;
            printVariabilityAsCorbaString(variability.clone())?;
            Print::printBuf((literal!(", direction = ")).clone())?;
            printDirectionAsCorbaString(direction.clone())?;
            if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                Print::printBuf((literal!(", isField = ")).clone())?;
                printIsFieldAsCorbaString(isField.clone())?;
            }
            Print::printBuf((literal!(", arrayDim = ")).clone())?;
            printArrayDimAsCorbaString(arrayDim.clone())?;
            Print::printBuf((literal!(" end Absyn.ATTR;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printElementItemAsCorbaString(el: Arc<Absyn::ElementItem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::ELEMENTITEM { element } => {
            let mut annotation_: Arc<Absyn::Annotation>;
            let mut cmt: ArcStr;
            Print::printBuf((literal!("record Absyn.ELEMENTITEM element = ")).clone())?;
            printElementAsCorbaString(element.clone())?;
            Print::printBuf((literal!(" end Absyn.ELEMENTITEM;")).clone())?;
            ()
        },
        Deref @ Absyn::LEXER_COMMENT { comment: cmt } => {
            let mut element: Arc<Absyn::Element>;
            let mut annotation_: Arc<Absyn::Annotation>;
            Print::printBuf((literal!("record Absyn.ELEMENTITEM element = \\\"")).clone())?;
            Print::printBuf((cmt.clone()).clone())?;
            Print::printBuf((literal!("\\\" end Absyn.ELEMENTITEM;")).clone())?;
            ()
        },
        _ => {
            let mut element: Arc<Absyn::Element>;
            let mut annotation_: Arc<Absyn::Annotation>;
            let mut cmt: ArcStr;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printElementItemAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printElementSpecAsCorbaString(spec: Arc<Absyn::ElementSpec>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::CLASSDEF { replaceable_, class_ } => {
            let mut import_: Absyn::Import;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut attributes: Absyn::ElementAttributes;
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut annotationOpt: Option<Arc<Absyn::Annotation>>;
            let mut elementArg: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut path: Arc<Absyn::Path>;
            let mut info: SourceInfo;
            Print::printBuf((literal!("record Absyn.CLASSDEF replaceable_ = ")).clone())?;
            Print::printBuf((boolString(replaceable_.clone())).clone())?;
            Print::printBuf((literal!(", class_ = ")).clone())?;
            printClassAsCorbaString(class_.clone())?;
            Print::printBuf((literal!(" end Absyn.CLASSDEF;")).clone())?;
            ()
        },
        Deref @ Absyn::EXTENDS { path, elementArg, annotationOpt } => {
            let mut replaceable_: bool;
            let mut class_: Arc<Absyn::Class>;
            let mut import_: Absyn::Import;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut attributes: Absyn::ElementAttributes;
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut info: SourceInfo;
            Print::printBuf((literal!("record Absyn.EXTENDS path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(", elementArg = ")).clone())?;
            printListAsCorbaString(elementArg.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", annotationOpt = ")).clone())?;
            printOption(annotationOpt.clone(), Arc::new(printAnnotationAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.EXTENDS;")).clone())?;
            ()
        },
        Deref @ Absyn::IMPORT { import_, comment, info } => {
            let mut replaceable_: bool;
            let mut class_: Arc<Absyn::Class>;
            let mut attributes: Absyn::ElementAttributes;
            let mut typeSpec: Arc<Absyn::TypeSpec>;
            let mut components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut annotationOpt: Option<Arc<Absyn::Annotation>>;
            let mut elementArg: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut path: Arc<Absyn::Path>;
            Print::printBuf((literal!("record Absyn.IMPORT import_ = ")).clone())?;
            printImportAsCorbaString(import_.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.IMPORT;")).clone())?;
            ()
        },
        Deref @ Absyn::COMPONENTS { attributes, typeSpec, components } => {
            let mut replaceable_: bool;
            let mut class_: Arc<Absyn::Class>;
            let mut import_: Absyn::Import;
            let mut comment: Option<Arc<Absyn::Comment>>;
            let mut annotationOpt: Option<Arc<Absyn::Annotation>>;
            let mut elementArg: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut path: Arc<Absyn::Path>;
            let mut info: SourceInfo;
            Print::printBuf((literal!("record Absyn.COMPONENTS attributes = ")).clone())?;
            printElementAttributesAsCorbaString(attributes.clone())?;
            Print::printBuf((literal!(", typeSpec = ")).clone())?;
            printTypeSpecAsCorbaString(typeSpec.clone())?;
            Print::printBuf((literal!(", components = ")).clone())?;
            printListAsCorbaString(components.clone(), Arc::new(printComponentItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.COMPONENTS;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEnumDefAsCorbaString(enumDef: Arc<Absyn::EnumDef>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(enumDef.clone()) {
        Deref @ Absyn::ENUMLITERALS { enumLiterals } => {
            Print::printBuf((literal!("record Absyn.ENUMLITERALS enumLiterals = ")).clone())?;
            printListAsCorbaString(enumLiterals.clone(), Arc::new(printEnumLiteralAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!("end Absyn.ENUMLITERALS;")).clone())?;
            ()
        },
        Deref @ Absyn::ENUM_COLON => {
            let mut enumLiterals: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            Print::printBuf((literal!("record Absyn.ENUM_COLON end Absyn.ENUM_COLON;")).clone())?;
            ()
        },
        _ => {
            let mut enumLiterals: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printEnumDefAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printEnumLiteralAsCorbaString(enumLit: Arc<Absyn::EnumLiteral>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(enumLit.clone()) {
        Deref @ Absyn::ENUMLITERAL { literal, comment } => {
            Print::printBuf((literal!("record Absyn.ENUMLITERAL literal = \\\"")).clone())?;
            Print::printBuf((literal.clone()).clone())?;
            Print::printBuf((literal!("\\\", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.ENUMLITERAL;")).clone())?;
            ()
        },
        _ => {
            let mut literal: ArcStr;
            let mut comment: Option<Arc<Absyn::Comment>>;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printEnumLiteralAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printEqModAsCorbaString(eqMod: Arc<Absyn::EqMod>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(eqMod.clone()) {
        Deref @ Absyn::NOMOD => {
            let mut exp: Arc<Absyn::Exp>;
            let mut info: SourceInfo;
            Print::printBuf((literal!("record Absyn.NOMOD end Absyn.NOMOD;")).clone())?;
            ()
        },
        Deref @ Absyn::EQMOD { exp, info } => {
            Print::printBuf((literal!("record Absyn.EQMOD exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.EQMOD;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEquationAsCorbaString(eq: Arc<Absyn::Equation>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EQ_IF { ifExp, equationTrueItems, elseIfBranches, equationElseItems } => {
            let mut leftSide: Arc<Absyn::Exp>;
            let mut rightSide: Arc<Absyn::Exp>;
            let mut whenExp: Arc<Absyn::Exp>;
            let mut elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut connector1: Arc<Absyn::ComponentRef>;
            let mut connector2: Arc<Absyn::ComponentRef>;
            let mut functionName: Arc<Absyn::ComponentRef>;
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut equ: Arc<Absyn::EquationItem>;
            Print::printBuf((literal!("record Absyn.EQ_IF ifExp = ")).clone())?;
            printExpAsCorbaString(ifExp.clone())?;
            Print::printBuf((literal!(", equationTrueItems = ")).clone())?;
            printListAsCorbaString(equationTrueItems.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseIfBranches = ")).clone())?;
            printListAsCorbaString(elseIfBranches.clone(), Arc::new(printEquationBranchAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", equationElseItems = ")).clone())?;
            printListAsCorbaString(equationElseItems.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_IF;")).clone())?;
            ()
        },
        Deref @ Absyn::EQ_EQUALS { leftSide, rightSide } => {
            let mut ifExp: Arc<Absyn::Exp>;
            let mut whenExp: Arc<Absyn::Exp>;
            let mut elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut connector1: Arc<Absyn::ComponentRef>;
            let mut connector2: Arc<Absyn::ComponentRef>;
            let mut functionName: Arc<Absyn::ComponentRef>;
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut equ: Arc<Absyn::EquationItem>;
            Print::printBuf((literal!("record Absyn.EQ_EQUALS leftSide = ")).clone())?;
            printExpAsCorbaString(leftSide.clone())?;
            Print::printBuf((literal!(", rightSide = ")).clone())?;
            printExpAsCorbaString(rightSide.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_EQUALS;")).clone())?;
            ()
        },
        Deref @ Absyn::EQ_PDE { leftSide, rightSide, domain: cr } => {
            let mut ifExp: Arc<Absyn::Exp>;
            let mut whenExp: Arc<Absyn::Exp>;
            let mut elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut connector1: Arc<Absyn::ComponentRef>;
            let mut connector2: Arc<Absyn::ComponentRef>;
            let mut functionName: Arc<Absyn::ComponentRef>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut equ: Arc<Absyn::EquationItem>;
            Print::printBuf((literal!("record Absyn.EQ_PDE leftSide = ")).clone())?;
            printExpAsCorbaString(leftSide.clone())?;
            Print::printBuf((literal!(", rightSide = ")).clone())?;
            printExpAsCorbaString(rightSide.clone())?;
            Print::printBuf((literal!(", domain = ")).clone())?;
            printComponentRefAsCorbaString(cr.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_PDE;")).clone())?;
            ()
        },
        Deref @ Absyn::EQ_CONNECT { connector1, connector2 } => {
            let mut ifExp: Arc<Absyn::Exp>;
            let mut leftSide: Arc<Absyn::Exp>;
            let mut rightSide: Arc<Absyn::Exp>;
            let mut whenExp: Arc<Absyn::Exp>;
            let mut elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut functionName: Arc<Absyn::ComponentRef>;
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut equ: Arc<Absyn::EquationItem>;
            Print::printBuf((literal!("record Absyn.EQ_CONNECT connector1 = ")).clone())?;
            printComponentRefAsCorbaString(connector1.clone())?;
            Print::printBuf((literal!(", connector2 = ")).clone())?;
            printComponentRefAsCorbaString(connector2.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_CONNECT;")).clone())?;
            ()
        },
        Deref @ Absyn::EQ_FOR { iterators, forEquations } => {
            let mut ifExp: Arc<Absyn::Exp>;
            let mut leftSide: Arc<Absyn::Exp>;
            let mut rightSide: Arc<Absyn::Exp>;
            let mut whenExp: Arc<Absyn::Exp>;
            let mut elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut connector1: Arc<Absyn::ComponentRef>;
            let mut connector2: Arc<Absyn::ComponentRef>;
            let mut functionName: Arc<Absyn::ComponentRef>;
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut equ: Arc<Absyn::EquationItem>;
            Print::printBuf((literal!("record Absyn.EQ_FOR iterators = ")).clone())?;
            printListAsCorbaString(iterators.clone(), Arc::new(printForIteratorAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", forEquations = ")).clone())?;
            printListAsCorbaString(forEquations.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_FOR;")).clone())?;
            ()
        },
        Deref @ Absyn::EQ_WHEN_E { whenExp, whenEquations, elseWhenEquations } => {
            let mut ifExp: Arc<Absyn::Exp>;
            let mut leftSide: Arc<Absyn::Exp>;
            let mut rightSide: Arc<Absyn::Exp>;
            let mut elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut connector1: Arc<Absyn::ComponentRef>;
            let mut connector2: Arc<Absyn::ComponentRef>;
            let mut functionName: Arc<Absyn::ComponentRef>;
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut equ: Arc<Absyn::EquationItem>;
            Print::printBuf((literal!("record Absyn.EQ_WHEN_E whenExp = ")).clone())?;
            printExpAsCorbaString(whenExp.clone())?;
            Print::printBuf((literal!(", whenEquations = ")).clone())?;
            printListAsCorbaString(whenEquations.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseWhenEquations = ")).clone())?;
            printListAsCorbaString(elseWhenEquations.clone(), Arc::new(printEquationBranchAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_WHEN_E;")).clone())?;
            ()
        },
        Deref @ Absyn::EQ_NORETCALL { functionName, functionArgs } => {
            let mut ifExp: Arc<Absyn::Exp>;
            let mut leftSide: Arc<Absyn::Exp>;
            let mut rightSide: Arc<Absyn::Exp>;
            let mut whenExp: Arc<Absyn::Exp>;
            let mut elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut connector1: Arc<Absyn::ComponentRef>;
            let mut connector2: Arc<Absyn::ComponentRef>;
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut equ: Arc<Absyn::EquationItem>;
            Print::printBuf((literal!("record Absyn.EQ_NORETCALL functionName = ")).clone())?;
            printComponentRefAsCorbaString(functionName.clone())?;
            Print::printBuf((literal!(", functionArgs = ")).clone())?;
            printFunctionArgsAsCorbaString(functionArgs.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_NORETCALL;")).clone())?;
            ()
        },
        Deref @ Absyn::EQ_FAILURE { equ } => {
            let mut ifExp: Arc<Absyn::Exp>;
            let mut leftSide: Arc<Absyn::Exp>;
            let mut rightSide: Arc<Absyn::Exp>;
            let mut whenExp: Arc<Absyn::Exp>;
            let mut elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut connector1: Arc<Absyn::ComponentRef>;
            let mut connector2: Arc<Absyn::ComponentRef>;
            let mut functionName: Arc<Absyn::ComponentRef>;
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            Print::printBuf((literal!("record Absyn.EQ_FAILURE equ = ")).clone())?;
            printEquationItemAsCorbaString(equ.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_FAILURE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEquationBranchAsCorbaString(inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)) -> Result<()> {
    printTupleAsCorbaString(inBranch.clone(), Arc::new(printExpAsCorbaString), Arc::new(printEquationItemListAsCorbaString))?;
    Ok(())
}

fn printEquationItemAsCorbaString(el: Arc<Absyn::EquationItem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::EQUATIONITEM { equation_, comment, info } => {
            let mut annotation_: Arc<Absyn::Annotation>;
            Print::printBuf((literal!("\\nrecord Absyn.EQUATIONITEM equation_ = ")).clone())?;
            printEquationAsCorbaString(equation_.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.EQUATIONITEM;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEquationItemListAsCorbaString(inLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<()> {
    printListAsCorbaString(inLst.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
    Ok(())
}

fn printExpAsCorbaString(inExp: Arc<Absyn::Exp>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::INTEGER { value: i } => {
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.INTEGER value = ")).clone())?;
            Print::printBuf((intString(i.clone())).clone())?;
            Print::printBuf((literal!(" end Absyn.INTEGER;")).clone())?;
            ()
        },
        Deref @ Absyn::REAL { value: s } => {
            let mut i: i32;
            let mut r: f64;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.REAL value = ")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!(" end Absyn.REAL;")).clone())?;
            ()
        },
        Deref @ Absyn::CREF { componentRef } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.CREF componentRef = ")).clone())?;
            printComponentRefAsCorbaString(componentRef.clone())?;
            Print::printBuf((literal!(" end Absyn.CREF;")).clone())?;
            ()
        },
        Deref @ Absyn::STRING { value: s } => {
            let mut i: i32;
            let mut r: f64;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.STRING value = \\\"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\\\" end Absyn.STRING;")).clone())?;
            ()
        },
        Deref @ Absyn::BOOL { value: b } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.BOOL value = ")).clone())?;
            Print::printBuf((boolString(b.clone())).clone())?;
            Print::printBuf((literal!(" end Absyn.BOOL;")).clone())?;
            ()
        },
        Deref @ Absyn::BINARY { exp1, op, exp2 } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.BINARY exp1 = ")).clone())?;
            printExpAsCorbaString(exp1.clone())?;
            Print::printBuf((literal!(", op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp2 = ")).clone())?;
            printExpAsCorbaString(exp2.clone())?;
            Print::printBuf((literal!(" end Absyn.BINARY;")).clone())?;
            ()
        },
        Deref @ Absyn::UNARY { op, exp } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.UNARY op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(" end Absyn.UNARY;")).clone())?;
            ()
        },
        Deref @ Absyn::LBINARY { exp1, op, exp2 } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.LBINARY exp1 = ")).clone())?;
            printExpAsCorbaString(exp1.clone())?;
            Print::printBuf((literal!(", op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp2 = ")).clone())?;
            printExpAsCorbaString(exp2.clone())?;
            Print::printBuf((literal!(" end Absyn.LBINARY;")).clone())?;
            ()
        },
        Deref @ Absyn::LUNARY { op, exp } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.LUNARY op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(" end Absyn.LUNARY;")).clone())?;
            ()
        },
        Deref @ Absyn::RELATION { exp1, op, exp2 } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.RELATION exp1 = ")).clone())?;
            printExpAsCorbaString(exp1.clone())?;
            Print::printBuf((literal!(", op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp2 = ")).clone())?;
            printExpAsCorbaString(exp2.clone())?;
            Print::printBuf((literal!(" end Absyn.RELATION;")).clone())?;
            ()
        },
        Deref @ Absyn::IFEXP { ifExp, trueBranch, elseBranch, elseIfBranch } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.IFEXP ifExp = ")).clone())?;
            printExpAsCorbaString(ifExp.clone())?;
            Print::printBuf((literal!(", trueBranch = ")).clone())?;
            printExpAsCorbaString(trueBranch.clone())?;
            Print::printBuf((literal!(", elseBranch = ")).clone())?;
            printExpAsCorbaString(elseBranch.clone())?;
            Print::printBuf((literal!(", elseIfBranch = ")).clone())?;
            printListAsCorbaString(elseIfBranch.clone(), Arc::new(printTupleExpExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.IFEXP;")).clone())?;
            ()
        },
        Deref @ Absyn::CALL { function_, functionArgs, .. } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.CALL function_ = ")).clone())?;
            printComponentRefAsCorbaString(function_.clone())?;
            Print::printBuf((literal!(", functionArgs = ")).clone())?;
            printFunctionArgsAsCorbaString(functionArgs.clone())?;
            Print::printBuf((literal!(" end Absyn.CALL;")).clone())?;
            ()
        },
        Deref @ Absyn::PARTEVALFUNCTION { function_, functionArgs } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.PARTEVALFUNCTION function_ = ")).clone())?;
            printComponentRefAsCorbaString(function_.clone())?;
            Print::printBuf((literal!(", functionArgs = ")).clone())?;
            printFunctionArgsAsCorbaString(functionArgs.clone())?;
            Print::printBuf((literal!(" end Absyn.PARTEVALFUNCTION;")).clone())?;
            ()
        },
        Deref @ Absyn::ARRAY { arrayExp } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.ARRAY arrayExp = ")).clone())?;
            printListAsCorbaString(arrayExp.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ARRAY;")).clone())?;
            ()
        },
        Deref @ Absyn::MATRIX { matrix } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.MATRIX matrix = ")).clone())?;
            printListAsCorbaString(matrix.clone(), Arc::new(printListExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.MATRIX;")).clone())?;
            ()
        },
        Deref @ Absyn::RANGE { start, step, stop } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.RANGE start = ")).clone())?;
            printExpAsCorbaString(start.clone())?;
            Print::printBuf((literal!(", step = ")).clone())?;
            printOption(step.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!(", stop = ")).clone())?;
            printExpAsCorbaString(stop.clone())?;
            Print::printBuf((literal!(" end Absyn.RANGE;")).clone())?;
            ()
        },
        Deref @ Absyn::TUPLE { expressions } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.TUPLE expressions = ")).clone())?;
            printListAsCorbaString(expressions.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.TUPLE;")).clone())?;
            ()
        },
        Deref @ Absyn::END => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.END end Absyn.END;")).clone())?;
            ()
        },
        Deref @ Absyn::CODE { code } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.CODE code = ")).clone())?;
            printCodeAsCorbaString(code.clone())?;
            Print::printBuf((literal!(" end Absyn.CODE;")).clone())?;
            ()
        },
        Deref @ Absyn::AS { id, exp } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.AS id = \\\"")).clone())?;
            Print::printBuf((id.clone()).clone())?;
            Print::printBuf((literal!("\\\", exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(" end Absyn.AS;")).clone())?;
            ()
        },
        Deref @ Absyn::CONS { head, rest } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut inputExp: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            let mut matchTy: Absyn::MatchType;
            let mut localDecls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut comment: Option<ArcStr>;
            Print::printBuf((literal!("record Absyn.CONS head = ")).clone())?;
            printExpAsCorbaString(head.clone())?;
            Print::printBuf((literal!(", rest = ")).clone())?;
            printExpAsCorbaString(rest.clone())?;
            Print::printBuf((literal!(" end Absyn.CONS;")).clone())?;
            ()
        },
        Deref @ Absyn::MATCHEXP { matchTy, inputExp, localDecls, cases, comment } => {
            let mut i: i32;
            let mut r: f64;
            let mut s: ArcStr;
            let mut id: ArcStr;
            let mut b: bool;
            let mut componentRef: Arc<Absyn::ComponentRef>;
            let mut function_: Arc<Absyn::ComponentRef>;
            let mut functionArgs: Arc<Absyn::FunctionArgs>;
            let mut exp: Arc<Absyn::Exp>;
            let mut exp1: Arc<Absyn::Exp>;
            let mut exp2: Arc<Absyn::Exp>;
            let mut ifExp: Arc<Absyn::Exp>;
            let mut trueBranch: Arc<Absyn::Exp>;
            let mut elseBranch: Arc<Absyn::Exp>;
            let mut start: Arc<Absyn::Exp>;
            let mut stop: Arc<Absyn::Exp>;
            let mut head: Arc<Absyn::Exp>;
            let mut rest: Arc<Absyn::Exp>;
            let mut step: Option<Arc<Absyn::Exp>>;
            let mut op: Absyn::Operator;
            let mut arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut elseIfBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut code: Arc<Absyn::CodeNode>;
            Print::printBuf((literal!("record Absyn.MATCHEXP matchTy = ")).clone())?;
            printMatchTypeAsCorbaString(matchTy.clone())?;
            Print::printBuf((literal!(", inputExp = ")).clone())?;
            printExpAsCorbaString(inputExp.clone())?;
            Print::printBuf((literal!(", localDecls = ")).clone())?;
            printListAsCorbaString(localDecls.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",\\n")).clone())?;
            Print::printBuf((literal!(", cases = ")).clone())?;
            printListAsCorbaString(cases.clone(), Arc::new(printCaseAsCorbaString), (literal!(",\\n")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(" end Absyn.MATCHEXP;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printExpLstStr(expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = stringDelimitList(List::map(expl.clone(), Arc::new(printExpStr)), (literal!(", ")).clone());
    outString
}

pub fn printExpStr(inExp: Arc<Absyn::Exp>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpExp), inExp.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

fn printExternalDeclAsCorbaString(decl: Arc<Absyn::ExternalDecl>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(decl.clone()) {
        Deref @ Absyn::EXTERNALDECL { funcName, lang, output_, args, annotation_ } => {
            Print::printBuf((literal!("record Absyn.EXTERNALDECL funcName = ")).clone())?;
            printStringCommentOption(funcName.clone())?;
            Print::printBuf((literal!(", lang = ")).clone())?;
            printStringCommentOption(lang.clone())?;
            Print::printBuf((literal!(", output_ = ")).clone())?;
            printOption(output_.clone(), Arc::new(printComponentRefAsCorbaString))?;
            Print::printBuf((literal!(", args = ")).clone())?;
            printListAsCorbaString(args.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", annotation_ = ")).clone())?;
            printOption(annotation_.clone(), Arc::new(printAnnotationAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.EXTERNALDECL;")).clone())?;
            ()
        },
        _ => {
            let mut funcName: Option<ArcStr>;
            let mut lang: Option<ArcStr>;
            let mut output_: Option<Arc<Absyn::ComponentRef>>;
            let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut annotation_: Option<Arc<Absyn::Annotation>>;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printExternalDeclAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printForIteratorAsCorbaString(iter: Arc<Absyn::ForIterator>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ Absyn::ITERATOR { name: id, guardExp, range } => {
            Print::printBuf((literal!("record Absyn.ITERATOR name = \\\"")).clone())?;
            Print::printBuf((id.clone()).clone())?;
            Print::printBuf((literal!("\\\", guardExp = ")).clone())?;
            printOption(guardExp.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!(", range = ")).clone())?;
            printOption(range.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.ITERATOR;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printFunctionArgsAsCorbaString(fargs: Arc<Absyn::FunctionArgs>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(fargs.clone()) {
        Deref @ Absyn::FUNCTIONARGS { args, argNames } => {
            let mut exp: Arc<Absyn::Exp>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            Print::printBuf((literal!("record Absyn.FUNCTIONARGS args = ")).clone())?;
            printListAsCorbaString(args.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", argNames = ")).clone())?;
            printListAsCorbaString(argNames.clone(), Arc::new(printNamedArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.FUNCTIONARGS;")).clone())?;
            ()
        },
        Deref @ Absyn::FOR_ITER_FARG { exp, iterType: _, iterators } => {
            let mut args: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            Print::printBuf((literal!("record Absyn.FOR_ITER_FARG exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(", iterators = ")).clone())?;
            printListAsCorbaString(iterators.clone(), Arc::new(printForIteratorAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.FOR_ITER_FARG;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printFunctionArgsStr(inFunctionArgs: Arc<Absyn::FunctionArgs>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ('mc: {
        let __mc_input = inFunctionArgs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FUNCTIONARGS { argNames: nargs @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, args: expargs @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    let mut estr: ArcStr;
                    let mut istr: ArcStr;
                    let mut exp: Arc<Absyn::Exp>;
                    let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
                    s1 = (printListStr(expargs.clone(), Arc::new(printExpStr), (literal!(", ")).clone())?).clone();
                    s2 = (stringAppend((s1.clone()).clone(), (literal!(", ")).clone())).clone();
                    s3 = (printListStr(nargs.clone(), Arc::new(printNamedArgStr), (literal!(", ")).clone())?).clone();
                    r#str = (stringAppend((s2.clone()).clone(), (s3.clone()).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FUNCTIONARGS { argNames: nargs, args: Deref @ metamodelica::List::Nil } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    let mut estr: ArcStr;
                    let mut istr: ArcStr;
                    let mut expargs: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut exp: Arc<Absyn::Exp>;
                    let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
                    r#str = (printListStr(nargs.clone(), Arc::new(printNamedArgStr), (literal!(", ")).clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: expargs } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    let mut estr: ArcStr;
                    let mut istr: ArcStr;
                    let mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
                    let mut exp: Arc<Absyn::Exp>;
                    let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
                    r#str = (printListStr(expargs.clone(), Arc::new(printExpStr), (literal!(", ")).clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FOR_ITER_FARG { iterators, exp, .. } => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut r#str: ArcStr;
                    let mut estr: ArcStr;
                    let mut istr: ArcStr;
                    let mut expargs: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
                    estr = (printExpStr(exp.clone())?).clone();
                    istr = (printIteratorsStr(iterators.clone())?).clone();
                    r#str = stringAppendList(list![(estr.clone()).clone(), (literal!(" for ")).clone(), (istr.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn printFunctionPurityAsCorbaString(functionPurity: Absyn::FunctionPurity) -> Result<()> {
    let _ = (match functionPurity.clone() {
        Absyn::PURE => {
            Print::printBuf((literal!("record Absyn.PURE end Absyn.PURE;")).clone())?;
            ()
        },
        Absyn::IMPURE => {
            Print::printBuf((literal!("record Absyn.IMPURE end Absyn.IMPURE;")).clone())?;
            ()
        },
        Absyn::NO_PURITY => {
            Print::printBuf((literal!("record Absyn.NO_PURITY end Absyn.NO_PURITY;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printFunctionRestrictionAsCorbaString(functionRestriction: Absyn::FunctionRestriction) -> Result<()> {
    let _ = (match functionRestriction.clone() {
        Absyn::FR_NORMAL_FUNCTION { purity: mut purity } => {
            Print::printBuf((literal!("record Absyn.FR_NORMAL_FUNCTION purity = ")).clone())?;
            printFunctionPurityAsCorbaString(purity.clone())?;
            Print::printBuf((literal!(" end Absyn.FR_NORMAL_FUNCTION;")).clone())?;
            ()
        },
        Absyn::FR_OPERATOR_FUNCTION => {
            let mut purity: Absyn::FunctionPurity;
            Print::printBuf((literal!("record Absyn.FR_OPERATOR_FUNCTION end Absyn.FR_OPERATOR_FUNCTION;")).clone())?;
            ()
        },
        Absyn::FR_PARALLEL_FUNCTION => {
            let mut purity: Absyn::FunctionPurity;
            Print::printBuf((literal!("record Absyn.FR_PARALLEL_FUNCTION end Absyn.FR_PARALLEL_FUNCTION;")).clone())?;
            ()
        },
        Absyn::FR_KERNEL_FUNCTION => {
            let mut purity: Absyn::FunctionPurity;
            Print::printBuf((literal!("record Absyn.FR_KERNEL_FUNCTION end Absyn.FR_KERNEL_FUNCTION;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printImportAsCorbaString(import_: Absyn::Import) -> Result<()> {
    let _ = (match import_.clone() {
        Absyn::NAMED_IMPORT { name: mut name, path: mut path } => {
            Print::printBuf((literal!("record Absyn.NAMED_IMPORT name = \\\"")).clone())?;
            Print::printBuf((name.clone()).clone())?;
            Print::printBuf((literal!("\\\", path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.NAMED_IMPORT;")).clone())?;
            ()
        },
        Absyn::QUAL_IMPORT { path: mut path } => {
            let mut name: ArcStr;
            Print::printBuf((literal!("record Absyn.QUAL_IMPORT path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.QUAL_IMPORT;")).clone())?;
            ()
        },
        Absyn::UNQUAL_IMPORT { path: mut path } => {
            let mut name: ArcStr;
            Print::printBuf((literal!("record Absyn.UNQUAL_IMPORT path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.UNQUAL_IMPORT;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printInfoAsCorbaString(info: SourceInfo) -> Result<()> {
    let _ = (match info.clone() {
        SourceInfo { file_name: mut fileName, is_read_only: mut isReadOnly, line_number_start: mut lineNumberStart, column_number_start: mut columnNumberStart, line_number_end: mut lineNumberEnd, column_number_end: mut columnNumberEnd, last_modification: mut lastModified } => {
            Print::printBuf((literal!("record SOURCEINFO fileName = \\\"")).clone())?;
            Print::printBuf((fileName.clone()).clone())?;
            Print::printBuf((literal!("\\\", isReadOnly = ")).clone())?;
            Print::printBuf((boolString(isReadOnly.clone())).clone())?;
            Print::printBuf((literal!(", lineNumberStart = ")).clone())?;
            Print::printBuf((intString(lineNumberStart.clone())).clone())?;
            Print::printBuf((literal!(", columnNumberStart = ")).clone())?;
            Print::printBuf((intString(columnNumberStart.clone())).clone())?;
            Print::printBuf((literal!(", lineNumberEnd = ")).clone())?;
            Print::printBuf((intString(lineNumberEnd.clone())).clone())?;
            Print::printBuf((literal!(", columnNumberEnd = ")).clone())?;
            Print::printBuf((intString(columnNumberEnd.clone())).clone())?;
            Print::printBuf((literal!(", lastModified = ")).clone())?;
            Print::printBuf((realString(lastModified.clone())).clone())?;
            Print::printBuf((literal!(" end SOURCEINFO;")).clone())?;
            ()
        },
        _ => {
            let mut fileName: ArcStr;
            let mut isReadOnly: bool;
            let mut lineNumberStart: i32;
            let mut columnNumberStart: i32;
            let mut lineNumberEnd: i32;
            let mut columnNumberEnd: i32;
            let mut lastModified: f64;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printInfoAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
    });
    Ok(())
}

fn printInnerOuterAsCorbaString(innerOuter: Absyn::InnerOuter) -> Result<()> {
    let _ = (match innerOuter.clone() {
        Absyn::INNER => {
            Print::printBuf((literal!("record Absyn.INNER end Absyn.INNER;")).clone())?;
            ()
        },
        Absyn::OUTER => {
            Print::printBuf((literal!("record Absyn.OUTER end Absyn.OUTER;")).clone())?;
            ()
        },
        Absyn::INNER_OUTER => {
            Print::printBuf((literal!("record Absyn.INNER_OUTER end Absyn.INNER_OUTER;")).clone())?;
            ()
        },
        Absyn::NOT_INNER_OUTER => {
            Print::printBuf((literal!("record Absyn.NOT_INNER_OUTER end Absyn.NOT_INNER_OUTER;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printIsFieldAsCorbaString(isf: Absyn::IsField) -> Result<()> {
    let _ = (match isf.clone() {
        Absyn::NONFIELD => {
            Print::printBuf((literal!("record Absyn.NONFIELD end Absyn.NONFIELD;")).clone())?;
            ()
        },
        Absyn::FIELD => {
            Print::printBuf((literal!("record Absyn.FIELD end Absyn.FIELD;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn printIteratorsStr(iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>) -> Result<ArcStr> {
    let mut iteratorsStr: ArcStr;
    iteratorsStr = ('mc: {
        let __mc_input = iterators.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut guardExp: Arc<Absyn::Exp>;
                    let mut exp: Arc<Absyn::Exp>;
                    let mut id: ArcStr;
                    let mut rest: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
                    let mut x: Arc<Absyn::ForIterator>;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ITERATOR { name: id, guardExp: Some(guardExp), range: Some(exp) }, tail: Deref @ metamodelica::List::Nil } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut rest: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
                    let mut x: Arc<Absyn::ForIterator>;
                    s1 = (printExpStr(exp.clone())?).clone();
                    s2 = (printExpStr(guardExp.clone())?).clone();
                    s = stringAppendList(list![(id.clone()).clone(), (literal!(" guard ")).clone(), (s2.clone()).clone(), (literal!(" in ")).clone(), (s1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ITERATOR { name: id, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::List::Nil } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut guardExp: Arc<Absyn::Exp>;
                    let mut rest: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
                    let mut x: Arc<Absyn::ForIterator>;
                    s1 = (printExpStr(exp.clone())?).clone();
                    s = stringAppendList(list![(id.clone()).clone(), (literal!(" in ")).clone(), (s1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ITERATOR { name: id, guardExp: None, range: None }, tail: Deref @ metamodelica::List::Nil } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut guardExp: Arc<Absyn::Exp>;
                    let mut exp: Arc<Absyn::Exp>;
                    let mut rest: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
                    let mut x: Arc<Absyn::ForIterator>;
                    Ok(id.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: x, tail: rest } => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut guardExp: Arc<Absyn::Exp>;
                    let mut exp: Arc<Absyn::Exp>;
                    let mut id: ArcStr;
                    s1 = (printIteratorsStr(list![x.clone()])?).clone();
                    s2 = (printIteratorsStr(rest.clone())?).clone();
                    s = stringAppendList(list![(s1.clone()).clone(), (literal!(", ")).clone(), (s2.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(iteratorsStr)
}

pub fn printList<Type_a: Clone + 'static>(inTypeALst: Arc<metamodelica::List<Type_a>>, inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>, inString: ArcStr) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone> = fn(Type_a) -> Result<()>;

    let _ = 'mc: {
        let __mc_input = (inTypeALst.clone(), inFuncTypeTypeATo.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    let mut h: Type_a;
                    let mut r: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>;
                    let mut t: Arc<metamodelica::List<Type_a>>;
                    let mut sep: ArcStr;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, r, _) => {
                    let mut t: Arc<metamodelica::List<Type_a>>;
                    let mut sep: ArcStr;
                    r(h.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, r, sep) => {
                    r(h.clone())?;
                    Print::printBuf((sep.clone()).clone())?;
                    printList(t.clone(), r.clone(), (sep.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printListAsCorbaString<Type_a: Clone + 'static>(inTypeALst: Arc<metamodelica::List<Type_a>>, inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>, inString: ArcStr) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone> = fn(Type_a) -> Result<()>;

    Print::printBuf((literal!("{")).clone())?;
    printList(inTypeALst.clone(), inFuncTypeTypeATo.clone(), (inString.clone()).clone())?;
    Print::printBuf((literal!("}")).clone())?;
    Ok(())
}

fn printListExpAsCorbaString(inLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<()> {
    printListAsCorbaString(inLst.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
    Ok(())
}

fn printListStr<Type_a: Clone + 'static>(inTypeALst: Arc<metamodelica::List<Type_a>>, inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>, inString: ArcStr) -> Result<ArcStr> {
    pub type FuncTypeType_aToString<Type_a: Clone> = fn(Type_a) -> Result<ArcStr>;

    let mut outString: ArcStr;
    outString = ('mc: {
        let __mc_input = (inTypeALst.clone(), inFuncTypeTypeAToString.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    let mut s: ArcStr;
                    let mut srest: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut sep: ArcStr;
                    let mut h: Type_a;
                    let mut r: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;
                    let mut t: Arc<metamodelica::List<Type_a>>;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, r, _) => {
                    let mut s: ArcStr;
                    let mut srest: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut sep: ArcStr;
                    let mut t: Arc<metamodelica::List<Type_a>>;
                    s = r(h.clone())?;
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, r, sep) => {
                    let mut s: ArcStr;
                    let mut srest: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s = r(h.clone())?;
                    srest = (printListStr(t.clone(), r.clone(), (sep.clone()).clone())?).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (sep.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (srest.clone()).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn printMatchTypeAsCorbaString(matchTy: Absyn::MatchType) -> Result<()> {
    let _ = (match matchTy.clone() {
        Absyn::MATCH => {
            Print::printBuf((literal!("record Absyn.MATCH end Absyn.MATCH;")).clone())?;
            ()
        },
        Absyn::MATCHCONTINUE => {
            Print::printBuf((literal!("record Absyn.MATCHCONTINUE end Absyn.MATCHCONTINUE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printModificationAsCorbaString(r#mod: Arc<Absyn::Modification>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ Absyn::CLASSMOD { elementArgLst, eqMod } => {
            Print::printBuf((literal!("record Absyn.CLASSMOD elementArgLst = ")).clone())?;
            printListAsCorbaString(elementArgLst.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", eqMod = ")).clone())?;
            printEqModAsCorbaString(eqMod.clone())?;
            Print::printBuf((literal!(" end Absyn.CLASSMOD;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printNamedArgAsCorbaString(arg: Arc<Absyn::NamedArg>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::NAMEDARG { argName, argValue } => {
            Print::printBuf((literal!("record Absyn.NAMEDARG argName = \\\"")).clone())?;
            Print::printBuf((argName.clone()).clone())?;
            Print::printBuf((literal!("\\\", argValue = ")).clone())?;
            printExpAsCorbaString(argValue.clone())?;
            Print::printBuf((literal!(" end Absyn.NAMEDARG;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printNamedArgStr(inNamedArg: Arc<Absyn::NamedArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inNamedArg.clone()) {
        Deref @ Absyn::NAMEDARG { argValue: e, argName: ident } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut r#str: ArcStr;
            s1 = (stringAppend((ident.clone()).clone(), (literal!(" = ")).clone())).clone();
            s2 = (printExpStr(e.clone())?).clone();
            r#str = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printNamedArgValueStr(inNamedArg: Arc<Absyn::NamedArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inNamedArg.clone()) {
        Deref @ Absyn::NAMEDARG { argValue: e, .. } => {
            let mut r#str: ArcStr;
            r#str = (printExpStr(e.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn printOperandStr(inOperand: Arc<Absyn::Exp>, inOperation: Arc<Absyn::Exp>, inLhs: bool) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ('mc: {
        let __mc_input = (inOperand.clone(), inOperation.clone(), inLhs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut op_str: ArcStr;
                    let true = (shouldParenthesize(inOperand.clone(), inOperation.clone(), inLhs.clone())?) else { bail!("pattern mismatch") };
                    op_str = (printExpStr(inOperand.clone())?).clone();
                    op_str = stringAppendList(list![(literal!("(")).clone(), (op_str.clone()).clone(), (literal!(")")).clone()]);
                    Ok(op_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut op_str: ArcStr;
                    Ok(printExpStr(inOperand.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn printOperatorAsCorbaString(op: Absyn::Operator) -> Result<()> {
    let _ = (match op.clone() {
        Absyn::ADD => {
            Print::printBuf((literal!("record Absyn.ADD end Absyn.ADD;")).clone())?;
            ()
        },
        Absyn::SUB => {
            Print::printBuf((literal!("record Absyn.SUB end Absyn.SUB;")).clone())?;
            ()
        },
        Absyn::MUL => {
            Print::printBuf((literal!("record Absyn.MUL end Absyn.MUL;")).clone())?;
            ()
        },
        Absyn::DIV => {
            Print::printBuf((literal!("record Absyn.DIV end Absyn.DIV;")).clone())?;
            ()
        },
        Absyn::POW => {
            Print::printBuf((literal!("record Absyn.POW end Absyn.POW;")).clone())?;
            ()
        },
        Absyn::UPLUS => {
            Print::printBuf((literal!("record Absyn.UPLUS end Absyn.UPLUS;")).clone())?;
            ()
        },
        Absyn::UMINUS => {
            Print::printBuf((literal!("record Absyn.UMINUS end Absyn.UMINUS;")).clone())?;
            ()
        },
        Absyn::ADD_EW => {
            Print::printBuf((literal!("record Absyn.ADD_EW end Absyn.ADD_EW;")).clone())?;
            ()
        },
        Absyn::SUB_EW => {
            Print::printBuf((literal!("record Absyn.SUB_EW end Absyn.SUB_EW;")).clone())?;
            ()
        },
        Absyn::MUL_EW => {
            Print::printBuf((literal!("record Absyn.MUL_EW end Absyn.MUL_EW;")).clone())?;
            ()
        },
        Absyn::DIV_EW => {
            Print::printBuf((literal!("record Absyn.DIV_EW end Absyn.DIV_EW;")).clone())?;
            ()
        },
        Absyn::UPLUS_EW => {
            Print::printBuf((literal!("record Absyn.UPLUS_EW end Absyn.UPLUS_EW;")).clone())?;
            ()
        },
        Absyn::UMINUS_EW => {
            Print::printBuf((literal!("record Absyn.UMINUS_EW end Absyn.UMINUS_EW;")).clone())?;
            ()
        },
        Absyn::AND => {
            Print::printBuf((literal!("record Absyn.AND end Absyn.AND;")).clone())?;
            ()
        },
        Absyn::OR => {
            Print::printBuf((literal!("record Absyn.OR end Absyn.OR;")).clone())?;
            ()
        },
        Absyn::NOT => {
            Print::printBuf((literal!("record Absyn.NOT end Absyn.NOT;")).clone())?;
            ()
        },
        Absyn::LESS => {
            Print::printBuf((literal!("record Absyn.LESS end Absyn.LESS;")).clone())?;
            ()
        },
        Absyn::LESSEQ => {
            Print::printBuf((literal!("record Absyn.LESSEQ end Absyn.LESSEQ;")).clone())?;
            ()
        },
        Absyn::GREATER => {
            Print::printBuf((literal!("record Absyn.GREATER end Absyn.GREATER;")).clone())?;
            ()
        },
        Absyn::GREATEREQ => {
            Print::printBuf((literal!("record Absyn.GREATEREQ end Absyn.GREATEREQ;")).clone())?;
            ()
        },
        Absyn::EQUAL => {
            Print::printBuf((literal!("record Absyn.EQUAL end Absyn.EQUAL;")).clone())?;
            ()
        },
        Absyn::NEQUAL => {
            Print::printBuf((literal!("record Absyn.NEQUAL end Absyn.NEQUAL;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn printOption<Type_a: Clone + 'static>(inTypeAOption: Option<Type_a>, inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone> = fn(Type_a) -> Result<()>;

    let _ = (match inTypeAOption.clone() {
        None => {
            let mut x: Type_a;
            Print::printBuf((literal!("NONE()")).clone())?;
            ()
        },
        Some(mut x) => {
            Print::printBuf((literal!("SOME(")).clone())?;
            inFuncTypeTypeATo(x.clone())?;
            Print::printBuf((literal!(")")).clone())?;
            ()
        },
    });
    Ok(())
}

fn printParallelismAsCorbaString(parallelism: Absyn::Parallelism) -> Result<()> {
    let _ = (match parallelism.clone() {
        Absyn::PARGLOBAL => {
            Print::printBuf((literal!("record Absyn.PARGLOBAL end Absyn.PARGLOBAL;")).clone())?;
            ()
        },
        Absyn::PARLOCAL => {
            Print::printBuf((literal!("record Absyn.PARLOCAL end Absyn.PARLOCAL;")).clone())?;
            ()
        },
        Absyn::NON_PARALLEL => {
            Print::printBuf((literal!("record Absyn.NON_PARALLEL end Absyn.NON_PARALLEL;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printPathAsCorbaString(inPath: Arc<Absyn::Path>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::QUALIFIED { path: p, name: s } => {
            Print::printBuf((literal!("record Absyn.QUALIFIED name = \\\"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\\\", path = ")).clone())?;
            printPathAsCorbaString(p.clone())?;
            Print::printBuf((literal!(" end Absyn.QUALIFIED;")).clone())?;
            ()
        },
        Deref @ Absyn::IDENT { name: s } => {
            let mut p: Arc<Absyn::Path>;
            Print::printBuf((literal!("record Absyn.IDENT name = \\\"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\\\" end Absyn.IDENT;")).clone())?;
            ()
        },
        Deref @ Absyn::FULLYQUALIFIED { path: p } => {
            let mut s: ArcStr;
            Print::printBuf((literal!("record Absyn.FULLYQUALIFIED path = \\\"")).clone())?;
            printPathAsCorbaString(p.clone())?;
            Print::printBuf((literal!("\\\" end Absyn.FULLYQUALIFIED;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printRedeclareKeywordsAsCorbaString(redeclareKeywords: Absyn::RedeclareKeywords) -> Result<()> {
    let _ = (match redeclareKeywords.clone() {
        Absyn::REDECLARE => {
            Print::printBuf((literal!("record Absyn.REDECLARE end Absyn.REDECLARE;")).clone())?;
            ()
        },
        Absyn::REPLACEABLE => {
            Print::printBuf((literal!("record Absyn.REPLACEABLE end Absyn.REPLACEABLE;")).clone())?;
            ()
        },
        Absyn::REDECLARE_REPLACEABLE => {
            Print::printBuf((literal!("record Absyn.REDECLARE_REPLACEABLE end Absyn.REDECLARE_REPLACEABLE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printRestrictionAsCorbaString(r: Absyn::Restriction) -> Result<()> {
    let _ = (match r.clone() {
        Absyn::R_CLASS => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_CLASS end Absyn.R_CLASS;")).clone())?;
            ()
        },
        Absyn::R_OPTIMIZATION => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_OPTIMIZATION end Absyn.R_OPTIMIZATION;")).clone())?;
            ()
        },
        Absyn::R_MODEL => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_MODEL end Absyn.R_MODEL;")).clone())?;
            ()
        },
        Absyn::R_RECORD => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_RECORD end Absyn.R_RECORD;")).clone())?;
            ()
        },
        Absyn::R_BLOCK => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_BLOCK end Absyn.R_BLOCK;")).clone())?;
            ()
        },
        Absyn::R_CONNECTOR => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_CONNECTOR end Absyn.R_CONNECTOR;")).clone())?;
            ()
        },
        Absyn::R_EXP_CONNECTOR => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_EXP_CONNECTOR end Absyn.R_EXP_CONNECTOR;")).clone())?;
            ()
        },
        Absyn::R_TYPE => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_TYPE end Absyn.R_TYPE;")).clone())?;
            ()
        },
        Absyn::R_PACKAGE => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_PACKAGE end Absyn.R_PACKAGE;")).clone())?;
            ()
        },
        Absyn::R_FUNCTION { functionRestriction: mut functionRestriction } => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            Print::printBuf((literal!("record Absyn.R_FUNCTION functionRestriction = ")).clone())?;
            printFunctionRestrictionAsCorbaString(functionRestriction.clone())?;
            Print::printBuf((literal!("end Absyn.R_FUNCTION;")).clone())?;
            ()
        },
        Absyn::R_OPERATOR => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_OPERATOR end Absyn.R_OPERATOR;")).clone())?;
            ()
        },
        Absyn::R_ENUMERATION => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_ENUMERATION end Absyn.R_ENUMERATION;")).clone())?;
            ()
        },
        Absyn::R_PREDEFINED_INTEGER => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_INTEGER end Absyn.R_PREDEFINED_INTEGER;")).clone())?;
            ()
        },
        Absyn::R_PREDEFINED_REAL => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_REAL end Absyn.R_PREDEFINED_REAL;")).clone())?;
            ()
        },
        Absyn::R_PREDEFINED_STRING => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_STRING end Absyn.R_PREDEFINED_STRING;")).clone())?;
            ()
        },
        Absyn::R_PREDEFINED_BOOLEAN => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_BOOLEAN end Absyn.R_PREDEFINED_BOOLEAN;")).clone())?;
            ()
        },
        Absyn::R_PREDEFINED_CLOCK => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_CLOCK end Absyn.R_PREDEFINED_CLOCK;")).clone())?;
            ()
        },
        Absyn::R_PREDEFINED_ENUMERATION => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_ENUMERATION end Absyn.R_PREDEFINED_ENUMERATION;")).clone())?;
            ()
        },
        Absyn::R_UNIONTYPE => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_UNIONTYPE end Absyn.R_UNIONTYPE;")).clone())?;
            ()
        },
        Absyn::R_METARECORD { index: mut i, name: ref path, .. } => {
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_METARECORD name = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(", index = ")).clone())?;
            Print::printBuf((intString(i.clone())).clone())?;
            Print::printBuf((literal!(" end Absyn.R_METARECORD;")).clone())?;
            ()
        },
        Absyn::R_UNKNOWN => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Print::printBuf((literal!("record Absyn.R_UNKNOWN end Absyn.R_UNKNOWN;")).clone())?;
            ()
        },
        _ => {
            let mut path: Arc<Absyn::Path>;
            let mut i: i32;
            let mut functionRestriction: Absyn::FunctionRestriction;
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printRestrictionAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
    });
    Ok(())
}

fn printStringAsCorbaString(s: ArcStr) -> Result<()> {
    Print::printBuf((literal!("\\\"")).clone())?;
    Print::printBuf((s.clone()).clone())?;
    Print::printBuf((literal!("\\\"")).clone())?;
    Ok(())
}

fn printStringCommentOption(inStringOption: Option<ArcStr>) -> Result<()> {
    let () = (match inStringOption.clone() {
        None => {
            let mut r#str: ArcStr;
            let mut s: ArcStr;
            Print::printBuf((literal!("NONE()")).clone())?;
            ()
        },
        Some(mut s) => {
            let mut r#str: ArcStr;
            r#str = stringAppendList(list![(literal!("SOME(\\\"")).clone(), (s.clone()).clone(), (literal!("\\\")")).clone()]);
            Print::printBuf((r#str.clone()).clone())?;
            ()
        },
    });
    Ok(())
}

fn printSubscriptAsCorbaString(subscript: Arc<Absyn::Subscript>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Absyn::NOSUB => {
            let mut sub: Arc<Absyn::Exp>;
            Print::printBuf((literal!("record Absyn.NOSUB end Absyn.NOSUB;")).clone())?;
            ()
        },
        Deref @ Absyn::SUBSCRIPT { subscript: sub } => {
            Print::printBuf((literal!("record Absyn.SUBSCRIPT subscript = ")).clone())?;
            printExpAsCorbaString(sub.clone())?;
            Print::printBuf((literal!(" end Absyn.SUBSCRIPT;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn printSubscriptStr(inSubscript: Arc<Absyn::Subscript>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ Absyn::NOSUB => literal!(":"),
        Deref @ Absyn::SUBSCRIPT { subscript: e1 } => {
            let mut s: ArcStr;
            s = (printExpStr(e1.clone())?).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printSubscriptsStr(inAbsynSubscriptLst: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ('mc: {
        let __mc_input = inAbsynSubscriptLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    let mut l: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                l => {
                    let mut s: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    s = (printListStr(l.clone(), Arc::new(printSubscriptStr), (literal!(",")).clone())?).clone();
                    s_1 = (stringAppend((literal!("[")).clone(), (s.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!("]")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn printTupleAsCorbaString<Type_a: Clone + 'static, Type_b: Clone + 'static>(inTpl: (Type_a, Type_b), fnA: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>, fnB: Arc<dyn ::std::ops::Fn(Type_b) -> Result<()> + 'static>) -> Result<()> {
    pub type FuncTypeType_a<Type_a: Clone> = fn(Type_a) -> Result<()>;

    pub type FuncTypeType_b<Type_b: Clone> = fn(Type_b) -> Result<()>;

    let _ = (match (inTpl.clone(), fnA.clone(), fnB.clone()) {
        ((mut a, mut b), _, _) => {
            Print::printBuf((literal!("(")).clone())?;
            fnA(a.clone())?;
            Print::printBuf((literal!(",")).clone())?;
            fnB(b.clone())?;
            Print::printBuf((literal!(")")).clone())?;
            ()
        },
    });
    Ok(())
}

fn printTupleExpExpAsCorbaString(tpl: (Arc<Absyn::Exp>, Arc<Absyn::Exp>)) -> Result<()> {
    printTupleAsCorbaString(tpl.clone(), Arc::new(printExpAsCorbaString), Arc::new(printExpAsCorbaString))?;
    Ok(())
}

pub fn printTypeSpec(typeSpec: Arc<Absyn::TypeSpec>) -> Result<()> {
    let mut r#str: ArcStr;
    r#str = (unparseTypeSpec(typeSpec.clone())?).clone();
    println!("{}", (r#str.clone()).clone());
    Ok(())
}

fn printTypeSpecAsCorbaString(typeSpec: Arc<Absyn::TypeSpec>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(typeSpec.clone()) {
        Deref @ Absyn::TPATH { path, arrayDim } => {
            let mut typeSpecs: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>;
            Print::printBuf((literal!("record Absyn.TPATH path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(", arrayDim = ")).clone())?;
            printOption(arrayDim.clone(), Arc::new(printArrayDimAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.TPATH;")).clone())?;
            ()
        },
        Deref @ Absyn::TCOMPLEX { path, typeSpecs, arrayDim } => {
            Print::printBuf((literal!("record Absyn.TPATH path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(", typeSpecs = ")).clone())?;
            printListAsCorbaString(typeSpecs.clone(), Arc::new(printTypeSpecAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", arrayDim = ")).clone())?;
            printOption(arrayDim.clone(), Arc::new(printArrayDimAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.TPATH;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printVariabilityAsCorbaString(var: Absyn::Variability) -> Result<()> {
    let _ = (match var.clone() {
        Absyn::VAR => {
            Print::printBuf((literal!("record Absyn.VAR end Absyn.VAR;")).clone())?;
            ()
        },
        Absyn::DISCRETE => {
            Print::printBuf((literal!("record Absyn.DISCRETE end Absyn.DISCRETE;")).clone())?;
            ()
        },
        Absyn::PARAM => {
            Print::printBuf((literal!("record Absyn.PARAM end Absyn.PARAM;")).clone())?;
            ()
        },
        Absyn::CONST => {
            Print::printBuf((literal!("record Absyn.CONST end Absyn.CONST;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printWithinAsCorbaString(within_: Absyn::Within) -> Result<()> {
    let _ = (match within_.clone() {
        Absyn::WITHIN { path: mut path } => {
            Print::printBuf((literal!("record Absyn.WITHIN path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.WITHIN;")).clone())?;
            ()
        },
        Absyn::TOP => {
            let mut path: Arc<Absyn::Path>;
            Print::printBuf((literal!("record Absyn.TOP end Absyn.TOP;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn priorityBinopLhs(inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp.clone() {
        Absyn::ADD => 5,
        Absyn::SUB => 5,
        Absyn::MUL => 2,
        Absyn::DIV => 2,
        Absyn::POW => 1,
        Absyn::ADD_EW => 5,
        Absyn::SUB_EW => 5,
        Absyn::MUL_EW => 2,
        Absyn::DIV_EW => 2,
        Absyn::POW_EW => 1,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityBinopRhs(inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp.clone() {
        Absyn::ADD => 6,
        Absyn::SUB => 5,
        Absyn::MUL => 2,
        Absyn::DIV => 2,
        Absyn::POW => 1,
        Absyn::ADD_EW => 6,
        Absyn::SUB_EW => 5,
        Absyn::MUL_EW => 3,
        Absyn::DIV_EW => 2,
        Absyn::POW_EW => 1,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityLBinop(inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp.clone() {
        Absyn::AND => 8,
        Absyn::OR => 9,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

pub fn shouldParenthesize(inOperand: Arc<Absyn::Exp>, inOperator: Arc<Absyn::Exp>, inLhs: bool) -> Result<bool> {
    let mut outShouldParenthesize: bool;
    outShouldParenthesize = (::match_deref::match_deref! { match &(inOperand.clone()) {
        Deref @ Absyn::UNARY { .. } => true,
        _ => {
            let mut diff: i32;
            diff = Util::intCompare(expPriority(inOperand.clone(), inLhs.clone())?, expPriority(inOperator.clone(), inLhs.clone())?);
            shouldParenthesize2(diff.clone(), inOperand.clone(), inLhs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outShouldParenthesize)
}

fn shouldParenthesize2(inPrioDiff: i32, inOperand: Arc<Absyn::Exp>, inLhs: bool) -> bool {
    let mut outShouldParenthesize: bool;
    outShouldParenthesize = (match inPrioDiff.clone() {
        1 => true,
        0 => if (inLhs.clone()) {isNonAssociativeExp(inOperand.clone())} else {!(isAssociativeExp(inOperand.clone()))},
        _ => false,
    });
    outShouldParenthesize
}

pub fn shouldSeparateAfterElementArg(args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Arc<metamodelica::List<(Arc<Absyn::ElementArg>, bool)>> {
    let mut outArgs: Arc<metamodelica::List<(Arc<Absyn::ElementArg>, bool)>>;
    let mut numNonComment: i32 = 0;
    let mut cur: i32 = 0;
    let mut b: bool;
    for arg in &*args.clone() {
        numNonComment = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ELEMENTARGCOMMENT { .. } => numNonComment.clone(),
        _ => numNonComment.clone() + 1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outArgs = metamodelica::nil();
    for arg in &*args.clone() {
        b = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ELEMENTARGCOMMENT { .. } => false,
        _ => {
            cur = cur.clone() + 1;
            cur.clone() < numNonComment.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outArgs = cons((arg.clone(), b.clone()), outArgs.clone());
    }
    outArgs = outArgs.clone().reverse();
    outArgs
}

pub fn stdout() -> Result<()> {
    let mut r#str: ArcStr;
    r#str = (Print::getString()?).clone();
    println!("{}", (r#str.clone()).clone());
    Print::clearBuf();
    Ok(())
}

pub fn unparseAlgorithmStr(inAlgorithmItem: Arc<Absyn::AlgorithmItem>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpAlgorithmItem), inAlgorithmItem.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseAlgorithmStrLst(inAlgorithmItems: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, inSeparator: ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    outString = stringDelimitList(List::map(inAlgorithmItems.clone(), Arc::new(unparseAlgorithmStr)), (inSeparator.clone()).clone());
    outString
}

pub fn unparseAnnotation(inAnnotation: Arc<Absyn::Annotation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpAnnotation), inAnnotation.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseAnnotationOption(inAbsynAnnotation: Option<Arc<Absyn::Annotation>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inAbsynAnnotation.clone() {
        Some(mut ann) => unparseAnnotation(ann.clone())?,
        _ => literal!(""),
    })).clone();
    Ok(outString)
}

pub fn unparseClassAttributesStr(inClass: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::CLASS { restriction: r, encapsulatedPrefix: e, finalPrefix: f, partialPrefix: p, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s2_1: ArcStr;
            let mut s3: ArcStr;
            let mut r#str: ArcStr;
            let mut n: ArcStr;
            s1 = (if (p.clone()) {literal!("partial ")} else {literal!("")}).clone();
            s2 = (if (f.clone()) {literal!("final ")} else {literal!("")}).clone();
            s2_1 = (if (e.clone()) {literal!("encapsulated ")} else {literal!("")}).clone();
            s3 = (unparseRestrictionStr(r.clone())?).clone();
            r#str = stringAppendList(list![(s2_1.clone()).clone(), (s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn unparseClassList(inClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString2(Arc::new(AbsynDumpTpl::dump), Absyn::Program { classes: inClasses.clone(), within_: openmodelica_ast::Absyn::Within::TOP }, defaultDumpOptions.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseClassPart(classPart: Arc<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString3(Arc::new(AbsynDumpTpl::dumpClassPart), classPart.clone(), 0, defaultDumpOptions.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseClassStr(inClass: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString3(Arc::new(AbsynDumpTpl::dumpClass), inClass.clone(), (literal!("")).clone(), defaultDumpOptions.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseCommentOption(inComment: Option<Arc<Absyn::Comment>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpCommentOpt), inComment.clone())?).clone();
    Ok(outString)
}

pub fn unparseComponentCondition(inComponentCondition: Option<Arc<Absyn::Exp>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpComponentCondition), inComponentCondition.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseDirectionSymbolStr(inDirection: Absyn::Direction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inDirection.clone() {
        Absyn::BIDIR => literal!(""),
        Absyn::INPUT => literal!("input "),
        Absyn::OUTPUT => literal!("output "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseEachStr(inEach: Absyn::Each) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inEach.clone() {
        Absyn::EACH => literal!("each "),
        Absyn::NON_EACH => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseElementArgStr(inElementArg: Arc<Absyn::ElementArg>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpElementArg), inElementArg.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseElementItemStr(inElementItem: Arc<Absyn::ElementItem>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString2(Arc::new(AbsynDumpTpl::dumpElementItem), inElementItem.clone(), defaultDumpOptions.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseEquationItemStr(inEquation: Arc<Absyn::EquationItem>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpEquationItem), inEquation.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseEquationItemStrLst(inEquationItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, inSeparator: ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    outString = stringDelimitList(List::map(inEquationItems.clone(), Arc::new(unparseEquationItemStr)), (inSeparator.clone()).clone());
    outString
}

pub fn unparseEquationStr(inEquation: Arc<Absyn::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpEquation), inEquation.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

fn unparseGroupImport(gimp: Absyn::GroupImport) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match gimp.clone() {
        Absyn::GROUP_IMPORT_NAME { name: mut name } => name.clone(),
        Absyn::GROUP_IMPORT_RENAME { name: mut name, rename: mut rename } => { let mut __mm_s = String::new(); __mm_s.push_str(&*rename.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn unparseImportStr(inImport: Absyn::Import) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpImport), inImport.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseInnerOuterStr(inInnerOuter: Absyn::InnerOuter) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inInnerOuter.clone() {
        Absyn::INNER => literal!("inner "),
        Absyn::OUTER => literal!("outer "),
        Absyn::INNER_OUTER => literal!("inner outer "),
        Absyn::NOT_INNER_OUTER => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseModificationStr(inModification: Arc<Absyn::Modification>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpModification), inModification.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseParallelismSymbolStr(inParallelism: Absyn::Parallelism) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inParallelism.clone() {
        Absyn::NON_PARALLEL => literal!(""),
        Absyn::PARGLOBAL => literal!("parglobal "),
        Absyn::PARLOCAL => literal!("parlocal "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseRestrictionStr(inRestriction: Absyn::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpRestriction), inRestriction.clone())?).clone();
    Ok(outString)
}

pub fn unparseStr(inProgram: Absyn::Program, markup: bool, options: DumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString2(Arc::new(AbsynDumpTpl::dump), inProgram.clone(), options.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseTypeSpec(inTypeSpec: Arc<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpTypeSpec), inTypeSpec.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

fn unparseVariabilitySymbolStr(inVariability: Absyn::Variability) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inVariability.clone() {
        Absyn::VAR => literal!(""),
        Absyn::DISCRETE => literal!("discrete "),
        Absyn::PARAM => literal!("parameter "),
        Absyn::CONST => literal!("constant "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseWithin(inWithin: Absyn::Within) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut status: bool;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpWithin), inWithin.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn writePath(file: File::File, path: Arc<Absyn::Path>, escape: Escape, delimiter: ArcStr, initialDot: bool) -> Result<()> {
    let mut p: Arc<Absyn::Path> = path.clone();
    while true {
        p = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::IDENT { .. } => {
            File::writeEscape(file.clone(), (var_field!((*p).name, Absyn::Path::IDENT).clone()).clone(), escape.clone());
            return Ok(());
            bail!("fail")
        },
        Deref @ Absyn::QUALIFIED { .. } => {
            File::writeEscape(file.clone(), (var_field!((*p).name, Absyn::Path::QUALIFIED).clone()).clone(), escape.clone());
            File::writeEscape(file.clone(), (delimiter.clone()).clone(), escape.clone());
            var_field!((*p).path, Absyn::Path::QUALIFIED).clone()
        },
        Deref @ Absyn::FULLYQUALIFIED { .. } => {
            if initialDot.clone() {
                File::writeEscape(file.clone(), (delimiter.clone()).clone(), escape.clone());
            }
            var_field!((*p).path, Absyn::Path::FULLYQUALIFIED).clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    Ok(())
}

