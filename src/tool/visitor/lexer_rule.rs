
use serde::Serialize;





// 这个 visitor 负责处理 lexer。

#[derive(Debug, Serialize, Clone)]
pub struct LexerRule {
  pub token_type: usize,
  pub token_name: String,
  pub regex: String,

  pub channel: usize,
  pub skip: bool,
}


impl LexerRule {
  pub fn new(token_type: usize, token_name: &str, regex: &str, channel: usize, skip: bool) -> Self {
    Self {
      channel, skip, token_type,
      regex: regex.to_owned(),
      token_name: token_name.to_owned(),
    }
  }
}






