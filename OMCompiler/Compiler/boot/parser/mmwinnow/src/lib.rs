//! mmwinnow — winnow-based MetaModelica parser
//!
//! Source is first tokenised by [`lexer::lex`], then parsed by the functions
//! in this file.  AST types come from the `Absyn` module, mirroring the
//! ANTLR3 grammar from `grammars/Modelica.g`.
#![allow(non_snake_case)]

pub mod Absyn;
pub mod lexer;
pub mod token_input;

pub use Absyn::*;
pub use metamodelica::List;
pub use lexer::{Token as LexToken, TokenKind, LexError};
pub use token_input::TokenInput;

use lexer::{Token, TokenKind as TK};
use token_input::{t, next_tok, peek_kind, try_tok, t_ident, t_any_ident, t_str_token};
use winnow::stream::Stream;
use metamodelica::{cons, nil, SourceInfo};

use winnow::{Parser, ModalResult, combinator::{opt, alt, cut_err}, error::{AddContext, ContextError, StrContext, StrContextValue, ErrMode}};
use std::sync::Arc;
use std::cell::RefCell;
use arcstr::{ArcStr, literal};

thread_local! {
    static CURRENT_FILE: RefCell<ArcStr> = const { RefCell::new(literal!("")) };
    /// Comments collected by the lexer alongside the token stream.
    ///
    /// Stored as a thread-local because the parser is built on winnow
    /// combinators with the input type fixed to `&[Token]`; threading an
    /// extra mutable cursor through every parser function would be a
    /// large mechanical change. This mirrors the ANTLR3 grammar, which
    /// used the global `omc_first_comment` to drive comment splicing.
    ///
    /// Mutated **only** at strategic checkpoints in the parser (between
    /// `;`-delimited items, after a class definition, etc.), so it is
    /// safe under winnow's backtracking: backtracking does not move the
    /// cursor backwards, and we only consume comments once their
    /// surrounding tokens have been committed to.
    static COMMENT_STREAM: RefCell<CommentStream> = RefCell::new(CommentStream::empty());
}

/// Parser-side view over the lexer's parallel comment stream.
#[derive(Debug, Default)]
pub struct CommentStream {
    comments: Vec<lexer::CommentToken>,
    /// Index of the next comment that has not yet been spliced into the AST.
    cursor: usize,
}

impl CommentStream {
    pub fn empty() -> Self { CommentStream { comments: Vec::new(), cursor: 0 } }
    pub fn new(comments: Vec<lexer::CommentToken>) -> Self {
        CommentStream { comments, cursor: 0 }
    }
}

/// Drain all comments whose start position is *strictly before* `(line, col)`,
/// in source order, and clone their text payloads.
///
/// Used at AST checkpoint points (between elements, equations, etc.) to flush
/// any pending comments into the surrounding container before the next item.
fn take_comments_before(line: u32, col: u32) -> Vec<ArcStr> {
    COMMENT_STREAM.with(|s| {
        let mut s = s.borrow_mut();
        let mut out = Vec::new();
        while s.cursor < s.comments.len() {
            let c = &s.comments[s.cursor];
            if c.line < line || (c.line == line && c.col < col) {
                out.push(c.text.clone());
                s.cursor += 1;
            } else {
                break;
            }
        }
        out
    })
}

/// Snapshot the current comment cursor index. Paired with
/// [`restore_comment_cursor`] to make speculative parses (e.g. `expression`,
/// which winnow may attempt and then backtrack) leave the comment stream
/// untouched on failure.
fn save_comment_cursor() -> usize {
    COMMENT_STREAM.with(|s| s.borrow().cursor)
}

/// Reset the comment cursor to a previously saved index. Used by callers that
/// drained comments during a speculative parse that ultimately backtracked,
/// so the comments are still available to the next parse attempt.
fn restore_comment_cursor(idx: usize) {
    COMMENT_STREAM.with(|s| s.borrow_mut().cursor = idx);
}

/// Drain every remaining comment. Used at end-of-stream / after the last
/// `end ClassName;` for `commentsAfterEnd`.
fn take_comments_remaining() -> Vec<ArcStr> {
    COMMENT_STREAM.with(|s| {
        let mut s = s.borrow_mut();
        let out: Vec<ArcStr> = s.comments[s.cursor..].iter().map(|c| c.text.clone()).collect();
        s.cursor = s.comments.len();
        out
    })
}

/// Position helper: `(line, col)` of the *next* token, or one past EOF.
fn next_pos(input: &TokenInput) -> (u32, u32) {
    match input.first() {
        Some(t) => (t.line, t.col),
        None => (u32::MAX, u32::MAX),
    }
}

// ---------------------------------------------------------------------------
// Grammar selector
// ---------------------------------------------------------------------------

pub struct ParserConfig {
    pub filename: String,
    pub grammar: Grammar,
}

#[derive(Clone, Copy)]
pub enum Grammar {
    Modelica2,
    Modelica3,
    MetaModelica,
    Optimica,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Parse error with the position taken directly from the failing token.
#[derive(Debug)]
pub struct ParserError {
    pub line: u32,
    pub col: u32,
    pub inner: ContextError,
}

impl ParserError {
    pub fn from_parse_error(
        err: winnow::error::ParseError<&[LexToken], ContextError>,
        all_tokens: &[LexToken],
    ) -> Self {
        let offset = err.offset();
        let (line, col) = all_tokens
            .get(offset)
            .or_else(|| all_tokens.last())
            .map(|t| (t.line, t.col))
            .unwrap_or((0, 0));
        ParserError { line, col, inner: err.inner().clone() }
    }

