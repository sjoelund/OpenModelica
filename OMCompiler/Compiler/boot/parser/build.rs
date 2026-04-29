use std::fs;
use std::path::PathBuf;

fn main() {
    let grammar_file = "src/metamodelica.par";
    let out_dir = PathBuf::from("src/generated");
    let parser_file = out_dir.join("metamodelica_parser.rs");

    println!("cargo:rerun-if-changed={}", grammar_file);

    fs::create_dir_all(&out_dir).unwrap();

    parol::build::Builder::with_explicit_output_dir(&out_dir)
        .grammar_file(grammar_file)
        .parser_output_file("metamodelica_parser.rs")
        .actions_output_file("metamodelica_grammar_trait.rs")
        .user_type_name("MetaModelicaGrammar")
        .user_trait_module_name("metamodelica_grammar")
        .generate_parser()
        .unwrap();

    // Fix broken regex patterns emitted by parol's scanner generator.
    // Special regex chars in literal token patterns must be escaped.
    let content = fs::read_to_string(&parser_file).unwrap();
    let content = fix_scanner_regexes(&content);
    // The trait file moved to src/generated/; update the import path.
    let content = content.replace(
        "use crate::metamodelica_grammar_trait::",
        "use crate::generated::metamodelica_grammar_trait::",
    );
    fs::write(&parser_file, content).unwrap();
}

fn fix_scanner_regexes(s: &str) -> String {
    // Each replacement is (broken_pattern, fixed_pattern) as they appear
    // in the raw-string token r"..." lines of the scanner! macro.
    let fixes: &[(&str, &str)] = &[
        // Block comment
        (r#"r"/*([^*]|*[^/])**/" "#,  r#"r"/\*([^*]|\*[^/])*\*/" "#),
        // Grouping chars
        (r#"r"(""#,                  r#"r"\(""#),
        (r#"r")""#,                  r#"r"\)""#),
        (r#"r"[""#,                  r#"r"\[""#),
        (r#"r"]""#,                  r#"r"\]""#),
        (r#"r"{""#,                  r#"r"\{""#),
        (r#"r"}""#,                  r#"r"\}""#),
        // Dot-prefixed operators (must come before bare "." and bare "*"/"+")
        (r#"r".*""#,                 r#"r"\.\*""#),
        (r#"r".+""#,                 r#"r"\.\+""#),
        (r#"r".-""#,                 r#"r"\.-""#),
        (r#"r"./""#,                 r#"r"\.\/""#),
        (r#"r".^""#,                 r#"r"\.\^""#),
        // Bare operators
        (r#"r"*""#,                  r#"r"\*""#),
        (r#"r"+""#,                  r#"r"\+""#),
        (r#"r"^""#,                  r#"r"\^""#),
        // Literal dot (Dot token, id 91) – leave the catch-all Error token alone
        (r#"r"." => 91;"#,           r#"r"\." => 91;"#),
    ];

    let mut result = s.to_owned();
    for (from, to) in fixes {
        result = result.replace(from, to);
    }
    result
}
