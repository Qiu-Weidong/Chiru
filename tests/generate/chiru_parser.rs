
use once_cell::sync::Lazy;

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::runtime::{
  parser::LL1Analyzer, token_stream::TokenStream, 
  error_strategy::error_listener::ConsoleErrorListener,
  production::Production,
  production::ProductionItem
};

use super::chiru_context::{
   EbnfSuffixContext, GrammarNameContext, AnnotationContext, CompilationUnitContext, AttributesContext, AlternativeContext, LexerRuleContext, ParserRuleContext, EpsilonContext, BlockContext, RegularContext, RulesContext, ElementContext, AttributeContext,
};


pub struct ChiruParser {
  pub analyzer: LL1Analyzer,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (7, 3) => 25,
    (27, 13) => 42,
    (14, 4) => 1,
    (9, 15) => 34,
    (23, 12) => 30,
    (10, 20) => 35,
    (8, 12) => 31,
    (20, 3) => 21,
    (25, 7) => 38,
    (19, 8) => 14,
    (5, 19) => 16,
    (21, 13) => 23,
    (5, 3) => 16,
    (0, 2) => 0,
    (4, 19) => 13,
    (26, 7) => 40,
    (23, 14) => 29,
    (6, 9) => 18,
    (22, 12) => 28,
    (2, 3) => 8,
    (5, 13) => 16,
    (21, 3) => 23,
    (13, 3) => 45,
    (9, 4) => 34,
    (5, 9) => 17,
    (11, 16) => 37,
    (7, 19) => 25,
    (19, 3) => 15,
    (24, 16) => 33,
    (21, 14) => 23,
    (21, 10) => 24,
    (4, 4) => 13,
    (28, 4) => 43,
    (21, 6) => 23,
    (12, 3) => 41,
    (8, 10) => 31,
    (16, 3) => 7,
    (28, 18) => 43,
    (2, 4) => 8,
    (18, 14) => 11,
    (18, 8) => 12,
    (2, 1) => 8,
    (21, 19) => 23,
    (15, 15) => 5,
    (4, 3) => 13,
    (21, 4) => 23,
    (18, 6) => 11,
    (16, 15) => 7,
    (2, 15) => 8,
    (5, 4) => 16,
    (23, 19) => 29,
    (8, 11) => 31,
    (23, 6) => 29,
    (16, 16) => 7,
    (20, 13) => 22,
    (9, 16) => 34,
    (26, 18) => 39,
    (19, 6) => 14,
    (16, 1) => 6,
    (16, 4) => 7,
    (24, 15) => 33,
    (28, 7) => 43,
    (19, 4) => 15,
    (19, 14) => 14,
    (15, 3) => 4,
    (28, 13) => 44,
    (17, 8) => 10,
    (24, 4) => 32,
    (4, 9) => 13,
    (14, 3) => 2,
    (23, 3) => 29,
    (21, 12) => 24,
    (23, 13) => 29,
    (23, 4) => 29,
    (21, 11) => 24,
    (22, 11) => 27,
    (4, 13) => 13,
    (15, 4) => 5,
    (1, 2) => 3,
    (7, 13) => 25,
    (23, 8) => 29,
    (7, 4) => 25,
    (22, 10) => 26,
    (15, 16) => 5,
    (20, 4) => 19,
    (3, 3) => 9,
    (19, 19) => 15,
    (20, 19) => 20,
    (19, 13) => 15,
    (2, 16) => 8,
    (11, 15) => 36,
    (21, 8) => 23,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    29 => Production::new(29, 23, &vec![]),
    11 => Production::new(11, 18, &vec![]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    23 => Production::new(23, 21, &vec![]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    43 => Production::new(43, 28, &vec![]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    32 => Production::new(32, 24, &vec![]),
    14 => Production::new(14, 19, &vec![]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    39 => Production::new(39, 26, &vec![]),
    6 => Production::new(6, 16, &vec![]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, &'static str>> = Lazy::new(|| {
  hashmap! {
    
    8 => "ebnf_suffix",
    1 => "grammar_name",
    11 => "annotation",
    0 => "compilation_unit",
    12 => "attributes",
    5 => "alternative",
    9 => "lexer_rule",
    3 => "parser_rule",
    6 => "epsilon",
    4 => "block",
    10 => "regular",
    2 => "rules",
    7 => "element",
    13 => "attribute",
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, &'static str>> = Lazy::new(|| {
  hashmap! {
    
    10 => "STAR",
    0 => "_START",
    21 => "WHITE_SPACE",
    1 => "_STOP",
    22 => "LINE_COMMENT",
    6 => "SEMI",
    2 => "GRAMMAR",
    5 => "COLON",
    4 => "TOKEN_REF",
    14 => "RPAREN",
    13 => "LPAREN",
    17 => "LBRACKET",
    8 => "OR",
    9 => "EPSILON",
    16 => "SHARP",
    11 => "PLUS",
    23 => "BLOCK_COMMENT",
    20 => "REGULAR_LITERAL",
    12 => "QUESTION",
    15 => "AT",
    7 => "COMMA",
    19 => "STRING_LITERAL",
    18 => "RBRACKET",
    3 => "RULE_REF",
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (10, 6),
    (1, 16),
    (7, 19),
    (22, 13),
    (1, 1),
    (3, 1),
    (21, 14),
    (3, 3),
    (21, 8),
    (1, 4),
    (21, 19),
    (7, 3),
    (23, 14),
    (23, 6),
    (23, 13),
    (4, 14),
    (2, 1),
    (12, 18),
    (19, 14),
    (9, 16),
    (24, 4),
    (23, 4),
    (6, 14),
    (23, 3),
    (18, 6),
    (19, 8),
    (17, 8),
    (15, 3),
    (7, 13),
    (22, 12),
    (22, 14),
    (19, 6),
    (23, 8),
    (21, 13),
    (9, 15),
    (8, 13),
    (20, 11),
    (8, 8),
    (20, 14),
    (8, 19),
    (17, 6),
    (4, 6),
    (20, 4),
    (7, 6),
    (9, 3),
    (28, 18),
    (9, 1),
    (15, 16),
    (20, 3),
    (20, 10),
    (21, 4),
    (27, 7),
    (28, 7),
    (11, 4),
    (14, 6),
    (3, 15),
    (17, 14),
    (21, 6),
    (16, 1),
    (15, 4),
    (5, 14),
    (20, 8),
    (1, 15),
    (15, 1),
    (23, 19),
    (0, 1),
    (5, 6),
    (8, 14),
    (22, 19),
    (27, 4),
    (8, 6),
    (1, 3),
    (22, 8),
    (13, 18),
    (13, 7),
    (20, 6),
    (20, 12),
    (5, 8),
    (22, 3),
    (22, 4),
    (13, 4),
    (7, 8),
    (28, 4),
    (26, 18),
    (20, 19),
    (20, 13),
    (9, 4),
    (15, 15),
    (8, 4),
    (3, 16),
    (25, 18),
    (18, 14),
    (7, 4),
    (7, 14),
    (6, 8),
    (21, 3),
    (22, 6),
    (27, 18),
    (8, 3),
    (25, 7),
    (3, 4),
    (6, 6),
  }
});


#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const EBNF_SUFFIX: usize = 8; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const ANNOTATION: usize = 11; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const ATTRIBUTES: usize = 12; 
  pub const ALTERNATIVE: usize = 5; 
  pub const LEXER_RULE: usize = 9; 
  pub const PARSER_RULE: usize = 3; 
  pub const EPSILON: usize = 6; 
  pub const BLOCK: usize = 4; 
  pub const REGULAR: usize = 10; 
  pub const RULES: usize = 2; 
  pub const ELEMENT: usize = 7; 
  pub const ATTRIBUTE: usize = 13; 



  pub fn new() -> Self {
    Self {
      analyzer: LL1Analyzer { 
        error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
        table: &LL1_TABLE, 
        productions: &PRODUCTIONS, 
        rule_names: &NONTERMINALS, 
        sync: &SYNC, 
      }
    }
  }


  // 使用模板生成
  
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
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
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTES);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
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
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Box<dyn RulesContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULES);
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

}






