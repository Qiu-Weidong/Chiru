use std::fmt::Display;

use crate::runtime::token::Token;


#[derive(Clone, Debug)]
pub struct TerminalContext {
  pub symbol: Token,
}

impl TerminalContext {
  pub fn get_text(&self) -> &str {
    &self.symbol.text
  }

  pub fn to_string(&self) -> String {
    self.symbol.to_string()
  }
}


impl Display for TerminalContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.symbol.token_name)
  }
}
