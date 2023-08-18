use std::fmt::Display;

use super::{terminal_context::TerminalContext, rule_context::RuleContext, error_context::ErrorContext};


#[derive(Clone, Debug)]
pub enum ASTContext {
  Terminal(TerminalContext),
  Rule(RuleContext),
  Error(ErrorContext),
}


// impl Clone for ASTContext {
//   fn clone(&self) -> Self {
//     match self {
//       Self::Ternimal(ctx) => Self::Ternimal(Rc::clone(ctx)),
//       Self::Rule(ctx) => Self::Rule(Rc::clone(ctx)),
//       Self::Error(ctx) => Self::Error(Rc::clone(ctx)),
//     }
//   }
// }

impl Display for ASTContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      ASTContext::Terminal(ctx) => write!(f, "{}", ctx),
      ASTContext::Rule(ctx) => write!(f, "{}", ctx),
      ASTContext::Error(ctx) => write!(f, "{}", ctx),
    }
  }
}

