We are writing a grammar for MetaModelica in Rust.

ANTLR3 grammar we are basing it on: grammars/Modelica.g

Please follow the structure of the grammar to keep code duplication to a minimum

The AST to create should mimic the one listed in Modelica.g (which is the same as mmwinnow/tests/data/Absyn.mo)

You can check the code using cargo check -p mmwinnow
