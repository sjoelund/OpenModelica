//! mmwinnow — winnow-based MetaModelica parser
//!
//! Lexer combinators are embedded in the parser — no separate tokenizer.
//! AST types come from `Absyn` module, matching the ANTLR3 grammar from `grammars/Modelica.g`.

#[allow(non_snake_case)]
mod Absyn;
mod metamodelica;

pub use Absyn::*;
use metamodelica::{List, cons, SourceInfo};
use metamodelica::List::Cons;

use winnow::{Parser, ModalResult, combinator::{opt, peek}, error::{ContextError, ErrMode}};
use winnow::token::*;
use winnow::ascii;
use std::rc::Rc;

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
    Public, Protected, Pure, Impure, External,
    Equation, Algorithm,
    // Class type modifiers
    Model, Operator, Parallel, Kernel, Expandable, Optimization,
    // Structure
    Within, Der, Code, Equality, Initial,
    // Control flow
    Else, If, For, While, Try, Elseif, ElseWhen, Return,
    Break, Continue, Match, Matchcontinue, Case,
    // Redeclaration
    Each, Replaceable, Declareunit, Constraint, Assert,
    // Enumeration
    Enumeration, Subtypeof, Pder, Overload,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let s = match self {
            Token::Algorithm => "algorithm",
            Token::Equation => "equation",
            Token::External => "external",
            Token::Enumeration => "enumeration",
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
            Token::Assert => "ASSERT",
            Token::Subtypeof => "SUBTYPEOF",
            Token::Pder => "PDER",
            Token::Overload => "OVERLOAD",
            Token::Flow => "flow",
            Token::Stream => "stream",
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
// Intermediate representation for class_specifier
// ---------------------------------------------------------------------------

/// Intermediate result of class_specifier rule.
/// The caller (class_definition) converts this to Absyn::Class.
#[derive(Debug, Clone)]
pub enum ClassSpecifier {
    /// identifier class_specifier2
    Normal {
        name: Ident,
        /// body of the class (type_vars, class_parts, annotation, etc.)
        body: Rc<ClassDef>,
    },
    /// EXTENDS identifier class_modification? composition END IDENT
    Extends {
        name: Ident,
        /// body built from class_modification + composition
        body: Rc<ClassDef>,
    },
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

fn ident<'a>(input: &mut &'a str) -> ModalResult<String> {
    if let Token::Ident(s) = keyword_or_ident(input)? {
        Ok(s.to_string())
    } else {
        Err(ErrMode::Backtrack(ContextError::default()))
    }
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
        "algorithm" => Token::Algorithm,
        "annotation" => Token::Annotation,
        "assert" => Token::Assert,
        "break" => Token::Break,
        "case" => Token::Case,
        "class" => Token::Class,
        "code" => Token::Code,
        "connector" => Token::Connector,
        "constraint" => Token::Constraint,
        "continue" => Token::Continue,
        "declareunit" => Token::Declareunit,
        "der" => Token::Der,
        "each" => Token::Each,
        "else" => Token::Else,
        "elseif" => Token::Elseif,
        "elsewhen" => Token::ElseWhen,
        "encapsulated" => Token::Encapsulated,
        "end" => Token::End,
        "enumeration" => Token::Enumeration,
        "equation" => Token::Equation,
        "equality" => Token::Equality,
        "expandable" => Token::Expandable,
        "extends" => Token::Extends,
        "final" => Token::Final,
        "flow" => Token::Flow,
        "for" => Token::For,
        "function" => Token::Function,
        "if" => Token::If,
        "import" => Token::Import,
        "impure" => Token::Impure,
        "initial" => Token::Initial,
        "kernel" => Token::Kernel,
        "match" => Token::Match,
        "matchcontinue" => Token::Matchcontinue,
        "model" => Token::Model,
        "operator" => Token::Operator,
        "optimization" => Token::Optimization,
        "overload" => Token::Overload,
        "package" => Token::Package,
        "parallel" => Token::Parallel,
        "partial" => Token::Partial,
        "pder" => Token::Pder,
        "protected" => Token::Protected,
        "public" => Token::Public,
        "pure" => Token::Pure,
        "record" => Token::Record,
        "replaceable" => Token::Replaceable,
        "return" => Token::Return,
        "stream" => Token::Stream,
        "subtypeof" => Token::Subtypeof,
        "try" => Token::Try,
        "type" => Token::Type,
        "uniontype" => Token::Uniontype,
        "while" => Token::While,
        "within" => Token::Within,
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

/// A dummy SourceInfo placeholder until line/column tracking is added.
fn dummy_info() -> SourceInfo {
    SourceInfo {
        file_name: String::new(),
        is_read_only: false,
        line_number_start: 0,
        column_number_start: 0,
        line_number_end: 0,
        column_number_end: 0,
        last_modification: 0.0,
    }
}

fn name_path<'a>(input: &mut &'a str) -> ModalResult<Path> {
    let mut parts = Vec::new();
    let fq = opt(".").parse_next(input)?.is_some();
    let mut last_id = ident(input)?;
    loop {
        if opt(".").parse_next(input)?.is_none() {
            break;
        }
        parts.push(last_id);
        last_id = ident(input)?;
    }
    let mut res = Path::IDENT { name: last_id };
    for id in parts.iter().rev() {
        res = Path::QUALIFIED {
            name: id.to_string(),
            path: Rc::new(res),
        };
    }
    if fq {
        Ok(Path::FULLYQUALIFIED { path: Rc::new(res) })
    } else {
        Ok(res)
    }
}

/// Parse a class name - accepts any keyword or identifier as a class name
fn class_name<'a>(input: &mut &'a str) -> ModalResult<String> {
    skip_trivia(input)?;
    let word: &str = take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
    Ok(word.to_string())
}

