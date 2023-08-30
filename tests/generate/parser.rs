

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use chiru::tool::syntaxis::syntaxis_context::{
   RegularContext, ElementContext, EpsilonContext, ParserRuleContext, AlternativeContext, BlockContext, RuleListContext, LexerRuleContext, EbnfSuffixContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (12, 3) => 11,
    (12, 12) => 10,
    (12, 6) => 10,
    (3, 2) => 12,
    (13, 2) => 17,
    (12, 11) => 11,
    (2, 2) => 9,
    (3, 3) => 12,
    (14, 8) => 20,
    (15, 8) => 22,
    (12, 2) => 11,
    (6, 10) => 27,
    (11, 6) => 8,
    (16, 3) => 25,
    (12, 5) => 10,
    (12, 13) => 11,
    (14, 9) => 20,
    (7, 3) => 28,
    (5, 2) => 21,
    (16, 2) => 25,
    (14, 2) => 19,
    (11, 12) => 7,
    (14, 12) => 19,
    (5, 13) => 21,
    (14, 10) => 20,
    (16, 13) => 25,
    (14, 5) => 19,
    (9, 2) => 3,
    (14, 13) => 19,
    (16, 6) => 25,
    (16, 11) => 25,
    (14, 11) => 19,
    (6, 9) => 27,
    (9, 1) => 2,
    (16, 10) => 26,
    (16, 12) => 25,
    (1, 2) => 5,
    (4, 7) => 14,
    (9, 14) => 3,
    (2, 3) => 9,
    (2, 13) => 9,
    (5, 11) => 21,
    (13, 13) => 16,
    (0, 14) => 4,
    (3, 7) => 13,
    (0, 3) => 4,
    (14, 6) => 19,
    (0, 1) => 4,
    (10, 6) => 6,
    (0, 2) => 4,
    (11, 5) => 7,
    (2, 7) => 9,
    (8, 3) => 1,
    (6, 8) => 27,
    (8, 2) => 0,
    (15, 9) => 23,
    (13, 11) => 18,
    (8, 14) => 29,
    (14, 3) => 19,
    (13, 3) => 15,
    (2, 11) => 9,
    (3, 11) => 12,
    (5, 3) => 21,
    (15, 10) => 24,
    (9, 3) => 3,
    (16, 5) => 25,
    (3, 13) => 12,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    23 => Production::new(23, 15, &vec![ProductionItem::Terminal(9),]),
    6 => Production::new(6, 10, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(3),]),
    0 => Production::new(0, 8, &vec![ProductionItem::NonTerminal(1),]),
    15 => Production::new(15, 13, &vec![ProductionItem::Terminal(3),]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(9),]),
    20 => Production::new(20, 14, &vec![ProductionItem::NonTerminal(6),]),
    28 => Production::new(28, 7, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    29 => Production::new(29, 8, &vec![ProductionItem::Terminal(14),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(14),]),
    18 => Production::new(18, 13, &vec![ProductionItem::Terminal(11),ProductionItem::NonTerminal(2),ProductionItem::Terminal(12),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(7),]),
    17 => Production::new(17, 13, &vec![ProductionItem::Terminal(2),]),
    3 => Production::new(3, 9, &vec![ProductionItem::NonTerminal(8),ProductionItem::NonTerminal(9),]),
    8 => Production::new(8, 11, &vec![ProductionItem::NonTerminal(10),ProductionItem::NonTerminal(11),]),
    11 => Production::new(11, 12, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(12),]),
    25 => Production::new(25, 16, &vec![]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    7 => Production::new(7, 11, &vec![]),
    16 => Production::new(16, 13, &vec![ProductionItem::Terminal(13),]),
    19 => Production::new(19, 14, &vec![]),
    24 => Production::new(24, 15, &vec![ProductionItem::Terminal(10),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(11),]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(12),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    26 => Production::new(26, 16, &vec![ProductionItem::Terminal(10),]),
    10 => Production::new(10, 12, &vec![]),
    2 => Production::new(2, 9, &vec![]),
    1 => Production::new(1, 8, &vec![ProductionItem::NonTerminal(7),]),
    22 => Production::new(22, 15, &vec![ProductionItem::Terminal(8),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    8 => "regular",
    5 => "element",
    4 => "epsilon",
    1 => "parser_rule",
    3 => "alternative",
    2 => "block",
    0 => "rule_list",
    7 => "lexer_rule",
    6 => "ebnf_suffix",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    8 => "STAR",
    7 => "EPSILON",
    13 => "STRING_LITERAL",
    15 => "WHITE_SPACE",
    9 => "PLUS",
    3 => "TOKEN_REF",
    2 => "RULE_REF",
    10 => "QUESTION",
    0 => "_START",
    12 => "RPAREN",
    11 => "LPAREN",
    4 => "COLON",
    14 => "REGULAR_LITERAL",
    1 => "_STOP",
    5 => "SEMI",
    6 => "OR",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (5, 3),
    (14, 2),
    (4, 5),
    (11, 5),
    (13, 8),
    (13, 9),
    (14, 3),
    (16, 3),
    (16, 12),
    (15, 2),
    (3, 6),
    (13, 3),
    (12, 5),
    (6, 3),
    (15, 10),
    (12, 12),
    (2, 5),
    (14, 12),
    (16, 2),
    (8, 5),
    (5, 11),
    (10, 5),
    (5, 5),
    (1, 5),
    (6, 6),
    (13, 6),
    (5, 6),
    (6, 12),
    (10, 12),
    (10, 6),
    (4, 12),
    (7, 1),
    (6, 11),
    (4, 6),
    (7, 14),
    (13, 5),
    (5, 13),
    (12, 6),
    (13, 2),
    (9, 1),
    (15, 5),
    (15, 3),
    (14, 5),
    (15, 12),
    (6, 13),
    (1, 1),
    (13, 10),
    (11, 12),
    (3, 5),
    (7, 3),
    (8, 3),
    (6, 2),
    (15, 13),
    (14, 11),
    (3, 12),
    (1, 2),
    (5, 12),
    (13, 12),
    (14, 13),
    (6, 5),
    (14, 6),
    (7, 5),
    (13, 11),
    (15, 11),
    (8, 1),
    (8, 14),
    (5, 2),
    (15, 6),
    (16, 11),
    (16, 13),
    (16, 6),
    (0, 1),
    (16, 5),
    (8, 2),
    (2, 12),
    (13, 13),
    (1, 14),
    (1, 3),
    (7, 2),
  };
}


impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const REGULAR: usize = 8; 
  pub const ELEMENT: usize = 5; 
  pub const EPSILON: usize = 4; 
  pub const PARSER_RULE: usize = 1; 
  pub const ALTERNATIVE: usize = 3; 
  pub const BLOCK: usize = 2; 
  pub const RULE_LIST: usize = 0; 
  pub const LEXER_RULE: usize = 7; 
  pub const EBNF_SUFFIX: usize = 6; 



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
  
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULE_LIST);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

