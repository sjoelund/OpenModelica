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

package AbsynToRust
"
 Translates Absyn to Rust.
 @Authors: OpenModelica contributors
"

import interface AbsynToRustTV;
import AbsynDumpTpl;

template dumpProgram(Absyn.Program program)
::=
match program
  case PROGRAM(classes = {}) then ""
  case PROGRAM(__) then
    let allModules = classes |> cls as CLASS(__) => 'mod <%toSnakeCase(name)%>;' ; separator="\n" ; empty
    (classes |> cls as CLASS(__) =>
      let &classFile = buffer ""
      let &classFile += redirectToFile('src/<%toSnakeCase(name)%>.rs')
      let &classFile += dumpClass(cls, defaultDumpOptions, allModules)
      let &classFile += closeFile()
      "")
end dumpProgram;

template dumpSCodeElements(list<SCode.Element> elements)
"
       Dumps forward declaration of uniontypes and partial functions unless elements is empty.
       Recursion needed to find all partial functions!. This should be call on a per module basis"
::= dumpSCodeElements2(filterElements(elements, defaultOptions))
end dumpSCodeElements;

template dumpSCodeElements2(list<SCode.Element> elements)
::=
  let str = elements |> el hasindex i1 fromindex 1 =>
  (
    match el
      case CLASS(restriction=SCode.R_UNIONTYPE(__)) then
        '/// Forward declaration of uniontype <%name%><%"\n"%>'
      case CLASS(classDef = parts as SCode.PARTS(__), partialPrefix = SCode.NOT_PARTIAL(), restriction=SCode.R_FUNCTION(__)) then
      dumpSCodeElements2(parts.elementLst)
      case CLASS(partialPrefix = SCode.PARTIAL(), restriction=SCode.R_FUNCTION(__)) then
       '/// Partial function: <%name%><%"\n"%>'
      else ''
  )
  if str then
  '<%"\n"%><%str%>'
  else ''
end dumpSCodeElements2;

template dumpClass(Absyn.Class cls, DumpOptions options, Text allModules)
/*We do not yet know our context in Absyn */
::=
  let &functionBuffer = buffer ""
  let res = <<
  #![ allow( unused_parens, while_true, unused_import ) ]
  /// Translation of MetaModelica to Rust
  ///
  /// This module provides code generation from Absyn to Rust.
  <% match cls case CLASS(name="Main") then allModules %>

  use metamodelica::*;
  use std::fmt;
  use list_comprehension_macro::comp;
  use anyhow::Result;
  use anyhow::bail;
  <% getImports(cls) |> imp => dumpImport(imp) ; separator="\n" ; empty %>

  <% dumpClassElement(cls, options, topContext, functionBuffer) %>
  >>
  res + functionBuffer
end dumpClass;

template dumpClassElement(Absyn.Class class, DumpOptions options, Context context, Text &functionBuffer)
"
  Note that partial functions are not handled here. They cannot really be translated to Rust in the way they are used in MetaModelica
  they are dumped as forward decls along with Uniontypes within the packages they occur.
"
::=
let elementSeparator = match context
  case UNIONTYPE(__) then ","
  else ""
match class
  case CLASS(body=parts as PARTS(__), restriction=R_UNIONTYPE(__)) then
    let commentStr = dumpCommentStrOpt(parts.comment)
    let &functionsBuffer = buffer ""
    let class_def_str = dumpClassDef(parts, makeUniontypeContext(name), options, functionsBuffer)
      <<
      <%commentStr%>/// Uniontype <%name%>
      #[derive(Debug, Clone, PartialEq)]
      #[allow(non_camel_case_types)]
      pub enum <%name%> {
        <%class_def_str%>
      }
      <% functionsBuffer %>
      >>
  /* We need to forward declare partial functions in Rust */
  case CLASS(partialPrefix=true, restriction=R_FUNCTION(__)) then ''
  case CLASS(partialPrefix=false, body=parts as PARTS(__), restriction=R_FUNCTION(__)) then
    let commentStr = dumpCommentStrOpt(parts.comment)
    let returnType = dumpReturnTypeRust(allPublicElementItems(parts.classParts))
    let return_str = '<%(parts.classParts |> cp => dumpReturnStrRust(getElementItemsInClassPart(cp), functionContext))%>'
    let inputs_str = (parts.classParts |> cp => dumpInputsRust(getElementItemsInClassPart(cp), inputContext))
    let header = dumpClassHeader(parts, restriction)
    let functionBodyStr = dumpClassDef(parts, makeFunctionContext(return_str), options, functionBuffer)
    /*
      Input output variables are treated as parameters
      output variables occur as local variables in Rust
    */
    let res = <<
    <%commentStr%>/// Function: <%name%>
    pub fn <%toSnakeCase(name)%>(<%inputs_str%>) -> <%returnType%> <%header%> {
      <%functionBodyStr%>
      <%return_str%>
    }
    >>
    match context
      case UNIONTYPE(__) then
        let &functionBuffer += res + "\n"
        ""
      else res

  case CLASS(body=parts as PARTS(__)) then
    let enc_str = if encapsulatedPrefix then "" else ""
    let partial_str = if partialPrefix then "/// Originally partial\n" else ""
    let class_type_str = (match restriction
      case R_RECORD(__) then dumpClassType(restriction, context)
      else dumpClassType(restriction, context))
    let cdef_str1 = match restriction
      case R_PACKAGE(__) then
        dumpClassDef(parts, packageContext, options, functionBuffer)
      case R_RECORD(__) then
        dumpClassDef(parts, structContext, options, functionBuffer)
      else
       dumpClassDef(parts, context, options, functionBuffer)
    let forwardDeclarations = dumpSCodeElements(AbsynToSCode.translateClassdefElements(parts.classParts))
    let inform  = if forwardDeclarations then
                    '/// Forward declarations needed for Rust enum variants'
                  else ''
    let cdef_str2 = match restriction
      case R_PACKAGE(__) then
        <<
        <%"\n"%>
        <%inform%>
        <%forwardDeclarations%>
        <%"\n"%>
        <%cdef_str1%>
        >>
      else
      <<
        <%cdef_str1%>
      >>
    let begin_str = match restriction
      case R_RECORD(__)
      case R_METARECORD(__)
      case R_UNIONTYPE(__) then '{'
      else ''
    let end_str = match restriction
      case R_RECORD(__)
      case R_METARECORD(__)
      case R_UNIONTYPE(__) then '}'
      else ''

    let cdef_str = cdef_str2
    let cmt_str = dumpCommentStrOpt(parts.comment)
    let header_str = dumpClassHeader(parts, restriction)
    let footer_str = dumpClassFooter(parts, cdef_str, name, cmt_str, "" /*ann_str*/)
    let partial_str_and_class_type = '<%partial_str%><%class_type_str%>'
    <<
    <%partial_str_and_class_type%> <%name%><%begin_str%>
      <%header_str%>
      <%"\n"%>
    <%footer_str%>
    <%end_str%><%elementSeparator%>
    >>
  /*Regular type redefinitions*/
  case CLASS(body=parts as DERIVED(__), restriction=R_TYPE(__)) then
    let comment = dumpCommentOpt(parts.comment, context)
    let spec = dumpTypeSpec(parts.typeSpec, context)
    let args = (parts.arguments |> earg => dumpElementArg(earg, context, functionBuffer) ;separator=', ')
    let attr = dumpElementAttr(parts.attributes)
    <<
    type <%name%> = <%spec%><%attr%><%comment%>;
    >>
  /*
    Modelica style function redefinition - not directly supported in Rust.
    Use type alias with a function pointer or a wrapper function.
  */
  case CLASS(body=parts as DERIVED(__), restriction=R_FUNCTION(__)) then
    let comment = dumpCommentOpt(parts.comment, context)
    let spec = dumpTypeSpec(parts.typeSpec, context)
    let args = (parts.arguments |> earg => dumpElementArg(earg, context, functionBuffer) ;separator=', ')
    let attr = dumpElementAttr(parts.attributes)
    let name_of_new_function = '<%name%>'
    <<
      <%comment%>
      /// Type alias for function redefinition
      pub type <%name_of_new_function%> = <%spec%>(<%args%>)
    >>
  /*PDER. Should not occur. Derived Enumeration and Overload might?*/
