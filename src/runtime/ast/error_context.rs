
use std::fmt::Display;
use serde::Serialize;
use crate::runtime::token::Token;

#[derive(Clone, Debug)]
pub struct ErrorContext {

  pub symbol: ErrorNode,
}




#[derive(Clone, Debug)]
pub enum ErrorNode {
  // 多余了一个 token, 放置识别到的 token 即可
  Redundant(Token),

  // 错误匹配了一个 token given expect
  Mistake(Token),

  // 缺少一个 token
  Missing,
}



impl ErrorContext {
  pub fn get_text(&self) -> &str {
    // &self.symbol.text
    todo!()
  }

  pub fn to_string(&self) -> String {
    todo!()
  }

  pub fn redundant(symbol: &Token) -> Self {
    Self {
      symbol: ErrorNode::Redundant(symbol.to_owned())
    }
  }

  pub fn mistake(symbol: &Token) -> Self {
    Self {
      symbol: ErrorNode::Mistake(symbol.to_owned())
    }
  }

  pub fn missing() -> Self {
    Self { symbol: ErrorNode::Missing }
  }

}

impl Display for ErrorContext {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // write!(f, "{}", self.symbol.token_name)
    todo!()
  }
}

impl Serialize for ErrorContext {
  fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer {
      
    // let mut state = serializer.serialize_struct("ErrorContext", 3)?;
    // state.serialize_field("token_name", &self.symbol.token_name)?;
    // state.serialize_field("token_type", &self.symbol.token_type)?;
    // state.serialize_field("text", &self.symbol.text)?;
    // state.end()
    todo!()
    
  }
}

