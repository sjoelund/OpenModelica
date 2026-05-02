//! mmwinnow — winnow-based MetaModelica parser
//!
//! Lexer combinators are embedded in the parser — no separate tokenizer.
//! AST types come from `Absyn` module, matching the ANTLR3 grammar from `grammars/Modelica.g`.
#![allow(non_snake_case)]

mod Absyn;
mod metamodelica;

pub use Absyn::*;
use metamodelica::{List, cons, SourceInfo};

use winnow::stream::Stream;
use winnow::{Parser, ModalResult, combinator::{opt, alt, peek, cut_err}, error::{ContextError, StrContext, StrContextValue, ErrMode}};
use winnow::token::*;
use winnow::ascii;
use std::rc::Rc;

pub struct ParserConfig {
    pub filename: String,
    pub grammar: Grammar,
}

pub enum Grammar {
    Modelica2,
    Modelica3,
    MetaModelica,
}

/// Custom error type with line, column, and context display.
#[derive(Debug)]
pub struct ParserError<'a> {
    pub offset: usize,
    pub remaining: &'a str,
    pub inner: ContextError,
    _original: &'a str,
}

impl<'a> ParserError<'a> {
    pub fn from_parse_error(err: winnow::error::ParseError<&'a str, ContextError>, original: &'a str) -> Self {
        let range = err.char_span();
        let offset = range.end;
        let remaining = &original[offset..];
        let inner = err.inner().clone();
        ParserError { offset, remaining, inner, _original: original }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();
        output.push_str("error: parsing failed\n");
        let line = self._original[..self.offset].matches('\n').count() + 1;
        let col_offset = self._original[..self.offset]
            .rfind('\n').map(|i| self.offset - i - 1).unwrap_or(self.offset);
        let col = col_offset + 1;
        output.push_str(&format!("  --> line {}:{}\n", line, col));

        // Find the source line containing the error offset
        let context_start = if self.offset >= 200 { self.offset - 200 } else { 0 };
        let ctx_end = (self.offset + 100).min(self._original.len());
        let ctx = &self._original[context_start..ctx_end];
        let ctx_line_start = context_start
            + ctx[..].rfind('\n').map(|i| i + 1).unwrap_or(context_start);
        let ctx_line_end = ctx[ctx_line_start - context_start..].find('\n')
            .map(|i| ctx_line_start + i)
            .unwrap_or(self._original.len().min(ctx_end + 100));
        let context_line = &self._original[ctx_line_start..ctx_line_end.min(self._original.len())];
        let arrow_offset = self.offset - ctx_line_start;
        let line_num_str = line.to_string();
        output.push_str(&format!(
            "    |\n{:>4} | {}\n",
            line_num_str,
            context_line
        ));
        output.push_str(&format!("    | {}\n", " ".repeat(arrow_offset) + "^"));

        // Winnow context chain: show labels and expected tokens from innermost to outermost
        let mut labels: Vec<String> = Vec::new();
        let mut expected: Vec<String> = Vec::new();
        for ctx_item in self.inner.context() {
            match ctx_item {
                StrContext::Label(l) => labels.push(l.to_string()),
                StrContext::Expected(StrContextValue::StringLiteral(s)) => {
                    expected.push(format!("{:?}", s));
                }
                StrContext::Expected(StrContextValue::CharLiteral(c)) => {
                    expected.push(format!("{:?}", c));
                }
                StrContext::Expected(e) => expected.push(e.to_string()),
                _ => {}
            }
        }
        if !expected.is_empty() {
            output.push_str(&format!("  expected: {}\n", expected.join(", ")));
        }
        if !labels.is_empty() {
            output.push_str(&format!("  while parsing: {}\n", labels.join(" > ")));
        }
        if let Some(cause) = self.inner.cause() {
            output.push_str(&format!("  caused by: {}\n", cause));
        }
        output
    }
}

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
// Token types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Package, Class, Record, Type, Function, Connector, Uniontype,
    Encapsulated, Partial, Final, Extends, End, Annotation, Import,
    Public, Protected, Pure, Impure, External,
    Equation, Algorithm,
    Model, Operator, Parallel, Kernel, Expandable, Optimization,
    Within, Der, Code, Equality, Initial,
    Else, If, Then, For, While, Try, Elseif, ElseWhen, Return,
    Break, Continue, Match, Matchcontinue, Case,
    Each, Replaceable, Declareunit, Constraint, Assert,
    Enumeration, Subtypeof, Pder, Overload,
    Flow, Stream,
    And, Or, Not, In, When, BoolTrue, BoolFalse,
    Local, As, Guard, Threaded,
    Ident(&'a str),
    StringLit(&'a str),
    IntLit(&'a str),
    RealLit(&'a str),
    Equal, Assign, EqEq,
    Less, Leq, Greater, Geq, NotEq,
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Dot, DotDot, Colon, Semi, Comma,
    Star, Plus, Minus, Slash, Power, Pipe,
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
            Token::Code => "$Code",
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
            Token::Then => "then",
            Token::And => "and",
            Token::Or => "or",
            Token::Not => "not",
            Token::In => "in",
            Token::When => "when",
            Token::BoolTrue => "true",
            Token::BoolFalse => "false",
            Token::Local => "local",
            Token::As => "as",
            Token::Guard => "guard",
            Token::Threaded => "threaded",
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
// Intermediate types for parser rules (converted to Absyn at boundaries)
// ---------------------------------------------------------------------------

/// Items that appear in element_list / composition before conversion to Absyn::ClassPart.
#[derive(Debug, Clone)]
pub enum ClassBodyItem {
    /// A public or protected section header with its content
    Section { section: SectionKind, items: Rc<List<ClassBodyItem>> },
    /// An element (component, import, extends, nested class)
    Element(Absyn::Element),
    /// An annotation
    Annotation(Absyn::Annotation),
    /// An equation section with raw items
    Equations(List<EquationItem>),
    /// An initial equation section
    InitialEquations(List<EquationItem>),
    /// An algorithm section
    Algorithms(List<AlgorithmItem>),
    /// An initial algorithm section
    InitialAlgorithms(List<AlgorithmItem>),
    /// An equation constraint section
    Constraints, // TODO
    /// An external declaration
    External {
        funcName: Option<String>,
        annotation_opt: Option<Absyn::Annotation>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionKind {
    Public,
    Protected,
}

/// Intermediate result of class_specifier rule.
#[derive(Debug, Clone)]
pub enum ClassSpecifier {
    Normal {
        name: Ident,
        body: Rc<ClassDef>,
    },
    Extends {
        name: Ident,
        body: Rc<ClassDef>,
    },
}

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

/// An extends clause from element_list context.
#[derive(Debug, Clone)]
struct ExtendsClause {
    path: Path,
    modification: Option<List<Rc<ElementArg>>>,
    annotation_opt: Option<Annotation>,
}

/// Intermediate result of component_clause rule.
#[derive(Debug, Clone)]
struct ComponentClause {
    typePrefix: ElementAttributes,
    typeSpec: TypeSpec,
    components: List<Rc<ComponentItem>>,
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

/// Convert ClassBodyItem list to Absyn::ClassPart list.
fn body_items_to_classparts(items: List<ClassBodyItem>) -> List<ClassPart> {
    match items {
        List::Nil() => List::Nil(),
        List::Cons { head, tail } => {
            let converted = match head {
                ClassBodyItem::Section { section, items } => {
                    let content = body_items_to_element_items((*items).clone());
                    match section {
                        SectionKind::Public => ClassPart::PUBLIC { contents: content },
                        SectionKind::Protected => ClassPart::PROTECTED { contents: content },
                    }
                }
                ClassBodyItem::Element(elem) => {
                    // Wrap element as an ElementItem, then as a ClassPart
                    // We need to store it somehow. Use a local representation.
                    // For now, store in the composition's parts directly via PARTS.classParts
                    // Since Absyn doesn't have a direct ClassPart for elements,
                    // we embed them in the PARTS node.
                    // Workaround: use PUBLIC section with single element
                    let ei = ElementItem::ELEMENTITEM { element: elem };
                    ClassPart::PUBLIC { contents: cons(ei, List::Nil()) }
                }
                ClassBodyItem::Annotation(ann) => {
                    // Store annotation as EXTERNAL with annotation_ field
                    // TODO: annotations should go to ClassDef::PARTS.ann
                    ClassPart::EXTERNAL {
                        externalDecl: ExternalDecl::EXTERNALDECL {
                            funcName: None,
                            lang: None,
                            output_: None,
                            args: List::Nil(),
                            annotation_: Some(ann),
                        },
                        annotation_: None,
                    }
                }
                ClassBodyItem::Equations(items) => ClassPart::EQUATIONS { contents: items },
                ClassBodyItem::InitialEquations(items) => ClassPart::INITIALEQUATIONS { contents: items },
                ClassBodyItem::Algorithms(items) => ClassPart::ALGORITHMS { contents: items },
                ClassBodyItem::InitialAlgorithms(items) => ClassPart::INITIALALGORITHMS { contents: items },
                ClassBodyItem::Constraints => {
                    // TODO
                    ClassPart::CONSTRAINTS { contents: List::Nil() }
                }
                ClassBodyItem::External { funcName, annotation_opt } => ClassPart::EXTERNAL {
                    externalDecl: ExternalDecl::EXTERNALDECL {
                        funcName,
                        lang: None,
                        output_: None,
                        args: List::Nil(),
                        annotation_: annotation_opt,
                    },
                    annotation_: None,
                },
            };
            let rest = body_items_to_classparts((*tail).clone());
            cons(converted, rest)
        }
    }
}

/// Convert ClassBodyItem items for use inside PUBLIC/PROTECTED sections.
fn body_items_to_element_items(items: List<ClassBodyItem>) -> List<ElementItem> {
    match items {
        List::Nil() => List::Nil(),
        List::Cons { head, tail } => {
            let converted = match head {
                ClassBodyItem::Element(elem) => ElementItem::ELEMENTITEM { element: elem },
                ClassBodyItem::Annotation(ann) => ElementItem::LEXER_COMMENT {
                    comment: format!("{:?}", ann),
                },
                ClassBodyItem::External { funcName, .. } => ElementItem::LEXER_COMMENT {
                    comment: format!("external {:?}", funcName),
                },
                _ => ElementItem::LEXER_COMMENT {
                    comment: "unclassified body item".to_string(),
                },
            };
            let rest = body_items_to_element_items((*tail).clone());
            cons(converted, rest)
        }
    }
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
        if input.is_empty() { break; }
        let before = *input;
        if input.starts_with("//") {
            take_while(0.., |c: char| c != '\n' && c != '\r').parse_next(input)?;
            let _: ModalResult<char> = ascii::newline.parse_next(input);
        } else if input.starts_with("/*") {
            take_until(0.., "*/").parse_next(input)?;
            "*/".parse_next(input)?;
        }
        if *input == before { break; }
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
    match word {
        "algorithm" => Token::Algorithm,
        "annotation" => Token::Annotation,
        "assert" => Token::Assert,
        "break" => Token::Break,
        "case" => Token::Case,
        "class" => Token::Class,
        "$code" => Token::Code,
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
        "then" => Token::Then,
        "threaded" => Token::Threaded,
        "true" => Token::BoolTrue,
        "try" => Token::Try,
        "and" => Token::And,
        "or" => Token::Or,
        "not" => Token::Not,
        "in" => Token::In,
        "when" => Token::When,
        "false" => Token::BoolFalse,
        "local" => Token::Local,
        "as" => Token::As,
        "guard" => Token::Guard,
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

fn name_path<'a>(input: &mut &'a str) -> ModalResult<Path> {
    skip_trivia(input)?;
    let fq = opt(".").parse_next(input)?.is_some();
    let res = name_path2.parse_next(input)?;
    if fq {
        Ok(Path::FULLYQUALIFIED { path: Rc::new(res) })
    } else {
        Ok(res)
    }
}

fn name_path2<'a>(input: &mut &'a str) -> ModalResult<Path> {
    let mut parts = Vec::new();
    let mut last_id = ident(input)?;
    loop {
        skip_trivia(input)?;
        if opt(".").parse_next(input)?.is_none() { break; }
        parts.push(last_id);
        last_id = ident(input)?;
    }
    let mut res = Path::IDENT { name: last_id };
    for id in parts.iter().rev() {
        res = Path::QUALIFIED { name: id.to_string(), path: Rc::new(res) };
    }
    Ok(res)
}

fn class_name<'a>(input: &mut &'a str) -> ModalResult<String> {
    skip_trivia(input)?;
    let word: &str = take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
    Ok(word.to_string())
}

// ---------------------------------------------------------------------------
// Parser rules — mirror the grammar structure, return Absyn AST
// ---------------------------------------------------------------------------

/// stored_definition: BOM? (within_clause SEMICOLON)? class_definition_list EOF
pub fn stored_definition<'a>(input: &mut &'a str) -> ModalResult<Program> {
    opt("\u{feff}").parse_next(input)?;

    skip_trivia(input)?;
    let within_ = if opt("within").parse_next(input)?.is_some() {
        let path = opt(name_path).parse_next(input)?;
        cut_err(";")
            .context(StrContext::Label("';' after within clause"))
            .parse_next(input)?;
        match path {
            Some(path) => Within::WITHIN { path },
            None => Within::TOP {},
        }
    } else {
        Within::TOP {}
    };

    let classes = class_definition_list(input)?;

    skip_trivia(input)?;
    if !input.is_empty() {
        eprintln!("stored_definition: remaining input: {:?}", &input[..input.len().min(200)]);
        return Err(ErrMode::Backtrack(ContextError::default()));
    }

    Ok(Program::PROGRAM { classes, within_ })
}

/// class_definition_list: (FINAL? class_definition SEMICOLON)*
fn class_definition_list<'a>(input: &mut &'a str) -> ModalResult<List<Class>> {
    let mut defs: List<Class> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() { break; }
        let _final = opt("final").parse_next(input)?.is_some();
        if let Some(def) = opt(class_definition).parse_next(input)? {
            defs = cons(def, defs);
            skip_trivia(input)?;
            ";".parse_next(input)?;
        } else {
            break;
        }
    }
    Ok(defs.reverse())
}

/// class_definition: ENCAPSULATED? PARTIAL? class_type class_specifier
fn class_definition<'a>(input: &mut &'a str) -> ModalResult<Class> {
    let encapsulatedPrefix = opt("encapsulated").parse_next(input)?.is_some();
    let partialPrefix = opt("partial").parse_next(input)?.is_some();
    let finalPrefix = opt("final").parse_next(input)?.is_some();

    let restriction = class_type(input)?;
    // Once we have a restriction keyword, commit — errors below report at the actual failure point
    let specifier = cut_err(class_specifier)
        .context(StrContext::Label("class specifier"))
        .parse_next(input)?;

    Ok(Class::CLASS {
        name: specifier.name(),
        partialPrefix: partialPrefix,
        finalPrefix: finalPrefix,
        encapsulatedPrefix: encapsulatedPrefix,
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
    alt((class_type2,class_type_function)).parse_next(input)
}

fn class_type2<'a>(input: &mut &'a str) -> ModalResult<Restriction> {
    let res = match keyword_or_ident(input)? {
        Token::Class => Restriction::R_CLASS{},
        Token::Ident("optimization") => Restriction::R_OPTIMIZATION{},
        Token::Model => Restriction::R_MODEL{},
        Token::Record => Restriction::R_RECORD{},
        Token::Ident("block") => Restriction::R_BLOCK{},
        Token::Expandable => {
            // must be followed by "connector"
            match keyword_or_ident(input)? {
                Token::Connector => Restriction::R_EXP_CONNECTOR{},
                _ => return Err(ErrMode::Backtrack(ContextError::default())),
            }
        },
        Token::Connector => Restriction::R_CONNECTOR{},
        Token::Type => Restriction::R_TYPE{},
        Token::Package => Restriction::R_PACKAGE{},
        Token::Uniontype => Restriction::R_UNIONTYPE{},
        Token::Operator => Restriction::R_OPERATOR_RECORD{},
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    };
    Ok(res)
}

fn class_type_function<'a>(input: &mut &'a str) -> ModalResult<Restriction> {
    let purity = try_tok(input, |t| match t {
        Token::Pure => Some(Absyn::FunctionPurity::PURE{}),
        Token::Impure => Some(Absyn::FunctionPurity::IMPURE{}),
        _ => None,
    }).unwrap_or(Absyn::FunctionPurity::NO_PURITY{});

    let functionRestriction = try_tok(input, |t| match t {
        Token::Operator => Some(Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION{}),
        Token::Parallel => Some(Absyn::FunctionRestriction::FR_PARALLEL_FUNCTION{}),
        Token::Ident("parkernel") => Some(Absyn::FunctionRestriction::FR_KERNEL_FUNCTION{}),
        _ => None,
    }).unwrap_or(Absyn::FunctionRestriction::FR_NORMAL_FUNCTION{purity});

    match keyword_or_ident(input)? {
        Token::Function => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    Ok(Absyn::Restriction::R_FUNCTION{functionRestriction})
}

/// class_specifier: identifier class_specifier2
///                 | EXTENDS identifier class_modification? composition END IDENT
fn class_specifier<'a>(input: &mut &'a str) -> ModalResult<ClassSpecifier> {
    if opt("extends").parse_next(input)?.is_some() {
        let name = cut_err(ident)
            .context(StrContext::Label("class name after 'extends'"))
            .parse_next(input)?;
        let modifications = opt(class_modification).parse_next(input)?
            .unwrap_or(List::Nil());
        let comment = string_comment(input)?;
        let parts = cut_err(composition)
            .context(StrContext::Label("class-extends body"))
            .parse_next(input)?;
        let classParts = body_items_to_classparts(parts);
        skip_trivia(input)?;
        cut_err("end")
            .context(StrContext::Label("'end' closing class-extends"))
            .parse_next(input)?;
        if ident(input)? != name {
            return Err(ErrMode::Backtrack(ContextError::default()));
        }
        let ann: List<Annotation> = List::Nil();
        Ok(ClassSpecifier::Extends {
            name: name.clone(),
            body: Rc::new(ClassDef::CLASS_EXTENDS {
                baseClassName: name,
                modifications,
                comment,
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

/// class_specifier2
fn class_specifier2<'a>(input: &mut &'a str) -> ModalResult<Rc<ClassDef>> {
    skip_trivia(input)?;
    if opt("subtypeof").parse_next(input)?.is_some() {
        let typeSpec = type_specifier(input)?;
        return Ok(Rc::new(ClassDef::DERIVED {
            typeSpec,
            attributes: default_element_attrs(),
            arguments: List::Nil(),
            comment: None,
        }));
    }

    if opt("=").parse_next(input)?.is_some() {
        if opt("enumeration").parse_next(input)?.is_some() {
            let literals = cut_err(enum_list)
                .context(StrContext::Label("enumeration literal list"))
                .parse_next(input)?;
            return Ok(Rc::new(ClassDef::ENUMERATION {
                enumLiterals: EnumDef::ENUMLITERALS { enumLiterals: literals },
                comment: None,
            }));
        }

        let typeSpec = cut_err(type_specifier)
            .context(StrContext::Label("type specifier after '='"))
            .parse_next(input)?;
        let arguments: List<Rc<ElementArg>> = opt(class_modification).parse_next(input)?
            .unwrap_or_default();
        let comment = opt(string_comment).parse_next(input)?.map(|c| Comment::COMMENT {
            annotation_: None,
            comment: c,
        });

        return Ok(Rc::new(ClassDef::DERIVED {
            typeSpec,
            attributes: default_element_attrs(),
            arguments,
            comment,
        }));
    }

    let mut typeVars: List<String> = List::Nil();
    if opt("<").parse_next(input)?.is_some() {
        loop {
            let id = ident(input)?;
            typeVars = cons(id, typeVars);
            if opt(">").parse_next(input)?.is_some() { break; }
            ",".parse_next(input)?;
        }
        typeVars = typeVars.reverse()
    } else if opt("(").parse_next(input)?.is_some() {
        // Only for Optimica
        return Err(ErrMode::Backtrack(ContextError::default()));
    };

    let comment = string_comment.parse_next(input)?;
    let parts = cut_err(composition)
        .context(StrContext::Label("class body"))
        .parse_next(input)?;
    let classParts = body_items_to_classparts(parts);

    skip_trivia(input)?;
    cut_err("end")
        .context(StrContext::Label("'end' closing class body"))
        .parse_next(input)?;
    let _end_name = cut_err(ident)
        .context(StrContext::Label("class name after 'end'"))
        .parse_next(input)?;
    // TODO: Check that the names match up...

    Ok(Rc::new(ClassDef::PARTS {
        typeVars,
        classAttrs: List::Nil(),
        classParts,
        ann: List::Nil(),
        comment,
    }))
}

fn default_element_attrs() -> ElementAttributes {
    ElementAttributes::ATTR {
        flowPrefix: false,
        streamPrefix: false,
        parallelism: Parallelism::NON_PARALLEL {},
        variability: Variability::VAR {},
        direction: Direction::INPUT {},
        isField: IsField::NONFIELD {},
        arrayDim: ArrayDim::Nil(),
    }
}

/// composition: element_list composition2 (annotation SEMICOLON)?
fn composition<'a>(input: &mut &'a str) -> ModalResult<List<ClassBodyItem>> {
    let el_items = element_list(input)?;
    let c2_items = composition2(input)?;

    // Concatenate el_items + c2_items
    let combined = el_items.append(&c2_items);

    // (annotation SEMICOLON)?
    skip_trivia(input)?;
    if let Some(ann) = opt(annotation).parse_next(input)? {
        ";".parse_next(input)?;
        // TODO: class-level annotations should go to ClassDef::PARTS.ann
        let mut result = combined;
        result = cons(ClassBodyItem::Annotation(ann), result);
        Ok(result)
    } else {
        Ok(combined)
    }
}

/// composition2
fn composition2<'a>(input: &mut &'a str) -> ModalResult<List<ClassBodyItem>> {
    skip_trivia(input)?;
    if input.is_empty() { return Ok(List::Nil()); }
    let mut parts: List<ClassBodyItem> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() { break; }

        if let Some(ext) = opt(external_part).parse_next(input)? {
            parts = cons(ext, parts);
            continue;
        }
        if opt("public").parse_next(input)?.is_some() {
            let items = element_list(input)?;
            let tail = composition2(input)?;
            parts = cons(ClassBodyItem::Section { section: SectionKind::Public, items: Rc::new(items) }, parts);
            parts = parts.append(&tail);
            continue;
        }
        if opt("protected").parse_next(input)?.is_some() {
            let items = element_list(input)?;
            let tail = composition2(input)?;
            parts = cons(ClassBodyItem::Section { section: SectionKind::Protected, items: Rc::new(items) }, parts);
            parts = parts.append(&tail);
            continue;
        }
        if opt("initial").parse_next(input)?.is_some() {
            skip_trivia(input)?;
            if opt("equation").parse_next(input)?.is_some() {
                let items = equation_section_items(input)?;
                let tail = composition2(input)?;
                parts = cons(ClassBodyItem::InitialEquations(items), parts);
                parts = parts.append(&tail);
            } else if opt("algorithm").parse_next(input)?.is_some() {
                let items = algorithm_section_items(input)?;
                let tail = composition2(input)?;
                parts = cons(ClassBodyItem::InitialAlgorithms(items), parts);
                parts = parts.append(&tail);
            } else {
                return Err(ErrMode::Backtrack(ContextError::default()));
            }
            continue;
        }
        if opt("equation").parse_next(input)?.is_some() {
            let items = equation_section_items(input)?;
            let tail = composition2(input)?;
            parts = cons(ClassBodyItem::Equations(items), parts);
            parts = parts.append(&tail);
            continue;
        }
        if opt("algorithm").parse_next(input)?.is_some() {
            let items = algorithm_section_items(input)?;
            let tail = composition2(input)?;
            parts = cons(ClassBodyItem::Algorithms(items), parts);
            parts = parts.append(&tail);
            continue;
        }
        break;
    }
    Ok(parts.reverse())
}

/// element_list: ((element | annotation) SEMICOLON)*
fn element_list<'a>(input: &mut &'a str) -> ModalResult<List<ClassBodyItem>> {
    let mut items: List<ClassBodyItem> = List::Nil();
    loop {
        // Stop at section keywords or end of input
        match peek(keyword_or_ident).parse_next(input) {
            Ok(Token::Public) | Ok(Token::Protected) | Ok(Token::Equation) | Ok(Token::Algorithm)
            | Ok(Token::External) | Ok(Token::End) | Ok(Token::Initial) | Ok(Token::Case)
            | Ok(Token::Else) | Ok(Token::Then) => break,
            Err(_) => break,
            _ => (),
        };

        skip_trivia(input)?;

        // annotation SEMICOLON
        if let Some(ann) = opt(annotation).parse_next(input)? {
            skip_trivia(input)?;
            ";".parse_next(input)?;
            items = cons(ClassBodyItem::Annotation(ann), items);
            continue;
        }

        // import_clause SEMICOLON
        if let Some(imp) = opt(import_clause).parse_next(input)? {
            skip_trivia(input)?;
            ";".parse_next(input)?;
            let elem = mk_element(
                ElementSpec::IMPORT { import_: imp, comment: None, info: dummy_info() },
            );
            items = cons(ClassBodyItem::Element(elem), items);
            continue;
        }

        // extends_clause SEMICOLON
        if let Some(ext) = opt(extends_clause).parse_next(input)? {
            skip_trivia(input)?;
            ";".parse_next(input)?;
            let elem = mk_element(
                ElementSpec::EXTENDS {
                    path: ext.path,
                    elementArg: ext.modification.unwrap_or_else(|| List::Nil()),
                    annotationOpt: ext.annotation_opt,
                },
            );
            items = cons(ClassBodyItem::Element(elem), items);
            continue;
        }

        // Nested class_definition SEMICOLON
        if let Some(cls) = opt(class_definition).parse_next(input)? {
            skip_trivia(input)?;
            ";".parse_next(input)?;
            let elem = mk_element(
                ElementSpec::CLASSDEF { replaceable_: false, class_: Rc::new(cls) },
            );
            items = cons(ClassBodyItem::Element(elem), items);
            continue;
        }

        // component_clause
        if let Some(cc) = opt(component_clause).parse_next(input)? {
            let elem = mk_element(ElementSpec::COMPONENTS {
                attributes: cc.typePrefix,
                typeSpec: cc.typeSpec,
                components: cc.components,
            });
            items = cons(ClassBodyItem::Element(elem), items);
            continue;
        }
        break;
    }
    Ok(items.reverse())
}

/// Build an Absyn::Element with default attributes.
fn mk_element(specification: ElementSpec) -> Absyn::Element {
    Absyn::Element::ELEMENT {
        finalPrefix: false,
        redeclareKeywords: None,
        innerOuter: InnerOuter::NOT_INNER_OUTER {},
        specification,
        info: dummy_info(),
        constrainClass: None,
    }
}

/// Consume a keyword via keyword_or_ident if it matches the predicate; otherwise restore input.
fn try_tok<'a, F, T>(input: &mut &'a str, f: F) -> Option<T>
where
    F: Fn(Token<'a>) -> Option<T>,
{
    let saved = *input;
    match keyword_or_ident(input) {
        Ok(tok) => match f(tok) {
            Some(v) => Some(v),
            None => { *input = saved; None }
        },
        Err(_) => { *input = saved; None }
    }
}

/// type_prefix: (flow|stream)? (parlocal|parglobal)? (discrete|parameter|constant)? input? output? (field|nonfield)?
fn type_prefix<'a>(input: &mut &'a str) -> ModalResult<ElementAttributes> {
    let flow = try_tok(input, |t| matches!(t, Token::Flow).then_some(())).is_some();
    let stream = !flow && try_tok(input, |t| matches!(t, Token::Stream).then_some(())).is_some();

    let parallelism = try_tok(input, |t| match t {
        Token::Ident("parlocal") => Some(Parallelism::PARLOCAL {}),
        Token::Ident("parglobal") => Some(Parallelism::PARGLOBAL {}),
        _ => None,
    }).unwrap_or(Parallelism::NON_PARALLEL {});

    let variability = try_tok(input, |t| match t {
        Token::Ident("discrete") => Some(Variability::DISCRETE {}),
        Token::Ident("parameter") => Some(Variability::PARAM {}),
        Token::Ident("constant") => Some(Variability::CONST {}),
        _ => None,
    }).unwrap_or(Variability::VAR {});

    let has_input = try_tok(input, |t| matches!(t, Token::Ident("input")).then_some(())).is_some();
    let has_output = try_tok(input, |t| matches!(t, Token::Ident("output")).then_some(())).is_some();
    let direction = match (has_input, has_output) {
        (true, true) => Direction::INPUT_OUTPUT {},
        (true, false) => Direction::INPUT {},
        (false, true) => Direction::OUTPUT {},
        (false, false) => Direction::BIDIR {},
    };

    let is_field = try_tok(input, |t| match t {
        Token::Ident("field") => Some(IsField::FIELD {}),
        Token::Ident("nonfield") => Some(IsField::NONFIELD {}),
        _ => None,
    }).unwrap_or(IsField::NONFIELD {});

    Ok(ElementAttributes::ATTR {
        flowPrefix: flow,
        streamPrefix: stream,
        parallelism,
        variability,
        direction,
        isField: is_field,
        arrayDim: ArrayDim::Nil(),
    })
}

fn component_clause<'a>(input: &mut &'a str) -> ModalResult<ComponentClause> {
    let typePrefix = type_prefix.parse_next(input)?;
    let typeSpec = type_specifier.parse_next(input)?;
    let components = cut_err(component_list)
        .context(StrContext::Label("component list"))
        .parse_next(input)?;
    skip_trivia(input)?;
    cut_err(";")
        .context(StrContext::Label("';' after component list"))
        .parse_next(input)?;
    Ok(ComponentClause { typePrefix, typeSpec, components })
}

/// component_list: component_declaration (COMMA component_declaration)*
fn component_list<'a>(input: &mut &'a str) -> ModalResult<List<Rc<ComponentItem>>> {
    let first = component_declaration.parse_next(input)?;
    let mut items = List::new(Rc::new(first));
    loop {
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() { break; }
        items = cons(Rc::new(component_declaration.parse_next(input)?), items);
    }
    Ok(items.reverse())
}

