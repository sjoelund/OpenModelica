//! mmwinnow — winnow-based MetaModelica parser
//!
//! Lexer combinators are embedded in the parser — no separate tokenizer.
//! AST types mirror the ANTLR3 grammar structure from `grammars/Modelica.g`.

use winnow::{Parser, ModalResult, error::{ContextError, ErrMode}};
use winnow::token::*;
use winnow::ascii;

// ---------------------------------------------------------------------------
// Token types — mirrors the ANTLR3 token set from Modelica.g / MetaModelica_Lexer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    // Keywords — class types
    Package, Class, Record, Type, Function, Connector, Uniontype,
    Encapsulated, Partial, Final, Extends, End, Annotation, Import,
    Public, Protected, Pure, Impure,
    // Class type modifiers
    Model, Operator, Parallel, Kernel, Expandable, Optimization,
    // Structure
    Within, Der, Code, Equality, Initial,
    // Control flow
    Else, If, For, While, Try, Elseif, ElseWhen, Return,
    Break, Continue, Match, Matchcontinue, Case,
    // Redeclaration
    Each, Replaceable, Declareunit, Constraint, Assume, Assert,
    // MetaModelica extensions
    Println, Printerr, Print, Readln, Read,
    Throw, Throwmsg, Throwfmt,
    Matchcase,
    // Enumeration
    Enum, Subtypeof, Pder, Overload, Enumerations,
    // Connector
    Flow, Stream,
    // Literals
    Ident(&'a str),
    StringLit(&'a str),
    IntLit(&'a str),
    RealLit(&'a str),
    // Operators
    Equal, Assign, EqEq,
    Less, Leq, Greater, Geq, NotEq,
    // Delimiters
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Dot, DotDot, Colon, Semi, Comma,
    Star, Plus, Minus, Slash, Power, Pipe,
    // Special
    BOM,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::Package => "PACKAGE",
            Token::Class => "CLASS",
            Token::Record => "RECORD",
            Token::Type => "TYPE",
            Token::Function => "FUNCTION",
            Token::Connector => "CONNECTOR",
            Token::Uniontype => "UNIONTYPE",
            Token::Encapsulated => "ENCAPSULATED",
            Token::Partial => "PARTIAL",
            Token::Final => "FINAL",
            Token::Extends => "EXTENDS",
            Token::End => "END",
            Token::Annotation => "ANNOTATION",
            Token::Import => "IMPORT",
            Token::Public => "PUBLIC",
            Token::Protected => "PROTECTED",
            Token::Pure => "PURE",
            Token::Impure => "IMPURE",
            Token::Model => "MODEL",
            Token::Operator => "OPERATOR",
            Token::Parallel => "PARALLEL",
            Token::Kernel => "KERNEL",
            Token::Expandable => "EXPANDABLE",
            Token::Optimization => "OPTIMIZATION",
            Token::Within => "WITHIN",
            Token::Der => "DER",
            Token::Code => "CODE",
            Token::Equality => "EQUALITY",
            Token::Initial => "INITIAL",
            Token::Else => "ELSE",
            Token::If => "IF",
            Token::For => "FOR",
            Token::While => "WHILE",
            Token::Try => "TRY",
            Token::Elseif => "ELSEIF",
            Token::ElseWhen => "ELSEWHEN",
            Token::Return => "RETURN",
            Token::Break => "BREAK",
            Token::Continue => "CONTINUE",
            Token::Match => "MATCH",
            Token::Matchcontinue => "MATCHCONTINUE",
            Token::Case => "CASE",
            Token::Each => "EACH",
            Token::Replaceable => "REPLACEABLE",
            Token::Declareunit => "DECLAREUNIT",
            Token::Constraint => "CONSTRAINT",
            Token::Assume => "ASSUME",
            Token::Assert => "ASSERT",
            Token::Println => "PRINTLN",
            Token::Printerr => "PRINTERR",
            Token::Print => "PRINT",
            Token::Readln => "READLN",
            Token::Read => "READ",
            Token::Throw => "THROW",
            Token::Throwmsg => "THROWMSG",
            Token::Throwfmt => "THROWFMT",
            Token::Matchcase => "MATCHCASE",
            Token::Enum => "ENUM",
            Token::Subtypeof => "SUBTYPEOF",
            Token::Pder => "PDER",
            Token::Overload => "OVERLOAD",
            Token::Enumerations => "ENUMERATIONS",
            Token::Flow => "FLOW",
            Token::Stream => "STREAM",
            Token::Ident(s) => return write!(f, "IDENT({s:?})"),
            Token::StringLit(s) => return write!(f, "STRING({s:?})"),
            Token::IntLit(s) => return write!(f, "INT({s:?})"),
            Token::RealLit(s) => return write!(f, "REAL({s:?})"),
            Token::Equal => "=",
            Token::Assign => ":=",
            Token::EqEq => "==",
            Token::Less => "<",
            Token::Leq => "<=",
            Token::Greater => ">",
            Token::Geq => ">=",
            Token::NotEq => "<>",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBracket => "[",
            Token::RBracket => "]",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Dot => ".",
            Token::DotDot => "..",
            Token::Colon => ":",
            Token::Semi => ";",
            Token::Comma => ",",
            Token::Star => "*",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Slash => "/",
            Token::Power => "^",
            Token::Pipe => "|",
            Token::BOM => "BOM",
        };
        f.write_str(s)
    }
}

// ---------------------------------------------------------------------------
// AST types — mirror the grammar structure
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct StoredDefinition<'a> {
    pub classes: Vec<ClassDef<'a>>,
}

