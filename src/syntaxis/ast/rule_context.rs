use std::{rc::Rc, any::Any};

use crate::syntaxis::to_any::ToAny;

use super::{terminal_context::TerminalContext, error_context::ErrorContext};


pub struct RuleContext {
  pub parent: Option<Rc<RuleContext>>, 
  pub children: Vec<Rc<dyn Any>>,

  // 可能需要添加一个 rule id
  pub rule_index: usize,
}

impl ToAny for RuleContext {
  fn as_any(&self) -> &dyn std::any::Any { self }

  fn as_any_mut(&mut self) ->  &mut dyn std::any::Any { self }
}


// 其他函数
impl RuleContext {
  // pub fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> {
  //   // 不要调用 visit_children, 调用 visit, visit会根据 ctx 的类型调用对应的 visit_xxx 函数
  //   visitor.visit_rule(self)
  // }

  pub fn add_child(&mut self, child: Rc<dyn Any>) { self.children.push(child) }

  pub fn get_child(&self, index: usize) -> Option<Rc<dyn Any>> { 
    if self.children.len() <= index {None} 
    else { 
      let result = Rc::clone( &self.children[index]);
      Some(result)
    }  
  }

  pub fn get_children(&self) -> &Vec<Rc<dyn Any>> { &self.children }

  pub fn get_child_count(&self) -> usize { self.children.len() }

  pub fn get_rule_index(&self) -> usize { self.rule_index }

  pub fn get_start_token(&self) -> Option<Rc<TerminalContext>> { 
    if self.children.len() <= 0 { return None }
    else if self.children[0].is::<TerminalContext>() {
      let result = Rc::clone(&self.children[0]).downcast::<TerminalContext>().unwrap();
      return Some(result);
    }
    else if self.children[0].is::<RuleContext>() {
      let rule_context = self.children[0].downcast_ref::<RuleContext>().unwrap();
      return rule_context.get_start_token();
    }
    None
  }

  pub fn get_stop_token(&self) -> Option<Rc<TerminalContext>> { 
    if self.children.len() <= 0 { return None }
    else if self.children.last().unwrap().is::<TerminalContext>() {
      let result = Rc::clone(&self.children.last().unwrap()).downcast::<TerminalContext>().unwrap();
      return Some(result);
    }
    else if self.children.last().unwrap().is::<RuleContext>() {
      let rule_context = self.children.last().unwrap().downcast_ref::<RuleContext>().unwrap();
      return rule_context.get_stop_token();
    }
    None
  }

  pub fn get_token(&self, token_type: usize, i: usize) -> Option<Rc<TerminalContext>> {
    let tokens = self.get_tokens(token_type);
    if i < tokens.len() { Some(Rc::clone(&tokens[i])) } else { None }
  }

  pub fn get_tokens(&self, token_type: usize) -> Vec<Rc<TerminalContext>> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if child.is::<TerminalContext>() {
        let child = Rc::clone(child).downcast::<TerminalContext>().unwrap();
        if child.symbol.token_type == token_type { result.push(child) }
      }
    }
    result
  }

  pub fn get_errornode(&self, token_type: usize, i: usize) -> Option<Rc<ErrorContext>> { 
    let errors = self.get_errornodes(token_type);
    if i < errors.len() { Some(Rc::clone(&errors[i])) } else { None }
  }

  pub fn get_errornodes(&self, token_type: usize) -> Vec<Rc<ErrorContext>> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if child.is::<ErrorContext>() {
        let child = Rc::clone(child).downcast::<ErrorContext>().unwrap();
        if child.symbol.token_type == token_type { result.push(child) }
      }
    }
    result
  }

  // pub fn enter_rule(&self, _listener: &dyn ASTListener) {}

  // pub fn exit_rule(&self, _listener: &dyn ASTListener) {}

  pub fn get_rule_context(&self, rule_type: usize, index: usize) -> Option<Rc<RuleContext>> {  
    let rules = self.get_rule_contexts(rule_type);
    if index < rules.len() { Some(Rc::clone(&rules[index])) } else { None }
  }

  pub fn get_rule_contexts(&self, rule_type: usize) -> Vec<Rc<RuleContext>> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if child.is::<RuleContext>() {
        let child = Rc::clone(child).downcast::<RuleContext>().unwrap();
        if child.get_rule_index() == rule_type { result.push(child) }
      }
    }
    result
  }

}


impl ToString for RuleContext {
  fn to_string(&self) -> String {
    todo!()
  }
}

