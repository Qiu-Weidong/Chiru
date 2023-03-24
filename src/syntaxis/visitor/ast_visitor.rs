use std::any::Any;


use crate::syntaxis::{to_any::ToAny, ast::{rule_context::RuleContext, terminal_context::TerminalContext, error_context::ErrorContext}};

pub trait ASTVisitor: ToAny {
  /**
   * visit 的参数是 Any 的话, visit_children 的时候可以方便一点, 但是静态检查功能会弱一些, 一些没有实现 Acceptable 的参数也可以传进来
   * visit 参数是 Acceptable 的话, 同样也需要根据 rule_index 来确定应该调用哪一个 visit 函数。
   * 如果在 visit 函数中直接 ast.accept(self) 的话, 对 RuleContext, 会导致递归调用。
   * 
   * 这里的 visit 函数手动实现了多态的功能, 当需要多态的时候, 都可以调用该函数
   */
  fn visit(&self, ast: &dyn Any) -> Box<dyn Any> {

    if ast.is::<TerminalContext>() {
      let ast = ast.downcast_ref::<TerminalContext>().unwrap();
      self.visit_terminal(ast)
    }
    else if ast.is::<ErrorContext>() {
      let ast = ast.downcast_ref::<ErrorContext>().unwrap();
      self.visit_errornode(ast)
    }
    else if ast.is::<RuleContext>() {
      let ast = ast.downcast_ref::<RuleContext>().unwrap();
      self.visit_rule(ast)
    }
    else {
      todo!()
    }
    
  }

  fn visit_children(&self, context: &RuleContext) -> Box<dyn Any> {
    let mut result = self.default_result();
    for child in context.children.iter() {
      result = self.visit(child);
    }
    result
  }

  fn visit_rule(&self, rule: &RuleContext) -> Box<dyn Any>;

  fn visit_terminal(&self, _terminal: &TerminalContext) -> Box<dyn Any>  { self.default_result() }

  fn visit_errornode(&self, _errornode: &ErrorContext) -> Box<dyn Any>  { self.default_result() }

  fn default_result(&self) -> Box<dyn Any> { Box::new(()) }

  fn aggregate_result(&self, _aggregate: Box<dyn Any> , next_result: Box<dyn Any> ) -> Box<dyn Any>  { next_result }

  fn should_visit_next_child(&self, _context: &RuleContext, _current_result: &dyn Any) -> bool {true}
}


