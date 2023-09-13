
use chiru::runtime::error_strategy::error_listener::ErrorListener;
use chiru::runtime::ll1_analyzer::ll1_analyze;
use once_cell::sync::Lazy;

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::runtime::{
  token_stream::TokenStream, 
  error_strategy::error_listener::ConsoleErrorListener,
  production::Production,
  production::ProductionItem
};

use super::chiru_context::{
   ParserRuleContext, ElementContext, AttributeContext, AnnotationContext, EbnfSuffixContext, BlockContext, LexerRuleContext, GrammarNameContext, AttributesContext, EpsilonContext, AlternativeContext, RegularContext, CompilationUnitContext, RulesContext,
};


pub struct ChiruParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (4, 4) => 13,
    (21, 13) => 23,
    (9, 16) => 34,
    (18, 14) => 11,
    (21, 14) => 23,
    (22, 10) => 26,
    (5, 3) => 16,
    (21, 11) => 24,
    (23, 8) => 29,
    (21, 12) => 24,
    (23, 19) => 29,
    (21, 19) => 23,
    (11, 16) => 37,
    (24, 16) => 33,
    (20, 3) => 21,
    (26, 18) => 39,
    (5, 13) => 16,
    (16, 16) => 7,
    (11, 15) => 36,
    (15, 3) => 4,
    (28, 13) => 44,
    (19, 19) => 15,
    (28, 7) => 43,
    (21, 4) => 23,
    (23, 3) => 29,
    (12, 3) => 41,
    (7, 4) => 25,
    (2, 16) => 8,
    (2, 4) => 8,
    (21, 8) => 23,
    (4, 3) => 13,
    (3, 3) => 9,
    (20, 13) => 22,
    (8, 11) => 31,
    (19, 4) => 15,
    (15, 4) => 5,
    (23, 14) => 29,
    (23, 6) => 29,
    (9, 15) => 34,
    (21, 3) => 23,
    (26, 7) => 40,
    (19, 3) => 15,
    (18, 8) => 12,
    (8, 12) => 31,
    (15, 15) => 5,
    (27, 13) => 42,
    (7, 13) => 25,
    (19, 13) => 15,
    (22, 11) => 27,
    (4, 9) => 13,
    (5, 9) => 17,
    (13, 3) => 45,
    (20, 19) => 20,
    (28, 4) => 43,
    (4, 19) => 13,
    (1, 2) => 3,
    (22, 12) => 28,
    (19, 8) => 14,
    (18, 6) => 11,
    (16, 1) => 6,
    (15, 16) => 5,
    (28, 18) => 43,
    (24, 15) => 33,
    (21, 10) => 24,
    (23, 12) => 30,
    (7, 3) => 25,
    (23, 13) => 29,
    (21, 6) => 23,
    (20, 4) => 19,
    (10, 20) => 35,
    (7, 19) => 25,
    (14, 4) => 1,
    (16, 4) => 7,
    (4, 13) => 13,
    (2, 1) => 8,
    (9, 4) => 34,
    (19, 6) => 14,
    (24, 4) => 32,
    (5, 19) => 16,
    (2, 15) => 8,
    (5, 4) => 16,
    (2, 3) => 8,
    (16, 15) => 7,
    (8, 10) => 31,
    (6, 9) => 18,
    (0, 2) => 0,
    (19, 14) => 14,
    (17, 8) => 10,
    (25, 7) => 38,
    (23, 4) => 29,
    (14, 3) => 2,
    (16, 3) => 7,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    32 => Production::new(32, 24, &vec![]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    43 => Production::new(43, 28, &vec![]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    39 => Production::new(39, 26, &vec![]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    11 => Production::new(11, 18, &vec![]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    6 => Production::new(6, 16, &vec![]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    29 => Production::new(29, 23, &vec![]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    23 => Production::new(23, 21, &vec![]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    14 => Production::new(14, 19, &vec![]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    3 => String::from("parser_rule"),
    7 => String::from("element"),
    13 => String::from("attribute"),
    11 => String::from("annotation"),
    8 => String::from("ebnf_suffix"),
    4 => String::from("block"),
    9 => String::from("lexer_rule"),
    1 => String::from("grammar_name"),
    12 => String::from("attributes"),
    6 => String::from("epsilon"),
    5 => String::from("alternative"),
    10 => String::from("regular"),
    0 => String::from("compilation_unit"),
    2 => String::from("rules"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    20 => String::from("REGULAR_LITERAL"),
    13 => String::from("LPAREN"),
    8 => String::from("OR"),
    21 => String::from("WHITE_SPACE"),
    4 => String::from("TOKEN_REF"),
    2 => String::from("GRAMMAR"),
    3 => String::from("RULE_REF"),
    16 => String::from("SHARP"),
    17 => String::from("LBRACKET"),
    0 => String::from("_START"),
    5 => String::from("COLON"),
    18 => String::from("RBRACKET"),
    23 => String::from("BLOCK_COMMENT"),
    1 => String::from("_STOP"),
    14 => String::from("RPAREN"),
    15 => String::from("AT"),
    19 => String::from("STRING_LITERAL"),
    11 => String::from("PLUS"),
    12 => String::from("QUESTION"),
    10 => String::from("STAR"),
    9 => String::from("EPSILON"),
    22 => String::from("LINE_COMMENT"),
    7 => String::from("COMMA"),
    6 => String::from("SEMI"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (23, 4),
    (11, 4),
    (15, 4),
    (19, 6),
    (27, 7),
    (23, 19),
    (21, 4),
    (23, 6),
    (16, 1),
    (27, 18),
    (7, 8),
    (7, 14),
    (9, 1),
    (22, 12),
    (20, 10),
    (23, 3),
    (9, 3),
    (1, 16),
    (2, 1),
    (15, 15),
    (27, 4),
    (21, 14),
    (21, 3),
    (23, 13),
    (28, 4),
    (22, 14),
    (6, 14),
    (6, 8),
    (8, 3),
    (0, 1),
    (25, 7),
    (8, 14),
    (22, 8),
    (23, 8),
    (28, 7),
    (20, 6),
    (7, 13),
    (17, 14),
    (6, 6),
    (22, 6),
    (5, 8),
    (20, 12),
    (1, 15),
    (26, 18),
    (20, 8),
    (8, 6),
    (22, 13),
    (20, 3),
    (20, 19),
    (21, 19),
    (3, 1),
    (17, 8),
    (22, 19),
    (19, 8),
    (7, 3),
    (18, 14),
    (13, 7),
    (4, 6),
    (4, 14),
    (9, 15),
    (20, 11),
    (19, 14),
    (14, 6),
    (3, 3),
    (10, 6),
    (28, 18),
    (15, 3),
    (22, 4),
    (9, 4),
    (20, 4),
    (5, 6),
    (17, 6),
    (5, 14),
    (25, 18),
    (3, 16),
    (8, 13),
    (7, 19),
    (7, 6),
    (15, 1),
    (3, 4),
    (21, 6),
    (21, 13),
    (21, 8),
    (15, 16),
    (22, 3),
    (20, 13),
    (13, 4),
    (1, 3),
    (8, 4),
    (8, 19),
    (1, 1),
    (24, 4),
    (9, 16),
    (23, 14),
    (20, 14),
    (18, 6),
    (1, 4),
    (3, 15),
    (7, 4),
    (8, 8),
    (12, 18),
    (13, 18),
  }
});


#[allow(unused)]
impl ChiruParser{

  // 使用模板生成 每个非终结符的编号
  
  pub const PARSER_RULE: usize = 3; 
  pub const ELEMENT: usize = 7; 
  pub const ATTRIBUTE: usize = 13; 
  pub const ANNOTATION: usize = 11; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const BLOCK: usize = 4; 
  pub const LEXER_RULE: usize = 9; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const ATTRIBUTES: usize = 12; 
  pub const EPSILON: usize = 6; 
  pub const ALTERNATIVE: usize = 5; 
  pub const REGULAR: usize = 10; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const RULES: usize = 2; 



  pub fn new() -> Self {
    Self {
      error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 

    }
  }


  // 使用模板生成
  
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Box<dyn CompilationUnitContext> {
    let result = ll1_analyze(token_stream, Self::COMPILATION_UNIT, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Box<dyn RulesContext> {
    let result = ll1_analyze(token_stream, Self::PARSER_RULE, 
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners).unwrap();
    Box::new(result)
  } 

}






