/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2019, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.2.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GPL VERSION 3,
 * ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the Open Source Modelica
 * Consortium (OSMC) Public License (OSMC-PL) are obtained
 * from OSMC, either from the above address,
 * from the URLs: http://www.ida.liu.se/projects/OpenModelica or
 * http://www.openmodelica.org, and in the OpenModelica distribution.
 * GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of  MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */

interface package AbsynToJuliaTV

package builtin

  function listHead
    input list<T> lst;
    output T head;
    replaceable type T subtypeof Any;
  end listHead;

  function listEmpty
    input list<T> lst;
    output Boolean b;
    replaceable type T subtypeof Any;
  end listEmpty;

  function listReverse
    input list<T> inLst;
    output list<T> outLst;
    replaceable type T subtypeof Any;
  end listReverse;

  function boolAnd
    input Boolean b1;
    input Boolean b2;
    output Boolean b;
  end boolAnd;

  function boolOr
    input Boolean a;
    input Boolean b;
    output Boolean c;
  end boolOr;

  function boolNot
    input Boolean b;
    output Boolean nb;
  end boolNot;

  uniontype SourceInfo
    record SOURCEINFO
      String fileName;
      Boolean isReadOnly;
      Integer lineNumberStart;
      Integer columnNumberStart;
      Integer lineNumberEnd;
      Integer columnNumberEnd;
    end SOURCEINFO;
  end SourceInfo;
end builtin;

package Static
  function fromEquationsToAlgAssignments
    input Absyn.ClassPart cp;
    output list<Absyn.AlgorithmItem> algsOut;
  end fromEquationsToAlgAssignments;
end Static;

