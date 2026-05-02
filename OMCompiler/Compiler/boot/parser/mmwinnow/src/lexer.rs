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

//! Lexer for Modelica / MetaModelica source files.
//!
//! Call [`lex`] to obtain a flat `Vec<Token>` from a source string.
//! Whitespace and comments are discarded. All keyword discrimination is
//! grammar-dependent; see [`Grammar`].

use crate::Grammar;

/// A single token with its start position in the source file.
/// Line and column are both 1-based.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub col: u32,
}

/// All possible token kinds produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // -----------------------------------------------------------------------
    // Literals
    // -----------------------------------------------------------------------
    /// Identifier or quoted identifier (content between single quotes).
    Ident(String),
    /// Integer literal, stored as i32.
    Int(i32),
    /// Real literal, stored as f64.
    Real(f64),
    /// String literal: raw content between the double-quote delimiters.
    /// Escape sequences are preserved as written (e.g. `\n` stays `\n`).
    Str(String),

    // -----------------------------------------------------------------------
    // Base Modelica keywords (all grammars)
    // -----------------------------------------------------------------------
    Algorithm, And, Annotation, Block, Break,
    Class, Connect, Connector, Constant, Constrainedby,
    Der, Discrete, Each, Else, Elseif, Elsewhen, Encapsulated,
    End, Enumeration, Equation, Expandable, Extends, External,
    False, Final, Flow, For, Function,
    If, Import, In, Initial, Inner, Input,
    Loop, Model, Not, Operator, Or, Outer, Output,
    Overload, Package, Parameter, Partial, Protected, Public,
    Record, Redeclare, Replaceable, Return,
    Then, True, Type, When, While, Within,

    // -----------------------------------------------------------------------
    // MetaModelica keywords (Grammar::MetaModelica only)
    // -----------------------------------------------------------------------
    As, Case, Continue, Equality, Failure, Guard, Local,
    Match, Matchcontinue, Subtypeof, Threaded, Try, Uniontype,
    Wild,    // `_`  — standalone wildcard
    Allwild, // `__` — double wildcard

    // ParModelica extensions (Grammar::MetaModelica only)
    Parfor, Parallel, Parlocal, Parglobal, Parkernel,

    // -----------------------------------------------------------------------
    // Grammar-version-gated keywords
    // -----------------------------------------------------------------------
    /// `stream` — Modelica 3.1+, always a keyword in MetaModelica.
    Stream,
    /// `pure` — Modelica 3.3+, always a keyword in MetaModelica.
    Pure,
    /// `impure` — Modelica 3.3+, always a keyword in MetaModelica.
    Impure,
    /// `optimization` — Optimica extension; treated as identifier in Modelica2.
    Optimization,
    /// `constraint` — Optimica extension; treated as identifier in Modelica2.
    Constraint,

    // -----------------------------------------------------------------------
    // OpenModelica dollar-prefixed extensions
    // -----------------------------------------------------------------------
    Code,     // $Code
    CodeName, // $TypeName
    CodeExp,  // $Expression
    CodeVar,  // $Var

    // -----------------------------------------------------------------------
    // Operators — arithmetic
    // -----------------------------------------------------------------------
    Plus, Minus, Star, Slash, Power, Percent,
    /// `.+` element-wise plus
    PlusEw,
    /// `.-` element-wise minus
    MinusEw,
    /// `.*` element-wise multiply
    StarEw,
    /// `./` element-wise divide
    SlashEw,
    /// `.^` element-wise power
    PowerEw,

    // -----------------------------------------------------------------------
    // Operators — comparison
    // -----------------------------------------------------------------------
    Less, Leq, Greater, Geq,
    /// `==`
    EqEq,
    /// `<>`
    NotEq,

    // -----------------------------------------------------------------------
    // Operators — punctuation
    // -----------------------------------------------------------------------
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    /// `=`
    Equal,
    /// `:=`
    Assign,
    Comma, Colon,
    /// `::`
    ColonColon,
    Semi, Dot, Pipe,

    // -----------------------------------------------------------------------
    // Special
    // -----------------------------------------------------------------------
    /// UTF-8 byte-order mark (U+FEFF).
    BOM,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct LexError {
    pub line: u32,
    pub col: u32,
    pub message: String,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lex error at {}:{}: {}", self.line, self.col, self.message)
    }
}