// ---------------------------------------------------------------------------
// Parser rules — mirror the grammar structure, return Absyn AST
// ---------------------------------------------------------------------------

/// within_clause: WITHIN (name_path)?
fn within_clause<'a>(input: &mut &'a str) -> ModalResult<Within> {
    "within".parse_next(input)?;
    skip_trivia(input)?;
    match opt(name_path).parse_next(input)? {
        Some(path) => Ok(Within::WITHIN { path }),
        None => Ok(Within::TOP {}),
    }
}

/// stored_definition: BOM? (within_clause SEMICOLON)? class_definition_list EOF
pub fn stored_definition<'a>(input: &mut &'a str) -> ModalResult<Program> {
    opt("\u{feff}").parse_next(input)?;

    // (within_clause SEMICOLON)?
    skip_trivia(input)?;
    let within_ = if opt("within").parse_next(input)?.is_some() {
        // Rewind: we already consumed "within", so just parse the path part
        // within_clause expects to consume "within" itself, so we need a different approach.
        // Parse the path directly here since we've already matched the keyword.
        let path = opt(name_path).parse_next(input)?;
        ";".parse_next(input)?;
        match path {
            Some(path) => Within::WITHIN { path },
            None => Within::TOP {},
        }
    } else {
        Within::TOP {}
    };

    let classes = class_definition_list(input)?;

    _ = skip_trivia(input);
    if !input.is_empty() {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }

    Ok(Program::PROGRAM { classes, within_ })
}

/// class_definition_list: (FINAL? class_definition SEMICOLON)*
fn class_definition_list<'a>(input: &mut &'a str) -> ModalResult<List<Class>> {
    let mut defs: List<Class> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }
        let _final = opt("final").parse_next(input)?.is_some();
        let def = match class_definition(input) {
            Ok(d) => d,
            _ => return Ok(defs),
        };
        ";".parse_next(input)?;
        defs = cons(def, defs);
    }
    Ok(defs.reverse())
}

/// class_definition: ENCAPSULATED? PARTIAL? class_type class_specifier
fn class_definition<'a>(input: &mut &'a str) -> ModalResult<Class> {
    let enc = opt("encapsulated").parse_next(input)?.is_some();
    let partial = opt("partial").parse_next(input)?.is_some();
    let _final = opt("final").parse_next(input)?.is_some(); // handled by caller too

    let restriction = class_type(input)?;
    let specifier = class_specifier(input)?;

    Ok(Class::CLASS {
        name: specifier.name(),
        partialPrefix: partial,
        finalPrefix: _final,
        encapsulatedPrefix: enc,
        restriction,
        body: specifier.body(),
        commentsBeforeClass: List::Nil(),
        commentsBeforeEnd: List::Nil(),
        commentsAfterEnd: List::Nil(),
        info: dummy_info(),
    })
}

