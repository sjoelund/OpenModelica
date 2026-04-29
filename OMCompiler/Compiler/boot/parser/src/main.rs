mod metamodelica_grammar;
mod metamodelica_grammar_trait;
mod metamodelica_parser;

use anyhow::Result;
use metamodelica_grammar::MetaModelicaGrammar;
use std::fs::File;
use std::io::{self, BufRead};

fn parse_modelica<'t>(input: &'t str, file_name: &str) -> Result<MetaModelicaGrammar<'t>> {
    let mut grammar = MetaModelicaGrammar::new();
    metamodelica_parser::parse(input, file_name, &mut grammar)
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
    let metamodelica_code = r#"
        package SimpleSystem "Returns the index of the set the entry belongs to, or fails if the
   entry doesn't."
            Real x(start=0);
        end SimpleSystem;
    "#;

    match parse_modelica(metamodelica_code, "<inline>") {
        Ok(_) => println!("Parse succeeded."),
        Err(e) => {
            eprintln!("Parse failed: {}", e);
            return;
        }
    }

    match parse_mm_files() {
        Ok(_) => println!("All files parsed successfully."),
        Err(e) => eprintln!("Error parsing files: {}", e),
    }
}
