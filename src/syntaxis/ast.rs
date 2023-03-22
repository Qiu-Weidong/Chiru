
// 语法树相关数据结构
use crate::syntaxis::token::Token;
use std::rc::Rc;
use std::any::Any;


pub trait ASTContext: Any {
  // 父节点可能为空，因此使用 Option
  // fn get_parent(&self) -> Option<Rc<RuleContext>>;

  // fn set_parent(&mut self, parent: &RuleContext);

  fn get_text(&self) -> &str;



  // 这是为了向下转换，首先需要转换为 Any
  fn as_any(&self) -> &dyn Any;

  fn as_any_mut(&mut self) -> &mut dyn Any;

  // 为了在 ASTContext 这个 trait 里面声明 accept, visitor 的返回值使用 Any 而不是泛型
  fn accept(&self, visitor: &dyn ASTVisitor) -> Rc<dyn Any>;
}



pub struct TerminalContext {
  pub symbol: Token,
  pub parent: Rc<RuleContext>, // 终结符一定有父节点，不需要 Option, 并且父节点一定是 RuleContext 。
}

impl ASTContext for TerminalContext {

  fn get_text(&self) -> &str {
    &self.symbol.text
  }

  fn as_any(&self) -> &dyn Any {
    self as &dyn Any 
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any 
  }

  fn accept(&self, visitor: &dyn ASTVisitor) -> Rc<dyn Any> { visitor.visit_terminal(self) }

}


pub struct ErrorContext {
  pub symbol: Token,
  pub parent: Rc<RuleContext>,

  // error message
}

impl ASTContext for ErrorContext {

  fn get_text(&self) -> &str {
    &self.symbol.text
  }

  fn as_any(&self) -> &dyn Any {
    self as &dyn Any 
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any 
  }

  fn accept(&self, _visitor: &dyn ASTVisitor) -> Rc<dyn Any> { _visitor.visit_errornode(self) }

}



pub struct RuleContext {
  pub parent: Option<Rc<RuleContext>>, 
  pub children: Vec<Rc<dyn ASTContext>>,

  // 可能需要包含一个 rule_index
}


impl ASTContext for RuleContext {
  fn get_text(&self) -> &str {
    // 思路，找到第一个 token 的位置和最后一个 token 的位置，然后在输入流中切片
    todo!()
  }

  fn as_any(&self) -> &dyn Any {
    self as &dyn Any 
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any 
  }


  fn accept(&self, visitor: &dyn ASTVisitor) -> Rc<dyn Any> {
    visitor.visit_children(self)
  }
}
impl RuleContext {
  pub fn add_child(&mut self, child: Rc<dyn ASTContext>) { self.children.push(child) }

  pub fn get_child(&self, index: usize) -> Option<Rc<dyn ASTContext>> { 
    if self.children.len() <= index {None} 
    else { 
      let result = Rc::clone( &self.children[index]);
      Some(result)
    }  
  }

  pub fn get_children(&self) -> &Vec<Rc<dyn ASTContext>> { &self.children }

  pub fn get_child_count(&self) -> usize { self.children.len() }

  pub fn get_rule_index(&self) -> i32 { -1 }

  pub fn get_start_token(&self) -> Option<Rc<TerminalContext>> { 
    if self.children.len() <= 0 { return None }
    else if self.children[0].as_any().is::<TerminalContext>() {
      // let terminal = Rc::downcast::<TerminalContext>(self.children[0].as_any());
      // let terminal = Rc::clone(&self.children[0]);
      // let t = terminal.as_ref();
      // let terminal = self.children[0].as_any().downcast_ref::<TerminalContext>().unwrap();
      // return Some(Rc::new(*terminal));
    }
    else if self.children[0].as_any().is::<RuleContext>() {
      let rule_context = self.children[0].as_any().downcast_ref::<RuleContext>().unwrap();
      return rule_context.get_start_token();
    }
    None
  }

  pub fn get_stop_token(&self) -> Option<Rc<TerminalContext>> { todo!() }

  pub fn get_token(&self, token_type: usize, i: usize) -> Option<Rc<TerminalContext>> { todo!() }

  pub fn get_tokens(&self, token_type: usize) -> Vec<Rc<TerminalContext>> { todo!() }

  // pub fn get_errornode(&self, token_type: usize, i: usize) -> Option<Rc<TerminalContext>> { todo!() }

  // pub fn get_errornodes(&self, token_type: usize) -> Vec<Rc<TerminalContext>> { todo!() }

  pub fn enter_rule(&self, _listener: &dyn ASTListener) {}

  pub fn exit_rule(&self, _listener: &dyn ASTListener) {}

  pub fn get_rule_context(&self, rule_type: usize, index: usize) -> Option<Rc<RuleContext>> { todo!() }

  pub fn get_rule_contexts(&self, rule_type: usize) -> Vec<Rc<RuleContext>> { todo!() }
}


// 返回结果貌似可以修改为 Box 而不是 Rc 。
pub trait ASTVisitor: Any {
  fn visit(&self, ast: &dyn ASTContext) -> Rc<dyn Any>; // { ast.accept(self) }

  fn visit_children(&self, context: &RuleContext) -> Rc<dyn Any> ;

  fn visit_terminal(&self, _terminal: &TerminalContext) -> Rc<dyn Any> { self.default_result() }

  fn visit_errornode(&self, _errornode: &ErrorContext) -> Rc<dyn Any> { self.default_result() }

  fn default_result(&self) -> Rc<dyn Any>;

  fn aggregate_result(&self, _aggregate: Rc<dyn Any>, next_result: Rc<dyn Any>) -> Rc<dyn Any> { next_result }

  fn should_visit_next_child(&self, _context: &RuleContext, _current_result: &dyn Any) -> bool {true}

  // 这是为了向下转换，首先需要转换为 Any
  fn as_any(&self) -> &dyn Any;

  fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ASTListener: Any {
  fn visit_terminal(&self, _context: &RuleContext) {}

  fn visit_errornode(&self, _context: &RuleContext) {}

  fn enter_every_rule(&self, _context: Rc<RuleContext>) {}

  fn exit_every_rule(&self, _context: Rc<RuleContext>) {}

  // 这是为了向下转换，首先需要转换为 Any
  fn as_any(&self) -> &dyn Any;

  fn as_any_mut(&mut self) -> &mut dyn Any;
}
