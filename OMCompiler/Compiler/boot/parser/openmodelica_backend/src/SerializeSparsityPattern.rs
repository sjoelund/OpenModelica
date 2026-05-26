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

use crate::SimCode;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn serialize(mut code: SimCode::SimCode) -> Result<ArcStr> {
    let mut dummy: ArcStr = literal!("");
    let mut columnPointers: metamodelica::Array<i32>;
    let mut rowIndices: metamodelica::Array<i32>;
    let mut columns: metamodelica::Array<i32>;
    let mut fname: ArcStr = arcstr::literal!("");
    let mut pattern: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut colorList: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    for mut jac in &*code.jacobianMatrices.clone() {
        let mut jac = jac.clone();
        if jac.isAdjoint.clone() {
            pattern = jac.sparsityT.clone();
            if !(jac.coloredRows.clone().is_empty()) {
                colorList = jac.coloredRows.clone();
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SerializeSparsityPattern.serialize")); __mm_s.push_str(&*literal!(" failed because no row coloring for the adjoint jacobian exists.")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
        } else {
            pattern = jac.sparsity.clone();
            colorList = jac.coloredCols.clone();
        }
        if !(pattern.clone().is_empty()) {
            fname = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*code.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Jac")); __mm_s.push_str(&*jac.matrixName.clone()); __mm_s.push_str(&*literal!(".bin")); ArcStr::from(__mm_s) }).clone();
            columnPointers = metamodelica::arrayFromVec(cons(0, {
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut column in (pattern.clone()).into_iter().cloned() {
            let __x = (Util::tuple22(column.clone()).len() as i32);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
            rowIndices = metamodelica::arrayFromVec(List::flatten({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut column in (pattern.clone()).into_iter().cloned() {
            let __x = Util::tuple22(column.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
            serializeJacobian((fname.clone()).clone(), (columnPointers.clone().borrow().len() as i32), (rowIndices.clone().borrow().len() as i32), columnPointers.clone(), rowIndices.clone());
            for mut color in &*colorList.clone() {
                let mut color = color.clone();
                columns = metamodelica::arrayFromVec(color.clone().into_iter().cloned().collect());
                serializeColor((fname.clone()).clone(), (columns.clone().borrow().len() as i32), columns.clone());
            }
        }
    }
    Ok(dummy)
}

// *********************
// write to binary stuff
// *********************
fn serializeJacobian(mut name: ArcStr, mut numCols: i32, mut nnz: i32, mut colPtrs: metamodelica::Array<i32>, mut rowInds: metamodelica::Array<i32>) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("serializeJ"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "name", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "numCols", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "nnz", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "colPtrs", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "rowInds", subscripts: Nil } }, tail: Nil } } } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\n  extern FILE* omc_fopen(const char *filename, const char *mode);\n  extern size_t omc_fwrite(void *buffer, size_t size, size_t count, FILE *stream);\n\n  static void serializeJ(const char* name, int numCols, int nnz, modelica_metatype colPtrs, modelica_metatype rowInds)\n  {\n    unsigned int i, j;\n    size_t count;\n    FILE* pFile = omc_fopen(name, \\\"wb\\\");\n    if (pFile == NULL) {\n      throwStreamPrint(NULL, \\\"Could not open sparsity pattern file %s.\\\", name);\n    }\n\n    /* compute and write sparsePattern->leadindex */\n    j = 0;\n    for (i = 0; i < numCols; i++) {\n      j += (unsigned int) MMC_UNTAGFIXNUM(MMC_STRUCTDATA(colPtrs)[i]);\n      count = omc_fwrite(&j, sizeof(unsigned int), 1, pFile);\n      if (count != 1) {\n        throwStreamPrint(NULL, \\\"Error while writing sparsePattern->leadindex. Expected %d, got %zu\\\", 1, count);\n      }\n    }\n\n    /* write sparsePattern->index */\n    for (i = 0; i < nnz; i++) {\n      j = (unsigned int) MMC_UNTAGFIXNUM(MMC_STRUCTDATA(rowInds)[i]);\n      count = omc_fwrite(&j, sizeof(unsigned int), 1, pFile);\n      if (count != 1) {\n        throwStreamPrint(NULL, \\\"Error while writing sparsePattern->index. Expected %d, got %zu\\\", 1, count);\n      }\n    }\n\n    fclose(pFile);\n  }\n  " }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/SimCode/SerializeSparsityPattern.mo", isReadOnly: false, lineNumberStart: 91, columnNumberStart: 83, lineNumberEnd: 125, columnNumberEnd: 3, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/SimCode/SerializeSparsityPattern.mo", isReadOnly: false, lineNumberStart: 91, columnNumberStart: 76, lineNumberEnd: 125, columnNumberEnd: 3, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

fn serializeColor(mut name: ArcStr, mut size: i32, mut columns: metamodelica::Array<i32>) -> () {
    todo!(); // ExternalSection { decl: ExternalDecl { funcName: Some("serializeC"), lang: Some("C"), output_: None, args: Cons { head: CREF { componentRef: CREF_IDENT { name: "name", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "size", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "columns", subscripts: Nil } }, tail: Nil } } }, annotation_: Some(Annotation { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(Modification { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\n  extern FILE* omc_fopen(const char *filename, const char *mode);\n  extern size_t omc_fwrite(void *buffer, size_t size, size_t count, FILE *stream);\n\n  static void serializeC(const char* name, int size, modelica_metatype columns)\n  {\n    unsigned int i, j;\n    size_t count;\n    FILE* pFile = fopen(name, \\\"ab\\\");\n    if (pFile == NULL) {\n      throwStreamPrint(NULL, \\\"Could not open sparsity pattern file %s.\\\", name);\n    }\n\n    /* write sparsePattern->colorCols */\n    for (i = 0; i < size; i++) {\n      j = (unsigned int) MMC_UNTAGFIXNUM(MMC_STRUCTDATA(columns)[i]);\n      count = omc_fwrite(&j, sizeof(unsigned int), 1, pFile);\n      if (count != 1) {\n        throwStreamPrint(NULL, \\\"Error while writing sparsePattern->colorCols. Expected %d, got %zu\\\", 1, count);\n      }\n    }\n\n    fclose(pFile);\n  }\n  " }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/SimCode/SerializeSparsityPattern.mo", isReadOnly: false, lineNumberStart: 132, columnNumberStart: 66, lineNumberEnd: 156, columnNumberEnd: 3, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/SimCode/SerializeSparsityPattern.mo", isReadOnly: false, lineNumberStart: 132, columnNumberStart: 59, lineNumberEnd: 156, columnNumberEnd: 3, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    ()
}

