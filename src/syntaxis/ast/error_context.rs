use std::rc::Rc;

use crate::syntaxis::{token::Token, to_any::ToAny};

use super::rule_context::RuleContext;




pub struct ErrorContext {
  pub symbol: Token,
  pub parent: Rc<RuleContext>,

  // error message
}

impl ToAny for ErrorContext {
  fn as_any(&self) -> &dyn std::any::Any { self }

  fn as_any_mut(&mut self) ->  &mut dyn std::any::Any { self }
}

// impl ErrorContext {
//   pub fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn std::any::Any> {
//     visitor.visit_errornode(self)
//   }
// }


impl ToString for ErrorContext {
  fn to_string(&self) -> String {
    todo!()
  }
}


