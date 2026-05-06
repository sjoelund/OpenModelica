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
    /// Real literal, stored as f64 and original string.
    Real(f64, String),
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
    Code,          // $Code
    CodeName,      // $TypeName
    CodeExp,       // $Expression
    CodeVar,       // $Var
    CodeAnnotation,// $annotation

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
// Token end-position helpers
// ---------------------------------------------------------------------------

impl TokenKind {
    /// Number of source characters spanned by this token kind.
    ///
    /// Limitations:
    /// - `Ident` produced from a quoted identifier (`'...'`) is
    ///   indistinguishable from a plain one; returns content length, not `+2`.
    /// - `And` from `&&` and `Not` from `!` return the keyword spelling length
    ///   (`"and"` = 3, `"not"` = 3) rather than 2 or 1.
    pub fn source_char_len(&self) -> u32 {
        match self {
            TokenKind::Ident(s)     => s.chars().count() as u32,
            TokenKind::Int(n)       => n.to_string().len() as u32,
            TokenKind::Real(_, s)   => s.chars().count() as u32,
            // Content has escape sequences preserved (e.g. `\n` → `\` + `n`,
            // 2 chars in both raw string and source), so char count + 2 quotes
            // gives the correct source length for single-line strings.
            TokenKind::Str(s)       => s.chars().count() as u32 + 2,

            TokenKind::Wild         => 1,  // `_`
            TokenKind::Allwild      => 2,  // `__`
            TokenKind::Operator     => 8,  // "operator" (absent from keyword_as_str)

            // Single-char punctuation / operators
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash |
            TokenKind::Power | TokenKind::Percent | TokenKind::Less | TokenKind::Greater |
            TokenKind::LParen | TokenKind::RParen | TokenKind::LBracket |
            TokenKind::RBracket | TokenKind::LBrace | TokenKind::RBrace |
            TokenKind::Equal | TokenKind::Comma | TokenKind::Colon |
            TokenKind::Semi | TokenKind::Dot | TokenKind::Pipe | TokenKind::BOM => 1,

            // Two-char operators
            TokenKind::EqEq | TokenKind::NotEq | TokenKind::Leq | TokenKind::Geq |
            TokenKind::Assign | TokenKind::ColonColon |
            TokenKind::PlusEw | TokenKind::MinusEw | TokenKind::StarEw |
            TokenKind::SlashEw | TokenKind::PowerEw => 2,

            // $-prefixed OpenModelica extensions
            TokenKind::Code           => 5,   // "$Code"
            TokenKind::CodeName       => 9,   // "$TypeName"
            TokenKind::CodeExp        => 11,  // "$Expression"
            TokenKind::CodeVar        => 4,   // "$Var"
            TokenKind::CodeAnnotation => 11,  // "$annotation"
            TokenKind::Overload       => 9,   // "$overload"

            // All remaining variants are keywords covered by keyword_as_str.
            // All keywords are ASCII so .len() == char count.
            _ => keyword_as_str(self).unwrap().len() as u32,
        }
    }
}

impl Token {
    /// Compute the 1-based `(end_line, end_col)` of the last character of this token.
    ///
    /// String literals may contain embedded newlines; those are scanned to
    /// determine the correct end position.  All other tokens are assumed
    /// single-line and end at `(line, col + source_char_len - 1)`.
    pub fn end_pos(&self) -> (u32, u32) {
        if let TokenKind::Str(s) = &self.kind {
            let mut line = self.line;
            let mut col = self.col + 1; // step past opening `"`
            for ch in s.chars() {
                if ch == '\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }
            }
            return (line, col); // col is now on the closing `"`
        }
        (self.line, self.col + self.kind.source_char_len() - 1)
    }
}

// ---------------------------------------------------------------------------
// Keyword-to-string helper
// ---------------------------------------------------------------------------

