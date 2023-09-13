
use std::fmt::Display;
use serde::Serialize;
use crate::runtime::token::Token;

#[derive(Clone, Debug, Serialize)]
pub struct ErrorContext {
  pub symbol: Token,

  // error message
  // 多余、缺少和错误，其中
}


impl ErrorContext {
  pub fn get_text(&self) -> &str {
    &self.symbol.text
  }
}

impl Display for ErrorContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.symbol.token_name)
  }
}
