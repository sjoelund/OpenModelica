mod metamodelica;

#[allow(unused_parens)]
pub mod generated {
    pub mod metamodelicalexer;
    pub mod metamodelicaparser;
    pub mod metamodelicalistener; // optional, if you use listeners
}

use anyhow::{Context, Result};
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::tree::ParseTree;
use antlr4rust::InputStream;
use crate::generated::metamodelicaparser::Class_definition_listContextAttrs;
use crate::generated::*;
use crate::generated::metamodelicaparser::Stored_definitionContextAttrs;
use crate::generated::metamodelicaparser::Class_definitionContextAttrs;
use crate::generated::metamodelicaparser::Class_specifierContextAttrs;
use std::rc::Rc;

pub fn parse_modelica(input: &str) -> Result<Rc<metamodelicaparser::Stored_definitionContext<'_>>, Box<dyn std::error::Error>> {
    let lexer = metamodelicalexer::metamodelicaLexer::new(InputStream::new(input));
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = metamodelicaparser::metamodelicaParser::new(tokens);

    // Optional: suppress ANTLR's default stderr error printing
    // parser.remove_error_listeners();

    // Call the start rule method
    let tree = parser.stored_definition()?;
    Ok(tree)
}

pub fn analyze(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree = parse_modelica(input)?;

    // stored_definition -> class_definition*
    let ctx = tree.class_definition_list().context("")?;
    for class_def in ctx.class_definition_all() {
        let spec = class_def.class_specifier().context("")?;
        let id = spec.identifier().context("")?;
        println!("class: {}", id.get_text());
    }
    Ok(())
}

fn main() {
    let metamodelica_code = r#"
        model SimpleSystem
            Real x(start=0);
        equation
            der(x) = -x;
        end SimpleSystem;
    "#;

    if let Err(e) = analyze(metamodelica_code) {
        eprintln!("Parse failed: {}", e);
    }
}
