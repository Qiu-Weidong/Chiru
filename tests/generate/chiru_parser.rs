



// generated from chiru.chiru by chiru 0.7.0



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
   BlockContext, AnnotationContext, CompilationUnitContext, GrammarNameContext, EbnfSuffixContext, ParserRuleContext, AttributesContext, AttributeContext, LexerRuleContext, RulesContext, RegularContext, AlternativeContext, EpsilonContext, ElementContext,
};


pub struct ChiruParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (18, 6) => 11,
    (28, 7) => 43,
    (0, 2) => 0,
    (15, 16) => 5,
    (28, 18) => 43,
    (21, 3) => 23,
    (7, 4) => 25,
    (28, 4) => 43,
    (8, 12) => 31,
    (18, 8) => 12,
    (21, 8) => 23,
    (16, 4) => 7,
    (7, 19) => 25,
    (21, 6) => 23,
    (16, 16) => 7,
    (23, 12) => 30,
    (9, 16) => 34,
    (19, 6) => 14,
    (11, 16) => 37,
    (21, 10) => 24,
    (27, 13) => 42,
    (19, 3) => 15,
    (9, 4) => 34,
    (2, 3) => 8,
    (22, 11) => 27,
    (19, 14) => 14,
    (2, 16) => 8,
    (6, 9) => 18,
    (2, 4) => 8,
    (4, 3) => 13,
    (21, 11) => 24,
    (22, 10) => 26,
    (12, 3) => 41,
    (21, 12) => 24,
    (11, 15) => 36,
    (15, 15) => 5,
    (24, 16) => 33,
    (14, 4) => 1,
    (19, 8) => 14,
    (7, 3) => 25,
    (17, 8) => 10,
    (8, 11) => 31,
    (21, 4) => 23,
    (5, 13) => 16,
    (16, 15) => 7,
    (22, 12) => 28,
    (26, 7) => 40,
    (15, 4) => 5,
    (10, 20) => 35,
    (2, 1) => 8,
    (14, 3) => 2,
    (4, 19) => 13,
    (4, 9) => 13,
    (5, 9) => 17,
    (20, 4) => 19,
    (18, 14) => 11,
    (16, 3) => 7,
    (2, 15) => 8,
    (8, 10) => 31,
    (16, 1) => 6,
    (15, 3) => 4,
    (19, 13) => 15,
    (19, 19) => 15,
    (23, 6) => 29,
    (23, 3) => 29,
    (23, 8) => 29,
    (25, 7) => 38,
    (23, 13) => 29,
    (24, 15) => 33,
    (24, 4) => 32,
    (20, 13) => 22,
    (4, 4) => 13,
    (28, 13) => 44,
    (9, 15) => 34,
    (26, 18) => 39,
    (19, 4) => 15,
    (5, 3) => 16,
    (5, 4) => 16,
    (13, 3) => 45,
    (21, 13) => 23,
    (1, 2) => 3,
    (20, 3) => 21,
    (3, 3) => 9,
    (20, 19) => 20,
    (4, 13) => 13,
    (23, 19) => 29,
    (21, 19) => 23,
    (7, 13) => 25,
    (23, 4) => 29,
    (5, 19) => 16,
    (21, 14) => 23,
    (23, 14) => 29,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    14 => Production::new(14, 19, &vec![]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    39 => Production::new(39, 26, &vec![]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    43 => Production::new(43, 28, &vec![]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    11 => Production::new(11, 18, &vec![]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    23 => Production::new(23, 21, &vec![]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    6 => Production::new(6, 16, &vec![]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    29 => Production::new(29, 23, &vec![]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    32 => Production::new(32, 24, &vec![]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    4 => String::from("block"),
    11 => String::from("annotation"),
    0 => String::from("compilation_unit"),
    1 => String::from("grammar_name"),
    8 => String::from("ebnf_suffix"),
    3 => String::from("parser_rule"),
    12 => String::from("attributes"),
    13 => String::from("attribute"),
    9 => String::from("lexer_rule"),
    2 => String::from("rules"),
    10 => String::from("regular"),
    5 => String::from("alternative"),
    6 => String::from("epsilon"),
    7 => String::from("element"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    22 => String::from("LINE_COMMENT"),
    12 => String::from("QUESTION"),
    2 => String::from("GRAMMAR"),
    19 => String::from("STRING_LITERAL"),
    20 => String::from("REGULAR_LITERAL"),
    7 => String::from("COMMA"),
    18 => String::from("RBRACKET"),
    1 => String::from("_STOP"),
    4 => String::from("TOKEN_REF"),
    9 => String::from("EPSILON"),
    21 => String::from("WHITE_SPACE"),
    15 => String::from("AT"),
    5 => String::from("COLON"),
    6 => String::from("SEMI"),
    8 => String::from("OR"),
    13 => String::from("LPAREN"),
    3 => String::from("RULE_REF"),
    14 => String::from("RPAREN"),
    16 => String::from("SHARP"),
    17 => String::from("LBRACKET"),
    0 => String::from("_START"),
    11 => String::from("PLUS"),
    10 => String::from("STAR"),
    23 => String::from("BLOCK_COMMENT"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (21, 8),
    (1, 16),
    (21, 13),
    (9, 16),
    (23, 8),
    (18, 6),
    (6, 8),
    (4, 6),
    (18, 14),
    (1, 1),
    (23, 3),
    (1, 15),
    (8, 14),
    (23, 19),
    (8, 8),
    (5, 6),
    (27, 18),
    (20, 3),
    (7, 4),
    (25, 18),
    (7, 3),
    (7, 13),
    (28, 18),
    (20, 8),
    (19, 6),
    (8, 6),
    (21, 6),
    (13, 7),
    (23, 6),
    (24, 4),
    (17, 8),
    (28, 7),
    (6, 14),
    (10, 6),
    (22, 14),
    (20, 6),
    (15, 4),
    (22, 3),
    (23, 4),
    (26, 18),
    (20, 13),
    (7, 19),
    (11, 4),
    (21, 19),
    (6, 6),
    (9, 3),
    (9, 1),
    (1, 4),
    (8, 3),
    (20, 12),
    (20, 14),
    (3, 1),
    (22, 19),
    (1, 3),
    (21, 3),
    (21, 14),
    (3, 3),
    (27, 7),
    (16, 1),
    (7, 14),
    (23, 14),
    (17, 14),
    (9, 15),
    (4, 14),
    (7, 6),
    (12, 18),
    (14, 6),
    (15, 15),
    (27, 4),
    (22, 13),
    (13, 18),
    (20, 11),
    (3, 4),
    (28, 4),
    (20, 10),
    (19, 14),
    (25, 7),
    (20, 19),
    (19, 8),
    (22, 4),
    (22, 12),
    (2, 1),
    (22, 6),
    (13, 4),
    (23, 13),
    (8, 19),
    (15, 1),
    (15, 3),
    (7, 8),
    (3, 15),
    (9, 4),
    (3, 16),
    (22, 8),
    (5, 14),
    (5, 8),
    (8, 4),
    (21, 4),
    (20, 4),
    (0, 1),
    (15, 16),
    (17, 6),
    (8, 13),
  }
});


#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const BLOCK: usize = 4; 
  pub const ANNOTATION: usize = 11; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const PARSER_RULE: usize = 3; 
  pub const ATTRIBUTES: usize = 12; 
  pub const ATTRIBUTE: usize = 13; 
  pub const LEXER_RULE: usize = 9; 
  pub const RULES: usize = 2; 
  pub const REGULAR: usize = 10; 
  pub const ALTERNATIVE: usize = 5; 
  pub const EPSILON: usize = 6; 
  pub const ELEMENT: usize = 7; 



  pub fn new() -> Self {
    Self {
      error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
    }
  }


  // 使用模板生成
  
  pub fn block(&self, token_stream: &mut TokenStream) -> Result<Box<dyn BlockContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::BLOCK,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AnnotationContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ANNOTATION,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Result<Box<dyn CompilationUnitContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::COMPILATION_UNIT,
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
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributesContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ATTRIBUTES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributeContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ATTRIBUTE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn LexerRuleContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::LEXER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RulesContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::RULES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RegularContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::REGULAR,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AlternativeContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ALTERNATIVE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EpsilonContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::EPSILON,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ElementContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ELEMENT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 

}






