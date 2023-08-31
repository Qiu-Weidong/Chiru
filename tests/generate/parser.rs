

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::context::{
   AttributeContext, LexerRuleContext, AttributeListContext, ElementContext, RegularContext, EpsilonContext, EbnfSuffixContext, AlternativeContext, RuleListContext, AnnotationContext, ParserRuleContext, BlockContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (10, 3) => 37,
    (25, 12) => 40,
    (0, 1) => 4,
    (18, 13) => 19,
    (20, 13) => 25,
    (6, 9) => 27,
    (6, 11) => 27,
    (18, 10) => 20,
    (12, 14) => 1,
    (23, 17) => 35,
    (16, 12) => 11,
    (18, 11) => 20,
    (16, 3) => 11,
    (7, 14) => 30,
    (6, 10) => 27,
    (15, 5) => 7,
    (26, 3) => 41,
    (26, 6) => 41,
    (11, 2) => 43,
    (26, 17) => 41,
    (7, 15) => 30,
    (21, 15) => 29,
    (9, 14) => 32,
    (0, 14) => 4,
    (1, 2) => 5,
    (15, 7) => 8,
    (18, 12) => 19,
    (16, 18) => 11,
    (16, 2) => 11,
    (18, 9) => 20,
    (3, 12) => 12,
    (11, 3) => 43,
    (20, 7) => 25,
    (24, 2) => 39,
    (5, 18) => 21,
    (18, 3) => 19,
    (0, 2) => 4,
    (13, 1) => 2,
    (20, 3) => 25,
    (2, 18) => 9,
    (3, 8) => 13,
    (2, 2) => 9,
    (0, 15) => 4,
    (21, 3) => 28,
    (19, 10) => 23,
    (18, 2) => 19,
    (21, 14) => 29,
    (18, 7) => 19,
    (2, 8) => 9,
    (5, 12) => 21,
    (17, 2) => 17,
    (16, 7) => 10,
    (4, 8) => 14,
    (14, 7) => 6,
    (20, 12) => 25,
    (9, 15) => 33,
    (26, 12) => 42,
    (12, 15) => 1,
    (12, 2) => 0,
    (16, 5) => 10,
    (17, 18) => 16,
    (16, 13) => 10,
    (3, 18) => 12,
    (13, 2) => 3,
    (2, 3) => 9,
    (3, 3) => 12,
    (20, 5) => 25,
    (24, 3) => 38,
    (19, 9) => 22,
    (7, 3) => 30,
    (12, 3) => 1,
    (23, 6) => 36,
    (18, 18) => 19,
    (19, 11) => 24,
    (13, 3) => 3,
    (2, 12) => 9,
    (0, 3) => 4,
    (22, 6) => 34,
    (17, 12) => 18,
    (5, 3) => 21,
    (20, 11) => 26,
    (20, 2) => 25,
    (13, 15) => 3,
    (15, 13) => 7,
    (5, 2) => 21,
    (8, 19) => 31,
    (10, 2) => 37,
    (20, 18) => 25,
    (3, 2) => 12,
    (18, 5) => 19,
    (13, 14) => 3,
    (17, 3) => 15,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    7 => Production::new(7, 15, &vec![]),
    22 => Production::new(22, 19, &vec![ProductionItem::Terminal(9),]),
    31 => Production::new(31, 8, &vec![ProductionItem::Terminal(19),]),
    38 => Production::new(38, 24, &vec![ProductionItem::Terminal(3),]),
    15 => Production::new(15, 17, &vec![ProductionItem::Terminal(3),]),
    23 => Production::new(23, 19, &vec![ProductionItem::Terminal(10),]),
    35 => Production::new(35, 23, &vec![]),
    30 => Production::new(30, 7, &vec![ProductionItem::NonTerminal(21),ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    17 => Production::new(17, 17, &vec![ProductionItem::Terminal(2),]),
    10 => Production::new(10, 16, &vec![]),
    16 => Production::new(16, 17, &vec![ProductionItem::Terminal(18),]),
    29 => Production::new(29, 21, &vec![ProductionItem::NonTerminal(9),]),
    41 => Production::new(41, 26, &vec![]),
    11 => Production::new(11, 16, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    1 => Production::new(1, 12, &vec![ProductionItem::NonTerminal(7),]),
    32 => Production::new(32, 9, &vec![ProductionItem::Terminal(14),ProductionItem::NonTerminal(11),]),
    40 => Production::new(40, 25, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(24),ProductionItem::Terminal(13),]),
    42 => Production::new(42, 26, &vec![ProductionItem::NonTerminal(25),]),
    20 => Production::new(20, 18, &vec![ProductionItem::NonTerminal(6),]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(13),]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    37 => Production::new(37, 10, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(23),]),
    0 => Production::new(0, 12, &vec![ProductionItem::NonTerminal(1),]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    36 => Production::new(36, 23, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    39 => Production::new(39, 24, &vec![ProductionItem::Terminal(2),]),
    3 => Production::new(3, 13, &vec![ProductionItem::NonTerminal(12),ProductionItem::NonTerminal(13),]),
    28 => Production::new(28, 21, &vec![]),
    34 => Production::new(34, 22, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(11),]),
    19 => Production::new(19, 18, &vec![]),
    2 => Production::new(2, 13, &vec![]),
    6 => Production::new(6, 14, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(3),]),
    43 => Production::new(43, 11, &vec![ProductionItem::NonTerminal(24),ProductionItem::NonTerminal(26),]),
    18 => Production::new(18, 17, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(2),ProductionItem::Terminal(13),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(8),]),
    25 => Production::new(25, 20, &vec![]),
    26 => Production::new(26, 20, &vec![ProductionItem::Terminal(11),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(19),ProductionItem::NonTerminal(20),]),
    33 => Production::new(33, 9, &vec![ProductionItem::Terminal(15),ProductionItem::Terminal(16),ProductionItem::NonTerminal(10),ProductionItem::Terminal(17),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    8 => Production::new(8, 15, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    24 => Production::new(24, 19, &vec![ProductionItem::Terminal(11),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(15),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    11 => "attribute",
    7 => "lexer_rule",
    10 => "attribute_list",
    5 => "element",
    8 => "regular",
    4 => "epsilon",
    6 => "ebnf_suffix",
    3 => "alternative",
    0 => "rule_list",
    9 => "annotation",
    1 => "parser_rule",
    2 => "block",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    4 => "COLON",
    7 => "OR",
    1 => "_STOP",
    19 => "REGULAR_LITERAL",
    8 => "EPSILON",
    3 => "TOKEN_REF",
    11 => "QUESTION",
    0 => "_START",
    16 => "LBRACKET",
    9 => "STAR",
    13 => "RPAREN",
    14 => "AT",
    18 => "STRING_LITERAL",
    15 => "SHARP",
    17 => "RBRACKET",
    5 => "SEMI",
    20 => "WHITE_SPACE",
    6 => "COMMA",
    12 => "LPAREN",
    2 => "RULE_REF",
    10 => "PLUS",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (4, 13),
    (2, 13),
    (19, 18),
    (17, 11),
    (5, 18),
    (6, 3),
    (7, 2),
    (5, 5),
    (3, 5),
    (4, 5),
    (23, 17),
    (6, 7),
    (26, 6),
    (26, 17),
    (1, 14),
    (20, 2),
    (25, 6),
    (12, 14),
    (1, 3),
    (21, 3),
    (6, 13),
    (0, 1),
    (3, 7),
    (7, 3),
    (18, 2),
    (2, 5),
    (17, 3),
    (18, 5),
    (20, 12),
    (6, 2),
    (7, 14),
    (14, 5),
    (19, 12),
    (17, 5),
    (18, 18),
    (18, 7),
    (19, 3),
    (18, 13),
    (8, 5),
    (16, 7),
    (9, 3),
    (25, 17),
    (26, 3),
    (1, 1),
    (16, 5),
    (5, 13),
    (11, 17),
    (17, 10),
    (12, 3),
    (12, 15),
    (5, 7),
    (19, 13),
    (11, 6),
    (19, 7),
    (19, 5),
    (1, 2),
    (19, 2),
    (14, 13),
    (22, 17),
    (4, 7),
    (19, 11),
    (20, 13),
    (5, 2),
    (20, 5),
    (17, 2),
    (24, 12),
    (16, 13),
    (15, 13),
    (25, 3),
    (12, 2),
    (17, 13),
    (5, 3),
    (7, 15),
    (18, 3),
    (15, 5),
    (1, 15),
    (20, 3),
    (3, 13),
    (7, 1),
    (17, 18),
    (13, 1),
    (6, 18),
    (12, 1),
    (6, 5),
    (10, 17),
    (17, 9),
    (24, 6),
    (24, 17),
    (17, 12),
    (5, 12),
    (22, 6),
    (20, 18),
    (17, 7),
    (18, 12),
    (14, 7),
    (11, 3),
    (6, 12),
    (24, 13),
    (20, 7),
    (24, 3),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ATTRIBUTE: usize = 11; 
  pub const LEXER_RULE: usize = 7; 
  pub const ATTRIBUTE_LIST: usize = 10; 
  pub const ELEMENT: usize = 5; 
  pub const REGULAR: usize = 8; 
  pub const EPSILON: usize = 4; 
  pub const EBNF_SUFFIX: usize = 6; 
  pub const ALTERNATIVE: usize = 3; 
  pub const RULE_LIST: usize = 0; 
  pub const ANNOTATION: usize = 9; 
  pub const PARSER_RULE: usize = 1; 
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
  
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn attribute_list(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeListContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE_LIST);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
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
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

