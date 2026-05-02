We are writing a grammar for MetaModelica in Rust.

ANTLR3 grammar we are basing it on: grammars/Modelica.g

Use idiomatic winnow expressions, such as:
`opt(...).parse_next(input)`
`peek(opt(...)).parse_next(input)`

We want the parser to take tokens as input, but it currently takes strings.

Write the lexer that creates a stream of tokens without whitespace or line/block-comments.

mmwinnow/src/Absyn.rs contains the declaration of the AST to return

You can check the code using `cd boot/parser/ && cargo check -p mmwinnow`