#[derive(Debug, Clone)]
pub struct ClassDef<'a> {
    pub encapsulated: bool,
    pub partial: bool,
    pub kind: ClassKind,
    pub specifier: ClassSpecifier<'a>,
}

#[derive(Debug, Clone)]
pub enum ClassKind {
    Package,
    Class,
    Record,
    Type,
    Function,
    Connector,
    Uniontype,
    Model,
    Operator,
    Parallel,
    Kernel,
    Expansion,
    Optimization,
}

/// class_specifier
#[derive(Debug, Clone)]
pub enum ClassSpecifier<'a> {
    /// identifier class_specifier2
    Normal {
        name: &'a str,
        spec2: ClassSpecifier2<'a>,
    },
    /// EXTENDS identifier class_modification? string_comment composition END IDENT
    Extends {
        base_name: &'a str,
        modification: Option<ClassModification<'a>>,
        composition: Vec<ClassPart<'a>>,
        end_name: &'a str,
    },
}

/// class_specifier2
#[derive(Debug, Clone)]
pub enum ClassSpecifier2<'a> {
    /// (LESS ident_list GREATER)? string_comment composition END IDENT
    Composition {
        type_vars: Vec<&'a str>,
        comment: Option<String>,
        parts: Vec<ClassPart<'a>>,
        end_name: &'a str,
    },
    /// EQUALS base_prefix type_specifier class_modification? comment
    TypeAlias {
        base_type: &'a str,
        typ: TypeSpec<'a>,
        modification: Option<ClassModification<'a>>,
        comment: Option<String>,
    },
    /// EQUALS enumeration
    Enumeration(Vec<EnumLiteral<'a>>),
    /// SUBTYPEOF type_specifier
    SubTypeOf(TypeSpec<'a>),
}

/// A class part (public, protected, equation, algorithm, external, etc.)
#[derive(Debug, Clone)]
pub enum ClassPart<'a> {
    Public,
    Protected,
    Equations,
    InitialEquations,
    Algorithms,
    InitialAlgorithms,
    External {
        language: Option<&'a str>,
        body: String,
    },
    Element(Element<'a>),
    Annotation(Annotation<'a>),
}

/// An element in a class body.
#[derive(Debug, Clone)]
pub enum Element<'a> {
    Component(ComponentDecl<'a>),
}

/// component_declaration
#[derive(Debug, Clone)]
pub struct ComponentDecl<'a> {
    pub typ: TypeSpec<'a>,
    pub name: &'a str,
    pub attributes: Option<ComponentAttributes<'a>>,
}

#[derive(Debug, Clone)]
pub struct ComponentAttributes<'a> {
    pub items: Vec<ComponentItem<'a>>,
}

#[derive(Debug, Clone)]
pub enum ComponentItem<'a> {
    ComponentReference(Path<'a>),
}

/// type_prefix type_specifier_no_dims component_declaration1
#[derive(Debug, Clone)]
pub enum TypeSpec<'a> {
    Builtin(&'a str),
    Path(Path<'a>),
    List(Box<TypeSpec<'a>>),
    Option(Box<TypeSpec<'a>>),
    Extension {
        base: Box<TypeSpec<'a>>,
        dims: Vec<Subscript>,
    },
}

#[derive(Debug, Clone)]
pub struct Path<'a>(pub Vec<&'a str>);

#[derive(Debug, Clone)]
pub enum Subscript {
    Expr,
}

/// class_modification: ( modification (COMMA modification)* )?
#[derive(Debug, Clone)]
pub struct ClassModification<'a> {
    pub arguments: Vec<Modification<'a>>,
}

#[derive(Debug, Clone)]
pub struct Modification<'a> {
    pub name: &'a str,
    pub value: Option<ModificationValue<'a>>,
}

#[derive(Debug, Clone)]
pub enum ModificationValue<'a> {
    Simple,
    Equals(Vec<AnnotationValue<'a>>),
}

