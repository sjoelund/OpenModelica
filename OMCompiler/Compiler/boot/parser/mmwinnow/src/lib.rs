//! mmwinnow — winnow-based MetaModelica parser
//!
//! Lexer combinators are embedded in the parser — no separate tokenizer.
//!
//! This module provides enough of the grammar to parse the example
//! `package SimpleSystem ... end SimpleSystem;` from the original `parser/src/main.rs`.

use winnow::{Parser, ModalResult, error::{ContextError, ErrMode}};
use winnow::token::*;
use winnow::ascii;

// ---------------------------------------------------------------------------
// Token types
// ---------------------------------------------------------------------------

/// MetaModelica token types.
///
/// Mirrors the ANTLR4 lexer rules from `grammars/metamodelica.g4`.
/// Only the tokens needed for the current parsing scope are listed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    // Keywords needed for stored_definition → class_definition
    Package,
    Class,
    Record,
    Type,
    Function,
    Encapsulated,
    Partial,
    Final,
    Extends,
    End,
    // Keywords for identifiers (REAL, INTEGER, BOOLEAN, etc.)
    Real,
    Integer,
    // Operator keywords
    Equal,      // =
    Assign,     // :=
    EqEq,       // ==
    Less,       // <
    Leq,        // <=
    Greater,    // >
    Geq,        // >=
    Not,        // !
    NotEq,      // <>
    And,
    Or,
    NotKW,      // not
    // Delimiters
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    LBrace,     // {
    RBrace,     // }
    Dot,        // .
    DotDot,     // ..
    Colon,      // :
    Semi,       // ;
    Comma,      // ,
    Star,       // *
    Plus,       // +
    Minus,      // -
    Slash,      // /
    Power,      // ^
    // Literals / identifiers
    Ident(&'a str),
    StringLit(&'a str),
    IntLit(&'a str),
    RealLit(&'a str),
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Package => f.write_str("PACKAGE"),
            Token::Class => f.write_str("CLASS"),
            Token::Record => f.write_str("RECORD"),
            Token::Type => f.write_str("TYPE"),
            Token::Function => f.write_str("FUNCTION"),
            Token::Encapsulated => f.write_str("ENCAPSULATED"),
            Token::Partial => f.write_str("PARTIAL"),
            Token::Final => f.write_str("FINAL"),
            Token::Extends => f.write_str("EXTENDS"),
            Token::End => f.write_str("END"),
            Token::Real => f.write_str("REAL"),
            Token::Integer => f.write_str("INTEGER"),
            Token::Equal => f.write_str("="),
            Token::Assign => f.write_str(":="),
            Token::EqEq => f.write_str("=="),
            Token::Less => f.write_str("<"),
            Token::Leq => f.write_str("<="),
            Token::Greater => f.write_str(">"),
            Token::Geq => f.write_str(">="),
            Token::Not => f.write_str("!"),
            Token::NotEq => f.write_str("<>"),
            Token::And => f.write_str("AND"),
            Token::Or => f.write_str("OR"),
            Token::NotKW => f.write_str("NOT"),
            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::LBracket => f.write_str("["),
            Token::RBracket => f.write_str("]"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),
            Token::Dot => f.write_str("."),
            Token::DotDot => f.write_str(".."),
            Token::Colon => f.write_str(":"),
            Token::Semi => f.write_str(";"),
            Token::Comma => f.write_str(","),
            Token::Star => f.write_str("*"),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Slash => f.write_str("/"),
            Token::Power => f.write_str("^"),
            Token::Ident(s) => write!(f, "IDENT(\"{s}\")"),
            Token::StringLit(s) => write!(f, "STRING(\"{s}\")"),
            Token::IntLit(s) => write!(f, "INT(\"{s}\")"),
            Token::RealLit(s) => write!(f, "REAL_LIT(\"{s}\")"),
        }
    }
}

// ---------------------------------------------------------------------------
// AST types
// ---------------------------------------------------------------------------

/// A stored definition: top-level program unit.
#[derive(Debug, Clone)]
pub struct StoredDefinition<'a> {
    /// Parsed class definitions.
    pub classes: Vec<ClassDef<'a>>,
}

