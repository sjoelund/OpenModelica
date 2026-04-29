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

/*
 * ANTLR4 combined grammar for MetaModelica.
 *
 * Translated from the ANTLRv3 grammar:
 *   OMCompiler/Parser/Modelica.g          (parser rules)
 *   OMCompiler/Parser/MetaModelica_Lexer.g (lexer, imports BaseModelica_Lexer.g)
 *
 * Translation assumptions:
 *   metamodelica_enabled() = true  (all MetaModelica branches included)
 *   optimica_enabled()     = false (Optimica-only class_specifier2 branch omitted)
 *   pdemodelica_enabled()  = false (FIELD/NONFIELD/INDOMAIN tokens kept but
 *                                   treated as regular keywords)
 *
 * All ANTLRv3 action blocks, return-type annotations, @init/@declarations
 * blocks, OM_PUSHZ* / OM_POP calls, and finally{} blocks have been removed.
 * The grammar is pure structure — no semantic actions.
 *
 * Multi-word end-tokens (END_IF, END_FOR, END_WHEN, END_WHILE, END_PARFOR,
 * END_TRY, END_MATCH, END_MATCHCONTINUE, END_IDENT) from the ANTLRv3 lexer
 * are replaced in parser rules by two-token sequences such as END 'if',
 * END 'for', END MATCH, END (IDENT|CODE), etc.
 */

grammar metamodelica;

/*------------------------------------------------------------------
 * PARSER RULES
 *------------------------------------------------------------------*/

stored_definition
    : BOM? (within_clause SEMICOLON)? class_definition_list? EOF
    ;

within_clause
    : WITHIN name_path?
    ;

class_definition_list
    : (FINAL? class_definition SEMICOLON) class_definition_list?
    ;

class_definition
    : ENCAPSULATED? PARTIAL? class_type class_specifier
    ;

class_type
    : CLASS
    | OPTIMIZATION
    | MODEL
    | RECORD
    | BLOCK
    | EXPANDABLE? CONNECTOR
    | TYPE
    | T_PACKAGE
    | (PURE | IMPURE)? (OPERATOR | T_PARALLEL | T_KERNEL)? FUNCTION
    | UNIONTYPE
    | OPERATOR RECORD?
    ;

identifier
    : IDENT | DER | CODE | EQUALITY | INITIAL
    ;

class_specifier
    : identifier class_specifier2
    | EXTENDS identifier class_modification? string_comment composition END (IDENT | CODE)
    ;

class_specifier2
    : (LESS ident_list GREATER)? string_comment composition END (IDENT | CODE)
    | EQUALS base_prefix type_specifier class_modification? comment
    | EQUALS enumeration
    | EQUALS pder
    | EQUALS overloading
    | SUBTYPEOF type_specifier
    ;

pder
    : DER LPAR name_path COMMA ident_list RPAR comment
    ;

ident_list
    : IDENT (COMMA ident_list)?
    ;

overloading
    : OVERLOAD LPAR name_list RPAR comment
    ;

base_prefix
    : type_prefix
    ;

name_list
    : name_path (COMMA name_list)?
    ;

enumeration
    : ENUMERATION LPAR (enum_list | COLON) RPAR comment
    ;

enum_list
    : enumeration_literal (COMMA enum_list)?
    ;

enumeration_literal
    : IDENT comment
    ;

composition
    : element_list composition2 (annotation SEMICOLON)?
    ;

composition2
    : external_clause?
    | (  public_element_list
       | protected_element_list
       | initial_equation_clause
       | initial_algorithm_clause
       | equation_clause
       | constraint_clause
       | algorithm_clause
       ) composition2
    ;

external_clause
    : EXTERNAL language_specification?
        ((component_reference EQUALS)? IDENT LPAR expression_list? RPAR)?
        annotation? SEMICOLON
    ;

public_element_list
    : PUBLIC element_list
    ;

protected_element_list
    : PROTECTED element_list
    ;

language_specification
    : STRING
    ;

element_list
    : ((element | annotation) SEMICOLON)*
    ;

