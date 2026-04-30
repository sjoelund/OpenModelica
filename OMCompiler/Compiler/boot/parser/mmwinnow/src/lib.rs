//! mmwinnow — winnow-based MetaModelica parser
//!
//! Lexer combinators are embedded in the parser — no separate tokenizer.

use winnow::{Parser, ModalResult, error::{ContextError, ErrMode}};
use winnow::token::*;
use winnow::ascii;

// ---------------------------------------------------------------------------
// Token types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Package,
    Ident(&'a str),
}

// ---------------------------------------------------------------------------
// Lexer combinators
// ---------------------------------------------------------------------------

/// Skip whitespace (spaces, tabs, newlines, CR).
#[allow(dead_code)]
fn ws<'a>(input: &mut &'a str) -> ModalResult<()> {
    take_while(0.., |c: char| c.is_whitespace()).parse_next(input)?;
    Ok(())
}

/// Skip all trivia: whitespace, line comments, block comments.
#[allow(dead_code)]
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
#[allow(dead_code)]
fn keyword_or_ident<'a>(input: &mut &'a str) -> ModalResult<Token<'a>> {
    let word: &str =
        take_while(0.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
    match word {
        "package" => Ok(Token::Package),
        _ => Ok(Token::Ident(word)),
    }
}

// ---------------------------------------------------------------------------
// Parser rules
// ---------------------------------------------------------------------------

/// stub: class_definition_list
///
/// Real implementation will parse `class_definition SEMICOLON` repetitions.
/// For now, returns an empty vec.
#[allow(dead_code)]
fn class_definition_list<'a>(_input: &mut &'a str) -> ModalResult<Vec<Token<'a>>> {
    Ok(Vec::new())
}

/// stored_definition
///   : BOM? (within_clause SEMICOLON)? class_definition_list EOF
///
/// BOM, within_clause, and class_definition_list are stubs for now.
pub fn stored_definition(input: &mut &str) -> ModalResult<()> {
    // BOM? (U+FEFF)
    let _: ModalResult<&str> = "\u{feff}".parse_next(input);

    // (within_clause SEMICOLON)?  — stub
    // within_clause: WITHIN name_path?

    // class_definition_list (stub)
    class_definition_list(input)?;

    // EOF
    if !input.is_empty() {
        return Err(ErrMode::Backtrack(ContextError::default()));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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

    /// Parse the same inline package as parser/src/main.rs.
    ///
    /// class_definition_list is a stub (returns empty), so this test
    /// verifies that stored_definition runs to completion and the stub
    /// wires into the pipeline.
    #[test]
    fn parse_first_token() {
        let code = "package SimpleSystem \"Returns the index...\"\nend SimpleSystem;";
        let result = stored_definition.parse(code);
        // Expected: Err — class_definition_list is a stub, EOF not reached
        assert!(result.is_err(), "expected parse failure (stub class_definition_list)");
    }
}