/// component_declaration: declaration (IF expression)? string_comment annotation?
/// declaration: (IDENT | OPERATOR) (array_subscripts)? (modification)?
fn component_declaration<'a>(input: &mut &'a str) -> ModalResult<ComponentItem> {
    skip_trivia(input)?;

    let name = match keyword_or_ident.parse_next(input)? {
        Token::Ident(n) => n.to_string(),
        Token::Operator => "operator".to_string(),
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    };

    let arrayDim = opt(array_subscripts).parse_next(input)?.unwrap_or_else(|| ArrayDim::Nil());

    let m = opt(modification).parse_next(input)?;

    let condition = if opt("if").parse_next(input)?.is_some() {
        Some(expression.parse_next(input)?)
    } else {
        None
    };
    let _comment = string_comment.parse_next(input)?;
    let _ann = opt(annotation).parse_next(input)?;

    Ok(ComponentItem::COMPONENTITEM {
        component: Component::COMPONENT { name, arrayDim, modification: m },
        condition,
        comment: None,
    })
}

fn modification<'a>(input: &mut &'a str) -> ModalResult<Modification> {
    let cm = opt(class_modification).parse_next(input)?.unwrap_or(List::Nil());
    skip_trivia(input)?;
    let eq = if opt(alt((":=", "="))).parse_next(input)?.is_some() {
        Absyn::EqMod::EQMOD{exp: Rc::new(cut_err(modification_expression).context(StrContext::Label("Modification with =")).parse_next(input)?), info: dummy_info()}
    } else {
        Absyn::EqMod::NOMOD{}
    };
    Ok(Modification::CLASSMOD { elementArgLst: cm, eqMod: eq })
}

