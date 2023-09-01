

use std::collections::{HashMap, HashSet};
use maplit::hashmap;
use maplit::hashset;

use chiru::{
  runtime::{parser::LL1, token_stream::TokenStream, error_strategy::error_listener::ConsoleErrorListener}, 

  tool::grammar::{production::ProductionItem, production::Production}

};

use super::chiru_context::{
   CompilationUnitContext, AttributeListContext, ElementContext, GrammarNameContext, EpsilonContext, AlternativeContext, EbnfSuffixContext, RulesContext, AnnotationContext, BlockContext, ParserRuleContext, RegularContext, LexerRuleContext, AttributeContext,
};


pub struct ChiruParser {
  pub analyzer: LL1,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    
    (8, 12) => 31,
    (22, 11) => 27,
    (23, 14) => 29,
    (19, 6) => 14,
    (3, 3) => 9,
    (16, 1) => 6,
    (21, 12) => 24,
    (16, 15) => 7,
    (16, 3) => 7,
    (17, 8) => 10,
    (9, 16) => 34,
    (18, 6) => 11,
    (16, 16) => 7,
    (1, 2) => 3,
    (23, 12) => 30,
    (4, 9) => 13,
    (23, 19) => 29,
    (23, 13) => 29,
    (12, 3) => 41,
    (2, 1) => 8,
    (10, 20) => 35,
    (9, 4) => 34,
    (5, 9) => 17,
    (21, 13) => 23,
    (16, 4) => 7,
    (28, 7) => 43,
    (7, 3) => 25,
    (27, 13) => 42,
    (7, 4) => 25,
    (26, 18) => 39,
    (15, 15) => 5,
    (19, 3) => 15,
    (28, 18) => 43,
    (14, 3) => 2,
    (21, 14) => 23,
    (15, 3) => 4,
    (20, 3) => 21,
    (18, 14) => 11,
    (26, 7) => 40,
    (19, 13) => 15,
    (5, 3) => 16,
    (2, 3) => 8,
    (20, 4) => 19,
    (11, 15) => 36,
    (0, 2) => 0,
    (19, 19) => 15,
    (5, 19) => 16,
    (2, 4) => 8,
    (15, 16) => 5,
    (21, 11) => 24,
    (21, 3) => 23,
    (18, 8) => 12,
    (13, 3) => 45,
    (7, 19) => 25,
    (19, 14) => 14,
    (21, 8) => 23,
    (23, 8) => 29,
    (24, 15) => 33,
    (25, 7) => 38,
    (21, 10) => 24,
    (5, 4) => 16,
    (11, 16) => 37,
    (22, 12) => 28,
    (9, 15) => 34,
    (19, 8) => 14,
    (20, 19) => 20,
    (6, 9) => 18,
    (20, 13) => 22,
    (22, 10) => 26,
    (7, 13) => 25,
    (24, 16) => 33,
    (4, 4) => 13,
    (21, 19) => 23,
    (23, 4) => 29,
    (4, 13) => 13,
    (15, 4) => 5,
    (8, 11) => 31,
    (23, 3) => 29,
    (2, 15) => 8,
    (8, 10) => 31,
    (19, 4) => 15,
    (24, 4) => 32,
    (2, 16) => 8,
    (21, 6) => 23,
    (14, 4) => 1,
    (5, 13) => 16,
    (28, 13) => 44,
    (4, 3) => 13,
    (4, 19) => 13,
    (28, 4) => 43,
    (21, 4) => 23,
    (23, 6) => 29,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    
    8 => Production::new(8, 2, &vec![ProductionItem::NonTerminal(16),]),
    38 => Production::new(38, 25, &vec![ProductionItem::Terminal(7),ProductionItem::NonTerminal(13),]),
    2 => Production::new(2, 14, &vec![ProductionItem::Terminal(3),]),
    16 => Production::new(16, 5, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
    42 => Production::new(42, 27, &vec![ProductionItem::Terminal(13),ProductionItem::Terminal(4),ProductionItem::Terminal(14),]),
    17 => Production::new(17, 5, &vec![ProductionItem::NonTerminal(6),]),
    39 => Production::new(39, 26, &vec![]),
    4 => Production::new(4, 15, &vec![ProductionItem::NonTerminal(3),]),
    23 => Production::new(23, 21, &vec![]),
    29 => Production::new(29, 23, &vec![]),
    19 => Production::new(19, 20, &vec![ProductionItem::Terminal(4),]),
    32 => Production::new(32, 24, &vec![]),
    36 => Production::new(36, 11, &vec![ProductionItem::Terminal(15),ProductionItem::NonTerminal(13),]),
    7 => Production::new(7, 16, &vec![ProductionItem::NonTerminal(15),ProductionItem::NonTerminal(16),]),
    21 => Production::new(21, 20, &vec![ProductionItem::Terminal(3),]),
    22 => Production::new(22, 20, &vec![ProductionItem::Terminal(13),ProductionItem::NonTerminal(4),ProductionItem::Terminal(14),]),
    11 => Production::new(11, 18, &vec![]),
    27 => Production::new(27, 22, &vec![ProductionItem::Terminal(11),]),
    33 => Production::new(33, 24, &vec![ProductionItem::NonTerminal(11),]),
    40 => Production::new(40, 26, &vec![ProductionItem::NonTerminal(25),ProductionItem::NonTerminal(26),]),
    24 => Production::new(24, 21, &vec![ProductionItem::NonTerminal(8),]),
    10 => Production::new(10, 17, &vec![ProductionItem::Terminal(8),ProductionItem::NonTerminal(5),]),
    1 => Production::new(1, 14, &vec![ProductionItem::Terminal(4),]),
    30 => Production::new(30, 23, &vec![ProductionItem::Terminal(12),]),
    37 => Production::new(37, 11, &vec![ProductionItem::Terminal(16),ProductionItem::Terminal(17),ProductionItem::NonTerminal(12),ProductionItem::Terminal(18),]),
    41 => Production::new(41, 12, &vec![ProductionItem::NonTerminal(13),ProductionItem::NonTerminal(26),]),
    43 => Production::new(43, 28, &vec![]),
    20 => Production::new(20, 20, &vec![ProductionItem::Terminal(19),]),
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(1),ProductionItem::NonTerminal(2),]),
    14 => Production::new(14, 19, &vec![]),
    28 => Production::new(28, 22, &vec![ProductionItem::Terminal(12),]),
    18 => Production::new(18, 6, &vec![ProductionItem::Terminal(9),]),
    12 => Production::new(12, 18, &vec![ProductionItem::NonTerminal(17),ProductionItem::NonTerminal(18),]),
    3 => Production::new(3, 1, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(14),ProductionItem::Terminal(6),]),
    9 => Production::new(9, 3, &vec![ProductionItem::Terminal(3),ProductionItem::Terminal(5),ProductionItem::NonTerminal(4),ProductionItem::Terminal(6),]),
    35 => Production::new(35, 10, &vec![ProductionItem::Terminal(20),]),
    45 => Production::new(45, 13, &vec![ProductionItem::Terminal(3),ProductionItem::NonTerminal(28),]),
    34 => Production::new(34, 9, &vec![ProductionItem::NonTerminal(24),ProductionItem::Terminal(4),ProductionItem::Terminal(5),ProductionItem::NonTerminal(10),ProductionItem::Terminal(6),]),
    6 => Production::new(6, 16, &vec![]),
    13 => Production::new(13, 4, &vec![ProductionItem::NonTerminal(5),ProductionItem::NonTerminal(18),]),
    26 => Production::new(26, 22, &vec![ProductionItem::Terminal(10),]),
    25 => Production::new(25, 7, &vec![ProductionItem::NonTerminal(20),ProductionItem::NonTerminal(21),]),
    5 => Production::new(5, 15, &vec![ProductionItem::NonTerminal(9),]),
    31 => Production::new(31, 8, &vec![ProductionItem::NonTerminal(22),ProductionItem::NonTerminal(23),]),
    44 => Production::new(44, 28, &vec![ProductionItem::NonTerminal(27),]),
    15 => Production::new(15, 19, &vec![ProductionItem::NonTerminal(7),ProductionItem::NonTerminal(19),]),
  };

  // 非终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    0 => "compilation_unit",
    12 => "attribute_list",
    7 => "element",
    1 => "grammar_name",
    6 => "epsilon",
    5 => "alternative",
    8 => "ebnf_suffix",
    2 => "rules",
    11 => "annotation",
    4 => "block",
    3 => "parser_rule",
    10 => "regular",
    9 => "lexer_rule",
    13 => "attribute",
  };

  // 终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    
    10 => "STAR",
    1 => "_STOP",
    6 => "SEMI",
    18 => "RBRACKET",
    22 => "LINE_COMMENT",
    12 => "QUESTION",
    9 => "EPSILON",
    20 => "REGULAR_LITERAL",
    13 => "LPAREN",
    23 => "BLOCK_COMMENT",
    15 => "AT",
    7 => "COMMA",
    14 => "RPAREN",
    21 => "WHITE_SPACE",
    17 => "LBRACKET",
    3 => "RULE_REF",
    16 => "SHARP",
    4 => "TOKEN_REF",
    8 => "OR",
    11 => "PLUS",
    5 => "COLON",
    19 => "STRING_LITERAL",
    0 => "_START",
    2 => "GRAMMAR",
  };

  pub static ref SYNC: HashSet<(usize, usize)> = hashset! {
    
    (17, 8),
    (19, 6),
    (21, 8),
    (20, 4),
    (7, 13),
    (14, 6),
    (15, 4),
    (23, 4),
    (1, 4),
    (22, 12),
    (18, 6),
    (20, 12),
    (8, 19),
    (9, 15),
    (21, 3),
    (9, 3),
    (25, 7),
    (20, 13),
    (6, 6),
    (8, 6),
    (28, 18),
    (27, 4),
    (16, 1),
    (18, 14),
    (7, 19),
    (1, 1),
    (24, 4),
    (4, 14),
    (4, 6),
    (7, 14),
    (1, 15),
    (8, 4),
    (20, 10),
    (11, 4),
    (27, 7),
    (27, 18),
    (20, 14),
    (21, 6),
    (22, 8),
    (28, 7),
    (19, 14),
    (20, 11),
    (7, 6),
    (28, 4),
    (5, 6),
    (22, 19),
    (20, 6),
    (3, 16),
    (22, 6),
    (0, 1),
    (1, 16),
    (9, 16),
    (15, 1),
    (17, 6),
    (13, 7),
    (21, 4),
    (20, 3),
    (9, 1),
    (3, 3),
    (15, 3),
    (21, 13),
    (19, 8),
    (22, 13),
    (6, 14),
    (9, 4),
    (6, 8),
    (23, 14),
    (5, 14),
    (8, 8),
    (7, 4),
    (21, 14),
    (1, 3),
    (20, 8),
    (22, 4),
    (7, 8),
    (5, 8),
    (23, 3),
    (23, 19),
    (15, 15),
    (23, 6),
    (2, 1),
    (7, 3),
    (3, 1),
    (8, 3),
    (21, 19),
    (20, 19),
    (8, 14),
    (13, 18),
    (15, 16),
    (12, 18),
    (22, 14),
    (8, 13),
    (23, 13),
    (3, 4),
    (22, 3),
    (10, 6),
    (23, 8),
    (17, 14),
    (26, 18),
    (25, 18),
    (13, 4),
    (3, 15),
  };
}

