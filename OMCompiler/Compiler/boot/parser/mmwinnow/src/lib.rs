//! mmwinnow — winnow-based MetaModelica parser
//!
//! Lexer combinators are embedded in the parser — no separate tokenizer.
//! AST types mirror the ANTLR3 grammar structure from `grammars/Modelica.g`.

use winnow::{Parser, ModalResult, combinator::opt, error::{ContextError, ErrMode}};
use winnow::token::*;
use winnow::ascii;

pub struct ParserConfig {
    pub filename: String,
    pub grammar: Grammar,
}

enum Grammar {
    Modelica2,
    Modelica3,
    MetaModelica,
}

/// Custom error type with line, column, and context display.
#[derive(Debug)]
pub struct ParserError<'a> {
    /// Byte offset where parsing failed
    pub offset: usize,
    /// Remaining input at the failure point
    pub remaining: &'a str,
    /// Inner error context
    pub inner: ContextError,
    /// Full original input for context display
    _original: &'a str,
}

impl<'a> ParserError<'a> {
    /// Create a ParserError from a winnow parse error.
    pub fn from_parse_error(err: winnow::error::ParseError<&'a str, ContextError>, original: &'a str) -> Self {
        let range = err.char_span();
        let offset = range.end;  // end is the failure position
        let remaining = &original[offset..];
        let inner = err.inner().clone();
        ParserError {
            offset,
            remaining,
            inner,
            _original: original,
        }
    }

    /// Convert to a human-readable error string with line, column, and context.
    pub fn display(&self) -> String {
        let mut output = String::new();
        output.push_str("error: parsing failed\n");

        // Calculate line and column from byte offset
        let line = self._original[..self.offset].matches('\n').count() + 1;
        let col_offset = self._original[..self.offset]
            .rfind('\n')
            .map(|i| self.offset - i - 1)
            .unwrap_or(self.offset);
        let col = col_offset + 1;

        output.push_str(&format!("  --> line {}:{}\n", line, col));

        // Show the context: 2 lines before up to the error + 1 line after
        let context_start = if self.offset >= 200 {
            self.offset - 200
        } else {
            0
        };
        let ctx_end = (self.offset + 100).min(self._original.len());
        let ctx = &self._original[context_start..ctx_end];

        // Find the line containing the error within context
        let ctx_line_start = context_start
            + ctx[..].rfind('\n')
                .map(|i| i + 1)
                .unwrap_or(context_start);
        let ctx_line_end = ctx[ctx_line_start - context_start..].find('\n')
            .map(|i| ctx_line_start + i)
            .unwrap_or(self._original.len().min(ctx_end + 100));

        // Build the context snippet
        let context_line = &self._original[ctx_line_start..ctx_line_end.min(self._original.len())];
        let arrow_offset = self.offset - ctx_line_start;

        output.push_str(&format!(
            "    |\n  {} | {}\n",
            " ".repeat(line.saturating_sub(1).to_string().len()),
            context_line
        ));
        output.push_str(&format!(
            "    | {}\n",
            " ".repeat(arrow_offset) + "^"
        ));

        // Show the error message
        output.push_str(&format!("  --> remaining input: {:?}\n", self.remaining[0..(100).min(self.remaining.len())].to_string()));

        // Show what was expected (from context)
        let ctx_str = format!("{:?}", self.inner);
        if !ctx_str.is_empty() && ctx_str != "ContextError { context: [], cause: None }" {
            output.push_str(&format!("  --> reason: {}\n", ctx_str));
        }

        output
    }
}

