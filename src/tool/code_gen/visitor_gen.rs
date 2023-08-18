use std::io::Write;

use tera::Context;

// use crate::runtime::vocabulary::Vocabulary;

use crate::tool::grammar::vocabulary::Vocabulary;

use super::{RuleRenderContext, TEMPLATES};


fn pascal(input: &str) -> String {
  let mut result = String::new();

  input.split("_").for_each(|word| {
    let mut chars: Vec<char> = word.chars().collect();
    if chars.len() >= 1 {
      chars[0] = chars[0].to_ascii_uppercase();
      let s: String = chars.into_iter().collect();
      result += &s;
    }
    
  });
  result
}



// 生成 visitor 
pub fn generate_visitor(vocabulary: &Vocabulary) {
  let mut rules: Vec<RuleRenderContext> = Vec::new();

  vocabulary.nonterminals.values().for_each(|nonterminal| {
    if let Some(name) = &nonterminal.name {
      rules.push(RuleRenderContext { items: Vec::new(),
        pascal: pascal(name), underscore: name.to_owned(), upcase: name.to_ascii_uppercase(), rule_index: nonterminal.id });
    }
  }); 


  let mut context = Context::new();
  context.insert("rules", &rules);
  context.insert("name", "Syntaxis");

  let result = TEMPLATES.render("target/rust/visitor.tera", &context).unwrap();
    

  let mut file = std::fs::File::create("visitor.rs").unwrap();
  file.write(result.as_bytes()).unwrap();

  let result = TEMPLATES.render("target/rust/context.tera", &context).unwrap();
    

  let mut file = std::fs::File::create("context.rs").unwrap();
  file.write(result.as_bytes()).unwrap();


}



