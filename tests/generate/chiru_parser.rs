
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
   EpsilonContext, AlternativeContext, AnnotationContext, GrammarNameContext, BlockContext, LexerRuleContext, ParserRuleContext, EbnfSuffixContext, RulesContext, ElementContext, AttributeContext, AttributesContext, RegularContext, CompilationUnitContext,
};


pub struct ChiruParser<'a> {
  pub analyzer: LL1Analyzer<'a>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (19, 8) => 14,
    (27, 13) => 42,
    (21, 4) => 23,
    (15, 4) => 5,
    (23, 12) => 30,
    (24, 4) => 32,
    (5, 3) => 16,
    (7, 13) => 25,
    (9, 16) => 34,
    (2, 1) => 8,
    (19, 13) => 15,
    (14, 3) => 2,
    (18, 14) => 11,
    (18, 6) => 11,
    (3, 3) => 9,
    (8, 12) => 31,
    (7, 3) => 25,
    (11, 15) => 36,
    (1, 2) => 3,
    (2, 3) => 8,
    (2, 4) => 8,
    (23, 6) => 29,
    (23, 4) => 29,
    (19, 4) => 15,
    (23, 13) => 29,
    (9, 4) => 34,
    (14, 4) => 1,
    (24, 15) => 33,
    (4, 3) => 13,
    (21, 12) => 24,
    (7, 4) => 25,
    (19, 19) => 15,
    (19, 6) => 14,
    (16, 1) => 6,
    (4, 19) => 13,
    (20, 4) => 19,
    (5, 13) => 16,
    (22, 12) => 28,
    (2, 16) => 8,
    (16, 3) => 7,
    (6, 9) => 18,
    (2, 15) => 8,
    (15, 3) => 4,
    (21, 10) => 24,
    (20, 13) => 22,
    (19, 3) => 15,
    (21, 3) => 23,
    (4, 9) => 13,
    (8, 11) => 31,
    (21, 11) => 24,
    (28, 7) => 43,
    (4, 4) => 13,
    (16, 4) => 7,
    (28, 4) => 43,
    (21, 6) => 23,
    (23, 19) => 29,
    (20, 3) => 21,
    (15, 16) => 5,
    (23, 14) => 29,
    (23, 8) => 29,
    (26, 18) => 39,
    (22, 11) => 27,
    (19, 14) => 14,
    (0, 2) => 0,
    (13, 3) => 45,
    (28, 18) => 43,
    (17, 8) => 10,
    (21, 19) => 23,
    (21, 13) => 23,
    (28, 13) => 44,
    (5, 19) => 16,
    (5, 4) => 16,
    (18, 8) => 12,
    (4, 13) => 13,
    (20, 19) => 20,
    (5, 9) => 17,
    (24, 16) => 33,
    (25, 7) => 38,
    (15, 15) => 5,
    (10, 20) => 35,
    (12, 3) => 41,
    (9, 15) => 34,
    (16, 16) => 7,
    (21, 8) => 23,
    (8, 10) => 31,
    (23, 3) => 29,
    (26, 7) => 40,
    (22, 10) => 26,
    (16, 15) => 7,
    (21, 14) => 23,
    (7, 19) => 25,
    (11, 16) => 37,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    23 => Production::new(23, 21, &vec![]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    14 => Production::new(14, 19, &vec![]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    39 => Production::new(39, 26, &vec![]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    43 => Production::new(43, 28, &vec![]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    11 => Production::new(11, 18, &vec![]),
    32 => Production::new(32, 24, &vec![]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    29 => Production::new(29, 23, &vec![]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    6 => Production::new(6, 16, &vec![]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    6 => String::from("epsilon"),
    5 => String::from("alternative"),
    11 => String::from("annotation"),
    1 => String::from("grammar_name"),
    4 => String::from("block"),
    9 => String::from("lexer_rule"),
    3 => String::from("parser_rule"),
    8 => String::from("ebnf_suffix"),
    2 => String::from("rules"),
    7 => String::from("element"),
    13 => String::from("attribute"),
    12 => String::from("attributes"),
    10 => String::from("regular"),
    0 => String::from("compilation_unit"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    21 => String::from("WHITE_SPACE"),
    0 => String::from("_START"),
    18 => String::from("RBRACKET"),
    12 => String::from("QUESTION"),
    13 => String::from("LPAREN"),
    17 => String::from("LBRACKET"),
    7 => String::from("COMMA"),
    11 => String::from("PLUS"),
    5 => String::from("COLON"),
    10 => String::from("STAR"),
    1 => String::from("_STOP"),
    22 => String::from("LINE_COMMENT"),
    15 => String::from("AT"),
    20 => String::from("REGULAR_LITERAL"),
    3 => String::from("RULE_REF"),
    4 => String::from("TOKEN_REF"),
    23 => String::from("BLOCK_COMMENT"),
    9 => String::from("EPSILON"),
    14 => String::from("RPAREN"),
    19 => String::from("STRING_LITERAL"),
    16 => String::from("SHARP"),
    2 => String::from("GRAMMAR"),
    6 => String::from("SEMI"),
    8 => String::from("OR"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (23, 19),
    (20, 10),
    (6, 8),
    (20, 12),
    (25, 7),
    (9, 4),
    (5, 8),
    (3, 3),
    (18, 6),
    (8, 19),
    (9, 3),
    (9, 16),
    (8, 3),
    (20, 14),
    (7, 19),
    (19, 8),
    (20, 6),
    (7, 6),
    (18, 14),
    (21, 6),
    (7, 14),
    (22, 4),
    (21, 14),
    (20, 3),
    (8, 6),
    (22, 8),
    (19, 14),
    (17, 8),
    (15, 4),
    (14, 6),
    (9, 15),
    (23, 13),
    (7, 8),
    (7, 4),
    (8, 13),
    (15, 16),
    (5, 6),
    (15, 3),
    (20, 19),
    (3, 16),
    (15, 1),
    (13, 7),
    (4, 6),
    (8, 4),
    (3, 4),
    (3, 1),
    (20, 8),
    (6, 6),
    (17, 6),
    (23, 6),
    (6, 14),
    (28, 4),
    (23, 8),
    (21, 13),
    (21, 8),
    (17, 14),
    (9, 1),
    (20, 4),
    (1, 3),
    (7, 13),
    (21, 19),
    (12, 18),
    (5, 14),
    (22, 12),
    (22, 13),
    (22, 14),
    (22, 6),
    (19, 6),
    (22, 19),
    (2, 1),
    (13, 4),
    (0, 1),
    (11, 4),
    (1, 1),
    (1, 16),
    (24, 4),
    (23, 14),
    (26, 18),
    (4, 14),
    (20, 13),
    (3, 15),
    (23, 4),
    (16, 1),
    (8, 8),
    (27, 18),
    (28, 7),
    (10, 6),
    (27, 4),
    (21, 4),
    (13, 18),
    (1, 15),
    (1, 4),
    (8, 14),
    (23, 3),
    (7, 3),
    (15, 15),
    (25, 18),
    (27, 7),
    (20, 11),
    (28, 18),
    (21, 3),
    (22, 3),
  }
});


#[allow(unused)]
impl ChiruParser<'_> {

  // 使用模板生成 每个非终结符的编号
  
  pub const EPSILON: usize = 6; 
  pub const ALTERNATIVE: usize = 5; 
  pub const ANNOTATION: usize = 11; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const BLOCK: usize = 4; 
  pub const LEXER_RULE: usize = 9; 
  pub const PARSER_RULE: usize = 3; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const RULES: usize = 2; 
  pub const ELEMENT: usize = 7; 
  pub const ATTRIBUTE: usize = 13; 
  pub const ATTRIBUTES: usize = 12; 
  pub const REGULAR: usize = 10; 
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
  
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
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
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
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
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTES);
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

}






