use crate::tool::grammar::Grammar;



pub fn mod_generate(grammar: &Grammar,
  lexer: bool,
  parser: bool, 
  context: bool,
  listener: bool,
  visitor: bool,
  walker: bool,
) -> String {
  let mut result = String::new();
  
  if lexer {
    result += &format!("pub mod {}_lexer;\n", grammar.name.to_lowercase());
  }
  if parser {
    result += &format!("pub mod {}_parser;\n", grammar.name.to_lowercase());
  }

  if context {
    result += &format!("pub mod {}_context;\n", grammar.name.to_lowercase());
  }
  if listener {
    result += &format!("pub mod {}_listener;\n", grammar.name.to_lowercase());
  }
  if visitor {
    result += &format!("pub mod {}_visitor;\n", grammar.name.to_lowercase());
  }
  if walker {
    result += &format!("pub mod {}_walker;\n", grammar.name.to_lowercase());
  }
  
  result
}





