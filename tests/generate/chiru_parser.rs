
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
   ElementContext, ParserRuleContext, AlternativeContext, RegularContext, GrammarNameContext, AttributesContext, EbnfSuffixContext, LexerRuleContext, AttributeContext, BlockContext, EpsilonContext, RulesContext, CompilationUnitContext, AnnotationContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (8, 12) => 31,
    (2, 15) => 8,
    (7, 19) => 25,
    (19, 8) => 14,
    (5, 3) => 16,
    (21, 8) => 23,
    (25, 7) => 38,
    (11, 16) => 37,
    (5, 9) => 17,
    (12, 3) => 41,
    (2, 4) => 8,
    (4, 9) => 13,
    (23, 14) => 29,
    (15, 15) => 5,
    (9, 16) => 34,
    (21, 10) => 24,
    (2, 1) => 8,
    (20, 3) => 21,
    (16, 1) => 6,
    (20, 19) => 20,
    (19, 13) => 15,
    (18, 6) => 11,
    (3, 3) => 9,
    (22, 11) => 27,
    (23, 8) => 29,
    (7, 4) => 25,
    (27, 13) => 42,
    (18, 8) => 12,
    (19, 4) => 15,
    (16, 4) => 7,
    (14, 4) => 1,
    (22, 12) => 28,
    (19, 6) => 14,
    (20, 13) => 22,
    (7, 3) => 25,
    (21, 13) => 23,
    (13, 3) => 45,
    (11, 15) => 36,
    (15, 3) => 4,
    (19, 19) => 15,
    (23, 4) => 29,
    (10, 20) => 35,
    (14, 3) => 2,
    (5, 4) => 16,
    (21, 4) => 23,
    (28, 18) => 43,
    (4, 4) => 13,
    (26, 18) => 39,
    (4, 19) => 13,
    (28, 13) => 44,
    (23, 19) => 29,
    (24, 15) => 33,
    (22, 10) => 26,
    (2, 3) => 8,
    (16, 16) => 7,
    (1, 2) => 3,
    (26, 7) => 40,
    (21, 19) => 23,
    (16, 3) => 7,
    (16, 15) => 7,
    (6, 9) => 18,
    (24, 4) => 32,
    (24, 16) => 33,
    (19, 14) => 14,
    (7, 13) => 25,
    (9, 15) => 34,
    (5, 13) => 16,
    (5, 19) => 16,
    (23, 12) => 30,
    (0, 2) => 0,
    (18, 14) => 11,
    (8, 10) => 31,
    (23, 13) => 29,
    (17, 8) => 10,
    (15, 4) => 5,
    (21, 6) => 23,
    (21, 14) => 23,
    (20, 4) => 19,
    (21, 12) => 24,
    (2, 16) => 8,
    (19, 3) => 15,
    (21, 11) => 24,
    (23, 3) => 29,
    (4, 3) => 13,
    (8, 11) => 31,
    (9, 4) => 34,
    (4, 13) => 13,
    (21, 3) => 23,
    (15, 16) => 5,
    (28, 4) => 43,
    (23, 6) => 29,
    (28, 7) => 43,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    23 => Production::new(23, 21, &vec![]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    14 => Production::new(14, 19, &vec![]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    32 => Production::new(32, 24, &vec![]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    39 => Production::new(39, 26, &vec![]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    11 => Production::new(11, 18, &vec![]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    43 => Production::new(43, 28, &vec![]),
    6 => Production::new(6, 16, &vec![]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    29 => Production::new(29, 23, &vec![]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    7 => "element",
    3 => "parser_rule",
    5 => "alternative",
    10 => "regular",
    1 => "grammar_name",
    12 => "attributes",
    8 => "ebnf_suffix",
    9 => "lexer_rule",
    13 => "attribute",
    4 => "block",
    6 => "epsilon",
    2 => "rules",
    0 => "compilation_unit",
    11 => "annotation",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    10 => "STAR",
    12 => "QUESTION",
    8 => "OR",
    17 => "LBRACKET",
    6 => "SEMI",
    0 => "_START",
    3 => "RULE_REF",
    13 => "LPAREN",
    18 => "RBRACKET",
    4 => "TOKEN_REF",
    20 => "REGULAR_LITERAL",
    14 => "RPAREN",
    1 => "_STOP",
    9 => "EPSILON",
    21 => "WHITE_SPACE",
    15 => "AT",
    7 => "COMMA",
    5 => "COLON",
    2 => "GRAMMAR",
    23 => "BLOCK_COMMENT",
    19 => "STRING_LITERAL",
    22 => "LINE_COMMENT",
    11 => "PLUS",
    16 => "SHARP",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (17, 14),
    (15, 1),
    (8, 8),
    (1, 15),
    (23, 6),
    (20, 8),
    (9, 15),
    (8, 6),
    (28, 18),
    (5, 8),
    (20, 11),
    (1, 4),
    (21, 19),
    (8, 19),
    (23, 13),
    (3, 4),
    (24, 4),
    (19, 14),
    (14, 6),
    (7, 13),
    (10, 6),
    (8, 13),
    (9, 1),
    (22, 14),
    (22, 19),
    (18, 14),
    (3, 3),
    (1, 1),
    (19, 8),
    (16, 1),
    (23, 4),
    (21, 6),
    (27, 18),
    (7, 19),
    (1, 3),
    (27, 7),
    (1, 16),
    (3, 15),
    (11, 4),
    (5, 6),
    (22, 12),
    (5, 14),
    (22, 6),
    (3, 1),
    (21, 13),
    (7, 14),
    (22, 13),
    (9, 4),
    (21, 14),
    (13, 4),
    (21, 3),
    (20, 19),
    (20, 3),
    (8, 3),
    (4, 14),
    (22, 8),
    (18, 6),
    (6, 8),
    (23, 19),
    (4, 6),
    (25, 18),
    (21, 4),
    (23, 8),
    (20, 10),
    (25, 7),
    (15, 4),
    (13, 18),
    (21, 8),
    (26, 18),
    (0, 1),
    (15, 15),
    (28, 4),
    (20, 6),
    (19, 6),
    (6, 14),
    (8, 14),
    (8, 4),
    (12, 18),
    (9, 3),
    (7, 6),
    (20, 14),
    (3, 16),
    (20, 12),
    (23, 3),
    (6, 6),
    (7, 3),
    (28, 7),
    (7, 8),
    (22, 3),
    (22, 4),
    (17, 6),
    (15, 3),
    (20, 4),
    (15, 16),
    (20, 13),
    (27, 4),
    (2, 1),
    (7, 4),
    (13, 7),
    (17, 8),
    (23, 14),
    (9, 16),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ELEMENT: usize = 7; 
  pub const PARSER_RULE: usize = 3; 
  pub const ALTERNATIVE: usize = 5; 
  pub const REGULAR: usize = 10; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const ATTRIBUTES: usize = 12; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const LEXER_RULE: usize = 9; 
  pub const ATTRIBUTE: usize = 13; 
  pub const BLOCK: usize = 4; 
  pub const EPSILON: usize = 6; 
  pub const RULES: usize = 2; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const ANNOTATION: usize = 11; 



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
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
    Box::new(result)
  } 
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTES);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
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
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
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
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 

}