/// class_type -> Restriction
fn class_type<'a>(input: &mut &'a str) -> ModalResult<Restriction> {
    let tok = keyword_or_ident(input)?;
    let restriction = match tok {
        Token::Package => Restriction::R_PACKAGE {},
        Token::Class => Restriction::R_CLASS {},
        Token::Record => Restriction::R_RECORD {},
        Token::Type => Restriction::R_TYPE {},
        Token::Function => Restriction::R_FUNCTION {
            functionRestriction: FunctionRestriction::FR_NORMAL_FUNCTION {
                purity: FunctionPurity::NO_PURITY {},
            },
        },
        Token::Connector => Restriction::R_CONNECTOR {},
        Token::Uniontype => Restriction::R_UNIONTYPE {},
        Token::Model => Restriction::R_MODEL {},
        Token::Operator => Restriction::R_OPERATOR {},
        Token::Parallel => Restriction::R_OPERATOR {}, // TODO: proper parallel restriction
        Token::Kernel => Restriction::R_OPERATOR {},   // TODO: proper kernel restriction
        Token::Optimization => Restriction::R_OPTIMIZATION {},
        _ => Restriction::R_CLASS {},
    };
    Ok(restriction)
}

/// class_specifier: identifier class_specifier2
///                 | EXTENDS identifier class_modification? composition END IDENT
fn class_specifier<'a>(input: &mut &'a str) -> ModalResult<ClassSpecifier> {
    if opt("extends").parse_next(input)?.is_some() {
        let name = ident(input)?;
        let modifications = opt(class_modification_list).parse_next(input)?.unwrap_or_else(|| List::Nil());
        string_comments(input)?;
        let classParts = composition(input)?;
        skip_trivia(input)?;
        "end".parse_next(input)?;
        if ident(input)? != name {
            return Err(ErrMode::Backtrack(ContextError::default()));
        }
        let ann: List<Annotation> = List::Nil(); // TODO: parse annotation
        Ok(ClassSpecifier::Extends {
            name,
            body: Rc::new(ClassDef::CLASS_EXTENDS {
                baseClassName: name.clone(),
                modifications,
                comment: None,
                parts: classParts,
                ann,
            }),
        })
    } else {
        let name = class_name(input)?;
        let body = class_specifier2(input)?;
        Ok(ClassSpecifier::Normal { name, body })
    }
}

/// class_specifier2: (LESS ident_list GREATER)? composition END IDENT
///                 | EQUALS base_prefix type_specifier class_modification? comment
///                 | EQUALS enumeration
///                 | SUBTYPEOF type_specifier
fn class_specifier2<'a>(input: &mut &'a str) -> ModalResult<Rc<ClassDef>> {
    if opt("subtypeof").parse_next(input)?.is_some() {
        let typeSpec = type_spec(input)?;
        let attributes = ElementAttributes::ATTR {
            flowPrefix: false,
            streamPrefix: false,
            parallelism: Parallelism::NON_PARALLEL {},
            variability: Variability::VAR {},
            direction: Direction::INPUT {}, // TODO
            isField: IsField::NONFIELD {},
            arrayDim: ArrayDim::Nil(),
        };
        return Ok(Rc::new(ClassDef::DERIVED {
            typeSpec,
            attributes,
            arguments: List::Nil(),
            comment: None,
        }));
    }

    if opt("=").parse_next(input)?.is_some() {
        if opt("enumeration").parse_next(input)?.is_some() {
            let literals = enum_list(input)?;
            return Ok(Rc::new(ClassDef::ENUMERATION {
                enumLiterals: EnumDef::ENUMLITERALS { enumLiterals: literals },
                comment: None,
            }));
        }

        // TODO: base_prefix is missing
        let typeSpec = type_spec(input)?;
        let arguments = opt(class_modification_list).parse_next(input)?.unwrap_or_else(|| List::Nil());
        let comment = string_comments(input)?.map(|c| Comment::COMMENT {
            annotation_: None,
            comment: Some(c),
        });

        return Ok(Rc::new(ClassDef::DERIVED {
            typeSpec,
            attributes: ElementAttributes::ATTR {
                flowPrefix: false,
                streamPrefix: false,
                parallelism: Parallelism::NON_PARALLEL {},
                variability: Variability::VAR {},
                direction: Direction::INPUT {},
                isField: IsField::NONFIELD {},
                arrayDim: ArrayDim::Nil(),
            },
            arguments,
            comment,
        }));
    }

    // (LESS ident_list GREATER)?
    let type_vars: List<String> = if opt("<").parse_next(input)?.is_some() {
        let mut vars: List<String> = List::Nil();
        loop {
            skip_trivia(input)?;
            let tok = keyword_or_ident(input)?;
            let id = tok_as_ident(tok)?.to_string();
            vars = cons(id, vars);
            skip_trivia(input)?;
            if opt(">").parse_next(input)?.is_some() {
                break;
            }
            ",".parse_next(input)?;
        }
        vars.reverse()
    } else {
        List::Nil()
    };

    string_comments(input)?;

    let classParts = composition(input)?;

    skip_trivia(input)?;
    let end_tok = keyword_or_ident(input)?;
    if !matches!(end_tok, Token::End) {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }
    skip_trivia(input)?;
    let _end_name = if !input.is_empty()
        && input.starts_with(|c: char| c.is_alphabetic() || c == '_')
    {
        let word: &str = take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
        word.to_string()
    } else {
        String::new()
    };

    let ann: List<Annotation> = List::Nil(); // annotation is parsed in composition
    let classAttrs: List<NamedArg> = List::Nil();

    Ok(Rc::new(ClassDef::PARTS {
        typeVars: type_vars,
        classAttrs,
        classParts,
        ann,
        comment: None, // TODO
    }))
}

