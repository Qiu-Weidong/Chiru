
use lazy_static::lazy_static; 

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::runtime::{
  parser::LL1, token_stream::TokenStream, 
  error_strategy::error_listener::ConsoleErrorListener,
  production::Production,
  production::ProductionItem
};

use super::chiru_context::{
   LexerRuleContext, RulesContext, AttributesContext, EpsilonContext, AnnotationContext, CompilationUnitContext, ElementContext, BlockContext, GrammarNameContext, RegularContext, AttributeContext, ParserRuleContext, AlternativeContext, EbnfSuffixContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (21, 12) => 24,
    (23, 14) => 29,
    (16, 15) => 7,
    (4, 3) => 13,
    (15, 16) => 5,
    (15, 15) => 5,
    (24, 15) => 33,
    (27, 13) => 42,
    (4, 19) => 13,
    (19, 14) => 14,
    (22, 10) => 26,
    (16, 16) => 7,
    (23, 13) => 29,
    (8, 12) => 31,
    (10, 20) => 35,
    (21, 19) => 23,
    (2, 4) => 8,
    (20, 3) => 21,
    (20, 4) => 19,
    (4, 9) => 13,
    (6, 9) => 18,
    (1, 2) => 3,
    (15, 4) => 5,
    (22, 12) => 28,
    (4, 13) => 13,
    (25, 7) => 38,
    (28, 7) => 43,
    (2, 15) => 8,
    (19, 8) => 14,
    (8, 10) => 31,
    (11, 16) => 37,
    (21, 4) => 23,
    (5, 19) => 16,
    (17, 8) => 10,
    (19, 3) => 15,
    (26, 18) => 39,
    (4, 4) => 13,
    (5, 4) => 16,
    (9, 4) => 34,
    (16, 3) => 7,
    (2, 1) => 8,
    (28, 18) => 43,
    (7, 3) => 25,
    (7, 4) => 25,
    (22, 11) => 27,
    (2, 16) => 8,
    (5, 3) => 16,
    (24, 4) => 32,
    (28, 13) => 44,
    (21, 3) => 23,
    (23, 6) => 29,
    (19, 19) => 15,
    (26, 7) => 40,
    (23, 19) => 29,
    (18, 14) => 11,
    (21, 14) => 23,
    (21, 6) => 23,
    (21, 10) => 24,
    (19, 13) => 15,
    (3, 3) => 9,
    (8, 11) => 31,
    (9, 15) => 34,
    (18, 8) => 12,
    (19, 6) => 14,
    (16, 4) => 7,
    (21, 11) => 24,
    (5, 9) => 17,
    (11, 15) => 36,
    (23, 3) => 29,
    (21, 8) => 23,
    (9, 16) => 34,
    (21, 13) => 23,
    (20, 19) => 20,
    (19, 4) => 15,
    (12, 3) => 41,
    (20, 13) => 22,
    (13, 3) => 45,
    (5, 13) => 16,
    (2, 3) => 8,
    (18, 6) => 11,
    (23, 8) => 29,
    (23, 12) => 30,
    (14, 4) => 1,
    (14, 3) => 2,
    (16, 1) => 6,
    (15, 3) => 4,
    (28, 4) => 43,
    (7, 19) => 25,
    (24, 16) => 33,
    (23, 4) => 29,
    (0, 2) => 0,
    (7, 13) => 25,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    43 => Production::new(43, 28, &vec![]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    29 => Production::new(29, 23, &vec![]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    23 => Production::new(23, 21, &vec![]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    32 => Production::new(32, 24, &vec![]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    6 => Production::new(6, 16, &vec![]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    39 => Production::new(39, 26, &vec![]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    14 => Production::new(14, 19, &vec![]),
    11 => Production::new(11, 18, &vec![]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    9 => "lexer_rule",
    2 => "rules",
    12 => "attributes",
    6 => "epsilon",
    11 => "annotation",
    0 => "compilation_unit",
    7 => "element",
    4 => "block",
    1 => "grammar_name",
    10 => "regular",
    13 => "attribute",
    3 => "parser_rule",
    5 => "alternative",
    8 => "ebnf_suffix",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    0 => "_START",
    2 => "GRAMMAR",
    7 => "COMMA",
    18 => "RBRACKET",
    19 => "STRING_LITERAL",
    9 => "EPSILON",
    6 => "SEMI",
    1 => "_STOP",
    21 => "WHITE_SPACE",
    10 => "STAR",
    13 => "LPAREN",
    3 => "RULE_REF",
    4 => "TOKEN_REF",
    14 => "RPAREN",
    16 => "SHARP",
    22 => "LINE_COMMENT",
    15 => "AT",
    11 => "PLUS",
    8 => "OR",
    20 => "REGULAR_LITERAL",
    5 => "COLON",
    23 => "BLOCK_COMMENT",
    12 => "QUESTION",
    17 => "LBRACKET",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (10, 6),
    (1, 4),
    (3, 16),
    (15, 15),
    (7, 4),
    (20, 14),
    (20, 3),
    (18, 6),
    (9, 15),
    (9, 4),
    (19, 6),
    (24, 4),
    (17, 6),
    (21, 19),
    (20, 11),
    (22, 13),
    (3, 15),
    (22, 19),
    (19, 8),
    (18, 14),
    (6, 6),
    (15, 1),
    (8, 13),
    (7, 3),
    (7, 19),
    (7, 6),
    (26, 18),
    (23, 6),
    (23, 13),
    (23, 19),
    (1, 15),
    (2, 1),
    (22, 4),
    (3, 4),
    (21, 13),
    (20, 8),
    (15, 3),
    (13, 4),
    (7, 13),
    (23, 3),
    (28, 18),
    (3, 1),
    (22, 12),
    (15, 4),
    (5, 8),
    (8, 14),
    (12, 18),
    (5, 14),
    (20, 10),
    (21, 3),
    (20, 12),
    (17, 14),
    (0, 1),
    (27, 7),
    (21, 6),
    (20, 13),
    (4, 6),
    (22, 8),
    (25, 7),
    (25, 18),
    (13, 7),
    (8, 8),
    (1, 1),
    (21, 4),
    (8, 6),
    (11, 4),
    (4, 14),
    (7, 8),
    (8, 19),
    (5, 6),
    (23, 8),
    (28, 4),
    (20, 6),
    (27, 18),
    (21, 8),
    (16, 1),
    (20, 19),
    (23, 14),
    (22, 3),
    (1, 16),
    (6, 8),
    (9, 16),
    (19, 14),
    (6, 14),
    (27, 4),
    (17, 8),
    (13, 18),
    (8, 4),
    (22, 14),
    (21, 14),
    (22, 6),
    (7, 14),
    (23, 4),
    (28, 7),
    (8, 3),
    (9, 1),
    (14, 6),
    (15, 16),
    (9, 3),
    (1, 3),
    (3, 3),
    (20, 4),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const LEXER_RULE: usize = 9; 
  pub const RULES: usize = 2; 
  pub const ATTRIBUTES: usize = 12; 
  pub const EPSILON: usize = 6; 
  pub const ANNOTATION: usize = 11; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const ELEMENT: usize = 7; 
  pub const BLOCK: usize = 4; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const REGULAR: usize = 10; 
  pub const ATTRIBUTE: usize = 13; 
  pub const PARSER_RULE: usize = 3; 
  pub const ALTERNATIVE: usize = 5; 
  pub const EBNF_SUFFIX: usize = 8; 



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
  
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Box<dyn RulesContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULES);
    Box::new(result)
  } 
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTES);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Box<dyn CompilationUnitContext> {
    let result = self.analyzer.analyse(token_stream, Self::COMPILATION_UNIT);
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
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
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
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 

}






