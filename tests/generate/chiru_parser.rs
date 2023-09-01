

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::chiru_context::{
   ParserRuleContext, ElementContext, AttributeContext, AnnotationContext, EpsilonContext, RegularContext, LexerRuleContext, AlternativeContext, AttributeListContext, EbnfSuffixContext, RuleListContext, BlockContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (25, 12) => 40,
    (2, 18) => 9,
    (16, 7) => 10,
    (15, 5) => 7,
    (15, 13) => 7,
    (19, 11) => 24,
    (12, 2) => 0,
    (9, 15) => 33,
    (23, 17) => 35,
    (5, 18) => 21,
    (2, 2) => 9,
    (22, 6) => 34,
    (0, 2) => 4,
    (3, 2) => 12,
    (18, 10) => 20,
    (25, 3) => 39,
    (11, 2) => 41,
    (16, 18) => 11,
    (16, 3) => 11,
    (20, 7) => 25,
    (8, 19) => 31,
    (16, 13) => 10,
    (5, 3) => 21,
    (10, 2) => 37,
    (7, 15) => 30,
    (3, 18) => 12,
    (20, 18) => 25,
    (12, 14) => 1,
    (2, 12) => 9,
    (18, 12) => 19,
    (19, 10) => 23,
    (17, 12) => 18,
    (12, 3) => 1,
    (0, 3) => 4,
    (5, 2) => 21,
    (4, 8) => 14,
    (18, 11) => 20,
    (1, 2) => 5,
    (13, 3) => 3,
    (18, 9) => 20,
    (6, 10) => 27,
    (0, 14) => 4,
    (16, 5) => 10,
    (2, 3) => 9,
    (24, 12) => 38,
    (3, 12) => 12,
    (20, 13) => 25,
    (18, 2) => 19,
    (17, 2) => 17,
    (12, 15) => 1,
    (21, 14) => 29,
    (25, 6) => 39,
    (18, 3) => 19,
    (13, 1) => 2,
    (20, 5) => 25,
    (5, 12) => 21,
    (17, 3) => 15,
    (13, 2) => 3,
    (9, 14) => 32,
    (14, 7) => 6,
    (17, 18) => 16,
    (19, 9) => 22,
    (0, 1) => 4,
    (6, 11) => 27,
    (7, 14) => 30,
    (20, 2) => 25,
    (0, 15) => 4,
    (16, 12) => 11,
    (21, 15) => 29,
    (13, 15) => 3,
    (20, 12) => 25,
    (16, 2) => 11,
    (3, 8) => 13,
    (3, 3) => 12,
    (21, 3) => 28,
    (20, 11) => 26,
    (18, 18) => 19,
    (20, 3) => 25,
    (18, 13) => 19,
    (7, 3) => 30,
    (6, 9) => 27,
    (18, 7) => 19,
    (23, 6) => 36,
    (18, 5) => 19,
    (13, 14) => 3,
    (15, 7) => 8,
    (25, 17) => 39,
    (2, 8) => 9,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    36 => Production::new(36, 23, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    8 => Production::new(8, 15, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    0 => Production::new(0, 12, &vec![ProductionItem::NonTerminal(1),]),
    37 => Production::new(37, 10, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(23),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(19),ProductionItem::NonTerminal(20),]),
    7 => Production::new(7, 15, &vec![]),
    34 => Production::new(34, 22, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(11),]),
    17 => Production::new(17, 17, &vec![ProductionItem::Terminal(2),]),
    18 => Production::new(18, 17, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(2),ProductionItem::Terminal(13),]),
    3 => Production::new(3, 13, &vec![ProductionItem::NonTerminal(12),ProductionItem::NonTerminal(13),]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(13),]),
    22 => Production::new(22, 19, &vec![ProductionItem::Terminal(9),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(15),]),
    31 => Production::new(31, 8, &vec![ProductionItem::Terminal(19),]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    24 => Production::new(24, 19, &vec![ProductionItem::Terminal(11),]),
    32 => Production::new(32, 9, &vec![ProductionItem::Terminal(14),ProductionItem::NonTerminal(11),]),
    6 => Production::new(6, 14, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(3),]),
    23 => Production::new(23, 19, &vec![ProductionItem::Terminal(10),]),
    1 => Production::new(1, 12, &vec![ProductionItem::NonTerminal(7),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(8),]),
    33 => Production::new(33, 9, &vec![ProductionItem::Terminal(15),ProductionItem::Terminal(16),ProductionItem::NonTerminal(10),ProductionItem::Terminal(17),]),
    35 => Production::new(35, 23, &vec![]),
    29 => Production::new(29, 21, &vec![ProductionItem::NonTerminal(9),]),
    38 => Production::new(38, 24, &vec![ProductionItem::Terminal(12),ProductionItem::Terminal(3),ProductionItem::Terminal(13),]),
    19 => Production::new(19, 18, &vec![]),
    25 => Production::new(25, 20, &vec![]),
    15 => Production::new(15, 17, &vec![ProductionItem::Terminal(3),]),
    40 => Production::new(40, 25, &vec![ProductionItem::NonTerminal(24),]),
    10 => Production::new(10, 16, &vec![]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    20 => Production::new(20, 18, &vec![ProductionItem::NonTerminal(6),]),
    28 => Production::new(28, 21, &vec![]),
    2 => Production::new(2, 13, &vec![]),
    39 => Production::new(39, 25, &vec![]),
    41 => Production::new(41, 11, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(25),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    11 => Production::new(11, 16, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    26 => Production::new(26, 20, &vec![ProductionItem::Terminal(11),]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    30 => Production::new(30, 7, &vec![ProductionItem::NonTerminal(21),ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    16 => Production::new(16, 17, &vec![ProductionItem::Terminal(18),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    1 => "parser_rule",
    5 => "element",
    11 => "attribute",
    9 => "annotation",
    4 => "epsilon",
    8 => "regular",
    7 => "lexer_rule",
    3 => "alternative",
    10 => "attribute_list",
    6 => "ebnf_suffix",
    0 => "rule_list",
    2 => "block",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    1 => "_STOP",
    17 => "RBRACKET",
    0 => "_START",
    4 => "COLON",
    19 => "REGULAR_LITERAL",
    11 => "QUESTION",
    14 => "AT",
    3 => "TOKEN_REF",
    9 => "STAR",
    12 => "LPAREN",
    8 => "EPSILON",
    15 => "SHARP",
    10 => "PLUS",
    7 => "OR",
    6 => "COMMA",
    13 => "RPAREN",
    18 => "STRING_LITERAL",
    5 => "SEMI",
    2 => "RULE_REF",
    20 => "WHITE_SPACE",
    16 => "LBRACKET",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (6, 5),
    (12, 3),
    (22, 6),
    (8, 5),
    (20, 7),
    (19, 13),
    (23, 17),
    (18, 12),
    (18, 7),
    (18, 5),
    (12, 14),
    (18, 3),
    (17, 10),
    (20, 18),
    (14, 7),
    (17, 18),
    (19, 2),
    (1, 3),
    (17, 5),
    (1, 1),
    (14, 13),
    (7, 3),
    (1, 15),
    (19, 3),
    (19, 18),
    (2, 13),
    (4, 13),
    (7, 1),
    (3, 7),
    (25, 3),
    (21, 3),
    (3, 5),
    (6, 13),
    (20, 13),
    (15, 13),
    (11, 17),
    (5, 3),
    (1, 2),
    (17, 9),
    (17, 11),
    (20, 3),
    (20, 5),
    (24, 17),
    (20, 12),
    (9, 3),
    (16, 5),
    (5, 2),
    (7, 2),
    (17, 12),
    (15, 5),
    (6, 12),
    (16, 13),
    (18, 18),
    (6, 2),
    (3, 13),
    (24, 3),
    (18, 13),
    (12, 2),
    (12, 15),
    (20, 2),
    (2, 5),
    (17, 3),
    (22, 17),
    (19, 12),
    (5, 18),
    (19, 11),
    (13, 1),
    (5, 12),
    (6, 18),
    (7, 15),
    (17, 2),
    (18, 2),
    (12, 1),
    (14, 5),
    (11, 3),
    (6, 7),
    (5, 5),
    (5, 7),
    (4, 5),
    (17, 7),
    (11, 6),
    (5, 13),
    (6, 3),
    (19, 5),
    (25, 17),
    (1, 14),
    (16, 7),
    (25, 6),
    (7, 14),
    (4, 7),
    (24, 6),
    (0, 1),
    (10, 17),
    (19, 7),
    (17, 13),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const PARSER_RULE: usize = 1; 
  pub const ELEMENT: usize = 5; 
  pub const ATTRIBUTE: usize = 11; 
  pub const ANNOTATION: usize = 9; 
  pub const EPSILON: usize = 4; 
  pub const REGULAR: usize = 8; 
  pub const LEXER_RULE: usize = 7; 
  pub const ALTERNATIVE: usize = 3; 
  pub const ATTRIBUTE_LIST: usize = 10; 
  pub const EBNF_SUFFIX: usize = 6; 
  pub const RULE_LIST: usize = 0; 
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
  
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE);
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
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
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
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

}






// impl Parser for SyntaxisParser {}

