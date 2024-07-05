



// generated from .\src\tool\syntaxis\chiru.chiru by chiru 0.7.0
 


use std::error::Error;
use std::collections::{HashMap, HashSet};

use chiru::runtime::error_strategy::error_listener::ErrorListener;
use chiru::runtime::ll1_analyzer::ll1_analyze;

use chiru::maplit::hashmap;
use chiru::maplit::hashset;
use chiru::once_cell::sync::Lazy;

use chiru::runtime::vocabulary::Vocabulary;
use chiru::runtime::{
  token_stream::TokenStream, 
  error_strategy::error_listener::ConsoleErrorListener,
  production::Production,
  production::ProductionItem
};

use super::chiru_context::{
   BlockContext, AnnotationContext, LexerRuleContext, ParserRuleContext, RulesContext, AttributeContext, GrammarNameContext, RegularContext, ElementContext, AlternativeContext, EpsilonContext, EbnfSuffixContext, AttributesContext, CompilationUnitContext,
};


pub struct ChiruParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (23, 6) => 29,
    (14, 4) => 1,
    (16, 4) => 7,
    (19, 6) => 14,
    (15, 15) => 5,
    (18, 14) => 11,
    (23, 1) => 29,
    (21, 1) => 23,
    (19, 13) => 15,
    (19, 4) => 15,
    (19, 19) => 15,
    (7, 4) => 25,
    (11, 16) => 37,
    (16, 3) => 7,
    (19, 8) => 14,
    (5, 3) => 16,
    (26, 1) => 39,
    (5, 13) => 16,
    (5, 19) => 16,
    (23, 3) => 29,
    (23, 4) => 29,
    (8, 10) => 31,
    (23, 12) => 30,
    (26, 7) => 40,
    (16, 16) => 7,
    (28, 7) => 43,
    (15, 4) => 5,
    (9, 15) => 34,
    (9, 16) => 34,
    (8, 11) => 31,
    (21, 14) => 23,
    (21, 19) => 23,
    (20, 4) => 19,
    (18, 8) => 12,
    (19, 14) => 14,
    (22, 10) => 26,
    (4, 4) => 13,
    (24, 15) => 33,
    (7, 3) => 25,
    (28, 13) => 44,
    (2, 1) => 8,
    (21, 8) => 23,
    (2, 15) => 8,
    (2, 4) => 8,
    (14, 3) => 2,
    (21, 4) => 23,
    (25, 7) => 38,
    (23, 19) => 29,
    (21, 10) => 24,
    (7, 19) => 25,
    (6, 9) => 18,
    (10, 20) => 35,
    (4, 3) => 13,
    (21, 6) => 23,
    (26, 18) => 39,
    (0, 2) => 0,
    (21, 11) => 24,
    (2, 3) => 8,
    (23, 14) => 29,
    (12, 3) => 41,
    (19, 3) => 15,
    (20, 19) => 20,
    (15, 16) => 5,
    (20, 3) => 21,
    (1, 2) => 3,
    (20, 13) => 22,
    (13, 3) => 45,
    (22, 11) => 27,
    (9, 4) => 34,
    (8, 12) => 31,
    (18, 6) => 11,
    (5, 4) => 16,
    (5, 9) => 17,
    (23, 13) => 29,
    (27, 13) => 42,
    (28, 1) => 43,
    (17, 8) => 10,
    (16, 1) => 6,
    (21, 13) => 23,
    (11, 15) => 36,
    (7, 13) => 25,
    (4, 13) => 13,
    (15, 3) => 4,
    (4, 19) => 13,
    (2, 16) => 8,
    (28, 18) => 43,
    (4, 9) => 13,
    (19, 1) => 14,
    (24, 16) => 33,
    (16, 15) => 7,
    (21, 12) => 24,
    (18, 1) => 11,
    (3, 3) => 9,
    (21, 3) => 23,
    (22, 12) => 28,
    (24, 1) => 32,
    (23, 8) => 29,
    (24, 4) => 32,
    (28, 4) => 43,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    11 => Production::new(11, 18, &vec![]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    23 => Production::new(23, 21, &vec![]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    29 => Production::new(29, 23, &vec![]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    32 => Production::new(32, 24, &vec![]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    43 => Production::new(43, 28, &vec![]),
    14 => Production::new(14, 19, &vec![]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    6 => Production::new(6, 16, &vec![]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    39 => Production::new(39, 26, &vec![]),
  }
}); 

// 删掉终结符和非终结符这两个全局静态变量，改用 Vocabulary 管理
// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    4 => String::from("block"),
    11 => String::from("annotation"),
    9 => String::from("lexer_rule"),
    3 => String::from("parser_rule"),
    2 => String::from("rules"),
    13 => String::from("attribute"),
    1 => String::from("grammar_name"),
    10 => String::from("regular"),
    7 => String::from("element"),
    5 => String::from("alternative"),
    6 => String::from("epsilon"),
    8 => String::from("ebnf_suffix"),
    12 => String::from("attributes"),
    0 => String::from("compilation_unit"),
  }
});




