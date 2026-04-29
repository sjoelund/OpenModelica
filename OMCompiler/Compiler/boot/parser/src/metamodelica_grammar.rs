use crate::generated::metamodelica_grammar_trait::MetaModelicaGrammarTrait;

pub struct MetaModelicaGrammar<'t> {
    pub class_names: Vec<&'t str>,
}

impl<'t> MetaModelicaGrammar<'t> {
    pub fn new() -> Self {
        Self {
            class_names: Vec::new(),
        }
    }
}

impl<'t> MetaModelicaGrammarTrait<'t> for MetaModelicaGrammar<'t> {}