/// A single class definition (package, class, function, ...).
#[derive(Debug, Clone)]
pub struct ClassDef<'a> {
    /// The class type keyword that was matched.
    pub kind: ClassKind,
    /// The class name (identifier after class_type).
    pub name: &'a str,
    /// The class body (elements inside the braces).
    pub body: Vec<ClassElement<'a>>,
    /// The identifier after `END` (for consistency with the grammar).
    pub end_name: Option<&'a str>,
}

/// Which class keyword was used.
#[derive(Debug, Clone)]
pub enum ClassKind {
    Package,
    Class,
    Record,
    Type,
    Function,
    Other(&'static str),
}

/// An element inside a class body.
#[derive(Debug, Clone)]
pub enum ClassElement<'a> {
    /// A component declaration: e.g. `Real x(start = 0)`
    Component {
        typ: ComponentType<'a>,
        name: &'a str,
        modification: Option<Modification<'a>>,
    },
}

/// The type part of a component declaration.
#[derive(Debug, Clone)]
pub enum ComponentType<'a> {
    /// Built-in type keyword (REAL, INTEGER, ...).
    Builtin(&'static str),
    /// User-defined type (identifier).
    Named(&'a str),
}

/// A modification: `(name = value, ...)` or `name = value`.
#[derive(Debug, Clone)]
pub struct Modification<'a> {
    pub arguments: Vec<NamedArg<'a>>,
}

/// A single named argument inside a modification.
#[derive(Debug, Clone)]
pub struct NamedArg<'a> {
    pub name: &'a str,
    pub value: ModificationValue<'a>,
}

#[derive(Debug, Clone)]
pub enum ModificationValue<'a> {
    Simple,          // just the name, no value (e.g. `start`)
    Equals(&'a str),  // name = <expression>
}

// ---------------------------------------------------------------------------
// Lexer combinators
// ---------------------------------------------------------------------------

/// Skip whitespace (spaces, tabs, newlines, CR).
fn ws<'a>(input: &mut &'a str) -> ModalResult<()> {
    take_while(0.., |c: char| c.is_whitespace()).parse_next(input)?;
    Ok(())
}

/// Skip all trivia: whitespace, line comments, block comments.
fn trivia<'a>(input: &mut &'a str) -> ModalResult<()> {
    loop {
        ws(input)?;
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

/// Parse a keyword or identifier.
///
/// Matches the longest possible alphanumeric+underscore sequence and checks
/// if it matches one of the known keywords.
fn keyword_or_ident<'a>(input: &mut &'a str) -> ModalResult<Token<'a>> {
    let word: &str =
        take_while(0.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
    Ok(token_from_word(word))
}

/// Convert a word string to the appropriate Token.
fn token_from_word(word: &str) -> Token<'_> {
    match word.to_lowercase().as_str() {
        "package" => Token::Package,
        "class" => Token::Class,
        "record" => Token::Record,
        "type" => Token::Type,
        "function" => Token::Function,
        "encapsulated" => Token::Encapsulated,
        "partial" => Token::Partial,
        "final" => Token::Final,
        "extends" => Token::Extends,
        "end" => Token::End,
        "real" => Token::Real,
        "integer" => Token::Integer,
        "=" => Token::Equal,
        ":=" => Token::Assign,
        "==" => Token::EqEq,
        "<" => Token::Less,
        "<=" => Token::Leq,
        ">" => Token::Greater,
        ">=" => Token::Geq,
        "!" => Token::Not,
        "<>" => Token::NotEq,
        "and" => Token::And,
        "or" => Token::Or,
        "not" => Token::NotKW,
        _ => Token::Ident(word),
    }
}

// ---------------------------------------------------------------------------
// Parser rules
// ---------------------------------------------------------------------------

