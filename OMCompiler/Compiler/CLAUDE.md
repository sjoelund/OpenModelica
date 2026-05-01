We are writing a grammar for MetaModelica in Rust.

ANTLR3 grammar we are basing it on: grammars/Modelica.g

Use idiomatic winnow expressions, such as:
`opt("class").parse_next(input)`
`peek(keyword_or_ident).parse_next(input)`

Note that using `"class".parse_next(input)` requires `skip_trivia` first (it does not handle comments and whitespace)

MetaModelica keywords are case-sensitive. You can even check the lexers if you want to see the exact tokens.

Avoid input.startswith() - it does not know about whitespace and comments.

skip_trivia should not be needed if you use the winnow idioms

Mark cases not covered with TODO, and try to add those when you are done with your current task.

Please follow the structure of the ANTLR grammar to keep code duplication to a minimum.
Note that ANTLR3 uses recursion a lot - favor repetition instead.

mmwinnow/src/Absyn.rs contains the declaration of the AST to return

You can check the code using `cd boot/parser/ && cargo check -p mmwinnow`
