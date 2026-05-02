pub mod grammar {
    pub mod metamodelica_grammar;
    pub mod metamodelica_grammar_trait;
    pub mod metamodelica_parser;
}

use anyhow::Result;
use grammar::metamodelica_grammar::MetaModelicaGrammar;
use std::fs::File;
use std::io::{self, BufRead};
use mmwinnow::parser::stored_definition;

fn parse_modelica<'t>(input: &'t str, file_name: &str) -> Result<MetaModelicaGrammar<'t>> {
    let mut grammar = MetaModelicaGrammar::new();
    grammar::metamodelica_parser::parse(input, file_name, &mut grammar)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    Ok(grammar)
}

fn parse_mm_files() -> Result<()> {
    let file = File::open("compilerSources.txt")?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let path = line?;
        if path.ends_with(".mo") {
            let content = std::fs::read_to_string(&path)?;
            let t0 = std::time::Instant::now();
            println!("Parsing {}...", &path);
            let result = parse_modelica(&content, &path);
            println!("Parsed {}... {:.3}s", &path, t0.elapsed().as_secs_f64());
            if let Err(e) = result {
                eprintln!("  Error: {}", e);
            }
        }
    }
    Ok(())
}

fn main() {
   let code = std::fs::read_to_string("/home/martin/OpenModelica/OMCompiler/Compiler/Template/CodegenC.mo").
            expect("CodegenC.mo not found");
    let result = stored_definition.parse(&*code);
    if let Some(err) = &result.err() {
        assert!(false, "expected CodegenC.mo to parse, got: {}", err);
    }

    return;
}
