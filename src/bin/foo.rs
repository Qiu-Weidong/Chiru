/*
 * grammar Foo;
 * 
 * start: stmt+ ;
 * stmt: expr ';'=5
 * expr: Number=1 | ID=2 | '('=3 expr ')'=4 ;
 * 
 * 
 * input: ( 234 ) ; Hello ; 190 ;
 * ast: (start (stmt (expr ('(', expr ( Number ), ')'), ';'), stmt (expr (ID), ';'), stmt (expr (Number), ';')))
 */

use std::{any::Any, rc::Rc, vec};

use syntaxis::syntaxis::{ast::{to_rule::ToRule, terminal_context::TerminalContext, ast_context::ASTContext, error_context::ErrorContext, rule_context::RuleContext}, token::{Token, Position}};


fn main() {


  let lb = Token {
    token_type: 3,
      token_name: String::from("'('"),
      start: Position { line: 0, char_position: 0 },
      stop: Position { line: 0, char_position: 1 },
      channel: 0,
      text: String::from("\'(\'"),
      token_index: 0,
      char_start_index: 0,
      char_stop_index: 1,
  };

  let number = Token {
    token_type: 1,
    token_name: String::from("Number"),
    start: Position { line: 0, char_position: 2 },
    stop: Position { line: 0, char_position: 5 },
    channel: 0,
    text: String::from("234"),
    token_index: 1,
    char_start_index: 2,
    char_stop_index: 5,
  };

  let rb = Token {
    token_type: 4,
    token_name: String::from("')'"),
    start: Position { line: 0, char_position: 0 },
    stop: Position { line: 0, char_position: 1 },
    channel: 0,
    text: String::from("\')\'"),
    token_index: 0,
    char_start_index: 0,
    char_stop_index: 1,
  } ;
  let semicolone = Token {
    token_type: 5,
    token_name: String::from("';'"),
    start: Position { line: 0, char_position: 0 },
    stop: Position { line: 0, char_position: 1 },
    channel: 0,
    text: String::from("\';\'"),
    token_index: 0,
    char_start_index: 0,
    char_stop_index: 1,
  } ;


  let lbcontext = TerminalContext { 
    symbol: lb.clone()
  };


  let n1 = TerminalContext { 
    symbol: number.clone()
  };

  let rbcontext = TerminalContext { 
    symbol: rb.clone()
  };
  let semicontext = TerminalContext { symbol: semicolone.clone() };

  // 构造 expr
  let children = vec![
    ASTContext::Ternimal(Rc::new(lbcontext)),
    ASTContext::Ternimal(Rc::new(n1)),
    ASTContext::Ternimal(Rc::new(rbcontext)),
  ];

  let expr = RuleContext {
    rule_index: FooParser::RULE_EXPR,
    rule_name: String::from("expr"),
    children
  };

  let children = vec![
    ASTContext::Rule(Rc::new(expr)),
    ASTContext::Ternimal(Rc::new(semicontext)),
  ];

  let stmt = RuleContext {
    rule_index: 2, 
    rule_name: String::from("stmt"),
    children
  };

  let children = vec![ASTContext::Rule(Rc::new(stmt))];
  let start = RuleContext {
    rule_index: 1, 
    rule_name: String::from("start"), 
    children
  };



  println!("{}", start);


  let visitor = FooBaseVisitor {};
  start.accept(&visitor);

  let walker = FooBaseWalker {};
  let listener = FooBaseListener {};

  walker.walk(&listener, &start);
}


pub struct FooParser;
impl FooParser {
  const RULE_START: usize = 1;
  const RULE_STMT: usize = 2;
  const RULE_EXPR: usize = 3;
}

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

pub struct FooBaseVisitor;
impl FooVisitor for FooBaseVisitor {
  fn visit_expr(&self, ctx: &dyn ExprContext) -> Box<dyn Any> {
    if let Some(ctx) = ctx.number() {
      println!("{}", ctx.symbol.text);
    }

    self.visit_children(ctx.as_rule())
  }
}


pub trait FooWalker {
  // 类似于 visit , 使用 Walkerable 的好处是 xxxContext 都可以作为参数传入, 而不一定都是 RuleContext 。
  fn walk(&self, listener: &dyn FooListener, ast: &RuleContext) {
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


pub struct FooBaseWalker;
impl FooWalker for FooBaseWalker {}


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

