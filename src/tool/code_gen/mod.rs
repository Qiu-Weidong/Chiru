use tera::Tera;

use lazy_static::lazy_static;  // 这里导入了宏
pub mod lexer_gen;
pub mod visitor_gen;
pub mod parser_gen;
pub mod context_gen;
pub mod listener_gen;
pub mod mod_gen;
pub mod walker_gen;




lazy_static! {
  pub static ref TEMPLATES: Tera = {
    let mut tera = Tera::new("src/tool/templates/**/*").unwrap();
    tera.autoescape_on(vec![]);
    tera
  };
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



