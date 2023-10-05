



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
   EbnfSuffixContext, GrammarNameContext, ElementContext, RulesContext, EpsilonContext, LexerRuleContext, AlternativeContext, ParserRuleContext, AnnotationContext, AttributesContext, BlockContext, AttributeContext, CompilationUnitContext, RegularContext,
};


pub struct ChiruParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (21, 19) => 23,
    (22, 10) => 26,
    (23, 4) => 29,
    (9, 4) => 34,
    (23, 12) => 30,
    (7, 3) => 25,
    (9, 16) => 34,
    (26, 18) => 39,
    (20, 19) => 20,
    (15, 4) => 5,
    (4, 3) => 13,
    (5, 4) => 16,
    (16, 1) => 6,
    (20, 13) => 22,
    (2, 16) => 8,
    (5, 13) => 16,
    (16, 3) => 7,
    (22, 11) => 27,
    (2, 4) => 8,
    (16, 4) => 7,
    (2, 15) => 8,
    (11, 15) => 36,
    (21, 11) => 24,
    (18, 8) => 12,
    (21, 3) => 23,
    (21, 6) => 23,
    (8, 12) => 31,
    (23, 8) => 29,
    (5, 3) => 16,
    (24, 15) => 33,
    (15, 3) => 4,
    (3, 3) => 9,
    (16, 15) => 7,
    (4, 13) => 13,
    (4, 9) => 13,
    (15, 16) => 5,
    (23, 14) => 29,
    (21, 13) => 23,
    (26, 7) => 40,
    (8, 10) => 31,
    (5, 19) => 16,
    (11, 16) => 37,
    (0, 2) => 0,
    (5, 9) => 17,
    (2, 3) => 8,
    (4, 4) => 13,
    (23, 19) => 29,
    (14, 4) => 1,
    (4, 19) => 13,
    (24, 4) => 32,
    (20, 3) => 21,
    (23, 6) => 29,
    (7, 4) => 25,
    (12, 3) => 41,
    (7, 13) => 25,
    (21, 14) => 23,
    (24, 16) => 33,
    (20, 4) => 19,
    (28, 18) => 43,
    (28, 13) => 44,
    (18, 6) => 11,
    (16, 16) => 7,
    (21, 12) => 24,
    (28, 4) => 43,
    (19, 6) => 14,
    (27, 13) => 42,
    (21, 10) => 24,
    (23, 3) => 29,
    (17, 8) => 10,
    (25, 7) => 38,
    (19, 14) => 14,
    (1, 2) => 3,
    (13, 3) => 45,
    (7, 19) => 25,
    (19, 4) => 15,
    (19, 8) => 14,
    (9, 15) => 34,
    (19, 3) => 15,
    (19, 19) => 15,
    (22, 12) => 28,
    (23, 13) => 29,
    (21, 8) => 23,
    (14, 3) => 2,
    (18, 14) => 11,
    (10, 20) => 35,
    (8, 11) => 31,
    (19, 13) => 15,
    (15, 15) => 5,
    (2, 1) => 8,
    (6, 9) => 18,
    (21, 4) => 23,
    (28, 7) => 43,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    6 => Production::new(6, 16, &vec![]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    14 => Production::new(14, 19, &vec![]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    32 => Production::new(32, 24, &vec![]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    23 => Production::new(23, 21, &vec![]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    29 => Production::new(29, 23, &vec![]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    39 => Production::new(39, 26, &vec![]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    43 => Production::new(43, 28, &vec![]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    11 => Production::new(11, 18, &vec![]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    8 => String::from("ebnf_suffix"),
    1 => String::from("grammar_name"),
    7 => String::from("element"),
    2 => String::from("rules"),
    6 => String::from("epsilon"),
    9 => String::from("lexer_rule"),
    5 => String::from("alternative"),
    3 => String::from("parser_rule"),
    11 => String::from("annotation"),
    12 => String::from("attributes"),
    4 => String::from("block"),
    13 => String::from("attribute"),
    0 => String::from("compilation_unit"),
    10 => String::from("regular"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    22 => String::from("LINE_COMMENT"),
    6 => String::from("SEMI"),
    2 => String::from("GRAMMAR"),
    11 => String::from("PLUS"),
    0 => String::from("_START"),
    15 => String::from("AT"),
    13 => String::from("LPAREN"),
    17 => String::from("LBRACKET"),
    20 => String::from("REGULAR_LITERAL"),
    9 => String::from("EPSILON"),
    12 => String::from("QUESTION"),
    14 => String::from("RPAREN"),
    3 => String::from("RULE_REF"),
    19 => String::from("STRING_LITERAL"),
    7 => String::from("COMMA"),
    1 => String::from("_STOP"),
    8 => String::from("OR"),
    5 => String::from("COLON"),
    16 => String::from("SHARP"),
    23 => String::from("BLOCK_COMMENT"),
    18 => String::from("RBRACKET"),
    21 => String::from("WHITE_SPACE"),
    4 => String::from("TOKEN_REF"),
    10 => String::from("STAR"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (9, 16),
    (23, 13),
    (1, 15),
    (17, 8),
    (3, 1),
    (15, 4),
    (5, 8),
    (22, 19),
    (13, 18),
    (13, 7),
    (20, 14),
    (8, 4),
    (23, 4),
    (23, 3),
    (5, 6),
    (20, 10),
    (28, 7),
    (4, 14),
    (9, 3),
    (21, 13),
    (20, 13),
    (0, 1),
    (7, 13),
    (1, 3),
    (1, 16),
    (21, 14),
    (6, 6),
    (9, 4),
    (20, 19),
    (22, 12),
    (8, 14),
    (22, 3),
    (5, 14),
    (20, 3),
    (9, 1),
    (23, 8),
    (8, 13),
    (8, 3),
    (15, 15),
    (7, 14),
    (23, 14),
    (27, 18),
    (4, 6),
    (7, 3),
    (13, 4),
    (22, 4),
    (18, 6),
    (7, 4),
    (8, 8),
    (8, 19),
    (21, 6),
    (3, 4),
    (3, 3),
    (7, 6),
    (7, 8),
    (18, 14),
    (16, 1),
    (27, 7),
    (24, 4),
    (15, 16),
    (20, 11),
    (1, 4),
    (8, 6),
    (2, 1),
    (6, 8),
    (3, 15),
    (21, 3),
    (25, 18),
    (21, 19),
    (19, 14),
    (19, 6),
    (27, 4),
    (20, 8),
    (17, 6),
    (28, 18),
    (9, 15),
    (14, 6),
    (20, 12),
    (23, 6),
    (22, 14),
    (21, 4),
    (25, 7),
    (15, 3),
    (6, 14),
    (20, 4),
    (10, 6),
    (17, 14),
    (26, 18),
    (22, 13),
    (15, 1),
    (21, 8),
    (1, 1),
    (22, 8),
    (22, 6),
    (12, 18),
    (23, 19),
    (19, 8),
    (3, 16),
    (20, 6),
    (11, 4),
    (28, 4),
    (7, 19),
  }
});


#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const EBNF_SUFFIX: usize = 8; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const ELEMENT: usize = 7; 
  pub const RULES: usize = 2; 
  pub const EPSILON: usize = 6; 
  pub const LEXER_RULE: usize = 9; 
  pub const ALTERNATIVE: usize = 5; 
  pub const PARSER_RULE: usize = 3; 
  pub const ANNOTATION: usize = 11; 
  pub const ATTRIBUTES: usize = 12; 
  pub const BLOCK: usize = 4; 
  pub const ATTRIBUTE: usize = 13; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const REGULAR: usize = 10; 



  pub fn new() -> Self {
    Self {
      error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
    }
  }


  // 使用模板生成
  
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EbnfSuffixContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::EBNF_SUFFIX,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Result<Box<dyn GrammarNameContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::GRAMMAR_NAME,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ElementContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ELEMENT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RulesContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::RULES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EpsilonContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::EPSILON,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn LexerRuleContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::LEXER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AlternativeContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ALTERNATIVE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ParserRuleContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::PARSER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AnnotationContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ANNOTATION,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributesContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ATTRIBUTES,
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
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Result<Box<dyn CompilationUnitContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::COMPILATION_UNIT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RegularContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::REGULAR,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 

}






