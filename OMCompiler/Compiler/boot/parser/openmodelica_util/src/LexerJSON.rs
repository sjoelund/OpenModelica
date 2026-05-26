// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::Error;
use crate::System;

/*
   Template for Lexer Code
   replace keywords:
   %LexerCode
   %time
   %Token
   %Lexer
   %LexTable
   %constant
   %nameSpan
   %functions
   %caseAction
  */
pub const debug: bool = false;

pub mod LexTable {
    use super::*;
    pub const yy_limit: i32 = 46;

    pub const yy_finish: i32 = 82;

    pub static yy_acclist: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![17, 16, 15, 16, 16, 13, 16, 5, 16, 14, 16, 11, 16, 12, 16, 16, 16, 16, 9, 16, 10, 16, 15, 1, 5, 2, 3, 4, 8, 6, 3, 7].into_iter().cloned().collect()) });

    pub static yy_accept: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![1, 1, 1, 2, 3, 5, 6, 8, 10, 12, 14, 16, 17, 18, 19, 21, 23, 24, 24, 25, 25, 25, 26, 26, 26, 26, 26, 27, 27, 28, 28, 29, 29, 29, 29, 29, 29, 29, 30, 31, 31, 31, 32, 33, 33, 33].into_iter().cloned().collect()) });

    pub static yy_ec: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 4, 5, 6, 7, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 10, 1, 1, 1, 1, 1, 1, 11, 11, 11, 11, 12, 11, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 13, 14, 15, 1, 1, 1, 16, 17, 11, 11, 18, 19, 1, 1, 1, 1, 1, 20, 1, 21, 1, 1, 1, 22, 23, 24, 25, 1, 1, 1, 1, 1, 26, 1, 27, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].into_iter().cloned().collect()) });

    pub static yy_meta: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![1, 1, 2, 1, 1, 1, 1, 2, 3, 1, 3, 3, 1, 2, 1, 3, 4, 3, 4, 1, 2, 2, 1, 2, 2, 1, 1].into_iter().cloned().collect()) });

    pub static yy_base: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![0, 0, 81, 82, 78, 25, 82, 22, 82, 82, 82, 63, 53, 55, 82, 82, 74, 27, 82, 50, 65, 26, 39, 53, 52, 37, 82, 0, 37, 45, 43, 27, 27, 24, 0, 47, 19, 82, 82, 0, 27, 23, 82, 0, 82, 56, 59, 61, 63, 65, 67].into_iter().cloned().collect()) });

    pub static yy_def: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![45, 1, 45, 45, 45, 46, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 46, 45, 47, 45, 45, 45, 45, 45, 45, 45, 48, 45, 45, 45, 45, 45, 45, 49, 45, 45, 45, 45, 50, 45, 45, 45, 51, 0, 45, 45, 45, 45, 45, 45].into_iter().cloned().collect()) });

    pub static yy_nxt: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![4, 5, 6, 4, 7, 4, 4, 4, 8, 9, 4, 4, 10, 4, 11, 4, 4, 4, 12, 4, 13, 4, 4, 14, 4, 15, 16, 19, 21, 27, 22, 42, 21, 23, 22, 42, 43, 23, 20, 23, 20, 39, 30, 23, 30, 29, 38, 31, 36, 37, 41, 31, 41, 31, 36, 42, 18, 18, 18, 18, 18, 34, 18, 35, 35, 40, 40, 44, 44, 18, 18, 33, 32, 29, 28, 17, 26, 25, 24, 17, 45, 3, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45].into_iter().cloned().collect()) });

    pub static yy_chk: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 6, 8, 18, 8, 42, 22, 8, 22, 41, 37, 22, 6, 8, 18, 34, 23, 22, 23, 29, 33, 23, 29, 32, 36, 31, 36, 30, 29, 36, 46, 46, 46, 46, 47, 26, 47, 48, 48, 49, 49, 50, 50, 51, 51, 25, 24, 21, 20, 17, 14, 13, 12, 5, 3, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45, 45].into_iter().cloned().collect()) });

}

pub fn scan(mut fileName: ArcStr) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Token>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut errorTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut contents: ArcStr = arcstr::literal!("");
    contents = (System::readFile((fileName.clone()).clone())?).clone();
    (tokens, errorTokens) = lex((fileName.clone()).clone(), (contents.clone()).clone())?;
    Ok((tokens, errorTokens))
}