package Absyn
  type Ident = String;

  uniontype Program
    record PROGRAM
      list<Class> classes;
      Within within_;
    end PROGRAM;
  end Program;

  uniontype Within
    record WITHIN
      Path path;
    end WITHIN;

    record TOP end TOP;
  end Within;

  uniontype Class
    record CLASS
      Ident name;
      Boolean partialPrefix;
      Boolean finalPrefix;
      Boolean encapsulatedPrefix;
      Restriction restriction;
      ClassDef body;
      builtin.SourceInfo info;
    end CLASS;
  end Class;

  uniontype ClassDef
    record PARTS
      list<String> typeVars;
      list<NamedArg> classAttrs;
      list<ClassPart> classParts;
      list<Annotation> ann;
      Option<String> comment;
    end PARTS;

    record DERIVED
      TypeSpec typeSpec;
      ElementAttributes attributes;
      list<ElementArg> arguments;
      Option<Comment> comment;
    end DERIVED;

    record ENUMERATION
      EnumDef enumLiterals;
      Option<Comment> comment;
    end ENUMERATION;

    record OVERLOAD
      list<Path> functionNames;
      Option<Comment> comment;
    end OVERLOAD;

    record CLASS_EXTENDS
      Ident baseClassName;
      list<ElementArg> modifications;
      Option<String> comment;
      list<ClassPart> parts;
      list<Annotation> ann;
    end CLASS_EXTENDS;

    record PDER
      Path functionName;
      list<Ident> vars;
      Option<Comment> comment;
    end PDER;
  end ClassDef;

  uniontype TypeSpec
    record TPATH
      Path path;
      Option<ArrayDim> arrayDim;
    end TPATH;

    record TCOMPLEX
      Path path;
      list<TypeSpec> typeSpecs;
      Option<ArrayDim> arrayDim;
    end TCOMPLEX;
  end TypeSpec;

  uniontype EnumDef
    record ENUMLITERALS
      list<EnumLiteral> enumLiterals;
    end ENUMLITERALS;

    record ENUM_COLON end ENUM_COLON;
  end EnumDef;

  uniontype EnumLiteral
    record ENUMLITERAL
      Ident literal;
      Option<Comment> comment;
    end ENUMLITERAL;
  end EnumLiteral;

  uniontype ClassPart
    record PUBLIC
      list<ElementItem> contents;
    end PUBLIC;

    record PROTECTED
      list<ElementItem> contents;
    end PROTECTED;

    record CONSTRAINTS
      list<Exp> contents;
    end CONSTRAINTS;

    record EQUATIONS
      list<EquationItem> contents;
    end EQUATIONS;

    record INITIALEQUATIONS
      list<EquationItem> contents;
    end INITIALEQUATIONS;

    record ALGORITHMS
      list<AlgorithmItem> contents;
    end ALGORITHMS;

    record INITIALALGORITHMS
      list<AlgorithmItem> contents;
    end INITIALALGORITHMS;

    record EXTERNAL
      ExternalDecl externalDecl;
      Option<Annotation> annotation_;
    end EXTERNAL;
  end ClassPart;

  uniontype ElementItem
    record ELEMENTITEM
      Element element;
    end ELEMENTITEM;

    record LEXER_COMMENT
      String comment;
    end LEXER_COMMENT;
  end ElementItem;

  uniontype Element
    record ELEMENT
      Boolean finalPrefix;
      Option<RedeclareKeywords> redeclareKeywords;
      InnerOuter innerOuter;
      ElementSpec specification;
      builtin.SourceInfo info;
      Option<ConstrainClass> constrainClass;
    end ELEMENT;

    record DEFINEUNIT
      Ident name;
      list<NamedArg> args;
    end DEFINEUNIT;

    record TEXT
      Option<Ident> optName;
      String string;
      builtin.SourceInfo info;
    end TEXT;
  end Element;

  uniontype ConstrainClass
    record CONSTRAINCLASS
      ElementSpec elementSpec;
      Option<Comment> comment;
    end CONSTRAINCLASS;
  end ConstrainClass;

  uniontype ElementSpec
    record CLASSDEF
      Boolean replaceable_;
      Class class_;
    end CLASSDEF;

    record EXTENDS
      Path path;
      list<ElementArg> elementArg;
      Option<Annotation> annotationOpt;
    end EXTENDS;

    record IMPORT
      Import import_;
      Option<Comment> comment;
      builtin.SourceInfo info;
    end IMPORT;

    record COMPONENTS
      ElementAttributes attributes;
      TypeSpec typeSpec;
      list<ComponentItem> components;
    end COMPONENTS;
  end ElementSpec;

  uniontype InnerOuter
    record INNER end INNER;
    record OUTER end OUTER;
    record INNER_OUTER end INNER_OUTER;
    record NOT_INNER_OUTER end NOT_INNER_OUTER;
  end InnerOuter;

  uniontype Import
    record NAMED_IMPORT
      Ident name;
      Path path;
    end NAMED_IMPORT;

    record QUAL_IMPORT
      Path path;
    end QUAL_IMPORT;

    record UNQUAL_IMPORT
      Path path;
    end UNQUAL_IMPORT;

    record GROUP_IMPORT
      Path prefix;
      list<GroupImport> groups;
    end GROUP_IMPORT;
  end Import;

  uniontype GroupImport
    record GROUP_IMPORT_NAME
      String name;
    end GROUP_IMPORT_NAME;

    record GROUP_IMPORT_RENAME
      String rename;
      String name;
    end GROUP_IMPORT_RENAME;
  end GroupImport;

  uniontype ComponentItem
    record COMPONENTITEM
      Component component;
      Option<ComponentCondition> condition;
      Option<Comment> comment;
    end COMPONENTITEM;
  end ComponentItem;

  type ComponentCondition = Exp;

  uniontype Component
    record COMPONENT
      Ident name;
      ArrayDim arrayDim;
      Option<Modification> modification;
    end COMPONENT;
  end Component;

  uniontype EquationItem
    record EQUATIONITEM
      Equation equation_;
      Option<Comment> comment;
    end EQUATIONITEM;

    record EQUATIONITEMCOMMENT
      String comment;
    end EQUATIONITEMCOMMENT;
  end EquationItem;

  uniontype AlgorithmItem
    record ALGORITHMITEM
      Algorithm algorithm_;
      Option<Comment> comment;
    end ALGORITHMITEM;

    record ALGORITHMITEMCOMMENT
      String comment;
    end ALGORITHMITEMCOMMENT;
  end AlgorithmItem;

  uniontype Equation
    record EQ_IF
      Exp ifExp;
      list<EquationItem> equationTrueItems;
      list<tuple<Exp, list<EquationItem>>> elseIfBranches;
      list<EquationItem> equationElseItems;
    end EQ_IF;

    record EQ_EQUALS
      Exp leftSide;
      Exp rightSide;
    end EQ_EQUALS;

    record EQ_PDE
      Exp leftSide;
      Exp rightSide;
      ComponentRef domain;
    end EQ_PDE;

    record EQ_CONNECT
      ComponentRef connector1;
      ComponentRef connector2;
    end EQ_CONNECT;

    record EQ_FOR
      ForIterators iterators;
      list<EquationItem> forEquations;
    end EQ_FOR;

    record EQ_WHEN_E
      Exp whenExp;
      list<EquationItem> whenEquations;
      list<tuple<Exp, list<EquationItem>>> elseWhenEquations;
    end EQ_WHEN_E;

    record EQ_NORETCALL
      ComponentRef functionName;
      FunctionArgs functionArgs;
    end EQ_NORETCALL;

    record EQ_FAILURE
      EquationItem equ;
    end EQ_FAILURE;
  end Equation;

  uniontype Algorithm
    record ALG_ASSIGN
      Exp assignComponent;
      Exp value;
    end ALG_ASSIGN;

    record ALG_IF
      Exp ifExp;
      list<AlgorithmItem> trueBranch;
      list<tuple<Exp, list<AlgorithmItem>>> elseIfAlgorithmBranch;
      list<AlgorithmItem> elseBranch;
    end ALG_IF;

    record ALG_FOR
      ForIterators iterators;
      list<AlgorithmItem> forBody;
    end ALG_FOR;

    record ALG_PARFOR
      ForIterators iterators;
      list<AlgorithmItem> parforBody;
    end ALG_PARFOR;

    record ALG_WHILE
      Exp boolExpr;
      list<AlgorithmItem> whileBody;
    end ALG_WHILE;

    record ALG_WHEN_A
      Exp boolExpr;
      list<AlgorithmItem> whenBody;
      list<tuple<Exp, list<AlgorithmItem>>> elseWhenAlgorithmBranch;
    end ALG_WHEN_A;

    record ALG_NORETCALL
      ComponentRef functionCall;
      FunctionArgs functionArgs;
    end ALG_NORETCALL;

    record ALG_RETURN end ALG_RETURN;
    record ALG_BREAK end ALG_BREAK;

    record ALG_FAILURE
      list<AlgorithmItem> equ;
    end ALG_FAILURE;

    record ALG_TRY
      list<AlgorithmItem> body;
      list<AlgorithmItem> elseBody;
    end ALG_TRY;

    record ALG_CONTINUE end ALG_CONTINUE;
  end Algorithm;

  uniontype Modification
    record CLASSMOD
      list<ElementArg> elementArgLst;
      EqMod eqMod;
    end CLASSMOD;
  end Modification;

  uniontype EqMod
    record NOMOD end NOMOD;

    record EQMOD
      Exp exp;
    end EQMOD;
  end EqMod;

  uniontype ElementArg
    record MODIFICATION
      Boolean finalPrefix;
      Each eachPrefix;
      Path path;
      Option<Modification> modification;
      Option<String> comment;
    end MODIFICATION;

    record REDECLARATION
      Boolean finalPrefix;
      RedeclareKeywords redeclareKeywords;
      Each eachPrefix;
      ElementSpec elementSpec;
      Option<ConstrainClass> constrainClass;
    end REDECLARATION;
  end ElementArg;

  uniontype RedeclareKeywords
    record REDECLARE end REDECLARE;
    record REPLACEABLE end REPLACEABLE;
    record REDECLARE_REPLACEABLE end REDECLARE_REPLACEABLE;
  end RedeclareKeywords;

  uniontype Each
    record EACH end EACH;
    record NON_EACH end NON_EACH;
  end Each;

  uniontype ElementAttributes
    record ATTR
      Boolean flowPrefix;
      Boolean streamPrefix;
      Parallelism parallelism;
      Variability variability;
      IsField isField;
      Direction direction;
      ArrayDim arrayDim;
    end ATTR;
  end ElementAttributes;

  uniontype IsField "Is field"
    record NONFIELD "variable is not a field"  end NONFIELD;
    record FIELD "variable is a field"         end FIELD;
  end IsField;

  uniontype Parallelism
    record PARGLOBAL end PARGLOBAL;
    record PARLOCAL end PARLOCAL;
    record NON_PARALLEL end NON_PARALLEL;
  end Parallelism;

  uniontype FlowStream
    record FLOW end FLOW;
    record STREAM end STREAM;
    record NOT_FLOW_STREAM end NOT_FLOW_STREAM;
  end FlowStream;

  uniontype Variability
    record VAR end VAR;
    record DISCRETE end DISCRETE;
    record PARAM end PARAM;
    record CONST end CONST;
  end Variability;

  uniontype Direction
    record INPUT end INPUT;
    record OUTPUT end OUTPUT;
    record BIDIR end BIDIR;
    record INPUT_OUTPUT end INPUT_OUTPUT;
  end Direction;

  uniontype ForIterator
    record ITERATOR
      String name;
      Option<Exp> guardExp;
      Option<Exp> range;
    end ITERATOR;
  end ForIterator;

  type ForIterators = list<ForIterator>;

  uniontype Exp
    record INTEGER
      Integer value;
    end INTEGER;

    record REAL
      String value;
    end REAL;

    record CREF
      ComponentRef componentRef;
    end CREF;

    record STRING
      String value;
    end STRING;

    record BOOL
      Boolean value;
    end BOOL;

    record BINARY
      Exp exp1;
      Operator op;
      Exp exp2;
    end BINARY;

    record UNARY
      Operator op;
      Exp exp;
    end UNARY;

    record LBINARY
      Exp exp1;
      Operator op;
      Exp exp2;
    end LBINARY;

    record LUNARY
      Operator op;
      Exp exp;
    end LUNARY;

    record RELATION
      Exp exp1;
      Operator op;
      Exp exp2;
    end RELATION;

    record IFEXP
      Exp ifExp;
      Exp trueBranch;
      Exp elseBranch;
      list<tuple<Exp, Exp>> elseIfBranch;
    end IFEXP;

    record CALL
      ComponentRef function_;
      FunctionArgs functionArgs ;
    end CALL;

    record PARTEVALFUNCTION
      ComponentRef function_;
      FunctionArgs functionArgs ;
    end PARTEVALFUNCTION;

    record ARRAY
      list<Exp> arrayExp ;
    end ARRAY;

    record MATRIX
      list<list<Exp>> matrix ;
    end MATRIX;

    record RANGE
      Exp start;
      Option<Exp> step;
      Exp stop;
    end RANGE;

    record TUPLE
      list<Exp> expressions;
    end TUPLE;

    record END
    end END;

    record CODE
      CodeNode code;
    end CODE;

    record AS
      Ident id;
      Exp exp;
    end AS;

    record CONS
      Exp head;
      Exp rest;
    end CONS;

    record MATCHEXP
      MatchType matchTy;
      Exp inputExp;
      list<ElementItem> localDecls;
      list<Case> cases;
      Option<String> comment;
    end MATCHEXP;

    record LIST
      list<Exp> exps;
    end LIST;

    record DOT "exp.index"
      Exp exp;
      Exp index;
    end DOT;

  end Exp;

  uniontype Case
    record CASE
      Exp pattern;
      Option<Exp> patternGuard;
      builtin.SourceInfo patternInfo;
      list<ElementItem> localDecls;
      ClassPart classPart;
      Exp result;
      builtin.SourceInfo resultInfo;
      Option<String> comment;
      builtin.SourceInfo info;
    end CASE;

    record ELSE
      list<ElementItem> localDecls;
      ClassPart classPart;
      Exp result;
      builtin.SourceInfo resultInfo;
      Option<String> comment;
      builtin.SourceInfo info;
    end ELSE;
  end Case;

  uniontype MatchType
    record MATCH end MATCH;
    record MATCHCONTINUE end MATCHCONTINUE;
  end MatchType;

  uniontype CodeNode
    record C_TYPENAME
      Path path;
    end C_TYPENAME;

    record C_VARIABLENAME
      ComponentRef componentRef;
    end C_VARIABLENAME;

    record C_CONSTRAINTSECTION
      Boolean boolean;
      list<EquationItem> equationItemLst;
    end C_CONSTRAINTSECTION;

    record C_EQUATIONSECTION
      Boolean boolean;
      list<EquationItem> equationItemLst;
    end C_EQUATIONSECTION;

    record C_ALGORITHMSECTION
      Boolean boolean;
      list<AlgorithmItem> algorithmItemLst;
    end C_ALGORITHMSECTION;

    record C_ELEMENT
      Element element;
    end C_ELEMENT;

    record C_EXPRESSION
      Exp exp;
    end C_EXPRESSION;

    record C_MODIFICATION
      Modification modification;
    end C_MODIFICATION;
  end CodeNode;

  uniontype FunctionArgs
    record FUNCTIONARGS
      list<Exp> args;
      list<NamedArg> argNames;
    end FUNCTIONARGS;

    record FOR_ITER_FARG
      Exp  exp;
      ReductionIterType iterType;
      ForIterators iterators;
    end FOR_ITER_FARG;
  end FunctionArgs;

  uniontype ReductionIterType
    record COMBINE
    end COMBINE;
    record THREAD
    end THREAD;
  end ReductionIterType;

  uniontype NamedArg
    record NAMEDARG
      Ident argName;
      Exp argValue;
    end NAMEDARG;
  end NamedArg;

  uniontype Operator
    record ADD end ADD;
    record SUB end SUB;
    record MUL end MUL;
    record DIV end DIV;
    record POW end POW;
    record UPLUS end UPLUS;
    record UMINUS end UMINUS;
    record ADD_EW end ADD_EW;
    record SUB_EW end SUB_EW;
    record MUL_EW end MUL_EW;
    record DIV_EW end DIV_EW;
    record POW_EW end POW_EW;
    record UPLUS_EW end UPLUS_EW;
    record UMINUS_EW end UMINUS_EW;
    record AND end AND;
    record OR end OR;
    record NOT end NOT;
    record LESS end LESS;
    record LESSEQ end LESSEQ;
    record GREATER end GREATER;
    record GREATEREQ end GREATEREQ;
    record EQUAL end EQUAL;
    record NEQUAL end NEQUAL;
  end Operator;

  uniontype Subscript
    record NOSUB end NOSUB;

    record SUBSCRIPT
      Exp subscript;
    end SUBSCRIPT;
  end Subscript;

  type ArrayDim = list<Subscript>;

  uniontype ComponentRef
    record CREF_FULLYQUALIFIED
      ComponentRef componentRef;
    end CREF_FULLYQUALIFIED;
    record CREF_QUAL
      Ident name;
      list<Subscript> subscripts;
      ComponentRef componentRef;
    end CREF_QUAL;

    record CREF_IDENT
      Ident name;
      list<Subscript> subscripts;
    end CREF_IDENT;

    record WILD end WILD;
    record ALLWILD end ALLWILD;
  end ComponentRef;

  uniontype Path
    record QUALIFIED
      Ident name;
      Path path;
    end QUALIFIED;

    record IDENT
      Ident name;
    end IDENT;

    record FULLYQUALIFIED
      Path path;
    end FULLYQUALIFIED;
  end Path;

  uniontype Restriction
    record R_CLASS end R_CLASS;
    record R_OPTIMIZATION end R_OPTIMIZATION;
    record R_MODEL end R_MODEL;
    record R_RECORD end R_RECORD;
    record R_BLOCK end R_BLOCK;
    record R_CONNECTOR end R_CONNECTOR;
    record R_EXP_CONNECTOR end R_EXP_CONNECTOR;
    record R_TYPE end R_TYPE;
    record R_PACKAGE end R_PACKAGE;
    record R_FUNCTION
      FunctionRestriction functionRestriction;
    end R_FUNCTION;
    record R_OPERATOR end R_OPERATOR;
    record R_OPERATOR_RECORD end R_OPERATOR_RECORD;
    record R_ENUMERATION end R_ENUMERATION;
    record R_PREDEFINED_INTEGER end R_PREDEFINED_INTEGER;
    record R_PREDEFINED_REAL end R_PREDEFINED_REAL;
    record R_PREDEFINED_STRING end R_PREDEFINED_STRING;
    record R_PREDEFINED_BOOLEAN end R_PREDEFINED_BOOLEAN;
    record R_PREDEFINED_ENUMERATION end R_PREDEFINED_ENUMERATION;
    record R_UNIONTYPE end R_UNIONTYPE;
    record R_METARECORD
      Path name;
      Integer index;
      Boolean singleton;
      list<String> typeVars;
    end R_METARECORD;
    record R_UNKNOWN  end R_UNKNOWN;
  end Restriction;

  uniontype FunctionPurity
    record PURE end PURE;
    record IMPURE end IMPURE;
    record NO_PURITY end NO_PURITY;
  end FunctionPurity;

  uniontype FunctionRestriction
    record FR_NORMAL_FUNCTION
      FunctionPurity purity;
    end FR_NORMAL_FUNCTION;

    record FR_OPERATOR_FUNCTION end FR_OPERATOR_FUNCTION;
    record FR_PARALLEL_FUNCTION end FR_PARALLEL_FUNCTION;
    record FR_KERNEL_FUNCTION end FR_KERNEL_FUNCTION;
  end FunctionRestriction;

  uniontype Annotation
    record ANNOTATION
      list<ElementArg> elementArgs;
    end ANNOTATION;
  end Annotation;

  uniontype Comment
    record COMMENT
      Option<Annotation> annotation_;
      Option<String> comment;
    end COMMENT;
  end Comment;

  uniontype ExternalDecl
    record EXTERNALDECL
      Option<Ident> funcName;
      Option<String> lang;
      Option<ComponentRef> output_;
      list<Exp> args;
      Option<Annotation> annotation_;
    end EXTERNALDECL;
  end ExternalDecl;
