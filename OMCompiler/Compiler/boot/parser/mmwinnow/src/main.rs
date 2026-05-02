use mmwinnow::lexer::lex;
use mmwinnow::Grammar;

fn main() {
   let code = std::fs::read_to_string("/home/martin/OpenModelica/OMCompiler/Compiler/Template/CodegenC.mo").
            expect("CodegenC.mo not found");
    /*let result = lex(&code, Grammar::MetaModelica);
    if let Some(err) = &result.err() {
        assert!(false, "expected CodegenC.mo to lex, got: {}", err);
    }*/

    return;
}
