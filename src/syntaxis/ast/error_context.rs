
use crate::syntaxis::token::Token;


pub struct ErrorContext {
  pub symbol: Token,

  // error message
}



impl ToString for ErrorContext {
  fn to_string(&self) -> String {
    todo!()
  }
}