/// composition: element_list composition2 (annotation SEMICOLON)?
fn composition<'a>(input: &mut &'a str) -> ModalResult<List<ClassPart>> {
    let mut parts = element_list(input)?;
    let mut comp2 = composition2(input)?;

    // Append composition2 results
    while let Cons { head, tail } = &comp2 {
        parts = cons(head.clone(), parts);
        comp2 = tail.clone();
    }
    parts = parts.reverse();
    comp2 = comp2.reverse();
    // Concatenate
    parts = parts.append(&comp2);

    // (annotation SEMICOLON)?
    skip_trivia(input)?;
    if let Some(ann) = opt(parse_annotation).parse_next(input)? {
        ";".parse_next(input)?;
        parts = cons(ClassPart::EXTERNAL {
            externalDecl: ExternalDecl::EXTERNALDECL {
                funcName: None,
                lang: None,
                output_: None,
                args: List::Nil(),
                annotation_: Some(ann),
            },
            annotation_: None,
        }, parts);
        // TODO: class-annotation should be tracked separately in ClassDef::PARTS.ann field
    }
    Ok(parts)
}

/// composition2: (public_element_list | protected_element_list |
///    initial_equation_clause | initial_algorithm_clause |
///    equation_clause | algorithm_clause | external_clause)*
fn composition2<'a>(input: &mut &'a str) -> ModalResult<List<ClassPart>> {
    skip_trivia(input)?;
    if input.is_empty() {
        return Ok(List::Nil());
    }
    let mut parts: List<ClassPart> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }

        // external_clause?
        if let Some(ext) = opt(external_part).parse_next(input)? {
            parts = cons(ext, parts);
            continue;
        }

        // public_element_list
        if opt("public").parse_next(input)?.is_some() {
            let contents = element_list(input)?;
            let tail = composition2(input)?;
            parts = cons(ClassPart::PUBLIC { contents }, parts);
            // append tail
            let mut t = tail;
            while let Cons { head, tail: rest } = &t {
                parts = cons(head.clone(), parts);
                t = rest.clone();
            }
            continue;
        }

        // protected_element_list
        if opt("protected").parse_next(input)?.is_some() {
            let contents = element_list(input)?;
            let tail = composition2(input)?;
            parts = cons(ClassPart::PROTECTED { contents }, parts);
            let mut t = tail;
            while let Cons { head, tail: rest } = &t {
                parts = cons(head.clone(), parts);
                t = rest.clone();
            }
            continue;
        }

        // initial equation/algorithm
        if opt("initial").parse_next(input)?.is_some() {
            skip_trivia(input)?;
            if opt("equation").parse_next(input)?.is_some() {
                let items = equation_section_items(input)?;
                let tail = composition2(input)?;
                parts = cons(ClassPart::INITIALEQUATIONS { contents: items }, parts);
                let mut t = tail;
                while let Cons { head, tail: rest } = &t {
                    parts = cons(head.clone(), parts);
                    t = rest.clone();
                }
            } else if opt("algorithm").parse_next(input)?.is_some() {
                let items = algorithm_section_items(input)?;
                let tail = composition2(input)?;
                parts = cons(ClassPart::INITIALALGORITHMS { contents: items }, parts);
                let mut t = tail;
                while let Cons { head, tail: rest } = &t {
                    parts = cons(head.clone(), parts);
                    t = rest.clone();
                }
            } else {
                return Err(ErrMode::Backtrack(ContextError::default()));
            }
            continue;
        }

        // equation_clause
        if opt("equation").parse_next(input)?.is_some() {
            let items = equation_section_items(input)?;
            let tail = composition2(input)?;
            parts = cons(ClassPart::EQUATIONS { contents: items }, parts);
            let mut t = tail;
            while let Cons { head, tail: rest } = &t {
                parts = cons(head.clone(), parts);
                t = rest.clone();
            }
            continue;
        }

        // algorithm_clause
        if opt("algorithm").parse_next(input)?.is_some() {
            let items = algorithm_section_items(input)?;
            let tail = composition2(input)?;
            parts = cons(ClassPart::ALGORITHMS { contents: items }, parts);
            let mut t = tail;
            while let Cons { head, tail: rest } = &t {
                parts = cons(head.clone(), parts);
                t = rest.clone();
            }
            continue;
        }

        break;
    }
    Ok(parts.reverse())
}