#[derive(Debug, Clone)]
pub enum AnnotationValue<'a> {
    Ident(&'a str),
    StringLit(&'a str),
    Call(&'a str, Vec<AnnotationValue<'a>>),
}

/// annotation: annotation modification
#[derive(Debug, Clone)]
pub struct Annotation<'a> {
    pub attrs: Vec<AnnotationAttr<'a>>,
}

#[derive(Debug, Clone)]
pub struct AnnotationAttr<'a> {
    pub name: &'a str,
    pub value: Option<AnnotationValue<'a>>,
}

/// enumeration: ENUMERATION LPAR (enum_list | COLON) RPAR comment
#[derive(Debug, Clone)]
pub struct EnumLiteral<'a> {
    pub name: &'a str,
    pub value: Option<i64>,
}

// ---------------------------------------------------------------------------
// Lexer helpers
// ---------------------------------------------------------------------------

fn skip_ws<'a>(input: &mut &'a str) -> ModalResult<()> {
    take_while(0.., |c: char| c.is_whitespace()).parse_next(input)?;
    Ok(())
}

fn skip_trivia<'a>(input: &mut &'a str) -> ModalResult<()> {
    loop {
        skip_ws(input)?;
        if input.is_empty() {
            break;
        }
        let before = *input;
        if input.starts_with("//") {
            take_while(0.., |c: char| c != '\n' && c != '\r').parse_next(input)?;
            let _: ModalResult<char> = ascii::newline.parse_next(input);
        } else if input.starts_with("/*") {
            take_until(0.., "*/").parse_next(input)?;
            "*/".parse_next(input)?;
        }
        if *input == before {
            break;
        }
    }
    Ok(())
}

fn keyword_or_ident<'a>(input: &mut &'a str) -> ModalResult<Token<'a>> {
    skip_trivia(input)?;
    let word: &str =
        take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
    Ok(token_from_word(word))
}

fn token_from_word<'a>(word: &'a str) -> Token<'a> {
    let w = word.to_ascii_lowercase();
    match w.as_str() {
        "package" => Token::Package,
        "class" => Token::Class,
        "record" => Token::Record,
        "type" => Token::Type,
        "function" => Token::Function,
        "connector" => Token::Connector,
        "uniontype" => Token::Uniontype,
        "encapsulated" => Token::Encapsulated,
        "partial" => Token::Partial,
        "final" => Token::Final,
        "extends" => Token::Extends,
        "end" => Token::End,
        "annotation" => Token::Annotation,
        "import" => Token::Import,
        "public" => Token::Public,
        "protected" => Token::Protected,
        "pure" => Token::Pure,
        "impure" => Token::Impure,
        "model" => Token::Model,
        "operator" => Token::Operator,
        "parallel" => Token::Parallel,
        "kernel" => Token::Kernel,
        "expandable" => Token::Expandable,
        "optimization" => Token::Optimization,
        "within" => Token::Within,
        "der" => Token::Der,
        "code" => Token::Code,
        "equality" => Token::Equality,
        "initial" => Token::Initial,
        "else" => Token::Else,
        "if" => Token::If,
        "for" => Token::For,
        "while" => Token::While,
        "try" => Token::Try,
        "elseif" => Token::Elseif,
        "elsewhen" => Token::ElseWhen,
        "return" => Token::Return,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "match" => Token::Match,
        "matchcontinue" => Token::Matchcontinue,
        "case" => Token::Case,
        "each" => Token::Each,
        "replaceable" => Token::Replaceable,
        "declareunit" => Token::Declareunit,
        "constraint" => Token::Constraint,
        "assume" => Token::Assume,
        "assert" => Token::Assert,
        "println" => Token::Println,
        "printerr" => Token::Printerr,
        "print" => Token::Print,
        "readln" => Token::Readln,
        "read" => Token::Read,
        "throw" => Token::Throw,
        "throwmsg" => Token::Throwmsg,
        "throwfmt" => Token::Throwfmt,
        "matchcase" => Token::Matchcase,
        "enumeration" => Token::Enum,
        "subtypeof" => Token::Subtypeof,
        "pder" => Token::Pder,
        "overload" => Token::Overload,
        "enumerations" => Token::Enumerations,
        "flow" => Token::Flow,
        "stream" => Token::Stream,
        "=" => Token::Equal,
        ":=" => Token::Assign,
        "==" => Token::EqEq,
        "<" => Token::Less,
        "<=" => Token::Leq,
        ">" => Token::Greater,
        ">=" => Token::Geq,
        "!=" => Token::NotEq,
        "(" => Token::LParen,
        ")" => Token::RParen,
        "[" => Token::LBracket,
        "]" => Token::RBracket,
        "{" => Token::LBrace,
        "}" => Token::RBrace,
        "." => Token::Dot,
        ".." => Token::DotDot,
        ":" => Token::Colon,
        ";" => Token::Semi,
        "," => Token::Comma,
        "*" => Token::Star,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "/" => Token::Slash,
        "^" => Token::Power,
        "|" => Token::Pipe,
        _ => Token::Ident(word),
    }
}

