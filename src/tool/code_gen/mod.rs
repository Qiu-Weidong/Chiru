use tera::Tera;


pub mod lexer_gen;
pub mod visitor_gen;
pub mod parser_gen;
pub mod context_gen;
pub mod listener_gen;


use serde::Serialize;

lazy_static! {
  pub static ref TEMPLATES: Tera = {
    let mut tera = Tera::new("src/tool/templates/**/*").unwrap();
    tera.autoescape_on(vec![]);
    tera
  };
}


#[derive(Serialize)]
pub struct RuleRenderContext {
  pub pascal: String,
  pub underscore: String,
  pub upcase: String,
  pub rule_index: usize,

  pub items: Vec<RuleRenderItem>,
}

#[derive(Serialize)]
pub struct RuleRenderItem {
  pub pascal: String,
  pub underscore: String,
  pub upcase: String,
}


pub fn pascal(input: &str) -> String {
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