fn modification_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    skip_trivia(input)?;
    if opt("break").parse_next(input)?.is_some() {
        return Ok(Absyn::Exp::BREAK{});
    };
    let res = expression.parse_next(input);
    res
}

fn class_modification<'a>(input: &mut &'a str) -> ModalResult<List<Rc<ElementArg>>> {
    "(".parse_next(input)?;
    let arguments = opt(argument_list).parse_next(input)?.unwrap_or(List::Nil());
    cut_err(")")
        .context(StrContext::Label("')' closing modification list"))
        .parse_next(input)?;
    Ok(arguments)
}

fn argument_list<'a>(input: &mut &'a str) -> ModalResult<List<Rc<ElementArg>>> {
    let mut res = List::new(Rc::new(argument.parse_next(input)?));

    loop {
        if opt(",").parse_next(input)?.is_none() { break; }
        res = cons(Rc::new(argument.parse_next(input)?), res);
    }
    Ok(res.reverse())
}

fn argument<'a>(input: &mut &'a str) -> ModalResult<ElementArg> {
    if let Some(r) = opt(element_redeclaration).parse_next(input)? {
        return Ok(r);
    }
    let eachPrefix_ = opt("each").parse_next(input)?.is_some();
    let finalPrefix_ = opt("final").parse_next(input)?.is_some();
    let mut res = alt((element_replaceable,element_modification)).parse_next(input)?;
    match res {
        ElementArg::MODIFICATION{ref mut eachPrefix, ref mut finalPrefix, ..} => {
            if eachPrefix_ {
                *eachPrefix = Each::EACH{};
            } else {
                *eachPrefix = Each::NON_EACH{};
            }
            *finalPrefix = finalPrefix_;
        },
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    };
    Ok(res)
}

