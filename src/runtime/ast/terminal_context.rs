use std::fmt::Display;
use serde::{Serialize, ser::SerializeStruct};
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

  pub fn new(symbol: &Token) -> Self {
    Self {  symbol: symbol.to_owned(), }
  }
}


impl Display for TerminalContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.symbol.token_name)
  }
}

impl Serialize for TerminalContext {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer {
    
      let mut state = serializer.serialize_struct("TerminalContext", 3)?;
      state.serialize_field("token_name", &self.symbol.token_name)?;
      state.serialize_field("token_type", &self.symbol.token_type)?;
      state.serialize_field("text", &self.symbol.text)?;
      state.end()
  }
}
