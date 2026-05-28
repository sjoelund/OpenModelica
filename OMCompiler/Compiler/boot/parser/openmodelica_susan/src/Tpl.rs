// Auto-generated from MetaModelica source
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
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// indentation will be implemented through spaces
// where tabs will be converted where 1 tab = 4 spaces ??
pub type Tokens = Arc<metamodelica::List<Arc<StringToken>>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Text {
    MEM_TEXT {
        tokens: Tokens,
        blocksStack: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<StringToken>>>, Arc<BlockType>)>>,
    },
    FILE_TEXT {
        opaqueFile: Option<i32>,
        nchars: Mutable::Mutable<i32>,
        aind: Mutable::Mutable<i32>,
        isstart: Mutable::Mutable<bool>,
        blocksStack: Mutable::Mutable<Arc<metamodelica::List<BlockTypeFileText>>>,
    },
}
pub use self::Text::{MEM_TEXT,FILE_TEXT};

pub static emptyTxt: std::sync::LazyLock<Text> = std::sync::LazyLock::new(|| { Text::MEM_TEXT { tokens: metamodelica::nil(), blocksStack: metamodelica::nil() } });

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockTypeFileText {
    /// The block type
    pub bt: Arc<BlockType>,
    pub nchars: i32,
    pub aind: i32,
    pub isstart: bool,
    /// Usage depends on bt; stores the last file position to know if it is empty or not.
    pub tell: Mutable::Mutable<i32>,
    pub septok: Mutable::Mutable<Option<Arc<StringToken>>>,
}

pub type BT_FILE_TEXT = BlockTypeFileText;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StringToken {
    /// Always outputs the new-line char.
    ST_NEW_LINE,
    /// A string without new-lines in it.
    ST_STRING {
        value: ArcStr,
    },
    /// A (non-empty) string with new-line at the end.
    ST_LINE {
        line: ArcStr,
    },
    /// Every string in the list can have a new-line at its end (but does not have to).
    ST_STRING_LIST {
        strList: Arc<metamodelica::List<ArcStr>>,
        /// True when the last string in the list has new-line at the end.
        lastHasNewLine: bool,
    },
    ST_BLOCK {
        tokens: Tokens,
        blockType: Arc<BlockType>,
    },
}
impl Default for StringToken {
    fn default() -> Self { Self::ST_NEW_LINE }
}
pub use self::StringToken::{ST_NEW_LINE,ST_STRING,ST_LINE,ST_STRING_LIST,ST_BLOCK};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockType {
    BT_TEXT,
    BT_INDENT {
        width: i32,
    },
    BT_ABS_INDENT {
        width: i32,
    },
    BT_REL_INDENT {
        offset: i32,
    },
    BT_ANCHOR {
        offset: i32,
    },
    /// Iteration items block, every token in the block is an item.
    ///                index0 is the active index during the build phase, then it is the last one + 1.
    BT_ITER {
        options: Arc<IterOptions>,
        index0: Mutable::Mutable<i32>,
    },
}
pub use self::BlockType::{BT_TEXT,BT_INDENT,BT_ABS_INDENT,BT_REL_INDENT,BT_ANCHOR,BT_ITER};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IterOptions {
    pub startIndex0: i32,
    pub empty: Option<Arc<StringToken>>,
    pub separator: Option<Arc<StringToken>>,
    /// Number of items to be aligned by. When 0, no alignment.
    pub alignNum: i32,
    pub alignOfset: i32,
    pub alignSeparator: Arc<StringToken>,
    /// Number of chars on a line, after that the wrapping can occur. When 0, no wrapping.
    pub wrapWidth: i32,
    pub wrapSeparator: Arc<StringToken>,
}

impl Default for IterOptions {
    fn default() -> Self {
        Self {
            startIndex0: Default::default(),
            empty: Default::default(),
            separator: Default::default(),
            alignNum: Default::default(),
            alignOfset: Default::default(),
            alignSeparator: Default::default(),
            wrapWidth: Default::default(),
            wrapSeparator: Default::default(),
        }
    }
}

pub type ITER_OPTIONS = IterOptions;


