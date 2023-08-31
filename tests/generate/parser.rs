

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::context::{
   BlockContext, LexerRuleContext, RuleListContext, EbnfSuffixContext, EpsilonContext, RegularContext, AttributeContext, ElementContext, AttributeListContext, AnnotationContext, AlternativeContext, ParserRuleContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (18, 7) => 19,
    (25, 12) => 40,
    (10, 3) => 37,
    (3, 8) => 13,
    (20, 3) => 25,
    (15, 13) => 7,
    (7, 14) => 30,
    (6, 10) => 27,
    (22, 6) => 34,
    (18, 11) => 20,
    (18, 9) => 20,
    (16, 13) => 10,
    (11, 2) => 43,
    (18, 18) => 19,
    (18, 2) => 19,
    (2, 18) => 9,
    (20, 11) => 26,
    (13, 1) => 2,
    (19, 9) => 22,
    (3, 12) => 12,
    (13, 2) => 3,
    (13, 14) => 3,
    (20, 7) => 25,
    (8, 19) => 31,
    (0, 14) => 4,
    (20, 2) => 25,
    (7, 3) => 30,
    (1, 2) => 5,
    (2, 3) => 9,
    (15, 7) => 8,
    (18, 12) => 19,
    (17, 18) => 16,
    (16, 18) => 11,
    (16, 5) => 10,
    (5, 12) => 21,
    (2, 2) => 9,
    (26, 17) => 41,
    (16, 3) => 11,
    (17, 3) => 15,
    (9, 15) => 33,
    (21, 3) => 28,
    (20, 5) => 25,
    (0, 15) => 4,
    (12, 15) => 1,
    (7, 15) => 30,
    (18, 10) => 20,
    (20, 12) => 25,
    (5, 3) => 21,
    (0, 2) => 4,
    (5, 2) => 21,
    (16, 7) => 10,
    (24, 3) => 38,
    (10, 2) => 37,
    (23, 6) => 36,
    (21, 14) => 29,
    (20, 13) => 25,
    (19, 11) => 24,
    (12, 2) => 0,
    (14, 7) => 6,
    (17, 12) => 18,
    (13, 3) => 3,
    (17, 2) => 17,
    (9, 14) => 32,
    (21, 15) => 29,
    (3, 18) => 12,
    (13, 15) => 3,
    (4, 8) => 14,
    (0, 1) => 4,
    (2, 8) => 9,
    (19, 10) => 23,
    (26, 3) => 41,
    (5, 18) => 21,
    (26, 6) => 41,
    (12, 3) => 1,
    (12, 14) => 1,
    (24, 2) => 39,
    (18, 13) => 19,
    (3, 2) => 12,
    (3, 3) => 12,
    (20, 18) => 25,
    (16, 2) => 11,
    (6, 9) => 27,
    (0, 3) => 4,
    (11, 3) => 43,
    (16, 12) => 11,
    (6, 11) => 27,
    (26, 12) => 42,
    (15, 5) => 7,
    (2, 12) => 9,
    (23, 17) => 35,
    (18, 3) => 19,
    (18, 5) => 19,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    25 => Production::new(25, 20, &vec![]),
    34 => Production::new(34, 22, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(11),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(19),ProductionItem::NonTerminal(20),]),
    38 => Production::new(38, 24, &vec![ProductionItem::Terminal(3),]),
    0 => Production::new(0, 12, &vec![ProductionItem::NonTerminal(1),]),
    7 => Production::new(7, 15, &vec![]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    28 => Production::new(28, 21, &vec![]),
    6 => Production::new(6, 14, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(3),]),
    37 => Production::new(37, 10, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(23),]),
    41 => Production::new(41, 26, &vec![]),
    11 => Production::new(11, 16, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(13),]),
    18 => Production::new(18, 17, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(2),ProductionItem::Terminal(13),]),
    30 => Production::new(30, 7, &vec![ProductionItem::NonTerminal(21),ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(15),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(8),]),
    15 => Production::new(15, 17, &vec![ProductionItem::Terminal(3),]),
    29 => Production::new(29, 21, &vec![ProductionItem::NonTerminal(9),]),
    35 => Production::new(35, 23, &vec![]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    20 => Production::new(20, 18, &vec![ProductionItem::NonTerminal(6),]),
    39 => Production::new(39, 24, &vec![ProductionItem::Terminal(2),]),
    2 => Production::new(2, 13, &vec![]),
    8 => Production::new(8, 15, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    23 => Production::new(23, 19, &vec![ProductionItem::Terminal(10),]),
    31 => Production::new(31, 8, &vec![ProductionItem::Terminal(19),]),
    1 => Production::new(1, 12, &vec![ProductionItem::NonTerminal(7),]),
    10 => Production::new(10, 16, &vec![]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    17 => Production::new(17, 17, &vec![ProductionItem::Terminal(2),]),
    33 => Production::new(33, 9, &vec![ProductionItem::Terminal(15),ProductionItem::Terminal(16),ProductionItem::NonTerminal(10),ProductionItem::Terminal(17),]),
    36 => Production::new(36, 23, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    32 => Production::new(32, 9, &vec![ProductionItem::Terminal(14),ProductionItem::NonTerminal(11),]),
    43 => Production::new(43, 11, &vec![ProductionItem::NonTerminal(24),ProductionItem::NonTerminal(26),]),
    19 => Production::new(19, 18, &vec![]),
    42 => Production::new(42, 26, &vec![ProductionItem::NonTerminal(25),]),
    26 => Production::new(26, 20, &vec![ProductionItem::Terminal(11),]),
    22 => Production::new(22, 19, &vec![ProductionItem::Terminal(9),]),
    40 => Production::new(40, 25, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(24),ProductionItem::Terminal(13),]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    24 => Production::new(24, 19, &vec![ProductionItem::Terminal(11),]),
    3 => Production::new(3, 13, &vec![ProductionItem::NonTerminal(12),ProductionItem::NonTerminal(13),]),
    16 => Production::new(16, 17, &vec![ProductionItem::Terminal(18),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    2 => "block",
    7 => "lexer_rule",
    0 => "rule_list",
    6 => "ebnf_suffix",
    4 => "epsilon",
    8 => "regular",
    11 => "attribute",
    5 => "element",
    10 => "attribute_list",
    9 => "annotation",
    3 => "alternative",
    1 => "parser_rule",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    7 => "OR",
    10 => "PLUS",
    13 => "RPAREN",
    12 => "LPAREN",
    5 => "SEMI",
    6 => "COMMA",
    0 => "_START",
    1 => "_STOP",
    15 => "SHARP",
    17 => "RBRACKET",
    20 => "WHITE_SPACE",
    2 => "RULE_REF",
    14 => "AT",
    11 => "QUESTION",
    19 => "REGULAR_LITERAL",
    18 => "STRING_LITERAL",
    16 => "LBRACKET",
    9 => "STAR",
    8 => "EPSILON",
    4 => "COLON",
    3 => "TOKEN_REF",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (20, 7),
    (18, 7),
    (13, 1),
    (2, 5),
    (2, 13),
    (17, 18),
    (26, 6),
    (7, 14),
    (5, 12),
    (20, 2),
    (26, 17),
    (14, 13),
    (12, 15),
    (16, 5),
    (19, 12),
    (24, 13),
    (1, 14),
    (1, 15),
    (20, 18),
    (19, 11),
    (9, 3),
    (20, 12),
    (11, 6),
    (7, 3),
    (6, 12),
    (25, 3),
    (20, 3),
    (1, 2),
    (7, 2),
    (16, 7),
    (7, 1),
    (20, 5),
    (15, 5),
    (17, 13),
    (17, 3),
    (1, 1),
    (18, 5),
    (5, 5),
    (17, 7),
    (3, 13),
    (6, 13),
    (5, 18),
    (5, 3),
    (26, 3),
    (19, 13),
    (8, 5),
    (15, 13),
    (19, 7),
    (16, 13),
    (11, 3),
    (5, 7),
    (24, 3),
    (11, 17),
    (24, 6),
    (24, 12),
    (18, 2),
    (18, 13),
    (18, 18),
    (19, 5),
    (22, 6),
    (5, 13),
    (6, 2),
    (6, 18),
    (14, 5),
    (17, 2),
    (22, 17),
    (24, 17),
    (3, 7),
    (6, 5),
    (12, 1),
    (14, 7),
    (17, 11),
    (23, 17),
    (4, 5),
    (19, 3),
    (25, 6),
    (18, 12),
    (19, 2),
    (21, 3),
    (17, 5),
    (10, 17),
    (0, 1),
    (7, 15),
    (12, 3),
    (12, 14),
    (4, 13),
    (12, 2),
    (4, 7),
    (3, 5),
    (1, 3),
    (20, 13),
    (17, 9),
    (5, 2),
    (18, 3),
    (25, 17),
    (6, 3),
    (19, 18),
    (6, 7),
    (17, 12),
    (17, 10),
  };
}


impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const BLOCK: usize = 2; 
  pub const LEXER_RULE: usize = 7; 
  pub const RULE_LIST: usize = 0; 
  pub const EBNF_SUFFIX: usize = 6; 
  pub const EPSILON: usize = 4; 
  pub const REGULAR: usize = 8; 
  pub const ATTRIBUTE: usize = 11; 
  pub const ELEMENT: usize = 5; 
  pub const ATTRIBUTE_LIST: usize = 10; 
  pub const ANNOTATION: usize = 9; 
  pub const ALTERNATIVE: usize = 3; 
  pub const PARSER_RULE: usize = 1; 



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
  
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULE_LIST);
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
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
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
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

