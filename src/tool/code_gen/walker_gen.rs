

use crate::tool::grammar::Grammar;
use tera::Context;

use super::{TEMPLATES, pascal};




pub fn walker_generate(grammar: &Grammar) -> String {


  let mut context = Context::new();
  context.insert("grammar_name", &(grammar.name.to_lowercase(), grammar.name.to_uppercase(), pascal(&grammar.name)));
  TEMPLATES.render("target/rust/walker.tera", &context).unwrap()
}