    pub fn display(&self) -> String {
        let mut out = format!("error: parsing failed at {} {}:{}\n", CURRENT_FILE.take(), self.line, self.col);
        let mut labels: Vec<String> = Vec::new();
        let mut expected: Vec<String> = Vec::new();
        for ctx in self.inner.context() {
            match ctx {
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
            out.push_str(&format!("  expected: {}\n", expected.join(", ")));
        }
        if !labels.is_empty() {
            out.push_str(&format!("  while parsing: {}\n", labels.join(" > ")));
        }
        if let Some(cause) = self.inner.cause() {
            out.push_str(&format!("  caused by: {}\n", cause));
        }
        out
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display())
    }
}

impl std::error::Error for ParserError {}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Lex then parse `src`.  Returns the AST or the first error encountered.
pub fn parse(src: &str, filename: &str, grammar: Grammar) -> Result<Program, Box<dyn std::error::Error>> {
    CURRENT_FILE.with(|f| *f.borrow_mut() = ArcStr::from(filename));
    let (tokens, comments) = lexer::lex_with_comments(src, grammar)?;
    COMMENT_STREAM.with(|s| *s.borrow_mut() = CommentStream::new(comments));
    let result = stored_definition
        .parse(tokens.as_slice())
        .map_err(|e| Box::new(ParserError::from_parse_error(e, &tokens)) as Box<dyn std::error::Error>);
    // Don't keep references to the previous file's comments alive across calls.
    COMMENT_STREAM.with(|s| *s.borrow_mut() = CommentStream::empty());
    result
}

// ---------------------------------------------------------------------------
// Intermediate types used during parsing
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum ClassBodyItem {
    Section { section: SectionKind, items: Arc<List<ClassBodyItem>> },
    Element(Absyn::Element),
    Annotation(Absyn::Annotation),
    /// A `//` or `/*` lexer comment captured between elements. Lowered to
    /// `ElementItem::LEXER_COMMENT` inside a class section, preserving the
    /// comment's relative source position next to the surrounding elements.
    LexerComment(ArcStr),
    Equations(Arc<List<EquationItem>>),
    InitialEquations(Arc<List<EquationItem>>),
    Algorithms(Arc<List<AlgorithmItem>>),
    InitialAlgorithms(Arc<List<AlgorithmItem>>),
    Constraints,
    External {
        /// Language tag from the `external "C"` clause (Modelica allows "C"
        /// and "FORTRAN 77"; OpenModelica only uses "C"). Absent for the bare
        /// `external` marker form.
        lang: Option<ArcStr>,
        /// Explicit C symbol name when the clause spells one out as
        /// `external "C" funcName(...)`; absent when the C name defaults to
        /// the enclosing MetaModelica function's name (the common case).
        funcName: Option<ArcStr>,
        /// Optional `output = ...` binding (`external "C" out = foo(...)`):
        /// the wrapped function returns through this component instead of
        /// through a Modelica `output` declaration position.
        output_: Option<Absyn::ComponentRef>,
        /// Positional argument expressions passed to the C function.
        args: Arc<List<Arc<Absyn::Exp>>>,
        annotation_opt: Option<Absyn::Annotation>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionKind { Public, Protected }

#[derive(Debug, Clone)]
pub enum ClassSpecifier {
    Normal  { name: Ident, body: Arc<ClassDef> },
    Extends { name: Ident, body: Arc<ClassDef> },
}

impl ClassSpecifier {
    pub fn name(&self) -> Ident {
        match self {
            ClassSpecifier::Normal  { name, .. } => name.clone(),
            ClassSpecifier::Extends { name, .. } => name.clone(),
        }
    }
    pub fn body(&self) -> Arc<ClassDef> {
        match self {
            ClassSpecifier::Normal  { body, .. } => body.clone(),
            ClassSpecifier::Extends { body, .. } => body.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct ExtendsClause {
    path: Path,
    modification: Option<Arc<List<Arc<ElementArg>>>>,
    annotation_opt: Option<Annotation>,
}

#[derive(Debug, Clone)]
struct ComponentClause {
    typePrefix: ElementAttributes,
    typeSpec: TypeSpec,
    components: Arc<List<Arc<ComponentItem>>>,
}

fn source_info(tok1: &Token, tok2: &Token) -> SourceInfo {
    let (end_line, end_col) = tok2.end_pos();
    SourceInfo {
        fileName: CURRENT_FILE.with(|f| f.borrow().clone()),
        isReadOnly: false,
        lineNumberStart: tok1.line as i32,
        columnNumberStart: tok1.col as i32,
        lineNumberEnd: end_line as i32,
        columnNumberEnd: end_col as i32,
        lastModification: metamodelica::Real::from(0.0_f64),
    }
}

// ---------------------------------------------------------------------------
// AST conversion helpers
// ---------------------------------------------------------------------------

/// Separate class-body annotation items from other items.
/// Annotations at the top level and annotations that end up as the trailing items in a
/// public/protected section (when the class has top-level public/protected blocks) are
/// both promoted to class-level annotations.
/// Returns `(non_annotation_items, annotations)`.
fn split_annotations(items: Arc<List<ClassBodyItem>>) -> (Arc<List<ClassBodyItem>>, Arc<List<Absyn::Annotation>>) {
    let mut parts: Arc<List<ClassBodyItem>> = Arc::new(List::Nil);
    let mut anns:  Arc<List<Absyn::Annotation>> = Arc::new(List::Nil);
    for item in &*items {
        match item {
            ClassBodyItem::Annotation(ann) => anns = cons(ann.clone(), anns),
            ClassBodyItem::Section { section, items: sec_items } => {
                // Annotations that appear directly in a section's element list are
                // class-level annotations (function-level ones are nested inside element bodies).
                let (inner_parts, inner_anns) = split_annotations(Arc::clone(sec_items));
                for ann in &*inner_anns { anns = cons(ann.clone(), anns); }
                parts = cons(ClassBodyItem::Section { section: *section, items: inner_parts }, parts);
            }
            other => parts = cons(other.clone(), parts),
        }
    }
    (parts.reverse(), anns.reverse())
}

fn body_items_to_classparts(items: Arc<List<ClassBodyItem>>) -> Arc<List<ClassPart>> {
    let mut res: Arc<List<ClassPart>> = Arc::new(List::Nil);
    for item in &*items {
        let converted = match item {
            ClassBodyItem::Section { section, items } => {
                let content = body_items_to_element_items(Arc::clone(items));
                match section {
                    SectionKind::Public    => ClassPart::PUBLIC    { contents: content },
                    SectionKind::Protected => ClassPart::PROTECTED { contents: content },
                }
            }
            ClassBodyItem::Element(elem) => {
                let ei = ElementItem::ELEMENTITEM { element: elem.clone() };
                ClassPart::PUBLIC { contents: cons(ei, Arc::new(List::Nil)) }
            }
            ClassBodyItem::LexerComment(text) => {
                let ei = ElementItem::LEXER_COMMENT { comment: text.clone() };
                ClassPart::PUBLIC { contents: cons(ei, Arc::new(List::Nil)) }
            }
            ClassBodyItem::Annotation(_) => unreachable!("annotations should be split out before body_items_to_classparts"),
            ClassBodyItem::Equations(items)        => ClassPart::EQUATIONS        { contents: items.clone() },
            ClassBodyItem::InitialEquations(items) => ClassPart::INITIALEQUATIONS { contents: items.clone() },
            ClassBodyItem::Algorithms(items)       => ClassPart::ALGORITHMS       { contents: items.clone() },
            ClassBodyItem::InitialAlgorithms(items)=> ClassPart::INITIALALGORITHMS{ contents: items.clone() },
            ClassBodyItem::Constraints             => ClassPart::CONSTRAINTS      { contents: Arc::new(List::Nil) },
            ClassBodyItem::External { lang, funcName, output_, args, annotation_opt } => ClassPart::EXTERNAL {
                externalDecl: ExternalDecl::EXTERNALDECL {
                    funcName: funcName.clone(),
                    lang: lang.clone(),
                    output_: output_.clone(),
                    args: args.clone(),
                    annotation_: annotation_opt.clone(),
                },
                annotation_: None,
            },
        };
        res = cons(converted, res);
    }
    res.reverse()
}

fn body_items_to_element_items(items: Arc<List<ClassBodyItem>>) -> Arc<List<ElementItem>> {
    match &*items {
        List::Nil => Arc::new(List::Nil),
        List::Cons { head, tail } => {
            let converted = match head {
                ClassBodyItem::Element(elem)         => ElementItem::ELEMENTITEM { element: elem.clone() },
                ClassBodyItem::LexerComment(text)    => ElementItem::LEXER_COMMENT { comment: text.clone() },
                _ => panic!("only Element/LexerComment items can appear inside public/protected sections, but found {:?}", head),
            };
            cons(converted, body_items_to_element_items(tail.clone()))
        }
    }
}

fn to_rc_list<T: Clone>(lst: Arc<List<T>>) -> Arc<List<Arc<T>>> {
    let mut result: Arc<List<Arc<T>>> = Arc::new(List::Nil);
    let rev = lst.reverse();
    for item in &*rev { result = cons(Arc::new(item.clone()), result); }
    result
}

fn default_element_attrs() -> ElementAttributes {
    ElementAttributes::ATTR {
        flowPrefix: false, streamPrefix: false,
        parallelism: Parallelism::NON_PARALLEL {},
        variability: Variability::VAR {},
        direction: Direction::INPUT {},
        isField: IsField::NONFIELD {},
        arrayDim: Arc::new(List::Nil),
    }
}

// ---------------------------------------------------------------------------
// Parser rules
// ---------------------------------------------------------------------------

/// stored_definition: BOM? (within_clause SEMICOLON)? class_definition_list EOF
fn stored_definition(input: &mut TokenInput) -> ModalResult<Program> {
    // Skip optional BOM token.
    if matches!(peek_kind(input), Some(TK::BOM)) { next_tok(input)?; }

    let within_ = if opt(t(TK::Within)).parse_next(input)?.is_some() {
        let path = opt(name_path).parse_next(input)?;
        cut_err(t(TK::Semi))
            .context(StrContext::Label("';' after within clause"))
            .parse_next(input)?;
        match path {
            Some(path) => Within::WITHIN { path },
            None       => Within::TOP {},
        }
    } else {
        Within::TOP {}
    };

    let classes = class_definition_list(input)?;

    if !input.is_empty() {
        eprintln!("stored_definition: remaining tokens: {:?}", &input[..input.len().min(5)]);
        return Err(ErrMode::Backtrack(ContextError::default()));
    }
    Ok(Program::PROGRAM { classes, within_ })
}

/// class_definition_list: (FINAL? class_definition SEMICOLON)*
fn class_definition_list(input: &mut TokenInput) -> ModalResult<Arc<List<Class>>> {
    let mut defs: Arc<List<Class>> = Arc::new(List::Nil);
    loop {
        if input.is_empty() { break; }
        // Take everything that lies textually before the next class header
        // (and its FINAL prefix, if present). These are this class's
        // commentsBeforeClass per ANTLR3 `Modelica.g`.
        let (next_l, next_c) = next_pos(input);
        let before: Vec<ArcStr> = take_comments_before(next_l, next_c);
        let _final = opt(t(TK::Final)).parse_next(input)?.is_some();
        if let Some(def) = opt(class_definition).parse_next(input)? {
            let def = attach_comments_before(def, before);
            defs = cons(def, defs);
            t(TK::Semi).parse_next(input)?;
        } else {
            // No further class: any comments we already drained from the
            // lookahead belong to the previously-parsed last class as
            // commentsAfterEnd.
            if !before.is_empty() {
                defs = attach_comments_after_end_on_head(defs, before);
            }
            break;
        }
    }
    // Drain anything left after the last `end Name;` (e.g. trailing
    // comments at EOF) onto the last class's commentsAfterEnd.
    let trailing = take_comments_remaining();
    if !trailing.is_empty() {
        defs = attach_comments_after_end_on_head(defs, trailing);
    }
    Ok(defs.reverse())
}

/// Returns `c` with its `commentsBeforeClass` field set to `before` (in source
/// order). Used by [`class_definition_list`].
fn attach_comments_before(c: Class, before: Vec<ArcStr>) -> Class {
    if before.is_empty() { return c; }
    let Class::CLASS {
        name, partialPrefix, finalPrefix, encapsulatedPrefix, restriction,
        body, commentsBeforeClass: _old, commentsBeforeEnd, commentsAfterEnd, info,
    } = c;
    let mut lst: Arc<List<ArcStr>> = Arc::new(List::Nil);
    for txt in before.into_iter().rev() { lst = cons(txt, lst); }
    Class::CLASS {
        name, partialPrefix, finalPrefix, encapsulatedPrefix, restriction,
        body, commentsBeforeClass: lst, commentsBeforeEnd, commentsAfterEnd, info,
    }
}

/// `defs` is a reverse-order list — its head is the most-recently-parsed
/// class. Append `tail` to that class's `commentsAfterEnd` list.
fn attach_comments_after_end_on_head(
    defs: Arc<List<Class>>,
    tail: Vec<ArcStr>,
) -> Arc<List<Class>> {
    match &*defs {
        List::Nil => defs, // no class to attach to; drop comments silently
        List::Cons { head, tail: rest } => {
            let Class::CLASS {
                name, partialPrefix, finalPrefix, encapsulatedPrefix, restriction,
                body, commentsBeforeClass, commentsBeforeEnd, commentsAfterEnd, info,
            } = head.clone();
            let lst = commentsAfterEnd;
            // Existing list ordering follows the source; append the new
            // entries to the end.
            let mut new_tail: Arc<List<ArcStr>> = Arc::new(List::Nil);
            for txt in tail.into_iter().rev() { new_tail = cons(txt, new_tail); }
            // Concatenate lst ++ new_tail.
            let mut acc = new_tail;
            let existing: Vec<ArcStr> = (&*lst).into_iter().cloned().collect();
            for txt in existing.into_iter().rev() { acc = cons(txt, acc); }
            let new_head = Class::CLASS {
                name, partialPrefix, finalPrefix, encapsulatedPrefix, restriction,
                body, commentsBeforeClass, commentsBeforeEnd, commentsAfterEnd: acc, info,
            };
            cons(new_head, rest.clone())
        }
    }
}

/// class_definition: ENCAPSULATED? PARTIAL? FINAL? class_type class_specifier
fn class_definition(input: &mut TokenInput) -> ModalResult<Class> {
    let start = *input;
    let encapsulatedPrefix = opt(t(TK::Encapsulated)).parse_next(input)?.is_some();
    let partialPrefix      = opt(t(TK::Partial)).parse_next(input)?.is_some();
    let finalPrefix        = opt(t(TK::Final)).parse_next(input)?.is_some();
    let restriction        = class_type(input)?;
    let specifier = cut_err(class_specifier)
        .context(StrContext::Label("class specifier"))
        .parse_next(input)?;
    Ok(Class::CLASS {
        name: specifier.name(), partialPrefix, finalPrefix, encapsulatedPrefix,
        restriction, body: specifier.body(),
        commentsBeforeClass: Arc::new(List::Nil), commentsBeforeEnd: Arc::new(List::Nil),
        commentsAfterEnd: Arc::new(List::Nil), info: source_info(&start[0], &start[start.len() - input.len() - 1]),
    })
}

fn class_type(input: &mut TokenInput) -> ModalResult<Restriction> {
    alt((class_type2, class_type_function)).parse_next(input)
}

fn class_type2(input: &mut TokenInput) -> ModalResult<Restriction> {
    let res = match next_tok(input)? {
        TK::Class        => Restriction::R_CLASS,
        TK::Optimization => Restriction::R_OPTIMIZATION,
        TK::Model        => Restriction::R_MODEL,
        TK::Record       => Restriction::R_RECORD,
        TK::Block        => Restriction::R_BLOCK,
        TK::Expandable   => match next_tok(input)? {
            TK::Connector => Restriction::R_EXP_CONNECTOR,
            _             => return Err(ErrMode::Backtrack(ContextError::default())),
        },
        TK::Connector    => Restriction::R_CONNECTOR,
        TK::Type         => Restriction::R_TYPE,
        TK::Package      => Restriction::R_PACKAGE,
        TK::Uniontype    => Restriction::R_UNIONTYPE,
        TK::Operator     => {
            match opt(alt((t(TK::Record),t(TK::Function)))).parse_next(input)? {
                Some(TK::Function) => Restriction::R_FUNCTION {functionRestriction: FunctionRestriction::FR_OPERATOR_FUNCTION },
                Some(TK::Record)   => Restriction::R_OPERATOR_RECORD,
                _                  => Restriction::R_OPERATOR,
            }
        },
        _                => return Err(ErrMode::Backtrack(ContextError::default())),
    };
    Ok(res)
}

fn class_type_function(input: &mut TokenInput) -> ModalResult<Restriction> {
    let purity = match opt(alt((t(TK::Pure), t(TK::Impure)))).parse_next(input)? {
        Some(TK::Pure)   => Absyn::FunctionPurity::PURE,
        Some(TK::Impure) => Absyn::FunctionPurity::IMPURE,
        _ => Absyn::FunctionPurity::NO_PURITY,
    };
    let functionRestriction = try_tok(input, |k| match k {
        TK::Operator  => Some(Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION),
        TK::Parallel  => Some(Absyn::FunctionRestriction::FR_PARALLEL_FUNCTION),
        TK::Parkernel => Some(Absyn::FunctionRestriction::FR_KERNEL_FUNCTION),
        _             => None,
    }).unwrap_or(Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity });

    t(TK::Function).parse_next(input)?;
    Ok(Absyn::Restriction::R_FUNCTION { functionRestriction })
}

fn class_specifier(input: &mut TokenInput) -> ModalResult<ClassSpecifier> {
    if opt(t(TK::Extends)).parse_next(input)?.is_some() {
        let name = cut_err(t_ident)
            .context(StrContext::Label("class name after 'extends'"))
            .parse_next(input)?;
        let modifications = opt(class_modification).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
        let comment   = string_comment(input)?;
        let parts     = cut_err(composition)
            .context(StrContext::Label("class-extends body"))
            .parse_next(input)?;
        let classParts = body_items_to_classparts(parts);
        cut_err(t(TK::End))
            .context(StrContext::Label("'end' closing class-extends"))
            .parse_next(input)?;
            let _end_name = t_ident(input)?;
        let ann = match opt(annotation).parse_next(input)? {
            Some(ann) => {
                t(TK::Semi).parse_next(input)?;
                List::new(ann)
            },
            None => Arc::new(List::Nil)
        };
        Ok(ClassSpecifier::Extends {
            name: name.clone(),
            body: Arc::new(ClassDef::CLASS_EXTENDS {
                baseClassName: name, modifications, comment, parts: classParts, ann,
            }),
        })
    } else {
        let name = t_ident(input)?;
        let body = class_specifier2(input)?;
        Ok(ClassSpecifier::Normal { name, body })
    }
}

fn class_specifier2(input: &mut TokenInput) -> ModalResult<Arc<ClassDef>> {
    if opt(t(TK::Subtypeof)).parse_next(input)?.is_some() {
        let ts = type_specifier(input)?;
        return Ok(Arc::new(ClassDef::DERIVED {
            typeSpec: TypeSpec::TCOMPLEX { path: Path::IDENT{name: "polymorphic".into()}, typeSpecs: List::new(Arc::new(ts)), arrayDim: None }, attributes: default_element_attrs(), arguments: Arc::new(List::Nil), comment: None,
        }));
    }

    if opt(t(TK::Equal)).parse_next(input)?.is_some() {
        if opt(t(TK::Enumeration)).parse_next(input)?.is_some() {
            t(TK::LParen).parse_next(input)?;
            if opt(t(TK::Colon)).parse_next(input)?.is_some() {
                t(TK::RParen).parse_next(input)?;
                return Ok(Arc::new(ClassDef::ENUMERATION {
                    enumLiterals: EnumDef::ENUM_COLON {},
                    comment: None,
                }));
            }
            let literals = cut_err(enum_list)
                .context(StrContext::Label("enumeration literal list"))
                .parse_next(input)?;
            t(TK::RParen).parse_next(input)?;
            let comment = comment.parse_next(input)?;
            return Ok(Arc::new(ClassDef::ENUMERATION {
                enumLiterals: EnumDef::ENUMLITERALS { enumLiterals: literals },
                comment,
            }));
        }
        if opt(t(TK::Overload)).parse_next(input)?.is_some() {
            // function div = $overload(OpenModelica.Internal.intDiv,OpenModelica.Internal.realDiv)
            t(TK::LParen).parse_next(input)?;
            let mut functionNames = List::new(name_path.parse_next(input)?);
            while opt(t(TK::Comma)).parse_next(input)?.is_some() {
                functionNames = cons(name_path.parse_next(input)?, functionNames);
            };
            t(TK::RParen).parse_next(input)?;
            let comment = comment.parse_next(input)?;
            return Ok(Arc::new(ClassDef::OVERLOAD { functionNames, comment }));
        }
        let attributes = type_prefix.parse_next(input)?;
        let typeSpec = cut_err(type_specifier)
            .context(StrContext::Label("type specifier after '='"))
            .parse_next(input)?;
        let arguments: Arc<List<Arc<ElementArg>>> = opt(class_modification).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
        let comment = comment.parse_next(input)?;
        return Ok(Arc::new(ClassDef::DERIVED {
            typeSpec, attributes, arguments, comment,
        }));
    }

    let mut typeVars: Arc<List<ArcStr>> = Arc::new(List::Nil);
    if opt(t(TK::Less)).parse_next(input)?.is_some() {
        loop {
            let id = t_ident(input)?;
            typeVars = cons(id, typeVars);
            if opt(t(TK::Greater)).parse_next(input)?.is_some() { break; }
            t(TK::Comma).parse_next(input)?;
        }
        typeVars = typeVars.reverse();
    } else if opt(t(TK::LParen)).parse_next(input)?.is_some() {
        // Optimica: unsupported
        return Err(ErrMode::Backtrack(ContextError::default()));
    }

    let comment   = string_comment(input)?;
    let parts     = cut_err(composition)
        .context(StrContext::Label("class body"))
        .parse_next(input)?;
    let (non_ann_parts, body_ann) = split_annotations(parts);
    let classParts = body_items_to_classparts(non_ann_parts);
    cut_err(t(TK::End))
        .context(StrContext::Label("'end' closing class body"))
        .parse_next(input)?;
    let _end_name = cut_err(t_ident)
        .context(StrContext::Label("class name after 'end'"))
        .parse_next(input)?;

    // Annotations can appear either inside the class body (body_ann) or after `end Name`
    // (Modelica2 style). Collect both into ann.
    let ann = match opt(annotation).parse_next(input)? {
        Some(ann) => {
            cut_err(t(TK::Semi)).context(StrContext::Label("';' after annotation")).parse_next(input)?;
            body_ann.append(&List::new(ann))
        },
        None => body_ann
    };

    Ok(Arc::new(ClassDef::PARTS {
        typeVars, classAttrs: Arc::new(List::Nil), classParts, ann, comment,
    }))
}

fn composition(input: &mut TokenInput) -> ModalResult<Arc<List<ClassBodyItem>>> {
    let el_items = element_list(input)?;
    let c2_items = composition2(input)?;
    let mut result = el_items.append(&c2_items);
    while let Some(ann) = opt(annotation).parse_next(input)? {
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after annotation")).parse_next(input)?;
        result = cons(ClassBodyItem::Annotation(ann), result);
    }
    Ok(result)
}

fn composition2(input: &mut TokenInput) -> ModalResult<Arc<List<ClassBodyItem>>> {
    let mut parts: Arc<List<ClassBodyItem>> = Arc::new(List::Nil);
    loop {
        if input.is_empty() { break; }
        if let Some(ext) = opt(external_part).parse_next(input)? {
            parts = cons(ext, parts); continue;
        }
        if opt(t(TK::Public)).parse_next(input)?.is_some() {
            let items = element_list(input)?;
            parts = cons(ClassBodyItem::Section { section: SectionKind::Public, items }, parts);
            continue;
        }
        if opt(t(TK::Protected)).parse_next(input)?.is_some() {
            let items = element_list(input)?;
            parts = cons(ClassBodyItem::Section { section: SectionKind::Protected, items }, parts);
            continue;
        }
        if opt(t(TK::Initial)).parse_next(input)?.is_some() {
            if opt(t(TK::Equation)).parse_next(input)?.is_some() {
                let items = cut_err(equation_section_items)
                    .context(StrContext::Label("initial equation section"))
                    .parse_next(input)?;
                parts = cons(ClassBodyItem::InitialEquations(items), parts);
            } else if opt(t(TK::Algorithm)).parse_next(input)?.is_some() {
                let items = cut_err(algorithm_section_items)
                    .context(StrContext::Label("initial algorithm section"))
                    .parse_next(input)?;
                parts = cons(ClassBodyItem::InitialAlgorithms(items), parts);
            } else {
                return Err(ErrMode::Backtrack(ContextError::default()));
            }
            continue;
        }
        if opt(t(TK::Equation)).parse_next(input)?.is_some() {
            let items = cut_err(equation_section_items)
                .context(StrContext::Label("equation section"))
                .parse_next(input)?;
            parts = cons(ClassBodyItem::Equations(items), parts); continue;
        }
        if opt(t(TK::Algorithm)).parse_next(input)?.is_some() {
            let items = cut_err(algorithm_section_items)
                .context(StrContext::Label("algorithm section"))
                .parse_next(input)?;
            parts = cons(ClassBodyItem::Algorithms(items), parts); continue;
        }
        break;
    }
    Ok(parts.reverse())
}

fn element_list(input: &mut TokenInput) -> ModalResult<Arc<List<ClassBodyItem>>> {
    let mut items: Arc<List<ClassBodyItem>> = Arc::new(List::Nil);
    loop {
        if input.is_empty() {
            // Flush any comments that follow the last element so they are
            // still preserved at the tail of the list.
            for txt in take_comments_before(u32::MAX, u32::MAX) {
                items = cons(ClassBodyItem::LexerComment(txt), items);
            }
            break;
        }
        // Drain any lexer comments whose source position precedes the next
        // token. They get spliced into the element list as LexerComment
        // items, preserving source order. This is a safe checkpoint because
        // `element_list` only commits forward through `;`-terminated items;
        // no caller backtracks across an entire element.
        let (next_l, next_c) = (input[0].line, input[0].col);
        for txt in take_comments_before(next_l, next_c) {
            items = cons(ClassBodyItem::LexerComment(txt), items);
        }
        let first_tok = &input[0];
        match peek_kind(input) {
            Some(TK::Public) | Some(TK::Protected) | Some(TK::Equation) | Some(TK::Algorithm)
            | Some(TK::External) | Some(TK::End) | Some(TK::Initial) | Some(TK::Case)
            | Some(TK::Else) | Some(TK::Then) | None => break,
            _ => {}
        }

        if let Some(ann) = opt(annotation).parse_next(input)? {
            cut_err(t(TK::Semi)).context(StrContext::Label("';' after annotation")).parse_next(input)?;
            items = cons(ClassBodyItem::Annotation(ann), items); continue;
        }
        if let Some(elem) = opt(element).parse_next(input)? {
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }
        if let Some(imp) = opt(import_clause).parse_next(input)? {
            let comment = comment.parse_next(input)?;
            let last_tok = &input[0];
            cut_err(t(TK::Semi)).context(StrContext::Label("';' after import clause")).parse_next(input)?;
            let info = source_info(first_tok, last_tok);
            let elem = Absyn::Element::ELEMENT {
                finalPrefix: false, redeclareKeywords: None,
                innerOuter: InnerOuter::NOT_INNER_OUTER, specification: ElementSpec::IMPORT { import_: imp, comment, info: info.clone() },
                info, constrainClass: None,
            };
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }
        if let Some(ext) = opt(extends_clause).parse_next(input)? {
            let last_tok = &input[0];
            cut_err(t(TK::Semi)).context(StrContext::Label("';' after extends clause")).parse_next(input)?;
            let info = source_info(first_tok, last_tok);
            let elem = Absyn::Element::ELEMENT {
                finalPrefix: false,
                redeclareKeywords: None,
                innerOuter: InnerOuter::NOT_INNER_OUTER {},
                specification: ElementSpec::EXTENDS {
                    path: ext.path,
                    elementArg: ext.modification.unwrap_or_else(|| Arc::new(List::Nil)),
                    annotationOpt: ext.annotation_opt,
                },
                info,
                constrainClass: None,
            };
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }
        // element prefixes: [ redeclare ] [ final ] [ inner ] [ outer ]
        //   then ( [replaceable] class_definition | [replaceable] component_clause )
        //   with optional constrainedby clause if replaceable
        let redeclare_  = opt(t(TK::Redeclare)).parse_next(input)?.is_some();
        let final_      = opt(t(TK::Final)).parse_next(input)?.is_some();
        let inner_      = opt(t(TK::Inner)).parse_next(input)?.is_some();
        let outer_      = opt(t(TK::Outer)).parse_next(input)?.is_some();
        let replaceable_ = opt(t(TK::Replaceable)).parse_next(input)?.is_some();

        let redeclareKeywords: Option<RedeclareKeywords> = match (redeclare_, replaceable_) {
            (true,  true)  => Some(RedeclareKeywords::REDECLARE_REPLACEABLE),
            (true,  false) => Some(RedeclareKeywords::REDECLARE),
            (false, true)  => Some(RedeclareKeywords::REPLACEABLE),
            (false, false) => None,
        };
        let innerOuter = match (inner_, outer_) {
            (true,  true)  => InnerOuter::INNER_OUTER,
            (true,  false) => InnerOuter::INNER,
            (false, true)  => InnerOuter::OUTER,
            (false, false) => InnerOuter::NOT_INNER_OUTER,
        };

        let had_prefixes = redeclare_ || final_ || inner_ || outer_ || replaceable_;

        if let Some(cls) = opt(class_definition).parse_next(input)? {
            let constrainClass = if replaceable_ && opt(t(TK::Constrainedby)).parse_next(input)?.is_some() {
                let path       = cut_err(name_path).context(StrContext::Label("path in constrainedby")).parse_next(input)?;
                let elementArg = opt(class_modification).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
                let cmt        = comment(input)?;
                Some(ConstrainClass::CONSTRAINCLASS {
                    elementSpec: ElementSpec::EXTENDS { path, elementArg, annotationOpt: None },
                    comment: cmt,
                })
            } else { None };
            let last_tok = &input[0];
            cut_err(t(TK::Semi)).context(StrContext::Label("';' after class definition")).parse_next(input)?;
            let elem = Absyn::Element::ELEMENT {
                finalPrefix: final_, redeclareKeywords, innerOuter,
                specification: ElementSpec::CLASSDEF { replaceable_, class_: Arc::new(cls) },
                info: source_info(first_tok, last_tok), constrainClass,
            };
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }
        if let Some(cc) = opt(component_clause).parse_next(input)? {
            let constrainClass = if replaceable_ && opt(t(TK::Constrainedby)).parse_next(input)?.is_some() {
                let path       = cut_err(name_path).context(StrContext::Label("path in constrainedby")).parse_next(input)?;
                let elementArg = opt(class_modification).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
                let cmt        = comment(input)?;
                Some(ConstrainClass::CONSTRAINCLASS {
                    elementSpec: ElementSpec::EXTENDS { path, elementArg, annotationOpt: None },
                    comment: cmt,
                })
            } else { None };
            let last_tok = &input[0];
            let elem = Absyn::Element::ELEMENT {
                finalPrefix: final_, redeclareKeywords, innerOuter,
                specification: ElementSpec::COMPONENTS {
                    attributes: cc.typePrefix, typeSpec: cc.typeSpec, components: cc.components,
                },
                info: source_info(first_tok, last_tok), constrainClass,
            };
            cut_err(t(TK::Semi))
                .context(StrContext::Label("';' after component list"))
                .parse_next(input)?;
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }

        if had_prefixes {
            return Err(ErrMode::Cut(ContextError::new().add_context(
                input, &input.checkpoint(),
                StrContext::Label("class definition or component clause after element prefixes"),
            )));
        }
        break;
    }
    Ok(items.reverse())
}

fn element(input: &mut TokenInput) -> ModalResult<Absyn::Element> {
    let first_tok = &input[0];
    if let Some(imp) = opt(import_clause).parse_next(input)? {
        let comment = comment.parse_next(input)?;
        let last_tok = &input[0];
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after import clause")).parse_next(input)?;
        let info = source_info(first_tok, last_tok);
        let elem = Absyn::Element::ELEMENT {
            finalPrefix: false, redeclareKeywords: None,
            innerOuter: InnerOuter::NOT_INNER_OUTER, specification: ElementSpec::IMPORT { import_: imp, comment, info: info.clone() },
            info, constrainClass: None,
        };
        return Ok(elem);
    }
    if let Some(ext) = opt(extends_clause).parse_next(input)? {
        let last_tok = &input[0];
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after extends clause")).parse_next(input)?;
        let info = source_info(first_tok, last_tok);
        let elem = Absyn::Element::ELEMENT {
            finalPrefix: false,
            redeclareKeywords: None,
            innerOuter: InnerOuter::NOT_INNER_OUTER {},
            specification: ElementSpec::EXTENDS {
                path: ext.path,
                elementArg: ext.modification.unwrap_or_else(|| Arc::new(List::Nil)),
                annotationOpt: ext.annotation_opt,
            },
            info,
            constrainClass: None,
        };
        return Ok(elem);
    }
    // element prefixes: [ redeclare ] [ final ] [ inner ] [ outer ]
    //   then ( [replaceable] class_definition | [replaceable] component_clause )
    //   with optional constrainedby clause if replaceable
    let redeclare_  = opt(t(TK::Redeclare)).parse_next(input)?.is_some();
    let final_      = opt(t(TK::Final)).parse_next(input)?.is_some();
    let inner_      = opt(t(TK::Inner)).parse_next(input)?.is_some();
    let outer_      = opt(t(TK::Outer)).parse_next(input)?.is_some();
    let replaceable_ = opt(t(TK::Replaceable)).parse_next(input)?.is_some();

    let redeclareKeywords: Option<RedeclareKeywords> = match (redeclare_, replaceable_) {
        (true,  true)  => Some(RedeclareKeywords::REDECLARE_REPLACEABLE),
        (true,  false) => Some(RedeclareKeywords::REDECLARE),
        (false, true)  => Some(RedeclareKeywords::REPLACEABLE),
        (false, false) => None,
    };
    let innerOuter = match (inner_, outer_) {
        (true,  true)  => InnerOuter::INNER_OUTER,
        (true,  false) => InnerOuter::INNER,
        (false, true)  => InnerOuter::OUTER,
        (false, false) => InnerOuter::NOT_INNER_OUTER,
    };

    let had_prefixes = redeclare_ || final_ || inner_ || outer_ || replaceable_;

    if let Some(cls) = opt(class_definition).parse_next(input)? {
        let constrainClass = if replaceable_ && opt(t(TK::Constrainedby)).parse_next(input)?.is_some() {
            let path       = cut_err(name_path).context(StrContext::Label("path in constrainedby")).parse_next(input)?;
            let elementArg = opt(class_modification).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
            let cmt        = comment(input)?;
            Some(ConstrainClass::CONSTRAINCLASS {
                elementSpec: ElementSpec::EXTENDS { path, elementArg, annotationOpt: None },
                comment: cmt,
            })
        } else { None };
        let last_tok = &input[0];
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after class definition")).parse_next(input)?;
        let elem = Absyn::Element::ELEMENT {
            finalPrefix: final_, redeclareKeywords, innerOuter,
            specification: ElementSpec::CLASSDEF { replaceable_, class_: Arc::new(cls) },
            info: source_info(first_tok, last_tok), constrainClass,
        };
        return Ok(elem);
    }
    if let Some(cc) = opt(component_clause).parse_next(input)? {
        let constrainClass = if replaceable_ && opt(t(TK::Constrainedby)).parse_next(input)?.is_some() {
            let path       = cut_err(name_path).context(StrContext::Label("path in constrainedby")).parse_next(input)?;
            let elementArg = opt(class_modification).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
            let cmt        = comment(input)?;
            Some(ConstrainClass::CONSTRAINCLASS {
                elementSpec: ElementSpec::EXTENDS { path, elementArg, annotationOpt: None },
                comment: cmt,
            })
        } else { None };
        let last_tok = &input[0];
        let elem = Absyn::Element::ELEMENT {
            finalPrefix: final_, redeclareKeywords, innerOuter,
            specification: ElementSpec::COMPONENTS {
                attributes: cc.typePrefix, typeSpec: cc.typeSpec, components: cc.components,
            },
            info: source_info(first_tok, last_tok), constrainClass,
        };
        cut_err(t(TK::Semi))
            .context(StrContext::Label("';' after component list"))
            .parse_next(input)?;
        return Ok(elem);
    }

    if had_prefixes {
        return Err(ErrMode::Cut(ContextError::new().add_context(
            input, &input.checkpoint(),
            StrContext::Label("class definition or component clause after element prefixes"),
        )));
    }
    Err(ErrMode::Backtrack(ContextError::default()))
}

fn type_prefix(input: &mut TokenInput) -> ModalResult<ElementAttributes> {
    let flow   = try_tok(input, |k| matches!(k, TK::Flow).then_some(())).is_some();
    let stream = !flow && try_tok(input, |k| matches!(k, TK::Stream).then_some(())).is_some();

    let parallelism = try_tok(input, |k| match k {
        TK::Parlocal  => Some(Parallelism::PARLOCAL),
        TK::Parglobal => Some(Parallelism::PARGLOBAL),
        _             => None,
    }).unwrap_or(Parallelism::NON_PARALLEL);

    let variability = try_tok(input, |k| match k {
        TK::Discrete  => Some(Variability::DISCRETE),
        TK::Parameter => Some(Variability::PARAM),
        TK::Constant  => Some(Variability::CONST),
        _             => None,
    }).unwrap_or(Variability::VAR);

    let has_input  = opt(t(TK::Input)).parse_next(input)?.is_some();
    let has_output = opt(t(TK::Output)).parse_next(input)?.is_some();
    let direction  = match (has_input, has_output) {
        (true,  true)  => Direction::INPUT_OUTPUT,
        (true,  false) => Direction::INPUT,
        (false, true)  => Direction::OUTPUT,
        (false, false) => Direction::BIDIR,
    };

    let is_field = try_tok(input, |k| match k {
        TK::Ident(s) if s == "field"    => Some(IsField::FIELD),
        TK::Ident(s) if s == "nonfield" => Some(IsField::NONFIELD),
        _                                => None,
    }).unwrap_or(IsField::NONFIELD);

    Ok(ElementAttributes::ATTR {
        flowPrefix: flow, streamPrefix: stream, parallelism, variability, direction,
        isField: is_field, arrayDim: Arc::new(List::Nil),
    })
}

fn component_clause(input: &mut TokenInput) -> ModalResult<ComponentClause> {
    let typePrefix = type_prefix(input)?;
    let typeSpec   = type_specifier(input)?;
    let components = cut_err(component_list)
        .context(StrContext::Label("component list"))
        .parse_next(input)?;
    Ok(ComponentClause { typePrefix, typeSpec, components })
}

fn component_list(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<ComponentItem>>>> {
    let first = component_declaration(input)?;
    let mut items = List::new(Arc::new(first));
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        items = cons(Arc::new(component_declaration(input)?), items);
    }
    Ok(items.reverse())
}

fn component_declaration(input: &mut TokenInput) -> ModalResult<ComponentItem> {
    let name = match next_tok(input)? {
        TK::Ident(n)  => n,
        TK::Operator  => literal!("operator"),
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    };
    let arrayDim  = opt(array_subscripts).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
    let m         = opt(modification).parse_next(input)?;
    let condition = if opt(t(TK::If)).parse_next(input)?.is_some() {
        Some(expression(input)?)
    } else { None };
    let _comment  = string_comment(input)?;
    let _ann      = opt(annotation).parse_next(input)?;
    Ok(ComponentItem::COMPONENTITEM {
        component: Component::COMPONENT { name, arrayDim, modification: m },
        condition, comment: None,
    })
}

fn modification(input: &mut TokenInput) -> ModalResult<Modification> {
    let start = *input;
    let cm = opt(class_modification).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
    let eq = if opt(alt((t(TK::Assign), t(TK::Equal)))).parse_next(input)?.is_some() {
        let exp = cut_err(modification_expression)
                .context(StrContext::Label("modification expression"))
                .parse_next(input)?;
        Absyn::EqMod::EQMOD {
            exp: Arc::new(exp),
            info: source_info(&start[0], &start[start.len() - input.len() - 1]),
        }
    } else {
        Absyn::EqMod::NOMOD
    };
    Ok(Modification::CLASSMOD { elementArgLst: cm, eqMod: eq })
}

fn modification_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    if opt(t(TK::Break)).parse_next(input)?.is_some() {
        return Ok(Absyn::Exp::BREAK {});
    }
    expression(input)
}

fn class_modification(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<ElementArg>>>> {
    t(TK::LParen).parse_next(input)?;
    let arguments = opt(argument_list).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
    cut_err(t(TK::RParen))
        .context(StrContext::Label("')' closing modification list"))
        .parse_next(input)?;
    Ok(arguments)
}

fn argument_list(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<ElementArg>>>> {
    let mut res = List::new(Arc::new(argument(input)?));
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        res = cons(Arc::new(argument(input)?), res);
    }
    Ok(res.reverse())
}

fn argument(input: &mut TokenInput) -> ModalResult<ElementArg> {
    if let Some(r) = opt(element_redeclaration).parse_next(input)? { return Ok(r); }
    let eachPrefix_  = opt(t(TK::Each)).parse_next(input)?.is_some();
    let finalPrefix_ = opt(t(TK::Final)).parse_next(input)?.is_some();
    let mut res = alt((element_replaceable, element_modification)).parse_next(input)?;
    match res {
        ElementArg::MODIFICATION { ref mut eachPrefix, ref mut finalPrefix, .. } => {
            *eachPrefix  = if eachPrefix_  { Each::EACH } else { Each::NON_EACH };
            *finalPrefix = finalPrefix_;
        }
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    Ok(res)
}

// Shared body for the 'replaceable' branch: parses the class-or-component spec
// and the optional 'constrainedby' clause.  Called by both element_replaceable
// and the REDECLARE_REPLACEABLE branch of element_redeclaration.
fn parse_replaceable_spec(input: &mut TokenInput) -> ModalResult<(ElementSpec, Option<ConstrainClass>)> {
    let elementSpec = if let Some(cls) = opt(class_definition).parse_next(input)? {
        ElementSpec::CLASSDEF { replaceable_: true, class_: Arc::new(cls) }
    } else {
        let typePrefix = type_prefix(input)?;
        let typeSpec   = cut_err(type_specifier)
            .context(StrContext::Label("type specifier in replaceable"))
            .parse_next(input)?;
        let comp       = cut_err(component_declaration)
            .context(StrContext::Label("component declaration in replaceable"))
            .parse_next(input)?;
        ElementSpec::COMPONENTS { attributes: typePrefix, typeSpec, components: List::new(Arc::new(comp)) }
    };
    let constrainClass = if opt(t(TK::Constrainedby)).parse_next(input)?.is_some() {
        let path       = cut_err(name_path)
            .context(StrContext::Label("path in constrainedby clause"))
            .parse_next(input)?;
        let elementArg = opt(class_modification).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
        let comment    = comment(input)?;
        Some(ConstrainClass::CONSTRAINCLASS {
            elementSpec: ElementSpec::EXTENDS { path, elementArg, annotationOpt: None },
            comment,
        })
    } else {
        None
    };
    Ok((elementSpec, constrainClass))
}

fn element_redeclaration(input: &mut TokenInput) -> ModalResult<ElementArg> {
    let start = *input;
    t(TK::Redeclare).parse_next(input)?;
    let each_  = opt(t(TK::Each)).parse_next(input)?.is_some();
    let final_ = opt(t(TK::Final)).parse_next(input)?.is_some();

    let (redeclareKeywords, elementSpec, constrainClass) =
        if opt(t(TK::Replaceable)).parse_next(input)?.is_some() {
            let (es, cc) = parse_replaceable_spec(input)?;
            (RedeclareKeywords::REDECLARE_REPLACEABLE {}, es, cc)
        } else if let Some(cls) = opt(class_definition).parse_next(input)? {
            (RedeclareKeywords::REDECLARE, ElementSpec::CLASSDEF { replaceable_: false, class_: Arc::new(cls) }, None)
        } else {
            let typePrefix = type_prefix(input)?;
            let typeSpec   = cut_err(type_specifier)
                .context(StrContext::Label("type specifier in redeclaration"))
                .parse_next(input)?;
            let comp       = cut_err(component_declaration)
                .context(StrContext::Label("component declaration in redeclaration"))
                .parse_next(input)?;
            (RedeclareKeywords::REDECLARE, ElementSpec::COMPONENTS {
                attributes: typePrefix, typeSpec, components: List::new(Arc::new(comp)),
            }, None)
        };

    Ok(ElementArg::REDECLARATION {
        finalPrefix: final_,
        eachPrefix: if each_ { Each::EACH } else { Each::NON_EACH },
        redeclareKeywords, elementSpec, constrainClass, info: source_info(&start[0], &start[start.len() - input.len() - 1]),
    })
}

fn element_modification(input: &mut TokenInput) -> ModalResult<ElementArg> {
    let start = *input;
    let path = name_path(input)?;
    if opt(t(TK::LBracket))
        .context(StrContext::Label("subscripting modifiers not allowed"))
        .parse_next(input)?.is_some()
    {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }
    let modification = opt(modification).parse_next(input)?;
    let comment      = string_comment(input)?;
    Ok(Absyn::ElementArg::MODIFICATION {
        eachPrefix: Each::NON_EACH {}, finalPrefix: false,
        modification, comment, path, info: source_info(&start[0], &start[start.len() - input.len() - 1]),
    })
}

fn element_replaceable(input: &mut TokenInput) -> ModalResult<ElementArg> {
    let start = *input;
    t(TK::Replaceable).parse_next(input)?;
    let (elementSpec, constrainClass) = parse_replaceable_spec(input)?;
    Ok(ElementArg::REDECLARATION {
        finalPrefix: false, eachPrefix: Each::NON_EACH {},
        redeclareKeywords: RedeclareKeywords::REPLACEABLE {},
        elementSpec, constrainClass, info: source_info(&start[0], &start[start.len() - input.len() - 1]),
    })
}

fn annotation(input: &mut TokenInput) -> ModalResult<Annotation> {
    t(TK::Annotation).parse_next(input)?;
    Ok(Absyn::Annotation::ANNOTATION {
        elementArgs: cut_err(class_modification)
            .context(StrContext::Label("annotation body"))
            .parse_next(input)?,
    })
}

fn import_clause(input: &mut TokenInput) -> ModalResult<Import> {
    t(TK::Import).parse_next(input)?;
    let path = name_path(input)?;
    // Group import: import Path.{Name, NewName = OldName, ...}
    // The dot before '{' is not consumed by name_path (it only follows dots to idents).
    match opt(alt((t(TK::StarEw), t(TK::Dot), t(TK::Equal)))).parse_next(input)? {
        Some(TK::StarEw) => Ok(Import::UNQUAL_IMPORT { path }),
        Some(TK::Dot) => match alt((t(TK::LBrace),t(TK::Star))).parse_next(input)? {
            TK::Star => Ok(Import::UNQUAL_IMPORT { path }), // Modelica 2 where .* is not a separate token
            TK::LBrace => {
                let mut groups: Arc<List<GroupImport>> = Arc::new(List::Nil);
                loop {
                    let first = t_any_ident(input)?;
                    let gi = if opt(t(TK::Equal)).parse_next(input)?.is_some() {
                        GroupImport::GROUP_IMPORT_RENAME { rename: first, name: t_any_ident(input)? }
                    } else {
                        GroupImport::GROUP_IMPORT_NAME { name: first }
                    };
                    groups = cons(gi, groups);
                    if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
                }
                cut_err(t(TK::RBrace))
                    .context(StrContext::Label("'}' closing group import"))
                    .parse_next(input)?;
                Ok(Import::GROUP_IMPORT { prefix: path, groups: groups.reverse() })
            }
            _ => unreachable!(),
        },
        Some(TK::Equal) => {
            let name = match path {
                Path::IDENT{name} => name,
                _ => return Err(ErrMode::Cut(ContextError::new().add_context(
                    input,
                    &input.checkpoint(),
                    StrContext::Label("Named imports take identifiers only, but found a path before equals."),
                )))
            };
            let path = name_path.parse_next(input)?;
            Ok(Import::NAMED_IMPORT { name, path })
        }
        _ => Ok(Import::QUAL_IMPORT { path }),
    }
}

fn extends_clause(input: &mut TokenInput) -> ModalResult<ExtendsClause> {
    t(TK::Extends).parse_next(input)?;
    let path         = name_path(input)?;
    let modification = opt(class_modification).parse_next(input)?;
    let annotation_opt = opt(annotation).parse_next(input)?;
    Ok(ExtendsClause { path, modification, annotation_opt })
}

/// Parse an `external` clause according to the Modelica spec:
///
/// ```text
/// external_clause      ::= "external" [ language_specification ]
///                                     [ external_function_call ]
///                                     [ annotation ] ";"
/// language_specification ::= STRING                              // e.g. "C"
/// external_function_call ::= [ component_reference "=" ]
///                            IDENT "(" [ expression_list ] ")"
/// ```
///
/// All sub-parts are optional individually — `external;` is a legal bare
/// marker, and `external annotation(...);` skips both the language tag and
/// the function call. We commit to one possible shape with `opt` per part so
/// the grammar stays close to the spec and unusual but legal combinations
/// (`external "C"  ;`) are accepted without bespoke special cases.
fn external_part(input: &mut TokenInput) -> ModalResult<ClassBodyItem> {
    if !matches!(peek_kind(input), Some(TK::External)) {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }
    next_tok(input)?; // consume 'external'

    // 1. Optional language specification — a quoted string literal.
    let lang = opt(t_str_token).parse_next(input)?;

    // 2. Optional external function call:
    //      [ component_reference "=" ] IDENT "(" expression_list? ")"
    //
    // The `[component_reference "="]` prefix is rare (most externals omit it)
    // but legal — Modelica allows binding the C return value into a named
    // output component when the wrapping function's declared output is the
    // same component. We need a checkpoint-and-backtrack here because the
    // first identifier could be either the output component (followed by
    // `=`) or the function name (followed by `(`).
    let mut output_: Option<Absyn::ComponentRef> = None;
    let mut func_name: Option<ArcStr> = None;
    let mut args: Arc<List<Arc<Absyn::Exp>>> = nil();

    // The function-call body is only present when the next token is an
    // identifier; otherwise we are looking at `annotation` or `;`.
    if matches!(peek_kind(input), Some(TK::Ident(_))) {
        let checkpoint = input.checkpoint();
        // Try `component_reference "="` prefix first.
        let with_output = (|| -> ModalResult<(Absyn::ComponentRef, ArcStr)> {
            let cref = component_reference(input)?;
            t(TK::Equal).parse_next(input)?;
            let name = t_any_ident(input)?;
            Ok((cref, name))
        })();
        match with_output {
            Ok((cref, name)) => {
                output_ = Some(cref);
                func_name = Some(name);
            }
            Err(_) => {
                // No `=`; this identifier IS the function name.
                input.reset(&checkpoint);
                func_name = Some(t_any_ident(input)?);
            }
        }

        // Argument list. Required by the grammar once a function name is
        // present, but we accept the bare-identifier form gracefully — a few
        // MetaModelicaBuiltin entries declare e.g. `external "C" foo;`.
        if opt(t(TK::LParen)).parse_next(input)?.is_some() {
            let fa = function_arguments(input)?;
            t(TK::RParen).parse_next(input)?;
            // Only the positional-arg form is valid here — named arguments
            // and for-iterators are nonsense in an external binding. Drop
            // anything that isn't FUNCTIONARGS.args.
            if let Absyn::FunctionArgs::FUNCTIONARGS { args: a, .. } = fa {
                args = a;
            }
        }
    }

    // 3. Optional annotation, then mandatory `;`.
    let annotation_opt = opt(annotation).parse_next(input)?;
    t(TK::Semi).parse_next(input)?;

    Ok(ClassBodyItem::External {
        lang,
        funcName: func_name,
        output_,
        args,
        annotation_opt,
    })
}

// ---------------------------------------------------------------------------
// Equation / algorithm sections
// ---------------------------------------------------------------------------

fn equation_section_items(input: &mut TokenInput) -> ModalResult<Arc<List<EquationItem>>> {
    let mut items: Arc<List<EquationItem>> = Arc::new(List::Nil);
    loop {
        let (next_l, next_c) = next_pos(input);
        for txt in take_comments_before(next_l, next_c) {
            items = cons(EquationItem::EQUATIONITEMCOMMENT { comment: txt }, items);
        }
        if input.is_empty() { break; }
        match peek_kind(input) {
            Some(TK::Public) | Some(TK::Protected) | Some(TK::Equation) | Some(TK::Algorithm)
            | Some(TK::External) | Some(TK::End) | Some(TK::Initial) | Some(TK::Annotation) => break,
            _ => {}
        }
        items = cons(equation_item(input)?, items);
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after equation")).parse_next(input)?;
    }
    Ok(items.reverse())
}

fn algorithm_section_items(input: &mut TokenInput) -> ModalResult<Arc<List<AlgorithmItem>>> {
    let mut items: Arc<List<AlgorithmItem>> = Arc::new(List::Nil);
    loop {
        let (next_l, next_c) = next_pos(input);
        for txt in take_comments_before(next_l, next_c) {
            items = cons(AlgorithmItem::ALGORITHMITEMCOMMENT { comment: txt }, items);
        }
        if input.is_empty() { break; }
        match peek_kind(input) {
            Some(TK::Public) | Some(TK::Protected) | Some(TK::Equation) | Some(TK::Algorithm)
            | Some(TK::Initial) | Some(TK::End) | Some(TK::External) | Some(TK::Annotation) => break,
            _ => {}
        }
        items = cons(algorithm_item(input)?, items);
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after statement")).parse_next(input)?;
    }
    Ok(items.reverse())
}

/// Equations stopping at Then / Else / Elseif / Elsewhen / End.
fn equation_list(input: &mut TokenInput) -> ModalResult<Arc<List<EquationItem>>> {
    let mut items: Arc<List<EquationItem>> = Arc::new(List::Nil);
    loop {
        // Flush lexer comments before the next token (or after the last
        // equation if we are about to break). They become EQUATIONITEMCOMMENT
        // entries interleaved with the parsed equations, preserving the
        // original source order.
        let (next_l, next_c) = next_pos(input);
        for txt in take_comments_before(next_l, next_c) {
            items = cons(EquationItem::EQUATIONITEMCOMMENT { comment: txt }, items);
        }
        if input.is_empty() { break; }
        match peek_kind(input) {
            Some(TK::Then) | Some(TK::Else) | Some(TK::Elseif)
            | Some(TK::Elsewhen) | Some(TK::End) | None => break,
            _ => {}
        }
        items = cons(equation_item(input)?, items);
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after equation")).parse_next(input)?;
    }
    Ok(items.reverse())
}

fn equation_list_then(input: &mut TokenInput) -> ModalResult<Arc<List<Absyn::EquationItem>>> {
    equation_list(input)
}

fn equation_item(input: &mut TokenInput) -> ModalResult<EquationItem> {
    let start = *input;
    let eq = match peek_kind(input) {
        Some(TK::If)   => if_equation_e(input)?,
        Some(TK::For)  => for_equation_e(input)?,
        Some(TK::When) => when_equation_e(input)?,
        Some(TK::Failure)  => failure_equation(input)?,
        Some(TK::Connect)  => connect_equation(input)?,
        _              => equality_or_noretcall_equation(input)?,
    };
    let comment = comment(input)?;
    Ok(EquationItem::EQUATIONITEM {
        equation_: Arc::new(eq),
        comment,
        info: source_info(&start[0], &start[start.len() - input.len() - 1]),
    })
}

fn equality_or_noretcall_equation(input: &mut TokenInput) -> ModalResult<Equation> {
    let lhs = simple_expression(input)?;
    if opt(t(TK::Equal)).parse_next(input)?.is_some() {
        let rhs = cut_err(expression)
            .context(StrContext::Label("right-hand side of equation"))
            .parse_next(input)?;
        Ok(Equation::EQ_EQUALS { leftSide: lhs, rightSide: rhs })
    } else {
        match lhs {
            Absyn::Exp::CALL { function_, functionArgs, .. } =>
                Ok(Equation::EQ_NORETCALL { functionName: (*function_).clone(), functionArgs }),
            _ => Err(ErrMode::Backtrack(ContextError::default())),
        }
    }
}

fn if_equation_e(input: &mut TokenInput) -> ModalResult<Equation> {
    next_tok(input)?; // If
    let cond = cut_err(expression).parse_next(input)?;
    match cut_err(next_tok)
        .context(StrContext::Label("'then' in if-equation"))
        .parse_next(input)?
    {
        TK::Then => {}
        _        => return Err(ErrMode::Cut(ContextError::default())),
    }
    let true_items = equation_list(input)?;
    let mut else_if_branches: Vec<(Absyn::Exp, Arc<List<Arc<EquationItem>>>)> = Vec::new();
    loop {
        if !matches!(peek_kind(input), Some(TK::Elseif)) { break; }
        next_tok(input)?;
        let elif_cond = cut_err(expression).parse_next(input)?;
        match cut_err(next_tok).parse_next(input)? {
            TK::Then => {}
            _        => return Err(ErrMode::Cut(ContextError::default())),
        }
        else_if_branches.push((elif_cond, to_rc_list(equation_list(input)?)));
    }
    let else_items = if matches!(peek_kind(input), Some(TK::Else)) {
        next_tok(input)?;
        equation_list(input)?
    } else { Arc::new(List::Nil) };
    match cut_err(next_tok)
        .context(StrContext::Label("'end' closing if-equation"))
        .parse_next(input)?
    {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "if" or end-ident
    let mut elseif_list: Arc<List<(Absyn::Exp, Arc<List<Arc<EquationItem>>>)>> = Arc::new(List::Nil);
    for branch in else_if_branches.into_iter().rev() { elseif_list = cons(branch, elseif_list); }
    Ok(Equation::EQ_IF {
        ifExp: cond,
        equationTrueItems: to_rc_list(true_items),
        elseIfBranches: elseif_list,
        equationElseItems: to_rc_list(else_items),
    })
}

fn for_equation_e(input: &mut TokenInput) -> ModalResult<Equation> {
    next_tok(input)?; // For
    let iterators = cut_err(for_indices).parse_next(input)?;
    match cut_err(next_tok)
        .context(StrContext::Label("'loop' in for-equation"))
        .parse_next(input)?
    {
        TK::Loop => {}
        _        => return Err(ErrMode::Cut(ContextError::default())),
    }
    let body = equation_list(input)?;
    match cut_err(next_tok)
        .context(StrContext::Label("'end' closing for-equation"))
        .parse_next(input)?
    {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "for"
    Ok(Equation::EQ_FOR { iterators, forEquations: to_rc_list(body) })
}

fn when_equation_e(input: &mut TokenInput) -> ModalResult<Equation> {
    next_tok(input)?; // When
    let when_cond = cut_err(expression).parse_next(input)?;
    match cut_err(next_tok)
        .context(StrContext::Label("'then' in when-equation"))
        .parse_next(input)?
    {
        TK::Then => {}
        _        => return Err(ErrMode::Cut(ContextError::default())),
    }
    let when_body = equation_list(input)?;
    let mut else_when: Vec<(Absyn::Exp, Arc<List<Arc<EquationItem>>>)> = Vec::new();
    loop {
        if !matches!(peek_kind(input), Some(TK::Elsewhen)) { break; }
        next_tok(input)?;
        let ew_cond = cut_err(expression).parse_next(input)?;
        match cut_err(next_tok).parse_next(input)? {
            TK::Then => {}
            _        => return Err(ErrMode::Cut(ContextError::default())),
        }
        else_when.push((ew_cond, to_rc_list(equation_list(input)?)));
    }
    match cut_err(next_tok)
        .context(StrContext::Label("'end' closing when-equation"))
        .parse_next(input)?
    {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "when"
    let mut ew_list: Arc<List<(Absyn::Exp, Arc<List<Arc<EquationItem>>>)>> = Arc::new(List::Nil);
    for branch in else_when.into_iter().rev() { ew_list = cons(branch, ew_list); }
    Ok(Equation::EQ_WHEN_E {
        whenExp: when_cond,
        whenEquations: to_rc_list(when_body),
        elseWhenEquations: ew_list,
    })
}

fn failure_equation(input: &mut TokenInput) -> ModalResult<Equation> {
    next_tok(input)?; // Failure
    t(TK::LParen).parse_next(input)?;
    let body = equation_item(input)?;
    t(TK::RParen).parse_next(input)?;
    Ok(Equation::EQ_FAILURE { equ: body })
}

fn connect_equation(input: &mut TokenInput) -> ModalResult<Equation> {
    next_tok(input)?; // Connect
    t(TK::LParen).parse_next(input)?;
    let connector1 = cut_err(component_reference)
        .context(StrContext::Label("first connector in connect equation"))
        .parse_next(input)?;
    t(TK::Comma).parse_next(input)?;
    let connector2 = cut_err(component_reference)
        .context(StrContext::Label("second connector in connect equation"))
        .parse_next(input)?;
    t(TK::RParen).parse_next(input)?;
    Ok(Equation::EQ_CONNECT { connector1, connector2 })
}

/// Algorithm statements stopping at Then / Else / Elseif / Elsewhen / End.
fn algorithm_list(input: &mut TokenInput) -> ModalResult<Arc<List<AlgorithmItem>>> {
    let mut items: Arc<List<AlgorithmItem>> = Arc::new(List::Nil);
    loop {
        // Mirror equation_list: drain lexer comments interleaved with
        // the algorithm statements so they round-trip through the AST.
        let (next_l, next_c) = next_pos(input);
        for txt in take_comments_before(next_l, next_c) {
            items = cons(AlgorithmItem::ALGORITHMITEMCOMMENT { comment: txt }, items);
        }
        if input.is_empty() { break; }
        match peek_kind(input) {
            Some(TK::Then) | Some(TK::Else) | Some(TK::Elseif)
            | Some(TK::Elsewhen) | Some(TK::End) | None => break,
            _ => {}
        }
        items = cons(algorithm_item(input)?, items);
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after statement")).parse_next(input)?;
    }
    Ok(items.reverse())
}

fn algorithm_list_then(input: &mut TokenInput) -> ModalResult<Arc<List<Absyn::AlgorithmItem>>> {
    algorithm_list(input)
}

fn algorithm_item(input: &mut TokenInput) -> ModalResult<AlgorithmItem> {
    let start = *input;
    let alg = match peek_kind(input) {
        Some(TK::If)       => if_algorithm(input)?,
        Some(TK::For)      => for_algorithm(input)?,
        Some(TK::While)    => while_algorithm(input)?,
        Some(TK::When)     => when_algorithm(input)?,
        Some(TK::Try)      => try_algorithm(input)?,
        Some(TK::Failure)  => { failure_algorithm(input)? }
        Some(TK::Return)   => { next_tok(input)?; Algorithm::ALG_RETURN {} }
        Some(TK::Break)    => { next_tok(input)?; Algorithm::ALG_BREAK {} }
        Some(TK::Continue) => { next_tok(input)?; Algorithm::ALG_CONTINUE {} }
        _                  => assign_clause_a(input)?,
    };
    let comment = comment(input)?;
    Ok(AlgorithmItem::ALGORITHMITEM {
        algorithm_: Arc::new(alg),
        comment,
        info: source_info(&start[0], &start[start.len() - input.len() - 1]),
    })
}

fn assign_clause_a(input: &mut TokenInput) -> ModalResult<Algorithm> {
    let lhs = simple_expression(input)?;
    if matches!(peek_kind(input), Some(TK::Assign) | Some(TK::Equal)) {
        next_tok(input)?;
        let value = cut_err(expression)
            .context(StrContext::Label("right-hand side of assignment"))
            .parse_next(input)?;
        Ok(Algorithm::ALG_ASSIGN { assignComponent: lhs, value })
    } else {
        match lhs {
            Absyn::Exp::CALL { function_, functionArgs, .. } =>
                Ok(Algorithm::ALG_NORETCALL { functionCall: (*function_).clone(), functionArgs }),
            _ => Err(ErrMode::Backtrack(ContextError::default())),
        }
    }
}

fn if_algorithm(input: &mut TokenInput) -> ModalResult<Algorithm> {
    next_tok(input)?; // If
    let cond = cut_err(expression).parse_next(input)?;
    match cut_err(next_tok).context(StrContext::Label("'then' in if-algorithm")).parse_next(input)? {
        TK::Then => {}
        _        => return Err(ErrMode::Cut(ContextError::default())),
    }
    let true_items = algorithm_list(input)?;
    let mut else_if_branches: Vec<(Absyn::Exp, Arc<List<AlgorithmItem>>)> = Vec::new();
    loop {
        if !matches!(peek_kind(input), Some(TK::Elseif)) { break; }
        next_tok(input)?;
        let elif_cond = cut_err(expression).parse_next(input)?;
        match cut_err(next_tok).parse_next(input)? {
            TK::Then => {}
            _        => return Err(ErrMode::Cut(ContextError::default())),
        }
        else_if_branches.push((elif_cond, algorithm_list(input)?));
    }
    let else_items = if matches!(peek_kind(input), Some(TK::Else)) {
        next_tok(input)?; algorithm_list(input)?
    } else { Arc::new(List::Nil) };
    match cut_err(next_tok).context(StrContext::Label("'end' closing if-algorithm")).parse_next(input)? {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "if" or end-ident
    let mut elseif_list: Arc<List<(Absyn::Exp, Arc<List<AlgorithmItem>>)>> = Arc::new(List::Nil);
    for branch in else_if_branches.into_iter().rev() { elseif_list = cons(branch, elseif_list); }
    Ok(Algorithm::ALG_IF {
        ifExp: cond, trueBranch: true_items,
        elseIfAlgorithmBranch: elseif_list, elseBranch: else_items,
    })
}

fn for_algorithm(input: &mut TokenInput) -> ModalResult<Algorithm> {
    next_tok(input)?; // For
    let iterators = cut_err(for_indices).parse_next(input)?;
    match cut_err(next_tok).context(StrContext::Label("'loop' in for-algorithm")).parse_next(input)? {
        TK::Loop => {}
        _        => return Err(ErrMode::Cut(ContextError::default())),
    }
    let body = algorithm_list(input)?;
    match cut_err(next_tok).context(StrContext::Label("'end' closing for-algorithm")).parse_next(input)? {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "for"
    Ok(Algorithm::ALG_FOR { iterators, forBody: body })
}

fn while_algorithm(input: &mut TokenInput) -> ModalResult<Algorithm> {
    next_tok(input)?; // While
    let cond = cut_err(expression).parse_next(input)?;
    match cut_err(next_tok).context(StrContext::Label("'loop' in while-algorithm")).parse_next(input)? {
        TK::Loop => {}
        _        => return Err(ErrMode::Cut(ContextError::default())),
    }
    let body = algorithm_list(input)?;
    match cut_err(next_tok).context(StrContext::Label("'end' closing while-algorithm")).parse_next(input)? {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "while"
    Ok(Algorithm::ALG_WHILE { boolExpr: cond, whileBody: body })
}

fn when_algorithm(input: &mut TokenInput) -> ModalResult<Algorithm> {
    next_tok(input)?; // When
    let when_cond = cut_err(expression).parse_next(input)?;
    match cut_err(next_tok).context(StrContext::Label("'then' in when-algorithm")).parse_next(input)? {
        TK::Then => {}
        _        => return Err(ErrMode::Cut(ContextError::default())),
    }
    let when_body = algorithm_list(input)?;
    let mut else_when: Vec<(Absyn::Exp, Arc<List<AlgorithmItem>>)> = Vec::new();
    loop {
        if !matches!(peek_kind(input), Some(TK::Elsewhen)) { break; }
        next_tok(input)?;
        let ew_cond = cut_err(expression).parse_next(input)?;
        match cut_err(next_tok).parse_next(input)? {
            TK::Then => {}
            _        => return Err(ErrMode::Cut(ContextError::default())),
        }
        else_when.push((ew_cond, algorithm_list(input)?));
    }
    match cut_err(next_tok).context(StrContext::Label("'end' closing when-algorithm")).parse_next(input)? {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "when"
    let mut ew_list: Arc<List<(Absyn::Exp, Arc<List<AlgorithmItem>>)>> = Arc::new(List::Nil);
    for branch in else_when.into_iter().rev() { ew_list = cons(branch, ew_list); }
    Ok(Algorithm::ALG_WHEN_A {
        boolExpr: when_cond, whenBody: when_body, elseWhenAlgorithmBranch: ew_list,
    })
}

fn try_algorithm(input: &mut TokenInput) -> ModalResult<Algorithm> {
    next_tok(input)?; // Try
    let body = algorithm_list(input)?;
    match cut_err(next_tok).context(StrContext::Label("'else' in try-algorithm")).parse_next(input)? {
        TK::Else => {}
        _        => return Err(ErrMode::Cut(ContextError::default())),
    }
    let else_body = algorithm_list(input)?;
    match cut_err(next_tok).context(StrContext::Label("'end' closing try-algorithm")).parse_next(input)? {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "try"
    Ok(Algorithm::ALG_TRY { body, elseBody: else_body })
}

fn failure_algorithm(input: &mut TokenInput) -> ModalResult<Algorithm> {
    next_tok(input)?; // Failure
    t(TK::LParen).parse_next(input)?;
    let equ = List::new(algorithm_item.parse_next(input)?);
    t(TK::RParen).parse_next(input)?;
    Ok(Algorithm::ALG_FAILURE{equ})
}

// ---------------------------------------------------------------------------
// Match expression helpers
// ---------------------------------------------------------------------------

fn match_case_body(input: &mut TokenInput) -> ModalResult<Absyn::ClassPart> {
    match peek_kind(input) {
        /*
        Some(TK::Equation) => {
            return Err(ErrMode::Cut(ContextError::new().add_context(
                input,
                &input.checkpoint(),
                StrContext::Label("equation in match is no longer supported - use algorithm instead"),
            )));
        },
        */
        Some(TK::Equation) => {
            next_tok(input)?;
            let contents = cut_err(equation_list_then)
                .context(StrContext::Label("equation list in match case"))
                .parse_next(input)?;
            Ok(Absyn::ClassPart::EQUATIONS { contents })
        },
        Some(TK::Algorithm) => {
            next_tok(input)?;
            let contents = cut_err(algorithm_list_then)
                .context(StrContext::Label("algorithm list in match case"))
                .parse_next(input)?;
            Ok(Absyn::ClassPart::ALGORITHMS { contents })
        }
        _ => Ok(Absyn::ClassPart::ALGORITHMS { contents: Arc::new(List::Nil) }),
    }
}

fn local_clause(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<Absyn::ElementItem>>>> {
    if !matches!(peek_kind(input), Some(TK::Local)) { return Ok(Arc::new(List::Nil)); }
    next_tok(input)?; // Local
    let items = element_list(input)?;
    let mut result: Arc<List<Arc<Absyn::ElementItem>>> = Arc::new(List::Nil);
    for item in &*items {
        let ei = match item {
            ClassBodyItem::Element(elem)   => Absyn::ElementItem::ELEMENTITEM { element: elem.clone() },
            ClassBodyItem::Annotation(ann) => Absyn::ElementItem::LEXER_COMMENT { comment: arcstr::format!("{ann:?}") },
            _ => continue,
        };
        result = cons(Arc::new(ei), result);
    }
    Ok(result.reverse())
}

fn match_onecase(input: &mut TokenInput) -> ModalResult<Absyn::Case> {
    let start_token = &input[0];
    match next_tok(input)? {
        TK::Case => {}
        _        => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    let start_pattern = *input;
    let pattern = expression(input)?;
    let end_pattern = &start_pattern[start_pattern.len() - input.len() - 1];
    let patternGuard = if opt(alt((t(TK::If),t(TK::Guard)))).parse_next(input)?.is_some() {
        Some(Arc::new(expression(input)?))
    } else {
        None
    };
    let comment    = None; // string_comment(input)?;
    let localDecls = local_clause(input)?;
    let classPart  = match_case_body(input)?;
    t(TK::Then).parse_next(input)?;
    let start_exp = &input[0];
    let result = expression(input)?;
    let end_exp = &input[0];
    t(TK::Semi).parse_next(input)?;
    Ok(Absyn::Case::CASE {
        pattern: Arc::new(pattern), patternGuard, patternInfo: source_info(&start_pattern[0], end_pattern),
        localDecls, classPart, result: Arc::new(result), resultInfo: source_info(start_exp, end_exp),
        comment, info: source_info(start_token, end_exp),
    })
}

fn match_cases(input: &mut TokenInput) -> ModalResult<Arc<List<Absyn::Case>>> {
    let mut cases: Arc<List<Absyn::Case>> = Arc::new(List::Nil);
    loop {
        match peek_kind(input) {
            Some(TK::Case) => { cases = cons(match_onecase(input)?, cases); }
            Some(TK::Else) => {
                let start_else = &input[0];
                cut_err(t(TK::Else)).context(StrContext::Label("else")).parse_next(input)?;
                let comment    = None; // string_comment(input)?;
                let localDecls = local_clause(input)?;
                let classPart  = match peek_kind(input) {
                    Some(TK::Equation) => {
                        let cp = match_case_body(input)?;
                        t(TK::Then).parse_next(input)?;
                        cp
                    },
                    Some(TK::Algorithm) => {
                        let cp = match_case_body(input)?;
                        t(TK::Then).parse_next(input)?;
                        cp
                    },
                    _ => {
                        opt(t(TK::Then)).parse_next(input)?;
                        Absyn::ClassPart::ALGORITHMS { contents: Arc::new(List::Nil) }
                    },
                };
                let start_exp = &input[0];
                let result = expression(input)?;
                let end_exp = &input[0];
                opt(t(TK::Semi)).parse_next(input)?;
                cases = cons(Absyn::Case::ELSE {
                    localDecls, classPart, result: Arc::new(result),
                    resultInfo: source_info(start_exp, end_exp), comment, info: source_info(start_else, end_exp),
                }, cases);
                break;
            }
            _ => break,
        }
    }
    Ok(cases.reverse())
}

fn match_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let matchTy = match next_tok(input)? {
        TK::Match         => Absyn::MatchType::MATCH {},
        TK::Matchcontinue => Absyn::MatchType::MATCHCONTINUE {},
        _                 => return Err(ErrMode::Backtrack(ContextError::default())),
    };
    let inputExp   = expression(input)?;
    let comment    = None; // string_comment(input)?;
    let localDecls = local_clause(input)?;
    let cases      = cut_err(match_cases).
        context(StrContext::Label(match matchTy {MatchType::MATCH => "match", MatchType::MATCHCONTINUE => "matchcontinue" })).parse_next(input)?;
    match next_tok(input)? {
        TK::End => {}
        _       => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    match next_tok(input)? {
        TK::Match | TK::Matchcontinue => {}
        _                              => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    Ok(Absyn::Exp::MATCHEXP { matchTy, inputExp: Arc::new(inputExp), localDecls, cases, comment })
}

// ---------------------------------------------------------------------------
// Name / path / component reference parsers
// ---------------------------------------------------------------------------

fn name_path(input: &mut TokenInput) -> ModalResult<Path> {
    let fq  = opt(t(TK::Dot)).parse_next(input)?.is_some();
    let res = name_path2(input)?;
    if fq { Ok(Path::FULLYQUALIFIED { path: Arc::new(res) }) } else { Ok(res) }
}

fn name_path2(input: &mut TokenInput) -> ModalResult<Path> {
    let mut parts = Vec::new();
    let mut last_id = t_ident(input)?;
    loop {
        // Only treat Dot as separator if the next token after it is an Ident.
        if input.len() >= 2
            && input[0].kind == TK::Dot
            && matches!(&input[1].kind, TK::Ident(_) | TK::Code)
        {
            *input = &input[1..]; // consume Dot
            parts.push(last_id);
            last_id = t_ident(input)?;
        } else {
            break;
        }
    }
    let mut res = Path::IDENT { name: last_id };
    for id in parts.iter().rev() {
        res = Path::QUALIFIED { name: id.clone(), path: Arc::new(res) };
    }
    Ok(res)
}

fn component_reference(input: &mut TokenInput) -> ModalResult<Absyn::ComponentRef> {
    let fq = opt(t(TK::Dot)).parse_next(input)?.is_some();
    let cr = component_reference2(input)?;
    if fq { Ok(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: Arc::new(cr) }) }
    else  { Ok(cr) }
}

fn component_reference2(input: &mut TokenInput) -> ModalResult<Absyn::ComponentRef> {
    let name     = t_ident(input)?;
    let raw_subs = opt(array_subscripts).parse_next(input)?.unwrap_or_else(|| Arc::new(List::Nil));
    let mut subscripts: Arc<List<Arc<Absyn::Subscript>>> = Arc::new(List::Nil);
    for s in &*raw_subs.reverse() { subscripts = cons(Arc::new(s.clone()), subscripts); }
    if input.len() >= 2
        && input[0].kind == TK::Dot
        && matches!(&input[1].kind, TK::Ident(_))
    {
        *input = &input[1..]; // consume Dot
        let rest = component_reference2(input)?;
        Ok(Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Arc::new(rest) })
    } else {
        Ok(Absyn::ComponentRef::CREF_IDENT { name, subscripts })
    }
}

// ---------------------------------------------------------------------------
// Expression parsers
// ---------------------------------------------------------------------------

/// Inner expression parser without comment splicing. Kept private so external
/// callers can't bypass the EXPRESSIONCOMMENT wrapper.
fn expression_inner(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match peek_kind(input) {
        Some(TK::If)                             => return if_expression(input),
        Some(TK::Match) | Some(TK::Matchcontinue) => return match_expression(input),
        Some(TK::Function)                       => return part_eval_function_expression(input),
        Some(TK::Code) | Some(TK::CodeName) | Some(TK::CodeExp) | Some(TK::CodeVar) | Some(TK::CodeAnnotation) => return code_expression(input),
        _ => {}
    }
    simple_expression(input)
}

/// Parse an expression, splicing any preceding/trailing lexer comments into
/// an `EXPRESSIONCOMMENT` wrapper that matches the ANTLR3 grammar's
/// non-bootstrap behaviour at `grammars/Modelica.g:1554-1599`.
///
/// Backtracking safety: `expression` is called speculatively in many places
/// (`opt(expression)`, `alt((..., expression))`, etc.). The comment cursor
/// is therefore snapshotted on entry and restored if the inner parse fails,
/// so a backtracked attempt leaves the parallel comment stream as if we had
/// never run.
fn expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let cursor_save = save_comment_cursor();
    let input_save  = *input;

    // Comments immediately before the next token (the expression's first
    // token) become `commentsBefore`. We must drain — not peek — because a
    // nested `expression` call would otherwise re-claim the same comments.
    let (line, col) = next_pos(input);
    let before = take_comments_before(line, col);

    match expression_inner(input) {
        Ok(exp) => {
            // Comments between the expression's last token and whatever
            // follows are `commentsAfter`. We only treat comments adjacent
            // to the expression as "after"; anything past the next non-
            // comment token belongs to a later checkpoint.
            let (line, col) = next_pos(input);
            let after = take_comments_before(line, col);

            if before.is_empty() && after.is_empty() {
                Ok(exp)
            } else {
                let mut b: Arc<List<ArcStr>> = Arc::new(List::Nil);
                for t in before.into_iter().rev() { b = cons(t, b); }
                let mut a: Arc<List<ArcStr>> = Arc::new(List::Nil);
                for t in after.into_iter().rev() { a = cons(t, a); }
                Ok(Absyn::Exp::EXPRESSIONCOMMENT {
                    commentsBefore: b,
                    exp: Arc::new(exp),
                    commentsAfter: a,
                })
            }
        }
        Err(e) => {
            // Restore both streams so a higher-level `alt` / `opt` retry sees
            // exactly the same state we started with.
            *input = input_save;
            restore_comment_cursor(cursor_save);
            Err(e)
        }
    }
}

fn if_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match next_tok(input)? { TK::If => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
    let cond    = expression(input)?;
    match next_tok(input)? { TK::Then => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
    let true_br = expression(input)?;
    let mut elseif: Arc<List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>> = nil();
    loop {
        if !matches!(peek_kind(input), Some(TK::Elseif)) { break; }
        next_tok(input)?;
        let ec = expression(input)?;
        match next_tok(input)? { TK::Then => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
        let et = expression(input)?;
        elseif = cons((Arc::new(ec), Arc::new(et)), elseif);
    }
    match next_tok(input)? { TK::Else => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
    let false_br = expression(input)?;
    Ok(Absyn::Exp::IFEXP {
        ifExp: Arc::new(cond), trueBranch: Arc::new(true_br), elseBranch: Arc::new(false_br),
        elseIfBranch: elseif.reverse(),
    })
}

/// code_expression — $Code / $TypeName / $Expression / $Var / $annotation
///
/// ANTLR3 rule (simplified):
///   CODE LPAR ( initial? ( EQUATION eq | CONSTRAINT constr | ALGORITHM alg )
///             | m=modification
///             | (LPAR expr RPAR) => expr          /* Code((expr)) */
///             | (expr RPAR) => expr               /* Code(expr)   */
///             | el=element (SEMICOLON)? ) RPAR
///   | CODE_NAME LPAR name_path RPAR
///   | CODE_ANNOTATION class_modification
///   | CODE_EXP  LPAR expression RPAR
///   | CODE_VAR  LPAR component_reference RPAR
fn code_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match next_tok(input)? {
        TK::CodeName => {
            t(TK::LParen).parse_next(input)?;
            let path = name_path(input)?;
            t(TK::RParen).parse_next(input)?;
            return Ok(Exp::CODE { code: CodeNode::C_TYPENAME { path } });
        },
        TK::CodeExp => {
            t(TK::LParen).parse_next(input)?;
            let exp = expression(input)?;
            t(TK::RParen).parse_next(input)?;
            return Ok(Exp::CODE { code: CodeNode::C_EXPRESSION { exp: Arc::new(exp) } });
        },
        TK::CodeVar => {
            t(TK::LParen).parse_next(input)?;
            let componentRef = component_reference(input)?;
            t(TK::RParen).parse_next(input)?;
            return Ok(Exp::CODE { code: CodeNode::C_VARIABLENAME { componentRef: Arc::new(componentRef) } });
        },
        TK::CodeAnnotation => {
            let elementArgLst = class_modification(input)?;
            return Ok(Exp::CODE { code: CodeNode::C_MODIFICATION { modification: Modification::CLASSMOD { elementArgLst, eqMod: EqMod::NOMOD } }});
        },
        TK::Code => {
                t(TK::LParen)
                    .context(StrContext::Label("'(' after $Code"))
                    .parse_next(input)?;

                // Check for Code((expr)) — double parenthesis means wrap expression
                if let Some(e) = opt(expression).parse_next(input)? {
                    cut_err(t(TK::RParen))
                        .context(StrContext::Label("')' closing $Code"))
                        .parse_next(input)?;
                    return Ok(Exp::CODE { code: CodeNode::C_EXPRESSION { exp: Arc::new(e) } });
                }

                // Optional 'initial' keyword before equation/constraint/algorithm sections
                let initial = matches!(opt(t(TK::Initial)).parse_next(input)?, Some(TK::Initial));

                // Try EQUATION code_equation_clause
                if matches!(peek_kind(input), Some(TK::Equation)) {
                    next_tok(input)?;
                    let eq = cut_err(code_equation_clause)
                        .context(StrContext::Label("equation clause in $Code"))
                        .parse_next(input)?;
                    cut_err(t(TK::RParen))
                        .context(StrContext::Label("')' closing $Code equation"))
                        .parse_next(input)?;
                    return Ok(Exp::CODE {
                        code: CodeNode::C_EQUATIONSECTION { boolean: initial, equationItemLst: eq },
                    });
                }

                // Try CONSTRAINT code_constraint_clause
                if matches!(peek_kind(input), Some(TK::Constraint)) {
                    next_tok(input)?;
                    let constr = cut_err(code_constraint_clause)
                        .context(StrContext::Label("constraint clause in $Code"))
                        .parse_next(input)?;
                    cut_err(t(TK::RParen))
                        .context(StrContext::Label("')' closing $Code constraint"))
                        .parse_next(input)?;
                    return Ok(Exp::CODE {
                        code: CodeNode::C_CONSTRAINTSECTION { boolean: initial, equationItemLst: constr },
                    });
                }

                // Try ALGORITHM code_algorithm_clause
                if matches!(peek_kind(input), Some(TK::Algorithm)) {
                    next_tok(input)?;
                    let alg = cut_err(code_algorithm_clause)
                        .context(StrContext::Label("algorithm clause in $Code"))
                        .parse_next(input)?;
                    cut_err(t(TK::RParen))
                        .context(StrContext::Label("')' closing $Code algorithm"))
                        .parse_next(input)?;
                    return Ok(Exp::CODE {
                        code: CodeNode::C_ALGORITHMSECTION { boolean: initial, algorithmItemLst: alg },
                    });
                }

                // Try modification
                if let Ok(elementArgLst) = class_modification.parse_next(input) {
                    return Ok(Exp::CODE { code: CodeNode::C_MODIFICATION { modification: Modification::CLASSMOD { elementArgLst, eqMod: EqMod::NOMOD } }});
                }

                // Try expression followed by ')'
                if let Ok(e) = expression.parse_next(input)
                    && matches!(peek_kind(input), Some(TK::RParen)) {
                        cut_err(t(TK::RParen))
                            .context(StrContext::Label("')' closing $Code expression"))
                            .parse_next(input)?;
                        return Ok(Exp::CODE {
                            code: CodeNode::C_EXPRESSION { exp: Arc::new(e) },
                        });
                    }

                // Try element (SEMICOLON)?
                if let Ok(element) = element.parse_next(input) {
                    opt(t(TK::Semi)).parse_next(input)?;
                    return Ok(Exp::CODE {
                        code: CodeNode::C_ELEMENT { element },
                    });
                }

        },
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }

    // ---- CODE_NAME / CODE_ANNOTATION / CODE_EXP / CODE_VAR ----
    // These alternatives are distinguished by the first token:
    //   $TypeName ( … )   — first token after 'Code' would not be LParen
    //   but in our lexer $Code, $TypeName, $Expression, $Var are separate tokens.
    // Since we already consumed Code ($Code), the next token should be LParen.

    Err(ErrMode::Backtrack(ContextError::default()))
}

/// code_equation_clause: equation SEMICOLON code_equation_clause?
fn code_equation_clause(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<EquationItem>>>> {
    let eq = Arc::new(equation_item(input)?);
    t(TK::Semi).parse_next(input)?;
    let rest = opt(code_equation_clause).parse_next(input)?.unwrap_or(nil());
    Ok(cons(eq, rest))
}

/// code_constraint_clause: equation SEMICOLON code_constraint_clause?
fn code_constraint_clause(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<EquationItem>>>> {
    let eq = Arc::new(equation_item(input)?);
    t(TK::Semi).parse_next(input)?;
    let rest = opt(code_constraint_clause).parse_next(input)?.unwrap_or(nil());
    Ok(cons(eq, rest))
}

/// code_algorithm_clause: algorithm SEMICOLON code_algorithm_clause?
fn code_algorithm_clause(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<AlgorithmItem>>>> {
    let alg = Arc::new(algorithm_item(input)?);
    t(TK::Semi).parse_next(input)?;
    let rest = opt(code_algorithm_clause).parse_next(input)?.unwrap_or(nil());
    Ok(cons(alg, rest))
}

fn part_eval_function_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    t(TK::Function).parse_next(input)?;
    let cr      = component_reference(input)?;
    t(TK::LParen).parse_next(input)?;
    let argNames = opt(named_arguments).parse_next(input)?.unwrap_or(nil());
    t(TK::RParen).parse_next(input)?;
    Ok(Absyn::Exp::PARTEVALFUNCTION {
        function_: Arc::new(cr),
        functionArgs: Absyn::FunctionArgs::FUNCTIONARGS { args: nil(), argNames },
    })
}

/// simple_expression: (ident AS simple_expr) | (simple_expr (:: simple_expression)?)
fn simple_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    // Check for ident AS pattern (MetaModelica).
    {
        let saved = *input;
        let as_result: Option<ArcStr> = (|| {
            let id = match input.first() {
                Some(tok) => match &tok.kind {
                    TK::Ident(s) => s.clone(),
                    _ => return None,
                },
                None => return None,
            };
            *input = &input[1..];
            match input.first() {
                Some(tok) if tok.kind == TK::As => { *input = &input[1..]; Some(id) }
                _ => None,
            }
        })();
        match as_result {
            Some(id) => {
                let e = simple_expression(input)?;
                return Ok(Absyn::Exp::AS { id, exp: Arc::new(e) });
            }
            None => { *input = saved; }
        }
    }

    let e1 = simple_expr(input)?;
    if matches!(peek_kind(input), Some(TK::ColonColon)) {
        next_tok(input)?;
        let e2 = simple_expression(input)?;
        Ok(Absyn::Exp::CONS { head: Arc::new(e1), rest: Arc::new(e2) })
    } else {
        Ok(e1)
    }
}

/// simple_expr: logical_expression (: logical_expression (: logical_expression)?)?
fn simple_expr(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let e1 = logical_expression(input)?;
    if !matches!(peek_kind(input), Some(TK::Colon)) {
        return Ok(e1);
    }
    next_tok(input)?; // ':'
    let e2 = logical_expression(input)?;
    if matches!(peek_kind(input), Some(TK::Colon)) {
        next_tok(input)?; // ':'
        let e3 = logical_expression(input)?;
        Ok(Absyn::Exp::RANGE { start: Arc::new(e1), step: Some(Arc::new(e2)), stop: Arc::new(e3) })
    } else {
        Ok(Absyn::Exp::RANGE { start: Arc::new(e1), step: None, stop: Arc::new(e2) })
    }
}

fn logical_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let mut e = logical_term(input)?;
    loop {
        if !matches!(peek_kind(input), Some(TK::Or)) { break; }
        next_tok(input)?;
        let e2 = logical_term(input)?;
        e = Absyn::Exp::LBINARY { exp1: Arc::new(e), op: Absyn::Operator::OR {}, exp2: Arc::new(e2) };
    }
    Ok(e)
}

fn logical_term(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let mut e = logical_factor(input)?;
    loop {
        if !matches!(peek_kind(input), Some(TK::And)) { break; }
        next_tok(input)?;
        let e2 = logical_factor(input)?;
        e = Absyn::Exp::LBINARY { exp1: Arc::new(e), op: Absyn::Operator::AND {}, exp2: Arc::new(e2) };
    }
    Ok(e)
}

fn logical_factor(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let has_not = matches!(peek_kind(input), Some(TK::Not));
    if has_not { next_tok(input)?; }
    let e = relation(input)?;
    if has_not { Ok(Absyn::Exp::LUNARY { op: Absyn::Operator::NOT {}, exp: Arc::new(e) }) }
    else       { Ok(e) }
}

fn relation(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let e1 = arithmetic_expression(input)?;
    let op = match peek_kind(input) {
        Some(TK::Leq)     => { next_tok(input)?; Some(Absyn::Operator::LESSEQ {}) }
        Some(TK::Geq)     => { next_tok(input)?; Some(Absyn::Operator::GREATEREQ {}) }
        Some(TK::NotEq)   => { next_tok(input)?; Some(Absyn::Operator::NEQUAL {}) }
        Some(TK::EqEq)    => { next_tok(input)?; Some(Absyn::Operator::EQUAL {}) }
        Some(TK::Less)    => { next_tok(input)?; Some(Absyn::Operator::LESS {}) }
        Some(TK::Greater) => { next_tok(input)?; Some(Absyn::Operator::GREATER {}) }
        _                 => None,
    };
    match op {
        Some(op) => Ok(Absyn::Exp::RELATION { exp1: Arc::new(e1), op, exp2: Arc::new(arithmetic_expression(input)?) }),
        None     => Ok(e1),
    }
}

fn arithmetic_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let mut e = unary_arithmetic_expression(input)?;
    loop {
        let op = match peek_kind(input) {
            Some(TK::PlusEw)  => { next_tok(input)?; Some(Absyn::Operator::ADD_EW {}) }
            Some(TK::MinusEw) => { next_tok(input)?; Some(Absyn::Operator::SUB_EW {}) }
            Some(TK::Plus)    => { next_tok(input)?; Some(Absyn::Operator::ADD {}) }
            Some(TK::Minus)   => { next_tok(input)?; Some(Absyn::Operator::SUB {}) }
            _                 => None,
        };
        match op {
            Some(op) => { let e2 = term(input)?; e = Absyn::Exp::BINARY { exp1: Arc::new(e), op, exp2: Arc::new(e2) }; }
            None     => break,
        }
    }
    Ok(e)
}

fn unary_arithmetic_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let op = match peek_kind(input) {
        Some(TK::PlusEw)  => { next_tok(input)?; Some(Absyn::Operator::UPLUS_EW {}) }
        Some(TK::MinusEw) => { next_tok(input)?; Some(Absyn::Operator::UMINUS_EW {}) }
        Some(TK::Plus)    => { next_tok(input)?; Some(Absyn::Operator::UPLUS {}) }
        Some(TK::Minus)   => { next_tok(input)?; Some(Absyn::Operator::UMINUS {}) }
        _                 => None,
    };
    let t_expr = term(input)?;
    match op {
        Some(op) => Ok(Absyn::Exp::UNARY { op, exp: Arc::new(t_expr) }),
        None     => Ok(t_expr),
    }
}

fn term(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let mut e = factor(input)?;
    loop {
        let op = match peek_kind(input) {
            Some(TK::StarEw)  => { next_tok(input)?; Some(Absyn::Operator::MUL_EW {}) }
            Some(TK::SlashEw) => { next_tok(input)?; Some(Absyn::Operator::DIV_EW {}) }
            Some(TK::Star)    => { next_tok(input)?; Some(Absyn::Operator::MUL {}) }
            Some(TK::Slash)   => { next_tok(input)?; Some(Absyn::Operator::DIV {}) }
            _                 => None,
        };
        match op {
            Some(op) => { let e2 = factor(input)?; e = Absyn::Exp::BINARY { exp1: Arc::new(e), op, exp2: Arc::new(e2) }; }
            None     => break,
        }
    }
    Ok(e)
}

fn factor(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let e1 = primary(input)?;
    let op = match peek_kind(input) {
        Some(TK::PowerEw) => { next_tok(input)?; Some(Absyn::Operator::POW_EW {}) }
        Some(TK::Power)   => { next_tok(input)?; Some(Absyn::Operator::POW {}) }
        _                 => None,
    };
    match op {
        Some(op) => Ok(Absyn::Exp::BINARY { exp1: Arc::new(e1), op, exp2: Arc::new(primary(input)?) }),
        None     => Ok(e1),
    }
}

fn primary(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match peek_kind(input) {
        Some(TK::End)   => { next_tok(input)?; return Ok(Absyn::Exp::END {}); }
        Some(TK::True)  => { next_tok(input)?; return Ok(Absyn::Exp::BOOL { value: true  }); }
        Some(TK::False) => { next_tok(input)?; return Ok(Absyn::Exp::BOOL { value: false }); }
        Some(TK::Str(s))=> { let value = s.clone(); next_tok(input)?; return Ok(Absyn::Exp::STRING { value }); }
        Some(TK::Int(_)) | Some(TK::Real(..)) => { return number_literal(input); }
        Some(TK::LParen) => {
            next_tok(input)?;
            let (exprs, is_tuple) = output_expression_list(input)?;
            let raw_subs = opt(array_subscripts).parse_next(input)?;
            if let Some(subs) = raw_subs {
                let mut rc_subs: Arc<List<Arc<Subscript>>> = nil();
                for s in &*(subs.reverse()) { rc_subs = cons(Arc::new(s.clone()), rc_subs); }
                return Ok(Absyn::Exp::SUBSCRIPTED_EXP {
                    exp: Arc::new(to_tuple_or_exp(exprs, is_tuple)), subscripts: rc_subs,
                });
            }
            return Ok(to_tuple_or_exp(exprs, is_tuple));
        }
        Some(TK::LBracket) => {
            next_tok(input)?;
            let rows = matrix_expression_list(input)?;
            t(TK::RBracket).parse_next(input)?;
            return Ok(Absyn::Exp::MATRIX { matrix: rows });
        }
        Some(TK::LBrace) => {
            next_tok(input)?;
            let fa = for_or_expression_list(input)?;
            t(TK::RBrace).parse_next(input)?;
            return match fa {
                Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators } => {
                    let cr = Absyn::ComponentRef::CREF_IDENT { name: "$array".into(), subscripts: nil() };
                    Ok(Absyn::Exp::CALL {
                        function_: Arc::new(cr),
                        functionArgs: Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators },
                        typeVars: nil(),
                    })
                }
                Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } if argNames.is_empty() =>
                    Ok(Absyn::Exp::ARRAY { arrayExp: args }),
                _ => Err(ErrMode::Backtrack(ContextError::default())),
            };
        }
        Some(TK::Der) => {
            next_tok(input)?;
            let fa = function_call(input)?;
            let cr = Absyn::ComponentRef::CREF_IDENT { name: "der".into(), subscripts: nil() };
            return Ok(Absyn::Exp::CALL { function_: Arc::new(cr), functionArgs: fa, typeVars: nil() });
        }
        Some(TK::Pure) => {
            next_tok(input)?;
            let fa = function_call(input)?;
            let cr = Absyn::ComponentRef::CREF_IDENT { name: "pure".into(), subscripts: nil() };
            return Ok(Absyn::Exp::CALL { function_: Arc::new(cr), functionArgs: fa, typeVars: nil() });
        }
        Some(TK::Wild) => {
            next_tok(input)?;
            return Ok(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::WILD {}) });
        }
        Some(TK::Allwild) => {
            next_tok(input)?;
            return Ok(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::ALLWILD {}) });
        }
        _ => {}
    }
    component_reference__function_call(input)
}

fn to_tuple_or_exp(exprs: Arc<List<Arc<Absyn::Exp>>>, is_tuple: bool) -> Absyn::Exp {
    if is_tuple {
        Absyn::Exp::TUPLE { expressions: exprs }
    } else {
        match *exprs {
            List::Cons { ref head, .. } => (**head).clone(),
            List::Nil                 => Absyn::Exp::TUPLE { expressions: nil() },
        }
    }
}

fn number_literal(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match next_tok(input)? {
        TK::Int(n)  => Ok(Absyn::Exp::INTEGER { value: n }),
        TK::Real(_, s) => Ok(Absyn::Exp::REAL    { value: s }),
        _           => Err(ErrMode::Backtrack(ContextError::default())),
    }
}

fn component_reference__function_call(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    // initial()
    if matches!(peek_kind(input), Some(TK::Initial)) {
        next_tok(input)?;
        if matches!(peek_kind(input), Some(TK::LParen)) {
            next_tok(input)?;
            t(TK::RParen).parse_next(input)?;
            let cr = Absyn::ComponentRef::CREF_IDENT { name: "initial".into(), subscripts: nil() };
            return Ok(Absyn::Exp::CALL {
                function_: Arc::new(cr),
                functionArgs: Absyn::FunctionArgs::FUNCTIONARGS { args: nil(), argNames: nil() },
                typeVars: nil(),
            });
        }
        // Not initial() — treat 'initial' as an identifier.
        // Fall through with synthetic cref.
        return Ok(Absyn::Exp::CREF {
            componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: "initial".into(), subscripts: nil() }),
        });
    }

    let cr = component_reference(input)?;

    // Polymorphic call: cr <T1,T2,...> ( args )
    if matches!(peek_kind(input), Some(TK::Less)) {
        let saved = *input;
        if let Ok(type_vars) = (|| -> ModalResult<Arc<List<Path>>> {
            next_tok(input)?; // '<'
            let mut vars: Arc<List<Path>> = nil();
            loop {
                if matches!(peek_kind(input), Some(TK::Greater)) { break; }
                vars = cons(name_path(input)?, vars);
                if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
            }
            t(TK::Greater).parse_next(input)?;
            Ok(vars.reverse())
        })() {
            if matches!(peek_kind(input), Some(TK::LParen)) {
                let fa = function_call(input)?;
                return Ok(Absyn::Exp::CALL { function_: Arc::new(cr), functionArgs: fa, typeVars: type_vars });
            }
            *input = saved;
        } else {
            *input = saved;
        }
    }

    // Optional function call.
    if matches!(peek_kind(input), Some(TK::LParen)) {
        let fa = function_call(input)?;
        // Optional .field access after call (MetaModelica dot operator).
        if input.len() >= 2
            && input[0].kind == TK::Dot
            && matches!(&input[1].kind, TK::Ident(_))
        {
            next_tok(input)?; // Dot
            let field = expression(input)?;
            return Ok(Absyn::Exp::DOT {
                exp:   Arc::new(Absyn::Exp::CALL { function_: Arc::new(cr), functionArgs: fa, typeVars: nil() }),
                index: Arc::new(field),
            });
        }
        return Ok(Absyn::Exp::CALL { function_: Arc::new(cr), functionArgs: fa, typeVars: nil() });
    }

    Ok(Absyn::Exp::CREF { componentRef: Arc::new(cr) })
}

fn function_call(input: &mut TokenInput) -> ModalResult<Absyn::FunctionArgs> {
    t(TK::LParen).parse_next(input)?;
    let fa = function_arguments(input)?;
    t(TK::RParen).parse_next(input)?;
    Ok(fa)
}

fn function_arguments(input: &mut TokenInput) -> ModalResult<Absyn::FunctionArgs> {
    for_or_expression_list(input)
    /* for_or_expression_list returns the named arguments now, and for array they trigger an error
    match fa {
        Absyn::FunctionArgs::FOR_ITER_FARG { .. } => Ok(fa),
        Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
            if !matches!(argNames, List::Nil) {
                return Ok(Absyn::FunctionArgs::FUNCTIONARGS { args, argNames });
            }
            let argNames = opt(named_arguments).parse_next(input)?.unwrap_or(nil());
            Ok(Absyn::FunctionArgs::FUNCTIONARGS { args, argNames })
        }
    }*/
}

fn for_or_expression_list(input: &mut TokenInput) -> ModalResult<Absyn::FunctionArgs> {
    // Empty.
    if matches!(peek_kind(input), Some(TK::RParen) | Some(TK::RBrace) | None) {
        return Ok(Absyn::FunctionArgs::FUNCTIONARGS { args: nil(), argNames: nil() });
    }

    // If the first token cannot start an expression (e.g. a keyword used as a record
    // field name like `constraint = value`), try all-named-arguments directly.
    let mut checkpoint = input.checkpoint();
    let mut exp = match expression(input) {
        Ok(e) => e,
        Err(ErrMode::Backtrack(_)) => {
            input.reset(&checkpoint);
            let arg_names = named_arguments(input)?;
            return Ok(Absyn::FunctionArgs::FUNCTIONARGS {
                args: nil(),
                argNames: arg_names,
            });
        }
        Err(e) => return Err(e),
    };

    // For-iterator.
    if matches!(peek_kind(input), Some(TK::For) | Some(TK::Threaded)) {
        let threaded = if matches!(peek_kind(input), Some(TK::Threaded)) {
            next_tok(input)?; true
        } else { false };
        t(TK::For).parse_next(input)?;
        let iterators = for_indices(input)?;
        return Ok(Absyn::FunctionArgs::FOR_ITER_FARG {
            exp: Arc::new(exp),
            iterType: if threaded { Absyn::ReductionIterType::THREAD {} } else { Absyn::ReductionIterType::COMBINE {} },
            iterators,
        });
    }

    // Expression list, possibly ending with named arguments.
    let mut args: Arc<List<Arc<Absyn::Exp>>> = nil();
    let mut arg_names: Arc<List<Arc<Absyn::NamedArg>>> = nil();
    loop {
        let is_plain_ident = matches!(
            &exp,
            Exp::CREF { componentRef }
            if matches!(&**componentRef, ComponentRef::CREF_IDENT { subscripts, .. } if subscripts.is_empty())
        );
        if is_plain_ident {
            let saved = *input;
            input.reset(&checkpoint);
            match named_arguments.parse_next(input) {
                Ok(na) => { arg_names = na; break; }
                Err(_) => { *input = saved; }
            }
        }
        args = cons(Arc::new(exp), args);
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        checkpoint = input.checkpoint();
        exp = expression(input)?;
    }
    Ok(Absyn::FunctionArgs::FUNCTIONARGS { args: args.reverse(), argNames: arg_names.reverse() })
}

fn named_argument(input: &mut TokenInput) -> ModalResult<Absyn::NamedArg> {
    let argName  = t_any_ident(input)?;
    t(TK::Equal).parse_next(input)?;
    let argValue = Arc::new(expression(input)?);
    Ok(Absyn::NamedArg::NAMEDARG { argName, argValue })
}

fn named_arguments(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<Absyn::NamedArg>>>> {
    let first = named_argument(input)?;
    let mut args: Arc<List<Arc<Absyn::NamedArg>>> = cons(Arc::new(first), nil());
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        match named_argument(input) {
            Ok(arg) => args = cons(Arc::new(arg), args),
            Err(_)  => break,
        }
    }
    Ok(args.reverse())
}

fn for_indices(input: &mut TokenInput) -> ModalResult<Absyn::ForIterators> {
    let first = for_index(input)?;
    let mut result: Arc<List<Absyn::ForIterator>> = List::new(first);
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        match for_index(input) {
            Ok(fi)  => result = cons(fi, result),
            Err(_)  => break,
        }
    }
    Ok(result.reverse())
}

fn for_index(input: &mut TokenInput) -> ModalResult<Absyn::ForIterator> {
    let name = t_ident(input)?;
    let guardExp = match peek_kind(input) {
        Some(TK::If) | Some(TK::Guard) => {
            next_tok(input)?;
            Some(Arc::new(expression(input)?))
        }
        _ => None,
    };
    let range = if matches!(peek_kind(input), Some(TK::In)) {
        next_tok(input)?;
        Some(Arc::new(expression(input)?))
    } else { None };
    Ok(Absyn::ForIterator::ITERATOR { name, guardExp, range })
}

fn expression_list(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<Absyn::Exp>>>> {
    let e = expression(input)?;
    let mut result: Arc<List<Arc<Absyn::Exp>>> = cons(Arc::new(e), nil());
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        match expression(input) {
            Ok(e)  => result = cons(Arc::new(e), result),
            Err(_) => break,
        }
    }
    Ok(result.reverse())
}

/// Consumes up to and including ')'; returns (expressions, isTuple).
fn output_expression_list(input: &mut TokenInput) -> ModalResult<(Arc<List<Arc<Absyn::Exp>>>, bool)> {
    // ()
    if opt(t(TK::RParen)).parse_next(input)?.is_some() {
        return Ok((nil(), true));
    }
    // Leading comma: (, b) → WILD, b
    if opt(t(TK::Comma)).parse_next(input)?.is_some() {
        let (rest, _) = output_expression_list(input)?;
        let wild_exp = Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::WILD {}) });
        return Ok((cons(wild_exp, rest), true));
    }
    let e1 = expression(input)?;
    if opt(t(TK::Comma)).parse_next(input)?.is_some() {
        let (mut result, _) = output_expression_list(input)?;
        if result.is_empty() {
            let wild = Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::WILD {}) });
            result = cons(wild, result);
        }
        return Ok((cons(Arc::new(e1), result), true));
    }
    t(TK::RParen).parse_next(input)?;
    Ok((cons(Arc::new(e1), nil()), false))
}

