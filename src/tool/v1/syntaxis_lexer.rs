use regex::Regex;

use crate::runtime::{lexer::{Lexer, Error}, token::{Token, Position}};

pub struct SyntaxisLexer {
  pub input: String, // 输入文本
  pub cursor: usize, // 字符游标，当前处理到的文本字符序号
  pub token_index: usize, // token 序号，表示当前扫描到了第几个 token
  pub position: Position, // 当前处理到的文本字符所在的位置
}


// 使用模板生成正则列表和token名称列表
lazy_static!{
  
  static ref REGEX_LIST: [Regex; 14] = [

    // 这里也需要使用模板

    Regex::new(r####"^([a-z][a-zA-Z0-9_]+)"####).unwrap(), // RULE_REF
    Regex::new(r####"^([A-Z][a-zA-Z0-9_]+)"####).unwrap(), // TOKEN_REF
    Regex::new(r####"^(::=|:=|->|=>|:|=)"####).unwrap(), // COLON
    Regex::new(r####"^(;)"####).unwrap(), // SEMI
    Regex::new(r####"^(\|)"####).unwrap(), // OR
    Regex::new(r####"^(ε|epsilon)"####).unwrap(), // EPSILON
    Regex::new(r####"^(\*)"####).unwrap(), // STAR
    Regex::new(r####"^(\+)"####).unwrap(), // PLUS
    Regex::new(r####"^(\?)"####).unwrap(), // QUESTION
    Regex::new(r####"^(\()"####).unwrap(), // LPAREN
    Regex::new(r####"^(\))"####).unwrap(), // RPAREN
    Regex::new(r####"^('([^\a\d\n\r\t\f\v\\']|(\\\\|\\'|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*')"####).unwrap(), // STRING_LITERAL
    Regex::new(r####"^(/(\\/|[^/])+/)"####).unwrap(), // REGULAR_LITERAL
    Regex::new(r####"^([ \r\n\t\f]+)"####).unwrap(), // WHITE_SPACE

  ];

  static ref TOKEN_NAMES: [&'static str; 16] = [
    "_START", "_STOP", 
    "RULE_REF", "TOKEN_REF", "COLON", "SEMI", "OR", "EPSILON", "STAR", 
    "PLUS", "QUESTION", "LPAREN", "RPAREN", "STRING_LITERAL", 
    "REGULAR_LITERAL", "WHITE_SPACE", 
  ];
}


impl SyntaxisLexer {
  // 前两个是开始符号和结束符号
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

  


  pub fn new(input: &str) -> Self {
  

    SyntaxisLexer { input: input.to_owned(), cursor: 0, token_index: 0, position: Position { line: 0, char_position: 0 } }
  }




  // 定义一些私有函数
}

impl Lexer for SyntaxisLexer {

  fn scan(&mut self) -> Result<Token, crate::runtime::lexer::Error> {
    if self.cursor > self.input.len() { return Err(Error {}) }
    else if self.cursor >= self.input.len() {
      self.cursor += 10;

      // 返回结束 token _stop
      return Ok(Token::new(SyntaxisLexer::_STOP, "_STOP", "_STOP",         
        self.position.clone(), self.position.clone(), self.token_index, 0, 
        self.cursor, self.cursor))
    }
    let mut len = 0;
    let mut token_type = 0;

    for i in 0..REGEX_LIST.len() {
      let result = REGEX_LIST[i].find_at(&self.input[self.cursor..], 0) ;
      if let Some(result) = result {
        if result.end() > len {
          len = result.end();
          token_type = i+2;
        }
      }
    }

    // 如果都不匹配，则报错
    if token_type <= 0 { return Err(Error {}) }

    // 将对应的文本找出来
    let text = String::from(&self.input[self.cursor..self.cursor+len]);
    let lines: Vec<_> = text.split("\n").collect();
    let new_pos;
    if lines.len() <= 1 {
      // 没有跨行
      new_pos = Position {
        line: self.position.line,
        char_position: self.position.char_position + len
      }
    }
    else {
      // 跨行
      new_pos = Position {
        line: self.position.line + lines.len()-1,
        char_position: lines.last().unwrap().len(),
      }
    }


    let token = Token::new(token_type, TOKEN_NAMES[token_type],&text, 
      self.position.clone(),new_pos.clone(), self.token_index, 0, self.cursor, self.cursor + len);

    self.cursor += len;
    self.token_index += 1;
    self.position = new_pos;
    return Ok(token);

  }

  fn scan_all_on_channel_tokens(&mut self, channel: usize) -> Vec<Token> {
    let mut result = Vec::new();
    while let Ok(token) = self.scan() {
      if token.token_type == SyntaxisLexer::WHITE_SPACE { continue; }
      if token.channel == channel {
        result.push(token);
      }
    }

    if self.cursor < self.input.len() {
      println!("{}", &self.input[self.cursor..]);
    }
    
    result
  }

fn scan_all_tokens(&mut self) -> Vec<Token> {
        todo!()
    }

fn scan_on_channel(&mut self, channel: usize) -> Result<Token, Error> {
        todo!()
    }

fn consume(&mut self) {
        todo!()
    }

fn release(&mut self) {
        todo!()
    }
    fn scan_all_tokens_and_group_by_channel(&mut self) -> std::collections::HashMap<usize, Vec<Token>> {
      todo!()
    }

    fn look_ahead(&mut self, n: usize) -> Result<Token, Error> {
        todo!()
    }

    fn look_back(&mut self, n:usize) -> Result<Token, Error> {
        todo!()
    }

    fn look_ahead_on_channel(&mut self, channel: usize, n: usize) -> Result<Token, Error> {
        todo!()
    }

    fn look_back_on_channel(&mut self, channel: usize, n:usize) -> Result<Token, Error> {
        todo!()
    }
}