pub fn scanString(mut fileSource: ArcStr, mut fileName: ArcStr) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Token>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut errorTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (tokens, errorTokens) = lex((fileName.clone()).clone(), (fileSource.clone()).clone())?;
    Ok((tokens, errorTokens))
}

/* grammar according to json.org */
pub fn action(mut act: i32, mut startSt: i32, mut mm_currSt: i32, mut mm_pos: i32, mut mm_sPos: i32, mut mm_ePos: i32, mut mm_linenr: i32, mut lineNrStart: i32, mut buffer: i32, mut fileNm: ArcStr, mut fileContents: ArcStr, mut inErrorTokens: Arc<metamodelica::List<Token>>) -> Result<(Token, i32, i32, Arc<metamodelica::List<Token>>)> {
    let mut token: Token;
    let mut mm_startSt: i32 = 0;
    let mut bufferRet: i32 = 0;
    let mut errorTokens: Arc<metamodelica::List<Token>> = inErrorTokens.clone();
    let mut info: SourceInfo;
    let mut sToken: ArcStr = arcstr::literal!("");
    mm_startSt = startSt.clone();
    bufferRet = 0;
    token = (match act.clone() {
        1 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::STRING.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        2 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::STRING.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        3 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::NUMBER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        4 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::NUMBER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        5 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::INTEGER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        6 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::TRUE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        7 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::FALSE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        8 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::NULL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        9 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::OBJECTBEGIN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        10 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::OBJECTEND.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        11 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ARRAYBEGIN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        12 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ARRAYEND.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        13 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::COMMA.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        14 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::COLON.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        15 => {
            noToken.clone()
        },
        16 => {
            let mut tok: Token;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::_NO_TOKEN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            errorTokens = cons(tok.clone(), errorTokens.clone());
            noToken.clone()
        },
        _ => {
            let mut tok: Token;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nLexer unknown rule, action=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", act.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::_NO_TOKEN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            println!("{}", (printToken(tok.clone())?).clone());
            bail!("fail")
        },
    });
    Ok((token, mm_startSt, bufferRet, errorTokens))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum TokenId {
    _NO_TOKEN = 1,
    ARRAYBEGIN = 2,
    ARRAYEND = 3,
    COLON = 4,
    COMMA = 5,
    FALSE = 6,
    INTEGER = 7,
    NULL = 8,
    NUMBER = 9,
    OBJECTBEGIN = 10,
    OBJECTEND = 11,
    STRING = 12,
    TRUE = 13,
}
impl PartialOrd for TokenId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for TokenId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub fileName: ArcStr,
    pub id: TokenId,
    pub fileContents: ArcStr,
    pub byteOffset: i32,
    pub length: i32,
    pub lineNumberStart: i32,
    pub columnNumberStart: i32,
    pub lineNumberEnd: i32,
    pub columnNumberEnd: i32,
}

pub type TOKEN = Token;


pub static noToken: std::sync::LazyLock<Token> = std::sync::LazyLock::new(|| { Token { fileName: (literal!("<NoFile>")).clone(), id: TokenId::_NO_TOKEN.clone(), fileContents: (literal!("")).clone(), byteOffset: 0, length: 0, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0 } });

pub fn printToken(mut token: Token) -> Result<ArcStr> {
    let mut strTk: ArcStr = arcstr::literal!("");
    let mut id: TokenId = TokenId::_NO_TOKEN;
    let mut contents: ArcStr = arcstr::literal!("");
    let mut byteOffset: i32 = 0;
    let mut length: i32 = 0;
    let Token { length: __pa0, byteOffset: __pa1, fileContents: __pa2, id: __pa3, .. } = (token.clone()) else { bail!("pattern mismatch") };
    length = __pa0.clone();
    byteOffset = __pa1.clone();
    contents = __pa2.clone();
    id = __pa3.clone();
    contents = (if (length.clone() > 0) {substring((contents.clone()).clone(), byteOffset.clone(), byteOffset.clone() + length.clone() - 1)?} else {literal!("")}).clone();
    strTk = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[TOKEN:")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", id.clone()))); __mm_s.push_str(&*literal!(" '")); __mm_s.push_str(&*contents.clone()); __mm_s.push_str(&*literal!("' (")); __mm_s.push_str(&*intString(token.lineNumberStart.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(token.columnNumberStart.clone())); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*intString(token.lineNumberEnd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(token.columnNumberEnd.clone())); __mm_s.push_str(&*literal!(")]")); ArcStr::from(__mm_s) }).clone();
    Ok(strTk)
}