fn element_redeclaration<'a>(input: &mut &'a str) -> ModalResult<ElementArg> {
    "redeclare".parse_next(input)?;
    let eachPrefix = opt("each").parse_next(input)?.is_some();
    let finalPrefix = opt("final").parse_next(input)?.is_some();
    Err(ErrMode::Backtrack(ContextError::default()))
}

fn element_modification<'a>(input: &mut &'a str) -> ModalResult<ElementArg> {
    let path: Result<Path, ErrMode<ContextError>> = name_path.parse_next(input); // TODO: Not FQ
    if opt("[").context(StrContext::Label("Subscripting modifiers is not allowed. Apply the modification on the whole identifier using an array-expression or an each-modifier.")).parse_next(input)?.is_some() {
        return Err(ErrMode::Backtrack(ContextError::default()))
    };
    let modification = opt(modification).parse_next(input)?;
    let comment = string_comment.parse_next(input)?;
    Ok(Absyn::ElementArg::MODIFICATION{eachPrefix: Each::NON_EACH{}, finalPrefix: false, modification, comment, path: path?, info: dummy_info()})
}

fn element_replaceable<'a>(input: &mut &'a str) -> ModalResult<ElementArg> {
    Err(ErrMode::Backtrack(ContextError::default()))
}


fn annotation<'a>(input: &mut &'a str) -> ModalResult<Annotation> {
    "annotation".parse_next(input)?;
    Ok(Absyn::Annotation::ANNOTATION{
        elementArgs: cut_err(class_modification)
            .context(StrContext::Label("annotation body"))
            .parse_next(input)?,
    })
}

fn import_clause<'a>(input: &mut &'a str) -> ModalResult<Import> {
    "import".parse_next(input)?;
    let path = name_path(input)?;
    match path {
        Path::IDENT { name } => {
            if opt("=").parse_next(input)?.is_some() {
                let path = name_path(input)?;
                Ok(Import::NAMED_IMPORT { name, path })
            } else {
                Ok(Import::QUAL_IMPORT { path: Path::IDENT { name } })
            }
        }
        _ => Ok(Import::QUAL_IMPORT { path }),
    }
}

fn extends_clause<'a>(input: &mut &'a str) -> ModalResult<ExtendsClause> {
    "extends".parse_next(input)?;
    let path = name_path(input)?;
    // TODO: should not allow BREAK, which needs to be passed down the parser, or use 2 different rules
    let modification = opt(class_modification).parse_next(input)?;
    let annotation_opt = opt(annotation).parse_next(input)?;
    Ok(ExtendsClause { path, modification, annotation_opt })
}

fn equation_section_items<'a>(input: &mut &'a str) -> ModalResult<List<EquationItem>> {
    let mut items: List<EquationItem> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() { break; }
        match peek(keyword_or_ident).parse_next(input) {
            Ok(Token::Public) | Ok(Token::Protected) | Ok(Token::Equation) | Ok(Token::Algorithm)
            | Ok(Token::External) | Ok(Token::End) | Ok(Token::Initial) => break,
            _ => ()
        };

        // TODO: Handle equation properly

        let item_text: &str = take_while(0.., |c: char| c != ';').parse_next(input)?;
        let trimmed = item_text.trim().to_string();
        if !trimmed.is_empty() {
            items = cons(EquationItem::EQUATIONITEMCOMMENT { comment: trimmed }, items);
        }
        skip_trivia(input)?;
        if input.starts_with(';') {
            ";".parse_next(input)?;
        } else { break; }
    }
    Ok(items.reverse())
}

fn algorithm_section_items<'a>(input: &mut &'a str) -> ModalResult<List<AlgorithmItem>> {
    let mut items: List<AlgorithmItem> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty() { break; }
        match peek(keyword_or_ident).parse_next(input) {
            Ok(Token::Public) | Ok(Token::Protected) | Ok(Token::Equation) | Ok(Token::Algorithm)
            | Ok(Token::Initial) | Ok(Token::End) | Ok(Token::External) => break,
            Err(_) => break,
            _ => (),
        };

        // Control flow keywords → TODO placeholder
        match peek(keyword_or_ident).parse_next(input) {
            Ok(Token::If) | Ok(Token::For) | Ok(Token::While) | Ok(Token::When)
            | Ok(Token::Try) => {
                // TODO: parse if/for/while/when/try/parfor algorithm statements
                let item_text: &str = take_while(0.., |c: char| c == ';' || c == '\n').parse_next(input)?;
                let trimmed = item_text.trim().to_string();
                if !trimmed.is_empty() {
                    items = cons(AlgorithmItem::ALGORITHMITEMCOMMENT { comment: trimmed }, items);
                }
                skip_trivia(input)?;
                if opt(";").parse_next(input)?.is_none() { break; }
                continue;
            }
            Ok(Token::Return) => {
                keyword_or_ident.parse_next(input)?;
                items = cons(AlgorithmItem::ALGORITHMITEM {
                    algorithm_: Rc::new(Algorithm::ALG_RETURN {}),
                    comment: None,
                    info: dummy_info(),
                }, items);
                continue;
            }
            Ok(Token::Break) => {
                keyword_or_ident.parse_next(input)?;
                items = cons(AlgorithmItem::ALGORITHMITEM {
                    algorithm_: Rc::new(Algorithm::ALG_BREAK {}),
                    comment: None,
                    info: dummy_info(),
                }, items);
                continue;
            }
            Ok(Token::Continue) => {
                keyword_or_ident.parse_next(input)?;
                items = cons(AlgorithmItem::ALGORITHMITEM {
                    algorithm_: Rc::new(Algorithm::ALG_CONTINUE {}),
                    comment: None,
                    info: dummy_info(),
                }, items);
                continue;
            }
            _ => (),
        };

        // Assignment clause: simple_expression (ASSIGN expression)?
        let lhs = simple_expression(input)?;
        skip_trivia(input)?;

        if opt(":=").parse_next(input)?.is_some() {
            let value = expression(input)?;
            let alg = Algorithm::ALG_ASSIGN {
                assignComponent: lhs,
                value,
            };
            items = cons(AlgorithmItem::ALGORITHMITEM {
                algorithm_: Rc::new(alg),
                comment: None,
                info: dummy_info(),
            }, items);
        } else {
            // No-return-value call
            // TODO: parse as ALG_NORETCALL properly
            items = cons(AlgorithmItem::ALGORITHMITEM {
                algorithm_: Rc::new(Algorithm::ALG_NORETCALL {
                    functionCall: Absyn::ComponentRef::CREF_IDENT {
                        name: "call".to_string(),
                        subscripts: List::Nil(),
                    },
                    functionArgs: FunctionArgs::FUNCTIONARGS {
                        args: List::Nil(),
                        argNames: List::Nil(),
                    },
                }),
                comment: None,
                info: dummy_info(),
            }, items);
        }

        skip_trivia(input)?;
        if opt(";").parse_next(input)?.is_none() { break; }
    }
    Ok(items.reverse())
}

fn component_reference<'a>(input: &mut &'a str) -> ModalResult<Absyn::ComponentRef> {
    let fq = opt(".").parse_next(input)?.is_some();
    let cr = component_reference2(input)?;
    if fq {
        Ok(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: Rc::new(cr) })
    } else {
        Ok(cr)
    }
}