element
    : import_clause
    | extends_clause
    | REDECLARE? FINAL? INNER? T_OUTER?
        ( (class_definition | component_clause)
        | REPLACEABLE (class_definition | component_clause) constraining_clause_comment?
        )
    ;

import_clause
    : IMPORT (explicit_import_name | implicit_import_name) comment
    ;

explicit_import_name
    : (IDENT | CODE) EQUALS name_path
    ;

implicit_import_name
    : name_path_star
    ;

extends_clause
    : EXTENDS name_path class_or_inheritance_modification? annotation?
    ;

constraining_clause_comment
    : constraining_clause comment
    ;

constraining_clause
    : EXTENDS name_path class_modification?
    | CONSTRAINEDBY name_path class_modification?
    ;

component_clause
    : type_prefix type_specifier component_list
    ;

type_prefix
    : (FLOW | STREAM)? (T_LOCAL | T_GLOBAL)? (DISCRETE | PARAMETER | CONSTANT)?
      T_INPUT? T_OUTPUT? (FIELD | NONFIELD)?
    ;

type_specifier
    : name_path (LESS type_specifier_list GREATER)? array_subscripts?
    ;

type_specifier_no_dims
    : name_path (LESS type_specifier_list GREATER)?
    ;

type_specifier_list
    : type_specifier (COMMA type_specifier_list)?
    ;

component_list
    : component_declaration (COMMA component_list)?
    ;

component_declaration
    : declaration conditional_attribute? comment
    ;

conditional_attribute
    : IF expression
    ;

declaration
    : (IDENT | OPERATOR) array_subscripts? modification?
    ;

modification
    : class_modification (EQUALS modification_expression)?
    | EQUALS modification_expression
    | ASSIGN modification_expression
    ;

modification_expression
    : expression
    | BREAK
    ;

class_modification
    : LPAR argument_list? RPAR
    ;

class_or_inheritance_modification
    : LPAR argument_list_inh? RPAR
    ;

argument_list
    : argument (COMMA argument_list)?
    ;

argument_list_inh
    : (argument | inheritance_modification) (COMMA argument_list_inh)?
    ;

inheritance_modification
    : BREAK (connect_clause | IDENT)
    ;

argument
    : element_modification_or_replaceable
    | element_redeclaration
    ;

element_modification_or_replaceable
    : EACH? FINAL? (element_modification | element_replaceable)
    ;

element_modification
    : name_path2 modification? string_comment
    ;

element_redeclaration
    : REDECLARE EACH? FINAL?
        ((class_definition | component_clause1) | element_replaceable)
    ;

element_replaceable
    : REPLACEABLE (class_definition | component_clause1) constraining_clause_comment?
    ;

component_clause1
    : base_prefix type_specifier_no_dims component_declaration1
    ;

component_declaration1
    : declaration comment
    ;

/*
 * initial_equation_clause and initial_algorithm_clause use a semantic
 * predicate to distinguish INITIAL EQUATION from INITIAL used as an
 * expression (e.g. initial()).  The ANTLRv3 grammar used LA(2) to
 * look ahead one token past INITIAL.
 */
initial_equation_clause
    : {_input.LT(2).getType()==EQUATION}? INITIAL EQUATION equation_annotation_list
    ;

equation_clause
    : EQUATION equation_annotation_list
    ;

constraint_clause
    : CONSTRAINT constraint_annotation_list
    ;

equation_annotation_list
    : ((equation | annotation) SEMICOLON)*
    ;

constraint_annotation_list
    : ((constraint_item | annotation) SEMICOLON)*
    ;

algorithm_clause
    : T_ALGORITHM algorithm_annotation_list
    ;

initial_algorithm_clause
    : {_input.LT(2).getType()==T_ALGORITHM}? INITIAL T_ALGORITHM algorithm_annotation_list
    ;

algorithm_annotation_list
    : ((algorithm | annotation) SEMICOLON)*
    ;

equation
    : equality_or_noretcall_equation comment
    | conditional_equation_e comment
    | for_clause_e comment
    | parfor_clause_e comment
    | connect_clause comment
    | when_clause_e comment
    | FAILURE LPAR equation RPAR comment
    | EQUALITY LPAR expression EQUALS expression RPAR comment
    ;

