
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
   BlockContext, CompilationUnitContext, ParserRuleContext, AttributesContext, RegularContext, RulesContext, EbnfSuffixContext, EpsilonContext, AnnotationContext, AlternativeContext, LexerRuleContext, AttributeContext, ElementContext, GrammarNameContext,
};


pub struct ChiruParser<'a> {
  pub analyzer: LL1Analyzer<'a>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (7, 13) => 25,
    (16, 16) => 7,
    (20, 13) => 22,
    (27, 13) => 42,
    (23, 8) => 29,
    (16, 4) => 7,
    (7, 3) => 25,
    (23, 12) => 30,
    (0, 2) => 0,
    (5, 19) => 16,
    (4, 19) => 13,
    (22, 11) => 27,
    (21, 3) => 23,
    (26, 7) => 40,
    (4, 4) => 13,
    (9, 16) => 34,
    (16, 1) => 6,
    (15, 4) => 5,
    (21, 4) => 23,
    (13, 3) => 45,
    (16, 3) => 7,
    (24, 15) => 33,
    (21, 6) => 23,
    (14, 4) => 1,
    (28, 13) => 44,
    (8, 12) => 31,
    (10, 20) => 35,
    (18, 14) => 11,
    (24, 16) => 33,
    (5, 9) => 17,
    (4, 3) => 13,
    (24, 4) => 32,
    (21, 12) => 24,
    (20, 19) => 20,
    (8, 10) => 31,
    (19, 13) => 15,
    (9, 4) => 34,
    (28, 7) => 43,
    (21, 10) => 24,
    (19, 14) => 14,
    (5, 13) => 16,
    (15, 16) => 5,
    (20, 4) => 19,
    (28, 4) => 43,
    (22, 12) => 28,
    (26, 18) => 39,
    (9, 15) => 34,
    (7, 19) => 25,
    (23, 3) => 29,
    (2, 1) => 8,
    (4, 9) => 13,
    (4, 13) => 13,
    (25, 7) => 38,
    (22, 10) => 26,
    (18, 6) => 11,
    (1, 2) => 3,
    (5, 3) => 16,
    (23, 19) => 29,
    (14, 3) => 2,
    (21, 11) => 24,
    (2, 4) => 8,
    (2, 16) => 8,
    (19, 19) => 15,
    (21, 19) => 23,
    (11, 15) => 36,
    (3, 3) => 9,
    (23, 13) => 29,
    (19, 4) => 15,
    (2, 15) => 8,
    (15, 3) => 4,
    (19, 8) => 14,
    (20, 3) => 21,
    (28, 18) => 43,
    (23, 14) => 29,
    (8, 11) => 31,
    (17, 8) => 10,
    (7, 4) => 25,
    (5, 4) => 16,
    (18, 8) => 12,
    (21, 13) => 23,
    (21, 8) => 23,
    (19, 3) => 15,
    (6, 9) => 18,
    (23, 6) => 29,
    (2, 3) => 8,
    (15, 15) => 5,
    (16, 15) => 7,
    (12, 3) => 41,
    (11, 16) => 37,
    (19, 6) => 14,
    (21, 14) => 23,
    (23, 4) => 29,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    32 => Production::new(32, 24, &vec![]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    39 => Production::new(39, 26, &vec![]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    6 => Production::new(6, 16, &vec![]),
    11 => Production::new(11, 18, &vec![]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    43 => Production::new(43, 28, &vec![]),
    23 => Production::new(23, 21, &vec![]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    29 => Production::new(29, 23, &vec![]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    14 => Production::new(14, 19, &vec![]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    4 => String::from("block"),
    0 => String::from("compilation_unit"),
    3 => String::from("parser_rule"),
    12 => String::from("attributes"),
    10 => String::from("regular"),
    2 => String::from("rules"),
    8 => String::from("ebnf_suffix"),
    6 => String::from("epsilon"),
    11 => String::from("annotation"),
    5 => String::from("alternative"),
    9 => String::from("lexer_rule"),
    13 => String::from("attribute"),
    7 => String::from("element"),
    1 => String::from("grammar_name"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    6 => String::from("SEMI"),
    16 => String::from("SHARP"),
    19 => String::from("STRING_LITERAL"),
    7 => String::from("COMMA"),
    15 => String::from("AT"),
    10 => String::from("STAR"),
    11 => String::from("PLUS"),
    21 => String::from("WHITE_SPACE"),
    0 => String::from("_START"),
    17 => String::from("LBRACKET"),
    23 => String::from("BLOCK_COMMENT"),
    18 => String::from("RBRACKET"),
    9 => String::from("EPSILON"),
    13 => String::from("LPAREN"),
    5 => String::from("COLON"),
    12 => String::from("QUESTION"),
    22 => String::from("LINE_COMMENT"),
    4 => String::from("TOKEN_REF"),
    14 => String::from("RPAREN"),
    8 => String::from("OR"),
    1 => String::from("_STOP"),
    3 => String::from("RULE_REF"),
    20 => String::from("REGULAR_LITERAL"),
    2 => String::from("GRAMMAR"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (6, 6),
    (23, 13),
    (1, 16),
    (10, 6),
    (2, 1),
    (20, 4),
    (23, 6),
    (8, 8),
    (7, 13),
    (18, 14),
    (22, 19),
    (3, 1),
    (20, 13),
    (22, 3),
    (22, 8),
    (20, 6),
    (13, 7),
    (7, 8),
    (9, 16),
    (8, 19),
    (6, 14),
    (21, 6),
    (1, 15),
    (17, 8),
    (22, 4),
    (15, 3),
    (15, 4),
    (22, 12),
    (20, 11),
    (15, 16),
    (24, 4),
    (3, 4),
    (3, 3),
    (17, 6),
    (9, 3),
    (8, 13),
    (20, 14),
    (4, 14),
    (15, 15),
    (25, 7),
    (15, 1),
    (20, 8),
    (9, 1),
    (23, 8),
    (19, 14),
    (22, 6),
    (1, 4),
    (7, 3),
    (8, 14),
    (12, 18),
    (23, 19),
    (27, 7),
    (6, 8),
    (21, 13),
    (7, 19),
    (21, 14),
    (20, 3),
    (9, 4),
    (13, 18),
    (27, 4),
    (7, 14),
    (21, 19),
    (18, 6),
    (27, 18),
    (3, 16),
    (14, 6),
    (22, 13),
    (8, 3),
    (5, 8),
    (5, 14),
    (28, 4),
    (20, 12),
    (11, 4),
    (19, 8),
    (28, 18),
    (25, 18),
    (4, 6),
    (23, 3),
    (22, 14),
    (9, 15),
    (23, 14),
    (26, 18),
    (8, 6),
    (28, 7),
    (19, 6),
    (7, 4),
    (3, 15),
    (16, 1),
    (17, 14),
    (7, 6),
    (20, 10),
    (5, 6),
    (23, 4),
    (13, 4),
    (1, 3),
    (0, 1),
    (21, 4),
    (20, 19),
    (21, 8),
    (1, 1),
    (8, 4),
    (21, 3),
  }
});


#[allow(unused)]
impl ChiruParser<'_> {

  // 使用模板生成 每个非终结符的编号
  
  pub const BLOCK: usize = 4; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const PARSER_RULE: usize = 3; 
  pub const ATTRIBUTES: usize = 12; 
  pub const REGULAR: usize = 10; 
  pub const RULES: usize = 2; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const EPSILON: usize = 6; 
  pub const ANNOTATION: usize = 11; 
  pub const ALTERNATIVE: usize = 5; 
  pub const LEXER_RULE: usize = 9; 
  pub const ATTRIBUTE: usize = 13; 
  pub const ELEMENT: usize = 7; 
  pub const GRAMMAR_NAME: usize = 1; 



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
  
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
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
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTES);
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
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
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
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
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
  pub fn element(&self, token_stream: &mut TokenStream) -> Box<dyn ElementContext> {
    let result = self.analyzer.analyse(token_stream, Self::ELEMENT);
    Box::new(result)
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
    Box::new(result)
  } 

}