pub static VOCABULARY: Lazy<Vocabulary> = Lazy::new(|| {
  let mut result = Vocabulary::new();

  // 添加匿名非终结符
  result.add_unnamed_nonterminals(&vec![7, 17, 13, 4, 5, 6, 10, 1, 3, 16, 18, 11, 9, 15, 0, 8, 27, 21, 19, 26, 28, 24, 23, 22, 14, 20, 12, 2, 25]);

  // 添加命名非终结符
  result.add_named_nonterminal(4, "block");
  result.add_named_nonterminal(13, "attribute");
  result.add_named_nonterminal(9, "lexer_rule");
  result.add_named_nonterminal(0, "compilation_unit");
  result.add_named_nonterminal(11, "annotation");
  result.add_named_nonterminal(7, "element");
  result.add_named_nonterminal(1, "grammar_name");
  result.add_named_nonterminal(5, "alternative");
  result.add_named_nonterminal(12, "attributes");
  result.add_named_nonterminal(2, "rules");
  result.add_named_nonterminal(10, "regular");
  result.add_named_nonterminal(3, "parser_rule");
  result.add_named_nonterminal(6, "epsilon");
  result.add_named_nonterminal(8, "ebnf_suffix");

  // 添加终结符
  
  result.add_terminal(10 ,"STAR");
  result.add_terminal(13 ,"LPAREN");
  result.add_terminal(6 ,"SEMI");
  result.add_terminal(0 ,"_START");
  result.add_terminal(5 ,"COLON");
  result.add_terminal(7 ,"COMMA");
  result.add_terminal(9 ,"EPSILON");
  result.add_terminal(20 ,"REGULAR_LITERAL");
  result.add_terminal(2 ,"GRAMMAR");
  result.add_terminal(15 ,"AT");
  result.add_terminal(18 ,"RBRACKET");
  result.add_terminal(4 ,"TOKEN_REF");
  result.add_terminal(22 ,"LINE_COMMENT");
  result.add_terminal(21 ,"WHITE_SPACE");
  result.add_terminal(1 ,"_STOP");
  result.add_terminal(17 ,"LBRACKET");
  result.add_terminal(23 ,"BLOCK_COMMENT");
  result.add_terminal(3 ,"RULE_REF");
  result.add_terminal(14 ,"RPAREN");
  result.add_terminal(19 ,"STRING_LITERAL");
  result.add_terminal(11 ,"PLUS");
  result.add_terminal(16 ,"SHARP");
  result.add_terminal(8 ,"OR");
  result.add_terminal(12 ,"QUESTION");


  result
});



pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (3, 16),
    (22, 19),
    (22, 12),
    (25, 1),
    (22, 4),
    (21, 6),
    (14, 1),
    (23, 19),
    (18, 14),
    (23, 6),
    (13, 4),
    (19, 1),
    (21, 3),
    (0, 1),
    (1, 4),
    (18, 1),
    (19, 14),
    (22, 13),
    (3, 3),
    (28, 1),
    (20, 14),
    (23, 8),
    (17, 8),
    (26, 18),
    (14, 6),
    (11, 4),
    (27, 4),
    (28, 4),
    (25, 7),
    (5, 8),
    (22, 8),
    (23, 4),
    (21, 1),
    (11, 1),
    (12, 1),
    (3, 15),
    (8, 3),
    (21, 14),
    (7, 6),
    (20, 12),
    (23, 1),
    (3, 4),
    (1, 15),
    (5, 6),
    (27, 1),
    (8, 8),
    (20, 6),
    (21, 8),
    (6, 8),
    (6, 6),
    (9, 15),
    (16, 1),
    (9, 16),
    (13, 7),
    (4, 1),
    (15, 15),
    (5, 1),
    (20, 3),
    (20, 13),
    (3, 1),
    (17, 6),
    (10, 1),
    (20, 10),
    (8, 4),
    (6, 1),
    (26, 1),
    (5, 14),
    (13, 1),
    (21, 13),
    (8, 14),
    (1, 16),
    (13, 18),
    (4, 6),
    (20, 11),
    (15, 1),
    (7, 8),
    (8, 1),
    (24, 4),
    (20, 4),
    (2, 1),
    (28, 18),
    (17, 14),
    (1, 3),
    (15, 16),
    (7, 4),
    (7, 3),
    (21, 19),
    (22, 14),
    (9, 3),
    (22, 6),
    (18, 6),
    (9, 4),
    (20, 1),
    (28, 7),
    (7, 13),
    (20, 8),
    (8, 19),
    (8, 13),
    (6, 14),
    (19, 8),
    (23, 14),
    (21, 4),
    (19, 6),
    (12, 18),
    (20, 19),
    (15, 4),
    (7, 1),
    (22, 3),
    (7, 19),
    (8, 6),
    (23, 3),
    (9, 1),
    (1, 1),
    (10, 6),
    (27, 18),
    (17, 1),
    (7, 14),
    (23, 13),
    (22, 1),
    (27, 7),
    (25, 18),
    (15, 3),
    (24, 1),
    (4, 14),
  }
});


#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const BLOCK: usize = 4; 
  pub const ANNOTATION: usize = 11; 
  pub const LEXER_RULE: usize = 9; 
  pub const PARSER_RULE: usize = 3; 
  pub const RULES: usize = 2; 
  pub const ATTRIBUTE: usize = 13; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const REGULAR: usize = 10; 
  pub const ELEMENT: usize = 7; 
  pub const ALTERNATIVE: usize = 5; 
  pub const EPSILON: usize = 6; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const ATTRIBUTES: usize = 12; 
  pub const COMPILATION_UNIT: usize = 0; 



  pub fn new() -> Self {
    Self {
      error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
    }
  }


  // 使用模板生成
  
  pub fn block(&self, token_stream: &mut TokenStream) -> Result<Box<dyn BlockContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::BLOCK,
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
  pub fn lexer_rule(&self, token_stream: &mut TokenStream) -> Result<Box<dyn LexerRuleContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::LEXER_RULE,
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
  pub fn rules(&self, token_stream: &mut TokenStream) -> Result<Box<dyn RulesContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::RULES,
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
  pub fn element(&self, token_stream: &mut TokenStream) -> Result<Box<dyn ElementContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::ELEMENT,
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
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Result<Box<dyn EpsilonContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::EPSILON,
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
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Result<Box<dyn AttributesContext>, Box<dyn Error>> {

    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }
    
    let result = ll1_analyze(token_stream, Self::ATTRIBUTES,
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

}






