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

use crate::ComponentReference;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionDump;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTpl;
use openmodelica_frontend_dump::DAEDumpTypes::*;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::Graphviz;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::Print;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub fn functionNameStr(mut inElement: DAE::Function) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ('mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Function::FUNCTION { path: ref fpath, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut res: ArcStr = res.clone();
            res = AbsynUtil::pathStringNoQual(fpath.clone(), (literal!(".")).clone(), true, false)?;
            Ok(res.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Function::RECORD_CONSTRUCTOR { path: ref fpath, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut res: ArcStr = res.clone();
            res = AbsynUtil::pathStringNoQual(fpath.clone(), (literal!(".")).clone(), true, false)?;
            Ok(res.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!(""))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(res)
}

fn sortFunctions(mut funcs: Arc<metamodelica::List<DAE::Function>>) -> Result<Arc<metamodelica::List<DAE::Function>>> {
    let mut sortedFuncs: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
    sortedFuncs = List::sort(funcs.clone(), Arc::new(funcGreaterThan))?;
    Ok(sortedFuncs)
}

fn funcGreaterThan(mut func1: DAE::Function, mut func2: DAE::Function) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (func1.clone(), func2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut res: bool = res.clone();
            res = stringCompare((functionNameStr(func1.clone())?).clone(), (functionNameStr(func2.clone())?).clone()) > 0;
            Ok(res.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn dumpOperatorString(mut op: DAE::Operator) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match op.clone() {
        DAE::Operator::ADD { .. } => {
            literal!(" ADD ")
        },
        DAE::Operator::SUB { .. } => {
            literal!(" SUB ")
        },
        DAE::Operator::MUL { .. } => {
            literal!(" MUL ")
        },
        DAE::Operator::DIV { .. } => {
            literal!(" DIV ")
        },
        DAE::Operator::POW { .. } => {
            literal!(" POW ")
        },
        DAE::Operator::UMINUS { .. } => {
            literal!(" UMINUS ")
        },
        DAE::Operator::UMINUS_ARR { .. } => {
            literal!(" UMINUS_ARR ")
        },
        DAE::Operator::ADD_ARR { .. } => {
            literal!(" ADD_ARR ")
        },
        DAE::Operator::SUB_ARR { .. } => {
            literal!(" SUB_ARR ")
        },
        DAE::Operator::MUL_ARR { .. } => {
            literal!(" MUL_ARR ")
        },
        DAE::Operator::DIV_ARR { .. } => {
            literal!(" DIV_ARR ")
        },
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => {
            literal!(" MUL_ARRAY_SCALAR ")
        },
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => {
            literal!(" ADD_ARRAY_SCALAR ")
        },
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => {
            literal!(" SUB_SCALAR_ARRAY ")
        },
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => {
            literal!(" MUL_SCALAR_PRODUCT ")
        },
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => {
            literal!(" MUL_MATRIX_PRODUCT ")
        },
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => {
            literal!(" DIV_ARRAY_SCALAR ")
        },
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => {
            literal!(" DIV_SCALAR_ARRAY ")
        },
        DAE::Operator::POW_ARRAY_SCALAR { .. } => {
            literal!(" POW_ARRAY_SCALAR ")
        },
        DAE::Operator::POW_SCALAR_ARRAY { .. } => {
            literal!(" POW_SCALAR_ARRAY ")
        },
        DAE::Operator::POW_ARR { .. } => {
            literal!(" POW_ARR ")
        },
        DAE::Operator::POW_ARR2 { .. } => {
            literal!(" POW_ARR2 ")
        },
        DAE::Operator::OR { ty: _ } => {
            literal!(" OR ")
        },
        DAE::Operator::AND { ty: _ } => {
            literal!(" AND ")
        },
        DAE::Operator::NOT { ty: _ } => {
            literal!(" NOT ")
        },
        DAE::Operator::LESSEQ { .. } => {
            literal!(" LESSEQ ")
        },
        DAE::Operator::GREATER { .. } => {
            literal!(" GREATER ")
        },
        DAE::Operator::GREATEREQ { .. } => {
            literal!(" GREATEREQ ")
        },
        DAE::Operator::LESS { .. } => {
            literal!(" LESS ")
        },
        DAE::Operator::EQUAL { .. } => {
            literal!(" EQUAL ")
        },
        DAE::Operator::NEQUAL { .. } => {
            literal!(" NEQUAL ")
        },
        DAE::Operator::USERDEFINED { fqName: ref p } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" Userdefined:")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!(" --UNDEFINED-- ")
        },
    })).clone();
    Ok(r#str)
}

pub fn dumpOperatorSymbol(mut op: DAE::Operator) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match op.clone() {
        DAE::Operator::ADD { ty: _ } => {
            literal!(" + ")
        },
        DAE::Operator::SUB { ty: _ } => {
            literal!(" - ")
        },
        DAE::Operator::MUL { ty: _ } => {
            literal!(" .* ")
        },
        DAE::Operator::DIV { ty: _ } => {
            literal!(" / ")
        },
        DAE::Operator::POW { ty: _ } => {
            literal!(" ^ ")
        },
        DAE::Operator::UMINUS { ty: _ } => {
            literal!(" - ")
        },
        DAE::Operator::UMINUS_ARR { ty: _ } => {
            literal!(" - ")
        },
        DAE::Operator::ADD_ARR { ty: _ } => {
            literal!(" + ")
        },
        DAE::Operator::SUB_ARR { ty: _ } => {
            literal!(" - ")
        },
        DAE::Operator::MUL_ARR { ty: _ } => {
            literal!(" .* ")
        },
        DAE::Operator::DIV_ARR { ty: _ } => {
            literal!(" ./ ")
        },
        DAE::Operator::MUL_ARRAY_SCALAR { ty: _ } => {
            literal!(" * ")
        },
        DAE::Operator::ADD_ARRAY_SCALAR { ty: _ } => {
            literal!(" .+ ")
        },
        DAE::Operator::SUB_SCALAR_ARRAY { ty: _ } => {
            literal!(" .- ")
        },
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ } => {
            literal!(" * ")
        },
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ } => {
            literal!(" * ")
        },
        DAE::Operator::DIV_ARRAY_SCALAR { ty: _ } => {
            literal!(" / ")
        },
        DAE::Operator::DIV_SCALAR_ARRAY { ty: _ } => {
            literal!(" ./ ")
        },
        DAE::Operator::POW_ARRAY_SCALAR { ty: _ } => {
            literal!(" .^ ")
        },
        DAE::Operator::POW_SCALAR_ARRAY { ty: _ } => {
            literal!(" .^ ")
        },
        DAE::Operator::POW_ARR { ty: _ } => {
            literal!(" ^ ")
        },
        DAE::Operator::POW_ARR2 { ty: _ } => {
            literal!(" .^ ")
        },
        DAE::Operator::OR { ty: _ } => {
            literal!(" or ")
        },
        DAE::Operator::AND { ty: _ } => {
            literal!(" and ")
        },
        DAE::Operator::NOT { ty: _ } => {
            literal!(" not ")
        },
        DAE::Operator::LESSEQ { ty: _ } => {
            literal!(" <= ")
        },
        DAE::Operator::GREATER { ty: _ } => {
            literal!(" > ")
        },
        DAE::Operator::GREATEREQ { ty: _ } => {
            literal!(" >= ")
        },
        DAE::Operator::LESS { ty: _ } => {
            literal!(" < ")
        },
        DAE::Operator::EQUAL { ty: _ } => {
            literal!(" == ")
        },
        DAE::Operator::NEQUAL { ty: _ } => {
            literal!(" <> ")
        },
        DAE::Operator::USERDEFINED { fqName: ref p } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" Userdefined:")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!(" --UNDEFINED-- ")
        },
    })).clone();
    Ok(r#str)
}

