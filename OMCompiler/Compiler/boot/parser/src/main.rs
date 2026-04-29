mod metamodelica;

#[allow(unused_parens)]
pub mod generated {
    pub mod metamodelicalexer;
    pub mod metamodelicaparser;
    pub mod metamodelicalistener; // optional, if you use listeners
}

use anyhow::{Context, Result, bail};
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::errors::ANTLRError;
use antlr4rust::parser::Parser;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token_factory::TokenFactory;
use antlr4rust::tree::ParseTree;
use antlr4rust::InputStream;
use crate::generated::metamodelicaparser::Class_definition_listContextAttrs;
use crate::generated::*;
use crate::generated::metamodelicaparser::Stored_definitionContextAttrs;
use crate::generated::metamodelicaparser::Class_definitionContextAttrs;
use crate::generated::metamodelicaparser::Class_specifierContextAttrs;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::{self, BufRead};

struct CollectingErrorListener {
    errors: Rc<RefCell<Vec<String>>>,
}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for CollectingErrorListener {
    fn syntax_error(
        &self, _: &T,
        _: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize, column: isize, msg: &str,
        _: Option<&ANTLRError>,
    ) {
        self.errors.borrow_mut().push(format!("{}:{} {}", line, column, msg));
    }
}


pub fn parse_modelica(input: &str) -> Result<Rc<metamodelicaparser::Stored_definitionContext<'_>>, Box<dyn std::error::Error>> {
    let lexer = metamodelicalexer::metamodelicaLexer::new(InputStream::new(input));
    let tokens = CommonTokenStream::new(lexer);
    let mut parser = metamodelicaparser::metamodelicaParser::new(tokens);

    let error_list: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(CollectingErrorListener { errors: Rc::clone(&error_list) }));

    let tree = parser.stored_definition()?;

    let errs = error_list.borrow();
    if !errs.is_empty() {
        return Err(format!("{}", errs.join("\n")).into());
    }
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

fn parse_mm_files() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("compilerSources.txt")?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let path = line?;
        if path.ends_with(".mo") {
            let content = std::fs::read_to_string(&path)?;
            println!("Parsing {}... ", &path);
            _ = analyze(&content);
        }
    }
    Ok(())
}

fn main() {
    let metamodelica_code = r#"
        model SimpleSystem "Returns the index of the set the entry belongs to, or fails if the
   entry doesn't."
            Real x(start=0);
        equation
            der(x) = -x;
        end SimpleSystem;
    "#;

    if let Err(e) = analyze(metamodelica_code) {
        eprintln!("Parse failed: {}", e);
        return;
    };

    match parse_mm_files() {
        Ok(_) => println!("All files parsed successfully."),
        Err(e) => eprintln!("Error parsing files: {}", e),
    }
}
