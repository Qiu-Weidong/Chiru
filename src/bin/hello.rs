use std::{rc::Rc, any::Any};

use syntaxis::syntaxis::{ast::{rule_context::RuleContext, terminal_context::TerminalContext, error_context::ErrorContext}, to_any::ToAny, listener::ast_listener::ASTListener, visitor::ast_visitor::ASTVisitor};

/*
 * 定义一个 expr 语法来测试
 * grammar expr;
 * expr: 
 *   expr + expr
 *   | expr * expr
 *   | '(' expr ')'
 *   | Number
 * ;
 */

// expr_parser.rs
pub struct ExprParser {}
impl ExprParser {
  const RULE_EXPR: usize = 1;


  fn expr(&self) -> Rc<dyn ExprContext> {
    todo!()
  }

  fn stat(&self) -> Rc<dyn StatContext> {
    todo!()
  }
}

pub trait ExprContext: ToAny {
  fn expr_list(&self) -> Vec<Rc<dyn ExprContext>>;

  fn get_rule_index(&self) -> usize { ExprParser::RULE_EXPR }

  fn enter_rule(&self, listener: &dyn ASTListener);

  fn exit_rule(&self, listener: &dyn ASTListener);

  fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any>;
}

pub trait StatContext: ToAny {
  fn get_rule_index(&self) -> usize { ExprParser::RULE_EXPR }

  fn enter_rule(&self, listener: &dyn ASTListener);

  fn exit_rule(&self, listener: &dyn ASTListener);

  fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any>;
}

impl ExprContext for RuleContext {
  fn expr_list(&self) -> Vec<Rc<dyn ExprContext>> {
    let mut result = Vec::new();
    for context in self.get_rule_contexts(ExprParser::RULE_EXPR).iter() {
      result.push(Rc::clone(context) as Rc<dyn ExprContext>);
    }
    result
  }

  fn enter_rule(&self, listener: &dyn ASTListener) {
    if listener.as_any().is::<ExprListener>() {
      // 注意, enter 的是自己。
      listener.as_any().downcast_ref::<ExprListener>().unwrap().enter_expr(self);
    }
  }

  fn exit_rule(&self, listener: &dyn ASTListener) {
    if listener.as_any().is::<ExprListener>() {
      // 注意, enter 的是自己。
      listener.as_any().downcast_ref::<ExprListener>().unwrap().exit_expr(self);
    }
  }

  fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> {
    if visitor.as_any().is::<ExprVisitor>() {
      visitor.as_any().downcast_ref::<ExprVisitor>().unwrap().visit_expr(self)
    } else {
      visitor.visit_children(self)
    }
  }
}

impl StatContext for RuleContext {
  fn enter_rule(&self, listener: &dyn ASTListener) {
    todo!()
  }

  fn exit_rule(&self, listener: &dyn ASTListener) {
    todo!()
  }

  fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> {
    todo!()
  }
}

// expr_listener.rs
pub struct ExprListener {}

impl ToAny for ExprListener {
  fn as_any(&self) -> &dyn std::any::Any { self }

  fn as_any_mut(&mut self) ->  &mut dyn std::any::Any { self }
}

impl ASTListener for ExprListener {}


impl ExprListener {
  fn enter_expr(&self, _ctx: &dyn ExprContext) {}

  fn exit_expr(&self, _ctx: &dyn ExprContext) {}
}




// expr_visitor.rs
pub struct ExprVisitor {}

impl ToAny for ExprVisitor {
  fn as_any(&self) -> &dyn std::any::Any { self }

  fn as_any_mut(&mut self) ->  &mut dyn std::any::Any { self }
}

impl ASTVisitor for ExprVisitor {
  fn visit(&self, ast: &dyn Any) -> Box<dyn Any> {

    if ast.is::<TerminalContext>() {
      let ast = ast.downcast_ref::<TerminalContext>().unwrap();
      ast.accept(self)
    }
    else if ast.is::<ErrorContext>() {
      let ast = ast.downcast_ref::<ErrorContext>().unwrap();
      ast.accept(self)
    }
    else if ast.is::<RuleContext>() {
      let ast = ast.downcast_ref::<RuleContext>().unwrap();
      return match ast.get_rule_index() {
        // 根据 rule index 选择合适的 visit 函数
        ExprParser::RULE_EXPR => (ast as &dyn ExprContext).accept(self),
        _ => todo!()
      }
    }
    else {
      todo!()
    }
    
  }

  fn visit_children(&self, context: &RuleContext) -> Box<dyn Any> {
    let mut result = self.default_result();
    
    for child in context.children.iter() {
      if child.is::<TerminalContext>() {  
        let child = Rc::clone(child).downcast::<TerminalContext>().unwrap();
        result = child.accept(self);
      }
      else if child.is::<ErrorContext>() {
        let child = Rc::clone(child).downcast::<ErrorContext>().unwrap();
        result = child.accept(self);
      }
      else if child.is::<RuleContext>() {
        let child = Rc::clone(child).downcast::<RuleContext>().unwrap();
        result = self.visit(child.as_ref());
      }
    }
    
    result
  }

}

impl ExprVisitor {
  fn visit_expr(&self, ctx: &dyn ExprContext) -> Box<dyn Any> {
    let ctx = ctx.as_any().downcast_ref::<RuleContext>().unwrap();
    self.visit_children(ctx)    
  }
}


fn main() {
  print!("hello world!");

  let parser = ExprParser {};


  let ast = parser.expr();

  let visitor = ExprVisitor {};

  ast.accept(&visitor);

  visitor.visit(ast.as_any());

  let ast = parser.stat();
  ast.accept(&visitor);

  visitor.visit(ast.as_any());

  let ast = ast.as_any().downcast_ref::<RuleContext>().unwrap();
  ast.accept(&visitor);

  visitor.visit(ast.as_any());



}