/// stored_definition
///   : BOM? (within_clause SEMICOLON)? class_definition_list EOF
///
/// Parsed as:
///   BOM?
///   class_definition SEMICOLON
///   EOF
pub fn stored_definition<'a>(input: &mut &'a str) -> ModalResult<StoredDefinition<'a>> {
    // BOM? (U+FEFF)
    let _: ModalResult<&str> = "\u{feff}".parse_next(input);

    // (within_clause SEMICOLON)?  — skip for now
    // within_clause: WITHIN name_path?
    if input.starts_with("within") || input.starts_with("WITHIN") {
        let _: &str = take_while(0.., |c: char| c != ';').parse_next(input)?;
        ";".parse_next(input)?;
    }

    // class_definition_list → class_definition SEMICOLON (repeat)
    let classes = class_definition_list(input)?;

    // EOF
    if !input.is_empty() {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }

    Ok(StoredDefinition { classes })
}

/// class_definition_list
///   : (FINAL? class_definition SEMICOLON)*
fn class_definition_list<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassDef<'a>>> {
    let mut defs = Vec::new();
    loop {
        // Skip any trivia before a potential definition
        trivia(input)?;
        if input.is_empty() {
            break;
        }
        // FINAL? (optional, skip for now)
        if input.starts_with("final") || input.starts_with("FINAL") {
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
        }
        // Parse one class_definition
        let def = class_definition(input)?;
        // SEMICOLON
        trivia(input)?;
        ";".parse_next(input)?;
        defs.push(def);

        // Check if there's another definition (or EOF)
        trivia(input)?;
        if input.is_empty() {
            break;
        }
    }
    Ok(defs)
}

/// class_definition
///   : ENCAPSULATED? PARTIAL? class_type class_specifier
fn class_definition<'a>(input: &mut &'a str) -> ModalResult<ClassDef<'a>> {
    // ENCAPSULATED?
    if input.starts_with("encapsulated") || input.starts_with("ENCAPSULATED") {
        take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
    }
    // PARTIAL?
    if input.starts_with("partial") || input.starts_with("PARTIAL") {
        take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
    }

    // class_type
    let kind = class_type(input)?;

    // class_specifier
    let (name, body, end_name) = class_specifier(input)?;

    Ok(ClassDef {
        kind,
        name,
        body,
        end_name,
    })
}

/// class_type
///   : CLASS | RECORD | TYPE | T_PACKAGE | FUNCTION | ...
fn class_type<'a>(input: &mut &'a str) -> ModalResult<ClassKind> {
    let tok = keyword_or_ident(input)?;
    let kind = match &tok {
        Token::Package => ClassKind::Package,
        Token::Class => ClassKind::Class,
        Token::Record => ClassKind::Record,
        Token::Type => ClassKind::Type,
        Token::Function => ClassKind::Function,
        _ => ClassKind::Other(tok_as_str(&tok)),
    };
    Ok(kind)
}

/// class_specifier
///   : identifier class_specifier2
/// class_specifier2
///   : (LESS ident_list GREATER)? string_comment composition END IDENT
fn class_specifier<'a>(input: &mut &'a str) -> ModalResult<(&'a str, Vec<ClassElement<'a>>, Option<&'a str>)> {
    // identifier (class name)
    trivia(input)?;
    let name_tok = keyword_or_ident(input)?;
    let name = tok_as_ident(name_tok)?;

    // (LESS ident_list GREATER)?  — skip for now
    if input.starts_with("<") {
        "<".parse_next(input)?;
        take_while(0.., |c: char| c != '>').parse_next(input)?;
        ">".parse_next(input)?;
    }

    // string_comment → one or more STRING tokens (skip them)
    string_comment(input)?;

    // composition (class body)
    let body = composition(input)?;

    // END
    let end_tok = keyword_or_ident(input)?;
    match end_tok {
        Token::End => {}
        _ => {
            return Err(ErrMode::Backtrack(ContextError::default()));
        }
    }

    // IDENT (end name — optional in our simplified version)
    let end_name = {
        trivia(input)?;
        if !input.is_empty() && input.starts_with(|c: char| c.is_alphabetic() || c == '_' || c == '$') {
            let tok = keyword_or_ident(input)?;
            Some(tok_as_ident(tok)?)
        } else {
            None
        }
    };

    Ok((name, body, end_name))
}

