//! Translation of Lexers/LexerJSON.mo
//!
//! This module provides a JSON lexer based on a DFA (deterministic finite automaton).
//! It scans JSON source code and outputs a list of tokens (strings, numbers,
//! booleans, null, structural characters).
//!
//! # Assumptions
//! - `SourceInfo` is imported from `errortypes::SourceInfo`
//! - `System::read_file`, `System::strcmp_offset` are not yet translated;
//!   stubs are provided for `read_file` and `str_cmp_offset`
//! - `MetaModelica.Dangerous.list_reverse_in_place` is not yet translated;
//!   a local `list_reverse` helper is used
//! - `MetaModelica.Dangerous.string_get` (no-bounds-checking) is not yet
//!   translated; `string_get` is a safe Rust equivalent
//! - `intString`, `intStringChar`, `print`, `fail` are provided as stubs
//!   matching the MetaModelica runtime.
//!
//! # DFA Table Source
//! - All DFA lookup tables (YY_BASE, YY_NXT, YY_CHK, YY_DEF, YY_EC, YY_META,
//!   YY_ACCEPT, YY_ACCLIST) are extracted directly from the C code generated
//!   by the MetaModelica lexer generator (boot/build/LexerJSON.c).
//! - Tables are used with 0-based indexing in Rust.

use im::Vector;
use std::fmt;

// ============================================================================
// Type aliases / imports
// ============================================================================

/// Persistent list type (mapped to im::Vector)
type List<T> = Vector<T>;

/// SourceInfo from errortypes
#[path = "errortypes.rs"]
mod _errortypes;
use _errortypes::SourceInfo;

// ============================================================================
// TokenId - enumeration of JSON token types
// ============================================================================

/// Token identifiers for JSON lexer output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum TokenId {
    /// No token (error or whitespace)
    _NO_TOKEN,
    /// [ or {
    ARRAYBEGIN,
    /// ] or }
    ARRAYEND,
    /// :
    COLON,
    /// ,
    COMMA,
    /// false
    FALSE,
    /// Integer literal
    INTEGER,
    /// null
    NULL,
    /// Real/float literal
    NUMBER,
    /// {
    OBJECTBEGIN,
    /// }
    OBJECTEND,
    /// String literal
    STRING,
    /// true
    TRUE,
}

impl fmt::Display for TokenId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenId::_NO_TOKEN => write!(f, "_NO_TOKEN"),
            TokenId::ARRAYBEGIN => write!(f, "ARRAYBEGIN"),
            TokenId::ARRAYEND => write!(f, "ARRAYEND"),
            TokenId::COLON => write!(f, "COLON"),
            TokenId::COMMA => write!(f, "COMMA"),
            TokenId::FALSE => write!(f, "FALSE"),
            TokenId::INTEGER => write!(f, "INTEGER"),
            TokenId::NULL => write!(f, "NULL"),
            TokenId::NUMBER => write!(f, "NUMBER"),
            TokenId::OBJECTBEGIN => write!(f, "OBJECTBEGIN"),
            TokenId::OBJECTEND => write!(f, "OBJECTEND"),
            TokenId::STRING => write!(f, "STRING"),
            TokenId::TRUE => write!(f, "TRUE"),
        }
    }
}

// ============================================================================
// Token - uniontype for token data
// ============================================================================

/// Token produced by the lexer, containing position information and source text.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub file_name: String,
    pub id: TokenId,
    pub file_contents: String,
    pub byte_offset: i32,
    pub length: i32,
    pub line_number_start: i32,
    pub column_number_start: i32,
    pub line_number_end: i32,
    pub column_number_end: i32,
}

/// Token variant constructor (equivalent to TOKEN record in MM).
fn token(
    file_name: &str,
    id: TokenId,
    file_contents: &str,
    byte_offset: i32,
    length: i32,
    line_number_start: i32,
    column_number_start: i32,
    line_number_end: i32,
    column_number_end: i32,
) -> Token {
    Token {
        file_name: file_name.to_string(),
        id,
        file_contents: file_contents.to_string(),
        byte_offset,
        length,
        line_number_start,
        column_number_start,
        line_number_end,
        column_number_end,
    }
}

