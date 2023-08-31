

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::context::{
   ElementContext, AttributeListContext, AnnotationContext, BlockContext, RegularContext, AlternativeContext, ParserRuleContext, LexerRuleContext, EpsilonContext, EbnfSuffixContext, RuleListContext, AttributeContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (20, 11) => 26,
    (23, 17) => 35,
    (13, 15) => 3,
    (11, 2) => 43,
    (21, 15) => 29,
    (5, 12) => 21,
    (16, 13) => 10,
    (10, 2) => 37,
    (17, 12) => 18,
    (26, 12) => 42,
    (20, 3) => 25,
    (14, 7) => 6,
    (16, 7) => 10,
    (15, 7) => 8,
    (12, 15) => 1,
    (5, 18) => 21,
    (2, 12) => 9,
    (22, 6) => 34,
    (24, 3) => 38,
    (9, 15) => 33,
    (1, 2) => 5,
    (21, 14) => 29,
    (23, 6) => 36,
    (16, 12) => 11,
    (20, 12) => 25,
    (0, 3) => 4,
    (2, 8) => 9,
    (12, 2) => 0,
    (18, 7) => 19,
    (15, 5) => 7,
    (19, 9) => 22,
    (6, 9) => 27,
    (15, 13) => 7,
    (24, 2) => 39,
    (0, 2) => 4,
    (25, 12) => 40,
    (0, 1) => 4,
    (2, 18) => 9,
    (3, 12) => 12,
    (13, 2) => 3,
    (18, 12) => 19,
    (4, 8) => 14,
    (11, 3) => 43,
    (10, 3) => 37,
    (20, 5) => 25,
    (0, 15) => 4,
    (26, 6) => 41,
    (18, 10) => 20,
    (3, 18) => 12,
    (18, 11) => 20,
    (18, 5) => 19,
    (16, 5) => 10,
    (26, 17) => 41,
    (7, 15) => 30,
    (18, 3) => 19,
    (17, 18) => 16,
    (2, 2) => 9,
    (16, 18) => 11,
    (12, 14) => 1,
    (3, 8) => 13,
    (13, 1) => 2,
    (19, 11) => 24,
    (18, 18) => 19,
    (6, 11) => 27,
    (16, 2) => 11,
    (2, 3) => 9,
    (20, 18) => 25,
    (17, 2) => 17,
    (3, 3) => 12,
    (6, 10) => 27,
    (13, 14) => 3,
    (13, 3) => 3,
    (5, 2) => 21,
    (21, 3) => 28,
    (20, 7) => 25,
    (20, 13) => 25,
    (18, 9) => 20,
    (18, 13) => 19,
    (0, 14) => 4,
    (18, 2) => 19,
    (8, 19) => 31,
    (7, 14) => 30,
    (9, 14) => 32,
    (19, 10) => 23,
    (12, 3) => 1,
    (7, 3) => 30,
    (3, 2) => 12,
    (26, 3) => 41,
    (20, 2) => 25,
    (16, 3) => 11,
    (17, 3) => 15,
    (5, 3) => 21,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    7 => Production::new(7, 15, &vec![]),
    22 => Production::new(22, 19, &vec![ProductionItem::Terminal(9),]),
    24 => Production::new(24, 19, &vec![ProductionItem::Terminal(11),]),
    15 => Production::new(15, 17, &vec![ProductionItem::Terminal(3),]),
    2 => Production::new(2, 13, &vec![]),
    9 => Production::new(9, 2, &vec![ProductionItem::NonTerminal(3),ProductionItem::NonTerminal(15),]),
    6 => Production::new(6, 14, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(3),]),
    0 => Production::new(0, 12, &vec![ProductionItem::NonTerminal(1),]),
    18 => Production::new(18, 17, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(2),ProductionItem::Terminal(13),]),
    28 => Production::new(28, 21, &vec![]),
    35 => Production::new(35, 23, &vec![]),
    39 => Production::new(39, 24, &vec![ProductionItem::Terminal(2),]),
    20 => Production::new(20, 18, &vec![ProductionItem::NonTerminal(6),]),
    31 => Production::new(31, 8, &vec![ProductionItem::Terminal(19),]),
    12 => Production::new(12, 3, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    19 => Production::new(19, 18, &vec![]),
    1 => Production::new(1, 12, &vec![ProductionItem::NonTerminal(7),]),
    29 => Production::new(29, 21, &vec![ProductionItem::NonTerminal(9),]),
    13 => Production::new(13, 3, &vec![ProductionItem::NonTerminal(4),]),
    21 => Production::new(21, 5, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    33 => Production::new(33, 9, &vec![ProductionItem::Terminal(15),ProductionItem::Terminal(16),ProductionItem::NonTerminal(10),ProductionItem::Terminal(17),]),
    34 => Production::new(34, 22, &vec![ProductionItem::Terminal(6),ProductionItem::NonTerminal(11),]),
    36 => Production::new(36, 23, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    38 => Production::new(38, 24, &vec![ProductionItem::Terminal(3),]),
    10 => Production::new(10, 16, &vec![]),
    26 => Production::new(26, 20, &vec![ProductionItem::Terminal(11),]),
    23 => Production::new(23, 19, &vec![ProductionItem::Terminal(10),]),
    30 => Production::new(30, 7, &vec![ProductionItem::NonTerminal(21),ProductionItem::Terminal(3),ProductionItem::Terminal(4),ProductionItem::NonTerminal(8),ProductionItem::Terminal(5),]),
    41 => Production::new(41, 26, &vec![]),
    16 => Production::new(16, 17, &vec![ProductionItem::Terminal(18),]),
    17 => Production::new(17, 17, &vec![ProductionItem::Terminal(2),]),
    27 => Production::new(27, 6, &vec![ProductionItem::NonTerminal(19),ProductionItem::NonTerminal(20),]),
    42 => Production::new(42, 26, &vec![ProductionItem::NonTerminal(25),]),
    32 => Production::new(32, 9, &vec![ProductionItem::Terminal(14),ProductionItem::NonTerminal(11),]),
    14 => Production::new(14, 4, &vec![ProductionItem::Terminal(8),]),
    11 => Production::new(11, 16, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(16),]),
    3 => Production::new(3, 13, &vec![ProductionItem::NonTerminal(12),ProductionItem::NonTerminal(13),]),
    43 => Production::new(43, 11, &vec![ProductionItem::NonTerminal(24),ProductionItem::NonTerminal(26),]),
    5 => Production::new(5, 1, &vec![ProductionItem::Terminal(2),ProductionItem::Terminal(4),ProductionItem::NonTerminal(2),ProductionItem::Terminal(5),]),
    8 => Production::new(8, 15, &vec![ProductionItem::NonTerminal(14),ProductionItem::NonTerminal(15),]),
    37 => Production::new(37, 10, &vec![ProductionItem::NonTerminal(11),ProductionItem::NonTerminal(23),]),
    40 => Production::new(40, 25, &vec![ProductionItem::Terminal(12),ProductionItem::NonTerminal(24),ProductionItem::Terminal(13),]),
    25 => Production::new(25, 20, &vec![]),
    4 => Production::new(4, 0, &vec![ProductionItem::NonTerminal(13),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    5 => "element",
    10 => "attribute_list",
    9 => "annotation",
    2 => "block",
    8 => "regular",
    3 => "alternative",
    1 => "parser_rule",
    7 => "lexer_rule",
    4 => "epsilon",
    6 => "ebnf_suffix",
    0 => "rule_list",
    11 => "attribute",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    9 => "STAR",
    19 => "REGULAR_LITERAL",
    6 => "COMMA",
    10 => "PLUS",
    5 => "SEMI",
    12 => "LPAREN",
    13 => "RPAREN",
    7 => "OR",
    1 => "_STOP",
    20 => "WHITE_SPACE",
    2 => "RULE_REF",
    4 => "COLON",
    11 => "QUESTION",
    0 => "_START",
    16 => "LBRACKET",
    17 => "RBRACKET",
    14 => "AT",
    18 => "STRING_LITERAL",
    8 => "EPSILON",
    15 => "SHARP",
    3 => "TOKEN_REF",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (5, 12),
    (3, 13),
    (19, 11),
    (6, 13),
    (17, 13),
    (25, 17),
    (1, 15),
    (22, 17),
    (17, 2),
    (17, 11),
    (20, 3),
    (12, 1),
    (1, 3),
    (9, 3),
    (6, 18),
    (6, 7),
    (19, 12),
    (24, 12),
    (5, 18),
    (5, 13),
    (22, 6),
    (12, 15),
    (6, 12),
    (8, 5),
    (4, 7),
    (20, 12),
    (24, 6),
    (5, 2),
    (17, 7),
    (4, 13),
    (7, 15),
    (1, 1),
    (0, 1),
    (18, 7),
    (16, 5),
    (7, 2),
    (20, 18),
    (24, 13),
    (18, 13),
    (17, 18),
    (18, 18),
    (18, 5),
    (19, 3),
    (17, 3),
    (25, 3),
    (20, 13),
    (16, 13),
    (6, 3),
    (14, 7),
    (15, 13),
    (25, 6),
    (26, 6),
    (20, 5),
    (19, 5),
    (16, 7),
    (11, 6),
    (1, 14),
    (14, 5),
    (18, 3),
    (5, 5),
    (26, 3),
    (15, 5),
    (5, 7),
    (20, 7),
    (12, 14),
    (19, 2),
    (17, 10),
    (4, 5),
    (6, 2),
    (3, 5),
    (19, 7),
    (17, 12),
    (7, 1),
    (10, 17),
    (12, 3),
    (6, 5),
    (12, 2),
    (20, 2),
    (2, 13),
    (7, 3),
    (19, 18),
    (1, 2),
    (17, 5),
    (19, 13),
    (7, 14),
    (24, 17),
    (23, 17),
    (18, 2),
    (11, 17),
    (3, 7),
    (24, 3),
    (5, 3),
    (2, 5),
    (21, 3),
    (13, 1),
    (14, 13),
    (26, 17),
    (18, 12),
    (17, 9),
    (11, 3),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ELEMENT: usize = 5; 
  pub const ATTRIBUTE_LIST: usize = 10; 
  pub const ANNOTATION: usize = 9; 
  pub const BLOCK: usize = 2; 
  pub const REGULAR: usize = 8; 
  pub const ALTERNATIVE: usize = 3; 
  pub const PARSER_RULE: usize = 1; 
  pub const LEXER_RULE: usize = 7; 
  pub const EPSILON: usize = 4; 
  pub const EBNF_SUFFIX: usize = 6; 
  pub const RULE_LIST: usize = 0; 
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
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
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
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
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
  pub fn rule_list(&self, token_stream: &mut TokenStream) -> Box<dyn RuleListContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULE_LIST);
    Box::new(result)
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