/// Display a parse error for debugging.
pub fn print_error<'a>(
    result: Result<ParserError<'a>, winnow::error::ParseError<&'a str, ContextError>>,
) {
    match result {
        Ok(_) => println!("Parsing succeeded."),
        Err(e) => {
            let range = e.char_span();
            // We don't have the full original input here without passing it separately
            eprintln!("Parse error: char offset {}", range.end);
            eprintln!("  remaining: {:?}", &e.input()[..e.input().len().min(100)]);
            eprintln!("  inner: {:?}", e.inner());
        }
    }
}

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
pub struct StoredDefinition {
    pub classes: Vec<ClassDef>,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub encapsulated: bool,
    pub partial: bool,
    pub kind: ClassKind,
    pub specifier: ClassSpecifier,
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
pub enum ClassSpecifier {
    /// identifier class_specifier2
    Normal {
        name: String,
        spec2: ClassSpecifier2
    },
    /// EXTENDS identifier class_modification? string_comment composition END IDENT
    Extends {
        name: String,
        modification: Option<ClassModification>,
        composition: Vec<ClassPart>,
    },
}

/// class_specifier2
#[derive(Debug, Clone)]
pub enum ClassSpecifier2 {
    /// (LESS ident_list GREATER)? string_comment composition END IDENT
    Composition {
        type_vars: Vec<String>,
        comment: Option<String>,
        parts: Vec<ClassPart>,
        end_name: String,
    },
    /// EQUALS base_prefix type_specifier class_modification? comment
    TypeAlias {
        base_type: String,
        typ: TypeSpec,
        modification: Option<ClassModification>,
        comment: Option<String>,
    },
    /// EQUALS enumeration
    Enumeration(Vec<EnumLiteral>),
    /// SUBTYPEOF type_specifier
    SubTypeOf(TypeSpec),
}

/// A class part (public, protected, equation, algorithm, external, etc.)
#[derive(Debug, Clone)]
pub enum ClassPart {
    Public,
    Protected,
    Equations,
    InitialEquations,
    Algorithms,
    InitialAlgorithms,
    External {
        language: Option<String>,
        body: String,
    },
    Element(Element),
    Annotation(Annotation),
    NestedClass(ClassDef),
}

/// An element in a class body.
#[derive(Debug, Clone)]
pub enum Element {
    Component(ComponentDecl),
}

/// component_declaration
#[derive(Debug, Clone)]
pub struct ComponentDecl {
    pub typ: TypeSpec,
    pub name: String,
    pub attributes: Option<ComponentAttributes>,
}

#[derive(Debug, Clone)]
pub struct ComponentAttributes {
    pub items: Vec<ComponentItem>,
}

#[derive(Debug, Clone)]
pub enum ComponentItem {
    ComponentReference(Path),
}

/// type_prefix type_specifier_no_dims component_declaration1
#[derive(Debug, Clone)]
pub enum TypeSpec {
    Builtin(String),
    Path(Path),
    List(Box<TypeSpec>),
    Option(Box<TypeSpec>),
    Extension {
        base: Box<TypeSpec>,
        dims: Vec<Subscript>,
    },
}

#[derive(Debug, Clone)]
pub struct Path(pub Vec<String>);

#[derive(Debug, Clone)]
pub enum Subscript {
    Expr,
}

/// class_modification: ( modification (COMMA modification)* )?
#[derive(Debug, Clone)]
pub struct ClassModification {
    pub arguments: Vec<Modification>,
}

#[derive(Debug, Clone)]
pub struct Modification {
    pub name: String,
    pub value: Option<ModificationValue>,
}

#[derive(Debug, Clone)]
pub enum ModificationValue {
    Simple,
    Equals(Vec<AnnotationValue>),
}

#[derive(Debug, Clone)]
pub enum AnnotationValue {
    Ident(String),
    StringLit(String),
    Call(String, Vec<AnnotationValue>),
}

/// annotation: annotation modification
#[derive(Debug, Clone)]
pub struct Annotation {
    pub attrs: Vec<AnnotationAttr>,
}

#[derive(Debug, Clone)]
pub struct AnnotationAttr {
    pub name: String,
    pub value: Option<AnnotationValue>,
}

/// enumeration: ENUMERATION LPAR (enum_list | COLON) RPAR comment
#[derive(Debug, Clone)]
pub struct EnumLiteral {
    pub name: String,
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

fn name_path<'a>(input: &mut &'a str) -> ModalResult<String> {
    let mut parts = Vec::new();
    parts.push(opt(".").parse_next(input)?.unwrap_or("")); // allow leading dot for absolute paths
    loop {
        let tok = keyword_or_ident(input)?;
        parts.push(tok_as_ident(tok)?);
        skip_trivia(input)?;
        if opt(".").parse_next(input)?.is_none() {
            break;
        }
    }
    Ok(parts.join("."))
}

fn ident_or_fail<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    let tok = keyword_or_ident(input)?;
    tok_as_ident(tok)
}

/// Parse a class name - accepts any keyword or identifier as a class name
fn class_name<'a>(input: &mut &'a str) -> ModalResult<String> {
    skip_trivia(input)?;
    let word: &str = take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
    Ok(word.to_string())
}

