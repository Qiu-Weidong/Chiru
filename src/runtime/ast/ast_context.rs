use std::fmt::Display;
use serde::Serialize;

use super::{terminal_context::TerminalContext, rule_context::RuleContext, error_context::ErrorContext};


#[derive(Clone, Debug)]
pub enum AstContext<'a> {
  Terminal(TerminalContext<'a>),
  Rule(RuleContext<'a>),
  Error(ErrorContext<'a>),
}

impl<'a> AstContext<'a> {
  pub fn to_string(&self) -> String {
    use AstContext::{Terminal, Rule, Error};

    match &self {
      Terminal(ctx) => ctx.to_string(),
      Rule(ctx) => ctx.to_string(),
      Error(ctx) => ctx.to_string(),
    }
  }
}

impl<'a> Display for AstContext<'a> {
  

  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use AstContext::{Terminal, Rule, Error};
    match &self {
      Terminal(ctx) => write!(f, "{}", ctx),
      Rule(ctx) => write!(f, "{}", ctx),
      Error(ctx) => write!(f, "{}", ctx),
    }
  }
}

impl<'a> Serialize for AstContext<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer {
    
    use AstContext::{Terminal, Rule, Error};
    match &self {
      Terminal(ctx) => ctx.serialize(serializer),
      Rule(ctx) => ctx.serialize(serializer),
      Error(ctx) => ctx.serialize(serializer),
    }
    
  }
}

