use std::{rc::Rc, any::Any};

use syntaxis::syntaxis::ast::{rule_context::RuleContext, terminal_context::TerminalContext, error_context::ErrorContext, ast_context::ASTContext, to_rule::ToRule};

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









pub trait ExprContext: HelloAcceptor + ToRule {
  fn expr_list(&self) -> Vec<Rc<dyn ExprContext>>;


  fn get_rule_index(&self) -> usize { HelloParser::RULE_EXPR }

}

pub trait StatContext: HelloAcceptor + ToRule {



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

  // fn as_rule_context(&self) -> &RuleContext { self }
}

impl StatContext for RuleContext {

  // fn as_rule_context(&self) -> &RuleContext { self }

}



// expr_listener.rs
pub trait HelloListener {
  fn enter_expr(&self, _ctx: &dyn ExprContext) {}

  fn exit_expr(&self, _ctx: &dyn ExprContext) {}







  fn enter_every_rule(&self, _ctx: &RuleContext) {}

  fn exit_every_rule(&self, _ctx: &RuleContext) {}






  fn enter_rule(&self, ctx: &RuleContext) {
    // 在这里进行派发即可
    match ctx.get_rule_index() {
      HelloParser::RULE_EXPR => self.enter_expr(ctx),


      _ => todo!()
    }
  }

  fn exit_rule(&self, ctx: &RuleContext) {
    match ctx.get_rule_index() {
      HelloParser::RULE_EXPR => self.exit_expr(ctx),


      _ => todo!()
    }
  }

  fn enter_terminal(&self, _ctx: &TerminalContext) {}

  fn exit_terminal(&self, _ctx: &TerminalContext) {}

  fn enter_errornode(&self, _ctx: &ErrorContext) {}

  fn exit_errornode(&self, _ctx: &ErrorContext) {}
}



// 不如直接将 walker 定义为 struct 。
pub trait HelloWalker {
  // 类似于 visit , 使用 Walkerable 的好处是 xxxContext 都可以作为参数传入, 而不一定都是 RuleContext 。
  fn walk(&self, listener: &dyn HelloListener, ast: &RuleContext) {
    let ast = ast.as_rule();

    listener.enter_every_rule(ast);
    listener.enter_rule(ast);

    for child in ast.children.iter() {
      match child {
        ASTContext::Ternimal(ctx) => {
          listener.enter_terminal(ctx);
          listener.exit_terminal(ctx);
        },
        ASTContext::Rule(ctx) => self.walk(listener, ctx),
        ASTContext::Error(ctx) => {
          listener.enter_errornode(ctx);
          listener.exit_errornode(ctx);
        },
      }
    }

    listener.exit_rule(ast);
    listener.exit_every_rule(ast);
  }

}


pub struct HelloBaseListener;
impl HelloListener for HelloBaseListener {}
pub struct HelloBaseWalker;
impl HelloWalker for HelloBaseWalker {}






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
    self.visit_children(ctx.as_rule())
  }

  fn visit_stat(&self, ctx: &dyn StatContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }


  
  
  fn visit(&self, _ctx: &dyn HelloAcceptor) -> Box<dyn Any>  { todo!() }



  fn visit_rule(&self, ctx: &RuleContext) -> Box<dyn Any> {
    match ctx.get_rule_index() {
      HelloParser::RULE_EXPR => self.visit_expr(ctx),
      HelloParser::RULE_STAT => self.visit_stat(ctx),


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

  let walker = HelloBaseWalker {};
  let listener = HelloBaseListener {};

  walker.walk(&listener, ast.as_rule());

}