fn matrix_expression_list(input: &mut TokenInput) -> ModalResult<Arc<List<Arc<List<Arc<Absyn::Exp>>>>>> {
    let row = expression_list(input)?;
    let mut rows = cons(row, nil());
    loop {
        if matches!(peek_kind(input), Some(TK::Semi)) {
            next_tok(input)?;
            if matches!(peek_kind(input), Some(TK::RBracket)) { break; }
            match expression_list(input) {
                Ok(r)  => rows = cons(r, rows),
                Err(_) => break,
            }
        } else {
            break;
        }
    }
    Ok(rows.reverse())
}

// ---------------------------------------------------------------------------
// String comments and types
// ---------------------------------------------------------------------------

fn string_comment(input: &mut TokenInput) -> ModalResult<Option<ArcStr>> {
    let mut res: String = match opt(t_str_token).parse_next(input)? {
        Some(s) => s.to_string(),
        None    => return Ok(None),
    };
    while opt(t(TK::Plus)).parse_next(input)?.is_some() {
        res.push_str(&cut_err(t_str_token).parse_next(input)?);
    }
    Ok(Some(res.into()))
}

fn comment(input: &mut TokenInput) -> ModalResult<Option<Comment>> {
    let comment = string_comment.parse_next(input)?;
    let annotation_ = opt(annotation).parse_next(input)?;
    Ok(Some(Comment::COMMENT { comment, annotation_ }))
}

