use std::rc::Rc;

use crate::syntaxis::to_any::ToAny;

use super::{terminal_context::TerminalContext, error_context::ErrorContext, ast_context::ASTContext};


pub struct RuleContext {
  pub parent: Option<Rc<RuleContext>>, 
  pub children: Vec<ASTContext>,

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

  // pub fn add_child(&mut self, child: Rc<dyn Any>) { self.children.push(child) }

  // pub fn get_child(&self, index: usize) -> Option<Rc<dyn Any>> { 
  //   if self.children.len() <= index {None} 
  //   else { 
  //     let result = Rc::clone( &self.children[index]);
  //     Some(result)
  //   }  
  // }

  pub fn get_children(&self) -> &Vec<ASTContext> { &self.children }

  pub fn get_child_count(&self) -> usize { self.children.len() }

  pub fn get_rule_index(&self) -> usize { self.rule_index }

  pub fn get_start_token(&self) -> Option<Rc<TerminalContext>> { 
    if self.children.len() <= 0 { return None }
    match &self.children[0] {
      ASTContext::Ternimal(ctx) => Some(Rc::clone(ctx)),
      ASTContext::Rule(ctx) => ctx.get_start_token(),
      ASTContext::Error(_) => None,
    }

  }

  pub fn get_stop_token(&self) -> Option<Rc<TerminalContext>> { 
    if self.children.len() <= 0 { return None }
    match &self.children.last().unwrap() {
      ASTContext::Ternimal(ctx) => Some(Rc::clone(ctx)),
      ASTContext::Rule(ctx) => ctx.get_start_token(),
      ASTContext::Error(_) => None,
    }
  }

  pub fn get_token(&self, token_type: usize, i: usize) -> Option<Rc<TerminalContext>> {
    let tokens = self.get_tokens(token_type);
    if i < tokens.len() { Some(Rc::clone(&tokens[i])) } else { None }
  }

  pub fn get_tokens(&self, token_type: usize) -> Vec<Rc<TerminalContext>> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if let ASTContext::Ternimal(child) = child {
        if child.symbol.token_type == token_type { result.push(Rc::clone(child)) }
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
      if let ASTContext::Error(child) = child {
        if child.symbol.token_type == token_type { result.push(Rc::clone(child)) }
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
      if let ASTContext::Rule(child) = child {
        if child.get_rule_index() == rule_type { result.push(Rc::clone(child)) }
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

