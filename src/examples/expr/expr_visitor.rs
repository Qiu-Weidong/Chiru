use std::{any::Any, rc::Rc};

use crate::syntaxis::ast::{ASTVisitor, Acceptable, RuleContext, TerminalContext, ErrorContext};

use super::expr_parser::{ProgContext, StatContext, ExprContext};





pub trait AbstractExprVisitor: ASTVisitor {
  fn visit_prog(&self, ctx: &dyn ProgContext) -> Box<dyn Any>;

  fn visit_stat(&self, ctx: &dyn StatContext) -> Box<dyn Any>;

  fn visit_expr(&self, ctx: &dyn ExprContext) -> Box<dyn Any>;
}


pub struct ExprVisitor {
  // 在这里定义需要的数据结构
}

impl ASTVisitor for ExprVisitor {
  fn visit(&self, ast: &dyn Acceptable) -> Box<dyn Any> { ast.accept(self) }

  fn visit_children(&self, context: &crate::syntaxis::ast::RuleContext) -> Box<dyn Any>  {
    let mut result = self.default_result();
    for child in context.get_children().iter() {
      if ! self.should_visit_next_child(context, &result) {
        break;
      }

      // 只能够分别判断是否是 RuleContext, TernimalContext, ErrorContext 。
      if child.is::<RuleContext>() {
        let child = Rc::clone(child).downcast::<RuleContext>().unwrap();
        // result = child.accept(self);
        todo!()
      }
      else if child.is::<TerminalContext>() {
        let child = Rc::clone(child).downcast::<TerminalContext>().unwrap();
        result = child.accept(self);
      }
      else if child.is::<ErrorContext>() {
        let child = Rc::clone(child).downcast::<TerminalContext>().unwrap();
        result = child.accept(self);
      }

    }
    result
  }

  fn default_result(&self) -> Box<dyn Any> {
    todo!()
  }

  fn as_any(&self) -> &dyn Any {
    self as &dyn Any 
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any 
  }

}

impl AbstractExprVisitor for ExprVisitor {
  fn visit_prog(&self, ctx: &dyn ProgContext) -> Box<dyn Any> {
    let ctx = ctx.as_any().downcast_ref::<RuleContext>().unwrap();
    self.visit_children(ctx)
  }

  fn visit_stat(&self, ctx: &dyn StatContext) -> Box<dyn Any> {
    let ctx = ctx.as_any().downcast_ref::<RuleContext>().unwrap();
    self.visit_children(ctx)
  }

  fn visit_expr(&self, ctx: &dyn ExprContext) -> Box<dyn Any> {
    let ctx = ctx.as_any().downcast_ref::<RuleContext>().unwrap();
    self.visit_children(ctx)
  }
}







