
// // 语法树相关数据结构
// use crate::syntaxis::token::Token;
// use std::rc::Rc;
// use std::any::Any;



// // 定义一个包含 accept 的 trait , 显然，各种 context 都是可接收的。
// pub trait Acceptable: Any {
//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any>;
// }





// pub struct TerminalContext {
//   pub symbol: Token,
//   pub parent: Rc<RuleContext>, // 终结符一定有父节点，不需要 Option, 并且父节点一定是 RuleContext 。
// }

// impl Acceptable for TerminalContext {
//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> {
//     visitor.visit_terminal(self)
//   }
// }

// // impl  {
    
// // }

// pub struct ErrorContext {
//   pub symbol: Token,
//   pub parent: Rc<RuleContext>,

//   // error message
// }

// impl Acceptable for ErrorContext {
//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> {
//     visitor.visit_errornode(self)
//   }

//   fn as_any(&self) -> &dyn Any { self }

//   fn as_any_mut(&mut self) -> &mut dyn Any { self }
// }


// pub struct RuleContext {
//   pub parent: Option<Rc<RuleContext>>, 
//   pub children: Vec<Rc<dyn Any>>,

//   // 可能需要包含一个 rule_index
// }

// impl Acceptable for RuleContext {
//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> {
//     visitor.visit_children(self)
//   }

//   fn as_any(&self) -> &dyn Any { self }

//   fn as_any_mut(&mut self) -> &mut dyn Any { self }
// }

// impl RuleContext {
//   pub fn add_child(&mut self, child: Rc<dyn Any>) { self.children.push(child) }

//   pub fn get_child(&self, index: usize) -> Option<Rc<dyn Any>> { 
//     if self.children.len() <= index {None} 
//     else { 
//       let result = Rc::clone( &self.children[index]);
//       Some(result)
//     }  
//   }

//   pub fn get_children(&self) -> &Vec<Rc<dyn Any>> { &self.children }

//   pub fn get_child_count(&self) -> usize { self.children.len() }

//   pub fn get_rule_index(&self) -> usize { todo!() }

//   pub fn get_start_token(&self) -> Option<Rc<TerminalContext>> { 
//     if self.children.len() <= 0 { return None }
//     else if self.children[0].is::<TerminalContext>() {
//       let result = Rc::clone(&self.children[0]).downcast::<TerminalContext>().unwrap();
//       return Some(result);
//     }
//     else if self.children[0].is::<RuleContext>() {
//       let rule_context = self.children[0].downcast_ref::<RuleContext>().unwrap();
//       return rule_context.get_start_token();
//     }
//     None
//   }

//   pub fn get_stop_token(&self) -> Option<Rc<TerminalContext>> { 
//     if self.children.len() <= 0 { return None }
//     else if self.children.last().unwrap().is::<TerminalContext>() {
//       let result = Rc::clone(&self.children.last().unwrap()).downcast::<TerminalContext>().unwrap();
//       return Some(result);
//     }
//     else if self.children.last().unwrap().is::<RuleContext>() {
//       let rule_context = self.children.last().unwrap().downcast_ref::<RuleContext>().unwrap();
//       return rule_context.get_stop_token();
//     }
//     None
//   }

//   pub fn get_token(&self, token_type: usize, i: usize) -> Option<Rc<TerminalContext>> {
//     let tokens = self.get_tokens(token_type);
//     if i < tokens.len() { Some(Rc::clone(&tokens[i])) } else { None }
//   }

//   pub fn get_tokens(&self, token_type: usize) -> Vec<Rc<TerminalContext>> { 
//     let mut result = Vec::new();
//     for child in self.children.iter() {
//       if child.is::<TerminalContext>() {
//         let child = Rc::clone(child).downcast::<TerminalContext>().unwrap();
//         if child.symbol.token_type == token_type { result.push(child) }
//       }
//     }
//     result
//   }

//   pub fn get_errornode(&self, token_type: usize, i: usize) -> Option<Rc<ErrorContext>> { 
//     let errors = self.get_errornodes(token_type);
//     if i < errors.len() { Some(Rc::clone(&errors[i])) } else { None }
//   }

//   pub fn get_errornodes(&self, token_type: usize) -> Vec<Rc<ErrorContext>> { 
//     let mut result = Vec::new();
//     for child in self.children.iter() {
//       if child.is::<ErrorContext>() {
//         let child = Rc::clone(child).downcast::<ErrorContext>().unwrap();
//         if child.symbol.token_type == token_type { result.push(child) }
//       }
//     }
//     result
//   }

//   pub fn enter_rule(&self, _listener: &dyn ASTListener) {}

//   pub fn exit_rule(&self, _listener: &dyn ASTListener) {}

//   pub fn get_rule_context(&self, rule_type: usize, index: usize) -> Option<Rc<RuleContext>> {  
//     let rules = self.get_rule_contexts(rule_type);
//     if index < rules.len() { Some(Rc::clone(&rules[index])) } else { None }
//   }

//   pub fn get_rule_contexts(&self, rule_type: usize) -> Vec<Rc<RuleContext>> { 
//     let mut result = Vec::new();
//     for child in self.children.iter() {
//       if child.is::<RuleContext>() {
//         let child = Rc::clone(child).downcast::<RuleContext>().unwrap();
//         if child.get_rule_index() == rule_type { result.push(child) }
//       }
//     }
//     result
//   }

// }


// // 返回结果貌似可以修改为 Box 而不是 Rc 。
// pub trait ASTVisitor: Any {
//   fn visit(&self, ast: &dyn Acceptable) -> Box<dyn Any>; // { ast.accept(self) }

//   fn visit_children(&self, context: &RuleContext) -> Box<dyn Any> ;

//   fn visit_terminal(&self, _terminal: &TerminalContext) -> Box<dyn Any>  { self.default_result() }

//   fn visit_errornode(&self, _errornode: &ErrorContext) -> Box<dyn Any>  { self.default_result() }

//   fn default_result(&self) -> Box<dyn Any>;

//   fn aggregate_result(&self, _aggregate: Box<dyn Any> , next_result: Box<dyn Any> ) -> Box<dyn Any>  { next_result }

//   fn should_visit_next_child(&self, _context: &RuleContext, _current_result: &dyn Any) -> bool {true}

//   // 这是为了向下转换，首先需要转换为 Any
//   fn as_any(&self) -> &dyn Any;

//   fn as_any_mut(&mut self) -> &mut dyn Any;
// }

// pub trait ASTListener: Any {
//   fn visit_terminal(&self, _context: &RuleContext) {}

//   fn visit_errornode(&self, _context: &RuleContext) {}

//   fn enter_every_rule(&self, _context: Rc<RuleContext>) {}

//   fn exit_every_rule(&self, _context: Rc<RuleContext>) {}

//   // 这是为了向下转换，首先需要转换为 Any
//   fn as_any(&self) -> &dyn Any;

//   fn as_any_mut(&mut self) -> &mut dyn Any;
// }