/// element_list: ((element | annotation | class_definition) SEMICOLON)*
fn element_list<'a>(input: &mut &'a str) -> ModalResult<List<ElementItem>> {
    let mut items: List<ElementItem> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }

        // Stop at section keywords
        if opt("public").parse_next(input)?.is_some() { break; }
        if opt("protected").parse_next(input)?.is_some() { break; }
        if opt("equation").parse_next(input)?.is_some() { break; }
        if opt("algorithm").parse_next(input)?.is_some() { break; }
        if opt("external").parse_next(input)?.is_some() { break; }
        if opt("end").parse_next(input)?.is_some() { break; }
        if opt("initial").parse_next(input)?.is_some() { break; }

        // annotation SEMICOLON
        if let Some(ann) = opt(parse_annotation).parse_next(input)? {
            ";".parse_next(input)?;
            // Store annotation as a comment element item for now
            // TODO: annotations in element_list should probably be handled differently
            items = cons(ElementItem::LEXER_COMMENT {
                comment: format!("{:?}", ann),
            }, items);
            continue;
        }

        // import_clause SEMICOLON
        if let Some(imp) = opt(import_clause).parse_next(input)? {
            ";".parse_next(input)?;
            let elem = Element::ELEMENT {
                finalPrefix: false,
                redeclareKeywords: None,
                innerOuter: InnerOuter::INNER_OUTER {},
                specification: ElementSpec::IMPORT {
                    import_: imp,
                    comment: None,
                    info: dummy_info(),
                },
                info: dummy_info(),
                constrainClass: None,
            };
            items = cons(ElementItem::ELEMENTITEM { element: elem }, items);
            continue;
        }

        // extends_clause SEMICOLON
        if let Some(ext) = opt(extends_clause).parse_next(input)? {
            ";".parse_next(input)?;
            let elem = Element::ELEMENT {
                finalPrefix: false,
                redeclareKeywords: None,
                innerOuter: InnerOuter::INNER_OUTER {},
                specification: ElementSpec::EXTENDS {
                    path: ext.path,
                    elementArg: ext.modification
                        .map(|m| m.arguments.into_iter().map(|m| Rc::new(m.into())).collect())
                        .unwrap_or_else(|| List::Nil()),
                    annotationOpt: ext.annotation_opt,
                },
                info: dummy_info(),
                constrainClass: None,
            };
            items = cons(ElementItem::ELEMENTITEM { element: elem }, items);
            continue;
        }

        // Nested class_definition SEMICOLON
        if let Some(cls) = opt(class_definition).parse_next(input)? {
            ";".parse_next(input)?;
            let elem = Element::ELEMENT {
                finalPrefix: false,
                redeclareKeywords: None,
                innerOuter: InnerOuter::INNER_OUTER {},
                specification: ElementSpec::CLASSDEF {
                    replaceable_: false,
                    class_: Rc::new(cls),
                },
                info: dummy_info(),
                constrainClass: None,
            };
            items = cons(ElementItem::ELEMENTITEM { element: elem }, items);
            continue;
        }

        // component_clause
        match component_declaration(input) {
            Ok(elem) => {
                items = cons(ElementItem::ELEMENTITEM { element: elem }, items);
                skip_trivia(input)?;
                let _: Option<&str> = opt(";").parse_next(input)?;
            }
            Err(_) => break,
        }
    }
    Ok(items.reverse())
}