fn component_reference2<'a>(input: &mut &'a str) -> ModalResult<Absyn::ComponentRef> {
    let name = ident(input)?;
    let raw_subs = opt(array_subscripts).parse_next(input)?.unwrap_or(List::Nil());
    let mut subscripts: List<Rc<Absyn::Subscript>> = List::Nil();
    for s in &raw_subs.reverse() { subscripts = cons(Rc::new(s), subscripts); }
    // Only consume '.' if followed by an identifier char (not element-wise ops or '..')
    let is_qual_dot = input.starts_with('.') && input.chars().nth(1)
        .map_or(false, |c| c.is_alphanumeric() || c == '_');
    if is_qual_dot {
        ".".parse_next(input)?;
        let rest = component_reference2(input)?;
        Ok(Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Rc::new(rest) })
    } else {
        Ok(Absyn::ComponentRef::CREF_IDENT { name, subscripts })
    }
}

fn named_argument<'a>(input: &mut &'a str) -> ModalResult<Absyn::NamedArg> {
    let argName = ident(input)?;
    "=".parse_next(input)?;
    let argValue = Rc::new(expression(input)?);
    Ok(Absyn::NamedArg::NAMEDARG { argName, argValue })
}

fn named_arguments<'a>(input: &mut &'a str) -> ModalResult<List<Rc<Absyn::NamedArg>>> {
    let first = named_argument(input)?;
    let mut args: List<Rc<Absyn::NamedArg>> = cons(Rc::new(first), List::Nil());
    loop {
        if opt(",").parse_next(input)?.is_none() { break; }
        match named_argument(input) {
            Ok(arg) => args = cons(Rc::new(arg), args),
            Err(_) => break,
        }
    }
    Ok(args.reverse())
}

/// part_eval_function_expression: FUNCTION component_reference LPAR named_arguments? RPAR
fn part_eval_function_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    "function".parse_next(input)?;
    let cr = component_reference(input)?;
    "(".parse_next(input)?;
    let argNames = opt(named_arguments).parse_next(input)?.unwrap_or(List::Nil());
    ")".parse_next(input)?;
    Ok(Absyn::Exp::PARTEVALFUNCTION {
        function_: Rc::new(cr),
        functionArgs: Absyn::FunctionArgs::FUNCTIONARGS {
            args: List::Nil(),
            argNames,
        },
    })
}

// ============================================================================
// Expression parser — follows the ANTLR grammar structure
// ============================================================================

/// primary: literal | cref/call | parenthesised | array | matrix | end
fn primary<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    skip_trivia(input)?;
    // end
    let tok = peek(opt(keyword_or_ident)).parse_next(input)?;
    if let Some(Token::End) = tok {
        keyword_or_ident.parse_next(input)?;
        return Ok(Absyn::Exp::END {});
    }

    // true / false
    if let Some(Token::BoolTrue) = tok {
        keyword_or_ident.parse_next(input)?;
        return Ok(Absyn::Exp::BOOL{value: true});
    }
    if let Some(Token::BoolFalse) = tok {
        keyword_or_ident.parse_next(input)?;
        return Ok(Absyn::Exp::BOOL{value: false});
    }

    // String literal
    if input.starts_with('"') {
        let s = string_token(input)?;
        return Ok(Absyn::Exp::STRING { value: s });
    }

    // Numeric literal (real before integer by trying decimal/exponent)
    if input.starts_with(|c: char| c.is_ascii_digit()) || input.starts_with('.') {
        if let Ok(e) = number_literal(input) {
            return Ok(e);
        }
    }

    // ( output_expression_list array_subscripts? )
    if input.starts_with('(') {
        "(".parse_next(input)?;
        let (exprs, is_tuple) = output_expression_list(input)?;
        // output_expression_list already consumed ')'
        let raw_subs = opt(array_subscripts).parse_next(input)?;
        if let Some(subs) = raw_subs {
            // Wrap subscripts in Rc
            let mut rc_subs: List<Rc<Subscript>> = List::Nil();
            for s in &subs.reverse() { rc_subs = cons(Rc::new(s), rc_subs); }
            return Ok(Absyn::Exp::SUBSCRIPTED_EXP { exp: Rc::new(to_tuple_or_exp(exprs, is_tuple)), subscripts: rc_subs });
        }
        return Ok(to_tuple_or_exp(exprs, is_tuple));
    }

    // [ matrix_expression_list ]
    if input.starts_with('[') {
        "[".parse_next(input)?;
        let rows = matrix_expression_list(input)?;
        "]".parse_next(input)?;
        return Ok(Absyn::Exp::MATRIX { matrix: rows });
    }

println!("array? {}", &input[0..input.len().min(20)]);
    // { for_or_expression_list }
    if input.starts_with('{') {
        "{".parse_next(input)?;
println!("array? {}", &input[0..input.len().min(20)]);
        let fa = for_or_expression_list(input)?;
println!("array for {:?} {}", fa, &input[0..input.len().min(20)]);
        "}".parse_next(input)?;
        return match fa {
            Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators } => {
                let cr = Absyn::ComponentRef::CREF_IDENT {
                    name: "$array".to_string(), subscripts: List::Nil() };
                Ok(Absyn::Exp::CALL {
                    function_: Rc::new(cr),
                    functionArgs: Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators },
                    typeVars: List::Nil(),
                })
            }
            Absyn::FunctionArgs::FUNCTIONARGS { args, argNames: List::Nil() } =>
                Ok(Absyn::Exp::ARRAY{ arrayExp: args }),
            _ => Err(ErrMode::Backtrack(ContextError::default())), // TODO: Custom error-message here?
        };
    }

    // der function_call
    if let Some(Token::Der) = tok {
        keyword_or_ident.parse_next(input)?;
        let fa = function_call(input)?;
        let cr = Absyn::ComponentRef::CREF_IDENT { name: "der".to_string(), subscripts: List::Nil() };
        return Ok(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: List::Nil() });
    }

    // pure function_call
    if let Some(Token::Pure) = tok {
        keyword_or_ident.parse_next(input)?;
        let fa = function_call(input)?;
        let cr = Absyn::ComponentRef::CREF_IDENT { name: "pure".to_string(), subscripts: List::Nil() };
        return Ok(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: List::Nil() });
    }

    // component_reference__function_call
    component_reference__function_call(input)
}

fn to_tuple_or_exp(exprs: List<Rc<Absyn::Exp>>, is_tuple: bool) -> Absyn::Exp {
    if is_tuple {
        Absyn::Exp::TUPLE { expressions: exprs }
    } else {
        match exprs {
            List::Cons { ref head, .. } => (**head).clone(),
            List::Nil() => Absyn::Exp::TUPLE { expressions: List::Nil() },
        }
    }
}

/// number_literal: UNSIGNED_REAL | UNSIGNED_INTEGER
fn number_literal<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    skip_trivia(input)?;
    let start = *input;
    let has_int = take_while::<_, _, ContextError>(1.., |c: char| c.is_ascii_digit()).parse_next(input).is_ok();
    let mut is_real = false;

    if has_int {
        // decimal point (but not '..' or element-wise operators)
        if input.starts_with('.') && !input.starts_with("..") &&
           !matches!(input.chars().nth(1), Some('+'|'-'|'*'|'/'|'^')) {
            ".".parse_next(input)?;
            take_while(0.., |c: char| c.is_ascii_digit()).parse_next(input)?;
            is_real = true;
        }
    } else if input.starts_with('.') && !input.starts_with("..") {
        ".".parse_next(input)?;
        take_while(1.., |c: char| c.is_ascii_digit()).parse_next(input)?;
        is_real = true;
    } else {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }

    // optional exponent (only if followed by sign or digit)
    if (input.starts_with('e') || input.starts_with('E')) &&
       input.chars().nth(1).map_or(false, |c| c.is_ascii_digit() || c == '+' || c == '-') {
        take_while(1.., |c: char| c == 'e' || c == 'E').parse_next(input)?;
        opt(alt(('+', '-'))).parse_next(input)?;
        take_while(1.., |c: char| c.is_ascii_digit()).parse_next(input)?;
        is_real = true;
    }

    let len = start.len() - input.len();
    let s = &start[..len];
    if is_real {
        Ok(Absyn::Exp::REAL { value: s.to_string() })
    } else {
        Ok(Absyn::Exp::INTEGER { value: s.parse().unwrap_or(i32::MAX) })
    }
}

/// component_reference__function_call: cref (<type_vars> fc)? | cref fc? | initial()
fn component_reference__function_call<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    // initial()
    if let Token::Initial = peek(keyword_or_ident).parse_next(input)? {
        keyword_or_ident.parse_next(input)?;
        skip_trivia(input)?;
        if input.starts_with('(') {
            "(".parse_next(input)?;
            ")".parse_next(input)?;
            let cr = Absyn::ComponentRef::CREF_IDENT { name: "initial".to_string(), subscripts: List::Nil() };
            return Ok(Absyn::Exp::CALL {
                function_: Rc::new(cr),
                functionArgs: Absyn::FunctionArgs::FUNCTIONARGS { args: List::Nil(), argNames: List::Nil() },
                typeVars: List::Nil(),
            });
        }
    }

    let cr = component_reference(input)?;
    skip_trivia(input)?;

    // polymorphic call: cr < type_vars > function_call  (MetaModelica)
    if input.starts_with('<') {
        let saved = *input;
        if let Ok(type_vars) = (|| -> ModalResult<List<Path>> {
            "<".parse_next(input)?;
            let mut vars: List<Path> = List::Nil();
            loop {
                skip_trivia(input)?;
                if input.starts_with('>') { break; }
                vars = cons(name_path(input)?, vars);
                skip_trivia(input)?;
                if opt(",").parse_next(input)?.is_none() { break; }
            }
            ">".parse_next(input)?;
            Ok(vars.reverse())
        })() {
            skip_trivia(input)?;
            if input.starts_with('(') {
                let fa = function_call(input)?;
                return Ok(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: type_vars });
            }
            *input = saved;
        } else {
            *input = saved;
        }
    }

    // optional function call
    if input.starts_with('(') {
        let fa = function_call(input)?;
        // optional .field access after call (MetaModelica dot operator)
        skip_trivia(input)?;
        if input.starts_with('.') && input.chars().nth(1).map_or(false, |c| c.is_alphanumeric() || c == '_') {
            ".".parse_next(input)?;
            let field = expression(input)?;
            return Ok(Absyn::Exp::DOT {
                exp: Rc::new(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: List::Nil() }),
                index: Rc::new(field),
            });
        }
        return Ok(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: List::Nil() });
    }

    Ok(Absyn::Exp::CREF { componentRef: Rc::new(cr) })
}

