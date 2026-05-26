// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

fn getInteger<T: Clone>(a: T) -> i32 {
    let mut i: i32;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_cast_int"), lang: Some("C"), output_: Some(CREF_IDENT { name: "i", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "a", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_integer omc_cast_int(modelica_metatype a)\n{\n  return MMC_UNTAGFIXNUM(a);\n}" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 339, columnNumberStart: 52, lineNumberEnd: 343, columnNumberEnd: 2, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 339, columnNumberStart: 45, lineNumberEnd: 343, columnNumberEnd: 2, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    i
}

pub fn getList<TIN: Clone, TOUT: Clone>(iany: TIN) -> Arc<metamodelica::List<TOUT>> {
    let mut oany: Arc<metamodelica::List<TOUT>>;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_get_list"), lang: Some("C"), output_: Some(CREF_IDENT { name: "oany", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "iany", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_metatype omc_get_list(modelica_metatype any)\n{\n  return any;\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 391, columnNumberStart: 60, lineNumberEnd: 396, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 391, columnNumberStart: 53, lineNumberEnd: 396, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    oany
}

pub fn getListElement<TIN: Clone, TOUT: Clone>(iany: TIN, offset: i32) -> TOUT {
    let mut oany: TOUT;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_get_list_element"), lang: Some("C"), output_: Some(CREF_IDENT { name: "oany", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "iany", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "offset", subscripts: Nil } }, tail: Nil } }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_metatype omc_get_list_element(modelica_metatype any, modelica_integer offset)\n{\n  return boxptr_listGet(NULL, any, mmc_mk_icon(offset));\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 403, columnNumberStart: 76, lineNumberEnd: 408, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 403, columnNumberStart: 69, lineNumberEnd: 408, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    oany
}

fn getReal<T: Clone>(a: T) -> f64 {
    let mut r: f64;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_cast_real"), lang: Some("C"), output_: Some(CREF_IDENT { name: "r", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "a", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_real omc_cast_real(modelica_metatype a)\n{\n  return (double) mmc_prim_get_real(a);\n}" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 349, columnNumberStart: 53, lineNumberEnd: 353, columnNumberEnd: 2, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 349, columnNumberStart: 46, lineNumberEnd: 353, columnNumberEnd: 2, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    r
}

pub fn getRecordComponent<TIN: Clone, TOUT: Clone>(iany: TIN, offset: i32) -> TOUT {
    let mut oany: TOUT;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_get_record_component"), lang: Some("C"), output_: Some(CREF_IDENT { name: "oany", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "iany", subscripts: Nil } }, tail: Cons { head: CREF { componentRef: CREF_IDENT { name: "offset", subscripts: Nil } }, tail: Nil } }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_metatype omc_get_record_component(modelica_metatype any, modelica_integer offset)\n{\n  mmc_uint_t hdr;\n  mmc_sint_t numslots;\n  mmc_uint_t ctor;\n  mmc_sint_t i;\n  modelica_metatype out = mmc_mk_nil();\n  hdr = MMC_GETHDR(any);\n  numslots = MMC_HDRSLOTS(hdr);\n  ctor = MMC_HDRCTOR(hdr);\n  if (numslots > 0 && ctor > 1)\n  {\n     out = MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(any),offset+1));\n  }\n  return out;\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 316, columnNumberStart: 80, lineNumberEnd: 333, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 316, columnNumberStart: 73, lineNumberEnd: 333, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    oany
}

pub fn getRecordNames<T: Clone>(any: T) -> Arc<metamodelica::List<ArcStr>> {
    let mut nameAndComponentsNames: Arc<metamodelica::List<ArcStr>> = getRecordNamesHelper(any.clone()).reverse();
    nameAndComponentsNames
}

fn getRecordNamesHelper<T: Clone>(any: T) -> Arc<metamodelica::List<ArcStr>> {
    let mut nameAndComponentsNames: Arc<metamodelica::List<ArcStr>>;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_get_record_names"), lang: Some("C"), output_: Some(CREF_IDENT { name: "nameAndComponentsNames", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_metatype omc_get_record_names(modelica_metatype any)\n{\n  mmc_uint_t hdr;\n  mmc_sint_t numslots;\n  mmc_uint_t ctor;\n  mmc_sint_t i;\n  modelica_metatype lst = mmc_mk_nil();\n  hdr = MMC_GETHDR(any);\n  numslots = MMC_HDRSLOTS(hdr);\n  ctor = MMC_HDRCTOR(hdr);\n  if (numslots > 0 && ctor > 1)\n  {\n     struct record_description * desc = MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(any),1));\n     /* add the record name */\n     lst = mmc_mk_cons(mmc_mk_scon(desc->name), lst);\n     /* add the component names */\n     for (i = 2; i <= numslots; i++)\n       lst = mmc_mk_cons(mmc_mk_scon(desc->fieldNames[i-2]), lst);\n  }\n  return lst;\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 287, columnNumberStart: 85, lineNumberEnd: 309, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 287, columnNumberStart: 78, lineNumberEnd: 309, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    nameAndComponentsNames
}

