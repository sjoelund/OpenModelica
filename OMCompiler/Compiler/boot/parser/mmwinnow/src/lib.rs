//! mmwinnow — winnow-based MetaModelica parser
//!
//! Source is first tokenised by [`lexer::lex`], then parsed by the functions
//! in this file.  AST types come from the `Absyn` module, mirroring the
//! ANTLR3 grammar from `grammars/Modelica.g`.
#![allow(non_snake_case)]

mod Absyn;
mod metamodelica;
pub mod lexer;
pub mod token_input;

pub use Absyn::*;
pub use lexer::{Token as LexToken, TokenKind, LexError};
pub use token_input::TokenInput;

use lexer::TokenKind as TK;
use token_input::{t, next_tok, peek_kind, try_tok, t_ident, t_str_token};
use winnow::stream::Stream;
use metamodelica::{List, cons, SourceInfo};

use winnow::{Parser, ModalResult, combinator::{opt, alt, cut_err}, error::{ContextError, StrContext, StrContextValue, ErrMode}};
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Grammar selector
// ---------------------------------------------------------------------------

pub struct ParserConfig {
    pub filename: String,
    pub grammar: Grammar,
}

#[derive(Clone)]
pub enum Grammar {
    Modelica2,
    Modelica3,
    MetaModelica,
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
        let remaining = err.input();
        let (line, col) = remaining
            .first()
            .or_else(|| all_tokens.last())
            .map(|t| (t.line, t.col + 1))
            .unwrap_or((0, 0));
        ParserError { line, col, inner: err.inner().clone() }
    }

    pub fn display(&self) -> String {
        let mut out = format!("error: parsing failed at {}:{}\n", self.line, self.col);
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
pub fn parse(src: &str, grammar: Grammar) -> Result<Program, Box<dyn std::error::Error>> {
    let tokens = lexer::lex(src, grammar)?;
    stored_definition
        .parse(tokens.as_slice())
        .map_err(|e| Box::new(ParserError::from_parse_error(e, &tokens)) as Box<dyn std::error::Error>)
}

// ---------------------------------------------------------------------------
// Intermediate types used during parsing
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum ClassBodyItem {
    Section { section: SectionKind, items: Rc<List<ClassBodyItem>> },
    Element(Absyn::Element),
    Annotation(Absyn::Annotation),
    Equations(List<EquationItem>),
    InitialEquations(List<EquationItem>),
    Algorithms(List<AlgorithmItem>),
    InitialAlgorithms(List<AlgorithmItem>),
    Constraints,
    External { funcName: Option<String>, annotation_opt: Option<Absyn::Annotation> },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionKind { Public, Protected }

#[derive(Debug, Clone)]
pub enum ClassSpecifier {
    Normal  { name: Ident, body: Rc<ClassDef> },
    Extends { name: Ident, body: Rc<ClassDef> },
}

impl ClassSpecifier {
    pub fn name(&self) -> Ident {
        match self {
            ClassSpecifier::Normal  { name, .. } => name.clone(),
            ClassSpecifier::Extends { name, .. } => name.clone(),
        }
    }
    pub fn body(&self) -> Rc<ClassDef> {
        match self {
            ClassSpecifier::Normal  { body, .. } => body.clone(),
            ClassSpecifier::Extends { body, .. } => body.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct ExtendsClause {
    path: Path,
    modification: Option<List<Rc<ElementArg>>>,
    annotation_opt: Option<Annotation>,
}

#[derive(Debug, Clone)]
struct ComponentClause {
    typePrefix: ElementAttributes,
    typeSpec: TypeSpec,
    components: List<Rc<ComponentItem>>,
}

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

// ---------------------------------------------------------------------------
// AST conversion helpers
// ---------------------------------------------------------------------------

fn body_items_to_classparts(items: List<ClassBodyItem>) -> List<ClassPart> {
    let mut res = List::Nil();
    for item in items.into_iter() {
        let converted = match item {
            ClassBodyItem::Section { section, items } => {
                let content = body_items_to_element_items((*items).clone());
                match section {
                    SectionKind::Public    => ClassPart::PUBLIC    { contents: content },
                    SectionKind::Protected => ClassPart::PROTECTED { contents: content },
                }
            }
            ClassBodyItem::Element(elem) => {
                let ei = ElementItem::ELEMENTITEM { element: elem };
                ClassPart::PUBLIC { contents: cons(ei, List::Nil()) }
            }
            ClassBodyItem::Annotation(ann) => ClassPart::EXTERNAL {
                externalDecl: ExternalDecl::EXTERNALDECL {
                    funcName: None, lang: None, output_: None, args: List::Nil(),
                    annotation_: Some(ann),
                },
                annotation_: None,
            },
            ClassBodyItem::Equations(items)        => ClassPart::EQUATIONS        { contents: items },
            ClassBodyItem::InitialEquations(items) => ClassPart::INITIALEQUATIONS { contents: items },
            ClassBodyItem::Algorithms(items)       => ClassPart::ALGORITHMS       { contents: items },
            ClassBodyItem::InitialAlgorithms(items)=> ClassPart::INITIALALGORITHMS{ contents: items },
            ClassBodyItem::Constraints             => ClassPart::CONSTRAINTS      { contents: List::Nil() },
            ClassBodyItem::External { funcName, annotation_opt } => ClassPart::EXTERNAL {
                externalDecl: ExternalDecl::EXTERNALDECL {
                    funcName, lang: None, output_: None, args: List::Nil(),
                    annotation_: annotation_opt,
                },
                annotation_: None,
            },
        };
        res = cons(converted, res);
    }
    res.reverse()
}

fn body_items_to_element_items(items: List<ClassBodyItem>) -> List<ElementItem> {
    match items {
        List::Nil() => List::Nil(),
        List::Cons { head, tail } => {
            let converted = match head {
                ClassBodyItem::Element(elem)   => ElementItem::ELEMENTITEM { element: elem },
                ClassBodyItem::Annotation(ann) => ElementItem::LEXER_COMMENT { comment: format!("{ann:?}") },
                ClassBodyItem::External { funcName, .. } => ElementItem::LEXER_COMMENT { comment: format!("external {funcName:?}") },
                _ => ElementItem::LEXER_COMMENT { comment: "unclassified body item".into() },
            };
            cons(converted, body_items_to_element_items((*tail).clone()))
        }
    }
}

fn mk_element(specification: ElementSpec) -> Absyn::Element {
    Absyn::Element::ELEMENT {
        finalPrefix: false, redeclareKeywords: None,
        innerOuter: InnerOuter::NOT_INNER_OUTER {}, specification,
        info: dummy_info(), constrainClass: None,
    }
}

fn to_rc_list<T: Clone>(lst: List<T>) -> List<Rc<T>> {
    let mut result: List<Rc<T>> = List::Nil();
    for item in &lst.reverse() { result = cons(Rc::new(item.clone()), result); }
    result
}

fn default_element_attrs() -> ElementAttributes {
    ElementAttributes::ATTR {
        flowPrefix: false, streamPrefix: false,
        parallelism: Parallelism::NON_PARALLEL {},
        variability: Variability::VAR {},
        direction: Direction::INPUT {},
        isField: IsField::NONFIELD {},
        arrayDim: ArrayDim::Nil(),
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
fn class_definition_list(input: &mut TokenInput) -> ModalResult<List<Class>> {
    let mut defs: List<Class> = List::Nil();
    loop {
        if input.is_empty() { break; }
        let _final = opt(t(TK::Final)).parse_next(input)?.is_some();
        if let Some(def) = opt(class_definition).parse_next(input)? {
            defs = cons(def, defs);
            t(TK::Semi).parse_next(input)?;
        } else {
            break;
        }
    }
    Ok(defs.reverse())
}

/// class_definition: ENCAPSULATED? PARTIAL? FINAL? class_type class_specifier
fn class_definition(input: &mut TokenInput) -> ModalResult<Class> {
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
        commentsBeforeClass: List::Nil(), commentsBeforeEnd: List::Nil(),
        commentsAfterEnd: List::Nil(), info: dummy_info(),
    })
}

fn class_type(input: &mut TokenInput) -> ModalResult<Restriction> {
    alt((class_type2, class_type_function)).parse_next(input)
}

fn class_type2(input: &mut TokenInput) -> ModalResult<Restriction> {
    let res = match next_tok(input)? {
        TK::Class        => Restriction::R_CLASS {},
        TK::Optimization => Restriction::R_OPTIMIZATION {},
        TK::Model        => Restriction::R_MODEL {},
        TK::Record       => Restriction::R_RECORD {},
        TK::Block        => Restriction::R_BLOCK {},
        TK::Expandable   => match next_tok(input)? {
            TK::Connector => Restriction::R_EXP_CONNECTOR {},
            _             => return Err(ErrMode::Backtrack(ContextError::default())),
        },
        TK::Connector    => Restriction::R_CONNECTOR {},
        TK::Type         => Restriction::R_TYPE {},
        TK::Package      => Restriction::R_PACKAGE {},
        TK::Uniontype    => Restriction::R_UNIONTYPE {},
        TK::Operator     => Restriction::R_OPERATOR_RECORD {},
        _                => return Err(ErrMode::Backtrack(ContextError::default())),
    };
    Ok(res)
}

fn class_type_function(input: &mut TokenInput) -> ModalResult<Restriction> {
    let purity = try_tok(input, |k| match k {
        TK::Pure   => Some(Absyn::FunctionPurity::PURE {}),
        TK::Impure => Some(Absyn::FunctionPurity::IMPURE {}),
        _          => None,
    }).unwrap_or(Absyn::FunctionPurity::NO_PURITY {});

    let functionRestriction = try_tok(input, |k| match k {
        TK::Operator  => Some(Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION {}),
        TK::Parallel  => Some(Absyn::FunctionRestriction::FR_PARALLEL_FUNCTION {}),
        TK::Parkernel => Some(Absyn::FunctionRestriction::FR_KERNEL_FUNCTION {}),
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
        let modifications = opt(class_modification).parse_next(input)?.unwrap_or(List::Nil());
        let comment   = string_comment(input)?;
        let parts     = cut_err(composition)
            .context(StrContext::Label("class-extends body"))
            .parse_next(input)?;
        let classParts = body_items_to_classparts(parts);
        cut_err(t(TK::End))
            .context(StrContext::Label("'end' closing class-extends"))
            .parse_next(input)?;
        let _end_name = t_ident(input)?;
        let ann: List<Annotation> = List::Nil();
        Ok(ClassSpecifier::Extends {
            name: name.clone(),
            body: Rc::new(ClassDef::CLASS_EXTENDS {
                baseClassName: name, modifications, comment, parts: classParts, ann,
            }),
        })
    } else {
        let name = t_ident(input)?;
        let body = class_specifier2(input)?;
        Ok(ClassSpecifier::Normal { name, body })
    }
}

fn class_specifier2(input: &mut TokenInput) -> ModalResult<Rc<ClassDef>> {
    if opt(t(TK::Subtypeof)).parse_next(input)?.is_some() {
        let typeSpec = type_specifier(input)?;
        return Ok(Rc::new(ClassDef::DERIVED {
            typeSpec, attributes: default_element_attrs(), arguments: List::Nil(), comment: None,
        }));
    }

    if opt(t(TK::Equal)).parse_next(input)?.is_some() {
        if opt(t(TK::Enumeration)).parse_next(input)?.is_some() {
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
        let arguments: List<Rc<ElementArg>> = opt(class_modification).parse_next(input)?.unwrap_or_default();
        let comment = opt(string_comment).parse_next(input)?.flatten().map(|c| Comment::COMMENT {
            annotation_: None, comment: Some(c),
        });
        return Ok(Rc::new(ClassDef::DERIVED {
            typeSpec, attributes: default_element_attrs(), arguments, comment,
        }));
    }

    let mut typeVars: List<String> = List::Nil();
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
    let classParts = body_items_to_classparts(parts);
    cut_err(t(TK::End))
        .context(StrContext::Label("'end' closing class body"))
        .parse_next(input)?;
    let _end_name = cut_err(t_ident)
        .context(StrContext::Label("class name after 'end'"))
        .parse_next(input)?;

    Ok(Rc::new(ClassDef::PARTS {
        typeVars, classAttrs: List::Nil(), classParts, ann: List::Nil(), comment,
    }))
}

fn composition(input: &mut TokenInput) -> ModalResult<List<ClassBodyItem>> {
    let el_items = element_list(input)?;
    let c2_items = composition2(input)?;
    let combined = el_items.append(&c2_items);
    if let Some(ann) = opt(annotation).parse_next(input)? {
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after annotation")).parse_next(input)?;
        let mut result = combined;
        result = cons(ClassBodyItem::Annotation(ann), result);
        Ok(result)
    } else {
        Ok(combined)
    }
}

fn composition2(input: &mut TokenInput) -> ModalResult<List<ClassBodyItem>> {
    let mut parts: List<ClassBodyItem> = List::Nil();
    loop {
        if input.is_empty() { break; }

        if let Some(ext) = opt(external_part).parse_next(input)? {
            parts = cons(ext, parts); continue;
        }
        if opt(t(TK::Public)).parse_next(input)?.is_some() {
            let items = element_list(input)?;
            parts = cons(ClassBodyItem::Section { section: SectionKind::Public, items: Rc::new(items) }, parts);
            continue;
        }
        if opt(t(TK::Protected)).parse_next(input)?.is_some() {
            let items = element_list(input)?;
            parts = cons(ClassBodyItem::Section { section: SectionKind::Protected, items: Rc::new(items) }, parts);
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

fn element_list(input: &mut TokenInput) -> ModalResult<List<ClassBodyItem>> {
    let mut items: List<ClassBodyItem> = List::Nil();
    loop {
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
        if let Some(imp) = opt(import_clause).parse_next(input)? {
            cut_err(t(TK::Semi)).context(StrContext::Label("';' after import clause")).parse_next(input)?;
            let elem = mk_element(ElementSpec::IMPORT { import_: imp, comment: None, info: dummy_info() });
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }
        if let Some(ext) = opt(extends_clause).parse_next(input)? {
            cut_err(t(TK::Semi)).context(StrContext::Label("';' after extends clause")).parse_next(input)?;
            let elem = mk_element(ElementSpec::EXTENDS {
                path: ext.path,
                elementArg: ext.modification.unwrap_or_else(List::Nil),
                annotationOpt: ext.annotation_opt,
            });
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }
        if let Some(cls) = opt(class_definition).parse_next(input)? {
            cut_err(t(TK::Semi)).context(StrContext::Label("';' after class definition")).parse_next(input)?;
            let elem = mk_element(ElementSpec::CLASSDEF { replaceable_: false, class_: Rc::new(cls) });
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }
        if let Some(cc) = opt(component_clause).parse_next(input)? {
            let elem = mk_element(ElementSpec::COMPONENTS {
                attributes: cc.typePrefix, typeSpec: cc.typeSpec, components: cc.components,
            });
            items = cons(ClassBodyItem::Element(elem), items); continue;
        }
        break;
    }
    Ok(items.reverse())
}

fn type_prefix(input: &mut TokenInput) -> ModalResult<ElementAttributes> {
    let flow   = try_tok(input, |k| matches!(k, TK::Flow).then_some(())).is_some();
    let stream = !flow && try_tok(input, |k| matches!(k, TK::Stream).then_some(())).is_some();

    let parallelism = try_tok(input, |k| match k {
        TK::Parlocal  => Some(Parallelism::PARLOCAL {}),
        TK::Parglobal => Some(Parallelism::PARGLOBAL {}),
        _             => None,
    }).unwrap_or(Parallelism::NON_PARALLEL {});

    let variability = try_tok(input, |k| match k {
        TK::Discrete  => Some(Variability::DISCRETE {}),
        TK::Parameter => Some(Variability::PARAM {}),
        TK::Constant  => Some(Variability::CONST {}),
        _             => None,
    }).unwrap_or(Variability::VAR {});

    let has_input  = try_tok(input, |k| matches!(k, TK::Input).then_some(())).is_some();
    let has_output = try_tok(input, |k| matches!(k, TK::Output).then_some(())).is_some();
    let direction  = match (has_input, has_output) {
        (true,  true)  => Direction::INPUT_OUTPUT {},
        (true,  false) => Direction::INPUT {},
        (false, true)  => Direction::OUTPUT {},
        (false, false) => Direction::BIDIR {},
    };

    let is_field = try_tok(input, |k| match k {
        TK::Ident(s) if s == "field"    => Some(IsField::FIELD {}),
        TK::Ident(s) if s == "nonfield" => Some(IsField::NONFIELD {}),
        _                                => None,
    }).unwrap_or(IsField::NONFIELD {});

    Ok(ElementAttributes::ATTR {
        flowPrefix: flow, streamPrefix: stream, parallelism, variability, direction,
        isField: is_field, arrayDim: ArrayDim::Nil(),
    })
}

fn component_clause(input: &mut TokenInput) -> ModalResult<ComponentClause> {
    let typePrefix = type_prefix(input)?;
    let typeSpec   = type_specifier(input)?;
    let components = cut_err(component_list)
        .context(StrContext::Label("component list"))
        .parse_next(input)?;
    cut_err(t(TK::Semi))
        .context(StrContext::Label("';' after component list"))
        .parse_next(input)?;
    Ok(ComponentClause { typePrefix, typeSpec, components })
}

fn component_list(input: &mut TokenInput) -> ModalResult<List<Rc<ComponentItem>>> {
    let first = component_declaration(input)?;
    let mut items = List::new(Rc::new(first));
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        items = cons(Rc::new(component_declaration(input)?), items);
    }
    Ok(items.reverse())
}

fn component_declaration(input: &mut TokenInput) -> ModalResult<ComponentItem> {
    let name = match next_tok(input)? {
        TK::Ident(n)  => n,
        TK::Operator  => "operator".to_string(),
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    };
    let arrayDim  = opt(array_subscripts).parse_next(input)?.unwrap_or_else(ArrayDim::Nil);
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
    let cm = opt(class_modification).parse_next(input)?.unwrap_or(List::Nil());
    let eq = if opt(alt((t(TK::Assign), t(TK::Equal)))).parse_next(input)?.is_some() {
        Absyn::EqMod::EQMOD {
            exp: Rc::new(cut_err(modification_expression)
                .context(StrContext::Label("modification expression"))
                .parse_next(input)?),
            info: dummy_info(),
        }
    } else {
        Absyn::EqMod::NOMOD {}
    };
    Ok(Modification::CLASSMOD { elementArgLst: cm, eqMod: eq })
}

fn modification_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    if opt(t(TK::Break)).parse_next(input)?.is_some() {
        return Ok(Absyn::Exp::BREAK {});
    }
    expression(input)
}

fn class_modification(input: &mut TokenInput) -> ModalResult<List<Rc<ElementArg>>> {
    t(TK::LParen).parse_next(input)?;
    let arguments = opt(argument_list).parse_next(input)?.unwrap_or(List::Nil());
    cut_err(t(TK::RParen))
        .context(StrContext::Label("')' closing modification list"))
        .parse_next(input)?;
    Ok(arguments)
}

fn argument_list(input: &mut TokenInput) -> ModalResult<List<Rc<ElementArg>>> {
    let mut res = List::new(Rc::new(argument(input)?));
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        res = cons(Rc::new(argument(input)?), res);
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
            *eachPrefix  = if eachPrefix_  { Each::EACH {} } else { Each::NON_EACH {} };
            *finalPrefix = finalPrefix_;
        }
        _ => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    Ok(res)
}

fn element_redeclaration(input: &mut TokenInput) -> ModalResult<ElementArg> {
    t(TK::Redeclare).parse_next(input)?;
    let _each  = opt(t(TK::Each)).parse_next(input)?.is_some();
    let _final = opt(t(TK::Final)).parse_next(input)?.is_some();
    Err(ErrMode::Backtrack(ContextError::default()))
}

fn element_modification(input: &mut TokenInput) -> ModalResult<ElementArg> {
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
        modification, comment, path, info: dummy_info(),
    })
}

fn element_replaceable(_input: &mut TokenInput) -> ModalResult<ElementArg> {
    Err(ErrMode::Backtrack(ContextError::default()))
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
    match path {
        Path::IDENT { name } => {
            if opt(t(TK::Equal)).parse_next(input)?.is_some() {
                Ok(Import::NAMED_IMPORT { name, path: name_path(input)? })
            } else {
                Ok(Import::QUAL_IMPORT { path: Path::IDENT { name } })
            }
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

fn external_part(input: &mut TokenInput) -> ModalResult<ClassBodyItem> {
    if !matches!(peek_kind(input), Some(TK::External)) {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }
    next_tok(input)?; // consume 'external'
    // Consume tokens until the terminating ';'.
    let mut parts = Vec::new();
    loop {
        match peek_kind(input) {
            None | Some(TK::Semi) => break,
            _ => { parts.push(next_tok(input)?); }
        }
    }
    t(TK::Semi).parse_next(input)?;
    Ok(ClassBodyItem::External { funcName: Some(format!("{parts:?}")), annotation_opt: None })
}

// ---------------------------------------------------------------------------
// Equation / algorithm sections
// ---------------------------------------------------------------------------

fn equation_section_items(input: &mut TokenInput) -> ModalResult<List<EquationItem>> {
    let mut items: List<EquationItem> = List::Nil();
    loop {
        if input.is_empty() { break; }
        match peek_kind(input) {
            Some(TK::Public) | Some(TK::Protected) | Some(TK::Equation) | Some(TK::Algorithm)
            | Some(TK::External) | Some(TK::End) | Some(TK::Initial) => break,
            _ => {}
        }
        items = cons(equation_item(input)?, items);
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after equation")).parse_next(input)?;
    }
    Ok(items.reverse())
}

fn algorithm_section_items(input: &mut TokenInput) -> ModalResult<List<AlgorithmItem>> {
    let mut items: List<AlgorithmItem> = List::Nil();
    loop {
        if input.is_empty() { break; }
        match peek_kind(input) {
            Some(TK::Public) | Some(TK::Protected) | Some(TK::Equation) | Some(TK::Algorithm)
            | Some(TK::Initial) | Some(TK::End) | Some(TK::External) => break,
            _ => {}
        }
        items = cons(algorithm_item(input)?, items);
        cut_err(t(TK::Semi)).context(StrContext::Label("';' after statement")).parse_next(input)?;
    }
    Ok(items.reverse())
}

/// Equations stopping at Then / Else / Elseif / Elsewhen / End.
fn equation_list(input: &mut TokenInput) -> ModalResult<List<EquationItem>> {
    let mut items: List<EquationItem> = List::Nil();
    loop {
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

fn equation_list_then(input: &mut TokenInput) -> ModalResult<List<Absyn::EquationItem>> {
    equation_list(input)
}

fn equation_item(input: &mut TokenInput) -> ModalResult<EquationItem> {
    let eq = match peek_kind(input) {
        Some(TK::If)   => if_equation_e(input)?,
        Some(TK::For)  => for_equation_e(input)?,
        Some(TK::When) => when_equation_e(input)?,
        _              => equality_or_noretcall_equation(input)?,
    };
    let comment = string_comment(input)?;
    Ok(EquationItem::EQUATIONITEM {
        equation_: Rc::new(eq),
        comment: comment.map(|c| Comment::COMMENT { annotation_: None, comment: Some(c) }),
        info: dummy_info(),
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
    let mut else_if_branches: Vec<(Absyn::Exp, List<Rc<EquationItem>>)> = Vec::new();
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
    } else { List::Nil() };
    match cut_err(next_tok)
        .context(StrContext::Label("'end' closing if-equation"))
        .parse_next(input)?
    {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "if" or end-ident
    let mut elseif_list: List<(Absyn::Exp, List<Rc<EquationItem>>)> = List::Nil();
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
    let mut else_when: Vec<(Absyn::Exp, List<Rc<EquationItem>>)> = Vec::new();
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
    let mut ew_list: List<(Absyn::Exp, List<Rc<EquationItem>>)> = List::Nil();
    for branch in else_when.into_iter().rev() { ew_list = cons(branch, ew_list); }
    Ok(Equation::EQ_WHEN_E {
        whenExp: when_cond,
        whenEquations: to_rc_list(when_body),
        elseWhenEquations: ew_list,
    })
}

/// Algorithm statements stopping at Then / Else / Elseif / Elsewhen / End.
fn algorithm_list(input: &mut TokenInput) -> ModalResult<List<AlgorithmItem>> {
    let mut items: List<AlgorithmItem> = List::Nil();
    loop {
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

fn algorithm_list_then(input: &mut TokenInput) -> ModalResult<List<Absyn::AlgorithmItem>> {
    algorithm_list(input)
}

fn algorithm_item(input: &mut TokenInput) -> ModalResult<AlgorithmItem> {
    let alg = match peek_kind(input) {
        Some(TK::If)       => if_algorithm(input)?,
        Some(TK::For)      => for_algorithm(input)?,
        Some(TK::While)    => while_algorithm(input)?,
        Some(TK::When)     => when_algorithm(input)?,
        Some(TK::Try)      => try_algorithm(input)?,
        Some(TK::Return)   => { next_tok(input)?; Algorithm::ALG_RETURN {} }
        Some(TK::Break)    => { next_tok(input)?; Algorithm::ALG_BREAK {} }
        Some(TK::Continue) => { next_tok(input)?; Algorithm::ALG_CONTINUE {} }
        _                  => assign_clause_a(input)?,
    };
    let comment = string_comment(input)?;
    Ok(AlgorithmItem::ALGORITHMITEM {
        algorithm_: Rc::new(alg),
        comment: comment.map(|c| Comment::COMMENT { annotation_: None, comment: Some(c) }),
        info: dummy_info(),
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
    let mut else_if_branches: Vec<(Absyn::Exp, List<AlgorithmItem>)> = Vec::new();
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
    } else { List::Nil() };
    match cut_err(next_tok).context(StrContext::Label("'end' closing if-algorithm")).parse_next(input)? {
        TK::End => {}
        _       => return Err(ErrMode::Cut(ContextError::default())),
    }
    next_tok(input)?; // "if" or end-ident
    let mut elseif_list: List<(Absyn::Exp, List<AlgorithmItem>)> = List::Nil();
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
    let mut else_when: Vec<(Absyn::Exp, List<AlgorithmItem>)> = Vec::new();
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
    let mut ew_list: List<(Absyn::Exp, List<AlgorithmItem>)> = List::Nil();
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

// ---------------------------------------------------------------------------
// Match expression helpers
// ---------------------------------------------------------------------------

fn match_case_body(input: &mut TokenInput) -> ModalResult<Absyn::ClassPart> {
    match peek_kind(input) {
        Some(TK::Equation) => {
            next_tok(input)?;
            let eqs = cut_err(equation_list_then)
                .context(StrContext::Label("equation list in match case"))
                .parse_next(input)?;
            Ok(Absyn::ClassPart::EQUATIONS { contents: eqs })
        }
        Some(TK::Algorithm) => {
            next_tok(input)?;
            let algs = cut_err(algorithm_list_then)
                .context(StrContext::Label("algorithm list in match case"))
                .parse_next(input)?;
            Ok(Absyn::ClassPart::ALGORITHMS { contents: algs })
        }
        _ => Ok(Absyn::ClassPart::EQUATIONS { contents: List::Nil() }),
    }
}

fn local_clause(input: &mut TokenInput) -> ModalResult<List<Rc<Absyn::ElementItem>>> {
    if !matches!(peek_kind(input), Some(TK::Local)) { return Ok(List::Nil()); }
    next_tok(input)?; // Local
    let items = element_list(input)?;
    let mut result: List<Rc<Absyn::ElementItem>> = List::Nil();
    for item in &items {
        let ei = match item {
            ClassBodyItem::Element(elem)   => Absyn::ElementItem::ELEMENTITEM { element: elem },
            ClassBodyItem::Annotation(ann) => Absyn::ElementItem::LEXER_COMMENT { comment: format!("{ann:?}") },
            _ => continue,
        };
        result = cons(Rc::new(ei), result);
    }
    Ok(result.reverse())
}

fn match_onecase(input: &mut TokenInput) -> ModalResult<Absyn::Case> {
    match next_tok(input)? {
        TK::Case => {}
        _        => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    let pattern = expression(input)?;
    let patternGuard = match peek_kind(input) {
        Some(TK::If) | Some(TK::Guard) => {
            next_tok(input)?;
            Some(Rc::new(expression(input)?))
        }
        _ => None,
    };
    let comment    = string_comment(input)?;
    let localDecls = local_clause(input)?;
    let classPart  = match_case_body(input)?;
    match next_tok(input)? {
        TK::Then => {}
        _        => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    let result = expression(input)?;
    t(TK::Semi).parse_next(input)?;
    Ok(Absyn::Case::CASE {
        pattern: Rc::new(pattern), patternGuard, patternInfo: dummy_info(),
        localDecls, classPart, result: Rc::new(result), resultInfo: dummy_info(),
        comment, info: dummy_info(),
    })
}

fn match_cases(input: &mut TokenInput) -> ModalResult<List<Absyn::Case>> {
    let mut cases: List<Absyn::Case> = List::Nil();
    loop {
        match peek_kind(input) {
            Some(TK::Case) => { cases = cons(match_onecase(input)?, cases); }
            Some(TK::Else) => {
                next_tok(input)?;
                let comment    = string_comment(input)?;
                let localDecls = local_clause(input)?;
                let classPart  = match_case_body(input)?;
                match next_tok(input)? {
                    TK::Then => {}
                    _        => return Err(ErrMode::Backtrack(ContextError::default())),
                }
                let result = expression(input)?;
                t(TK::Semi).parse_next(input)?;
                cases = cons(Absyn::Case::ELSE {
                    localDecls, classPart, result: Rc::new(result),
                    resultInfo: dummy_info(), comment, info: dummy_info(),
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
    let comment    = string_comment(input)?;
    let localDecls = local_clause(input)?;
    let cases      = match_cases(input)?;
    match next_tok(input)? {
        TK::End => {}
        _       => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    match next_tok(input)? {
        TK::Match | TK::Matchcontinue => {}
        _                              => return Err(ErrMode::Backtrack(ContextError::default())),
    }
    Ok(Absyn::Exp::MATCHEXP { matchTy, inputExp: Rc::new(inputExp), localDecls, cases, comment })
}

// ---------------------------------------------------------------------------
// Name / path / component reference parsers
// ---------------------------------------------------------------------------

fn name_path(input: &mut TokenInput) -> ModalResult<Path> {
    let fq  = opt(t(TK::Dot)).parse_next(input)?.is_some();
    let res = name_path2(input)?;
    if fq { Ok(Path::FULLYQUALIFIED { path: Rc::new(res) }) } else { Ok(res) }
}

fn name_path2(input: &mut TokenInput) -> ModalResult<Path> {
    let mut parts = Vec::new();
    let mut last_id = t_ident(input)?;
    loop {
        // Only treat Dot as separator if the next token after it is an Ident.
        if input.len() >= 2
            && input[0].kind == TK::Dot
            && matches!(&input[1].kind, TK::Ident(_))
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
        res = Path::QUALIFIED { name: id.clone(), path: Rc::new(res) };
    }
    Ok(res)
}

fn component_reference(input: &mut TokenInput) -> ModalResult<Absyn::ComponentRef> {
    let fq = opt(t(TK::Dot)).parse_next(input)?.is_some();
    let cr = component_reference2(input)?;
    if fq { Ok(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: Rc::new(cr) }) }
    else  { Ok(cr) }
}

fn component_reference2(input: &mut TokenInput) -> ModalResult<Absyn::ComponentRef> {
    let name     = t_ident(input)?;
    let raw_subs = opt(array_subscripts).parse_next(input)?.unwrap_or(List::Nil());
    let mut subscripts: List<Rc<Absyn::Subscript>> = List::Nil();
    for s in &raw_subs.reverse() { subscripts = cons(Rc::new(s), subscripts); }
    if input.len() >= 2
        && input[0].kind == TK::Dot
        && matches!(&input[1].kind, TK::Ident(_))
    {
        *input = &input[1..]; // consume Dot
        let rest = component_reference2(input)?;
        Ok(Absyn::ComponentRef::CREF_QUAL { name, subscripts, componentRef: Rc::new(rest) })
    } else {
        Ok(Absyn::ComponentRef::CREF_IDENT { name, subscripts })
    }
}

// ---------------------------------------------------------------------------
// Expression parsers
// ---------------------------------------------------------------------------

fn expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match peek_kind(input) {
        Some(TK::If)                             => return if_expression(input),
        Some(TK::Match) | Some(TK::Matchcontinue) => return match_expression(input),
        Some(TK::Function)                       => return part_eval_function_expression(input),
        Some(TK::Code)                           => return code_expression(input),
        _ => {}
    }
    simple_expression(input)
}

fn if_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match next_tok(input)? { TK::If => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
    let cond    = expression(input)?;
    match next_tok(input)? { TK::Then => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
    let true_br = expression(input)?;
    let mut elseif: List<(Rc<Absyn::Exp>, Rc<Absyn::Exp>)> = List::Nil();
    loop {
        if !matches!(peek_kind(input), Some(TK::Elseif)) { break; }
        next_tok(input)?;
        let ec = expression(input)?;
        match next_tok(input)? { TK::Then => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
        let et = expression(input)?;
        elseif = cons((Rc::new(ec), Rc::new(et)), elseif);
    }
    match next_tok(input)? { TK::Else => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
    let false_br = expression(input)?;
    Ok(Absyn::Exp::IFEXP {
        ifExp: Rc::new(cond), trueBranch: Rc::new(true_br), elseBranch: Rc::new(false_br),
        elseIfBranch: elseif.reverse(),
    })
}

fn code_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match next_tok(input)? { TK::Code => {} _ => return Err(ErrMode::Backtrack(ContextError::default())) }
    t(TK::LParen).parse_next(input)?;
    let e = expression(input)?;
    t(TK::RParen).parse_next(input)?;
    Ok(Absyn::Exp::CODE { code: Absyn::CodeNode::C_EXPRESSION { exp: Rc::new(e) } })
}

fn part_eval_function_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    t(TK::Function).parse_next(input)?;
    let cr      = component_reference(input)?;
    t(TK::LParen).parse_next(input)?;
    let argNames = opt(named_arguments).parse_next(input)?.unwrap_or(List::Nil());
    t(TK::RParen).parse_next(input)?;
    Ok(Absyn::Exp::PARTEVALFUNCTION {
        function_: Rc::new(cr),
        functionArgs: Absyn::FunctionArgs::FUNCTIONARGS { args: List::Nil(), argNames },
    })
}

/// simple_expression: (ident AS simple_expr) | (simple_expr (:: simple_expression)?)
fn simple_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    // Check for ident AS pattern (MetaModelica).
    {
        let saved = *input;
        let as_result: Option<String> = (|| {
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
                return Ok(Absyn::Exp::AS { id, exp: Rc::new(e) });
            }
            None => { *input = saved; }
        }
    }

    let e1 = simple_expr(input)?;
    if matches!(peek_kind(input), Some(TK::ColonColon)) {
        next_tok(input)?;
        let e2 = simple_expression(input)?;
        Ok(Absyn::Exp::CONS { head: Rc::new(e1), rest: Rc::new(e2) })
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
        Ok(Absyn::Exp::RANGE { start: Rc::new(e1), step: Some(Rc::new(e2)), stop: Rc::new(e3) })
    } else {
        Ok(Absyn::Exp::RANGE { start: Rc::new(e1), step: None, stop: Rc::new(e2) })
    }
}

fn logical_expression(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let mut e = logical_term(input)?;
    loop {
        if !matches!(peek_kind(input), Some(TK::Or)) { break; }
        next_tok(input)?;
        let e2 = logical_term(input)?;
        e = Absyn::Exp::LBINARY { exp1: Rc::new(e), op: Absyn::Operator::OR {}, exp2: Rc::new(e2) };
    }
    Ok(e)
}

fn logical_term(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let mut e = logical_factor(input)?;
    loop {
        if !matches!(peek_kind(input), Some(TK::And)) { break; }
        next_tok(input)?;
        let e2 = logical_factor(input)?;
        e = Absyn::Exp::LBINARY { exp1: Rc::new(e), op: Absyn::Operator::AND {}, exp2: Rc::new(e2) };
    }
    Ok(e)
}

fn logical_factor(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    let has_not = matches!(peek_kind(input), Some(TK::Not));
    if has_not { next_tok(input)?; }
    let e = relation(input)?;
    if has_not { Ok(Absyn::Exp::LUNARY { op: Absyn::Operator::NOT {}, exp: Rc::new(e) }) }
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
        Some(op) => Ok(Absyn::Exp::RELATION { exp1: Rc::new(e1), op, exp2: Rc::new(arithmetic_expression(input)?) }),
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
            Some(op) => { let e2 = term(input)?; e = Absyn::Exp::BINARY { exp1: Rc::new(e), op, exp2: Rc::new(e2) }; }
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
        Some(op) => Ok(Absyn::Exp::UNARY { op, exp: Rc::new(t_expr) }),
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
            Some(op) => { let e2 = factor(input)?; e = Absyn::Exp::BINARY { exp1: Rc::new(e), op, exp2: Rc::new(e2) }; }
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
        Some(op) => Ok(Absyn::Exp::BINARY { exp1: Rc::new(e1), op, exp2: Rc::new(primary(input)?) }),
        None     => Ok(e1),
    }
}

fn primary(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match peek_kind(input) {
        Some(TK::End)   => { next_tok(input)?; return Ok(Absyn::Exp::END {}); }
        Some(TK::True)  => { next_tok(input)?; return Ok(Absyn::Exp::BOOL { value: true  }); }
        Some(TK::False) => { next_tok(input)?; return Ok(Absyn::Exp::BOOL { value: false }); }
        Some(TK::Str(_))=> { return Ok(Absyn::Exp::STRING { value: string_comment(input)?.unwrap_or_default() }); }
        Some(TK::Int(_)) | Some(TK::Real(_)) => { return number_literal(input); }
        Some(TK::LParen) => {
            next_tok(input)?;
            let (exprs, is_tuple) = output_expression_list(input)?;
            let raw_subs = opt(array_subscripts).parse_next(input)?;
            if let Some(subs) = raw_subs {
                let mut rc_subs: List<Rc<Subscript>> = List::Nil();
                for s in &subs.reverse() { rc_subs = cons(Rc::new(s), rc_subs); }
                return Ok(Absyn::Exp::SUBSCRIPTED_EXP {
                    exp: Rc::new(to_tuple_or_exp(exprs, is_tuple)), subscripts: rc_subs,
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
                    let cr = Absyn::ComponentRef::CREF_IDENT { name: "$array".into(), subscripts: List::Nil() };
                    Ok(Absyn::Exp::CALL {
                        function_: Rc::new(cr),
                        functionArgs: Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators },
                        typeVars: List::Nil(),
                    })
                }
                Absyn::FunctionArgs::FUNCTIONARGS { args, argNames: List::Nil() } =>
                    Ok(Absyn::Exp::ARRAY { arrayExp: args }),
                _ => Err(ErrMode::Backtrack(ContextError::default())),
            };
        }
        Some(TK::Der) => {
            next_tok(input)?;
            let fa = function_call(input)?;
            let cr = Absyn::ComponentRef::CREF_IDENT { name: "der".into(), subscripts: List::Nil() };
            return Ok(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: List::Nil() });
        }
        Some(TK::Pure) => {
            next_tok(input)?;
            let fa = function_call(input)?;
            let cr = Absyn::ComponentRef::CREF_IDENT { name: "pure".into(), subscripts: List::Nil() };
            return Ok(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: List::Nil() });
        }
        _ => {}
    }
    component_reference__function_call(input)
}

fn to_tuple_or_exp(exprs: List<Rc<Absyn::Exp>>, is_tuple: bool) -> Absyn::Exp {
    if is_tuple {
        Absyn::Exp::TUPLE { expressions: exprs }
    } else {
        match exprs {
            List::Cons { ref head, .. } => (**head).clone(),
            List::Nil()                 => Absyn::Exp::TUPLE { expressions: List::Nil() },
        }
    }
}

fn number_literal(input: &mut TokenInput) -> ModalResult<Absyn::Exp> {
    match next_tok(input)? {
        TK::Int(n)  => Ok(Absyn::Exp::INTEGER { value: n }),
        TK::Real(f) => Ok(Absyn::Exp::REAL    { value: f.to_string() }),
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
            let cr = Absyn::ComponentRef::CREF_IDENT { name: "initial".into(), subscripts: List::Nil() };
            return Ok(Absyn::Exp::CALL {
                function_: Rc::new(cr),
                functionArgs: Absyn::FunctionArgs::FUNCTIONARGS { args: List::Nil(), argNames: List::Nil() },
                typeVars: List::Nil(),
            });
        }
        // Not initial() — treat 'initial' as an identifier.
        // Fall through with synthetic cref.
        return Ok(Absyn::Exp::CREF {
            componentRef: Rc::new(Absyn::ComponentRef::CREF_IDENT { name: "initial".into(), subscripts: List::Nil() }),
        });
    }

    let cr = component_reference(input)?;

    // Polymorphic call: cr <T1,T2,...> ( args )
    if matches!(peek_kind(input), Some(TK::Less)) {
        let saved = *input;
        if let Ok(type_vars) = (|| -> ModalResult<List<Path>> {
            next_tok(input)?; // '<'
            let mut vars: List<Path> = List::Nil();
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
                return Ok(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: type_vars });
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
                exp:   Rc::new(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: List::Nil() }),
                index: Rc::new(field),
            });
        }
        return Ok(Absyn::Exp::CALL { function_: Rc::new(cr), functionArgs: fa, typeVars: List::Nil() });
    }

    Ok(Absyn::Exp::CREF { componentRef: Rc::new(cr) })
}

fn function_call(input: &mut TokenInput) -> ModalResult<Absyn::FunctionArgs> {
    t(TK::LParen).parse_next(input)?;
    let fa = function_arguments(input)?;
    t(TK::RParen).parse_next(input)?;
    Ok(fa)
}

fn function_arguments(input: &mut TokenInput) -> ModalResult<Absyn::FunctionArgs> {
    let fa = for_or_expression_list(input)?;
    match fa {
        Absyn::FunctionArgs::FOR_ITER_FARG { .. } => Ok(fa),
        Absyn::FunctionArgs::FUNCTIONARGS { args, argNames: _ } => {
            let argNames = opt(named_arguments).parse_next(input)?.unwrap_or(List::Nil());
            Ok(Absyn::FunctionArgs::FUNCTIONARGS { args, argNames })
        }
    }
}

fn for_or_expression_list(input: &mut TokenInput) -> ModalResult<Absyn::FunctionArgs> {
    // Empty.
    if matches!(peek_kind(input), Some(TK::RParen) | Some(TK::RBrace) | None) {
        return Ok(Absyn::FunctionArgs::FUNCTIONARGS { args: List::Nil(), argNames: List::Nil() });
    }

    let mut checkpoint = input.checkpoint();
    let mut exp = expression(input)?;

    // For-iterator.
    if matches!(peek_kind(input), Some(TK::For) | Some(TK::Threaded)) {
        let threaded = if matches!(peek_kind(input), Some(TK::Threaded)) {
            next_tok(input)?; true
        } else { false };
        t(TK::For).parse_next(input)?;
        let iterators = for_indices(input)?;
        return Ok(Absyn::FunctionArgs::FOR_ITER_FARG {
            exp: Rc::new(exp),
            iterType: if threaded { Absyn::ReductionIterType::THREAD {} } else { Absyn::ReductionIterType::COMBINE {} },
            iterators,
        });
    }

    // Expression list, possibly ending with named arguments.
    let mut args: List<Rc<Absyn::Exp>> = List::Nil();
    let mut arg_names: List<Rc<Absyn::NamedArg>> = List::Nil();
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
        args = cons(Rc::new(exp), args);
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        checkpoint = input.checkpoint();
        exp = expression(input)?;
    }
    Ok(Absyn::FunctionArgs::FUNCTIONARGS { args: args.reverse(), argNames: arg_names.reverse() })
}

fn named_argument(input: &mut TokenInput) -> ModalResult<Absyn::NamedArg> {
    let argName  = t_ident(input)?;
    t(TK::Equal).parse_next(input)?;
    let argValue = Rc::new(expression(input)?);
    Ok(Absyn::NamedArg::NAMEDARG { argName, argValue })
}

fn named_arguments(input: &mut TokenInput) -> ModalResult<List<Rc<Absyn::NamedArg>>> {
    let first = named_argument(input)?;
    let mut args: List<Rc<Absyn::NamedArg>> = cons(Rc::new(first), List::Nil());
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        match named_argument(input) {
            Ok(arg) => args = cons(Rc::new(arg), args),
            Err(_)  => break,
        }
    }
    Ok(args.reverse())
}

fn for_indices(input: &mut TokenInput) -> ModalResult<Absyn::ForIterators> {
    let first = for_index(input)?;
    let mut result: List<Absyn::ForIterator> = cons(first, List::Nil());
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
            Some(Rc::new(expression(input)?))
        }
        _ => None,
    };
    let range = if matches!(peek_kind(input), Some(TK::In)) {
        next_tok(input)?;
        Some(Rc::new(expression(input)?))
    } else { None };
    Ok(Absyn::ForIterator::ITERATOR { name, guardExp, range })
}

fn expression_list(input: &mut TokenInput) -> ModalResult<List<Rc<Absyn::Exp>>> {
    let e = expression(input)?;
    let mut result: List<Rc<Absyn::Exp>> = cons(Rc::new(e), List::Nil());
    loop {
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
        match expression(input) {
            Ok(e)  => result = cons(Rc::new(e), result),
            Err(_) => break,
        }
    }
    Ok(result.reverse())
}

/// Consumes up to and including ')'; returns (expressions, isTuple).
fn output_expression_list(input: &mut TokenInput) -> ModalResult<(List<Rc<Absyn::Exp>>, bool)> {
    // ()
    if matches!(peek_kind(input), Some(TK::RParen)) {
        next_tok(input)?;
        return Ok((List::Nil(), true));
    }
    // Leading comma: (, b) → WILD, b
    if matches!(peek_kind(input), Some(TK::Comma)) {
        next_tok(input)?;
        let (rest, _) = output_expression_list(input)?;
        let wild_exp = Rc::new(Absyn::Exp::CREF { componentRef: Rc::new(Absyn::ComponentRef::WILD {}) });
        return Ok((cons(wild_exp, rest), true));
    }
    // Named-argument detection: ident followed by '=' (not '==').
    let is_named = input.len() >= 2
        && matches!(&input[0].kind, TK::Ident(_))
        && input[1].kind == TK::Equal
        && input.get(2).map(|t| t.kind != TK::Equal).unwrap_or(true);
    if is_named {
        // Consume all tokens until the matching ')'.
        let mut content = Vec::new();
        let mut depth = 1u32;
        loop {
            match peek_kind(input) {
                None => break,
                Some(TK::LParen) => { depth += 1; content.push(next_tok(input)?); }
                Some(TK::RParen) => {
                    depth -= 1;
                    if depth == 0 { next_tok(input)?; break; }
                    content.push(next_tok(input)?);
                }
                _ => { content.push(next_tok(input)?); }
            }
        }
        let expr = Absyn::Exp::STRING { value: format!("{content:?}") };
        return Ok((cons(Rc::new(expr), List::Nil()), true));
    }

    let e1 = expression(input)?;
    if matches!(peek_kind(input), Some(TK::Comma)) {
        next_tok(input)?;
        let (rest, _) = output_expression_list(input)?;
        let mut result = rest;
        if result.is_empty() {
            let wild = Rc::new(Absyn::Exp::CREF { componentRef: Rc::new(Absyn::ComponentRef::WILD {}) });
            result = cons(wild, result);
        }
        return Ok((cons(Rc::new(e1), result), true));
    }
    t(TK::RParen).parse_next(input)?;
    Ok((cons(Rc::new(e1), List::Nil()), false))
}

fn matrix_expression_list(input: &mut TokenInput) -> ModalResult<List<List<Rc<Absyn::Exp>>>> {
    let row = expression_list(input)?;
    let mut rows: List<List<Rc<Absyn::Exp>>> = cons(row, List::Nil());
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

fn string_comment(input: &mut TokenInput) -> ModalResult<Option<String>> {
    let mut res = match opt(t_str_token).parse_next(input)? {
        Some(s) => s,
        None    => return Ok(None),
    };
    while opt(t(TK::Plus)).parse_next(input)?.is_some() {
        res.push_str(&cut_err(t_str_token).parse_next(input)?);
    }
    Ok(Some(res))
}

fn type_specifier(input: &mut TokenInput) -> ModalResult<TypeSpec> {
    let path = name_path(input)?;
    let mut ts: List<Rc<TypeSpec>> = List::Nil();
    if opt(t(TK::Less)).parse_next(input)?.is_some() {
        loop {
            if matches!(peek_kind(input), Some(TK::Greater)) || input.is_empty() { break; }
            let inner_ts = type_specifier(input)?;
            ts = cons(Rc::new(inner_ts), ts);
            if opt(t(TK::Comma)).parse_next(input)?.is_some() { continue; }
            break;
        }
        ts = ts.reverse();
        t(TK::Greater).parse_next(input)?;
    }
    let arrayDim = opt(array_subscripts).parse_next(input)?;
    ts = ts.reverse();
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
    Ok(Subscript::SUBSCRIPT { subscript: Rc::new(expression(input)?) })
}

fn array_subscripts(input: &mut TokenInput) -> ModalResult<ArrayDim> {
    t(TK::LBracket).parse_next(input)?;
    let mut subs: List<Subscript> = List::Nil();
    loop {
        if matches!(peek_kind(input), Some(TK::RBracket)) || input.is_empty() { break; }
        subs = cons(subscript(input)?, subs);
        if opt(t(TK::Comma)).parse_next(input)?.is_none() { break; }
    }
    t(TK::RBracket).parse_next(input)?;
    Ok(subs.reverse())
}

fn enum_list(input: &mut TokenInput) -> ModalResult<List<EnumLiteral>> {
    let mut literals: List<EnumLiteral> = List::Nil();
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
    let name = t_ident(input)?;
    if opt(t(TK::Equal)).parse_next(input)?.is_some() {
        expression(input)?; // skip value
    }
    Ok(EnumLiteral::ENUMLITERAL { literal: name, comment: None })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ok(src: &str) -> Program {
        parse(src, Grammar::MetaModelica).expect("parse should succeed")
    }

    #[test]
    fn empty_array() {
        let prog = parse_ok("package P equation; end P;");
        // Smoke-test: we got a program back.
        assert!(matches!(prog, Program::PROGRAM { .. }));
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
        match parse(code, Grammar::MetaModelica).unwrap() {
            Program::PROGRAM { classes, .. } => {
                assert!(!classes.is_empty());
                if let List::Cons { head: class, .. } = classes {
                    let Class::CLASS { name, .. } = &class;
                    assert_eq!(name, "SimpleSystem");
                }
            }
        }
    }

    #[test]
    fn parse_first_token() {
        let code = "package SimpleSystem \"Returns the index...\"\nend SimpleSystem;";
        parse(code, Grammar::MetaModelica).expect("expected parse success");
    }

    #[test]
    fn parse_absyn() {
        let code = std::fs::read_to_string("tests/data/Absyn.mo").expect("Absyn.mo not found");
        if let Err(e) = parse(&code, Grammar::MetaModelica) {
            panic!("expected Absyn.mo to parse: {e}");
        }
    }

    #[test]
    fn parse_codegen_c() {
        let code = std::fs::read_to_string("tests/data/CodegenC.mo").expect("CodegenC.mo not found");
        if let Err(e) = parse(&code, Grammar::MetaModelica) {
            panic!("expected CodegenC.mo to parse: {e}");
        }
    }
}
