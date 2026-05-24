// Tests for openmodelica_frontend_dump::Dump::unparseStr
//
// The Rust source is auto-generated from:
//   ~/OpenModelica/OMCompiler/Compiler/ModelicaBuiltin.mo (and AbsynDumpTpl.tpl)
//
// unparseStr round-trips a parsed Absyn::Program back to Modelica source text.
// Tests here parse a small model string, call unparseStr, and assert properties
// on the output string.
//
// Known bugs detected while writing these tests are documented inline with
// "Bug:" prefixes.

use anyhow::Result;
use std::sync::Arc;
use metamodelica::*;
use openmodelica_ast::parser::{parse, Grammar};
use openmodelica_frontend_dump::Dump;
use openmodelica_util::{FlagsUtil, Flags};

// ---------------------------------------------------------------------------
// Flags initialisation — required because unparseStr calls
// Flags::getConfigBool / FlagsUtil::setConfigBool internally.
// ---------------------------------------------------------------------------

fn init_flags() {
    let config_vec: Vec<Flags::FlagData> = FlagsUtil::allConfigFlags
        .clone()
        .into_iter()
        .cloned()
        .map(|f| f.defaultValue.clone())
        .collect();
    let debug_vec: Vec<bool> = FlagsUtil::allDebugFlags
        .clone()
        .into_iter()
        .cloned()
        .map(|f| f.default)
        .collect();
    FlagsUtil::saveFlags(Flags::Flag::FLAGS {
        debugFlags: metamodelica::arrayFromVec(debug_vec),
        configFlags: metamodelica::arrayFromVec(config_vec),
    });
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Parse `src` with the MetaModelica grammar and return the Absyn::Program.
fn parse_prog(src: &str) -> openmodelica_ast::Absyn::Program {
    parse(src, "test.mo", Grammar::MetaModelica)
        .unwrap_or_else(|e| panic!("parse failed: {}", e))
}

/// Parse `src` and call unparseStr with default options (no markup).
fn unparse(src: &str) -> Result<arcstr::ArcStr> {
    init_flags();
    let prog = parse_prog(src);
    Dump::unparseStr(prog, false, Dump::defaultDumpOptions.clone())
}

// ---------------------------------------------------------------------------
// smoke test — empty program (no classes)
// ---------------------------------------------------------------------------

/// An empty stored_definition (just `within;`) has no classes;
/// unparseStr should return an empty string.
#[test]
fn unparse_str_empty_program_returns_empty() -> Result<()> {
    init_flags();
    let prog = openmodelica_ast::Absyn::Program {
        classes: metamodelica::nil(),
        within_: openmodelica_ast::Absyn::Within::TOP,
    };
    let out = Dump::unparseStr(prog, false, Dump::defaultDumpOptions.clone())?;
    assert_eq!(out, "", "empty program should produce empty string, got: {:?}", out);
    Ok(())
}

// ---------------------------------------------------------------------------
// simple empty model
// ---------------------------------------------------------------------------

#[test]
fn unparse_str_empty_model_contains_keyword_and_name() -> Result<()> {
    let out = unparse("model Empty end Empty;")?;
    assert!(out.contains("model"), "expected 'model' keyword, got: {:?}", out);
    assert!(out.contains("Empty"), "expected class name 'Empty', got: {:?}", out);
    assert!(out.contains("end"), "expected 'end' keyword, got: {:?}", out);
    Ok(())
}

#[test]
fn unparse_str_empty_model_ends_with_semicolon() -> Result<()> {
    let out = unparse("model Empty end Empty;")?;
    // The model declaration should be terminated by a semicolon.
    assert!(
        out.trim_end().ends_with(';'),
        "expected output to end with ';', got: {:?}", out
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// package with a nested model
// ---------------------------------------------------------------------------

#[test]
fn unparse_str_package_preserves_name() -> Result<()> {
    let src = "package MyPkg end MyPkg;";
    let out = unparse(src)?;
    assert!(out.contains("package"), "expected 'package' keyword, got: {:?}", out);
    assert!(out.contains("MyPkg"), "expected 'MyPkg', got: {:?}", out);
    Ok(())
}

#[test]
fn unparse_str_nested_class_names_present() -> Result<()> {
    let src = "package Outer model Inner end Inner; end Outer;";
    let out = unparse(src)?;
    assert!(out.contains("Outer"), "expected 'Outer', got: {:?}", out);
    assert!(out.contains("Inner"), "expected 'Inner', got: {:?}", out);
    assert!(out.contains("model"), "expected 'model' keyword, got: {:?}", out);
    assert!(out.contains("package"), "expected 'package' keyword, got: {:?}", out);
    Ok(())
}

// ---------------------------------------------------------------------------
// connector
// ---------------------------------------------------------------------------

#[test]
fn unparse_str_connector_keyword_preserved() -> Result<()> {
    let src = "connector Pin Real v; flow Real i; end Pin;";
    let out = unparse(src)?;
    assert!(out.contains("connector"), "expected 'connector' keyword, got: {:?}", out);
    assert!(out.contains("Pin"), "expected 'Pin', got: {:?}", out);
    Ok(())
}

// ---------------------------------------------------------------------------
// function
// ---------------------------------------------------------------------------

#[test]
fn unparse_str_function_keyword_preserved() -> Result<()> {
    let src = "function add input Real a; input Real b; output Real c; algorithm c := a + b; end add;";
    let out = unparse(src)?;
    assert!(out.contains("function"), "expected 'function' keyword, got: {:?}", out);
    assert!(out.contains("add"), "expected function name 'add', got: {:?}", out);
    Ok(())
}

// ---------------------------------------------------------------------------
// within clause — Bug suspected: verify that the within path is emitted
// ---------------------------------------------------------------------------

/// The within clause (non-TOP) should appear in the output as "within <path>;".
/// Bug: if the template does not emit the within clause, this test will fail.
#[test]
fn unparse_str_within_clause_present() -> Result<()> {
    init_flags();
    // Build a program with a within clause manually, since the parser in
    // MetaModelica grammar may not support top-level `within` directives
    // for all model variants.  We construct the AST directly.
    use openmodelica_ast::Absyn;
    let dummy_info = metamodelica::SourceInfo {
        fileName: arcstr::literal!("test.mo"),
        isReadOnly: false,
        lineNumberStart: 1,
        columnNumberStart: 1,
        lineNumberEnd: 1,
        columnNumberEnd: 1,
        lastModification: metamodelica::OrderedFloat(0.0),
    };
    let within_path = Arc::new(Absyn::Path::IDENT { name: arcstr::literal!("Foo") });
    let prog = Absyn::Program {
        classes: metamodelica::list![Arc::new(Absyn::Class {
            name: arcstr::literal!("Bar"),
            partialPrefix: false,
            finalPrefix: false,
            encapsulatedPrefix: false,
            restriction: Absyn::Restriction::R_MODEL,
            body: Arc::new(Absyn::ClassDef::PARTS {
                typeVars: metamodelica::nil(),
                classAttrs: metamodelica::nil(),
                classParts: metamodelica::nil(),
                ann: metamodelica::nil(),
                comment: None,
            }),
            commentsBeforeClass: metamodelica::nil(),
            commentsBeforeEnd: metamodelica::nil(),
            commentsAfterEnd: metamodelica::nil(),
            info: dummy_info,
        })],
        within_: Absyn::Within::WITHIN { path: within_path },
    };
    let out = Dump::unparseStr(prog, false, Dump::defaultDumpOptions.clone())?;
    assert!(
        out.contains("within"),
        "expected 'within' keyword in output, got: {:?}", out
    );
    assert!(
        out.contains("Foo"),
        "expected within path 'Foo' in output, got: {:?}", out
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// idempotency: unparse(parse(unparse(parse(s)))) == unparse(parse(s))
// ---------------------------------------------------------------------------

/// Parsing and unparsing a simple model twice should give the same string on
/// the second pass (the template should be deterministic and idempotent).
/// Bug: if the formatter introduces or drops tokens on successive passes this
/// test will fail, surfacing a round-trip instability.
#[test]
fn unparse_str_idempotent_for_simple_model() -> Result<()> {
    let src = "model M end M;";
    let pass1 = unparse(src)?;
    // Re-parse the output of the first pass and unparse again.
    // If pass1 is empty or malformed this will panic at parse, revealing a bug.
    let pass2 = unparse(&pass1)?;
    assert_eq!(
        pass1, pass2,
        "unparseStr should be idempotent but pass1 != pass2:\npass1={:?}\npass2={:?}",
        pass1, pass2
    );
    Ok(())
}
