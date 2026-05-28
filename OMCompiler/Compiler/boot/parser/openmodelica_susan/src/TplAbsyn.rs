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

use crate::Tpl;
use crate::TplCodegen;
use openmodelica_util::AvlSetString;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/* Input AST */
pub type Ident = ArcStr;

pub type TypedIdents = Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>;

pub type EscOption = (ArcStr, Option<(Arc<ExpressionBase>, SourceInfo)>);

pub type StringToken = Arc<Tpl::StringToken>;

pub type Tokens = Arc<metamodelica::List<Arc<Tpl::StringToken>>>;

pub static dummySourceInfo: SourceInfo = SourceInfo { fileName: literal!("NoFileName.xxx"), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) };

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PathIdent {
    IDENT {
        ident: Ident,
    },
    PATH_IDENT {
        ident: Ident,
        path: Arc<PathIdent>,
    },
}
impl Default for PathIdent {
    fn default() -> Self {
        Self::IDENT {
            ident: Default::default(),
        }
    }
}
pub use self::PathIdent::{IDENT,PATH_IDENT};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeSignature {
    LIST_TYPE {
        ofType: Arc<TypeSignature>,
    },
    ARRAY_TYPE {
        ofType: Arc<TypeSignature>,
    },
    OPTION_TYPE {
        ofType: Arc<TypeSignature>,
    },
    TUPLE_TYPE {
        ofTypes: Arc<metamodelica::List<Arc<TypeSignature>>>,
    },
    /// key/path to a TypeInfo list from an AST definition
    NAMED_TYPE {
        name: Arc<PathIdent>,
    },
    STRING_TYPE,
    TEXT_TYPE,
    /// Used only for internal string constants.
    STRING_TOKEN_TYPE,
    INTEGER_TYPE,
    REAL_TYPE,
    BOOLEAN_TYPE,
    /// Errorneous resolving type. Only used during elaboration phase.
    UNRESOLVED_TYPE {
        reason: ArcStr,
    },
}
impl Default for TypeSignature {
    fn default() -> Self { Self::STRING_TYPE }
}
pub use self::TypeSignature::{LIST_TYPE,ARRAY_TYPE,OPTION_TYPE,TUPLE_TYPE,NAMED_TYPE,STRING_TYPE,TEXT_TYPE,STRING_TOKEN_TYPE,INTEGER_TYPE,REAL_TYPE,BOOLEAN_TYPE,UNRESOLVED_TYPE};

pub type Expression = (Arc<ExpressionBase>, SourceInfo);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExpressionBase {
    TEMPLATE {
        items: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>>,
        lquote: ArcStr,
        rquote: ArcStr,
    },
    STR_TOKEN {
        value: StringToken,
    },
    LITERAL {
        value: ArcStr,
        litType: Arc<TypeSignature>,
    },
    SOFT_NEW_LINE,
    BOUND_VALUE {
        boundPath: Arc<PathIdent>,
    },
    FUN_CALL {
        name: Arc<PathIdent>,
        args: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>>,
    },
    CONDITION {
        /// Is not or inequal
        isNot: bool,
        lhsExp: Expression,
        /// always NONE() for now; it is a residuum from the form 'if exp is PATTERN then ...'
        rhsValue: Option<Arc<MatchingExp>>,
        trueBranch: Expression,
        elseBranch: Option<(Arc<ExpressionBase>, SourceInfo)>,
    },
    MATCH {
        matchExp: Expression,
        cases: Arc<metamodelica::List<(Arc<MatchingExp>, (Arc<ExpressionBase>, SourceInfo))>>,
    },
    MAP {
        argExp: Expression,
        ofBinding: Arc<MatchingExp>,
        mapExp: Expression,
        hasIndexIdentOpt: Option<ArcStr>,
    },
    MAP_ARG_LIST {
        parts: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>>,
    },
    ESCAPED {
        exp: Expression,
        options: Arc<metamodelica::List<(ArcStr, Option<(Arc<ExpressionBase>, SourceInfo)>)>>,
    },
    /// Indented block.
    INDENTATION {
        width: i32,
        items: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>>,
    },
    LET {
        letExp: Expression,
        exp: Expression,
    },
    TEXT_CREATE {
        name: Ident,
        exp: Expression,
    },
    TEXT_ADD {
        name: Ident,
        exp: Expression,
    },
    NORET_CALL {
        name: Arc<PathIdent>,
        args: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>>,
    },
    /// Parse error expression used when parser error occured.
    ERROR_EXP,
}
pub use self::ExpressionBase::{TEMPLATE,STR_TOKEN,LITERAL,SOFT_NEW_LINE,BOUND_VALUE,FUN_CALL,CONDITION,MATCH,MAP,MAP_ARG_LIST,ESCAPED,INDENTATION,LET,TEXT_CREATE,TEXT_ADD,NORET_CALL,ERROR_EXP};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MatchingExp {
    BIND_AS_MATCH {
        bindIdent: Ident,
        matchingExp: Arc<MatchingExp>,
    },
    BIND_MATCH {
        bindIdent: Ident,
    },
    RECORD_MATCH {
        tagName: Arc<PathIdent>,
        fieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>,
    },
    SOME_MATCH {
        value: Arc<MatchingExp>,
    },
    NONE_MATCH,
    TUPLE_MATCH {
        tupleArgs: Arc<metamodelica::List<Arc<MatchingExp>>>,
    },
    LIST_MATCH {
        listElts: Arc<metamodelica::List<Arc<MatchingExp>>>,
    },
    LIST_CONS_MATCH {
        head: Arc<MatchingExp>,
        rest: Arc<MatchingExp>,
    },
    STRING_MATCH {
        value: ArcStr,
    },
    LITERAL_MATCH {
        value: ArcStr,
        /// only INTEGER_TYPE, REAL_TYPE or BOOLEAN_TYPE
        litType: Arc<TypeSignature>,
    },
    REST_MATCH,
}
pub use self::MatchingExp::{BIND_AS_MATCH,BIND_MATCH,RECORD_MATCH,SOME_MATCH,NONE_MATCH,TUPLE_MATCH,LIST_MATCH,LIST_CONS_MATCH,STRING_MATCH,LITERAL_MATCH,REST_MATCH};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeInfo {
    TI_UNION_TYPE {
        recTags: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>)>>,
    },
    TI_RECORD_TYPE {
        fields: TypedIdents,
    },
    TI_ALIAS_TYPE {
        aliasType: Arc<TypeSignature>,
    },
    /// Imported AST/builtin functions.
    TI_FUN_TYPE {
        inArgs: TypedIdents,
        outArgs: TypedIdents,
        tyVars: Arc<metamodelica::List<ArcStr>>,
    },
    /// Imported AST constants.
    TI_CONST_TYPE {
        constType: Arc<TypeSignature>,
    },
}
impl Default for TypeInfo {
    fn default() -> Self {
        Self::TI_UNION_TYPE {
            recTags: Default::default(),
        }
    }
}
pub use self::TypeInfo::{TI_UNION_TYPE,TI_RECORD_TYPE,TI_ALIAS_TYPE,TI_FUN_TYPE,TI_CONST_TYPE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ASTDef {
    pub importPackage: Arc<PathIdent>,
    pub isDefault: bool,
    pub types: Arc<metamodelica::List<(ArcStr, TypeInfo)>>,
}

impl Default for ASTDef {
    fn default() -> Self {
        Self {
            importPackage: Default::default(),
            isDefault: Default::default(),
            types: Default::default(),
        }
    }
}

pub type AST_DEF = ASTDef;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplPackage {
    pub name: Arc<PathIdent>,
    pub astDefs: Arc<metamodelica::List<ASTDef>>,
    pub templateDefs: Arc<metamodelica::List<(ArcStr, TemplateDef)>>,
    pub annotationFooter: ArcStr,
}

impl Default for TemplPackage {
    fn default() -> Self {
        Self {
            name: Default::default(),
            astDefs: Default::default(),
            templateDefs: Default::default(),
            annotationFooter: Default::default(),
        }
    }
}

pub type TEMPL_PACKAGE = TemplPackage;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TemplateDef {
    STR_TOKEN_DEF {
        value: StringToken,
    },
    LITERAL_DEF {
        value: ArcStr,
        litType: Arc<TypeSignature>,
    },
    TEMPLATE_DEF {
        args: TypedIdents,
        lesc: ArcStr,
        resc: ArcStr,
        exp: Expression,
    },
}
impl Default for TemplateDef {
    fn default() -> Self {
        Self::STR_TOKEN_DEF {
            value: Default::default(),
        }
    }
}
pub use self::TemplateDef::{STR_TOKEN_DEF,LITERAL_DEF,TEMPLATE_DEF};

/* Output AST */
//type MMPublic = Boolean;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MMPackage {
    pub name: Arc<PathIdent>,
    pub mmDeclarations: Arc<metamodelica::List<MMDeclaration>>,
    pub annotationFooter: ArcStr,
}

impl Default for MMPackage {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mmDeclarations: Default::default(),
            annotationFooter: Default::default(),
        }
    }
}

pub type MM_PACKAGE = MMPackage;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MMDeclaration {
    MM_IMPORT {
        isPublic: bool,
        packageName: Arc<PathIdent>,
    },
    MM_STR_TOKEN_DECL {
        isPublic: bool,
        name: Ident,
        value: StringToken,
    },
    MM_LITERAL_DECL {
        isPublic: bool,
        name: Ident,
        value: ArcStr,
        litType: Arc<TypeSignature>,
    },
    MM_FUN {
        isPublic: bool,
        name: Ident,
        inArgs: TypedIdents,
        outArgs: TypedIdents,
        locals: TypedIdents,
        statements: Arc<metamodelica::List<Arc<MMExp>>>,
        /// internal use only - a type of elaboration of the funtion.
        genInfoOpt: GenInfo,
    },
}
impl Default for MMDeclaration {
    fn default() -> Self {
        Self::MM_IMPORT {
            isPublic: Default::default(),
            packageName: Default::default(),
        }
    }
}
pub use self::MMDeclaration::{MM_IMPORT,MM_STR_TOKEN_DECL,MM_LITERAL_DECL,MM_FUN};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MMExp {
    MM_ASSIGN {
        lhsArgs: Arc<metamodelica::List<ArcStr>>,
        rhs: Arc<MMExp>,
    },
    MM_FN_CALL {
        fnName: Arc<PathIdent>,
        args: Arc<metamodelica::List<Arc<MMExp>>>,
    },
    MM_IDENT {
        ident: Arc<PathIdent>,
    },
    /// constructor of type StringToken
    MM_STR_TOKEN {
        value: StringToken,
    },
    /// to pass a string constant as parameter of type String
    MM_STRING {
        value: ArcStr,
    },
    /// to pass a literal constant as parameter of type Integer, Real or Boolean
    MM_LITERAL {
        value: ArcStr,
    },
    MM_MATCH {
        matchCases: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>)>>,
    },
    MM_FOR_LOOP {
        idxName: Ident,
        arrName: Ident,
        eltName: Ident,
        statements: Arc<metamodelica::List<Arc<MMExp>>>,
    },
}
pub use self::MMExp::{MM_ASSIGN,MM_FN_CALL,MM_IDENT,MM_STR_TOKEN,MM_STRING,MM_LITERAL,MM_MATCH,MM_FOR_LOOP};

pub type MMMatchCase = (Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>);

pub const imlicitTxt: &'static str = "txt";

pub const inPrefix: &'static str = "in_";

pub const outPrefix: &'static str = "out_";

//constant Ident imlicitInTxt = "intxt"; //not used ... there can be the same names for in/ou values
//constant Ident imlicitOutTxt = "outtxt";
pub const funArgNamePrefix: &'static str = "a_";

pub const extArgNamePrefix: &'static str = "e_";

pub const letValueNamePrefix: &'static str = "l_";

pub const indexNamePrefix: &'static str = "x_";

pub const caseBindingNamePrefix: &'static str = "i_";

pub const returnTempVarNamePrefix: &'static str = "ret_";

pub const constantNamePrefix: &'static str = "c_";

pub const textTempVarNamePrefix: &'static str = "txt_";

pub const textToStringNamePrefix: &'static str = "str_";

pub const matchFunPrefix: &'static str = "fun_";

pub const listMapFunPrefix: &'static str = "lm_";

pub const arrayMapFunPrefix: &'static str = "am_";

pub const scalarMapFunPrefix: &'static str = "smf_";

//constant Ident implicitTxtInArgName = "inTxt";
pub const matchDefaultArgName: &'static str = "mArg";

pub const impossibleIdent: &'static str = "*none*";

pub static imlicitTxtArg: std::sync::LazyLock<(ArcStr, Arc<TypeSignature>)> = std::sync::LazyLock::new(|| { (arcstr::literal!(imlicitTxt), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE)) });

//constant tuple<Ident,TypeSignature> imlicitTxtInputArg = (implicitTxtInArgName, TEXT_TYPE());
/* internal types */
pub static imlicitTxtMExp: std::sync::LazyLock<Arc<MatchingExp>> = std::sync::LazyLock::new(|| { Arc::new(MatchingExp::BIND_MATCH { bindIdent: (arcstr::literal!(imlicitTxt)).clone() }) });

pub static emptyExpression: std::sync::LazyLock<(Arc<ExpressionBase>, SourceInfo)> = std::sync::LazyLock::new(|| { (Arc::new(ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() }) }), dummySourceInfo.clone()) });

pub const emptyTxt: &'static str = "Tpl.emptyTxt";

pub const errorIdent: &'static str = "!error!";

pub static defaultIterOptions: std::sync::LazyLock<Arc<Tpl::IterOptions>> = std::sync::LazyLock::new(|| { Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: None, alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }) });

//only achievable by the 'from' clause
pub const indexOffsetOptionId: &'static str = "$indexOffset";

pub const emptyOptionId: &'static str = "empty";

pub const separatorOptionId: &'static str = "separator";

pub const alignNumOptionId: &'static str = "align";

pub const alignNumOffsetOptionId: &'static str = "alignOffset";

pub const alignSeparatorOptionId: &'static str = "alignSeparator";

pub const wrapWidthOptionId: &'static str = "wrap";

pub const wrapSeparatorOptionId: &'static str = "wrapSeparator";

pub const indentOptionId: &'static str = "indent";

pub const absIndentOptionId: &'static str = "absIndent";

pub const relIndentOptionId: &'static str = "relIndent";

pub const anchorOptionId: &'static str = "anchor";

//constant defaultMMOptions
pub static defaultEscOptions: std::sync::LazyLock<Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>> = std::sync::LazyLock::new(|| { list![(arcstr::literal!(indexOffsetOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(emptyOptionId), (Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::IDENT { ident: (literal!("SOME")).clone() }), args: list![Arc::new(MMExp::MM_STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() }) })] }), Arc::new(TypeSignature::OPTION_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE) }))), (arcstr::literal!(separatorOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("NONE()")).clone() }), Arc::new(TypeSignature::OPTION_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE) }))), (arcstr::literal!(alignNumOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("10")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(alignNumOffsetOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(alignSeparatorOptionId), (Arc::new(MMExp::MM_STR_TOKEN { value: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE))), (arcstr::literal!(wrapWidthOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("100")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(wrapSeparatorOptionId), (Arc::new(MMExp::MM_STR_TOKEN { value: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE))), (arcstr::literal!(indentOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(absIndentOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(relIndentOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(anchorOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE)))] });

pub static nonSpecifiedIterOptions: std::sync::LazyLock<Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>> = std::sync::LazyLock::new(|| { list![(arcstr::literal!(indexOffsetOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(emptyOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("NONE()")).clone() }), Arc::new(TypeSignature::OPTION_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE) }))), (arcstr::literal!(separatorOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("NONE()")).clone() }), Arc::new(TypeSignature::OPTION_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE) }))), (arcstr::literal!(alignNumOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(alignNumOffsetOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(alignSeparatorOptionId), (Arc::new(MMExp::MM_STR_TOKEN { value: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE))), (arcstr::literal!(wrapWidthOptionId), (Arc::new(MMExp::MM_LITERAL { value: (literal!("0")).clone() }), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE))), (arcstr::literal!(wrapSeparatorOptionId), (Arc::new(MMExp::MM_STR_TOKEN { value: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE)))] });

pub type MMEscOption = (ArcStr, (Arc<MMExp>, Arc<TypeSignature>));

pub type ScopeEnv = Arc<metamodelica::List<Scope>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    FUN_SCOPE {
        args: TypedIdents,
        /// local encoded args; used to elaborate the actual args of closures
        localArgs: TypedIdents,
    },
    CASE_SCOPE {
        mExp: Arc<MatchingExp>,
        mType: Arc<TypeSignature>,
        /// source name -> local declaration name table
        localNames: Arc<metamodelica::List<(ArcStr, ArcStr)>>,
        /// accumulated locals used by the cases in this match elaborated level
        accLocals: TypedIdents,
        /// local args from the upper scope - all of them are from their upper FUN_SCOPE()
        extArgs: TypedIdents,
        /// local name of the match argument
        matchArgName: Ident,
        /// true for 'match' or 'map', false for 'if' elaborated cases; desides if the implicit record fields' lookup can continue upwards the scope stack.
        hasImplicitScope: bool,
    },
    LET_SCOPE {
        /// original ident
        ident: Ident,
        idType: Arc<TypeSignature>,
        /// encoded ident with prefix and suffix unique for the local scope
        freshIdent: Ident,
        /// true when found by resolveBoundPath()
        isUsed: bool,
    },
    /// forbidden access - scope of a text add ident; to prevent recursive usage of texts;
    ///     or scope of an elaborated let binding; to force a fresh local ident to be created when the same name is re-bound inside the let expression.
    RECURSIVE_SCOPE {
        recIdent: Ident,
        /// local name
        freshIdent: Ident,
    },
}
pub use self::Scope::{FUN_SCOPE,CASE_SCOPE,LET_SCOPE,RECURSIVE_SCOPE};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapContext {
    pub ofBinding: Arc<MatchingExp>,
    pub mapExp: Expression,
    pub iterMMExpOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>,
    /// used index variable
    pub hasIndexIdentOpt: Option<ArcStr>,
    /// Whether PushIter/NextIter/PopIter is necessary.
    pub useIter: bool,
}

pub type MAP_CONTEXT = MapContext;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GenInfo {
    GI_TEMPL_FUN,
    GI_MATCH_FUN,
    GI_MAP_FUN {
        mapType: Arc<TypeSignature>,
        mapContext: MapContext,
    },
}
pub use self::GenInfo::{GI_TEMPL_FUN,GI_MATCH_FUN,GI_MAP_FUN};

// *** functions ***
pub fn transformAST(mut inTplPackage: TemplPackage) -> Result<MMPackage> {
    let mut outMMPackage: MMPackage = <MMPackage as ::std::default::Default>::default();
    outMMPackage = (match inTplPackage.clone() {
        _ => {
            let mut name: Arc<PathIdent>;
            let mut templateDefs: Arc<metamodelica::List<(ArcStr, TemplateDef)>> = metamodelica::nil();
            let mut mmDeclarations: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
            let mut tp: TemplPackage = <TemplPackage as ::std::default::Default>::default();
            let mut astDefs: Arc<metamodelica::List<ASTDef>> = metamodelica::nil();
            let mut annotationFooter: ArcStr = arcstr::literal!("");
            tp = fullyQualifyTemplatePackage(inTplPackage.clone())?;
            let TemplPackage { name: __pa0, astDefs: __pa1, templateDefs: __pa2, annotationFooter: __pa3 } = (tp.clone()) else { bail!("pattern mismatch") };
            name = __pa0.clone();
            astDefs = __pa1.clone();
            templateDefs = __pa2.clone();
            annotationFooter = __pa3.clone();
            mmDeclarations = importDeclarations(astDefs.clone())?;
            mmDeclarations = transformTemplateDefs(templateDefs.clone(), tp.clone(), mmDeclarations.clone())?;
            mmDeclarations = mmDeclarations.clone().reverse();
            MMPackage { name: name.clone(), mmDeclarations: mmDeclarations.clone(), annotationFooter: (annotationFooter.clone()).clone() }
        },
    });
    Ok(outMMPackage)
}