constraint_item
    : simple_expr comment
    | conditional_equation_a comment
    | for_clause_a comment
    | parfor_clause_a comment
    | while_clause comment
    | when_clause_a comment
    | BREAK comment
    | RETURN comment
    | CONTINUE comment
    | FAILURE LPAR algorithm RPAR comment
    | EQUALITY LPAR expression ASSIGN expression RPAR comment
    ;

algorithm
    : assign_clause_a comment
    | conditional_equation_a comment
    | for_clause_a comment
    | parfor_clause_a comment
    | while_clause comment
    | try_clause comment
    | when_clause_a comment
    | BREAK comment
    | RETURN comment
    | CONTINUE comment
    | FAILURE LPAR algorithm RPAR comment
    | EQUALITY LPAR expression ASSIGN expression RPAR comment
    ;

assign_clause_a
    : simple_expression ((ASSIGN | EQUALS) expression)?
    ;

equality_or_noretcall_equation
    : simple_expression ((EQUALS | ASSIGN) expression (INDOMAIN component_reference)?)?
    ;

conditional_equation_e
    : IF expression THEN equation_list equation_elseif_list? (ELSE equation_list)? END 'if'
    ;

conditional_equation_a
    : IF expression THEN algorithm_list algorithm_elseif_list? (ELSE algorithm_list)? END 'if'
    ;

for_clause_e
    : FOR for_indices LOOP equation_list END 'for'
    ;

for_clause_a
    : FOR for_indices LOOP algorithm_list END 'for'
    ;

parfor_clause_e
    : PARFOR for_indices LOOP equation_list END 'parfor'
    ;

parfor_clause_a
    : PARFOR for_indices LOOP algorithm_list END 'parfor'
    ;

while_clause
    : WHILE expression LOOP algorithm_list END 'while'
    ;

try_clause
    : TRY algorithm_list ELSE algorithm_list END 'try'
    ;

when_clause_e
    : WHEN expression THEN equation_list else_when_e* END 'when'
    ;

else_when_e
    : ELSEWHEN expression THEN equation_list
    ;

when_clause_a
    : WHEN expression THEN algorithm_list else_when_a* END 'when'
    ;

else_when_a
    : ELSEWHEN expression THEN algorithm_list
    ;

equation_elseif_list
    : equation_elseif+
    ;

equation_elseif
    : ELSEIF expression THEN equation_list
    ;

algorithm_elseif_list
    : algorithm_elseif+
    ;

algorithm_elseif
    : ELSEIF expression THEN algorithm_list
    ;

equation_list
    : (equation SEMICOLON)*
    ;

algorithm_list
    : (algorithm SEMICOLON)*
    ;

connect_clause
    : CONNECT LPAR component_reference COMMA component_reference RPAR
    ;

expression
    : if_expression
    | simple_expression
    | code_expression
    | match_expression
    | part_eval_function_expression
    ;

part_eval_function_expression
    : FUNCTION component_reference LPAR named_arguments? RPAR
    ;

if_expression
    : IF expression THEN expression elseif_expression* ELSE expression
    ;

elseif_expression
    : ELSEIF expression THEN expression
    ;

for_indices
    : for_index (COMMA for_index)*
    ;

for_index
    : IDENT ((IF | GUARD) expression)? (T_IN expression)?
    ;

simple_expression
    : simple_expr (COLONCOLON simple_expression)?
    | IDENT AS simple_expression
    ;

simple_expr
    : logical_expression (COLON logical_expression (COLON logical_expression)?)?
    ;

logical_expression
    : logical_term (T_OR logical_term)*
    ;

logical_term
    : logical_factor (T_AND logical_factor)*
    ;

logical_factor
    : T_NOT? relation
    ;

relation
    : arithmetic_expression
        ((LESS | LESSEQ | GREATER | GREATEREQ | EQEQ | LESSGT) arithmetic_expression)?
    ;

arithmetic_expression
    : unary_arithmetic_expression ((PLUS | MINUS | PLUS_EW | MINUS_EW) term)*
    ;