end Absyn;

package Config
  function acceptMetaModelicaGrammar
    output Boolean outBoolean;
  end acceptMetaModelicaGrammar;
end Config;

package Dump
  function shouldParenthesize
    input Absyn.Exp inOperand;
    input Absyn.Exp inOperator;
    input Boolean inLhs;
    output Boolean outShouldParenthesize;
  end shouldParenthesize;

  uniontype DumpOptions
    record DUMPOPTIONS
      String fileName;
    end DUMPOPTIONS;
  end DumpOptions;

  constant DumpOptions defaultDumpOptions;

  function boolUnparseFileFromInfo
    input builtin.SourceInfo info;
    input DumpOptions options;
    output Boolean b;
  end boolUnparseFileFromInfo;

end Dump;

package System
  function trimWhitespace
    input String inString;
    output String outString;
  end trimWhitespace;
  function stringReplace
    input String str;
    input String source;
    input String target;
    output String res;
  end stringReplace;
  function escapedString
    input String unescapedString;
    input Boolean unescapeNewline;
    output String escapedString;
  end escapedString;
end System;

package Tpl
  function addSourceTemplateError
    input String inErrMsg;
    input builtin.SourceInfo inInfo;
  end addSourceTemplateError;

  function addTemplateError
    input String inErrMsg;
  end addTemplateError;

