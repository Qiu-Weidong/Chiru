use crate::syntaxis::token::Token;


pub struct TerminalContext {
  pub symbol: Token,
}

impl TerminalContext {
  pub fn get_text(&self) -> &str {
    &self.symbol.text
  }
}


impl ToString for TerminalContext {
  fn to_string(&self) -> String {
    todo!()
  }
}