unary_arithmetic_expression
    : PLUS term
    | MINUS term
    | PLUS_EW term
    | MINUS_EW term
    | term
    ;

term
    : factor ((STAR | SLASH | STAR_EW | SLASH_EW) factor)*
    ;

factor
    : primary ((POWER | POWER_EW) primary)?
    ;

primary
    : UNSIGNED_INTEGER
    | UNSIGNED_REAL
    | STRING
    | T_FALSE
    | T_TRUE
    | component_reference__function_call
    | DER function_call
    | PURE function_call
    | LPAR output_expression_list array_subscripts? RPAR
    | LBRACK matrix_expression_list RBRACK
    | LBRACE for_or_expression_list RBRACE
    | END
    ;

matrix_expression_list
    : expression_list (SEMICOLON expression_list)*
    ;

component_reference__function_call
    : component_reference LESS name_list GREATER function_call
    | component_reference function_call? (DOT expression)?
    | INITIAL LPAR RPAR
    ;

name_path_end
    : name_path EOF
    ;

name_path
    : DOT? name_path2
    ;

name_path2
    : (IDENT | CODE) (DOT name_path2)?
    ;

name_path_star
    : (IDENT | CODE) (STAR_EW | DOT LBRACE name_path_group RBRACE | DOT name_path_star)?
    ;

name_path_group
    : (IDENT | CODE) (EQUALS (IDENT | CODE))? (COMMA name_path_group)?
    ;

component_reference_end
    : component_reference EOF
    ;

component_reference
    : DOT? component_reference2
    | ALLWILD
    | WILD
    ;

component_reference2
    : (IDENT | OPERATOR) array_subscripts? (DOT component_reference2)?
    ;

function_call
    : LPAR function_arguments RPAR
    ;

function_arguments
    : for_or_expression_list named_arguments?
    ;

for_or_expression_list
    : expression (COMMA expression)* (THREADED? FOR for_indices)?
    |
    ;

named_arguments
    : named_argument (COMMA named_argument)*
    ;

named_argument
    : (IDENT | OPERATOR) EQUALS expression
    ;

output_expression_list
    : expression? (COMMA expression?)*
    ;

expression_list
    : expression (COMMA expression)*
    ;

array_subscripts
    : LBRACK subscript (COMMA subscript)* RBRACK
    ;

subscript
    : expression
    | COLON
    ;

comment
    : string_comment annotation?
    ;

string_comment
    : (STRING (PLUS STRING)*)?
    ;

annotation
    : T_ANNOTATION class_modification
    ;

code_expression
    : CODE LPAR
        ( INITIAL? ((EQUATION code_equation_clause) | (CONSTRAINT code_constraint_clause) | (T_ALGORITHM code_algorithm_clause))
        | modification
        | expression
        | element SEMICOLON?
        ) RPAR
    | CODE_NAME LPAR name_path RPAR
    | CODE_ANNOTATION class_modification
    | CODE_EXP LPAR expression RPAR
    | CODE_VAR LPAR component_reference RPAR
    ;

code_equation_clause
    : (equation SEMICOLON)*
    ;

code_constraint_clause
    : (equation SEMICOLON)*
    ;

code_algorithm_clause
    : (algorithm SEMICOLON)*
    ;

match_expression
    : MATCHCONTINUE expression string_comment local_clause cases END MATCHCONTINUE
    | MATCH expression string_comment local_clause cases END MATCH
    ;

local_clause
    : (LOCAL element_list)?
    ;

cases
    : onecase cases2
    ;

cases2
    : onecase cases2
    | ELSE string_comment local_clause
        ((EQUATION equation_list_then) | (T_ALGORITHM algorithm_annotation_list))?
        THEN expression SEMICOLON
    |
    ;

equation_list_then
    : (equation SEMICOLON)*
    ;

onecase
    : CASE pattern ((IF | GUARD) expression)? string_comment local_clause
        ((EQUATION equation_list_then) | (T_ALGORITHM algorithm_annotation_list))?
        THEN expression SEMICOLON
    ;

pattern
    : expression
    ;

top_algorithm
    : expression SEMICOLON?
    | (  top_assign_clause_a
       | conditional_equation_a
       | for_clause_a
       | parfor_clause_a
       | while_clause
       | try_clause
       ) comment
    ;

