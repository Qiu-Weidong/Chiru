use syntaxis::runtime::ast::{rule_context::RuleContext, error_context::ErrorContext, terminal_context::TerminalContext};

use super::{foo_parser::FooParser, foo_context::{ExprContext, StmtContext, StartContext}};


pub trait FooListener {
  fn enter_expr(&self, _ctx: &dyn ExprContext) {}

  fn exit_expr(&self, _ctx: &dyn ExprContext) {}

  fn enter_stmt(&self, _ctx: &dyn StmtContext) {}

  fn exit_stmt(&self, _ctx: &dyn StmtContext) {}

  fn enter_start(&self, _ctx: &dyn StartContext) {}

  fn exit_start(&self, _ctx: &dyn StartContext) {}





  fn enter_every_rule(&self, _ctx: &RuleContext) {}

  fn exit_every_rule(&self, _ctx: &RuleContext) {}






  fn enter_rule(&self, ctx: &RuleContext) {
    // 在这里进行派发即可
    match ctx.get_rule_index() {
      FooParser::RULE_EXPR => self.enter_expr(ctx),
      FooParser::RULE_START => self.enter_start(ctx),
      FooParser::RULE_STMT => self.enter_stmt(ctx),


      _ => {}
    }
  }

  fn exit_rule(&self, ctx: &RuleContext) {
    match ctx.get_rule_index() {
      FooParser::RULE_EXPR => self.exit_expr(ctx),
      FooParser::RULE_START => self.exit_start(ctx),
      FooParser::RULE_STMT => self.exit_stmt(ctx),


      _ => {}
    }
  }

  fn enter_terminal(&self, _ctx: &TerminalContext) {}

  fn exit_terminal(&self, _ctx: &TerminalContext) {}

  fn enter_errornode(&self, _ctx: &ErrorContext) {}

  fn exit_errornode(&self, _ctx: &ErrorContext) {}
}




pub struct FooBaseListener;

impl FooListener for FooBaseListener {
  fn enter_expr(&self, _ctx: &dyn ExprContext) {
    println!("enter expr")
  }

  fn exit_expr(&self, _ctx: &dyn ExprContext) {
    println!("exit expr")
  }

  fn enter_stmt(&self, _ctx: &dyn StmtContext) {
    println!("enter stmt")
  }

  fn exit_stmt(&self, _ctx: &dyn StmtContext) {
    println!("exit stmt")
  }
}