fn tok_as_ident<'a>(tok: Token<'a>) -> ModalResult<&'a str> {
    match tok {
        Token::Ident(s) => Ok(s),
        _ => Err(ErrMode::Backtrack(ContextError::default())),
    }
}

fn ident_or_fail<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let tok = keyword_or_ident(input)?;
    tok_as_ident(tok)
}

fn skip_type_prefix<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let tok = keyword_or_ident(input)?;
    tok_as_ident(tok)
}

// ---------------------------------------------------------------------------
// Parser rules — mirror the grammar structure
// ---------------------------------------------------------------------------

/// stored_definition: BOM? (within_clause SEMICOLON)? class_definition_list EOF
pub fn stored_definition<'a>(input: &mut &'a str) -> ModalResult<StoredDefinition<'a>> {
    let _: ModalResult<&str> = "\u{feff}".parse_next(input);

    // (within_clause SEMICOLON)?
    if input.starts_with("within") || input.starts_with("WITHIN") {
        let _: &str = take_while(0.., |c: char| c != ';').parse_next(input)?;
        ";".parse_next(input)?;
    }

    let classes = class_definition_list(input)?;

    if !input.is_empty() {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }

    Ok(StoredDefinition { classes })
}

/// class_definition_list: (FINAL? class_definition SEMICOLON)*
fn class_definition_list<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassDef<'a>>> {
    let mut defs = Vec::new();
    loop {
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }
        // FINAL?
        if input.starts_with("final") || input.starts_with("FINAL") {
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
        }
        let def = class_definition(input)?;
        skip_trivia(input)?;
        ";".parse_next(input)?;
        defs.push(def);
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }
    }
    Ok(defs)
}

/// class_definition: ENCAPSULATED? PARTIAL? class_type class_specifier
fn class_definition<'a>(input: &mut &'a str) -> ModalResult<ClassDef<'a>> {
    let enc = input.starts_with("encapsulated") || input.starts_with("ENCAPSULATED");
    if enc {
        take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
    }
    let partial = input.starts_with("partial") || input.starts_with("PARTIAL");
    if partial {
        take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
    }

    let kind = class_type(input)?;
    let specifier = class_specifier(input)?;

    Ok(ClassDef {
        encapsulated: enc,
        partial,
        kind,
        specifier,
    })
}

/// class_type: CLASS | RECORD | TYPE | T_PACKAGE | FUNCTION variants | UNIONTYPE | OPERATOR variants | ...
fn class_type<'a>(input: &mut &'a str) -> ModalResult<ClassKind> {
    let tok = keyword_or_ident(input)?;
    let kind = match tok {
        Token::Package => ClassKind::Package,
        Token::Class => ClassKind::Class,
        Token::Record => ClassKind::Record,
        Token::Type => ClassKind::Type,
        Token::Function => ClassKind::Function,
        Token::Connector => ClassKind::Connector,
        Token::Uniontype => ClassKind::Uniontype,
        Token::Model => ClassKind::Model,
        Token::Operator => ClassKind::Operator,
        Token::Parallel => ClassKind::Parallel,
        Token::Kernel => ClassKind::Kernel,
        Token::Optimization => ClassKind::Optimization,
        _ => ClassKind::Class, // fallback
    };
    Ok(kind)
}

/// class_specifier: identifier class_specifier2
///                 | EXTENDS identifier class_modification? composition END IDENT
fn class_specifier<'a>(input: &mut &'a str) -> ModalResult<ClassSpecifier<'a>> {
    let lower = input.to_lowercase();
    if lower.starts_with("extends") {
        let _: &str = take_while(0.., |c: char| !c.is_whitespace()).parse_next(input)?;
        let base_name = ident_or_fail(input)?;
        let modification = class_modification(input)?;
        string_comments(input)?;
        let composition = composition(input)?;
        skip_trivia(input)?;
        let end_tok = keyword_or_ident(input)?;
        if !matches!(end_tok, Token::End) {
            return Err(ErrMode::Backtrack(ContextError::default()));
        }
        skip_trivia(input)?;
        let end_name = if !input.is_empty()
            && input.starts_with(|c: char| c.is_alphabetic() || c == '_')
        {
            ident_or_fail(input)?
        } else {
            ""
        };
        Ok(ClassSpecifier::Extends {
            base_name,
            modification,
            composition,
            end_name,
        })
    } else {
        let name = ident_or_fail(input)?;
        let spec2 = class_specifier2(input)?;
        Ok(ClassSpecifier::Normal { name, spec2 })
    }
}

