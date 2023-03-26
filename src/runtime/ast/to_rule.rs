use super::rule_context::RuleContext;


pub trait ToRule {
  fn as_rule(&self) -> &RuleContext;

  fn as_mut_rule(&mut self) -> &mut RuleContext;
}