pub fn tokenContent(mut token: Token) -> Result<ArcStr> {
    let mut contents: ArcStr = arcstr::literal!("");
    let mut byteOffset: i32 = 0;
    let mut length: i32 = 0;
    let Token { length: __pa0, byteOffset: __pa1, fileContents: __pa2, .. } = (token.clone()) else { bail!("pattern mismatch") };
    length = __pa0.clone();
    byteOffset = __pa1.clone();
    contents = __pa2.clone();
    contents = (if (length.clone() > 0) {substring((contents.clone()).clone(), byteOffset.clone(), byteOffset.clone() + length.clone() - 1)?} else {literal!("")}).clone();
    Ok(contents)
}

pub fn tokenContentEq(mut token1: Token, mut token2: Token) -> Result<bool> {
    let mut b: bool = false;
    let mut contents1: ArcStr = arcstr::literal!("");
    let mut contents2: ArcStr = arcstr::literal!("");
    let mut offset1: i32 = 0;
    let mut length1: i32 = 0;
    let mut offset2: i32 = 0;
    let mut length2: i32 = 0;
    let Token { length: __pa0, byteOffset: __pa1, fileContents: __pa2, .. } = (token1.clone()) else { bail!("pattern mismatch") };
    length1 = __pa0.clone();
    offset1 = __pa1.clone();
    contents1 = __pa2.clone();
    let Token { length: __pa3, byteOffset: __pa4, fileContents: __pa5, .. } = (token2.clone()) else { bail!("pattern mismatch") };
    length2 = __pa3.clone();
    offset2 = __pa4.clone();
    contents2 = __pa5.clone();
    b = if (length1.clone() != length2.clone()) {false} else {0 == System::strcmp_offset((contents1.clone()).clone(), offset1.clone(), length1.clone(), (contents2.clone()).clone(), offset2.clone(), length2.clone())};
    Ok(b)
}

pub fn tokenSourceInfo(mut token: Token) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = { let t = token.clone(); (match t.clone() {
        Token { .. } => SourceInfo { fileName: (t.fileName.clone()).clone(), isReadOnly: false, lineNumberStart: t.lineNumberStart.clone(), columnNumberStart: t.columnNumberStart.clone(), lineNumberEnd: t.lineNumberEnd.clone(), columnNumberEnd: t.columnNumberEnd.clone(), lastModification: metamodelica::OrderedFloat(0.0_f64) },
        _ => bail!("match: no arm matched"),
    }) };
    Ok(info)
}

fn lex(mut fileName: ArcStr, mut contents: ArcStr) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Token>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut errorTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut startSt: i32 = 0;
    let mut i: i32 = 0;
    let mut r: i32 = 0;
    let mut cTok: i32 = 0;
    let mut cTok2: i32 = 0;
    let mut currSt: i32 = 0;
    let mut pos: i32 = 0;
    let mut sPos: i32 = 0;
    let mut ePos: i32 = 0;
    let mut linenr: i32 = 0;
    let mut contentLen: i32 = 0;
    let mut numBacktrack: i32 = 0;
    let mut buffer: i32 = 0;
    let mut lineNrStart: i32 = 0;
    let mut cProg: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cProg2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut states: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    startSt = 1;
    currSt = 1;
    pos = 1;
    sPos = 0;
    ePos = 0;
    linenr = 1;
    lineNrStart = 1;
    buffer = 0;
    states = metamodelica::nil();
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nLexer analyzer LexerCode...")); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    tokens = metamodelica::nil();
    if debug.clone() {
        println!("{}", (literal!("\n TOTAL Chars:")).clone());
        println!("{}", (intString(((contents.clone()).clone().len() as i32))).clone());
    }
    contentLen = ((contents.clone()).clone().len() as i32);
    i = 1;
    while i.clone() <= contentLen.clone() {
        cTok = stringGet((contents.clone()).clone(),i.clone())?;
        (tokens, numBacktrack, startSt, currSt, pos, sPos, ePos, linenr, lineNrStart, buffer, states, errorTokens) = consume(cTok.clone(), tokens.clone(), (contents.clone()).clone(), startSt.clone(), currSt.clone(), pos.clone(), sPos.clone(), ePos.clone(), linenr.clone(), lineNrStart.clone(), buffer.clone(), states.clone(), (fileName.clone()).clone(), errorTokens.clone())?;
        i = i.clone() - numBacktrack.clone() + 1;
    }
    tokens = tokens.clone().reverse();
    errorTokens = errorTokens.clone().reverse();
    Ok((tokens, errorTokens))
}

