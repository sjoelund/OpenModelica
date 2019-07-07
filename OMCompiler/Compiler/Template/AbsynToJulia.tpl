package AbsynToJulia
/* TODOS:

TODO:Public/Private semantics
   Add export for all public functions in the module.
   Just do not export private modules. This should solve public and private for MetaModelica
   and keep the semantics the same.
   Simple go through all classes and export the public classes, not efficient but it should work..
   Ignore all protected.

  input output should be a function argument but not appear on local variables!

*/

import interface AbsynToJuliaTV;
import AbsynDumpTpl;

template dumpProgram(Absyn.Program program)
::=
match program
  case PROGRAM(classes = {}) then ""
  case PROGRAM(__) then
    /* Necessary forward declarations */
    let cls_str = (classes |> cls => dumpClass(cls, defaultDumpOptions) ;separator="\n\n")
    <<
      <%cls_str%>
    >>
end dumpProgram;

template dumpSCodeElements(list<SCode.Element> elements)
"Dumps forward declaration of uniontypes unless elements is empty"
::= dumpSCodeElements2(filterElements(elements, defaultOptions))
end dumpSCodeElements;

template dumpSCodeElements2(list<SCode.Element> elements)
::=
  let str = elements |> el hasindex i1 fromindex 1 =>
  (
    match el
      case CLASS(restriction=SCode.R_UNIONTYPE(__)) then
        '@UniontypeDecl <%name%> <%\n%>'
      else ''
  )
  if str then
  '#= Necessary to write declarations for your uniontypes until Julia adds support for mutually recursive types =#<%\n%><%str%>'
  else
    ''
end dumpSCodeElements2;

template dumpClass(Absyn.Class cls, DumpOptions options)
/*We do not yet know our context in Absyn */
::= dumpClassElement(cls, options, noContext)
end dumpClass;

template dumpClassElement(Absyn.Class class, DumpOptions options, Context context)
"TODO handle input_output parameters correctly"
::=
match class
  case CLASS(body=parts as PARTS(__),restriction=R_UNIONTYPE(__)) then
      /*
        In Julia we treat uniontypes as declarations. They do not have a direct definition
        I use a uniontype macro for short here so things that belong together are grouped together..
      */
       let commentStr = dumpCommentStrOpt(parts.comment)
       let class_def_str = dumpClassDef(parts, makeUniontypeContext(name), options)
     <<
      <%commentStr%>
      @Uniontype <%name%> begin
       <%class_def_str%>
      end
     >>
  case CLASS(partialPrefix=true, restriction=R_FUNCTION(__)) then
    /* Julia does not really support types of higher-order functions */
    '<%name%> = Function'
  case CLASS(body=parts as PARTS(__), restriction=R_FUNCTION(__)) then
    let commentStr = dumpCommentStrOpt(parts.comment)
    let returnType = (parts.classParts |> cp => dumpReturnTypeJL(getElementItemsInClassPart(cp)))
    let return_str = '<%(parts.classParts |> cp => dumpReturnStrJL(getElementItemsInClassPart(cp), functionContext))%>'
    let inputs_str = (parts.classParts |> cp => dumpInputsJL(getElementItemsInClassPart(cp), functionContext))
    let header = dumpClassHeader(parts, restriction)
    let functionBodyStr = dumpClassDef(parts, makeFunctionContext(return_str), options)
    /*
      Input output variables are treated as parameters
      output and bidirectional variables occurs as local variables in Julia
    */
    <<
    <%commentStr%>
    <%header%>
    function <%name%>(<%inputs_str%>)<%returnType%>
      <%functionBodyStr%>
      <%return_str%>
    end
    >>
  case CLASS(body=parts as PARTS(__)) then
    let enc_str = if encapsulatedPrefix then "" /*Should we use a macro here?*/ else ""
    let partial_str = if partialPrefix then "abstract" else ""
    let class_type_str = dumpClassType(restriction)
    let cdef_str1 = dumpClassDef(parts, packageContext, options)
    let forwardDeclarations = dumpSCodeElements(SCodeUtil.translateClassdefElements(parts.classParts))
    let cdef_str2 = match restriction /*What about nested packages*/
      case R_PACKAGE(__) then
        <<
        <%\n%>
        using MetaModelica
        <%forwardDeclarations%>
        <%\n%>
        <%cdef_str1%>
        <%\n%>
        >>
      else
      <<
        <%cdef_str1%>
      >>
   let begin_str = match restriction
     case R_RECORD(__) then  'begin'
     else ''

    let cdef_str = cdef_str2
    let cmt_str = dumpCommentStrOpt(parts.comment)
    /* Investigate header_str and annotation string*/
    //let ann_str = dumpClassAnnotation(cmt)
    let header_str = dumpClassHeader(parts, restriction)
    let footer_str = dumpClassFooter(parts, cdef_str, name, cmt_str, "" /*ann_str*/)
    let partial_str_and_class_type = '<%partial_str%><%class_type_str%>'
    <<
    <%partial_str_and_class_type%> <%name%> <%begin_str%>
      <%header_str%>
      <%\n%>
    <%footer_str%>
    >>
  /*Regular type redefinitions*/
  case CLASS(body=parts as DERIVED(__), restriction=R_TYPE(__)) then
    /* Derived should have the last context as it's context right? */
    let comment = dumpCommentOpt(parts.comment, context)
    let spec = dumpTypeSpec(parts.typeSpec, context)
    let args = (parts.arguments |> earg => dumpElementArg(earg, context) ;separator=', ')
    let attr = dumpElementAttr(parts.attributes)
    <<
    <%name%> = <%spec%> <%attr%><%comment%>
    >>
  /*
    This is a special case that seems to occur from time to time!
    Modelica style function redfinition something that is not support in Julia.
    I solve this using a macro @FunctionExtend..
    function pathStringNoQual = pathString(usefq=false);
    =>
      @ExtendedFunction pathStringNoQual pathString(usefq=false);
  */
  case CLASS(body=parts as DERIVED(__), restriction=R_FUNCTION(__)) then
    let comment = dumpCommentOpt(parts.comment, context)
    let spec = dumpTypeSpec(parts.typeSpec, context)
    let args = (parts.arguments |> earg => dumpElementArg(earg, context) ;separator=', ')
    let attr = dumpElementAttr(parts.attributes)
    let name_of_new_function = '<%name%>'
      <<
        <%comment%>
        @ExtendedFunction <%name_of_new_function%> <%spec%>(<%attr%>)
      >>
  /*PDER. Should not occur. Derived Enumeration and Overload might?*/