uniontype Text
  record MEM_TEXT
  end MEM_TEXT;
  record FILE_TEXT
  end FILE_TEXT;
end Text;

constant Text emptyTxt;

uniontype BlockTypeFileText
  record BT_FILE_TEXT
  end BT_FILE_TEXT;
end BlockTypeFileText;

uniontype StringToken
  record ST_NEW_LINE "Always outputs the new-line char." end ST_NEW_LINE;

  record ST_STRING "A string without new-lines in it."
    String value;
  end ST_STRING;

  record ST_LINE "A (non-empty) string with new-line at the end."
    String line;
  end ST_LINE;

  record ST_STRING_LIST "Every string in the list can have a new-line at its end (but does not have to)."
    list<String> strList;
    Boolean lastHasNewLine "True when the last string in the list has new-line at the end.";
  end ST_STRING_LIST;

  record ST_BLOCK
    Tokens tokens;
    BlockType blockType;
  end ST_BLOCK;
end StringToken;

uniontype BlockType
  record BT_TEXT  end BT_TEXT;

  record BT_INDENT
    Integer width;
  end BT_INDENT;

  record BT_ABS_INDENT
    Integer width;
  end BT_ABS_INDENT;

  record BT_REL_INDENT
    Integer offset;
  end BT_REL_INDENT;

  record BT_ANCHOR
    Integer offset;
  end BT_ANCHOR;

  record BT_ITER
  end BT_ITER;
