pub mod tool;

use std::{fs::File, io::Read};
use std::env;
use chiru::runtime::token_stream::TokenStream;
use clap::Parser;
use std::error::Error;
use tool::{syntaxis::{chiru_lexer::ChiruLexer, chiru_parser::ChiruParser}, grammar::Grammar, code_generator::CodeGenerator};

use tool::cli::Cli;


#[allow(unused_doc_comments)]
fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::parse();




  println!("{:?}", cli.language);

  /// 首先获取输出目录。
  let output_dir;
  if let Some(output) = cli.output_dir {
    output_dir = output;
  } else {
    output_dir = env::current_dir()?;
  }



  let mut file = File::open(&cli.input)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  

  let lexer = ChiruLexer::new(&contents);
  let mut stream = TokenStream::new(&lexer, 0);

  stream.consume().unwrap(); // 注意要先将 _START 消耗掉
  let parser = ChiruParser::new();
  let ast = parser.compilation_unit(&mut stream);

  let grammar = Grammar::from_ast(ast.as_ref());




  let code_generator = CodeGenerator::new(grammar, ast.as_ref());
  code_generator.generate(&output_dir);

  Ok(())
}



