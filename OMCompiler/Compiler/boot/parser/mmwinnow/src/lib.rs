//! mmwinnow — winnow-based MetaModelica parser
//!
//! Lexer combinators are embedded in the parser — no separate tokenizer.
//! AST types come from `Absyn` module, matching the ANTLR3 grammar from `grammars/Modelica.g`.
#![allow(non_snake_case)]

mod Absyn;
mod metamodelica;

pub use Absyn::*;
use metamodelica::{List, cons, SourceInfo};

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
    Else, If, For, While, Try, Elseif, ElseWhen, Return,
    Break, Continue, Match, Matchcontinue, Case,
    Each, Replaceable, Declareunit, Constraint, Assert,
    Enumeration, Subtypeof, Pder, Overload,
    Flow, Stream,
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
            | Ok(Token::External) | Ok(Token::End) | Ok(Token::Initial) => break,
            Err(_) => break,
            _ => (),
        };

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
    expression.parse_next(input)
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
    println!("TODO: handle element_redeclaration");
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
    println!("TODO: handle element_replaceable");
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
        if let Ok(tok) = peek(keyword_or_ident).parse_next(input) {
            match tok {
                Token::Public | Token::Protected | Token::Equation | Token::Algorithm |
                Token::Initial | Token::End | Token::External => break,
                _ => (),
            }
        };

        let item_text: &str = take_while(0.., |c: char| c != ';').parse_next(input)?;
        let trimmed = item_text.trim().to_string();
        if !trimmed.is_empty() {
            items = cons(AlgorithmItem::ALGORITHMITEMCOMMENT { comment: trimmed }, items);
        }
        skip_trivia(input)?;
        if input.starts_with(';') {
            ";".parse_next(input)?;
        } else { break; }
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
    let inner = opt(".").parse_next(input)?;
    if inner.is_some() {
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

fn expression<'a>(input: &mut &'a str) -> ModalResult<Absyn::Exp> {
    if let Some(e) = opt(part_eval_function_expression).parse_next(input)? {
        return Ok(e);
    }
    // TODO: if_expression
    // TODO: simple_expression
    // TODO: code_expression
    // TODO: match_expression
    if let Some(s) = opt(take_while(1.., '0'..='9')).parse_next(input)? {
        return Ok(Absyn::Exp::INTEGER{value: s.parse::<i32>().unwrap()})
    }
    println!("TODO: handle expressions");
    Err(ErrMode::Backtrack(ContextError::default()))
}

fn type_specifier<'a>(input: &mut &'a str) -> ModalResult<TypeSpec> {
    let path = name_path(input)?;
println!("Got path {:?}", path);
    let mut ts: List<Rc<TypeSpec>> = List::Nil();
    skip_trivia(input)?;
    if opt("<").parse_next(input)?.is_some() {
println!("Entered <");
        // Parse inner types as simple paths to avoid infinite recursion
        loop {
            skip_trivia(input)?;
            if input.starts_with('>') || input.is_empty() { break; }
println!("Input before name_path: {}", &input[0..input.len().min(20)]);
            let inner_ts = type_specifier.parse_next(input)?;
println!("Got inner_ts {:?}", inner_ts);
            ts = cons(Rc::new(inner_ts), ts);
            skip_trivia(input)?;
            if opt(",").parse_next(input)?.is_some() { continue; }
            break;
        }
        ts = ts.reverse();
        skip_trivia(input)?;
        ">".parse_next(input)?;
    };
println!("Got ts {:?}", ts);
    let arrayDim = opt(array_subscripts).parse_next(input)?;
    ts = ts.reverse();
    if ts.is_empty() {
        Ok(TypeSpec::TPATH { path, arrayDim })
    } else {
        Ok(TypeSpec::TCOMPLEX { path, typeSpecs: ts, arrayDim })
    }
}

fn array_subscripts<'a>(input: &mut &'a str) -> ModalResult<ArrayDim> {
    "[".parse_next(input)?;
    let mut subs: List<Subscript> = List::Nil();
    loop {
        skip_trivia(input)?;
        if input.starts_with(']') || input.is_empty() { break; }
        subs = cons(Subscript::SUBSCRIPT {
            subscript: Rc::new(Exp::END {}), // TODO: parse subscript expr
        }, subs);
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
        Some(mut s) => s,
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
