

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::chiru_context::{
   RegularContext, ElementContext, AnnotationContext, AttributeContext, LexerRuleContext, RuleListContext, BlockContext, AttributeListContext, AlternativeContext, EpsilonContext, EbnfSuffixContext, ParserRuleContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (21, 3) => 28,
    (0, 14) => 4,
    (9, 14) => 32,
    (19, 11) => 24,
    (18, 13) => 19,
    (2, 18) => 9,
    (2, 3) => 9,
    (24, 12) => 38,
    (22, 6) => 34,
    (18, 12) => 19,
    (16, 12) => 11,
    (18, 11) => 20,
    (13, 15) => 3,
    (12, 15) => 1,
    (12, 2) => 0,
    (19, 9) => 22,
    (20, 2) => 25,
    (6, 10) => 27,
    (10, 2) => 37,
    (9, 15) => 33,
    (4, 8) => 14,
    (7, 15) => 30,
    (3, 18) => 12,
    (20, 13) => 25,
    (15, 7) => 8,
    (17, 12) => 18,
    (3, 2) => 12,
    (5, 18) => 21,
    (20, 18) => 25,
    (20, 3) => 25,
    (25, 17) => 39,
    (20, 7) => 25,
    (17, 2) => 17,
    (15, 5) => 7,
    (21, 15) => 29,
    (18, 7) => 19,
    (13, 2) => 3,
    (18, 18) => 19,
    (16, 18) => 11,
    (16, 2) => 11,
    (5, 3) => 21,
    (6, 9) => 27,
    (25, 3) => 39,
    (20, 11) => 26,
    (16, 3) => 11,
    (25, 6) => 39,
    (11, 2) => 41,
    (17, 3) => 15,
    (17, 18) => 16,
    (18, 3) => 19,
    (2, 8) => 9,
    (18, 9) => 20,
    (25, 12) => 40,
    (18, 5) => 19,
    (13, 3) => 3,
    (8, 19) => 31,
    (5, 12) => 21,
    (16, 13) => 10,
    (5, 2) => 21,
    (18, 2) => 19,
    (16, 7) => 10,
    (1, 2) => 5,
    (3, 12) => 12,
    (0, 1) => 4,
    (3, 3) => 12,
    (23, 17) => 35,
    (21, 14) => 29,
    (2, 12) => 9,
    (13, 14) => 3,
    (0, 15) => 4,
    (18, 10) => 20,
    (6, 11) => 27,
    (7, 3) => 30,
    (7, 14) => 30,
    (14, 7) => 6,
    (20, 12) => 25,
    (12, 3) => 1,
    (19, 10) => 23,
    (2, 2) => 9,
    (3, 8) => 13,
    (23, 6) => 36,
    (12, 14) => 1,
    (0, 3) => 4,
    (13, 1) => 2,
    (16, 5) => 10,
    (20, 5) => 25,
    (0, 2) => 4,
    (15, 13) => 7,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    1 => Production::new(1, 12, &vec![ProductionItem::NonTerminal(7),]),
    26 => Production::new(26, 20, &vec![ProductionItem::Terminal(11),]),
    24 => Production::new(24, 19, &vec![ProductionItem::Terminal(11),]),
    10 => Production::new(10, 16, &vec![]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    36 => Production::new(36, 23, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    22 => Production::new(22, 19, &vec![ProductionItem::Terminal(9),]),
    25 => Production::new(25, 20, &vec![]),
    31 => Production::new(31, 8, &vec![ProductionItem::Terminal(19),]),
    40 => Production::new(40, 25, &vec![ProductionItem::NonTerminal(24),]),
    39 => Production::new(39, 25, &vec![]),
    23 => Production::new(23, 19, &vec![ProductionItem::Terminal(10),]),
    17 => Production::new(17, 17, &vec![ProductionItem::Terminal(2),]),
    28 => Production::new(28, 21, &vec![]),
    19 => Production::new(19, 18, &vec![]),
    8 => Production::new(8, 15, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(15),]),
    0 => Production::new(0, 12, &vec![ProductionItem::NonTerminal(1),]),
    38 => Production::new(38, 24, &vec![ProductionItem::Terminal(12),ProductionItem::Terminal(3),ProductionItem::Terminal(13),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    11 => Production::new(11, 16, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    3 => Production::new(3, 13, &vec![ProductionItem::NonTerminal(12),ProductionItem::NonTerminal(13),]),
    18 => Production::new(18, 17, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(2),ProductionItem::Terminal(13),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(19),ProductionItem::NonTerminal(20),]),
    33 => Production::new(33, 9, &vec![ProductionItem::Terminal(15),ProductionItem::Terminal(16),ProductionItem::NonTerminal(10),ProductionItem::Terminal(17),]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(13),]),
    35 => Production::new(35, 23, &vec![]),
    15 => Production::new(15, 17, &vec![ProductionItem::Terminal(3),]),
    7 => Production::new(7, 15, &vec![]),
    2 => Production::new(2, 13, &vec![]),
    34 => Production::new(34, 22, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(11),]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    20 => Production::new(20, 18, &vec![ProductionItem::NonTerminal(6),]),
    41 => Production::new(41, 11, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(25),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(8),]),
    29 => Production::new(29, 21, &vec![ProductionItem::NonTerminal(9),]),
    30 => Production::new(30, 7, &vec![ProductionItem::NonTerminal(21),ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    37 => Production::new(37, 10, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(23),]),
    16 => Production::new(16, 17, &vec![ProductionItem::Terminal(18),]),
    6 => Production::new(6, 14, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(3),]),
    32 => Production::new(32, 9, &vec![ProductionItem::Terminal(14),ProductionItem::NonTerminal(11),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    8 => "regular",
    5 => "element",
    9 => "annotation",
    11 => "attribute",
    7 => "lexer_rule",
    0 => "rule_list",
    2 => "block",
    10 => "attribute_list",
    3 => "alternative",
    4 => "epsilon",
    6 => "ebnf_suffix",
    1 => "parser_rule",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    3 => "TOKEN_REF",
    7 => "OR",
    2 => "RULE_REF",
    15 => "SHARP",
    5 => "SEMI",
    16 => "LBRACKET",
    17 => "RBRACKET",
    20 => "WHITE_SPACE",
    11 => "QUESTION",
    1 => "_STOP",
    10 => "PLUS",
    12 => "LPAREN",
    13 => "RPAREN",
    18 => "STRING_LITERAL",
    8 => "EPSILON",
    4 => "COLON",
    9 => "STAR",
    19 => "REGULAR_LITERAL",
    6 => "COMMA",
    21 => "LINE_COMMENT",
    22 => "BLOCK_COMMENT",
    0 => "_START",
    14 => "AT",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (19, 3),
    (25, 6),
    (4, 5),
    (9, 3),
    (17, 9),
    (16, 13),
    (19, 7),
    (13, 1),
    (17, 13),
    (12, 3),
    (10, 17),
    (20, 18),
    (6, 5),
    (19, 18),
    (7, 3),
    (5, 18),
    (17, 11),
    (17, 10),
    (24, 3),
    (17, 3),
    (20, 3),
    (19, 12),
    (21, 3),
    (3, 7),
    (15, 13),
    (6, 18),
    (6, 13),
    (20, 7),
    (16, 5),
    (1, 3),
    (19, 13),
    (17, 7),
    (0, 1),
    (5, 12),
    (1, 2),
    (6, 2),
    (19, 5),
    (5, 13),
    (7, 1),
    (6, 12),
    (15, 5),
    (1, 14),
    (16, 7),
    (8, 5),
    (25, 17),
    (12, 1),
    (20, 5),
    (2, 13),
    (6, 7),
    (20, 2),
    (5, 5),
    (5, 3),
    (1, 15),
    (12, 15),
    (17, 18),
    (5, 2),
    (18, 7),
    (14, 13),
    (11, 17),
    (3, 5),
    (5, 7),
    (14, 5),
    (24, 6),
    (18, 12),
    (18, 13),
    (17, 12),
    (20, 13),
    (12, 14),
    (22, 6),
    (17, 5),
    (12, 2),
    (3, 13),
    (6, 3),
    (2, 5),
    (20, 12),
    (24, 17),
    (7, 2),
    (11, 3),
    (18, 18),
    (18, 3),
    (23, 17),
    (4, 7),
    (22, 17),
    (11, 6),
    (19, 2),
    (18, 2),
    (19, 11),
    (17, 2),
    (1, 1),
    (14, 7),
    (4, 13),
    (25, 3),
    (7, 15),
    (7, 14),
    (18, 5),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const REGULAR: usize = 8; 
  pub const ELEMENT: usize = 5; 
  pub const ANNOTATION: usize = 9; 
  pub const ATTRIBUTE: usize = 11; 
  pub const LEXER_RULE: usize = 7; 
  pub const RULE_LIST: usize = 0; 
  pub const BLOCK: usize = 2; 
  pub const ATTRIBUTE_LIST: usize = 10; 
  pub const ALTERNATIVE: usize = 3; 
  pub const EPSILON: usize = 4; 
  pub const EBNF_SUFFIX: usize = 6; 
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
  
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE);
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
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn attribute_list(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeListContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE_LIST);
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
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

