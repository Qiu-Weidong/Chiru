

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::context::{
   ParserRuleContext, EpsilonContext, LexerRuleContext, RuleListContext, RegularContext, AttributeContext, EbnfSuffixContext, BlockContext, AnnotationContext, AlternativeContext, ElementContext, AttributeListContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (20, 13) => 25,
    (3, 2) => 12,
    (1, 2) => 5,
    (0, 2) => 4,
    (18, 18) => 19,
    (3, 8) => 13,
    (18, 10) => 20,
    (14, 7) => 6,
    (18, 11) => 20,
    (21, 14) => 29,
    (2, 18) => 9,
    (15, 5) => 7,
    (0, 15) => 4,
    (17, 12) => 18,
    (22, 6) => 34,
    (2, 8) => 9,
    (2, 3) => 9,
    (18, 13) => 19,
    (9, 15) => 33,
    (0, 3) => 4,
    (18, 5) => 19,
    (5, 18) => 21,
    (20, 12) => 25,
    (17, 3) => 15,
    (23, 17) => 35,
    (4, 8) => 14,
    (25, 12) => 40,
    (2, 2) => 9,
    (16, 3) => 11,
    (16, 2) => 11,
    (10, 2) => 37,
    (20, 7) => 25,
    (18, 9) => 20,
    (15, 13) => 7,
    (13, 15) => 3,
    (10, 3) => 37,
    (26, 12) => 42,
    (8, 19) => 31,
    (24, 2) => 39,
    (21, 3) => 28,
    (12, 14) => 1,
    (24, 3) => 38,
    (19, 11) => 24,
    (5, 12) => 21,
    (7, 14) => 30,
    (7, 3) => 30,
    (20, 11) => 26,
    (26, 17) => 41,
    (20, 2) => 25,
    (20, 18) => 25,
    (12, 2) => 0,
    (3, 12) => 12,
    (16, 13) => 10,
    (11, 3) => 43,
    (19, 10) => 23,
    (20, 5) => 25,
    (23, 6) => 36,
    (19, 9) => 22,
    (0, 14) => 4,
    (13, 2) => 3,
    (12, 3) => 1,
    (6, 10) => 27,
    (17, 2) => 17,
    (18, 2) => 19,
    (16, 5) => 10,
    (20, 3) => 25,
    (18, 7) => 19,
    (6, 11) => 27,
    (16, 12) => 11,
    (3, 3) => 12,
    (21, 15) => 29,
    (3, 18) => 12,
    (11, 2) => 43,
    (17, 18) => 16,
    (18, 12) => 19,
    (2, 12) => 9,
    (13, 14) => 3,
    (16, 18) => 11,
    (26, 3) => 41,
    (16, 7) => 10,
    (6, 9) => 27,
    (15, 7) => 8,
    (9, 14) => 32,
    (13, 1) => 2,
    (12, 15) => 1,
    (18, 3) => 19,
    (13, 3) => 3,
    (7, 15) => 30,
    (5, 2) => 21,
    (0, 1) => 4,
    (26, 6) => 41,
    (5, 3) => 21,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    16 => Production::new(16, 17, &vec![ProductionItem::Terminal(18),]),
    17 => Production::new(17, 17, &vec![ProductionItem::Terminal(2),]),
    34 => Production::new(34, 22, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(11),]),
    6 => Production::new(6, 14, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(3),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(15),]),
    2 => Production::new(2, 13, &vec![]),
    33 => Production::new(33, 9, &vec![ProductionItem::Terminal(15),ProductionItem::Terminal(16),ProductionItem::NonTerminal(10),ProductionItem::Terminal(17),]),
    25 => Production::new(25, 20, &vec![]),
    19 => Production::new(19, 18, &vec![]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    23 => Production::new(23, 19, &vec![ProductionItem::Terminal(10),]),
    42 => Production::new(42, 26, &vec![ProductionItem::NonTerminal(25),]),
    7 => Production::new(7, 15, &vec![]),
    10 => Production::new(10, 16, &vec![]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(13),]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    3 => Production::new(3, 13, &vec![ProductionItem::NonTerminal(12),ProductionItem::NonTerminal(13),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(8),]),
    20 => Production::new(20, 18, &vec![ProductionItem::NonTerminal(6),]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    28 => Production::new(28, 21, &vec![]),
    36 => Production::new(36, 23, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    24 => Production::new(24, 19, &vec![ProductionItem::Terminal(11),]),
    38 => Production::new(38, 24, &vec![ProductionItem::Terminal(3),]),
    22 => Production::new(22, 19, &vec![ProductionItem::Terminal(9),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(19),ProductionItem::NonTerminal(20),]),
    41 => Production::new(41, 26, &vec![]),
    1 => Production::new(1, 12, &vec![ProductionItem::NonTerminal(7),]),
    15 => Production::new(15, 17, &vec![ProductionItem::Terminal(3),]),
    31 => Production::new(31, 8, &vec![ProductionItem::Terminal(19),]),
    32 => Production::new(32, 9, &vec![ProductionItem::Terminal(14),ProductionItem::NonTerminal(11),]),
    29 => Production::new(29, 21, &vec![ProductionItem::NonTerminal(9),]),
    18 => Production::new(18, 17, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(2),ProductionItem::Terminal(13),]),
    30 => Production::new(30, 7, &vec![ProductionItem::NonTerminal(21),ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    35 => Production::new(35, 23, &vec![]),
    40 => Production::new(40, 25, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(24),ProductionItem::Terminal(13),]),
    8 => Production::new(8, 15, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    11 => Production::new(11, 16, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    0 => Production::new(0, 12, &vec![ProductionItem::NonTerminal(1),]),
    26 => Production::new(26, 20, &vec![ProductionItem::Terminal(11),]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    37 => Production::new(37, 10, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(23),]),
    39 => Production::new(39, 24, &vec![ProductionItem::Terminal(2),]),
    43 => Production::new(43, 11, &vec![ProductionItem::NonTerminal(24),ProductionItem::NonTerminal(26),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    1 => "parser_rule",
    4 => "epsilon",
    7 => "lexer_rule",
    0 => "rule_list",
    8 => "regular",
    11 => "attribute",
    6 => "ebnf_suffix",
    2 => "block",
    9 => "annotation",
    3 => "alternative",
    5 => "element",
    10 => "attribute_list",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    13 => "RPAREN",
    0 => "_START",
    19 => "REGULAR_LITERAL",
    1 => "_STOP",
    8 => "EPSILON",
    14 => "AT",
    20 => "WHITE_SPACE",
    17 => "RBRACKET",
    5 => "SEMI",
    12 => "LPAREN",
    15 => "SHARP",
    9 => "STAR",
    3 => "TOKEN_REF",
    16 => "LBRACKET",
    2 => "RULE_REF",
    11 => "QUESTION",
    6 => "COMMA",
    7 => "OR",
    4 => "COLON",
    18 => "STRING_LITERAL",
    10 => "PLUS",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (6, 5),
    (1, 1),
    (24, 12),
    (20, 7),
    (7, 2),
    (0, 1),
    (4, 13),
    (5, 7),
    (17, 9),
    (19, 7),
    (17, 12),
    (21, 3),
    (26, 3),
    (26, 6),
    (18, 13),
    (12, 1),
    (11, 3),
    (16, 5),
    (25, 3),
    (25, 17),
    (18, 7),
    (7, 1),
    (7, 15),
    (5, 12),
    (3, 7),
    (20, 5),
    (19, 12),
    (5, 5),
    (3, 13),
    (17, 2),
    (18, 5),
    (19, 2),
    (20, 2),
    (20, 3),
    (1, 15),
    (6, 2),
    (6, 13),
    (10, 17),
    (7, 3),
    (15, 13),
    (24, 6),
    (18, 12),
    (2, 5),
    (12, 14),
    (5, 18),
    (20, 18),
    (17, 11),
    (14, 5),
    (14, 13),
    (3, 5),
    (18, 3),
    (24, 3),
    (19, 5),
    (1, 2),
    (19, 18),
    (11, 6),
    (24, 17),
    (4, 5),
    (5, 2),
    (16, 13),
    (24, 13),
    (16, 7),
    (14, 7),
    (2, 13),
    (17, 7),
    (17, 10),
    (1, 14),
    (9, 3),
    (6, 3),
    (5, 13),
    (5, 3),
    (17, 5),
    (6, 7),
    (7, 14),
    (13, 1),
    (19, 11),
    (22, 17),
    (20, 12),
    (8, 5),
    (17, 3),
    (11, 17),
    (23, 17),
    (20, 13),
    (12, 2),
    (6, 12),
    (25, 6),
    (6, 18),
    (17, 13),
    (1, 3),
    (17, 18),
    (19, 13),
    (22, 6),
    (4, 7),
    (18, 18),
    (26, 17),
    (15, 5),
    (19, 3),
    (12, 3),
    (18, 2),
    (12, 15),
  };
}


impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const PARSER_RULE: usize = 1; 
  pub const EPSILON: usize = 4; 
  pub const LEXER_RULE: usize = 7; 
  pub const RULE_LIST: usize = 0; 
  pub const REGULAR: usize = 8; 
  pub const ATTRIBUTE: usize = 11; 
  pub const EBNF_SUFFIX: usize = 6; 
  pub const BLOCK: usize = 2; 
  pub const ANNOTATION: usize = 9; 
  pub const ALTERNATIVE: usize = 3; 
  pub const ELEMENT: usize = 5; 
  pub const ATTRIBUTE_LIST: usize = 10; 



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
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULE_LIST);
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
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
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
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn attribute_list(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeListContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE_LIST);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

