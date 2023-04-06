use std::any::Any;

use syntaxis::runtime::ast::{rule_context::RuleContext, ast_context::ASTContext, error_context::ErrorContext, terminal_context::TerminalContext};

use super::{foo_context::{ExprContext, StartContext, StmtContext}, foo_parser::FooParser};

pub trait FooAcceptor { fn accept(&self, visitor: &dyn FooVisitor) -> Box<dyn Any>; }
impl FooAcceptor for RuleContext {
  fn accept(&self, visitor: &dyn FooVisitor) -> Box<dyn Any> {
    visitor.visit_rule(self)
  }
}
impl FooAcceptor for TerminalContext {
  fn accept(&self, visitor: &dyn FooVisitor) -> Box<dyn Any> {
    visitor.visit_terminal(self)
  }
}
impl FooAcceptor for ErrorContext {
  fn accept(&self, visitor: &dyn FooVisitor) -> Box<dyn Any> {
    visitor.visit_errornode(self)
  }
}

pub trait FooVisitor {
  fn visit_start(&self, ctx: &dyn StartContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  fn visit_expr(&self, ctx: &dyn ExprContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  fn visit_stmt(&self, ctx: &dyn StmtContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }


  fn visit_rule(&self, ctx: &RuleContext) -> Box<dyn Any> {
    match ctx.get_rule_index() {
      FooParser::RULE_START => self.visit_start(ctx),
      FooParser::RULE_EXPR => self.visit_expr(ctx),
      FooParser::RULE_STMT => self.visit_stmt(ctx),


      _ => self.visit_children(ctx)
    }
  }

  fn visit_terminal(&self, _terminal: &TerminalContext) -> Box<dyn Any>  { self.default_result() }

  fn visit_errornode(&self, _errornode: &ErrorContext) -> Box<dyn Any>  { self.default_result() }

  fn visit_children(&self, ctx: &RuleContext) -> Box<dyn Any> {
    let mut result = self.default_result();
    for child in ctx.children.iter() {
      if ! self.should_visit_next_child(ctx, &result) { break; }

      let child_result = match child {
        ASTContext::Terminal(ctx) => self.visit_terminal(ctx),
        ASTContext::Rule(ctx) => self.visit_rule(ctx),
        ASTContext::Error(ctx) => self.visit_errornode(ctx),
      };

      result = self.aggregate_result(result, child_result);
    }
    result
  }

  fn default_result(&self) -> Box<dyn Any> { Box::new(()) }

  fn aggregate_result(&self, _aggregate: Box<dyn Any> , next_result: Box<dyn Any> ) -> Box<dyn Any>  { next_result }

  fn should_visit_next_child(&self, _context: &RuleContext, _current_result: &dyn Any) -> bool {true}
}


pub struct FooBaseVisitor;
impl FooVisitor for FooBaseVisitor {
  fn visit_expr(&self, ctx: &dyn ExprContext) -> Box<dyn Any> {
    if let Some(ctx) = ctx.number() {
      println!("{}", ctx.symbol.text);
    }

    self.visit_children(ctx.as_rule())
  }
}