/// Check if the input (after skipping trivia) starts with a class type keyword.
/// These indicate a nested class definition rather than a component declaration.
fn starts_with_class_type(input: &str) -> bool {
    let lower = input.to_lowercase();
    let keywords = [
        "package", "class", "record", "type", "function", "connector",
        "uniontype", "model", "operator", "parallel", "kernel",
        "expandable", "optimization", "enumeration",
    ];
    keywords.iter().any(|&kw| lower.starts_with(kw))
}

// ---------------------------------------------------------------------------
// Parser rules — mirror the grammar structure
// ---------------------------------------------------------------------------

/// stored_definition: BOM? (within_clause SEMICOLON)? class_definition_list EOF
pub fn stored_definition<'a>(input: &mut &'a str) -> ModalResult<StoredDefinition> {
    // BOM is optional - only consume if present
    if input.starts_with('\u{feff}') {
        let _: &str = "\u{feff}".parse_next(input)?;
    }

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
fn class_definition_list<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassDef>> {
    let mut defs = Vec::new();
    loop {
        skip_trivia(input)?;
        // Stop if we hit END (closes the enclosing class)
        if input.is_empty() {
            break;
        }
        let _final = opt("final").parse_next(input)?.is_some();
        let def= match class_definition(input) {
            Ok(d) => d,
            _ => return Ok(defs)
        };
        ";".parse_next(input)?;
        defs.push(def);
    }
    Ok(defs)
}

/// class_definition: ENCAPSULATED? PARTIAL? class_type class_specifier
fn class_definition<'a>(input: &mut &'a str) -> ModalResult<ClassDef> {
    let enc = opt("encapsulated").parse_next(input)?.is_some();
    let partial = opt("partial").parse_next(input)?.is_some();

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
fn class_specifier<'a>(input: &mut &'a str) -> ModalResult<ClassSpecifier> {
    if opt("extends").parse_next(input)?.is_some() {
        let name = name_path(input)?;
        let modification = opt(class_modification).parse_next(input)?;
        string_comments(input)?;
        let composition = composition(input)?;
        skip_trivia(input)?;
        "end".parse_next(input)?;
        if ident_or_fail(input)? != name {
            return Err(ErrMode::Backtrack(ContextError::default()));
        }
        Ok(ClassSpecifier::Extends {
            name: name.to_string(),
            modification,
            composition
        })
    } else {
        // Accept any keyword or identifier as a class name
        let name = class_name(input)?;
        let spec2 = class_specifier2(input)?;
        Ok(ClassSpecifier::Normal { name: name.to_string(), spec2 })
    }
}

