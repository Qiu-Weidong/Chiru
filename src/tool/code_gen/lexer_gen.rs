use std::collections::HashMap;

use tera::Context;

use crate::tool::{visitor::lexer_rule::LexerRule, code_gen::TEMPLATES};

use super::pascal;



pub fn lexer_generate(lexer_rule_map: &HashMap<String, LexerRule>, grammar_name: &str) -> String {
  let mut context = Context::new();

  let mut lexer_rules = lexer_rule_map.values().map(|v| {
    (
      v.token_name.to_lowercase(), 
      v.token_name.to_uppercase(), 
      pascal(&v.token_name),
      v.token_type,
      v.regex.clone(),
      v.channel, 
      v.skip,
    )
  }).collect::<Vec<_>>();

  // 这里一定要排序
  lexer_rules.sort_by(|a, b| a.3.cmp(&b.3));

  context.insert("lexer_rule_list", &lexer_rules);
  context.insert("grammar_name", &(grammar_name.to_lowercase(), grammar_name.to_uppercase(), pascal(grammar_name)));



  let result = TEMPLATES.render("target/rust/lexer.tera", &context).unwrap();
  result
}