end dumpClassElement;

template dumpClassHeader(ClassDef classDef, Absyn.Restriction restriction)
::=
match classDef
  case CLASS_EXTENDS(__) then AbsynDumpTpl.errorMsg("Extend  not supported")
  case PARTS(__) then '<%dumpClassTypeTypeVars(restriction, typeVars)%><%dumpClassTypeSuperType(restriction)%>'
  else AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassHeader: <%dumpClassTypeSuperType(classDef)%>")
end dumpClassHeader;

template dumpClassTypeSuperType(Absyn.Restriction r)
::=
match r
  case R_METARECORD(__) then '<: <%dumpPathJL(name)%>'
  case R_FUNCTION(__) then '' //Do nothing here for functions.. For now
end dumpClassTypeSuperType;

template dumpClassTypeTypeVars(Absyn.Restriction restriction, list<String> typeVars)
::=
match restriction
  case R_UNIONTYPE(__) then
    (if typeVars then ("{" + (typeVars |> tv => tv ; separator=",") + "}"))
  /*Not pretty. But should solve generic functions scathered here and there*/
  case R_FUNCTION(__) then
    (if typeVars then (typeVars |> tv => (tv + " = Any") ; separator="\n"))
  else ""
end dumpClassTypeTypeVars;

template dumpClassFooter(ClassDef classDef, String cdefStr, String name, String cmt, String ann)
::=
match classDef
  case DERIVED(__) then AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassFooter: Derived not yet supported.")
  case ENUMERATION(__) then AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassFooterf: ENUMERATION not yet supported.")
  case _ then
    let annotation_str = if ann then '<%ann%> ' else ''
    if cdefStr then
      <<
        <%cdefStr%>
      <%if annotation_str then " "%><%annotation_str%>
      end
      >>
    else
      <<
      <%annotation_str%>end
      >>
end dumpClassFooter;

template dumpInputsJL(list<ElementItem> inputs, Context context)
::=
  let inputStr = (listReverse((MMToJuliaUtil.filterOnDirection(inputs, MMToJuliaUtil.makeInputDirection())))
    |> ei
      => '<%dumpComponentItems(getComponentItemsFromElementItem(ei), context)%>::<%dumpTypeSpecOpt(getTypeSpecFromElementItemOpt(ei), context)%>'
      ;separator=", ")
 '<%inputStr%>'

end dumpInputsJL;

template dumpReturnTypeJL(list<ElementItem> outputs)
::=
match MMToJuliaUtil.filterOnDirection(outputs, MMToJuliaUtil.makeOutputDirection())
  case {} then ""
  case L as H::{} then '::<%dumpOutputsJL(L)%>'
  case L as H::T then '::Tuple{<%dumpOutputsJL(L)%>}'