/// class_specifier2: (LESS ident_list GREATER)? composition END IDENT
///                 | EQUALS base_prefix type_specifier class_modification? comment
///                 | EQUALS enumeration
///                 | SUBTYPEOF type_specifier
fn class_specifier2<'a>(input: &mut &'a str) -> ModalResult<ClassSpecifier2> {
    if opt("subtypeof").parse_next(input)?.is_some() {
        let typ = type_spec(input)?;
        return Ok(ClassSpecifier2::SubTypeOf(typ));
    }

    if opt("=").parse_next(input)?.is_some() {
        if opt("enumeration").parse_next(input)?.is_some() {
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

        // TODO: base_prefix is missing
        let typ = type_spec(input)?;
        let modification = opt(class_modification).parse_next(input)?;
        let comment = string_comments(input)?;

        return Ok(ClassSpecifier2::TypeAlias {
            base_type: "TODO".to_string(),
            typ,
            modification,
            comment,
        });
    }

    let type_vars = if opt("<").parse_next(input)?.is_some() {
        let mut vars = Vec::new();
        loop {
            skip_trivia(input)?;
            let tok = keyword_or_ident(input)?;
            vars.push(tok_as_ident(tok)?.to_string());
            skip_trivia(input)?;
            if opt(">").parse_next(input)?.is_some() {
                break;
            }
            ",".parse_next(input)?;
        }
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
        // Accept any keyword or identifier as end name
        skip_trivia(input)?;
        let word: &str = take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
        Ok(word)
    } else {
        Ok("")
    }?;

    Ok(ClassSpecifier2::Composition {
        type_vars,
        comment: None,
        parts,
        end_name: end_name.to_string(),
    })
}

fn composition<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassPart>> {
    let mut parts = element_list(input)?;

    loop {
        skip_trivia(input)?;
        if input.is_empty() || input.starts_with("end") {
            break;
        }

        if input.starts_with("public") ||
           input.starts_with("protected") ||
           input.starts_with("equation") ||
           input.starts_with("algorithm") ||
           input.starts_with("external") ||
           input.starts_with("initial") {
            parts.extend(composition2(input)?);
            break;
        }

        // Parse nested class definitions (record, type, function, etc. inside a class body)
        if starts_with_class_type(input) {
            let def = class_definition(input)?;
            skip_trivia(input)?;
            // Consume optional semicolon after nested class definition
            if input.starts_with(';') {
                ";".parse_next(input)?;
            }
            parts.push(ClassPart::NestedClass(def));
            continue;
        }

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

//   external_clause?
// | ( public_element_list
//   | protected_element_list
//   | initial_equation_clause
//   | initial_algorithm_clause
//   | equation_clause
//   | constraint_clause
//   | algorithm_clause
//   )*
fn composition2<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassPart>> {
    let mut parts = Vec::new();
    loop {
        if let Some(ext) = opt(external_part).parse_next(input)? {
            parts.push(ext);
            return Ok(parts);
        } else if opt("public").parse_next(input)?.is_some() {
            let _elts = element_list(input)?;
            parts.push(ClassPart::Public);
        } else if opt("protected").parse_next(input)?.is_some() {
            let _elts = element_list(input)?;
            parts.push(ClassPart::Protected);
        } else if opt("equation").parse_next(input)?.is_some() {
            // TODO: parse the equations
        } else if opt("initial").parse_next(input)?.is_some() {
            // TODO: parse the equations/algorithms
        } else if opt("algorithm").parse_next(input)?.is_some() {
            // TODO: parse the algorithms
        } else {
            return Err(ErrMode::Backtrack(ContextError::default()));
        }
        break;
    }
    Ok(parts)
}

fn element_list<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassPart>> {
    let mut parts = Vec::new();
    loop {
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
        // TODO: this whole function is just... wrong
    }
    Ok(parts)
}

fn component_declaration<'a>(input: &mut &'a str) -> ModalResult<Element> {
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
        name: name.to_string(),
        attributes,
    }))
}

fn component_attributes<'a>(input: &mut &'a str) -> ModalResult<ComponentAttributes> {
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

fn component_item<'a>(input: &mut &'a str) -> ModalResult<ComponentItem> {
    let path = path(input)?;
    Ok(ComponentItem::ComponentReference(path))
}

fn path<'a>(input: &mut &'a str) -> ModalResult<Path> {
    let mut parts = Vec::new();
    if input.starts_with('.') {
        ".".parse_next(input)?;
    }
    loop {
        let tok = keyword_or_ident(input)?;
        parts.push(tok_as_ident(tok)?.to_string());
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

fn class_modification<'a>(input: &mut &'a str,) -> ModalResult<ClassModification> {
    "(".parse_next(input)?;

    let mut arguments = Vec::new();
    loop {
        if input.starts_with(")") {
            break;
        }
        let m = modification(input)?;
        arguments.push(m);
        if opt(",").parse_next(input)?.is_none() {
            break;
        }
    }
    ")".parse_next(input)?;

    Ok(ClassModification { arguments })
}

fn modification<'a>(input: &mut &'a str) -> ModalResult<Modification> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    skip_trivia(input)?;

    let value = if input.starts_with('=') && !input.starts_with("==") {
        "=".parse_next(input)?;
        Some(modification_value(input)?)
    } else {
        None
    };

    Ok(Modification { name: name.to_string(), value })
}

fn modification_value<'a>(input: &mut &'a str) -> ModalResult<ModificationValue> {
    let _: &str = take_while(0.., |c: char| !",);".contains(c)).parse_next(input)?;
    Ok(ModificationValue::Simple)
}

fn annotation<'a>(input: &mut &'a str) -> ModalResult<Annotation> {
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

fn annotation_attr<'a>(input: &mut &'a str) -> ModalResult<AnnotationAttr> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    skip_trivia(input)?;

    let value = if input.starts_with('=') && !input.starts_with("==") {
        "=".parse_next(input)?;
        Some(annotation_value(input)?)
    } else {
        None
    };

    Ok(AnnotationAttr { name: name.to_string(), value })
}

