use std::fmt::Display;

use crate::syntaxis::token::Token;


pub struct TerminalContext {
  pub symbol: Token,
}

impl TerminalContext {
  pub fn get_text(&self) -> &str {
    &self.symbol.text
  }
}


impl Display for TerminalContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.symbol.token_name)
  }
}