top_assign_clause_a
    : simple_expression ASSIGN expression
    ;

interactive_stmt
    : BOM? interactive_stmt_list SEMICOLON? EOF
    ;

interactive_stmt_list
    : top_algorithm (SEMICOLON top_algorithm)*
    ;


/*------------------------------------------------------------------
 * LEXER RULES
 *------------------------------------------------------------------*/

/*
 * Keywords — must appear before IDENT so that the ANTLR4 lexer gives them
 * priority on exact-length matches.
 */

T_ALGORITHM   : 'algorithm';
T_AND         : 'and';
T_ANNOTATION  : 'annotation';
AS            : 'as';
BLOCK         : 'block';
BREAK         : 'break';
CASE          : 'case';
CLASS         : 'class';
CONNECT       : 'connect';
CONNECTOR     : 'connector';
CONSTANT      : 'constant';
CONSTRAINT    : 'constraint';
CONSTRAINEDBY : 'constrainedby';
CONTINUE      : 'continue';
DISCRETE      : 'discrete';
DER           : 'der';
EACH          : 'each';
ELSE          : 'else';
ELSEIF        : 'elseif';
ELSEWHEN      : 'elsewhen';
ENCAPSULATED  : 'encapsulated';
END           : 'end';
ENUMERATION   : 'enumeration';
EQUATION      : 'equation';
EQUALITY      : 'equality';
EXPANDABLE    : 'expandable';
EXTENDS       : 'extends';
EXTERNAL      : 'external';
T_FALSE       : 'false';
FAILURE       : 'failure';
FINAL         : 'final';
FIELD         : 'field';
FLOW          : 'flow';
FOR           : 'for';
FUNCTION      : 'function';
GUARD         : 'guard';
IF            : 'if';
IMPORT        : 'import';
IMPURE        : 'impure';
T_IN          : 'in';
INDOMAIN      : 'indomain';
INITIAL       : 'initial';
INNER         : 'inner';
T_INPUT       : 'input';
LOCAL         : 'local';
LOOP          : 'loop';
MATCH         : 'match';
MATCHCONTINUE : 'matchcontinue';
MODEL         : 'model';
T_NOT         : 'not';
NONFIELD      : 'nonfield';
OPERATOR      : 'operator';
OPTIMIZATION  : 'optimization';
T_OR          : 'or';
T_OUTER       : 'outer';
T_OUTPUT      : 'output';
T_PACKAGE     : 'package';
PARFOR        : 'parfor';
T_PARALLEL    : 'parallel';
T_LOCAL       : 'parlocal';
T_GLOBAL      : 'parglobal';
T_KERNEL      : 'parkernel';
PARAMETER     : 'parameter';
PARTIAL       : 'partial';
PROTECTED     : 'protected';
PUBLIC        : 'public';
PURE          : 'pure';
RECORD        : 'record';
REDECLARE     : 'redeclare';
REPLACEABLE   : 'replaceable';
RETURN        : 'return';
STREAM        : 'stream';
SUBTYPEOF     : 'subtypeof';
THEN          : 'then';
THREADED      : 'threaded';
T_TRUE        : 'true';
TRY           : 'try';
TYPE          : 'type';
UNIONTYPE     : 'uniontype';
WHEN          : 'when';
WHILE         : 'while';
WITHIN        : 'within';

/*
 * OpenModelica MetaModelica extensions — start with '$' so they must come
 * before IDENT (which also allows '$' in its body via NONDIGIT or the
 * literal '$cpuTime' alternative).
 */
OVERLOAD         : '$overload';
CODE             : '$Code';
CODE_NAME        : '$TypeName';
CODE_EXP         : '$Expression';
CODE_ANNOTATION  : '$annotation';
CODE_VAR         : '$Var';

/*
 * Multi-character operators — longer alternatives listed first so that ANTLR4
 * maximal-munch picks the right token (e.g. ':=' before ':', '::' before ':',
 * '.+' before '.').
 */
