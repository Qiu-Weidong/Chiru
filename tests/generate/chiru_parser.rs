
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
   EpsilonContext, BlockContext, AttributeContext, AnnotationContext, RegularContext, CompilationUnitContext, LexerRuleContext, ElementContext, ParserRuleContext, AttributesContext, GrammarNameContext, AlternativeContext, RulesContext, EbnfSuffixContext,
};


pub struct ChiruParser<'a> {
  pub analyzer: LL1Analyzer<'a>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (16, 3) => 7,
    (16, 4) => 7,
    (1, 2) => 3,
    (4, 4) => 13,
    (2, 3) => 8,
    (28, 4) => 43,
    (4, 3) => 13,
    (19, 8) => 14,
    (21, 11) => 24,
    (20, 19) => 20,
    (9, 15) => 34,
    (22, 10) => 26,
    (2, 1) => 8,
    (5, 13) => 16,
    (27, 13) => 42,
    (23, 8) => 29,
    (6, 9) => 18,
    (23, 12) => 30,
    (19, 3) => 15,
    (8, 11) => 31,
    (5, 3) => 16,
    (23, 4) => 29,
    (26, 18) => 39,
    (11, 15) => 36,
    (19, 13) => 15,
    (28, 18) => 43,
    (4, 19) => 13,
    (23, 6) => 29,
    (0, 2) => 0,
    (19, 4) => 15,
    (14, 4) => 1,
    (20, 4) => 19,
    (20, 13) => 22,
    (2, 16) => 8,
    (21, 13) => 23,
    (21, 10) => 24,
    (21, 6) => 23,
    (21, 14) => 23,
    (19, 19) => 15,
    (11, 16) => 37,
    (5, 9) => 17,
    (9, 4) => 34,
    (24, 4) => 32,
    (23, 14) => 29,
    (5, 19) => 16,
    (7, 4) => 25,
    (21, 12) => 24,
    (15, 3) => 4,
    (9, 16) => 34,
    (22, 11) => 27,
    (8, 12) => 31,
    (25, 7) => 38,
    (15, 16) => 5,
    (5, 4) => 16,
    (18, 6) => 11,
    (24, 16) => 33,
    (19, 14) => 14,
    (23, 19) => 29,
    (7, 3) => 25,
    (12, 3) => 41,
    (13, 3) => 45,
    (20, 3) => 21,
    (22, 12) => 28,
    (26, 7) => 40,
    (16, 1) => 6,
    (23, 13) => 29,
    (18, 14) => 11,
    (21, 3) => 23,
    (21, 8) => 23,
    (7, 19) => 25,
    (16, 16) => 7,
    (23, 3) => 29,
    (7, 13) => 25,
    (2, 15) => 8,
    (18, 8) => 12,
    (4, 9) => 13,
    (8, 10) => 31,
    (16, 15) => 7,
    (3, 3) => 9,
    (24, 15) => 33,
    (17, 8) => 10,
    (14, 3) => 2,
    (2, 4) => 8,
    (4, 13) => 13,
    (10, 20) => 35,
    (15, 4) => 5,
    (28, 13) => 44,
    (19, 6) => 14,
    (28, 7) => 43,
    (15, 15) => 5,
    (21, 4) => 23,
    (21, 19) => 23,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    6 => Production::new(6, 16, &vec![]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    32 => Production::new(32, 24, &vec![]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    29 => Production::new(29, 23, &vec![]),
    39 => Production::new(39, 26, &vec![]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    43 => Production::new(43, 28, &vec![]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    11 => Production::new(11, 18, &vec![]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    14 => Production::new(14, 19, &vec![]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    23 => Production::new(23, 21, &vec![]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, &'static str>> = Lazy::new(|| {
  hashmap! {
    
    6 => "epsilon",
    4 => "block",
    13 => "attribute",
    11 => "annotation",
    10 => "regular",
    0 => "compilation_unit",
    9 => "lexer_rule",
    7 => "element",
    3 => "parser_rule",
    12 => "attributes",
    1 => "grammar_name",
    5 => "alternative",
    2 => "rules",
    8 => "ebnf_suffix",
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, &'static str>> = Lazy::new(|| {
  hashmap! {
    
    21 => "WHITE_SPACE",
    13 => "LPAREN",
    2 => "GRAMMAR",
    3 => "RULE_REF",
    22 => "LINE_COMMENT",
    16 => "SHARP",
    20 => "REGULAR_LITERAL",
    11 => "PLUS",
    7 => "COMMA",
    19 => "STRING_LITERAL",
    15 => "AT",
    9 => "EPSILON",
    1 => "_STOP",
    4 => "TOKEN_REF",
    23 => "BLOCK_COMMENT",
    12 => "QUESTION",
    8 => "OR",
    0 => "_START",
    5 => "COLON",
    6 => "SEMI",
    18 => "RBRACKET",
    17 => "LBRACKET",
    10 => "STAR",
    14 => "RPAREN",
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (8, 6),
    (7, 19),
    (24, 4),
    (3, 15),
    (17, 6),
    (27, 18),
    (20, 19),
    (15, 3),
    (8, 13),
    (8, 19),
    (4, 6),
    (1, 4),
    (0, 1),
    (21, 3),
    (6, 6),
    (22, 14),
    (19, 14),
    (5, 6),
    (20, 3),
    (12, 18),
    (3, 3),
    (6, 14),
    (15, 16),
    (4, 14),
    (7, 3),
    (8, 4),
    (27, 7),
    (8, 8),
    (25, 7),
    (23, 3),
    (10, 6),
    (2, 1),
    (25, 18),
    (23, 6),
    (11, 4),
    (9, 16),
    (5, 14),
    (20, 12),
    (15, 4),
    (7, 8),
    (28, 18),
    (19, 6),
    (6, 8),
    (21, 8),
    (22, 12),
    (20, 11),
    (23, 14),
    (7, 6),
    (5, 8),
    (13, 4),
    (9, 3),
    (15, 1),
    (9, 4),
    (1, 3),
    (21, 14),
    (27, 4),
    (21, 19),
    (9, 1),
    (23, 4),
    (17, 14),
    (28, 7),
    (17, 8),
    (14, 6),
    (1, 15),
    (1, 1),
    (3, 4),
    (18, 14),
    (16, 1),
    (28, 4),
    (7, 13),
    (8, 14),
    (21, 6),
    (22, 4),
    (22, 3),
    (21, 13),
    (18, 6),
    (20, 13),
    (20, 8),
    (15, 15),
    (1, 16),
    (21, 4),
    (22, 13),
    (13, 7),
    (20, 4),
    (9, 15),
    (23, 19),
    (20, 10),
    (13, 18),
    (23, 13),
    (7, 4),
    (20, 14),
    (23, 8),
    (26, 18),
    (22, 6),
    (22, 8),
    (22, 19),
    (7, 14),
    (8, 3),
    (19, 8),
    (20, 6),
    (3, 16),
    (3, 1),
  }
});


#[allow(unused)]
impl ChiruParser<'_> {

  // 使用模板生成 每个非终结符的编号
  
  pub const EPSILON: usize = 6; 
  pub const BLOCK: usize = 4; 
  pub const ATTRIBUTE: usize = 13; 
  pub const ANNOTATION: usize = 11; 
  pub const REGULAR: usize = 10; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const LEXER_RULE: usize = 9; 
  pub const ELEMENT: usize = 7; 
  pub const PARSER_RULE: usize = 3; 
  pub const ATTRIBUTES: usize = 12; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const ALTERNATIVE: usize = 5; 
  pub const RULES: usize = 2; 
  pub const EBNF_SUFFIX: usize = 8; 



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
  
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
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
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Box<dyn CompilationUnitContext> {
    let result = self.analyzer.analyse(token_stream, Self::COMPILATION_UNIT);
    Box::new(result)
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Box<dyn LexerRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::LEXER_RULE);
    Box::new(result)
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTES);
    Box::new(result)
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Box<dyn RulesContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULES);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 

}






