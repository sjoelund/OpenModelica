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
use openmodelica_frontend::DAEDump;
use openmodelica_frontend_types::DAE;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Program {
    pub name: ArcStr,
    pub functions: Arc<metamodelica::List<Function>>,
}

pub type PROGRAM = Program;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Var {
    pub name: ArcStr,
    pub ty: Arc<DAE::Type>,
    /// Used for setjmp semantics in C.
    pub volatile: bool,
}

pub type VAR = Var;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarBuf {
    pub name: ArcStr,
}

pub type VARBUF = VarBuf;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarBufPtr {
    pub name: ArcStr,
}

pub type VARBUFPTR = VarBufPtr;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OutVar {
    OUT_VAR {
        var: Var,
    },
    OUT_WILD,
}
pub use self::OutVar::{OUT_VAR,OUT_WILD};

pub fn varString(mut var: Var) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*DAEDump::daeTypeStr(var.ty.clone())?); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*var.name.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Function {
    pub name: Arc<Absyn::Path>,
    pub locals: Arc<metamodelica::List<Var>>,
    pub localBufs: Arc<metamodelica::List<VarBuf>>,
    pub localBufPtrs: Arc<metamodelica::List<VarBufPtr>>,
    pub inputs: Arc<metamodelica::List<Var>>,
    pub outputs: Arc<metamodelica::List<Var>>,
    pub body: Arc<metamodelica::List<Block>>,
    pub entryId: i32,
    pub exitId: i32,
}

pub type FUNCTION = Function;


/// Basic block.
///  No control flow within block.
///  Can branch or jump on exit, called the block's terminator.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Block {
    pub id: i32,
    pub stmts: Arc<metamodelica::List<Stmt>>,
    pub terminator: Terminator,
}

pub type BLOCK = Block;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Terminator {
    GOTO {
        next: i32,
    },
    BRANCH {
        condition: Var,
        onTrue: i32,
        onFalse: i32,
    },
    CALL {
        func: Arc<Absyn::Path>,
        builtin: bool,
        inputs: Arc<metamodelica::List<Var>>,
        outputs: Arc<metamodelica::List<OutVar>>,
        next: i32,
    },
    RETURN,
    SWITCH {
        condition: Var,
        cases: Arc<metamodelica::List<(i32, i32)>>,
    },
    /// used for fail() stmts
    LONGJMP,
    /// used for match-continue fail() handling
    PUSHJMP {
        /// where to save old jmp_buf
        old_buf: VarBufPtr,
        /// what to use as new jmp_buf
        new_buf: VarBuf,
        /// where to goto next and the setjmp target
        next: i32,
    },
    /// used for match-continue fail() handling
    POPJMP {
        /// what to reset to
        old_buf: VarBufPtr,
        next: i32,
    },
    ASSERT {
        condition: Var,
        message: Var,
        level: Var,
        next: i32,
    },
    TERMINATE {
        message: Var,
    },
}
pub use self::Terminator::{GOTO,BRANCH,CALL,RETURN,SWITCH,LONGJMP,PUSHJMP,POPJMP,ASSERT,TERMINATE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stmt {
    NOP,
    ASSIGN {
        dest: Var,
        src: RValue,
    },
}
pub use self::Stmt::{NOP,ASSIGN};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RValue {
    VARIABLE {
        src: Var,
    },
    UNARYOP {
        op: UnaryOp,
        src: Var,
    },
    BINARYOP {
        op: BinaryOp,
        lsrc: Var,
        rsrc: Var,
    },
    LITERALINTEGER {
        value: i32,
    },
    LITERALREAL {
        value: metamodelica::Real,
    },
    LITERALBOOLEAN {
        value: bool,
    },
    LITERALSTRING {
        value: ArcStr,
    },
    LITERALMETATYPE {
        elements: Arc<metamodelica::List<Var>>,
        ty: Arc<DAE::Type>,
    },
    UNIONTYPEVARIANT {
        src: Var,
    },
    ISSOME {
        src: Var,
    },
    ISCONS {
        src: Var,
    },
    /// get value from metamodelica object
    METAFIELD {
        src: Var,
        index: i32,
        /// type of value
        ty: Arc<DAE::Type>,
    },
}
pub use self::RValue::{VARIABLE,UNARYOP,BINARYOP,LITERALINTEGER,LITERALREAL,LITERALBOOLEAN,LITERALSTRING,LITERALMETATYPE,UNIONTYPEVARIANT,ISSOME,ISCONS,METAFIELD};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOp {
    MOVE,
    UMINUS,
    NOT,
    UNBOX,
    BOX,
}
pub use self::UnaryOp::{MOVE,UMINUS,NOT,UNBOX,BOX};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    LESS,
    LESSEQ,
    GREATER,
    GREATEREQ,
    EQUAL,
    NEQUAL,
}
pub use self::BinaryOp::{ADD,SUB,MUL,DIV,POW,LESS,LESSEQ,GREATER,GREATEREQ,EQUAL,NEQUAL};

