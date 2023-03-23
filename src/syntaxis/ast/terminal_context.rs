use crate::syntaxis::{token::Token, to_any::ToAny};
use std::rc::Rc;

use super::{rule_context::RuleContext};




pub struct TerminalContext {
  pub symbol: Token,
  pub parent: Rc<RuleContext>, // 终结符一定有父节点，不需要 Option, 并且父节点一定是 RuleContext 。
}

impl ToAny for TerminalContext {
  fn as_any(&self) -> &dyn std::any::Any { self }

  fn as_any_mut(&mut self) ->  &mut dyn std::any::Any { self }
}

impl TerminalContext {
  pub fn accept(&self, visitor: &dyn crate::syntaxis::visitor::ASTVisitor) -> Box<dyn std::any::Any> {
    visitor.visit_terminal(self)
  }
}