fn getSome<TIN: Clone, TOUT: Clone>(a: TIN) -> TOUT {
    let mut o: TOUT;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_get_some"), lang: Some("C"), output_: Some(CREF_IDENT { name: "o", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "a", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_metatype omc_get_some(modelica_metatype any)\n{\n  return (MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(any),1)));\n}" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 369, columnNumberStart: 52, lineNumberEnd: 373, columnNumberEnd: 2, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 369, columnNumberStart: 45, lineNumberEnd: 373, columnNumberEnd: 2, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    o
}

fn getString<T: Clone>(a: T) -> ArcStr {
    let mut s: ArcStr;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_cast_string"), lang: Some("C"), output_: Some(CREF_IDENT { name: "s", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "a", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_string omc_cast_string(modelica_metatype a)\n{\n  return MMC_STRINGDATA(a);\n}" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 359, columnNumberStart: 55, lineNumberEnd: 363, columnNumberEnd: 2, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 359, columnNumberStart: 48, lineNumberEnd: 363, columnNumberEnd: 2, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    s
}

pub fn getTupleSize<T: Clone>(any: T) -> i32 {
    let mut sz: i32;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_get_tuple_size"), lang: Some("C"), output_: Some(CREF_IDENT { name: "sz", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nmodelica_integer omc_get_tuple_size(modelica_metatype any)\n{\n  mmc_sint_t numslots = MMC_HDRSLOTS(MMC_GETHDR(any));\n  return numslots;\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 379, columnNumberStart: 63, lineNumberEnd: 385, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 379, columnNumberStart: 56, lineNumberEnd: 385, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    sz
}

