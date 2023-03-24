use std::rc::Rc;

use super::{terminal_context::TerminalContext, rule_context::RuleContext, error_context::ErrorContext};


pub enum ASTContext {
  Ternimal(Rc<TerminalContext>),
  Rule(Rc<RuleContext>),
  Error(Rc<ErrorContext>),
}


impl Clone for ASTContext {
  fn clone(&self) -> Self {
    match self {
      Self::Ternimal(ctx) => Self::Ternimal(Rc::clone(ctx)),
      Self::Rule(ctx) => Self::Rule(Rc::clone(ctx)),
      Self::Error(ctx) => Self::Error(Rc::clone(ctx)),
    }
  }
}



