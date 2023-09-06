pub mod tool;


use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, io::Read};
use std::env;
use chiru::runtime::error_strategy::error_listener::ConsoleErrorListener;
use chiru::runtime::lexer::TokenIter;
use chiru::runtime::token::Token;
use chiru::runtime::token_stream::TokenStream;
use clap::Parser;
use std::error::Error;
use tool::{syntaxis::{chiru_lexer::ChiruLexer, chiru_parser::ChiruParser}, grammar::Grammar, code_generator::CodeGenerator};
use tool::cli::Cli;



fn dump_tokens(grammar: &Grammar, inputfile: Option<PathBuf>, outputfile: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
  // 首先获取输入
  let mut input = String::new();
  if let Some(inputfile) = inputfile {
    let mut file = File::open(inputfile)?;
    file.read_to_string(&mut input)?;
  } else {
    std::io::stdin().read_to_string(&mut input)?;
  }

  let error_listeners = vec![];

  // 构建 rules




  let rules = vec![];
  // 首先构建一个迭代器
  let iter = TokenIter::new(&input, &rules, &error_listeners);

  // let tokens = TokenStream {
  //   iter,
  //   channel: 0,
  //   consumed_tokens: VecDeque::new(),
  //   cached_tokens: VecDeque::new(),
  //   // 我们认为 start 和 stop 永远都和当前 stream 一样。
  //   next_token: Some(Token::start(0)),
  // };


  if let Some(output) = outputfile {
    let mut file;
    if output.is_file() {
      // 是文件的话就直接写文件
      file = OpenOptions::new().write(true).create(true).open(output)?;
    }
    else {
      // 创建一个 txt 文件
      file = OpenOptions::new().write(true).create(true).open(output.join("tokens.txt"))?;
    }
    
    for token in iter {
      file.write_all(token.to_string().as_bytes())?;
    }

  } else {

    // 输出到标准输出
    iter.for_each(|token| {
      println!("{}", token);
    });
  }

  todo!()
}

fn dump_ast(grammar: &Grammar, outputfile: Option<PathBuf>) -> String {
  todo!()
}

fn draw_ast(grammar: &Grammar, outputfile: Option<PathBuf>) -> String {
  todo!()
}





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
    let code_generator = CodeGenerator::new(grammar, ast.as_ref());
    code_generator.generate(&output_dir);
  } else {


    // 测试语法
  }



  
  

  Ok(())
}






// static VALUE: Lazy<>




