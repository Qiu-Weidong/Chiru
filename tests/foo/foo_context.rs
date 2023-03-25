use std::rc::Rc;

use syntaxis::runtime::ast::{terminal_context::TerminalContext, to_rule::ToRule, rule_context::RuleContext};

use super::{foo_visitor::FooAcceptor, foo_parser::FooParser};



pub trait StartContext: FooAcceptor + ToRule {
  fn stmt_list(&self) -> Vec<Rc<dyn StmtContext>>;
}
pub trait StmtContext: FooAcceptor + ToRule {
  fn expr(&self) -> Option<Rc<dyn ExprContext>>;
}
pub trait ExprContext: FooAcceptor + ToRule {
  fn expr(&self) -> Option<Rc<dyn ExprContext>>;
  fn number(&self) -> Option<Rc<TerminalContext>>;
  fn id(&self) -> Option<Rc<TerminalContext>>;
}


impl StartContext for RuleContext {
  fn stmt_list(&self) -> Vec<Rc<dyn StmtContext>> {
    let mut result = Vec::new();
    for ctx in self.get_rule_contexts(FooParser::RULE_STMT).iter() {
      result.push(Rc::clone(ctx) as Rc<dyn StmtContext>);
    }
    result
  }
}
impl StmtContext for RuleContext {
  fn expr(&self) -> Option<Rc<dyn ExprContext>> {
    let ctx =self.get_rule_context(FooParser::RULE_EXPR, 0);
    if let Some(ctx) = ctx {
      let ctx = Rc::clone(&ctx) as Rc<dyn ExprContext>;
      return Some(ctx);
    }
    None
  }
}
impl ExprContext for RuleContext {
  fn expr(&self) -> Option<Rc<dyn ExprContext>> { 
    let ctx = self.get_rule_context(FooParser::RULE_EXPR, 0);
    if let Some(ctx) = ctx {
      Some(Rc::clone(&ctx) as Rc<dyn ExprContext>)
    } else { None }
  }
  fn number(&self) -> Option<Rc<TerminalContext>> { 
    self.get_terminal(1, 0)
  }
  fn id(&self) -> Option<Rc<TerminalContext>> { 
    self.get_terminal(2, 0)
  }
}