/// function_call: LPAR function_arguments RPAR
fn function_call<'a>(input: &mut &'a str) -> ModalResult<Absyn::FunctionArgs> {
    "(".parse_next(input)?;
    let fa = function_arguments(input)?;
    ")".parse_next(input)?;
    Ok(fa)
}

/// function_arguments: for_or_expression_list named_arguments?
fn function_arguments<'a>(input: &mut &'a str) -> ModalResult<Absyn::FunctionArgs> {
    let fa = for_or_expression_list(input)?;
    match fa {
        Absyn::FunctionArgs::FOR_ITER_FARG { .. } => Ok(fa),
        Absyn::FunctionArgs::FUNCTIONARGS { args, argNames: _ } => {
            // Try named_arguments (they follow positional args or are the only args)
            let argNames = opt(named_arguments).parse_next(input)?.unwrap_or(List::Nil());
            Ok(Absyn::FunctionArgs::FUNCTIONARGS { args, argNames })
        }
    }
}

/// for_or_expression_list: empty | for-iterator | expression list
fn for_or_expression_list<'a>(input: &mut &'a str) -> ModalResult<Absyn::FunctionArgs> {
    skip_trivia(input)?;

    // Empty: next is ) or }
    if input.starts_with(')') || input.starts_with('}') {
        return Ok(Absyn::FunctionArgs::FUNCTIONARGS { args: List::Nil(), argNames: List::Nil() });
    }

    // Parse first expression
    let mut checkpoint = input.checkpoint();
    // Handle the ambiguity by parsing as an expression - if it is a named argument, detect that later
    let mut exp = expression(input)?;
    skip_trivia(input)?;

    // for-iterator: expr [threaded] for indices
    if matches!(peek(opt(keyword_or_ident)).parse_next(input)?, Some(Token::For) | Some(Token::Threaded)) {
        let threaded = if matches!(peek(keyword_or_ident).parse_next(input)?, Token::Threaded) {
            keyword_or_ident.parse_next(input)?;
            true
        } else { false };
        match keyword_or_ident(input)? {
            Token::For => {}
            _ => return Err(ErrMode::Backtrack(ContextError::default())),
        }
        let iterators = for_indices(input)?;
        return Ok(Absyn::FunctionArgs::FOR_ITER_FARG {
            exp: Rc::new(exp),
            iterType: if threaded { Absyn::ReductionIterType::THREAD {} } else { Absyn::ReductionIterType::COMBINE {} },
            iterators,
        });
    }

    // Expression list: e1 (, e2)*, with optional named args at the end
    let mut args = List::Nil();
    let mut arg_names = List::Nil();
    loop {
        match exp {
            Exp::CREF{componentRef} if matches!(*componentRef, ComponentRef::CREF_IDENT{subscripts=List::Nil()}) => {
                input.reset(&checkpoint);
                arg_names = named_arguments.parse_next(input)?;
                break;
            }
            _ => {}
        };
        args = cons(Rc::new(exp), args);
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() { break; }
        skip_trivia(input)?;
        checkpoint = input.checkpoint();
        exp = expression.parse_next(input)?;
    };

    Ok(Absyn::FunctionArgs::FUNCTIONARGS { args: args.reverse(), argNames: arg_names.reverse() })
}

/// for_indices: for_index (, for_index)*
fn for_indices<'a>(input: &mut &'a str) -> ModalResult<Absyn::ForIterators> {
    let first = for_index(input)?;
    let mut result: List<Absyn::ForIterator> = cons(first, List::Nil());
    loop {
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() { break; }
        match for_index(input) {
            Ok(fi) => result = cons(fi, result),
            Err(_) => break,
        }
    }
    Ok(result.reverse())
}

/// for_index: IDENT ((IF|GUARD) guard_expr)? IN range_expr
fn for_index<'a>(input: &mut &'a str) -> ModalResult<Absyn::ForIterator> {
    let name = ident(input)?;
    skip_trivia(input)?;
    let guardExp = match peek(keyword_or_ident).parse_next(input)? {
        Token::If | Token::Guard => {
            keyword_or_ident.parse_next(input)?;
            Some(Rc::new(expression(input)?))
        }
        _ => None,
    };
    skip_trivia(input)?;
    let range = match peek(keyword_or_ident).parse_next(input)? {
        Token::In => {
            keyword_or_ident.parse_next(input)?;
            Some(Rc::new(expression(input)?))
        }
        _ => None,
    };
    Ok(Absyn::ForIterator::ITERATOR { name, guardExp, range })
}

/// expression_list: expr (, expr)*
fn expression_list<'a>(input: &mut &'a str) -> ModalResult<List<Rc<Absyn::Exp>>> {
    let e = expression(input)?;
    let mut result: List<Rc<Absyn::Exp>> = cons(Rc::new(e), List::Nil());
    loop {
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() { break; }
        match expression(input) {
            Ok(e) => result = cons(Rc::new(e), result),
            Err(_) => break,
        }
    }
    Ok(result.reverse())
}

/// output_expression_list: consumes up to and including ')'
/// Returns (expressions, isTuple)
fn output_expression_list<'a>(input: &mut &'a str) -> ModalResult<(List<Rc<Absyn::Exp>>, bool)> {
    skip_trivia(input)?;
    // () case
    if input.starts_with(')') {
        ")".parse_next(input)?;
        return Ok((List::Nil(), true));
    }
    // Leading comma: (,b) → WILD, b
    if input.starts_with(',') {
        ",".parse_next(input)?;
        let (rest, _) = output_expression_list(input)?;
        let wild = Absyn::ComponentRef::WILD {};
        let wild_exp = Rc::new(Absyn::Exp::CREF { componentRef: Rc::new(wild) });
        return Ok((cons(wild_exp, rest), true));
    }

    // Check if first token is a named argument (ident = not ==)
    let saved = *input;
    let is_named = if let Ok(Token::Ident(_)) = peek(keyword_or_ident).parse_next(input) {
        skip_trivia(input)?;
        input.starts_with('=') && !input.starts_with("==")
    } else {
        false
    };
    *input = saved;

    if is_named {
        // TODO: parse named arguments properly
        // For now, consume everything up to the matching ) by finding the balance
        let content = consume_inside_parens(input)?;
        let expr = Absyn::Exp::STRING { value: content };
        return Ok((cons(Rc::new(expr), List::Nil()), true));
    }

    let e1 = expression(input)?;
    skip_trivia(input)?;
    if input.starts_with(',') {
        ",".parse_next(input)?;
        let (rest, _) = output_expression_list(input)?;
        let mut result = rest;
        // If rest is nil, add trailing WILD for (a,) pattern
        if result.is_empty() {
            let wild = Rc::new(Absyn::Exp::CREF { componentRef: Rc::new(Absyn::ComponentRef::WILD {}) });
            result = cons(wild, result);
        }
        return Ok((cons(Rc::new(e1), result), true));
    }
    ")".parse_next(input)?;
    Ok((cons(Rc::new(e1), List::Nil()), false))
}

/// Consume content inside parentheses when already inside (opening paren already consumed).
/// Returns content without the parens.
fn consume_inside_parens<'a>(input: &mut &'a str) -> ModalResult<String> {
    let mut depth = 1i32;
    let mut in_string = false;
    let mut escape_next = false;
    let mut pos = 0;
    let bytes = input.as_bytes();

    while pos < bytes.len() && depth > 0 {
        let ch = bytes[pos] as char;
        if escape_next {
            escape_next = false;
            pos += 1;
            continue;
        }
        match ch {
            '\\' if in_string => {
                escape_next = true;
                pos += 1;
            }
            '"' => {
                in_string = !in_string;
                pos += 1;
            }
            '(' if !in_string => {
                depth += 1;
                pos += 1;
            }
            ')' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    let content = input[..pos].to_string();
                    *input = &input[pos + 1..];
                    return Ok(content);
                }
                pos += 1;
            }
            _ => {
                pos += 1;
            }
        }
    }

    Err(ErrMode::Cut(ContextError::default()))
}

/// matrix_expression_list: expression_list (; expression_list)*
fn matrix_expression_list<'a>(input: &mut &'a str) -> ModalResult<List<List<Rc<Absyn::Exp>>>> {
    let row = expression_list(input)?;
    let mut rows: List<List<Rc<Absyn::Exp>>> = cons(row, List::Nil());
    loop {
        skip_trivia(input)?;
        if input.starts_with(';') && !input.starts_with(";;") {
            ";".parse_next(input)?;
            skip_trivia(input)?;
            if input.starts_with(']') { break; }
            match expression_list(input) {
                Ok(r) => rows = cons(r, rows),
                Err(_) => break,
            }
        } else {
            break;
        }
    }
    Ok(rows.reverse())
}

/// factor: primary ((^ | .^) primary)?
fn factor<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let e1 = primary(input)?;
    skip_trivia(input)?;
    let op = if input.starts_with(".^") {
        ".^".parse_next(input)?;
        Some(Absyn::Operator::POW_EW {})
    } else if input.starts_with('^') {
        "^".parse_next(input)?;
        Some(Absyn::Operator::POW {})
    } else {
        None
    };
    if let Some(op) = op {
        let e2 = primary(input)?;
        Ok(Absyn::Exp::BINARY { exp1: Rc::new(e1), op, exp2: Rc::new(e2) })
    } else {
        Ok(e1)
    }
}

/// term: factor ((* | / | .* | ./) factor)*
fn term<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let mut e = factor(input)?;
    loop {
        skip_trivia(input)?;
        let op = if input.starts_with(".*") {
            ".*".parse_next(input)?; Some(Absyn::Operator::MUL_EW {})
        } else if input.starts_with("./") {
            "./".parse_next(input)?; Some(Absyn::Operator::DIV_EW {})
        } else if input.starts_with('*') {
            "*".parse_next(input)?; Some(Absyn::Operator::MUL {})
        } else if input.starts_with('/') {
            "/".parse_next(input)?; Some(Absyn::Operator::DIV {})
        } else { None };
        match op {
            Some(op) => {
                let e2 = factor(input)?;
                e = Absyn::Exp::BINARY { exp1: Rc::new(e), op, exp2: Rc::new(e2) };
            }
            None => break,
        }
    }
    Ok(e)
}

