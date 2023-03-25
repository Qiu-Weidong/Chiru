
use std::fmt::Display;

use crate::syntaxis::token::Token;


pub struct ErrorContext {
  pub symbol: Token,

  // error message
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