pub fn fullyQualifyTemplatePackage(mut inTplPackage: TemplPackage) -> Result<TemplPackage> {
    let mut outTplPackage: TemplPackage = <TemplPackage as ::std::default::Default>::default();
    outTplPackage = (match inTplPackage.clone() {
        TemplPackage { name: mut name, astDefs: mut astDefs, templateDefs: mut templateDefs, annotationFooter: mut ann } => {
            let mut astDefs = astDefs.clone();
            let mut templateDefs = templateDefs.clone();
            astDefs = fullyQualifyASTDefs(astDefs.clone())?;
            templateDefs = listMap1Tuple22(templateDefs.clone(), (std::sync::Arc::new(fullyQualifyTemplateDef) as std::sync::Arc<dyn ::std::ops::Fn(TemplateDef, Arc<metamodelica::List<ASTDef>>) -> Result<TemplateDef> + 'static>), astDefs.clone())?;
            TemplPackage { name: name.clone(), astDefs: astDefs.clone(), templateDefs: templateDefs.clone(), annotationFooter: (ann.clone()).clone() }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outTplPackage)
}

pub fn importDeclarations(mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<Arc<metamodelica::List<MMDeclaration>>> {
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    let mut importPackage: Arc<PathIdent>;
    let mut isDefault: bool = false;
    for mut astDef in &*inASTDefs.clone() {
        let mut astDef = astDef.clone();
        let ASTDef { isDefault: __pa0, importPackage: __pa1, .. } = (astDef.clone()) else { bail!("pattern mismatch") };
        isDefault = __pa0.clone();
        importPackage = __pa1.clone();
        outMMDecls = cons(MMDeclaration::MM_IMPORT { isPublic: isDefault.clone(), packageName: importPackage.clone() }, outMMDecls.clone());
    }
    Ok(outMMDecls)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn transformTemplateDefs(mut inTemplateDefsRest: Arc<metamodelica::List<(ArcStr, TemplateDef)>>, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<Arc<metamodelica::List<MMDeclaration>>> {
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    outMMDecls = (::match_deref::match_deref! { match &((inTemplateDefsRest.clone(), inTplPackage.clone(), inAccMMDecls.clone())) {
        (Deref @ metamodelica::List::Nil, _, accMMDecls) => {
            accMMDecls.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (tplname, TemplateDef::STR_TOKEN_DEF { value: stvalue }), tail: restTDefs }, tplPackage, accMMDecls) => {
            let mut mmDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
            let mut tplname = (*tplname).clone();
            tplname = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(constantNamePrefix)); __mm_s.push_str(&*tplname.clone()); ArcStr::from(__mm_s) }).clone();
            mmDecls = transformTemplateDefs(restTDefs.clone(), tplPackage.clone(), cons(MMDeclaration::MM_STR_TOKEN_DECL { isPublic: true, name: (tplname.clone()).clone(), value: stvalue.clone() }, accMMDecls.clone()))?;
            mmDecls.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (tplname, TemplateDef::LITERAL_DEF { litType, value: svalue }), tail: restTDefs }, tplPackage, accMMDecls) => {
            let mut mmDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
            let mut tplname = (*tplname).clone();
            tplname = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(constantNamePrefix)); __mm_s.push_str(&*tplname.clone()); ArcStr::from(__mm_s) }).clone();
            mmDecls = transformTemplateDefs(restTDefs.clone(), tplPackage.clone(), cons(MMDeclaration::MM_LITERAL_DECL { isPublic: true, name: (tplname.clone()).clone(), value: (svalue.clone()).clone(), litType: litType.clone() }, accMMDecls.clone()))?;
            mmDecls.clone()
        },
        (Deref @ metamodelica::List::Cons { head: (tplname, TemplateDef::TEMPLATE_DEF { exp: texp, args: targs, .. }), tail: restTDefs }, tplPackage, accMMDecls) => {
            let mut encArgs: TypedIdents = metamodelica::nil();
            let mut locals: TypedIdents = metamodelica::nil();
            let mut iargs: TypedIdents = metamodelica::nil();
            let mut oargs: TypedIdents = metamodelica::nil();
            let mut stmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
            let mut mmFun: MMDeclaration;
            let mut accMMDecls = (*accMMDecls).clone();
            encArgs = List::map1(targs.clone(), (std::sync::Arc::new(encodeTypedIdent) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>), ArcStr) -> Result<(ArcStr, Arc<TypeSignature>)> + 'static>), (arcstr::literal!(funArgNamePrefix)).clone());
            (stmts, locals, _, accMMDecls, _) = statementsFromExp(texp.clone(), metamodelica::nil(), metamodelica::nil(), (arcstr::literal!(imlicitTxt)).clone(), (arcstr::literal!(imlicitTxt)).clone(), metamodelica::nil(), list![Scope::FUN_SCOPE { args: targs.clone(), localArgs: encArgs.clone() }], tplPackage.clone(), accMMDecls.clone())?;
            iargs = cons(imlicitTxtArg.clone(), encArgs.clone());
            oargs = List::filterOnTrue(iargs.clone(), (std::sync::Arc::new(fnptr!(isText, (ArcStr, Arc<TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>)) -> Result<bool> + 'static>));
            stmts = stmts.clone().reverse();
            stmts = addOutPrefixes(stmts.clone(), oargs.clone(), metamodelica::nil())?;
            (stmts, locals, accMMDecls) = inlineLastFunIfSingleCall(iargs.clone(), oargs.clone(), stmts.clone(), locals.clone(), accMMDecls.clone())?;
            mmFun = MMDeclaration::MM_FUN { isPublic: true, name: (tplname.clone()).clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: locals.clone(), statements: stmts.clone(), genInfoOpt: crate::TplAbsyn::GenInfo::GI_TEMPL_FUN };
            transformTemplateDefs(restTDefs.clone(), tplPackage.clone(), cons(mmFun.clone(), accMMDecls.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMMDecls)
}

pub fn inlineLastFunIfSingleCall(mut inInArgs: TypedIdents, mut inOutArgs: TypedIdents, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inLocals: TypedIdents, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents, Arc<metamodelica::List<MMDeclaration>>)> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    (outStmts, outLocals, outMMDecls) = 'mc: {
        let __mc_input = (inInArgs.clone(), inOutArgs.clone(), inStmts.clone(), inLocals.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (iargs, oargs, Deref @ metamodelica::List::Cons { head: Deref @ MMExp::MM_ASSIGN { rhs: Deref @ MMExp::MM_FN_CALL { fnName: Deref @ PathIdent::IDENT { ident: fidCalled }, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: MMDeclaration::MM_FUN { isPublic: _, name: fidLast, inArgs: iargsL, outArgs: oargsL, locals, statements: stmts, genInfoOpt: genInfo }, tail: accMMDecls }) => {
                    let true = (stringEq((fidCalled.clone()).clone(), (fidLast.clone()).clone())) else { bail!("pattern mismatch") };
                    if '__try0: {
                        let GenInfo::GI_TEMPL_FUN { .. } = (genInfo.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let true = (iargs.clone() == iargsL.clone()) else { bail!("pattern mismatch") };
                    let true = (oargs.clone() == oargsL.clone()) else { bail!("pattern mismatch") };
                    Ok((stmts.clone(), locals.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, stmts, locals, accMMDecls) => {
                    Ok((stmts.clone(), locals.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStmts, outLocals, outMMDecls))
}

//prepend "i" in front of the ident to obey the MM rule that no identifier can start with "_"
pub fn encodeIdent(mut inIdent: Ident, mut prefix: Ident) -> Result<Ident> {
    let mut outIdent: Ident = arcstr::literal!("");
    outIdent = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*encodeIdentNoPrefix((inIdent.clone()).clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(outIdent)
}

//every ident to be encoded as ".ident"
//where "." is encoded as "_" or "_0" in the case it is followed with "_" (idents starting with _)
fn encodeIdentNoPrefix(mut inIdent: Ident) -> Result<Ident> {
    let mut outIdent: Ident = arcstr::literal!("");
    outIdent = ('mc: {
        let __mc_input = inIdent.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut ident = __mc_input.clone() else { bail!("nomatch") };
            let true = (((ident.clone()).clone().len() as i32) > 0 && stringGetStringChar((ident.clone()).clone(), 1)? == literal!("_")) else { bail!("pattern mismatch") };
            ident = (System::stringReplace((ident.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            ident = (System::stringReplace((ident.clone()).clone(), (literal!("._")).clone(), (literal!("_0")).clone())?).clone();
            ident = (System::stringReplace((ident.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("0")); __mm_s.push_str(&*ident.clone()); ArcStr::from(__mm_s) }).clone();
            Ok(ident.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut ident = __mc_input.clone() else { bail!("nomatch") };
            ident = (System::stringReplace((ident.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            ident = (System::stringReplace((ident.clone()).clone(), (literal!("._")).clone(), (literal!("_0")).clone())?).clone();
            ident = (System::stringReplace((ident.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            Ok(ident.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!encodeIdentNoPrefix failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outIdent)
}

pub fn encodePathIdent(mut inPath: Arc<PathIdent>, mut prefix: Ident) -> Result<Ident> {
    let mut outEncIdent: Ident = arcstr::literal!("");
    outEncIdent = (encodeIdent((pathIdentString(inPath.clone())?).clone(), (prefix.clone()).clone())?).clone();
    Ok(outEncIdent)
}

pub fn encodeTypedIdent(mut inTypedIdent: (ArcStr, Arc<TypeSignature>), mut prefix: Ident) -> Result<(ArcStr, Arc<TypeSignature>)> {
    let mut outTypedIdent: (ArcStr, Arc<TypeSignature>);
    outTypedIdent = 'mc: {
        let __mc_input = (inTypedIdent.clone(), prefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((ident, ts), _) => {
                    let mut ident = (*ident).clone();
                    ident = (encodeIdent((ident.clone()).clone(), (prefix.clone()).clone())?).clone();
                    Ok((ident.clone(), ts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!encodeTypedIdent failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTypedIdent)
}

pub fn addOutPrefixes(mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inTextArgs: TypedIdents, mut inTranslatedTextArgs: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<Arc<metamodelica::List<Arc<MMExp>>>> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    outStmts = 'mc: {
        let __mc_input = (inStmts.clone(), inTextArgs.clone(), inTranslatedTextArgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, txtargs, trIdents) => {
                    let mut stmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    stmts = addOutTextAssigns(txtargs.clone(), trIdents.clone());
                    Ok(stmts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ MMExp::MM_ASSIGN { rhs, lhsArgs: largs }, tail: stmts }, txtargs, trIdents) => {
                    let mut rhs = (*rhs).clone();
                    let mut largs = (*largs).clone();
                    let mut stmts = (*stmts).clone();
                    let mut trIdents = (*trIdents).clone();
                    rhs = addOutPrefixesRhs(rhs.clone(), trIdents.clone())?;
                    (largs, trIdents) = addOutPrefixesLhs(largs.clone(), txtargs.clone(), trIdents.clone())?;
                    stmts = addOutPrefixes(stmts.clone(), txtargs.clone(), trIdents.clone())?;
                    Ok(cons(Arc::new(MMExp::MM_ASSIGN { lhsArgs: largs.clone(), rhs: rhs.clone() }), stmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: stmt, tail: stmts }, txtargs, trIdents) => {
                    let mut stmts = (*stmts).clone();
                    stmts = addOutPrefixes(stmts.clone(), txtargs.clone(), trIdents.clone())?;
                    Ok(cons(stmt.clone(), stmts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!addOutPrefixes failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStmts)
}

pub fn addOutPrefixesRhs(mut inStmt: Arc<MMExp>, mut inTranslatedTextArgs: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<Arc<MMExp>> {
    let mut outStmt: Arc<MMExp>;
    outStmt = 'mc: {
        let __mc_input = (inStmt.clone(), inTranslatedTextArgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MMExp::MM_IDENT { ident: Deref @ PathIdent::IDENT { ident } }, trIdents) => {
                    let mut outident: Ident = arcstr::literal!("");
                    outident = (lookupTupleList(trIdents.clone(), (ident.clone()).clone())?).clone();
                    Ok(Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (outident.clone()).clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MMExp::MM_FN_CALL { args: fargs, fnName: fpath }, trIdents) => {
                    let mut fargs = (*fargs).clone();
                    fargs = List::map1(fargs.clone(), (std::sync::Arc::new(addOutPrefixesRhs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<MMExp>, Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<Arc<MMExp>> + 'static>), trIdents.clone());
                    Ok(Arc::new(MMExp::MM_FN_CALL { fnName: fpath.clone(), args: fargs.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inStmt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStmt)
}

pub fn addOutPrefixesLhs(mut inLhsArgs: Arc<metamodelica::List<ArcStr>>, mut inTextArgs: TypedIdents, mut inTranslatedTextArgs: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<(ArcStr, ArcStr)>>)> {
    let mut outLhsArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outTranslatedTextArgs: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    (outLhsArgs, outTranslatedTextArgs) = 'mc: {
        let __mc_input = (inLhsArgs.clone(), inTextArgs.clone(), inTranslatedTextArgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, trIdents) => {
                    Ok((metamodelica::nil(), trIdents.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ident, tail: largs }, txtargs, trIdents) => {
                    let mut outident: ArcStr = arcstr::literal!("");
                    let mut largs = (*largs).clone();
                    let mut trIdents = (*trIdents).clone();
                    let _ = lookupTupleList(txtargs.clone(), (ident.clone()).clone())?;
                    outident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(outPrefix)); __mm_s.push_str(&*ident.clone()); ArcStr::from(__mm_s) }).clone();
                    trIdents = updateTupleList(trIdents.clone(), (ident.clone(), outident.clone()))?;
                    (largs, trIdents) = addOutPrefixesLhs(largs.clone(), txtargs.clone(), trIdents.clone())?;
                    Ok((cons((outident.clone()).clone(), largs.clone()), trIdents.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: ident, tail: largs }, txtargs, trIdents) => {
                    let mut largs = (*largs).clone();
                    let mut trIdents = (*trIdents).clone();
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(txtargs.clone(), (ident.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (largs, trIdents) = addOutPrefixesLhs(largs.clone(), txtargs.clone(), trIdents.clone())?;
                    Ok((cons((ident.clone()).clone(), largs.clone()), trIdents.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!addOutPrefixesLhs failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outLhsArgs, outTranslatedTextArgs))
}

pub fn addOutTextAssigns(mut inTextArgs: TypedIdents, mut inTranslatedTextArgs: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Arc<metamodelica::List<Arc<MMExp>>> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outident: ArcStr = arcstr::literal!("");
    let mut id: (ArcStr, Arc<TypeSignature>);
    let mut ident: Ident = arcstr::literal!("");
    for mut id in &*inTextArgs.clone() {
        let mut id = id.clone();
        (ident, _) = id.clone();
        if '__try0: {
            let _ = unwrap_break_err!(lookupTupleList(inTranslatedTextArgs.clone(), (ident.clone()).clone()), '__try0);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            outident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(outPrefix)); __mm_s.push_str(&*ident.clone()); ArcStr::from(__mm_s) }).clone();
            outStmts = cons(Arc::new(MMExp::MM_ASSIGN { lhsArgs: list![(outident.clone()).clone()], rhs: Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (ident.clone()).clone() }) }) }), outStmts.clone());
        }
    }
    outStmts = outStmts.clone().reverse();
    outStmts
}

pub fn isAssignedIdent(mut inStatementList: Arc<metamodelica::List<Arc<MMExp>>>, mut inIdent: Ident) -> Result<bool> {
    let mut outIsAssigned: bool = false;
    let mut largs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut st in &*inStatementList.clone() {
        let mut st = st.clone();
        let __pa0 = ::match_deref::match_deref! { match &(st.clone()) {
            Deref @ MMExp::MM_ASSIGN { lhsArgs: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        largs = __pa0.clone();
        if listMember((inIdent.clone()).clone(), largs.clone()) {
            outIsAssigned = true;
            return Ok(outIsAssigned);
        }
    }
    outIsAssigned = false;
    Ok(outIsAssigned)
}

pub fn statementsFromExp(mut inExp: Expression, mut inMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inInText: Ident, mut inOutText: Ident, mut inLocals: TypedIdents, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>, Ident)> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    let mut outInText: Ident = arcstr::literal!("");
    (outStmts, outLocals, outScopeEnv, outMMDecls, outInText) = 'mc: {
        let __mc_input = (inExp.clone(), inMMEscOptions.clone(), inStmts.clone(), inInText.clone(), inOutText.clone(), inLocals.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::TEMPLATE { items: explst, .. }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExpList(explst.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LITERAL { value: litvalue, .. }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    warnIfSomeOptions(mmopts.clone())?;
                    stmt = tplStatement((literal!("writeTok")).clone(), list![Arc::new(MMExp::MM_STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (litvalue.clone()).clone() }) })], (intxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::SOFT_NEW_LINE { .. }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    warnIfSomeOptions(mmopts.clone())?;
                    stmt = tplStatement((literal!("softNewLine")).clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING { value: Deref @ "" } }, _), mmopts, stmts, intxt, _, locals, scEnv, _, accMMDecls) => {
                    warnIfSomeOptions(mmopts.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::STR_TOKEN { value: st }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    warnIfSomeOptions(mmopts.clone())?;
                    stmt = tplStatement((literal!("writeTok")).clone(), list![Arc::new(MMExp::MM_STR_TOKEN { value: st.clone() })], (intxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::BOUND_VALUE { boundPath: path }, sinfo), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut mmexp: Arc<MMExp>;
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut exptype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n BOUND_VALUE resolving boundPath = ")); __mm_s.push_str(&*pathIdentString(path.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (mmexp, idtype, scEnv) = resolveBoundPath(path.clone(), scEnv.clone(), tplPackage.clone())?;
                    checkResolvedType(path.clone(), idtype.clone(), (literal!("bound value")).clone(), sinfo.clone())?;
                    exptype = deAliasedType(idtype.clone(), astDefs.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n BOUND_VALUE resolved mmexp = ")); __mm_s.push_str(&*mmExpString(mmexp.clone())?); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*typeSignatureString(idtype.clone())?); __mm_s.push_str(&*literal!(" (dealiased: ")); __mm_s.push_str(&*typeSignatureString(exptype.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (stmts, locals, scEnv, accMMDecls, intxt) = addWriteCallFromMMExp(true, mmexp.clone(), exptype.clone(), sinfo.clone(), mmopts.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::FUN_CALL { args: explst, name: fname }, sinfo), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    let mut mmexp: Arc<MMExp>;
                    let mut tyVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut rettype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut hasretval: bool = false;
                    let mut fname = (*fname).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n FUN_CALL fname = ")); __mm_s.push_str(&*pathIdentString(fname.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (fname, iargs, oargs, tyVars) = getFunSignature(fname.clone(), sinfo.clone(), tplPackage.clone())?;
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(explst.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!(" FUN_CALL argList stmts generation passed\n")).clone())?;
                    }
                    (hasretval, stmt, mmexp, rettype, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), tyVars.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), sinfo.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" FUN_CALL stmt =\n")); __mm_s.push_str(&*stmtsString(list![stmt.clone()])?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    rettype = deAliasedType(rettype.clone(), astDefs.clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = addWriteCallFromMMExp(hasretval.clone(), mmexp.clone(), rettype.clone(), sinfo.clone(), mmopts.clone(), cons(stmt.clone(), stmts.clone()), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::MATCH { cases: mcases, matchExp: exp }, sinfo), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    let mut fname: Arc<PathIdent>;
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut argval: (Arc<MMExp>, Arc<TypeSignature>, SourceInfo);
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut exp = (*exp).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    (argval, stmts, locals, scEnv, accMMDecls) = statementsFromArg(exp.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (argval, exp, stmts, locals) = adaptTextToString(argval.clone(), exp.clone(), stmts.clone(), locals.clone(), tplPackage.clone())?;
                    (argvals, fname, iargs, oargs, scEnv, accMMDecls) = makeMatchFun(argval.clone(), mcases.clone(), exp.clone(), true, scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (_, stmt, _, _, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), sinfo.clone())?;
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::CONDITION { elseBranch: ebranch, trueBranch: tbranch, rhsValue: rhsval, lhsExp: exp, isNot: isnot }, sinfo), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    let mut fname: Arc<PathIdent>;
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut exptype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut argval: (Arc<MMExp>, Arc<TypeSignature>, SourceInfo);
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut mcases: Arc<metamodelica::List<(Arc<MatchingExp>, (Arc<ExpressionBase>, SourceInfo))>> = metamodelica::nil();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    (argval, stmts, locals, scEnv, accMMDecls) = statementsFromArg(exp.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (_, exptype, _) = argval.clone();
                    exptype = deAliasedType(exptype.clone(), astDefs.clone())?;
                    mcases = elabCasesFromCondition(exptype.clone(), isnot.clone(), rhsval.clone(), tbranch.clone(), ebranch.clone(), tplPackage.clone())?;
                    (argvals, fname, iargs, oargs, scEnv, accMMDecls) = makeMatchFun(argval.clone(), mcases.clone(), exp.clone(), false, scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (_, stmt, _, _, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), sinfo.clone())?;
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::MAP { hasIndexIdentOpt: idxNmOpt, mapExp: mapexp, ofBinding: ofbind, argExp: argexp }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut explst: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>> = metamodelica::nil();
                    let mut mapctx: MapContext;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    explst = getExpListForMap(argexp.clone());
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(explst.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    mapctx = MapContext { ofBinding: ofbind.clone(), mapExp: mapexp.clone(), iterMMExpOptions: mmopts.clone(), hasIndexIdentOpt: idxNmOpt.clone(), useIter: false };
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(true, argvals.clone(), mapctx.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::MAP_ARG_LIST { parts: explst }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut mapctx: MapContext;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(explst.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    mapctx = MapContext { ofBinding: Arc::new(MatchingExp::BIND_MATCH { bindIdent: (literal!("it")).clone() }), mapExp: (Arc::new(ExpressionBase::BOUND_VALUE { boundPath: Arc::new(PathIdent::IDENT { ident: (literal!("it")).clone() }) }), dummySourceInfo.clone()), iterMMExpOptions: mmopts.clone(), hasIndexIdentOpt: None, useIter: false };
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(true, argvals.clone(), mapctx.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::ESCAPED { options: opts, exp }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut popstmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut mmopts = (*mmopts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    (mmopts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(opts.clone(), metamodelica::nil(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (mmopts, stmts, popstmts, intxt) = pushPopBlock(mmopts.clone(), (arcstr::literal!(absIndentOptionId)).clone(), (literal!("BT_ABS_INDENT")).clone(), stmts.clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone())?;
                    (mmopts, stmts, popstmts, intxt) = pushPopBlock(mmopts.clone(), (arcstr::literal!(indentOptionId)).clone(), (literal!("BT_INDENT")).clone(), stmts.clone(), popstmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone())?;
                    (mmopts, stmts, popstmts, intxt) = pushPopBlock(mmopts.clone(), (arcstr::literal!(relIndentOptionId)).clone(), (literal!("BT_REL_INDENT")).clone(), stmts.clone(), popstmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone())?;
                    (mmopts, stmts, popstmts, intxt) = pushPopBlock(mmopts.clone(), (arcstr::literal!(anchorOptionId)).clone(), (literal!("BT_ANCHOR")).clone(), stmts.clone(), popstmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(exp.clone(), mmopts.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    stmts = listAppend(popstmts.clone(), stmts.clone());
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::INDENTATION { items: explst, width: n }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    let mut istr: ArcStr = arcstr::literal!("");
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    istr = (intString(n.clone())).clone();
                    stmt = pushBlockStatement((literal!("BT_INDENT")).clone(), Arc::new(MMExp::MM_LITERAL { value: (istr.clone()).clone() }), (intxt.clone()).clone(), (outtxt.clone()).clone());
                    (stmts, locals, scEnv, accMMDecls, _) = statementsFromExpList(explst.clone(), cons(stmt.clone(), stmts.clone()), (outtxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    stmt = tplStatement((literal!("popBlock")).clone(), metamodelica::nil(), (outtxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LET { exp, letExp: (Deref @ ExpressionBase::TEXT_CREATE { exp: txtexp, name: ident }, _) }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    let mut letOuttxt: Ident = arcstr::literal!("");
                    let mut freshIdent: Ident = arcstr::literal!("");
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n TEXT_CREATE ident = ")); __mm_s.push_str(&*ident.clone()); ArcStr::from(__mm_s) }).clone())?;
                    }
                    encIdent = (encodeIdent((ident.clone()).clone(), (arcstr::literal!(letValueNamePrefix)).clone())?).clone();
                    (freshIdent, locals) = updateLocalsForLetExp((ident.clone()).clone(), (encIdent.clone()).clone(), 0, Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), locals.clone(), scEnv.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(statementsFromExp(txtexp.clone(), metamodelica::nil(), stmts.clone(), (arcstr::literal!(emptyTxt)).clone(), (freshIdent.clone()).clone(), locals.clone(), cons(Scope::RECURSIVE_SCOPE { recIdent: (ident.clone()).clone(), freshIdent: (freshIdent.clone()).clone() }, scEnv.clone()), tplPackage.clone(), accMMDecls.clone())?) {
                        (__pa0, __pa1, Deref @ metamodelica::List::Cons { head: _, tail: __pa2 }, __pa3, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    stmts = __pa0.clone();
                    locals = __pa1.clone();
                    scEnv = __pa2.clone();
                    accMMDecls = __pa3.clone();
                    letOuttxt = __pa4.clone();
                    stmts = if (letOuttxt.clone() == arcstr::literal!(emptyTxt)) {cons(Arc::new(MMExp::MM_ASSIGN { lhsArgs: list![(freshIdent.clone()).clone()], rhs: Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (arcstr::literal!(emptyTxt)).clone() }) }) }), stmts.clone())} else {stmts.clone()};
                    scEnv = cons(Scope::LET_SCOPE { ident: (ident.clone()).clone(), idType: Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), freshIdent: (freshIdent.clone()).clone(), isUsed: false }, scEnv.clone());
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(exp.clone(), metamodelica::nil(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    let __pa5 = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: Scope::LET_SCOPE { .. }, tail: __pa5 } => __pa5.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    scEnv = __pa5.clone();
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LET { exp, letExp: (Deref @ ExpressionBase::TEXT_ADD { exp: txtexp, name: ident }, sinfo2) }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut mmexp: Arc<MMExp>;
                    let mut encIdent: Ident = arcstr::literal!("");
                    let mut path: Arc<PathIdent>;
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    path = Arc::new(PathIdent::IDENT { ident: (ident.clone()).clone() });
                    (mmexp, idtype, scEnv) = resolveBoundPath(path.clone(), scEnv.clone(), tplPackage.clone())?;
                    checkResolvedType(path.clone(), idtype.clone(), (literal!("let +=")).clone(), sinfo2.clone())?;
                    idtype = checkTextType(idtype.clone(), (ident.clone()).clone(), (literal!("let +=")).clone(), sinfo2.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(mmexp.clone()) {
                        Deref @ MMExp::MM_IDENT { ident: Deref @ PathIdent::IDENT { ident: __pa0 } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    encIdent = __pa0.clone();
                    scEnv = cons(Scope::RECURSIVE_SCOPE { recIdent: (ident.clone()).clone(), freshIdent: (encIdent.clone()).clone() }, scEnv.clone());
                    (stmts, locals, scEnv, accMMDecls, _) = statementsFromExp(txtexp.clone(), metamodelica::nil(), stmts.clone(), (encIdent.clone()).clone(), (encIdent.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    let __pa2 = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: Scope::RECURSIVE_SCOPE { .. }, tail: __pa2 } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    scEnv = __pa2.clone();
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(exp.clone(), metamodelica::nil(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LET { exp, letExp: (Deref @ ExpressionBase::NORET_CALL { args: explst, name: fname }, sinfo2) }, _), mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { .. }, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    let mut tyVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut fname = (*fname).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n NORET_CALL fname = ")); __mm_s.push_str(&*pathIdentString(fname.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (fname, iargs, oargs, tyVars) = getFunSignature(fname.clone(), sinfo2.clone(), tplPackage.clone())?;
                    ::match_deref::match_deref! { match &(oargs.clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(explst.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!(" NORET_CALL argList stmts generation passed.\n")).clone())?;
                    }
                    (_, stmt, _, _, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), tyVars.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), sinfo2.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" NORET_CALL stmt =\n")); __mm_s.push_str(&*stmtsString(list![stmt.clone()])?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    stmts = cons(stmt.clone(), stmts.clone());
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(exp.clone(), metamodelica::nil(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LET { letExp: (Deref @ ExpressionBase::NORET_CALL { name: fname, .. }, sinfo2), .. }, _), _, _, _, _, _, _, tplPackage @ TemplPackage { .. }, _) => {
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut fname = (*fname).clone();
                    (fname, _, oargs, _) = getFunSignature(fname.clone(), sinfo2.clone(), tplPackage.clone())?;
                    ::match_deref::match_deref! { match &(oargs.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: _ } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - NORET_CALL with a '")); __mm_s.push_str(&*pathIdentString(fname.clone())?); __mm_s.push_str(&*literal!("' template or function that has output argument(s).\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!statementsFromExp failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStmts, outLocals, outScopeEnv, outMMDecls, outInText))
}

pub fn statementsFromExpList(mut inExpLst: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>>, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inInText: Ident, mut inOutText: Ident, mut inLocals: TypedIdents, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>, Ident)> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    let mut outInText: Ident = arcstr::literal!("");
    (outStmts, outLocals, outScopeEnv, outMMDecls, outInText) = 'mc: {
        let __mc_input = (inExpLst.clone(), inStmts.clone(), inInText.clone(), inOutText.clone(), inLocals.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, stmts, intxt, _, locals, scEnv, _, accMMDecls) => {
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: exp, tail: explst }, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExp(exp.clone(), metamodelica::nil(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromExpList(explst.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!statementsFromExpList failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStmts, outLocals, outScopeEnv, outMMDecls, outInText))
}

pub fn warnIfSomeOptions(mut inMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inMMEscOptions.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (optid, _), tail: _ } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - more options specified than expected for an expression (first option is '")); __mm_s.push_str(&*optid.clone()); __mm_s.push_str(&*literal!("').\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- warnIfSomeOptions failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn statementsFromEscOptions(mut inOptions: Arc<metamodelica::List<(ArcStr, Option<(Arc<ExpressionBase>, SourceInfo)>)>>, mut inAccMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inLocals: TypedIdents, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>)> {
    let mut outAccMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>> = metamodelica::nil();
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    (outAccMMEscOptions, outStmts, outLocals, outScopeEnv, outMMDecls) = 'mc: {
        let __mc_input = (inOptions.clone(), inAccMMEscOptions.clone(), inStmts.clone(), inLocals.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, accMMEscOpts, stmts, locals, scEnv, _, accMMDecls) => {
                    Ok((accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (optid, None), tail: opts }, accMMEscOpts, stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut defoptval: (Arc<MMExp>, Arc<TypeSignature>);
                    let mut accMMEscOpts = (*accMMEscOpts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    defoptval = lookupTupleList(defaultEscOptions.clone(), (optid.clone()).clone())?;
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(accMMEscOpts.clone(), (optid.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (accMMEscOpts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(opts.clone(), cons((optid.clone(), defoptval.clone()), accMMEscOpts.clone()), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (optid, Some(optexp)), tail: opts }, accMMEscOpts, stmts, locals, scEnv, tplPackage @ TemplPackage { astDefs: astdefs, .. }, accMMDecls) => {
                    let mut mmarg: Arc<MMExp>;
                    let mut exptype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut opttype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut sinfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut accMMEscOpts = (*accMMEscOpts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (_, opttype) = lookupTupleList(defaultEscOptions.clone(), (optid.clone()).clone())?;
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(accMMEscOpts.clone(), (optid.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let ((__pa1, __pa2, __pa3), __pa4, __pa5, __pa6, __pa7) = statementsFromArg(optexp.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    mmarg = __pa1.clone();
                    exptype = __pa2.clone();
                    sinfo = __pa3.clone();
                    stmts = __pa4.clone();
                    locals = __pa5.clone();
                    scEnv = __pa6.clone();
                    accMMDecls = __pa7.clone();
                    (mmarg, stmts, locals) = typeAdaptMMOption(mmarg.clone(), exptype.clone(), sinfo.clone(), opttype.clone(), stmts.clone(), locals.clone(), astdefs.clone())?;
                    (accMMEscOpts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(opts.clone(), cons((optid.clone(), (mmarg.clone(), opttype.clone())), accMMEscOpts.clone()), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (optid, _), tail: opts }, accMMEscOpts, stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut accMMEscOpts = (*accMMEscOpts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(defaultEscOptions.clone(), (optid.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - an unknown option'")); __mm_s.push_str(&*optid.clone()); __mm_s.push_str(&*literal!("' was specified. \n")); ArcStr::from(__mm_s) }).clone())?;
                    (accMMEscOpts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(opts.clone(), accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (optid, _), tail: opts }, accMMEscOpts, stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut accMMEscOpts = (*accMMEscOpts).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    let _ = lookupTupleList(defaultEscOptions.clone(), (optid.clone()).clone())?;
                    let _ = lookupTupleList(accMMEscOpts.clone(), (optid.clone()).clone())?;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning - a duplicit option'")); __mm_s.push_str(&*optid.clone()); __mm_s.push_str(&*literal!("' was specified. It will be ignored (not evaluated).\n")); ArcStr::from(__mm_s) }).clone())?;
                    (accMMEscOpts, stmts, locals, scEnv, accMMDecls) = statementsFromEscOptions(opts.clone(), accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((accMMEscOpts.clone(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!(" -statementsFromEscOptions failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAccMMEscOptions, outStmts, outLocals, outScopeEnv, outMMDecls))
}

pub fn getExpListForMap(mut inExp: Expression) -> Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>> {
    let mut outExpsForMap: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>> = metamodelica::nil();
    outExpsForMap = (::match_deref::match_deref! { match &(inExp.clone()) {
        (Deref @ ExpressionBase::MAP_ARG_LIST { parts: explst }, _) => {
            explst.clone()
        },
        _ => {
            list![inExp.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExpsForMap
}

pub fn pushPopBlock(mut inMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, mut inOptionIdent: Ident, mut inBlockTypeIdent: Ident, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inPopBlockStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inInText: Ident, mut inOutText: Ident) -> Result<(Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, Arc<metamodelica::List<Arc<MMExp>>>, Arc<metamodelica::List<Arc<MMExp>>>, Ident)> {
    let mut outMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>> = metamodelica::nil();
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outPopBlockStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outInText: Ident = arcstr::literal!("");
    (outMMEscOptions, outStmts, outPopBlockStmts, outInText) = 'mc: {
        let __mc_input = (inMMEscOptions.clone(), inOptionIdent.clone(), inBlockTypeIdent.clone(), inStmts.clone(), inPopBlockStmts.clone(), inInText.clone(), inOutText.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmopts, optid, btid, stmts, popstmts, intxt, outtxt) => {
                    let mut stmt: Arc<MMExp>;
                    let mut pstmt: Arc<MMExp>;
                    let mut mmexp: Arc<MMExp>;
                    let mut mmopts = (*mmopts).clone();
                    let mut popstmts = (*popstmts).clone();
                    let ((__pa0, _), __pa1) = lookupDeleteTupleList(mmopts.clone(), (optid.clone()).clone())?;
                    mmexp = __pa0.clone();
                    mmopts = __pa1.clone();
                    stmt = pushBlockStatement((btid.clone()).clone(), mmexp.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone());
                    pstmt = tplStatement((literal!("popBlock")).clone(), metamodelica::nil(), (outtxt.clone()).clone(), (outtxt.clone()).clone());
                    popstmts = List::appendElt(pstmt.clone(), popstmts.clone());
                    Ok((mmopts.clone(), cons(stmt.clone(), stmts.clone()), popstmts.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmopts, _, _, stmts, popstmts, intxt, _) => {
                    Ok((mmopts.clone(), stmts.clone(), popstmts.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!pushPopBlock failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMEscOptions, outStmts, outPopBlockStmts, outInText))
}

/*
public function addImplicitArgument
  input list<Expression> inArgLst;
  input TypedIdents inInArgs;
  input TypedIdents inOutArgs;
  input TemplPackage inTplPackage;

  output list<Expression> outArgLst;
algorithm
  outArgLst := matchcontinue (inArgLst, inInArgs, inOutArgs, inTplPackage)
    local
      list<Expression> explst;
      tuple<Ident,TypeSignature> iarg, oarg;
      TemplPackage tplPackage;

    //when the function is a template function
    //and the signature has the only one argument and none is specified on call
    // assume the 'it'
    case ( {}, { iarg, _ }, oarg :: _ , tplPackage)
      algorithm
        areTextInOutArgs(iarg, oarg, tplPackage);
      then { BOUND_VALUE(IDENT("it")) };

    //when the function is a non-template function
    //and the signature has the only one argument and none is specified on the call
    // assume the 'it'
    //- case with an output argument (check if it is not a template function with no argument - i.e. only one text input argument)
    case ( {}, { iarg }, oarg :: _ , tplPackage)
      algorithm
        failure(areTextInOutArgs(iarg, oarg, tplPackage));
      then { BOUND_VALUE(IDENT("it")) };

    //when the function is a non-template function
    //and the signature has the only one argument and none is specified on the call
    // assume the 'it'
    //- case with no output argument (evidently a no-ret non-template function)
    case ( {}, { iarg }, {} , tplPackage)
      then { BOUND_VALUE(IDENT("it")) };


    //otherwise no change
    else inArgLst;

  end matchcontinue;
end addImplicitArgument;
*/
pub fn statementsFromArg(mut inExp: Expression, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inLocals: TypedIdents, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<((Arc<MMExp>, Arc<TypeSignature>, SourceInfo), Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>)> {
    let mut outArgValue: (Arc<MMExp>, Arc<TypeSignature>, SourceInfo);
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    (outArgValue, outStmts, outLocals, outScopeEnv, outMMDecls) = 'mc: {
        let __mc_input = (inExp.clone(), inStmts.clone(), inLocals.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::LITERAL { litType: littype, value: litvalue }, sinfo), stmts, locals, scEnv, _, accMMDecls) => {
                    Ok(((Arc::new(MMExp::MM_LITERAL { value: (litvalue.clone()).clone() }), littype.clone(), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::STR_TOKEN { value: st }, sinfo), stmts, locals, scEnv, _, accMMDecls) => {
                    Ok(((Arc::new(MMExp::MM_STR_TOKEN { value: st.clone() }), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::BOUND_VALUE { boundPath: path }, sinfo), stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut mmexp: Arc<MMExp>;
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut scEnv = (*scEnv).clone();
                    (mmexp, idtype, scEnv) = resolveBoundPath(path.clone(), scEnv.clone(), tplPackage.clone())?;
                    checkResolvedType(path.clone(), idtype.clone(), (literal!("argument")).clone(), sinfo.clone())?;
                    Ok(((mmexp.clone(), idtype.clone(), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::FUN_CALL { args: Deref @ metamodelica::List::Nil, name: Deref @ PathIdent::IDENT { ident: Deref @ "sourceInfo" } }, sinfo @ SourceInfo { columnNumberStart, lineNumberStart, fileName, .. }), stmts, locals, scEnv, _, accMMDecls) => {
                    let mut mmexp: Arc<MMExp>;
                    let mut fname: Arc<PathIdent>;
                    let mut rettype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut lineStr: ArcStr = arcstr::literal!("");
                    let mut colStr: ArcStr = arcstr::literal!("");
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!(" arg sourceInfo \n")).clone())?;
                    }
                    fname = Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("sourceInfo")).clone() }) });
                    rettype = Arc::new(TypeSignature::NAMED_TYPE { name: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("builtin")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("SourceInfo")).clone() }) }) });
                    lineStr = (intString(lineNumberStart.clone())).clone();
                    colStr = (intString(columnNumberStart.clone())).clone();
                    mmexp = Arc::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: list![Arc::new(MMExp::MM_STRING { value: (fileName.clone()).clone() }), Arc::new(MMExp::MM_LITERAL { value: (lineStr.clone()).clone() }), Arc::new(MMExp::MM_LITERAL { value: (colStr.clone()).clone() })] });
                    Ok(((mmexp.clone(), rettype.clone(), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ ExpressionBase::FUN_CALL { args: explst, name: fname }, sinfo), stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    let mut outtxt: Ident = arcstr::literal!("");
                    let mut tyVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut mmexp: Arc<MMExp>;
                    let mut rettype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut fname = (*fname).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (fname, iargs, oargs, tyVars) = getFunSignature(fname.clone(), sinfo.clone(), tplPackage.clone())?;
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(explst.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    outtxt = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(textTempVarNamePrefix)); __mm_s.push_str(&*intString((locals.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                    (_, stmt, mmexp, rettype, locals, outtxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), tyVars.clone(), (arcstr::literal!(emptyTxt)).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), sinfo.clone())?;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" arg FUN_CALL stmt =\n")); __mm_s.push_str(&*stmtsString(list![stmt.clone()])?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    locals = addLocalValue((outtxt.clone()).clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), locals.clone())?;
                    Ok(((mmexp.clone(), rettype.clone(), sinfo.clone()), cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ (_, sinfo), stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut outtxt: Ident = arcstr::literal!("");
                    let mut mmexp: Arc<MMExp>;
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    outtxt = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(textTempVarNamePrefix)); __mm_s.push_str(&*intString((locals.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                    (stmts, locals, scEnv, accMMDecls, outtxt) = statementsFromExp(exp.clone(), metamodelica::nil(), stmts.clone(), (arcstr::literal!(emptyTxt)).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    locals = addLocalValue((outtxt.clone()).clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), locals.clone())?;
                    mmexp = Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (outtxt.clone()).clone() }) });
                    Ok(((mmexp.clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), sinfo.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!statementsFromArg failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outArgValue, outStmts, outLocals, outScopeEnv, outMMDecls))
}

pub fn statementsFromArgList(mut inExpLst: Arc<metamodelica::List<(Arc<ExpressionBase>, SourceInfo)>>, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inLocals: TypedIdents, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>>, Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>)> {
    let mut outArgValues: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    (outArgValues, outStmts, outLocals, outScopeEnv, outMMDecls) = 'mc: {
        let __mc_input = (inExpLst.clone(), inStmts.clone(), inLocals.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, stmts, locals, scEnv, _, accMMDecls) => {
                    Ok((metamodelica::nil(), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: exp, tail: explst }, stmts, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut argval: (Arc<MMExp>, Arc<TypeSignature>, SourceInfo);
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (argval, stmts, locals, scEnv, accMMDecls) = statementsFromArg(exp.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (argvals, stmts, locals, scEnv, accMMDecls) = statementsFromArgList(explst.clone(), stmts.clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((cons(argval.clone(), argvals.clone()), stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!statementsFromArgList failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outArgValues, outStmts, outLocals, outScopeEnv, outMMDecls))
}

pub fn tplStatement(mut inFunName: Ident, mut inArgs: Arc<metamodelica::List<Arc<MMExp>>>, mut inInText: Ident, mut inOutArg: Ident) -> Arc<MMExp> {
    let mut outStmt: Arc<MMExp>;
    outStmt = Arc::new(MMExp::MM_ASSIGN { lhsArgs: list![(inOutArg.clone()).clone()], rhs: Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (inFunName.clone()).clone() }) }), args: cons(Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (inInText.clone()).clone() }) }), inArgs.clone()) }) });
    outStmt
}

pub fn pushBlockStatement(mut inBlockType: Ident, mut inArg: Arc<MMExp>, mut inInText: Ident, mut inOutArg: Ident) -> Arc<MMExp> {
    let mut outStmt: Arc<MMExp>;
    outStmt = Arc::new(MMExp::MM_ASSIGN { lhsArgs: list![(inOutArg.clone()).clone()], rhs: Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("pushBlock")).clone() }) }), args: list![Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (inInText.clone()).clone() }) }), Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (inBlockType.clone()).clone() }) }), args: list![inArg.clone()] })] }) });
    outStmt
}

pub fn addWriteCallFromMMExp(mut inHasRetValue: bool, mut inMMExp: Arc<MMExp>, mut inType: Arc<TypeSignature>, mut inSourceInfo: SourceInfo, mut inMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inInText: Ident, mut inOutText: Ident, mut inLocals: TypedIdents, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>, Ident)> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    let mut outInText: Ident = arcstr::literal!("");
    (outStmts, outLocals, outScopeEnv, outMMDecls, outInText) = 'mc: {
        let __mc_input = (inHasRetValue.clone(), inMMExp.clone(), inType.clone(), inSourceInfo.clone(), inMMEscOptions.clone(), inStmts.clone(), inInText.clone(), inOutText.clone(), inLocals.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, _, _, _, _, stmts, intxt, _, locals, scEnv, _, accMMDecls) => {
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, exptype @ Deref @ TypeSignature::OPTION_TYPE { .. }, _, mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut fname: Arc<PathIdent>;
                    let mut stmt: Arc<MMExp>;
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    (argvals, fname, iargs, oargs, scEnv, accMMDecls) = makeMatchFun((mmexp.clone(), exptype.clone(), inSourceInfo.clone()), list![(Arc::new(MatchingExp::SOME_MATCH { value: Arc::new(MatchingExp::BIND_MATCH { bindIdent: (literal!("val")).clone() }) }), (Arc::new(ExpressionBase::BOUND_VALUE { boundPath: Arc::new(PathIdent::IDENT { ident: (literal!("val")).clone() }) }), dummySourceInfo.clone()))], emptyExpression.clone(), true, scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    (_, stmt, _, _, locals, intxt) = statementFromFun(argvals.clone(), fname.clone(), iargs.clone(), oargs.clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), inSourceInfo.clone())?;
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, exptype @ Deref @ TypeSignature::LIST_TYPE { .. }, _, mmopts, stmts, intxt, outtxt, locals, scEnv, tplPackage, accMMDecls) => {
                    let mut mapctx: MapContext;
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    mapctx = MapContext { ofBinding: Arc::new(MatchingExp::BIND_MATCH { bindIdent: (literal!("it")).clone() }), mapExp: (Arc::new(ExpressionBase::BOUND_VALUE { boundPath: Arc::new(PathIdent::IDENT { ident: (literal!("it")).clone() }) }), dummySourceInfo.clone()), iterMMExpOptions: mmopts.clone(), hasIndexIdentOpt: None, useIter: false };
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(true, list![(mmexp.clone(), exptype.clone(), inSourceInfo.clone())], mapctx.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, _, mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    warnIfSomeOptions(mmopts.clone())?;
                    stmt = tplStatement((literal!("writeTok")).clone(), list![mmexp.clone()], (intxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, Deref @ TypeSignature::TEXT_TYPE { .. }, _, mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    warnIfSomeOptions(mmopts.clone())?;
                    stmt = tplStatement((literal!("writeText")).clone(), list![mmexp.clone()], (intxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, mmexp, exptype, _, mmopts, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    let mut mmexp = (*mmexp).clone();
                    warnIfSomeOptions(mmopts.clone())?;
                    mmexp = mmExpToString(mmexp.clone(), exptype.clone(), inSourceInfo.clone())?;
                    stmt = tplStatement((literal!("writeStr")).clone(), list![mmexp.clone()], (intxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!addWriteCallFromMMExp failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStmts, outLocals, outScopeEnv, outMMDecls, outInText))
}

//no fail
pub fn mmExpToString(mut inMMExp: Arc<MMExp>, mut inType: Arc<TypeSignature>, mut inSourceInfo: SourceInfo) -> Result<Arc<MMExp>> {
    let mut outMMExp: Arc<MMExp>;
    outMMExp = 'mc: {
        let __mc_input = (inMMExp.clone(), inType.clone(), inSourceInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::STRING_TYPE { .. }, _) => {
                    Ok(mmexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MMExp::MM_LITERAL { value: r#str }, _, _) => {
                    Ok(Arc::new(MMExp::MM_STRING { value: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MMExp::MM_STR_TOKEN { value: st }, _, _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Tpl::strTokString(st.clone())?).clone();
                    Ok(Arc::new(MMExp::MM_STRING { value: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, _) => {
                    Ok(Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("strTokString")).clone() }) }), args: list![mmexp.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::TEXT_TYPE { .. }, _) => {
                    Ok(Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("textString")).clone() }) }), args: list![mmexp.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::INTEGER_TYPE { .. }, _) => {
                    Ok(Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::IDENT { ident: (literal!("intString")).clone() }), args: list![mmexp.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::REAL_TYPE { .. }, _) => {
                    Ok(Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::IDENT { ident: (literal!("realString")).clone() }), args: list![mmexp.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::BOOLEAN_TYPE { .. }, _) => {
                    Ok(Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("booleanString")).clone() }) }), args: list![mmexp.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, Deref @ TypeSignature::UNRESOLVED_TYPE { reason }, _) => {
                    let mut reason = (*reason).clone();
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#UnresType# ")); __mm_s.push_str(&*reason.clone()); __mm_s.push_str(&*literal!(" #")); ArcStr::from(__mm_s) }).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - an unresolved value trying to convert to string. Unresolution reason:\n    ")); __mm_s.push_str(&*reason.clone()); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::IDENT { ident: (reason.clone()).clone() }), args: list![mmexp.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, ts, _) => {
                    let mut reason: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Elaborated expression '")); __mm_s.push_str(&*mmExpString(mmexp.clone())?); __mm_s.push_str(&*literal!("' of type '")); __mm_s.push_str(&*typeSignatureString(ts.clone())?); __mm_s.push_str(&*literal!("' has no automatic to-string conversion.")); ArcStr::from(__mm_s) }).clone();
                    addSusanError((r#str.clone()).clone(), inSourceInfo.clone())?;
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error# ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" #")); ArcStr::from(__mm_s) }).clone();
                    Ok(Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::IDENT { ident: (reason.clone()).clone() }), args: list![mmexp.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!mmExpToString failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMMExp)
}

pub fn statementFromFun(mut inArgValues: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>>, mut inFunName: Arc<PathIdent>, mut inInArgs: TypedIdents, mut inOutArgs: TypedIdents, mut inTypeVars: Arc<metamodelica::List<ArcStr>>, mut inInText: Ident, mut inOutText: Ident, mut inLocals: TypedIdents, mut inTplPackage: TemplPackage, mut inInfo: SourceInfo) -> Result<(bool, Arc<MMExp>, Arc<MMExp>, Arc<TypeSignature>, TypedIdents, Ident)> {
    let mut outHasRetValue: bool = false;
    let mut outStmt: Arc<MMExp>;
    let mut outRetMMExp: Arc<MMExp>;
    let mut outRetType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outOutText: Ident = arcstr::literal!("");
    (outHasRetValue, outStmt, outRetMMExp, outRetType, outLocals, outOutText) = 'mc: {
        let __mc_input = (inArgValues.clone(), inFunName.clone(), inInArgs.clone(), inOutArgs.clone(), inTypeVars.clone(), inInText.clone(), inOutText.clone(), inLocals.clone(), inTplPackage.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, Deref @ metamodelica::List::Cons { head: iarg, tail: iargs }, Deref @ metamodelica::List::Cons { head: oarg, tail: Deref @ metamodelica::List::Nil }, tyVars, intxt, outtxt, locals, tplPackage @ TemplPackage { astDefs, .. }, _) => {
                    let mut mmargs: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut mmexp: Arc<MMExp>;
                    let mut mmtxt: Arc<MMExp>;
                    areTextInOutArgs(iarg.clone(), oarg.clone(), tplPackage.clone())?;
                    (mmargs, _) = typeAdaptMMArgsForFun(argvals.clone(), iargs.clone(), tyVars.clone(), metamodelica::nil(), astDefs.clone())?;
                    mmtxt = Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (outtxt.clone()).clone() }) });
                    mmexp = Arc::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: cons(Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (intxt.clone()).clone() }) }), mmargs.clone()) });
                    Ok((false, Arc::new(MMExp::MM_ASSIGN { lhsArgs: list![(outtxt.clone()).clone()], rhs: mmexp.clone() }), mmtxt.clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), locals.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, Deref @ metamodelica::List::Cons { head: iarg, tail: iargs }, Deref @ metamodelica::List::Cons { head: oarg, tail: oargs @ Deref @ metamodelica::List::Cons { head: _, tail: _ } }, tyVars, intxt, outtxt, locals, tplPackage @ TemplPackage { astDefs, .. }, _) => {
                    let mut mmargs: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut lhsArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut mmexp: Arc<MMExp>;
                    let mut mmtxt: Arc<MMExp>;
                    areTextInOutArgs(iarg.clone(), oarg.clone(), tplPackage.clone())?;
                    (mmargs, _) = typeAdaptMMArgsForFun(argvals.clone(), iargs.clone(), tyVars.clone(), metamodelica::nil(), astDefs.clone())?;
                    lhsArgs = elabOutTextArgs(mmargs.clone(), iargs.clone(), oargs.clone(), tplPackage.clone())?;
                    lhsArgs = cons((outtxt.clone()).clone(), lhsArgs.clone());
                    mmtxt = Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (outtxt.clone()).clone() }) });
                    mmexp = Arc::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: cons(Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (intxt.clone()).clone() }) }), mmargs.clone()) });
                    Ok((false, Arc::new(MMExp::MM_ASSIGN { lhsArgs: lhsArgs.clone(), rhs: mmexp.clone() }), mmtxt.clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), locals.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, iargs, Deref @ metamodelica::List::Cons { head: (_, outtype), tail: Deref @ metamodelica::List::Nil }, tyVars, intxt, _, locals, TemplPackage { astDefs, .. }, _) => {
                    let mut setTyVars: TypedIdents = metamodelica::nil();
                    let mut mmargs: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut retval: Ident = arcstr::literal!("");
                    let mut mmexp: Arc<MMExp>;
                    let mut outtype = (*outtype).clone();
                    let mut locals = (*locals).clone();
                    (mmargs, setTyVars) = typeAdaptMMArgsForFun(argvals.clone(), iargs.clone(), tyVars.clone(), metamodelica::nil(), astDefs.clone())?;
                    outtype = specializeType(outtype.clone(), tyVars.clone(), setTyVars.clone())?;
                    retval = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(returnTempVarNamePrefix)); __mm_s.push_str(&*intString((locals.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                    locals = addLocalValue((retval.clone()).clone(), outtype.clone(), locals.clone())?;
                    mmexp = Arc::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: mmargs.clone() });
                    Ok((true, Arc::new(MMExp::MM_ASSIGN { lhsArgs: list![(retval.clone()).clone()], rhs: mmexp.clone() }), Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (retval.clone()).clone() }) }), outtype.clone(), locals.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, iargs, Deref @ metamodelica::List::Nil, tyVars, intxt, _, locals, TemplPackage { astDefs, .. }, _) => {
                    let mut mmargs: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut mmexp: Arc<MMExp>;
                    (mmargs, _) = typeAdaptMMArgsForFun(argvals.clone(), iargs.clone(), tyVars.clone(), metamodelica::nil(), astDefs.clone())?;
                    mmexp = Arc::new(MMExp::MM_FN_CALL { fnName: fname.clone(), args: mmargs.clone() });
                    Ok((false, mmexp.clone(), mmexp.clone(), Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (literal!("No return value.")).clone() }), locals.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argvals, fname, iargs, oargs, _, _, _, _, _, _) => {
                    let mut errArgVals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>)>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    errArgVals = List::map(argvals.clone(), std::sync::Arc::new(fnptr!(Util::tuple312, _)));
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot elaborate function\n  ")); __mm_s.push_str(&*Tpl::tplString3((std::sync::Arc::new(TplCodegen::sFunSignature) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<PathIdent>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>) -> Result<Tpl::Text> + 'static>), fname.clone(), iargs.clone(), oargs.clone())?); __mm_s.push_str(&*literal!("\n  for actual parameters  ")); __mm_s.push_str(&*Tpl::tplString((std::sync::Arc::new(TplCodegen::sActualMMParams) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>)>>) -> Result<Tpl::Text> + 'static>), errArgVals.clone())?); __mm_s.push_str(&*literal!("\n  --> Invalid types (cannot convert) or number of in/out arguments (text in/out arguments must match by order and name equality where prefixes 'in' and 'out' can be used; A function has valid template signature only if all text out params have corresponding in text arguments.).\n")); ArcStr::from(__mm_s) }).clone();
                    addSusanError((r#str.clone()).clone(), inInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outHasRetValue, outStmt, outRetMMExp, outRetType, outLocals, outOutText))
}

pub fn areTextInOutArgs(mut inInArg: (ArcStr, Arc<TypeSignature>), mut inOutArg: (ArcStr, Arc<TypeSignature>), mut inTplPackage: TemplPackage) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inInArg.clone(), inOutArg.clone(), inTplPackage.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((inid, itype), (outid, otype), TemplPackage { astDefs: astdefs, .. }) => {
                    let true = (stringEq((inid.clone()).clone(), (outid.clone()).clone())) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(deAliasedType(itype.clone(), astdefs.clone())?) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(deAliasedType(otype.clone(), astdefs.clone())?) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((inid, itype), (outid, otype), TemplPackage { astDefs: astdefs, .. }) => {
                    let mut inlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut outlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((inid.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "i", tail: Deref @ metamodelica::List::Cons { head: Deref @ "n", tail: __pa0 } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    inlst = __pa0.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(stringListStringChar((outid.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: Deref @ "o", tail: Deref @ metamodelica::List::Cons { head: Deref @ "u", tail: Deref @ metamodelica::List::Cons { head: Deref @ "t", tail: __pa2 } } } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    outlst = __pa2.clone();
                    let true = (inlst.clone() == outlst.clone()) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(deAliasedType(itype.clone(), astdefs.clone())?) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(deAliasedType(otype.clone(), astdefs.clone())?) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn typeAdaptMMArgsForFun(mut inArgValues: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>>, mut inInArgs: TypedIdents, mut inTypeVars: Arc<metamodelica::List<ArcStr>>, mut inSetTypeVars: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents)> {
    let mut outMMArguments: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outSetTypeVars: TypedIdents = metamodelica::nil();
    (outMMArguments, outSetTypeVars) = 'mc: {
        let __mc_input = (inArgValues.clone(), inInArgs.clone(), inTypeVars.clone(), inSetTypeVars.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, setTyVars, _) => {
                    Ok((metamodelica::nil(), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (mmarg, argtype, sinfo), tail: argvals }, Deref @ metamodelica::List::Cons { head: (_, sigArgtype), tail: iargs }, tyVars, setTyVars, astdefs) => {
                    let mut mmargs: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut mmarg = (*mmarg).clone();
                    let mut argtype = (*argtype).clone();
                    let mut setTyVars = (*setTyVars).clone();
                    argtype = deAliasedType(argtype.clone(), astdefs.clone())?;
                    (mmarg, setTyVars) = typeAdaptMMArg(mmarg.clone(), argtype.clone(), sinfo.clone(), true, sigArgtype.clone(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    (mmargs, setTyVars) = typeAdaptMMArgsForFun(argvals.clone(), iargs.clone(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    Ok((cons(mmarg.clone(), mmargs.clone()), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Error - more arguments expected for a function.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Nil, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Error - less number of arguments expected for a function.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!! - typeAdaptMMArgsForFun failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMArguments, outSetTypeVars))
}

pub fn typeAdaptMMArg(mut inMMArg: Arc<MMExp>, mut inArgType: Arc<TypeSignature>, mut inSourceInfo: SourceInfo, mut errorWhenFail: bool, mut inTargetType: Arc<TypeSignature>, mut inTypeVars: Arc<metamodelica::List<ArcStr>>, mut inSetTypeVars: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<MMExp>, TypedIdents)> {
    let mut outMMArg: Arc<MMExp>;
    let mut outSetTypeVars: TypedIdents = metamodelica::nil();
    (outMMArg, outSetTypeVars) = 'mc: {
        let __mc_input = (inMMArg.clone(), inArgType.clone(), inSourceInfo.clone(), errorWhenFail.clone(), inTargetType.clone(), inTypeVars.clone(), inSetTypeVars.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, argtype @ Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, sinfo, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut mmarg: Arc<MMExp>;
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    mmarg = mmExpToString(mmexp.clone(), argtype.clone(), sinfo.clone())?;
                    Ok((mmarg.clone(), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, argtype @ Deref @ TypeSignature::TEXT_TYPE { .. }, sinfo, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut mmarg: Arc<MMExp>;
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    mmarg = mmExpToString(mmexp.clone(), argtype.clone(), sinfo.clone())?;
                    Ok((mmarg.clone(), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, _, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), argtype.clone(), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    Ok((mmarg.clone(), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmexp, argtype, sinfo, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut mmarg: Arc<MMExp>;
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    mmarg = mmExpToString(mmexp.clone(), argtype.clone(), sinfo.clone())?;
                    Ok((mmarg.clone(), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, _, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    Ok((Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("strTokText")).clone() }) }), args: list![mmarg.clone()] }), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, sinfo, _, targettype, tyVars, setTyVars, astdefs) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut setTyVars = (*setTyVars).clone();
                    setTyVars = typesEqual(targettype.clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), tyVars.clone(), setTyVars.clone(), astdefs.clone())?;
                    mmarg = mmExpToString(mmarg.clone(), argtype.clone(), sinfo.clone())?;
                    Ok((Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("stringText")).clone() }) }), args: list![mmarg.clone()] }), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, sinfo, true, targettype, _, setTyVars, _) => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Elaborated expression '")); __mm_s.push_str(&*mmExpString(mmarg.clone())?); __mm_s.push_str(&*literal!("' of type '")); __mm_s.push_str(&*typeSignatureString(argtype.clone())?); __mm_s.push_str(&*literal!("' failed to type adapt to its inferred type '")); __mm_s.push_str(&*typeSignatureString(targettype.clone())?); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) }).clone();
                    addSusanError((msg.clone()).clone(), sinfo.clone())?;
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#Error# ")); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!(" #")); ArcStr::from(__mm_s) }).clone();
                    Ok((Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::IDENT { ident: (msg.clone()).clone() }), args: list![mmarg.clone()] }), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, false, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Fail branch- typeAdaptMMArg failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMArg, outSetTypeVars))
}

pub fn typeAdaptMMOption(mut inMMArg: Arc<MMExp>, mut inArgType: Arc<TypeSignature>, mut sinfo: SourceInfo, mut inTargetType: Arc<TypeSignature>, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inLocals: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<MMExp>, Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents)> {
    let mut outMMArg: Arc<MMExp>;
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    (outMMArg, outStmts, outLocals) = 'mc: {
        let __mc_input = (inMMArg.clone(), inArgType.clone(), sinfo.clone(), inTargetType.clone(), inStmts.clone(), inLocals.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, _, Deref @ TypeSignature::OPTION_TYPE { ofType: targettype }, stmts, locals, astdefs) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut targettype = (*targettype).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    targettype = deAliasedType(targettype.clone(), astdefs.clone())?;
                    (mmarg, stmts, locals) = typeAdaptMMOption(mmarg.clone(), argtype.clone(), sinfo.clone(), targettype.clone(), stmts.clone(), locals.clone(), astdefs.clone())?;
                    mmarg = Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::IDENT { ident: (literal!("SOME")).clone() }), args: list![mmarg.clone()] });
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, _, targettype, stmts, locals, astdefs) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut argtype = (*argtype).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    argtype = deAliasedType(argtype.clone(), astdefs.clone())?;
                    (mmarg, _) = typeAdaptMMArg(mmarg.clone(), argtype.clone(), sinfo.clone(), false, targettype.clone(), metamodelica::nil(), metamodelica::nil(), astdefs.clone())?;
                    (mmarg, stmts, locals) = mmEnsureNonFunctionArg(mmarg.clone(), targettype.clone(), stmts.clone(), locals.clone())?;
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, Deref @ TypeSignature::TEXT_TYPE { .. }, _, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, stmts, locals, _) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    mmarg = Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("textStrTok")).clone() }) }), args: list![mmarg.clone()] });
                    (mmarg, stmts, locals) = mmEnsureNonFunctionArg(mmarg.clone(), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE), stmts.clone(), locals.clone())?;
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, argtype, _, Deref @ TypeSignature::STRING_TOKEN_TYPE { .. }, stmts, locals, _) => {
                    let mut mmarg = (*mmarg).clone();
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    mmarg = mmExpToString(mmarg.clone(), argtype.clone(), sinfo.clone())?;
                    (mmarg, stmts, locals) = mmEnsureNonFunctionArg(mmarg.clone(), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE), stmts.clone(), locals.clone())?;
                    mmarg = Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("ST_STRING")).clone() }) }), args: list![mmarg.clone()] });
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Error - typeAdaptMMOption failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMArg, outStmts, outLocals))
}

pub fn mmEnsureNonFunctionArg(mut inMMArg: Arc<MMExp>, mut inTargetType: Arc<TypeSignature>, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inLocals: TypedIdents) -> Result<(Arc<MMExp>, Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents)> {
    let mut outMMArg: Arc<MMExp>;
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    (outMMArg, outStmts, outLocals) = 'mc: {
        let __mc_input = (inMMArg.clone(), inTargetType.clone(), inStmts.clone(), inLocals.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg @ Deref @ MMExp::MM_FN_CALL { .. }, targettype, stmts, locals) => {
                    let mut retval: ArcStr = arcstr::literal!("");
                    let mut stmts = (*stmts).clone();
                    let mut locals = (*locals).clone();
                    retval = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(returnTempVarNamePrefix)); __mm_s.push_str(&*intString((locals.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                    locals = addLocalValue((retval.clone()).clone(), targettype.clone(), locals.clone())?;
                    stmts = cons(Arc::new(MMExp::MM_ASSIGN { lhsArgs: list![(retval.clone()).clone()], rhs: mmarg.clone() }), stmts.clone());
                    Ok((Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (retval.clone()).clone() }) }), stmts.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mmarg, _, stmts, locals) => {
                    if '__try0: {
                        ::match_deref::match_deref! { match &(mmarg.clone()) {
                            Deref @ MMExp::MM_FN_CALL { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((mmarg.clone(), stmts.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!!- mmEnsureNonFunctionArg failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMArg, outStmts, outLocals))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn elabOutTextArgs(mut inMMArguments: Arc<metamodelica::List<Arc<MMExp>>>, mut inInArgs: TypedIdents, mut inOutArgs: TypedIdents, mut inTplPackage: TemplPackage) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outLhsArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outLhsArgs = 'mc: {
        let __mc_input = (inMMArguments.clone(), inInArgs.clone(), inOutArgs.clone(), inTplPackage.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: mmargs }, Deref @ metamodelica::List::Cons { head: iarg, tail: iargs }, oargs @ Deref @ metamodelica::List::Cons { head: oarg, tail: _ }, tplPackage) => {
                    let mut lhsArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    if '__try0: {
                        unwrap_break_err!(areTextInOutArgs(iarg.clone(), oarg.clone(), tplPackage.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    lhsArgs = elabOutTextArgs(mmargs.clone(), iargs.clone(), oargs.clone(), tplPackage.clone())?;
                    Ok(lhsArgs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: exp @ Deref @ MMExp::MM_IDENT { ident: Deref @ PathIdent::IDENT { ident: txtarg } }, tail: mmargs }, Deref @ metamodelica::List::Cons { head: _, tail: iargs }, Deref @ metamodelica::List::Cons { head: _, tail: oargs }, tplPackage) => {
                    let mut lhsArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let false = (listMember(exp.clone(), mmargs.clone())) else { bail!("pattern mismatch") };
                    lhsArgs = elabOutTextArgs(mmargs.clone(), iargs.clone(), oargs.clone(), tplPackage.clone())?;
                    Ok(cons((txtarg.clone()).clone(), lhsArgs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: mmargs }, Deref @ metamodelica::List::Cons { head: _, tail: iargs }, Deref @ metamodelica::List::Cons { head: _, tail: oargs }, tplPackage) => {
                    let mut lhsArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lhsArgs = elabOutTextArgs(mmargs.clone(), iargs.clone(), oargs.clone(), tplPackage.clone())?;
                    Ok(cons((literal!("_")).clone(), lhsArgs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Error - inconsistent in/out Text arguments for a template function (Output texts are not a subset of input texts).\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("!!!- elabOutTextArgs failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLhsArgs)
}

pub fn statementsFromMapExp(mut inIsFirstArgToMap: bool, mut inArgValuesToMap: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>>, mut inMapContext: MapContext, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inInText: Ident, mut inOutText: Ident, mut inLocals: TypedIdents, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>, Ident)> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    let mut outInText: Ident = arcstr::literal!("");
    (outStmts, outLocals, outScopeEnv, outMMDecls, outInText) = 'mc: {
        let __mc_input = (inIsFirstArgToMap.clone(), inArgValuesToMap.clone(), inMapContext.clone(), inStmts.clone(), inInText.clone(), inOutText.clone(), inLocals.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, MapContext { useIter: true, .. }, stmts, intxt, outtxt, locals, scEnv, _, accMMDecls) => {
                    let mut stmt: Arc<MMExp>;
                    stmt = tplStatement((literal!("popIter")).clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, MapContext { useIter: false, .. }, stmts, intxt, _, locals, scEnv, _, accMMDecls) => {
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (isfirst, Deref @ metamodelica::List::Cons { head: argtomap @ (_, argtype, _), tail: restargs }, MapContext { useIter: useiter, hasIndexIdentOpt, iterMMExpOptions: iopts, mapExp: mapexp @ (_, sinfo), ofBinding: ofbind }, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut mapstmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut rhsMMArgs: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut stmt: Arc<MMExp>;
                    let mut mmRecCall: Arc<MMExp>;
                    let mut oftype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fname: Ident = arcstr::literal!("");
                    let mut idxName: Ident = arcstr::literal!("");
                    let mut freshIdxName: Ident = arcstr::literal!("");
                    let mut localArgs: TypedIdents = metamodelica::nil();
                    let mut encodedExtargs: TypedIdents = metamodelica::nil();
                    let mut maplocals: TypedIdents = metamodelica::nil();
                    let mut caseLocals: TypedIdents = metamodelica::nil();
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut mapctx: MapContext;
                    let mut extargvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut inMapExtargvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut ofbindEnc: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
                    let mut mexp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
                    let mut mmmcEmptyList: MMMatchCase;
                    let mut mmmcCons: MMMatchCase;
                    let mut mmFailCons: MMMatchCase;
                    let mut isUsed: bool = false;
                    let mut mmFun: MMDeclaration;
                    let mut mmmcases: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>)>> = metamodelica::nil();
                    let mut lhsArgs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut assignedIdents: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut localNames: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    let mut useiter = (*useiter).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(argtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    oftype = __pa0.clone();
                    ofbindEnc = typeCheckMatchingExp(ofbind.clone(), oftype.clone(), astDefs.clone())?;
                    idxName = (Util::getOptionOrDefault(hasIndexIdentOpt.clone(), (arcstr::literal!(impossibleIdent)).clone())).clone();
                    freshIdxName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(indexNamePrefix)); __mm_s.push_str(&*idxName.clone()); ArcStr::from(__mm_s) }).clone();
                    (mapstmts, maplocals, scEnv, accMMDecls, _) = statementsFromExp(mapexp.clone(), metamodelica::nil(), metamodelica::nil(), (arcstr::literal!(imlicitTxt)).clone(), (arcstr::literal!(imlicitTxt)).clone(), metamodelica::nil(), cons(Scope::LET_SCOPE { ident: (idxName.clone()).clone(), idType: Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE), freshIdent: (freshIdxName.clone()).clone(), isUsed: false }, cons(Scope::CASE_SCOPE { mExp: ofbindEnc.clone(), mType: oftype.clone(), localNames: metamodelica::nil(), accLocals: metamodelica::nil(), extArgs: metamodelica::nil(), matchArgName: (arcstr::literal!(impossibleIdent)).clone(), hasImplicitScope: true }, cons(Scope::FUN_SCOPE { args: metamodelica::nil(), localArgs: metamodelica::nil() }, scEnv.clone()))), tplPackage.clone(), accMMDecls.clone())?;
                    let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: Scope::LET_SCOPE { ident: _, idType: _, freshIdent: _, isUsed: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Scope::CASE_SCOPE { mExp: __pa2, mType: _, localNames: __pa3, accLocals: __pa4, extArgs: __pa5, matchArgName: _, hasImplicitScope: _ }, tail: Deref @ metamodelica::List::Cons { head: Scope::FUN_SCOPE { args: _, localArgs: __pa6 }, tail: __pa7 } } } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    isUsed = __pa1.clone();
                    mexp = __pa2.clone();
                    localNames = __pa3.clone();
                    caseLocals = __pa4.clone();
                    encodedExtargs = __pa5.clone();
                    localArgs = __pa6.clone();
                    scEnv = __pa7.clone();
                    (mexp, _) = rewriteMatchExpByLocalNames(mexp.clone(), oftype.clone(), localNames.clone(), metamodelica::nil(), astDefs.clone())?;
                    maplocals = listAppend(caseLocals.clone(), maplocals.clone());
                    useiter = shouldUseIterFunctions(isfirst.clone(), useiter.clone(), true, isUsed.clone(), iopts.clone(), restargs.clone())?;
                    stmt = tplStatement((literal!("nextIter")).clone(), metamodelica::nil(), (arcstr::literal!(imlicitTxt)).clone(), (arcstr::literal!(imlicitTxt)).clone());
                    mapstmts = if (useiter.clone()) {cons(stmt.clone(), mapstmts.clone())} else {mapstmts.clone()};
                    fname = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(listMapFunPrefix)); __mm_s.push_str(&*intString((accMMDecls.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                    iargs = cons(imlicitTxtArg.clone(), cons((literal!("items"), argtype.clone()), encodedExtargs.clone()));
                    assignedIdents = getAssignedIdents(mapstmts.clone(), metamodelica::nil())?;
                    oargs = List::filter1OnTrue(encodedExtargs.clone(), (std::sync::Arc::new(fnptr!(isAssignedText, (ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<ArcStr>>) -> Result<bool> + 'static>), assignedIdents.clone());
                    oargs = cons(imlicitTxtArg.clone(), oargs.clone());
                    lhsArgs = List::map(oargs.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
                    inMapExtargvals = List::map(encodedExtargs.clone(), (std::sync::Arc::new(fnptr!(makeMMArgValue, (ArcStr, Arc<TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>)) -> Result<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)> + 'static>));
                    rhsMMArgs = List::map(inMapExtargvals.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)));
                    mmRecCall = Arc::new(MMExp::MM_ASSIGN { lhsArgs: lhsArgs.clone(), rhs: Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::IDENT { ident: (fname.clone()).clone() }), args: cons(Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (arcstr::literal!(imlicitTxt)).clone() }) }), cons(Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (literal!("rest")).clone() }) }), rhsMMArgs.clone())) }) });
                    mapstmts = cons(mmRecCall.clone(), mapstmts.clone()).reverse();
                    (mapstmts, maplocals) = addGetIndex(isUsed.clone(), (freshIdxName.clone()).clone(), mapstmts.clone(), (arcstr::literal!(imlicitTxt)).clone(), maplocals.clone())?;
                    mmmcEmptyList = makeMMMatchCase((Arc::new(MatchingExp::LIST_MATCH { listElts: metamodelica::nil() }), metamodelica::nil(), metamodelica::nil()), encodedExtargs.clone(), oargs.clone())?;
                    mmmcCons = makeMMMatchCase((Arc::new(MatchingExp::LIST_CONS_MATCH { head: mexp.clone(), rest: Arc::new(MatchingExp::BIND_MATCH { bindIdent: (literal!("rest")).clone() }) }), encodedExtargs.clone(), mapstmts.clone()), encodedExtargs.clone(), oargs.clone())?;
                    mmFailCons = makeMMMatchCase((Arc::new(MatchingExp::LIST_CONS_MATCH { head: Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH), rest: Arc::new(MatchingExp::BIND_MATCH { bindIdent: (literal!("rest")).clone() }) }), encodedExtargs.clone(), list![mmRecCall.clone()]), encodedExtargs.clone(), oargs.clone())?;
                    mmmcases = if (isAlwaysMatchedBool(mexp.clone())?) {list![mmmcEmptyList.clone(), mmmcCons.clone()]} else {list![mmmcEmptyList.clone(), mmmcCons.clone(), mmFailCons.clone()]};
                    mapctx = MapContext { ofBinding: ofbind.clone(), mapExp: mapexp.clone(), iterMMExpOptions: iopts.clone(), hasIndexIdentOpt: hasIndexIdentOpt.clone(), useIter: useiter.clone() };
                    maplocals = listAppend(encodedExtargs.clone(), maplocals.clone());
                    maplocals = cons(imlicitTxtArg.clone(), cons((literal!("rest"), argtype.clone()), maplocals.clone()));
                    mmFun = MMDeclaration::MM_FUN { isPublic: false, name: (fname.clone()).clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: maplocals.clone(), statements: list![Arc::new(MMExp::MM_MATCH { matchCases: mmmcases.clone() })], genInfoOpt: GenInfo::GI_MAP_FUN { mapType: argtype.clone(), mapContext: mapctx.clone() } };
                    (stmts, intxt) = addPushIter(isfirst.clone() && useiter.clone(), iopts.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone())?;
                    extargvals = List::map(localArgs.clone(), (std::sync::Arc::new(fnptr!(makeMMArgValue, (ArcStr, Arc<TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>)) -> Result<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)> + 'static>));
                    (_, stmt, _, _, locals, intxt) = statementFromFun(cons(argtomap.clone(), extargvals.clone()), Arc::new(PathIdent::IDENT { ident: (fname.clone()).clone() }), iargs.clone(), oargs.clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), sinfo.clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(false, restargs.clone(), mapctx.clone(), cons(stmt.clone(), stmts.clone()), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), cons(mmFun.clone(), accMMDecls.clone()))?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (isfirst, Deref @ metamodelica::List::Cons { head: argtomap @ (_, argtype, _), tail: restargs }, MapContext { useIter: useiter, hasIndexIdentOpt, iterMMExpOptions: iopts, mapExp: mapexp @ (_, sinfo), ofBinding: ofbind }, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut mapstmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut stmt: Arc<MMExp>;
                    let mut oftype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fname: Ident = arcstr::literal!("");
                    let mut idxName: Ident = arcstr::literal!("");
                    let mut freshIdxName: Ident = arcstr::literal!("");
                    let mut arrName: Ident = arcstr::literal!("");
                    let mut eltName: Ident = arcstr::literal!("");
                    let mut localArgs: TypedIdents = metamodelica::nil();
                    let mut encodedExtargs: TypedIdents = metamodelica::nil();
                    let mut maplocals: TypedIdents = metamodelica::nil();
                    let mut caseLocals: TypedIdents = metamodelica::nil();
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut mapctx: MapContext;
                    let mut extargvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut ofbindEnc: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
                    let mut mexp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
                    let mut isUsed: bool = false;
                    let mut mmFun: MMDeclaration;
                    let mut assignedIdents: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut localNames: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    let mut useiter = (*useiter).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(argtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::ARRAY_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    oftype = __pa0.clone();
                    ofbindEnc = typeCheckMatchingExp(ofbind.clone(), oftype.clone(), astDefs.clone())?;
                    idxName = (Util::getOptionOrDefault(hasIndexIdentOpt.clone(), (arcstr::literal!(impossibleIdent)).clone())).clone();
                    freshIdxName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(indexNamePrefix)); __mm_s.push_str(&*idxName.clone()); ArcStr::from(__mm_s) }).clone();
                    (mapstmts, maplocals, scEnv, accMMDecls, _) = statementsFromExp(mapexp.clone(), metamodelica::nil(), metamodelica::nil(), (arcstr::literal!(imlicitTxt)).clone(), (arcstr::literal!(imlicitTxt)).clone(), metamodelica::nil(), cons(Scope::LET_SCOPE { ident: (idxName.clone()).clone(), idType: Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE), freshIdent: (freshIdxName.clone()).clone(), isUsed: false }, cons(Scope::CASE_SCOPE { mExp: ofbindEnc.clone(), mType: oftype.clone(), localNames: metamodelica::nil(), accLocals: metamodelica::nil(), extArgs: metamodelica::nil(), matchArgName: (arcstr::literal!(impossibleIdent)).clone(), hasImplicitScope: true }, cons(Scope::FUN_SCOPE { args: metamodelica::nil(), localArgs: metamodelica::nil() }, scEnv.clone()))), tplPackage.clone(), accMMDecls.clone())?;
                    let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: Scope::LET_SCOPE { ident: _, idType: _, freshIdent: _, isUsed: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Scope::CASE_SCOPE { mExp: __pa2, mType: _, localNames: __pa3, accLocals: __pa4, extArgs: __pa5, matchArgName: _, hasImplicitScope: _ }, tail: Deref @ metamodelica::List::Cons { head: Scope::FUN_SCOPE { args: _, localArgs: __pa6 }, tail: __pa7 } } } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    isUsed = __pa1.clone();
                    mexp = __pa2.clone();
                    localNames = __pa3.clone();
                    caseLocals = __pa4.clone();
                    encodedExtargs = __pa5.clone();
                    localArgs = __pa6.clone();
                    scEnv = __pa7.clone();
                    (mexp, _) = rewriteMatchExpByLocalNames(mexp.clone(), oftype.clone(), localNames.clone(), metamodelica::nil(), astDefs.clone())?;
                    maplocals = listAppend(caseLocals.clone(), maplocals.clone());
                    useiter = shouldUseIterFunctions(isfirst.clone(), useiter.clone(), true, isUsed.clone(), iopts.clone(), restargs.clone())?;
                    stmt = tplStatement((literal!("nextIter")).clone(), metamodelica::nil(), (arcstr::literal!(imlicitTxt)).clone(), (arcstr::literal!(imlicitTxt)).clone());
                    mapstmts = if (useiter.clone()) {cons(stmt.clone(), mapstmts.clone())} else {mapstmts.clone()};
                    fname = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(arrayMapFunPrefix)); __mm_s.push_str(&*intString((accMMDecls.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                    iargs = cons(imlicitTxtArg.clone(), cons((literal!("items"), argtype.clone()), encodedExtargs.clone()));
                    assignedIdents = getAssignedIdents(mapstmts.clone(), metamodelica::nil())?;
                    oargs = List::filter1OnTrue(encodedExtargs.clone(), (std::sync::Arc::new(fnptr!(isAssignedText, (ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<ArcStr>>) -> Result<bool> + 'static>), assignedIdents.clone());
                    oargs = cons(imlicitTxtArg.clone(), oargs.clone());
                    mapstmts = mapstmts.clone().reverse();
                    (mapstmts, maplocals) = addGetIndex(isUsed.clone(), (freshIdxName.clone()).clone(), mapstmts.clone(), (arcstr::literal!(imlicitTxt)).clone(), maplocals.clone())?;
                    idxName = (literal!("i")).clone();
                    arrName = (literal!("items")).clone();
                    eltName = ((::match_deref::match_deref! { match &(mexp.clone()) {
        Deref @ MatchingExp::BIND_MATCH { bindIdent: eltName } => eltName.clone(),
        Deref @ MatchingExp::REST_MATCH { .. } => literal!(""),
        _ => bail!("match: no arm matched"),
    } })).clone();
                    mapctx = MapContext { ofBinding: ofbind.clone(), mapExp: mapexp.clone(), iterMMExpOptions: iopts.clone(), hasIndexIdentOpt: hasIndexIdentOpt.clone(), useIter: useiter.clone() };
                    mmFun = MMDeclaration::MM_FUN { isPublic: false, name: (fname.clone()).clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: maplocals.clone(), statements: list![Arc::new(MMExp::MM_FOR_LOOP { idxName: (idxName.clone()).clone(), arrName: (arrName.clone()).clone(), eltName: (eltName.clone()).clone(), statements: mapstmts.clone() })], genInfoOpt: GenInfo::GI_MAP_FUN { mapType: argtype.clone(), mapContext: mapctx.clone() } };
                    (stmts, intxt) = addPushIter(isfirst.clone() && useiter.clone(), iopts.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone())?;
                    extargvals = List::map(localArgs.clone(), (std::sync::Arc::new(fnptr!(makeMMArgValue, (ArcStr, Arc<TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>)) -> Result<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)> + 'static>));
                    (_, stmt, _, _, locals, intxt) = statementFromFun(cons(argtomap.clone(), extargvals.clone()), Arc::new(PathIdent::IDENT { ident: (fname.clone()).clone() }), iargs.clone(), oargs.clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), sinfo.clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(false, restargs.clone(), mapctx.clone(), cons(stmt.clone(), stmts.clone()), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), cons(mmFun.clone(), accMMDecls.clone()))?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (isfirst, Deref @ metamodelica::List::Cons { head: argtomap @ (_, argtype, _), tail: restargs }, MapContext { useIter: useiter, hasIndexIdentOpt, iterMMExpOptions: iopts, mapExp: mapexp @ (_, sinfo), ofBinding: ofbind }, stmts, intxt, outtxt, locals, scEnv, tplPackage @ TemplPackage { astDefs, .. }, accMMDecls) => {
                    let mut mapstmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut stmt: Arc<MMExp>;
                    let mut fname: Ident = arcstr::literal!("");
                    let mut idxName: Ident = arcstr::literal!("");
                    let mut freshIdxName: Ident = arcstr::literal!("");
                    let mut localArgs: TypedIdents = metamodelica::nil();
                    let mut encodedExtargs: TypedIdents = metamodelica::nil();
                    let mut maplocals: TypedIdents = metamodelica::nil();
                    let mut caseLocals: TypedIdents = metamodelica::nil();
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut mapctx: MapContext;
                    let mut extargvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut ofbindEnc: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
                    let mut mexp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
                    let mut isUsed: bool = false;
                    let mut mmFun: MMDeclaration;
                    let mut elabcases: Arc<metamodelica::List<(Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>)>> = metamodelica::nil();
                    let mut mmmcases: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>)>> = metamodelica::nil();
                    let mut assignedIdents: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut localNames: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    let mut useiter = (*useiter).clone();
                    let mut stmts = (*stmts).clone();
                    let mut intxt = (*intxt).clone();
                    let mut locals = (*locals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(deAliasedType(argtype.clone(), astDefs.clone()), '__try0)) {
                            Deref @ TypeSignature::LIST_TYPE { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    if '__try1: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(deAliasedType(argtype.clone(), astDefs.clone()), '__try1)) {
                            Deref @ TypeSignature::ARRAY_TYPE { .. } => (),
                            _ => break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    ofbindEnc = typeCheckMatchingExp(ofbind.clone(), argtype.clone(), astDefs.clone())?;
                    idxName = (Util::getOptionOrDefault(hasIndexIdentOpt.clone(), (arcstr::literal!(impossibleIdent)).clone())).clone();
                    freshIdxName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(indexNamePrefix)); __mm_s.push_str(&*idxName.clone()); ArcStr::from(__mm_s) }).clone();
                    (mapstmts, maplocals, scEnv, accMMDecls, _) = statementsFromExp(mapexp.clone(), metamodelica::nil(), metamodelica::nil(), (arcstr::literal!(imlicitTxt)).clone(), (arcstr::literal!(imlicitTxt)).clone(), metamodelica::nil(), cons(Scope::LET_SCOPE { ident: (idxName.clone()).clone(), idType: Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE), freshIdent: (freshIdxName.clone()).clone(), isUsed: false }, cons(Scope::CASE_SCOPE { mExp: ofbindEnc.clone(), mType: argtype.clone(), localNames: metamodelica::nil(), accLocals: metamodelica::nil(), extArgs: metamodelica::nil(), matchArgName: (arcstr::literal!(impossibleIdent)).clone(), hasImplicitScope: true }, cons(Scope::FUN_SCOPE { args: metamodelica::nil(), localArgs: metamodelica::nil() }, scEnv.clone()))), tplPackage.clone(), accMMDecls.clone())?;
                    let (__pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: Scope::LET_SCOPE { ident: _, idType: _, freshIdent: _, isUsed: __pa2 }, tail: Deref @ metamodelica::List::Cons { head: Scope::CASE_SCOPE { mExp: __pa3, mType: _, localNames: __pa4, accLocals: __pa5, extArgs: __pa6, matchArgName: _, hasImplicitScope: _ }, tail: Deref @ metamodelica::List::Cons { head: Scope::FUN_SCOPE { args: _, localArgs: __pa7 }, tail: __pa8 } } } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    isUsed = __pa2.clone();
                    mexp = __pa3.clone();
                    localNames = __pa4.clone();
                    caseLocals = __pa5.clone();
                    encodedExtargs = __pa6.clone();
                    localArgs = __pa7.clone();
                    scEnv = __pa8.clone();
                    (mexp, _) = rewriteMatchExpByLocalNames(mexp.clone(), argtype.clone(), localNames.clone(), metamodelica::nil(), astDefs.clone())?;
                    maplocals = listAppend(caseLocals.clone(), maplocals.clone());
                    useiter = shouldUseIterFunctions(isfirst.clone(), useiter.clone(), false, isUsed.clone(), iopts.clone(), restargs.clone())?;
                    stmt = tplStatement((literal!("nextIter")).clone(), metamodelica::nil(), (arcstr::literal!(imlicitTxt)).clone(), (arcstr::literal!(imlicitTxt)).clone());
                    mapstmts = if (useiter.clone()) {cons(stmt.clone(), mapstmts.clone())} else {mapstmts.clone()};
                    fname = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(scalarMapFunPrefix)); __mm_s.push_str(&*intString((accMMDecls.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                    iargs = cons(imlicitTxtArg.clone(), cons((literal!("it"), argtype.clone()), encodedExtargs.clone()));
                    assignedIdents = getAssignedIdents(mapstmts.clone(), metamodelica::nil())?;
                    oargs = List::filter1OnTrue(encodedExtargs.clone(), (std::sync::Arc::new(fnptr!(isAssignedText, (ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<ArcStr>>) -> Result<bool> + 'static>), assignedIdents.clone());
                    oargs = cons(imlicitTxtArg.clone(), oargs.clone());
                    mapstmts = mapstmts.clone().reverse();
                    (mapstmts, maplocals) = addGetIndex(isUsed.clone(), (freshIdxName.clone()).clone(), mapstmts.clone(), (arcstr::literal!(imlicitTxt)).clone(), maplocals.clone())?;
                    elabcases = addRestElabCase(list![(mexp.clone(), encodedExtargs.clone(), mapstmts.clone())])?;
                    mmmcases = List::map2(elabcases.clone(), (std::sync::Arc::new(makeMMMatchCase) as std::sync::Arc<dyn ::std::ops::Fn((Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>), Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>) -> Result<(Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>)> + 'static>), encodedExtargs.clone(), oargs.clone());
                    mapctx = MapContext { ofBinding: ofbind.clone(), mapExp: mapexp.clone(), iterMMExpOptions: iopts.clone(), hasIndexIdentOpt: hasIndexIdentOpt.clone(), useIter: useiter.clone() };
                    maplocals = listAppend(encodedExtargs.clone(), maplocals.clone());
                    maplocals = cons(imlicitTxtArg.clone(), maplocals.clone());
                    mmFun = MMDeclaration::MM_FUN { isPublic: false, name: (fname.clone()).clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: maplocals.clone(), statements: list![Arc::new(MMExp::MM_MATCH { matchCases: mmmcases.clone() })], genInfoOpt: GenInfo::GI_MAP_FUN { mapType: argtype.clone(), mapContext: mapctx.clone() } };
                    (stmts, intxt) = addPushIter(isfirst.clone() && useiter.clone(), iopts.clone(), stmts.clone(), (intxt.clone()).clone(), (outtxt.clone()).clone())?;
                    extargvals = List::map(localArgs.clone(), (std::sync::Arc::new(fnptr!(makeMMArgValue, (ArcStr, Arc<TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>)) -> Result<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)> + 'static>));
                    (_, stmt, _, _, locals, intxt) = statementFromFun(cons(argtomap.clone(), extargvals.clone()), Arc::new(PathIdent::IDENT { ident: (fname.clone()).clone() }), iargs.clone(), oargs.clone(), metamodelica::nil(), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), tplPackage.clone(), sinfo.clone())?;
                    (stmts, locals, scEnv, accMMDecls, intxt) = statementsFromMapExp(false, restargs.clone(), mapctx.clone(), cons(stmt.clone(), stmts.clone()), (intxt.clone()).clone(), (outtxt.clone()).clone(), locals.clone(), scEnv.clone(), tplPackage.clone(), cons(mmFun.clone(), accMMDecls.clone()))?;
                    Ok((stmts.clone(), locals.clone(), scEnv.clone(), accMMDecls.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!statementsFromMapExp failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStmts, outLocals, outScopeEnv, outMMDecls, outInText))
}

pub fn intersectInOutArgs(mut inList1: TypedIdents, mut inList2: TypedIdents) -> Result<(Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>)> {
    pub fn areTypedIdentsEqual(mut inTypedIdent1: (ArcStr, Arc<TypeSignature>), mut inTypedIdent2: (ArcStr, Arc<TypeSignature>)) -> bool {
        let mut equal: bool = false;
        let mut ident1: Ident = arcstr::literal!("");
        let mut ident2: Ident = arcstr::literal!("");
        (ident1, _) = inTypedIdent1.clone();
        (ident2, _) = inTypedIdent2.clone();
        equal = stringEq((ident1.clone()).clone(), (ident2.clone()).clone());
        equal
    }

    let mut outIntersectionAndRests: (Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>);
    let mut outIntersection: TypedIdents = metamodelica::nil();
    let mut outList1Rest: TypedIdents = metamodelica::nil();
    let mut outList2Rest: TypedIdents = metamodelica::nil();
    (outIntersection, outList1Rest, outList2Rest) = List::intersection1OnTrue(inList1.clone(), inList2.clone(), (std::sync::Arc::new(fnptr!(areTypedIdentsEqual, (ArcStr, Arc<TypeSignature>), (ArcStr, Arc<TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>), (ArcStr, Arc<TypeSignature>)) -> Result<bool> + 'static>))?;
    outIntersectionAndRests = (outIntersection.clone(), outList1Rest.clone(), outList2Rest.clone());
    Ok(outIntersectionAndRests)
}

/*
function isIndexArg
  input tuple<Ident, TypeSignature> inArg;
  output Boolean outIsIndexArg;
algorithm
  outIsIndexArg := match inArg
    case ( ("i_i0" , _) )  then true;
    case ( ("i_i1" , _) )  then true;
    case ( _ )            then false;
  end match;
end isIndexArg;
*/
pub fn shouldUseIterFunctions(mut inIsFirstArgToMap: bool, mut inUseIterLast: bool, mut inIsListArgToMap: bool, mut wasIndexVarUsed: bool, mut inIterOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, mut inRestArgValsToMap: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>>) -> Result<bool> {
    let mut outUseIterFuns: bool = false;
    outUseIterFuns = 'mc: {
        let __mc_input = (inIsFirstArgToMap.clone(), inUseIterLast.clone(), inIsListArgToMap.clone(), wasIndexVarUsed.clone(), inIterOptions.clone(), inRestArgValsToMap.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, useiter, _, _, _, _) => {
                    Ok(useiter.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, true, false, iopts, _) => {
                    let mut iopts = (*iopts).clone();
                    iopts = listAppend(iopts.clone(), nonSpecifiedIterOptions.clone());
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), (arcstr::literal!(emptyOptionId)).clone())?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "NONE()" }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), (arcstr::literal!(separatorOptionId)).clone())?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "NONE()" }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), (arcstr::literal!(alignNumOptionId)).clone())?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "0" }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), (arcstr::literal!(wrapWidthOptionId)).clone())?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "0" }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, false, false, iopts, Deref @ metamodelica::List::Nil) => {
                    let mut iopts = (*iopts).clone();
                    iopts = listAppend(iopts.clone(), nonSpecifiedIterOptions.clone());
                    ::match_deref::match_deref! { match &(lookupTupleList(iopts.clone(), (arcstr::literal!(emptyOptionId)).clone())?) {
                        (Deref @ MMExp::MM_LITERAL { value: Deref @ "NONE()" }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outUseIterFuns)
}

/*
public function addNextIter
  input Boolean inUseIterFun;
  input list<MMExp> inStmts;
  input Ident inInText;
  input Ident inOutText;

  output list<MMExp> outStmts;
  output Ident outInText;
algorithm
  (outStmts, outInText)
  := matchcontinue (inUseIterFun, inStmts, inInText, inOutText)
    local
      list<MMExp> stmts;
      MMExp stmt;
      Ident intxt, outtxt;

    case ( true, stmts, intxt, outtxt)
      algorithm
        stmt = tplStatement("nextIter", {}, intxt, outtxt);
      then ( stmt :: stmts, outtxt );

    case ( false, stmts, intxt, _)
      then ( stmts, intxt );

    //cannot happen
    else
      algorithm
        true = Flags.isSet(Flags.FAILTRACE); Debug.trace("-!!!addNextIter failed\n");
      then
        fail();
  end matchcontinue;
end addNextIter;
*/
pub fn addGetIndex(mut wasIndexUsed: bool, mut inLocalIdxValIdent: Ident, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inInText: Ident, mut inLocals: TypedIdents) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents)> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    (outStmts, outLocals) = 'mc: {
        let __mc_input = (wasIndexUsed.clone(), inLocalIdxValIdent.clone(), inStmts.clone(), inInText.clone(), inLocals.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, localidxid, stmts, intxt, locals) => {
                    let mut stmt: Arc<MMExp>;
                    let mut locals = (*locals).clone();
                    stmt = tplStatement((literal!("getIteri_i0")).clone(), metamodelica::nil(), (intxt.clone()).clone(), (localidxid.clone()).clone());
                    locals = addLocalValue((localidxid.clone()).clone(), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE), locals.clone())?;
                    Ok((cons(stmt.clone(), stmts.clone()), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, _, stmts, _, locals) => {
                    Ok((stmts.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!addGetIndex failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStmts, outLocals))
}

pub fn addPushIter(mut inDoAddPushIter: bool, mut inMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inInText: Ident, mut inOutText: Ident) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, Ident)> {
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outInText: Ident = arcstr::literal!("");
    (outStmts, outInText) = 'mc: {
        let __mc_input = (inDoAddPushIter.clone(), inMMEscOptions.clone(), inStmts.clone(), inInText.clone(), inOutText.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, _, stmts, intxt, _) => {
                    Ok((stmts.clone(), intxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, opts, stmts, intxt, outtxt) => {
                    let mut mmopts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut stmt: Arc<MMExp>;
                    (mmopts, _) = makeMMExpOptions(nonSpecifiedIterOptions.clone(), opts.clone())?;
                    stmt = tplStatement((literal!("pushIter")).clone(), list![Arc::new(MMExp::MM_FN_CALL { fnName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("ITER_OPTIONS")).clone() }) }), args: mmopts.clone() })], (intxt.clone()).clone(), (outtxt.clone()).clone());
                    Ok((cons(stmt.clone(), stmts.clone()), outtxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!addNextIter failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStmts, outInText))
}

pub fn makeMMExpOptions(mut inMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>, mut inSpecifiedMMEscOptions: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>) -> Result<(Arc<metamodelica::List<Arc<MMExp>>>, Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>>)> {
    let mut outMMExpOpts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outRestSpecifiedMMExpOpts: Arc<metamodelica::List<(ArcStr, (Arc<MMExp>, Arc<TypeSignature>))>> = metamodelica::nil();
    (outMMExpOpts, outRestSpecifiedMMExpOpts) = 'mc: {
        let __mc_input = (inMMEscOptions.clone(), inSpecifiedMMEscOptions.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, specopts) => {
                    warnIfSomeOptions(specopts.clone())?;
                    Ok((metamodelica::nil(), specopts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (optid, _), tail: rest }, specopts) => {
                    let mut mexpOpts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut mexpopt: Arc<MMExp>;
                    let mut specopts = (*specopts).clone();
                    let ((__pa0, _), __pa1) = lookupDeleteTupleList(specopts.clone(), (optid.clone()).clone())?;
                    mexpopt = __pa0.clone();
                    specopts = __pa1.clone();
                    (mexpOpts, specopts) = makeMMExpOptions(rest.clone(), specopts.clone())?;
                    Ok((cons(mexpopt.clone(), mexpOpts.clone()), specopts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (_, (mexpopt, _)), tail: rest }, specopts) => {
                    let mut mexpOpts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut specopts = (*specopts).clone();
                    (mexpOpts, specopts) = makeMMExpOptions(rest.clone(), specopts.clone())?;
                    Ok((cons(mexpopt.clone(), mexpOpts.clone()), specopts.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!makeMMExpOptions failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMExpOpts, outRestSpecifiedMMExpOpts))
}

/*
public function mmexpFromStrTokOption
  input Option<StringToken> inStrTokOption;
  output MMExp outMMExp;
algorithm
  outMMExp := match inStrTokOption
    local
      StringToken st;

    case NONE()
      then MM_LITERAL("NONE()");

    case ( SOME(st) )
      then MM_FN_CALL(IDENT("SOME"), { MM_STR_TOKEN(st) });

  end match;
end mmexpFromStrTokOption;
*/
//fail and error
pub fn makeMatchFun(mut inArgval: (Arc<MMExp>, Arc<TypeSignature>, SourceInfo), mut inMCases: Arc<metamodelica::List<(Arc<MatchingExp>, (Arc<ExpressionBase>, SourceInfo))>>, mut inArgExp: Expression, mut hasImplicitLookup: bool, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>>, Arc<PathIdent>, TypedIdents, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>)> {
    let mut outArgvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
    let mut outFunName: Arc<PathIdent>;
    let mut outInArgs: TypedIdents = metamodelica::nil();
    let mut outOutArgs: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    (outArgvals, outFunName, outInArgs, outOutArgs, outScopeEnv, outMMDecls) = 'mc: {
        let __mc_input = (inArgval.clone(), inMCases.clone(), inArgExp.clone(), hasImplicitLookup.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argval @ (mmexp, exptype, _), mcases, _, _, scEnv, tplPackage, accMMDecls) => {
                    let mut argvals: Arc<metamodelica::List<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)>> = metamodelica::nil();
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut extargs: TypedIdents = metamodelica::nil();
                    let mut localArgs: TypedIdents = metamodelica::nil();
                    let mut encodedExtargs: TypedIdents = metamodelica::nil();
                    let mut funLocals: TypedIdents = metamodelica::nil();
                    let mut elabcases: Arc<metamodelica::List<(Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>)>> = metamodelica::nil();
                    let mut mmmcases: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>)>> = metamodelica::nil();
                    let mut mmFun: MMDeclaration;
                    let mut fname: Ident = arcstr::literal!("");
                    let mut matchArgName: Ident = arcstr::literal!("");
                    let mut implicitValueName: Ident = arcstr::literal!("");
                    let mut assignedIdents: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    (implicitValueName, matchArgName) = getMatchArgName(inArgExp.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(elabMatchCases((mmexp.clone(), exptype.clone()), (implicitValueName.clone()).clone(), mcases.clone(), hasImplicitLookup.clone(), metamodelica::nil(), metamodelica::nil(), cons(Scope::FUN_SCOPE { args: metamodelica::nil(), localArgs: metamodelica::nil() }, scEnv.clone()), tplPackage.clone(), accMMDecls.clone())?) {
                        (__pa0, __pa1, Deref @ metamodelica::List::Cons { head: Scope::FUN_SCOPE { args: __pa2, localArgs: __pa3 }, tail: __pa4 }, __pa5, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    elabcases = __pa0.clone();
                    funLocals = __pa1.clone();
                    extargs = __pa2.clone();
                    localArgs = __pa3.clone();
                    scEnv = __pa4.clone();
                    accMMDecls = __pa5.clone();
                    assignedIdents = __pa6.clone();
                    elabcases = addRestElabCase(elabcases.clone())?;
                    (extargs, localArgs) = alignExtArgsToScopeEnv(extargs.clone(), localArgs.clone(), scEnv.clone())?;
                    encodedExtargs = List::map1(extargs.clone(), (std::sync::Arc::new(encodeTypedIdent) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>), ArcStr) -> Result<(ArcStr, Arc<TypeSignature>)> + 'static>), (arcstr::literal!(funArgNamePrefix)).clone());
                    iargs = cons(imlicitTxtArg.clone(), cons((matchArgName.clone(), exptype.clone()), encodedExtargs.clone()));
                    oargs = List::filter1OnTrue(encodedExtargs.clone(), (std::sync::Arc::new(fnptr!(isAssignedText, (ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<ArcStr>>) -> Result<bool> + 'static>), assignedIdents.clone());
                    oargs = cons(imlicitTxtArg.clone(), oargs.clone());
                    funLocals = listAppend(encodedExtargs.clone(), funLocals.clone());
                    mmmcases = List::map2(elabcases.clone(), (std::sync::Arc::new(makeMMMatchCase) as std::sync::Arc<dyn ::std::ops::Fn((Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>), Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>) -> Result<(Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>)> + 'static>), encodedExtargs.clone(), oargs.clone());
                    fname = (stringAppend((arcstr::literal!(matchFunPrefix)).clone(), (intString((accMMDecls.clone().len() as i32))).clone())).clone();
                    mmFun = MMDeclaration::MM_FUN { isPublic: false, name: (fname.clone()).clone(), inArgs: iargs.clone(), outArgs: oargs.clone(), locals: cons(imlicitTxtArg.clone(), funLocals.clone()), statements: list![Arc::new(MMExp::MM_MATCH { matchCases: mmmcases.clone() })], genInfoOpt: crate::TplAbsyn::GenInfo::GI_MATCH_FUN };
                    argvals = List::map(localArgs.clone(), (std::sync::Arc::new(fnptr!(makeMMArgValue, (ArcStr, Arc<TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>)) -> Result<(Arc<MMExp>, Arc<TypeSignature>, SourceInfo)> + 'static>));
                    argvals = cons(argval.clone(), argvals.clone());
                    Ok((argvals.clone(), Arc::new(PathIdent::IDENT { ident: (fname.clone()).clone() }), iargs.clone(), oargs.clone(), scEnv.clone(), cons(mmFun.clone(), accMMDecls.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!makeMatchFun failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outArgvals, outFunName, outInArgs, outOutArgs, outScopeEnv, outMMDecls))
}

//no fail
pub fn alignExtArgsToScopeEnv(mut inExtraArgs: TypedIdents, mut inEncExtraArgs: TypedIdents, mut inScopeEnv: ScopeEnv) -> Result<(TypedIdents, TypedIdents)> {
    let mut outExtraArgs: TypedIdents = metamodelica::nil();
    let mut outEncExtraArgs: TypedIdents = metamodelica::nil();
    (outExtraArgs, outEncExtraArgs) = 'mc: {
        let __mc_input = (inExtraArgs.clone(), inEncExtraArgs.clone(), inScopeEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (extargs, encExtargs, Deref @ metamodelica::List::Cons { head: Scope::FUN_SCOPE { localArgs, args: fargs }, tail: _ }) => {
                    let mut extargsAligned: TypedIdents = metamodelica::nil();
                    let mut encExtargsAligned: TypedIdents = metamodelica::nil();
                    extargsAligned = alignTupleList(extargs.clone(), fargs.clone())?;
                    encExtargsAligned = alignTupleList(encExtargs.clone(), localArgs.clone())?;
                    let true = ((extargsAligned.clone().len() as i32) == (extargs.clone().len() as i32)) else { bail!("pattern mismatch") };
                    let true = ((encExtargsAligned.clone().len() as i32) == (encExtargs.clone().len() as i32)) else { bail!("pattern mismatch") };
                    Ok((extargsAligned.clone(), encExtargsAligned.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExtraArgs.clone(), inEncExtraArgs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExtraArgs, outEncExtraArgs))
}

//no fail
pub fn getMatchArgName(mut inArgExp: Expression) -> Result<(Ident, Ident)> {
    let mut outInputValueName: Ident = arcstr::literal!("");
    let mut outMatchArgName: Ident = arcstr::literal!("");
    (outInputValueName, outMatchArgName) = 'mc: {
        let __mc_input = inArgExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ ExpressionBase::BOUND_VALUE { boundPath: path }, _) => {
                    let mut outMatchArgName: ArcStr = outMatchArgName.clone();
                    let mut outInputValueName: ArcStr = outInputValueName.clone();
                    outInputValueName = (pathIdentString(path.clone())?).clone();
                    outMatchArgName = (encodeIdent((outInputValueName.clone()).clone(), (arcstr::literal!(funArgNamePrefix)).clone())?).clone();
                    Ok((outInputValueName.clone(), outMatchArgName.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((arcstr::literal!(impossibleIdent), arcstr::literal!(matchDefaultArgName)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outInputValueName, outMatchArgName))
}

//no fail
pub fn makeMMArgValue(mut inTypedIdent: (ArcStr, Arc<TypeSignature>)) -> (Arc<MMExp>, Arc<TypeSignature>, SourceInfo) {
    let mut outArgValue: (Arc<MMExp>, Arc<TypeSignature>, SourceInfo);
    outArgValue = (::match_deref::match_deref! { match &(inTypedIdent.clone()) {
        (argname, ts) => {
            (Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (argname.clone()).clone() }) }), ts.clone(), dummySourceInfo.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArgValue
}

pub fn isText(mut inArg: (ArcStr, Arc<TypeSignature>)) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &(inArg.clone()) {
        (_, Deref @ TypeSignature::TEXT_TYPE { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn isAssignedText(mut inArg: (ArcStr, Arc<TypeSignature>), mut inAssignedTexts: Arc<metamodelica::List<ArcStr>>) -> bool {
    let mut outB: bool = false;
    outB = (::match_deref::match_deref! { match &((inArg.clone(), inAssignedTexts.clone())) {
        ((ident, Deref @ TypeSignature::TEXT_TYPE { .. }), assignedTexts) if (listMember((ident.clone()).clone(), assignedTexts.clone())) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn elabMatchCases(mut inItArgVal: (Arc<MMExp>, Arc<TypeSignature>), mut inImplicitValueName: Ident, mut inMCases: Arc<metamodelica::List<(Arc<MatchingExp>, (Arc<ExpressionBase>, SourceInfo))>>, mut hasImplicitLookup: bool, mut inLocals: TypedIdents, mut inAccCaseLocals: TypedIdents, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage, mut inAccMMDecls: Arc<metamodelica::List<MMDeclaration>>) -> Result<(Arc<metamodelica::List<(Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>)>>, TypedIdents, ScopeEnv, Arc<metamodelica::List<MMDeclaration>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outMMMCases: Arc<metamodelica::List<(Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>)>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    let mut outMMDecls: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
    let mut outAssignedIdents: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outMMMCases, outLocals, outScopeEnv, outMMDecls, outAssignedIdents) = 'mc: {
        let __mc_input = (inItArgVal.clone(), inImplicitValueName.clone(), inMCases.clone(), hasImplicitLookup.clone(), inLocals.clone(), inAccCaseLocals.clone(), inScopeEnv.clone(), inTplPackage.clone(), inAccMMDecls.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil, _, locals, _, scEnv, _, accMMDecls) => {
                    let mut locals = (*locals).clone();
                    locals = listAppend(inAccCaseLocals.clone(), locals.clone());
                    Ok((metamodelica::nil(), locals.clone(), scEnv.clone(), accMMDecls.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argval @ (_, exptype), _, Deref @ metamodelica::List::Cons { head: (mexp, exp), tail: mcases }, _, locals, accCaseLocals, scEnv, tplPackage @ TemplPackage { astDefs: astdefs, .. }, accMMDecls) => {
                    let mut extargs: TypedIdents = metamodelica::nil();
                    let mut elabcases: Arc<metamodelica::List<(Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>)>> = metamodelica::nil();
                    let mut stmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
                    let mut assignedIdents: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut localNames: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    let mut mexp = (*mexp).clone();
                    let mut locals = (*locals).clone();
                    let mut accCaseLocals = (*accCaseLocals).clone();
                    let mut scEnv = (*scEnv).clone();
                    let mut accMMDecls = (*accMMDecls).clone();
                    mexp = typeCheckMatchingExp(mexp.clone(), exptype.clone(), astdefs.clone())?;
                    (stmts, locals, scEnv, accMMDecls, _) = statementsFromExp(exp.clone(), metamodelica::nil(), metamodelica::nil(), (arcstr::literal!(imlicitTxt)).clone(), (arcstr::literal!(imlicitTxt)).clone(), locals.clone(), cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: exptype.clone(), localNames: metamodelica::nil(), accLocals: accCaseLocals.clone(), extArgs: metamodelica::nil(), matchArgName: (inImplicitValueName.clone()).clone(), hasImplicitScope: hasImplicitLookup.clone() }, scEnv.clone()), tplPackage.clone(), accMMDecls.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(scEnv.clone()) {
                        Deref @ metamodelica::List::Cons { head: Scope::CASE_SCOPE { mExp: __pa0, mType: _, localNames: __pa1, accLocals: __pa2, extArgs: __pa3, matchArgName: _, hasImplicitScope: _ }, tail: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    mexp = __pa0.clone();
                    localNames = __pa1.clone();
                    accCaseLocals = __pa2.clone();
                    extargs = __pa3.clone();
                    scEnv = __pa4.clone();
                    stmts = stmts.clone().reverse();
                    (mexp, _) = rewriteMatchExpByLocalNames(mexp.clone(), exptype.clone(), localNames.clone(), metamodelica::nil(), astdefs.clone())?;
                    (elabcases, locals, scEnv, accMMDecls, assignedIdents) = elabMatchCases(argval.clone(), (inImplicitValueName.clone()).clone(), mcases.clone(), hasImplicitLookup.clone(), locals.clone(), accCaseLocals.clone(), scEnv.clone(), tplPackage.clone(), accMMDecls.clone())?;
                    assignedIdents = getAssignedIdents(stmts.clone(), assignedIdents.clone())?;
                    Ok((cons((mexp.clone(), extargs.clone(), stmts.clone()), elabcases.clone()), locals.clone(), scEnv.clone(), accMMDecls.clone(), assignedIdents.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!elabMatchCases failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMMCases, outLocals, outScopeEnv, outMMDecls, outAssignedIdents))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getAssignedIdents(mut inStatements: Arc<metamodelica::List<Arc<MMExp>>>, mut inAssignedIdents: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outAssignedIdents: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outAssignedIdents = 'mc: {
        let __mc_input = (inStatements.clone(), inAssignedIdents.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, assignedIdents) => {
                    Ok(assignedIdents.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ MMExp::MM_ASSIGN { lhsArgs: largs, .. }, tail: stmts }, assignedIdents) => {
                    let mut assignedIdents = (*assignedIdents).clone();
                    assignedIdents = List::fold(largs.clone(), std::sync::Arc::new(fnptr!(List::unionElt, _, _)), assignedIdents.clone());
                    Ok(getAssignedIdents(stmts.clone(), assignedIdents.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: stmts }, assignedIdents) => {
                    Ok(getAssignedIdents(stmts.clone(), assignedIdents.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!getAssignedTexts failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAssignedIdents)
}

/*
public function getItNameFromArg
  input MMExp inItArgMMExp;
  input TypeSignature inMType;
  input MatchingExp inMatchingExp;
  input list<ASTDef> inASTDefs;

  output Ident outItName;
algorithm
  outItName := matchcontinue (inItArgMMExp, inMType, inMatchingExp, inASTDefs)
    local
      TypeSignature exptype;
      MatchingExp mexp;
      list<ASTDef> astdefs;
      MMExp mmexp;
      PathIdent path;
      Ident argid;

    //name it by the arg name if the name is not bound
    case ( MM_IDENT(path as IDENT(argid)), exptype, mexp, astdefs)
      algorithm
        //only when the argid is not yet bound by the user to do it explicit or hide the name from the upper scope
        failure( (_,_) = lookupUpdateMatchingExp(argid, path, mexp, exptype, astdefs) );
      then
        argid;

    //otherwise return "it" as the name
    else "it";

  end matchcontinue;
end getItNameFromArg;
*/
//fail and error
pub fn typeCheckMatchingExp(mut inMatchingExp: Arc<MatchingExp>, mut inMType: Arc<TypeSignature>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<Arc<MatchingExp>> {
    let mut outTransformedMatchingExp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    outTransformedMatchingExp = 'mc: {
        let __mc_input = (inMatchingExp.clone(), inMType.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_AS_MATCH { matchingExp: mexp, bindIdent: bid }, mtype, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    mexp = typeCheckMatchingExp(mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok(Arc::new(MatchingExp::BIND_AS_MATCH { bindIdent: (bid.clone()).clone(), matchingExp: mexp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::BIND_MATCH { .. }, _, _) => {
                    Ok(mexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::RECORD_MATCH { .. }, Deref @ TypeSignature::TEXT_TYPE { .. }, _) => {
                    Ok(mexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::RECORD_MATCH { fieldMatchings: fms, tagName: tagpath }, mtype, astDefs) => {
                    let mut fields: TypedIdents = metamodelica::nil();
                    let mut fms = (*fms).clone();
                    let mut tagpath = (*tagpath).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    (fields, tagpath) = getFieldsForRecord(mtype.clone(), tagpath.clone(), astDefs.clone())?;
                    fms = typeCheckMatchingExpRecord(fms.clone(), fields.clone(), astDefs.clone())?;
                    Ok(Arc::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::SOME_MATCH { value: mexp }, mtype, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    let mut mtype = (*mtype).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::OPTION_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    mtype = __pa0.clone();
                    mexp = typeCheckMatchingExp(mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok(Arc::new(MatchingExp::SOME_MATCH { value: mexp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::NONE_MATCH { .. }, mtype, astDefs) => {
                    ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::OPTION_TYPE { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(mexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst }, mtype, astDefs) => {
                    let mut otLst: Arc<metamodelica::List<Arc<TypeSignature>>> = metamodelica::nil();
                    let mut mexpLst = (*mexpLst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::TUPLE_TYPE { ofTypes: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    otLst = __pa0.clone();
                    mexpLst = typeCheckMatchingExpList(mexpLst.clone(), otLst.clone(), astDefs.clone())?;
                    Ok(Arc::new(MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::LIST_MATCH { listElts: mexpLst }, mtype, astDefs) => {
                    let mut ot: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut otLst: Arc<metamodelica::List<Arc<TypeSignature>>> = metamodelica::nil();
                    let mut mexpLst = (*mexpLst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ot = __pa0.clone();
                    otLst = List::fill(ot.clone(), (mexpLst.clone().len() as i32));
                    mexpLst = typeCheckMatchingExpList(mexpLst.clone(), otLst.clone(), astDefs.clone())?;
                    Ok(Arc::new(MatchingExp::LIST_MATCH { listElts: mexpLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::LIST_CONS_MATCH { rest: restmexp, head: mexp }, mtype, astDefs) => {
                    let mut ot: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut restmexp = (*restmexp).clone();
                    let mut mexp = (*mexp).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(mtype.clone()) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ot = __pa0.clone();
                    mexp = typeCheckMatchingExp(mexp.clone(), ot.clone(), astDefs.clone())?;
                    restmexp = typeCheckMatchingExp(restmexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok(Arc::new(MatchingExp::LIST_CONS_MATCH { head: mexp.clone(), rest: restmexp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::STRING_MATCH { .. }, mtype, astDefs) => {
                    ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::STRING_TYPE { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(mexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::LITERAL_MATCH { litType: ot, .. }, mtype, astDefs) => {
                    typesEqualConcrete(deAliasedType(ot.clone(), astDefs.clone())?, deAliasedType(mtype.clone(), astDefs.clone())?, astDefs.clone())?;
                    Ok(mexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::REST_MATCH { .. }, _, _) => {
                    Ok(mexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("Error - typeCheckMatchingExp failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTransformedMatchingExp)
}

pub fn typeCheckMatchingExpRecord(mut inFieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>, mut fields: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>> {
    let mut outTransformedMatchingExp: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>> = metamodelica::nil();
    outTransformedMatchingExp = 'mc: {
        let __mc_input = (inFieldMatchings.clone(), fields.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ident, mexp), tail: fms }, _, astDefs) => {
                    let mut mtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    let mut fms = (*fms).clone();
                    mtype = lookupTupleList(fields.clone(), (ident.clone()).clone())?;
                    mexp = typeCheckMatchingExp(mexp.clone(), mtype.clone(), astDefs.clone())?;
                    fms = typeCheckMatchingExpRecord(fms.clone(), fields.clone(), astDefs.clone())?;
                    Ok(cons((ident.clone(), mexp.clone()), fms.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ident, _), tail: _ }, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(fields.clone(), (ident.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - typeCheckMatchingExpRecord failed to find field '")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("'\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTransformedMatchingExp)
}

pub fn typeCheckMatchingExpList(mut inMatchingExpLst: Arc<metamodelica::List<Arc<MatchingExp>>>, mut inTypeLst: Arc<metamodelica::List<Arc<TypeSignature>>>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<Arc<metamodelica::List<Arc<MatchingExp>>>> {
    let mut outTransformedMatchingExp: Arc<metamodelica::List<Arc<MatchingExp>>> = metamodelica::nil();
    outTransformedMatchingExp = (::match_deref::match_deref! { match &((inMatchingExpLst.clone(), inTypeLst.clone(), inASTDefs.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: mexp, tail: mexpLst }, Deref @ metamodelica::List::Cons { head: mtype, tail: tsLst }, astDefs) => {
            let mut mexp = (*mexp).clone();
            let mut mexpLst = (*mexpLst).clone();
            mexp = typeCheckMatchingExp(mexp.clone(), mtype.clone(), astDefs.clone())?;
            mexpLst = typeCheckMatchingExpList(mexpLst.clone(), tsLst.clone(), astDefs.clone())?;
            cons(mexp.clone(), mexpLst.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Nil, _) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("Error - typeCheckMatchingExpList more expressions to chceck than required (a tuple type has less arguments than provided?).\n")).clone())?;
            bail!("fail")
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("Error - typeCheckMatchingExpList more arguments expected (the tuple type has more arguments than provided).\n")).clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTransformedMatchingExp)
}

pub fn eliminateWildAs(mut inMatchingExp: Arc<MatchingExp>) -> Arc<MatchingExp> {
    let mut outRewrittenMatchingExp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    outRewrittenMatchingExp = (::match_deref::match_deref! { match &(inMatchingExp.clone()) {
        Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, matchingExp: Deref @ MatchingExp::REST_MATCH { .. } } => {
            Arc::new(MatchingExp::BIND_MATCH { bindIdent: (bid.clone()).clone() })
        },
        _ => {
            inMatchingExp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outRewrittenMatchingExp
}

pub fn rewriteMatchExpByLocalNames(mut inMatchingExp: Arc<MatchingExp>, mut inMType: Arc<TypeSignature>, mut inLocalNames: Arc<metamodelica::List<(ArcStr, ArcStr)>>, mut inUsedLocals: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<MatchingExp>, TypedIdents)> {
    let mut outRewrittenMatchingExp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    let mut outUsedLocals: TypedIdents = metamodelica::nil();
    (outRewrittenMatchingExp, outUsedLocals) = 'mc: {
        let __mc_input = (inMatchingExp.clone(), inMType.clone(), inLocalNames.clone(), inUsedLocals.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_AS_MATCH { matchingExp: mexp, bindIdent: bid }, mtype, _, usedLocals, astDefs) => {
                    let mut localIdent: Ident = arcstr::literal!("");
                    let mut mexp = (*mexp).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    localIdent = (lookupTupleList(inLocalNames.clone(), (bid.clone()).clone())?).clone();
                    usedLocals = addLocalValue((bid.clone()).clone(), mtype.clone(), usedLocals.clone())?;
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    mexp = eliminateWildAs(Arc::new(MatchingExp::BIND_AS_MATCH { bindIdent: (localIdent.clone()).clone(), matchingExp: mexp.clone() }));
                    Ok((mexp.clone(), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_AS_MATCH { matchingExp: mexp, bindIdent: bid }, mtype, _, usedLocals, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(inLocalNames.clone(), (bid.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((mexp.clone(), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }, mtype, _, usedLocals, _) => {
                    let mut localIdent: Ident = arcstr::literal!("");
                    let mut usedLocals = (*usedLocals).clone();
                    localIdent = (lookupTupleList(inLocalNames.clone(), (bid.clone()).clone())?).clone();
                    usedLocals = addLocalValue((bid.clone()).clone(), mtype.clone(), usedLocals.clone())?;
                    Ok((Arc::new(MatchingExp::BIND_MATCH { bindIdent: (localIdent.clone()).clone() }), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }, _, _, usedLocals, _) => {
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(inLocalNames.clone(), (bid.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::RECORD_MATCH { fieldMatchings: Deref @ metamodelica::List::Nil, tagName: tagpath }, mtype, _, usedLocals, astDefs) => {
                    let mut fldId: Ident = arcstr::literal!("");
                    let mut tagpath = (*tagpath).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getFieldsForRecord(mtype.clone(), tagpath.clone(), astDefs.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: (__pa0, _), tail: _ }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    fldId = __pa0.clone();
                    tagpath = __pa1.clone();
                    Ok((Arc::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: list![(fldId.clone(), Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH))] }), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::RECORD_MATCH { fieldMatchings: fms, tagName: tagpath }, mtype, _, usedLocals, astDefs) => {
                    let mut fields: TypedIdents = metamodelica::nil();
                    let mut fms = (*fms).clone();
                    let mut tagpath = (*tagpath).clone();
                    let mut mtype = (*mtype).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    (fields, tagpath) = getFieldsForRecord(mtype.clone(), tagpath.clone(), astDefs.clone())?;
                    (fms, usedLocals) = rewriteMatchExpByLocalNamesRecord(fms.clone(), fields.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((Arc::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() }), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::SOME_MATCH { value: mexp }, mtype, _, usedLocals, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    let mut mtype = (*mtype).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::OPTION_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    mtype = __pa0.clone();
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((Arc::new(MatchingExp::SOME_MATCH { value: mexp.clone() }), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst }, mtype, _, usedLocals, astDefs) => {
                    let mut otLst: Arc<metamodelica::List<Arc<TypeSignature>>> = metamodelica::nil();
                    let mut mexpLst = (*mexpLst).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::TUPLE_TYPE { ofTypes: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    otLst = __pa0.clone();
                    (mexpLst, usedLocals) = rewriteMatchExpByLocalNamesList(mexpLst.clone(), otLst.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((Arc::new(MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst.clone() }), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::LIST_MATCH { listElts: mexpLst }, mtype, _, usedLocals, astDefs) => {
                    let mut ot: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut otLst: Arc<metamodelica::List<Arc<TypeSignature>>> = metamodelica::nil();
                    let mut mexpLst = (*mexpLst).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ot = __pa0.clone();
                    otLst = List::fill(ot.clone(), (mexpLst.clone().len() as i32));
                    (mexpLst, usedLocals) = rewriteMatchExpByLocalNamesList(mexpLst.clone(), otLst.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((Arc::new(MatchingExp::LIST_MATCH { listElts: mexpLst.clone() }), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::LIST_CONS_MATCH { rest: restmexp, head: mexp }, mtype, _, usedLocals, astDefs) => {
                    let mut ot: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut restmexp = (*restmexp).clone();
                    let mut mexp = (*mexp).clone();
                    let mut mtype = (*mtype).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(mtype.clone()) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    ot = __pa0.clone();
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), ot.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    (restmexp, usedLocals) = rewriteMatchExpByLocalNames(restmexp.clone(), mtype.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((Arc::new(MatchingExp::LIST_CONS_MATCH { head: mexp.clone(), rest: restmexp.clone() }), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp, _, _, usedLocals, _) => {
                    Ok((mexp.clone(), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRewrittenMatchingExp, outUsedLocals))
}

pub fn rewriteMatchExpByLocalNamesRecord(mut inFieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>, mut fields: TypedIdents, mut inLocalNames: Arc<metamodelica::List<(ArcStr, ArcStr)>>, mut inUsedLocals: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>, TypedIdents)> {
    let mut outRewrittenMatchingExp: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>> = metamodelica::nil();
    let mut outUsedLocals: TypedIdents = metamodelica::nil();
    (outRewrittenMatchingExp, outUsedLocals) = 'mc: {
        let __mc_input = (inFieldMatchings.clone(), fields.clone(), inLocalNames.clone(), inUsedLocals.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _, _) => {
                    Ok((metamodelica::nil(), inUsedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ident, mexp), tail: fms }, _, _, usedLocals, astDefs) => {
                    let mut mtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    let mut fms = (*fms).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    mtype = lookupTupleList(fields.clone(), (ident.clone()).clone())?;
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    (fms, usedLocals) = rewriteMatchExpByLocalNamesRecord(fms.clone(), fields.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((cons((ident.clone(), mexp.clone()), fms.clone()), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (ident, mexp), tail: fms }, _, _, usedLocals, astDefs) => {
                    let mut fms = (*fms).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(fields.clone(), (ident.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - rewriteMatchExpByLocalNamesRecord failed to find field '")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("'\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (fms, usedLocals) = rewriteMatchExpByLocalNamesRecord(fms.clone(), fields.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((cons((ident.clone(), mexp.clone()), fms.clone()), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!rewriteMatchExpByLocalNamesRecord failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRewrittenMatchingExp, outUsedLocals))
}

pub fn rewriteMatchExpByLocalNamesList(mut inMatchingExpLst: Arc<metamodelica::List<Arc<MatchingExp>>>, mut inTypeLst: Arc<metamodelica::List<Arc<TypeSignature>>>, mut inLocalNames: Arc<metamodelica::List<(ArcStr, ArcStr)>>, mut inUsedLocals: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<metamodelica::List<Arc<MatchingExp>>>, TypedIdents)> {
    let mut outRewrittenMatchingExp: Arc<metamodelica::List<Arc<MatchingExp>>> = metamodelica::nil();
    let mut outUsedLocals: TypedIdents = metamodelica::nil();
    (outRewrittenMatchingExp, outUsedLocals) = 'mc: {
        let __mc_input = (inMatchingExpLst.clone(), inTypeLst.clone(), inLocalNames.clone(), inUsedLocals.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, usedLocals, _) => {
                    Ok((metamodelica::nil(), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: mexp, tail: mexpLst }, Deref @ metamodelica::List::Cons { head: mtype, tail: tsLst }, _, usedLocals, astDefs) => {
                    let mut mexp = (*mexp).clone();
                    let mut mexpLst = (*mexpLst).clone();
                    let mut usedLocals = (*usedLocals).clone();
                    (mexp, usedLocals) = rewriteMatchExpByLocalNames(mexp.clone(), mtype.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    (mexpLst, usedLocals) = rewriteMatchExpByLocalNamesList(mexpLst.clone(), tsLst.clone(), inLocalNames.clone(), usedLocals.clone(), astDefs.clone())?;
                    Ok((cons(mexp.clone(), mexpLst.clone()), usedLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!localsFromMatchExpList failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outRewrittenMatchingExp, outUsedLocals))
}

pub fn addLocalValue(mut inIdent: Ident, mut inMType: Arc<TypeSignature>, mut inLocals: TypedIdents) -> Result<TypedIdents> {
    let mut outLocals: TypedIdents = metamodelica::nil();
    outLocals = 'mc: {
        let __mc_input = (inIdent.clone(), inMType.clone(), inLocals.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, Deref @ TypeSignature::TEXT_TYPE { .. }, locals) => {
                    let true = (stringEq((ident.clone()).clone(), (arcstr::literal!(emptyTxt)).clone())) else { bail!("pattern mismatch") };
                    Ok(locals.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, mtype, locals) => {
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(locals.clone(), (ident.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(cons((ident.clone(), mtype.clone()), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, mtype, locals) => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    let _ = lookupTupleList(locals.clone(), (ident.clone()).clone())?;
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("A duplicite identifier '")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("' bound in a matching expression.")); ArcStr::from(__mm_s) }).clone();
                    addSusanError((msg.clone()).clone(), dummySourceInfo.clone())?;
                    Ok(cons((ident.clone(), mtype.clone()), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outLocals)
}

pub fn makeMMMatchCase(mut inElabCase: (Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>), mut inExtraArgs: TypedIdents, mut inOutArgs: TypedIdents) -> Result<MMMatchCase> {
    let mut outMMMCase: MMMatchCase;
    outMMMCase = 'mc: {
        let __mc_input = (inElabCase.clone(), inExtraArgs.clone(), inOutArgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((mexp, caseargs, stmts), extargs, oargs) => {
                    let mut mmmcase: MMMatchCase;
                    let mut mexpLst: Arc<metamodelica::List<Arc<MatchingExp>>> = metamodelica::nil();
                    mexpLst = List::map2(extargs.clone(), (std::sync::Arc::new(makeExtraArgBinding) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>), Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>) -> Result<Arc<MatchingExp>> + 'static>), caseargs.clone(), oargs.clone());
                    mmmcase = (cons(imlicitTxtMExp.clone(), cons(mexp.clone(), mexpLst.clone())), stmts.clone());
                    Ok(mmmcase.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!makeMMMatchCase failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMMMCase)
}

pub fn makeExtraArgBinding(mut inExtraArg: (ArcStr, Arc<TypeSignature>), mut inCaseArgs: TypedIdents, mut inOutArgs: TypedIdents) -> Result<Arc<MatchingExp>> {
    let mut outExtraArgBinding: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    outExtraArgBinding = 'mc: {
        let __mc_input = (inExtraArg.clone(), inCaseArgs.clone(), inOutArgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((argname, _), _, oargs) => {
                    let _ = lookupTupleList(oargs.clone(), (argname.clone()).clone())?;
                    Ok(Arc::new(MatchingExp::BIND_MATCH { bindIdent: (argname.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((argname, _), caseargs, _) => {
                    let _ = lookupTupleList(caseargs.clone(), (argname.clone()).clone())?;
                    Ok(Arc::new(MatchingExp::BIND_MATCH { bindIdent: (argname.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!makeExtraArgBinding failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExtraArgBinding)
}

pub fn addRestElabCase(mut inElabCases: Arc<metamodelica::List<(Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>)>>) -> Result<Arc<metamodelica::List<(Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>)>>> {
    let mut outElabCases: Arc<metamodelica::List<(Arc<MatchingExp>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<metamodelica::List<Arc<MMExp>>>)>> = metamodelica::nil();
    outElabCases = 'mc: {
        let __mc_input = inElabCases.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(list![(Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH), metamodelica::nil(), metamodelica::nil())])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                restcases @ Deref @ metamodelica::List::Cons { head: (mexp, _, _), tail: _ } => {
                    isAlwaysMatched(mexp.clone())?;
                    Ok(restcases.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: elabcase @ (_, _, _), tail: restcases } => {
                    let mut restcases = (*restcases).clone();
                    restcases = addRestElabCase(restcases.clone())?;
                    Ok(cons(elabcase.clone(), restcases.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!addRestElabCase failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElabCases)
}

pub fn isAlwaysMatched(mut inMatchingExp: Arc<MatchingExp>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inMatchingExp.clone()) {
        Deref @ MatchingExp::BIND_AS_MATCH { matchingExp: mexp, .. } => {
            isAlwaysMatched(mexp.clone())?;
            ()
        },
        Deref @ MatchingExp::BIND_MATCH { .. } => {
            ()
        },
        Deref @ MatchingExp::TUPLE_MATCH { tupleArgs: mexplst } => {
            List::map_0(mexplst.clone(), (std::sync::Arc::new(isAlwaysMatched) as std::sync::Arc<dyn ::std::ops::Fn(Arc<MatchingExp>) -> Result<()> + 'static>));
            ()
        },
        Deref @ MatchingExp::REST_MATCH { .. } => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub fn isAlwaysMatchedBool(mut inMatchingExp: Arc<MatchingExp>) -> Result<bool> {
    let mut isAlwaysMatched: bool = false;
    isAlwaysMatched = 'mc: {
        let __mc_input = inMatchingExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                mexp => {
                    self::isAlwaysMatched(mexp.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isAlwaysMatched)
}

pub fn adaptTextToString(mut inArgValue: (Arc<MMExp>, Arc<TypeSignature>, SourceInfo), mut inArgExp: Expression, mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>, mut inLocals: TypedIdents, mut inTplPackage: TemplPackage) -> Result<((Arc<MMExp>, Arc<TypeSignature>, SourceInfo), Expression, Arc<metamodelica::List<Arc<MMExp>>>, TypedIdents)> {
    let mut outArgValue: (Arc<MMExp>, Arc<TypeSignature>, SourceInfo);
    let mut outArgExp: Expression;
    let mut outStmts: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    (outArgValue, outArgExp, outStmts, outLocals) = 'mc: {
        let __mc_input = (inArgValue.clone(), inArgExp.clone(), inStmts.clone(), inLocals.clone(), inTplPackage.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((mmexp, exptype, sinfo), _, stmts, locals, TemplPackage { astDefs: astdefs, .. }) => {
                    let mut stmt: Arc<MMExp>;
                    let mut strid: Ident = arcstr::literal!("");
                    let mut mmexp = (*mmexp).clone();
                    let mut locals = (*locals).clone();
                    ::match_deref::match_deref! { match &(deAliasedType(exptype.clone(), astdefs.clone())?) {
                        Deref @ TypeSignature::TEXT_TYPE { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    strid = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(textToStringNamePrefix)); __mm_s.push_str(&*intString((locals.clone().len() as i32))); ArcStr::from(__mm_s) }).clone();
                    locals = addLocalValue((strid.clone()).clone(), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE), locals.clone())?;
                    mmexp = mmExpToString(mmexp.clone(), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE), sinfo.clone())?;
                    stmt = Arc::new(MMExp::MM_ASSIGN { lhsArgs: list![(strid.clone()).clone()], rhs: mmexp.clone() });
                    Ok(((Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (strid.clone()).clone() }) }), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE), sinfo.clone()), emptyExpression.clone(), cons(stmt.clone(), stmts.clone()), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (argval, _, stmts, locals, _) => {
                    Ok((argval.clone(), inArgExp.clone(), stmts.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!adaptTextToString failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outArgValue, outArgExp, outStmts, outLocals))
}

pub fn elabCasesFromCondition(mut inArgType: Arc<TypeSignature>, mut inIsNot: bool, mut inRhsValue: Option<Arc<MatchingExp>>, mut inTrueBranch: Expression, mut inElseBranchOpt: Option<(Arc<ExpressionBase>, SourceInfo)>, mut inTplPackage: TemplPackage) -> Result<Arc<metamodelica::List<(Arc<MatchingExp>, (Arc<ExpressionBase>, SourceInfo))>>> {
    let mut outMCases: Arc<metamodelica::List<(Arc<MatchingExp>, (Arc<ExpressionBase>, SourceInfo))>> = metamodelica::nil();
    outMCases = 'mc: {
        let __mc_input = (inArgType.clone(), inIsNot.clone(), inRhsValue.clone(), inTrueBranch.clone(), inElseBranchOpt.clone(), inTplPackage.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { .. }, isnot, None, tbranch, ebranchOpt, _) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), Arc::new(MatchingExp::LIST_MATCH { listElts: metamodelica::nil() }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { .. }, isnot, None, tbranch, ebranchOpt, _) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), Arc::new(crate::TplAbsyn::MatchingExp::NONE_MATCH), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::STRING_TYPE { .. }, isnot, None, tbranch, ebranchOpt, _) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), Arc::new(MatchingExp::STRING_MATCH { value: (literal!("")).clone() }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::INTEGER_TYPE { .. }, isnot, None, tbranch, ebranchOpt, _) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), Arc::new(MatchingExp::LITERAL_MATCH { value: (literal!("0")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE) }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::REAL_TYPE { .. }, isnot, None, tbranch, ebranchOpt, _) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), Arc::new(MatchingExp::LITERAL_MATCH { value: (literal!("0.0")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::REAL_TYPE) }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::BOOLEAN_TYPE { .. }, isnot, None, tbranch, ebranchOpt, _) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), Arc::new(MatchingExp::LITERAL_MATCH { value: (literal!("false")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::BOOLEAN_TYPE) }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TEXT_TYPE { .. }, isnot, None, tbranch, ebranchOpt, _) => {
                    Ok(casesForTrueFalseCondition(isnot.clone(), Arc::new(MatchingExp::RECORD_MATCH { tagName: Arc::new(PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(PathIdent::IDENT { ident: (literal!("MEM_TEXT")).clone() }) }), fieldMatchings: list![(literal!("tokens"), Arc::new(MatchingExp::LIST_MATCH { listElts: metamodelica::nil() }))] }), tbranch.clone(), ebranchOpt.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!elabCasesFromCondition failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMCases)
}

pub fn casesForTrueFalseCondition(mut inIsNot: bool, mut inNotMatchingExp: Arc<MatchingExp>, mut inTrueBranch: Expression, mut inElseBranchOpt: Option<(Arc<ExpressionBase>, SourceInfo)>) -> Result<Arc<metamodelica::List<(Arc<MatchingExp>, (Arc<ExpressionBase>, SourceInfo))>>> {
    let mut outMCases: Arc<metamodelica::List<(Arc<MatchingExp>, (Arc<ExpressionBase>, SourceInfo))>> = metamodelica::nil();
    outMCases = 'mc: {
        let __mc_input = (inIsNot.clone(), inNotMatchingExp.clone(), inTrueBranch.clone(), inElseBranchOpt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, notmexp, tbranch, ebranchOpt) => {
                    let mut ebranch: Expression;
                    ebranch = getElseBranch(ebranchOpt.clone())?;
                    Ok(list![(notmexp.clone(), ebranch.clone()), (Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH), tbranch.clone())])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, notmexp, tbranch, ebranchOpt) => {
                    let mut ebranch: Expression;
                    ebranch = getElseBranch(ebranchOpt.clone())?;
                    Ok(list![(notmexp.clone(), tbranch.clone()), (Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH), ebranch.clone())])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!casesForTrueFalseCondition failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMCases)
}

pub fn getElseBranch(mut inElseBranchOpt: Option<(Arc<ExpressionBase>, SourceInfo)>) -> Result<Expression> {
    let mut outElseBranch: Expression;
    outElseBranch = (::match_deref::match_deref! { match &(inElseBranchOpt.clone()) {
        Some(ebranch) => {
            ebranch.clone()
        },
        None => {
            emptyExpression.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!getElseBranch failed\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElseBranch)
}

//does not fail, when not resolved ... UNRESOLVED_TYPE() is returned
pub fn resolveBoundPath(mut inPath: Arc<PathIdent>, mut inScopeEnv: ScopeEnv, mut inTplPackage: TemplPackage) -> Result<(Arc<MMExp>, Arc<TypeSignature>, ScopeEnv)> {
    let mut outMMExp: Arc<MMExp>;
    let mut outType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    (outMMExp, outType, outScopeEnv) = 'mc: {
        let __mc_input = (inPath.clone(), inScopeEnv.clone(), inTplPackage.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, scEnv, TemplPackage { astDefs, .. }) => {
                    let mut ident: Ident = arcstr::literal!("");
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut scEnv = (*scEnv).clone();
                    ident = (pathIdentString(path.clone())?).clone();
                    (ident, idtype, scEnv) = resolvePathInScopeEnv((ident.clone()).clone(), path.clone(), true, scEnv.clone(), astDefs.clone())?;
                    Ok((Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (ident.clone()).clone() }) }), idtype.clone(), scEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ PathIdent::IDENT { ident }, scEnv, TemplPackage { templateDefs: tpldefs, .. }) => {
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut tpldef: TemplateDef;
                    let mut mmexp: Arc<MMExp>;
                    tpldef = lookupTupleList(tpldefs.clone(), (ident.clone()).clone())?;
                    (mmexp, idtype) = makeMMExpFromTemplateConstant(tpldef.clone(), (ident.clone()).clone())?;
                    Ok((mmexp.clone(), idtype.clone(), scEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, scEnv, TemplPackage { astDefs, .. }) => {
                    let mut typepckg: Arc<PathIdent>;
                    let mut typeident: Ident = arcstr::literal!("");
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut typepckgOpt: Option<Arc<PathIdent>> = None;
                    let mut path = (*path).clone();
                    (typepckgOpt, typeident) = splitPackageAndIdent(path.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getTypeInfo(typepckgOpt.clone(), (typeident.clone()).clone(), astDefs.clone())?) {
                        (__pa0, TypeInfo::TI_CONST_TYPE { constType: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    typepckg = __pa0.clone();
                    idtype = __pa1.clone();
                    path = makePathIdent(typepckg.clone(), (typeident.clone()).clone())?;
                    Ok((Arc::new(MMExp::MM_IDENT { ident: path.clone() }), idtype.clone(), scEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, scEnv, TemplPackage { astDefs, .. }) => {
                    let mut typepckg: Arc<PathIdent>;
                    let mut typeident: Ident = arcstr::literal!("");
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut typepckgOpt: Option<Arc<PathIdent>> = None;
                    let mut reason: ArcStr = arcstr::literal!("");
                    let mut path = (*path).clone();
                    (typepckgOpt, typeident) = splitPackageAndIdent(path.clone())?;
                    (typepckg, _) = getTypeInfo(typepckgOpt.clone(), (typeident.clone()).clone(), astDefs.clone())?;
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path - imported symbol '")); __mm_s.push_str(&*pathIdentString(path.clone())?); __mm_s.push_str(&*literal!("' other than a constant used in a value context (missing parenthesis ?).")); ArcStr::from(__mm_s) }).clone();
                    idtype = Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (reason.clone()).clone() });
                    path = makePathIdent(typepckg.clone(), (typeident.clone()).clone())?;
                    Ok((Arc::new(MMExp::MM_IDENT { ident: path.clone() }), idtype.clone(), scEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, scEnv, _) => {
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut reason: ArcStr = arcstr::literal!("");
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path '")); __mm_s.push_str(&*pathIdentString(path.clone())?); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) }).clone();
                    idtype = Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (reason.clone()).clone() });
                    Ok((Arc::new(MMExp::MM_IDENT { ident: path.clone() }), idtype.clone(), scEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!resolveBoundPath failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMExp, outType, outScopeEnv))
}

pub fn checkResolvedType(mut inPath: Arc<PathIdent>, mut inType: Arc<TypeSignature>, mut inUnresolvedMsg: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inPath.clone(), inType.clone(), inUnresolvedMsg.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ TypeSignature::UNRESOLVED_TYPE { reason }, msg, _) => {
                    let mut msg = (*msg).clone();
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*reason.clone()); ArcStr::from(__mm_s) }).clone();
                    addSusanError((msg.clone()).clone(), inInfo.clone())?;
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

pub fn checkTextType(mut inType: Arc<TypeSignature>, mut inIdent: Ident, mut inUnresolvedMsg: ArcStr, mut inInfo: SourceInfo) -> Result<Arc<TypeSignature>> {
    let mut outType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    outType = 'mc: {
        let __mc_input = (inType.clone(), inIdent.clone(), inUnresolvedMsg.clone(), inInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TEXT_TYPE { .. }, _, _, _) => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::UNRESOLVED_TYPE { .. }, _, _, _) => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ts, _, msg, _) => {
                    let mut msg = (*msg).clone();
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*msg.clone()); __mm_s.push_str(&*literal!(") identifier '")); __mm_s.push_str(&*inIdent.clone()); __mm_s.push_str(&*literal!("' was expected to have Text& type but resolved to ")); __mm_s.push_str(&*typeSignatureString(ts.clone())?); __mm_s.push_str(&*literal!(".\n Only Text& typed variables can be appended to.")); ArcStr::from(__mm_s) }).clone();
                    addSusanError((msg.clone()).clone(), inInfo.clone())?;
                    Ok(Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (msg.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

pub fn makeMMExpFromTemplateConstant(mut inTplDef: TemplateDef, mut inTemplIdent: Ident) -> Result<(Arc<MMExp>, Arc<TypeSignature>)> {
    let mut outMMExp: Arc<MMExp>;
    let mut outConstType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    (outMMExp, outConstType) = 'mc: {
        let __mc_input = (inTplDef.clone(), inTemplIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (TemplateDef::STR_TOKEN_DEF { .. }, mut ident) = __mc_input.clone() else { bail!("nomatch") };
            ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(constantNamePrefix)); __mm_s.push_str(&*ident.clone()); ArcStr::from(__mm_s) }).clone();
            Ok((Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (ident.clone()).clone() }) }), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TOKEN_TYPE)))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (TemplateDef::LITERAL_DEF { litType: ref lt, value: mut litstr }, _) = __mc_input.clone() else { bail!("nomatch") };
            Ok((Arc::new(MMExp::MM_LITERAL { value: (litstr.clone()).clone() }), lt.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (TemplateDef::TEMPLATE_DEF { .. }, mut ident) = __mc_input.clone() else { bail!("nomatch") };
            let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
            let mut reason: ArcStr = arcstr::literal!("");
            reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved identifier - the template '")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("'in a value context found (missing parenthesis ?) .")); ArcStr::from(__mm_s) }).clone();
            idtype = Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (reason.clone()).clone() });
            Ok((Arc::new(MMExp::MM_IDENT { ident: Arc::new(PathIdent::IDENT { ident: (ident.clone()).clone() }) }), idtype.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!makeMMExpFromTemplateConstant failed\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outMMExp, outConstType))
}

pub fn prepareMatchArgument(mut inMExp: Arc<MatchingExp>, mut inMatchArgName: Ident) -> Result<(Ident, Arc<MatchingExp>)> {
    let mut outIdent: Ident = arcstr::literal!("");
    let mut outMExp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    (outIdent, outMExp) = 'mc: {
        let __mc_input = (inMExp.clone(), inMatchArgName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::BIND_MATCH { bindIdent: ident }, _) => {
                    Ok((ident.clone(), mexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mexp @ Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: ident, .. }, _) => {
                    Ok((ident.clone(), mexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ MatchingExp::REST_MATCH { .. }, _) => {
                    Ok((inMatchArgName.clone(), Arc::new(MatchingExp::BIND_MATCH { bindIdent: (inMatchArgName.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inMatchArgName.clone(), Arc::new(MatchingExp::BIND_AS_MATCH { bindIdent: (inMatchArgName.clone()).clone(), matchingExp: inMExp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outIdent, outMExp))
}

pub fn resolvePathInScopeEnv(mut inIdent: Ident, mut inPath: Arc<PathIdent>, mut canDoImplicitLookup: bool, mut inScopeEnv: ScopeEnv, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Ident, Arc<TypeSignature>, ScopeEnv)> {
    let mut outLocalIdent: Ident = arcstr::literal!("");
    let mut outType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut outScopeEnv: ScopeEnv = metamodelica::nil();
    (outLocalIdent, outType, outScopeEnv) = 'mc: {
        let __mc_input = (inIdent.clone(), inPath.clone(), canDoImplicitLookup.clone(), inScopeEnv.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, Deref @ metamodelica::List::Cons { head: Scope::RECURSIVE_SCOPE { recIdent: letIdent, .. }, tail: _ }, _) => {
                    let true = (stringEq((ident.clone()).clone(), (letIdent.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - trying to use '")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("' recursively inside a let scope or text addition. Use an additional Text variable if a self addition/duplication is needed, like  let b = a  let &a += b ... \n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::List::Cons { head: scope @ Scope::RECURSIVE_SCOPE { recIdent: letIdent, .. }, tail: restEnv }, astdefs) => {
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut ident = (*ident).clone();
                    let mut restEnv = (*restEnv).clone();
                    let false = (stringEq((ident.clone()).clone(), (letIdent.clone()).clone())) else { bail!("pattern mismatch") };
                    (ident, idtype, restEnv) = resolvePathInScopeEnv((ident.clone()).clone(), path.clone(), canDoImplicitLookup.clone(), restEnv.clone(), astdefs.clone())?;
                    Ok((ident.clone(), idtype.clone(), cons(scope.clone(), restEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, Deref @ metamodelica::List::Cons { head: Scope::LET_SCOPE { freshIdent, idType: idtype, ident: letIdent, .. }, tail: restEnv }, _) => {
                    let true = (stringEq((ident.clone()).clone(), (letIdent.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((freshIdent.clone(), idtype.clone(), cons(Scope::LET_SCOPE { ident: (letIdent.clone()).clone(), idType: idtype.clone(), freshIdent: (freshIdent.clone()).clone(), isUsed: true }, restEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::List::Cons { head: scope @ Scope::LET_SCOPE { .. }, tail: restEnv }, astdefs) => {
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut ident = (*ident).clone();
                    let mut restEnv = (*restEnv).clone();
                    (ident, idtype, restEnv) = resolvePathInScopeEnv((ident.clone()).clone(), path.clone(), canDoImplicitLookup.clone(), restEnv.clone(), astdefs.clone())?;
                    Ok((ident.clone(), idtype.clone(), cons(scope.clone(), restEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, scEnv @ Deref @ metamodelica::List::Cons { head: Scope::FUN_SCOPE { args: fargs, .. }, tail: _ }, _) => {
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut ident = (*ident).clone();
                    idtype = lookupTupleList(fargs.clone(), (ident.clone()).clone())?;
                    ident = (encodePathIdent(path.clone(), (arcstr::literal!(funArgNamePrefix)).clone())?).clone();
                    Ok((ident.clone(), idtype.clone(), scEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::List::Cons { head: Scope::FUN_SCOPE { localArgs, args: fargs }, tail: restEnv }, astdefs) => {
                    let mut localIdent: Ident = arcstr::literal!("");
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut ident = (*ident).clone();
                    let mut localArgs = (*localArgs).clone();
                    let mut fargs = (*fargs).clone();
                    let mut restEnv = (*restEnv).clone();
                    (localIdent, idtype, restEnv) = resolvePathInScopeEnv((ident.clone()).clone(), path.clone(), canDoImplicitLookup.clone(), restEnv.clone(), astdefs.clone())?;
                    fargs = cons((ident.clone(), idtype.clone()), fargs.clone());
                    localArgs = cons((localIdent.clone(), idtype.clone()), localArgs.clone());
                    ident = (encodeIdent((ident.clone()).clone(), (arcstr::literal!(funArgNamePrefix)).clone())?).clone();
                    Ok((ident.clone(), idtype.clone(), cons(Scope::FUN_SCOPE { args: fargs.clone(), localArgs: localArgs.clone() }, restEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::List::Cons { head: Scope::CASE_SCOPE { hasImplicitScope, matchArgName, extArgs: extargs, accLocals, localNames, mType: mtype, mExp: mexp }, tail: restEnv }, astdefs) => {
                    let mut encident: Ident = arcstr::literal!("");
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut accLocals = (*accLocals).clone();
                    let mut localNames = (*localNames).clone();
                    let mut mexp = (*mexp).clone();
                    (idtype, mexp) = lookupUpdateMatchingExp((ident.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astdefs.clone())?;
                    encident = (encodeIdent((ident.clone()).clone(), (arcstr::literal!(caseBindingNamePrefix)).clone())?).clone();
                    (encident, localNames, accLocals) = updateLocalsForMatchingExp((ident.clone()).clone(), (encident.clone()).clone(), 0, idtype.clone(), localNames.clone(), accLocals.clone())?;
                    Ok((encident.clone(), idtype.clone(), cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: mtype.clone(), localNames: localNames.clone(), accLocals: accLocals.clone(), extArgs: extargs.clone(), matchArgName: (matchArgName.clone()).clone(), hasImplicitScope: hasImplicitScope.clone() }, restEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, true, Deref @ metamodelica::List::Cons { head: Scope::CASE_SCOPE { hasImplicitScope: true, matchArgName, extArgs: extargs, accLocals, localNames, mType: mtype, mExp: mexp }, tail: restEnv }, astdefs) => {
                    let mut encident: Ident = arcstr::literal!("");
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut accLocals = (*accLocals).clone();
                    let mut localNames = (*localNames).clone();
                    let mut mexp = (*mexp).clone();
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n trying [it.]path for '")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!(" / ")); __mm_s.push_str(&*pathIdentString(path.clone())?); __mm_s.push_str(&*literal!("' : ")); __mm_s.push_str(&*typeSignatureString(mtype.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (idtype, mexp) = lookupUpdateMExpDotPath((ident.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astdefs.clone())?;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(idtype.clone()) {
                            Deref @ TypeSignature::UNRESOLVED_TYPE { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n [it.]path for '")); __mm_s.push_str(&*pathIdentString(path.clone())?); __mm_s.push_str(&*literal!("' : ")); __mm_s.push_str(&*typeSignatureString(idtype.clone())?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    encident = (encodePathIdent(path.clone(), (arcstr::literal!(caseBindingNamePrefix)).clone())?).clone();
                    (encident, localNames, accLocals) = updateLocalsForMatchingExp((ident.clone()).clone(), (encident.clone()).clone(), 0, idtype.clone(), localNames.clone(), accLocals.clone())?;
                    Ok((encident.clone(), idtype.clone(), cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: mtype.clone(), localNames: localNames.clone(), accLocals: accLocals.clone(), extArgs: extargs.clone(), matchArgName: (matchArgName.clone()).clone(), hasImplicitScope: true }, restEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, Deref @ metamodelica::List::Cons { head: Scope::CASE_SCOPE { hasImplicitScope, matchArgName, extArgs: extargs, accLocals, localNames, mType: mtype, mExp: mexp }, tail: restEnv }, _) => {
                    let mut encident: Ident = arcstr::literal!("");
                    let mut ident = (*ident).clone();
                    let mut accLocals = (*accLocals).clone();
                    let mut localNames = (*localNames).clone();
                    let mut mexp = (*mexp).clone();
                    let true = (stringEq((ident.clone()).clone(), (matchArgName.clone()).clone())) else { bail!("pattern mismatch") };
                    (ident, mexp) = prepareMatchArgument(mexp.clone(), (matchArgName.clone()).clone())?;
                    encident = (encodeIdent((ident.clone()).clone(), (arcstr::literal!(caseBindingNamePrefix)).clone())?).clone();
                    (encident, localNames, accLocals) = updateLocalsForMatchingExp((ident.clone()).clone(), (encident.clone()).clone(), 0, mtype.clone(), localNames.clone(), accLocals.clone())?;
                    Ok((encident.clone(), mtype.clone(), cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: mtype.clone(), localNames: localNames.clone(), accLocals: accLocals.clone(), extArgs: extargs.clone(), matchArgName: (matchArgName.clone()).clone(), hasImplicitScope: hasImplicitScope.clone() }, restEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, path, _, Deref @ metamodelica::List::Cons { head: Scope::CASE_SCOPE { hasImplicitScope, matchArgName, extArgs: extargs, accLocals, localNames, mType: mtype, mExp: mexp }, tail: restEnv }, astdefs) => {
                    let mut encident: Ident = arcstr::literal!("");
                    let mut idtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut extargs = (*extargs).clone();
                    let mut restEnv = (*restEnv).clone();
                    (encident, idtype, restEnv) = resolvePathInScopeEnv((ident.clone()).clone(), path.clone(), canDoImplicitLookup.clone() && !(hasImplicitScope.clone()), restEnv.clone(), astdefs.clone())?;
                    extargs = updateTupleList(extargs.clone(), (encident.clone(), idtype.clone()))?;
                    Ok((encident.clone(), idtype.clone(), cons(Scope::CASE_SCOPE { mExp: mexp.clone(), mType: mtype.clone(), localNames: localNames.clone(), accLocals: accLocals.clone(), extArgs: extargs.clone(), matchArgName: (matchArgName.clone()).clone(), hasImplicitScope: hasImplicitScope.clone() }, restEnv.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outLocalIdent, outType, outScopeEnv))
}

pub fn addPostfixToIdent(mut inIdent: Ident, mut inPostfix: i32) -> Ident {
    let mut outPostfixedIdent: Ident = arcstr::literal!("");
    outPostfixedIdent = ((match (inIdent.clone(), inPostfix.clone()) {
        (_, 0) => {
            inIdent.clone()
        },
        (mut ident, _) => {
            ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(inPostfix.clone())); ArcStr::from(__mm_s) }).clone();
            ident.clone()
        },
    })).clone();
    outPostfixedIdent
}

pub fn updateLocalsForMatchingExp(mut inIdent: Ident, mut inEncIdent: Ident, mut inPostfix: i32, mut inType: Arc<TypeSignature>, mut inLocalNames: Arc<metamodelica::List<(ArcStr, ArcStr)>>, mut inLocals: TypedIdents) -> Result<(Ident, Arc<metamodelica::List<(ArcStr, ArcStr)>>, TypedIdents)> {
    let mut outLocalIdent: Ident = arcstr::literal!("");
    let mut outLocalNames: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut outLocals: TypedIdents = metamodelica::nil();
    (outLocalIdent, outLocalNames, outLocals) = 'mc: {
        let __mc_input = (inIdent.clone(), inEncIdent.clone(), inPostfix.clone(), inType.clone(), inLocalNames.clone(), inLocals.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, _, localNames, locals) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    encIdent = (lookupTupleList(localNames.clone(), (ident.clone()).clone())?).clone();
                    Ok((encIdent.clone(), localNames.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, _, localNames, locals) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    encIdent = (addPostfixToIdent((inEncIdent.clone()).clone(), inPostfix.clone())).clone();
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(locals.clone(), (encIdent.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((encIdent.clone(), cons((ident.clone(), encIdent.clone()), localNames.clone()), cons((encIdent.clone(), inType.clone()), locals.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, _, localNames, locals) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    let mut loctype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    encIdent = (addPostfixToIdent((inEncIdent.clone()).clone(), inPostfix.clone())).clone();
                    loctype = lookupTupleList(locals.clone(), (encIdent.clone()).clone())?;
                    let true = (loctype.clone() == inType.clone()) else { bail!("pattern mismatch") };
                    Ok((encIdent.clone(), cons((ident.clone(), encIdent.clone()), localNames.clone()), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ident, _, _, _, localNames, locals) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    let mut loctype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut localNames = (*localNames).clone();
                    let mut locals = (*locals).clone();
                    encIdent = (addPostfixToIdent((inEncIdent.clone()).clone(), inPostfix.clone())).clone();
                    loctype = lookupTupleList(locals.clone(), (encIdent.clone()).clone())?;
                    let false = (loctype.clone() == inType.clone()) else { bail!("pattern mismatch") };
                    (encIdent, localNames, locals) = updateLocalsForMatchingExp((ident.clone()).clone(), (inEncIdent.clone()).clone(), inPostfix.clone() + 1, inType.clone(), localNames.clone(), locals.clone())?;
                    Ok((encIdent.clone(), localNames.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!updateLocalsForMatchingExp failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outLocalIdent, outLocalNames, outLocals))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn usedInImmediateLetScope(mut inIdent: Ident, mut inFreshIdent: Ident, mut inScopeEnv: ScopeEnv) -> Result<bool> {
    let mut outIsUsed: bool = false;
    outIsUsed = 'mc: {
        let __mc_input = (inIdent.clone(), inFreshIdent.clone(), inScopeEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: Scope::LET_SCOPE { freshIdent, ident: letIdent, .. }, tail: _ }) => {
                    let true = (stringEq((inIdent.clone()).clone(), (letIdent.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (stringEq((inFreshIdent.clone()).clone(), (freshIdent.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: Scope::LET_SCOPE { .. }, tail: restEnv }) => {
                    Ok(usedInImmediateLetScope((inIdent.clone()).clone(), (inFreshIdent.clone()).clone(), restEnv.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: Scope::RECURSIVE_SCOPE { freshIdent, recIdent: letIdent }, tail: _ }) => {
                    let true = (stringEq((inIdent.clone()).clone(), (letIdent.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (stringEq((inFreshIdent.clone()).clone(), (freshIdent.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: Scope::RECURSIVE_SCOPE { .. }, tail: restEnv }) => {
                    Ok(usedInImmediateLetScope((inIdent.clone()).clone(), (inFreshIdent.clone()).clone(), restEnv.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIsUsed)
}

pub fn updateLocalsForLetExp(mut inIdent: Ident, mut inEncIdent: Ident, mut inPostfix: i32, mut inType: Arc<TypeSignature>, mut inLocals: TypedIdents, mut inScopeEnv: ScopeEnv) -> Result<(Ident, TypedIdents)> {
    let mut outLocalIdent: Ident = arcstr::literal!("");
    let mut outLocals: TypedIdents = metamodelica::nil();
    (outLocalIdent, outLocals) = 'mc: {
        let __mc_input = (inIdent.clone(), inEncIdent.clone(), inPostfix.clone(), inType.clone(), inLocals.clone(), inScopeEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    encIdent = (addPostfixToIdent((inEncIdent.clone()).clone(), inPostfix.clone())).clone();
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(inLocals.clone(), (encIdent.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok((encIdent.clone(), cons((encIdent.clone(), inType.clone()), inLocals.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    let mut loctype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut locals: TypedIdents = metamodelica::nil();
                    encIdent = (addPostfixToIdent((inEncIdent.clone()).clone(), inPostfix.clone())).clone();
                    loctype = lookupTupleList(inLocals.clone(), (encIdent.clone()).clone())?;
                    let false = (loctype.clone() == inType.clone()) else { bail!("pattern mismatch") };
                    (encIdent, locals) = updateLocalsForLetExp((inIdent.clone()).clone(), (inEncIdent.clone()).clone(), inPostfix.clone() + 1, inType.clone(), inLocals.clone(), inScopeEnv.clone())?;
                    Ok((encIdent.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    let mut loctype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    encIdent = (addPostfixToIdent((inEncIdent.clone()).clone(), inPostfix.clone())).clone();
                    loctype = lookupTupleList(inLocals.clone(), (encIdent.clone()).clone())?;
                    let true = (loctype.clone() == inType.clone()) else { bail!("pattern mismatch") };
                    let false = (usedInImmediateLetScope((inIdent.clone()).clone(), (encIdent.clone()).clone(), inScopeEnv.clone())?) else { bail!("pattern mismatch") };
                    Ok((encIdent.clone(), inLocals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    let mut encIdent: Ident = arcstr::literal!("");
                    let mut loctype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut locals: TypedIdents = metamodelica::nil();
                    encIdent = (addPostfixToIdent((inEncIdent.clone()).clone(), inPostfix.clone())).clone();
                    loctype = lookupTupleList(inLocals.clone(), (encIdent.clone()).clone())?;
                    let true = (loctype.clone() == inType.clone()) else { bail!("pattern mismatch") };
                    let true = (usedInImmediateLetScope((inIdent.clone()).clone(), (encIdent.clone()).clone(), inScopeEnv.clone())?) else { bail!("pattern mismatch") };
                    (encIdent, locals) = updateLocalsForLetExp((inIdent.clone()).clone(), (inEncIdent.clone()).clone(), inPostfix.clone() + 1, inType.clone(), inLocals.clone(), inScopeEnv.clone())?;
                    Ok((encIdent.clone(), locals.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!updateLocalsForLetExp failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outLocalIdent, outLocals))
}

pub fn lookupUpdateMatchingExp(mut inIdent: Ident, mut inPathIdent: Arc<PathIdent>, mut inMatchingExp: Arc<MatchingExp>, mut inMType: Arc<TypeSignature>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<TypeSignature>, Arc<MatchingExp>)> {
    let mut outValueType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut outMatchingExp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    (outValueType, outMatchingExp) = 'mc: {
        let __mc_input = (inIdent.clone(), inPathIdent.clone(), inMatchingExp.clone(), inMType.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ PathIdent::IDENT { ident: id }, inmexp @ Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, .. }, mtype, _) => {
                    let true = (stringEq((id.clone()).clone(), (bid.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((mtype.clone(), inmexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::PATH_IDENT { path, ident: id }, Deref @ MatchingExp::BIND_AS_MATCH { matchingExp: mexp, bindIdent: bid }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    let true = (stringEq((id.clone()).clone(), (bid.clone()).clone())) else { bail!("pattern mismatch") };
                    (valtype, mexp) = lookupUpdateMExpDotPath((inid.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::BIND_AS_MATCH { bindIdent: (bid.clone()).clone(), matchingExp: mexp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::BIND_AS_MATCH { matchingExp: mexp, bindIdent: bid }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    (valtype, mexp) = lookupUpdateMatchingExp((inid.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::BIND_AS_MATCH { bindIdent: (bid.clone()).clone(), matchingExp: mexp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ PathIdent::IDENT { ident: id }, inmexp @ Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }, mtype, _) => {
                    let true = (stringEq((id.clone()).clone(), (bid.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok((mtype.clone(), inmexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::PATH_IDENT { ident: id, .. }, inmexp @ Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }, _, _) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut reason: ArcStr = arcstr::literal!("");
                    let true = (stringEq((id.clone()).clone(), (bid.clone()).clone())) else { bail!("pattern mismatch") };
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path '")); __mm_s.push_str(&*inid.clone()); __mm_s.push_str(&*literal!("' after first dot - only the first part '")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!("' resolved as a bind match.")); ArcStr::from(__mm_s) }).clone();
                    valtype = Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (reason.clone()).clone() });
                    Ok((valtype.clone(), inmexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::RECORD_MATCH { fieldMatchings: fms, tagName: tagpath }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fields: TypedIdents = metamodelica::nil();
                    let mut fms = (*fms).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    (fields, _) = getFieldsForRecord(mtype.clone(), tagpath.clone(), astDefs.clone())?;
                    (valtype, fms) = lookupUpdateMExpRecord((inid.clone()).clone(), path.clone(), fms.clone(), fields.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::SOME_MATCH { value: mexp }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    let mut mtype = (*mtype).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::OPTION_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    mtype = __pa0.clone();
                    (valtype, mexp) = lookupUpdateMatchingExp((inid.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::SOME_MATCH { value: mexp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mtypeLst: Arc<metamodelica::List<Arc<TypeSignature>>> = metamodelica::nil();
                    let mut mexpLst = (*mexpLst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::TUPLE_TYPE { ofTypes: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    mtypeLst = __pa0.clone();
                    (valtype, mexpLst) = lookupUpdateMExpList((inid.clone()).clone(), path.clone(), mexpLst.clone(), mtypeLst.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::TUPLE_MATCH { tupleArgs: mexpLst.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::LIST_MATCH { listElts: mexpLst }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mtypeLst: Arc<metamodelica::List<Arc<TypeSignature>>> = metamodelica::nil();
                    let mut mexpLst = (*mexpLst).clone();
                    let mut mtype = (*mtype).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    mtype = __pa0.clone();
                    mtypeLst = List::fill(mtype.clone(), (mexpLst.clone().len() as i32));
                    (valtype, mexpLst) = lookupUpdateMExpList((inid.clone()).clone(), path.clone(), mexpLst.clone(), mtypeLst.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::LIST_MATCH { listElts: mexpLst.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::LIST_CONS_MATCH { rest: restmexp, head: mexp }, mtype, astDefs) => {
                    let mut otype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut restmexp = (*restmexp).clone();
                    let mut mexp = (*mexp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(mtype.clone(), astDefs.clone())?) {
                        Deref @ TypeSignature::LIST_TYPE { ofType: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    otype = __pa0.clone();
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(lookupUpdateMExpList((inid.clone()).clone(), path.clone(), list![mexp.clone(), restmexp.clone()], list![otype.clone(), mtype.clone()], astDefs.clone())?) {
                        (__pa1, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } }) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    valtype = __pa1.clone();
                    mexp = __pa2.clone();
                    restmexp = __pa3.clone();
                    Ok((valtype.clone(), Arc::new(MatchingExp::LIST_CONS_MATCH { head: mexp.clone(), rest: restmexp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outValueType, outMatchingExp))
}

pub fn lookupUpdateMExpDotPath(mut inIdent: Ident, mut inPathIdent: Arc<PathIdent>, mut inMatchingExp: Arc<MatchingExp>, mut inMType: Arc<TypeSignature>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<TypeSignature>, Arc<MatchingExp>)> {
    let mut outValueType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut outMatchingExp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    (outValueType, outMatchingExp) = 'mc: {
        let __mc_input = (inIdent.clone(), inPathIdent.clone(), inMatchingExp.clone(), inMType.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ MatchingExp::BIND_AS_MATCH { matchingExp: mexp, bindIdent: bid }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    (valtype, mexp) = lookupUpdateMExpDotPath((inid.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::BIND_AS_MATCH { bindIdent: (bid.clone()).clone(), matchingExp: mexp.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::IDENT { ident: id }, Deref @ MatchingExp::RECORD_MATCH { fieldMatchings: fms, tagName: tagpath }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fields: TypedIdents = metamodelica::nil();
                    let mut fms = (*fms).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    (fields, _) = getFieldsForRecord(mtype.clone(), tagpath.clone(), astDefs.clone())?;
                    valtype = lookupTupleList(fields.clone(), (id.clone()).clone())?;
                    fms = updateFieldMatchingsForField((inid.clone()).clone(), (id.clone()).clone(), fms.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::IDENT { ident: id }, Deref @ MatchingExp::RECORD_MATCH { fieldMatchings: fms, tagName: tagpath }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fields: TypedIdents = metamodelica::nil();
                    let mut reason: ArcStr = arcstr::literal!("");
                    let mut tagpath = (*tagpath).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    (fields, tagpath) = getFieldsForRecord(mtype.clone(), tagpath.clone(), astDefs.clone())?;
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(fields.clone(), (id.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path - failed in lookup for field '")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!("' at the end of the path '")); __mm_s.push_str(&*inid.clone()); __mm_s.push_str(&*literal!("', no such field in '")); __mm_s.push_str(&*pathIdentString(tagpath.clone())?); __mm_s.push_str(&*literal!("' record fields.\n")); ArcStr::from(__mm_s) }).clone();
                    valtype = Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (reason.clone()).clone() });
                    Ok((valtype.clone(), Arc::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::PATH_IDENT { path, ident: id }, Deref @ MatchingExp::RECORD_MATCH { fieldMatchings: fms, tagName: tagpath }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fields: TypedIdents = metamodelica::nil();
                    let mut fms = (*fms).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    (fields, _) = getFieldsForRecord(mtype.clone(), tagpath.clone(), astDefs.clone())?;
                    mtype = lookupTupleList(fields.clone(), (id.clone()).clone())?;
                    (valtype, fms) = lookupUpdateMExpDotPathRecord((inid.clone()).clone(), (id.clone()).clone(), path.clone(), fms.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), Arc::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ PathIdent::PATH_IDENT { ident: id, .. }, Deref @ MatchingExp::RECORD_MATCH { fieldMatchings: fms, tagName: tagpath }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fields: TypedIdents = metamodelica::nil();
                    let mut reason: ArcStr = arcstr::literal!("");
                    let mut tagpath = (*tagpath).clone();
                    let mut mtype = (*mtype).clone();
                    mtype = deAliasedType(mtype.clone(), astDefs.clone())?;
                    (fields, tagpath) = getFieldsForRecord(mtype.clone(), tagpath.clone(), astDefs.clone())?;
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(fields.clone(), (id.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path - failed in lookup for field '")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!("' inside the (encoded) path '")); __mm_s.push_str(&*inid.clone()); __mm_s.push_str(&*literal!("', no such field in '")); __mm_s.push_str(&*pathIdentString(tagpath.clone())?); __mm_s.push_str(&*literal!("' record fields.\n")); ArcStr::from(__mm_s) }).clone();
                    valtype = Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (reason.clone()).clone() });
                    Ok((valtype.clone(), Arc::new(MatchingExp::RECORD_MATCH { tagName: tagpath.clone(), fieldMatchings: fms.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, mexp, _, _) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut reason: ArcStr = arcstr::literal!("");
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path (encoded) '")); __mm_s.push_str(&*inid.clone()); __mm_s.push_str(&*literal!("', cannot follow the rest path '")); __mm_s.push_str(&*pathIdentString(path.clone())?); __mm_s.push_str(&*literal!("', no record match available to look down the path.")); ArcStr::from(__mm_s) }).clone();
                    valtype = Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (reason.clone()).clone() });
                    Ok((valtype.clone(), mexp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-!!!lookupUpdateMExpDotPath failed for ident '")); __mm_s.push_str(&*inIdent.clone()); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outValueType, outMatchingExp))
}

pub fn updateFieldMatchingsForField(mut inIdent: Ident, mut inField: Ident, mut inFieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>> {
    let mut outFieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>> = metamodelica::nil();
    outFieldMatchings = 'mc: {
        let __mc_input = (inIdent.clone(), inField.clone(), inFieldMatchings.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, Deref @ metamodelica::List::Nil) => {
                    Ok(list![(fieldid.clone(), Arc::new(MatchingExp::BIND_MATCH { bindIdent: (inid.clone()).clone() }))])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, Deref @ metamodelica::List::Cons { head: (ident, mexp), tail: fms }) => {
                    let mut mexp = (*mexp).clone();
                    let true = (stringEq((fieldid.clone()).clone(), (ident.clone()).clone())) else { bail!("pattern mismatch") };
                    mexp = makeBindAs((inid.clone()).clone(), mexp.clone())?;
                    Ok(cons((fieldid.clone(), mexp.clone()), fms.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, Deref @ metamodelica::List::Cons { head: fm, tail: fms }) => {
                    let mut fms = (*fms).clone();
                    fms = updateFieldMatchingsForField((inid.clone()).clone(), (fieldid.clone()).clone(), fms.clone())?;
                    Ok(cons(fm.clone(), fms.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!updateFieldMatchingsForField failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFieldMatchings)
}

pub fn makeBindAs(mut inIdent: Ident, mut inMExp: Arc<MatchingExp>) -> Result<Arc<MatchingExp>> {
    let mut outMExp: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    outMExp = 'mc: {
        let __mc_input = (inIdent.clone(), inMExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, inmexp @ Deref @ MatchingExp::BIND_AS_MATCH { bindIdent: bid, .. }) => {
                    let true = (stringEq((inid.clone()).clone(), (bid.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(inmexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, Deref @ MatchingExp::BIND_AS_MATCH { matchingExp: mexp, bindIdent: bid }) => {
                    let mut mexp = (*mexp).clone();
                    mexp = makeBindAs((inid.clone()).clone(), mexp.clone())?;
                    Ok(Arc::new(MatchingExp::BIND_AS_MATCH { bindIdent: (bid.clone()).clone(), matchingExp: mexp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, inmexp @ Deref @ MatchingExp::BIND_MATCH { bindIdent: bid }) => {
                    let true = (stringEq((inid.clone()).clone(), (bid.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(inmexp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, mexp) => {
                    Ok(Arc::new(MatchingExp::BIND_AS_MATCH { bindIdent: (inid.clone()).clone(), matchingExp: mexp.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!makeBindAs failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMExp)
}

pub fn lookupUpdateMExpDotPathRecord(mut inIdent: Ident, mut inField: Ident, mut inPathIdent: Arc<PathIdent>, mut inFieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>, mut inMType: Arc<TypeSignature>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<TypeSignature>, Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>)> {
    let mut outValueType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut outFieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>> = metamodelica::nil();
    (outValueType, outFieldMatchings) = 'mc: {
        let __mc_input = (inIdent.clone(), inField.clone(), inPathIdent.clone(), inFieldMatchings.clone(), inMType.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, _, Deref @ metamodelica::List::Nil, _, _) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut reason: ArcStr = arcstr::literal!("");
                    reason = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved path '")); __mm_s.push_str(&*inid.clone()); __mm_s.push_str(&*literal!("', cannot follow the path after a dot, no record match available to look down the path after '")); __mm_s.push_str(&*fieldid.clone()); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone();
                    valtype = Arc::new(TypeSignature::UNRESOLVED_TYPE { reason: (reason.clone()).clone() });
                    Ok((valtype.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, path, Deref @ metamodelica::List::Cons { head: (ident, mexp), tail: fms }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    let true = (stringEq((fieldid.clone()).clone(), (ident.clone()).clone())) else { bail!("pattern mismatch") };
                    (valtype, mexp) = lookupUpdateMExpDotPath((inid.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), cons((ident.clone(), mexp.clone()), fms.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, fieldid, path, Deref @ metamodelica::List::Cons { head: fm, tail: fms }, mtype, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fms = (*fms).clone();
                    (valtype, fms) = lookupUpdateMExpDotPathRecord((inid.clone()).clone(), (fieldid.clone()).clone(), path.clone(), fms.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), cons(fm.clone(), fms.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-!!!lookupUpdateMExpDotPathRecord failed for ident '")); __mm_s.push_str(&*inIdent.clone()); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outValueType, outFieldMatchings))
}

pub fn lookupUpdateMExpRecord(mut inIdent: Ident, mut inPathIdent: Arc<PathIdent>, mut inFieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>, mut inFields: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<TypeSignature>, Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>>)> {
    let mut outValueType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut outFieldMatchings: Arc<metamodelica::List<(ArcStr, Arc<MatchingExp>)>> = metamodelica::nil();
    (outValueType, outFieldMatchings) = 'mc: {
        let __mc_input = (inIdent.clone(), inPathIdent.clone(), inFieldMatchings.clone(), inFields.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ metamodelica::List::Cons { head: (ident, mexp), tail: fms }, fields, astDefs) => {
                    let mut mtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    mtype = lookupTupleList(fields.clone(), (ident.clone()).clone())?;
                    (valtype, mexp) = lookupUpdateMatchingExp((inid.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), cons((ident.clone(), mexp.clone()), fms.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: (ident, _), tail: _ }, fields, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(fields.clone(), (ident.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Error!!!lookupUpdateMExpRecord failed in lookup for field (type) ident '")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ metamodelica::List::Cons { head: fm, tail: fms }, fields, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut fms = (*fms).clone();
                    (valtype, fms) = lookupUpdateMExpRecord((inid.clone()).clone(), path.clone(), fms.clone(), fields.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), cons(fm.clone(), fms.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outValueType, outFieldMatchings))
}

pub fn lookupUpdateMExpList(mut inIdent: Ident, mut inPathIdent: Arc<PathIdent>, mut inMExpList: Arc<metamodelica::List<Arc<MatchingExp>>>, mut inMTypeList: Arc<metamodelica::List<Arc<TypeSignature>>>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<TypeSignature>, Arc<metamodelica::List<Arc<MatchingExp>>>)> {
    let mut outValueType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut outMExpList: Arc<metamodelica::List<Arc<MatchingExp>>> = metamodelica::nil();
    (outValueType, outMExpList) = 'mc: {
        let __mc_input = (inIdent.clone(), inPathIdent.clone(), inMExpList.clone(), inMTypeList.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ metamodelica::List::Cons { head: mexp, tail: mexpLst }, Deref @ metamodelica::List::Cons { head: mtype, tail: _ }, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexp = (*mexp).clone();
                    (valtype, mexp) = lookupUpdateMatchingExp((inid.clone()).clone(), path.clone(), mexp.clone(), mtype.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), cons(mexp.clone(), mexpLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (inid, path, Deref @ metamodelica::List::Cons { head: mexp, tail: mexpLst }, Deref @ metamodelica::List::Cons { head: _, tail: mtypeLst }, astDefs) => {
                    let mut valtype: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut mexpLst = (*mexpLst).clone();
                    (valtype, mexpLst) = lookupUpdateMExpList((inid.clone()).clone(), path.clone(), mexpLst.clone(), mtypeLst.clone(), astDefs.clone())?;
                    Ok((valtype.clone(), cons(mexp.clone(), mexpLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outValueType, outMExpList))
}

pub fn getFieldsForRecord(mut inMType: Arc<TypeSignature>, mut inTagPath: Arc<PathIdent>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(TypedIdents, Arc<PathIdent>)> {
    let mut outFields: TypedIdents = metamodelica::nil();
    let mut inFullyQualifiedTagPath: Arc<PathIdent>;
    (outFields, inFullyQualifiedTagPath) = 'mc: {
        let __mc_input = (inMType.clone(), inTagPath.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: typepath }, tagpath, astDefs) => {
                    let mut typeident: Ident = arcstr::literal!("");
                    let mut tagident: Ident = arcstr::literal!("");
                    let mut typepckg: Arc<PathIdent>;
                    let mut typepckgOpt: Option<Arc<PathIdent>> = None;
                    let mut tagpckgOpt: Option<Arc<PathIdent>> = None;
                    let mut typeinfo: TypeInfo;
                    let mut fields: TypedIdents = metamodelica::nil();
                    let mut typepath = (*typepath).clone();
                    (typepckgOpt, typeident) = splitPackageAndIdent(typepath.clone())?;
                    (typepckg, typeinfo) = getTypeInfo(typepckgOpt.clone(), (typeident.clone()).clone(), astDefs.clone())?;
                    (tagpckgOpt, tagident) = splitPackageAndIdent(tagpath.clone())?;
                    checkPackageOpt(typepckg.clone(), tagpckgOpt.clone())?;
                    fields = getFields((tagident.clone()).clone(), typeinfo.clone(), (typeident.clone()).clone())?;
                    typepath = makePathIdent(typepckg.clone(), (tagident.clone()).clone())?;
                    Ok((fields.clone(), typepath.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { .. }, tagpath, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - (getFieldsForRecord) for case tag '")); __mm_s.push_str(&*pathIdentString(tagpath.clone())?); __mm_s.push_str(&*literal!("' failed for reason above.\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, tagpath, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - for case tag '")); __mm_s.push_str(&*pathIdentString(tagpath.clone())?); __mm_s.push_str(&*literal!("' the input type is not a NAME_TYPE hence not a union/record type.\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outFields, inFullyQualifiedTagPath))
}

pub fn splitPackageAndIdent(mut inTypePathIdent: Arc<PathIdent>) -> Result<(Option<Arc<PathIdent>>, Ident)> {
    let mut outPackagePath: Option<Arc<PathIdent>> = None;
    let mut outTypeIdent: Ident = arcstr::literal!("");
    (outPackagePath, outTypeIdent) = 'mc: {
        let __mc_input = inTypePathIdent.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ PathIdent::IDENT { ident: typeident } => {
                    Ok((None, typeident.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ PathIdent::PATH_IDENT { path: Deref @ PathIdent::IDENT { ident: typeident }, ident: pckgident } => {
                    Ok((Some(Arc::new(PathIdent::IDENT { ident: (pckgident.clone()).clone() })), typeident.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ PathIdent::PATH_IDENT { path: typepath @ Deref @ PathIdent::PATH_IDENT { .. }, ident: pckgident } => {
                    let mut typeident: Ident = arcstr::literal!("");
                    let mut typepckg: Arc<PathIdent>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(splitPackageAndIdent(typepath.clone())?) {
                        (Some(__pa0), __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    typepckg = __pa0.clone();
                    typeident = __pa1.clone();
                    Ok((Some(Arc::new(PathIdent::PATH_IDENT { ident: (pckgident.clone()).clone(), path: typepckg.clone() })), typeident.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!splitPackageAndIdent failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outPackagePath, outTypeIdent))
}

fn getPackageIdent(mut inTypePathIdent: Arc<PathIdent>) -> Result<Ident> {
    let mut outTypeIdent: Ident = arcstr::literal!("");
    (_, outTypeIdent) = splitPackageAndIdent(inTypePathIdent.clone())?;
    Ok(outTypeIdent)
}

pub fn makePathIdent(mut inPackage: Arc<PathIdent>, mut inIdent: Ident) -> Result<Arc<PathIdent>> {
    let mut outPathIdent: Arc<PathIdent>;
    outPathIdent = 'mc: {
        let __mc_input = (inPackage.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ PathIdent::IDENT { ident: pckgident }, ident) => {
                    Ok(Arc::new(PathIdent::PATH_IDENT { ident: (pckgident.clone()).clone(), path: Arc::new(PathIdent::IDENT { ident: (ident.clone()).clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ PathIdent::PATH_IDENT { path: pckgpath, ident: pckgident }, ident) => {
                    let mut path: Arc<PathIdent>;
                    path = makePathIdent(pckgpath.clone(), (ident.clone()).clone())?;
                    Ok(Arc::new(PathIdent::PATH_IDENT { ident: (pckgident.clone()).clone(), path: path.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!makePathIdent failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPathIdent)
}

pub fn getTypeInfo(mut inTypePackageOpt: Option<Arc<PathIdent>>, mut inTypeIdent: Ident, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<(Arc<PathIdent>, TypeInfo)> {
    let mut outTypePackage: Arc<PathIdent>;
    let mut outTypeInfo: TypeInfo;
    (outTypePackage, outTypeInfo) = 'mc: {
        let __mc_input = (inTypePackageOpt.clone(), inTypeIdent.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, typeident, Deref @ metamodelica::List::Cons { head: ASTDef { types: typeLst, isDefault: true, importPackage: importckg }, tail: _ }) => {
                    let mut typeinfo: TypeInfo;
                    typeinfo = lookupTupleList(typeLst.clone(), (typeident.clone()).clone())?;
                    Ok((importckg.clone(), typeinfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(typepckg), typeident, Deref @ metamodelica::List::Cons { head: ASTDef { types: typeLst, importPackage: importckg, .. }, tail: _ }) => {
                    let mut typeinfo: TypeInfo;
                    let true = (typepckg.clone() == importckg.clone()) else { bail!("pattern mismatch") };
                    typeinfo = lookupTupleList(typeLst.clone(), (typeident.clone()).clone())?;
                    Ok((typepckg.clone(), typeinfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (typepckgOpt, typeident, Deref @ metamodelica::List::Cons { head: _, tail: astDefs }) => {
                    let mut typepckg: Arc<PathIdent>;
                    let mut typeinfo: TypeInfo;
                    (typepckg, typeinfo) = getTypeInfo(typepckgOpt.clone(), (typeident.clone()).clone(), astDefs.clone())?;
                    Ok((typepckg.clone(), typeinfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, typeident, Deref @ metamodelica::List::Nil) => {
                    addSusanNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - getTypeInfo failed to lookup the type '")); __mm_s.push_str(&*typeident.clone()); __mm_s.push_str(&*literal!("' after looking up all AST definitions.")); ArcStr::from(__mm_s) }).clone(), dummySourceInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(typepckg), typeident, Deref @ metamodelica::List::Nil) => {
                    addSusanNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getTypeInfo failed to lookup the type '")); __mm_s.push_str(&*pathIdentString(typepckg.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*typeident.clone()); __mm_s.push_str(&*literal!("' after looking up all AST definitions.")); ArcStr::from(__mm_s) }).clone(), dummySourceInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outTypePackage, outTypeInfo))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deAliasedType(mut inType: Arc<TypeSignature>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<Arc<TypeSignature>> {
    let mut outType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    outType = 'mc: {
        let __mc_input = (inType.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: typepath }, astDefs) => {
                    let mut dt: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut typeident: Ident = arcstr::literal!("");
                    let mut typepckgOpt: Option<Arc<PathIdent>> = None;
                    (typepckgOpt, typeident) = splitPackageAndIdent(typepath.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(getTypeInfo(typepckgOpt.clone(), (typeident.clone()).clone(), astDefs.clone())?) {
                        (_, TypeInfo::TI_ALIAS_TYPE { aliasType: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    dt = __pa0.clone();
                    Ok(deAliasedType(dt.clone(), astDefs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn typesEqual(mut inType: Arc<TypeSignature>, mut inTypeConcrete: Arc<TypeSignature>, mut inTypeVars: Arc<metamodelica::List<ArcStr>>, mut inSetTypeVars: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<TypedIdents> {
    let mut outSetTypeVars: TypedIdents = metamodelica::nil();
    outSetTypeVars = 'mc: {
        let __mc_input = (inType.clone(), inTypeConcrete.clone(), inTypeVars.clone(), inSetTypeVars.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { ofType: ota }, Deref @ TypeSignature::LIST_TYPE { ofType: otb }, tyVars, setTyVars, astDefs) => {
                    Ok(typesEqual(ota.clone(), otb.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::ARRAY_TYPE { ofType: ota }, Deref @ TypeSignature::ARRAY_TYPE { ofType: otb }, tyVars, setTyVars, astDefs) => {
                    Ok(typesEqual(ota.clone(), otb.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { ofType: ota }, Deref @ TypeSignature::OPTION_TYPE { ofType: otb }, tyVars, setTyVars, astDefs) => {
                    Ok(typesEqual(ota.clone(), otb.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TUPLE_TYPE { ofTypes: otaLst }, Deref @ TypeSignature::TUPLE_TYPE { ofTypes: otbLst }, tyVars, setTyVars, astDefs) => {
                    Ok(typesEqualList(otaLst.clone(), otbLst.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::PATH_IDENT { .. } }, tyConcrete, _, setTyVars, astDefs) => {
                    let mut ty: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut tyConcrete = (*tyConcrete).clone();
                    ty = deAliasedType(inType.clone(), astDefs.clone())?;
                    tyConcrete = deAliasedType(tyConcrete.clone(), astDefs.clone())?;
                    typesEqualConcrete(ty.clone(), tyConcrete.clone(), astDefs.clone())?;
                    Ok(setTyVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyConcrete, tyVars, setTyVars, astDefs) => {
                    let mut ty: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut tyConcrete = (*tyConcrete).clone();
                    let false = (listMember((tid.clone()).clone(), tyVars.clone())) else { bail!("pattern mismatch") };
                    ty = deAliasedType(inType.clone(), astDefs.clone())?;
                    tyConcrete = deAliasedType(tyConcrete.clone(), astDefs.clone())?;
                    typesEqualConcrete(ty.clone(), tyConcrete.clone(), astDefs.clone())?;
                    Ok(setTyVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyConcrete, Deref @ metamodelica::List::Cons { head: _, tail: _ }, setTyVars, astDefs) => {
                    let mut ty: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut tyConcreteDA: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    ty = lookupTupleList(setTyVars.clone(), (tid.clone()).clone())?;
                    tyConcreteDA = deAliasedType(tyConcrete.clone(), astDefs.clone())?;
                    typesEqualConcrete(ty.clone(), tyConcreteDA.clone(), astDefs.clone())?;
                    Ok(setTyVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyConcrete, Deref @ metamodelica::List::Cons { head: _, tail: _ }, setTyVars, astDefs) => {
                    let mut ty: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let mut tyConcreteDA: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    ty = lookupTupleList(setTyVars.clone(), (tid.clone()).clone())?;
                    tyConcreteDA = deAliasedType(tyConcrete.clone(), astDefs.clone())?;
                    if '__try0: {
                        unwrap_break_err!(typesEqualConcrete(ty.clone(), tyConcreteDA.clone(), astDefs.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - unmatched type for type variable '")); __mm_s.push_str(&*tid.clone()); __mm_s.push_str(&*literal!("'. Firstly inferred '")); __mm_s.push_str(&*typeSignatureString(ty.clone())?); __mm_s.push_str(&*literal!("', next inferred '")); __mm_s.push_str(&*typeSignatureString(tyConcrete.clone())?); __mm_s.push_str(&*literal!("'(dealiased '")); __mm_s.push_str(&*typeSignatureString(tyConcreteDA.clone())?); __mm_s.push_str(&*literal!("').\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyConcrete, tyVars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, setTyVars, astDefs) => {
                    let mut tyConcreteDA: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(setTyVars.clone(), (tid.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let true = (listMember((tid.clone()).clone(), tyVars.clone())) else { bail!("pattern mismatch") };
                    tyConcreteDA = deAliasedType(tyConcrete.clone(), astDefs.clone())?;
                    Ok(cons((tid.clone(), tyConcreteDA.clone()), setTyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::UNRESOLVED_TYPE { .. }, Deref @ TypeSignature::UNRESOLVED_TYPE { reason: _ }, _, setTyVars, _) => {
                    Ok(setTyVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, tyConcrete, _, setTyVars, _) => {
                    if '__try0: {
                        ::match_deref::match_deref! { match &(ty.clone()) {
                            Deref @ TypeSignature::NAMED_TYPE { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let true = (ty.clone() == tyConcrete.clone()) else { bail!("pattern mismatch") };
                    Ok(setTyVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outSetTypeVars)
}

fn typesEqualConcrete(mut inTypeA: Arc<TypeSignature>, mut inTypeB: Arc<TypeSignature>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inTypeA.clone(), inTypeB.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: na }, Deref @ TypeSignature::NAMED_TYPE { name: nb }, _) => {
                    let true = (na.clone() == nb.clone()) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tyA, tyB, astDefs) => {
                    if '__try0: {
                        ::match_deref::match_deref! { match &(tyA.clone()) {
                            Deref @ TypeSignature::NAMED_TYPE { .. } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let _ = typesEqual(tyA.clone(), tyB.clone(), metamodelica::nil(), metamodelica::nil(), astDefs.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn typesEqualList(mut inTypeAList: Arc<metamodelica::List<Arc<TypeSignature>>>, mut inTypeBList: Arc<metamodelica::List<Arc<TypeSignature>>>, mut inTypeVars: Arc<metamodelica::List<ArcStr>>, mut inSetTypeVars: TypedIdents, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<TypedIdents> {
    let mut outSetTypeVars: TypedIdents = metamodelica::nil();
    outSetTypeVars = (::match_deref::match_deref! { match &((inTypeAList.clone(), inTypeBList.clone(), inTypeVars.clone(), inSetTypeVars.clone(), inASTDefs.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, setTyVars, _) => {
            setTyVars.clone()
        },
        (Deref @ metamodelica::List::Cons { head: ota, tail: otaLst }, Deref @ metamodelica::List::Cons { head: otb, tail: otbLst }, tyVars, setTyVars, astDefs) => {
            let mut setTyVars = (*setTyVars).clone();
            setTyVars = typesEqual(ota.clone(), otb.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?;
            typesEqualList(otaLst.clone(), otbLst.clone(), tyVars.clone(), setTyVars.clone(), astDefs.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSetTypeVars)
}

fn specializeType(mut inType: Arc<TypeSignature>, mut inTypeVars: Arc<metamodelica::List<ArcStr>>, mut inSetTypeVars: TypedIdents) -> Result<Arc<TypeSignature>> {
    let mut outType: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    outType = 'mc: {
        let __mc_input = (inType.clone(), inTypeVars.clone(), inSetTypeVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { ofType: ota }, tyVars, setTyVars) => {
                    let mut ota = (*ota).clone();
                    ota = specializeType(ota.clone(), tyVars.clone(), setTyVars.clone())?;
                    Ok(Arc::new(TypeSignature::LIST_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::ARRAY_TYPE { ofType: ota }, tyVars, setTyVars) => {
                    let mut ota = (*ota).clone();
                    ota = specializeType(ota.clone(), tyVars.clone(), setTyVars.clone())?;
                    Ok(Arc::new(TypeSignature::ARRAY_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { ofType: ota }, tyVars, setTyVars) => {
                    let mut ota = (*ota).clone();
                    ota = specializeType(ota.clone(), tyVars.clone(), setTyVars.clone())?;
                    Ok(Arc::new(TypeSignature::OPTION_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TUPLE_TYPE { ofTypes: otaLst }, tyVars, setTyVars) => {
                    let mut otaLst = (*otaLst).clone();
                    otaLst = List::map2(otaLst.clone(), (std::sync::Arc::new(specializeType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<TypeSignature>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>) -> Result<Arc<TypeSignature>> + 'static>), tyVars.clone(), setTyVars.clone());
                    Ok(Arc::new(TypeSignature::TUPLE_TYPE { ofTypes: otaLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tyConcrete @ Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyVars, _) => {
                    let false = (listMember((tid.clone()).clone(), tyVars.clone())) else { bail!("pattern mismatch") };
                    Ok(tyConcrete.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, Deref @ metamodelica::List::Cons { head: _, tail: _ }, setTyVars) => {
                    let mut tyConcrete: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    tyConcrete = lookupTupleList(setTyVars.clone(), (tid.clone()).clone())?;
                    Ok(tyConcrete.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: tid } }, tyVars @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, setTyVars) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    let true = (listMember((tid.clone()).clone(), tyVars.clone())) else { bail!("pattern mismatch") };
                    if '__try0: {
                        let _ = unwrap_break_err!(lookupTupleList(setTyVars.clone(), (tid.clone()).clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - cannot infer type variable '")); __mm_s.push_str(&*tid.clone()); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tyConcrete, _, _) => {
                    if '__try0: {
                        ::match_deref::match_deref! { match &(tyConcrete.clone()) {
                            Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { .. } } => (),
                            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Ok(tyConcrete.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

//for now, succeed or  error + fail
pub fn getFunSignature(mut inFunName: Arc<PathIdent>, mut inSourceInfo: SourceInfo, mut inTplPackage: TemplPackage) -> Result<(Arc<PathIdent>, TypedIdents, TypedIdents, Arc<metamodelica::List<ArcStr>>)> {
    let mut outPath: Arc<PathIdent>;
    let mut outInArgs: TypedIdents = metamodelica::nil();
    let mut outOutArgs: TypedIdents = metamodelica::nil();
    let mut outTypeVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outPath, outInArgs, outOutArgs, outTypeVars) = 'mc: {
        let __mc_input = (inFunName.clone(), inSourceInfo.clone(), inTplPackage.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fname @ Deref @ PathIdent::IDENT { ident: templname }, _, TemplPackage { templateDefs, .. }) => {
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let TemplateDef::TEMPLATE_DEF { args: __pa0, .. } = (lookupTupleList(templateDefs.clone(), (templname.clone()).clone())?) else { bail!("pattern mismatch") };
                    iargs = __pa0.clone();
                    iargs = cons(imlicitTxtArg.clone(), iargs.clone());
                    oargs = List::filterOnTrue(iargs.clone(), (std::sync::Arc::new(fnptr!(isText, (ArcStr, Arc<TypeSignature>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<TypeSignature>)) -> Result<bool> + 'static>));
                    Ok((fname.clone(), iargs.clone(), oargs.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ PathIdent::IDENT { ident: templname }, _, TemplPackage { templateDefs, .. }) => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    let _ = lookupTupleList(templateDefs.clone(), (templname.clone()).clone())?;
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Constant template '")); __mm_s.push_str(&*templname.clone()); __mm_s.push_str(&*literal!("' is used in a function/template context (while it is defined as a constant).")); ArcStr::from(__mm_s) }).clone();
                    addSusanError((msg.clone()).clone(), inSourceInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fname, _, TemplPackage { astDefs, .. }) => {
                    let mut funpckg: Arc<PathIdent>;
                    let mut funpckgOpt: Option<Arc<PathIdent>> = None;
                    let mut fident: Ident = arcstr::literal!("");
                    let mut tyVars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut iargs: TypedIdents = metamodelica::nil();
                    let mut oargs: TypedIdents = metamodelica::nil();
                    let mut fname = (*fname).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(deAliasedType(Arc::new(TypeSignature::NAMED_TYPE { name: fname.clone() }), astDefs.clone())?) {
                        Deref @ TypeSignature::NAMED_TYPE { name: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    fname = __pa0.clone();
                    (funpckgOpt, fident) = splitPackageAndIdent(fname.clone())?;
                    let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(getTypeInfo(funpckgOpt.clone(), (fident.clone()).clone(), astDefs.clone())?) {
                        (__pa1, TypeInfo::TI_FUN_TYPE { tyVars: __pa2, outArgs: __pa3, inArgs: __pa4 }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    funpckg = __pa1.clone();
                    tyVars = __pa2.clone();
                    oargs = __pa3.clone();
                    iargs = __pa4.clone();
                    fname = if (Arc::new(PathIdent::IDENT { ident: (literal!("builtin")).clone() }) == funpckg.clone()) {Arc::new(PathIdent::IDENT { ident: (fident.clone()).clone() })} else {makePathIdent(funpckg.clone(), (fident.clone()).clone())?};
                    Ok((fname.clone(), iargs.clone(), oargs.clone(), tyVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut msg: ArcStr = arcstr::literal!("");
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unresolved template/function name '")); __mm_s.push_str(&*pathIdentString(inFunName.clone())?); __mm_s.push_str(&*literal!("'.")); ArcStr::from(__mm_s) }).clone();
                    addSusanError((msg.clone()).clone(), inSourceInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outPath, outInArgs, outOutArgs, outTypeVars))
}

pub fn checkPackageOpt(mut inPackage: Arc<PathIdent>, mut inPackageOpt: Option<Arc<PathIdent>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inPackage.clone(), inPackageOpt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Some(pckgpath)) => {
                    let true = (path.clone() == pckgpath.clone()) else { bail!("pattern mismatch") };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!checkPackageOpt failed - package paths are not the same.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn getFields(mut inTagIdent: Ident, mut inTypeInfo: TypeInfo, mut inTypeIdent: Ident) -> Result<TypedIdents> {
    let mut outFields: TypedIdents = metamodelica::nil();
    outFields = 'mc: {
        let __mc_input = (inTagIdent.clone(), inTypeInfo.clone(), inTypeIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tagident, TypeInfo::TI_UNION_TYPE { recTags: ref rectags }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut fields: TypedIdents = metamodelica::nil();
            fields = lookupTupleList(rectags.clone(), (tagident.clone()).clone())?;
            Ok(fields.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tagident, TypeInfo::TI_UNION_TYPE { recTags: ref rectags }, mut typeident) = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            if '__try0: {
                let _ = unwrap_break_err!(lookupTupleList(rectags.clone(), (tagident.clone()).clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - getFields failed to lookup the union tag '")); __mm_s.push_str(&*tagident.clone()); __mm_s.push_str(&*literal!("', that is not found in type '")); __mm_s.push_str(&*typeident.clone()); __mm_s.push_str(&*literal!("'.\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tagident, TypeInfo::TI_RECORD_TYPE { fields: mut fields }, mut typeident) = __mc_input.clone() else { bail!("nomatch") };
            let true = (stringEq((tagident.clone()).clone(), (typeident.clone()).clone())) else { bail!("pattern mismatch") };
            Ok(fields.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut tagident, TypeInfo::TI_RECORD_TYPE { .. }, mut typeident) = __mc_input.clone() else { bail!("nomatch") };
            let false = (stringEq((tagident.clone()).clone(), (typeident.clone()).clone())) else { bail!("pattern mismatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - getFields failed to match the tag '")); __mm_s.push_str(&*tagident.clone()); __mm_s.push_str(&*literal!("', the type '")); __mm_s.push_str(&*typeident.clone()); __mm_s.push_str(&*literal!("' expected.\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut typeinfo, _) = __mc_input.clone() else { bail!("nomatch") };
            if '__try0: {
                let TypeInfo::TI_UNION_TYPE { .. } = (typeinfo.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            if '__try1: {
                let TypeInfo::TI_RECORD_TYPE { .. } = (typeinfo.clone()) else { break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- getFields failed - the typeinfo is neither union nor record type.\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFields)
}

pub fn isRecordTag(mut inTagIdent: Ident, mut inTypeInfo: TypeInfo, mut inTypeIdent: Ident) -> Result<()> {
    let _ = (match (inTagIdent.clone(), inTypeInfo.clone(), inTypeIdent.clone()) {
        (mut tagident, TypeInfo::TI_UNION_TYPE { recTags: ref rectags }, _) => {
            let _ = lookupTupleList(rectags.clone(), (tagident.clone()).clone())?;
            ()
        },
        (mut tagident, TypeInfo::TI_RECORD_TYPE { .. }, mut typeident) => {
            let true = (stringEq((tagident.clone()).clone(), (typeident.clone()).clone())) else { bail!("pattern mismatch") };
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn fullyQualifyASTDefs(mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<Arc<metamodelica::List<ASTDef>>> {
    let mut outFullyQualifiedASTDefs: Arc<metamodelica::List<ASTDef>> = metamodelica::nil();
    outFullyQualifiedASTDefs = 'mc: {
        let __mc_input = inASTDefs.clone();
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
                Deref @ metamodelica::List::Cons { head: ASTDef { types: typeLst, isDefault: isdefault, importPackage: importckg }, tail: restAstDefs } => {
                    let mut typeLst = (*typeLst).clone();
                    let mut restAstDefs = (*restAstDefs).clone();
                    typeLst = listMap1Tuple22(typeLst.clone(), (std::sync::Arc::new(fullyQualifyAstTypeInfo) as std::sync::Arc<dyn ::std::ops::Fn(TypeInfo, Arc<PathIdent>) -> Result<TypeInfo> + 'static>), importckg.clone())?;
                    restAstDefs = fullyQualifyASTDefs(restAstDefs.clone())?;
                    Ok(cons(ASTDef { importPackage: importckg.clone(), isDefault: isdefault.clone(), types: typeLst.clone() }, restAstDefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: ASTDef { types: typeLst, importPackage: importckg, .. }, tail: _ } => {
                    let mut typeLst = (*typeLst).clone();
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    if '__try0: {
                        typeLst = unwrap_break_err!(listMap1Tuple22(typeLst.clone(), (std::sync::Arc::new(fullyQualifyAstTypeInfo) as std::sync::Arc<dyn ::std::ops::Fn(TypeInfo, Arc<PathIdent>) -> Result<TypeInfo> + 'static>), importckg.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-fullyQualifyASTDefs failed for importckg = ")); __mm_s.push_str(&*pathIdentString(importckg.clone())?); __mm_s.push_str(&*literal!(" .\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!! fullyQualifyASTDefs failed .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFullyQualifiedASTDefs)
}

pub fn fullyQualifyAstTypeInfo(mut inASTTypeInfo: TypeInfo, mut inImportPackage: Arc<PathIdent>) -> Result<TypeInfo> {
    let mut outFullyQualifiedASTTypeInfo: TypeInfo;
    outFullyQualifiedASTTypeInfo = 'mc: {
        let __mc_input = (inASTTypeInfo.clone(), inImportPackage.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_UNION_TYPE { recTags }, importpckg) => {
                    let mut recTags = (*recTags).clone();
                    recTags = listMap2Tuple22(recTags.clone(), (std::sync::Arc::new(fullyQualifyAstTypedIdents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>, Arc<PathIdent>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<(ArcStr, Arc<TypeSignature>)>>> + 'static>), importpckg.clone(), metamodelica::nil())?;
                    Ok(TypeInfo::TI_UNION_TYPE { recTags: recTags.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_RECORD_TYPE { fields }, importpckg) => {
                    let mut fields = (*fields).clone();
                    fields = fullyQualifyAstTypedIdents(fields.clone(), importpckg.clone(), metamodelica::nil())?;
                    Ok(TypeInfo::TI_RECORD_TYPE { fields: fields.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_ALIAS_TYPE { aliasType }, importpckg) => {
                    let mut aliasType = (*aliasType).clone();
                    aliasType = fullyQualifyAstTypeSignature(aliasType.clone(), importpckg.clone(), metamodelica::nil())?;
                    Ok(TypeInfo::TI_ALIAS_TYPE { aliasType: aliasType.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_FUN_TYPE { tyVars: tyvars, outArgs, inArgs }, importpckg) => {
                    let mut outArgs = (*outArgs).clone();
                    let mut inArgs = (*inArgs).clone();
                    inArgs = fullyQualifyAstTypedIdents(inArgs.clone(), importpckg.clone(), tyvars.clone())?;
                    outArgs = fullyQualifyAstTypedIdents(outArgs.clone(), importpckg.clone(), tyvars.clone())?;
                    Ok(TypeInfo::TI_FUN_TYPE { inArgs: inArgs.clone(), outArgs: outArgs.clone(), tyVars: tyvars.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TypeInfo::TI_CONST_TYPE { constType }, importpckg) => {
                    let mut constType = (*constType).clone();
                    constType = fullyQualifyAstTypeSignature(constType.clone(), importpckg.clone(), metamodelica::nil())?;
                    Ok(TypeInfo::TI_CONST_TYPE { constType: constType.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!! fullyQualifyAstTypeInfo failed .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFullyQualifiedASTTypeInfo)
}

pub fn fullyQualifyAstTypedIdents(mut inASTDefTypedIdents: TypedIdents, mut inImportPackage: Arc<PathIdent>, mut inTypeVars: Arc<metamodelica::List<ArcStr>>) -> Result<TypedIdents> {
    let mut outASTDefTypedIdents: TypedIdents = metamodelica::nil();
    outASTDefTypedIdents = listMap2Tuple22(inASTDefTypedIdents.clone(), (std::sync::Arc::new(fullyQualifyAstTypeSignature) as std::sync::Arc<dyn ::std::ops::Fn(Arc<TypeSignature>, Arc<PathIdent>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<TypeSignature>> + 'static>), inImportPackage.clone(), inTypeVars.clone())?;
    Ok(outASTDefTypedIdents)
}

pub fn fullyQualifyAstTypeSignature(mut inASTDefTypeSignature: Arc<TypeSignature>, mut inImportPackage: Arc<PathIdent>, mut inTypeVars: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<TypeSignature>> {
    let mut outASTDefTypeSignature: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    outASTDefTypeSignature = 'mc: {
        let __mc_input = (inASTDefTypeSignature.clone(), inImportPackage.clone(), inTypeVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { ofType: ota }, importpckg, tyVars) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyAstTypeSignature(ota.clone(), importpckg.clone(), tyVars.clone())?;
                    Ok(Arc::new(TypeSignature::LIST_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::ARRAY_TYPE { ofType: ota }, importpckg, tyVars) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyAstTypeSignature(ota.clone(), importpckg.clone(), tyVars.clone())?;
                    Ok(Arc::new(TypeSignature::ARRAY_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { ofType: ota }, importpckg, tyVars) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyAstTypeSignature(ota.clone(), importpckg.clone(), tyVars.clone())?;
                    Ok(Arc::new(TypeSignature::OPTION_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TUPLE_TYPE { ofTypes: typeLst }, importpckg, tyVars) => {
                    let mut typeLst = (*typeLst).clone();
                    typeLst = List::map2(typeLst.clone(), (std::sync::Arc::new(fullyQualifyAstTypeSignature) as std::sync::Arc<dyn ::std::ops::Fn(Arc<TypeSignature>, Arc<PathIdent>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<TypeSignature>> + 'static>), importpckg.clone(), tyVars.clone());
                    Ok(Arc::new(TypeSignature::TUPLE_TYPE { ofTypes: typeLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ts @ Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: typeident } }, _, tyVars) => {
                    let true = (listMember((typeident.clone()).clone(), tyVars.clone())) else { bail!("pattern mismatch") };
                    Ok(ts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: typeident } }, importpckg, _) => {
                    let mut na: Arc<PathIdent>;
                    let mut ts: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    na = makePathIdent(importpckg.clone(), (typeident.clone()).clone())?;
                    ts = convertNameTypeIfIntrinsic(na.clone());
                    Ok(ts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: na @ Deref @ PathIdent::PATH_IDENT { .. } }, _, _) => {
                    let mut ts: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
                    ts = convertNameTypeIfIntrinsic(na.clone());
                    Ok(ts.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inASTDefTypeSignature.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outASTDefTypeSignature)
}

pub fn convertNameTypeIfIntrinsic(mut inNameOfType: Arc<PathIdent>) -> Arc<TypeSignature> {
    let mut outTypeSignature: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    outTypeSignature = (::match_deref::match_deref! { match &(inNameOfType.clone()) {
        Deref @ PathIdent::PATH_IDENT { path: Deref @ PathIdent::IDENT { ident: Deref @ "Text" }, ident: Deref @ "Tpl" } => Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE),
        _ => Arc::new(TypeSignature::NAMED_TYPE { name: inNameOfType.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTypeSignature
}

pub fn fullyQualifyTemplateDef(mut inTemplateDef: TemplateDef, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<TemplateDef> {
    let mut outTemplateDef: TemplateDef;
    outTemplateDef = 'mc: {
        let __mc_input = (inTemplateDef.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TemplateDef::LITERAL_DEF { litType, value: r#str }, astDefs) => {
                    let mut litType = (*litType).clone();
                    litType = fullyQualifyTemplateTypeSignature(litType.clone(), astDefs.clone())?;
                    Ok(TemplateDef::LITERAL_DEF { value: (r#str.clone()).clone(), litType: litType.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (def @ TemplateDef::STR_TOKEN_DEF { .. }, _) => {
                    Ok(def.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (TemplateDef::TEMPLATE_DEF { exp: texp, resc, lesc, args: targs }, astDefs) => {
                    let mut targs = (*targs).clone();
                    targs = listMap1Tuple22(targs.clone(), (std::sync::Arc::new(fullyQualifyTemplateTypeSignature) as std::sync::Arc<dyn ::std::ops::Fn(Arc<TypeSignature>, Arc<metamodelica::List<ASTDef>>) -> Result<Arc<TypeSignature>> + 'static>), astDefs.clone())?;
                    Ok(TemplateDef::TEMPLATE_DEF { args: targs.clone(), lesc: (lesc.clone()).clone(), resc: (resc.clone()).clone(), exp: texp.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- fullyQualifyTemplateDef failed .\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTemplateDef)
}

pub fn fullyQualifyTemplateTypeSignature(mut inTemplateTypeSignature: Arc<TypeSignature>, mut inASTDefs: Arc<metamodelica::List<ASTDef>>) -> Result<Arc<TypeSignature>> {
    let mut outFullyQualifiedTypeSignature: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    outFullyQualifiedTypeSignature = 'mc: {
        let __mc_input = (inTemplateTypeSignature.clone(), inASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::LIST_TYPE { ofType: ota }, astDefs) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyTemplateTypeSignature(ota.clone(), astDefs.clone())?;
                    Ok(Arc::new(TypeSignature::LIST_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::ARRAY_TYPE { ofType: ota }, astDefs) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyTemplateTypeSignature(ota.clone(), astDefs.clone())?;
                    Ok(Arc::new(TypeSignature::ARRAY_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::OPTION_TYPE { ofType: ota }, astDefs) => {
                    let mut ota = (*ota).clone();
                    ota = fullyQualifyTemplateTypeSignature(ota.clone(), astDefs.clone())?;
                    Ok(Arc::new(TypeSignature::OPTION_TYPE { ofType: ota.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::TUPLE_TYPE { ofTypes: typeLst }, astDefs) => {
                    let mut typeLst = (*typeLst).clone();
                    typeLst = List::map1(typeLst.clone(), (std::sync::Arc::new(fullyQualifyTemplateTypeSignature) as std::sync::Arc<dyn ::std::ops::Fn(Arc<TypeSignature>, Arc<metamodelica::List<ASTDef>>) -> Result<Arc<TypeSignature>> + 'static>), astDefs.clone());
                    Ok(Arc::new(TypeSignature::TUPLE_TYPE { ofTypes: typeLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: Deref @ PathIdent::IDENT { ident: Deref @ "Text" } }, _) => {
                    Ok(Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ TypeSignature::NAMED_TYPE { name: typepath }, astDefs) => {
                    let mut typeident: Ident = arcstr::literal!("");
                    let mut typepckg: Arc<PathIdent>;
                    let mut typepckgOpt: Option<Arc<PathIdent>> = None;
                    let mut typepath = (*typepath).clone();
                    (typepckgOpt, typeident) = splitPackageAndIdent(typepath.clone())?;
                    (typepckg, _) = getTypeInfo(typepckgOpt.clone(), (typeident.clone()).clone(), astDefs.clone())?;
                    typepath = makePathIdent(typepckg.clone(), (typeident.clone()).clone())?;
                    Ok(Arc::new(TypeSignature::NAMED_TYPE { name: typepath.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inTemplateTypeSignature.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFullyQualifiedTypeSignature)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lookupTupleList<Type_a: Clone + 'static + PartialEq, Type_b: Clone + 'static>(mut inList: Arc<metamodelica::List<(Type_a, Type_b)>>, mut inItemA: Type_a) -> Result<Type_b> {
    let mut outItemB: Type_b;
    outItemB = 'mc: {
        let __mc_input = (inList.clone(), inItemA.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (a, itemB), tail: _ }, itemA) => {
                    let true = (a.clone() == itemA.clone()) else { bail!("pattern mismatch") };
                    Ok(itemB.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, itemA) => {
                    Ok(lookupTupleList(rest.clone(), itemA.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outItemB)
}

fn updateTupleList<Type_a: Clone + 'static + PartialEq, Type_b: Clone + 'static>(mut inList: Arc<metamodelica::List<(Type_a, Type_b)>>, mut inTuple: (Type_a, Type_b)) -> Result<Arc<metamodelica::List<(Type_a, Type_b)>>> {
    let mut outList: Arc<metamodelica::List<(Type_a, Type_b)>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = (inList.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, (a, _)) => {
                    let _ = lookupTupleList(lst.clone(), a.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cons(inTuple.clone(), inList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

fn lookupDeleteTupleList<Type_a: Clone + 'static + PartialEq, Type_b: Clone + 'static>(mut inList: Arc<metamodelica::List<(Type_a, Type_b)>>, mut inItemA: Type_a) -> Result<(Type_b, Arc<metamodelica::List<(Type_a, Type_b)>>)> {
    let mut outItemB: Type_b;
    let mut outList: Arc<metamodelica::List<(Type_a, Type_b)>> = metamodelica::nil();
    (outItemB, outList) = 'mc: {
        let __mc_input = (inList.clone(), inItemA.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (a, itemB), tail: rest }, itemA) => {
                    let true = (a.clone() == itemA.clone()) else { bail!("pattern mismatch") };
                    Ok((itemB.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: rest }, itemA) => {
                    let mut itemB: Type_b;
                    let mut rest = (*rest).clone();
                    (itemB, rest) = lookupDeleteTupleList(rest.clone(), itemA.clone())?;
                    Ok((itemB.clone(), cons(h.clone(), rest.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outItemB, outList))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn alignTupleList<Type_a: Clone + 'static + PartialEq, Type_b: Clone + 'static, Type_c: Clone + 'static>(mut inListToAlign: Arc<metamodelica::List<(Type_a, Type_b)>>, mut inListAlignBy: Arc<metamodelica::List<(Type_a, Type_c)>>) -> Result<Arc<metamodelica::List<(Type_a, Type_b)>>> {
    let mut outAlignedList: Arc<metamodelica::List<(Type_a, Type_b)>> = metamodelica::nil();
    outAlignedList = 'mc: {
        let __mc_input = (inListToAlign.clone(), inListAlignBy.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lstAl, Deref @ metamodelica::List::Cons { head: (a, _), tail: lstBy }) => {
                    let mut b: Type_b;
                    let mut lst: Arc<metamodelica::List<(Type_a, Type_b)>> = metamodelica::nil();
                    b = lookupTupleList(lstAl.clone(), a.clone())?;
                    lst = alignTupleList(lstAl.clone(), lstBy.clone())?;
                    Ok(cons((a.clone(), b.clone()), lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lstAl, Deref @ metamodelica::List::Cons { head: _, tail: lstBy }) => {
                    let mut lst: Arc<metamodelica::List<(Type_a, Type_b)>> = metamodelica::nil();
                    lst = alignTupleList(lstAl.clone(), lstBy.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAlignedList)
}

fn listMap1Tuple22<Type_a: Clone + 'static, Type_b: Clone + 'static, Type_d: Clone + 'static, Type_c: Clone + 'static>(mut inList: Arc<metamodelica::List<(Type_a, Type_b)>>, mut inFun_Tbd_to_Tc: Arc<dyn ::std::ops::Fn(Type_b, Type_d) -> Result<Type_c> + 'static>, mut inExtraArg: Type_d) -> Result<Arc<metamodelica::List<(Type_a, Type_c)>>> {
    pub type Fun_Tbd_to_Tc<Type_b: Clone + 'static, Type_c: Clone + 'static, Type_d: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_b, Type_d) -> Result<Type_c> + 'static>;

    let mut outList: Arc<metamodelica::List<(Type_a, Type_c)>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &((inList.clone(), inFun_Tbd_to_Tc.clone(), inExtraArg.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: (a, itemB), tail: restB }, funBDtoC, extarg) => {
            let mut itemC: Type_c;
            let mut restC: Arc<metamodelica::List<(Type_a, Type_c)>> = metamodelica::nil();
            itemC = funBDtoC(itemB.clone(), extarg.clone())?;
            restC = listMap1Tuple22(restB.clone(), funBDtoC.clone(), extarg.clone())?;
            cons((a.clone(), itemC.clone()), restC.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

fn listMap2Tuple22<Type_a: Clone + 'static, Type_b: Clone + 'static, Type_d: Clone + 'static, Type_e: Clone + 'static, Type_c: Clone + 'static>(mut inList: Arc<metamodelica::List<(Type_a, Type_b)>>, mut inFun_Tbde_to_Tc: Arc<dyn ::std::ops::Fn(Type_b, Type_d, Type_e) -> Result<Type_c> + 'static>, mut inExtraArg: Type_d, mut inExtraArg2: Type_e) -> Result<Arc<metamodelica::List<(Type_a, Type_c)>>> {
    pub type Fun_Tbde_to_Tc<Type_b: Clone + 'static, Type_c: Clone + 'static, Type_d: Clone + 'static, Type_e: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_b, Type_d, Type_e) -> Result<Type_c> + 'static>;

    let mut outList: Arc<metamodelica::List<(Type_a, Type_c)>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &((inList.clone(), inFun_Tbde_to_Tc.clone(), inExtraArg.clone(), inExtraArg2.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: (a, itemB), tail: restB }, funBDEtoC, extarg, extarg2) => {
            let mut itemC: Type_c;
            let mut restC: Arc<metamodelica::List<(Type_a, Type_c)>> = metamodelica::nil();
            itemC = funBDEtoC(itemB.clone(), extarg.clone(), extarg2.clone())?;
            restC = listMap2Tuple22(restB.clone(), funBDEtoC.clone(), extarg.clone(), extarg2.clone())?;
            cons((a.clone(), itemC.clone()), restC.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

//**************************************
// *** debug output functions
//**************************************
pub fn addSusanError(mut inErrMsg: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    if Flags::isSet(Flags::FAILTRACE.clone())? {
        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error - ")); __mm_s.push_str(&*inErrMsg.clone()); ArcStr::from(__mm_s) }).clone())?;
    }
    Error::addSourceMessage(Error::SUSAN_ERROR.clone(), list![(inErrMsg.clone()).clone()], inInfo.clone())?;
    Ok(())
}

fn addSusanNotification(mut inErrMsg: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    Error::addSourceMessage(Error::SUSAN_NOTIFY.clone(), list![(inErrMsg.clone()).clone()], inInfo.clone())?;
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn canBeEscapedUnquoted(mut inStringList: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut outCanBeUnquoted: bool = false;
    outCanBeUnquoted = 'mc: {
        let __mc_input = inStringList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#str, tail: Deref @ metamodelica::List::Nil } => {
                    let true = (((r#str.clone()).clone().len() as i32) > 0) else { bail!("pattern mismatch") };
                    let true = (canBeEscapedUnquotedChars(stringListStringChar((r#str.clone()).clone()))) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#str, tail: rest @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
                    let true = (((r#str.clone()).clone().len() as i32) > 0) else { bail!("pattern mismatch") };
                    let true = (canBeEscapedUnquotedChars(stringListStringChar((r#str.clone()).clone()))) else { bail!("pattern mismatch") };
                    Ok(canBeEscapedUnquoted(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCanBeUnquoted)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn canBeEscapedUnquotedChars(mut inChars: Arc<metamodelica::List<ArcStr>>) -> bool {
    let mut outCanBeUnquoted: bool = false;
    outCanBeUnquoted = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::List::Nil => {
            true
        },
        Deref @ metamodelica::List::Cons { head: c, tail: chars } if (c.clone() == literal!("'") || c.clone() == literal!("\"") || c.clone() == literal!("?") || c.clone() == literal!("\\") || c.clone() == literal!("\n") || c.clone() == literal!("\t") || c.clone() == literal!(" ")) => {
            canBeEscapedUnquotedChars(chars.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCanBeUnquoted
}

pub fn canBeOnOneLine(mut inStringList: Arc<metamodelica::List<ArcStr>>) -> bool {
    let mut outCanBeOnOneLine: bool = false;
    outCanBeOnOneLine = (inStringList.clone().len() as i32) <= 4 && (stringAppendList(inStringList.clone()).len() as i32) <= 10;
    outCanBeOnOneLine
}

pub fn pathIdentString(mut inPathIndent: Arc<PathIdent>) -> Result<ArcStr> {
    let mut outPathIdentString: ArcStr = arcstr::literal!("");
    outPathIdentString = ('mc: {
        let __mc_input = inPathIndent.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ PathIdent::IDENT { ident } => {
                    Ok(ident.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ PathIdent::PATH_IDENT { path, ident } => {
                    let mut ident = (*ident).clone();
                    ident = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*pathIdentString(path.clone())?); ArcStr::from(__mm_s) }).clone();
                    Ok(ident.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-!!!pathIdentString failed.\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outPathIdentString)
}

pub static eTxt: std::sync::LazyLock<Tpl::Text> = std::sync::LazyLock::new(|| { Tpl::Text::MEM_TEXT { tokens: metamodelica::nil(), blocksStack: metamodelica::nil() } });

pub fn typeSignatureString(mut inTS: Arc<TypeSignature>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    let mut txt: Tpl::Text;
    txt = TplCodegen::typeSig(eTxt.clone(), inTS.clone())?;
    outStr = (Tpl::textString(txt.clone())?).clone();
    Ok(outStr)
}

pub fn mmExpString(mut inMMExp: Arc<MMExp>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    let mut txt: Tpl::Text;
    txt = TplCodegen::mmExp(eTxt.clone(), inMMExp.clone(), (literal!("=")).clone())?;
    outStr = (Tpl::textString(txt.clone())?).clone();
    Ok(outStr)
}

pub fn stmtsString(mut inStmts: Arc<metamodelica::List<Arc<MMExp>>>) -> Result<ArcStr> {
    let mut outStr: ArcStr = arcstr::literal!("");
    let mut txt: Tpl::Text;
    txt = TplCodegen::mmStatements(eTxt.clone(), inStmts.clone())?;
    outStr = (Tpl::textString(txt.clone())?).clone();
    Ok(outStr)
}

pub fn removeUnusedImports(mut pkg: MMPackage) -> Result<MMPackage> {
    let mut pkg: MMPackage = pkg;
    let mut set: Arc<AvlSetString::Tree> = Arc::new(AvlSetString::Tree::EMPTY);
    let mut name: Arc<PathIdent>;
    let mut b: bool = false;
    set = Arc::new(openmodelica_util::AvlSetString::Tree::EMPTY);
    for mut e in &*pkg.mmDeclarations.clone() {
        let mut e = e.clone();
        let _ = (match e.clone() {
        MMDeclaration::MM_FUN { .. } => {
            set = addTypedIdentsToSet(set.clone(), var_field!(e.inArgs, MMDeclaration::MM_FUN).clone())?;
            set = addTypedIdentsToSet(set.clone(), var_field!(e.outArgs, MMDeclaration::MM_FUN).clone())?;
            set = addTypedIdentsToSet(set.clone(), var_field!(e.locals, MMDeclaration::MM_FUN).clone())?;
            for mut exp in &*var_field!(e.statements, MMDeclaration::MM_FUN).clone() {
                let mut exp = exp.clone();
                set = addExpToSet(set.clone(), exp.clone())?;
            }
            ()
        },
        _ => (),
    });
    }
    pkg.mmDeclarations = {
        let mut __acc: Arc<metamodelica::List<MMDeclaration>> = metamodelica::nil();
        for mut elt in (pkg.mmDeclarations.clone()).into_iter().cloned() {
            if !((match elt.clone() {
        MMDeclaration::MM_IMPORT { packageName: ref name, .. } => {
            b = AvlSetString::hasKey(set.clone(), (getPackageIdent(name.clone())?).clone())?;
            if !(b.clone()) && Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("removeUnusedImports: ")); __mm_s.push_str(&*encodePathIdent(name.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            b.clone()
        },
        _ => true,
    })) { continue; }
            let __x = elt.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(pkg)
}

fn addTypedIdentsToSet(mut set: Arc<AvlSetString::Tree>, mut ids: TypedIdents) -> Result<Arc<AvlSetString::Tree>> {
    let mut set: Arc<AvlSetString::Tree> = set;
    let mut sig: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    for mut tpl in &*ids.clone() {
        let mut tpl = tpl.clone();
        (_, sig) = tpl.clone();
        set = addTypeSignatureToSet(set.clone(), sig.clone())?;
    }
    Ok(set)
}

fn addTypeSignatureToSet(mut set: Arc<AvlSetString::Tree>, mut sig: Arc<TypeSignature>) -> Result<Arc<AvlSetString::Tree>> {
    let mut set: Arc<AvlSetString::Tree> = set;
    let mut sig2: Arc<TypeSignature> = Arc::new(TypeSignature::BOOLEAN_TYPE);
    let mut sigs: Arc<metamodelica::List<Arc<TypeSignature>>> = metamodelica::nil();
    let mut name: Arc<PathIdent>;
    set = (::match_deref::match_deref! { match &(sig.clone()) {
        Deref @ TypeSignature::LIST_TYPE { ofType: sig2 } => addTypeSignatureToSet(set.clone(), sig2.clone())?,
        Deref @ TypeSignature::ARRAY_TYPE { ofType: sig2 } => addTypeSignatureToSet(set.clone(), sig2.clone())?,
        Deref @ TypeSignature::OPTION_TYPE { ofType: sig2 } => addTypeSignatureToSet(set.clone(), sig2.clone())?,
        Deref @ TypeSignature::TUPLE_TYPE { ofTypes: sigs } => List::foldr(sigs.clone(), (std::sync::Arc::new(addTypeSignatureToSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AvlSetString::Tree>, Arc<TypeSignature>) -> Result<Arc<AvlSetString::Tree>> + 'static>), set.clone()),
        Deref @ TypeSignature::NAMED_TYPE { name } => addPathIdentToSet(set.clone(), name.clone())?,
        _ => set.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(set)
}

fn addPathIdentToSet(mut set: Arc<AvlSetString::Tree>, mut name: Arc<PathIdent>) -> Result<Arc<AvlSetString::Tree>> {
    let mut set: Arc<AvlSetString::Tree> = set;
    set = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ PathIdent::IDENT { .. } => AvlSetString::add(set.clone(), (var_field!((*name).ident, PathIdent::IDENT).clone()).clone())?,
        Deref @ PathIdent::PATH_IDENT { .. } => AvlSetString::add(set.clone(), (var_field!((*name).ident, PathIdent::PATH_IDENT).clone()).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(set)
}

fn addExpToSet(mut set: Arc<AvlSetString::Tree>, mut exp: Arc<MMExp>) -> Result<Arc<AvlSetString::Tree>> {
    let mut set: Arc<AvlSetString::Tree> = set;
    set = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ MMExp::MM_ASSIGN { .. } => addExpToSet(set.clone(), var_field!((*exp).rhs, MMExp::MM_ASSIGN).clone())?,
        Deref @ MMExp::MM_FN_CALL { .. } => List::foldr(var_field!((*exp).args, MMExp::MM_FN_CALL).clone(), (std::sync::Arc::new(addExpToSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AvlSetString::Tree>, Arc<MMExp>) -> Result<Arc<AvlSetString::Tree>> + 'static>), addPathIdentToSet(set.clone(), var_field!((*exp).fnName, MMExp::MM_FN_CALL).clone())?),
        Deref @ MMExp::MM_IDENT { .. } => addPathIdentToSet(set.clone(), var_field!((*exp).ident, MMExp::MM_IDENT).clone())?,
        Deref @ MMExp::MM_MATCH { .. } => List::foldr(var_field!((*exp).matchCases, MMExp::MM_MATCH).clone(), (std::sync::Arc::new(fnptr!(addMatchCaseToSet, Arc<AvlSetString::Tree>, (Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AvlSetString::Tree>, (Arc<metamodelica::List<Arc<MatchingExp>>>, Arc<metamodelica::List<Arc<MMExp>>>)) -> Result<Arc<AvlSetString::Tree>> + 'static>), set.clone()),
        _ => set.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(set)
}

fn addMatchCaseToSet(mut set: Arc<AvlSetString::Tree>, mut c: MMMatchCase) -> Arc<AvlSetString::Tree> {
    let mut set: Arc<AvlSetString::Tree> = set;
    let mut mexps: Arc<metamodelica::List<Arc<MatchingExp>>> = metamodelica::nil();
    let mut exps: Arc<metamodelica::List<Arc<MMExp>>> = metamodelica::nil();
    (mexps, exps) = c.clone();
    set = List::foldr(exps.clone(), (std::sync::Arc::new(addExpToSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AvlSetString::Tree>, Arc<MMExp>) -> Result<Arc<AvlSetString::Tree>> + 'static>), set.clone());
    set = List::foldr(mexps.clone(), (std::sync::Arc::new(addMatchingExpToSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AvlSetString::Tree>, Arc<MatchingExp>) -> Result<Arc<AvlSetString::Tree>> + 'static>), set.clone());
    set
}

fn addMatchingExpToSet(mut set: Arc<AvlSetString::Tree>, mut exp: Arc<MatchingExp>) -> Result<Arc<AvlSetString::Tree>> {
    let mut set: Arc<AvlSetString::Tree> = set;
    let mut e: Arc<MatchingExp> = Arc::new(MatchingExp::NONE_MATCH);
    set = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ MatchingExp::BIND_AS_MATCH { .. } => addMatchingExpToSet(set.clone(), var_field!((*exp).matchingExp, MatchingExp::BIND_AS_MATCH).clone())?,
        Deref @ MatchingExp::RECORD_MATCH { .. } => {
            set = addPathIdentToSet(set.clone(), var_field!((*exp).tagName, MatchingExp::RECORD_MATCH).clone())?;
            for mut tpl in &*var_field!((*exp).fieldMatchings, MatchingExp::RECORD_MATCH).clone() {
                let mut tpl = tpl.clone();
                (_, e) = tpl.clone();
                set = addMatchingExpToSet(set.clone(), e.clone())?;
            }
            set.clone()
        },
        Deref @ MatchingExp::SOME_MATCH { .. } => addMatchingExpToSet(set.clone(), var_field!((*exp).value, MatchingExp::SOME_MATCH).clone())?,
        Deref @ MatchingExp::TUPLE_MATCH { .. } => List::foldr(var_field!((*exp).tupleArgs, MatchingExp::TUPLE_MATCH).clone(), (std::sync::Arc::new(addMatchingExpToSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AvlSetString::Tree>, Arc<MatchingExp>) -> Result<Arc<AvlSetString::Tree>> + 'static>), set.clone()),
        Deref @ MatchingExp::LIST_MATCH { .. } => List::foldr(var_field!((*exp).listElts, MatchingExp::LIST_MATCH).clone(), (std::sync::Arc::new(addMatchingExpToSet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<AvlSetString::Tree>, Arc<MatchingExp>) -> Result<Arc<AvlSetString::Tree>> + 'static>), set.clone()),
        Deref @ MatchingExp::LIST_CONS_MATCH { .. } => addMatchingExpToSet(addMatchingExpToSet(set.clone(), var_field!((*exp).head, MatchingExp::LIST_CONS_MATCH).clone())?, var_field!((*exp).rest, MatchingExp::LIST_CONS_MATCH).clone())?,
        _ => set.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(set)
}

