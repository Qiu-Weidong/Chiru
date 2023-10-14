
use std::fmt::Display;
use serde::{Serialize, ser::SerializeStruct};
use crate::runtime::token::Token;

#[derive(Clone, Debug)]
pub struct ErrorContext {

  pub symbol: ErrorSymbol,

  // 考虑添加一个 error_message
}




#[derive(Clone, Debug)]
pub enum ErrorSymbol {

  Redundant(Token),

  Mistake(Token),

  Missing,
}



impl ErrorContext {
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
      Redundant(symbol) | Mistake(symbol) => symbol.token_name.clone(),
      Missing => "missing".to_string(),
    }
  }

  pub fn redundant(symbol: &Token) -> Self {
    Self {
      symbol: ErrorSymbol::Redundant(symbol.to_owned())
    }
  }

  pub fn mistake(symbol: &Token) -> Self {
    Self {
      symbol: ErrorSymbol::Mistake(symbol.to_owned())
    }
  }

  pub fn missing() -> Self {
    Self { symbol: ErrorSymbol::Missing }
  }

}

impl Display for ErrorContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use ErrorSymbol::*;
    match &self.symbol {
      Redundant(symbol) | Mistake(symbol) => write!(f, "{}", symbol.token_name),
      Missing => write!(f, "missing"),
    }
  }
}

impl Serialize for ErrorContext {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer {
    
    use ErrorSymbol::*;
    match &self.symbol {
      Redundant(symbol) | Mistake(symbol) => {
        let mut state = serializer.serialize_struct("ErrorContext", 4)?;
        state.serialize_field("token_name", &symbol.token_name)?;
        state.serialize_field("token_type", &symbol.token_type)?;
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

