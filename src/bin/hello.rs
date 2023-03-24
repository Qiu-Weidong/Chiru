use std::{rc::Rc, any::Any};

use syntaxis::syntaxis::ast::{rule_context::RuleContext, terminal_context::TerminalContext, error_context::ErrorContext, ast_context::ASTContext};

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









pub trait ExprContext: HelloAcceptable + HelloWalkable {
  fn expr_list(&self) -> Vec<Rc<dyn ExprContext>>;



  fn as_rule_context(&self) -> &RuleContext;

  fn get_rule_index(&self) -> usize { HelloParser::RULE_EXPR }

}

pub trait StatContext: HelloAcceptable + HelloWalkable {



  fn as_rule_context(&self) -> &RuleContext ;

  fn get_rule_index(&self) -> usize { HelloParser::RULE_EXPR }

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
}

impl StatContext for RuleContext {
  fn as_rule_context(&self) -> &RuleContext { self }


}






pub trait HelloWalkable {
  fn enter(&self, listener: &dyn HelloListener) ;

  fn exit(&self, listener: &dyn HelloListener) ;
}

impl HelloWalkable for RuleContext {
  fn enter(&self, listener: &dyn HelloListener)  {
    listener.enter_rule(self)
  }

  fn exit(&self, listener: &dyn HelloListener)  {
    listener.exit_rule(self)
  }
}
impl HelloWalkable for TerminalContext {
  fn enter(&self, listener: &dyn HelloListener)  {
    listener.enter_terminal(self)
  }

  fn exit(&self, listener: &dyn HelloListener)  {
    listener.exit_terminal(self)
  }
}
impl HelloWalkable for ErrorContext {
  fn enter(&self, listener: &dyn HelloListener) {
    listener.enter_errornode(self)
  }

  fn exit(&self, listener: &dyn HelloListener) {
    listener.exit_errornode(self)
  }
}

// expr_listener.rs
pub trait HelloListener {
  fn enter_expr(&self, _ctx: &dyn ExprContext) {}

  fn exit_expr(&self, _ctx: &dyn ExprContext) {}




  fn enter_every_rule(&self, _ctx: &RuleContext) {}

  fn exit_every_rule(&self, _ctx: &RuleContext) {}



  fn enter_rule(&self, ctx: &RuleContext) {
    // 在这里进行派发即可
  }

  fn exit_rule(&self, ctx: &RuleContext) {}

  fn enter_terminal(&self, ctx: &TerminalContext) {}

  fn exit_terminal(&self, ctx: &TerminalContext) {}

  fn enter_errornode(&self, ctx: &ErrorContext) {}

  fn exit_errornode(&self, ctx: &ErrorContext) {}
}



pub trait HelloWalker {
  // 类似于 visit 
  fn walk(&self, listener: &dyn HelloListener, ast: &dyn HelloWalkable) {
    // listener.enter_every_rule(ast)
    ast.enter(listener);


    ast.exit(listener);
  }

  // 提供前序遍历和后序遍历

  // fn enter(&self, rule: &dyn HelloWalkable, listener: &dyn HelloListener);
  // fn exit(&self, rule: &dyn HelloWalkable, listener: &dyn HelloListener);
}








pub trait HelloAcceptable {
  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any>;
}
impl HelloAcceptable for RuleContext {
  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any> {
    visitor.visit_rule(self)
  }
}
impl HelloAcceptable for TerminalContext {
  fn accept(&self, visitor: &dyn HelloVisitor) -> Box<dyn Any> {
    visitor.visit_terminal(self)
  }
}
impl HelloAcceptable for ErrorContext {
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

  fn visit(&self, ctx: &dyn HelloAcceptable) -> Box<dyn Any> ;

  fn visit_children(&self, ctx: &RuleContext) -> Box<dyn Any> {
    let mut result = self.default_result();
    for child in ctx.children.iter() {
      if ! self.should_visit_next_child(ctx, &result) { break; }

      let child_result = match child {
        ASTContext::Ternimal(ctx) => self.visit_terminal(ctx),
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


pub struct HelloBaseVisitor {
  // 定义需要的数据结构
}

impl HelloVisitor for HelloBaseVisitor {
  fn visit(&self, ctx: &dyn HelloAcceptable) -> Box<dyn Any> {
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



}

