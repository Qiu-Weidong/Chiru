

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::chiru_context::{
   RegularContext, EbnfSuffixContext, AttributesContext, ElementContext, GrammarNameContext, ParserRuleContext, AlternativeContext, LexerRuleContext, RulesContext, BlockContext, AnnotationContext, AttributeContext, EpsilonContext, CompilationUnitContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (4, 13) => 13,
    (2, 4) => 8,
    (22, 10) => 26,
    (6, 9) => 18,
    (2, 16) => 8,
    (25, 7) => 38,
    (15, 15) => 5,
    (22, 11) => 27,
    (23, 12) => 30,
    (7, 3) => 25,
    (23, 6) => 29,
    (24, 16) => 33,
    (20, 19) => 20,
    (20, 13) => 22,
    (14, 4) => 1,
    (15, 3) => 4,
    (9, 4) => 34,
    (4, 19) => 13,
    (4, 9) => 13,
    (8, 11) => 31,
    (24, 4) => 32,
    (28, 13) => 44,
    (19, 13) => 15,
    (18, 14) => 11,
    (21, 13) => 23,
    (2, 3) => 8,
    (21, 8) => 23,
    (21, 19) => 23,
    (28, 7) => 43,
    (5, 13) => 16,
    (28, 4) => 43,
    (3, 3) => 9,
    (15, 16) => 5,
    (21, 14) => 23,
    (5, 3) => 16,
    (27, 13) => 42,
    (20, 4) => 19,
    (12, 3) => 41,
    (11, 15) => 36,
    (19, 14) => 14,
    (26, 7) => 40,
    (23, 3) => 29,
    (17, 8) => 10,
    (8, 10) => 31,
    (7, 4) => 25,
    (4, 3) => 13,
    (16, 15) => 7,
    (16, 16) => 7,
    (0, 2) => 0,
    (21, 3) => 23,
    (24, 15) => 33,
    (23, 19) => 29,
    (23, 14) => 29,
    (19, 4) => 15,
    (11, 16) => 37,
    (21, 12) => 24,
    (1, 2) => 3,
    (9, 15) => 34,
    (26, 18) => 39,
    (28, 18) => 43,
    (19, 8) => 14,
    (23, 8) => 29,
    (22, 12) => 28,
    (5, 9) => 17,
    (2, 15) => 8,
    (8, 12) => 31,
    (19, 6) => 14,
    (21, 4) => 23,
    (14, 3) => 2,
    (21, 11) => 24,
    (19, 3) => 15,
    (2, 1) => 8,
    (7, 19) => 25,
    (23, 4) => 29,
    (19, 19) => 15,
    (9, 16) => 34,
    (18, 6) => 11,
    (15, 4) => 5,
    (16, 3) => 7,
    (23, 13) => 29,
    (10, 20) => 35,
    (16, 4) => 7,
    (4, 4) => 13,
    (18, 8) => 12,
    (5, 19) => 16,
    (16, 1) => 6,
    (5, 4) => 16,
    (7, 13) => 25,
    (21, 10) => 24,
    (13, 3) => 45,
    (21, 6) => 23,
    (20, 3) => 21,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    32 => Production::new(32, 24, &vec![]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    39 => Production::new(39, 26, &vec![]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    29 => Production::new(29, 23, &vec![]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    43 => Production::new(43, 28, &vec![]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    14 => Production::new(14, 19, &vec![]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    6 => Production::new(6, 16, &vec![]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    11 => Production::new(11, 18, &vec![]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    23 => Production::new(23, 21, &vec![]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    10 => "regular",
    8 => "ebnf_suffix",
    12 => "attributes",
    7 => "element",
    1 => "grammar_name",
    3 => "parser_rule",
    5 => "alternative",
    9 => "lexer_rule",
    2 => "rules",
    4 => "block",
    11 => "annotation",
    13 => "attribute",
    6 => "epsilon",
    0 => "compilation_unit",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    15 => "AT",
    4 => "TOKEN_REF",
    8 => "OR",
    6 => "SEMI",
    14 => "RPAREN",
    12 => "QUESTION",
    10 => "STAR",
    22 => "LINE_COMMENT",
    13 => "LPAREN",
    5 => "COLON",
    16 => "SHARP",
    21 => "WHITE_SPACE",
    19 => "STRING_LITERAL",
    20 => "REGULAR_LITERAL",
    1 => "_STOP",
    0 => "_START",
    18 => "RBRACKET",
    3 => "RULE_REF",
    23 => "BLOCK_COMMENT",
    2 => "GRAMMAR",
    9 => "EPSILON",
    11 => "PLUS",
    17 => "LBRACKET",
    7 => "COMMA",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (20, 8),
    (17, 14),
    (15, 4),
    (20, 4),
    (22, 6),
    (13, 7),
    (28, 4),
    (14, 6),
    (23, 8),
    (9, 15),
    (7, 14),
    (2, 1),
    (0, 1),
    (22, 14),
    (20, 11),
    (15, 16),
    (5, 14),
    (23, 14),
    (22, 13),
    (7, 13),
    (22, 8),
    (24, 4),
    (7, 19),
    (19, 14),
    (12, 18),
    (26, 18),
    (21, 19),
    (9, 3),
    (20, 14),
    (1, 1),
    (9, 1),
    (23, 13),
    (3, 4),
    (6, 8),
    (23, 3),
    (23, 4),
    (25, 18),
    (16, 1),
    (21, 4),
    (23, 6),
    (20, 13),
    (15, 1),
    (19, 8),
    (27, 18),
    (7, 4),
    (7, 6),
    (15, 15),
    (3, 3),
    (22, 19),
    (1, 4),
    (19, 6),
    (8, 8),
    (3, 16),
    (4, 6),
    (11, 4),
    (8, 3),
    (20, 10),
    (13, 4),
    (3, 1),
    (8, 4),
    (13, 18),
    (6, 6),
    (23, 19),
    (10, 6),
    (15, 3),
    (21, 8),
    (20, 3),
    (20, 12),
    (17, 8),
    (8, 14),
    (8, 19),
    (22, 12),
    (28, 18),
    (3, 15),
    (21, 13),
    (20, 6),
    (18, 6),
    (5, 8),
    (22, 3),
    (5, 6),
    (25, 7),
    (4, 14),
    (27, 7),
    (1, 3),
    (6, 14),
    (20, 19),
    (27, 4),
    (8, 13),
    (21, 6),
    (8, 6),
    (9, 4),
    (1, 15),
    (7, 8),
    (22, 4),
    (28, 7),
    (1, 16),
    (9, 16),
    (21, 14),
    (18, 14),
    (7, 3),
    (21, 3),
    (17, 6),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const REGULAR: usize = 10; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const ATTRIBUTES: usize = 12; 
  pub const ELEMENT: usize = 7; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const PARSER_RULE: usize = 3; 
  pub const ALTERNATIVE: usize = 5; 
  pub const LEXER_RULE: usize = 9; 
  pub const RULES: usize = 2; 
  pub const BLOCK: usize = 4; 
  pub const ANNOTATION: usize = 11; 
  pub const ATTRIBUTE: usize = 13; 
  pub const EPSILON: usize = 6; 
  pub const COMPILATION_UNIT: usize = 0; 



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
  
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
    Box::new(result)
  } 
  pub fn ebnf_suffix(&self, token_stream: &mut TokenStream) -> Box<dyn EbnfSuffixContext> {
    let result = self.analyzer.analyse(token_stream, Self::EBNF_SUFFIX);
    Box::new(result)
  } 
  pub fn attributes(&self, token_stream: &mut TokenStream) -> Box<dyn AttributesContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTES);
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
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
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
  pub fn rules(&self, token_stream: &mut TokenStream) -> Box<dyn RulesContext> {
    let result = self.analyzer.analyse(token_stream, Self::RULES);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn attribute(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE);
    Box::new(result)
  } 
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Box<dyn CompilationUnitContext> {
    let result = self.analyzer.analyse(token_stream, Self::COMPILATION_UNIT);
    Box::new(result)
  } 

}






// impl Parser for SyntaxisParser {}