/// unary_arithmetic_expression: (+ | - | .+ | .-) term | term
fn unary_arithmetic_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    skip_trivia(input)?;
    let op = if input.starts_with(".+") {
        ".+".parse_next(input)?; Some(Absyn::Operator::UPLUS_EW {})
    } else if input.starts_with(".-") {
        ".-".parse_next(input)?; Some(Absyn::Operator::UMINUS_EW {})
    } else if input.starts_with('+') {
        "+".parse_next(input)?; Some(Absyn::Operator::UPLUS {})
    } else if input.starts_with('-') {
        "-".parse_next(input)?; Some(Absyn::Operator::UMINUS {})
    } else { None };
    let t = term(input)?;
    match op {
        Some(op) => Ok(Absyn::Exp::UNARY { op, exp: Rc::new(t) }),
        None => Ok(t),
    }
}

/// arithmetic_expression: unary ((+ | - | .+ | .-) term)*
fn arithmetic_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let mut e = unary_arithmetic_expression(input)?;
    loop {
        skip_trivia(input)?;
        let op = if input.starts_with(".+") {
            ".+".parse_next(input)?; Some(Absyn::Operator::ADD_EW {})
        } else if input.starts_with(".-") {
            ".-".parse_next(input)?; Some(Absyn::Operator::SUB_EW {})
        } else if input.starts_with('+') {
            "+".parse_next(input)?; Some(Absyn::Operator::ADD {})
        } else if input.starts_with('-') {
            "-".parse_next(input)?; Some(Absyn::Operator::SUB {})
        } else { None };
        match op {
            Some(op) => {
                let e2 = term(input)?;
                e = Absyn::Exp::BINARY { exp1: Rc::new(e), op, exp2: Rc::new(e2) };
            }
            None => break,
        }
    }
    Ok(e)
}

/// relation: arithmetic_expression ((< | <= | > | >= | == | <>) arithmetic_expression)?
fn relation<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let e1 = arithmetic_expression(input)?;
    skip_trivia(input)?;
    let op = if input.starts_with("<=") {
        "<=".parse_next(input)?; Some(Absyn::Operator::LESSEQ {})
    } else if input.starts_with(">=") {
        ">=".parse_next(input)?; Some(Absyn::Operator::GREATEREQ {})
    } else if input.starts_with("<>") {
        "<>".parse_next(input)?; Some(Absyn::Operator::NEQUAL {})
    } else if input.starts_with("==") {
        "==".parse_next(input)?; Some(Absyn::Operator::EQUAL {})
    } else if input.starts_with('<') {
        "<".parse_next(input)?; Some(Absyn::Operator::LESS {})
    } else if input.starts_with('>') {
        ">".parse_next(input)?; Some(Absyn::Operator::GREATER {})
    } else { None };
    match op {
        Some(op) => {
            let e2 = arithmetic_expression(input)?;
            Ok(Absyn::Exp::RELATION { exp1: Rc::new(e1), op, exp2: Rc::new(e2) })
        }
        None => Ok(e1),
    }
}

/// logical_factor: not? relation
fn logical_factor<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let has_not = matches!(opt(peek(keyword_or_ident)).parse_next(input)?, Some(Token::Not));
    if has_not { keyword_or_ident.parse_next(input)?; }
    let e = relation(input)?;
    if has_not {
        Ok(Absyn::Exp::LUNARY { op: Absyn::Operator::NOT {}, exp: Rc::new(e) })
    } else {
        Ok(e)
    }
}

/// logical_term: logical_factor (and logical_factor)*
fn logical_term<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let mut e = logical_factor(input)?;
    loop {
        skip_trivia(input)?;
        if opt("and").parse_next(input)?.is_some() {
            let e2 = logical_factor(input)?;
            e = Absyn::Exp::LBINARY { exp1: Rc::new(e), op: Absyn::Operator::AND {}, exp2: Rc::new(e2) };
        } else {
            break;
        }
    }
    Ok(e)
}

/// logical_expression: logical_term (or logical_term)*
fn logical_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let mut e = logical_term(input)?;
    loop {
        skip_trivia(input)?;
        if opt("or").parse_next(input)?.is_some() {
            let e2 = logical_term(input)?;
            e = Absyn::Exp::LBINARY { exp1: Rc::new(e), op: Absyn::Operator::OR {}, exp2: Rc::new(e2) };
        } else {
            break;
        };
    }
    Ok(e)
}

/// simple_expr: logical_expression (: logical_expression (: logical_expression)?)?
fn simple_expr<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let e1 = logical_expression(input)?;
    skip_trivia(input)?;
    if !input.starts_with(':') || input.starts_with(":=") || input.starts_with("::") {
        return Ok(e1);
    }
    ":".parse_next(input)?;
    let e2 = logical_expression(input)?;
    skip_trivia(input)?;
    if input.starts_with(':') && !input.starts_with(":=") && !input.starts_with("::") {
        ":".parse_next(input)?;
        let e3 = logical_expression(input)?;
        Ok(Absyn::Exp::RANGE { start: Rc::new(e1), step: Some(Rc::new(e2)), stop: Rc::new(e3) })
    } else {
        Ok(Absyn::Exp::RANGE { start: Rc::new(e1), step: None, stop: Rc::new(e2) })
    }
}

/// simple_expression: (ident AS simple_expression) | (simple_expr (:: simple_expression)?)
fn simple_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    // Check for ident AS pattern (MetaModelica)
    {
        let saved = *input;
        let as_pattern = (|| {
            match keyword_or_ident.parse_next(input)? {
                Token::Ident(s) => {
                    let id = s.to_string();
                    skip_trivia(input).ok();
                    if matches!(keyword_or_ident.parse_next(input)?, Token::As) {
                        return Ok(Some(id));
                    }
                    Err(ErrMode::Backtrack(ContextError::default()))
                }
                _ => Err(ErrMode::Backtrack(ContextError::default())),
            }
        })();
        match as_pattern {
            Ok(Some(id)) => {
                // Consumed ident and AS, now parse the rest
                let e = simple_expression(input)?;
                return Ok(Absyn::Exp::AS { id, exp: Rc::new(e) });
            }
            _ => { *input = saved; }
        }
    }

    let e1 = simple_expr(input)?;

    skip_trivia(input)?;
    if input.starts_with("::") {
        "::".parse_next(input)?;
        let e2 = simple_expression(input)?;
        Ok(Absyn::Exp::CONS { head: Rc::new(e1), rest: Rc::new(e2) })
    } else {
        Ok(e1)
    }
}

/// if_expression: IF cond THEN e1 (ELSEIF cond THEN e)* ELSE e2
fn if_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    match keyword_or_ident(input)? {
        Token::If => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    let cond = expression(input)?;
    match keyword_or_ident(input)? {
        Token::Then => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    let true_br = expression(input)?;
    let mut elseif: List<(Rc<Absyn::Exp>, Rc<Absyn::Exp>)> = List::Nil();
    loop {
        skip_trivia(input)?;
        if !opt("elseif").parse_next(input)?.is_some() { break; }
        let ec = expression(input)?;
        match keyword_or_ident(input)? {
            Token::Then => {}
            _ => return Err(ErrMode::Backtrack(ContextError::default())),
        }
        let et = expression(input)?;
        elseif = cons((Rc::new(ec), Rc::new(et)), elseif);
    }
    match keyword_or_ident(input)? {
        Token::Else => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    let false_br = expression(input)?;
    Ok(Absyn::Exp::IFEXP {
        ifExp: Rc::new(cond),
        trueBranch: Rc::new(true_br),
        elseBranch: Rc::new(false_br),
        elseIfBranch: elseif.reverse(),
    })
}

/// code_expression: code ( ... ) quotations — basic form
fn code_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    match keyword_or_ident(input)? {
        Token::Code => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    "(".parse_next(input)?;
    // TODO: full code expression support (equations, algorithms, elements, modifications)
    let e = expression(input)?;
    ")".parse_next(input)?;
    Ok(Absyn::Exp::CODE { code: Absyn::CodeNode::C_EXPRESSION { exp: Rc::new(e) } })
}

/// local_clause: (local element_list)?  — returns Rc-wrapped ElementItems
fn local_clause<'a>(input: &mut &'a str) -> ModalResult<List<Rc<Absyn::ElementItem>>> {
    if !matches!(peek(keyword_or_ident).parse_next(input)?, Token::Local) {
        return Ok(List::Nil());
    }
    keyword_or_ident.parse_next(input)?;
    let items = element_list(input)?;
    let mut result: List<Rc<Absyn::ElementItem>> = List::Nil();
    for item in &items {
        let ei = match item {
            ClassBodyItem::Element(elem) => Absyn::ElementItem::ELEMENTITEM { element: elem },
            ClassBodyItem::Annotation(ann) => Absyn::ElementItem::LEXER_COMMENT { comment: format!("{ann:?}") },
            _ => continue,
        };
        result = cons(Rc::new(ei), result);
    }
    Ok(result.reverse())
}

/// equation_list_then: equations up to THEN keyword
fn equation_list_then<'a>(input: &mut &'a str) -> ModalResult<List<Absyn::EquationItem>> {
    let mut items: List<Absyn::EquationItem> = List::Nil();
    loop {
        skip_trivia(input)?;
        if matches!(peek(keyword_or_ident).parse_next(input)?, Token::Then | Token::End | Token::Else) { break; }
        if input.is_empty() { break; }
        let item_text: &str = take_while(0.., |c: char| c != ';').parse_next(input)?;
        let trimmed = item_text.trim().to_string();
        if !trimmed.is_empty() {
            items = cons(Absyn::EquationItem::EQUATIONITEMCOMMENT { comment: trimmed }, items);
        }
        skip_trivia(input)?;
        if opt(";").parse_next(input)?.is_none() { break; }
    }
    Ok(items.reverse())
}

/// algorithm_list_then: algorithms up to THEN keyword
fn algorithm_list_then<'a>(input: &mut &'a str) -> ModalResult<List<Absyn::AlgorithmItem>> {
    let mut items: List<Absyn::AlgorithmItem> = List::Nil();
    loop {
        skip_trivia(input)?;
        if matches!(peek(keyword_or_ident).parse_next(input)?, Token::Then | Token::End | Token::Else) { break; }
        if input.is_empty() { break; }
        let item_text: &str = take_while(0.., |c: char| c != ';').parse_next(input)?;
        let trimmed = item_text.trim().to_string();
        if !trimmed.is_empty() {
            items = cons(Absyn::AlgorithmItem::ALGORITHMITEMCOMMENT { comment: trimmed }, items);
        }
        skip_trivia(input)?;
        if opt(";").parse_next(input)?.is_none() { break; }
    }
    Ok(items.reverse())
}