end BlockType;

uniontype IterOptions
  record ITER_OPTIONS
  end ITER_OPTIONS;
end IterOptions;

end Tpl;

package Flags
  uniontype ConfigFlag end ConfigFlag;
  constant ConfigFlag MODELICA_OUTPUT;
  function getConfigBool
    input ConfigFlag inFlag;
    output Boolean outValue;
  end getConfigBool;
end Flags;

package MMToJuliaUtil
  uniontype Context
    record FUNCTION
      String retValsStr;
      String ty_str;
    end FUNCTION;
    record FUNCTION_RETURN_CONTEXT
      String retValsStr "Contains return values";
      String ty_str "String of the type we are currently operating on";
    end FUNCTION_RETURN_CONTEXT;
    record PACKAGE
    end PACKAGE;
    record UNIONTYPE
      String name;
    end UNIONTYPE;
    record INPUT_CONTEXT
    end INPUT_CONTEXT;
    record NO_CONTEXT
    end NO_CONTEXT;
    record MATCH_CONTEXT
      Tpl.Text asString;
    end MATCH_CONTEXT;
  end Context;
  constant Context packageContext;
  constant Context functionContext;
  constant Context noContext;
  constant Context inputContext;
  function makeUniontypeContext
    input String name;
    output Context context;
  end makeUniontypeContext;
  function makeFunctionContext
    input String returnValuesStr;
    output Context context;
  end makeFunctionContext;
  function makeFunctionReturnContext
    input String returnValuesStr;
    input String ty_str;
    output Context context;
  end makeFunctionReturnContext;
  function makeAsContext
    input Tpl.Text asString;
   output Context context;
  end makeAsContext;
  function filterOnDirection
    input list<Absyn.ElementItem> inputs;
    input Absyn.Direction direction;
    output list<Absyn.ElementItem> outputs;
  end filterOnDirection;
  function makeInputDirection
    output Absyn.Direction direction;
  end makeInputDirection;
  function makeOutputDirection
    output Absyn.Direction direction;
  end makeOutputDirection;
  function isFunctionContext
    input Context givenCTX;
    output Boolean isFuncCTX;
  end isFunctionContext;
  function explicitReturnInClassPart
    input list<Absyn.ClassPart> classParts;
    output Boolean existsImplicitReturn;
  end explicitReturnInClassPart;
  function algorithmItemsContainsReturn
    input list<Absyn.AlgorithmItem> contents;
    output Boolean existsReturn;
  end algorithmItemsContainsReturn;
  function elementSpecIsBIDIR
    input Absyn.ElementSpec spec;
    output Boolean isBidr;
  end elementSpecIsBIDIR;
  function elementSpecIsOUTPUT
    input Absyn.ElementSpec spec;
    output Boolean isOutput;
  end elementSpecIsOUTPUT;
  function elementSpecIsOUTPUT_OR_BIDIR
    input Absyn.ElementSpec spec;
    output Boolean isOutput;
  end elementSpecIsOUTPUT_OR_BIDIR;