/// If `kind` is a keyword, return its source spelling; otherwise return `None`.
pub fn keyword_as_str(kind: &TokenKind) -> Option<&'static str> {
    match kind {
        TokenKind::Algorithm => Some("algorithm"),
        TokenKind::And => Some("and"),
        TokenKind::Annotation => Some("annotation"),
        TokenKind::Block => Some("block"),
        TokenKind::Break => Some("break"),
        TokenKind::Class => Some("class"),
        TokenKind::Connect => Some("connect"),
        TokenKind::Connector => Some("connector"),
        TokenKind::Constant => Some("constant"),
        TokenKind::Constrainedby => Some("constrainedby"),
        TokenKind::Der => Some("der"),
        TokenKind::Discrete => Some("discrete"),
        TokenKind::Each => Some("each"),
        TokenKind::Else => Some("else"),
        TokenKind::Elseif => Some("elseif"),
        TokenKind::Elsewhen => Some("elsewhen"),
        TokenKind::Encapsulated => Some("encapsulated"),
        TokenKind::End => Some("end"),
        TokenKind::Enumeration => Some("enumeration"),
        TokenKind::Equation => Some("equation"),
        TokenKind::Expandable => Some("expandable"),
        TokenKind::Extends => Some("extends"),
        TokenKind::External => Some("external"),
        TokenKind::False => Some("false"),
        TokenKind::Final => Some("final"),
        TokenKind::Flow => Some("flow"),
        TokenKind::For => Some("for"),
        TokenKind::Function => Some("function"),
        TokenKind::If => Some("if"),
        TokenKind::Import => Some("import"),
        TokenKind::In => Some("in"),
        TokenKind::Initial => Some("initial"),
        TokenKind::Inner => Some("inner"),
        TokenKind::Input => Some("input"),
        TokenKind::Loop => Some("loop"),
        TokenKind::Model => Some("model"),
        TokenKind::Not => Some("not"),
        TokenKind::Operator => Some("operator"),
        TokenKind::Or => Some("or"),
        TokenKind::Outer => Some("outer"),
        TokenKind::Output => Some("output"),
        TokenKind::Overload => Some("$overload"),
        TokenKind::Package => Some("package"),
        TokenKind::Parameter => Some("parameter"),
        TokenKind::Partial => Some("partial"),
        TokenKind::Protected => Some("protected"),
        TokenKind::Public => Some("public"),
        TokenKind::Record => Some("record"),
        TokenKind::Redeclare => Some("redeclare"),
        TokenKind::Replaceable => Some("replaceable"),
        TokenKind::Return => Some("return"),
        TokenKind::Then => Some("then"),
        TokenKind::True => Some("true"),
        TokenKind::Type => Some("type"),
        TokenKind::When => Some("when"),
        TokenKind::While => Some("while"),
        TokenKind::Within => Some("within"),
        TokenKind::As => Some("as"),
        TokenKind::Case => Some("case"),
        TokenKind::Continue => Some("continue"),
        TokenKind::Equality => Some("equality"),
        TokenKind::Failure => Some("failure"),
        TokenKind::Guard => Some("guard"),
        TokenKind::Local => Some("local"),
        TokenKind::Match => Some("match"),
        TokenKind::Matchcontinue => Some("matchcontinue"),
        TokenKind::Subtypeof => Some("subtypeof"),
        TokenKind::Threaded => Some("threaded"),
        TokenKind::Try => Some("try"),
        TokenKind::Uniontype => Some("uniontype"),
        TokenKind::Parfor => Some("parfor"),
        TokenKind::Parallel => Some("parallel"),
        TokenKind::Parlocal => Some("parlocal"),
        TokenKind::Parglobal => Some("parglobal"),
        TokenKind::Parkernel => Some("parkernel"),
        TokenKind::Stream => Some("stream"),
        TokenKind::Pure => Some("pure"),
        TokenKind::Impure => Some("impure"),
        TokenKind::Optimization => Some("optimization"),
        TokenKind::Constraint => Some("constraint"),
        _ => None,
    }
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

impl std::error::Error for LexError {}

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
        let m3 = matches!(self.grammar, Grammar::Modelica3);
        let optimica = matches!(self.grammar, Grammar::Optimica);

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
            "der"           if !meta => TokenKind::Der,
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
            "inner"         if !meta => TokenKind::Inner,
            "input"         => TokenKind::Input,
            "loop"          => TokenKind::Loop,
            "model"         => TokenKind::Model,
            "not"           => TokenKind::Not,
            "operator"      if m3 => TokenKind::Operator,
            "or"            => TokenKind::Or,
            "outer"         if !meta => TokenKind::Outer,
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
            "stream"  if m3 => TokenKind::Stream,
            "pure"    if m3 => TokenKind::Pure,
            "impure"  if m3 || meta => TokenKind::Impure,

            // ---- Optimica extensions (always enabled for now) ----
            "optimization" if optimica => TokenKind::Optimization,
            "constraint"   if optimica => TokenKind::Constraint,

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
            match s.parse::<f64>() {
                Ok(n) if n.is_finite() => Ok(TokenKind::Real(n, s)),
                Ok(_) => Err(self.err(format!("real literal '{}' is infinite or NaN, which is not allowed", s))),
                Err(e) => Err(self.err(format!("invalid real literal '{}': {}", s, e))),
            }
        } else {
            s.parse::<i32>()
                .map(|n| TokenKind::Int(n))
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
        match s.parse::<f64>() {
            Ok(n) if n.is_finite() => Ok(TokenKind::Real(n, s[1..].to_string())),
            Ok(_) => Err(self.err(format!("real literal '{}' is infinite or NaN, which is not allowed", s))),
            Err(e) => Err(self.err(format!("invalid real literal '{}': {}", s, e))),
        }
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
                    "$annotation" => TokenKind::CodeAnnotation,
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
            '|' => TokenKind::Pipe,
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
        assert_eq!(toks[1].kind, TokenKind::Real(3.14, "3.14".into()));
        assert_eq!(toks[2].kind, TokenKind::Real(1.0e5, "1.0e5".into()));
        assert_eq!(toks[3], Token{kind:TokenKind::Real(0.5, ".5".into()), line:1, col:15});
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
    fn test_end_pos() {
        // Single-char token
        let toks = lex("+", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (1, 1));

        // Two-char token
        let toks = lex(":=", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (1, 2));

        // Keyword
        let toks = lex("algorithm", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (1, 9));

        // Identifier
        let toks = lex("foo", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (1, 3));

        // Integer literal
        let toks = lex("42", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (1, 2));

        // Real literal
        let toks = lex("3.14", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (1, 4));

        // String literal (single-line)
        let toks = lex(r#""hello""#, Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (1, 7));

        // String literal with embedded newline
        let toks = lex("\"a\nb\"", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (2, 2));

        // Token not at column 1
        let toks = lex("ab cd", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].end_pos(), (1, 2));
        assert_eq!(toks[1].end_pos(), (1, 5));
    }

    #[test]
    fn test_real_then_plus() {
        // '1.+2' → Real(1.0) Plus Int(2), matching ANTLR greedy behaviour.
        let toks = lex("1.+2", Grammar::Modelica3).unwrap();
        assert_eq!(toks[0].kind, TokenKind::Real(1.0, "1.".into()));
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