fn component_declaration<'a>(input: &mut &'a str) -> ModalResult<Element> {
    // TODO: parse inner/outer/replaceable/redeclare/each/final prefixes
    // TODO: parse flow/stream/parallelism/variability/direction type_prefix
    let typeSpec = type_spec(input)?;
    let first_component = match keyword_or_ident.parse_next(input)? {
        Token::Ident(name) => {
            let arrayDim = opt(array_subscripts).parse_next(input)?.unwrap_or_else(|| ArrayDim::Nil());
            let modification = opt(modification_parens).parse_next(input)?;
            Component::COMPONENT {
                name: name.to_string(),
                arrayDim,
                modification,
            }
        }
        Token::Operator => {
            // "operator" as identifier
            let arrayDim = opt(array_subscripts).parse_next(input)?.unwrap_or_else(|| ArrayDim::Nil());
            let modification = opt(modification_parens).parse_next(input)?;
            Component::COMPONENT {
                name: "operator".to_string(),
                arrayDim,
                modification,
            }
        }
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    };

    let mut components: List<Rc<ComponentItem>> = List::Nil();
    components = cons(Rc::new(ComponentItem::COMPONENTITEM {
        component: first_component,
        condition: None,
        comment: None,
    }), components);

    // Handle comma-separated additional components
    loop {
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() {
            break;
        }
        let comp = match keyword_or_ident.parse_next(input)? {
            Token::Ident(name) => {
                let arrayDim = opt(array_subscripts).parse_next(input)?.unwrap_or_else(|| ArrayDim::Nil());
                let modification = opt(modification_parens).parse_next(input)?;
                Component::COMPONENT {
                    name: name.to_string(),
                    arrayDim,
                    modification,
                }
            }
            _ => return Err(ErrMode::Backtrack(ContextError::default())),
        };
        components = cons(Rc::new(ComponentItem::COMPONENTITEM {
            component: comp,
            condition: None,
            comment: None,
        }), components);
    }

    let attributes = ElementAttributes::ATTR {
        flowPrefix: false,
        streamPrefix: false,
        parallelism: Parallelism::NON_PARALLEL {},
        variability: Variability::VAR {},
        direction: Direction::INPUT {}, // TODO
        isField: IsField::NONFIELD {},
        arrayDim: ArrayDim::Nil(),
    };

    Ok(Element::ELEMENT {
        finalPrefix: false,
        redeclareKeywords: None,
        innerOuter: InnerOuter::INNER_OUTER {},
        specification: ElementSpec::COMPONENTS {
            attributes,
            typeSpec,
            components: components.reverse(),
        },
        info: dummy_info(),
        constrainClass: None,
    })
}

fn modification_parens<'a>(input: &mut &'a str) -> ModalResult<Modification> {
    "(".parse_next(input)?;
    let elementArgLst = modification_list(input)?;
    let eqMod = if opt("=").parse_next(input)?.is_some() {
        // TODO: parse eqMod exp
        EqMod::NOMOD {}
    } else {
        EqMod::NOMOD {}
    };
    ")".parse_next(input)?;
    Ok(Modification::CLASSMOD { elementArgLst, eqMod })
}

fn modification_list<'a>(input: &mut &'a str) -> ModalResult<List<Rc<ElementArg>>> {
    let mut args: List<Rc<ElementArg>> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.starts_with(')') || input.is_empty() {
            break;
        }
        let arg = modification_arg(input)?;
        args = cons(Rc::new(arg), args);
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() {
            break;
        }
    }
    Ok(args.reverse())
}

fn modification_arg<'a>(input: &mut &'a str) -> ModalResult<ElementArg> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    let path = Path::IDENT { name: name.to_string() };
    skip_trivia(input)?;

    let modification = if input.starts_with('=') && !input.starts_with("==") {
        "=".parse_next(input)?;
        // TODO: parse proper modification value
        Some(Modification::CLASSMOD {
            elementArgLst: List::Nil(),
            eqMod: EqMod::NOMOD {},
        })
    } else {
        None
    };

    Ok(ElementArg::MODIFICATION {
        finalPrefix: false,
        eachPrefix: Each::NON_EACH {},
        path,
        modification,
        comment: None,
        info: dummy_info(),
    })
}

