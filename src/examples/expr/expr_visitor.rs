use std::{rc::Rc, any::Any};

use crate::syntaxis::ast::{ASTVisitor, ASTContext};

use super::expr_parser::{ProgContext, StatContext, ExprContext};





pub trait AbstractExprVisitor: ASTVisitor {
  fn visit_prog(&self, ctx: &dyn ProgContext) -> Rc<dyn Any>;

  fn visit_stat(&self, ctx: &dyn StatContext) -> Rc<dyn Any>;

  fn visit_expr(&self, ctx: &dyn ExprContext) -> Rc<dyn Any>;
}


pub struct ExprVisitor {
  // 在这里定义需要的数据结构
}

impl ASTVisitor for ExprVisitor {
  fn visit(&self, ast: &dyn ASTContext) -> Rc<dyn Any> { ast.accept(self) }

  fn visit_children(&self, context: &crate::syntaxis::ast::RuleContext) -> Rc<dyn Any>  {
    let mut result = self.default_result();
    for child in context.get_children().iter() {
      if ! self.should_visit_next_child(context, &result) {
        break;
      }
      let child_result = child.accept(self);
      result = self.aggregate_result(result, child_result);
    }
    result
  }

  fn default_result(&self) -> Rc<dyn Any> {
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
  fn visit_prog(&self, _ctx: &dyn ProgContext) -> Rc<dyn Any> {
    // 需要 downcast 转换为 RuleContext 。
    // self.visit_children(ctx)
    todo!()
  }

  fn visit_stat(&self, _ctx: &dyn StatContext) -> Rc<dyn Any> {
    todo!()
  }

  fn visit_expr(&self, _ctx: &dyn ExprContext) -> Rc<dyn Any> {
    todo!()
  }
}







