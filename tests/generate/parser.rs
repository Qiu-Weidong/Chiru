

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use chiru::tool::syntaxis::syntaxis_context::{
   AlternativeContext, RegularContext, LexerRuleContext, ParserRuleContext, ElementContext, EbnfSuffixContext, BlockContext, EpsilonContext, RuleListContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (6, 8) => 27,
    (6, 10) => 27,
    (8, 14) => 29,
    (15, 8) => 22,
    (16, 6) => 25,
    (16, 13) => 25,
    (3, 7) => 13,
    (2, 3) => 9,
    (5, 3) => 21,
    (10, 6) => 6,
    (12, 13) => 11,
    (1, 2) => 5,
    (11, 5) => 7,
    (9, 2) => 3,
    (14, 2) => 19,
    (16, 2) => 25,
    (9, 14) => 3,
    (9, 1) => 2,
    (12, 5) => 10,
    (11, 6) => 8,
    (2, 11) => 9,
    (15, 10) => 24,
    (15, 9) => 23,
    (12, 6) => 10,
    (5, 11) => 21,
    (13, 2) => 17,
    (5, 13) => 21,
    (6, 9) => 27,
    (11, 12) => 7,
    (3, 11) => 12,
    (14, 10) => 20,
    (14, 6) => 19,
    (3, 2) => 12,
    (12, 12) => 10,
    (4, 7) => 14,
    (13, 11) => 18,
    (0, 3) => 4,
    (14, 8) => 20,
    (14, 12) => 19,
    (16, 11) => 25,
    (2, 13) => 9,
    (0, 14) => 4,
    (3, 3) => 12,
    (0, 1) => 4,
    (8, 3) => 1,
    (3, 13) => 12,
    (2, 7) => 9,
    (2, 2) => 9,
    (16, 3) => 25,
    (8, 2) => 0,
    (12, 3) => 11,
    (9, 3) => 3,
    (13, 3) => 15,
    (13, 13) => 16,
    (14, 13) => 19,
    (14, 5) => 19,
    (16, 10) => 26,
    (14, 9) => 20,
    (12, 11) => 11,
    (5, 2) => 21,
    (16, 5) => 25,
    (12, 2) => 11,
    (7, 3) => 28,
    (14, 3) => 19,
    (14, 11) => 19,
    (0, 2) => 4,
    (16, 12) => 25,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(12),]),
    0 => Production::new(0, 8, &vec![ProductionItem::NonTerminal(1),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    23 => Production::new(23, 15, &vec![ProductionItem::Terminal(9),]),
    2 => Production::new(2, 9, &vec![]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(9),]),
    29 => Production::new(29, 8, &vec![ProductionItem::Terminal(14),]),
    22 => Production::new(22, 15, &vec![ProductionItem::Terminal(8),]),
    25 => Production::new(25, 16, &vec![]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    1 => Production::new(1, 8, &vec![ProductionItem::NonTerminal(7),]),
    11 => Production::new(11, 12, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(12),]),
    15 => Production::new(15, 13, &vec![ProductionItem::Terminal(3),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(7),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(11),]),
    17 => Production::new(17, 13, &vec![ProductionItem::Terminal(2),]),
    6 => Production::new(6, 10, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(3),]),
    26 => Production::new(26, 16, &vec![ProductionItem::Terminal(10),]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(14),]),
    7 => Production::new(7, 11, &vec![]),
    18 => Production::new(18, 13, &vec![ProductionItem::Terminal(11),ProductionItem::NonTerminal(2),ProductionItem::Terminal(12),]),
    28 => Production::new(28, 7, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    16 => Production::new(16, 13, &vec![ProductionItem::Terminal(13),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    3 => Production::new(3, 9, &vec![ProductionItem::NonTerminal(8),ProductionItem::NonTerminal(9),]),
    24 => Production::new(24, 15, &vec![ProductionItem::Terminal(10),]),
    8 => Production::new(8, 11, &vec![ProductionItem::NonTerminal(10),ProductionItem::NonTerminal(11),]),
    19 => Production::new(19, 14, &vec![]),
    20 => Production::new(20, 14, &vec![ProductionItem::NonTerminal(6),]),
    10 => Production::new(10, 12, &vec![]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    3 => "alternative",
    8 => "regular",
    7 => "lexer_rule",
    1 => "parser_rule",
    5 => "element",
    6 => "ebnf_suffix",
    2 => "block",
    4 => "epsilon",
    0 => "rule_list",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    15 => "WHITE_SPACE",
    5 => "SEMI",
    7 => "EPSILON",
    1 => "_STOP",
    12 => "RPAREN",
    3 => "TOKEN_REF",
    8 => "STAR",
    6 => "OR",
    10 => "QUESTION",
    9 => "PLUS",
    14 => "REGULAR_LITERAL",
    13 => "STRING_LITERAL",
    0 => "_START",
    4 => "COLON",
    11 => "LPAREN",
    2 => "RULE_REF",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (1, 2),
    (5, 6),
    (5, 2),
    (16, 12),
    (15, 13),
    (5, 3),
    (16, 13),
    (14, 11),
    (7, 14),
    (6, 5),
    (6, 3),
    (13, 9),
    (11, 5),
    (12, 12),
    (15, 11),
    (14, 12),
    (6, 6),
    (15, 3),
    (14, 13),
    (10, 5),
    (15, 5),
    (14, 5),
    (10, 6),
    (1, 3),
    (8, 14),
    (12, 5),
    (13, 3),
    (8, 1),
    (8, 5),
    (12, 6),
    (0, 1),
    (9, 1),
    (15, 12),
    (15, 2),
    (16, 11),
    (16, 2),
    (13, 11),
    (13, 2),
    (13, 12),
    (1, 1),
    (6, 11),
    (6, 2),
    (15, 10),
    (5, 5),
    (16, 6),
    (7, 5),
    (16, 3),
    (14, 2),
    (13, 10),
    (14, 3),
    (2, 5),
    (14, 6),
    (7, 3),
    (1, 5),
    (13, 13),
    (1, 14),
    (11, 12),
    (13, 5),
    (4, 5),
    (2, 12),
    (6, 12),
    (3, 5),
    (3, 6),
    (7, 1),
    (5, 13),
    (4, 12),
    (16, 5),
    (8, 3),
    (10, 12),
    (13, 6),
    (8, 2),
    (3, 12),
    (7, 2),
    (5, 12),
    (15, 6),
    (6, 13),
    (4, 6),
    (5, 11),
    (13, 8),
  };
}


impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ALTERNATIVE: usize = 3; 
  pub const REGULAR: usize = 8; 
  pub const LEXER_RULE: usize = 7; 
  pub const PARSER_RULE: usize = 1; 
  pub const ELEMENT: usize = 5; 
  pub const EBNF_SUFFIX: usize = 6; 
  pub const BLOCK: usize = 2; 
  pub const EPSILON: usize = 4; 
  pub const RULE_LIST: usize = 0; 



  pub fn new() -> Self {
    Self {
      analyzer: LL1 { 
        error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
        table: &LL1_TABLE, 
        productions: &PRODUCTIONS, 
        rule_names: &TERMINALS, 
        sync: &SYNC, 
      }
    }
  }


  // 使用模板生成
  
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
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
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULE_LIST);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

