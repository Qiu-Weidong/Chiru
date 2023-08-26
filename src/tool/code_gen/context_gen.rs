
// use std::task::Context;

use tera::Context;

use crate::tool::grammar::Grammar;

use super::{TEMPLATES, pascal};


// 生成 context 文件 生成某一个非终结符的 context
fn ctx_gen(grammar: &Grammar, rule_id: usize) -> String {
  let rule_name = grammar.vocabulary.get_nonterminal_name_by_id(rule_id).unwrap();








  let mut context = Context::new();
  context.insert("grammar_name", &(grammar.name.to_lowercase(), grammar.name.to_uppercase(), pascal(&grammar.name)));
  context.insert("ctx_name", &(rule_name.to_lowercase(), rule_name.to_uppercase(), pascal(&rule_name)));




  TEMPLATES.render("target/rust/context.tera", &context).unwrap()
}


pub fn context_generate(grammar: &Grammar) -> String {
  let nonterminals = grammar.vocabulary.get_all_named_nonterminals();
  let ctx_list = nonterminals.iter()
    .map(|id| ctx_gen(grammar, *id)).collect::<Vec<_>>();
  let mut context = Context::new();
  context.insert("ctx_list", &ctx_list);
  context.insert("grammar_name", &(grammar.name.to_lowercase(), grammar.name.to_uppercase(), pascal(&grammar.name)));

  TEMPLATES.render("target/rust/context.tera", &context).unwrap()
}










