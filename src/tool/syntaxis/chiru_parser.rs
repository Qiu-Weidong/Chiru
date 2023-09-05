
use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;
use lazy_static::lazy_static;

use chiru::runtime::production::Production;
use chiru::runtime::production::ProductionItem;
use chiru::runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener};

use super::chiru_context::{
   EbnfSuffixContext, ParserRuleContext, AttributesContext, BlockContext, LexerRuleContext, AttributeContext, ElementContext, AlternativeContext, RegularContext, EpsilonContext, AnnotationContext, CompilationUnitContext, GrammarNameContext, RulesContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (8, 11) => 31,
    (11, 15) => 36,
    (4, 4) => 13,
    (20, 3) => 21,
    (19, 3) => 15,
    (23, 19) => 29,
    (23, 13) => 29,
    (7, 3) => 25,
    (6, 9) => 18,
    (21, 11) => 24,
    (21, 19) => 23,
    (23, 8) => 29,
    (22, 11) => 27,
    (2, 3) => 8,
    (15, 15) => 5,
    (28, 4) => 43,
    (24, 16) => 33,
    (25, 7) => 38,
    (19, 8) => 14,
    (19, 19) => 15,
    (15, 3) => 4,
    (16, 1) => 6,
    (11, 16) => 37,
    (18, 8) => 12,
    (23, 3) => 29,
    (4, 9) => 13,
    (2, 16) => 8,
    (5, 9) => 17,
    (21, 4) => 23,
    (26, 18) => 39,
    (22, 10) => 26,
    (21, 12) => 24,
    (23, 14) => 29,
    (28, 18) => 43,
    (19, 6) => 14,
    (23, 4) => 29,
    (21, 8) => 23,
    (2, 4) => 8,
    (17, 8) => 10,
    (18, 14) => 11,
    (2, 15) => 8,
    (5, 4) => 16,
    (19, 4) => 15,
    (4, 19) => 13,
    (21, 13) => 23,
    (15, 4) => 5,
    (21, 10) => 24,
    (5, 3) => 16,
    (4, 3) => 13,
    (7, 4) => 25,
    (16, 16) => 7,
    (13, 3) => 45,
    (7, 19) => 25,
    (8, 12) => 31,
    (3, 3) => 9,
    (21, 6) => 23,
    (12, 3) => 41,
    (16, 4) => 7,
    (7, 13) => 25,
    (21, 3) => 23,
    (18, 6) => 11,
    (23, 6) => 29,
    (16, 3) => 7,
    (28, 13) => 44,
    (5, 19) => 16,
    (14, 3) => 2,
    (9, 4) => 34,
    (21, 14) => 23,
    (0, 2) => 0,
    (9, 15) => 34,
    (8, 10) => 31,
    (24, 4) => 32,
    (1, 2) => 3,
    (10, 20) => 35,
    (5, 13) => 16,
    (24, 15) => 33,
    (20, 4) => 19,
    (23, 12) => 30,
    (22, 12) => 28,
    (26, 7) => 40,
    (4, 13) => 13,
    (28, 7) => 43,
    (14, 4) => 1,
    (19, 13) => 15,
    (20, 13) => 22,
    (16, 15) => 7,
    (15, 16) => 5,
    (19, 14) => 14,
    (20, 19) => 20,
    (27, 13) => 42,
    (2, 1) => 8,
    (9, 16) => 34,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    11 => Production::new(11, 18, &vec![]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    43 => Production::new(43, 28, &vec![]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    6 => Production::new(6, 16, &vec![]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    32 => Production::new(32, 24, &vec![]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    29 => Production::new(29, 23, &vec![]),
    14 => Production::new(14, 19, &vec![]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    39 => Production::new(39, 26, &vec![]),
    23 => Production::new(23, 21, &vec![]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    8 => "ebnf_suffix",
    3 => "parser_rule",
    12 => "attributes",
    4 => "block",
    9 => "lexer_rule",
    13 => "attribute",
    7 => "element",
    5 => "alternative",
    10 => "regular",
    6 => "epsilon",
    11 => "annotation",
    0 => "compilation_unit",
    1 => "grammar_name",
    2 => "rules",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    5 => "COLON",
    15 => "AT",
    12 => "QUESTION",
    20 => "REGULAR_LITERAL",
    1 => "_STOP",
    23 => "BLOCK_COMMENT",
    17 => "LBRACKET",
    0 => "_START",
    11 => "PLUS",
    7 => "COMMA",
    13 => "LPAREN",
    9 => "EPSILON",
    4 => "TOKEN_REF",
    19 => "STRING_LITERAL",
    21 => "WHITE_SPACE",
    10 => "STAR",
    18 => "RBRACKET",
    8 => "OR",
    6 => "SEMI",
    16 => "SHARP",
    22 => "LINE_COMMENT",
    3 => "RULE_REF",
    2 => "GRAMMAR",
    14 => "RPAREN",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (15, 1),
    (3, 15),
    (17, 6),
    (8, 13),
    (1, 15),
    (21, 14),
    (8, 3),
    (13, 18),
    (22, 8),
    (13, 7),
    (21, 3),
    (20, 3),
    (3, 16),
    (22, 6),
    (1, 1),
    (27, 18),
    (13, 4),
    (28, 7),
    (25, 7),
    (23, 13),
    (20, 11),
    (27, 7),
    (19, 6),
    (15, 4),
    (21, 8),
    (28, 18),
    (9, 4),
    (18, 6),
    (20, 4),
    (5, 8),
    (20, 12),
    (7, 13),
    (24, 4),
    (14, 6),
    (4, 6),
    (5, 6),
    (7, 14),
    (7, 3),
    (6, 6),
    (21, 19),
    (6, 8),
    (1, 4),
    (17, 8),
    (4, 14),
    (23, 6),
    (22, 14),
    (9, 1),
    (7, 4),
    (21, 6),
    (27, 4),
    (25, 18),
    (3, 1),
    (20, 14),
    (22, 19),
    (9, 3),
    (8, 8),
    (8, 19),
    (16, 1),
    (22, 12),
    (23, 19),
    (7, 8),
    (8, 6),
    (15, 3),
    (20, 10),
    (20, 8),
    (7, 6),
    (9, 16),
    (6, 14),
    (0, 1),
    (2, 1),
    (7, 19),
    (8, 14),
    (19, 14),
    (17, 14),
    (1, 16),
    (8, 4),
    (20, 19),
    (15, 16),
    (23, 14),
    (22, 4),
    (22, 13),
    (11, 4),
    (3, 3),
    (20, 6),
    (28, 4),
    (5, 14),
    (21, 13),
    (23, 4),
    (1, 3),
    (10, 6),
    (9, 15),
    (23, 8),
    (3, 4),
    (18, 14),
    (23, 3),
    (15, 15),
    (20, 13),
    (21, 4),
    (12, 18),
    (26, 18),
    (22, 3),
    (19, 8),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const EBNF_SUFFIX: usize = 8; 
  pub const PARSER_RULE: usize = 3; 
  pub const ATTRIBUTES: usize = 12; 
  pub const BLOCK: usize = 4; 
  pub const LEXER_RULE: usize = 9; 
  pub const ATTRIBUTE: usize = 13; 
  pub const ELEMENT: usize = 7; 
  pub const ALTERNATIVE: usize = 5; 
  pub const REGULAR: usize = 10; 
  pub const EPSILON: usize = 6; 
  pub const ANNOTATION: usize = 11; 
  pub const COMPILATION_UNIT: usize = 0; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const RULES: usize = 2; 



  pub fn new() -> Self {
    Self {
      analyzer: LL1 { 
        error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
        table: &LL1_TABLE, 
        productions: &PRODUCTIONS, 
        rule_names: &NONTERMINALS, 
        sync: &SYNC, 
      }
    }
  }


  // 使用模板生成
  
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
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
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
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
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
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
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Box<dyn CompilationUnitContext> {
    let result = self.analyzer.analyse(token_stream, Self::COMPILATION_UNIT);
    Box::new(result)
  } 
  pub fn grammar_name(&self, token_stream: &mut TokenStream) -> Box<dyn GrammarNameContext> {
    let result = self.analyzer.analyse(token_stream, Self::GRAMMAR_NAME);
    Box::new(result)
  } 
  pub fn rules(&self, token_stream: &mut TokenStream) -> Box<dyn RulesContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULES);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}
