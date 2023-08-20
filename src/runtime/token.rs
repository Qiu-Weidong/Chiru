use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Position {
  pub line: usize, 
  pub char_position: usize,
}

impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "line: {}, position: {}", self.line, self.char_position)
  }
}

#[derive(Clone, Debug)]
pub struct Token {
  pub token_type: usize, 
  pub token_name: String,

  // 起始位置和结束位置，左闭右开，行号从0开始编号
  pub start: Position,
  pub stop: Position,

  // token 所在的频道
  pub channel: usize,

  // token 的文本
  pub text: String,

  // token 在token序列中的序号(所有频道一起排号)
  pub token_index: usize,

  // token 开始的字符序号和最后一个字符的序号，左闭右开
  pub char_start_index: usize,
  pub char_stop_index: usize,
}


impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[@{}, {}:{}='{}', <{}>, <{}>, start: <{}>, stop: <{}>]", 
      self.token_index, 
      self.char_start_index, 
      self.char_stop_index,
      self.text,
      self.token_name,
      self.token_type,
      self.start,
      self.stop
    )
  }
}


impl Token {
  // pub const START : Token = Token { 
  //   token_type: todo!(), 
  //   token_name: todo!(), 
  //   start: todo!(), 
  //   stop: todo!(), 
  //   channel: todo!(), 
  //   text: todo!(), 
  //   token_index: todo!(), 
  //   char_start_index: todo!(), 
  //   char_stop_index: todo!() };
  

  // 提供一个方法快速创建 start token
  pub fn start(channel: usize) -> Self {
    Token {
      token_type: 0,
      token_name: "_START".to_owned(),
      start: Position { line: 0, char_position: 0 },
      stop: Position { line: 0, char_position: 0 },
      channel,
      text: "_START".to_owned(),
      token_index: 0,
      char_start_index: 0,
      char_stop_index: 0,
    }
  }


  pub fn new(token_type: usize, token_name: &str, text: &str, 
      start: Position, stop: Position, token_index: usize, channel: 
      usize, char_start_index: usize, char_stop_index: usize) -> Self {
    Token {
      token_type,
      token_name: token_name.to_owned(),
      start,
      stop,
      token_index,      channel,      char_start_index,      char_stop_index,
      text: text.to_owned(),
    }
  }
}