fn consume(mut cp: i32, mut tokens: Arc<metamodelica::List<Token>>, mut fileContents: ArcStr, mut startSt: i32, mut currSt: i32, mut pos: i32, mut sPos: i32, mut ePos: i32, mut linenr: i32, mut inLineNrStart: i32, mut inBuffer: i32, mut inStates: Arc<metamodelica::List<i32>>, mut fileName: ArcStr, mut inErrorTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<metamodelica::List<Token>>, i32, i32, i32, i32, i32, i32, i32, i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Token>>)> {
    let mut resToken: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut bkBuffer: i32 = 0;
    let mut mm_startSt: i32 = 0;
    let mut mm_currSt: i32 = 0;
    let mut mm_pos: i32 = 0;
    let mut mm_sPos: i32 = 0;
    let mut mm_ePos: i32 = 0;
    let mut mm_linenr: i32 = 0;
    let mut lineNrStart: i32 = 0;
    let mut buffer: i32 = 0;
    let mut states: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut errorTokens: Arc<metamodelica::List<Token>> = inErrorTokens.clone();
    let mut tok: Token;
    let mut act: i32 = 0;
    let mut buffer2: i32 = 0;
    let mut c: i32 = 0;
    let mut baseCond: i32 = 0;
    mm_startSt = startSt.clone();
    mm_currSt = currSt.clone();
    mm_pos = pos.clone();
    mm_sPos = sPos.clone();
    mm_ePos = ePos.clone();
    mm_linenr = linenr.clone();
    lineNrStart = inLineNrStart.clone();
    buffer = inBuffer.clone();
    states = inStates.clone();
    baseCond = LexTable::yy_base.borrow()[(mm_currSt.clone()-1) as usize].clone();
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPROGRAM:{")); __mm_s.push_str(&*intString(cp.clone())); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBUFFER:{")); __mm_s.push_str(&*intString(buffer.clone())); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("base:")); __mm_s.push_str(&*intString(baseCond.clone())); __mm_s.push_str(&*literal!(" st:")); __mm_s.push_str(&*intString(mm_currSt.clone())); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone());
    }
    buffer = buffer.clone() + 1;
    mm_pos = mm_pos.clone() + 1;
    if cp.clone() == 10 {
        mm_linenr = mm_linenr.clone() + 1;
        mm_sPos = 0;
    } else {
        mm_sPos = mm_sPos.clone() + 1;
    }
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[Reading:'")); __mm_s.push_str(&*intStringChar(cp.clone())); __mm_s.push_str(&*literal!("' at p:")); __mm_s.push_str(&*intString(mm_pos.clone() - 1)); __mm_s.push_str(&*literal!(" line:")); __mm_s.push_str(&*intString(mm_linenr.clone())); __mm_s.push_str(&*literal!(" rPos:")); __mm_s.push_str(&*intString(mm_sPos.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone());
    }
    c = LexTable::yy_ec.borrow()[(cp.clone()-1) as usize].clone();
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" evalState Before[c")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!(",s")); __mm_s.push_str(&*intString(mm_currSt.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone());
    }
    (mm_currSt, c) = evalState(mm_currSt.clone(), c.clone());
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" After[c")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!(",s")); __mm_s.push_str(&*intString(mm_currSt.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone());
    }
    if mm_currSt.clone() > 0 {
        mm_currSt = LexTable::yy_base.borrow()[(mm_currSt.clone()-1) as usize].clone();
        mm_currSt = LexTable::yy_nxt.borrow()[(mm_currSt.clone() + c.clone()-1) as usize].clone();
    } else {
        mm_currSt = LexTable::yy_nxt.borrow()[(c.clone()-1) as usize].clone();
    }
    states = cons(mm_currSt.clone(), states.clone());
    baseCond = LexTable::yy_base.borrow()[(mm_currSt.clone()-1) as usize].clone();
    if baseCond.clone() == LexTable::yy_finish.clone() {
        if debug.clone() == true {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[RESTORE=")); __mm_s.push_str(&*intString(LexTable::yy_accept.borrow()[(mm_currSt.clone()-1) as usize].clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone());
        }
        (act, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states) = findRule((fileContents.clone()).clone(), mm_currSt.clone(), mm_pos.clone(), mm_sPos.clone(), mm_ePos.clone(), mm_linenr.clone(), buffer.clone(), bkBuffer.clone(), states.clone())?;
        if debug.clone() == true {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFound rule: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", act.clone()))); ArcStr::from(__mm_s) }).clone());
        }
        (tok, mm_startSt, buffer2, errorTokens) = action(act.clone(), mm_startSt.clone(), mm_currSt.clone(), mm_pos.clone(), mm_sPos.clone(), mm_ePos.clone(), mm_linenr.clone(), lineNrStart.clone(), buffer.clone(), (fileName.clone()).clone(), (fileContents.clone()).clone(), errorTokens.clone())?;
        if debug.clone() == true {
            println!("{}", (literal!("\nDid action")).clone());
        }
        mm_currSt = mm_startSt.clone();
        states = metamodelica::nil();
        if buffer.clone() != buffer2.clone() {
            mm_ePos = mm_sPos.clone();
            lineNrStart = linenr.clone();
        }
        buffer = buffer2.clone();
        resToken = (match tok.clone() {
        Token { id: TokenId::_NO_TOKEN, .. } => tokens.clone(),
        _ => cons(tok.clone(), tokens.clone()),
    });
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n CountTokens:")); __mm_s.push_str(&*intString((resToken.clone().len() as i32))); ArcStr::from(__mm_s) }).clone());
        }
    } else {
        bkBuffer = 0;
        resToken = tokens.clone();
    }
    Ok((resToken, bkBuffer, mm_startSt, mm_currSt, mm_pos, mm_sPos, mm_ePos, mm_linenr, lineNrStart, buffer, states, errorTokens))
}