/// class_specifier2: (LESS ident_list GREATER)? composition END IDENT
///                 | EQUALS base_prefix type_specifier class_modification? comment
///                 | EQUALS enumeration
///                 | SUBTYPEOF type_specifier
fn class_specifier2<'a>(input: &mut &'a str) -> ModalResult<ClassSpecifier2<'a>> {
    skip_trivia(input)?;

    if input.starts_with("subtypeof") || input.starts_with("SUBTYPEOF") {
        take_while(0.., |c: char| !c.is_whitespace()).parse_next(input)?;
        let typ = type_spec(input)?;
        return Ok(ClassSpecifier2::SubTypeOf(typ));
    }

    if input.starts_with('=') {
        if input.starts_with("==") {
            "==".parse_next(input)?;
        } else {
            "=".parse_next(input)?;
        }

        if input.starts_with("enumeration") || input.starts_with("ENUMERATION") {
            let _: &str = take_while(0.., |c: char| !c.is_whitespace()).parse_next(input)?;
            let mut literals = Vec::new();
            loop {
                skip_trivia(input)?;
                if input.starts_with('|') || input.starts_with(',') || input.starts_with(';') {
                    break;
                }
                if input.is_empty() {
                    break;
                }
                if let Ok(lit) = enum_literal(input) {
                    literals.push(lit);
                } else {
                    break;
                }
                skip_trivia(input)?;
                if input.starts_with(',') {
                    ",".parse_next(input)?;
                }
            }
            return Ok(ClassSpecifier2::Enumeration(literals));
        }

        let base_type = skip_type_prefix(input)?;
        let typ = type_spec(input)?;
        let modification = class_modification(input)?;
        let comment = string_comments(input)?;

        return Ok(ClassSpecifier2::TypeAlias {
            base_type,
            typ,
            modification,
            comment,
        });
    }

    let type_vars = if input.starts_with('<') {
        "<".parse_next(input)?;
        let mut vars = Vec::new();
        loop {
            skip_trivia(input)?;
            if input.starts_with('>') {
                break;
            }
            let tok = keyword_or_ident(input)?;
            vars.push(tok_as_ident(tok)?);
            skip_trivia(input)?;
            if input.starts_with(',') {
                ",".parse_next(input)?;
            }
        }
        ">".parse_next(input)?;
        vars
    } else {
        Vec::new()
    };

    string_comments(input)?;

    let parts = composition(input)?;

    skip_trivia(input)?;
    let end_tok = keyword_or_ident(input)?;
    if !matches!(end_tok, Token::End) {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }
    skip_trivia(input)?;
    let end_name = if !input.is_empty()
        && input.starts_with(|c: char| c.is_alphabetic() || c == '_')
    {
        ident_or_fail(input)?
    } else {
        ""
    };

    Ok(ClassSpecifier2::Composition {
        type_vars,
        comment: None,
        parts,
        end_name,
    })
}

fn composition<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassPart<'a>>> {
    let mut parts = element_list(input)?;
    let mut loop_count = 0u32;

    loop {
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }
        if input.starts_with("end") || input.starts_with("END") {
            break;
        }

        let lower = input.to_lowercase();
        if lower.starts_with("public") {
            parts.push(ClassPart::Public);
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if lower.starts_with("protected") {
            parts.push(ClassPart::Protected);
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if lower.starts_with("equation") {
            parts.push(ClassPart::Equations);
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if lower.starts_with("algorithm") {
            parts.push(ClassPart::Algorithms);
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if lower.starts_with("external") {
            let part = external_part(input)?;
            parts.push(part);
            continue;
        }

        // If we hit END, break out (class_specifier2 will consume it)
        if lower.starts_with("end") {
            break;
        }

        let before = *input;
        // Try to parse a component
        if let Ok(elem) = component_declaration(input) {
            parts.push(ClassPart::Element(elem));
        } else {
            // Fallback: skip to next ';'
            let _: &str = take_while(0.., |c: char| !";\n".contains(c)).parse_next(input)?;
            if input.starts_with(';') {
                ";".parse_next(input)?;
            }
        }

        loop_count += 1;
        if loop_count > 100 {
            break;
        }
        if *input == before {
            break;
        }

        skip_trivia(input)?;
        while input.starts_with(';') {
            ";".parse_next(input)?;
            skip_trivia(input)?;
        }
    }

    skip_trivia(input)?;
    if input.starts_with("annotation") || input.starts_with("ANNOTATION") {
        let ann = annotation(input)?;
        skip_trivia(input)?;
        ";".parse_next(input)?;
        parts.push(ClassPart::Annotation(ann));
    }

    Ok(parts)
}

fn element_list<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassPart<'a>>> {
    let mut parts = Vec::new();
    let mut loop_count = 0u32;
    loop {
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }
        if input.starts_with("end") || input.starts_with("END") {
            break;
        }

        let lower = input.to_lowercase();
        if lower.starts_with("public") {
            parts.push(ClassPart::Public);
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if lower.starts_with("protected") {
            parts.push(ClassPart::Protected);
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if lower.starts_with("equation") {
            parts.push(ClassPart::Equations);
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if lower.starts_with("algorithm") {
            parts.push(ClassPart::Algorithms);
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if lower.starts_with("external") {
            let part = external_part(input)?;
            parts.push(part);
            continue;
        }
        if lower.starts_with("annotation") {
            let ann = annotation(input)?;
            parts.push(ClassPart::Annotation(ann));
            continue;
        }

        if let Ok(elem) = component_declaration(input) {
            parts.push(ClassPart::Element(elem));
            continue;
        }

        let before = *input;
        let _: &str = take_while(0.., |c: char| !";\n".contains(c)).parse_next(input)?;
        if input.starts_with(';') {
            ";".parse_next(input)?;
        }

        loop_count += 1;
        if loop_count > 100 {
            break;
        }
        if *input == before {
            break;
        }
    }
    Ok(parts)
}

fn component_declaration<'a>(input: &mut &'a str) -> ModalResult<Element<'a>> {
    let typ = type_spec(input)?;
    skip_trivia(input)?;
    let name = if input.starts_with(|c: char| c.is_alphabetic() || c == '_') {
        Some(ident_or_fail(input)?)
    } else {
        None
    };
    let name = name.unwrap_or("unnamed");
    skip_trivia(input)?;
    let attributes = if input.starts_with('(') {
        Some(component_attributes(input)?)
    } else {
        None
    };

    Ok(Element::Component(ComponentDecl {
        typ,
        name,
        attributes,
    }))
}

fn component_attributes<'a>(input: &mut &'a str) -> ModalResult<ComponentAttributes<'a>> {
    "(".parse_next(input)?;
    let mut items = Vec::new();
    if !input.starts_with(')') {
        loop {
            skip_trivia(input)?;
            if input.starts_with(')') {
                break;
            }
            let item = component_item(input)?;
            items.push(item);
            skip_trivia(input)?;
            if input.starts_with(',') {
                ",".parse_next(input)?;
            }
        }
    }
    ")".parse_next(input)?;
    Ok(ComponentAttributes { items })
}

