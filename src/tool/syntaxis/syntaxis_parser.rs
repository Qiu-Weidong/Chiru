

use std::collections::HashMap;
use maplit::hashmap;

use crate::{runtime::{ast::rule_context::RuleContext, parser::Parser, token_stream::TokenStream}, 
  tool::{grammar::{production::ProductionItem, production::Production}, 
  syntaxis::syntaxis_context::{RuleListContext, ParserRuleContext, BlockContext, 
    AlternativeContext, EpsilonContext, ElementContext, EbnfSuffixContext, LexerRuleContext, RegularContext}}};


pub struct SyntaxisParser;


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    (0, 0) => 1,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(0)]),
  };

  // 终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    0 => "",
  };

  // 非终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    0 => "",
  };
}


impl SyntaxisParser {

  // pub static ref GLOBAL_VALUE: i32 = 42;

  // 使用模板生成 每个非终结符的编号
  pub const RULE_LIST: usize = 0;
  pub const PARSER_RULE: usize = 1;
  pub const BLOCK: usize = 2;
  pub const ALTERNATIVE: usize = 3;
  pub const EPSILON: usize = 4;
  pub const ELEMENT: usize = 5;
  pub const EBNF_SUFFIX: usize = 6;
  pub const LEXER_RULE: usize = 7;
  pub const REGULAR: usize = 8;

  // 生成一个预测分析表



  pub fn new() -> Self {
    Self {
    }
  }


  // 使用模板生成
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::RULE_LIST);
    Box::new(result)
  }

  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::PARSER_RULE);
    Box::new(result)
  }

  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::BLOCK);
    Box::new(result)
  }

  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::ALTERNATIVE);
    Box::new(result)
  }

  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::EPSILON);
    Box::new(result)
  }

  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::ELEMENT);
    Box::new(result)
  }

  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::EBNF_SUFFIX);
    Box::new(result)
  }

  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::LEXER_RULE);
    Box::new(result)
  }

  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::REGULAR);
    Box::new(result)
  }


}






impl Parser for SyntaxisParser {
  fn parse(&self, token_stream: &mut TokenStream) -> RuleContext {
    self.parse_ast(token_stream, &LL1_TABLE, &PRODUCTIONS, &NONTERMINALS, Self::RULE_LIST)
  }
}