pub fn isArray<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_array"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_array(modelica_metatype any)\n{\n  mmc_uint_t hdr;\n  mmc_sint_t numslots;\n  mmc_uint_t ctor;\n  hdr = MMC_GETHDR(any);\n  numslots = MMC_HDRSLOTS(hdr);\n  ctor = MMC_HDRCTOR(hdr);\n  return (numslots >= 0 && ctor == MMC_ARRAY_TAG);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 169, columnNumberStart: 56, lineNumberEnd: 180, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 169, columnNumberStart: 49, lineNumberEnd: 180, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isCons<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_cons"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_cons(modelica_metatype any)\n{\n  mmc_uint_t hdr;\n  mmc_sint_t numslots;\n  mmc_uint_t ctor;\n  hdr = MMC_GETHDR(any);\n  numslots = MMC_HDRSLOTS(hdr);\n  ctor = MMC_HDRCTOR(hdr);\n  return (numslots == 2 && ctor == 1);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 265, columnNumberStart: 55, lineNumberEnd: 276, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 265, columnNumberStart: 48, lineNumberEnd: 276, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isInteger<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_integer"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_integer(modelica_metatype any)\n{\n  return MMC_IS_INTEGER(any);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 136, columnNumberStart: 58, lineNumberEnd: 141, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 136, columnNumberStart: 51, lineNumberEnd: 141, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isNONE<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_none"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_none(modelica_metatype any)\n{\n  mmc_uint_t hdr;\n  mmc_sint_t numslots;\n  mmc_uint_t ctor;\n  hdr = MMC_GETHDR(any);\n  numslots = MMC_HDRSLOTS(hdr);\n  ctor = MMC_HDRCTOR(hdr);\n  return (numslots == 0 && ctor == 1);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 220, columnNumberStart: 55, lineNumberEnd: 231, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 220, columnNumberStart: 48, lineNumberEnd: 231, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isNil<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_nil"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_nil(modelica_metatype any)\n{\n  return (MMC_GETHDR(any) == MMC_NILHDR);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 254, columnNumberStart: 54, lineNumberEnd: 259, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 254, columnNumberStart: 47, lineNumberEnd: 259, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isReal<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_real"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_real(modelica_metatype any)\n{\n  return (MMC_GETHDR(any) == MMC_REALHDR);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 147, columnNumberStart: 55, lineNumberEnd: 152, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 147, columnNumberStart: 48, lineNumberEnd: 152, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isRecord<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_record"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_record(modelica_metatype any)\n{\n  mmc_uint_t hdr;\n  mmc_sint_t numslots;\n  mmc_uint_t ctor;\n  hdr = MMC_GETHDR(any);\n  numslots = MMC_HDRSLOTS(hdr);\n  ctor = MMC_HDRCTOR(hdr);\n  return (numslots > 0 && ctor > 1);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 186, columnNumberStart: 57, lineNumberEnd: 197, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 186, columnNumberStart: 50, lineNumberEnd: 197, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isSOME<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_some"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_some(modelica_metatype any)\n{\n  mmc_uint_t hdr;\n  mmc_sint_t numslots;\n  mmc_uint_t ctor;\n  hdr = MMC_GETHDR(any);\n  numslots = MMC_HDRSLOTS(hdr);\n  ctor = MMC_HDRCTOR(hdr);\n  return (numslots == 1 && ctor == 1);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 237, columnNumberStart: 55, lineNumberEnd: 248, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 237, columnNumberStart: 48, lineNumberEnd: 248, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isString<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_string"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_string(modelica_metatype any)\n{\n  return (MMC_HDRISSTRING(MMC_GETHDR(any)));\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 158, columnNumberStart: 57, lineNumberEnd: 163, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 158, columnNumberStart: 50, lineNumberEnd: 163, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn isTuple<T: Clone>(any: T) -> bool {
    let mut b: bool;
    todo!(); // ExternalSection { decl: EXTERNALDECL { funcName: Some("omc_is_tuple"), lang: Some("C"), output_: Some(CREF_IDENT { name: "b", subscripts: Nil }), args: Cons { head: CREF { componentRef: CREF_IDENT { name: "any", subscripts: Nil } }, tail: Nil }, annotation_: Some(ANNOTATION { elementArgs: Cons { head: MODIFICATION { finalPrefix: false, eachPrefix: NON_EACH, path: IDENT { name: "Include" }, modification: Some(CLASSMOD { elementArgLst: Nil, eqMod: EQMOD { exp: STRING { value: "\nint omc_is_tuple(modelica_metatype any)\n{\n  mmc_uint_t hdr;\n  mmc_sint_t numslots;\n  mmc_uint_t ctor;\n  hdr = MMC_GETHDR(any);\n  numslots = MMC_HDRSLOTS(hdr);\n  ctor = MMC_HDRCTOR(hdr);\n  return (numslots > 0 && ctor == 0);\n}\n" }, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 203, columnNumberStart: 56, lineNumberEnd: 214, columnNumberEnd: 1, lastModification: 0.0 } } }), comment: None, info: SourceInfo { fileName: "/home/martin/OpenModelica/OMCompiler/Compiler/Util/JSONExt.mo", isReadOnly: false, lineNumberStart: 203, columnNumberStart: 49, lineNumberEnd: 214, columnNumberEnd: 1, lastModification: 0.0 } }, tail: Nil } }) }, annotation: None }
    b
}

pub fn serialize<T: Clone>(any: T, filter: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut s: ArcStr = literal!("");
    let mut name: ArcStr;
    let mut components: Arc<metamodelica::List<ArcStr>>;
    let mut lst: Arc<metamodelica::List<ArcStr>>;
    let mut no: i32 = 1;
    if isInteger(any.clone()) {
        s = (intString(getInteger(any.clone()))).clone();
        return Ok(s);
    }
    if isReal(any.clone()) {
        s = (realString(getReal(any.clone()))).clone();
        return Ok(s);
    }
    if isString(any.clone()) {
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\\\"")); __mm_s.push_str(&*getString(any.clone())); __mm_s.push_str(&*literal!("\\\"")); ArcStr::from(__mm_s) }).clone();
        return Ok(s);
    }
    if isRecord(any.clone()) {
        components = getRecordNames(any.clone());
        let metamodelica::List::Cons { head: __pa0, tail: __pa1 } = &(components.clone()) else { bail!("pattern mismatch") };
        name = __pa0.clone();
        components = __pa1.clone();
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{\\\"")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\\\":{")); ArcStr::from(__mm_s) }).clone();
        no = 1;
        lst = metamodelica::nil();
        for c in &*components.clone() {
            if !(listMember((c.clone()).clone(), filter.clone())) {
                lst = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\\\"")); __mm_s.push_str(&*c.clone()); __mm_s.push_str(&*literal!("\\\":")); __mm_s.push_str(&*serialize(getRecordComponent(any.clone(), no.clone()), filter.clone())?); ArcStr::from(__mm_s) }, lst.clone());
            }
            no = no.clone() + 1;
        }
        lst = lst.clone().reverse();
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*stringDelimitList(lst.clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}}")); ArcStr::from(__mm_s) }).clone();
        return Ok(s);
    }
    if isNil(any.clone()) {
        s = (literal!("[]")).clone();
        return Ok(s);
    }
    if isCons(any.clone()) {
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("[")); ArcStr::from(__mm_s) }).clone();
        no = 1;
        lst = metamodelica::nil();
        for c in &*getList(any.clone()) {
            lst = cons(serialize(c.clone(), filter.clone())?, lst.clone());
        }
        lst = lst.clone().reverse();
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*stringDelimitList(lst.clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        return Ok(s);
    }
    if isNONE(any.clone()) {
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("[]")); ArcStr::from(__mm_s) }).clone();
        return Ok(s);
    }
    if isSOME(any.clone()) {
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*serialize(getSome(any.clone()), filter.clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        return Ok(s);
    }
    if isTuple(any.clone()) {
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("{\\\"Tuple\\\":{")); ArcStr::from(__mm_s) }).clone();
        no = 1;
        lst = metamodelica::nil();
        for i in 1..=getTupleSize(any.clone()) {
            lst = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\\\"")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("\\\":")); __mm_s.push_str(&*serialize(getListElement(any.clone(), no.clone()), filter.clone())?); ArcStr::from(__mm_s) }, lst.clone());
        }
        lst = lst.clone().reverse();
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*stringDelimitList(lst.clone(), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}} ")); ArcStr::from(__mm_s) }).clone();
        return Ok(s);
    }
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNKNOWN(")); __mm_s.push_str(&*anyString(any.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