ASSIGN      : ':=';
COLONCOLON  : '::';
LESSEQ      : '<=';
LESSGT      : '<>';
GREATEREQ   : '>=';
EQEQ        : '==';
PLUS_EW     : '.+';
MINUS_EW    : '.-';
STAR_EW     : '.*';
SLASH_EW    : './';
POWER_EW    : '.^';

/* Single-character operators */
DOT         : '.';
LPAR        : '(';
RPAR        : ')';
LBRACK      : '[';
RBRACK      : ']';
LBRACE      : '{';
RBRACE      : '}';
EQUALS      : '=';
COMMA       : ',';
COLON       : ':';
SEMICOLON   : ';';
MOD         : '%';
STAR        : '*';
MINUS       : '-';
PLUS        : '+';
LESS        : '<';
GREATER     : '>';
POWER       : '^';
SLASH       : '/';

/*
 * ALLWILD ('__') and WILD ('_') must be declared BEFORE IDENT so that when
 * the lexer sees exactly '_' or '__' it chooses these tokens (same-length
 * tie-breaking by rule order).  Longer inputs like '_abc' still match IDENT
 * via maximal munch, because NONDIGIT includes '_'.
 */
ALLWILD     : '__';
WILD        : '_';

/* UTF-8 byte-order mark */
BOM : '\u00EF\u00BB\u00BF';

/* Whitespace and comments — routed to the HIDDEN channel */
WS          : [ \t\r\n]+              -> channel(HIDDEN);
LINE_COMMENT: '//' ~[\r\n]*          -> channel(HIDDEN);
ML_COMMENT  : '/*' .*? '*/'          -> channel(HIDDEN);

/*
 * Identifiers.
 *
 * NONDIGIT includes '_' (matching the original ANTLRv3 grammar).  Bare '_'
 * and '__' are still tokenised as WILD / ALLWILD because those rules appear
 * earlier in this file, giving them priority when the match length is equal.
 *
 * The special identifier '$cpuTime' is handled as a literal alternative
 * so it does not require '$' to be in NONDIGIT.
 *
 * Quoted identifiers use single-quote delimiters and allow the same character
 * set as in the Modelica specification (QCHAR | SESCAPE).
 */
IDENT
    : NONDIGIT (NONDIGIT | DIGIT)*
    | '$cpuTime'
    | '\'' (QCHAR | SESCAPE) (QCHAR | SESCAPE)* '\''
    ;

fragment NONDIGIT : [_a-zA-Z];
fragment DIGIT    : [0-9];

fragment QCHAR
    : DIGIT | NONDIGIT
    | '!' | '#' | '$' | '%' | '&' | '(' | ')' | '*' | '+' | ',' | '-' | '.'
    | '/' | ':' | ';' | '<' | '>' | '=' | '?' | '@' | '[' | ']' | '^'
    | '{' | '}' | '|' | '~' | ' ' | '"'
    ;

fragment SESCAPE
    : '\\' ( '\'' | '"' | '\\' | '?' | 'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' )
    ;

/*
 * String literals.  SCHAR covers everything except backslash, double-quote,
 * and bare CR/LF (which must go through SESCAPE or the NL alternative).
 */
fragment SCHAR : [\r\n] | ~[\r\n\\'"];

STRING : '"' (SCHAR | SESCAPE)* '"';

/*
 * Numeric literals.
 *
 * UNSIGNED_REAL must be tried before UNSIGNED_INTEGER so that e.g. "1.0"
 * matches UNSIGNED_REAL (the first rule to match the full token wins in
 * ANTLR4 when both would produce the same length — rule order decides).
 *
 * The '.'-prefix form (".5", ".5e2") is also handled here; note that
 * element-wise operator tokens (.+, .-, .*, ./, .^) are declared before
 * this rule, so DOT followed by an operator character will not be consumed
 * by UNSIGNED_REAL.
 */
fragment EXPONENT : [eE] [+\-]? DIGIT+;

UNSIGNED_REAL
    : DIGIT+ '.' DIGIT* EXPONENT?
    | DIGIT+ EXPONENT
    | '.' DIGIT+ EXPONENT?
    ;

UNSIGNED_INTEGER
    : DIGIT+
    ;
