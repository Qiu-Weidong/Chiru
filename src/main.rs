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

  let mut file = File::open(&cli.input)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  
  let lexer = ChiruLexer::new(&contents);
  let mut stream = TokenStream::new(&lexer, 0);
  stream.consume().unwrap(); // 注意要先将 _START 消耗掉
  let parser = ChiruParser::new();
  let ast = parser.compilation_unit(&mut stream);

  // 解析出语法
  let grammar = Grammar::from_ast(ast.as_ref());


  if ! cli.tokens && ! cli.gui && ! cli.string_ast {
    let output_dir;
    if let Some(out) = &cli.output {
      output_dir = out.clone();
    } else {
      output_dir = env::current_dir()?;
    }


    // 代码生成
    let code_generator = CodeGenerator::new(grammar, ast.as_ref(), cli.language);
    code_generator.generate(&output_dir);
  } else {


    // 测试语法
  }



  
  

  Ok(())
}





