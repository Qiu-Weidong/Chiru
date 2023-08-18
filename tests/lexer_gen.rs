

use std::io::Write;


use tera::{Tera, Context};

mod ast_loader;

#[test]
fn gen_lexer() {
  let (_, data) = ast_loader::load_ast();


  let mut tera = Tera::new("src/tool/templates/target/rust/*.tera").unwrap();
  tera.autoescape_on(vec![]);

  let mut context = Context::new();
  context.insert("tokens", &data);
  context.insert("name", "chiru");

  let result = tera.render("lexer.tera", &context).unwrap();
    

  let mut file = std::fs::File::create("tests/lexer_generate/lexer.rs").unwrap();
  file.write(result.as_bytes()).unwrap();
}