fn dumpStartValue(mut inStartValue: Option<Arc<DAE::Exp>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inStartValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(e) => {
                    Print::printBuf((literal!("(start=")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dumpStartValueStr(mut inStartValue: Option<Arc<DAE::Exp>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inStartValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(e) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    s = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    res = stringAppendList(list![(literal!("(start=")).clone(), (s.clone()).clone(), (literal!(")")).clone()]);
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn dumpExtDeclStr(mut inExternalDecl: DAE::ExternalDecl) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inExternalDecl.clone() {
        DAE::ExternalDecl { language: mut lang, returnArg: mut retty, args: ref extargs, name: mut id, .. } => {
            let mut extargsstr: ArcStr = arcstr::literal!("");
            let mut rettystr: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            extargsstr = (List::toString(extargs.clone(), Arc::new(dumpExtArgStr), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), true, 0)?).clone();
            rettystr = (dumpExtArgStr(retty.clone())?).clone();
            rettystr = (if (stringEq((rettystr.clone()).clone(), (literal!("")).clone())) {rettystr.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*rettystr.clone()); __mm_s.push_str(&*literal!(" = ")); ArcStr::from(__mm_s) }}).clone();
            r#str = stringAppendList(list![(literal!("external \"")).clone(), (lang.clone()).clone(), (literal!("\" ")).clone(), (rettystr.clone()).clone(), (id.clone()).clone(), (literal!("(")).clone(), (extargsstr.clone()).clone(), (literal!(");")).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn dumpExtArgStr(mut inExtArg: DAE::ExtArg) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inExtArg.clone() {
        DAE::ExtArg::NOEXTARG => {
            literal!("")
        },
        DAE::ExtArg::EXTARG { componentRef: ref cr, .. } => {
            let mut crstr: ArcStr = arcstr::literal!("");
            crstr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            crstr.clone()
        },
        DAE::ExtArg::EXTARGEXP { exp: mut exp, .. } => {
            let mut crstr: ArcStr = arcstr::literal!("");
            crstr = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            crstr.clone()
        },
        DAE::ExtArg::EXTARGSIZE { exp: ref dim, componentRef: ref cr, .. } => {
            let mut crstr: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut dimstr: ArcStr = arcstr::literal!("");
            crstr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            dimstr = (ExpressionBasics::printExpStr(dim.clone())?).clone();
            r#str = stringAppendList(list![(literal!("size(")).clone(), (crstr.clone()).clone(), (literal!(", ")).clone(), (dimstr.clone()).clone(), (literal!(")")).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn dumpCompElement(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMP { comment: c, dAElist: l, ident: n, .. } => {
                    Print::printBuf((literal!("class ")).clone())?;
                    Print::printBuf((n.clone()).clone())?;
                    dumpCommentOption(c.clone())?;
                    Print::printBuf((literal!("\n")).clone())?;
                    dumpElements(l.clone())?;
                    Print::printBuf((literal!("end ")).clone())?;
                    Print::printBuf((n.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dumpElements(mut l: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<()> {
    dumpVars(l.clone(), false)?;
    List::map_0(l.clone(), Arc::new(dumpExtObjectClass));
    Print::printBuf((literal!("initial equation\n")).clone())?;
    List::map_0(l.clone(), Arc::new(dumpInitialEquation));
    Print::printBuf((literal!("equation\n")).clone())?;
    List::map_0(l.clone(), Arc::new(dumpEquation));
    List::map_0(l.clone(), Arc::new(dumpInitialAlgorithm));
    List::map_0(l.clone(), Arc::new(dumpAlgorithm));
    List::map_0(l.clone(), Arc::new(dumpCompElement));
    Ok(())
}

pub fn dumpFunctionElements(mut l: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<()> {
    dumpVars(l.clone(), true)?;
    List::map_0(l.clone(), Arc::new(dumpAlgorithm));
    Ok(())
}

fn dumpVars(mut lst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut printTypeDimension: bool) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut myStream: IOStream::IOStream;
    myStream = IOStream::create((literal!("")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    myStream = dumpVarsStream(lst.clone(), printTypeDimension.clone(), myStream.clone())?;
    r#str = (IOStream::string(myStream.clone())?).clone();
    Print::printBuf((r#str.clone()).clone())?;
    Ok(())
}

fn dumpKind(mut inVarKind: DAE::VarKind) -> Result<()> {
    let _ = (match inVarKind.clone() {
        DAE::VarKind::CONST => {
            Print::printBuf((literal!(" constant  ")).clone())?;
            ()
        },
        DAE::VarKind::PARAM => {
            Print::printBuf((literal!(" parameter ")).clone())?;
            ()
        },
        DAE::VarKind::DISCRETE => {
            Print::printBuf((literal!(" discrete  ")).clone())?;
            ()
        },
        DAE::VarKind::VARIABLE => {
            Print::printBuf((literal!("           ")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn dumpKindStr(mut inVarKind: DAE::VarKind) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVarKind.clone() {
        DAE::VarKind::CONST => literal!("constant "),
        DAE::VarKind::PARAM => literal!("parameter "),
        DAE::VarKind::DISCRETE => literal!("discrete "),
        DAE::VarKind::VARIABLE => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn dumpDirection(mut inVarDirection: DAE::VarDirection) -> Result<()> {
    let _ = (match inVarDirection.clone() {
        DAE::VarDirection::INPUT => {
            Print::printBuf((literal!(" input  ")).clone())?;
            ()
        },
        DAE::VarDirection::OUTPUT => {
            Print::printBuf((literal!(" output ")).clone())?;
            ()
        },
        DAE::VarDirection::BIDIR => {
            Print::printBuf((literal!("        ")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn dumpParallelism(mut inVarParallelism: DAE::VarParallelism) -> Result<()> {
    let _ = (match inVarParallelism.clone() {
        DAE::VarParallelism::NON_PARALLEL => {
            Print::printBuf((literal!("        ")).clone())?;
            ()
        },
        DAE::VarParallelism::PARGLOBAL => {
            Print::printBuf((literal!(" parglobal ")).clone())?;
            ()
        },
        DAE::VarParallelism::PARLOCAL => {
            Print::printBuf((literal!(" parlocal ")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn dumpDirectionStr(mut inVarDirection: DAE::VarDirection) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVarDirection.clone() {
        DAE::VarDirection::INPUT => literal!("input "),
        DAE::VarDirection::OUTPUT => literal!("output "),
        DAE::VarDirection::BIDIR => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn dumpStateSelectStr(mut inStateSelect: DAE::StateSelect) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inStateSelect.clone() {
        DAE::StateSelect::NEVER => literal!("StateSelect.never"),
        DAE::StateSelect::AVOID => literal!("StateSelect.avoid"),
        DAE::StateSelect::PREFER => literal!("StateSelect.prefer"),
        DAE::StateSelect::ALWAYS => literal!("StateSelect.always"),
        DAE::StateSelect::DEFAULT => literal!("StateSelect.default"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn dumpUncertaintyStr(mut uncertainty: DAE::Uncertainty) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((match uncertainty.clone() {
        DAE::Uncertainty::GIVEN => literal!("Uncertainty.given"),
        DAE::Uncertainty::SOUGHT => literal!("Uncertainty.sought"),
        DAE::Uncertainty::REFINE => literal!("Uncertainty.refine"),
        DAE::Uncertainty::PROPAGATE => literal!("Uncertainty.propagate"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(out)
}

fn dumpDistributionStr(mut distribution: Arc<DAE::Distribution>) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ((::match_deref::match_deref! { match &(distribution.clone()) {
        Deref @ DAE::Distribution { paramNames, params, name } => {
            let mut name_str: ArcStr = arcstr::literal!("");
            let mut params_str: ArcStr = arcstr::literal!("");
            let mut paramNames_str: ArcStr = arcstr::literal!("");
            name_str = (ExpressionBasics::printExpStr(name.clone())?).clone();
            params_str = (ExpressionBasics::printExpStr(params.clone())?).clone();
            paramNames_str = (ExpressionBasics::printExpStr(paramNames.clone())?).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Distribution(name = ")); __mm_s.push_str(&*name_str.clone()); __mm_s.push_str(&*literal!(", params = ")); __mm_s.push_str(&*params_str.clone()); __mm_s.push_str(&*literal!(", paramNames= ")); __mm_s.push_str(&*paramNames_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(out)
}

pub fn dumpVariableAttributes(mut attr: Option<Arc<DAE::VariableAttributes>>) -> Result<()> {
    let mut res: ArcStr = arcstr::literal!("");
    res = (dumpVariableAttributesStr(attr.clone())?).clone();
    Print::printBuf((res.clone()).clone())?;
    Ok(())
}

pub fn dumpVariableAttributesStr(mut inVariableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inVariableAttributesOption.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quant, unit, displayUnit, min, max, start: initialExp, fixed, nominal, stateSelectOption: stateSel, uncertainOption: uncertainty, distributionOption: dist, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr = arcstr::literal!("");
                    let mut unit_str: ArcStr = arcstr::literal!("");
                    let mut displayUnit_str: ArcStr = arcstr::literal!("");
                    let mut stateSel_str: ArcStr = arcstr::literal!("");
                    let mut min_str: ArcStr = arcstr::literal!("");
                    let mut max_str: ArcStr = arcstr::literal!("");
                    let mut nominal_str: ArcStr = arcstr::literal!("");
                    let mut initial_str: ArcStr = arcstr::literal!("");
                    let mut fixed_str: ArcStr = arcstr::literal!("");
                    let mut uncertainty_str: ArcStr = arcstr::literal!("");
                    let mut dist_str: ArcStr = arcstr::literal!("");
                    let mut res_1: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut startOriginStr: ArcStr = arcstr::literal!("");
                    quantity = (getOptionWithConcatStr(quant.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("quantity = ")).clone())?).clone();
                    unit_str = (getOptionWithConcatStr(unit.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("unit = ")).clone())?).clone();
                    displayUnit_str = (getOptionWithConcatStr(displayUnit.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("displayUnit = ")).clone())?).clone();
                    stateSel_str = (getOptionWithConcatStr(stateSel.clone(), Arc::new(dumpStateSelectStr), (literal!("stateSelect = ")).clone())?).clone();
                    min_str = (getOptionWithConcatStr(min.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("min = ")).clone())?).clone();
                    max_str = (getOptionWithConcatStr(max.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("max = ")).clone())?).clone();
                    nominal_str = (getOptionWithConcatStr(nominal.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("nominal = ")).clone())?).clone();
                    initial_str = (getOptionWithConcatStr(initialExp.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("start = ")).clone())?).clone();
                    fixed_str = (getOptionWithConcatStr(fixed.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("fixed = ")).clone())?).clone();
                    uncertainty_str = (getOptionWithConcatStr(uncertainty.clone(), Arc::new(dumpUncertaintyStr), (literal!("uncertainty = ")).clone())?).clone();
                    dist_str = (getOptionWithConcatStr(dist.clone(), Arc::new(dumpDistributionStr), (literal!("distribution = ")).clone())?).clone();
                    startOriginStr = (getStartOrigin(startOrigin.clone())?).clone();
                    res_1 = (Util::stringDelimitListNonEmptyElts(list![(quantity.clone()).clone(), (unit_str.clone()).clone(), (displayUnit_str.clone()).clone(), (min_str.clone()).clone(), (max_str.clone()).clone(), (initial_str.clone()).clone(), (fixed_str.clone()).clone(), (nominal_str.clone()).clone(), (stateSel_str.clone()).clone(), (uncertainty_str.clone()).clone(), (dist_str.clone()).clone(), (startOriginStr.clone()).clone()], (literal!(", ")).clone())).clone();
                    res = (if (stringEmpty((res_1.clone()).clone())) {literal!("")} else {stringAppendList(list![(literal!("(")).clone(), (res_1.clone()).clone(), (literal!(")")).clone()])}).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: quant, min, max, start: initialExp, fixed, uncertainOption: uncertainty, distributionOption: dist, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr = arcstr::literal!("");
                    let mut min_str: ArcStr = arcstr::literal!("");
                    let mut max_str: ArcStr = arcstr::literal!("");
                    let mut initial_str: ArcStr = arcstr::literal!("");
                    let mut fixed_str: ArcStr = arcstr::literal!("");
                    let mut uncertainty_str: ArcStr = arcstr::literal!("");
                    let mut dist_str: ArcStr = arcstr::literal!("");
                    let mut res_1: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut startOriginStr: ArcStr = arcstr::literal!("");
                    quantity = (getOptionWithConcatStr(quant.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("quantity = ")).clone())?).clone();
                    min_str = (getOptionWithConcatStr(min.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("min = ")).clone())?).clone();
                    max_str = (getOptionWithConcatStr(max.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("max = ")).clone())?).clone();
                    initial_str = (getOptionWithConcatStr(initialExp.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("start = ")).clone())?).clone();
                    fixed_str = (getOptionWithConcatStr(fixed.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("fixed = ")).clone())?).clone();
                    uncertainty_str = (getOptionWithConcatStr(uncertainty.clone(), Arc::new(dumpUncertaintyStr), (literal!("uncertainty = ")).clone())?).clone();
                    dist_str = (getOptionWithConcatStr(dist.clone(), Arc::new(dumpDistributionStr), (literal!("distribution = ")).clone())?).clone();
                    startOriginStr = (getStartOrigin(startOrigin.clone())?).clone();
                    res_1 = (Util::stringDelimitListNonEmptyElts(list![(quantity.clone()).clone(), (min_str.clone()).clone(), (max_str.clone()).clone(), (initial_str.clone()).clone(), (fixed_str.clone()).clone(), (uncertainty_str.clone()).clone(), (dist_str.clone()).clone(), (startOriginStr.clone()).clone()], (literal!(", ")).clone())).clone();
                    res = (if (stringEmpty((res_1.clone()).clone())) {literal!("")} else {stringAppendList(list![(literal!("(")).clone(), (res_1.clone()).clone(), (literal!(")")).clone()])}).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quant, start: initialExp, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr = arcstr::literal!("");
                    let mut initial_str: ArcStr = arcstr::literal!("");
                    let mut fixed_str: ArcStr = arcstr::literal!("");
                    let mut res_1: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut startOriginStr: ArcStr = arcstr::literal!("");
                    quantity = (getOptionWithConcatStr(quant.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("quantity = ")).clone())?).clone();
                    initial_str = (getOptionWithConcatStr(initialExp.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("start = ")).clone())?).clone();
                    fixed_str = (getOptionWithConcatStr(fixed.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("fixed = ")).clone())?).clone();
                    startOriginStr = (getStartOrigin(startOrigin.clone())?).clone();
                    res_1 = (Util::stringDelimitListNonEmptyElts(list![(quantity.clone()).clone(), (initial_str.clone()).clone(), (fixed_str.clone()).clone(), (startOriginStr.clone()).clone()], (literal!(", ")).clone())).clone();
                    res = (if (stringEmpty((res_1.clone()).clone())) {literal!("")} else {stringAppendList(list![(literal!("(")).clone(), (res_1.clone()).clone(), (literal!(")")).clone()])}).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quant, start: initialExp, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr = arcstr::literal!("");
                    let mut initial_str: ArcStr = arcstr::literal!("");
                    let mut fixed_str: ArcStr = arcstr::literal!("");
                    let mut res_1: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut startOriginStr: ArcStr = arcstr::literal!("");
                    quantity = (getOptionWithConcatStr(quant.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("quantity = ")).clone())?).clone();
                    initial_str = (getOptionWithConcatStr(initialExp.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("start = ")).clone())?).clone();
                    fixed_str = (getOptionWithConcatStr(fixed.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("fixed = ")).clone())?).clone();
                    startOriginStr = (getStartOrigin(startOrigin.clone())?).clone();
                    res_1 = (Util::stringDelimitListNonEmptyElts(list![(quantity.clone()).clone(), (initial_str.clone()).clone(), (fixed_str.clone()).clone(), (startOriginStr.clone()).clone()], (literal!(", ")).clone())).clone();
                    res = (if (stringEmpty((res_1.clone()).clone())) {literal!("")} else {stringAppendList(list![(literal!("(")).clone(), (res_1.clone()).clone(), (literal!(")")).clone()])}).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quant, min, max, start: initialExp, fixed, equationBound: _, isProtected: _, finalPrefix: _, startOrigin }) => {
                    let mut quantity: ArcStr = arcstr::literal!("");
                    let mut min_str: ArcStr = arcstr::literal!("");
                    let mut max_str: ArcStr = arcstr::literal!("");
                    let mut initial_str: ArcStr = arcstr::literal!("");
                    let mut fixed_str: ArcStr = arcstr::literal!("");
                    let mut res_1: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut startOriginStr: ArcStr = arcstr::literal!("");
                    quantity = (getOptionWithConcatStr(quant.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("quantity = ")).clone())?).clone();
                    min_str = (getOptionWithConcatStr(min.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("min = ")).clone())?).clone();
                    max_str = (getOptionWithConcatStr(max.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("max = ")).clone())?).clone();
                    initial_str = (getOptionWithConcatStr(initialExp.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("start = ")).clone())?).clone();
                    fixed_str = (getOptionWithConcatStr(fixed.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("fixed = ")).clone())?).clone();
                    startOriginStr = (getStartOrigin(startOrigin.clone())?).clone();
                    res_1 = (Util::stringDelimitListNonEmptyElts(list![(quantity.clone()).clone(), (min_str.clone()).clone(), (max_str.clone()).clone(), (initial_str.clone()).clone(), (fixed_str.clone()).clone(), (startOriginStr.clone()).clone()], (literal!(", ")).clone())).clone();
                    res = (if (stringEmpty((res_1.clone()).clone())) {literal!("")} else {stringAppendList(list![(literal!("(")).clone(), (res_1.clone()).clone(), (literal!(")")).clone()])}).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("(unknown VariableAttributes)"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn getStartOrigin(mut inStartOrigin: Option<Arc<DAE::Exp>>) -> Result<ArcStr> {
    let mut outStartOrigin: ArcStr = arcstr::literal!("");
    outStartOrigin = ((::match_deref::match_deref! { match &(inStartOrigin.clone()) {
        None => {
            literal!("")
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            if Flags::isSet(Flags::SHOW_START_ORIGIN.clone())? {
                r#str = (getOptionWithConcatStr(inStartOrigin.clone(), Arc::new(ExpressionBasics::printExpStr), (literal!("startOrigin = ")).clone())?).clone();
            } else {
                r#str = (literal!("")).clone();
            }
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outStartOrigin)
}

fn dumpVarVisibilityStr(mut prot: DAE::VarVisibility) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match prot.clone() {
        DAE::VarVisibility::PUBLIC => literal!(""),
        DAE::VarVisibility::PROTECTED => literal!("protected "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn dumpVarParallelismStr(mut inVarParallelism: DAE::VarParallelism) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVarParallelism.clone() {
        DAE::VarParallelism::NON_PARALLEL => literal!(""),
        DAE::VarParallelism::PARGLOBAL => literal!("parglobal "),
        DAE::VarParallelism::PARLOCAL => literal!("parlocal "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn dumpCommentOption(mut comment: Option<Arc<SCode::Comment>>) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (dumpCommentAnnotationStr(comment.clone())?).clone();
    Print::printBuf((r#str.clone()).clone())?;
    Ok(())
}

fn dumpEquation(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { source: src, scalar: e2, exp: e1 } => {
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("  ")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    Print::printBuf((sourceStr.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { source: src, cr2, cr1 } => {
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("  ")).clone())?;
                    ComponentReference::printComponentRef(cr1.clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    ComponentReference::printComponentRef(cr2.clone())?;
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    Print::printBuf((sourceStr.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { source: src, array: e2, exp: e1, .. } => {
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("  ")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    Print::printBuf((sourceStr.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { source: src, rhs: e2, lhs: e1 } => {
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("  ")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    Print::printBuf((sourceStr.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { source: src, exp: e, componentRef: c } => {
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("  ")).clone())?;
                    ComponentReference::printComponentRef(c.clone())?;
                    Print::printBuf((literal!(" ::= ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    Print::printBuf((sourceStr.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { source: src, message: e2, condition: e1, .. } => {
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("assert(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(") ")).clone())?;
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    Print::printBuf((sourceStr.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { source: src, exp: e1 } => {
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    ExpressionDump::printExp(e1.clone())?;
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    Print::printBuf((sourceStr.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf((literal!("/* FIXME: UNHANDLED_EQUATION in DAEDump.dumpEquation */;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpInitialEquation(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALEQUATION { exp2: e2, exp1: e1, .. } => {
                    Print::printBuf((literal!("  ")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALDEFINE { exp: e, componentRef: c, .. } => {
                    Print::printBuf((literal!("  ")).clone())?;
                    ComponentReference::printComponentRef(c.clone())?;
                    Print::printBuf((literal!(" ::= ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { array: e2, exp: e1, .. } => {
                    Print::printBuf((literal!("  ")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { rhs: e2, lhs: e1, .. } => {
                    Print::printBuf((literal!("  ")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(" = ")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_IF_EQUATION { equations3: xs2, equations2: Deref @ metamodelica::List::Cons { head: xs1, tail: trueBranches }, condition1: Deref @ metamodelica::List::Cons { head: e, tail: conds }, .. } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut r#str: IOStream::IOStream;
                    Print::printBuf((literal!("  if ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf((literal!(" then\n")).clone())?;
                    List::map_0(xs1.clone(), Arc::new(dumpInitialEquation));
                    r#str = dumpIfEquationsStream(conds.clone(), trueBranches.clone(), IOStream::emptyStreamOfTypeList.clone())?;
                    s = (IOStream::string(r#str.clone())?).clone();
                    Print::printBuf((s.clone()).clone())?;
                    Print::printBuf((literal!("  else\n")).clone())?;
                    List::map_0(xs2.clone(), Arc::new(dumpInitialEquation));
                    Print::printBuf((literal!("end if;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ASSERT { source: src, message: e2, condition: e1, .. } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    s = stringAppendList(list![(literal!("  assert(")).clone(), (s1.clone()).clone(), (literal!(",")).clone(), (s2.clone()).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_TERMINATE { source: src, message: e1 } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s = stringAppendList(list![(literal!("  terminate(")).clone(), (s1.clone()).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Print::printBuf((s.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_NORETCALL { exp: e1, .. } => {
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dumpEquationStr(mut inElement: Arc<DAE::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { source: src, scalar: e2, exp: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { source: src, cr2, cr1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ComponentReferenceBasics::printComponentRefStr(cr1.clone())?).clone();
                    s2 = (ComponentReferenceBasics::printComponentRefStr(cr2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { source: src, array: e2, exp: e1, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { source: src, rhs: e2, lhs: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { source: src, exp: e, componentRef: c } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    s2 = (stringAppend((literal!("  ")).clone(), (s1.clone()).clone())).clone();
                    s3 = (stringAppend((literal!(" ::= ")).clone(), (s2.clone()).clone())).clone();
                    s4 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    s5 = (stringAppend((s3.clone()).clone(), (s4.clone()).clone())).clone();
                    r#str = (stringAppend((s5.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { source: src, message: e2, condition: e1, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  assert(")).clone(), (s1.clone()).clone(), (literal!(",")).clone(), (s2.clone()).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::TERMINATE { source: src, message: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  terminate(")).clone(), (s1.clone()).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { source: src, exp: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  ")).clone(), (s1.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("#UNKNOWN_EQUATION#"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn dumpAlgorithm(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
                    Print::printBuf((literal!("algorithm\n")).clone())?;
                    Dump::printList(stmts.clone(), Arc::new(ppStatement), (literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpInitialAlgorithm(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
                    Print::printBuf((literal!("initial algorithm\n")).clone())?;
                    Dump::printList(stmts.clone(), Arc::new(ppStatement), (literal!("")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpExtObjectClass(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EXTOBJECTCLASS { path: fpath, .. } => {
                    let mut fstr: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("class ")).clone())?;
                    fstr = (AbsynUtil::pathString(fpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Print::printBuf((fstr.clone()).clone())?;
                    Print::printBuf((literal!("\n extends ExternalObject;\n")).clone())?;
                    Print::printBuf((literal!("end ")).clone())?;
                    Print::printBuf((fstr.clone()).clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn derivativeCondStr(mut dc: DAE::derivativeCond) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match dc.clone() {
        DAE::derivativeCond::NO_DERIVATIVE { binding: ref e } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("noDerivative(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        DAE::derivativeCond::ZERO_DERIVATIVE => {
            literal!("zeroDerivative")
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

fn dumpFunction(mut inElement: DAE::Function) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { comment: c, isImpure, type_: t, functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, path: fpath, .. } => {
                    let mut fstr: ArcStr = arcstr::literal!("");
                    let mut parallelism_str: ArcStr = arcstr::literal!("");
                    let mut impureStr: ArcStr = arcstr::literal!("");
                    let mut typeStr: ArcStr = arcstr::literal!("");
                    typeStr = (TypesDump::printTypeStr(t.clone())?).clone();
                    Print::printBuf((typeStr.clone()).clone())?;
                    parallelism_str = (dumpParallelismStr(t.clone())).clone();
                    Print::printBuf((parallelism_str.clone()).clone())?;
                    impureStr = (if (isImpure.clone()) {literal!("impure ")} else {literal!("")}).clone();
                    Print::printBuf((impureStr.clone()).clone())?;
                    Print::printBuf((literal!("function ")).clone())?;
                    fstr = AbsynUtil::pathStringNoQual(fpath.clone(), (literal!(".")).clone(), true, false)?;
                    Print::printBuf((fstr.clone()).clone())?;
                    Print::printBuf((dumpCommentStr(c.clone())).clone())?;
                    Print::printBuf((literal!("\n")).clone())?;
                    dumpFunctionElements(daeElts.clone())?;
                    Print::printBuf((dumpClassAnnotationStr(c.clone())?).clone())?;
                    Print::printBuf((literal!("end ")).clone())?;
                    Print::printBuf((fstr.clone()).clone())?;
                    Print::printBuf((literal!(";\n\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { language: Deref @ "builtin", .. }, .. }, tail: _ }, .. } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { comment: c, isImpure, functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: ext_decl, body: daeElts }, tail: _ }, path: fpath, .. } => {
                    let mut fstr: ArcStr = arcstr::literal!("");
                    let mut ext_decl_str: ArcStr = arcstr::literal!("");
                    let mut impureStr: ArcStr = arcstr::literal!("");
                    impureStr = (if (isImpure.clone()) {literal!("impure ")} else {literal!("")}).clone();
                    Print::printBuf((impureStr.clone()).clone())?;
                    Print::printBuf((literal!("function ")).clone())?;
                    fstr = AbsynUtil::pathStringNoQual(fpath.clone(), (literal!(".")).clone(), true, false)?;
                    Print::printBuf((fstr.clone()).clone())?;
                    Print::printBuf((dumpCommentStr(c.clone())).clone())?;
                    Print::printBuf((literal!("\n")).clone())?;
                    dumpFunctionElements(daeElts.clone())?;
                    ext_decl_str = (dumpExtDeclStr(ext_decl.clone())?).clone();
                    Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*ext_decl_str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Print::printBuf((dumpClassAnnotationStr(c.clone())?).clone())?;
                    Print::printBuf((literal!("end ")).clone())?;
                    Print::printBuf((fstr.clone()).clone())?;
                    Print::printBuf((literal!(";\n\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::RECORD_CONSTRUCTOR { type_: t, path: fpath, .. } => {
                    let mut fstr: ArcStr = arcstr::literal!("");
                    let false = (Flags::isSet(Flags::DISABLE_RECORD_CONSTRUCTOR_OUTPUT.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::PRINT_RECORD_TYPES.clone())? {
                        Print::printBuf((TypesDump::unparseType(t.clone())?).clone())?;
                        Print::printBuf((literal!("\n")).clone())?;
                    } else {
                        Print::printBuf((literal!("function ")).clone())?;
                        fstr = AbsynUtil::pathStringNoQual(fpath.clone(), (literal!(".")).clone(), true, false)?;
                        Print::printBuf((fstr.clone()).clone())?;
                        Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" \"Automatically generated record constructor for ")); __mm_s.push_str(&*fstr.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone())?;
                        Print::printBuf((printRecordConstructorInputsStr(t.clone())?).clone())?;
                        Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  output ")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(fpath.clone())?); __mm_s.push_str(&*literal!(" res;\n")); ArcStr::from(__mm_s) }).clone())?;
                        Print::printBuf((literal!("end ")).clone())?;
                        Print::printBuf((fstr.clone()).clone())?;
                        Print::printBuf((literal!(";\n\n")).clone())?;
                    }
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn dumpParallelismStr(mut inType: Arc<DAE::Type>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcArg: _, funcResultType: _, functionAttributes: DAE::FunctionAttributes { functionParallelism: DAE::FunctionParallelism::FP_NON_PARALLEL, .. }, path: _ } => literal!(""),
        Deref @ DAE::Type::T_FUNCTION { funcArg: _, funcResultType: _, functionAttributes: DAE::FunctionAttributes { functionParallelism: DAE::FunctionParallelism::FP_PARALLEL_FUNCTION, .. }, path: _ } => literal!("parallel "),
        Deref @ DAE::Type::T_FUNCTION { funcArg: _, funcResultType: _, functionAttributes: DAE::FunctionAttributes { functionParallelism: DAE::FunctionParallelism::FP_KERNEL_FUNCTION, .. }, path: _ } => literal!("kernel "),
        _ => literal!("#dumpParallelismStr failed#"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

pub fn dumpInlineTypeStr(mut inlineType: DAE::InlineType) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inlineType.clone() {
        DAE::InlineType::NO_INLINE => literal!("\"Inline never\""),
        DAE::InlineType::AFTER_INDEX_RED_INLINE => literal!(" \"Inline after index reduction\""),
        DAE::InlineType::NORM_INLINE => literal!(" \"Inline before index reduction\""),
        DAE::InlineType::DEFAULT_INLINE => literal!("\"Inline if necessary\""),
        DAE::InlineType::EARLY_INLINE => literal!("\"Inline earier than normal inline\""),
        DAE::InlineType::BUILTIN_EARLY_INLINE => literal!("\"Inline even if inlining is disabled\""),
        _ => literal!("\"unknown\""),
    })).clone();
    r#str
}

pub fn dumpInlineTypeBackendStr(mut inlineType: DAE::InlineType) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inlineType.clone() {
        DAE::InlineType::NO_INLINE => literal!("NONE"),
        DAE::InlineType::AFTER_INDEX_RED_INLINE => literal!("AFTER_INDEX_RED"),
        DAE::InlineType::NORM_INLINE => literal!("NORMAL"),
        DAE::InlineType::DEFAULT_INLINE => literal!("DEFAULT"),
        DAE::InlineType::EARLY_INLINE => literal!("EARLY"),
        DAE::InlineType::BUILTIN_EARLY_INLINE => literal!("BUILTIN_EARLY"),
        _ => literal!("UNKNOWN"),
    })).clone();
    r#str
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn printRecordConstructorInputsStr(mut itp: Arc<DAE::Type>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(itp.clone()) {
        Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. } => {
            let mut var_strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            var_strl = List::map(vars.clone(), Arc::new(printRecordConstructorInputStr));
            stringAppendList(var_strl.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: tp, .. } => {
            printRecordConstructorInputsStr(tp.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

fn printRecordConstructorInputStr(mut inVar: Arc<DAE::Var>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut attr_str: ArcStr = arcstr::literal!("");
    let mut binding_str: ArcStr = arcstr::literal!("");
    let mut ty_str: ArcStr = arcstr::literal!("");
    let mut ty_vars_str: ArcStr = arcstr::literal!("");
    let mut attr: Arc<DAE::Attributes>;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut binding: Arc<DAE::Binding> = Arc::new(DAE::Binding::UNBOUND);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Var { binding: __pa0, ty: __pa1, attributes: __pa2, name: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    binding = __pa0.clone();
    ty = __pa1.clone();
    attr = __pa2.clone();
    name = __pa3.clone();
    attr_str = (printRecordConstructorInputAttrStr(attr.clone())).clone();
    binding_str = (printRecordConstructorBinding(binding.clone())?).clone();
    (ty_str, ty_vars_str) = printTypeStr(ty.clone())?;
    outString = stringAppendList(list![(literal!("  ")).clone(), (attr_str.clone()).clone(), (ty_str.clone()).clone(), (literal!(" ")).clone(), (name.clone()).clone(), (ty_vars_str.clone()).clone(), (binding_str.clone()).clone(), (literal!(";\n")).clone()]);
    Ok(outString)
}

fn printRecordConstructorInputAttrStr(mut inAttributes: Arc<DAE::Attributes>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inAttributes.clone()) {
        Deref @ DAE::Attributes { visibility: SCode::Visibility::PROTECTED, .. } => literal!("protected "),
        Deref @ DAE::Attributes { variability: SCode::Variability::CONST, .. } => literal!("constant "),
        _ => literal!("input "),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

fn printRecordConstructorBinding(mut binding: Arc<DAE::Binding>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ DAE::Binding::UNBOUND => {
            literal!("")
        },
        Deref @ DAE::Binding::EQBOUND { source: DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE, exp: e, .. } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ DAE::Binding::EQBOUND { source: DAE::BindingSource::BINDING_FROM_RECORD_SUBMODS, exp: e, .. } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Deref @ DAE::Binding::VALBOUND { source: DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE, valBound: v } => {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ValuesDump::valString(v.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

fn ppStatement(mut alg: Arc<DAE::Statement>) -> Result<()> {
    ppStmt(alg.clone(), 2)?;
    Ok(())
}

pub fn ppStatementStr(mut alg: Arc<DAE::Statement>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (ppStmtStr(alg.clone(), 2)?).clone();
    Ok(r#str)
}

fn ppStmt(mut inStatement: Arc<DAE::Statement>, mut inInteger: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inStatement.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { source, exp: e, exp1: e2, .. }, i) => {
                    indent(i.clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(" := ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    if Config::typeinfo()? {
                        Print::printBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" /* ")); __mm_s.push_str(&*Error::infoStr(ElementSource::getElementSourceFileInfo(source.clone()))?); __mm_s.push_str(&*literal!(" */")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: e, lhs: e2, .. }, i) => {
                    indent(i.clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(" := ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: e, expExpLst: expl, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut es: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    es = List::map(expl.clone(), Arc::new(ExpressionBasics::printExpStr));
                    s3 = stringDelimitList(es.clone(), (literal!(", ")).clone());
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("(")).clone(), (s3.clone()).clone(), (literal!(") := ")).clone(), (s2.clone()).clone(), (literal!(";\n")).clone()]);
                    Print::printBuf((r#str.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { else_, statementLst: then_, exp: e, .. }, i) => {
                    let mut i_1: i32 = 0;
                    indent(i.clone())?;
                    Print::printBuf((literal!("if ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf((literal!(" then\n")).clone())?;
                    i_1 = i.clone() + 2;
                    ppStmtList(then_.clone(), i_1.clone())?;
                    ppElse(else_.clone(), i.clone())?;
                    indent(i.clone())?;
                    Print::printBuf((literal!("end if;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { statementLst: stmts, range: e, iter: id, .. }, i) => {
                    let mut i_1: i32 = 0;
                    indent(i.clone())?;
                    Print::printBuf((literal!("for ")).clone())?;
                    Print::printBuf((id.clone()).clone())?;
                    Print::printBuf((literal!(" in ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf((literal!(" loop\n")).clone())?;
                    i_1 = i.clone() + 2;
                    ppStmtList(stmts.clone(), i_1.clone())?;
                    indent(i.clone())?;
                    Print::printBuf((literal!("end for;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_PARFOR { statementLst: stmts, range: e, iter: id, .. }, i) => {
                    let mut i_1: i32 = 0;
                    indent(i.clone())?;
                    Print::printBuf((literal!("parfor ")).clone())?;
                    Print::printBuf((id.clone()).clone())?;
                    Print::printBuf((literal!(" in ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf((literal!(" loop\n")).clone())?;
                    i_1 = i.clone() + 2;
                    ppStmtList(stmts.clone(), i_1.clone())?;
                    indent(i.clone())?;
                    Print::printBuf((literal!("end parfor;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHILE { statementLst: stmts, exp: e, .. }, i) => {
                    let mut i_1: i32 = 0;
                    indent(i.clone())?;
                    Print::printBuf((literal!("while ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    Print::printBuf((literal!(" loop\n")).clone())?;
                    i_1 = i.clone() + 2;
                    ppStmtList(stmts.clone(), i_1.clone())?;
                    indent(i.clone())?;
                    Print::printBuf((literal!("end while;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_NORETCALL { exp: e1, .. }, i) => {
                    indent(i.clone())?;
                    let _ = (::match_deref::match_deref! { match &(e1.clone()) {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { .. }, .. }, .. } => {
                    Print::printBuf((literal!("return ")).clone())?;
                    ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(";\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stmt @ Deref @ DAE::Statement::STMT_WHEN { .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf((ppWhenStmtStr(stmt.clone(), 1)?).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSERT { msg, cond, .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf((literal!("assert(")).clone())?;
                    ExpressionDump::printExp(cond.clone())?;
                    Print::printBuf((literal!(", ")).clone())?;
                    ExpressionDump::printExp(msg.clone())?;
                    Print::printBuf((literal!(");\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_RETURN { .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf((literal!("return;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_BREAK { .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf((literal!("break;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_REINIT { value: e2, var: e1, .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf((literal!("reinit(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(");\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf((literal!("begin failure\n")).clone())?;
                    ppStmtList(stmts.clone(), i.clone() + 2)?;
                    Print::printBuf((literal!("end try;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ARRAY_INIT { ty, name, .. }, i) => {
                    indent(i.clone())?;
                    Print::printBuf((literal!("/* ")).clone())?;
                    Print::printBuf((name.clone()).clone())?;
                    Print::printBuf((literal!(" := array_alloc(")).clone())?;
                    Print::printBuf((TypesDump::unparseType(ty.clone())?).clone())?;
                    Print::printBuf((literal!(") */;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, i) => {
                    indent(i.clone())?;
                    Print::printBuf((literal!("**ALGORITHM**;\n")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn ppWhenStmtStr(mut inStatement: Arc<DAE::Statement>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inStatement.clone(), inInteger.clone())) {
        (Deref @ DAE::Statement::STMT_WHEN { elseWhen: None, statementLst: stmts, exp: e, .. }, i) => {
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s5: ArcStr = arcstr::literal!("");
            let mut s6: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut s7: ArcStr = arcstr::literal!("");
            let mut s8: ArcStr = arcstr::literal!("");
            let mut s9: ArcStr = arcstr::literal!("");
            let mut i_1: i32 = 0;
            s3 = (stringAppend((literal!("when ")).clone(), (ExpressionBasics::printExpStr(e.clone())?).clone())).clone();
            s5 = (stringAppend((s3.clone()).clone(), (literal!(" then\n")).clone())).clone();
            i_1 = i.clone() + 2;
            s6 = (ppStmtListStr(stmts.clone(), i_1.clone())?).clone();
            s7 = (stringAppend((s5.clone()).clone(), (s6.clone()).clone())).clone();
            s8 = (indentStr(i.clone())?).clone();
            s9 = (stringAppend((s7.clone()).clone(), (s8.clone()).clone())).clone();
            r#str = (stringAppend((s9.clone()).clone(), (literal!("end when;\n")).clone())).clone();
            r#str.clone()
        },
        (Deref @ DAE::Statement::STMT_WHEN { elseWhen: Some(stmt), statementLst: stmts, exp: e, .. }, i) => {
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s4: ArcStr = arcstr::literal!("");
            let mut s5: ArcStr = arcstr::literal!("");
            let mut s6: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut s7: ArcStr = arcstr::literal!("");
            let mut s8: ArcStr = arcstr::literal!("");
            let mut s9: ArcStr = arcstr::literal!("");
            let mut s10: ArcStr = arcstr::literal!("");
            let mut i_1: i32 = 0;
            s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s4 = (stringAppend((literal!("when ")).clone(), (s3.clone()).clone())).clone();
            s5 = (stringAppend((s4.clone()).clone(), (literal!(" then\n")).clone())).clone();
            i_1 = i.clone() + 2;
            s6 = (ppStmtListStr(stmts.clone(), i_1.clone())?).clone();
            s7 = (stringAppend((s5.clone()).clone(), (s6.clone()).clone())).clone();
            s8 = (ppWhenStmtStr(stmt.clone(), i.clone())?).clone();
            s9 = (stringAppend((indentStr(i.clone())?).clone(), (literal!("else")).clone())).clone();
            s10 = (stringAppend((s7.clone()).clone(), (s9.clone()).clone())).clone();
            r#str = (stringAppend((s10.clone()).clone(), (s8.clone()).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn ppStmtStr(mut inStatement: Arc<DAE::Statement>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inStatement.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { exp: e, exp1: e2, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (s2.clone()).clone(), (literal!(" := ")).clone(), (s3.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: e, lhs: e2, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (s2.clone()).clone(), (literal!(" := ")).clone(), (s3.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: e, expExpLst: expl, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut es: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    es = List::map(expl.clone(), Arc::new(ExpressionBasics::printExpStr));
                    s3 = stringDelimitList(es.clone(), (literal!(", ")).clone());
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("(")).clone(), (s3.clone()).clone(), (literal!(") := ")).clone(), (s2.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { else_, statementLst: then_, exp: e, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut s6: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut s7: ArcStr = arcstr::literal!("");
                    let mut s8: ArcStr = arcstr::literal!("");
                    let mut s9: ArcStr = arcstr::literal!("");
                    let mut s10: ArcStr = arcstr::literal!("");
                    let mut s11: ArcStr = arcstr::literal!("");
                    let mut i_1: i32 = 0;
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = (stringAppend((s1.clone()).clone(), (literal!("if ")).clone())).clone();
                    s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    s4 = (stringAppend((s2.clone()).clone(), (s3.clone()).clone())).clone();
                    s5 = (stringAppend((s4.clone()).clone(), (literal!(" then\n")).clone())).clone();
                    i_1 = i.clone() + 2;
                    s6 = (ppStmtListStr(then_.clone(), i_1.clone())?).clone();
                    s7 = (stringAppend((s5.clone()).clone(), (s6.clone()).clone())).clone();
                    s8 = (ppElseStr(else_.clone(), i.clone())?).clone();
                    s9 = (stringAppend((s7.clone()).clone(), (s8.clone()).clone())).clone();
                    s10 = (indentStr(i.clone())?).clone();
                    s11 = (stringAppend((s9.clone()).clone(), (s10.clone()).clone())).clone();
                    r#str = (stringAppend((s11.clone()).clone(), (literal!("end if;\n")).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { statementLst: stmts, range: e, iter: id, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut i_1: i32 = 0;
                    s1 = (indentStr(i.clone())?).clone();
                    s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    i_1 = i.clone() + 2;
                    s4 = (ppStmtListStr(stmts.clone(), i_1.clone())?).clone();
                    s5 = (indentStr(i.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("for ")).clone(), (id.clone()).clone(), (literal!(" in ")).clone(), (s3.clone()).clone(), (literal!(" loop\n")).clone(), (s4.clone()).clone(), (s5.clone()).clone(), (literal!("end for;\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_PARFOR { statementLst: stmts, range: e, iter: id, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut i_1: i32 = 0;
                    s1 = (indentStr(i.clone())?).clone();
                    s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    i_1 = i.clone() + 2;
                    s4 = (ppStmtListStr(stmts.clone(), i_1.clone())?).clone();
                    s5 = (indentStr(i.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("parfor ")).clone(), (id.clone()).clone(), (literal!(" in ")).clone(), (s3.clone()).clone(), (literal!(" loop\n")).clone(), (s4.clone()).clone(), (s5.clone()).clone(), (literal!("end for;\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHILE { statementLst: stmts, exp: e, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut s6: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut s7: ArcStr = arcstr::literal!("");
                    let mut s8: ArcStr = arcstr::literal!("");
                    let mut s9: ArcStr = arcstr::literal!("");
                    let mut i_1: i32 = 0;
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = (stringAppend((s1.clone()).clone(), (literal!("while ")).clone())).clone();
                    s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    s4 = (stringAppend((s2.clone()).clone(), (s3.clone()).clone())).clone();
                    s5 = (stringAppend((s4.clone()).clone(), (literal!(" loop\n")).clone())).clone();
                    i_1 = i.clone() + 2;
                    s6 = (ppStmtListStr(stmts.clone(), i_1.clone())?).clone();
                    s7 = (stringAppend((s5.clone()).clone(), (s6.clone()).clone())).clone();
                    s8 = (indentStr(i.clone())?).clone();
                    s9 = (stringAppend((s7.clone()).clone(), (s8.clone()).clone())).clone();
                    r#str = (stringAppend((s9.clone()).clone(), (literal!("end while;\n")).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stmt @ Deref @ DAE::Statement::STMT_WHEN { .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = (ppWhenStmtStr(stmt.clone(), i.clone())?).clone();
                    r#str = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSERT { msg, cond, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut cond_str: ArcStr = arcstr::literal!("");
                    let mut msg_str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    cond_str = (ExpressionBasics::printExpStr(cond.clone())?).clone();
                    msg_str = (ExpressionBasics::printExpStr(msg.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("assert(")).clone(), (cond_str.clone()).clone(), (literal!(", ")).clone(), (msg_str.clone()).clone(), (literal!(");\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TERMINATE { msg, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut msg_str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    msg_str = (ExpressionBasics::printExpStr(msg.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("terminate(")).clone(), (msg_str.clone()).clone(), (literal!(");\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_NORETCALL { exp: e, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = ((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { .. }, .. }, .. } => literal!("return "),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
                    s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_RETURN { .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    r#str = (stringAppend((s1.clone()).clone(), (literal!("return;\n")).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_BREAK { .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    r#str = (stringAppend((s1.clone()).clone(), (literal!("break;\n")).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_REINIT { value: e2, var: e1, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut e1_str: ArcStr = arcstr::literal!("");
                    let mut e2_str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    e1_str = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    e2_str = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("reinit(")).clone(), (e1_str.clone()).clone(), (literal!(", ")).clone(), (e2_str.clone()).clone(), (literal!(");\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    s2 = (ppStmtListStr(stmts.clone(), i.clone() + 2)?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("failure(\n")).clone(), (s2.clone()).clone(), (s1.clone()).clone(), (literal!(");\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ARRAY_INIT { name: s2, .. }, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    r#str = stringAppendList(list![(s1.clone()).clone(), (literal!("arrayInit(\n")).clone(), (s2.clone()).clone(), (s1.clone()).clone(), (literal!(");\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, i) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (indentStr(i.clone())?).clone();
                    r#str = (stringAppend((s1.clone()).clone(), (literal!("**ALGORITHM COULD NOT BE GENERATED(DAE.mo)**;\n")).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn ppStmtList(mut inAlgorithmStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inInteger: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inAlgorithmStatementLst.clone(), inInteger.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: stmt, tail: stmts }, i) => {
            ppStmt(stmt.clone(), i.clone())?;
            ppStmtList(stmts.clone(), i.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn ppStmtListStr(mut inAlgorithmStatementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inAlgorithmStatementLst.clone(), inInteger.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            literal!("")
        },
        (Deref @ metamodelica::List::Cons { head: stmt, tail: stmts }, i) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            s1 = (ppStmtStr(stmt.clone(), i.clone())?).clone();
            s2 = (ppStmtListStr(stmts.clone(), i.clone())?).clone();
            r#str = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn ppElse(mut inElse: Arc<DAE::Else>, mut inInteger: i32) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inElse.clone(), inInteger.clone())) {
        (Deref @ DAE::Else::NOELSE, _) => {
            ()
        },
        (Deref @ DAE::Else::ELSEIF { else_, statementLst: then_, exp: e }, i) => {
            let mut i_1: i32 = 0;
            indent(i.clone())?;
            Print::printBuf((literal!("elseif ")).clone())?;
            ExpressionDump::printExp(e.clone())?;
            Print::printBuf((literal!(" then\n")).clone())?;
            i_1 = i.clone() + 2;
            ppStmtList(then_.clone(), i_1.clone())?;
            ppElse(else_.clone(), i.clone())?;
            ()
        },
        (Deref @ DAE::Else::ELSE { statementLst: stmts }, i) => {
            let mut i_1: i32 = 0;
            indent(i.clone())?;
            Print::printBuf((literal!("else\n")).clone())?;
            i_1 = i.clone() + 2;
            ppStmtList(stmts.clone(), i_1.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn ppElseStr(mut inElse: Arc<DAE::Else>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inElse.clone(), inInteger.clone())) {
        (Deref @ DAE::Else::NOELSE, _) => {
            literal!("")
        },
        (Deref @ DAE::Else::ELSEIF { else_, statementLst: then_, exp: e }, i) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut s4: ArcStr = arcstr::literal!("");
            let mut s5: ArcStr = arcstr::literal!("");
            let mut s6: ArcStr = arcstr::literal!("");
            let mut s7: ArcStr = arcstr::literal!("");
            let mut s8: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut i_1: i32 = 0;
            s1 = (indentStr(i.clone())?).clone();
            s2 = (stringAppend((s1.clone()).clone(), (literal!("elseif ")).clone())).clone();
            s3 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            s4 = (stringAppend((s2.clone()).clone(), (s3.clone()).clone())).clone();
            s5 = (stringAppend((s4.clone()).clone(), (literal!(" then\n")).clone())).clone();
            i_1 = i.clone() + 2;
            s6 = (ppStmtListStr(then_.clone(), i_1.clone())?).clone();
            s7 = (stringAppend((s5.clone()).clone(), (s6.clone()).clone())).clone();
            s8 = (ppElseStr(else_.clone(), i.clone())?).clone();
            r#str = (stringAppend((s7.clone()).clone(), (s8.clone()).clone())).clone();
            r#str.clone()
        },
        (Deref @ DAE::Else::ELSE { statementLst: stmts }, i) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut i_1: i32 = 0;
            s1 = (indentStr(i.clone())?).clone();
            s2 = (stringAppend((s1.clone()).clone(), (literal!("else\n")).clone())).clone();
            i_1 = i.clone() + 2;
            s3 = (ppStmtListStr(stmts.clone(), i_1.clone())?).clone();
            r#str = (stringAppend((s2.clone()).clone(), (s3.clone()).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn indent(mut inInteger: i32) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inInteger.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut i = __mc_input.clone() else { bail!("nomatch") };
            let mut i_1: i32 = 0;
            Print::printBuf((literal!(" ")).clone())?;
            i_1 = i.clone() - 1;
            indent(i_1.clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn indentStr(mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inInteger.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!(""))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut i = __mc_input.clone() else { bail!("nomatch") };
            let mut i_1: i32 = 0;
            let mut s1: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            i_1 = i.clone() - 1;
            s1 = (indentStr(i_1.clone())?).clone();
            r#str = (stringAppend((literal!(" ")).clone(), (s1.clone()).clone())).clone();
            Ok(r#str.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn dumpDebug(mut inDAElist: DAE::DAElist) -> Result<()> {
    let _ = (match inDAElist.clone() {
        DAE::DAElist { elementLst: ref elist } => {
            Print::printBuf((literal!("DAE(")).clone())?;
            dumpDebugElist(elist.clone())?;
            Print::printBuf((literal!(")")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn dumpDebugElist(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inElementLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
            dumpDebugElement(first.clone())?;
            Print::printBuf((literal!("\n")).clone())?;
            dumpDebugElist(rest.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn dumpDebugDAE(mut dae: DAE::DAElist) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match dae.clone() {
        DAE::DAElist { elementLst: ref elems } => {
            Print::clearBuf();
            dumpDebugElist(elems.clone())?;
            r#str = (Print::getString()?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn dumpDebugElement(mut inElement: Arc<DAE::Element>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { comment, variableAttributesOption: dae_var_attr, binding: None, kind: vk, componentRef: cr, .. } => {
                    let mut comment_str: ArcStr = arcstr::literal!("");
                    let mut tmp_str: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("VAR(")).clone())?;
                    ComponentReference::printComponentRef(cr.clone())?;
                    Print::printBuf((literal!(", ")).clone())?;
                    dumpKind(vk.clone())?;
                    comment_str = (dumpCommentAnnotationStr(comment.clone())?).clone();
                    Print::printBuf((literal!("  comment:")).clone())?;
                    Print::printBuf((comment_str.clone()).clone())?;
                    tmp_str = (dumpVariableAttributesStr(dae_var_attr.clone())?).clone();
                    Print::printBuf((tmp_str.clone()).clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { comment, variableAttributesOption: dae_var_attr, binding: Some(e), kind: vk, componentRef: cr, .. } => {
                    let mut comment_str: ArcStr = arcstr::literal!("");
                    let mut tmp_str: ArcStr = arcstr::literal!("");
                    Print::printBuf((literal!("VAR(")).clone())?;
                    ComponentReference::printComponentRef(cr.clone())?;
                    Print::printBuf((literal!(", ")).clone())?;
                    dumpKind(vk.clone())?;
                    Print::printBuf((literal!(", binding: ")).clone())?;
                    ExpressionDump::printExp(e.clone())?;
                    comment_str = (dumpCommentAnnotationStr(comment.clone())?).clone();
                    Print::printBuf((literal!("  comment:")).clone())?;
                    Print::printBuf((comment_str.clone()).clone())?;
                    tmp_str = (dumpVariableAttributesStr(dae_var_attr.clone())?).clone();
                    Print::printBuf((tmp_str.clone()).clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { exp, componentRef: cr, .. } => {
                    Print::printBuf((literal!("DEFINE(")).clone())?;
                    ComponentReference::printComponentRef(cr.clone())?;
                    Print::printBuf((literal!(", ")).clone())?;
                    ExpressionDump::printExp(exp.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALDEFINE { exp, componentRef: cr, .. } => {
                    Print::printBuf((literal!("INITIALDEFINE(")).clone())?;
                    ComponentReference::printComponentRef(cr.clone())?;
                    Print::printBuf((literal!(", ")).clone())?;
                    ExpressionDump::printExp(exp.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { scalar: e2, exp: e1, .. } => {
                    Print::printBuf((literal!("EQUATION(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { cr2, cr1, .. } => {
                    Print::printBuf((literal!("EQUATION(")).clone())?;
                    ComponentReference::printComponentRef(cr1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ComponentReference::printComponentRef(cr2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALEQUATION { exp2: e2, exp1: e1, .. } => {
                    Print::printBuf((literal!("INITIALEQUATION(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { .. } => {
                    Print::printBuf((literal!("ALGORITHM()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALALGORITHM { .. } => {
                    Print::printBuf((literal!("INITIALALGORITHM()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMP { dAElist: l, ident: n, .. } => {
                    Print::printBuf((literal!("COMP(")).clone())?;
                    Print::printBuf((n.clone()).clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    dumpDebugElist(l.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { array: e2, exp: e1, .. } => {
                    Print::printBuf((literal!("ARRAY_EQUATION(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { array: e2, exp: e1, .. } => {
                    Print::printBuf((literal!("INITIAL_ARRAY_EQUATION(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { rhs: e2, lhs: e1, .. } => {
                    Print::printBuf((literal!("COMPLEX_EQUATION(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { rhs: e2, lhs: e1, .. } => {
                    Print::printBuf((literal!("INITIAL_COMPLEX_EQUATION(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::IF_EQUATION { .. } => {
                    Print::printBuf((literal!("IF_EQUATION()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
                    Print::printBuf((literal!("INITIAL_IF_EQUATION()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::WHEN_EQUATION { .. } => {
                    Print::printBuf((literal!("WHEN_EQUATION()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
                    Print::printBuf((literal!("EXTOBJECTCLASS()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { message: e2, condition: e1, .. } => {
                    Print::printBuf((literal!("ASSERT(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ASSERT { message: e2, condition: e1, .. } => {
                    Print::printBuf((literal!("INITIAL_ASSERT(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    ExpressionDump::printExp(e2.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::TERMINATE { message: e1, .. } => {
                    Print::printBuf((literal!("TERMINATE(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, .. } => {
                    Print::printBuf((literal!("INITIAL_TERMINATE(")).clone())?;
                    ExpressionDump::printExp(e1.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::REINIT { .. } => {
                    Print::printBuf((literal!("REINIT()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { .. } => {
                    Print::printBuf((literal!("NORETCALL()")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::SM_COMP { dAElist: l, componentRef: cr } => {
                    Print::printBuf((literal!("SM_COMP(")).clone())?;
                    ComponentReference::printComponentRef(cr.clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    dumpDebugElist(l.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::FLAT_SM { dAElist: l, ident: n } => {
                    Print::printBuf((literal!("FLAT_SM(")).clone())?;
                    Print::printBuf((n.clone()).clone())?;
                    Print::printBuf((literal!(",")).clone())?;
                    dumpDebugElist(l.clone())?;
                    Print::printBuf((literal!(")")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf((literal!("UNKNOWN ")).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn dumpFlow(mut var: Arc<DAE::ConnectorType>) -> Result<ArcStr> {
    let mut flowString: ArcStr = arcstr::literal!("");
    flowString = ((::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::ConnectorType::FLOW => literal!("flow"),
        Deref @ DAE::ConnectorType::POTENTIAL => literal!("effort"),
        Deref @ DAE::ConnectorType::NON_CONNECTOR => literal!("non_connector"),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(flowString)
}

pub fn dumpConnectorType(mut inConnectorType: Arc<DAE::ConnectorType>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inConnectorType.clone()) {
        Deref @ DAE::ConnectorType::FLOW => literal!("flow"),
        Deref @ DAE::ConnectorType::STREAM { .. } => literal!("stream"),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

pub fn dumpGraphviz(mut dae: DAE::DAElist) -> Result<()> {
    let mut r: Arc<Graphviz::Node>;
    r = buildGraphviz(dae.clone())?;
    Graphviz::dump(r.clone())?;
    Ok(())
}

fn buildGraphviz(mut inDAElist: DAE::DAElist) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = (match inDAElist.clone() {
        DAE::DAElist { elementLst: ref els } => {
            let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut nonvars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut nonvarnodes: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            let mut varnodes: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            let mut nodelist: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            vars = DAEUtil::getMatchingElements(els.clone(), Arc::new(fnptr!(DAEUtil::isVar, Arc<DAE::Element>)));
            nonvars = DAEUtil::getMatchingElements(els.clone(), Arc::new(fnptr!(DAEUtil::isNotVar, Arc<DAE::Element>)));
            nonvarnodes = buildGrList(nonvars.clone())?;
            varnodes = buildGrVars(vars.clone())?;
            nodelist = listAppend(nonvarnodes.clone(), varnodes.clone());
            Arc::new(Graphviz::Node::NODE { type_: (literal!("DAE")).clone(), attributes: metamodelica::nil(), children: nodelist.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outNode)
}

fn buildGrList(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<Graphviz::Node>>>> {
    let mut outGraphvizNodeLst: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
    outGraphvizNodeLst = (::match_deref::match_deref! { match &(inElementLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: el, tail: rest } => {
            let mut node: Arc<Graphviz::Node>;
            let mut nodelist: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            node = buildGrElement(el.clone())?;
            nodelist = buildGrList(rest.clone())?;
            cons(node.clone(), nodelist.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outGraphvizNodeLst)
}

fn buildGrVars(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<Graphviz::Node>>>> {
    let mut outGraphvizNodeLst: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
    outGraphvizNodeLst = 'mc: {
        let __mc_input = inElementLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                vars => {
                    let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (strlist, _) = buildGrStrlist(vars.clone(), Arc::new(buildGrVarStr), 10)?;
                    Ok(list![Arc::new(Graphviz::Node::LNODE { type_: (literal!("VARS")).clone(), labelLst: strlist.clone(), attributes: list![Graphviz::r#box.clone()], children: metamodelica::nil() })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outGraphvizNodeLst)
}

pub fn buildGrStrlist<Type_a: Clone + 'static>(mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>, mut inInteger: i32) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Type_a>>)> {
    pub type FuncTypeType_aToString<Type_a: Clone> = fn(Type_a) -> Result<ArcStr>;

    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outTypeALst: Arc<metamodelica::List<Type_a>> = metamodelica::nil();
    (outStringLst, outTypeALst) = (::match_deref::match_deref! { match &((inTypeALst.clone(), inFuncTypeTypeAToString.clone(), inInteger.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            (metamodelica::nil(), metamodelica::nil())
        },
        (ignored, _, count) if (count.clone() <= 0) => {
            (list![(literal!("...")).clone()], ignored.clone())
        },
        (Deref @ metamodelica::List::Cons { head: var, tail: rest }, printer, count) if (count.clone() > 0) => {
            let mut ignored: Arc<metamodelica::List<Type_a>> = metamodelica::nil();
            let mut count_1: i32 = 0;
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut r#str: ArcStr = arcstr::literal!("");
            count_1 = count.clone() - 1;
            (strlist, ignored) = buildGrStrlist(rest.clone(), printer.clone(), count_1.clone())?;
            r#str = printer(var.clone())?;
            (cons(r#str.clone(), strlist.clone()), ignored.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outStringLst, outTypeALst))
}

fn buildGrVarStr(mut inElement: Arc<DAE::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { binding: None, componentRef: cr, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            r#str.clone()
        },
        Deref @ DAE::Element::VAR { binding: Some(exp), componentRef: cr, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut expstr: ArcStr = arcstr::literal!("");
            let mut str_1: ArcStr = arcstr::literal!("");
            let mut str_2: ArcStr = arcstr::literal!("");
            r#str = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            expstr = (printExpStrSpecial(exp.clone())?).clone();
            str_1 = (stringAppend((r#str.clone()).clone(), (literal!(" = ")).clone())).clone();
            str_2 = (stringAppend((str_1.clone()).clone(), (expstr.clone()).clone())).clone();
            str_2.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn printExpStrSpecial(mut inExp: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s_1 = (stringAppend((literal!("\\\"")).clone(), (s.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!("\\\"")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn buildGrElement(mut inElement: Arc<DAE::Element>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { binding: None, kind: vk, componentRef: cr, .. } => {
            let mut crstr: ArcStr = arcstr::literal!("");
            let mut vkstr: ArcStr = arcstr::literal!("");
            crstr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            vkstr = (dumpKindStr(vk.clone())?).clone();
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("VAR")).clone(), labelLst: list![(crstr.clone()).clone(), (vkstr.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::VAR { binding: Some(exp), kind: vk, componentRef: cr, .. } => {
            let mut crstr: ArcStr = arcstr::literal!("");
            let mut vkstr: ArcStr = arcstr::literal!("");
            let mut expstr: ArcStr = arcstr::literal!("");
            let mut expstr_1: ArcStr = arcstr::literal!("");
            crstr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            vkstr = (dumpKindStr(vk.clone())?).clone();
            expstr = (printExpStrSpecial(exp.clone())?).clone();
            expstr_1 = (stringAppend((literal!("= ")).clone(), (expstr.clone()).clone())).clone();
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("VAR")).clone(), labelLst: list![(crstr.clone()).clone(), (vkstr.clone()).clone(), (expstr_1.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::DEFINE { exp, componentRef: cr, .. } => {
            let mut crstr: ArcStr = arcstr::literal!("");
            let mut expstr: ArcStr = arcstr::literal!("");
            let mut expstr_1: ArcStr = arcstr::literal!("");
            crstr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            expstr = (printExpStrSpecial(exp.clone())?).clone();
            expstr_1 = (stringAppend((literal!("= ")).clone(), (expstr.clone()).clone())).clone();
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("DEFINE")).clone(), labelLst: list![(crstr.clone()).clone(), (expstr_1.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::EQUATION { scalar: e2, exp: e1, .. } => {
            let mut e1str: ArcStr = arcstr::literal!("");
            let mut e2str: ArcStr = arcstr::literal!("");
            e1str = (printExpStrSpecial(e1.clone())?).clone();
            e2str = (printExpStrSpecial(e2.clone())?).clone();
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("EQUATION")).clone(), labelLst: list![(e1str.clone()).clone(), (literal!("=")).clone(), (e2str.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::EQUEQUATION { cr2, cr1, .. } => {
            let mut e1str: ArcStr = arcstr::literal!("");
            let mut e2str: ArcStr = arcstr::literal!("");
            e1str = (printExpStrSpecial(Expression::crefExp(cr1.clone())?)?).clone();
            e2str = (printExpStrSpecial(Expression::crefExp(cr2.clone())?)?).clone();
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("EQUEQUATION")).clone(), labelLst: list![(e1str.clone()).clone(), (literal!("=")).clone(), (e2str.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            Arc::new(Graphviz::Node::NODE { type_: (literal!("ALGORITHM")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::INITIALDEFINE { exp, componentRef: cr, .. } => {
            let mut crstr: ArcStr = arcstr::literal!("");
            let mut expstr: ArcStr = arcstr::literal!("");
            let mut expstr_1: ArcStr = arcstr::literal!("");
            crstr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            expstr = (printExpStrSpecial(exp.clone())?).clone();
            expstr_1 = (stringAppend((literal!("= ")).clone(), (expstr.clone()).clone())).clone();
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("INITIALDEFINE")).clone(), labelLst: list![(crstr.clone()).clone(), (expstr_1.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::INITIALEQUATION { exp2: e2, exp1: e1, .. } => {
            let mut e1str: ArcStr = arcstr::literal!("");
            let mut e2str: ArcStr = arcstr::literal!("");
            e1str = (printExpStrSpecial(e1.clone())?).clone();
            e2str = (printExpStrSpecial(e2.clone())?).clone();
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("INITIALEQUATION")).clone(), labelLst: list![(e1str.clone()).clone(), (literal!("=")).clone(), (e2str.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            Arc::new(Graphviz::Node::NODE { type_: (literal!("INITIALALGORITHM")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() })
        },
        Deref @ DAE::Element::COMP { dAElist: elts, ident: n, .. } => {
            let mut nodes: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            nodes = buildGrList(elts.clone())?;
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("COMP")).clone(), labelLst: list![(n.clone()).clone()], attributes: metamodelica::nil(), children: nodes.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn unparseType(mut tp: Arc<DAE::Type>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = tp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, .. } => {
                    let mut name: ArcStr = arcstr::literal!("");
                    name = AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?;
                    Ok(name.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, .. } => {
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut dim_str: ArcStr = arcstr::literal!("");
                    let mut path: Arc<Absyn::Path>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(Types::arrayElementType(ty.clone())) {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    dims = TypesDump::getDimensions(tp.clone());
                    name = AbsynUtil::pathStringNoQual(path.clone(), (literal!(".")).clone(), true, false)?;
                    dim_str = (List::toString(dims.clone(), Arc::new(ExpressionBasics::dimensionString), (literal!("")).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), false, 0)?).clone();
                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*dim_str.clone()); ArcStr::from(__mm_s) })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: ty @ Deref @ DAE::Type::T_SUBTYPE_BASIC { .. }, .. } => {
                    Ok(unparseType(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: bc_tp, .. } => {
                    Ok(TypesDump::unparseType(bc_tp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(TypesDump::unparseType(tp.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

pub fn unparseDimensions(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut printTypeDimension: bool) -> Result<ArcStr> {
    let mut dimsStr: ArcStr = arcstr::literal!("");
    dimsStr = ('mc: {
        let __mc_input = (dims.clone(), printTypeDimension.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, false) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, true) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(List::map(dims.clone(), Arc::new(ExpressionBasics::dimensionString)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(dimsStr)
}

pub fn dumpElementsStr(mut els: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(els.clone()) {
        _ => {
            let mut myStream: IOStream::IOStream;
            let mut r#str: ArcStr = arcstr::literal!("");
            myStream = IOStream::create((literal!("dae")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = dumpElementsStream(els.clone(), myStream.clone())?;
            r#str = (IOStream::string(myStream.clone())?).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn dumpAlgorithmsStr(mut algs: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(algs.clone()) {
        _ => {
            let mut myStream: IOStream::IOStream;
            let mut r#str: ArcStr = arcstr::literal!("");
            myStream = IOStream::create((literal!("algs")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = dumpAlgorithmsStream(algs.clone(), myStream.clone())?;
            r#str = (IOStream::string(myStream.clone())?).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn dumpConstraintsStr(mut constrs: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(constrs.clone()) {
        _ => {
            let mut myStream: IOStream::IOStream;
            let mut r#str: ArcStr = arcstr::literal!("");
            myStream = IOStream::create((literal!("constrs")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = dumpConstraintStream(constrs.clone(), myStream.clone())?;
            r#str = (IOStream::string(myStream.clone())?).clone();
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

/* *********** IOStream based implementation ***************/
/* *********** IOStream based implementation ***************/
/* *********** IOStream based implementation ***************/
/* *********** IOStream based implementation ***************/
fn isVisibleFunction(mut inFunc: DAE::Function, mut inHideRecordCons: bool, mut inInliningEnabled: bool) -> Result<bool> {
    let mut outIsVisible: bool = false;
    outIsVisible = (::match_deref::match_deref! { match &((inFunc.clone(), inHideRecordCons.clone(), inInliningEnabled.clone())) {
        (DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { language: Deref @ "builtin", .. }, .. }, tail: _ }, .. }, _, _) => {
            false
        },
        (DAE::Function::FUNCTION { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", .. } }, .. }, _, _) => {
            false
        },
        (DAE::Function::FUNCTION { inlineType: DAE::InlineType::BUILTIN_EARLY_INLINE, .. }, _, _) => {
            false
        },
        (DAE::Function::FUNCTION { inlineType: DAE::InlineType::EARLY_INLINE, .. }, _, true) => {
            false
        },
        (DAE::Function::FUNCTION { comment: cmt, .. }, _, _) => {
            !(SCodeUtil::optCommentHasBooleanNamedAnnotation(cmt.clone(), (literal!("__OpenModelica_builtin")).clone())?)
        },
        (DAE::Function::RECORD_CONSTRUCTOR { .. }, true, _) => {
            false
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsVisible)
}

fn dumpCompElementStream(mut inElement: Arc<DAE::Element>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElement.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::COMP { comment: c, dAElist: l, ident: n, .. }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), (literal!("class ")).clone())?;
                    r#str = IOStream::append(r#str.clone(), (n.clone()).clone())?;
                    r#str = IOStream::append(r#str.clone(), (dumpCommentStr(c.clone())).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("\n")).clone())?;
                    r#str = dumpElementsStream(l.clone(), r#str.clone())?;
                    r#str = IOStream::append(r#str.clone(), (dumpClassAnnotationStr(c.clone())?).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("end ")).clone())?;
                    r#str = IOStream::append(r#str.clone(), (n.clone()).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!(";\n")).clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

pub fn dumpElementsStream(mut l: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = (::match_deref::match_deref! { match &((l.clone(), inStream.clone())) {
        (_, r#str) => {
            let mut v: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut ie: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut ia: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut a: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut co: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut sm: Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>> = metamodelica::nil();
            let mut comments: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
            let mut ann: Option<Arc<SCode::Annotation>> = None;
            let mut r#str = (*r#str).clone();
            (v, ie, ia, e, a, _, co, _, sm, comments) = DAEUtil::splitElements(l.clone())?;
            r#str = dumpCompWithSplitElementsStream(sm.clone(), r#str.clone())?;
            r#str = dumpVarsStream(v.clone(), false, r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (if (ie.clone().is_empty()) {literal!("")} else {literal!("initial equation\n")}).clone())?;
            r#str = dumpInitialEquationsStream(ie.clone(), r#str.clone())?;
            r#str = dumpInitialAlgorithmsStream(ia.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (if (e.clone().is_empty()) {literal!("")} else {literal!("equation\n")}).clone())?;
            r#str = dumpEquationsStream(e.clone(), r#str.clone())?;
            r#str = dumpAlgorithmsStream(a.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (if (co.clone().is_empty()) {literal!("")} else {literal!("constraint\n")}).clone())?;
            r#str = dumpConstraintStream(co.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut cmt in (comments.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(cmt.clone()) {
        Deref @ SCode::Comment { annotation_: ann @ Some(_), .. } => SCodeDump::printCommentStr(Arc::new(SCode::Comment { annotation_: ann.clone(), comment: None }), SCodeDump::defaultOptions.clone())?,
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            r#str.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStream)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpCompWithSplitElementsStream(mut inCompLst: Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = (::match_deref::match_deref! { match &((inCompLst.clone(), inStream.clone())) {
        (Deref @ metamodelica::List::Nil, r#str) => {
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAEDumpTypes::compWithSplitElements { comment, spltElems, name }, tail: xs }, r#str) => {
            let mut cstr: ArcStr = arcstr::literal!("");
            let mut v: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut ie: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut ia: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut e: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut a: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut co: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
            let mut sm: Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>> = metamodelica::nil();
            let mut r#str = (*r#str).clone();
            match '__try0: {
                let __pa1 = ::match_deref::match_deref! { match &(comment.clone()) {
                    Some(Deref @ SCode::Comment { comment: Some(__pa1), .. }) => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                cstr = __pa1.clone();
                cstr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" \"")); __mm_s.push_str(&*cstr.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                Ok::<_, anyhow::Error>((cstr.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    cstr = __try0_o0;
                }
                Err(_) => {
                    cstr = (literal!("")).clone();
                }
            }
            let (__pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(spltElems.clone()) {
                Deref @ DAEDumpTypes::splitElements { v: __pa2, ie: __pa3, ia: __pa4, e: __pa5, a: __pa6, co: __pa7, o: _, ca: _, sm: __pa8 } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                _ => bail!("pattern mismatch"),
            } };
            v = __pa2.clone();
            ie = __pa3.clone();
            ia = __pa4.clone();
            e = __pa5.clone();
            a = __pa6.clone();
            co = __pa7.clone();
            sm = __pa8.clone();
            r#str = IOStream::append(r#str.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*cstr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            r#str = dumpCompWithSplitElementsStream(sm.clone(), r#str.clone())?;
            r#str = dumpVarsStream(v.clone(), false, r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (if (ie.clone().is_empty()) {literal!("")} else {literal!("initial equation\n")}).clone())?;
            r#str = dumpInitialEquationsStream(ie.clone(), r#str.clone())?;
            r#str = dumpInitialAlgorithmsStream(ia.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (if (e.clone().is_empty()) {literal!("")} else {literal!("equation\n")}).clone())?;
            r#str = dumpEquationsStream(e.clone(), r#str.clone())?;
            r#str = dumpAlgorithmsStream(a.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (if (co.clone().is_empty()) {literal!("")} else {literal!("constraint\n")}).clone())?;
            r#str = dumpConstraintStream(co.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("end ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*cstr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())?;
            r#str = dumpCompWithSplitElementsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStream)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpAlgorithmsStream(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElementLst.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), (literal!("algorithm\n")).clone())?;
                    r#str = IOStream::appendList(r#str.clone(), List::map(stmts.clone(), Arc::new(ppStatementStr)));
                    r#str = dumpAlgorithmsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = dumpAlgorithmsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn dumpInitialAlgorithmsStream(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElementLst.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), (literal!("initial algorithm\n")).clone())?;
                    r#str = IOStream::appendList(r#str.clone(), List::map(stmts.clone(), Arc::new(ppStatementStr)));
                    r#str = dumpInitialAlgorithmsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = dumpInitialAlgorithmsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn dumpEquationsStream(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = (::match_deref::match_deref! { match &((inElementLst.clone(), inStream.clone())) {
        (Deref @ metamodelica::List::Nil, r#str) => {
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { source: src, scalar: e2, exp: e1 }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUEQUATION { source: src, cr2, cr1 }, tail: xs }, r#str) => {
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            r#str = IOStream::append(r#str.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr2.clone())?); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())?;
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { source: src, array: e2, exp: e1, dimension: dims }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            s3 = (if (Config::typeinfo()?) {TypesDump::printDimensionsStr(dims.clone())} else {literal!("")}).clone();
            s3 = (if (Config::typeinfo()?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" /* array equation [")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("] */")); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (s3.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { source: src, rhs: e2, lhs: e1 }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::DEFINE { source: src, exp: e, componentRef: c }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            s1 = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ASSERT { source: src, level: Deref @ DAE::Exp::ENUM_LITERAL { index: 1, .. }, message: e2, condition: e1 }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  assert(")).clone(), (s1.clone()).clone(), (literal!(",")).clone(), (s2.clone()).clone(), (literal!(")")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::TERMINATE { source: src, message: e1 }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  terminate(")).clone(), (s1.clone()).clone(), (literal!(")")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::FOR_EQUATION { source: src, equations: xs1, range: e1, iter: s, .. }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            let _ = getSourceInformationStr(src.clone())?;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  for ")).clone(), (s.clone()).clone(), (literal!(" in ")).clone(), (s1.clone()).clone(), (literal!(" loop\n")).clone()]);
            r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  end for;\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { equations3: Deref @ metamodelica::List::Nil, equations2: Deref @ metamodelica::List::Nil, condition1: Deref @ metamodelica::List::Nil, .. }, tail: _ }, r#str) => {
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { source: src, equations3: Deref @ metamodelica::List::Nil, equations2: Deref @ metamodelica::List::Cons { head: xs1, tail: tb }, condition1: Deref @ metamodelica::List::Cons { head: e, tail: conds } }, tail: xs }, r#str) => {
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            r#str = IOStream::append(r#str.clone(), (literal!("  if ")).clone())?;
            r#str = IOStream::append(r#str.clone(), (ExpressionBasics::printExpStr(e.clone())?).clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!(" then\n")).clone())?;
            r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
            r#str = dumpIfEquationsStream(conds.clone(), tb.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!("  end if")).clone())?;
            r#str = IOStream::append(r#str.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())?;
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::IF_EQUATION { source: src, equations3: xs2, equations2: Deref @ metamodelica::List::Cons { head: xs1, tail: tb }, condition1: Deref @ metamodelica::List::Cons { head: e, tail: conds } }, tail: xs }, r#str) => {
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            r#str = IOStream::append(r#str.clone(), (literal!("  if ")).clone())?;
            r#str = IOStream::append(r#str.clone(), (ExpressionBasics::printExpStr(e.clone())?).clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!(" then\n")).clone())?;
            r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
            r#str = dumpIfEquationsStream(conds.clone(), tb.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!("  else\n")).clone())?;
            r#str = dumpEquationsStream(xs2.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  end if")); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())?;
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { source: src, elsewhen_: Some(el), equations: xs1, condition: e }, tail: xs }, r#str) => {
            let mut r#str = (*r#str).clone();
            let _ = getSourceInformationStr(src.clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!("when ")).clone())?;
            r#str = IOStream::append(r#str.clone(), (ExpressionBasics::printExpStr(e.clone())?).clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!(" then\n")).clone())?;
            r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!(" else")).clone())?;
            r#str = dumpEquationsStream(cons(el.clone(), xs.clone()), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::WHEN_EQUATION { source: src, elsewhen_: None, equations: xs1, condition: e }, tail: xs }, r#str) => {
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            r#str = IOStream::append(r#str.clone(), (literal!("  when ")).clone())?;
            r#str = IOStream::append(r#str.clone(), (ExpressionBasics::printExpStr(e.clone())?).clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!(" then\n")).clone())?;
            r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
            r#str = IOStream::append(r#str.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  end when")); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())?;
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::REINIT { source: src, exp: e, componentRef: cr }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s: ArcStr = arcstr::literal!("");
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  reinit(")).clone(), (s.clone()).clone(), (literal!(",")).clone(), (s1.clone()).clone(), (literal!(")")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::NORETCALL { source: src, exp: e }, tail: xs }, r#str) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut sourceStr: ArcStr = arcstr::literal!("");
            let mut r#str = (*r#str).clone();
            sourceStr = (getSourceInformationStr(src.clone())?).clone();
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: xs }, r#str) => {
            let mut r#str = (*r#str).clone();
            r#str = IOStream::append(r#str.clone(), (literal!("  /* unhandled equation in DAEDump.dumpEquationsStream FIXME! */\n")).clone())?;
            r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStream)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn dumpIfEquationsStream(mut iconds: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut itbs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = (::match_deref::match_deref! { match &((iconds.clone(), itbs.clone(), inStream.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, r#str) => {
            r#str.clone()
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: conds }, Deref @ metamodelica::List::Cons { head: tb, tail: tbs }, r#str) => {
            let mut r#str = (*r#str).clone();
            r#str = IOStream::append(r#str.clone(), (literal!("  elseif ")).clone())?;
            r#str = IOStream::append(r#str.clone(), (ExpressionBasics::printExpStr(c.clone())?).clone())?;
            r#str = IOStream::append(r#str.clone(), (literal!(" then\n")).clone())?;
            r#str = dumpEquationsStream(tb.clone(), r#str.clone())?;
            r#str = dumpIfEquationsStream(conds.clone(), tbs.clone(), r#str.clone())?;
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStream)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn dumpInitialEquationsStream(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElementLst.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIALEQUATION { exp2: e2, exp1: e1, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!(";\n")).clone()]);
                    r#str = dumpInitialEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { array: e2, exp: e1, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!(";\n")).clone()]);
                    r#str = dumpInitialEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { rhs: e2, lhs: e1, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!(";\n")).clone()]);
                    r#str = dumpInitialEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIALDEFINE { exp: e, componentRef: c, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    s1 = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (literal!(";\n")).clone()]);
                    r#str = dumpInitialEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_FOR_EQUATION { source: src, equations: xs1, range: e1, iter: s2, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    let _ = getSourceInformationStr(src.clone())?;
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  for ")).clone(), (s2.clone()).clone(), (literal!(" in ")).clone(), (s1.clone()).clone(), (literal!(" loop\n")).clone()]);
                    r#str = dumpEquationsStream(xs1.clone(), r#str.clone())?;
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  end for;\n")).clone()]);
                    r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_IF_EQUATION { equations3: xs2, equations2: Deref @ metamodelica::List::Cons { head: xs1, tail: trueBranches }, condition1: Deref @ metamodelica::List::Cons { head: e, tail: conds }, .. }, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), (literal!("  if ")).clone())?;
                    r#str = IOStream::append(r#str.clone(), (ExpressionBasics::printExpStr(e.clone())?).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!(" then\n")).clone())?;
                    r#str = dumpInitialEquationsStream(xs1.clone(), r#str.clone())?;
                    r#str = dumpIfEquationsStream(conds.clone(), trueBranches.clone(), r#str.clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("  else\n")).clone())?;
                    r#str = dumpInitialEquationsStream(xs2.clone(), r#str.clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("  end if;\n")).clone())?;
                    r#str = dumpInitialEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_NORETCALL { exp: e, .. }, tail: xs }, r#str) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(";\n")).clone()]);
                    r#str = dumpInitialEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_ASSERT { source: src, level: Deref @ DAE::Exp::ENUM_LITERAL { index: 1, .. }, message: e2, condition: e1 }, tail: xs }, r#str) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  assert(")).clone(), (s1.clone()).clone(), (literal!(",")).clone(), (s2.clone()).clone(), (literal!(")")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::INITIAL_TERMINATE { source: src, message: e1 }, tail: xs }, r#str) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    sourceStr = (getSourceInformationStr(src.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  terminate(")).clone(), (s1.clone()).clone(), (literal!(")")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    r#str = dumpEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = dumpInitialEquationsStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpConstraintStream(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElementLst.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::CONSTRAINT { constraints: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps }, .. }, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), (literal!("  ")).clone())?;
                    r#str = IOStream::append(r#str.clone(), stringDelimitList(List::map(exps.clone(), Arc::new(ExpressionBasics::printExpStr)), (literal!(";\n  ")).clone()))?;
                    r#str = IOStream::append(r#str.clone(), (literal!(";\n")).clone())?;
                    r#str = dumpConstraintStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = dumpConstraintStream(xs.clone(), r#str.clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

pub fn dumpDAEElementsStr(mut d: DAE::DAElist) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match d.clone() {
        DAE::DAElist { elementLst: ref l } => {
            let mut myStream: IOStream::IOStream;
            myStream = IOStream::create((literal!("")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = dumpElementsStream(l.clone(), myStream.clone())?;
            r#str = (IOStream::string(myStream.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpVarsStream(mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut printTypeDimension: bool, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = (::match_deref::match_deref! { match &((inElementLst.clone(), printTypeDimension.clone(), inStream.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            inStream.clone()
        },
        (Deref @ metamodelica::List::Cons { head: first, tail: rest }, _, r#str) => {
            let mut r#str = (*r#str).clone();
            r#str = dumpVarStream(first.clone(), printTypeDimension.clone(), r#str.clone())?;
            r#str = dumpVarsStream(rest.clone(), printTypeDimension.clone(), r#str.clone())?;
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStream)
}

pub fn daeTypeStr(mut inType: Arc<DAE::Type>) -> Result<ArcStr> {
    let mut outTypeStr: ArcStr = arcstr::literal!("");
    let mut typeAttrStr: ArcStr = arcstr::literal!("");
    (outTypeStr, typeAttrStr) = printTypeStr(inType.clone())?;
    if typeAttrStr.clone() != literal!("") {
        outTypeStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outTypeStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*typeAttrStr.clone()); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outTypeStr)
}

pub fn printTypeStr(mut inType: Arc<DAE::Type>) -> Result<(ArcStr, ArcStr)> {
    let mut outTypeStr: ArcStr = arcstr::literal!("");
    let mut outTypeAttrStr: ArcStr = arcstr::literal!("");
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut ty_vars: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
    (ty, ty_vars) = TypesDump::stripTypeVars(inType.clone());
    outTypeStr = (unparseType(ty.clone())?).clone();
    outTypeAttrStr = (List::toString(ty_vars.clone(), Arc::new(TypesDump::unparseVarAttr), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), false, 0)?).clone();
    Ok((outTypeStr, outTypeAttrStr))
}

pub fn dumpCallAttr(mut ca: Arc<DAE::CallAttributes>) -> Result<()> {
    let mut tpl: bool = false;
    let mut bi: bool = false;
    let mut impure_: bool = false;
    let mut isFunc: bool = false;
    let mut iType: DAE::InlineType = DAE::InlineType::AFTER_INDEX_RED_INLINE;
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut tailCall: DAE::TailCall = DAE::TailCall::NO_TAIL;
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(ca.clone()) {
        Deref @ DAE::CallAttributes { tailCall: __pa0, inlineType: __pa1, isFunctionPointerCall: __pa2, isImpure: __pa3, builtin: __pa4, tuple_: __pa5, ty: __pa6 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    tailCall = __pa0.clone();
    iType = __pa1.clone();
    isFunc = __pa2.clone();
    impure_ = __pa3.clone();
    bi = __pa4.clone();
    tpl = __pa5.clone();
    ty = __pa6.clone();
    println!("{}", (literal!("Call attributes: \n----------------------\n")).clone());
    (s1, s2) = printTypeStr(ty.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE-type: ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAE-type attributes :")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tuple_: ")); __mm_s.push_str(&*boolString(tpl.clone())); __mm_s.push_str(&*literal!(" builtin: ")); __mm_s.push_str(&*boolString(bi.clone())); __mm_s.push_str(&*literal!(" impure: ")); __mm_s.push_str(&*boolString(impure_.clone())); __mm_s.push_str(&*literal!(" isFunctionPointerCall: ")); __mm_s.push_str(&*boolString(isFunc.clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub fn dumpVarBindingStr(mut inBinding: Option<Arc<DAE::Exp>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inBinding.clone()) {
        Some(exp) => {
            let mut bind_str: ArcStr = arcstr::literal!("");
            bind_str = (ExpressionBasics::printExpStr(exp.clone())?).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*bind_str.clone()); ArcStr::from(__mm_s) }
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn dumpVarStream(mut inElement: Arc<DAE::Element>, mut printTypeDimension: bool, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElement.clone(), printTypeDimension.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { comment: cmt, variableAttributesOption: attr, binding, dims, ty, protection: vis, parallelism: prl, direction: dir, kind, componentRef: id, .. }, _, r#str) => {
                    let mut final_str: ArcStr = arcstr::literal!("");
                    let mut kind_str: ArcStr = arcstr::literal!("");
                    let mut dir_str: ArcStr = arcstr::literal!("");
                    let mut ty_str: ArcStr = arcstr::literal!("");
                    let mut ty_vars_str: ArcStr = arcstr::literal!("");
                    let mut dim_str: ArcStr = arcstr::literal!("");
                    let mut name_str: ArcStr = arcstr::literal!("");
                    let mut vis_str: ArcStr = arcstr::literal!("");
                    let mut par_str: ArcStr = arcstr::literal!("");
                    let mut cmt_str: ArcStr = arcstr::literal!("");
                    let mut attr_str: ArcStr = arcstr::literal!("");
                    let mut binding_str: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    final_str = (if (DAEUtil::getFinalAttr(attr.clone())) {literal!("final ")} else {literal!("")}).clone();
                    kind_str = (dumpKindStr(kind.clone())?).clone();
                    dir_str = (dumpDirectionStr(dir.clone())?).clone();
                    (ty_str, ty_vars_str) = printTypeStr(ty.clone())?;
                    dim_str = (unparseDimensions(dims.clone(), printTypeDimension.clone())?).clone();
                    name_str = (ComponentReferenceBasics::printComponentRefStr(id.clone())?).clone();
                    vis_str = (dumpVarVisibilityStr(vis.clone())?).clone();
                    par_str = (dumpVarParallelismStr(prl.clone())?).clone();
                    cmt_str = (dumpCommentAnnotationStr(cmt.clone())?).clone();
                    attr_str = (dumpVariableAttributesStr(attr.clone())?).clone();
                    binding_str = (dumpVarBindingStr(binding.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("  ")).clone(), (vis_str.clone()).clone(), (final_str.clone()).clone(), (par_str.clone()).clone(), (kind_str.clone()).clone(), (dir_str.clone()).clone(), (ty_str.clone()).clone(), (dim_str.clone()).clone(), (literal!(" ")).clone(), (name_str.clone()).clone(), (ty_vars_str.clone()).clone(), (attr_str.clone()).clone(), (binding_str.clone()).clone(), (cmt_str.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inStream.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

pub fn dumpAlgorithmStream(mut inElement: Arc<DAE::Element>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElement.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), (literal!("algorithm\n")).clone())?;
                    r#str = List::fold(stmts.clone(), Arc::new(ppStatementStream), r#str.clone());
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

pub fn dumpInitialAlgorithmStream(mut inElement: Arc<DAE::Element>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElement.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, r#str) => {
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), (literal!("initial algorithm\n")).clone())?;
                    r#str = List::fold(stmts.clone(), Arc::new(ppStatementStream), r#str.clone());
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

pub fn ppStatementStream(mut alg: Arc<DAE::Statement>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    let mut tmp: ArcStr = arcstr::literal!("");
    let mut hnd: i32 = 0;
    hnd = Print::saveAndClearBuf()?;
    ppStatement(alg.clone())?;
    outStream = IOStream::append(inStream.clone(), (Print::getString()?).clone())?;
    Print::restoreBuf(hnd.clone())?;
    Ok(outStream)
}

pub fn dumpFunctionStr(mut inElement: DAE::Function) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut s: ArcStr = arcstr::literal!("");
            let mut hnd: i32 = 0;
            hnd = Print::saveAndClearBuf()?;
            dumpFunction(inElement.clone())?;
            s = (Print::getString()?).clone();
            Print::restoreBuf(hnd.clone())?;
            Ok(s.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(literal!(""))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn dumpExtObjClassStr(mut inElement: Arc<DAE::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut hnd: i32 = 0;
                    hnd = Print::saveAndClearBuf()?;
                    dumpExtObjectClass(inElement.clone())?;
                    s = (Print::getString()?).clone();
                    Print::restoreBuf(hnd.clone())?;
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn dumpFunctionStream(mut inElement: DAE::Function, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = 'mc: {
        let __mc_input = (inElement.clone(), inStream.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { comment: c, isImpure, type_: t, functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, path: fpath, .. }, r#str) => {
                    let mut fstr: ArcStr = arcstr::literal!("");
                    let mut impureStr: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    r#str = IOStream::append(r#str.clone(), (dumpParallelismStr(t.clone())).clone())?;
                    fstr = AbsynUtil::pathStringNoQual(fpath.clone(), (literal!(".")).clone(), true, false)?;
                    impureStr = (if (isImpure.clone()) {literal!("impure ")} else {literal!("")}).clone();
                    r#str = IOStream::append(r#str.clone(), (impureStr.clone()).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("function ")).clone())?;
                    r#str = IOStream::append(r#str.clone(), (fstr.clone()).clone())?;
                    r#str = IOStream::append(r#str.clone(), (dumpCommentStr(c.clone())).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("\n")).clone())?;
                    r#str = dumpFunctionElementsStream(daeElts.clone(), r#str.clone())?;
                    r#str = IOStream::append(r#str.clone(), (dumpClassAnnotationStr(c.clone())?).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("end ")).clone())?;
                    r#str = IOStream::append(r#str.clone(), (fstr.clone()).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!(";\n\n")).clone())?;
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { language: Deref @ "builtin", .. }, .. }, tail: _ }, .. }, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { comment: c, isImpure, functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: ext_decl, body: daeElts }, tail: _ }, path: fpath, .. }, r#str) => {
                    let mut fstr: ArcStr = arcstr::literal!("");
                    let mut ext_decl_str: ArcStr = arcstr::literal!("");
                    let mut impureStr: ArcStr = arcstr::literal!("");
                    let mut ann_str: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    fstr = AbsynUtil::pathStringNoQual(fpath.clone(), (literal!(".")).clone(), true, false)?;
                    impureStr = (if (isImpure.clone()) {literal!("impure ")} else {literal!("")}).clone();
                    r#str = IOStream::append(r#str.clone(), (impureStr.clone()).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("function ")).clone())?;
                    r#str = IOStream::append(r#str.clone(), (fstr.clone()).clone())?;
                    r#str = IOStream::append(r#str.clone(), (dumpCommentStr(c.clone())).clone())?;
                    r#str = IOStream::append(r#str.clone(), (literal!("\n")).clone())?;
                    r#str = dumpFunctionElementsStream(daeElts.clone(), r#str.clone())?;
                    ext_decl_str = (dumpExtDeclStr(ext_decl.clone())?).clone();
                    ann_str = (dumpClassAnnotationStr(c.clone())?).clone();
                    r#str = IOStream::appendList(r#str.clone(), list![(literal!("\n  ")).clone(), (ext_decl_str.clone()).clone(), (literal!("\n")).clone(), (ann_str.clone()).clone(), (literal!("end ")).clone(), (fstr.clone()).clone(), (literal!(";\n\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::RECORD_CONSTRUCTOR { type_: tp, path: fpath, .. }, r#str) => {
                    let mut fstr: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    let false = (Flags::isSet(Flags::DISABLE_RECORD_CONSTRUCTOR_OUTPUT.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::PRINT_RECORD_TYPES.clone())? {
                        r#str = IOStream::append(r#str.clone(), (TypesDump::unparseType(tp.clone())?).clone())?;
                        r#str = IOStream::append(r#str.clone(), (literal!("\n")).clone())?;
                    } else {
                        fstr = AbsynUtil::pathStringNoQual(fpath.clone(), (literal!(".")).clone(), true, false)?;
                        r#str = IOStream::append(r#str.clone(), (literal!("function ")).clone())?;
                        r#str = IOStream::append(r#str.clone(), (fstr.clone()).clone())?;
                        r#str = IOStream::append(r#str.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" \"Automatically generated record constructor for ")); __mm_s.push_str(&*fstr.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone())?;
                        r#str = IOStream::append(r#str.clone(), (printRecordConstructorInputsStr(tp.clone())?).clone())?;
                        r#str = IOStream::append(r#str.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  output ")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(fpath.clone())?); __mm_s.push_str(&*literal!(" res;\n")); ArcStr::from(__mm_s) }).clone())?;
                        r#str = IOStream::append(r#str.clone(), (literal!("end ")).clone())?;
                        r#str = IOStream::append(r#str.clone(), (fstr.clone()).clone())?;
                        r#str = IOStream::append(r#str.clone(), (literal!(";\n\n")).clone())?;
                    }
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, r#str) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

pub fn dumpFunctionElementsStream(mut l: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inStream: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut outStream: IOStream::IOStream;
    outStream = dumpVarsStream(l.clone(), true, inStream.clone())?;
    outStream = List::fold(l.clone(), Arc::new(dumpAlgorithmStream), outStream.clone());
    Ok(outStream)
}

pub fn unparseVarKind(mut inVarKind: DAE::VarKind) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVarKind.clone() {
        DAE::VarKind::VARIABLE => literal!(""),
        DAE::VarKind::PARAM => literal!("parameter"),
        DAE::VarKind::CONST => literal!("const"),
        DAE::VarKind::DISCRETE => literal!("discrete"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseVarDirection(mut inVarDirection: DAE::VarDirection) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVarDirection.clone() {
        DAE::VarDirection::BIDIR => literal!(""),
        DAE::VarDirection::INPUT => literal!("input"),
        DAE::VarDirection::OUTPUT => literal!("output"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseVarInnerOuter(mut io: DAE::VarInnerOuter) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match io.clone() {
        DAE::VarInnerOuter::INNER { .. } => literal!("inner"),
        DAE::VarInnerOuter::OUTER { .. } => literal!("outer"),
        DAE::VarInnerOuter::INNER_OUTER { .. } => literal!("inner outer"),
        _ => literal!(""),
    })).clone();
    r#str
}

pub fn getSourceInformationStr(mut inSource: Arc<DAE::ElementSource>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ('mc: {
        let __mc_input = inSource.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (Flags::isSet(Flags::SHOW_EQUATION_SOURCE.clone())?) else { bail!("pattern mismatch") };
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ElementSource { info: _, partOfLst: po, instance: _, connectEquationOptLst: ceol, typeLst: _, operations: _, comment: cmt } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (cmtListToString(cmt.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" /* models: {")); __mm_s.push_str(&*stringDelimitList(List::map(po.clone(), Arc::new(withinString)), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); __mm_s.push_str(&*literal!(" connects: {")); __mm_s.push_str(&*stringDelimitList(connectsStr(ceol.clone())?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("} */")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outStr)
}

fn connectsStr(mut inLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStr: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStr = 'mc: {
        let __mc_input = inLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (c1, c2), tail: Deref @ metamodelica::List::Nil } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c1.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c2.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    Ok(list![(r#str.clone()).clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (c1, c2), tail: rest } => {
                    let mut slst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c1.clone())?); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(c2.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    slst = connectsStr(rest.clone())?;
                    Ok(cons(r#str.clone(), slst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStr)
}

fn withinString(mut w: Absyn::Within) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match w.clone() {
        Absyn::Within::TOP => {
            literal!("TOP")
        },
        Absyn::Within::WITHIN { path: ref p1 } => {
            AbsynUtil::pathString(p1.clone(), (literal!(".")).clone(), true, false)?
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn cmtListToString(mut inCmtLst: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    outStr = ('mc: {
        let __mc_input = inCmtLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: Deref @ metamodelica::List::Nil } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (dumpCommentAnnotationStr(Some(c.clone()))?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (dumpCommentAnnotationStr(Some(c.clone()))?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*cmtListToString(rest.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outStr)
}

pub fn clockKindString(mut cK: Arc<DAE::ClockKind>) -> Result<ArcStr> {
    let mut sOut: ArcStr = arcstr::literal!("");
    sOut = ((::match_deref::match_deref! { match &(cK.clone()) {
        Deref @ DAE::ClockKind::INFERRED_CLOCK => {
            literal!("Inferred Clock")
        },
        Deref @ DAE::ClockKind::RATIONAL_CLOCK { resolution: e2, intervalCounter: e1 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Rational Clock(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!("; ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::ClockKind::REAL_CLOCK { interval: e1 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Real Clock(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::ClockKind::EVENT_CLOCK { startInterval: e2, condition: e1 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Event Clock(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!("; ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        Deref @ DAE::ClockKind::SOLVER_CLOCK { solverMethod: e2, c: e1 } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Solver Clock(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!("; ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(sOut)
}

pub fn dumpDebugElementStr(mut inElement: Arc<DAE::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: c, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("VAR:  ")).clone(), (s1.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { source: src, componentRef: c, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    r#str = (stringAppend((s1.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALDEFINE { source: src, componentRef: c, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
                    r#str = (stringAppend((s1.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { source: src, scalar: e2, exp: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { source: src, cr2, cr1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ComponentReferenceBasics::printComponentRefStr(cr1.clone())?).clone();
                    s2 = (ComponentReferenceBasics::printComponentRefStr(cr2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("EQUEQUATION  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { source: src, array: e2, exp: e1, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ARRAY_EQUATION  ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { source: src, array: e2, exp: e1, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("INITIAL_ARRAY_EQUATION  ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { source: src, rhs: e2, lhs: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("COMPLEX_EQUATION  ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { source: src, rhs: e2, lhs: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("INITIAL_COMPLEX_EQUATION  ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*sourceStr.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::WHEN_EQUATION { source: src, condition: e1, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("WHEN_EQUATION:  ")).clone(), (s1.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::IF_EQUATION { source: src, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("IF_EQUATION:  ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_IF_EQUATION { source: src, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("INITIAL_IF_EQUATION:  ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALEQUATION { source: src, exp2: e2, exp1: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("INITIALEQUATION  ")).clone(), (s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ALGORITHM { source: src, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("ALGO  ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIALALGORITHM { source: src, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("INITIALALGORITHM  ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMP { dAElist: elst, source: src, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = stringDelimitList(List::map(elst.clone(), Arc::new(dumpDebugElementStr)), (literal!("\n")).clone());
                    r#str = stringAppendList(list![(literal!("COMP  ")).clone(), (s1.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EXTOBJECTCLASS { source: src, path } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    r#str = stringAppendList(list![(literal!("EXTOBJ  ")).clone(), (s1.clone()).clone(), (literal!("  ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ASSERT { source: src, message: e2, condition: e1, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  assert(")).clone(), (s1.clone()).clone(), (literal!(",")).clone(), (s2.clone()).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_ASSERT { source: src, message: e2, condition: e1, .. } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  /* initial */ assert(")).clone(), (s1.clone()).clone(), (literal!(",")).clone(), (s2.clone()).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::TERMINATE { source: src, message: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  terminate(")).clone(), (s1.clone()).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::INITIAL_TERMINATE { source: src, message: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  /* initial */ terminate(")).clone(), (s1.clone()).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::REINIT { source: src, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  reinit(")).clone(), (literal!(") ")).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::NORETCALL { source: src, exp: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut sourceStr: ArcStr = arcstr::literal!("");
                    let mut cmt: Arc<metamodelica::List<Arc<SCode::Comment>>> = metamodelica::nil();
                    cmt = ElementSource::getComments(src.clone())?;
                    sourceStr = (cmtListToString(cmt.clone())?).clone();
                    s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("  ")).clone(), (s1.clone()).clone(), (sourceStr.clone()).clone(), (literal!(";\n")).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("#UNKNOWN_EQUATION#"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn getOptionWithConcatStr<Type_a: Clone + 'static>(mut inTypeAOption: Option<Type_a>, mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>, mut inString: ArcStr) -> Result<ArcStr> {
    pub type FuncTypeType_aToString<Type_a: Clone> = fn(Type_a) -> Result<ArcStr>;

    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match (inTypeAOption.clone(), inFuncTypeTypeAToString.clone(), inString.clone()) {
        (Some(mut a), mut r, mut default_str) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut str_1: ArcStr = arcstr::literal!("");
            r#str = r(a.clone())?;
            str_1 = (stringAppend((default_str.clone()).clone(), (r#str.clone()).clone())).clone();
            str_1.clone()
        },
        (None, _, _) => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

