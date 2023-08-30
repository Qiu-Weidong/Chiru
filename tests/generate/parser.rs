

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use chiru::tool::syntaxis::syntaxis_context::{
   EbnfSuffixContext, RuleListContext, BlockContext, AlternativeContext, EpsilonContext, LexerRuleContext, ParserRuleContext, RegularContext, ElementContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (15, 9) => 20,
    (9, 3) => 1,
    (5, 3) => 21,
    (3, 3) => 12,
    (0, 2) => 4,
    (16, 8) => 22,
    (12, 5) => 7,
    (3, 11) => 12,
    (2, 11) => 9,
    (16, 9) => 23,
    (15, 12) => 19,
    (13, 13) => 11,
    (17, 12) => 25,
    (4, 7) => 14,
    (14, 11) => 18,
    (15, 3) => 19,
    (10, 1) => 2,
    (6, 8) => 27,
    (17, 10) => 26,
    (14, 3) => 15,
    (13, 2) => 11,
    (15, 5) => 19,
    (16, 10) => 24,
    (17, 13) => 25,
    (11, 6) => 6,
    (5, 2) => 21,
    (3, 13) => 12,
    (14, 13) => 16,
    (6, 9) => 27,
    (0, 3) => 4,
    (2, 13) => 9,
    (12, 12) => 7,
    (15, 10) => 20,
    (15, 2) => 19,
    (15, 11) => 19,
    (15, 13) => 19,
    (17, 2) => 25,
    (12, 6) => 8,
    (5, 13) => 21,
    (13, 3) => 11,
    (6, 10) => 27,
    (2, 3) => 9,
    (17, 6) => 25,
    (17, 11) => 25,
    (17, 5) => 25,
    (13, 11) => 11,
    (7, 3) => 28,
    (10, 2) => 3,
    (0, 1) => 4,
    (3, 2) => 12,
    (17, 3) => 25,
    (15, 8) => 20,
    (10, 3) => 3,
    (8, 14) => 29,
    (1, 2) => 5,
    (15, 6) => 19,
    (2, 2) => 9,
    (5, 11) => 21,
    (14, 2) => 17,
    (2, 7) => 9,
    (9, 2) => 0,
    (13, 12) => 10,
    (13, 6) => 10,
    (3, 7) => 13,
    (13, 5) => 10,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(13),]),
    8 => Production::new(8, 12, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(12),]),
    0 => Production::new(0, 9, &vec![ProductionItem::NonTerminal(1),]),
    20 => Production::new(20, 15, &vec![ProductionItem::NonTerminal(6),]),
    26 => Production::new(26, 17, &vec![ProductionItem::Terminal(10),]),
    15 => Production::new(15, 14, &vec![ProductionItem::Terminal(3),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(12),]),
    7 => Production::new(7, 12, &vec![]),
    22 => Production::new(22, 16, &vec![ProductionItem::Terminal(8),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(16),ProductionItem::NonTerminal(17),]),
    3 => Production::new(3, 10, &vec![ProductionItem::NonTerminal(9),ProductionItem::NonTerminal(10),]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(10),]),
    16 => Production::new(16, 14, &vec![ProductionItem::Terminal(13),]),
    29 => Production::new(29, 8, &vec![ProductionItem::Terminal(14),]),
    23 => Production::new(23, 16, &vec![ProductionItem::Terminal(9),]),
    19 => Production::new(19, 15, &vec![]),
    10 => Production::new(10, 13, &vec![]),
    6 => Production::new(6, 11, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(3),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    11 => Production::new(11, 13, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(13),]),
    18 => Production::new(18, 14, &vec![ProductionItem::Terminal(11),ProductionItem::NonTerminal(2),ProductionItem::Terminal(12),]),
    17 => Production::new(17, 14, &vec![ProductionItem::Terminal(2),]),
    24 => Production::new(24, 16, &vec![ProductionItem::Terminal(10),]),
    25 => Production::new(25, 17, &vec![]),
    2 => Production::new(2, 10, &vec![]),
    1 => Production::new(1, 9, &vec![ProductionItem::NonTerminal(7),]),
    28 => Production::new(28, 7, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(7),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    6 => "ebnf_suffix",
    0 => "rule_list",
    2 => "block",
    3 => "alternative",
    4 => "epsilon",
    7 => "lexer_rule",
    1 => "parser_rule",
    8 => "regular",
    5 => "element",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    3 => "TOKEN_REF",
    0 => "_START",
    11 => "LPAREN",
    14 => "REGULAR_LITERAL",
    13 => "STRING_LITERAL",
    8 => "STAR",
    2 => "RULE_REF",
    9 => "PLUS",
    10 => "QUESTION",
    1 => "_STOP",
    5 => "SEMI",
    4 => "COLON",
    6 => "OR",
    15 => "WHITE_SPACE",
    12 => "RPAREN",
    7 => "EPSILON",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (6, 6),
    (15, 2),
    (16, 13),
    (6, 12),
    (14, 8),
    (1, 3),
    (14, 3),
    (11, 6),
    (6, 13),
    (5, 3),
    (17, 3),
    (9, 2),
    (9, 3),
    (16, 6),
    (14, 13),
    (3, 5),
    (17, 12),
    (14, 6),
    (4, 5),
    (16, 2),
    (5, 11),
    (16, 12),
    (5, 12),
    (9, 1),
    (1, 2),
    (11, 12),
    (5, 6),
    (7, 3),
    (14, 11),
    (14, 12),
    (15, 11),
    (11, 5),
    (13, 12),
    (3, 6),
    (17, 13),
    (10, 1),
    (6, 5),
    (4, 12),
    (13, 6),
    (14, 2),
    (15, 13),
    (16, 10),
    (15, 5),
    (8, 5),
    (5, 13),
    (12, 12),
    (16, 11),
    (15, 12),
    (1, 1),
    (13, 5),
    (17, 5),
    (6, 11),
    (14, 9),
    (5, 2),
    (14, 10),
    (6, 3),
    (2, 5),
    (6, 2),
    (2, 12),
    (4, 6),
    (14, 5),
    (17, 11),
    (7, 2),
    (7, 1),
    (0, 1),
    (17, 6),
    (15, 3),
    (16, 5),
    (5, 5),
    (3, 12),
    (17, 2),
    (15, 6),
    (16, 3),
    (12, 5),
  };
}

#[allow(dead_code)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const EBNF_SUFFIX: usize = 6; 
  pub const RULE_LIST: usize = 0; 
  pub const BLOCK: usize = 2; 
  pub const ALTERNATIVE: usize = 3; 
  pub const EPSILON: usize = 4; 
  pub const LEXER_RULE: usize = 7; 
  pub const PARSER_RULE: usize = 1; 
  pub const REGULAR: usize = 8; 
  pub const ELEMENT: usize = 5; 



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
  
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULE_LIST);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

