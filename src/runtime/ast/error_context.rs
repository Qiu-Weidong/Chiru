
use std::fmt::Display;
use serde::{Serialize, ser::SerializeStruct};
use crate::runtime::token::Token;

#[derive(Clone, Debug)]
pub struct ErrorContext<'a> {

  pub symbol: ErrorSymbol<'a>,

  // 考虑添加一个 error_message
}




#[derive(Clone, Debug)]
pub enum ErrorSymbol<'a> {

  Redundant(Token<'a>),

  Mistake(Token<'a>),

  Missing,
}



impl<'a> ErrorContext<'a> {
  pub fn get_text(&self) -> &str {
    use ErrorSymbol::*;
    match &self.symbol {
      Missing => "<missing>",
      Redundant(symbol) | Mistake(symbol) => &symbol.text
    }
  }

  pub fn to_string(&self) -> String {
    use ErrorSymbol::*;
    match &self.symbol {
      Redundant(symbol) | Mistake(symbol) => symbol.terminal.name.to_string(),
      Missing => "missing".to_string(),
    }
  }

  pub fn redundant(symbol: &Token<'a>) -> Self {
    Self {
      symbol: ErrorSymbol::Redundant(symbol.to_owned())
    }
  }

  pub fn mistake(symbol: &Token<'a>) -> Self {
    Self {
      symbol: ErrorSymbol::Mistake(symbol.to_owned())
    }
  }

  pub fn missing() -> Self {
    Self { symbol: ErrorSymbol::Missing }
  }

}

impl<'a> Display for ErrorContext<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use ErrorSymbol::*;
    match &self.symbol {
      Redundant(symbol) | Mistake(symbol) => write!(f, "{}", symbol.terminal.name),
      Missing => write!(f, "missing"),
    }
  }
}

impl<'a> Serialize for ErrorContext<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer {
    
    use ErrorSymbol::*;
    match &self.symbol {
      Redundant(symbol) | Mistake(symbol) => {
        let mut state = serializer.serialize_struct("ErrorContext", 4)?;
        state.serialize_field("token_name", &symbol.terminal.name)?;
        state.serialize_field("token_type", &symbol.terminal.id)?;
        state.serialize_field("text", &symbol.text)?;
        
        let mut error_type = "redundant";
        if let Mistake(_) = self.symbol {
          error_type = "mistake";
        }

        state.serialize_field("error_type", error_type)?;
        
        // 考虑添加 error message
        state.end()
      },
      Missing => {
        let mut state = serializer.serialize_struct("ErrorContext", 1)?;
        state.serialize_field("error_type", "missing")?;
        // 考虑添加 message
        state.end()
      },
    }
  }
}