/// match_case_body: (equation eq_list | algorithm alg_list)?
fn match_case_body<'a>(input: &mut &'a str) -> ModalResult<Absyn::ClassPart> {
    match peek(opt(keyword_or_ident)).parse_next(input)? {
        Some(Token::Equation) => {
            keyword_or_ident.parse_next(input)?;
            let eqs = equation_list_then(input)?;
            Ok(Absyn::ClassPart::EQUATIONS { contents: eqs })
        }
        Some(Token::Algorithm) => {
            keyword_or_ident.parse_next(input)?;
            let algs = algorithm_list_then(input)?;
            Ok(Absyn::ClassPart::ALGORITHMS { contents: algs })
        }
        _ => Ok(Absyn::ClassPart::EQUATIONS { contents: List::Nil() }),
    }
}

/// onecase: CASE pattern (IF|GUARD guard)? string_comment local_clause body THEN result ;
fn match_onecase<'a>(input: &mut &'a str) -> ModalResult<Absyn::Case> {
    match keyword_or_ident(input)? {
        Token::Case => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    let pattern = expression(input)?;
    let patternGuard = match peek(keyword_or_ident).parse_next(input)? {
        Token::If | Token::Guard => {
            keyword_or_ident.parse_next(input)?;
            Some(Rc::new(expression(input)?))
        }
        _ => None,
    };
    let comment = string_comment(input)?;
    let localDecls = local_clause(input)?;
    let classPart = match_case_body(input)?;
    match keyword_or_ident(input)? {
        Token::Then => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    let result = expression(input)?;
    ";".parse_next(input)?;
    Ok(Absyn::Case::CASE {
        pattern: Rc::new(pattern),
        patternGuard,
        patternInfo: dummy_info(),
        localDecls,
        classPart,
        result: Rc::new(result),
        resultInfo: dummy_info(),
        comment,
        info: dummy_info(),
    })
}

/// match_cases: onecase* else_case?
fn match_cases<'a>(input: &mut &'a str) -> ModalResult<List<Absyn::Case>> {
    let mut cases: List<Absyn::Case> = List::Nil();
    loop {
        skip_trivia(input)?;
        match peek(keyword_or_ident).parse_next(input)? {
            Token::Case => {
                cases = cons(match_onecase(input)?, cases);
            }
            Token::Else => {
                keyword_or_ident.parse_next(input)?;
                let comment = string_comment(input)?;
                let localDecls = local_clause(input)?;
                let classPart = match_case_body(input)?;
                match keyword_or_ident(input)? {
                    Token::Then => {}
                    _ => return Err(ErrMode::Backtrack(ContextError::default())),
                }
                let result = expression(input)?;
                ";".parse_next(input)?;
                cases = cons(Absyn::Case::ELSE {
                    localDecls,
                    classPart,
                    result: Rc::new(result),
                    resultInfo: dummy_info(),
                    comment,
                    info: dummy_info(),
                }, cases);
                break;
            }
            _ => break,
        }
    }
    Ok(cases.reverse())
}

/// match_expression: (match | matchcontinue) expr string_comment local_clause cases end match/matchcontinue
fn match_expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    let matchTy = match keyword_or_ident(input)? {
        Token::Match => Absyn::MatchType::MATCH {},
        Token::Matchcontinue => Absyn::MatchType::MATCHCONTINUE {},
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    };
    let inputExp = expression(input)?;
    let comment = string_comment(input)?;
    let localDecls = local_clause(input)?;
    let cases = match_cases(input)?;
    // end match / end matchcontinue
    match keyword_or_ident(input)? {
        Token::End => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    match keyword_or_ident(input)? {
        Token::Match | Token::Matchcontinue => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    Ok(Absyn::Exp::MATCHEXP {
        matchTy,
        inputExp: Rc::new(inputExp),
        localDecls,
        cases,
        comment,
    })
}

fn expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    skip_trivia(input)?;
    match peek(opt(keyword_or_ident)).parse_next(input)? {
        Some(Token::If) => return if_expression(input),
        Some(Token::Match) | Some(Token::Matchcontinue) => return match_expression(input),
        Some(Token::Function) => return part_eval_function_expression(input),
        Some(Token::Code) => return code_expression(input),
        _ => {}
    }
    simple_expression(input)
}

fn type_specifier<'a>(input: &mut &'a str) -> ModalResult<TypeSpec> {
    let path = name_path(input)?;
    let mut ts: List<Rc<TypeSpec>> = List::Nil();
    skip_trivia(input)?;
    if opt("<").parse_next(input)?.is_some() {
        // Parse inner types as simple paths to avoid infinite recursion
        loop {
            skip_trivia(input)?;
            if input.starts_with('>') || input.is_empty() { break; }
            let inner_ts = type_specifier.parse_next(input)?;
            ts = cons(Rc::new(inner_ts), ts);
            skip_trivia(input)?;
            if opt(",").parse_next(input)?.is_some() { continue; }
            break;
        }
        ts = ts.reverse();
        skip_trivia(input)?;
        ">".parse_next(input)?;
    };
    let arrayDim = opt(array_subscripts).parse_next(input)?;
    ts = ts.reverse();
    if ts.is_empty() {
        Ok(TypeSpec::TPATH { path, arrayDim })
    } else {
        Ok(TypeSpec::TCOMPLEX { path, typeSpecs: ts, arrayDim })
    }
}

fn subscript<'a>(input: &mut &'a str) -> ModalResult<Subscript> {
    skip_trivia(input)?;
    if input.starts_with(':') && !input.starts_with(":=") && !input.starts_with("::") {
        ":".parse_next(input)?;
        return Ok(Subscript::NOSUB {});
    }
    Ok(Subscript::SUBSCRIPT { subscript: Rc::new(expression(input)?) })
}

fn array_subscripts<'a>(input: &mut &'a str) -> ModalResult<ArrayDim> {
    "[".parse_next(input)?;
    let mut subs: List<Subscript> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.starts_with(']') || input.is_empty() { break; }
        subs = cons(subscript(input)?, subs);
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_none() { break; }
    }
    "]".parse_next(input)?;
    Ok(subs.reverse())
}

fn enum_list<'a>(input: &mut &'a str) -> ModalResult<List<EnumLiteral>> {
    let mut literals: List<EnumLiteral> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.is_empty()
            || input.starts_with('|') || input.starts_with(',')
            || input.starts_with(';') || input.starts_with('"')
            || input.starts_with(')')
        { break; }
        match enum_literal(input) {
            Ok(lit) => { literals = cons(lit, literals); }
            Err(_) => break,
        }
        skip_trivia(input)?;
        if opt(",").parse_next(input)?.is_some() { continue; }
        break;
    }
    Ok(literals.reverse())
}

fn enum_literal<'a>(input: &mut &'a str) -> ModalResult<EnumLiteral> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    skip_trivia(input)?;
    if input.starts_with('=') {
        "=".parse_next(input)?;
        let _n: &str = take_while(0.., |c: char| !c.is_whitespace() && c != ',').parse_next(input)?;
    }
    Ok(EnumLiteral::ENUMLITERAL { literal: name.to_string(), comment: None })
}

fn external_part<'a>(input: &mut &'a str) -> ModalResult<ClassBodyItem> {
    // TODO: implement properly per grammar: external (STRING_COMMENT)? (IDENT = )? IDENT ( external_function_call_args )? annotation? SEMICOLON
    match peek(keyword_or_ident).parse_next(input) {
        Ok(Token::External) => {}
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    // consume "external" keyword
    keyword_or_ident(input)?;
    let mut body = String::new();
    loop {
        skip_trivia(input)?;
        if input.is_empty() || input.starts_with(';') { break; }
        let line: &str = take_while(1.., |c: char| c != ';').parse_next(input)?;
        body.push_str(line);
    }
    ";".parse_next(input)?;
    Ok(ClassBodyItem::External {
        funcName: Some(body),
        annotation_opt: None,
    })
}

fn string_comment<'a>(input: &mut &'a str) -> ModalResult<Option<String>> {
    let mut res = match opt(string_token).parse_next(input)? {
        Some(s) => s,
        None => return Ok(None),
    };
    while opt("+").parse_next(input)?.is_some() {
        res.push_str(&string_token.parse_next(input)?);
    }
    Ok(Some(res))
}

fn string_token<'a>(input: &mut &'a str) -> ModalResult<String> {
    skip_trivia(input)?;
    '"'.parse_next(input)?;
    let orig_in = *input;
    while !input.starts_with('"') && !input.is_empty() {
        if input.starts_with('\\') {
            "\\".parse_next(input)?;
            cut_err(alt(('\\', '"', '\'', '?', 'a', 'b', 'f', 'n', 'r', 't', 'v')))
                .context(StrContext::Label("string escape sequence"))
                .parse_next(input)?;
        } else {
            take_while(1.., |c: char| c != '"' && c != '\\').parse_next(input)?;
        }
    }
    cut_err('"')
        .context(StrContext::Label("closing '\"' of string literal"))
        .parse_next(input)?;
    Ok(orig_in[0..orig_in.len()-input.len()-1].to_string())
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
    fn empty_array() {
        let mut input = "{};";
        let exp = super::expression(&mut input).unwrap();
        assert_eq!(exp, Exp::ARRAY{arrayExp: List::Nil()});
    }

    #[test]
    fn array_of_3() {
        let mut input = "{1,2,3};";
        match super::expression(&mut input).unwrap() {
           Exp::ARRAY{arrayExp: lst} => assert_eq!(3, lst.len()),
           _ => assert!(false)
        };
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
                assert!(!classes.is_empty(), "Expected some class parsed: {:?}", result);
                if let List::Cons { head: class, .. } = classes {
                    let Class::CLASS { name, .. } = &*class;
                    assert_eq!(name, "SimpleSystem");
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

    #[test]
    fn parse_codegen_c() {
        let code = std::fs::read_to_string("tests/data/CodegenC.mo")
            .expect("CodegenC.mo not found");
        let result = stored_definition.parse(&*code);
        if let Some(err) = &result.err() {
            assert!(false, "expected CodegenC.mo to parse, got: {}", err);
        }
    }
}