// ---------------------------------------------------------------------------
// Lexer internals
// ---------------------------------------------------------------------------

struct Lexer<'s> {
    src: &'s str,
    /// Byte offset of the next character to read.
    pos: usize,
    line: u32,
    col: u32,
    grammar: &'s Grammar,
}

impl<'s> Lexer<'s> {
    fn new(src: &'s str, grammar: &'s Grammar) -> Self {
        Lexer { src, pos: 0, line: 1, col: 1, grammar }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    /// Returns the character *after* the next one without consuming either.
    fn peek2(&self) -> Option<char> {
        let mut it = self.src[self.pos..].chars();
        it.next()?;
        it.next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.src[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn err(&self, msg: impl Into<String>) -> LexError {
        LexError { line: self.line, col: self.col, message: msg.into() }
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), LexError> {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                    self.advance();
                }
                Some('/') => match self.peek2() {
                    Some('/') => {
                        // Line comment: skip to end of line.
                        while !matches!(self.peek(), None | Some('\n')) {
                            self.advance();
                        }
                    }
                    Some('*') => {
                        // Block comment.
                        let err_line = self.line;
                        let err_col = self.col;
                        self.advance(); // '/'
                        self.advance(); // '*'
                        loop {
                            match self.advance() {
                                None => {
                                    return Err(LexError {
                                        line: err_line,
                                        col: err_col,
                                        message: "unterminated block comment".into(),
                                    });
                                }
                                Some('*') if self.peek() == Some('/') => {
                                    self.advance(); // consume '/'
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => break,
                },
                _ => break,
            }
        }
        Ok(())
    }

    /// Classify a bare word (already collected) as keyword or identifier,
    /// taking the active grammar into account.
    fn keyword_or_ident(&self, word: &str) -> TokenKind {
        let meta = matches!(self.grammar, Grammar::MetaModelica);
        let not_m2 = !matches!(self.grammar, Grammar::Modelica2);

        match word {
            // ---- base keywords ----
            "algorithm"     => TokenKind::Algorithm,
            "and"           => TokenKind::And,
            "annotation"    => TokenKind::Annotation,
            "block"         => TokenKind::Block,
            "break"         => TokenKind::Break,
            "class"         => TokenKind::Class,
            "connect"       => TokenKind::Connect,
            "connector"     => TokenKind::Connector,
            "constant"      => TokenKind::Constant,
            "constrainedby" => TokenKind::Constrainedby,
            "der"           => TokenKind::Der,
            "discrete"      => TokenKind::Discrete,
            "each"          => TokenKind::Each,
            "else"          => TokenKind::Else,
            "elseif"        => TokenKind::Elseif,
            "elsewhen"      => TokenKind::Elsewhen,
            "encapsulated"  => TokenKind::Encapsulated,
            "end"           => TokenKind::End,
            "enumeration"   => TokenKind::Enumeration,
            "equation"      => TokenKind::Equation,
            "expandable"    => TokenKind::Expandable,
            "extends"       => TokenKind::Extends,
            "external"      => TokenKind::External,
            "false"         => TokenKind::False,
            "final"         => TokenKind::Final,
            "flow"          => TokenKind::Flow,
            "for"           => TokenKind::For,
            "function"      => TokenKind::Function,
            "if"            => TokenKind::If,
            "import"        => TokenKind::Import,
            "in"            => TokenKind::In,
            "initial"       => TokenKind::Initial,
            "inner"         => TokenKind::Inner,
            "input"         => TokenKind::Input,
            "loop"          => TokenKind::Loop,
            "model"         => TokenKind::Model,
            "not"           => TokenKind::Not,
            "operator"      => TokenKind::Operator,
            "or"            => TokenKind::Or,
            "outer"         => TokenKind::Outer,
            "output"        => TokenKind::Output,
            "package"       => TokenKind::Package,
            "parameter"     => TokenKind::Parameter,
            "partial"       => TokenKind::Partial,
            "protected"     => TokenKind::Protected,
            "public"        => TokenKind::Public,
            "record"        => TokenKind::Record,
            "redeclare"     => TokenKind::Redeclare,
            "replaceable"   => TokenKind::Replaceable,
            "return"        => TokenKind::Return,
            "then"          => TokenKind::Then,
            "true"          => TokenKind::True,
            "type"          => TokenKind::Type,
            "when"          => TokenKind::When,
            "while"         => TokenKind::While,
            "within"        => TokenKind::Within,

            // ---- MetaModelica-only keywords ----
            "as"            if meta => TokenKind::As,
            "case"          if meta => TokenKind::Case,
            "continue"      if meta => TokenKind::Continue,
            "equality"      if meta => TokenKind::Equality,
            "failure"       if meta => TokenKind::Failure,
            "guard"         if meta => TokenKind::Guard,
            "local"         if meta => TokenKind::Local,
            "match"         if meta => TokenKind::Match,
            "matchcontinue" if meta => TokenKind::Matchcontinue,
            "subtypeof"     if meta => TokenKind::Subtypeof,
            "threaded"      if meta => TokenKind::Threaded,
            "try"           if meta => TokenKind::Try,
            "uniontype"     if meta => TokenKind::Uniontype,
            // ParModelica (also MetaModelica grammar)
            "parfor"        if meta => TokenKind::Parfor,
            "parallel"      if meta => TokenKind::Parallel,
            "parlocal"      if meta => TokenKind::Parlocal,
            "parglobal"     if meta => TokenKind::Parglobal,
            "parkernel"     if meta => TokenKind::Parkernel,
            // MetaModelica wildcards (bare _ / __ — with no following ident chars)
            "_"             if meta => TokenKind::Wild,
            "__"            if meta => TokenKind::Allwild,

            // ---- Modelica 3.x keywords ----
            "stream"  if not_m2 => TokenKind::Stream,
            "pure"    if not_m2 => TokenKind::Pure,
            "impure"  if not_m2 => TokenKind::Impure,

            // ---- Optimica extensions (always enabled for now) ----
            "optimization" => TokenKind::Optimization,
            "constraint"   => TokenKind::Constraint,

            _ => TokenKind::Ident(word.to_string()),
        }
    }

    /// Lex a string literal; the opening `"` has already been consumed.
    /// The raw content (with escape sequences preserved) is returned.
    fn lex_string(&mut self) -> Result<TokenKind, LexError> {
        let mut raw = String::new();
        loop {
            match self.advance() {
                None => return Err(self.err("unterminated string literal")),
                Some('"') => break,
                Some('\\') => {
                    raw.push('\\');
                    match self.advance() {
                        None => return Err(self.err("unterminated escape sequence in string")),
                        Some(c) => raw.push(c),
                    }
                }
                Some(c) => raw.push(c),
            }
        }
        Ok(TokenKind::Str(raw))
    }

    /// Lex a quoted identifier; the opening `'` has already been consumed.
    fn lex_qident(&mut self) -> Result<TokenKind, LexError> {
        let mut s = String::new();
        loop {
            match self.advance() {
                None => return Err(self.err("unterminated quoted identifier")),
                Some('\'') => break,
                Some('\\') => {
                    s.push('\\');
                    match self.advance() {
                        None => return Err(self.err("unterminated escape sequence in quoted identifier")),
                        Some(c) => s.push(c),
                    }
                }
                Some(c) => s.push(c),
            }
        }
        Ok(TokenKind::Ident(s))
    }

    /// Lex a numeric literal; `first` is the first digit already consumed.
    ///
    /// Grammar rule (from BaseModelica_Lexer.g):
    ///   `(DIGIT)+ ('.' (DIGIT)*)? EXPONENT?`
    ///
    /// A `.` that immediately follows digits is *always* consumed as part of
    /// the real number, even if no further digits follow.  This matches the
    /// greedy ANTLR behaviour: `1.+2` lexes as `Real(1.0) Plus Int(2)`, not
    /// `Int(1) PlusEw Int(2)`.
    fn lex_number(&mut self, first: char) -> Result<TokenKind, LexError> {
        let mut s = String::new();
        let mut is_real = false;
        s.push(first);

        while matches!(self.peek(), Some('0'..='9')) {
            s.push(self.advance().unwrap());
        }

        // Optional decimal part: consume '.' that directly follows digits.
        // Exception: do NOT consume if a second '.' follows (would be two DOTs).
        if self.peek() == Some('.') && self.peek2() != Some('.') {
            is_real = true;
            s.push(self.advance().unwrap()); // '.'
            while matches!(self.peek(), Some('0'..='9')) {
                s.push(self.advance().unwrap());
            }
        }

        // Optional exponent.
        if matches!(self.peek(), Some('e') | Some('E')) {
            is_real = true;
            s.push(self.advance().unwrap());
            if matches!(self.peek(), Some('+') | Some('-')) {
                s.push(self.advance().unwrap());
            }
            if !matches!(self.peek(), Some('0'..='9')) {
                return Err(self.err("expected digits after exponent"));
            }
            while matches!(self.peek(), Some('0'..='9')) {
                s.push(self.advance().unwrap());
            }
        }

        if is_real {
            s.parse::<f64>()
                .map(TokenKind::Real)
                .map_err(|e| self.err(format!("invalid real literal '{}': {}", s, e)))
        } else {
            s.parse::<i32>()
                .map(TokenKind::Int)
                .map_err(|e| self.err(format!("integer literal '{}' out of i32 range: {}", s, e)))
        }
    }

    /// Lex a real literal that begins with `.digit…`; the leading `.` has
    /// already been consumed.
    fn lex_dot_number(&mut self) -> Result<TokenKind, LexError> {
        let mut s = String::from("0.");
        while matches!(self.peek(), Some('0'..='9')) {
            s.push(self.advance().unwrap());
        }
        if matches!(self.peek(), Some('e') | Some('E')) {
            s.push(self.advance().unwrap());
            if matches!(self.peek(), Some('+') | Some('-')) {
                s.push(self.advance().unwrap());
            }
            if !matches!(self.peek(), Some('0'..='9')) {
                return Err(self.err("expected digits after exponent"));
            }
            while matches!(self.peek(), Some('0'..='9')) {
                s.push(self.advance().unwrap());
            }
        }
        s.parse::<f64>()
            .map(TokenKind::Real)
            .map_err(|e| self.err(format!("invalid real literal '{}': {}", s, e)))
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexError> {
        self.skip_whitespace_and_comments()?;

        let line = self.line;
        let col = self.col;

        let c = match self.advance() {
            None => return Ok(None),
            Some(c) => c,
        };

        let kind = match c {
            // UTF-8 BOM (U+FEFF, a single Rust char)
            '\u{FEFF}' => TokenKind::BOM,

            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semi,
            '|' => TokenKind::Pipe,
            '%' => TokenKind::Percent,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '^' => TokenKind::Power,

            '/' => {
                // Note: '//' and '/*' are handled by skip_whitespace_and_comments
                // before we get here, so bare '/' is always division.
                TokenKind::Slash
            }

            '=' => {
                if self.peek() == Some('=') { self.advance(); TokenKind::EqEq }
                else { TokenKind::Equal }
            }

            '<' => match self.peek() {
                Some('=') => { self.advance(); TokenKind::Leq }
                Some('>') => { self.advance(); TokenKind::NotEq }
                _ => TokenKind::Less,
            },

            '>' => {
                if self.peek() == Some('=') { self.advance(); TokenKind::Geq }
                else { TokenKind::Greater }
            }

            ':' => {
                if self.peek() == Some('=') { self.advance(); TokenKind::Assign }
                else if self.peek() == Some(':') { self.advance(); TokenKind::ColonColon }
                else { TokenKind::Colon }
            }

            '.' => match self.peek() {
                Some('0'..='9') => self.lex_dot_number()?,
                Some('+') => { self.advance(); TokenKind::PlusEw }
                Some('-') => { self.advance(); TokenKind::MinusEw }
                Some('*') => { self.advance(); TokenKind::StarEw }
                Some('/') => { self.advance(); TokenKind::SlashEw }
                Some('^') => { self.advance(); TokenKind::PowerEw }
                _ => TokenKind::Dot,
            },

            '"' => self.lex_string()?,

            '\'' => self.lex_qident()?,

            // Dollar-prefixed OpenModelica extensions and $overload.
            '$' => {
                let mut word = String::from("$");
                while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
                    word.push(self.advance().unwrap());
                }
                match word.as_str() {
                    "$Code"       => TokenKind::Code,
                    "$TypeName"   => TokenKind::CodeName,
                    "$Expression" => TokenKind::CodeExp,
                    "$Var"        => TokenKind::CodeVar,
                    "$overload"   => TokenKind::Overload,
                    // $cpuTime and other $-prefixed identifiers become Ident.
                    _ => TokenKind::Ident(word),
                }
            }

            // Identifiers and keywords.
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut word = String::new();
                word.push(c);
                while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
                    word.push(self.advance().unwrap());
                }
                self.keyword_or_ident(&word)
            }

            // Numeric literals.
            c if c.is_ascii_digit() => self.lex_number(c)?,

            // Non-standard but tolerated (with error in original grammar):
            // '&&' → And, '||' → Or, '!' → Not.
            '&' if self.peek() == Some('&') => { self.advance(); TokenKind::And }
            '|' if self.peek() == Some('|') => { self.advance(); TokenKind::Or }
            '!' => TokenKind::Not,

            other => {
                return Err(LexError {
                    line,
                    col,
                    message: format!("unexpected character: {:?}", other),
                });
            }
        };

        Ok(Some(Token { kind, line, col }))
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Lex a Modelica / MetaModelica source string into a token stream.
///
/// `grammar` controls which keywords are active:
/// - [`Grammar::MetaModelica`]: all extensions enabled
/// - [`Grammar::Modelica3`]: `stream`, `pure`, `impure` are keywords
/// - [`Grammar::Modelica2`]: those three are plain identifiers
///
/// All other keyword sets (`optimization`, `constraint`, and the base keyword
/// set) are grammar-independent.
pub fn lex(src: &str, grammar: Grammar) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(src, &grammar);
    let mut tokens = Vec::new();
    while let Some(tok) = lexer.next_token()? {
        tokens.push(tok);
    }
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        lex(src, Grammar::MetaModelica).unwrap().into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_keywords() {
        assert_eq!(kinds("algorithm equation model"), vec![
            TokenKind::Algorithm, TokenKind::Equation, TokenKind::Model,
        ]);
    }

    #[test]
    fn test_meta_keywords() {
        assert_eq!(kinds("match matchcontinue case"), vec![
            TokenKind::Match, TokenKind::Matchcontinue, TokenKind::Case,
        ]);
        // In Modelica3, 'match' is just an identifier.
        let toks = lex("match", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].kind, TokenKind::Ident("match".into()));
    }

    #[test]
    fn test_wildcards() {
        assert_eq!(kinds("_ __"), vec![TokenKind::Wild, TokenKind::Allwild]);
        // In Modelica3, _ and __ are plain identifiers.
        let toks = lex("_ __", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].kind, TokenKind::Ident("_".into()));
        assert_eq!(toks[1].kind, TokenKind::Ident("__".into()));
        // _foo is always an identifier.
        assert_eq!(kinds("_foo"), vec![TokenKind::Ident("_foo".into())]);
    }

    #[test]
    fn test_stream_pure_impure() {
        assert_eq!(
            lex("stream pure impure", Grammar::Modelica3).unwrap().iter().map(|t| &t.kind).collect::<Vec<_>>(),
            vec![&TokenKind::Stream, &TokenKind::Pure, &TokenKind::Impure],
        );
        // In Modelica2 these are identifiers.
        let toks = lex("stream pure impure", Grammar::Modelica2).unwrap();
        assert!(matches!(&toks[0].kind, TokenKind::Ident(s) if s == "stream"));
        assert!(matches!(&toks[1].kind, TokenKind::Ident(s) if s == "pure"));
        assert!(matches!(&toks[2].kind, TokenKind::Ident(s) if s == "impure"));
    }

    #[test]
    fn test_literals() {
        let toks = lex(r#"42 3.14 1.0e5 .5 "hello\nworld" 'quoted ident'"#, Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].kind, TokenKind::Int(42));
        assert_eq!(toks[1].kind, TokenKind::Real(3.14));
        assert_eq!(toks[2].kind, TokenKind::Real(1.0e5));
        assert_eq!(toks[3].kind, TokenKind::Real(0.5));
        assert_eq!(toks[4].kind, TokenKind::Str("hello\\nworld".into()));
        assert_eq!(toks[5].kind, TokenKind::Ident("quoted ident".into()));
    }

    #[test]
    fn test_operators() {
        let toks = lex(":= :: .+ .* <= <> ==", Grammar::MetaModelica).unwrap();
        let ks: Vec<_> = toks.iter().map(|t| &t.kind).collect();
        assert_eq!(ks, vec![
            &TokenKind::Assign, &TokenKind::ColonColon, &TokenKind::PlusEw,
            &TokenKind::StarEw, &TokenKind::Leq, &TokenKind::NotEq, &TokenKind::EqEq,
        ]);
    }

    #[test]
    fn test_positions() {
        let toks = lex("a\nb", Grammar::Modelica3).unwrap();
        assert_eq!((toks[0].line, toks[0].col), (1, 1));
        assert_eq!((toks[1].line, toks[1].col), (2, 1));
    }

    #[test]
    fn test_real_then_plus() {
        // '1.+2' → Real(1.0) Plus Int(2), matching ANTLR greedy behaviour.
        let toks = lex("1.+2", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].kind, TokenKind::Real(1.0));
        assert_eq!(toks[1].kind, TokenKind::Plus);
        assert_eq!(toks[2].kind, TokenKind::Int(2));
    }

    #[test]
    fn test_elementwise_after_ident() {
        // 'a.+b' → Ident Plus-EW Ident.
        let toks = lex("a.+b", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].kind, TokenKind::Ident("a".into()));
        assert_eq!(toks[1].kind, TokenKind::PlusEw);
        assert_eq!(toks[2].kind, TokenKind::Ident("b".into()));
    }

    #[test]
    fn test_comments() {
        let toks = lex("a // line comment\nb /* block */ c", Grammar::Modelica3).unwrap();
        let ks: Vec<_> = toks.iter().map(|t| &t.kind).collect();
        assert_eq!(ks, vec![
            &TokenKind::Ident("a".into()),
            &TokenKind::Ident("b".into()),
            &TokenKind::Ident("c".into()),
        ]);
    }

    #[test]
    fn test_dollar_tokens() {
        let toks = lex("$Code $TypeName $Expression $Var $overload", Grammar::MetaModelica).unwrap();
        let ks: Vec<_> = toks.iter().map(|t| &t.kind).collect();
        assert_eq!(ks, vec![
            &TokenKind::Code, &TokenKind::CodeName, &TokenKind::CodeExp,
            &TokenKind::CodeVar, &TokenKind::Overload,
        ]);
    }

    #[test]
    fn lex_codegen_c() {
        let code = std::fs::read_to_string("tests/data/CodegenC.mo")
            .expect("CodegenC.mo not found");
        let result = lex(&code, Grammar::MetaModelica);
        if let Some(err) = &result.err() {
            assert!(false, "expected CodegenC.mo to lex, got: {}", err);
        }
    }
}