/// Parse and skip a string_comment (one or more quoted strings).
fn string_comment<'a>(input: &mut &'a str) -> ModalResult<()> {
    loop {
        trivia(input)?;
        if !input.starts_with('"') {
            break;
        }
        // Skip one string literal: "..."
        let _: &str = "\"".parse_next(input)?; // start quote
        let _: &str = take_while(0.., |c: char| c != '"').parse_next(input)?;
        "\"".parse_next(input)?; // end quote
    }
    Ok(())
}

/// composition
///   : element_list composition2 (annotation SEMICOLON)?
fn composition<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassElement<'a>>> {
    element_list(input)
}

/// composition2: external_clause? | (public_element_list | protected_element_list |
///                                    equation_clause | algorithm_clause) composition2
/// For now, just consume element_list once.
///
/// element_list
///   : (public_element_list | protected_element_list | equation_clause
///      | algorithm_clause | component_clause)*
fn element_list<'a>(input: &mut &'a str) -> ModalResult<Vec<ClassElement<'a>>> {
    let mut elements = Vec::new();
    loop {
        trivia(input)?;
        if input.is_empty() {
            break;
        }
        // Stop if we hit END (end of composition)
        if input.starts_with("end") || input.starts_with("END") {
            break;
        }
        // Check for PUBLIC/PROTECTED/equation/algorithm markers
        if input.starts_with("public") || input.starts_with("PUBLIC") {
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if input.starts_with("protected") || input.starts_with("PROTECTED") {
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if input.starts_with("equation") || input.starts_with("EQUATION") {
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }
        if input.starts_with("algorithm") || input.starts_with("ALGORITHM") {
            take_while(0.., |c: char| !c.is_whitespace() && c != ';').parse_next(input)?;
            continue;
        }

        // Try to parse a component declaration
        match component_declaration(input) {
            Ok(elem) => {
                elements.push(elem);
            }
            Err(_) => {
                // Not a component — skip one token to avoid infinite loop
                trivia(input)?;
                if input.is_empty() || input.starts_with("end") || input.starts_with("END") {
                    break;
                }
                let _: &str = take_while(0.., |c: char| !c.is_whitespace() && !";(),=".contains(c)).parse_next(input)?;
            }
        }

        // Skip trailing semicolons
        trivia(input)?;
        while input.starts_with(';') {
            ";".parse_next(input)?;
            trivia(input)?;
        }
    }
    Ok(elements)
}

/// component_declaration (simplified)
///   : type_prefix ident component_attributes?
/// component_attributes: component_item*
/// component_item: component_reference
///                | component_clause
/// component_clause: base_prefix type_specifier_no_dims component_declaration1
///
/// For our example `Real x(start=0)`:
///   type_prefix = REAL
///   ident = x
///   modification = (start = 0)
fn component_declaration<'a>(input: &mut &'a str) -> ModalResult<ClassElement<'a>> {
    // type_prefix — either a keyword like REAL, or an identifier
    trivia(input)?;
    let type_tok = keyword_or_ident(input)?;
    let typ = match &type_tok {
        Token::Real => ComponentType::Builtin("REAL"),
        Token::Integer => ComponentType::Builtin("INTEGER"),
        Token::Ident(s) => ComponentType::Named(*s),
        _ => ComponentType::Named(tok_as_ident(type_tok)?),
    };

    // ident
    trivia(input)?;
    let name_tok = keyword_or_ident(input)?;
    let name = tok_as_ident(name_tok)?;

    // component_attributes? → check for LPAR (modification)
    trivia(input)?;
    let modification = if input.starts_with('(') {
        Some(modification(input)?)
    } else {
        None
    };

    Ok(ClassElement::Component {
        typ,
        name,
        modification,
    })
}

/// modification: LPAR (named_argument (COMMA named_argument)*)? RPAR
fn modification<'a>(input: &mut &'a str) -> ModalResult<Modification<'a>> {
    "(".parse_next(input)?;

    let mut arguments = Vec::new();
    if !input.starts_with(')') {
        loop {
            trivia(input)?;
            if input.starts_with(')') {
                break;
            }
            // named_argument: ident (EQUALS expr)?
            let arg = named_argument(input)?;
            arguments.push(arg);
            trivia(input)?;
            if input.starts_with(',') {
                ",".parse_next(input)?;
            }
        }
    }

    ")".parse_next(input)?;

    Ok(Modification { arguments })
}