fn type_specifier(input: &mut TokenInput) -> ModalResult<TypeSpec> {
    let path = name_path(input)?;
    let mut ts: Arc<List<Arc<TypeSpec>>> = nil();
    if opt(t(TK::Less)).parse_next(input)?.is_some() {
        loop {
            if matches!(peek_kind(input), Some(TK::Greater)) || input.is_empty() { break; }
            let inner_ts = type_specifier(input)?;
            ts = cons(Arc::new(inner_ts), ts);
            if opt(t(TK::Comma)).parse_next(input)?.is_some() { continue; }
            break;
        }
        ts = ts.reverse();
        t(TK::Greater).parse_next(input)?;
    }
    let arrayDim = opt(array_subscripts).parse_next(input)?;
    if ts.is_empty() {
        Ok(TypeSpec::TPATH { path, arrayDim })
    } else {
        Ok(TypeSpec::TCOMPLEX { path, typeSpecs: ts, arrayDim })
    }
}

fn subscript(input: &mut TokenInput) -> ModalResult<Subscript> {
    if matches!(peek_kind(input), Some(TK::Colon)) {
        next_tok(input)?;
        return Ok(Subscript::NOSUB {});
    }
    Ok(Subscript::SUBSCRIPT { subscript: Arc::new(expression(input)?) })
}

