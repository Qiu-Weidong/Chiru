// 语法树相关数据结构
use crate::syntaxis::token::Token;
use std::rc::Rc;

pub trait ASTContext {
  fn get_parent(&self) -> Rc<dyn ASTContext>;

  fn set_parent(&mut self, parent: Rc<dyn ASTContext>);

  fn get_text(&self) -> &str;

  fn accept(&self, visitor: Rc<dyn ASTVisitor>);
}



pub trait RuleContext: ASTContext {
  fn add_child(&mut self, child: Rc<dyn ASTContext>);

  fn get_child(&self) -> Rc<dyn ASTContext>;

  fn get_children(&self) -> &[Rc<dyn ASTContext>];

  fn get_child_count(&self) -> i32;

  fn get_rule_index(&self) -> i32 {     -1 }

  fn get_start_token(&self) -> Rc<TerminalContext>;

  fn get_stop_token(&self) -> Rc<TerminalContext>;

  fn get_token(&self, token_type: usize, i: usize) -> Rc<TerminalContext>;

  fn get_tokens(&self, token_type: usize) -> Vec<Rc<TerminalContext>>;

  fn enter_rule(&self) {}

  fn exit_rule(&self) {}

  fn get_rule_context(&self, rule_type: usize, index: usize) -> Rc<dyn RuleContext>;

  fn get_rule_contexts(&self, rule_type: usize) -> Vec<Rc<dyn RuleContext>>;
}

pub struct TerminalContext {
  pub symbol: Token,
  pub parent: Rc<dyn ASTContext>,
}

impl ASTContext for TerminalContext {
  fn get_parent(&self) -> Rc<dyn ASTContext> {
    todo!()
  }

  fn set_parent(&mut self, _parent: Rc<dyn ASTContext>) {
    todo!()
  }

  fn get_text(&self) -> &str {
    todo!()
  }

  fn accept(&self, _visitor: Rc<dyn ASTVisitor>) where Self: Sized {
    todo!()
  }


}


pub struct ErrorContext {
  pub symbol: Token,
  pub parent: Rc<dyn ASTContext>,
}

impl ASTContext for ErrorContext {
  fn get_parent(&self) -> Rc<dyn ASTContext> {
    todo!()
  }

  fn set_parent(&mut self, _parent: Rc<dyn ASTContext>) {
    todo!()
  }

  fn get_text(&self) -> &str {
    todo!()
  }

  fn accept(&self, _visitor: Rc<dyn ASTVisitor>) where Self: Sized {
    todo!()
  }
}




pub trait ASTVisitor {
  fn visit(&self, ast: Rc<dyn ASTContext>) { ast.accept(Rc::clone(self)) }

  fn visit_children(&self, context: Rc<dyn RuleContext>);

  fn visit_terminal(&self, _terminal: Rc<TerminalContext>) {}

  fn visit_errornode(&self, _errornode: Rc<ErrorContext>) {}

  // fn default_result(&self) -> T;

  // fn aggregate_result(&self, _aggregate: T, next_result: T) -> T { next_result }

  // fn should_visit_next_child(&self, _context: Rc<dyn RuleContext>, _current_result: T) -> bool {true}
}

pub trait ASTListener {
  fn visit(&self, ast: Rc<dyn ASTContext>);

  fn visit_children(&self, context: Rc<dyn RuleContext>);

  fn enter_every_rule(&self, context: Rc<dyn RuleContext>);

  fn exit_every_rule(&self, context: Rc<dyn RuleContext>);
}


