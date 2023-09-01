

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::chiru_context::{
   EpsilonContext, RegularContext, AlternativeContext, BlockContext, ParserRuleContext, LexerRuleContext, AttributeContext, AnnotationContext, RuleListContext, AttributeListContext, ElementContext, EbnfSuffixContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (5, 18) => 21,
    (4, 8) => 14,
    (18, 9) => 20,
    (18, 18) => 19,
    (20, 3) => 25,
    (20, 12) => 25,
    (0, 2) => 4,
    (20, 5) => 25,
    (25, 3) => 39,
    (16, 7) => 10,
    (16, 3) => 11,
    (22, 6) => 34,
    (16, 12) => 11,
    (18, 10) => 20,
    (5, 12) => 21,
    (12, 14) => 1,
    (17, 18) => 16,
    (18, 2) => 19,
    (2, 3) => 9,
    (19, 11) => 24,
    (16, 18) => 11,
    (21, 15) => 29,
    (3, 8) => 13,
    (2, 12) => 9,
    (2, 2) => 9,
    (20, 13) => 25,
    (16, 13) => 10,
    (3, 12) => 12,
    (3, 2) => 12,
    (8, 19) => 31,
    (0, 14) => 4,
    (17, 2) => 17,
    (13, 3) => 3,
    (13, 14) => 3,
    (3, 18) => 12,
    (23, 17) => 35,
    (0, 15) => 4,
    (19, 9) => 22,
    (13, 2) => 3,
    (5, 3) => 21,
    (24, 12) => 38,
    (18, 5) => 19,
    (17, 3) => 15,
    (2, 8) => 9,
    (5, 2) => 21,
    (25, 6) => 39,
    (18, 3) => 19,
    (15, 13) => 7,
    (16, 2) => 11,
    (18, 7) => 19,
    (7, 14) => 30,
    (6, 11) => 27,
    (7, 3) => 30,
    (23, 6) => 36,
    (12, 2) => 0,
    (14, 7) => 6,
    (18, 11) => 20,
    (11, 2) => 41,
    (17, 12) => 18,
    (15, 5) => 7,
    (20, 7) => 25,
    (13, 15) => 3,
    (21, 14) => 29,
    (20, 2) => 25,
    (25, 17) => 39,
    (1, 2) => 5,
    (6, 10) => 27,
    (9, 14) => 32,
    (2, 18) => 9,
    (16, 5) => 10,
    (10, 2) => 37,
    (13, 1) => 2,
    (18, 13) => 19,
    (0, 3) => 4,
    (15, 7) => 8,
    (20, 11) => 26,
    (3, 3) => 12,
    (6, 9) => 27,
    (12, 3) => 1,
    (25, 12) => 40,
    (20, 18) => 25,
    (19, 10) => 23,
    (21, 3) => 28,
    (7, 15) => 30,
    (18, 12) => 19,
    (9, 15) => 33,
    (12, 15) => 1,
    (0, 1) => 4,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(8),]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    16 => Production::new(16, 17, &vec![ProductionItem::Terminal(18),]),
    18 => Production::new(18, 17, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(2),ProductionItem::Terminal(13),]),
    19 => Production::new(19, 18, &vec![]),
    24 => Production::new(24, 19, &vec![ProductionItem::Terminal(11),]),
    25 => Production::new(25, 20, &vec![]),
    3 => Production::new(3, 13, &vec![ProductionItem::NonTerminal(12),ProductionItem::NonTerminal(13),]),
    38 => Production::new(38, 24, &vec![ProductionItem::Terminal(12),ProductionItem::Terminal(3),ProductionItem::Terminal(13),]),
    40 => Production::new(40, 25, &vec![ProductionItem::NonTerminal(24),]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    30 => Production::new(30, 7, &vec![ProductionItem::NonTerminal(21),ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    1 => Production::new(1, 12, &vec![ProductionItem::NonTerminal(7),]),
    7 => Production::new(7, 15, &vec![]),
    8 => Production::new(8, 15, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    26 => Production::new(26, 20, &vec![ProductionItem::Terminal(11),]),
    39 => Production::new(39, 25, &vec![]),
    11 => Production::new(11, 16, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    10 => Production::new(10, 16, &vec![]),
    37 => Production::new(37, 10, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(23),]),
    20 => Production::new(20, 18, &vec![ProductionItem::NonTerminal(6),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(19),ProductionItem::NonTerminal(20),]),
    2 => Production::new(2, 13, &vec![]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(15),]),
    34 => Production::new(34, 22, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(11),]),
    17 => Production::new(17, 17, &vec![ProductionItem::Terminal(2),]),
    41 => Production::new(41, 11, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(25),]),
    6 => Production::new(6, 14, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(3),]),
    36 => Production::new(36, 23, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    0 => Production::new(0, 12, &vec![ProductionItem::NonTerminal(1),]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(13),]),
    22 => Production::new(22, 19, &vec![ProductionItem::Terminal(9),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    23 => Production::new(23, 19, &vec![ProductionItem::Terminal(10),]),
    28 => Production::new(28, 21, &vec![]),
    15 => Production::new(15, 17, &vec![ProductionItem::Terminal(3),]),
    32 => Production::new(32, 9, &vec![ProductionItem::Terminal(14),ProductionItem::NonTerminal(11),]),
    33 => Production::new(33, 9, &vec![ProductionItem::Terminal(15),ProductionItem::Terminal(16),ProductionItem::NonTerminal(10),ProductionItem::Terminal(17),]),
    31 => Production::new(31, 8, &vec![ProductionItem::Terminal(19),]),
    29 => Production::new(29, 21, &vec![ProductionItem::NonTerminal(9),]),
    35 => Production::new(35, 23, &vec![]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    4 => "epsilon",
    8 => "regular",
    3 => "alternative",
    2 => "block",
    1 => "parser_rule",
    7 => "lexer_rule",
    11 => "attribute",
    9 => "annotation",
    0 => "rule_list",
    10 => "attribute_list",
    5 => "element",
    6 => "ebnf_suffix",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    1 => "_STOP",
    16 => "LBRACKET",
    14 => "AT",
    15 => "SHARP",
    6 => "COMMA",
    2 => "RULE_REF",
    20 => "WHITE_SPACE",
    17 => "RBRACKET",
    7 => "OR",
    18 => "STRING_LITERAL",
    5 => "SEMI",
    9 => "STAR",
    11 => "QUESTION",
    0 => "_START",
    4 => "COLON",
    10 => "PLUS",
    8 => "EPSILON",
    3 => "TOKEN_REF",
    13 => "RPAREN",
    19 => "REGULAR_LITERAL",
    12 => "LPAREN",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (6, 2),
    (6, 7),
    (7, 2),
    (10, 17),
    (6, 5),
    (18, 12),
    (17, 11),
    (18, 2),
    (17, 5),
    (19, 13),
    (6, 3),
    (7, 14),
    (7, 3),
    (7, 15),
    (14, 5),
    (24, 3),
    (13, 1),
    (12, 1),
    (12, 3),
    (11, 6),
    (22, 6),
    (3, 5),
    (5, 7),
    (1, 1),
    (18, 3),
    (12, 15),
    (17, 3),
    (18, 7),
    (20, 18),
    (1, 14),
    (12, 2),
    (24, 6),
    (11, 3),
    (20, 5),
    (2, 13),
    (25, 6),
    (2, 5),
    (0, 1),
    (16, 7),
    (5, 12),
    (18, 13),
    (6, 13),
    (20, 12),
    (4, 5),
    (17, 18),
    (5, 18),
    (19, 12),
    (8, 5),
    (16, 13),
    (11, 17),
    (17, 12),
    (21, 3),
    (9, 3),
    (24, 17),
    (17, 10),
    (5, 13),
    (14, 13),
    (16, 5),
    (1, 15),
    (19, 2),
    (5, 2),
    (6, 12),
    (1, 2),
    (5, 5),
    (22, 17),
    (3, 13),
    (20, 13),
    (3, 7),
    (20, 3),
    (5, 3),
    (12, 14),
    (20, 2),
    (19, 11),
    (19, 7),
    (7, 1),
    (4, 13),
    (14, 7),
    (25, 17),
    (17, 13),
    (18, 5),
    (15, 5),
    (6, 18),
    (15, 13),
    (4, 7),
    (17, 7),
    (17, 2),
    (25, 3),
    (23, 17),
    (20, 7),
    (19, 3),
    (19, 18),
    (19, 5),
    (17, 9),
    (18, 18),
    (1, 3),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const EPSILON: usize = 4; 
  pub const REGULAR: usize = 8; 
  pub const ALTERNATIVE: usize = 3; 
  pub const BLOCK: usize = 2; 
  pub const PARSER_RULE: usize = 1; 
  pub const LEXER_RULE: usize = 7; 
  pub const ATTRIBUTE: usize = 11; 
  pub const ANNOTATION: usize = 9; 
  pub const RULE_LIST: usize = 0; 
  pub const ATTRIBUTE_LIST: usize = 10; 
  pub const ELEMENT: usize = 5; 
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
  
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
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
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
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
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULE_LIST);
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
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