/// named_argument: IDENT (EQUALS expression)?
fn named_argument<'a>(input: &mut &'a str) -> ModalResult<NamedArg<'a>> {
    let tok = keyword_or_ident(input)?;
    let name = tok_as_ident(tok)?;
    trivia(input)?;

    let value = if input.starts_with('=') && !input.starts_with("==") {
        "=".parse_next(input)?;
        // expression — skip for now
        let _: &str = take_while(0.., |c: char| !c.is_whitespace() && !",)".contains(c)).parse_next(input)?;
        ModificationValue::Equals("")
    } else {
        ModificationValue::Simple
    };

    Ok(NamedArg { name, value })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn tok_as_str(tok: &Token<'_>) -> &'static str {
    match tok {
        Token::Package => "package",
        Token::Class => "class",
        Token::Record => "record",
        Token::Type => "type",
        Token::Function => "function",
        _ => "unknown",
    }
}

fn tok_as_ident<'a>(tok: Token<'a>) -> ModalResult<&'a str> {
    match tok {
        Token::Ident(s) => Ok(s),
        _ => Err(ErrMode::Backtrack(ContextError::default())),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Skip whitespace (spaces, tabs, newlines, CR).
    #[test]
    fn skip_ws() {
        let mut input = "   \n\tpackage Foo";
        ws(&mut input).unwrap();
        assert_eq!(input, "package Foo");
    }

    #[test]
    fn skip_line_comment() {
        let mut input = "// comment\npackage Foo";
        trivia(&mut input).unwrap();
        assert_eq!(input, "package Foo");
    }

    #[test]
    fn skip_block_comment() {
        let mut input = "/* comment */package Foo";
        trivia(&mut input).unwrap();
        assert_eq!(input, "package Foo");
    }

    #[test]
    fn keyword_package() {
        let mut input = "package Foo";
        let tok = keyword_or_ident(&mut input).unwrap();
        assert_eq!(tok, Token::Package);
    }

    #[test]
    fn ident_simple_system() {
        let mut input = "SimpleSystem \"Returns the index...\"\nend SimpleSystem;";
        let tok = keyword_or_ident(&mut input).unwrap();
        assert_eq!(tok, Token::Ident("SimpleSystem"));
    }

    /// Parse the full inline package from parser/src/main.rs.
    ///
    /// package SimpleSystem "..."
    ///     /* ... */
    ///     Real x(start=0);
    /// end SimpleSystem;
    #[test]
    fn parse_simple_package() {
        let code = "package SimpleSystem \"Returns the index...\"\n\
                    /* ... */\n\
                    Real x(start=0);\n\
                    end SimpleSystem;";
        let result = stored_definition.parse(code);
        assert!(result.is_ok(), "expected parse success, got: {:?}", result);
        let def = result.unwrap();
        assert_eq!(def.classes.len(), 1, "expected one class definition");
        let class = &def.classes[0];
        assert_eq!(class.name, "SimpleSystem");
        assert_eq!(class.body.len(), 1, "expected one body element");
        // Check it parsed the Real x(start=0) component
        match class.body[0] {
            ClassElement::Component { ref typ, ref name, .. } => {
                assert_eq!(*name, "x", "expected variable name 'x', got '{}'", name);
                match typ {
                    ComponentType::Builtin(s) => assert_eq!(*s, "REAL", "expected builtin REAL"),
                    ComponentType::Named(s) => panic!("expected builtin, got named '{}'", s),
                }
            }
        }
        // Check end name matches
        assert_eq!(class.end_name, Some("SimpleSystem"));
    }

    /// Parse the same inline package as parser/src/main.rs (minimal variant).
    #[test]
    fn parse_first_token() {
        let code = "package SimpleSystem \"Returns the index...\"\nend SimpleSystem;";
        let result = stored_definition.parse(code);
        assert!(result.is_ok(), "expected parse success, got: {:?}", result);
    }
}
