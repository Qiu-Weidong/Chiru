
use super::{position::Position, location::Location};
use std::fmt::Display;


#[derive(Clone, Debug)]
pub struct Token {
  pub token_type: usize, 
  pub token_name: String,


  
  // token 所在的位置
  pub location: Location,
  
  // token 所在的频道
  pub channel: usize,

  // token 的文本
  pub text: String,

  // token 在token序列中的序号(所有频道一起排号)
  pub token_index: usize,
}

// #[derive(Clone, Debug)]
// pub struct AbstractToken {
//   pub token_type: usize, 
//   pub token_name: String,
// }

// impl AbstractToken {
//   pub fn new(token_type: usize, token_name: &str) -> Self {
//     Self {
//       token_name: token_name.to_owned(),
//       token_type,
//     }
//   }
// }



impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[@{}, {}:{}='{}', <{}>, <{}>, start: <{}>, stop: <{}>]", 
      self.token_index, 
      self.location.byte_index_start, 
      self.location.byte_index_stop,
      self.text,
      self.token_name,
      self.token_type,
      self.location.start,
      self.location.stop,
    )
  }
}


impl Token {
  

  // 提供一个方法快速创建 start token
  pub fn start(channel: usize) -> Self {
    let pos = Position { line: 0, char_position: 0 } ;
    Token {
      token_type: 0, // start 的编号是 0
      token_name: "_START".to_owned(),
      location: Location::new(pos, pos, 0, 0),
      channel,
      text: "_START".to_owned(),
      token_index: 0,
    }
  }


  pub fn new(
    token_type: usize, 
    token_name: &str, 
    text: &str, 
    location: Location,
    token_index: usize, 
    channel:   usize, 
  ) -> Self {
    Token {
      token_type,
      token_name: token_name.to_owned(),
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
      self.token_name,
      self.token_type,
      self.location.start,
      self.location.stop,
    )
  }

}






