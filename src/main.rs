pub mod tool;


use std::{fs::File, io::Read};
use std::env;
use chiru::runtime::token_stream::TokenStream;
use tool::{syntaxis::{chiru_lexer::ChiruLexer, chiru_parser::ChiruParser}, grammar::Grammar, code_generator::CodeGenerator};


#[allow(unused_doc_comments)]
fn main() {
  if let Ok(current_dir) = env::current_dir() {
    println!("Current directory: {:?}", current_dir);
  } else {
    println!("Failed to retrieve current directory");
  }

  let mut file = File::open("src/tool/syntaxis/chiru.chiru").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  

  let lexer = ChiruLexer::new(&contents);
  let mut stream = TokenStream::new(&lexer, 0);

  stream.consume().unwrap(); // 注意要先将 _START 消耗掉
  let parser = ChiruParser::new();
  let ast = parser.compilation_unit(&mut stream);

  let grammar = Grammar::from_ast(ast.as_ref());

  // let base_dir = "src/tool/syntaxis";
  let base_dir = "tests/generate";



  let code_generator = CodeGenerator::new(grammar, ast.as_ref());
  code_generator.generate(base_dir);

}


