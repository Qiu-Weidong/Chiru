

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use crate::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::chiru_context::{
   AttributeContext, AnnotationContext, RegularContext, AlternativeContext, GrammarNameContext, AttributeListContext, LexerRuleContext, EbnfSuffixContext, RulesContext, CompilationUnitContext, ParserRuleContext, ElementContext, EpsilonContext, BlockContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (19, 14) => 14,
    (19, 6) => 14,
    (5, 3) => 16,
    (7, 19) => 25,
    (2, 4) => 8,
    (21, 6) => 23,
    (26, 7) => 40,
    (27, 13) => 42,
    (18, 8) => 12,
    (6, 9) => 18,
    (7, 4) => 25,
    (12, 3) => 41,
    (5, 4) => 16,
    (20, 19) => 20,
    (15, 4) => 5,
    (24, 15) => 33,
    (21, 8) => 23,
    (23, 13) => 29,
    (21, 4) => 23,
    (7, 13) => 25,
    (5, 19) => 16,
    (14, 3) => 2,
    (8, 11) => 31,
    (21, 13) => 23,
    (8, 10) => 31,
    (15, 15) => 5,
    (15, 16) => 5,
    (17, 8) => 10,
    (23, 14) => 29,
    (18, 6) => 11,
    (19, 19) => 15,
    (19, 4) => 15,
    (9, 15) => 34,
    (21, 10) => 24,
    (21, 19) => 23,
    (24, 16) => 33,
    (2, 1) => 8,
    (4, 4) => 13,
    (15, 3) => 4,
    (23, 3) => 29,
    (28, 7) => 43,
    (21, 3) => 23,
    (20, 4) => 19,
    (22, 11) => 27,
    (11, 16) => 37,
    (3, 3) => 9,
    (20, 3) => 21,
    (8, 12) => 31,
    (23, 4) => 29,
    (2, 16) => 8,
    (4, 13) => 13,
    (2, 3) => 8,
    (16, 1) => 6,
    (16, 15) => 7,
    (9, 4) => 34,
    (4, 9) => 13,
    (0, 2) => 0,
    (16, 3) => 7,
    (5, 13) => 16,
    (22, 12) => 28,
    (28, 4) => 43,
    (28, 13) => 44,
    (7, 3) => 25,
    (22, 10) => 26,
    (23, 12) => 30,
    (13, 3) => 45,
    (11, 15) => 36,
    (10, 20) => 35,
    (23, 6) => 29,
    (19, 3) => 15,
    (25, 7) => 38,
    (23, 8) => 29,
    (18, 14) => 11,
    (5, 9) => 17,
    (14, 4) => 1,
    (16, 16) => 7,
    (20, 13) => 22,
    (21, 14) => 23,
    (9, 16) => 34,
    (2, 15) => 8,
    (21, 12) => 24,
    (1, 2) => 3,
    (4, 19) => 13,
    (24, 4) => 32,
    (19, 8) => 14,
    (26, 18) => 39,
    (16, 4) => 7,
    (28, 18) => 43,
    (19, 13) => 15,
    (4, 3) => 13,
    (21, 11) => 24,
    (23, 19) => 29,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    43 => Production::new(43, 28, &vec![]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    29 => Production::new(29, 23, &vec![]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    32 => Production::new(32, 24, &vec![]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    11 => Production::new(11, 18, &vec![]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    39 => Production::new(39, 26, &vec![]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    14 => Production::new(14, 19, &vec![]),
    6 => Production::new(6, 16, &vec![]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    23 => Production::new(23, 21, &vec![]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    13 => "attribute",
    11 => "annotation",
    10 => "regular",
    5 => "alternative",
    1 => "grammar_name",
    12 => "attribute_list",
    9 => "lexer_rule",
    8 => "ebnf_suffix",
    2 => "rules",
    0 => "compilation_unit",
    3 => "parser_rule",
    7 => "element",
    6 => "epsilon",
    4 => "block",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    21 => "WHITE_SPACE",
    18 => "RBRACKET",
    4 => "TOKEN_REF",
    9 => "EPSILON",
    10 => "STAR",
    2 => "GRAMMAR",
    22 => "LINE_COMMENT",
    3 => "RULE_REF",
    11 => "PLUS",
    19 => "STRING_LITERAL",
    16 => "SHARP",
    7 => "COMMA",
    5 => "COLON",
    6 => "SEMI",
    20 => "REGULAR_LITERAL",
    0 => "_START",
    14 => "RPAREN",
    12 => "QUESTION",
    8 => "OR",
    15 => "AT",
    23 => "BLOCK_COMMENT",
    17 => "LBRACKET",
    1 => "_STOP",
    13 => "LPAREN",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (5, 6),
    (22, 6),
    (23, 6),
    (23, 14),
    (6, 14),
    (23, 4),
    (1, 1),
    (8, 3),
    (17, 8),
    (27, 18),
    (3, 3),
    (22, 4),
    (20, 19),
    (2, 1),
    (20, 4),
    (4, 14),
    (7, 4),
    (9, 15),
    (16, 1),
    (23, 19),
    (21, 8),
    (20, 6),
    (21, 3),
    (1, 3),
    (27, 7),
    (8, 4),
    (22, 19),
    (7, 14),
    (12, 18),
    (23, 8),
    (23, 13),
    (20, 10),
    (4, 6),
    (17, 14),
    (3, 16),
    (9, 4),
    (7, 13),
    (28, 18),
    (25, 7),
    (20, 11),
    (18, 6),
    (28, 4),
    (6, 8),
    (9, 16),
    (13, 18),
    (21, 6),
    (19, 14),
    (6, 6),
    (27, 4),
    (24, 4),
    (22, 13),
    (19, 8),
    (7, 3),
    (20, 14),
    (20, 8),
    (13, 7),
    (21, 14),
    (9, 1),
    (20, 12),
    (21, 13),
    (9, 3),
    (1, 4),
    (18, 14),
    (7, 6),
    (3, 1),
    (5, 14),
    (13, 4),
    (1, 15),
    (14, 6),
    (0, 1),
    (8, 13),
    (8, 8),
    (8, 14),
    (15, 16),
    (17, 6),
    (7, 19),
    (15, 1),
    (20, 13),
    (10, 6),
    (8, 19),
    (22, 14),
    (15, 4),
    (20, 3),
    (11, 4),
    (21, 4),
    (28, 7),
    (1, 16),
    (5, 8),
    (25, 18),
    (8, 6),
    (3, 15),
    (22, 3),
    (26, 18),
    (15, 3),
    (23, 3),
    (21, 19),
    (3, 4),
    (7, 8),
    (19, 6),
    (15, 15),
    (22, 12),
    (22, 8),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ATTRIBUTE: usize = 13; 
  pub const ANNOTATION: usize = 11; 
  pub const REGULAR: usize = 10; 
  pub const ALTERNATIVE: usize = 5; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const ATTRIBUTE_LIST: usize = 12; 
  pub const LEXER_RULE: usize = 9; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const RULES: usize = 2; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const PARSER_RULE: usize = 3; 
  pub const ELEMENT: usize = 7; 
  pub const EPSILON: usize = 6; 
  pub const BLOCK: usize = 4; 



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
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
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
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
    Box::new(result)
  } 
  pub fn attribute_list(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeListContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE_LIST);
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
  pub fn rules(&self, token_stream: &mut TokenStream) -> Box<dyn RulesContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULES);
    Box::new(result)
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Box<dyn CompilationUnitContext> {
    let result = self.analyzer.analyse(token_stream, Self::COMPILATION_UNIT);
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
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