end dumpReturnTypeJL;

template dumpReturnStrJL(list<ElementItem> outputs, Context context)
::=
match MMToJuliaUtil.filterOnDirection(outputs, MMToJuliaUtil.makeOutputDirection())
  case {} then ""
  case L as H::{} then
  '<%(L |> e => dumpElementItemRaw(e, defaultDumpOptions, context); separator=", ")%>'
  case L as H::T then
  <<
  (<%(L |> e => dumpElementItemRaw(e, defaultDumpOptions, context); separator=", ")%>)
  >>
end dumpReturnStrJL;


template dumpClassDef(Absyn.ClassDef cdef, Context context, DumpOptions options)
::=
match cdef
  case PARTS(__) then
    let body_str = (classParts |> class_part hasindex idx =>
        dumpClassPart(class_part, idx, context, options) ;separator="\n")
    <<
      <%body_str%>
    >>
  case DERIVED(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassDef: Derived not yet supported.")
  case CLASS_EXTENDS(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassDef: CLASS_EXETENDS not yet supported.")
  case ENUMERATION(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassDef: CLASS_ENUMERATION not yet supported.")
  else "TODO Unkown class definition"
end dumpClassDef;

template dumpClassType(Absyn.Restriction restriction)
::=
match restriction
  case R_PACKAGE(__) then "module"
  case R_METARECORD(__) then "struct"
  case R_RECORD(__) then '@Record' //Only handles Metamodelica records(!)
  case R_UNIONTYPE(__) then "uniontype"
  case R_TYPE(__) then '' // Should be const iff we are in a global scope (Julia 1.0 (Packages are not))
  case R_FUNCTION(__) then "function"
  /* TODO: The other ones are probably not relevant for now */
  else AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassType: Unknown restriction for class." + AbsynDumpTpl.dumpRestriction(restriction))
end dumpClassType;

template dumpClassPart(Absyn.ClassPart class_part, Integer idx, Context context, DumpOptions options)
::=
match class_part
  case PUBLIC(__) then
        let el_str = if isFunctionContext(context) then
                       dumpElementItems(filterOnDirection(contents, makeOutputDirection()), context, "", true, options)
                     else
                       dumpElementItems(contents, context,"", true, options)
      <<
        <%el_str%>
      >>
  case PROTECTED(__) then
    let el_str = dumpElementItems(contents, context, "", true, options)
    <<
      <%el_str%>
    >>
  case CONSTRAINTS(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassPart: CONSTRAINTS(__) not supported.")
  case EQUATIONS(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassPart: EQUATIONS(__) not supported.")
  case INITIALEQUATIONS(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassPart: INITIALEQUATIONS() not supported.")
  case ALGORITHMS(__) then
    <<
      <%(contents |> eq => dumpAlgorithmItem(eq, context) ;separator="\n")%>
    >>
  case INITIALALGORITHMS(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassPart: INITIALALGORITHMS() not supported.")
  case EXTERNAL(__) then
    let ann_str = match annotation_ case SOME(ann) then ' <%dumpAnnotation(ann, context)%>;'
    match externalDecl
      case EXTERNALDECL(__) then //Turned of temporary to translate builtin
        "#= Defined in the runtime =#" //AbsynDumpTpl.errorMsg("AbsynToJulia.dumpClassPart: EXTERNALDECL(__) not supported.")
end dumpClassPart;

template dumpElementItems(list<Absyn.ElementItem> items, Context context, String prevSpacing, Boolean first, DumpOptions options)
::=
match items
  case item :: rest_items then
    let spacing = dumpElementItemSpacing(item)
    let pre_spacing = if not first then
      dumpElementItemPreSpacing(spacing, prevSpacing)
    let item_str = dumpElementItem(item, options, context)
    let rest_str = dumpElementItems(rest_items, context, spacing, false, options)
    let post_spacing = if rest_str then spacing
    <<
    <%pre_spacing%>
    <%item_str%><%post_spacing%><%\n%>
    <%if rest_str then rest_str%>
    >>
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
  case PARTS(__) then '<%\n%>'
  case CLASS_EXTENDS(__) then '<%\n%>'
end dumpClassDefSpacing;

template dumpElementItem(Absyn.ElementItem eitem, DumpOptions options, Context context)
::=
match eitem
  case ELEMENTITEM(__) then '<%dumpElement(element, options, context)%>'
  case LEXER_COMMENT(__) then dumpCommentStr(comment)
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
            AbsynDumpTpl.errorMsg("AbsynToJulia.dumpElementItem: on none component type")
      else
        AbsynDumpTpl.errorMsg("AbsynToJulia.dumpElementItem: on none component type")
  case LEXER_COMMENT(__) then dumpCommentStr(comment)
end dumpElementItemRaw;

template dumpElement(Absyn.Element elem, DumpOptions options, Context context)
::=
match elem
  case ELEMENT(__) then
    if boolOr(boolUnparseFileFromInfo(info, options), boolNot(isClassdef(elem))) then
    let final_str = dumpFinal(finalPrefix)
    let redecl_str = match redeclareKeywords case SOME(re) then dumpRedeclare(re)
    let repl_str = match redeclareKeywords case SOME(re) then dumpReplaceable(re)
    let elementSpec_str = dumpElementSpec(specification, options, context)
    let constrainClass_str = match constrainClass case SOME(cc) then dumpConstrainClass(cc, context)
    '<%elementSpec_str%><%constrainClass_str%>'
  case DEFINEUNIT(__) then AbsynDumpTpl.errorMsg("AbsynToJulia.dumpElement: DEFINEUNIT(__) not supported")
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
  case ANNOTATION(elementArgs={}) then "#= annotation() =#"
  case ANNOTATION(__) then
    <<
    #= annotation(
      <%(elementArgs |> earg => dumpElementArg(earg, context) ;separator=',<%\n%>')%>) #=
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
let replaceAllRegular = '<%\ %>#= <%System.stringReplace(System.escapedString(comment, false), "//","")%> =#'
'<%replaceAllRegular%>'
end dumpCommentStr;

template dumpElementArg(Absyn.ElementArg earg, Context context)
::=
match earg
  case MODIFICATION(__) then
    let each_str = dumpEach(eachPrefix)
    let final_str = dumpFinal(finalPrefix)
    let path_str = dumpPathJL(path)
    let mod_str = match modification case SOME(mod) then dumpModification(mod, context)
    let cmt_str = dumpCommentStrOpt(comment)
    '<%each_str%><%final_str%><%path_str%><%mod_str%><%cmt_str%>'
  case REDECLARATION(__) then
    let each_str = dumpEach(eachPrefix)
    let final_str = dumpFinal(finalPrefix)
    let redecl_str = dumpRedeclare(redeclareKeywords)
    let repl_str = dumpReplaceable(redeclareKeywords)
    let eredecl_str = '<%redecl_str%><%each_str%>'
    let elem_str = dumpElementSpec(elementSpec, defaultDumpOptions, context)
    let cc_str = match constrainClass case SOME(cc) then dumpConstrainClass(cc, context)
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
      '(<%(elementArgLst |> earg => dumpElementArg(earg, context) ;separator=", ")%>)'
    let eq_str = dumpEqMod(eqMod, context)
    '<%arg_str%><%eq_str%>'
end dumpModification;

template dumpEqMod(Absyn.EqMod eqmod, Context context)
::= match eqmod case EQMOD(__) then '<%\ %>= <%dumpExp(exp, context)%>'
end dumpEqMod;

template dumpElementSpec(ElementSpec specification, DumpOptions options, Context context)
::=
match specification
  case CLASSDEF(__) then dumpClassElement(class_, options, context)
  case EXTENDS(__) then
    let bc_str = dumpPathJL(path)
    let args_str = (elementArg |> earg => dumpElementArg(earg, context) ;separator=", ")
    let mod_str = if args_str then '(<%args_str%>)'
    let ann_str = dumpAnnotationOptSpace(annotationOpt, context)
    'extends <%bc_str%><%mod_str%><%ann_str%>'
  case COMPONENTS(__) then
    let ty_str = dumpTypeSpec(typeSpec, context)
    let attr_str = dumpElementAttr(attributes)
    /* Remove all items with input-output specification. They are handled earlier and separate! */
    let comps_str = if elementSpecIsOUTPUT_OR_BIDIR(specification) then
                      (components |> comp =>
                        let comp_str = dumpComponentItem(comp, context)
                          '<%comp_str%>::<%ty_str%>'
                      ;separator=", ")
                    else ''
    //TODO more check for more complex variables..
    match context
      case FUNCTION(__) then
        /* No local decl if we do not have a comps_str! */
        if comps_str then
          'local <%comps_str%>'
        else ''
      case UNIONTYPE(__) then
        '<%comps_str%>'
      case PACKAGE(__) then
        '<%comps_str%>'
      else 'ERROR'
  case IMPORT(__) then
    let imp_str = dumpImport(import_)
    'import <%imp_str%>'
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
  /*
    Constants are currently only allowed in the global scope (Julia 1.1).
    TODO: Global scope is only defined as a scope outside a module.
          What do to here, Scope as a parameter?
  */
  case VAR() then ""
  case CONST() then ""
  else AbsynDumpTpl.errorMsg("AbsynToJulia.dumpVariability: Only const and var are supported")
end dumpVariability;

template dumpConstrainClass(Absyn.ConstrainClass cc, Context context)
::=
match cc
  case CONSTRAINCLASS(elementSpec = Absyn.EXTENDS(path = p, elementArg = el)) then
    let path_str = dumpPathJL(p)
    let el_str = if el then '(<%(el |> e => dumpElementArg(e, context) ;separator=", ")%>)'
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
    let cond_str = dumpComponentCondition(condition, context) //TODO. This will complicate things...
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
    '<%name%><%dim_str%><%mod_str%>'
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
  case NAMED_IMPORT(__) then AbsynDumpTpl.errorMsg("Named imports are not implemented!.")
  case QUAL_IMPORT(__) then dumpPathJL(path)
  case UNQUAL_IMPORT(__) then '<%dumpPathJL(path)%>.*'
  case GROUP_IMPORT(__) then
    let prefix_str = dumpPathJL(prefix)
    let groups_str = (groups |> group => dumpGroupImport(group) ;separator=",")
    '<%prefix_str%>.{<%groups_str%>}'
end dumpImport;

template dumpGroupImport(Absyn.GroupImport gimp)
::=
match gimp
  case GROUP_IMPORT_NAME(__) then name
  case GROUP_IMPORT_RENAME(__) then '<%rename%> = <%name%>'
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
  case ALGORITHMITEMCOMMENT(__) then dumpCommentStr(comment)
end dumpAlgorithmItem;

template dumpAlgorithm(Absyn.Algorithm alg, Context context)
::=
match alg
  case ALG_ASSIGN(__) then
    let lhs_str = dumpLhsExp(assignComponent, context)
    let rhs_str = dumpExp(value, context)
    '<%lhs_str%> = <%rhs_str%>'
  case ALG_IF(__) then
    let if_str = dumpAlgorithmBranch(ifExp, trueBranch, "if", context)
    let elseif_str = (elseIfAlgorithmBranch |> (c, b) =>
        dumpAlgorithmBranch(c, b, "elseif", context) ;separator="\n")
    let else_branch_str = dumpAlgorithmItems(elseBranch, context)
    let else_str = if else_branch_str then
      <<
      else
        <%else_branch_str%>
      >>
    <<
    <%if_str%>
    <%elseif_str%>
    <%else_str%>
    end
    >>
  case ALG_FOR(__) then
    let iter_str = dumpForIterators(iterators, context)
    let body_str = dumpAlgorithmItems(forBody, context)
    <<
    for <%iter_str%>
      <%body_str%>
    end
    >>
  case ALG_WHILE(__) then
    let while_str = dumpAlgorithmBranch(boolExpr, whileBody, "while", context)
    <<
    <%while_str%>
    end
    >>
  case ALG_WHEN_A(__) then  AbsynDumpTpl.errorMsg("When statements are not allowed!.")
  case ALG_NORETCALL(__) then
    let name_str = dumpCref(functionCall, context)
    let args_str = dumpFunctionArgs(functionArgs, context)
    '<%name_str%>(<%args_str%>)'
  /*Here we need to gather all return values for the function*/
  case ALG_RETURN(__) then dumpAlgReturnString(context)
  case ALG_BREAK(__) then "break"
  case ALG_FAILURE(__) then
    let arg_str = if equ then dumpAlgorithmItems(equ, context) else "..."
    'failure(<%arg_str%>)' //We could simply use throw or we can define a macro for this
  case ALG_TRY(__) then
    let arg1 = dumpAlgorithmItems(body, context)
    let arg2 = dumpAlgorithmItems(elseBody, context)
    <<
    try
      <%arg1%>
    catch
      <%arg2%>
    end
    >>
  case ALG_CONTINUE(__) then "continue"
end dumpAlgorithm;

template dumpAlgReturnString(Context context)
  "Dumps the return string for a specific function context"
::= match context
    case FUNCTION(__) then 'return <%retValsStr%>'
    /*TODO: Should not occur? Models with sections?*/
    else "return"
end dumpAlgReturnString;

template dumpAlgorithmBranch(Absyn.Exp cond, list<Absyn.AlgorithmItem> body,
String header, Context context)
::=
  let cond_str = dumpExp(cond, context)
  let body_str = (body |> eq => dumpAlgorithmItem(eq, context) ;separator="\n")
  <<
  <%header%> <%cond_str%>
    <%body_str%>
  >>
end dumpAlgorithmBranch;

template dumpPathJL(Absyn.Path path)
"Wrapper function for dump path.
 Needed since certain keywords will have a sligthly different meaning in Julia"
::=
match path
  case FULLYQUALIFIED(__) then
    '.<%AbsynDumpTpl.dumpPath(path)%>'
  case QUALIFIED(__) then
    if (Flags.getConfigBool(Flags.MODELICA_OUTPUT)) then
    '<%name%>__<%AbsynDumpTpl.dumpPath(path)%>'
    else
    '<%name%>.<%AbsynDumpTpl.dumpPath(path)%>'
  /*Julia keywords have a slightly different semantic meaning*/
  /*
    ModelicaReal = Union {Signed, AbstractFloat}
    ModelicaInteger is represented as it's own type.
    Bool "should" not require any change
    Clock e.t.c are probably not necessary to add since it is pure Modelica only.
  */
  case IDENT(__) then
    match name
      case "Real" then 'ModelicaReal'
      case "Integer" then 'ModelicaInteger'
      case "Boolean" then 'Bool'
      case "list" then 'List'
      case "array" then 'Array'
      case "tuple" then 'Tuple'
      else '<%name%>'
  else
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpPathJL: Unknown path.")
end dumpPathJL;

template dumpPathNoQual(Absyn.Path path)
::=
match path
  case FULLYQUALIFIED(__) then
    dumpPathJL(path)
  else
    dumpPathJL(path)
end dumpPathNoQual;

template dumpTypeSpecOpt(Option<Absyn.TypeSpec> typespecOpt, Context context)
::= match typespecOpt case SOME(ts) then dumpTypeSpec(ts, context) else ""
end dumpTypeSpecOpt;

template dumpTypeSpec(Absyn.TypeSpec typeSpec, Context context)
"Dumps the type specification"
::=
match typeSpec
  case TPATH(__) then
    let path_str = dumpPathJL(path)
    let arraydim_str = dumpArrayDimOpt(arrayDim, context)
    '<%path_str%><%arraydim_str%>'
  case TCOMPLEX(__) then
    let path_str = dumpPathJL(path)
    let ty_str = (typeSpecs |> ty => dumpTypeSpec(ty, context) ;separator=", ")
    let arraydim_str = dumpArrayDimOpt(arrayDim, context)
    '<%path_str%>{<%ty_str%>}<%arraydim_str%>'
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
    'Array{<%sub_str%>}'
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
  case REAL(__) then value
  case CREF(__) then dumpCref(componentRef, context)
  case STRING(__) then ('"<%value; absIndent=0%>"')
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
    '<%op_str%> <%exp_str%>'
  case e as RELATION(__) then
    let lhs_str = dumpOperand(exp1, e, true, context)
    let rhs_str = dumpOperand(exp2, e, false, context)
    let op_str = dumpOperator(op)
    '<%lhs_str%> <%op_str%> <%rhs_str%>'
  case IFEXP(__) then dumpIfExp(exp, context)
  case CALL(function_=Absyn.CREF_IDENT(name="$array")) then
    let args_str = dumpFunctionArgs(functionArgs, context)
    '{<%args_str%>}'
  case CALL(__) then
    let func_str = dumpCref(function_, context)
    let args_str = dumpFunctionArgs(functionArgs, context)
    '<%func_str%>(<%args_str%>)'
  case PARTEVALFUNCTION(__) then
    let func_str = dumpCref(function_, context)
    let args_str = dumpFunctionArgs(functionArgs, context)
    /*  Same scenario when extending functions.
        We pass a function and change the parameters
    */
    '@ExtendedAnonFunction <%func_str%>(<%args_str%>)'
  case ARRAY(__) /*MM grammar changing behaviour... Remember to change this IF regular arrays would occur... Probably not used so can be ignored */ then
    let array_str = (arrayExp |> e => dumpExp(e, context) ;separator=", ")
    'list(<%array_str%>)'
  case MATRIX(__) then
    let matrix_str = (matrix |> row =>
        (row |> e => dumpExp(e, context) ;separator=", ") ;separator="; ")
    '[<%matrix_str%>]'
  case e as RANGE(step = SOME(step)) then
    let start_str = dumpOperand(start, e, false, context)
    let step_str = dumpOperand(step, e, false, context)
    let stop_str = dumpOperand(stop, e, false, context)
    '<%start_str%>:<%step_str%>:<%stop_str%>'
  case e as RANGE(step = NONE()) then
    let start_str = dumpOperand(start, e, false, context)
    let stop_str = dumpOperand(stop, e, false, context)
    '<%start_str%>:<%stop_str%>'
  case TUPLE(__) then
    let tuple_str = (expressions |> e => dumpExp(e,context); separator=", " ;empty)
    '(<%tuple_str%>)'
  case END(__) then 'end'
  case CODE(__) then '$Code(<%dumpCodeNode(code, context)%>)'
  case AS(__) then
    let exp_str = dumpExp(exp, context)
    /* TODO Macro might be needed for this case*/
    '<%id%> = <%exp_str%>'
  case CONS(__) then
    let head_str = dumpExp(head, context)
    let rest_str = dumpExp(rest, context)
    '<%head_str%> => <%rest_str%>'
  case MATCHEXP(__) then dumpMatchExp(exp)
  case LIST(__) then
    let list_str = (exps |> e => dumpExp(e, context) ;separator=", ")
    'list(<%list_str%>)'
  case DOT(__) then
    '<%dumpExp(exp, context)%>.<%dumpExp(index, context)%>'
  case _ then '/* AbsynDumpTpl.dumpExp: UNHANDLED Abyn.Exp */'
end dumpExp;

template dumpPattern(Absyn.Exp exp)
::=
match exp
  case INTEGER(__) then value
  case REAL(__) then value
  case CREF(__) then dumpCref(componentRef, functionContext /*Only occurs in fc*/)
  case STRING(__) then ('"<%stringReplace(value,"$","\\$"); absIndent=0%>"')
  case BOOL(__) then value
  case ARRAY(arrayExp=exps)
  case LIST(__)
  case CALL(function_=Absyn.CREF_IDENT(name="list"), functionArgs=FUNCTIONARGS(args=exps))
  case CALL(function_=Absyn.CREF_IDENT(name="$array"), functionArgs=FUNCTIONARGS(args=exps)) then
    '<%exps |> e => '<%dumpPattern(e)%> => '%> Nil()'
  case CALL(function_=function_ as CREF_IDENT(name=id)) then
    let args_str = dumpFunctionArgsPattern(functionArgs)
    let func_str = (match id
    case "list" then "List"
    else dumpCref(function_, functionContext))
    '<%func_str%>(<%args_str%>)'
  case CALL(__) then
    let func_str = dumpCref(function_, functionContext)
    let args_str = dumpFunctionArgsPattern(functionArgs)
    '<%func_str%>(<%args_str%>)'
  case TUPLE(__) then
    let tuple_str = (expressions |> e => dumpPattern(e); separator=", " ;empty)
    '(<%tuple_str%>)'
  case AS(__) then
  /*TODO: Macro might be needed here*/
    let exp_str = dumpPattern(exp)
    '<%id%> = <%exp_str%>'
  case CONS(__) then
    let consOp = dumpCons(dumpPattern(head), dumpPattern(rest))
    '<%consOp%>'
  case _ then '#= AbsynDumpTpl.dumpPattern: UNHANDLED Abyn.Exp  =#'
end dumpPattern;

template dumpCons(String headString, String tailString)
::= '<%headString%> => <%tailString%>'
end dumpCons;

template dumpFunctionArgsPattern(Absyn.FunctionArgs args)
::=
match args
  case FUNCTIONARGS(__) then
    let args_str = (args |> arg => dumpPattern(arg) ;separator=", ")
    let namedargs_str = (argNames |> narg => dumpNamedArgPattern(narg) ;separator=", ")
    let separator = if args_str then if argNames then ', '
    '<%args_str%><%separator%><%namedargs_str%>'
  else 'ERROR FOR_ITER_FARG in pattern'
end dumpFunctionArgsPattern;

template dumpNamedArgPattern(Absyn.NamedArg narg)
::=
match narg
  case NAMEDARG(__) then
    '<%argName%> = <%dumpPattern(argValue)%>'
end dumpNamedArgPattern;

template dumpLhsExp(Absyn.Exp lhs, Context context)
::=
match lhs
  case IFEXP(__) then '(<%dumpExp(lhs, context)%>)'
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
    'if (<%cond_str%>) <%true_branch_str%><%else_if_str%> else <%else_branch_str%> end'
end dumpIfExp;

template dumpElseIfExp(list<tuple<Absyn.Exp, Absyn.Exp>> else_if, Context context)
::=
  else_if |> eib as (cond, branch) =>
    let cond_str = dumpExp(cond, context)
    let branch_str = dumpExp(branch, context)
    ' elseif (<%cond_str%>) <%branch_str%>' ;separator="\n"
end dumpElseIfExp;

template dumpCodeNode(Absyn.CodeNode code, Context context)
::=
match code
  case C_TYPENAME(__) then dumpPathJL(path)
  case C_VARIABLENAME(__) then dumpCref(componentRef, context)
  case C_CONSTRAINTSECTION(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpCodeNode: C_CONSTRAINTSECTION not supported")
  case C_EQUATIONSECTION(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpCodeNode: C_CONSTRAINTSECTION not supported")
  case C_ALGORITHMSECTION(__) then
    AbsynDumpTpl.errorMsg("AbsynToJulia.dumpCodeNode: C_ALGORITHMSECTION not supported")
  case C_ELEMENT(__) then dumpElement(element, Dump.defaultDumpOptions, context)
  case C_EXPRESSION(__) then dumpExp(exp, context)
  case C_MODIFICATION(__) then dumpModification(modification, context)
end dumpCodeNode;

//John: look at this one more time...
template dumpMatchExp(Absyn.Exp match_exp)
::=
match match_exp
  case MATCHEXP(__) then
    let match_ty_str = dumpMatchType(matchTy)
    let input_str = dumpExp(inputExp, functionContext)
    let locals_str = dumpMatchLocals(localDecls)
    let cases_str = (cases |> c => dumpMatchCase(c, functionContext) ;separator="\n\n")
    let cmt_str = dumpCommentStrOpt(comment)
    <<
    begin
      <%locals_str%>
      <%match_ty_str%> <%input_str%> begin
        <%cases_str%><%cmt_str%>
      end
    end
    >>
end dumpMatchExp;

template dumpMatchType(Absyn.MatchType match_type)
::=
match match_type
  case MATCH() then "@match"
  case MATCHCONTINUE() then "@matchcontinue"
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
  <<
    <%(locals |> decl => dumpElementItem(decl, defaultDumpOptions, functionContext) ;separator="\n")%>
  >>
end dumpMatchLocals;

template dumpMatchCase(Absyn.Case c, Context context)
::=
match c
  case CASE(__) then
    let pattern_str = dumpPattern(pattern)
    let guard_str = match patternGuard case SOME(g) then 'guard <%dumpExp(g, context)%> '
    let eql_str = dumpMatchContents(classPart)
    let result_str = dumpExp(result, context)
    let cmt_str = dumpCommentStrOpt(comment)
    <<
    <%pattern_str%> <%guard_str%><%cmt_str%> => begin
      <%eql_str%>
      <%result_str%>
    end
    >>
  case ELSE(__) then
    let eql_str = dumpMatchContents(classPart)
    let result_str = dumpExp(result, context)
    let cmt_str = dumpCommentStrOpt(comment)
    <<
    _ <%cmt_str%> => begin
        <%eql_str%>
        <%result_str%>
    end
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
    '<%name%><%dumpSubscripts(subscripts, context)%>.<%dumpCref(componentRef, context)%>'
  case CREF_IDENT(__)
    then '<%name%><%dumpSubscripts(subscripts, context)%>'
  case CREF_FULLYQUALIFIED(__) then '.<%dumpCref(componentRef, context)%>'
  case WILD(__) then if Config.acceptMetaModelicaGrammar() then "_" else ""
  case ALLWILD(__) then '__'
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
    '<%exp_str%> <%match iterType case THREAD(__) then "threaded "%>for <%iter_str%>'
end dumpFunctionArgs;

template dumpNamedArg(Absyn.NamedArg narg, Context context)
::=
match narg
  case NAMEDARG(__) then
    '<%argName%> = <%dumpExp(argValue, context)%>'
end dumpNamedArg;

template dumpForIterators(Absyn.ForIterators iters, Context context)
::= (iters |> i => dumpForIterator(i, context) ;separator=", ")
end dumpForIterators;

template dumpForIterator(Absyn.ForIterator iterator, Context context)
::=
match iterator
  case ITERATOR(__) then
    let range_str = match range case SOME(r) then ' in <%dumpExp(r, context)%>'
    let guard_str = match guardExp case SOME(g) then ' guard <%dumpExp(g, context)%>'
    '<%name%><%guard_str%><%range_str%>'
end dumpForIterator;

template dumpOutputsJL(list<ElementItem> elements)
::=
  let outputStr = (elements |> e => dumpTypeSpecOpt(AbsynUtil.getTypeSpecFromElementItemOpt(e), functionContext) ;separator=", ")
  '<%outputStr%>'
end dumpOutputsJL;

annotation(__OpenModelica_Interface="backend");
end AbsynToJulia;
