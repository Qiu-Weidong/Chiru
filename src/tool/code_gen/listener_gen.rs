use tera::Context;

use crate::tool::{grammar::Grammar, code_gen::pascal};

use super::TEMPLATES;





// 生成 listener
pub fn listener_generate(grammar: &Grammar) -> String {

  let mut nonterminals: Vec<(usize, String, String, String)> = Vec::new();
  for (id, t) in grammar.vocabulary.nonterminals.iter() {
    if let Some(name) = &t.name {
      nonterminals.push((*id, name.clone(), name.to_uppercase(), pascal(&name)));
    }
  }

  let mut context = Context::new();
  context.insert("nonterminals", &nonterminals);
  context.insert("grammar_name", &(grammar.name.to_lowercase(), grammar.name.to_uppercase(), pascal(&grammar.name)));
  TEMPLATES.render("target/rust/listener.tera", &context).unwrap()

}