//by default, we will parse new lines in every non-token string
pub fn writeStr(mut inText: Text, mut inStr: ArcStr) -> Result<Text> {
    let mut outText: Text;
    outText = (::match_deref::match_deref! { match &((inText.clone(), inStr.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (Text::MEM_TEXT { blocksStack: blstack, tokens: toks }, r#str) if (-1 == System::stringFind((r#str.clone()).clone(), (literal!("\n")).clone())?) => {
            Text::MEM_TEXT { tokens: cons(Arc::new(StringToken::ST_STRING { value: (r#str.clone()).clone() }), toks.clone()), blocksStack: blstack.clone() }
        },
        (Text::FILE_TEXT { .. }, r#str) if (-1 == System::stringFind((r#str.clone()).clone(), (literal!("\n")).clone())?) => {
            stringFile(inText.clone(), (r#str.clone()).clone(), false, true)?;
            inText.clone()
        },
        _ => {
            writeChars(inText.clone(), System::splitOnNewline((inStr.clone()).clone(), true)?)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outText)
}

pub fn writeTok(mut inText: Text, mut inToken: Arc<StringToken>) -> Result<Text> {
    let mut outText: Text;
    outText = (::match_deref::match_deref! { match &((inText.clone(), inToken.clone())) {
        (txt, Deref @ StringToken::ST_BLOCK { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, Deref @ StringToken::ST_STRING { value: Deref @ "" }) => {
            txt.clone()
        },
        (Text::MEM_TEXT { blocksStack: blstack, tokens: toks }, tok) => {
            Text::MEM_TEXT { tokens: cons(tok.clone(), toks.clone()), blocksStack: blstack.clone() }
        },
        (Text::FILE_TEXT { .. }, tok) => {
            tokFileText(inText.clone(), tok.clone(), true)?;
            inText.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outText)
}

pub fn writeText(mut inText: Text, mut inTextToWrite: Text) -> Result<Text> {
    let mut outText: Text;
    outText = (::match_deref::match_deref! { match &((inText.clone(), inTextToWrite.clone())) {
        (txt, Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (Text::MEM_TEXT { blocksStack: blstack, tokens: toks }, Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Nil, tokens: txttoks }) => {
            Text::MEM_TEXT { tokens: cons(Arc::new(StringToken::ST_BLOCK { tokens: txttoks.clone(), blockType: Arc::new(crate::Tpl::BlockType::BT_TEXT) }), toks.clone()), blocksStack: blstack.clone() }
        },
        (Text::FILE_TEXT { .. }, Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Nil, tokens: txttoks }) => {
            for mut tok in &*txttoks.clone().reverse() {
                let mut tok = tok.clone();
                writeTok(inText.clone(), tok.clone())?;
            }
            inText.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.writeText failed - incomplete text was passed to be written\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outText)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn writeChars(mut inText: Text, mut inChars: Arc<metamodelica::List<ArcStr>>) -> Result<Text> {
    let mut outText: Text;
    outText = (::match_deref::match_deref! { match &((inText.clone(), inChars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ "\n", tail: chars }) => {
            let mut txt = (*txt).clone();
            txt = newLine(txt.clone())?;
            writeChars(txt.clone(), chars.clone())?
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ "\r\n", tail: chars }) => {
            let mut txt = (*txt).clone();
            txt = newLine(txt.clone())?;
            writeChars(txt.clone(), chars.clone())?
        },
        (txt, Deref @ metamodelica::List::Cons { head: c, tail: chars }) => {
            let mut lschars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut isline: bool = false;
            let mut txt = (*txt).clone();
            let mut chars = (*chars).clone();
            (lschars, chars, isline) = takeLineOrString(chars.clone());
            txt = writeLineOrStr(txt.clone(), stringAppendList(cons((c.clone()).clone(), lschars.clone())), isline.clone())?;
            writeChars(txt.clone(), chars.clone())?
        },
        (_, _) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.writeChars failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outText)
}

fn writeLineOrStr(mut inText: Text, mut inStr: ArcStr, mut inIsLine: bool) -> Result<Text> {
    let mut outText: Text;
    outText = (::match_deref::match_deref! { match &((inText.clone(), inStr.clone(), inIsLine.clone())) {
        (txt, Deref @ "", _) => {
            txt.clone()
        },
        (Text::MEM_TEXT { blocksStack: blstack, tokens: toks }, r#str, false) => {
            Text::MEM_TEXT { tokens: cons(Arc::new(StringToken::ST_STRING { value: (r#str.clone()).clone() }), toks.clone()), blocksStack: blstack.clone() }
        },
        (Text::MEM_TEXT { blocksStack: blstack, tokens: toks }, r#str, true) => {
            Text::MEM_TEXT { tokens: cons(Arc::new(StringToken::ST_LINE { line: (r#str.clone()).clone() }), toks.clone()), blocksStack: blstack.clone() }
        },
        (Text::FILE_TEXT { .. }, r#str, _) => {
            stringFile(inText.clone(), (r#str.clone()).clone(), inIsLine.clone(), true)?;
            inText.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outText)
}

fn takeLineOrString(mut inChars: Arc<metamodelica::List<ArcStr>>) -> (Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, bool) {
    let mut outTillNewLineChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outRestChars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outIsLine: bool = false;
    (outTillNewLineChars, outRestChars, outIsLine) = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil(), false)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ "\n", tail: chars } => {
            (list![(literal!("\n")).clone()], chars.clone(), true)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ "\r\n", tail: chars } => {
            (list![(literal!("\n")).clone()], chars.clone(), true)
        },
        Deref @ metamodelica::List::Cons { head: char, tail: chars } => {
            let mut tnlchars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut restchars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut isline: bool = false;
            (tnlchars, restchars, isline) = takeLineOrString(chars.clone());
            (cons((char.clone()).clone(), tnlchars.clone()), restchars.clone(), isline.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outTillNewLineChars, outRestChars, outIsLine)
}

pub fn softNewLine(mut inText: Text) -> Result<Text> {
    let mut outText: Text;
    outText = (::match_deref::match_deref! { match &(inText.clone()) {
        txt @ Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. } => {
            txt.clone()
        },
        txt @ Text::MEM_TEXT { tokens: toks, .. } => {
            let mut txt = (*txt).clone();
            if !(isAtStartOfLine(txt.clone())?) {
                let __owned_variant_tokens_0 = cons(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), toks.clone());
                if let Text::MEM_TEXT { tokens, .. } = &mut txt {
                    *tokens = __owned_variant_tokens_0;
                } else { panic!("owned-variant field-assign: value held a different variant than Text::MEM_TEXT"); }
            }
            txt.clone()
        },
        Text::FILE_TEXT { .. } => {
            if !(isAtStartOfLine(inText.clone())?) {
                newlineFile(inText.clone())?;
            }
            inText.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.softNL failed. \n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outText)
}

fn isAtStartOfLine(mut text: Text) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(text.clone()) {
        Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Cons { head: tok, tail: _ }, .. } => {
            isAtStartOfLineTok(tok.clone())
        },
        Text::FILE_TEXT { .. } => {
            Mutable::access(var_field!(text.isstart, Text::FILE_TEXT).clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isAtStartOfLineTok(mut inTok: Arc<StringToken>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(inTok.clone()) {
        Deref @ StringToken::ST_NEW_LINE { .. } => {
            true
        },
        Deref @ StringToken::ST_LINE { .. } => {
            true
        },
        Deref @ StringToken::ST_STRING_LIST { lastHasNewLine: true, .. } => {
            true
        },
        Deref @ StringToken::ST_BLOCK { tokens: Deref @ metamodelica::List::Cons { head: tok, tail: _ }, .. } => {
            isAtStartOfLineTok(tok.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn newLine(mut inText: Text) -> Result<Text> {
    let mut outText: Text;
    outText = (match inText.clone() {
        Text::MEM_TEXT { blocksStack: ref blstack, tokens: ref toks } => {
            Text::MEM_TEXT { tokens: cons(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), toks.clone()), blocksStack: blstack.clone() }
        },
        Text::FILE_TEXT { .. } => {
            newlineFile(inText.clone())?;
            inText.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outText)
}

pub fn pushBlock(mut txt: Text, mut inBlockType: Arc<BlockType>) -> Result<Text> {
    let mut txt: Text = txt;
    txt = (match txt.clone() {
        Text::MEM_TEXT { blocksStack: ref blstack, tokens: ref toks } => {
            Text::MEM_TEXT { tokens: metamodelica::nil(), blocksStack: cons((toks.clone(), inBlockType.clone()), blstack.clone()) }
        },
        Text::FILE_TEXT { .. } => {
            let mut nchars: i32 = 0;
            let mut aind: i32 = 0;
            let mut w: i32 = 0;
            let mut isstart: bool = false;
            nchars = Mutable::access(var_field!(txt.nchars, Text::FILE_TEXT).clone());
            aind = Mutable::access(var_field!(txt.aind, Text::FILE_TEXT).clone());
            isstart = Mutable::access(var_field!(txt.isstart, Text::FILE_TEXT).clone());
            Mutable::update(var_field!(txt.blocksStack, Text::FILE_TEXT).clone(), cons(BlockTypeFileText { bt: inBlockType.clone(), nchars: nchars.clone(), aind: aind.clone(), isstart: isstart.clone(), tell: Mutable::create(textFileTell(txt.clone())), septok: Mutable::create(None) }, Mutable::access(var_field!(txt.blocksStack, Text::FILE_TEXT).clone())));
            let _ = (::match_deref::match_deref! { match &(inBlockType.clone()) {
        Deref @ BlockType::BT_INDENT { width: w } => {
            Mutable::update(var_field!(txt.nchars, Text::FILE_TEXT).clone(), nchars.clone() + w.clone());
            Mutable::update(var_field!(txt.aind, Text::FILE_TEXT).clone(), aind.clone() + w.clone());
            ()
        },
        Deref @ BlockType::BT_ABS_INDENT { width: w } => {
            if isstart.clone() {
                Mutable::update(var_field!(txt.nchars, Text::FILE_TEXT).clone(), 0);
            }
            Mutable::update(var_field!(txt.aind, Text::FILE_TEXT).clone(), w.clone());
            ()
        },
        Deref @ BlockType::BT_REL_INDENT { offset: w } => {
            Mutable::update(var_field!(txt.aind, Text::FILE_TEXT).clone(), aind.clone() + w.clone());
            ()
        },
        Deref @ BlockType::BT_ANCHOR { offset: w } => {
            Mutable::update(var_field!(txt.aind, Text::FILE_TEXT).clone(), nchars.clone() + w.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            txt.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.pushBlock failed \n")).clone())?;
            bail!("fail")
        },
    });
    Ok(txt)
}

pub fn popBlock(mut txt: Text) -> Result<Text> {
    let mut txt: Text = txt;
    txt = (::match_deref::match_deref! { match &(txt.clone()) {
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (stacktoks, _), tail: blstack }, tokens: Deref @ metamodelica::List::Nil } => {
            Text::MEM_TEXT { tokens: stacktoks.clone(), blocksStack: blstack.clone() }
        },
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (stacktoks, blType), tail: blstack }, tokens: toks } => {
            Text::MEM_TEXT { tokens: cons(Arc::new(StringToken::ST_BLOCK { tokens: toks.clone(), blockType: blType.clone() }), stacktoks.clone()), blocksStack: blstack.clone() }
        },
        Text::FILE_TEXT { .. } => {
            let mut rest: Arc<metamodelica::List<BlockTypeFileText>> = metamodelica::nil();
            let mut blk: BlockTypeFileText;
            let mut oldisstart: bool = false;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mutable::access(var_field!(txt.blocksStack, Text::FILE_TEXT).clone())) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            blk = __pa0.clone();
            rest = __pa1.clone();
            Mutable::update(var_field!(txt.blocksStack, Text::FILE_TEXT).clone(), rest.clone());
            let _ = (::match_deref::match_deref! { match &(blk.bt.clone()) {
        Deref @ BlockType::BT_INDENT { .. } => {
            if Mutable::access(var_field!(txt.isstart, Text::FILE_TEXT).clone()) {
                Mutable::update(var_field!(txt.nchars, Text::FILE_TEXT).clone(), blk.nchars.clone());
            }
            Mutable::update(var_field!(txt.aind, Text::FILE_TEXT).clone(), blk.aind.clone());
            ()
        },
        _ if ((::match_deref::match_deref! { match &(blk.bt.clone()) {
        Deref @ BlockType::BT_ABS_INDENT { .. } => true,
        Deref @ BlockType::BT_REL_INDENT { .. } => true,
        Deref @ BlockType::BT_ANCHOR { .. } => true,
        _ => bail!("match: no arm matched"),
    } })) => {
            oldisstart = Mutable::access(var_field!(txt.isstart, Text::FILE_TEXT).clone());
            if oldisstart.clone() {
                if textFileTell(txt.clone()) == Mutable::access(blk.tell.clone()) {
                    Mutable::update(var_field!(txt.nchars, Text::FILE_TEXT).clone(), blk.nchars.clone());
                } else {
                    if Mutable::access(var_field!(txt.isstart, Text::FILE_TEXT).clone()) {
                        Mutable::update(var_field!(txt.nchars, Text::FILE_TEXT).clone(), blk.aind.clone());
                    }
                }
            } else {
                if Mutable::access(var_field!(txt.isstart, Text::FILE_TEXT).clone()) {
                    Mutable::update(var_field!(txt.nchars, Text::FILE_TEXT).clone(), blk.aind.clone());
                }
            }
            Mutable::update(var_field!(txt.aind, Text::FILE_TEXT).clone(), blk.aind.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            txt.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.popBlock failed - probably pushBlock and popBlock are not well balanced !\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(txt)
}

pub fn pushIter(mut txt: Text, mut inIterOptions: Arc<IterOptions>) -> Result<Text> {
    let mut txt: Text = txt;
    txt = (::match_deref::match_deref! { match &((txt.clone(), inIterOptions.clone())) {
        (Text::MEM_TEXT { blocksStack: blstack, tokens: toks }, iopts @ Deref @ IterOptions { startIndex0: i0, .. }) => {
            Text::MEM_TEXT { tokens: metamodelica::nil(), blocksStack: cons((metamodelica::nil(), Arc::new(BlockType::BT_ITER { options: iopts.clone(), index0: Mutable::create(i0.clone()) })), cons((toks.clone(), Arc::new(crate::Tpl::BlockType::BT_TEXT)), blstack.clone())) }
        },
        (Text::FILE_TEXT { .. }, iopts @ Deref @ IterOptions { startIndex0: i0, .. }) => {
            let _ = (::match_deref::match_deref! { match &(iopts.clone()) {
        Deref @ IterOptions { wrapWidth: 0, alignNum: 0, .. } => (),
        _ => {
            Error::addInternalError((literal!("Tpl.mo FILE_TEXT does not support aligning or wrapping elements")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            pushBlock(txt.clone(), Arc::new(BlockType::BT_ITER { options: inIterOptions.clone(), index0: Mutable::create(i0.clone()) }))?;
            txt.clone()
        },
        (_, _) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.pushIter failed \n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(txt)
}

pub fn popIter(mut txt: Text) -> Result<Text> {
    let mut txt: Text = txt;
    txt = (::match_deref::match_deref! { match &(txt.clone()) {
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (Deref @ metamodelica::List::Nil, _), tail: Deref @ metamodelica::List::Cons { head: (stacktoks, _), tail: blstack } }, tokens: Deref @ metamodelica::List::Nil } => {
            Text::MEM_TEXT { tokens: stacktoks.clone(), blocksStack: blstack.clone() }
        },
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (itertoks, blType), tail: Deref @ metamodelica::List::Cons { head: (stacktoks, _), tail: blstack } }, tokens: Deref @ metamodelica::List::Nil } => {
            Text::MEM_TEXT { tokens: cons(Arc::new(StringToken::ST_BLOCK { tokens: itertoks.clone(), blockType: blType.clone() }), stacktoks.clone()), blocksStack: blstack.clone() }
        },
        Text::FILE_TEXT { .. } => {
            Mutable::update(var_field!(txt.blocksStack, Text::FILE_TEXT).clone(), listRest(Mutable::access(var_field!(txt.blocksStack, Text::FILE_TEXT).clone()))?);
            txt.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.popIter failed - probably pushIter and popIter are not well balanced or something was written between the last nextIter and popIter ?\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(txt)
}

pub fn nextIter(mut txt: Text) -> Result<Text> {
    let mut txt: Text = txt;
    txt = (::match_deref::match_deref! { match &(txt.clone()) {
        txt @ Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (_, Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { empty: None, .. }, .. }), tail: _ }, tokens: Deref @ metamodelica::List::Nil } => {
            txt.clone()
        },
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (itertoks, bt @ Deref @ BlockType::BT_ITER { index0: i0, options: Deref @ IterOptions { empty: Some(emptok), .. } }), tail: blstack }, tokens: Deref @ metamodelica::List::Nil } => {
            Mutable::update(i0.clone(), Mutable::access(i0.clone()) + 1);
            Text::MEM_TEXT { tokens: metamodelica::nil(), blocksStack: cons((cons(emptok.clone(), itertoks.clone()), bt.clone()), blstack.clone()) }
        },
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (itertoks, bt @ Deref @ BlockType::BT_ITER { index0: i0, .. }), tail: blstack }, tokens: Deref @ metamodelica::List::Cons { head: tok, tail: Deref @ metamodelica::List::Nil } } => {
            Mutable::update(i0.clone(), Mutable::access(i0.clone()) + 1);
            Text::MEM_TEXT { tokens: metamodelica::nil(), blocksStack: cons((cons(tok.clone(), itertoks.clone()), bt.clone()), blstack.clone()) }
        },
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (itertoks, bt @ Deref @ BlockType::BT_ITER { index0: i0, .. }), tail: blstack }, tokens: toks } => {
            Mutable::update(i0.clone(), Mutable::access(i0.clone()) + 1);
            Text::MEM_TEXT { tokens: metamodelica::nil(), blocksStack: cons((cons(Arc::new(StringToken::ST_BLOCK { tokens: toks.clone(), blockType: Arc::new(crate::Tpl::BlockType::BT_TEXT) }), itertoks.clone()), bt.clone()), blstack.clone()) }
        },
        Text::FILE_TEXT { .. } => {
            let mut emptok: Arc<StringToken> = Arc::new(StringToken::ST_NEW_LINE);
            let mut i0: Mutable::Mutable<i32>;
            let mut tell: Mutable::Mutable<i32>;
            let mut tellpos: i32 = 0;
            let mut curIndex: i32 = 0;
            let mut txt2: Text;
            let mut haveToken: bool = false;
            let mut septok: Mutable::Mutable<Option<Arc<StringToken>>>;
            let _ = (::match_deref::match_deref! { match &((Mutable::access(var_field!(txt.blocksStack, Text::FILE_TEXT).clone())).get(1)?) {
        BlockTypeFileText { septok, tell, bt: Deref @ BlockType::BT_ITER { index0: i0, options: iopts }, .. } => {
            tellpos = textFileTell(txt.clone());
            if Mutable::access(tell.clone()) != tellpos.clone() {
                Mutable::update(tell.clone(), tellpos.clone());
                txt2 = txt.clone();
                haveToken = true;
            } else {
                txt2 = (::match_deref::match_deref! { match &(iopts.empty.clone()) {
        None => {
            haveToken = false;
            txt.clone()
        },
        Some(emptok) => {
            Mutable::update(i0.clone(), Mutable::access(i0.clone()) + 1);
            haveToken = true;
            writeTok(txt.clone(), emptok.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            if haveToken.clone() {
                curIndex = Mutable::access(i0.clone());
                Mutable::update(septok.clone(), iopts.separator.clone());
                Mutable::update(i0.clone(), curIndex.clone() + 1);
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            txt2.clone()
        },
        _ => {
            Error::addInternalError((literal!("-!!!Tpl.nextIter failed - nextIter was called in a non-iteration context?")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(txt)
}

pub fn getIteri_i0(mut inText: Text) -> Result<i32> {
    let mut outI0: i32 = 0;
    outI0 = (::match_deref::match_deref! { match &(inText.clone()) {
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: (_, Deref @ BlockType::BT_ITER { index0: i0, .. }), tail: _ }, .. } => {
            Mutable::access(i0.clone())
        },
        Text::FILE_TEXT { .. } => {
            let mut i0: Mutable::Mutable<i32>;
            (::match_deref::match_deref! { match &((Mutable::access(var_field!(inText.blocksStack, Text::FILE_TEXT).clone())).get(1)?) {
        BlockTypeFileText { bt: Deref @ BlockType::BT_ITER { index0: i0, .. }, .. } => Mutable::access(i0.clone()),
        _ => bail!("match: no arm matched"),
    } })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.getIter_i0 failed - getIter_i0 was called in a non-iteration context ? \n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outI0)
}

pub fn textString(mut inText: Text) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inText.clone() {
        mut txt => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut handle: i32 = 0;
            handle = Print::saveAndClearBuf()?;
            textStringBuf(txt.clone())?;
            r#str = (Print::getString()?).clone();
            Print::restoreBuf(handle.clone())?;
            r#str.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.textString failed.\n")).clone())?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

pub fn textStringBuf(mut inText: Text) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inText.clone()) {
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Nil, tokens: toks } => {
            (_, _, _) = tokensString(toks.clone().reverse(), 0, true, 0)?;
            ()
        },
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.textString failed - a non-comlete text was given.\n")).clone())?;
            bail!("fail")
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.textString failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn tokensString(mut inTokens: Tokens, mut actualPositionOnLine: i32, mut atStartOfLine: bool, mut afterNewLineIndent: i32) -> Result<(i32, bool, i32)> {
    let mut actualPositionOnLine: i32 = actualPositionOnLine;
    let mut atStartOfLine: bool = atStartOfLine;
    let mut afterNewLineIndent: i32 = afterNewLineIndent;
    for mut tok in &*inTokens.clone() {
        let mut tok = tok.clone();
        (actualPositionOnLine, atStartOfLine, afterNewLineIndent) = tokString(tok.clone(), actualPositionOnLine.clone(), atStartOfLine.clone(), afterNewLineIndent.clone())?;
    }
    Ok((actualPositionOnLine, atStartOfLine, afterNewLineIndent))
}

fn tokensFile(mut file: File::File, mut inTokens: Tokens, mut actualPositionOnLine: i32, mut atStartOfLine: bool, mut afterNewLineIndent: i32) -> Result<(i32, bool, i32)> {
    let mut actualPositionOnLine: i32 = actualPositionOnLine;
    let mut atStartOfLine: bool = atStartOfLine;
    let mut afterNewLineIndent: i32 = afterNewLineIndent;
    for mut tok in &*inTokens.clone() {
        let mut tok = tok.clone();
        (actualPositionOnLine, atStartOfLine, afterNewLineIndent) = tokFile(file.clone(), tok.clone(), actualPositionOnLine.clone(), atStartOfLine.clone(), afterNewLineIndent.clone())?;
    }
    Ok((actualPositionOnLine, atStartOfLine, afterNewLineIndent))
}

fn tokString(mut inStringToken: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool, i32)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    let mut outAfterNewLineIndent: i32 = 0;
    (outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent) = (::match_deref::match_deref! { match &((inStringToken.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (Deref @ StringToken::ST_NEW_LINE { .. }, _, _, aind) => {
            Print::printBufNewLine()?;
            (aind.clone(), true, aind.clone())
        },
        (Deref @ StringToken::ST_STRING { value: r#str }, nchars, true, aind) => {
            let mut blen: i32 = 0;
            blen = Print::getBufLength();
            Print::printBufSpace(nchars.clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            blen = Print::getBufLength() - blen.clone();
            (blen.clone(), false, aind.clone())
        },
        (Deref @ StringToken::ST_STRING { value: r#str }, nchars, false, aind) => {
            let mut blen: i32 = 0;
            blen = Print::getBufLength();
            Print::printBuf((r#str.clone()).clone())?;
            blen = Print::getBufLength() - blen.clone();
            (nchars.clone() + blen.clone(), false, aind.clone())
        },
        (Deref @ StringToken::ST_LINE { line: r#str }, nchars, true, aind) => {
            Print::printBufSpace(nchars.clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            (aind.clone(), true, aind.clone())
        },
        (Deref @ StringToken::ST_LINE { line: r#str }, _, false, aind) => {
            Print::printBuf((r#str.clone()).clone())?;
            (aind.clone(), true, aind.clone())
        },
        (Deref @ StringToken::ST_STRING_LIST { strList: strLst, .. }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = stringListString(strLst.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ StringToken::ST_BLOCK { blockType: bt, tokens: toks }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = blockString(bt.clone(), toks.clone().reverse(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.tokString failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent))
}

fn tokFileText(mut inText: Text, mut inStringToken: Arc<StringToken>, mut doHandleTok: bool) -> Result<()> {
    let mut file: File::File = File::File(getTextOpaqueFile(inText.clone())?)?;
    let mut nchars: i32 = 0;
    let mut aind: i32 = 0;
    let mut isstart: bool = false;
    if doHandleTok.clone() {
        handleTok(inText.clone())?;
    }
    let _ = (match inText.clone() {
        Text::FILE_TEXT { .. } => {
            nchars = Mutable::access(var_field!(inText.nchars, Text::FILE_TEXT).clone());
            aind = Mutable::access(var_field!(inText.aind, Text::FILE_TEXT).clone());
            isstart = Mutable::access(var_field!(inText.isstart, Text::FILE_TEXT).clone());
            (nchars, isstart, aind) = tokFile(file.clone(), inStringToken.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            Mutable::update(var_field!(inText.nchars, Text::FILE_TEXT).clone(), nchars.clone());
            Mutable::update(var_field!(inText.aind, Text::FILE_TEXT).clone(), aind.clone());
            Mutable::update(var_field!(inText.isstart, Text::FILE_TEXT).clone(), isstart.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn tokFile(mut file: File::File, mut inStringToken: Arc<StringToken>, mut nchars: i32, mut isstart: bool, mut aind: i32) -> Result<(i32, bool, i32)> {
    let mut nchars: i32 = nchars;
    let mut isstart: bool = isstart;
    let mut aind: i32 = aind;
    (nchars, isstart, aind) = (::match_deref::match_deref! { match &((inStringToken.clone(), nchars.clone(), isstart.clone(), aind.clone())) {
        (Deref @ StringToken::ST_NEW_LINE { .. }, _, _, aind) => {
            File::write(file.clone(), (literal!("\n")).clone());
            (aind.clone(), true, aind.clone())
        },
        (Deref @ StringToken::ST_STRING { value: r#str }, nchars, true, aind) => {
            File::writeSpace(file.clone(), nchars.clone());
            File::write(file.clone(), (r#str.clone()).clone());
            (nchars.clone() + ((r#str.clone()).clone().len() as i32), false, aind.clone())
        },
        (Deref @ StringToken::ST_STRING { value: r#str }, nchars, false, aind) => {
            File::write(file.clone(), (r#str.clone()).clone());
            (nchars.clone() + ((r#str.clone()).clone().len() as i32), false, aind.clone())
        },
        (Deref @ StringToken::ST_LINE { line: r#str }, nchars, true, aind) => {
            File::writeSpace(file.clone(), nchars.clone());
            File::write(file.clone(), (r#str.clone()).clone());
            (aind.clone(), true, aind.clone())
        },
        (Deref @ StringToken::ST_LINE { line: r#str }, _, false, aind) => {
            File::write(file.clone(), (r#str.clone()).clone());
            (aind.clone(), true, aind.clone())
        },
        (Deref @ StringToken::ST_STRING_LIST { strList: strLst, .. }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = stringListFile(file.clone(), strLst.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ StringToken::ST_BLOCK { blockType: bt, tokens: toks }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = blockFile(file.clone(), bt.clone(), toks.clone().reverse(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((nchars, isstart, aind))
}

fn stringListString(mut inStringList: Arc<metamodelica::List<ArcStr>>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool, i32)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    let mut outAfterNewLineIndent: i32 = 0;
    (outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent) = (::match_deref::match_deref! { match &((inStringList.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (Deref @ metamodelica::List::Nil, _, isstart, aind) => {
            (aind.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ "", tail: strLst }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = stringListString(strLst.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ metamodelica::List::Cons { head: r#str, tail: strLst }, nchars, true, aind) => {
            let mut blen: i32 = 0;
            let mut isstart: bool = false;
            let mut hasNL: bool = false;
            let mut nchars = (*nchars).clone();
            let mut aind = (*aind).clone();
            blen = Print::getBufLength();
            Print::printBufSpace(nchars.clone())?;
            Print::printBuf((r#str.clone()).clone())?;
            blen = Print::getBufLength() - blen.clone();
            hasNL = Print::hasBufNewLineAtEnd();
            nchars = if (hasNL.clone()) {aind.clone()} else {blen.clone()};
            (nchars, isstart, aind) = stringListString(strLst.clone(), nchars.clone(), hasNL.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ metamodelica::List::Cons { head: r#str, tail: strLst }, nchars, false, aind) => {
            let mut blen: i32 = 0;
            let mut isstart: bool = false;
            let mut hasNL: bool = false;
            let mut nchars = (*nchars).clone();
            let mut aind = (*aind).clone();
            blen = Print::getBufLength();
            Print::printBuf((r#str.clone()).clone())?;
            blen = Print::getBufLength() - blen.clone();
            hasNL = Print::hasBufNewLineAtEnd();
            nchars = if (hasNL.clone()) {aind.clone()} else {nchars.clone() + blen.clone()};
            (nchars, isstart, aind) = stringListString(strLst.clone(), nchars.clone(), hasNL.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.stringListString failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent))
}

fn stringListFile(mut file: File::File, mut inStringList: Arc<metamodelica::List<ArcStr>>, mut nchars: i32, mut isstart: bool, mut aind: i32) -> Result<(i32, bool, i32)> {
    let mut nchars: i32 = nchars;
    let mut isstart: bool = isstart;
    let mut aind: i32 = aind;
    (nchars, isstart, aind) = (::match_deref::match_deref! { match &((inStringList.clone(), nchars.clone(), isstart.clone(), aind.clone())) {
        (Deref @ metamodelica::List::Nil, _, isstart, aind) => {
            (aind.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ "", tail: strLst }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = stringListFile(file.clone(), strLst.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ metamodelica::List::Cons { head: r#str, tail: strLst }, nchars, true, aind) => {
            let mut hasNL: bool = false;
            let mut nchars = (*nchars).clone();
            let mut aind = (*aind).clone();
            File::writeSpace(file.clone(), nchars.clone());
            File::write(file.clone(), (r#str.clone()).clone());
            hasNL = StringUtil::endsWithNewline((r#str.clone()).clone());
            nchars = if (hasNL.clone()) {aind.clone()} else {nchars.clone() + ((r#str.clone()).clone().len() as i32)};
            (nchars, isstart, aind) = stringListFile(file.clone(), strLst.clone(), nchars.clone(), hasNL.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ metamodelica::List::Cons { head: r#str, tail: strLst }, nchars, false, aind) => {
            let mut hasNL: bool = false;
            let mut nchars = (*nchars).clone();
            let mut aind = (*aind).clone();
            File::write(file.clone(), (r#str.clone()).clone());
            hasNL = StringUtil::endsWithNewline((r#str.clone()).clone());
            nchars = if (hasNL.clone()) {aind.clone()} else {nchars.clone() + ((r#str.clone()).clone().len() as i32)};
            (nchars, isstart, aind) = stringListFile(file.clone(), strLst.clone(), nchars.clone(), hasNL.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.stringListFile failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((nchars, isstart, aind))
}

fn blockString(mut inBlockType: Arc<BlockType>, mut inTokens: Tokens, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool, i32)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    let mut outAfterNewLineIndent: i32 = 0;
    (outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent) = (::match_deref::match_deref! { match &((inBlockType.clone(), inTokens.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (Deref @ BlockType::BT_TEXT { .. }, toks, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = tokensString(toks.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_INDENT { width: w }, toks, nchars, true, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            (tsnchars, isstart, _) = tokensString(toks.clone(), w.clone() + nchars.clone(), true, w.clone() + aind.clone())?;
            nchars = if (isstart.clone()) {nchars.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_INDENT { width: w }, toks, nchars, false, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            Print::printBufSpace(w.clone())?;
            (tsnchars, isstart, _) = tokensString(toks.clone(), w.clone() + nchars.clone(), false, w.clone() + aind.clone())?;
            nchars = if (isstart.clone()) {aind.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ABS_INDENT { width: w }, toks, nchars, true, aind) => {
            let mut tsnchars: i32 = 0;
            let mut blen: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            blen = Print::getBufLength();
            (tsnchars, isstart, _) = tokensString(toks.clone(), 0, true, w.clone())?;
            blen = Print::getBufLength() - blen.clone();
            nchars = if (blen.clone() == 0) {nchars.clone()} else {if (isstart.clone()) {aind.clone()} else {tsnchars.clone()}};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ABS_INDENT { width: w }, toks, nchars, false, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            (tsnchars, isstart, _) = tokensString(toks.clone(), nchars.clone(), false, w.clone())?;
            nchars = if (isstart.clone()) {aind.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_REL_INDENT { offset: w }, toks, nchars, true, aind) => {
            let mut tsnchars: i32 = 0;
            let mut blen: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            blen = Print::getBufLength();
            (tsnchars, isstart, _) = tokensString(toks.clone(), nchars.clone(), true, aind.clone() + w.clone())?;
            blen = Print::getBufLength() - blen.clone();
            nchars = if (blen.clone() == 0) {nchars.clone()} else {if (isstart.clone()) {aind.clone()} else {tsnchars.clone()}};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_REL_INDENT { offset: w }, toks, nchars, false, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            (tsnchars, isstart, _) = tokensString(toks.clone(), nchars.clone(), false, aind.clone() + w.clone())?;
            nchars = if (isstart.clone()) {aind.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ANCHOR { offset: w }, toks, nchars, true, aind) => {
            let mut tsnchars: i32 = 0;
            let mut blen: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            blen = Print::getBufLength();
            (tsnchars, isstart, _) = tokensString(toks.clone(), nchars.clone(), true, nchars.clone() + w.clone())?;
            blen = Print::getBufLength() - blen.clone();
            nchars = if (blen.clone() == 0) {nchars.clone()} else {if (isstart.clone()) {aind.clone()} else {tsnchars.clone()}};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ANCHOR { offset: w }, toks, nchars, false, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            (tsnchars, isstart, _) = tokensString(toks.clone(), nchars.clone(), false, nchars.clone() + w.clone())?;
            nchars = if (isstart.clone()) {aind.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { .. }, Deref @ metamodelica::List::Nil, nchars, isstart, aind) => {
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { wrapWidth: 0, alignNum: 0, separator: None, .. }, .. }, toks, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = tokensString(toks.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { wrapWidth: 0, alignNum: 0, separator: Some(septok), .. }, .. }, Deref @ metamodelica::List::Cons { head: tok, tail: toks }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = tokString(tok.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars, isstart) = iterSeparatorString(toks.clone(), septok.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { wrapSeparator: wsep, wrapWidth: wwidth, alignSeparator: asep, alignOfset: aoffset, alignNum: anum, separator: Some(septok), .. }, .. }, Deref @ metamodelica::List::Cons { head: tok, tail: toks }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = tokString(tok.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars, isstart) = iterSeparatorAlignWrapString(toks.clone(), septok.clone(), 1 + aoffset.clone(), anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { wrapSeparator: wsep, wrapWidth: wwidth, alignSeparator: asep, alignOfset: aoffset, alignNum: anum, separator: None, .. }, .. }, toks, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            (nchars, isstart) = iterAlignWrapString(toks.clone(), aoffset.clone(), anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.tokString failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent))
}

fn iterSeparatorString(mut inTokens: Tokens, mut inSeparator: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    (outActualPositionOnLine, outAtStartOfLine) = (::match_deref::match_deref! { match &((inTokens.clone(), inSeparator.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (Deref @ metamodelica::List::Nil, _, pos, isstart, _) => {
            (pos.clone(), isstart.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tok, tail: toks }, septok, pos, isstart, aind) => {
            let mut pos = (*pos).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (pos, isstart, aind) = tokString(septok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart, aind) = tokString(tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart) = iterSeparatorString(toks.clone(), septok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos.clone(), isstart.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine))
}

fn iterSeparatorAlignWrapString(mut inTokens: Tokens, mut inSeparator: Arc<StringToken>, mut inActualIndex: i32, mut inAlignNum: i32, mut inAlignSeparator: Arc<StringToken>, mut inWrapWidth: i32, mut inWrapSeparator: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    let mut toks: Tokens = inTokens.clone();
    let mut tok: Arc<StringToken> = Arc::new(StringToken::ST_NEW_LINE);
    let mut septok: Arc<StringToken> = inSeparator.clone();
    let mut idx: i32 = inActualIndex.clone();
    let mut anum: i32 = inAlignNum.clone();
    let mut asep: Arc<StringToken> = inAlignSeparator.clone();
    let mut wwidth: i32 = inWrapWidth.clone();
    let mut wsep: Arc<StringToken> = inWrapSeparator.clone();
    let mut pos: i32 = inActualPositionOnLine.clone();
    let mut isstart: bool = inAtStartOfLine.clone();
    let mut aind: i32 = inAfterNewLineIndent.clone();
    while boolNot(toks.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(toks.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        tok = __pa0.clone();
        toks = __pa1.clone();
        if idx.clone() > 0 && intMod(idx.clone(), anum.clone()) == 0 {
            (pos, isstart, aind) = tokString(asep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
        } else {
            (pos, isstart, aind) = tokString(septok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
        }
        (pos, isstart, aind) = tryWrapString(wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
        (pos, isstart, aind) = tokString(tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
        idx = idx.clone() + 1;
    }
    (outActualPositionOnLine, outAtStartOfLine) = (pos.clone(), isstart.clone());
    Ok((outActualPositionOnLine, outAtStartOfLine))
}

fn iterAlignWrapString(mut inTokens: Tokens, mut inActualIndex: i32, mut inAlignNum: i32, mut inAlignSeparator: Arc<StringToken>, mut inWrapWidth: i32, mut inWrapSeparator: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    (outActualPositionOnLine, outAtStartOfLine) = (::match_deref::match_deref! { match &((inTokens.clone(), inActualIndex.clone(), inAlignNum.clone(), inAlignSeparator.clone(), inWrapWidth.clone(), inWrapSeparator.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, pos, isstart, _) => {
            (pos.clone(), isstart.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tok, tail: toks }, idx, anum, asep, wwidth, wsep, pos, isstart, aind) if (idx.clone() > 0 && intMod(idx.clone(), anum.clone()) == 0) => {
            let mut pos = (*pos).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (pos, isstart, aind) = tokString(asep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart, aind) = tryWrapString(wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart, aind) = tokString(tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart) = iterAlignWrapString(toks.clone(), idx.clone() + 1, anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos.clone(), isstart.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tok, tail: toks }, idx, anum, asep, wwidth, wsep, pos, isstart, aind) if (wwidth.clone() > 0 && pos.clone() >= wwidth.clone()) => {
            let mut pos = (*pos).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (pos, isstart, aind) = tokString(wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart, aind) = tokString(tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart) = iterAlignWrapString(toks.clone(), idx.clone() + 1, anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos.clone(), isstart.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tok, tail: toks }, idx, anum, asep, wwidth, wsep, pos, isstart, aind) => {
            let mut pos = (*pos).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (pos, isstart, aind) = tokString(tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart) = iterAlignWrapString(toks.clone(), idx.clone() + 1, anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos.clone(), isstart.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.iterAlignWrapString failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine))
}

fn tryWrapString(mut inWrapWidth: i32, mut inWrapSeparator: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool, i32)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    let mut outAfterNewLineIndent: i32 = 0;
    (outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent) = (::match_deref::match_deref! { match &((inWrapWidth.clone(), inWrapSeparator.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (wwidth, wsep, pos, isstart, aind) if (wwidth.clone() > 0 && pos.clone() >= wwidth.clone()) => {
            tokString(wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?
        },
        _ => {
            (inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent))
}

fn blockFile(mut file: File::File, mut inBlockType: Arc<BlockType>, mut inTokens: Tokens, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool, i32)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    let mut outAfterNewLineIndent: i32 = 0;
    (outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent) = (::match_deref::match_deref! { match &((inBlockType.clone(), inTokens.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (Deref @ BlockType::BT_TEXT { .. }, toks, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = tokensFile(file.clone(), toks.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_INDENT { width: w }, toks, nchars, true, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            (tsnchars, isstart, _) = tokensFile(file.clone(), toks.clone(), w.clone() + nchars.clone(), true, w.clone() + aind.clone())?;
            nchars = if (isstart.clone()) {nchars.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_INDENT { width: w }, toks, nchars, false, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            File::writeSpace(file.clone(), w.clone());
            (tsnchars, isstart, _) = tokensFile(file.clone(), toks.clone(), w.clone() + nchars.clone(), false, w.clone() + aind.clone())?;
            nchars = if (isstart.clone()) {aind.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ABS_INDENT { width: w }, toks, nchars, true, aind) => {
            let mut tsnchars: i32 = 0;
            let mut blen: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            blen = File::tell(file.clone());
            (tsnchars, isstart, _) = tokensFile(file.clone(), toks.clone(), 0, true, w.clone())?;
            blen = File::tell(file.clone()) - blen.clone();
            nchars = if (blen.clone() == 0) {nchars.clone()} else {if (isstart.clone()) {aind.clone()} else {tsnchars.clone()}};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ABS_INDENT { width: w }, toks, nchars, false, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            (tsnchars, isstart, _) = tokensFile(file.clone(), toks.clone(), nchars.clone(), false, w.clone())?;
            nchars = if (isstart.clone()) {aind.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_REL_INDENT { offset: w }, toks, nchars, true, aind) => {
            let mut tsnchars: i32 = 0;
            let mut blen: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            blen = File::tell(file.clone());
            (tsnchars, isstart, _) = tokensFile(file.clone(), toks.clone(), nchars.clone(), true, aind.clone() + w.clone())?;
            blen = File::tell(file.clone()) - blen.clone();
            nchars = if (blen.clone() == 0) {nchars.clone()} else {if (isstart.clone()) {aind.clone()} else {tsnchars.clone()}};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_REL_INDENT { offset: w }, toks, nchars, false, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            (tsnchars, isstart, _) = tokensFile(file.clone(), toks.clone(), nchars.clone(), false, aind.clone() + w.clone())?;
            nchars = if (isstart.clone()) {aind.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ANCHOR { offset: w }, toks, nchars, true, aind) => {
            let mut tsnchars: i32 = 0;
            let mut blen: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            blen = File::tell(file.clone());
            (tsnchars, isstart, _) = tokensFile(file.clone(), toks.clone(), nchars.clone(), true, nchars.clone() + w.clone())?;
            blen = File::tell(file.clone()) - blen.clone();
            nchars = if (blen.clone() == 0) {nchars.clone()} else {if (isstart.clone()) {aind.clone()} else {tsnchars.clone()}};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ANCHOR { offset: w }, toks, nchars, false, aind) => {
            let mut tsnchars: i32 = 0;
            let mut isstart: bool = false;
            let mut nchars = (*nchars).clone();
            (tsnchars, isstart, _) = tokensFile(file.clone(), toks.clone(), nchars.clone(), false, nchars.clone() + w.clone())?;
            nchars = if (isstart.clone()) {aind.clone()} else {tsnchars.clone()};
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { .. }, Deref @ metamodelica::List::Nil, nchars, isstart, aind) => {
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { wrapWidth: 0, alignNum: 0, separator: None, .. }, .. }, toks, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = tokensFile(file.clone(), toks.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { wrapWidth: 0, alignNum: 0, separator: Some(septok), .. }, .. }, Deref @ metamodelica::List::Cons { head: tok, tail: toks }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = tokFile(file.clone(), tok.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars, isstart) = iterSeparatorFile(file.clone(), toks.clone(), septok.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { wrapSeparator: wsep, wrapWidth: wwidth, alignSeparator: asep, alignOfset: aoffset, alignNum: anum, separator: Some(septok), .. }, .. }, Deref @ metamodelica::List::Cons { head: tok, tail: toks }, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (nchars, isstart, aind) = tokFile(file.clone(), tok.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars, isstart) = iterSeparatorAlignWrapFile(file.clone(), toks.clone(), septok.clone(), 1 + aoffset.clone(), anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        (Deref @ BlockType::BT_ITER { options: Deref @ IterOptions { wrapSeparator: wsep, wrapWidth: wwidth, alignSeparator: asep, alignOfset: aoffset, alignNum: anum, separator: None, .. }, .. }, toks, nchars, isstart, aind) => {
            let mut nchars = (*nchars).clone();
            let mut isstart = (*isstart).clone();
            (nchars, isstart) = iterAlignWrapFile(file.clone(), toks.clone(), aoffset.clone(), anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), nchars.clone(), isstart.clone(), aind.clone())?;
            (nchars.clone(), isstart.clone(), aind.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.tokString failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent))
}

fn iterSeparatorFile(mut file: File::File, mut inTokens: Tokens, mut inSeparator: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    (outActualPositionOnLine, outAtStartOfLine) = (::match_deref::match_deref! { match &((inTokens.clone(), inSeparator.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (Deref @ metamodelica::List::Nil, _, pos, isstart, _) => {
            (pos.clone(), isstart.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tok, tail: toks }, septok, pos, isstart, aind) => {
            let mut pos = (*pos).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (pos, isstart, aind) = tokFile(file.clone(), septok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart, aind) = tokFile(file.clone(), tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart) = iterSeparatorFile(file.clone(), toks.clone(), septok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos.clone(), isstart.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine))
}

fn iterSeparatorAlignWrapFile(mut file: File::File, mut inTokens: Tokens, mut inSeparator: Arc<StringToken>, mut inActualIndex: i32, mut inAlignNum: i32, mut inAlignSeparator: Arc<StringToken>, mut inWrapWidth: i32, mut inWrapSeparator: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    let mut toks: Tokens = inTokens.clone();
    let mut tok: Arc<StringToken> = Arc::new(StringToken::ST_NEW_LINE);
    let mut septok: Arc<StringToken> = inSeparator.clone();
    let mut idx: i32 = inActualIndex.clone();
    let mut anum: i32 = inAlignNum.clone();
    let mut asep: Arc<StringToken> = inAlignSeparator.clone();
    let mut wwidth: i32 = inWrapWidth.clone();
    let mut wsep: Arc<StringToken> = inWrapSeparator.clone();
    let mut pos: i32 = inActualPositionOnLine.clone();
    let mut isstart: bool = inAtStartOfLine.clone();
    let mut aind: i32 = inAfterNewLineIndent.clone();
    while boolNot(toks.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(toks.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        tok = __pa0.clone();
        toks = __pa1.clone();
        if idx.clone() > 0 && intMod(idx.clone(), anum.clone()) == 0 {
            (pos, isstart, aind) = tokFile(file.clone(), asep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
        } else {
            (pos, isstart, aind) = tokFile(file.clone(), septok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
        }
        (pos, isstart, aind) = tryWrapFile(file.clone(), wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
        (pos, isstart, aind) = tokFile(file.clone(), tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
        idx = idx.clone() + 1;
    }
    (outActualPositionOnLine, outAtStartOfLine) = (pos.clone(), isstart.clone());
    Ok((outActualPositionOnLine, outAtStartOfLine))
}

fn iterAlignWrapFile(mut file: File::File, mut inTokens: Tokens, mut inActualIndex: i32, mut inAlignNum: i32, mut inAlignSeparator: Arc<StringToken>, mut inWrapWidth: i32, mut inWrapSeparator: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    (outActualPositionOnLine, outAtStartOfLine) = (::match_deref::match_deref! { match &((inTokens.clone(), inActualIndex.clone(), inAlignNum.clone(), inAlignSeparator.clone(), inWrapWidth.clone(), inWrapSeparator.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (Deref @ metamodelica::List::Nil, _, _, _, _, _, pos, isstart, _) => {
            (pos.clone(), isstart.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tok, tail: toks }, idx, anum, asep, wwidth, wsep, pos, isstart, aind) if (idx.clone() > 0 && intMod(idx.clone(), anum.clone()) == 0) => {
            let mut pos = (*pos).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (pos, isstart, aind) = tokFile(file.clone(), asep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart, aind) = tryWrapFile(file.clone(), wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart, aind) = tokFile(file.clone(), tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart) = iterAlignWrapFile(file.clone(), toks.clone(), idx.clone() + 1, anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos.clone(), isstart.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tok, tail: toks }, idx, anum, asep, wwidth, wsep, pos, isstart, aind) if (wwidth.clone() > 0 && pos.clone() >= wwidth.clone()) => {
            let mut pos = (*pos).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (pos, isstart, aind) = tokFile(file.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart, aind) = tokFile(file.clone(), tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart) = iterAlignWrapFile(file.clone(), toks.clone(), idx.clone() + 1, anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos.clone(), isstart.clone())
        },
        (Deref @ metamodelica::List::Cons { head: tok, tail: toks }, idx, anum, asep, wwidth, wsep, pos, isstart, aind) => {
            let mut pos = (*pos).clone();
            let mut isstart = (*isstart).clone();
            let mut aind = (*aind).clone();
            (pos, isstart, aind) = tokFile(file.clone(), tok.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos, isstart) = iterAlignWrapFile(file.clone(), toks.clone(), idx.clone() + 1, anum.clone(), asep.clone(), wwidth.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?;
            (pos.clone(), isstart.clone())
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.iterAlignWrapString failed.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine))
}

fn tryWrapFile(mut file: File::File, mut inWrapWidth: i32, mut inWrapSeparator: Arc<StringToken>, mut inActualPositionOnLine: i32, mut inAtStartOfLine: bool, mut inAfterNewLineIndent: i32) -> Result<(i32, bool, i32)> {
    let mut outActualPositionOnLine: i32 = 0;
    let mut outAtStartOfLine: bool = false;
    let mut outAfterNewLineIndent: i32 = 0;
    (outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent) = (::match_deref::match_deref! { match &((inWrapWidth.clone(), inWrapSeparator.clone(), inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())) {
        (wwidth, wsep, pos, isstart, aind) if (wwidth.clone() > 0 && pos.clone() >= wwidth.clone()) => {
            tokFile(file.clone(), wsep.clone(), pos.clone(), isstart.clone(), aind.clone())?
        },
        _ => {
            (inActualPositionOnLine.clone(), inAtStartOfLine.clone(), inAfterNewLineIndent.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outActualPositionOnLine, outAtStartOfLine, outAfterNewLineIndent))
}

pub fn strTokText(mut inStringToken: Arc<StringToken>) -> Text {
    let mut outText: Text;
    outText = Text::MEM_TEXT { tokens: list![inStringToken.clone()], blocksStack: metamodelica::nil() };
    outText
}

pub fn textStrTok(mut inText: Text) -> Result<Arc<StringToken>> {
    let mut outStringToken: Arc<StringToken> = Arc::new(StringToken::ST_NEW_LINE);
    outStringToken = (::match_deref::match_deref! { match &(inText.clone()) {
        Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. } => {
            Arc::new(StringToken::ST_STRING { value: (literal!("")).clone() })
        },
        Text::MEM_TEXT { blocksStack: Deref @ metamodelica::List::Nil, tokens: txttoks } => {
            Arc::new(StringToken::ST_BLOCK { tokens: txttoks.clone(), blockType: Arc::new(crate::Tpl::BlockType::BT_TEXT) })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.textStrTok failed - incomplete text was passed to be converted.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStringToken)
}

pub fn stringText(mut inString: ArcStr) -> Text {
    let mut outText: Text;
    outText = Text::MEM_TEXT { tokens: list![Arc::new(StringToken::ST_STRING { value: (inString.clone()).clone() })], blocksStack: metamodelica::nil() };
    outText
}

pub fn strTokString(mut inStringToken: Arc<StringToken>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (textString(Text::MEM_TEXT { tokens: list![inStringToken.clone()], blocksStack: metamodelica::nil() })?).clone();
    Ok(outString)
}

pub fn failIfTrue(mut istrue: bool) -> Result<()> {
    if istrue.clone() {
        bail!("fail");
    }
    Ok(())
}

fn tplCallHandleErrors(mut inFun: Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>, mut txt: Text) -> Result<Text> {
    pub type Tpl_Fun = std::sync::Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    match '__try0: {
        if let Ok(__iflet1) = inFun(txt.clone()) {
            txt = __iflet1;
        } else {
            unwrap_break_err!(addTemplateErrorFunc(inFun.clone()), '__try0);
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        Ok::<_, anyhow::Error>((txt.clone(),))
    } {
        Ok((__try0_o0,)) => {
            txt = __try0_o0;
        }
        Err(_) => {
            if StackOverflow::hasStacktraceMessages() {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow when evaluating function:\n")); __mm_s.push_str(&*stringDelimitList(StackOverflow::readableStacktraceMessages()?, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
            addTemplateErrorFunc(inFun.clone())?;
            bail!("fail");
        }
    }
    Ok(txt)
}

pub fn tplCallWithFailErrorNoArg(mut inFun: Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>, mut txt: Text) -> Result<Text> {
    pub type Tpl_Fun = std::sync::Arc<dyn ::std::ops::Fn(Text) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    txt = tplCallHandleErrors(inFun.clone(), txt.clone())?;
    Ok(txt)
}

pub fn tplCallWithFailError<ArgType1: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>, mut inArg: ArgType1, mut txt: Text) -> Result<Text> {
    pub type Tpl_Fun<ArgType1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    let mut arg: ArgType1;
    txt = tplCallHandleErrors(Arc::new({ let __pe_b1 = inArg.clone(); move |__pe_a0| inFun(__pe_a0, __pe_b1.clone()) }), txt.clone())?;
    Ok(txt)
}

pub fn tplCallWithFailError2<ArgType1: Clone + 'static, ArgType2: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>, mut inArgA: ArgType1, mut inArgB: ArgType2, mut txt: Text) -> Result<Text> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    let mut argA: ArgType1;
    let mut argB: ArgType2;
    txt = tplCallHandleErrors(Arc::new({ let __pe_b1 = inArgA.clone(); let __pe_b2 = inArgB.clone(); move |__pe_a0| inFun(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), txt.clone())?;
    Ok(txt)
}

pub fn tplCallWithFailError3<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>, mut inArgA: ArgType1, mut inArgB: ArgType2, mut inArgC: ArgType3, mut txt: Text) -> Result<Text> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>;

    let mut txt: Text = txt;
    txt = tplCallHandleErrors(Arc::new({ let __pe_b1 = inArgA.clone(); let __pe_b2 = inArgB.clone(); let __pe_b3 = inArgC.clone(); move |__pe_a0| inFun(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }), txt.clone())?;
    Ok(txt)
}

pub fn tplString<ArgType1: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>, mut inArg: ArgType1) -> Result<ArcStr> {
    pub type Tpl_Fun<ArgType1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    let mut txt: Text;
    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError(inFun.clone(), inArg.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    outString = (textString(txt.clone())?).clone();
    Ok(outString)
}

pub fn tplString2<ArgType1: Clone + 'static, ArgType2: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>, mut inArgA: ArgType1, mut inArgB: ArgType2) -> Result<ArcStr> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    let mut txt: Text;
    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError2(inFun.clone(), inArgA.clone(), inArgB.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    outString = (textString(txt.clone())?).clone();
    Ok(outString)
}

pub fn tplString3<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>, mut inArgA: ArgType1, mut inArgB: ArgType2, mut inArgC: ArgType3) -> Result<ArcStr> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>;

    let mut outString: ArcStr = arcstr::literal!("");
    let mut txt: Text;
    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError3(inFun.clone(), inArgA.clone(), inArgB.clone(), inArgC.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    outString = (textString(txt.clone())?).clone();
    Ok(outString)
}

pub fn tplPrint<ArgType1: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>, mut inArg: ArgType1) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>;

    let mut txt: Text;
    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError(inFun.clone(), inArg.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    textStringBuf(txt.clone())?;
    Ok(())
}

pub fn tplPrint2<ArgType1: Clone + 'static, ArgType2: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>, mut inArgA: ArgType1, mut inArgB: ArgType2) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>;

    let mut txt: Text;
    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError2(inFun.clone(), inArgA.clone(), inArgB.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    textStringBuf(txt.clone())?;
    Ok(())
}

pub fn tplPrint3<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>, mut inArgA: ArgType1, mut inArgB: ArgType2, mut inArgC: ArgType3) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>;

    let mut txt: Text;
    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    txt = tplCallWithFailError3(inFun.clone(), inArgA.clone(), inArgB.clone(), inArgC.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    textStringBuf(txt.clone())?;
    Ok(())
}

pub fn tplNoret3<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>, mut inArg: ArgType1, mut inArg2: ArgType2, mut inArg3: ArgType3) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static, ArgType3: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2, ArgType3) -> Result<Text> + 'static>;

    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    let _ = tplCallWithFailError3(inFun.clone(), inArg.clone(), inArg2.clone(), inArg3.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    Ok(())
}

pub fn tplNoret2<ArgType1: Clone + 'static, ArgType2: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>, mut inArg: ArgType1, mut inArg2: ArgType2) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static, ArgType2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1, ArgType2) -> Result<Text> + 'static>;

    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    let _ = tplCallWithFailError2(inFun.clone(), inArg.clone(), inArg2.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    Ok(())
}

pub fn tplNoret<ArgType1: Clone + 'static>(mut inFun: Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>, mut inArg: ArgType1) -> Result<()> {
    pub type Tpl_Fun<ArgType1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Text, ArgType1) -> Result<Text> + 'static>;

    let mut nErr: i32 = 0;
    nErr = Error::getNumErrorMessages();
    let _ = tplCallWithFailError(inFun.clone(), inArg.clone(), emptyTxt.clone())?;
    failIfTrue(Error::getNumErrorMessages() > nErr.clone())?;
    Ok(())
}

pub fn textFile(mut inText: Text, mut inFileName: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inText.clone(), inFileName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, mut file) = __mc_input.clone() else { bail!("nomatch") };
            let mut rtTickTxt: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rtTickW: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rtTickTxt = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
            Print::clearBuf();
            textStringBuf(txt.clone())?;
            rtTickW = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
            Print::writeBuf((file.clone()).clone())?;
            if Testsuite::isRunning()? {
                System::appendFile((Testsuite::getTempFilesFile()?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            Print::clearBuf();
            if Flags::isSet(Flags::TPL_PERF_TIMES.clone())? {
                Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("textFile ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("\n    text:")); __mm_s.push_str(&*realString((rtTickW.clone()) - (rtTickTxt.clone()))); __mm_s.push_str(&*literal!("\n   write:")); __mm_s.push_str(&*realString((System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?) - (rtTickW.clone()))); ArcStr::from(__mm_s) }).clone())?;
            }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace((literal!("-!!!Tpl.textFile failed - a system error ?\n")).clone())?;
            }
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn textFileConvertLines(mut inText: Text, mut inFileName: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = (inText.clone(), inFileName.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, mut file) = __mc_input.clone() else { bail!("nomatch") };
            let mut rtTickTxt: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut rtTickW: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            rtTickTxt = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
            Print::clearBuf();
            textStringBuf(txt.clone())?;
            rtTickW = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
            System::writeFile((file.clone()).clone(), (literal!("")).clone())?;
            if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
                Print::writeBufConvertLines((System::realpath((file.clone()).clone())?).clone())?;
            } else {
                Print::writeBuf((file.clone()).clone())?;
            }
            if Testsuite::isRunning()? {
                System::appendFile((Testsuite::getTempFilesFile()?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            Print::clearBuf();
            if Flags::isSet(Flags::TPL_PERF_TIMES.clone())? {
                Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("textFile ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("\n    text:")); __mm_s.push_str(&*realString((rtTickW.clone()) - (rtTickTxt.clone()))); __mm_s.push_str(&*literal!("\n   write:")); __mm_s.push_str(&*realString((System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?) - (rtTickW.clone()))); ArcStr::from(__mm_s) }).clone())?;
            }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.textFile failed - a system error ?\n")).clone())?;
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub fn sourceInfo(mut inFileName: ArcStr, mut inLineNum: i32, mut inColumnNum: i32) -> SourceInfo {
    let mut outSourceInfo: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    outSourceInfo = SourceInfo { fileName: (inFileName.clone()).clone(), isReadOnly: false, lineNumberStart: inLineNum.clone(), columnNumberStart: inColumnNum.clone(), lineNumberEnd: inLineNum.clone(), columnNumberEnd: inColumnNum.clone(), lastModification: metamodelica::OrderedFloat(0.0_f64) };
    outSourceInfo
}

//we do not import Error.addSourceMessage() directly
//because of list creation in Susan is not possible (yet by design)
pub fn addSourceTemplateError(mut inErrMsg: ArcStr, mut inInfo: SourceInfo) -> Result<()> {
    Error::addSourceMessage(Error::TEMPLATE_ERROR.clone(), list![(inErrMsg.clone()).clone()], inInfo.clone())?;
    Ok(())
}

//for completeness
fn addTemplateErrorFunc<T: Clone + 'static>(mut func: T) -> Result<()> {
    Error::addMessage(Error::TEMPLATE_ERROR_FUNC.clone(), list![((System::dladdr(func.clone())).0).clone()])?;
    Ok(())
}

pub fn addTemplateError(mut msg: ArcStr) -> Result<()> {
    Error::addMessage(Error::TEMPLATE_ERROR.clone(), list![(msg.clone()).clone()])?;
    Ok(())
}

pub fn redirectToFile(mut text: Text, mut fileName: ArcStr) -> Result<Text> {
    let mut text: Text = text;
    let mut file: File::File = File::File(File::noReference())?;
    if Testsuite::isRunning()? {
        System::appendFile((Testsuite::getTempFilesFile()?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    }
    File::open(file.clone(), (fileName.clone()).clone(), File::Mode::Write.clone());
    text = writeText(Text::FILE_TEXT { opaqueFile: File::getReference(file.clone()), nchars: Mutable::create(0), aind: Mutable::create(0), isstart: Mutable::create(true), blocksStack: Mutable::create(metamodelica::nil()) }, text.clone())?;
    Ok(text)
}

pub fn closeFile(mut text: Text) -> Text {
    let mut text: Text = text;
    let mut file: File::File = File::File(getTextOpaqueFile(text.clone()).unwrap()).unwrap();
    File::releaseReference(file.clone());
    text = emptyTxt.clone();
    text
}

pub fn booleanString(mut b: bool) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    s = ArcStr::from(::std::format!("{}", b.clone()));
    s
}

fn getTextOpaqueFile(mut text: Text) -> Result<Option<i32>> {
    let mut opaqueFile: Option<i32> = None;
    opaqueFile = (match text.clone() {
        Text::FILE_TEXT { .. } => var_field!(text.opaqueFile, Text::FILE_TEXT).clone(),
        _ => {
            Error::addInternalError((literal!("tokFile got non-file text input")).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
    Ok(opaqueFile)
}

fn stringFile(mut inText: Text, mut r#str: ArcStr, mut line: bool, mut recurseSeparator: bool) -> Result<()> {
    let mut file: File::File = File::File(getTextOpaqueFile(inText.clone())?)?;
    let mut nchars: i32 = 0;
    let mut iopts: Arc<IterOptions> = Arc::new(<IterOptions as ::std::default::Default>::default());
    let mut septok: Arc<StringToken> = Arc::new(StringToken::ST_NEW_LINE);
    let _ = (match inText.clone() {
        Text::FILE_TEXT { .. } => {
            handleTok(inText.clone())?;
            nchars = Mutable::access(var_field!(inText.nchars, Text::FILE_TEXT).clone());
            if !(line.clone()) {
                if Mutable::access(var_field!(inText.isstart, Text::FILE_TEXT).clone()) {
                    File::writeSpace(file.clone(), nchars.clone());
                    File::write(file.clone(), (r#str.clone()).clone());
                    Mutable::update(var_field!(inText.nchars, Text::FILE_TEXT).clone(), nchars.clone() + ((r#str.clone()).clone().len() as i32));
                    Mutable::update(var_field!(inText.isstart, Text::FILE_TEXT).clone(), false);
                } else {
                    File::write(file.clone(), (r#str.clone()).clone());
                    Mutable::update(var_field!(inText.nchars, Text::FILE_TEXT).clone(), nchars.clone() + ((r#str.clone()).clone().len() as i32));
                }
            } else {
                if Mutable::access(var_field!(inText.isstart, Text::FILE_TEXT).clone()) {
                    File::writeSpace(file.clone(), nchars.clone());
                } else {
                    Mutable::update(var_field!(inText.isstart, Text::FILE_TEXT).clone(), true);
                }
                File::write(file.clone(), (r#str.clone()).clone());
                Mutable::update(var_field!(inText.nchars, Text::FILE_TEXT).clone(), Mutable::access(var_field!(inText.aind, Text::FILE_TEXT).clone()));
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn newlineFile(mut inText: Text) -> Result<()> {
    let mut file: File::File = File::File(getTextOpaqueFile(inText.clone())?)?;
    let mut nchars: i32 = 0;
    let _ = (match inText.clone() {
        Text::FILE_TEXT { .. } => {
            File::write(file.clone(), (literal!("\n")).clone());
            Mutable::update(var_field!(inText.nchars, Text::FILE_TEXT).clone(), Mutable::access(var_field!(inText.aind, Text::FILE_TEXT).clone()));
            Mutable::update(var_field!(inText.isstart, Text::FILE_TEXT).clone(), true);
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn textFileTell(mut inText: Text) -> i32 {
    let mut tell: i32 = 0;
    let mut file: File::File = File::File(getTextOpaqueFile(inText.clone()).unwrap()).unwrap();
    tell = File::tell(file.clone());
    tell
}

fn handleTok(mut txt: Text) -> Result<()> {
    let mut septok: Arc<StringToken> = Arc::new(StringToken::ST_NEW_LINE);
    let mut aseptok: Mutable::Mutable<Option<Arc<StringToken>>>;
    let _ = (match txt.clone() {
        Text::FILE_TEXT { .. } => {
            let _ = (::match_deref::match_deref! { match &(Mutable::access(var_field!(txt.blocksStack, Text::FILE_TEXT).clone())) {
        Deref @ metamodelica::List::Cons { head: BlockTypeFileText { septok: aseptok, bt: Deref @ BlockType::BT_ITER { .. }, .. }, tail: _ } => {
            let _ = (::match_deref::match_deref! { match &(Mutable::access(aseptok.clone())) {
        Some(septok) => {
            Mutable::update(aseptok.clone(), None);
            tokFileText(txt.clone(), septok.clone(), false)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn debugSusan() -> Result<bool> {
    let mut b: bool = false;
    b = Flags::isSet(Flags::SUSAN_MATCHCONTINUE_DEBUG.clone())?;
    Ok(b)
}

pub fn fakeStackOverflow() -> Result<()> {
    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow:\n")); __mm_s.push_str(&*StackOverflow::generateReadableMessage(1000, 4, (literal!("\n")).clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    StackOverflow::triggerStackOverflow()?;
    Ok(())
}

