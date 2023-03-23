use std::rc::Rc;

use crate::syntaxis::{token::Token, to_any::ToAny};

use super::{rule_context::RuleContext, acceptable::Acceptable};




pub struct ErrorContext {
  pub symbol: Token,
  pub parent: Rc<RuleContext>,

  // error message
}

impl ToAny for ErrorContext {
  fn as_any(&self) -> &dyn std::any::Any { self }

  fn as_any_mut(&mut self) ->  &mut dyn std::any::Any { self }
}

impl Acceptable for ErrorContext {
  fn accept(&self, visitor: &dyn crate::syntaxis::visitor::ASTVisitor) -> Box<dyn std::any::Any> {
    visitor.visit_errornode(self)
  }
}



