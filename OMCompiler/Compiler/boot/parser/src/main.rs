mod metamodelica;

#[allow(unused_parens)]
pub mod generated {
    pub mod modelicalexer;
    pub mod modelicaparser;
    pub mod modelicalistener; // optional, if you use listeners
}

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::tree::ParseTree;
use antlr4rust::InputStream;
use crate::generated::*;
use crate::generated::modelicaparser::Stored_definitionContextAttrs;
use crate::generated::modelicaparser::Class_definitionContextAttrs;
use crate::generated::modelicaparser::Class_specifierContextAttrs;
use crate::generated::modelicaparser::Long_class_specifierContextAttrs;
use crate::generated::modelicaparser::Short_class_specifierContextAttrs;
use crate::generated::modelicaparser::Der_class_specifierContextAttrs;
use std::rc::Rc;

pub fn parse_modelica(input: &str) -> Result<Rc<modelicaparser::Stored_definitionContext<'_>>, Box<dyn std::error::Error>> {
    let lexer = modelicalexer::modelicaLexer::new(InputStream::new(input));
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = modelicaparser::modelicaParser::new(tokens);

    // Optional: suppress ANTLR's default stderr error printing
    // parser.remove_error_listeners();

    // Call the start rule method
    let tree = parser.stored_definition()?;
    Ok(tree)
}

pub fn analyze(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree = parse_modelica(input)?;

    // stored_definition -> class_definition*
    for class_def in tree.class_definition_all() {
        if let Some(spec) = class_def.class_specifier() {
            if let Some(ctx) = spec.long_class_specifier() {
                println!("Long class: {}", ctx.IDENT(0).unwrap().get_text());
            } else if let Some(ctx) = spec.short_class_specifier() {
                println!("Short class: {}", ctx.IDENT().unwrap().get_text());
            } else if let Some(ctx) = spec.der_class_specifier() {
                println!("DER class: {}", ctx.IDENT(0).unwrap().get_text());
            }
        }
    }
    Ok(())
}

fn main() {
    let modelica_code = r#"
        model SimpleSystem
            Real x(start=0);
        equation
            der(x) = -x;
        end SimpleSystem;
    "#;

    if let Err(e) = analyze(modelica_code) {
        eprintln!("Parse failed: {}", e);
    }
}
