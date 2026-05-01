We are writing a grammar for MetaModelica in Rust.

ANTLR3 grammar we are basing it on: boot/parser/grammars/Modelica.g

Use idiomatic winnow expressions, such as:
opt("class").parse_next(input)

Do not check input.startswith() - it does not know about whitespace and comments.

rather than trying to manipulate the input string directly

skip_trivia should not be needed if you use the winnow idioms

Mark cases not covered with TODO, and try to add those when you are done with your current task.

Please follow the structure of the ANTLR grammar to keep code duplication to a minimum.
Note that ANTLR3 uses recursion a lot - favor repetition instead.

Skip creating the AST for now, focusing only on parsing.
We will later mimic the AST listed in Modelica.g (which is the same as mmwinnow/tests/data/Absyn.mo)

You can check the code using `cd boot/parser/ && cargo check -p mmwinnow`
