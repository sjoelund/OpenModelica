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

use crate::MidCode;
use openmodelica_util_datatypes_basic::List;

/*
Longjmps are not allowed to land in the same function.
This is handled in midtomid.
Handling it here allows other tranformations to
deal with goto instead of longjmp, which might enable
further transformation.

pushpopjmp possible.
can remove push-pop -jmp pairs if there is no possible longjmp in between.

Typechecking possible.
Useful for correctness of midcode transformations.

Normalisation possble. (AKA canonicalisation)
Probably essential to simplify other transformations.
Remove greater than comparisons and similar.

Inlining possible.
Important catalyst for other optimisations.

Common subexpression elimination possible.
But requires some data flow and side effect analysis.
Some SSA variables and purity marked functions perhaps.

*/
pub fn longJmpGoto(mut oldFunction: MidCode::Function) -> Result<MidCode::Function> {
    let mut newFunction: MidCode::Function;
    let mut newBody: Arc<metamodelica::List<MidCode::Block>> = metamodelica::nil();
    let mut oldBody: Arc<metamodelica::List<MidCode::Block>> = metamodelica::nil();
    let mut newBlock: MidCode::Block;
    let mut oldBlock: MidCode::Block;
    let mut node: i32 = 0;
    let mut jump: i32 = 0;
    let mut jumps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodes_tmp: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut checkedNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tasks: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    let mut tasks_tmp: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
    oldBody = oldFunction.body.clone();
    newBody = metamodelica::nil();
    checkedNodes = list![oldFunction.entryId.clone()];
    tasks = list![(metamodelica::nil(), oldFunction.entryId.clone())];
    while !(tasks.clone().is_empty()) {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(tasks.clone()) {
            Deref @ metamodelica::List::Cons { head: (__pa0, __pa1), tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        jumps = __pa0.clone();
        node = __pa1.clone();
        tasks = __pa2.clone();
        oldBlock = lookupId(oldBody.clone(), node.clone())?;
        newBlock = oldBlock.clone();
        if isPushJmp(oldBlock.terminator.clone()) {
            jumps = cons(listHead(getSuccessors(oldBlock.clone())?)?, jumps.clone());
        } else if isLongJmp(oldBlock.terminator.clone()) && !(jumps.clone().is_empty()) {
            let __pa3 = ::match_deref::match_deref! { match &(jumps.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa3, tail: _ } => __pa3.clone(),
                _ => bail!("pattern mismatch"),
            } };
            jump = __pa3.clone();
            newBlock = MidCode::Block { terminator: MidCode::Terminator::GOTO { next: jump.clone() }, stmts: oldBlock.stmts.clone(), id: oldBlock.id.clone() };
        } else if isPopJmp(oldBlock.terminator.clone()) {
            let __pa4 = ::match_deref::match_deref! { match &(jumps.clone()) {
                Deref @ metamodelica::List::Cons { head: _, tail: __pa4 } => __pa4.clone(),
                _ => bail!("pattern mismatch"),
            } };
            jumps = __pa4.clone();
        }
        newBody = cons(newBlock.clone(), newBody.clone());
        nodes_tmp = List::setDifference(getSuccessors(oldBlock.clone())?, checkedNodes.clone())?;
        checkedNodes = listAppend(nodes_tmp.clone(), checkedNodes.clone());
        tasks_tmp = {
        let mut __acc: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, i32)>> = metamodelica::nil();
        for mut node_tmp in (nodes_tmp.clone()).into_iter().cloned() {
            let __x = (jumps.clone(), node_tmp.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        tasks = listAppend(tasks_tmp.clone(), tasks.clone());
    }
    newBody = newBody.clone().reverse();
    newFunction = MidCode::Function { exitId: oldFunction.exitId.clone(), entryId: oldFunction.entryId.clone(), body: newBody.clone(), outputs: oldFunction.outputs.clone(), inputs: oldFunction.inputs.clone(), localBufPtrs: oldFunction.localBufPtrs.clone(), localBufs: oldFunction.localBufs.clone(), locals: oldFunction.locals.clone(), name: oldFunction.name.clone() };
    Ok(newFunction)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn lookupId(mut blocks: Arc<metamodelica::List<MidCode::Block>>, mut id: i32) -> Result<MidCode::Block> {
    let mut block_: MidCode::Block;
    let mut blocks_local: Arc<metamodelica::List<MidCode::Block>> = metamodelica::nil();
    let mut block_local: MidCode::Block;
    block_ = (::match_deref::match_deref! { match &(blocks.clone()) {
        Deref @ metamodelica::List::Cons { head: block_local, tail: _ } if (block_local.id.clone() == id.clone()) => block_local.clone(),
        Deref @ metamodelica::List::Cons { head: _, tail: blocks_local } => lookupId(blocks_local.clone(), id.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(block_)
}

fn getSuccessors(mut block_: MidCode::Block) -> Result<Arc<metamodelica::List<i32>>> {
    let mut neighbours: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut l0: i32 = 0;
    let mut l1: i32 = 0;
    let mut switchList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    neighbours = (match block_.terminator.clone() {
        MidCode::Terminator::GOTO { next: mut l0 } => list![l0.clone()],
        MidCode::Terminator::BRANCH { condition: _, onTrue: mut l0, onFalse: mut l1 } => list![l0.clone(), l1.clone()],
        MidCode::Terminator::CALL { func: _, builtin: _, inputs: _, outputs: _, next: mut l0 } => list![l0.clone()],
        MidCode::Terminator::RETURN => metamodelica::nil(),
        MidCode::Terminator::SWITCH { condition: _, cases: ref switchList } => {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut x in (switchList.clone()).into_iter().cloned() {
            let __x = tupleSnd(x.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        MidCode::Terminator::LONGJMP => metamodelica::nil(),
        MidCode::Terminator::PUSHJMP { old_buf: _, new_buf: _, next: mut l0 } => list![l0.clone()],
        MidCode::Terminator::POPJMP { old_buf: _, next: mut l0 } => list![l0.clone()],
        _ => bail!("match: no arm matched"),
    });
    Ok(neighbours)
}

fn tupleSnd(mut t: (i32, i32)) -> i32 {
    let mut i: i32 = 0;
    (_, i) = t.clone();
    i
}

fn isLongJmp(mut t: MidCode::Terminator) -> bool {
    let mut b: bool = false;
    b = (match t.clone() {
        MidCode::Terminator::LONGJMP => true,
        _ => false,
    });
    b
}

fn isPushJmp(mut t: MidCode::Terminator) -> bool {
    let mut b: bool = false;
    b = (match t.clone() {
        MidCode::Terminator::PUSHJMP { old_buf: _, new_buf: _, next: _ } => true,
        _ => false,
    });
    b
}

fn isPopJmp(mut t: MidCode::Terminator) -> bool {
    let mut b: bool = false;
    b = (match t.clone() {
        MidCode::Terminator::POPJMP { old_buf: _, next: _ } => true,
        _ => false,
    });
    b
}