fn component_item<'a>(input: &mut &'a str) -> ModalResult<ComponentItem<'a>> {
    let path = path(input)?;
    Ok(ComponentItem::ComponentReference(path))
}

fn path<'a>(input: &mut &'a str) -> ModalResult<Path<'a>> {
    let mut parts = Vec::new();
    if input.starts_with('.') {
        ".".parse_next(input)?;
    }
    loop {
        let tok = keyword_or_ident(input)?;
        parts.push(tok_as_ident(tok)?);
        skip_trivia(input)?;
        if input.starts_with('.') {
            if input.starts_with("..") {
                break;
            }
            if input.starts_with(".*") || input.starts_with(".(") {
                break;
            }
            ".".parse_next(input)?;
            continue;
        }
        break;
    }
    Ok(Path(parts))
}

fn class_modification<'a>(
    input: &mut &'a str,
) -> ModalResult<Option<ClassModification<'a>>> {
    skip_trivia(input)?;
    if !input.starts_with('(') {
        return Ok(None);
    }
    "(".parse_next(input)?;

    let mut arguments = Vec::new();
    if !input.starts_with(')') {
        loop {
            skip_trivia(input)?;
            if input.starts_with(')') {
                break;
            }
            let m = modification(input)?;
            arguments.push(m);
            skip_trivia(input)?;
            if input.starts_with(',') {
                ",".parse_next(input)?;
            }
        }
    }

    ")".parse_next(input)?;
    Ok(Some(ClassModification { arguments }))
}

fn modification<'a>(input: &mut &'a str) -> ModalResult<Modification<'a>> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    skip_trivia(input)?;

    let value = if input.starts_with('=') && !input.starts_with("==") {
        "=".parse_next(input)?;
        Some(modification_value(input)?)
    } else {
        None
    };

    Ok(Modification { name, value })
}

fn modification_value<'a>(input: &mut &'a str) -> ModalResult<ModificationValue<'a>> {
    let _: &str = take_while(0.., |c: char| !",);".contains(c)).parse_next(input)?;
    Ok(ModificationValue::Simple)
}

fn annotation<'a>(input: &mut &'a str) -> ModalResult<Annotation<'a>> {
    let _: &str = take_while(0.., |c: char| !c.is_whitespace()).parse_next(input)?;
    let mut attrs = Vec::new();

    loop {
        skip_trivia(input)?;
        if input.is_empty() || input.starts_with(';') {
            break;
        }
        let attr = annotation_attr(input)?;
        attrs.push(attr);
        skip_trivia(input)?;
        if input.starts_with(',') {
            ",".parse_next(input)?;
        }
    }

    Ok(Annotation { attrs })
}

