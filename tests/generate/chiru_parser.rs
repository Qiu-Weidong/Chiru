
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
   RulesContext, BlockContext, AttributeContext, EbnfSuffixContext, LexerRuleContext, ElementContext, GrammarNameContext, RegularContext, AttributesContext, AnnotationContext, EpsilonContext, AlternativeContext, ParserRuleContext, CompilationUnitContext,
};


pub struct ChiruParser<'a> {
  pub analyzer: LL1Analyzer<'a>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (26, 7) => 40,
    (2, 3) => 8,
    (5, 4) => 16,
    (4, 19) => 13,
    (4, 9) => 13,
    (15, 16) => 5,
    (7, 3) => 25,
    (20, 3) => 21,
    (23, 19) => 29,
    (28, 18) => 43,
    (11, 16) => 37,
    (17, 8) => 10,
    (22, 11) => 27,
    (19, 19) => 15,
    (6, 9) => 18,
    (21, 13) => 23,
    (19, 13) => 15,
    (16, 4) => 7,
    (0, 2) => 0,
    (24, 15) => 33,
    (21, 12) => 24,
    (21, 3) => 23,
    (21, 8) => 23,
    (5, 9) => 17,
    (22, 10) => 26,
    (25, 7) => 38,
    (8, 12) => 31,
    (5, 13) => 16,
    (4, 3) => 13,
    (23, 13) => 29,
    (24, 16) => 33,
    (21, 6) => 23,
    (3, 3) => 9,
    (8, 11) => 31,
    (19, 4) => 15,
    (19, 14) => 14,
    (9, 15) => 34,
    (16, 1) => 6,
    (11, 15) => 36,
    (21, 4) => 23,
    (23, 4) => 29,
    (5, 3) => 16,
    (2, 1) => 8,
    (24, 4) => 32,
    (28, 7) => 43,
    (19, 8) => 14,
    (21, 14) => 23,
    (12, 3) => 41,
    (27, 13) => 42,
    (20, 4) => 19,
    (23, 12) => 30,
    (9, 4) => 34,
    (28, 4) => 43,
    (18, 6) => 11,
    (28, 13) => 44,
    (10, 20) => 35,
    (21, 19) => 23,
    (19, 6) => 14,
    (7, 4) => 25,
    (8, 10) => 31,
    (7, 13) => 25,
    (23, 3) => 29,
    (16, 15) => 7,
    (21, 10) => 24,
    (15, 3) => 4,
    (13, 3) => 45,
    (23, 14) => 29,
    (15, 15) => 5,
    (23, 8) => 29,
    (22, 12) => 28,
    (20, 19) => 20,
    (7, 19) => 25,
    (2, 15) => 8,
    (4, 4) => 13,
    (18, 14) => 11,
    (23, 6) => 29,
    (20, 13) => 22,
    (19, 3) => 15,
    (2, 16) => 8,
    (16, 3) => 7,
    (16, 16) => 7,
    (4, 13) => 13,
    (14, 4) => 1,
    (21, 11) => 24,
    (18, 8) => 12,
    (1, 2) => 3,
    (15, 4) => 5,
    (26, 18) => 39,
    (2, 4) => 8,
    (9, 16) => 34,
    (5, 19) => 16,
    (14, 3) => 2,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    11 => Production::new(11, 18, &vec![]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    29 => Production::new(29, 23, &vec![]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    6 => Production::new(6, 16, &vec![]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    32 => Production::new(32, 24, &vec![]),
    14 => Production::new(14, 19, &vec![]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    39 => Production::new(39, 26, &vec![]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    43 => Production::new(43, 28, &vec![]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    23 => Production::new(23, 21, &vec![]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    2 => String::from("rules"),
    4 => String::from("block"),
    13 => String::from("attribute"),
    8 => String::from("ebnf_suffix"),
    9 => String::from("lexer_rule"),
    7 => String::from("element"),
    1 => String::from("grammar_name"),
    10 => String::from("regular"),
    12 => String::from("attributes"),
    11 => String::from("annotation"),
    6 => String::from("epsilon"),
    5 => String::from("alternative"),
    3 => String::from("parser_rule"),
    0 => String::from("compilation_unit"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    5 => String::from("COLON"),
    23 => String::from("BLOCK_COMMENT"),
    8 => String::from("OR"),
    17 => String::from("LBRACKET"),
    4 => String::from("TOKEN_REF"),
    18 => String::from("RBRACKET"),
    6 => String::from("SEMI"),
    22 => String::from("LINE_COMMENT"),
    12 => String::from("QUESTION"),
    13 => String::from("LPAREN"),
    19 => String::from("STRING_LITERAL"),
    14 => String::from("RPAREN"),
    2 => String::from("GRAMMAR"),
    0 => String::from("_START"),
    7 => String::from("COMMA"),
    20 => String::from("REGULAR_LITERAL"),
    3 => String::from("RULE_REF"),
    16 => String::from("SHARP"),
    21 => String::from("WHITE_SPACE"),
    1 => String::from("_STOP"),
    10 => String::from("STAR"),
    11 => String::from("PLUS"),
    9 => String::from("EPSILON"),
    15 => String::from("AT"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (23, 4),
    (19, 6),
    (17, 8),
    (20, 19),
    (19, 8),
    (23, 19),
    (23, 8),
    (8, 8),
    (2, 1),
    (19, 14),
    (11, 4),
    (17, 6),
    (21, 8),
    (20, 3),
    (3, 4),
    (1, 3),
    (1, 4),
    (20, 11),
    (20, 12),
    (18, 14),
    (13, 4),
    (20, 4),
    (1, 15),
    (10, 6),
    (27, 7),
    (21, 14),
    (4, 14),
    (9, 15),
    (8, 6),
    (8, 19),
    (9, 3),
    (22, 13),
    (7, 14),
    (7, 13),
    (27, 18),
    (17, 14),
    (9, 4),
    (20, 8),
    (8, 4),
    (0, 1),
    (8, 13),
    (28, 18),
    (3, 15),
    (21, 13),
    (13, 7),
    (8, 3),
    (7, 3),
    (28, 7),
    (20, 13),
    (7, 6),
    (7, 19),
    (1, 16),
    (23, 6),
    (6, 8),
    (22, 8),
    (22, 14),
    (12, 18),
    (21, 19),
    (22, 4),
    (8, 14),
    (23, 13),
    (27, 4),
    (18, 6),
    (13, 18),
    (21, 3),
    (23, 3),
    (1, 1),
    (5, 6),
    (5, 8),
    (4, 6),
    (20, 6),
    (21, 6),
    (25, 18),
    (15, 4),
    (15, 15),
    (3, 1),
    (6, 6),
    (28, 4),
    (15, 3),
    (5, 14),
    (22, 19),
    (21, 4),
    (3, 3),
    (7, 8),
    (22, 6),
    (22, 12),
    (9, 16),
    (20, 14),
    (6, 14),
    (7, 4),
    (24, 4),
    (16, 1),
    (9, 1),
    (3, 16),
    (26, 18),
    (20, 10),
    (23, 14),
    (25, 7),
    (22, 3),
    (15, 16),
    (15, 1),
    (14, 6),
  }
});


#[allow(unused)]
impl ChiruParser<'_> {

  // 使用模板生成 每个非终结符的编号
  
  pub const RULES: usize = 2; 
  pub const BLOCK: usize = 4; 
  pub const ATTRIBUTE: usize = 13; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const LEXER_RULE: usize = 9; 
  pub const ELEMENT: usize = 7; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const REGULAR: usize = 10; 
  pub const ATTRIBUTES: usize = 12; 
  pub const ANNOTATION: usize = 11; 
  pub const EPSILON: usize = 6; 
  pub const ALTERNATIVE: usize = 5; 
  pub const PARSER_RULE: usize = 3; 
  pub const COMPILATION_UNIT: usize = 0; 



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
  
  pub fn rules(&self, token_stream: &mut TokenStream) -> Box<dyn RulesContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULES);
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
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
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
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTES);
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
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
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Box<dyn CompilationUnitContext> {
    let result = self.analyzer.analyse(token_stream, Self::COMPILATION_UNIT);
    Box::new(result)
  } 

}






