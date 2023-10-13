
use std::fmt::Display;
use serde::{Serialize, ser::SerializeStruct};
use crate::runtime::token::{Token, AbstractToken};

#[derive(Clone, Debug)]
pub struct ErrorContext {
  // 错误和多余可以使用 symbol
  pub symbol: Token,

  // pub error_message: String,
}


#[derive(Clone, Debug)]
pub enum ErrorNode {
  // 多余了一个 token, 放置识别到的 token 即可
  Redundant(Token),

  // 错误匹配了一个 token given expect, 注意将 location 配置为相同
  Mistake {
    given: Token,
    expect: AbstractToken,
  },

  // 缺少一个 token
  Missing(AbstractToken),
}



impl ErrorContext {
  pub fn get_text(&self) -> &str {
    &self.symbol.text
  }

  pub fn to_string(&self) -> String {
    todo!()
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