fn annotation_attr<'a>(input: &mut &'a str) -> ModalResult<AnnotationAttr<'a>> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    skip_trivia(input)?;

    let value = if input.starts_with('=') && !input.starts_with("==") {
        "=".parse_next(input)?;
        Some(annotation_value(input)?)
    } else {
        None
    };

    Ok(AnnotationAttr { name, value })
}

fn annotation_value<'a>(input: &mut &'a str) -> ModalResult<AnnotationValue<'a>> {
    skip_trivia(input)?;
    if input.starts_with('"') {
        let _: &str = "\"".parse_next(input)?;
        let lit: &str = take_while(0.., |c: char| c != '"').parse_next(input)?;
        "\"".parse_next(input)?;
        return Ok(AnnotationValue::StringLit(lit));
    }
    if input.starts_with('(') {
        return parse_annotation_call(input);
    }
    let tok = keyword_or_ident(input)?;
    Ok(AnnotationValue::Ident(tok_as_ident(tok)?))
}

fn parse_annotation_call<'a>(
    input: &mut &'a str,
) -> ModalResult<AnnotationValue<'a>> {
    "(".parse_next(input)?;
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    let mut inner = Vec::new();
    if !input.starts_with(')') {
        loop {
            skip_trivia(input)?;
            if input.starts_with(')') {
                break;
            }
            inner.push(annotation_value(input)?);
            skip_trivia(input)?;
            if input.starts_with(',') {
                ",".parse_next(input)?;
            }
        }
    }
    ")".parse_next(input)?;
    Ok(AnnotationValue::Call(name, inner))
}

fn external_part<'a>(input: &mut &'a str) -> ModalResult<ClassPart<'a>> {
    // Consume the "external" keyword
    let _: &str = take_while(0.., |c: char| !c.is_whitespace()).parse_next(input)?;
    // Collect body: everything until ';'
    let mut body = String::new();
    while !input.starts_with(';') && !input.is_empty() {
        if input.starts_with('\n') {
            body.push('\n');
            "\n".parse_next(input)?;
        } else {
            let s: &str = take_while(1.., |c: char| c != ';' && c != '\n').parse_next(input)?;
            body.push_str(s);
        }
    }
    ";".parse_next(input)?;
    Ok(ClassPart::External {
        language: None,
        body,
    })
}

fn string_comments<'a>(input: &mut &'a str) -> ModalResult<Option<String>> {
    let mut parts = Vec::new();
    loop {
        skip_trivia(input)?;
        if !input.starts_with('"') {
            break;
        }
        let text = skip_string_comment_text(input)?;
        parts.push(text);
    }
    if parts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(parts.join(" ")))
    }
}

fn skip_string_comment_text(input: &mut &str) -> ModalResult<String> {
    let _: &str = "\"".parse_next(input)?;
    let mut result = String::new();
    loop {
        let r: ModalResult<&str> = "\"".parse_next(input);
        if r.is_ok() {
            return Ok(result);
        }
        if input.starts_with("\\\"") {
            "\"".parse_next(input)?;
            let ch: &str =
                take_while(1..4, |c: char| c.is_alphabetic() || c.is_ascii_digit())
                    .parse_next(input)?;
            let _: &str = ";".parse_next(input)?;
            let c = ch.chars().next().unwrap_or('?');
            result.push(c);
            continue;
        }
        let ch: &str = take_while(1.., |c: char| c != '"' && c != '\n')
            .parse_next(input)?;
        result.push_str(ch);
        if input.starts_with('\n') {
            result.push('\n');
            "\n".parse_next(input)?;
        }
    }
}

fn type_spec<'a>(input: &mut &'a str) -> ModalResult<TypeSpec<'a>> {
    skip_trivia(input)?;

    let tok = keyword_or_ident(input)?;
    match tok {
        Token::Ident(name) if name == "list" => {
            "<".parse_next(input)?;
            let inner = type_spec(input)?;
            skip_trivia(input)?;
            ">".parse_next(input)?;
            return Ok(TypeSpec::List(Box::new(inner)));
        }
        Token::Ident(name) if name == "option" => {
            "<".parse_next(input)?;
            let inner = type_spec(input)?;
            skip_trivia(input)?;
            ">".parse_next(input)?;
            return Ok(TypeSpec::Option(Box::new(inner)));
        }
        _ => {}
    }

    let name = tok_as_ident(tok.clone()).unwrap_or("unknown");

    let mut path = vec![name];
    loop {
        skip_trivia(input)?;
        if !input.starts_with('.') || input.starts_with("..") {
            break;
        }
        ".".parse_next(input)?;
        let tok = keyword_or_ident(input)?;
        let s = tok_as_ident(tok).unwrap_or("unknown");
        path.push(s);
    }

    if path.len() == 1 {
        match path[0] {
            "real" => Ok(TypeSpec::Builtin("REAL")),
            "integer" => Ok(TypeSpec::Builtin("INTEGER")),
            "boolean" => Ok(TypeSpec::Builtin("BOOLEAN")),
            "string" => Ok(TypeSpec::Builtin("STRING")),
            _ => {
                // Unknown type — treat as builtin with the name as-is
                Ok(TypeSpec::Builtin(path[0]))
            }
        }
    } else {
        Ok(TypeSpec::Path(Path(path)))
    }
}

