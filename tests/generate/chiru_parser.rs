
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
   AnnotationContext, ParserRuleContext, RegularContext, LexerRuleContext, EpsilonContext, GrammarNameContext, AlternativeContext, AttributesContext, RulesContext, EbnfSuffixContext, ElementContext, CompilationUnitContext, BlockContext, AttributeContext,
};


pub struct ChiruParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (8, 11) => 31,
    (2, 4) => 8,
    (25, 7) => 38,
    (23, 14) => 29,
    (24, 4) => 32,
    (20, 4) => 19,
    (15, 16) => 5,
    (13, 3) => 45,
    (5, 4) => 16,
    (10, 20) => 35,
    (5, 19) => 16,
    (21, 11) => 24,
    (16, 15) => 7,
    (14, 3) => 2,
    (23, 8) => 29,
    (11, 15) => 36,
    (22, 12) => 28,
    (8, 10) => 31,
    (15, 15) => 5,
    (21, 14) => 23,
    (23, 19) => 29,
    (4, 13) => 13,
    (19, 13) => 15,
    (28, 4) => 43,
    (19, 3) => 15,
    (2, 1) => 8,
    (20, 13) => 22,
    (1, 2) => 3,
    (23, 6) => 29,
    (16, 3) => 7,
    (16, 4) => 7,
    (2, 16) => 8,
    (22, 11) => 27,
    (17, 8) => 10,
    (28, 13) => 44,
    (23, 13) => 29,
    (26, 18) => 39,
    (19, 14) => 14,
    (5, 13) => 16,
    (21, 12) => 24,
    (28, 7) => 43,
    (19, 8) => 14,
    (4, 19) => 13,
    (23, 4) => 29,
    (7, 13) => 25,
    (28, 18) => 43,
    (18, 14) => 11,
    (21, 19) => 23,
    (27, 13) => 42,
    (7, 19) => 25,
    (0, 2) => 0,
    (5, 9) => 17,
    (19, 19) => 15,
    (7, 3) => 25,
    (24, 16) => 33,
    (5, 3) => 16,
    (21, 8) => 23,
    (15, 3) => 4,
    (8, 12) => 31,
    (18, 8) => 12,
    (4, 3) => 13,
    (2, 3) => 8,
    (9, 15) => 34,
    (19, 4) => 15,
    (21, 10) => 24,
    (21, 4) => 23,
    (26, 7) => 40,
    (3, 3) => 9,
    (20, 19) => 20,
    (23, 12) => 30,
    (14, 4) => 1,
    (19, 6) => 14,
    (6, 9) => 18,
    (12, 3) => 41,
    (9, 4) => 34,
    (2, 15) => 8,
    (16, 1) => 6,
    (21, 6) => 23,
    (7, 4) => 25,
    (9, 16) => 34,
    (23, 3) => 29,
    (4, 9) => 13,
    (21, 3) => 23,
    (4, 4) => 13,
    (11, 16) => 37,
    (21, 13) => 23,
    (15, 4) => 5,
    (18, 6) => 11,
    (16, 16) => 7,
    (24, 15) => 33,
    (20, 3) => 21,
    (22, 10) => 26,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    11 => Production::new(11, 18, &vec![]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    6 => Production::new(6, 16, &vec![]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    43 => Production::new(43, 28, &vec![]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    14 => Production::new(14, 19, &vec![]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    23 => Production::new(23, 21, &vec![]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    29 => Production::new(29, 23, &vec![]),
    32 => Production::new(32, 24, &vec![]),
    39 => Production::new(39, 26, &vec![]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    11 => String::from("annotation"),
    3 => String::from("parser_rule"),
    10 => String::from("regular"),
    9 => String::from("lexer_rule"),
    6 => String::from("epsilon"),
    1 => String::from("grammar_name"),
    5 => String::from("alternative"),
    12 => String::from("attributes"),
    2 => String::from("rules"),
    8 => String::from("ebnf_suffix"),
    7 => String::from("element"),
    0 => String::from("compilation_unit"),
    4 => String::from("block"),
    13 => String::from("attribute"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    5 => String::from("COLON"),
    16 => String::from("SHARP"),
    17 => String::from("LBRACKET"),
    19 => String::from("STRING_LITERAL"),
    1 => String::from("_STOP"),
    8 => String::from("OR"),
    15 => String::from("AT"),
    10 => String::from("STAR"),
    3 => String::from("RULE_REF"),
    9 => String::from("EPSILON"),
    11 => String::from("PLUS"),
    0 => String::from("_START"),
    22 => String::from("LINE_COMMENT"),
    13 => String::from("LPAREN"),
    14 => String::from("RPAREN"),
    4 => String::from("TOKEN_REF"),
    12 => String::from("QUESTION"),
    7 => String::from("COMMA"),
    18 => String::from("RBRACKET"),
    23 => String::from("BLOCK_COMMENT"),
    2 => String::from("GRAMMAR"),
    21 => String::from("WHITE_SPACE"),
    6 => String::from("SEMI"),
    20 => String::from("REGULAR_LITERAL"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (1, 16),
    (15, 4),
    (15, 3),
    (6, 14),
    (20, 13),
    (20, 6),
    (20, 4),
    (20, 10),
    (3, 4),
    (23, 14),
    (20, 3),
    (0, 1),
    (13, 18),
    (20, 8),
    (20, 19),
    (7, 19),
    (7, 13),
    (23, 4),
    (24, 4),
    (27, 18),
    (16, 1),
    (21, 6),
    (6, 8),
    (12, 18),
    (7, 6),
    (15, 1),
    (22, 8),
    (8, 19),
    (28, 18),
    (28, 4),
    (1, 1),
    (22, 13),
    (10, 6),
    (20, 12),
    (19, 14),
    (25, 7),
    (22, 12),
    (15, 15),
    (23, 13),
    (17, 8),
    (23, 6),
    (18, 14),
    (27, 7),
    (20, 11),
    (19, 6),
    (21, 4),
    (8, 8),
    (22, 4),
    (19, 8),
    (8, 13),
    (1, 4),
    (22, 19),
    (18, 6),
    (4, 14),
    (9, 4),
    (9, 16),
    (11, 4),
    (14, 6),
    (2, 1),
    (25, 18),
    (13, 4),
    (22, 6),
    (23, 8),
    (23, 3),
    (8, 6),
    (9, 1),
    (17, 14),
    (20, 14),
    (17, 6),
    (8, 14),
    (26, 18),
    (23, 19),
    (21, 13),
    (7, 8),
    (21, 3),
    (7, 14),
    (13, 7),
    (15, 16),
    (6, 6),
    (22, 14),
    (27, 4),
    (3, 1),
    (5, 6),
    (7, 4),
    (3, 3),
    (1, 3),
    (3, 15),
    (7, 3),
    (9, 3),
    (8, 3),
    (22, 3),
    (21, 8),
    (1, 15),
    (21, 14),
    (5, 14),
    (9, 15),
    (3, 16),
    (8, 4),
    (28, 7),
    (21, 19),
    (4, 6),
    (5, 8),
  }
});


#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const ANNOTATION: usize = 11; 
  pub const PARSER_RULE: usize = 3; 
  pub const REGULAR: usize = 10; 
  pub const LEXER_RULE: usize = 9; 
  pub const EPSILON: usize = 6; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const ALTERNATIVE: usize = 5; 
  pub const ATTRIBUTES: usize = 12; 
  pub const RULES: usize = 2; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const ELEMENT: usize = 7; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const BLOCK: usize = 4; 
  pub const ATTRIBUTE: usize = 13; 



  pub fn new() -> Self {
    Self {
      error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
    }
  }


  // 使用模板生成
  
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AnnotationContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ANNOTATION,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ParserRuleContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::PARSER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RegularContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::REGULAR,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn LexerRuleContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::LEXER_RULE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EpsilonContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::EPSILON,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Result<Box<dyn GrammarNameContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::GRAMMAR_NAME,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AlternativeContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ALTERNATIVE,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributesContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ATTRIBUTES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RulesContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::RULES,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EbnfSuffixContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::EBNF_SUFFIX,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn element(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ElementContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::ELEMENT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Result<Box<dyn CompilationUnitContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::COMPILATION_UNIT,
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

}






