


use regex::Regex;

use syntaxis::runtime::{lexer::{Lexer, Error}, token::{Token, Position}};



pub struct GreetLexer {
  pub input: String,
  pub cursor: usize, // 每次都匹配 input[cursor..]
  pub regex_list: Vec<regex::Regex>,
  pub token_names: Vec<&'static str>,
  pub token_index: usize,
  pub position: Position,
}

#[allow(dead_code)]
impl GreetLexer {
  // 前两个是开始符号和结束符号
  pub const _START: usize = 0;
  pub const _STOP: usize = 1;

  // 从这里开始使用模板
  
  pub const T_2: usize = 2;
  pub const T_3: usize = 3;
  pub const T_4: usize = 4;
  pub const T_5: usize = 5;
  pub const T_6: usize = 6;
  pub const T_7: usize = 7;
  pub const WHITE_SPACE: usize = 8;
  pub const NUMBER: usize = 9;
  pub const IDENTIFY: usize = 10;


  pub fn new(input: &str) -> Self {
    let regex_list = vec![

      // 这里也需要使用模板
      
      Regex::new(r####"^(hello)"####).unwrap(), // T_2
      Regex::new(r####"^(if)"####).unwrap(), // T_3
      Regex::new(r####"^(while)"####).unwrap(), // T_4
      Regex::new(r####"^(do)"####).unwrap(), // T_5
      Regex::new(r####"^(const)"####).unwrap(), // T_6
      Regex::new(r####"^(\+)"####).unwrap(), // T_7
      Regex::new(r####"^([ \r\t\n]+)"####).unwrap(), // WHITE_SPACE
      Regex::new(r####"^([0-9]+)"####).unwrap(), // NUMBER
      Regex::new(r####"^([a-zA-Z_]+)"####).unwrap(), // IDENTIFY
    
    ];
    
    // 同样使用模板
    let token_names = vec![
      "_START", "_STOP", 
      "T_2", "T_3", "T_4", "T_5", "T_6", "T_7", "WHITE_SPACE", "NUMBER", "IDENTIFY", 
    ];

    GreetLexer { input: input.to_owned(), cursor: 0, regex_list, token_names, token_index: 0, position: Position { line: 0, char_position: 0 } }
  }



}


impl Lexer for GreetLexer {


  fn scan(&mut self) -> Result<Token, Error> {
    let mut len = 0;
    let mut token_type = 0;

    for i in 0..self.regex_list.len() {
      let result = self.regex_list[i].find_at(&self.input[self.cursor..], 0) ;
      if let Some(result) = result {
        if result.end() > len {
          len = result.end();
          token_type = i+2;
        }
      }
    }

    if token_type <= 0 { return Err(Error {}) }
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



    let token = Token {
      token_type,
      token_name: String::from(self.token_names[token_type]),

      start: self.position.clone(),
      stop: new_pos.clone(),
      
      channel: 0,
      text,
      token_index: self.token_index,
      char_start_index: self.cursor,
      char_stop_index: self.cursor + len,
    };

    self.cursor += len;
    self.token_index += 1;
    self.position = new_pos;
    return Ok(token);

  }

  fn scan_all_on_channel_tokens(&mut self, _channel: usize) -> Vec<Token> {
    todo!()
  }
}