fn enum_literal<'a>(input: &mut &'a str) -> ModalResult<EnumLiteral<'a>> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    skip_trivia(input)?;
    let value = if input.starts_with('=') {
        "=".parse_next(input)?;
        let n: &str =
            take_while(0.., |c: char| !c.is_whitespace() && c != ',').parse_next(input)?;
        n.parse().ok()
    } else {
        None
    };
    Ok(EnumLiteral { name, value })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_whitespace() {
        let mut input = "   \n\tpackage Foo";
        super::skip_ws(&mut input).unwrap();
        assert_eq!(input, "package Foo");
    }

    #[test]
    fn skip_line_comment() {
        let mut input = "// comment\npackage Foo";
        super::skip_trivia(&mut input).unwrap();
        assert_eq!(input, "package Foo");
    }

    #[test]
    fn skip_block_comment() {
        let mut input = "/* comment */package Foo";
        super::skip_trivia(&mut input).unwrap();
        assert_eq!(input, "package Foo");
    }

    #[test]
    fn keyword_package() {
        let mut input = "package Foo";
        let tok = super::keyword_or_ident(&mut input).unwrap();
        assert_eq!(tok, Token::Package);
    }

    #[test]
    fn ident_simple_system() {
        let mut input = "SimpleSystem \"Returns the index...\"\nend SimpleSystem;";
        let tok = super::keyword_or_ident(&mut input).unwrap();
        assert_eq!(tok, Token::Ident("SimpleSystem"));
    }

    #[test]
    fn parse_simple_package() {
        let code = "package SimpleSystem \"Returns the index...\"\n\
                    /* ... */\n\
                    Real x(start=0);\n\
                    end SimpleSystem;";
        let result = stored_definition.parse(code);
        assert!(result.is_ok(), "expected parse success, got: {:?}", result);
        let def = result.unwrap();
        assert_eq!(def.classes.len(), 1);
        let class = &def.classes[0];
        assert_eq!(class.specifier.name(), "SimpleSystem");
    }

    #[test]
    fn parse_first_token() {
        let code = "package SimpleSystem \"Returns the index...\"\nend SimpleSystem;";
        let result = stored_definition.parse(code);
        assert!(result.is_ok(), "expected parse success, got: {:?}", result);
    }

    #[test]
    fn parse_absyn() {
        let code = std::fs::read_to_string("tests/data/Absyn.mo")
            .expect("Absyn.mo not found");
        let result = stored_definition.parse(&*code);
        assert!(result.is_ok(), "expected Absyn.mo to parse, got: {:?}", result);
    }
}

// ---------------------------------------------------------------------------
// Convenience helpers
// ---------------------------------------------------------------------------

impl<'a> ClassSpecifier<'a> {
    pub fn name(&self) -> &'a str {
        match self {
            ClassSpecifier::Normal { name, .. } => *name,
            ClassSpecifier::Extends { base_name, .. } => *base_name,
        }
    }
}

#[cfg(test)]
mod debug_tests {
    use super::*;

    #[test]
    fn debug_parse() {
        let code = "package SimpleSystem end SimpleSystem;";

        // Parse step by step
        let mut input = code;

        // BOM
        let _: ModalResult<&str> = "\u{feff}".parse_next(&mut input);

        // class_type
        let _kind = class_type(&mut input).unwrap();

        // class_specifier
        let _spec = class_specifier(&mut input).unwrap();

        let result = stored_definition.parse(code);
        assert!(result.is_ok());
    }

    #[test]
    fn debug_element_list() {
        let mut input = "end SimpleSystem;";
        let _result = element_list(&mut input);
    }

    #[test]
    fn debug_composition() {
        let mut input = "end SimpleSystem;";
        let _result = composition(&mut input);
    }

    #[test]
    fn debug_end_token() {
        let mut input = "end SimpleSystem;";
        let _tok = keyword_or_ident(&mut input);
    }
}

#[cfg(test)]
mod hang_debug {
    use super::*;

    #[test]
    fn debug_simple_package() {
        let code = "package SimpleSystem \"Returns the index...\"\nend SimpleSystem;";
        let result = stored_definition.parse(code);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod hang_debug2 {
    use super::*;

    #[test]
    fn debug_simple_package_full() {
        let code = "package SimpleSystem \"Returns the index...\"\n\
                    /* ... */\n\
                    Real x(start=0);\n\
                    end SimpleSystem;";
        let result = stored_definition.parse(code);
        assert!(result.is_ok());
    }
}
