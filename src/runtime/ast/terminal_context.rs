use std::fmt::Display;
use serde::{Serialize, ser::SerializeStruct};
use crate::runtime::token::Token;


#[derive(Clone, Debug)]
pub struct TerminalContext<'a> {
  pub symbol: Token<'a>,
}

impl<'a> TerminalContext<'a> {
  pub fn get_text(&self) -> &str {
    &self.symbol.text
  }

  pub fn to_string(&self) -> String {
    self.symbol.terminal.name.to_string()
  }

  pub fn new(symbol: &Token<'a>) -> Self {
    Self {  symbol: symbol.to_owned(), }
  }
}


impl<'a> Display for TerminalContext<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.symbol.terminal.name)
  }
}

impl<'a> Serialize for TerminalContext<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer {
    
      let mut state = serializer.serialize_struct("TerminalContext", 3)?;
      state.serialize_field("token_name", &self.symbol.terminal.name)?;
      state.serialize_field("token_type", &self.symbol.terminal.id)?;
      state.serialize_field("text", &self.symbol.text)?;
      state.end()
  }
}
