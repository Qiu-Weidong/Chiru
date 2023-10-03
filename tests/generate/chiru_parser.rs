
use std::error::Error;
use std::collections::{HashMap, HashSet};

use chiru::runtime::error_strategy::error_listener::ErrorListener;
use chiru::runtime::ll1_analyzer::ll1_analyze;

use chiru::maplit::hashmap;
use chiru::maplit::hashset;
use chiru::once_cell::sync::Lazy;

use chiru::runtime::{
  token_stream::TokenStream, 
  error_strategy::error_listener::ConsoleErrorListener,
  production::Production,
  production::ProductionItem
};

use super::chiru_context::{
   AttributesContext, GrammarNameContext, EbnfSuffixContext, ParserRuleContext, ElementContext, RegularContext, BlockContext, AttributeContext, EpsilonContext, AlternativeContext, CompilationUnitContext, RulesContext, LexerRuleContext, AnnotationContext,
};


pub struct ChiruParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (7, 4) => 25,
    (21, 11) => 24,
    (7, 3) => 25,
    (4, 19) => 13,
    (20, 19) => 20,
    (4, 13) => 13,
    (28, 13) => 44,
    (6, 9) => 18,
    (3, 3) => 9,
    (21, 10) => 24,
    (7, 13) => 25,
    (27, 13) => 42,
    (14, 4) => 1,
    (19, 14) => 14,
    (7, 19) => 25,
    (2, 16) => 8,
    (16, 1) => 6,
    (5, 3) => 16,
    (23, 14) => 29,
    (20, 13) => 22,
    (17, 8) => 10,
    (5, 4) => 16,
    (23, 8) => 29,
    (19, 4) => 15,
    (21, 14) => 23,
    (19, 8) => 14,
    (9, 15) => 34,
    (20, 4) => 19,
    (12, 3) => 41,
    (8, 10) => 31,
    (2, 3) => 8,
    (19, 13) => 15,
    (11, 16) => 37,
    (2, 15) => 8,
    (9, 16) => 34,
    (8, 11) => 31,
    (16, 3) => 7,
    (16, 4) => 7,
    (18, 8) => 12,
    (23, 13) => 29,
    (1, 2) => 3,
    (5, 9) => 17,
    (23, 19) => 29,
    (19, 3) => 15,
    (22, 12) => 28,
    (16, 16) => 7,
    (15, 3) => 4,
    (15, 4) => 5,
    (23, 12) => 30,
    (14, 3) => 2,
    (2, 1) => 8,
    (23, 4) => 29,
    (13, 3) => 45,
    (20, 3) => 21,
    (26, 18) => 39,
    (9, 4) => 34,
    (18, 14) => 11,
    (24, 16) => 33,
    (21, 12) => 24,
    (28, 7) => 43,
    (21, 3) => 23,
    (18, 6) => 11,
    (5, 19) => 16,
    (23, 3) => 29,
    (28, 18) => 43,
    (24, 15) => 33,
    (25, 7) => 38,
    (4, 3) => 13,
    (4, 9) => 13,
    (11, 15) => 36,
    (10, 20) => 35,
    (0, 2) => 0,
    (28, 4) => 43,
    (8, 12) => 31,
    (15, 15) => 5,
    (16, 15) => 7,
    (21, 8) => 23,
    (21, 6) => 23,
    (22, 11) => 27,
    (19, 6) => 14,
    (2, 4) => 8,
    (5, 13) => 16,
    (23, 6) => 29,
    (15, 16) => 5,
    (21, 19) => 23,
    (22, 10) => 26,
    (21, 4) => 23,
    (24, 4) => 32,
    (21, 13) => 23,
    (26, 7) => 40,
    (4, 4) => 13,
    (19, 19) => 15,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    39 => Production::new(39, 26, &vec![]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    32 => Production::new(32, 24, &vec![]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    11 => Production::new(11, 18, &vec![]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    43 => Production::new(43, 28, &vec![]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    6 => Production::new(6, 16, &vec![]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    29 => Production::new(29, 23, &vec![]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    23 => Production::new(23, 21, &vec![]),
    14 => Production::new(14, 19, &vec![]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    12 => String::from("attributes"),
    1 => String::from("grammar_name"),
    8 => String::from("ebnf_suffix"),
    3 => String::from("parser_rule"),
    7 => String::from("element"),
    10 => String::from("regular"),
    4 => String::from("block"),
    13 => String::from("attribute"),
    6 => String::from("epsilon"),
    5 => String::from("alternative"),
    0 => String::from("compilation_unit"),
    2 => String::from("rules"),
    9 => String::from("lexer_rule"),
    11 => String::from("annotation"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    5 => String::from("COLON"),
    23 => String::from("BLOCK_COMMENT"),
    16 => String::from("SHARP"),
    17 => String::from("LBRACKET"),
    1 => String::from("_STOP"),
    8 => String::from("OR"),
    22 => String::from("LINE_COMMENT"),
    0 => String::from("_START"),
    13 => String::from("LPAREN"),
    7 => String::from("COMMA"),
    15 => String::from("AT"),
    3 => String::from("RULE_REF"),
    21 => String::from("WHITE_SPACE"),
    20 => String::from("REGULAR_LITERAL"),
    10 => String::from("STAR"),
    18 => String::from("RBRACKET"),
    19 => String::from("STRING_LITERAL"),
    11 => String::from("PLUS"),
    12 => String::from("QUESTION"),
    2 => String::from("GRAMMAR"),
    9 => String::from("EPSILON"),
    14 => String::from("RPAREN"),
    6 => String::from("SEMI"),
    4 => String::from("TOKEN_REF"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (3, 4),
    (21, 3),
    (7, 3),
    (15, 1),
    (7, 14),
    (20, 10),
    (17, 8),
    (22, 13),
    (15, 15),
    (13, 7),
    (1, 15),
    (17, 14),
    (22, 12),
    (20, 6),
    (19, 6),
    (23, 3),
    (23, 8),
    (8, 6),
    (1, 16),
    (23, 14),
    (7, 6),
    (25, 7),
    (13, 4),
    (1, 4),
    (9, 16),
    (9, 4),
    (0, 1),
    (21, 14),
    (20, 14),
    (20, 19),
    (20, 8),
    (5, 8),
    (10, 6),
    (15, 3),
    (25, 18),
    (21, 6),
    (18, 14),
    (19, 8),
    (9, 15),
    (8, 19),
    (23, 6),
    (19, 14),
    (3, 1),
    (8, 14),
    (6, 14),
    (23, 19),
    (9, 1),
    (13, 18),
    (21, 13),
    (27, 18),
    (7, 19),
    (5, 14),
    (20, 13),
    (20, 4),
    (20, 11),
    (28, 18),
    (8, 8),
    (27, 7),
    (8, 4),
    (11, 4),
    (1, 3),
    (26, 18),
    (12, 18),
    (24, 4),
    (16, 1),
    (22, 4),
    (7, 13),
    (5, 6),
    (4, 14),
    (9, 3),
    (14, 6),
    (8, 13),
    (6, 8),
    (21, 19),
    (3, 15),
    (22, 6),
    (22, 19),
    (17, 6),
    (20, 3),
    (28, 7),
    (7, 8),
    (21, 4),
    (27, 4),
    (21, 8),
    (23, 13),
    (22, 3),
    (18, 6),
    (3, 3),
    (28, 4),
    (22, 8),
    (2, 1),
    (23, 4),
    (7, 4),
    (1, 1),
    (6, 6),
    (4, 6),
    (22, 14),
    (15, 16),
    (3, 16),
    (20, 12),
    (8, 3),
    (15, 4),
  }
});


#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ATTRIBUTES: usize = 12; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const PARSER_RULE: usize = 3; 
  pub const ELEMENT: usize = 7; 
  pub const REGULAR: usize = 10; 
  pub const BLOCK: usize = 4; 
  pub const ATTRIBUTE: usize = 13; 
  pub const EPSILON: usize = 6; 
  pub const ALTERNATIVE: usize = 5; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const RULES: usize = 2; 
  pub const LEXER_RULE: usize = 9; 
  pub const ANNOTATION: usize = 11; 



  pub fn new() -> Self {
    Self {
      error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
    }
  }


  // 使用模板生成
  
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributesContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ATTRIBUTES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Result<Box<dyn GrammarNameContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::GRAMMAR_NAME,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EbnfSuffixContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::EBNF_SUFFIX,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ParserRuleContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::PARSER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ElementContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ELEMENT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RegularContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::REGULAR,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Result<Box<dyn BlockContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::BLOCK,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributeContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ATTRIBUTE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EpsilonContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::EPSILON,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AlternativeContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ALTERNATIVE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Result<Box<dyn CompilationUnitContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::COMPILATION_UNIT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RulesContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::RULES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn LexerRuleContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::LEXER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AnnotationContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ANNOTATION,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 

}