fn array_subscripts(input: &mut TokenInput) -> ModalResult<ArrayDim> {
    t(TK::LBracket).parse_next(input)?;
    let mut subs: Arc<List<Subscript>> = nil();
    loop {
        if matches!(peek_kind(input), Some(TK::RBracket)) || input.is_empty() { break; }
        subs = cons(subscript(input)?, subs);
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
    }
    t(TK::RBracket).parse_next(input)?;
    Ok(subs.reverse())
}

fn enum_list(input: &mut TokenInput) -> ModalResult<Arc<List<EnumLiteral>>> {
    let mut literals: Arc<List<EnumLiteral>> = nil();
    loop {
        match peek_kind(input) {
            None | Some(TK::Pipe) | Some(TK::Comma) | Some(TK::Semi)
            | Some(TK::Str(_)) | Some(TK::RParen) => break,
            _ => {}
        }
        match enum_literal(input) {
            Ok(lit) => literals = cons(lit, literals),
            Err(_)  => break,
        }
        if opt(t(TK::Comma)).parse_next(input)?.is_some() { continue; }
        break;
    }
    Ok(literals.reverse())
}

fn enum_literal(input: &mut TokenInput) -> ModalResult<EnumLiteral> {
    let literal = t_ident(input)?;
    let comment = comment.parse_next(input)?;
    Ok(EnumLiteral::ENUMLITERAL { literal, comment })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array() {
        let tokens = lexer::lex("{};", Grammar::Modelica3).unwrap();
        let mut ts = tokens.as_slice();
        let exp = expression(&mut ts).unwrap();
        assert!(matches!(exp, Exp::ARRAY { arrayExp } if arrayExp.is_empty()));
    }

    #[test]
    fn array_expr() {
        let tokens = lexer::lex("{1,2,3};", Grammar::Modelica3).unwrap();
        let mut ts = tokens.as_slice();
        let exp = expression(&mut ts).unwrap();
        assert!(matches!(exp, Exp::ARRAY { arrayExp } if arrayExp.len() == 3));
    }

    #[test]
    fn parse_simple_package() {
        let code = "package SimpleSystem \"Returns the index...\"\n\
                    /* ... */\n\
                    Real x(start=0);\n\
                    end SimpleSystem;";
        match parse(code, "", Grammar::MetaModelica).unwrap() {
            Program::PROGRAM { classes, .. } => {
                assert!(!classes.is_empty());
                if let List::Cons { head: Class::CLASS { name, .. }, .. } = &*classes {
                    assert_eq!(&**name, "SimpleSystem");
                }
            }
        }
    }

    #[test]
    fn parse_first_token() {
        let code = "package SimpleSystem \"Returns the index...\"\nend SimpleSystem;";
        parse(code, "", Grammar::MetaModelica).expect("expected parse success");
    }

    #[test]
    fn parse_absyn() {
        let code = std::fs::read_to_string("tests/data/Absyn.mo").expect("Absyn.mo not found");
        if let Err(e) = parse(&code, "Absyn.mo", Grammar::MetaModelica) {
            panic!("expected Absyn.mo to parse: {e}");
        }
    }

    #[test]
    fn comments_spliced_into_ast() {
        // Three distinct comment placements that round-trip through the
        // parser: between classes (commentsBeforeClass / commentsAfterEnd),
        // between elements (LEXER_COMMENT inside a PUBLIC section), and
        // between algorithm statements (ALGORITHMITEMCOMMENT).
        let code = "\
// before A\n\
package A\n\
  // between elements\n\
  Real x;\n\
algorithm\n\
  // between statements\n\
  x := 1.0;\n\
end A;\n\
// after A\n\
";
        let prog = parse(code, "t.mo", Grammar::MetaModelica).expect("parse");
        let Program::PROGRAM { classes, .. } = prog;
        let first = match &*classes { List::Cons { head, .. } => head.clone(), _ => panic!("no classes") };
        let Class::CLASS { commentsBeforeClass, commentsAfterEnd, body, .. } = first;
        assert!((&*commentsBeforeClass).into_iter().any(|c| c.contains("before A")),
                "commentsBeforeClass = {:?}", commentsBeforeClass);
        assert!((&*commentsAfterEnd).into_iter().any(|c| c.contains("after A")),
                "commentsAfterEnd = {:?}", commentsAfterEnd);
        // Walk the body for the embedded comments.
        let ClassDef::PARTS { classParts, .. } = &*body else { panic!("expected PARTS"); };
        let mut saw_lexer_comment = false;
        let mut saw_alg_comment = false;
        for cp in &**classParts {
            match &*cp {
                ClassPart::PUBLIC { contents } => {
                    for ei in &**contents {
                        if let ElementItem::LEXER_COMMENT { comment } = &*ei {
                            if comment.contains("between elements") { saw_lexer_comment = true; }
                        }
                    }
                }
                ClassPart::ALGORITHMS { contents } => {
                    for ai in &**contents {
                        if let AlgorithmItem::ALGORITHMITEMCOMMENT { comment } = &*ai {
                            if comment.contains("between statements") { saw_alg_comment = true; }
                        }
                    }
                }
                _ => {}
            }
        }
        assert!(saw_lexer_comment, "expected LEXER_COMMENT in element list");
        assert!(saw_alg_comment, "expected ALGORITHMITEMCOMMENT in algorithm list");
    }

    #[test]
    fn expression_comment_wraps_inner_expression() {
        // A `/* … */` comment placed immediately before/after an expression
        // should round-trip as an `EXPRESSIONCOMMENT` wrapper, mirroring the
        // ANTLR3 non-bootstrap behaviour at `grammars/Modelica.g:1554`.
        let code = "\
package P\n\
algorithm\n\
  x := /* before */ 1 /* after */;\n\
end P;\n\
";
        let prog = parse(code, "t.mo", Grammar::MetaModelica).expect("parse");
        let Program::PROGRAM { classes, .. } = prog;
        let first = match &*classes { List::Cons { head, .. } => head.clone(), _ => panic!("no classes") };
        let Class::CLASS { body, .. } = first;
        let ClassDef::PARTS { classParts, .. } = &*body else { panic!("expected PARTS"); };
        let mut saw_wrapper = false;
        for cp in &**classParts {
            if let ClassPart::ALGORITHMS { contents } = &*cp {
                for ai in &**contents {
                    if let AlgorithmItem::ALGORITHMITEM { algorithm_, .. } = &*ai
                        && let Algorithm::ALG_ASSIGN { value, .. } = &**algorithm_
                        && let Exp::EXPRESSIONCOMMENT { commentsBefore, commentsAfter, .. } = value
                    {
                        assert!((&*commentsBefore.clone()).into_iter().any(|c| c.contains("before")),
                                "commentsBefore = {commentsBefore:?}");
                        assert!((&*commentsAfter.clone()).into_iter().any(|c| c.contains("after")),
                                "commentsAfter = {commentsAfter:?}");
                        saw_wrapper = true;
                    }
                }
            }
        }
        assert!(saw_wrapper, "expected an EXPRESSIONCOMMENT wrapper around the RHS");
    }

    #[test]
    fn expression_comment_backtracks_cleanly() {
        // `equality_or_noretcall_equation` does a speculative `simple_expression`
        // probe followed by `opt(Equal)`. If `expression`'s comment drain were
        // not undone on backtrack, the trailing `/* …` comment could land on
        // the wrong node depending on which alt branch wins. This test pins
        // down that the comment ends up on the EQUATIONITEM, not lost.
        let code = "\
package P\n\
equation\n\
  /* eq-comment */\n\
  x = 1;\n\
end P;\n\
";
        let prog = parse(code, "t.mo", Grammar::MetaModelica).expect("parse");
        let Program::PROGRAM { classes, .. } = prog;
        let first = match &*classes { List::Cons { head, .. } => head.clone(), _ => panic!("no classes") };
        let Class::CLASS { body, .. } = first;
        let ClassDef::PARTS { classParts, .. } = &*body else { panic!("expected PARTS"); };
        let mut saw = false;
        for cp in &**classParts {
            if let ClassPart::EQUATIONS { contents } = &*cp {
                for eq in &**contents {
                    if let EquationItem::EQUATIONITEMCOMMENT { comment } = &*eq
                        && comment.contains("eq-comment")
                    {
                        saw = true;
                    }
                }
            }
        }
        assert!(saw, "expected the /* eq-comment */ to surface as an EQUATIONITEMCOMMENT");
    }

    #[test]
    fn parse_codegen_c() {
        let code = std::fs::read_to_string("tests/data/CodegenC.mo").expect("CodegenC.mo not found");
        if let Err(e) = parse(&code, "CodegenC.mo", Grammar::MetaModelica) {
            panic!("expected CodegenC.mo to parse: {e}");
        }
    }
}
