
use super::{location::Location, position::Position, vocabulary::Terminal};
use std::fmt::Display;


#[derive(Clone, Debug)]
pub struct Token<'a> {
  // 名称和类型
  pub terminal: Terminal<'a>,

  // token 所在的位置
  pub location: Location,
  
  // token 所在的频道
  pub channel: usize,

  // token 的文本
  pub text: String,

  // token 在token序列中的序号(所有频道一起排号)
  pub token_index: usize,
}


impl<'a> Display for Token<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[@{}, {}:{}='{}', <{}>, <{}>, start: <{}>, stop: <{}>]", 
      self.token_index, 
      self.location.byte_index_start, 
      self.location.byte_index_stop,
      self.text,
      self.terminal.name,
      self.terminal.id,
      self.location.start,
      self.location.stop,
    )
  }
}


impl<'a> Token<'a> {
  

  // 提供一个方法快速创建 start token
  pub fn start(channel: usize) -> Self {
    let pos = Position { line: 0, char_position: 0 } ;
    Token {
      // token_type: 0, // start 的编号是 0
      // token_name: "_START".to_owned(),
      terminal: Terminal::new("_START", 0),
      location: Location::new(pos, pos, 0, 0),
      channel,
      text: "_START".to_owned(),
      token_index: 0,
    }
  }


  pub fn new(
    token_type: usize, 
    token_name: &'a str, 
    text: &str, 
    location: Location,
    token_index: usize, 
    channel:   usize, 
  ) -> Self {
    Token {
      terminal: Terminal::new(token_name, token_type),
      location,
      token_index,      channel,      
      text: text.to_owned(),
    }
  }


  pub fn to_string(&self) -> String {
    format!("[@{}, {}:{}='{}', <{}>, <{}>, start: <{}>, stop: <{}>]", 
      self.token_index, 
      self.location.byte_index_start, 
      self.location.byte_index_stop,
      self.text,
      self.terminal.name,
      self.terminal.id,
      self.location.start,
      self.location.stop,
    )
  }

}






