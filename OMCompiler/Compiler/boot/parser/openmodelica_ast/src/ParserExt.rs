// Manually written file.
//
// Rust port of `OMCompiler/Compiler/FrontEnd/ParserExt.mo`'s
// `external "C"` declarations.  The MetaModelica module is a thin shim
// over the C entry points defined in `OMCompiler/Parser/Parser_omc.c`
// (which in turn drive the ANTLR3 grammar at `grammars/Modelica.g`).
//
// Here we forward to the winnow-based parser already living in the
// same crate at `crate::parser`, so callers like `Parser.mo` /
// `openmodelica_frontend::Parser` keep working without going through
// any C runtime.
//
// Grammar selection (`acceptedGram`) follows the integer encoding used
// by `Flags.GRAMMAR` (see `OMCompiler/Compiler/Util/Flags.mo:154-158`):
//
//   1 = Modelica       → `Grammar::Modelica2` if `languageStandardInt < 30`
//                        otherwise `Grammar::Modelica3`
//   2 = MetaModelica   → `Grammar::MetaModelica`
//   3 = ParModelica    → `Grammar::MetaModelica`     (parmodelica keywords are
//                        lexed by the MetaModelica lexer in mmwinnow)
//   4 = Optimica       → `Grammar::Optimica`
//   5 = PDEModelica    → `Grammar::Modelica3`        (no dedicated grammar yet)
//
// Functions that are not yet implementable on top of the current parser
// (e.g. statement / .mos parsing, the library-vendor-executable hooks)
// stay as `todo!()` with a comment so the gap is explicit.

#![allow(non_snake_case)]

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use arcstr::ArcStr;

use crate::Absyn;
use crate::GlobalScript;
use crate::parser::{self, Grammar};

/// Map `(acceptedGram, languageStandardInt)` to the parser's [`Grammar`]
/// enum. Mirrors the `set_grammar_flag` switch in
/// `OMCompiler/Parser/Parser_omc.c`.
fn select_grammar(acceptedGram: i32, languageStandardInt: i32) -> Grammar {
    match acceptedGram {
        2 | 3 => Grammar::MetaModelica,
        4 => Grammar::Optimica,
        // 1 = Modelica, 5 = PDEModelica, and anything unknown falls back
        // to the Modelica grammar.  The language-standard integer follows
        // `Flags.LANGUAGE_STANDARD`: values 10/20 are Modelica 1.x / 2.x,
        // 30+ are Modelica 3.x.
        _ => {
            if languageStandardInt < 30 {
                Grammar::Modelica2
            } else {
                Grammar::Modelica3
            }
        }
    }
}

/// Wrap [`parser::parse`]'s `Box<dyn Error>` into an `anyhow::Error` so
/// the MetaModelica-facing signatures (which return `anyhow::Result`)
/// can use `?` directly.
fn run_parse(src: &str, filename: &str, grammar: Grammar) -> Result<Absyn::Program> {
    parser::parse(src, filename, grammar).map_err(|e| anyhow!(e.to_string()))
}

pub fn parse(
    filename: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    encoding: ArcStr,
    languageStandardInt: i32,
    _strict: bool,
    _runningTestsuite: bool,
    _libraryPath: ArcStr,
    _lveInstance: Option<i32>,
) -> Result<Absyn::Program> {
    // The Rust parser operates on UTF-8 `&str` directly.  Anything else
    // would need transcoding via the `encoding_rs` crate; bail explicitly
    // rather than silently misinterpreting the bytes.
    if !encoding.is_empty() && !encoding.eq_ignore_ascii_case("UTF-8") && !encoding.eq_ignore_ascii_case("UTF8") {
        return Err(anyhow!(
            "ParserExt::parse: only UTF-8 input is supported, got encoding {:?}",
            encoding.as_str()
        ));
    }
    let src = std::fs::read_to_string(filename.as_str())
        .with_context(|| format!("ParserExt::parse: cannot read {filename}"))?;
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    run_parse(&src, infoFilename.as_str(), grammar)
}

pub fn parsestring(
    r#str: ArcStr,
    infoFilename: ArcStr,
    acceptedGram: i32,
    languageStandardInt: i32,
    _strict: bool,
    _runningTestsuite: bool,
) -> Result<Absyn::Program> {
    let grammar = select_grammar(acceptedGram, languageStandardInt);
    run_parse(r#str.as_str(), infoFilename.as_str(), grammar)
}

// ---------------------------------------------------------------------
// The remaining entry points are interactive-mode helpers (parse a
// single expression / path / cref / equation, or drive the .mos script
// front-end).  The winnow parser currently only exposes a top-level
// `stored_definition` entry point; the per-construct entry points need
// to be added before these can be implemented properly.  Leaving them
// as `todo!()` keeps callers honest instead of silently returning
// garbage AST.
// ---------------------------------------------------------------------

pub fn parseexp(
    _filename: ArcStr,
    _infoFilename: ArcStr,
    _acceptedGram: i32,
    _languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<GlobalScript::Statements> {
    todo!("ParserExt::parseexp: .mos / interactive statement parsing not yet ported")
}

pub fn parsestringexp(
    _str: ArcStr,
    _infoFilename: ArcStr,
    _acceptedGram: i32,
    _languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<GlobalScript::Statements> {
    todo!("ParserExt::parsestringexp: interactive statement parsing not yet ported")
}

pub fn stringPath(
    _str: ArcStr,
    _infoFilename: ArcStr,
    _acceptedGram: i32,
    _languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<Arc<Absyn::Path>> {
    todo!("ParserExt::stringPath: no `name_path` entry point exposed on the parser yet")
}

pub fn stringCref(
    _str: ArcStr,
    _infoFilename: ArcStr,
    _acceptedGram: i32,
    _languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<Arc<Absyn::ComponentRef>> {
    todo!("ParserExt::stringCref: no `component_reference` entry point exposed on the parser yet")
}

pub fn stringMod(
    _str: ArcStr,
    _infoFilename: ArcStr,
    _acceptedGram: i32,
    _languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Result<Arc<Absyn::ElementArg>> {
    todo!("ParserExt::stringMod: no `element_modification` entry point exposed on the parser yet")
}

pub fn stringEq(
    _str: ArcStr,
    _infoFilename: ArcStr,
    _acceptedGram: i32,
    _languageStandardInt: i32,
    _runningTestsuite: bool,
) -> Arc<Absyn::EquationItem> {
    todo!("ParserExt::stringEq: no `equation` entry point exposed on the parser yet")
}

// ---------------------------------------------------------------------
// Library Vendor Executable (LVE) hooks.  These wrap a proprietary
// shared library used by some commercial libraries to validate license
// tokens; OpenModelica's open-source builds disable the feature by
// returning "not started".  Mirror that behaviour here so unrelated
// flows still type-check without dragging in dlopen plumbing.
// ---------------------------------------------------------------------

pub fn startLibraryVendorExecutable(_lvePath: ArcStr) -> (bool, Option<i32>) {
    (false, None)
}

pub fn checkLVEToolLicense(_lveInstance: Option<i32>, _packageName: ArcStr) -> bool {
    false
}

pub fn checkLVEToolFeature(_lveInstance: Option<i32>, _feature: ArcStr) -> bool {
    false
}

pub fn stopLibraryVendorExecutable(_lveInstance: Option<i32>) -> () {
    ()
}