/// class_modification: LPAR (modification (COMMA modification)*)? RPAR
fn class_modification_list<'a>(input: &mut &'a str) -> ModalResult<List<ElementArg>> {
    "(".parse_next(input)?;
    let mut arguments: List<ElementArg> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.starts_with(')') || input.is_empty() {
            break;
        }
        let m = modification_arg(input)?;
        arguments = cons(m, arguments);
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() {
            break;
        }
    }
    ")".parse_next(input)?;
    Ok(arguments.reverse())
}

fn parse_annotation<'a>(input: &mut &'a str) -> ModalResult<Annotation> {
    "annotation".parse_next(input)?;
    // annotation contains a class_modification
    let elementArgs = opt(class_modification_list).parse_next(input)?;
    Ok(Annotation::ANNOTATION {
        elementArgs: elementArgs
            .map(|l| l.into_iter().map(|a| Rc::new(a)).collect())
            .unwrap_or_else(|| List::Nil()),
    })
}

/// import_clause: IMPORT (explicit_import_name | implicit_import_name) comment
fn import_clause<'a>(input: &mut &'a str) -> ModalResult<Import> {
    "import".parse_next(input)?;
    let path = name_path(input)?;
    match path {
        Path::IDENT { name } => {
            if opt("=").parse_next(input)?.is_some() {
                let path = name_path(input)?;
                return Ok(Import::NAMED_IMPORT {
                    name,
                    path,
                });
            } else {
                return Ok(Import::QUAL_IMPORT {
                    path: Path::IDENT { name },
                });
            }
        }
        _ => Ok(Import::QUAL_IMPORT { path }),
    }
}

/// extends_clause: EXTENDS name_path (class_modification)? (annotation)?
fn extends_clause<'a>(input: &mut &'a str) -> ModalResult<ExtendsClause> {
    skip_trivia(input)?;
    let path = name_path(input)?;
    let modification = opt(class_modification_list).parse_next(input)?;
    let annotation_opt = opt(parse_annotation).parse_next(input)?;
    Ok(ExtendsClause {
        path,
        modification,
        annotation_opt,
    })
}

/// extends_clause intermediate
#[derive(Debug, Clone)]
struct ExtendsClause {
    path: Path,
    modification: Option<List<ElementArg>>,
    annotation_opt: Option<Annotation>,
}

/// Parse equation section items until section keyword or end
fn equation_section_items<'a>(input: &mut &'a str) -> ModalResult<List<EquationItem>> {
    let mut items: List<EquationItem> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }
        // Stop at section keywords or end
        if opt("public").parse_next(input)?.is_some() { break; }
        if opt("protected").parse_next(input)?.is_some() { break; }
        if opt("equation").parse_next(input)?.is_some() { break; }
        if opt("algorithm").parse_next(input)?.is_some() { break; }
        if opt("initial").parse_next(input)?.is_some() { break; }
        if opt("end").parse_next(input)?.is_some() { break; }
        if opt("external").parse_next(input)?.is_some() { break; }

        // TODO: parse actual equation items
        // For now, consume until ';' and store as a placeholder
        let item_text: &str = take_while(0.., |c: char| c != ';').parse_next(input)?;
        let trimmed = item_text.trim().to_string();
        if !trimmed.is_empty() {
            // Store as an equation item with a placeholder equation
            // TODO: parse the equation properly
            items = cons(EquationItem::EQUATIONITEMCOMMENT { comment: trimmed }, items);
        }
        skip_trivia(input)?;
        if input.starts_with(';') {
            ";".parse_next(input)?;
        } else {
            break;
        }
    }
    Ok(items.reverse())
}

/// Parse algorithm section items until section keyword or end
fn algorithm_section_items<'a>(input: &mut &'a str) -> ModalResult<List<AlgorithmItem>> {
    let mut items: List<AlgorithmItem> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() {
            break;
        }
        // Stop at section keywords or end
        if let Ok(tok) = peek(keyword_or_ident).parse_next(input) {
            match tok {
                Token::Public | Token::Protected | Token::Equation | Token::Algorithm |
                Token::Initial | Token::End | Token::External => break,
                _ => (),
            }
        };

        // TODO: parse actual algorithm items
        let item_text: &str = take_while(0.., |c: char| c != ';').parse_next(input)?;
        let trimmed = item_text.trim().to_string();
        if !trimmed.is_empty() {
            items = cons(AlgorithmItem::ALGORITHMITEMCOMMENT { comment: trimmed }, items);
        }
        skip_trivia(input)?;
        if input.starts_with(';') {
            ";".parse_next(input)?;
        } else {
            break;
        }
    }
    Ok(items.reverse())
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
    let path = name_path(input)?;

    let mut ts: List<Rc<TypeSpec>> = List::Nil();
    if opt("<").parse_next(input)?.is_some() {
        loop {
            if let Some(t) = opt(type_spec).parse_next(input)? {
                ts = cons(Rc::new(t), ts);
            } else {
                break;
            }
        };
        ">".parse_next(input)?;
    };
    let arrayDim = opt(array_subscripts).parse_next(input)?;
    ts = ts.reverse();
    if ts.is_empty() {
        Ok(TypeSpec::TPATH { path, arrayDim })
    } else {
        Ok(TypeSpec::TCOMPLEX {
            path,
            typeSpecs: ts,
            arrayDim,
        })
    }
}

