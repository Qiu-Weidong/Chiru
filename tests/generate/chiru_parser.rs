

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::chiru_context::{
   AlternativeContext, ElementContext, ParserRuleContext, AttributeListContext, LexerRuleContext, RegularContext, AttributeContext, RuleListContext, AnnotationContext, EbnfSuffixContext, EpsilonContext, BlockContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    3 => "alternative",
    5 => "element",
    1 => "parser_rule",
    10 => "attribute_list",
    7 => "lexer_rule",
    8 => "regular",
    11 => "attribute",
    0 => "rule_list",
    9 => "annotation",
    6 => "ebnf_suffix",
    4 => "epsilon",
    2 => "block",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    0 => "_START",
    1 => "_STOP",
    12 => "LPAREN",
    14 => "AT",
    2 => "RULE_REF",
    20 => "WHITE_SPACE",
    15 => "SHARP",
    10 => "PLUS",
    19 => "REGULAR_LITERAL",
    4 => "COLON",
    17 => "RBRACKET",
    13 => "RPAREN",
    18 => "STRING_LITERAL",
    3 => "TOKEN_REF",
    11 => "QUESTION",
    7 => "OR",
    16 => "LBRACKET",
    9 => "STAR",
    8 => "EPSILON",
    5 => "SEMI",
    6 => "COMMA",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (0, 1),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ALTERNATIVE: usize = 3; 
  pub const ELEMENT: usize = 5; 
  pub const PARSER_RULE: usize = 1; 
  pub const ATTRIBUTE_LIST: usize = 10; 
  pub const LEXER_RULE: usize = 7; 
  pub const REGULAR: usize = 8; 
  pub const ATTRIBUTE: usize = 11; 
  pub const RULE_LIST: usize = 0; 
  pub const ANNOTATION: usize = 9; 
  pub const EBNF_SUFFIX: usize = 6; 
  pub const EPSILON: usize = 4; 
  pub const BLOCK: usize = 2; 



  pub fn new() -> Self {
    Self {
      analyzer: LL1 { 
        error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
        table: &LL1_TABLE, 
        productions: &PRODUCTIONS, 
        rule_names: &NONTERMINALS, 
        sync: &SYNC, 
      }
    }
  }


  // 使用模板生成
  
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn attribute_list(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeListContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE_LIST);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE);
    Box::new(result)
  } 
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULE_LIST);
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

