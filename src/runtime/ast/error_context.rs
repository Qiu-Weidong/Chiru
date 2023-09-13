
use std::fmt::Display;
use serde::{Serialize, ser::SerializeStruct};
use crate::runtime::token::Token;

#[derive(Clone, Debug)]
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

impl Serialize for ErrorContext {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer {
      
    let mut state = serializer.serialize_struct("ErrorContext", 3)?;
    state.serialize_field("token_name", &self.symbol.token_name)?;
    state.serialize_field("token_type", &self.symbol.token_type)?;
    state.serialize_field("text", &self.symbol.text)?;
    state.end()
  }
}

