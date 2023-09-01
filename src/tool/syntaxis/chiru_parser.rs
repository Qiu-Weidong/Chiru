

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use crate::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::chiru_context::{
   ParserRuleContext, EpsilonContext, RuleListContext, AlternativeContext, AttributeListContext, AnnotationContext, RegularContext, EbnfSuffixContext, ElementContext, BlockContext, LexerRuleContext, AttributeContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (6, 10) => 27,
    (20, 5) => 25,
    (11, 3) => 43,
    (1, 2) => 5,
    (6, 11) => 27,
    (20, 13) => 25,
    (14, 7) => 6,
    (19, 9) => 22,
    (15, 5) => 7,
    (13, 15) => 3,
    (18, 5) => 19,
    (3, 3) => 12,
    (23, 6) => 36,
    (12, 14) => 1,
    (10, 2) => 37,
    (17, 3) => 15,
    (19, 11) => 24,
    (24, 2) => 39,
    (0, 14) => 4,
    (12, 15) => 1,
    (18, 7) => 19,
    (5, 12) => 21,
    (12, 3) => 1,
    (15, 7) => 8,
    (2, 3) => 9,
    (18, 11) => 20,
    (25, 12) => 40,
    (26, 12) => 42,
    (3, 12) => 12,
    (13, 3) => 3,
    (17, 2) => 17,
    (16, 2) => 11,
    (13, 1) => 2,
    (8, 19) => 31,
    (5, 2) => 21,
    (21, 14) => 29,
    (19, 10) => 23,
    (10, 3) => 37,
    (18, 18) => 19,
    (24, 3) => 38,
    (18, 13) => 19,
    (3, 2) => 12,
    (11, 2) => 43,
    (13, 2) => 3,
    (3, 8) => 13,
    (17, 18) => 16,
    (22, 6) => 34,
    (12, 2) => 0,
    (13, 14) => 3,
    (20, 3) => 25,
    (20, 11) => 26,
    (21, 3) => 28,
    (7, 3) => 30,
    (5, 3) => 21,
    (7, 15) => 30,
    (18, 3) => 19,
    (9, 14) => 32,
    (26, 17) => 41,
    (4, 8) => 14,
    (21, 15) => 29,
    (18, 2) => 19,
    (26, 6) => 41,
    (2, 12) => 9,
    (26, 3) => 41,
    (0, 2) => 4,
    (0, 15) => 4,
    (16, 7) => 10,
    (3, 18) => 12,
    (0, 3) => 4,
    (2, 18) => 9,
    (9, 15) => 33,
    (20, 12) => 25,
    (7, 14) => 30,
    (16, 13) => 10,
    (2, 2) => 9,
    (2, 8) => 9,
    (20, 7) => 25,
    (15, 13) => 7,
    (16, 3) => 11,
    (18, 10) => 20,
    (20, 18) => 25,
    (17, 12) => 18,
    (18, 12) => 19,
    (16, 5) => 10,
    (23, 17) => 35,
    (20, 2) => 25,
    (5, 18) => 21,
    (16, 18) => 11,
    (18, 9) => 20,
    (0, 1) => 4,
    (16, 12) => 11,
    (6, 9) => 27,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    22 => Production::new(22, 19, &vec![ProductionItem::Terminal(9),]),
    38 => Production::new(38, 24, &vec![ProductionItem::Terminal(3),]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    1 => Production::new(1, 12, &vec![ProductionItem::NonTerminal(7),]),
    6 => Production::new(6, 14, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(3),]),
    16 => Production::new(16, 17, &vec![ProductionItem::Terminal(18),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    10 => Production::new(10, 16, &vec![]),
    25 => Production::new(25, 20, &vec![]),
    30 => Production::new(30, 7, &vec![ProductionItem::NonTerminal(21),ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    31 => Production::new(31, 8, &vec![ProductionItem::Terminal(19),]),
    43 => Production::new(43, 11, &vec![ProductionItem::NonTerminal(24),ProductionItem::NonTerminal(26),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(15),]),
    39 => Production::new(39, 24, &vec![ProductionItem::Terminal(2),]),
    36 => Production::new(36, 23, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    23 => Production::new(23, 19, &vec![ProductionItem::Terminal(10),]),
    11 => Production::new(11, 16, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    7 => Production::new(7, 15, &vec![]),
    3 => Production::new(3, 13, &vec![ProductionItem::NonTerminal(12),ProductionItem::NonTerminal(13),]),
    2 => Production::new(2, 13, &vec![]),
    40 => Production::new(40, 25, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(24),ProductionItem::Terminal(13),]),
    18 => Production::new(18, 17, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(2),ProductionItem::Terminal(13),]),
    24 => Production::new(24, 19, &vec![ProductionItem::Terminal(11),]),
    8 => Production::new(8, 15, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    26 => Production::new(26, 20, &vec![ProductionItem::Terminal(11),]),
    15 => Production::new(15, 17, &vec![ProductionItem::Terminal(3),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(19),ProductionItem::NonTerminal(20),]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    28 => Production::new(28, 21, &vec![]),
    29 => Production::new(29, 21, &vec![ProductionItem::NonTerminal(9),]),
    32 => Production::new(32, 9, &vec![ProductionItem::Terminal(14),ProductionItem::NonTerminal(11),]),
    35 => Production::new(35, 23, &vec![]),
    37 => Production::new(37, 10, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(23),]),
    20 => Production::new(20, 18, &vec![ProductionItem::NonTerminal(6),]),
    41 => Production::new(41, 26, &vec![]),
    19 => Production::new(19, 18, &vec![]),
    42 => Production::new(42, 26, &vec![ProductionItem::NonTerminal(25),]),
    34 => Production::new(34, 22, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(11),]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(13),]),
    17 => Production::new(17, 17, &vec![ProductionItem::Terminal(2),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(8),]),
    0 => Production::new(0, 12, &vec![ProductionItem::NonTerminal(1),]),
    33 => Production::new(33, 9, &vec![ProductionItem::Terminal(15),ProductionItem::Terminal(16),ProductionItem::NonTerminal(10),ProductionItem::Terminal(17),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    1 => "parser_rule",
    4 => "epsilon",
    0 => "rule_list",
    3 => "alternative",
    10 => "attribute_list",
    9 => "annotation",
    8 => "regular",
    6 => "ebnf_suffix",
    5 => "element",
    2 => "block",
    7 => "lexer_rule",
    11 => "attribute",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    10 => "PLUS",
    17 => "RBRACKET",
    5 => "SEMI",
    20 => "WHITE_SPACE",
    0 => "_START",
    9 => "STAR",
    14 => "AT",
    11 => "QUESTION",
    8 => "EPSILON",
    15 => "SHARP",
    16 => "LBRACKET",
    6 => "COMMA",
    7 => "OR",
    3 => "TOKEN_REF",
    2 => "RULE_REF",
    13 => "RPAREN",
    12 => "LPAREN",
    1 => "_STOP",
    4 => "COLON",
    18 => "STRING_LITERAL",
    19 => "REGULAR_LITERAL",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (3, 7),
    (18, 2),
    (17, 5),
    (16, 7),
    (13, 1),
    (5, 7),
    (5, 5),
    (20, 18),
    (24, 6),
    (8, 5),
    (18, 12),
    (2, 5),
    (7, 2),
    (2, 13),
    (14, 13),
    (4, 13),
    (4, 5),
    (11, 6),
    (16, 13),
    (25, 3),
    (12, 14),
    (17, 3),
    (20, 3),
    (14, 7),
    (17, 18),
    (17, 2),
    (7, 3),
    (18, 3),
    (18, 13),
    (17, 13),
    (10, 17),
    (19, 3),
    (6, 13),
    (16, 5),
    (12, 15),
    (19, 18),
    (17, 9),
    (1, 1),
    (18, 18),
    (25, 6),
    (9, 3),
    (11, 3),
    (26, 17),
    (7, 15),
    (5, 18),
    (19, 12),
    (20, 12),
    (20, 7),
    (6, 2),
    (1, 2),
    (3, 13),
    (6, 3),
    (14, 5),
    (5, 12),
    (12, 1),
    (7, 14),
    (26, 3),
    (6, 7),
    (15, 5),
    (12, 2),
    (26, 6),
    (6, 12),
    (18, 7),
    (24, 12),
    (5, 3),
    (19, 2),
    (19, 11),
    (24, 13),
    (0, 1),
    (4, 7),
    (11, 17),
    (22, 17),
    (6, 5),
    (17, 12),
    (21, 3),
    (24, 3),
    (19, 5),
    (24, 17),
    (20, 5),
    (25, 17),
    (3, 5),
    (17, 7),
    (20, 13),
    (18, 5),
    (23, 17),
    (19, 13),
    (1, 14),
    (17, 10),
    (6, 18),
    (17, 11),
    (5, 2),
    (1, 15),
    (12, 3),
    (19, 7),
    (5, 13),
    (22, 6),
    (20, 2),
    (1, 3),
    (7, 1),
    (15, 13),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const PARSER_RULE: usize = 1; 
  pub const EPSILON: usize = 4; 
  pub const RULE_LIST: usize = 0; 
  pub const ALTERNATIVE: usize = 3; 
  pub const ATTRIBUTE_LIST: usize = 10; 
  pub const ANNOTATION: usize = 9; 
  pub const REGULAR: usize = 8; 
  pub const EBNF_SUFFIX: usize = 6; 
  pub const ELEMENT: usize = 5; 
  pub const BLOCK: usize = 2; 
  pub const LEXER_RULE: usize = 7; 
  pub const ATTRIBUTE: usize = 11; 



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
  
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
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
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn attribute_list(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeListContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE_LIST);
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