fn annotation_value<'a>(input: &mut &'a str) -> ModalResult<AnnotationValue> {
    skip_trivia(input)?;
    if input.starts_with('"') {
        let _: &str = "\"".parse_next(input)?;
        let lit: &str = take_while(0.., |c: char| c != '"').parse_next(input)?;
        "\"".parse_next(input)?;
        return Ok(AnnotationValue::StringLit(lit.to_string()));
    }
    if input.starts_with('(') {
        return parse_annotation_call(input);
    }
    let tok = keyword_or_ident(input)?;
    Ok(AnnotationValue::Ident(tok_as_ident(tok)?.to_string()))
}

fn parse_annotation_call<'a>(
    input: &mut &'a str,
) -> ModalResult<AnnotationValue> {
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
    Ok(AnnotationValue::Call(name.to_string(), inner))
}

fn external_part<'a>(input: &mut &'a str) -> ModalResult<ClassPart> {
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
        let ch: &str = take_while(0.., |c: char| c != '"' && c != '\n')
            .parse_next(input)?;
        result.push_str(ch);
        if input.starts_with('\n') {
            result.push('\n');
            "\n".parse_next(input)?;
        }
    }
}

fn type_spec<'a>(input: &mut &'a str) -> ModalResult<TypeSpec> {
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

    let mut path = vec![name.to_string()];
    loop {
        skip_trivia(input)?;
        if !input.starts_with('.') || input.starts_with("..") {
            break;
        }
        ".".parse_next(input)?;
        let tok = keyword_or_ident(input)?;
        let s = tok_as_ident(tok).unwrap_or("unknown");
        path.push(s.to_string());
    }

    Ok(TypeSpec::Path(Path(path)))
}

fn enum_literal<'a>(input: &mut &'a str) -> ModalResult<EnumLiteral> {
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
    Ok(EnumLiteral { name: name.to_string(), value })
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
        if let Some(err) = &result.err() {
            assert!(false, "expected Absyn.mo to parse, got: {}", err);
        }
    }
}

// ---------------------------------------------------------------------------
// Convenience helpers
// ---------------------------------------------------------------------------

impl ClassSpecifier {
    pub fn name(&self) -> &String {
        match self {
            ClassSpecifier::Normal { name, .. } => name,
            ClassSpecifier::Extends { name, .. } => name,
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

#[cfg(test)]
mod error_debug {
    use super::*;

    #[test]
    fn debug_trace_end() {
        // Test what happens when parsing "end Within"
        let mut inp = "end Within";
        match class_definition(&mut inp) {
            Ok(def) => eprintln!("class_definition('end Within') = {:?}", def),
            Err(e) => eprintln!("class_definition('end Within') failed: {:?}", e),
        }

        // Test the full parse
        let input = "uniontype Within \"comment\"\n  record R\n  end R;\nend Within;";
        let result = stored_definition.parse(input);
        match &result {
            Ok(d) => eprintln!("Full parse succeeded, {} classes", d.classes.len()),
            Err(e) => {
                let range = e.char_span();
                eprintln!("Full parse failed at {:?}, remaining: {:?}", range, &input[range.end.min(input.len())..]);
            }
        }

        // Also test string_comments
        let code = std::fs::read_to_string("tests/data/Absyn.mo")
            .expect("Absyn.mo not found");
        let absyn_pos = code.find("encapsulated package Absyn").unwrap();
        let after_keyword = code[absyn_pos + "encapsulated package Absyn".len()..].trim_start();
        let mut input2 = after_keyword;
        match string_comments(&mut input2) {
            Ok(result) => eprintln!("string_comments: OK ({} chars)", result.as_ref().map(|s| s.len()).unwrap_or(0)),
            Err(e) => eprintln!("string_comments failed: {:?}", e),
        }

        // Test full Absyn.mo parse
        let result = stored_definition.parse(&*code);
        match &result {
            Ok(d) => eprintln!("Full Absyn parse succeeded, {} classes", d.classes.len()),
            Err(e) => {
                let range = e.char_span();
                let offset = range.end;
                let remaining_str = if offset < code.len() {
                    format!("{:?}", &code[offset..(offset+100).min(code.len())])
                } else {
                    "<EOF>".to_string()
                };
                eprintln!("Absyn parse error at byte {} (file len {}), remaining: {}",
                    offset,
                    code.len(),
                    remaining_str);
            }
        }
    }
}