/// No-token constant factory.
pub fn no_token() -> Token {
    Token {
        file_name: "<NoFile>".to_string(),
        id: TokenId::_NO_TOKEN,
        file_contents: String::new(),
        byte_offset: 0,
        length: 0,
        line_number_start: 0,
        column_number_start: 0,
        line_number_end: 0,
        column_number_end: 0,
    }
}

// ============================================================================
// Stub functions for MetaModelica runtime (not yet translated)
// ============================================================================

/// Stub for System.readFile. Returns empty string.
fn read_file(_path: &str) -> String {
    String::new()
}

/// Stub for System.strcmp_offset. Compares substrings by offset and length.
fn str_cmp_offset(
    s1: &str,
    offset1: i32,
    len1: i32,
    s2: &str,
    offset2: i32,
    len2: i32,
) -> i32 {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let end1 = (offset1 + len1) as usize;
    let end2 = (offset2 + len2) as usize;
    let sub1: String = s1[offset1 as usize..end1.min(s1.len())].iter().collect();
    let sub2: String = s2[offset2 as usize..end2.min(s2.len())].iter().collect();
    if sub1 == sub2 {
        0
    } else if sub1 < sub2 {
        -1
    } else {
        1
    }
}

/// Stub for intString. Converts Integer to String.
fn int_string(val: i32) -> String {
    format!("{}", val)
}

/// Stub for intStringChar. Converts character code to String.
fn int_string_char(code: i32) -> String {
    if code >= 0 && code <= 127 {
        String::from(code as u8 as char)
    } else {
        format!("{}", code)
    }
}

/// Stub for print.
fn print(_s: &str) {
    eprint!("{}", _s);
}

/// Stub for fail. Unwraps with an error.
fn fail() {
    panic!("fail() called in MetaModelica code");
}

/// Stub for sourceInfo.
fn source_info() -> SourceInfo {
    SourceInfo {
        file_name: String::new(),
        is_read_only: false,
        start_line: 0,
        start_column: 0,
        end_line: 0,
        end_column: 0,
    }
}

/// Stub for checkArrayModelica. No-op in release, checks bounds in debug.
#[allow(dead_code)]
fn check_array_modelica(_arr: &[i32], _index: i32, _info: SourceInfo) {
    #[cfg(debug_assertions)]
    {
        if _index < 1 || (_index as usize) > _arr.len() {
            print(&format!(
                "\n[checkArray failed: arrayLength={} index={}\n",
                _arr.len(),
                _index
            ));
            fail();
        }
    }
}

/// Stub for listGet. Returns element at 1-based index.
fn list_get<T: Clone>(list: &List<T>, index: i32) -> T {
    list.get((index - 1) as usize)
        .cloned()
        .expect("list_get: index out of bounds")
}

/// Stub for listReverseInPlace. Reverses a vector in place.
fn list_reverse_in_place<T: Clone>(mut list: List<T>) -> List<T> {
    let mut result = List::new();
    while let Some(item) = list.pop_front() {
        result.push_front(item);
    }
    result
}

/// Safe string character getter at 1-based index.
fn string_get(s: &str, pos: i32) -> i32 {
    let idx = (pos - 1) as usize;
    if idx < s.len() {
        let ch = s.as_bytes()[idx] as i32;
        ch
    } else {
        0
    }
}

// ============================================================================
// Token functions
// ============================================================================

