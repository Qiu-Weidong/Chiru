
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

impl ToString for ErrorContext {
  fn to_string(&self) -> String {
    todo!()
  }
}