end MMToJuliaUtil;

package AbsynUtil
function getTypeSpecFromElementItemOpt
  input Absyn.ElementItem inElementItem;
  output Option<Absyn.TypeSpec> outTypeSpec;
end getTypeSpecFromElementItemOpt;
function getComponentItemsFromElementItem
  input Absyn.ElementItem inElementItem;
  output list<Absyn.ComponentItem> componentItems;
end getComponentItemsFromElementItem;
function isClassdef
  input Absyn.Element inElement;
  output Boolean b;
end isClassdef;
function getElementItemsInClassPart
  input Absyn.ClassPart inClassPart;
  output list<Absyn.ElementItem> outElements;
end getElementItemsInClassPart;
end AbsynUtil;

package SCodeUtil
  function translateAbsyn2SCode
    input Absyn.Program inProgram;
    output SCode.Program outProgram;
  end translateAbsyn2SCode;
  function translateClassdefElements
    input list<Absyn.ClassPart> inAbsynClassPartLst;
    output list<SCode.Element> outElementLst;
  end translateClassdefElements;
end SCodeUtil;

package SCode
  type Program = list<SCode.Element>;
  type Ident = String;
uniontype Element
  record IMPORT
  end IMPORT;
  record EXTENDS
  end EXTENDS;
  record CLASS
    Ident   name;
    Restriction restriction;
    Partial partialPrefix;
    ClassDef classDef;
  end CLASS;
  record COMPONENT
  end COMPONENT;
  record DEFINEUNIT
  end DEFINEUNIT;