fn array_subscripts<'a>(input: &mut &'a str) -> ModalResult<ArrayDim> {
    "[".parse_next(input)?;
    let mut subs: List<Subscript> = List::Nil();
    // TODO: handle subscripts properly
    loop {
        skip_trivia(input)?;
        if input.starts_with(']') || input.is_empty() {
            break;
        }
        subs = cons(Subscript::SUBSCRIPT {
            subscript: Rc::new(Exp::END {}), // TODO: parse subscript expr
        }, subs);
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() {
            break;
        }
    }
    "]".parse_next(input)?;
    Ok(subs.reverse())
}

fn enum_list<'a>(input: &mut &'a str) -> ModalResult<List<EnumLiteral>> {
    let mut literals: List<EnumLiteral> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty()
            || input.starts_with('|')
            || input.starts_with(',')
            || input.starts_with(';')
            || input.starts_with('"')
            || input.starts_with(')')
        {
            break;
        }
        match enum_literal(input) {
            Ok(lit) => {
                literals = cons(lit, literals);
            }
            Err(_) => break,
        }
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_some() {
            continue;
        }
        break;
    }
    Ok(literals.reverse())
}

fn enum_literal<'a>(input: &mut &'a str) -> ModalResult<EnumLiteral> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    skip_trivia(input)?;
    let _value = if input.starts_with('=') {
        "=".parse_next(input)?;
        let n: &str =
            take_while(0.., |c: char| !c.is_whitespace() && c != ',').parse_next(input)?;
        n.parse::<i32>().ok()
    } else {
        None
    };
    Ok(EnumLiteral::ENUMLITERAL {
        literal: name.to_string(),
        comment: None, // TODO
    })
}

fn external_part<'a>(input: &mut &'a str) -> ModalResult<ClassPart> {
    skip_trivia(input)?;
    // Consume the "external" keyword
    let _: &str = take_while(0.., |c: char| !c.is_whitespace()).parse_next(input)?;
    // Collect body: everything until ';'
    let mut body = String::new();
    loop {
        skip_trivia(input)?;
        if input.is_empty() || input.starts_with(';') {
            break;
        }
        if input.starts_with('\n') {
            body.push('\n');
            "\n".parse_next(input)?;
            continue;
        }
        let line: &str = take_while(1.., |c: char| c != ';' && c != '\n').parse_next(input)?;
        body.push_str(line);
    }
    ";".parse_next(input)?;
    Ok(ClassPart::EXTERNAL {
        externalDecl: ExternalDecl::EXTERNALDECL {
            funcName: Some(body),
            lang: None,
            output_: None,
            args: List::Nil(),
            annotation_: None,
        },
        annotation_: None,
    })
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
        match &result {
            Ok(Program::PROGRAM { classes, .. }) => {
                // classes is a List<Class>
                assert!(!classes.is_empty());
                if let Cons { head: class, .. } = classes {
                    if let Class::CLASS { name, .. } = &*class {
                        assert_eq!(name, "SimpleSystem");
                    } else {
                        panic!("expected CLASS variant");
                    }
                }
            }
            Err(err) => {
                assert!(false, "expected parse success, got: {}", err);
            }
        }
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
    pub fn name(&self) -> Ident {
        match self {
            ClassSpecifier::Normal { name, .. } => name.clone(),
            ClassSpecifier::Extends { name, .. } => name.clone(),
        }
    }

    pub fn body(&self) -> Rc<ClassDef> {
        match self {
            ClassSpecifier::Normal { body, .. } => body.clone(),
            ClassSpecifier::Extends { body, .. } => body.clone(),
        }
    }
}
