use std::{rc::Rc, any::Any};

use syntaxis::syntaxis::ast::{rule_context::RuleContext, terminal_context::TerminalContext, error_context::ErrorContext};



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
pub struct HelloParser {}
impl HelloParser {
  const RULE_EXPR: usize = 1;
  const RULE_STAT: usize = 2;


  fn expr(&self) -> Rc<dyn ExprContext> {
    todo!()
  }

  fn stat(&self) -> Rc<dyn StatContext> {
    todo!()
  }
}

pub trait ExprContext {
  fn expr_list(&self) -> Vec<Rc<dyn ExprContext>>;



  fn as_rule_context(&self) -> &RuleContext;

  fn get_rule_index(&self) -> usize { HelloParser::RULE_EXPR }

  fn enter_rule(&self, listener: &dyn HelloListener);

  fn exit_rule(&self, listener: &dyn HelloListener);

  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any>;
}

pub trait StatContext {



  fn as_rule_context(&self) -> &RuleContext ;

  fn get_rule_index(&self) -> usize { HelloParser::RULE_EXPR }

  fn enter_rule(&self, listener: &dyn HelloListener);

  fn exit_rule(&self, listener: &dyn HelloListener);

  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any>;
}

impl ExprContext for RuleContext {
  fn expr_list(&self) -> Vec<Rc<dyn ExprContext>> {
    let mut result = Vec::new();
    for context in self.get_rule_contexts(HelloParser::RULE_EXPR).iter() {
      result.push(Rc::clone(context) as Rc<dyn ExprContext>);
    }
    result
  }

  fn as_rule_context(&self) -> &RuleContext { self }

  fn enter_rule(&self, listener: &dyn HelloListener) {
    listener.enter_expr(self)
  }

  fn exit_rule(&self, listener: &dyn HelloListener) {
    listener.exit_expr(self)
  }

  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any> {
    visitor.visit_expr(self)
  }
}

impl StatContext for RuleContext {
  fn as_rule_context(&self) -> &RuleContext { self }

  fn enter_rule(&self, _listener: &dyn HelloListener) {
    todo!()
  }

  fn exit_rule(&self, _listener: &dyn HelloListener) {
    todo!()
  }

  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any> {
    visitor.visit_stat(self)
  }
}


// expr_listener.rs
pub trait HelloListener {
  fn enter_expr(&self, _ctx: &dyn ExprContext) {}

  fn exit_expr(&self, _ctx: &dyn ExprContext) {}


  fn enter_every_rule(&self, _ctx: &RuleContext) {}

  fn exit_every_rule(&self, _ctx: &RuleContext) {}
}












pub trait HelloAcceptor {
  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any>;
}
impl HelloAcceptor for RuleContext {
  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any> {
    visitor.visit_rule(self)
  }
}
impl HelloAcceptor for TerminalContext {
  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any> {
    visitor.visit_terminal(self)
  }
}
impl HelloAcceptor for ErrorContext {
  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any> {
    visitor.visit_errornode(self)
  }
}



// visitor
pub trait HelloVisitor {
  fn visit_expr(&self, ctx: &dyn ExprContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule_context())
  }

  fn visit_stat(&self, ctx: &dyn StatContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule_context())
  }



  fn visit_rule(&self, ctx: &RuleContext) -> Box<dyn Any> {
    match ctx.get_rule_index() {
      HelloParser::RULE_EXPR => self.visit_expr(ctx),
      HelloParser::RULE_STAT => self.visit_stat(ctx),


      _ => self.visit_children(ctx)
    }
  }

  fn visit_terminal(&self, _terminal: &TerminalContext) -> Box<dyn Any>  { self.default_result() }

  fn visit_errornode(&self, _errornode: &ErrorContext) -> Box<dyn Any>  { self.default_result() }

  fn visit(&self, ctx: &dyn HelloAcceptor) -> Box<dyn Any> ;

  fn visit_children(&self, ctx: &RuleContext) -> Box<dyn Any> {
    let mut result = self.default_result();
    for child in ctx.children.iter() {
      
    }
    result
  }

  fn default_result(&self) -> Box<dyn Any> { Box::new(()) }

  fn aggregate_result(&self, _aggregate: Box<dyn Any> , next_result: Box<dyn Any> ) -> Box<dyn Any>  { next_result }

  fn should_visit_next_child(&self, _context: &RuleContext, _current_result: &dyn Any) -> bool {true}

}


pub struct HelloBaseVisitor {
  // 定义需要的数据结构
}

impl HelloVisitor for HelloBaseVisitor {
  fn visit(&self, ctx: &dyn HelloAcceptor) -> Box<dyn Any> {
    ctx.accept(self)
  }
}



fn main() {
  print!("hello world!");

  let parser = HelloParser {};


  let ast = parser.expr();

  let visitor = HelloBaseVisitor {};

  ast.accept(&visitor);

  visitor.visit_expr(ast.as_ref());
  // ExprContext::accept(&ast, &visitor);

  // visitor.visit(ast.as_any());

  // let ast = parser.stat();
  // ast.accept(&visitor);

  // visitor.visit(ast.as_any());

  // let ast = ast.as_any().downcast_ref::<RuleContext>().unwrap();
  // ast.accept(&visitor);

  // visitor.visit(ast.as_any());



}