end Element;

uniontype ClassDef
  record PARTS
    list<Element> elementLst;
  end PARTS;
  record CLASS_EXTENDS
  end CLASS_EXTENDS;
  record DERIVED
  end DERIVED;
  record ENUMERATION
  end ENUMERATION;
  record OVERLOAD
  end OVERLOAD;
  record PDER
  end PDER;
end ClassDef;
uniontype Restriction
  record R_CLASS end R_CLASS;
  record R_OPTIMIZATION end R_OPTIMIZATION;
  record R_MODEL end R_MODEL;
  record R_RECORD
  end R_RECORD;
  record R_BLOCK end R_BLOCK;
  record R_CONNECTOR
  end R_CONNECTOR;
  record R_OPERATOR end R_OPERATOR;
  record R_TYPE end R_TYPE;
  record R_PACKAGE end R_PACKAGE;
  record R_FUNCTION
  end R_FUNCTION;
  record R_ENUMERATION end R_ENUMERATION;
  record R_PREDEFINED_INTEGER     "predefined IntegerType" end R_PREDEFINED_INTEGER;
  record R_PREDEFINED_REAL        "predefined RealType"    end R_PREDEFINED_REAL;
  record R_PREDEFINED_STRING      "predefined StringType"  end R_PREDEFINED_STRING;
  record R_PREDEFINED_BOOLEAN     "predefined BooleanType" end R_PREDEFINED_BOOLEAN;
  record R_PREDEFINED_ENUMERATION "predefined EnumType"    end R_PREDEFINED_ENUMERATION;
  record R_PREDEFINED_CLOCK       "predefined ClockType"   end R_PREDEFINED_CLOCK;
  record R_METARECORD "Metamodelica extension"
  end R_METARECORD;
  record R_UNIONTYPE
  end R_UNIONTYPE;
end Restriction;

uniontype Partial "the partial prefix"
  record PARTIAL     "a partial prefix"     end PARTIAL;
  record NOT_PARTIAL "a non partial prefix" end NOT_PARTIAL;
end Partial;

end SCode;

package SCodeDump
  uniontype SCodeDumpOptions
    record OPTIONS
      Boolean stripAlgorithmSections;
      Boolean stripProtectedImports;
      Boolean stripStringComments;
      Boolean stripExternalDecl;
      Boolean stripOutputBindings;
    end OPTIONS;
  end SCodeDumpOptions;
  constant SCodeDumpOptions defaultOptions;
  function filterElements
    input list<SCode.Element> element;
    input SCodeDumpOptions options;
    output list<SCode.Element> outElements;
  end filterElements;
end SCodeDump;

end AbsynToJuliaTV;