/// Print token information as a string.
pub fn print_token(token: &Token) -> String {
    let contents = if token.length > 0 {
        let bytes = token.file_contents.as_bytes();
        let start = token.byte_offset as usize;
        let end = ((token.byte_offset + token.length) as usize).min(bytes.len());
        if start <= end {
            String::from_utf8_lossy(&bytes[start..end]).into_owned()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    format!(
        "[TOKEN:{} '{}' ({}:{}-{}:{})]",
        token.id,
        contents,
        token.line_number_start,
        token.column_number_start,
        token.line_number_end,
        token.column_number_end,
    )
}

/// Get the content of a token (the substring it represents).
pub fn token_content(token: &Token) -> String {
    if token.length > 0 {
        let bytes = token.file_contents.as_bytes();
        let start = token.byte_offset as usize;
        let end = ((token.byte_offset + token.length) as usize).min(bytes.len());
        if start < end {
            String::from_utf8_lossy(&bytes[start..end]).into_owned()
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Compare the content of two tokens for equality.
pub fn token_content_eq(token1: &Token, token2: &Token) -> bool {
    let (contents1, offset1, length1) =
        (&token1.file_contents, token1.byte_offset, token1.length);
    let (contents2, offset2, length2) =
        (&token2.file_contents, token2.byte_offset, token2.length);

    if length1 != length2 {
        return false;
    }
    if length1 == 0 {
        return true;
    }
    0 == str_cmp_offset(contents1, offset1, length1, contents2, offset2, length2)
}

/// Convert a Token to a SourceInfo.
pub fn token_source_info(token: &Token) -> SourceInfo {
    SourceInfo {
        file_name: token.file_name.clone(),
        is_read_only: false,
        start_line: token.line_number_start,
        start_column: token.column_number_start,
        end_line: token.line_number_end,
        end_column: token.column_number_end,
    }
}

// ============================================================================
// LexTable - DFA lookup tables
// ============================================================================

/// DFA lookup tables generated by the lexer generator.
pub mod lex_table {
    /// The limit value.
    pub const YY_LIMIT: i32 = 51;
    /// The finish marker value.
    pub const YY_FINISH: i32 = 82;

    /// yy_acclist table (32 elements).
    pub const YY_ACCLIST: [i32; 32] = [
        17, 16, 15, 16, 16, 13, 16, 5, 16, 14, 16, 11, 16, 12, 16, 16,
        16, 16, 9, 16, 10, 16, 15, 1, 5, 2, 3, 4, 8, 6, 3, 7,
    ];

    /// yy_accept table (46 elements).
    pub const YY_ACCEPT: [i32; 46] = [
        1, 1, 1, 2, 3, 5, 6, 8, 10, 12,
        14, 16, 17, 18, 19, 21, 23, 24, 24, 25,
        25, 25, 26, 26, 26, 26, 26, 27, 27, 28,
        28, 29, 29, 29, 29, 29, 29, 29, 30, 31,
        31, 31, 32, 33, 33, 33,
    ];

    /// yy_ec table (225 elements).
    pub const YY_EC: [i32; 255] = [
        1, 1, 1, 1, 1, 1, 1, 1, 2, 2,
        1, 1, 2, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 2, 1, 3, 1, 1, 1, 1, 1, 1,
        1, 1, 4, 5, 6, 7, 8, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 10, 1, 1,
        1, 1, 1, 1, 11, 11, 11, 11, 12, 11,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        13, 14, 15, 1, 1, 1, 16, 17, 11, 11,
        18, 19, 1, 1, 1, 1, 1, 20, 1, 21,
        1, 1, 1, 22, 23, 24, 25, 1, 1, 1,
        1, 1, 26, 1, 27, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    /// yy_meta table (27 elements).
    pub const YY_META: [i32; 27] = [
        1, 1, 2, 1, 1, 1, 1, 2, 3, 1, 3, 3, 4, 1, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1,
        1, 1, 1,
    ];

    /// yy_base table (51 elements).
    /// When base[state] == YY_FINISH (82), the state is a finish state.
    pub const YY_BASE: [i32; 51] = [
        0, 0, 81, 82, 78, 25, 82, 22, 82, 82, 82, 63, 53, 55, 82, 82, 74, 27, 82,
        50, 65, 26, 39, 53, 52, 37, 82, 0, 37, 45, 43, 27, 27, 24, 0, 47, 19,
        82, 82, 0, 27, 23, 82, 0, 82, 56, 59, 61, 63, 65, 67,
    ];

    /// yy_def table (51 elements).
    pub const YY_DEF: [i32; 51] = [
        45, 1, 45, 45, 45, 46, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 46,
        45, 47, 45, 45, 45, 45, 45, 45, 45, 48, 45, 45, 45, 45, 45, 45, 49, 45,
        45, 45, 45, 50, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45,
    ];

    /// yy_nxt table (109 elements) - next state transitions indexed by base[state] + char_class.
    pub const YY_NXT: [i32; 109] = [
        4, 5, 6, 4, 7, 4, 4, 4, 8, 9,
        4, 4, 10, 4, 11, 4, 4, 4, 12, 4,
        13, 4, 4, 14, 4, 15, 16, 19, 21, 27,
        22, 42, 21, 23, 22, 42, 43, 23, 20, 23,
        20, 39, 30, 23, 30, 29, 38, 31, 36, 37,
        41, 31, 41, 31, 36, 42, 18, 18, 18, 18,
        18, 34, 18, 35, 35, 40, 40, 44, 44, 18,
        18, 33, 32, 29, 28, 17, 26, 25, 24, 17,
        45, 3, 45, 45, 45, 45, 45, 45, 45, 45,
        45, 45, 45, 45, 45, 45, 45, 45, 45,
        45, 45, 45, 45, 45, 45, 45, 45, 45, 45,
    ];

    /// yy_chk table (109 elements) - validates transitions: chk[base[state]+char_class] must equal state.
    pub const YY_CHK: [i32; 109] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 6, 8, 18,
        8, 42, 22, 8, 22, 41, 37, 22, 6, 8,
        18, 34, 23, 22, 23, 29, 33, 23, 29, 32,
        36, 31, 36, 30, 29, 36, 46, 46, 46, 46,
        47, 26, 47, 48, 48, 49, 49, 50, 50, 51,
        51, 25, 24, 21, 20, 17, 14, 13, 12, 5,
        3, 45, 45, 45, 45, 45, 45, 45, 45, 45,
        45, 45, 45, 45, 45, 45, 45, 45, 45, 45,
        45, 45, 45, 45, 45, 45, 45, 45, 45,
    ];
}

// ============================================================================
// action - executes the action for a given token type
// ============================================================================

/// Executes the action for a given act value.
/// Returns the token, start state, buffer result, and error tokens.
fn action(
    act: i32,
    start_st: i32,
    _mm_curr_st: i32,
    mm_pos: i32,
    mm_s_pos: i32,
    mm_e_pos: i32,
    mm_linenr: i32,
    line_nr_start: i32,
    buffer: i32,
    _debug: bool,
    file_nm: &str,
    file_contents: &str,
    mut in_error_tokens: List<Token>,
) -> (Token, i32, i32, List<Token>) {
    let mm_start_st = start_st;
    let buffer_ret = 0;

    let tok = match act {
        1 | 2 => {
            token(
                file_nm,
                TokenId::STRING,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        3 | 4 => {
            token(
                file_nm,
                TokenId::NUMBER,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        5 => {
            token(
                file_nm,
                TokenId::INTEGER,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        6 => {
            token(
                file_nm,
                TokenId::TRUE,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        7 => {
            token(
                file_nm,
                TokenId::FALSE,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        8 => {
            token(
                file_nm,
                TokenId::NULL,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        9 => {
            token(
                file_nm,
                TokenId::OBJECTBEGIN,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        10 => {
            token(
                file_nm,
                TokenId::OBJECTEND,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        11 => {
            token(
                file_nm,
                TokenId::ARRAYBEGIN,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        12 => {
            token(
                file_nm,
                TokenId::ARRAYEND,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        13 => {
            token(
                file_nm,
                TokenId::COMMA,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        14 => {
            token(
                file_nm,
                TokenId::COLON,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
        15 => {
            return (no_token(), mm_start_st, buffer_ret, in_error_tokens);
        }
        16 => {
            in_error_tokens.push_front(token(
                file_nm,
                TokenId::_NO_TOKEN,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            ));
            return (no_token(), mm_start_st, buffer_ret, in_error_tokens);
        }
        _ => {
            print(&format!("\nLexer unknown rule, action={}\n", act));
            token(
                file_nm,
                TokenId::_NO_TOKEN,
                file_contents,
                mm_pos - buffer,
                buffer,
                line_nr_start,
                mm_e_pos + 1,
                mm_linenr,
                mm_s_pos + 1,
            )
        }
    };

    // If act == 16 (error), push token to error_tokens
    if act == 16 {
        in_error_tokens.push_front(tok.clone());
    }

    (tok, mm_start_st, buffer_ret, in_error_tokens)
}

// ============================================================================
// evalState - evaluate DFA state transition
// ============================================================================

/// Evaluates the DFA state transition. Returns (new_state, new_c).
fn eval_state(mut c_state: i32, mut c: i32) -> (i32, i32) {
    // Use a visited set to prevent infinite loops from defective DFA tables.
    // YY_DEF can have cycles (e.g., YY_DEF[49] = 49).
    let mut max_iterations = 100i32;
    loop {
        if max_iterations <= 0 {
            break;
        }
        max_iterations -= 1;

        let orig_state = c_state;
        let base = lex_table::YY_BASE[orig_state as usize];
        let chk = base + c;
        if chk < 0 || chk >= lex_table::YY_NXT.len() as i32 {
            // Out of bounds - follow default
            c_state = lex_table::YY_DEF[orig_state as usize];
            if c_state >= lex_table::YY_LIMIT && c >= 0 && c < lex_table::YY_META.len() as i32 {
                c = lex_table::YY_META[c as usize];
            }
            if c_state > 0 && c_state != orig_state {
                let (ns, nc) = eval_state(c_state, c);
                c_state = ns;
                c = nc;
            } else {
                break;
            }
            continue;
        }
        let val = lex_table::YY_CHK[chk as usize];

        if orig_state != val {
            let def_state = lex_table::YY_DEF[orig_state as usize];
            // Prevent infinite loops: if def leads back to same state, stop
            // This handles states with base=YY_FINISH where chk doesn't match.
            if def_state == c_state {
                break;
            }
            // Also break if def_state equals orig_state (self-loop in default).
            if def_state == orig_state {
                break;
            }
            c_state = def_state;
            if c_state >= lex_table::YY_LIMIT && c >= 0 && c < lex_table::YY_META.len() as i32 {
                c = lex_table::YY_META[c as usize];
            }
            if c_state > 0 {
                let (ns, nc) = eval_state(c_state, c);
                c_state = ns;
                c = nc;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    (c_state, c)
}

// ============================================================================
// findRule - backtrack to find the longest match
// ============================================================================

/// Backtracks through states to find the longest matching rule.
/// Matches the C code from boot/build/LexerJSON.c findRule function.
fn find_rule(
    file_contents: &str,
    curr_st: i32,
    mut pos: i32,
    mut s_pos: i32,
    e_pos: i32,
    mut linenr: i32,
    mut buffer: i32,
    mut bk_buffer: i32,
    mut states: List<i32>,
) -> (i32, i32, i32, i32, i32, i32, i32, List<i32>) {
    // Safety: stop backtracking if we've consumed all characters
    // to prevent infinite recursion.
    if buffer <= 0 {
        return (0, curr_st, pos, s_pos, linenr, buffer, bk_buffer, states);
    }

    // Iteratively process states from the stack to find longest match.
    // Matches the C code findRule loop.
    let mut act: i32 = 0;
    let mut lp1: i32 = 0;
    let mut tmp_states = states;

    while let Some(st) = tmp_states.front().copied() {
        let lp = lex_table::YY_ACCEPT[st as usize];
        if lp > 0 && lp < lp1 {
            act = lex_table::YY_ACCLIST[lp as usize];
        }
        lp1 = st;
        tmp_states.pop_front();
    }

    if lp1 > 0 {
        act = lex_table::YY_ACCLIST[lp1 as usize];
    } else if buffer <= 0 {
        return (0, curr_st, pos, s_pos, linenr, buffer, bk_buffer, tmp_states);
    }

    (act, curr_st, pos, s_pos, linenr, buffer, bk_buffer, tmp_states)
}

// ============================================================================
// consume - process one character through the lexer DFA
// ============================================================================

/// Consumes one character code through the lexer DFA.
/// Returns (tokens, bk_buffer, start_st, curr_st, pos, s_pos, e_pos, linenr,
///         line_nr_start, buffer, states, error_tokens).
#[allow(clippy::too_many_arguments)]
fn consume(
    cp: i32,
    mut tokens: List<Token>,
    file_contents: &str,
    start_st: i32,
    curr_st: i32,
    pos: i32,
    s_pos: i32,
    e_pos: i32,
    linenr: i32,
    line_nr_start: i32,
    mut buffer: i32,
    mut states: List<i32>,
    file_name: &str,
    error_tokens: List<Token>,
) -> (List<Token>, i32, i32, i32, i32, i32, i32, i32, i32, i32, List<i32>, List<Token>) {
    let mut mm_start_st = start_st;

    let mut mm_curr_st = curr_st;

    let mut mm_pos = pos;
    let mut mm_s_pos = s_pos;
    let mut mm_e_pos = e_pos;
    let mut mm_linenr = linenr;

    buffer += 1;
    mm_pos += 1;

    if cp == 10 {
        mm_linenr += 1;
        mm_s_pos = 0;
    } else {
        mm_s_pos += 1;
    }

    let mut c = lex_table::YY_EC[cp as usize];

    let (new_st, new_c) = eval_state(mm_curr_st, c);
    mm_curr_st = new_st;
    c = new_c;

    // Nxt lookup - matches C code structure (no fallback)
    if mm_curr_st > 0 {
        let base = lex_table::YY_BASE[mm_curr_st as usize];
        mm_curr_st = lex_table::YY_NXT[base as usize + c as usize];
    } else {
        mm_curr_st = lex_table::YY_NXT[c as usize];
    }

    // Always push state to states stack (matches C code)
    states.push_front(mm_curr_st);

    let base_cond = lex_table::YY_BASE[mm_curr_st as usize];

    if base_cond == lex_table::YY_FINISH {
        // Handle finish state - backtrack to find longest match
        let (act, mm_curr_st, mm_pos, mm_s_pos, mm_linenr, mut buffer, bk_buffer, _states) =
            find_rule(
                file_contents,
                mm_curr_st,
                mm_pos,
                mm_s_pos,
                mm_e_pos,
                mm_linenr,
                buffer,
                0,
                states,
            );

        let (tok, mm_start_st, buffer2, error_tokens) = action(
            act,
            mm_start_st,
            mm_curr_st,
            mm_pos,
            mm_s_pos,
            mm_e_pos,
            mm_linenr,
            line_nr_start,
            buffer,
            false,
            file_name,
            file_contents,
            error_tokens,
        );

        let mm_curr_st = mm_start_st;
        let states = List::new();

        if buffer != buffer2 {
            mm_e_pos = mm_s_pos;
        }
        buffer = buffer2;

        let res_token = match tok.id {
            TokenId::_NO_TOKEN => tokens,
            _ => {
                tokens.push_front(tok);
                tokens
            }
        };

        (
            res_token,
            bk_buffer,
            mm_start_st,
            mm_curr_st,
            mm_pos,
            mm_s_pos,
            mm_e_pos,
            mm_linenr,
            line_nr_start,
            buffer,
            states,
            error_tokens,
        )
    } else {
        let bk_buffer = 0;
        (
            tokens,
            bk_buffer,
            mm_start_st,
            mm_curr_st,
            mm_pos,
            mm_s_pos,
            mm_e_pos,
            mm_linenr,
            line_nr_start,
            buffer,
            states,
            error_tokens,
        )
    }
}

// ============================================================================
// lex - main lexer function
// ============================================================================

/// Lex the given source code content. Returns (tokens, error_tokens).
pub fn lex(file_name: &str, contents: &str) -> (List<Token>, List<Token>) {
    let content_len = contents.len() as i32;
    let mut i = 1;
    let mut tokens: List<Token> = List::new();
    let mut error_tokens: List<Token> = List::new();

    // Initialize state
    let mut start_st = 1;
    let mut curr_st = 1;
    let mut pos = 1;
    let mut s_pos = 0;
    let mut e_pos = 0;
    let mut linenr = 1;
    let mut line_nr_start = 1;
    let mut buffer = 0;
    let mut states: List<i32> = List::new();

    while i <= content_len {
        let c_tok = string_get(contents, i);
        let (result_tokens, bk_buffer, new_start_st, new_curr_st, new_pos, new_s_pos, new_e_pos, new_linenr, new_line_nr_start, new_buffer, new_states, new_error_tokens) =
            consume(
                c_tok,
                tokens.clone(),
                contents,
                start_st,
                curr_st,
                pos,
                s_pos,
                e_pos,
                linenr,
                line_nr_start,
                buffer,
                states.clone(),
                file_name,
                error_tokens.clone(),
            );
        tokens = result_tokens;
        error_tokens = new_error_tokens;

        // i := i - numBacktrack + 1
        i = i - bk_buffer + 1;

        start_st = new_start_st;
        curr_st = new_curr_st;
        pos = new_pos;
        s_pos = new_s_pos;
        e_pos = new_e_pos;
        linenr = new_linenr;
        line_nr_start = new_line_nr_start;
        buffer = new_buffer;
        states = new_states;
    }

    tokens = list_reverse_in_place(tokens);
    error_tokens = list_reverse_in_place(error_tokens);

    (tokens, error_tokens)
}

// ============================================================================
// scan / scanString - public entry points
// ============================================================================

/// Scans a file for JSON tokens.
pub fn scan(file_name: &str) -> (List<Token>, List<Token>) {
    let contents = read_file(file_name);
    lex(file_name, &contents)
}

/// Scans a string source for JSON tokens.
pub fn scan_string(file_source: &str, file_name: &str) -> (List<Token>, List<Token>) {
    let name = if file_name.is_empty() {
        "<StringSource>".to_string()
    } else {
        file_name.to_string()
    };
    lex(&name, file_source)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_id_display() {
        assert_eq!(format!("{}", TokenId::STRING), "STRING");
        assert_eq!(format!("{}", TokenId::NUMBER), "NUMBER");
        assert_eq!(format!("{}", TokenId::TRUE), "TRUE");
        assert_eq!(format!("{}", TokenId::FALSE), "FALSE");
        assert_eq!(format!("{}", TokenId::NULL), "NULL");
        assert_eq!(format!("{}", TokenId::OBJECTBEGIN), "OBJECTBEGIN");
        assert_eq!(format!("{}", TokenId::OBJECTEND), "OBJECTEND");
        assert_eq!(format!("{}", TokenId::ARRAYBEGIN), "ARRAYBEGIN");
        assert_eq!(format!("{}", TokenId::ARRAYEND), "ARRAYEND");
        assert_eq!(format!("{}", TokenId::COMMA), "COMMA");
        assert_eq!(format!("{}", TokenId::COLON), "COLON");
        assert_eq!(format!("{}", TokenId::INTEGER), "INTEGER");
        assert_eq!(format!("{}", TokenId::_NO_TOKEN), "_NO_TOKEN");
    }

    #[test]
    fn test_no_token() {
        let nt = no_token();
        assert_eq!(nt.id, TokenId::_NO_TOKEN);
        assert_eq!(nt.file_name, "<NoFile>");
    }

    #[test]
    fn test_scan_simple_json() {
        let (tokens, _errors) = scan_string(r#"{"key": "value"}"#, "test.json");
        assert!(!tokens.is_empty(), "Expected tokens for valid JSON");
    }

    #[test]
    fn test_scan_number() {
        let (tokens, errors) = scan_string("123", "test.json");
        eprintln!("DEBUG: tokens count = {}, errors count = {}", tokens.len(), errors.len());
        for t in tokens.iter() {
            eprintln!("  token: id={}, content={:?}, len={}", t.id, token_content(t), t.length);
        }
        for e in errors.iter() {
            eprintln!("  error: content={:?}, len={}", token_content(e), e.length);
        }
        assert!(!tokens.is_empty(), "Expected tokens for number");
    }

    #[test]
    fn test_scan_string_token() {
        let (tokens, _errors) = scan_string(r#""hello""#, "test.json");
        assert!(!tokens.is_empty(), "Expected tokens for string");
    }

    #[test]
     fn test_scan_bool() {
        let (tokens, _errors) = scan_string("true", "test.json");
        assert!(!tokens.is_empty(), "Expected tokens for boolean");
        assert!(tokens.len() == 1, "Expected exactly 1 token for 'true'");
    }

    #[test]
    fn test_scan_null() {
        let (tokens, _errors) = scan_string("null", "test.json");
        assert!(!tokens.is_empty(), "Expected tokens for null");
    }

    #[test]
    fn test_token_content() {
        let tok = token(
            "test.json",
            TokenId::STRING,
            "hello world",
            0,
            5,
            1,
            1,
            1,
            5,
        );
        assert_eq!(token_content(&tok), "hello");
    }

    #[test]
    fn test_token_content_eq() {
        let tok1 = token("f", TokenId::STRING, "abc", 0, 3, 1, 1, 1, 3);
        let tok2 = token("f", TokenId::STRING, "abc", 0, 3, 1, 1, 1, 3);
        assert!(token_content_eq(&tok1, &tok2));

        let tok3 = token("f", TokenId::STRING, "xyz", 0, 3, 1, 1, 1, 3);
        assert!(!token_content_eq(&tok1, &tok3));
    }

    #[test]
    fn test_token_source_info() {
        let tok = token(
            "test.json",
            TokenId::STRING,
            "hello",
            0,
            5,
            1,
            1,
            2,
            5,
        );
        let info = token_source_info(&tok);
        assert_eq!(info.file_name, "test.json");
        assert_eq!(info.start_line, 1);
        assert_eq!(info.start_column, 1);
        assert_eq!(info.end_line, 2);
        assert_eq!(info.end_column, 5);
    }
}
