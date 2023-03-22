// 语法树相关数据结构
use crate::syntaxis::token::Token;
use std::rc::Rc;
use std::any::Any;


pub trait ASTContext: Any {
  fn get_parent(&self) -> Option<Rc<dyn RuleContext>>;

  fn set_parent(&mut self, parent: &dyn RuleContext);

  fn get_text(&self) -> &str;




  fn as_any(&self) -> &dyn Any;

  fn as_any_mut(&mut self) -> &mut dyn Any;

  fn accept(&self, visitor: &dyn ASTVisitor) -> Rc<dyn Any>;
}



pub trait RuleContext: ASTContext {
  fn add_child(&mut self, child: Rc<dyn ASTContext>);

  fn get_child(&self) -> Option<Rc<dyn ASTContext>>;

  fn get_children(&self) -> Vec<Rc<dyn ASTContext>>;

  fn get_child_count(&self) -> i32;

  fn get_rule_index(&self) -> i32 { -1 }

  fn get_start_token(&self) -> Rc<TerminalContext>;

  fn get_stop_token(&self) -> Rc<TerminalContext>;

  fn get_token(&self, token_type: usize, i: usize) -> Option<Rc<TerminalContext>>;

  fn get_tokens(&self, token_type: usize) -> Vec<Rc<TerminalContext>>;

  fn get_errornode(&self, token_type: usize, i: usize) -> Option<Rc<TerminalContext>>;

  fn get_errornodes(&self, token_type: usize) -> Vec<Rc<TerminalContext>>;

  fn enter_rule(&self) {}

  fn exit_rule(&self) {}

  fn get_rule_context(&self, rule_type: usize, index: usize) -> Option<Rc<dyn RuleContext>>;

  fn get_rule_contexts(&self, rule_type: usize) -> Vec<Rc<dyn RuleContext>>;
}

pub struct TerminalContext {
  pub symbol: Token,
  pub parent: Rc<dyn ASTContext>,
}

impl ASTContext for TerminalContext {
  fn get_parent(&self) -> Option<Rc<dyn RuleContext>> {
    todo!()
  }

  fn set_parent(&mut self, _parent: &dyn RuleContext) {
    todo!()
  }

  fn get_text(&self) -> &str {
    todo!()
  }

  fn as_any(&self) -> &dyn Any {
    self as &dyn Any 
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any 
  }

  fn accept(&self, _visitor: &dyn ASTVisitor) -> Rc<dyn Any> {
    todo!()
  }

}


pub struct ErrorContext {
  pub symbol: Token,
  pub parent: Rc<dyn ASTContext>,

  // error message
}

impl ASTContext for ErrorContext {
  fn get_parent(&self) -> Option<Rc<dyn RuleContext>> {
    todo!()
  }

  fn set_parent(&mut self, _parent: &dyn RuleContext) {
    todo!()
  }

  fn get_text(&self) -> &str {
    todo!()
  }

  fn as_any(&self) -> &dyn Any {
    self as &dyn Any 
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any 
  }

  fn accept(&self, _visitor: &dyn ASTVisitor) -> Rc<dyn Any> { todo!() }

}




pub trait ASTVisitor {
  fn visit(&self, ast: &dyn ASTContext) -> Rc<dyn Any> ;

  fn visit_children(&self, context: &dyn RuleContext) -> Rc<dyn Any>;

  fn visit_terminal(&self, _terminal: &TerminalContext) -> Rc<dyn Any> { self.default_result() }

  fn visit_errornode(&self, _errornode: &ErrorContext) -> Rc<dyn Any> { self.default_result() }

  fn default_result(&self) -> Rc<dyn Any>;

  fn aggregate_result<'a>(&self, _aggregate: &'a dyn Any, next_result: &'a dyn Any) -> Rc<dyn Any + 'a> { Rc::new(next_result) }

  fn should_visit_next_child(&self, _context: &dyn RuleContext, _current_result: &dyn Any) -> bool {true}
}

pub trait ASTListener {
  fn visit(&self, ast: Rc<dyn ASTContext>);

  fn visit_children(&self, context: Rc<dyn RuleContext>);

  fn enter_every_rule(&self, context: Rc<dyn RuleContext>);

  fn exit_every_rule(&self, context: Rc<dyn RuleContext>);
}


