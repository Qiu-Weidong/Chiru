



// generated from .\src\tool\syntaxis\chiru.chiru by chiru 0.7.0
 


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
   AttributesContext, BlockContext, AlternativeContext, EbnfSuffixContext, RulesContext, ElementContext, AttributeContext, LexerRuleContext, CompilationUnitContext, AnnotationContext, ParserRuleContext, EpsilonContext, GrammarNameContext, RegularContext,
};


pub struct ChiruParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (5, 13) => 16,
    (3, 3) => 9,
    (19, 3) => 15,
    (8, 10) => 31,
    (27, 13) => 42,
    (23, 3) => 29,
    (21, 8) => 23,
    (20, 3) => 21,
    (8, 11) => 31,
    (14, 4) => 1,
    (19, 13) => 15,
    (15, 15) => 5,
    (23, 8) => 29,
    (11, 15) => 36,
    (9, 15) => 34,
    (22, 12) => 28,
    (2, 3) => 8,
    (19, 6) => 14,
    (8, 12) => 31,
    (2, 1) => 8,
    (18, 14) => 11,
    (20, 13) => 22,
    (28, 7) => 43,
    (9, 16) => 34,
    (16, 4) => 7,
    (24, 4) => 32,
    (2, 15) => 8,
    (19, 4) => 15,
    (4, 3) => 13,
    (18, 6) => 11,
    (23, 12) => 30,
    (28, 13) => 44,
    (7, 4) => 25,
    (4, 19) => 13,
    (26, 7) => 40,
    (23, 19) => 29,
    (21, 12) => 24,
    (5, 3) => 16,
    (5, 9) => 17,
    (2, 16) => 8,
    (21, 14) => 23,
    (7, 3) => 25,
    (19, 19) => 15,
    (16, 3) => 7,
    (21, 19) => 23,
    (23, 6) => 29,
    (7, 13) => 25,
    (20, 19) => 20,
    (28, 18) => 43,
    (0, 2) => 0,
    (7, 19) => 25,
    (24, 16) => 33,
    (17, 8) => 10,
    (16, 1) => 6,
    (26, 18) => 39,
    (16, 15) => 7,
    (21, 4) => 23,
    (10, 20) => 35,
    (12, 3) => 41,
    (21, 11) => 24,
    (15, 3) => 4,
    (28, 4) => 43,
    (24, 15) => 33,
    (19, 14) => 14,
    (13, 3) => 45,
    (19, 8) => 14,
    (2, 4) => 8,
    (23, 14) => 29,
    (4, 13) => 13,
    (22, 10) => 26,
    (23, 4) => 29,
    (23, 13) => 29,
    (22, 11) => 27,
    (21, 10) => 24,
    (15, 4) => 5,
    (16, 16) => 7,
    (14, 3) => 2,
    (11, 16) => 37,
    (21, 3) => 23,
    (21, 6) => 23,
    (4, 9) => 13,
    (9, 4) => 34,
    (4, 4) => 13,
    (1, 2) => 3,
    (6, 9) => 18,
    (5, 4) => 16,
    (25, 7) => 38,
    (18, 8) => 12,
    (21, 13) => 23,
    (20, 4) => 19,
    (15, 16) => 5,
    (5, 19) => 16,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    11 => Production::new(11, 18, &vec![]),
    29 => Production::new(29, 23, &vec![]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    6 => Production::new(6, 16, &vec![]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    32 => Production::new(32, 24, &vec![]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    14 => Production::new(14, 19, &vec![]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    23 => Production::new(23, 21, &vec![]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    43 => Production::new(43, 28, &vec![]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    39 => Production::new(39, 26, &vec![]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    12 => String::from("attributes"),
    4 => String::from("block"),
    5 => String::from("alternative"),
    8 => String::from("ebnf_suffix"),
    2 => String::from("rules"),
    7 => String::from("element"),
    13 => String::from("attribute"),
    9 => String::from("lexer_rule"),
    0 => String::from("compilation_unit"),
    11 => String::from("annotation"),
    3 => String::from("parser_rule"),
    6 => String::from("epsilon"),
    1 => String::from("grammar_name"),
    10 => String::from("regular"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    16 => String::from("SHARP"),
    18 => String::from("RBRACKET"),
    12 => String::from("QUESTION"),
    7 => String::from("COMMA"),
    1 => String::from("_STOP"),
    11 => String::from("PLUS"),
    17 => String::from("LBRACKET"),
    20 => String::from("REGULAR_LITERAL"),
    2 => String::from("GRAMMAR"),
    4 => String::from("TOKEN_REF"),
    15 => String::from("AT"),
    3 => String::from("RULE_REF"),
    21 => String::from("WHITE_SPACE"),
    5 => String::from("COLON"),
    0 => String::from("_START"),
    13 => String::from("LPAREN"),
    23 => String::from("BLOCK_COMMENT"),
    6 => String::from("SEMI"),
    10 => String::from("STAR"),
    14 => String::from("RPAREN"),
    19 => String::from("STRING_LITERAL"),
    8 => String::from("OR"),
    22 => String::from("LINE_COMMENT"),
    9 => String::from("EPSILON"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (5, 14),
    (20, 12),
    (18, 6),
    (28, 4),
    (23, 14),
    (23, 4),
    (1, 3),
    (27, 18),
    (15, 15),
    (25, 18),
    (23, 13),
    (8, 13),
    (22, 12),
    (9, 3),
    (20, 6),
    (7, 6),
    (19, 14),
    (8, 19),
    (14, 6),
    (28, 18),
    (1, 16),
    (6, 6),
    (21, 4),
    (17, 8),
    (4, 14),
    (3, 4),
    (25, 7),
    (7, 14),
    (17, 6),
    (9, 16),
    (7, 4),
    (1, 4),
    (9, 1),
    (12, 18),
    (20, 14),
    (16, 1),
    (5, 8),
    (15, 4),
    (17, 14),
    (8, 6),
    (13, 18),
    (15, 16),
    (1, 1),
    (19, 8),
    (24, 4),
    (23, 19),
    (22, 3),
    (22, 8),
    (6, 8),
    (28, 7),
    (22, 19),
    (20, 3),
    (22, 14),
    (9, 4),
    (3, 15),
    (7, 3),
    (26, 18),
    (20, 13),
    (22, 6),
    (23, 6),
    (9, 15),
    (27, 7),
    (8, 14),
    (20, 19),
    (20, 4),
    (7, 8),
    (15, 3),
    (15, 1),
    (20, 10),
    (21, 19),
    (21, 13),
    (11, 4),
    (21, 6),
    (20, 11),
    (5, 6),
    (19, 6),
    (6, 14),
    (4, 6),
    (23, 3),
    (21, 8),
    (8, 8),
    (7, 13),
    (2, 1),
    (18, 14),
    (20, 8),
    (21, 14),
    (1, 15),
    (7, 19),
    (10, 6),
    (21, 3),
    (27, 4),
    (13, 4),
    (23, 8),
    (0, 1),
    (3, 16),
    (3, 1),
    (3, 3),
    (8, 3),
    (22, 13),
    (13, 7),
    (22, 4),
    (8, 4),
  }
});


#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ATTRIBUTES: usize = 12; 
  pub const BLOCK: usize = 4; 
  pub const ALTERNATIVE: usize = 5; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const RULES: usize = 2; 
  pub const ELEMENT: usize = 7; 
  pub const ATTRIBUTE: usize = 13; 
  pub const LEXER_RULE: usize = 9; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const ANNOTATION: usize = 11; 
  pub const PARSER_RULE: usize = 3; 
  pub const EPSILON: usize = 6; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const REGULAR: usize = 10; 



  pub fn new() -> Self {
    Self {
      error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
    }
  }


  // 使用模板生成
  
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributesContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::ATTRIBUTES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Result<Box<dyn BlockContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::BLOCK,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AlternativeContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::ALTERNATIVE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EbnfSuffixContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::EBNF_SUFFIX,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RulesContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::RULES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ElementContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::ELEMENT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributeContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::ATTRIBUTE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn LexerRuleContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::LEXER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Result<Box<dyn CompilationUnitContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::COMPILATION_UNIT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AnnotationContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::ANNOTATION,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ParserRuleContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::PARSER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EpsilonContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::EPSILON,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Result<Box<dyn GrammarNameContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::GRAMMAR_NAME,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RegularContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::REGULAR,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 

}






