


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
    
    (Regex::new(r###"[a-z][a-zA-Z0-9_]*"###).unwrap(), 2, 0, "RULEREF", false), 
    (Regex::new(r###"[A-Z][a-zA-Z0-9_]*"###).unwrap(), 3, 0, "TOKENREF", false), 
    (Regex::new(r###"::=|:=|->|=>|:|="###).unwrap(), 4, 0, "COLON", false), 
    (Regex::new(r###";"###).unwrap(), 5, 0, "SEMI", false), 
    (Regex::new(r###"\|"###).unwrap(), 6, 0, "OR", false), 
    (Regex::new(r###"ε|epsilon"###).unwrap(), 7, 0, "EPSILON", false), 
    (Regex::new(r###"\*"###).unwrap(), 8, 0, "STAR", false), 
    (Regex::new(r###"\+"###).unwrap(), 9, 0, "PLUS", false), 
    (Regex::new(r###"\?"###).unwrap(), 10, 0, "QUESTION", false), 
    (Regex::new(r###"\("###).unwrap(), 11, 0, "LPAREN", false), 
    (Regex::new(r###"\)"###).unwrap(), 12, 0, "RPAREN", false), 
    (Regex::new(r###""((\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d|[^\a\d\n\r\t\f\v\\"])*""###).unwrap(), 13, 0, "STRINGLITERAL", false), 
    (Regex::new(r#####"(?s)r###".*?"###"#####).unwrap(), 14, 0, "REGULARLITERAL", false), 
    (Regex::new(r###"[ \r\n\t\f]+"###).unwrap(), 15, 0, "WHITESPACE", true), 
  ];
}


#[allow(dead_code)]
impl<'a> ChiruLexer<'a> {
  pub const _START: usize = 0;
  pub const _STOP: usize = 1;

  // 从这里开始使用模板
  
  pub const RULE_REF: usize = 2;
  pub const TOKEN_REF: usize = 3;
  pub const COLON: usize = 4;
  pub const SEMI: usize = 5;
  pub const OR: usize = 6;
  pub const EPSILON: usize = 7;
  pub const STAR: usize = 8;
  pub const PLUS: usize = 9;
  pub const QUESTION: usize = 10;
  pub const LPAREN: usize = 11;
  pub const RPAREN: usize = 12;
  pub const STRING_LITERAL: usize = 13;
  pub const REGULAR_LITERAL: usize = 14;
  pub const WHITE_SPACE: usize = 15;


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