end dumpClassElement;

template dumpClassHeader(ClassDef classDef, Absyn.Restriction restriction)
::=
match classDef
  case CLASS_EXTENDS(__) then AbsynDumpTpl.errorMsg("Extend  not supported")
  case PARTS(__) then '<%dumpClassTypeTypeVars(restriction, typeVars)%><%dumpClassTypeSuperType(restriction)%>'
  else AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassHeader: <%dumpClassTypeSuperType(classDef)%>")
end dumpClassHeader;

template dumpClassTypeSuperType(Absyn.Restriction r)
::=
match r
  case R_METARECORD(__) then ''
  case R_FUNCTION(__) then ''
end dumpClassTypeSuperType;

template dumpClassTypeTypeVars(Absyn.Restriction restriction, list<String> typeVars)
::=
match restriction
  case R_UNIONTYPE(__) then
    ''
  case R_FUNCTION(__) then
    (if typeVars then 'where <%((typeVars |> tv => <<<%tv%>:Any>> ; separator=", "))%>')
  else ""
end dumpClassTypeTypeVars;

template dumpClassFooter(ClassDef classDef, String cdefStr, String name, String cmt, String ann)
::=
match classDef
  case DERIVED(__) then AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassFooter: Derived not yet supported.")
  case ENUMERATION(__) then AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassFooterf: ENUMERATION not yet supported.")
  case _ then
    let annotation_str = if ann then '<%ann%> ' else ''
    if cdefStr then
      <<
      <%cdefStr%>
      <%if annotation_str then " "%><%annotation_str%>
      >>
    else
      <<
      <%annotation_str%>
      >>
end dumpClassFooter;

template dumpInputsRust(list<ElementItem> inputs, Context context)
::=
  let inputStr = (listReverse((MMToRustUtil.filterOnDirection(inputs, MMToRustUtil.makeInputDirection())))
    |> ei
      =>
      let res = dumpTypeSpecOpt(getTypeSpecFromElementItemOpt(ei), inputContext)
      '<%dumpComponentItems(getComponentItemsFromElementItem(ei), makeInputContext(res))%>'
      ;separator=", ")
 '<%inputStr%>'
end dumpInputsRust;

template dumpReturnTypeRust(list<ElementItem> outputs)
::=
match MMToRustUtil.filterOnDirection(outputs, MMToRustUtil.makeOutputDirection())
  case {} then "()"
  case L as H::{} then '<%dumpOutputsRust(L)%>'
  case L as H::T then '(<%dumpOutputsRust(L)%>)'
end dumpReturnTypeRust;

template dumpReturnStrRust(list<ElementItem> outputs, Context context)
::=
match listReverse(MMToRustUtil.filterOnDirection(outputs, MMToRustUtil.makeOutputDirection()))
  case {} then ""
  case L as H::{} then
  '<%(L |> e => dumpElementItemRaw(e, defaultDumpOptions, context); separator=", ")%>'
  case L as H::T then
  <<
  (<%(L |> e => dumpElementItemRaw(e, defaultDumpOptions, context); separator=", ")%>)
  >>
end dumpReturnStrRust;

