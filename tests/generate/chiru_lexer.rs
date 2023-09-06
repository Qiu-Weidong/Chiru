

use lazy_static::lazy_static; 
use regex::Regex;

use chiru::runtime::error_strategy::error_listener::{ErrorListener, ConsoleErrorListener};
use chiru::runtime::lexer::TokenIter;
use chiru::runtime::lexer::Lexer;


pub struct ChiruLexer<'a> {
  pub input: &'a str, 

  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}

lazy_static!{
  static ref LEXER_RULE_LIST: Vec<(Regex, usize, usize, &'static str, bool)> = vec![
    
    (Regex::new(r###"grammar"###).unwrap(), 2, 0, "GRAMMAR", false), 
    (Regex::new(r###"[a-z][a-zA-Z0-9_]*"###).unwrap(), 3, 0, "RULE_REF", false), 
    (Regex::new(r###"[A-Z][a-zA-Z0-9_]*"###).unwrap(), 4, 0, "TOKEN_REF", false), 
    (Regex::new(r###"::=|:=|->|=>|:|="###).unwrap(), 5, 0, "COLON", false), 
    (Regex::new(r###";"###).unwrap(), 6, 0, "SEMI", false), 
    (Regex::new(r###","###).unwrap(), 7, 0, "COMMA", false), 
    (Regex::new(r###"\|"###).unwrap(), 8, 0, "OR", false), 
    (Regex::new(r###"ε|epsilon"###).unwrap(), 9, 0, "EPSILON", false), 
    (Regex::new(r###"\*"###).unwrap(), 10, 0, "STAR", false), 
    (Regex::new(r###"\+"###).unwrap(), 11, 0, "PLUS", false), 
    (Regex::new(r###"\?"###).unwrap(), 12, 0, "QUESTION", false), 
    (Regex::new(r###"\("###).unwrap(), 13, 0, "LPAREN", false), 
    (Regex::new(r###"\)"###).unwrap(), 14, 0, "RPAREN", false), 
    (Regex::new(r###"@"###).unwrap(), 15, 0, "AT", false), 
    (Regex::new(r###"#"###).unwrap(), 16, 0, "SHARP", false), 
    (Regex::new(r###"\["###).unwrap(), 17, 0, "LBRACKET", false), 
    (Regex::new(r###"\]"###).unwrap(), 18, 0, "RBRACKET", false), 
    (Regex::new(r###""((\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d|[^\a\d\n\r\t\f\v\\"])*""###).unwrap(), 19, 0, "STRING_LITERAL", false), 
    (Regex::new(r####"(?s)r###".*?"###"####).unwrap(), 20, 0, "REGULAR_LITERAL", false), 
    (Regex::new(r###"[ \r\n\t\f]+"###).unwrap(), 21, 0, "WHITE_SPACE", true), 
    (Regex::new(r###"//.*?\n"###).unwrap(), 22, 1, "LINE_COMMENT", false), 
    (Regex::new(r###"(?s)/\*.*?\*\/"###).unwrap(), 23, 1, "BLOCK_COMMENT", false), 
  ];
}


#[allow(unused)]
impl<'a> ChiruLexer<'a> {
  pub const _START: usize = 0;
  pub const _STOP: usize = 1;

  // 从这里开始使用模板
  
  pub const GRAMMAR: usize = 2;
  pub const RULE_REF: usize = 3;
  pub const TOKEN_REF: usize = 4;
  pub const COLON: usize = 5;
  pub const SEMI: usize = 6;
  pub const COMMA: usize = 7;
  pub const OR: usize = 8;
  pub const EPSILON: usize = 9;
  pub const STAR: usize = 10;
  pub const PLUS: usize = 11;
  pub const QUESTION: usize = 12;
  pub const LPAREN: usize = 13;
  pub const RPAREN: usize = 14;
  pub const AT: usize = 15;
  pub const SHARP: usize = 16;
  pub const LBRACKET: usize = 17;
  pub const RBRACKET: usize = 18;
  pub const STRING_LITERAL: usize = 19;
  pub const REGULAR_LITERAL: usize = 20;
  pub const WHITE_SPACE: usize = 21;
  pub const LINE_COMMENT: usize = 22;
  pub const BLOCK_COMMENT: usize = 23;


  pub fn new(input: &'a str) -> Self {
    Self { 
      input, 
      error_listeners: vec![Box::new(ConsoleErrorListener::new())],
    }
  }

  // 考虑是否放入 trait 中
  pub fn remove_all_error_listeners(&mut self) {
    self.error_listeners.clear()
  }

  pub fn add_error_listener(&mut self, listener: Box<dyn ErrorListener>) {
    self.error_listeners.push(listener)
  }



}


impl Lexer for ChiruLexer<'_> {
  fn iter(&self) -> TokenIter {
    TokenIter::new(self.input, &LEXER_RULE_LIST, &self.error_listeners)
  }
}


