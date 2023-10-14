



// generated from .\src\tool\syntaxis\chiru.chiru by chiru 0.7.0
 


use chiru::once_cell::sync::Lazy;
use chiru::regex::Regex;

use chiru::runtime::error_strategy::error_listener::{ErrorListener, ConsoleErrorListener};
use chiru::runtime::lexer::TokenIter;
use chiru::runtime::lexer::Lexer;
use chiru::runtime::lexer_rule::LexerRule;

pub struct ChiruLexer<'a> {
  pub input: &'a str, 

  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LEXER_RULE_LIST: Lazy<Vec<LexerRule>> = Lazy::new(|| {
  vec![
    
    LexerRule { 
      rule: Regex::new(r###"grammar"###).unwrap(),  
      token_type: 2, 
      channel: 0, 
      token_name: String::from("GRAMMAR"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"[a-z][a-zA-Z0-9_]*"###).unwrap(),  
      token_type: 3, 
      channel: 0, 
      token_name: String::from("RULE_REF"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"[A-Z][a-zA-Z0-9_]*"###).unwrap(),  
      token_type: 4, 
      channel: 0, 
      token_name: String::from("TOKEN_REF"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"::=|:=|->|=>|:|="###).unwrap(),  
      token_type: 5, 
      channel: 0, 
      token_name: String::from("COLON"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###";"###).unwrap(),  
      token_type: 6, 
      channel: 0, 
      token_name: String::from("SEMI"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###","###).unwrap(),  
      token_type: 7, 
      channel: 0, 
      token_name: String::from("COMMA"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\|"###).unwrap(),  
      token_type: 8, 
      channel: 0, 
      token_name: String::from("OR"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"ε|epsilon"###).unwrap(),  
      token_type: 9, 
      channel: 0, 
      token_name: String::from("EPSILON"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\*"###).unwrap(),  
      token_type: 10, 
      channel: 0, 
      token_name: String::from("STAR"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\+"###).unwrap(),  
      token_type: 11, 
      channel: 0, 
      token_name: String::from("PLUS"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\?"###).unwrap(),  
      token_type: 12, 
      channel: 0, 
      token_name: String::from("QUESTION"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\("###).unwrap(),  
      token_type: 13, 
      channel: 0, 
      token_name: String::from("LPAREN"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\)"###).unwrap(),  
      token_type: 14, 
      channel: 0, 
      token_name: String::from("RPAREN"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"@"###).unwrap(),  
      token_type: 15, 
      channel: 0, 
      token_name: String::from("AT"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"#"###).unwrap(),  
      token_type: 16, 
      channel: 0, 
      token_name: String::from("SHARP"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\["###).unwrap(),  
      token_type: 17, 
      channel: 0, 
      token_name: String::from("LBRACKET"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"\]"###).unwrap(),  
      token_type: 18, 
      channel: 0, 
      token_name: String::from("RBRACKET"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###""((\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d|[^\a\d\n\r\t\f\v\\"])*""###).unwrap(),  
      token_type: 19, 
      channel: 0, 
      token_name: String::from("STRING_LITERAL"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r#####"(?s)r###".*?"###"#####).unwrap(),  
      token_type: 20, 
      channel: 0, 
      token_name: String::from("REGULAR_LITERAL"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"[ \r\n\t\f]+"###).unwrap(),  
      token_type: 21, 
      channel: 0, 
      token_name: String::from("WHITE_SPACE"), 
      skip: true,
    }, 
    LexerRule { 
      rule: Regex::new(r###"//.*?\n"###).unwrap(),  
      token_type: 22, 
      channel: 1, 
      token_name: String::from("LINE_COMMENT"), 
      skip: false,
    }, 
    LexerRule { 
      rule: Regex::new(r###"(?s)/\*.*?\*/"###).unwrap(),  
      token_type: 23, 
      channel: 1, 
      token_name: String::from("BLOCK_COMMENT"), 
      skip: false,
    }, 
  ]
});



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


