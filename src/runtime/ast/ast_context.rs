use std::fmt::Display;
use serde::Serialize;
// use serde_json::Serializer;

use super::{terminal_context::TerminalContext, rule_context::RuleContext, error_context::ErrorContext};


#[derive(Clone, Debug, Serialize)]
pub enum ASTContext {
  Terminal(TerminalContext),
  Rule(RuleContext),
  Error(ErrorContext),
}

impl Display for ASTContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      ASTContext::Terminal(ctx) => write!(f, "{}", ctx),
      ASTContext::Rule(ctx) => write!(f, "{}", ctx),
      ASTContext::Error(ctx) => write!(f, "{}", ctx),
    }
  }
}