#[allow(unused)]
impl ChiruParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const COMPILATION_UNIT: usize = 0; 
  pub const ATTRIBUTE_LIST: usize = 12; 
  pub const ELEMENT: usize = 7; 
  pub const GRAMMAR_NAME: usize = 1; 
  pub const EPSILON: usize = 6; 
  pub const ALTERNATIVE: usize = 5; 
  pub const EBNF_SUFFIX: usize = 8; 
  pub const RULES: usize = 2; 
  pub const ANNOTATION: usize = 11; 
  pub const BLOCK: usize = 4; 
  pub const PARSER_RULE: usize = 3; 
  pub const REGULAR: usize = 10; 
  pub const LEXER_RULE: usize = 9; 
  pub const ATTRIBUTE: usize = 13; 



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
  
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Box<dyn CompilationUnitContext> {
    let result = self.analyzer.analyse(token_stream, Self::COMPILATION_UNIT);
    Box::new(result)
  } 
  pub fn attribute_list(&self, token_stream: &mut TokenStream) -> Box<dyn AttributeListContext> {
    let result = self.analyzer.analyse(token_stream, Self::ATTRIBUTE_LIST);
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
  pub fn epsilon(&self, token_stream: &mut TokenStream) -> Box<dyn EpsilonContext> {
    let result = self.analyzer.analyse(token_stream, Self::EPSILON);
    Box::new(result)
  } 
  pub fn alternative(&self, token_stream: &mut TokenStream) -> Box<dyn AlternativeContext> {
    let result = self.analyzer.analyse(token_stream, Self::ALTERNATIVE);
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
  pub fn annotation(&self, token_stream: &mut TokenStream) -> Box<dyn AnnotationContext> {
    let result = self.analyzer.analyse(token_stream, Self::ANNOTATION);
    Box::new(result)
  } 
  pub fn block(&self, token_stream: &mut TokenStream) -> Box<dyn BlockContext> {
    let result = self.analyzer.analyse(token_stream, Self::BLOCK);
    Box::new(result)
  } 
  pub fn parser_rule(&self, token_stream: &mut TokenStream) -> Box<dyn ParserRuleContext> {
    let result = self.analyzer.analyse(token_stream, Self::PARSER_RULE);
    Box::new(result)
  } 
  pub fn regular(&self, token_stream: &mut TokenStream) -> Box<dyn RegularContext> {
    let result = self.analyzer.analyse(token_stream, Self::REGULAR);
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

}






// impl Parser for SyntaxisParser {}

