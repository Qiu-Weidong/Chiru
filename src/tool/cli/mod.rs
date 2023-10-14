use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use chiru::runtime::ast::rule_context::RuleContext;
use chiru::runtime::ast::rule_context::ToRule;
use chiru::runtime::lexer::Lexer;
use clap::CommandFactory;
use clap::Parser;
use clap::ValueEnum;

use std::{fs::File, io::Read};
use std::env;
use chiru::runtime::token_stream::TokenStream;
use super::analyzer::CommonLexer;
use super::analyzer::CommonParser;
use super::{code_generator::language::Language, gui::ast_drawer::ASTDrawer};
use super::{syntaxis::{chiru_lexer::ChiruLexer, chiru_parser::ChiruParser}, grammar::Grammar, code_generator::CodeGenerator};




#[derive(Parser)]
#[command(author, version, about, long_about = None, next_line_help = true)]
pub struct Cli {

  #[arg(short, long, value_name = "FILE", required = true)]
  pub input: PathBuf,

  #[arg(short, long, value_name = "OUTPUT")]
  pub output: Option<PathBuf>,



  #[arg(short, long, value_name = "LANGUAGE", value_enum, default_value_t = Language::Rust)]
  pub language: Language,

  #[arg(long, value_name = "ANALYZER", value_enum, default_value_t = Analyzer::LALR)]
  pub analyzer: Analyzer,

  #[arg(short, long, value_name = "PACKAGE")]
  pub package_name: Option<String>,

  #[arg(long, default_value_t = true)]
  pub visitor: bool,

  #[arg(long, default_value_t = true)]
  pub listener: bool,

  #[arg(long, default_value_t = true)]
  pub walker: bool,

  #[arg(long, default_value_t = false)]
  pub no_visitor: bool,

  #[arg(long, default_value_t = false)]
  pub no_listener: bool,

  #[arg(long, default_value_t = false)]
  pub no_walker: bool,






  #[arg(long, default_value_t = false)]
  pub gui: bool, 

  #[arg(long)]
  pub start_rule: Option<String>,

  #[arg(long)]
  pub test_file: Option<PathBuf>,


  #[arg(long, default_value_t = false)]
  pub tokens: bool,
  


  #[arg(long, default_value_t = false)]
  pub string_ast: bool,

  #[arg(long, default_value_t = false)]
  pub json_ast: bool,
  

}



#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Analyzer {
  LL1,

  LALR,
}


impl Cli {
  fn generate_code(&self) -> Result<(), Box<dyn Error>> {
    
    let mut input_file = File::open(&self.input)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;


    let lexer = ChiruLexer::new(&content);
    let mut tokens = TokenStream::new(&lexer, 0);
    let parser = ChiruParser::new();
    let ast = parser.compilation_unit(&mut tokens)?;


    let grammar = Grammar::from_ast(ast.as_ref())?;



    

    let base_dir: PathBuf;
    if let Some(p) = &self.output {
      base_dir = p.clone();
    } else {
      base_dir = env::current_dir()?;
    }
    

    let version = Cli::command().render_version();
    let code_generator = CodeGenerator::new(
      &grammar, ast.as_ref(), 
      &base_dir, &self.input, 
      self.language,
      self.package_name.clone(), 
      &version, self.analyzer,
    );
    code_generator.generate()?;
    Ok(())
  }






  // 打印 tokens
  fn dump_tokens(&self) -> Result<(), Box<dyn Error>> {
    let grammar = self.get_grammar()?;


    // content.clear();
    let mut content = String::new();
    // 然后从 test_file 中或 stdin 中读取测试文件
    if let Some(test_file) = &self.test_file {
      let mut file = File::open(test_file)?;
      file.read_to_string(&mut content)?;
    }
    else {
      std::io::stdin().read_to_string(&mut content)?;
    }


    let lexer = CommonLexer::from_grammar(&grammar, &content);

    if let Some(output) = &self.output {
      let mut file;
      if output.is_dir() {
        let output = output.join("tokens.txt");
        file = OpenOptions::new().write(true).create(true).open(output)?;
      } else {
        file = OpenOptions::new().write(true).create(true).open(output)?
      }
      for token in lexer.iter() {
        file.write_all(token.to_string().as_bytes())?
      }
      

    } else {
      for token in lexer.iter() {
        println!("{}", token)
      }
    }


    Ok(())
  }

