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

use std::fs::File;

use syntaxis::tool::{grammar::Grammar, visitor::{lexer_rule_visitor::LexerRuleData, grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor}}, serde_ast, syntaxis::syntaxis_context::RuleListContext};



fn main() {

  let (grammar, _) = load_ast();

  // 先查看一下 token 和 rule 的编号
  grammar.terminals.iter().for_each(|(id, name)| {
    println!("{}: {}", name, id);
  });








}


pub fn load_ast() -> (Grammar, Vec<LexerRuleData>) {
  let file = File::open("src/tool/syntaxis/syntaxis2.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();

  let mut grammar = Grammar::new("我的文法");
  let token_cnt;
  let data;
  {
    let mut visitor = StringLiteralToTokenVisitor::new(
      &mut grammar, 2
    );

    ast.accept(&mut visitor);
    token_cnt = visitor.next_token_id;
    data = visitor.data;
  }
  
  let rule_cnt; 
  let lexer_data;
  {
    let mut visitor = SymbolVisitor::new(&mut grammar, token_cnt, 0, data);
    ast.accept(&mut visitor);
    rule_cnt = visitor.next_rule_id;
    lexer_data = visitor.data;
  }

  {
    let mut visitor = ProductionVisitor::new(&mut grammar, rule_cnt);
    ast.accept(&mut visitor);
  }

  (grammar, lexer_data)

}

