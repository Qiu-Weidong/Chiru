

use crate::syntaxis::{ast::{terminal_context::TerminalContext, error_context::ErrorContext, rule_context::RuleContext}, to_any::ToAny};


pub trait ASTListener: ToAny {
  fn visit_terminal(&self, _context: &TerminalContext) {}

  fn visit_errornode(&self, _context: &ErrorContext) {}

  fn enter_every_rule(&self, _context: &RuleContext) {}

  fn exit_every_rule(&self, _context: &RuleContext) {}
}
