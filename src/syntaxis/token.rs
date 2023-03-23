
pub struct Position {
  pub line: usize, 
  pub char_position: usize,
}


pub struct Token {
  pub token_type: usize, 

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


impl ToString for Token {
  fn to_string(&self) -> String {
    todo!()
  }
}










