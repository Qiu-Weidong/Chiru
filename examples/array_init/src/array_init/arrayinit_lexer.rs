use chiru::once_cell::sync::Lazy;
use chiru::regex::Regex;

use chiru::runtime::error_strategy::error_listener::{ErrorListener, ConsoleErrorListener};
use chiru::runtime::lexer::TokenIter;
use chiru::runtime::lexer::Lexer;
use chiru::runtime::lexer_rule::LexerRule;

pub struct ArrayInitLexer<'a> {
  pub input: &'a str, 

  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LEXER_RULE_LIST: Lazy<Vec<LexerRule>> = Lazy::new(|| {
  vec![
    
    LexerRule { 
      rule: Regex::new(r###"\{"###).unwrap(),  
      token_type: 2, 
      channel: 0, 
      token_name: String::from("LBRACKET"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\}"###).unwrap(),  
      token_type: 3, 
      channel: 0, 
      token_name: String::from("RBRACKET"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"[0-9]+"###).unwrap(),  
      token_type: 4, 
      channel: 0, 
      token_name: String::from("NUM"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###","###).unwrap(),  
      token_type: 5, 
      channel: 0, 
      token_name: String::from("COMMA"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"[ \r\n\t\f]+"###).unwrap(),  
      token_type: 6, 
      channel: 0, 
      token_name: String::from("WHITE_SPACE"), 
      skip: true,
    }, 
    LexerRule { 
      rule: Regex::new(r###"//.*?\n"###).unwrap(),  
      token_type: 7, 
      channel: 1, 
      token_name: String::from("LINE_COMMENT"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"(?s)/\*.*?\*\/"###).unwrap(),  
      token_type: 8, 
      channel: 1, 
      token_name: String::from("BLOCK_COMMENT"), 
      skip: false,
    }, 
  ]
});



#[allow(unused)]
impl<'a> ArrayInitLexer<'a> {
  pub const _START: usize = 0;
  pub const _STOP: usize = 1;

  // 从这里开始使用模板
  
  pub const LBRACKET: usize = 2;
  pub const RBRACKET: usize = 3;
  pub const NUM: usize = 4;
  pub const COMMA: usize = 5;
  pub const WHITE_SPACE: usize = 6;
  pub const LINE_COMMENT: usize = 7;
  pub const BLOCK_COMMENT: usize = 8;


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


impl Lexer for ArrayInitLexer<'_> {
  fn iter(&self) -> TokenIter {
    TokenIter::new(self.input, &LEXER_RULE_LIST, &self.error_listeners)
  }
}