fn findRule(mut fileContents: ArcStr, mut currSt: i32, mut pos: i32, mut sPos: i32, mut mm_ePos: i32, mut linenr: i32, mut inBuffer: i32, mut inBkBuffer: i32, mut inStates: Arc<metamodelica::List<i32>>) -> Result<(i32, i32, i32, i32, i32, i32, i32, Arc<metamodelica::List<i32>>)> {
    let mut action: i32 = 0;
    let mut mm_currSt: i32 = 0;
    let mut mm_pos: i32 = 0;
    let mut mm_sPos: i32 = 0;
    let mut mm_linenr: i32 = 0;
    let mut buffer: i32 = 0;
    let mut bkBuffer: i32 = 0;
    let mut states: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mm_accept: metamodelica::Array<i32>;
    let mut mm_ec: metamodelica::Array<i32>;
    let mut mm_meta: metamodelica::Array<i32>;
    let mut mm_base: metamodelica::Array<i32>;
    let mut mm_def: metamodelica::Array<i32>;
    let mut mm_nxt: metamodelica::Array<i32>;
    let mut mm_chk: metamodelica::Array<i32>;
    let mut mm_acclist: metamodelica::Array<i32>;
    let mut lp: i32 = 0;
    let mut lp1: i32 = 0;
    let mut stCmp: i32 = 0;
    let mut cp: i32 = 0;
    let mut st: bool = false;
    mm_currSt = currSt.clone();
    mm_pos = pos.clone();
    mm_sPos = sPos.clone();
    mm_linenr = linenr.clone();
    buffer = inBuffer.clone();
    bkBuffer = inBkBuffer.clone();
    states = inStates.clone();
    stCmp = (states.clone()).get(1)?;
    lp = LexTable::yy_accept.borrow()[(stCmp.clone()-1) as usize].clone();
    lp1 = LexTable::yy_accept.borrow()[(stCmp.clone() + 1-1) as usize].clone();
    st = intGt(lp.clone(), 0) && intLt(lp.clone(), lp1.clone());
    if st.clone() {
        if debug.clone() {
            checkArrayModelica(LexTable::yy_accept.clone(), stCmp.clone(), metamodelica::sourceInfo!())?;
            checkArrayModelica(LexTable::yy_acclist.clone(), lp.clone(), metamodelica::sourceInfo!())?;
        }
        lp = LexTable::yy_accept.borrow()[(stCmp.clone()-1) as usize].clone();
        action = LexTable::yy_acclist.borrow()[(lp.clone()-1) as usize].clone();
    } else {
        cp = stringGet((fileContents.clone()).clone(),mm_pos.clone() - 1)?;
        buffer = buffer.clone() - 1;
        bkBuffer = bkBuffer.clone() + 1;
        mm_pos = mm_pos.clone() - 1;
        mm_sPos = mm_sPos.clone() - 1;
        if cp.clone() == 10 {
            mm_sPos = mm_ePos.clone();
            mm_linenr = mm_linenr.clone() - 1;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(states.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        mm_currSt = __pa0.clone();
        states = __pa1.clone();
        (action, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states) = findRule((fileContents.clone()).clone(), mm_currSt.clone(), mm_pos.clone(), mm_sPos.clone(), mm_ePos.clone(), mm_linenr.clone(), buffer.clone(), bkBuffer.clone(), states.clone())?;
    }
    Ok((action, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states))
}

fn evalState(mut cState: i32, mut c: i32) -> (i32, i32) {
    let mut new_state: i32 = 0;
    let mut new_c: i32 = 0;
    let mut cState1: i32 = cState.clone();
    let mut c1: i32 = c.clone();
    let mut val: i32 = 0;
    let mut val2: i32 = 0;
    let mut chk: i32 = 0;
    chk = LexTable::yy_base.borrow()[(cState1.clone()-1) as usize].clone();
    chk = chk.clone() + c1.clone();
    val = LexTable::yy_chk.borrow()[(chk.clone()-1) as usize].clone();
    val2 = LexTable::yy_base.borrow()[(cState1.clone()-1) as usize].clone() + c1.clone();
    if cState1.clone() != val.clone() {
        cState1 = LexTable::yy_def.borrow()[(cState1.clone()-1) as usize].clone();
        if cState1.clone() >= LexTable::yy_limit.clone() {
            c1 = LexTable::yy_meta.borrow()[(c1.clone()-1) as usize].clone();
        }
        if cState1.clone() > 0 {
            (cState1, c1) = evalState(cState1.clone(), c1.clone());
        }
    }
    new_state = cState1.clone();
    new_c = c1.clone();
    (new_state, new_c)
}

fn checkArray<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let mut filename: ArcStr = arcstr::literal!("");
    let mut lineStart: i32 = 0;
    if index.clone() < 1 || index.clone() > (arr.clone().borrow().len() as i32) {
        let SourceInfo { lineNumberStart: __pa0, fileName: __pa1, .. } = (info.clone()) else { bail!("pattern mismatch") };
        lineStart = __pa0.clone();
        filename = __pa1.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", lineStart.clone()))); __mm_s.push_str(&*literal!("]: checkArray failed: arrayLength=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (arr.clone().borrow().len() as i32)))); __mm_s.push_str(&*literal!(" index=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        bail!("fail");
    }
    Ok(())
}

fn checkArrayModelica(mut arr: metamodelica::Array<i32>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let mut filename: ArcStr = arcstr::literal!("");
    let mut lineStart: i32 = 0;
    if index.clone() < 1 || index.clone() > (arr.clone().borrow().len() as i32) {
        let SourceInfo { lineNumberStart: __pa0, fileName: __pa1, .. } = (info.clone()) else { bail!("pattern mismatch") };
        lineStart = __pa0.clone();
        filename = __pa1.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", lineStart.clone()))); __mm_s.push_str(&*literal!("]: checkArray failed: arrayLength=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (arr.clone().borrow().len() as i32)))); __mm_s.push_str(&*literal!(" index=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        bail!("fail");
    }
    Ok(())
}

