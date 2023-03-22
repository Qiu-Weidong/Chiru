// 语法树相关数据结构
use crate::syntaxis::token::Token;
use std::rc::Rc;
use std::any::Any;


pub trait ASTContext: Any {
  // 父节点可能为空，因此使用 Option
  fn get_parent(&self) -> Option<Rc<dyn RuleContext>>;

  fn set_parent(&mut self, parent: &dyn RuleContext);

  fn get_text(&self) -> &str;




  fn as_any(&self) -> &dyn Any;

  fn as_any_mut(&mut self) -> &mut dyn Any;

  // 为了在 ASTContext 这个 trait 里面声明 accept, visitor 的返回值使用 Any 而不是泛型
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
    _visitor.visit_terminal(self)
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

  fn accept(&self, _visitor: &dyn ASTVisitor) -> Rc<dyn Any> { _visitor.visit_errornode(self) }

}




pub trait ASTVisitor {
  // 虽然有默认实现，但是无法直接在 trait 里面编写
  fn visit(&self, ast: &dyn ASTContext) -> Rc<dyn Any> ;

  fn visit_children(&self, context: &dyn RuleContext) -> Rc<dyn Any> ;

  fn visit_terminal(&self, _terminal: &TerminalContext) -> Rc<dyn Any> { self.default_result() }

  fn visit_errornode(&self, _errornode: &ErrorContext) -> Rc<dyn Any> { self.default_result() }

  fn default_result(&self) -> Rc<dyn Any>;

  fn aggregate_result(&self, _aggregate: Rc<dyn Any>, next_result: Rc<dyn Any>) -> Rc<dyn Any> { next_result }

  fn should_visit_next_child(&self, _context: &dyn RuleContext, _current_result: &dyn Any) -> bool {true}
}

pub trait ASTListener {
  fn visit(&self, ast: Rc<dyn ASTContext>);

  fn visit_children(&self, context: Rc<dyn RuleContext>);

  fn enter_every_rule(&self, context: Rc<dyn RuleContext>);

  fn exit_every_rule(&self, context: Rc<dyn RuleContext>);
}








struct DotVisitor;


impl ASTVisitor for DotVisitor {
  fn visit(&self, ast: &dyn ASTContext) -> Rc<dyn Any> {
    ast.accept(self)
  }

  fn visit_children(&self, context: &dyn RuleContext) -> Rc<dyn Any> {
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
    // 必须提供一个默认值
    todo!()
  }

}

impl DotVisitor {
  
}