  fn draw_gui(&self) -> Result<(), Box<dyn Error>> {
    let grammar = self.get_grammar()?;

    let ast = self.parse_ast(&grammar)?;

    // 输出 ast 到文件
    let mut file;
    if let Some(output) = &self.output {
      
      if output.is_dir() {
        let output = output.join("ast.html");
        file = OpenOptions::new().write(true).create(true).open(output)?;
      } else {
        file = OpenOptions::new().write(true).create(true).open(output)?
      }

    } else {
      // 创建一个默认文件
      let path = env::current_dir()?.join("ast.html");
      file = OpenOptions::new().write(true).create(true).open(path)?;
    }



    ASTDrawer::new().draw(&ast, &grammar.name, &mut file);
    Ok(())
  }

  fn parse_ast(&self, grammar: &Grammar) -> Result<RuleContext, Box<dyn Error>> {
    let mut content = String::new();

    
    // 然后从 test_file 中或 stdin 中读取测试文件
    if let Some(test_file) = &self.test_file {
      let mut file = File::open(test_file)?;
      file.read_to_string(&mut content)?;
    }
    else {
      std::io::stdin().read_to_string(&mut content)?;
    }

    // 获取开始符号
    let start_rule_id;
    if let Some(start_rule) = &self.start_rule {
      start_rule_id = grammar.vocabulary.get_nonterminal_id_by_name(start_rule).unwrap_or(0);
    } else { start_rule_id = 0; }

    let lexer = CommonLexer::from_grammar(&grammar, &content);
    let mut tokens = TokenStream::new(&lexer, 0);
    tokens.consume()?;
    let parser = CommonParser::from_grammar(&grammar);
    let ast = parser.parse(&mut tokens, start_rule_id);
    Ok(ast)
  }

  fn dump_ast(&self) -> Result<(), Box<dyn Error>> {
    let grammar = self.get_grammar()?;

    let ast = self.parse_ast(&grammar)?;

    if let Some(output) = &self.output {
      let mut file;
      if output.is_dir() {
        let output = output.join("string-ast.txt");
        file = OpenOptions::new().write(true).create(true).open(output)?;
      } else {
        file = OpenOptions::new().write(true).create(true).open(output)?;
      }
      file.write_all(ast.to_string().as_bytes())?;
    } else {
      println!("{}", ast.to_string());
    }
    Ok(())
  }

  fn dump_json_ast(&self) -> Result<(), Box<dyn Error>> {
    let grammar = self.get_grammar()?;

    let ast = self.parse_ast(&grammar)?;

    if let Some(output) = &self.output {
      let mut file;
      if output.is_dir() {
        let output = output.join("ast.json");
        file = OpenOptions::new().write(true).create(true).open(output)?;
      } else {
        file = OpenOptions::new().write(true).create(true).open(output)?;
      }
      file.write_all(serde_json::to_string(ast.as_rule())?.as_bytes())?;
    } else {
      println!("{}", serde_json::to_string(ast.as_rule())?);
    }
    Ok(())
  }


  fn get_grammar(&self) -> Result<Grammar, Box<dyn Error>> {
    let mut input_file = File::open(&self.input)?;
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;


    let ast;
    let grammar;

    {
      let lexer = ChiruLexer::new(&content);
      let mut tokens = TokenStream::new(&lexer, 0);
      let parser = ChiruParser::new();
      ast = parser.compilation_unit(&mut tokens)?;
      grammar = Grammar::from_ast(ast.as_ref())?;
    }
    Ok(grammar)
  }





  pub fn execute_command(&self) -> Result<(), Box<dyn Error>> {
    
    if ! self.gui && ! self.tokens && ! self.string_ast  && ! self.json_ast {
      return self.generate_code();
    }

    if self.gui {
      // 绘制 ast
      self.draw_gui()?;
    }

    if self.tokens {
      // 输出 tokens
      self.dump_tokens()?;
    } 
    
    if self.string_ast {
      // 输出 string-ast
      self.dump_ast()?;
    }

    if self.json_ast {
      // 输出 json 格式的语法树
      self.dump_json_ast()?;
    }
    

    Ok(())
  }
}