template dumpClassDef(Absyn.ClassDef cdef, Context context, DumpOptions options, Text &functionBuffer)
::=
match cdef
  case PARTS(__) then
    let body_str = (classParts |> class_part hasindex idx =>
        dumpClassPart(class_part, idx, context, options, functionBuffer) ;separator="\n";empty)
    if body_str then <<
      <%body_str%>
    >> else ""
  case DERIVED(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassDef: Derived not yet supported.")
  case CLASS_EXTENDS(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassDef: CLASS_EXETENDS not yet supported.")
  case ENUMERATION(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassDef: CLASS_ENUMERATION not yet supported.")
  else "TODO Unkown class definition"
end dumpClassDef;

template dumpClassType(Absyn.Restriction restriction, Context context)
::=
match context
  case UNIONTYPE(__) then ''
  else (match restriction
  case R_PACKAGE(__) then "// mod"
  case R_METARECORD(__) then
    <<
    #[derive(Debug, Clone, PartialEq)]
    /*pub struct*/
    >>
  case R_RECORD(__) then
    <<
    #[derive(Debug, Clone, PartialEq)]
    pub struct
    >>
  case R_UNIONTYPE(__) then
    <<
    #[derive(Debug, Clone, PartialEq)]
    pub enum
    >>
  case R_TYPE(__) then 'pub type'
  case R_FUNCTION(__) then "pub fn"
  case R_CLASS(__) then "@todo: class"
  case R_MODEL(__) then "@todo: model"
  else AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassType: Unknown restriction for class:" + AbsynDumpTpl.dumpRestriction(restriction)))
end dumpClassType;

template dumpClassPart(Absyn.ClassPart class_part, Integer idx, Context context, DumpOptions options, Text &functionBuffer)
::=
match class_part
  case PUBLIC(__) then
      let el_str = if isFunctionContext(context) then
                       dumpElementItems(filterOnDirection(contents, makeOutputDirection()), context, "", true, options, functionBuffer)
                     else
                       dumpElementItems(contents, context,"", true, options, functionBuffer)
      if el_str then <<
        <%el_str%>
      >> else ""
  case PROTECTED(__) then
    let el_str = dumpElementItems(contents, context, "", true, options, functionBuffer)
    if el_str then <<
        <%el_str%>
      >> else ""
  case CONSTRAINTS(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassPart: CONSTRAINTS(__) not supported.")
  case EQUATIONS(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassPart: EQUATIONS(__) not supported.")
  case INITIALEQUATIONS(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpClassPart: INITIALEQUATIONS() not supported.")
  case ALGORITHMS(__) then
    <<
      <%(contents |> eq => dumpAlgorithmItem(eq, context) ;separator="\n";empty)%>
    >>
  case INITIALALGORITHMS(__) then
    "@todo: AbsynToRust.dumpClassPart: INITIALALGORITHMS() not supported."
  case EXTERNAL(__) then
    let ann_str = match annotation_ case SOME(ann) then ' <%dumpAnnotation(ann, context)%>;'
    match externalDecl
      case EXTERNALDECL(__) then
        '// TODO: Defined in the runtime'
end dumpClassPart;

template dumpElementItems(list<Absyn.ElementItem> items, Context context, String prevSpacing, Boolean first, DumpOptions options, Text &functionBuffer)
::=
match items
  case item :: rest_items then
    let spacing = dumpElementItemSpacing(item)
    let pre_spacing = if not first then
      dumpElementItemPreSpacing(spacing, prevSpacing)
    let item_str = dumpElementItem(item, options, context, functionBuffer)
    let rest_str = dumpElementItems(rest_items, context, spacing, false, options, functionBuffer)
    let post_spacing = if rest_str then spacing else ""
    <<<%if item_str then '<%pre_spacing%><%item_str%><%post_spacing%>' else ''%><%if rest_str then "\n" + rest_str%>>>
end dumpElementItems;

template dumpElementItemPreSpacing(String curSpacing, String prevSpacing)
::= if not prevSpacing then curSpacing
end dumpElementItemPreSpacing;

template dumpElementItemSpacing(Absyn.ElementItem item)
::=
match item
  case ELEMENTITEM(element = ELEMENT(specification = CLASSDEF(class_ = CLASS(body = cdef))))
    then dumpClassDefSpacing(cdef)
end dumpElementItemSpacing;

template dumpClassDefSpacing(Absyn.ClassDef cdef)
::=
match cdef
  case PARTS(__) then '<%"\n"%>'
  case CLASS_EXTENDS(__) then '<%"\n"%>'
end dumpClassDefSpacing;

template dumpElementItem(Absyn.ElementItem eitem, DumpOptions options, Context context, Text &functionBuffer)
::=
match eitem
  case ELEMENTITEM(__) then '<%dumpElement(element, options, context, functionBuffer)%>'
  case LEXER_COMMENT(__) then comment
end dumpElementItem;

template dumpElementItemRaw(Absyn.ElementItem eitem, DumpOptions options, Context context)
"Same as dumpElementItem but does not add the local prefix"
::=
match eitem
  case ELEMENTITEM(__) then
    match element
      case ELEMENT(__)  then
        match specification
          case COMPONENTS(__) then
            let comps_str = (components |> comp => dumpComponentItem(comp, context) ;separator=", ")
            '<%comps_str%>'
          else
            AbsynDumpTpl.errorMsg("AbsynToRust.dumpElementItem: on none component type")
      else
        AbsynDumpTpl.errorMsg("AbsynToRust.dumpElementItem: on none component type")
  case LEXER_COMMENT(__) then comment
end dumpElementItemRaw;

template dumpElement(Absyn.Element elem, DumpOptions options, Context context, Text &functionBuffer)
::=
match elem
  case ELEMENT(__) then
    if boolOr(boolUnparseFileFromInfo(info, options), boolNot(isClassdef(elem))) then
    let final_str = dumpFinal(finalPrefix)
    let redecl_str = match redeclareKeywords case SOME(re) then dumpRedeclare(re)
    let repl_str = match redeclareKeywords case SOME(re) then dumpReplaceable(re)
    let elementSpec_str = dumpElementSpec(specification, options, context, functionBuffer)
    let constrainClass_str = match constrainClass case SOME(cc) then dumpConstrainClass(cc, context, functionBuffer)
    '<%elementSpec_str%><%constrainClass_str%>'
  case DEFINEUNIT(__) then AbsynDumpTpl.errorMsg("AbsynToRust.dumpElement: DEFINEUNIT(__) not supported")
  case TEXT(__) then
    if boolUnparseFileFromInfo(info, options) then
    let name_str = match optName case SOME(name) then name
    let info_str = dumpInfo(info)
    '/* Absyn.TEXT(SOME("<%name_str%>"), "<%string%>", "<%info_str%>"); */'
end dumpElement;

template dumpInfo(builtin.SourceInfo info)
::=
match info
  case SOURCEINFO(__) then
    let rm_str = if isReadOnly then "readonly" else "writable"
    'SOURCEINFO("<%fileName%>", <%rm_str%>, <%lineNumberStart%>, <%columnNumberStart%>, <%lineNumberEnd%>, <%columnNumberEnd%>)\n'
end dumpInfo;

template dumpAnnotation(Absyn.Annotation ann, Context context)
::=
match ann
  case ANNOTATION(elementArgs={}) then "/// annotation()"
  case ANNOTATION(__) then
    let &functionBuffer = buffer ""
    <<
    /* annotation(
      <%(elementArgs |> earg => dumpElementArg(earg, context, functionBuffer) ;separator=',<%"\n"%>')%>)
    */
    >>
end dumpAnnotation;

template dumpAnnotationOpt(Option<Absyn.Annotation> oann, Context context)
::= match oann case SOME(ann) then dumpAnnotation(ann, context)
end dumpAnnotationOpt;

template dumpAnnotationOptSpace(Option<Absyn.Annotation> oann, Context context)
::= match oann case SOME(ann) then " " + dumpAnnotation(ann, context)
end dumpAnnotationOptSpace;

template dumpComment(Absyn.Comment cmt, Context context)
::=
match cmt
  case COMMENT(__) then
    dumpCommentStrOpt(comment) + dumpAnnotationOptSpace(annotation_, context)
end dumpComment;

template dumpCommentOpt(Option<Absyn.Comment> ocmt, Context context)
::= match ocmt case SOME(cmt) then dumpComment(cmt, context)
end dumpCommentOpt;

template dumpCommentStrOpt(Option<String> comment)
::= match comment case SOME(cmt) then dumpCommentStr(cmt)
end dumpCommentStrOpt;

template dumpCommentStr(String comment)
::=
  <</* <%comment%> */<%\n%>>>
end dumpCommentStr;

template dumpElementArg(Absyn.ElementArg earg, Context context, Text &functionBuffer)
::=
match earg
  case MODIFICATION(__) then
    let each_str = dumpEach(eachPrefix)
    let final_str = dumpFinal(finalPrefix)
    let path_str = dumpPathRust(path)
    let mod_str = match modification case SOME(mod) then dumpModification(mod, context)
    let cmt_str = dumpCommentStrOpt(comment)
    '<%each_str%><%final_str%><%path_str%><%mod_str%><%cmt_str%>'
  case REDECLARATION(__) then
    let each_str = dumpEach(eachPrefix)
    let final_str = dumpFinal(finalPrefix)
    let redecl_str = dumpRedeclare(redeclareKeywords)
    let repl_str = dumpReplaceable(redeclareKeywords)
    let eredecl_str = '<%redecl_str%><%each_str%>'
    let elem_str = dumpElementSpec(elementSpec, defaultDumpOptions, context, functionBuffer)
    let cc_str = match constrainClass case SOME(cc) then dumpConstrainClass(cc, context, functionBuffer)
    '<%elem_str%><%cc_str%>'
end dumpElementArg;

template dumpEach(Absyn.Each each)
::= match each case EACH() then "each "
end dumpEach;

template dumpFinal(Boolean final)
::= if final then "final "
end dumpFinal;

template dumpRedeclare(Absyn.RedeclareKeywords redecl)
::=
match redecl
  case REDECLARE() then "redeclare "
  case REDECLARE_REPLACEABLE() then "redeclare "
end dumpRedeclare;

template dumpReplaceable(Absyn.RedeclareKeywords repl)
::=
match repl
  case REPLACEABLE() then "replaceable "
  case REDECLARE_REPLACEABLE() then "replaceable "
end dumpReplaceable;

template dumpModification(Absyn.Modification mod, Context context)
::=
match mod
  case CLASSMOD(__) then
    let arg_str = if elementArgLst then
      let &functionBuffer = buffer ""
      '(<%(elementArgLst |> earg => dumpElementArg(earg, context, functionBuffer) ;separator=", ")%>)'
    let eq_str = dumpEqMod(eqMod, context)
    '<%arg_str%><%eq_str%>'
end dumpModification;

template dumpEqMod(Absyn.EqMod eqmod, Context context)
::= match eqmod case EQMOD(__) then ' = <%dumpExp(exp, context)%>'
end dumpEqMod;

template dumpElementSpec(ElementSpec specification, DumpOptions options, Context context, Text &functionBuffer)
::=
match specification
  case CLASSDEF(__) then dumpClassElement(class_, options, context, functionBuffer)
  case EXTENDS(__) then
    let bc_str = dumpPathRust(path)
    let args_str = (elementArg |> earg => dumpElementArg(earg, context, functionBuffer) ;separator=", ")
    let mod_str = if args_str then '(<%args_str%>)'
    let ann_str = dumpAnnotationOptSpace(annotationOpt, context)
    '/* TODO: extends <%bc_str%><%mod_str%><%ann_str%> */'
  case COMPONENTS(__) then
    let attr_str = dumpElementAttr(attributes)
    let ty_str = dumpTypeSpec(typeSpec, context)
    let comps_str = if elementSpecIsOUTPUT_OR_BIDIR(specification) then
                      (components |> comp =>
                        let comp_str = dumpComponentItem(comp, makeFunctionReturnContext("",ty_str))
                          'let <%comp_str%>'
                      ;separator="\n";empty)
                    else ''
    let comps_str_no_local = if elementSpecIsOUTPUT_OR_BIDIR(specification) then
                      (components |> comp =>
                        let comp_str = dumpComponentItem(comp, makeConstantContext(ty_str))
                          ' <%match context case PACKAGE(__) then "const "%><%comp_str%><%
                            match context
                              case STRUCT_CONTEXT(__) then ","
                              else ";"
                            %>'
                      ;separator="\n";empty)
                    else ''
   let rStr = match context
     case FUNCTION(__) then
       '<%retValsStr%>'
     else''
    match context
      case FUNCTION(__) then
        if comps_str then
          '<%comps_str%>'
        else ''
      case STRUCT_CONTEXT(__)
      case UNIONTYPE(__) then
        '<%comps_str_no_local%>'
      case PACKAGE(__) then
        '<%comps_str_no_local%>'
      else '@todo: dumpElementSpec for other contexts'
  case IMPORT(__) then
    // dumpImport(import_)
    ""
end dumpElementSpec;

template dumpElementSpecForComponents(ElementSpec specification, DumpOptions options, Context context)
::=
match specification
  case COMPONENTS(__) then
    let comps_str = (components |> comp => dumpComponentItem(comp, context) ;separator=", ")
    '<%comps_str%>'
end dumpElementSpecForComponents;

template dumpElementAttr(Absyn.ElementAttributes attr)
::=
match attr
  case ATTR(__) then
    let var_str = dumpVariability(variability)
    '<%var_str%>'
end dumpElementAttr;

template dumpVariability(Absyn.Variability var)
::=
match var
  case VAR() then ""
  case CONST() then "const "
  else AbsynDumpTpl.errorMsg("AbsynToRust.dumpVariability: Only const and var are supported")
end dumpVariability;

template dumpConstrainClass(Absyn.ConstrainClass cc, Context context, Text &functionBuffer)
::=
match cc
  case CONSTRAINCLASS(elementSpec = Absyn.EXTENDS(path = p, elementArg = el)) then
    let path_str = dumpPathRust(p)
    let el_str = if el then '(<%(el |> e => dumpElementArg(e, context, functionBuffer) ;separator=", ")%>)'
    let cmt_str = dumpCommentOpt(comment, context)
    ' constrainedby <%path_str%><%el_str%><%cmt_str%>'
end dumpConstrainClass;

template dumpComponentItems(list<Absyn.ComponentItem> componentItems, Context context)
"Returns a comma separated list of component items without the condition string"
::= (componentItems |> ci => dumpComponentItemWithoutCondString(ci, context) ;separator=", ")
end dumpComponentItems;

template dumpComponentItem(Absyn.ComponentItem comp, Context context)
::=
match comp
  case COMPONENTITEM(__) then
    let comp_str = dumpComponent(component, context)
    let cond_str = dumpComponentCondition(condition, context)
    let cmt = dumpCommentOpt(comment, context)
      '<%comp_str%><%cond_str%><%cmt%>'
end dumpComponentItem;

template dumpComponentItemWithoutCondString(Absyn.ComponentItem comp, Context context)
::=
match comp
  case COMPONENTITEM(__) then
    let comp_str = dumpComponent(component, context)
    let cmt = dumpCommentOpt(comment, context)
    '<%comp_str%><%cmt%>'
end dumpComponentItemWithoutCondString;

template dumpComponent(Absyn.Component comp, Context context)
::=
match comp
  case COMPONENT(__) then
    let dim_str = dumpSubscripts(arrayDim, context)
    let mod_str = match modification case SOME(mod) then dumpModification(mod, context)
    let component_name = fixKeywords(name)
    match context
      case FUNCTION_RETURN_CONTEXT(__) then '<%component_name%>: <%ty_str%><%dim_str%><%mod_str%>;'
      case FUNCTION(__) then '<%component_name%>'
      case INPUT_CONTEXT(__) then '<%component_name%>: <%ty_str%><%dim_str%>/* Not allowed: <%mod_str%> */'
      case CONSTANT_CONTEXT(__) then '<%component_name%>: <%ty_str%><%dim_str%><%mod_str%>'
      else '<%component_name%><%dim_str%><%mod_str%>'
end dumpComponent;

template dumpComponentCondition(Option<Absyn.ComponentCondition> cond, Context context)
::=
match cond
  case SOME(cexp) then
    let exp_str = dumpExp(cexp, context)
    ' if <%exp_str%>'
end dumpComponentCondition;

template dumpImport(Absyn.Import imp)
::=
match imp
  case NAMED_IMPORT(__) then
    'use <%dumpPathRust(path)%> as <%name%>;'
  case QUAL_IMPORT(__) then
    let path_str = dumpPathRust(path)
    match path_str
      case "Array" then 'use ArrayUtil;'
      case "List" then  'use ListUtil;'
      else 'use <%path_str%>;'
  case UNQUAL_IMPORT(__) then 'use <%dumpPathRust(path)%>;'
  case GROUP_IMPORT(__) then
    let prefix_str = dumpPathRust(prefix)
    let groups_str = (groups |> group => dumpGroupImport(group) ;separator=", ")
    <<
    use <%prefix_str%>::{<%groups_str%>};
    >>
end dumpImport;

template dumpGroupImport(Absyn.GroupImport gimp)
::=
match gimp
  case GROUP_IMPORT_NAME(__) then name
  case GROUP_IMPORT_RENAME(__) then '<%name%> as <%rename%>'
end dumpGroupImport;

template dumpEquation(Absyn.Equation eq)
::= "No equations allowed. Translate them to algorithms"
end dumpEquation;

template dumpAlgorithmItems(list<Absyn.AlgorithmItem> algs, Context context)
::= (algs |> alg => dumpAlgorithmItem(alg, context) ;separator="\n")
end dumpAlgorithmItems;

template dumpAlgorithmItem(Absyn.AlgorithmItem alg, Context context)
::=
match alg
  case ALGORITHMITEM(__) then
    let alg_str = dumpAlgorithm(algorithm_, context)
    let cmt_str = dumpCommentOpt(comment, context)
    '<%alg_str%><%cmt_str%>'
  case ALGORITHMITEMCOMMENT(__) then comment
end dumpAlgorithmItem;

template dumpAlgorithm(Absyn.Algorithm alg, Context context)
::=
match alg
  case ALG_ASSIGN(__) then
    if AbsynUtil.complexIsCref(assignComponent) then
      let lhs_str = dumpExp(assignComponent, context)
      let rhs_str = dumpExp(value, context)
      '<%lhs_str%> = <%rhs_str%>;'
    else
      let &as_str = buffer ""
      let lhs_str = dumpPattern(assignComponent, makeFunctionContext("listMatchAssign"), as_str)
      let rhs_str = dumpExp(value, context)
      <<
      match <%rhs_str%> {
        <%lhs_str%> => {<%as_str%>}
      }
      >>
  case ALG_IF(__) then
    let if_str = dumpAlgorithmBranch(ifExp, trueBranch, "if", context)
    let elseif_str = (elseIfAlgorithmBranch |> (c, b) =>
        dumpAlgorithmBranch(c, b, "} else if", context) ;separator="")
    let else_branch_str = dumpAlgorithmItems(elseBranch, context)
    let else_str = if else_branch_str then
      <<
      } else {
        <%else_branch_str%>
      >>
    <<
    <%if_str%>
    <%elseif_str%>
    <%else_str%>
    }
    >>
  case ALG_FOR(__) then
    let iter_str = dumpForIterators(iterators, context)
    let body_str = dumpAlgorithmItems(forBody, context)
    <<
    for <%iter_str%> {
      <%body_str%>
    }
    >>
  case ALG_WHILE(__) then
    <<
    <%dumpAlgorithmBranch(boolExpr, whileBody, "while", context)%>
    }
    >>
  case ALG_WHEN_A(__) then  AbsynDumpTpl.errorMsg("When statements are not allowed!.")
  case ALG_NORETCALL(__) then
    let name_str = dumpCref(functionCall, context)
    let args_str = dumpFunctionArgs(functionArgs, context)
    '<%name_str%>(<%args_str%>);'
  case ALG_RETURN(__) then dumpAlgReturnString(context)
  case ALG_BREAK(__) then "break;"
  case ALG_FAILURE(__) then
    let arg_str = if equ then dumpAlgorithmItems(equ, context) else "..."
    let tmp = 'tmp<%tmpTick()%>'
    <<
    let <%tmp%> = {
      <%arg_str%>
    };
    if let Ok(_) = tmp<%tmp%> {
      // success
      bail!("Expected failure, but code executed successfully.");
    }
    >>
  case ALG_TRY(__) then
    let arg1 = dumpAlgorithmItems(body, context)
    let arg2 = dumpAlgorithmItems(elseBody, context)
    let tmp = 'tmp<%tmpTick()%>'
    <<
    let <%tmp%> = {
      <%arg1%>
    };
    if let Ok(_) = <%tmp%> {
      // success
    } else {
      <%arg2%>
    }
    >>
  case ALG_CONTINUE(__) then "continue;"
end dumpAlgorithm;

template dumpAlgReturnString(Context context)
  "Dumps the return string for a specific function context"
::= match context
    case FUNCTION(__) then '<%retValsStr%>'
    else "unreachable!()"
end dumpAlgReturnString;

template dumpAlgorithmBranch(Absyn.Exp cond, list<Absyn.AlgorithmItem> body,
String header, Context context)
::=
  let cond_str = dumpExp(cond, context)
  let body_str = (body |> eq => dumpAlgorithmItem(eq, context) ;separator="\n")
  <<
  <%header%> <%cond_str%> {
    <%body_str%>
  >>
end dumpAlgorithmBranch;

template dumpPathRust(Absyn.Path path)
"Wrapper function for dump path.
 Needed since certain keywords will have a slightly different meaning in Rust"
::=
match path
  case FULLYQUALIFIED(__) then
    '::<%dumpPathRust(path)%>'
  case QUALIFIED(__) then
    if (Flags.getConfigBool(Flags.MODELICA_OUTPUT)) then
    '<%name%>__<%AbsynDumpTpl.dumpPath(path)%>'
    else
    '<%toSnakeCase(name)%>::<%dumpPathRust(path)%>'
  case IDENT(__) then
    match name
      case "Real" then 'f64'
      case "Integer" then 'i32'
      case "Boolean" then 'bool'
      case "String" then '&str'
      case "list" then 'List'
      case "array" then 'Array'
      case "tuple" then 'Tuple'
      case "polymorphic" then 'dyn std::any::Any'
      else fixKeywords(name)
  else
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpPathRust: Unknown path.")
end dumpPathRust;

template dumpPathNoQual(Absyn.Path path)
::=
match path
  case FULLYQUALIFIED(__) then
    dumpPathRust(path)
  else
    dumpPathRust(path)
end dumpPathNoQual;

template dumpTypeSpecOpt(Option<Absyn.TypeSpec> typespecOpt, Context context)
::= match typespecOpt case SOME(ts) then dumpTypeSpec(ts, context) else ""
end dumpTypeSpecOpt;

template dumpTypeSpec(Absyn.TypeSpec typeSpec, Context context)
"
Dumps the type specification for Rust.
"
::=
match typeSpec
  case TPATH(__) then
    let path_str = dumpPathRust(path)
    let arraydim_str = dumpArrayDimOpt(arrayDim, context)
    '<%path_str%><%arraydim_str%>'
  case TCOMPLEX(__) then
    let path_str = dumpPathRust(path)
    let ty_str = (typeSpecs |> ty => dumpTypeSpec(ty, context) ;separator=", ")
    let arraydim_str = dumpArrayDimOpt(arrayDim, context)
    let isFunc = match context
                   case INPUT_CONTEXT(__) then "iofunc"
                   else ""
    let isPackage  = match context
                       case PACKAGE(__) then "package"
                       else ""
    if isFunc then
      '<%path_str%><<%ty_str%>><%arraydim_str%>'
    else
      if isPackage then
        '<%path_str%>'
      else
        '<%path_str%><<%ty_str%>><%arraydim_str%>'
end dumpTypeSpec;

template dumpArrayDimOptTypeSpec(Option<Absyn.ArrayDim> arraydim, Context context)
"Not in use"
::= match arraydim case SOME(ad) then dumpSubscriptsTypeSpec(ad, context)
end dumpArrayDimOptTypeSpec;

template dumpSubscriptsTypeSpec(list<Subscript> subscripts, Context context)
"Not in use"
::=
  if subscripts then
    let sub_str = (subscripts |> s => 'Array' ;separator=", ")
    'Array<<%sub_str%>>'
end dumpSubscriptsTypeSpec;

template dumpArrayDimOpt(Option<Absyn.ArrayDim> arraydim, Context context)
::= match arraydim case SOME(ad) then dumpSubscripts(ad, context)
end dumpArrayDimOpt;

template dumpSubscripts(list<Subscript> subscripts, Context context)
::=
  if subscripts then
    let sub_str = (subscripts |> s => dumpSubscript(s, context) ;separator=", ")
    '[<%sub_str%>]'
end dumpSubscripts;

template dumpSubscript(Absyn.Subscript subscript, Context context)
::=
match subscript
  case NOSUB(__) then ':'
  case SUBSCRIPT(__) then dumpExp(subscript, context)
end dumpSubscript;

template dumpExp(Absyn.Exp exp, Context context)
::=
match exp
  case INTEGER(__) then value
  case REAL(__) then '<%value%>.0'
  case CREF(__) then dumpCref(componentRef, context)
  case STRING(__) then '"<%Util.escapeModelicaStringToCString(value)%>".to_string()'
  case BOOL(__) then value
  case e as BINARY(__) then
    let lhs_str = dumpOperand(exp1, e, true, context)
    let rhs_str = dumpOperand(exp2, e, false, context)
    let op_str = dumpOperator(op)
    '<%lhs_str%> <%op_str%> <%rhs_str%>'
  case e as UNARY(__) then
    let exp_str = dumpOperand(exp, e, false, context)
    let op_str = dumpOperator(op)
    '<%op_str%><%exp_str%>'
  case e as LBINARY(__) then
    let lhs_str = dumpOperand(exp1, e, true, context)
    let rhs_str = dumpOperand(exp2, e, false, context)
    let op_str = dumpOperator(op)
    '<%lhs_str%> <%op_str%> <%rhs_str%>'
  case e as LUNARY(__) then
    let exp_str = dumpOperand(exp, e, false, context)
    let op_str = dumpOperator(op)
    '<%op_str%><%exp_str%>'
  case e as RELATION(__) then
    let lhs_str = dumpOperand(exp1, e, true, context)
    let rhs_str = dumpOperand(exp2, e, false, context)
    let op_str = dumpOperator(op)
    '<%lhs_str%> <%op_str%> <%rhs_str%>'
  case IFEXP(__) then dumpIfExp(exp, context)
  case CALL(function_=Absyn.CREF_IDENT(name="$array")) then
    let args_str = dumpFunctionArgs(functionArgs, context)
    'vec![<%args_str%>]'
  case CALL(function_=Absyn.CREF_IDENT(name="list"), functionArgs=FOR_ITER_FARG(__)) then
    let args_str = dumpFunctionArgs(functionArgs, context)
    'comp![<%args_str%>]'
  case CALL(function_=Absyn.CREF_IDENT(name="min"), functionArgs=FOR_ITER_FARG(__)) then
    let args_str = dumpFunctionArgs(functionArgs, context)
    'min(comp![<%args_str%>])'
  case CALL(functionArgs=FOR_ITER_FARG(iterators=_::_::_, iterType=THREAD(__))) then
    'TODO: Threaded iteration is not fully supported yet.'
  case CALL(function_=function_, functionArgs=functionArgs as FOR_ITER_FARG(exp=exp, iterators=iterators)) then
    let func_str = dumpCref(function_, context)
    let exp_str = dumpExp(exp, context)
    let iter_str = (iterators |> i => dumpForIterator(i, context) ;separator=", ")
    // let iter_names = (iterators |> i => dumpForIteratorName(i, context) ;separator=", ")
    // let iter_ranges = (iterators |> i => dumpForIteratorRanges(i, context) ;separator=", ")
    '<%func_str%>(comp![<%exp_str%> for <%iter_str%>])'
  case CALL(__) then
    let func_str = dumpCref(function_, context)
    let args_str = dumpFunctionArgs(functionArgs, context)
    '<%func_str%>(<%args_str%>)'
  case PARTEVALFUNCTION(__) then
    let func_str = dumpCref(function_, context)
    let args_str = dumpFunctionArgs(functionArgs, context)
    let args2_str = match functionArgs
      case FUNCTIONARGS(__) then
        '<%(argNames |> na => dumpNamedArgPattern3(na) ;separator=", ")%>'
      else
        ''
    '(|| {<%func_str%>(<%args_str%>)})'
  case ARRAY(__) then
    let array_str = (arrayExp |> e => dumpExp(e, context) ;separator=", ")
    if array_str then
      'vec![<%array_str%>]'
    else
      'vec![]'
  case MATRIX(__) then
    let matrix_str = (matrix |> row =>
        (row |> e => dumpExp(e, context) ;separator=", ") ;separator=", ")
    'vec![<%matrix_str%>]'
  case e as RANGE(step = SOME(step)) then
    let start_str = dumpOperand(start, e, false, context)
    let step_str = dumpOperand(step, e, false, context)
    let stop_str = dumpOperand(stop, e, false, context)
    '(<%start_str%>..=<%stop_str%>).step_by(<%step_str%>)'
  case e as RANGE(step = NONE()) then
    let start_str = dumpOperand(start, e, false, context)
    let stop_str = dumpOperand(stop, e, false, context)
    '<%start_str%>..=<%stop_str%>'
  case TUPLE(__) then
    let tuple_str = (expressions |> e => dumpExp(e,context); separator=", " ;empty)
    if tuple_str then '(<%tuple_str%>)'
    else '()'
  case END(__) then '/* END */'
  case CODE(__) then '@todo: CodeNode'
  case AS(__) then
    let exp_str = dumpExp(exp, context)
    '<%id%>: <%exp_str%>'
  case CONS(__) then
    let head_str = dumpExp(head, context)
    let rest_str = dumpExp(rest, context)
    'List::cons(<%head_str%>, <%rest_str%>)'
  case MATCHEXP(__) then dumpMatchExp(exp, context)
  case LIST(__) then
    let list_str = (exps |> e => dumpExp(e, context) ;separator=", ")
    'vec![<%list_str%>]'
  case DOT(__) then
    '<%dumpExp(exp, context)%>.<%dumpExp(index, context)%>'
  case EXPRESSIONCOMMENT(__) then
    let exp_str = dumpExp(exp, context)
    '<%commentsBefore |> cmt => cmt ; separator="\n"%><%exp_str%><%commentsAfter |> cmt => cmt ; separator="\n"%>'
  case _ then '/* dumpExp: UNHANDLED Absyn.Exp: <%AbsynDumpTpl.dumpExp(exp)%> */'
end dumpExp;

template dumpPattern(Absyn.Exp exp, Context context, Text &as_str)
::=
match exp
  case UNARY(__) then '-<%dumpPattern(exp, context, as_str)%>'
  case INTEGER(__) then value
  case REAL(__) then value
  case CREF(componentRef=WILD(__)) then 'ignore<%tmpTick()%>'
  case CREF(componentRef=ALLWILD(__)) then '..'
  case CREF(__) then dumpCref(componentRef, functionContext)
  case STRING(__) then '"<%stringReplace(value,"\$","\\$"); absIndent=0%>"'
  case BOOL(__) then value
  case ARRAY(arrayExp=exps)
  case LIST(__)
  case CALL(function_=Absyn.CREF_IDENT(name="list"), functionArgs=FUNCTIONARGS(args=exps))
  case CALL(function_=Absyn.CREF_IDENT(name="$array"), functionArgs=FUNCTIONARGS(args=exps)) then
    '[<%exps |> e => '<%dumpPattern(e, context, &as_str)%>'; separator=", " %>]'
  case CALL(function_=Absyn.CREF_IDENT(name="NONE"), functionArgs=FUNCTIONARGS(args={})) then
    'None'
  case CALL(function_=Absyn.CREF_IDENT(name="SOME"), functionArgs=FUNCTIONARGS(args={exp})) then
    'Some(<%dumpPattern(exp, context, as_str)%>)'
  case CALL(__) then
    let func_str = dumpCref(function_, functionContext)
    let args_str = dumpFunctionArgsPattern(functionArgs)
      '<%func_str%>{<%args_str%>}'
  case TUPLE(__) then
    let tuple_str = (expressions |> e => dumpPattern(e, context, &as_str); separator=", " ;empty)
    '(<%tuple_str%>)'
  case AS(__) then
    let exp_str = dumpPattern(exp, context, &as_str)
    let id_str = '<%id%>'
    '<%id_str%> @ <%exp_str%>'
  case CONS(__) then
    "[" + dumpCons(head, rest, context, &as_str)
  case EXPRESSIONCOMMENT(__) then
    let exp_str = dumpPattern(exp, context, &as_str)
    '<%commentsBefore |> cmt => cmt ; separator="\n"%><%exp_str%><%commentsAfter |> cmt => cmt ; separator="\n"%>'
  case _ then '/* AbsynDumpTpl.dumpPattern: UNHANDLED: <%AbsynDumpTpl.dumpExp(exp)%> */'
end dumpPattern;

template dumpCons(Absyn.Exp head, Absyn.Exp tail, Context context, Text &as_str)
::=
  let headString = dumpPattern(head, context, &as_str)
  match tail
    case CONS(__) then headString + ", " + dumpCons(head, rest, context, &as_str)
    case CALL(function_=Absyn.CREF_IDENT(name="list"), functionArgs=FUNCTIONARGS(args=exps))
    case CALL(function_=Absyn.CREF_IDENT(name="$array"), functionArgs=FUNCTIONARGS(args=exps))
    case LIST(exps=exps)
    case ARRAY(arrayExp=exps) then (exps |> e => (dumpPattern(e, context, &as_str) ;separator=", ")) + "]"
    case CREF(componentRef=WILD(__)) then headString + ", ..]"
    case CREF(__) then <<<%headString%>, <%dumpPattern(tail, context, as_str)%> @ ..]>>
    else '[<%headString%>, <%dumpPattern(tail, context, as_str)%> @ ..]'
end dumpCons;

template dumpFunctionArgsPattern(Absyn.FunctionArgs args)
::=
match args
  case FUNCTIONARGS(__) then
    let args_str = (args |> arg hasindex i1 => 'postionalArg<%i1%>: <%dumpPattern(arg, functionContext, emptyTxt)%>' ;separator=", ")
    let namedargs_str = (argNames |> narg => dumpNamedArgPattern(narg) ;separator=", ")
    let separator = if args_str then if argNames then ', '
    '<%args_str%><%separator%><%namedargs_str%>'
  else 'ERROR FOR_ITER_FARG in pattern'
end dumpFunctionArgsPattern;

template dumpNamedArgPattern(Absyn.NamedArg narg)
::=
match narg
  case NAMEDARG(__) then
    '<%argName%>: <%dumpPattern(argValue, functionContext, emptyTxt)%>'
end dumpNamedArgPattern;

template dumpNamedArgPattern2(Absyn.NamedArg narg)
"Returns the argument name"
::=
match narg
  case NAMEDARG(__) then
    "<%argName%>"
end dumpNamedArgPattern2;

template dumpNamedArgPattern3(Absyn.NamedArg narg)
"Returns the argument value"
::=
match narg
  case NAMEDARG(__) then
    '<%dumpPattern(argValue, functionContext, emptyTxt)%>'
end dumpNamedArgPattern3;

template dumpLhsExp(Absyn.Exp lhs, Context context)
::=
match lhs
  case IFEXP(__) then '<%dumpExp(lhs, context)%>'
  else dumpExp(lhs, context)
end dumpLhsExp;

template dumpOperand(Absyn.Exp operand, Absyn.Exp operation, Boolean lhs, Context context)
::=
  let op_str = dumpExp(operand, context)
  if shouldParenthesize(operand, operation, lhs) then
    '(<%op_str%>)'
  else
    op_str
end dumpOperand;

template dumpIfExp(Absyn.Exp if_exp, Context context)
::=
match if_exp
  case IFEXP(__) then
    let cond_str = dumpExp(ifExp, context)
    let true_branch_str = dumpExp(trueBranch, context)
    let else_branch_str = dumpExp(elseBranch, context)
    let else_if_str = dumpElseIfExp(elseIfBranch, context)
    'if <%cond_str%> { <%true_branch_str%> }<%else_if_str%> else { <%else_branch_str%> }'
end dumpIfExp;

template dumpElseIfExp(list<tuple<Absyn.Exp, Absyn.Exp>> else_if, Context context)
::=
  else_if |> eib as (cond, branch) =>
    let cond_str = dumpExp(cond, context)
    let branch_str = dumpExp(branch, context)
    ' else if (<%cond_str%>) { <%branch_str%> }' ; separator=""
end dumpElseIfExp;

/*
template dumpCodeNode(Absyn.CodeNode code, Context context, Text &functionBuffer)
::=
match code
  case C_TYPENAME(__) then dumpPathRust(path)
  case C_VARIABLENAME(__) then dumpCref(componentRef, context)
  case C_CONSTRAINTSECTION(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpCodeNode: C_CONSTRAINTSECTION not supported")
  case C_EQUATIONSECTION(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpCodeNode: C_CONSTRAINTSECTION not supported")
  case C_ALGORITHMSECTION(__) then
    AbsynDumpTpl.errorMsg("AbsynToRust.dumpCodeNode: C_ALGORITHMSECTION not supported")
  case C_ELEMENT(__) then dumpElement(element, Dump.defaultDumpOptions, context, functionBuffer)
  case C_EXPRESSION(__) then dumpExp(exp, context)
  case C_MODIFICATION(__) then dumpModification(modification, context)
end dumpCodeNode;
*/

template dumpMatchExp(Absyn.Exp match_exp, Context context)
::=
match match_exp
  case MATCHEXP(__) then
    let match_ty_str = dumpMatchType(matchTy)
    let input_str = dumpExp(inputExp, functionContext)
    let locals_str = dumpMatchLocals(localDecls)
    let cases_str = (cases |> c => dumpMatchCase(c, makeMatchContext(inputExp)) ;separator="\n")
    let cmt_str = dumpCommentStrOpt(comment)
    if locals_str then
    <<
    {
      // Match local declarations
      <%locals_str%>
      match <%input_str%> {
        <%cases_str%><%cmt_str%>
      }
    }
    >>
    else
    <<
    match <%input_str%> {
      <%cases_str%><%cmt_str%>
    }
    >>
end dumpMatchExp;

template dumpMatchType(Absyn.MatchType match_type)
::=
match match_type
  case MATCH() then ""
  case MATCHCONTINUE() then ""
end dumpMatchType;

template dumpMatchContents(ClassPart cp)
::=
  match cp
  case EQUATIONS(contents={}) then ""
  case EQUATIONS(__) then
  <<
    <%(Static.fromEquationsToAlgAssignments(cp) |> alg => dumpAlgorithmItem(alg, functionContext) ;separator="\n")%>
  >>
  case ALGORITHMS(contents={}) then ""
  case ALGORITHMS(contents=algs) then
    <<
      <%(algs |> alg => dumpAlgorithmItem(alg, functionContext) ;separator="\n")%>
    >>
end dumpMatchContents;

template dumpMatchLocals(list<ElementItem> locals)
::= if locals then
  let &functionBuffer = buffer ""
  <<
    <%(locals |> decl => dumpElementItem(decl, defaultDumpOptions, functionContext, functionBuffer) ;separator="\n")%>
  >>
end dumpMatchLocals;

template dumpMatchCase(Absyn.Case c, Context context)
::=
match c
  case CASE(__) then
    let &as_str = buffer ""
    let pattern_str = dumpPattern(pattern, context, &as_str)
    let guard_str = match patternGuard case SOME(g) then ' if <%dumpExp(g, context)%>'
    let eql_str = dumpMatchContents(classPart)
    let result_str = dumpExp(result, context)
    let cmt_str = dumpCommentStrOpt(comment)
    let input_str = match context
      case MATCH_CONTEXT(__) then dumpExp(inputExp, context)
      else ''
  if as_str then
    <<
    <%pattern_str%><%guard_str%><%cmt_str%> => {
      <%&as_str%>
      <%eql_str%>
      <%result_str%>
    },
    >>
    else
    <<
    <%pattern_str%><%guard_str%><%cmt_str%> => {
      <%eql_str%>
      <%result_str%>
    },
    >>
  case ELSE(__) then
    let eql_str = dumpMatchContents(classPart)
    let result_str = dumpExp(result, context)
    let cmt_str = dumpCommentStrOpt(comment)
    <<
    _ <%cmt_str%> => {
        <%eql_str%>
        <%result_str%>
    }
    >>
end dumpMatchCase;

template dumpOperator(Absyn.Operator op)
::= match op
      case AND(__) then '&&'
      case OR(__) then '||'
      case NOT(__) then '!'
      case NEQUAL(__) then '!='
    else AbsynDumpTpl.dumpOperator(op)
end dumpOperator;

template dumpCref(Absyn.ComponentRef cref, Context context)
::=
match cref
  case CREF_QUAL(__) then
     let ss_str = dumpSubscripts(subscripts, context)
     let c_str = dumpCref(componentRef, context)
    match name
      case "List" then 'ListUtil<%ss_str%>::<%c_str%>'
      case "Array" then 'ArrayUtil<%ss_str%>::<%c_str%>'
      else '<%toSnakeCase(name)%>::<%c_str%>'
  case CREF_IDENT(__) then
    '<%fixKeywords(name)%><%dumpSubscripts(subscripts, context)%>'
  case CREF_FULLYQUALIFIED(__) then '::<%dumpCref(componentRef, context)%>'
  case WILD(__) then if Config.acceptMetaModelicaGrammar() then "_" else ""
  case ALLWILD(__) then '_ ..'
end dumpCref;

template dumpFunctionArgs(Absyn.FunctionArgs args, Context context)
::=
match args
  case FUNCTIONARGS(__) then
    let args_str = (args |> arg => dumpExp(arg, context) ;separator=", ")
    let namedargs_str = (argNames |> narg => dumpNamedArg(narg, context) ;separator=", ")
    let separator = if args_str then if argNames then ', '
    '<%args_str%><%separator%><%namedargs_str%>'
  case FOR_ITER_FARG(__) then
    let exp_str = dumpExp(exp, context)
    let iter_str = (iterators |> i => dumpForIterator(i, context) ;separator=", ")
    // let iter_names = (iterators |> i => dumpForIteratorName(i, context) ;separator=", ")
    // let iter_ranges = (iterators |> i => dumpForIteratorRanges(i, context) ;separator=", ")
    let res = '<%exp_str%> for <%iter_str%>'
    match iterType
      case THREAD(__) then
        if intGt(listLength(iterators),1) then '<%exp_str%> /* threaded iteration */' else res
      else res
end dumpFunctionArgs;

template dumpNamedArg(Absyn.NamedArg narg, Context context)
::=
match narg
  case NAMEDARG(__) then
    '/* NAMEDARG <%argName%>:*/ <%dumpExp(argValue, context)%>'
end dumpNamedArg;

template dumpForIterators(Absyn.ForIterators iters, Context context)
::= (iters |> i => dumpForIterator(i, context) ;separator=", ")
end dumpForIterators;

template dumpForIterator(Absyn.ForIterator iterator, Context context)
::=
match iterator
  case ITERATOR(__) then
    let range_str = match range case SOME(r) then ' in <%dumpExp(r, context)%>'
    let guard_str = match guardExp case SOME(g) then ' if <%dumpExp(g, context)%>'
    '<%name%><%range_str%><%guard_str%>'
end dumpForIterator;

template dumpForIteratorRanges(Absyn.ForIterator iterator, Context context)
::=
match iterator
  case ITERATOR(__) then
    let range_str = match range case SOME(r) then '<%dumpExp(r, context)%>'
    let guard_str = match guardExp case SOME(g) then ' if <%dumpExp(g, context)%>'
    '<%range_str%><%guard_str%>'
end dumpForIteratorRanges;

template dumpForIteratorName(Absyn.ForIterator iterator, Context context)
::=
match iterator
  case ITERATOR(__) then
    '<%name%>'
end dumpForIteratorName;

template dumpOutputsRust(list<ElementItem> elements)
::=
  listReverse(elements) |> e =>
    dumpTypeSpecOpt(AbsynUtil.getTypeSpecFromElementItemOpt(e), functionContext) ; separator=", "
end dumpOutputsRust;

annotation(__OpenModelica_Interface="backend");
end AbsynToRust;
