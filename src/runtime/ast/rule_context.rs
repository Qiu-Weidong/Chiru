use std::{rc::Rc, fmt::Display};


use crate::runtime::token::Token;

use super::{terminal_context::TerminalContext, error_context::ErrorContext, ast_context::ASTContext, to_rule::ToRule};


pub struct RuleContext {
  pub rule_name: String,
  pub children: Vec<ASTContext>,

  // 可能需要添加一个 rule id
  pub rule_index: usize,
}

impl ToRule for RuleContext {
  fn as_mut_rule(&mut self) -> &mut RuleContext {
    self
  }

  fn as_rule(&self) -> &RuleContext {
    self
  }

}


// 其他函数
impl RuleContext {
  pub fn get_children(&self) -> &Vec<ASTContext> { &self.children }

  pub fn get_child_count(&self) -> usize { self.children.len() }

  pub fn get_rule_index(&self) -> usize { self.rule_index }

  pub fn get_first_terminal(&self) -> Option<Rc<TerminalContext>> { 
    if self.children.len() <= 0 { return None }
    match &self.children[0] {
      ASTContext::Ternimal(ctx) => Some(Rc::clone(ctx)),
      ASTContext::Rule(ctx) => ctx.get_first_terminal(),
      ASTContext::Error(_) => None,
    }

  }

  pub fn get_last_terminal(&self) -> Option<Rc<TerminalContext>> { 
    if self.children.len() <= 0 { return None }
    match &self.children.last().unwrap() {
      ASTContext::Ternimal(ctx) => Some(Rc::clone(ctx)),
      ASTContext::Rule(ctx) => ctx.get_last_terminal(),
      ASTContext::Error(_) => None,
    }
  }

  pub fn get_start_token(&self) -> Option<Token> {
    if self.children.len() <= 0 { return None }
    match &self.children[0] {
      ASTContext::Ternimal(ctx) => Some(ctx.symbol.clone()),
      ASTContext::Rule(ctx) => ctx.get_start_token(),
      ASTContext::Error(ctx) => Some(ctx.symbol.clone()),
    }

  }

  pub fn get_stop_token(&self) -> Option<Token> {
    if self.children.len() <= 0 { return None }
    match self.children.last().unwrap() {
      ASTContext::Ternimal(ctx) => Some(ctx.symbol.clone()),
      ASTContext::Rule(ctx) => ctx.get_stop_token(),
      ASTContext::Error(ctx) => Some(ctx.symbol.clone()),
    }
  }

  pub fn get_terminal(&self, token_type: usize, i: usize) -> Option<Rc<TerminalContext>> {
    let tokens = self.get_terminals(token_type);
    if i < tokens.len() { Some(Rc::clone(&tokens[i])) } else { None }
  }

  pub fn get_terminals(&self, token_type: usize) -> Vec<Rc<TerminalContext>> { 
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


  pub fn get_text<'a>(&self, input: &'a str) -> &'a str { 
    let start = self.get_start_token();
    if let None = start { return &input[0..0] }
    let start = start.unwrap();
    let stop = self.get_stop_token();
    if let None = stop { return &input[0..0] }
    let stop = stop.unwrap();
    &input[start.char_start_index .. stop.char_stop_index]
  }
}

impl Display for RuleContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut result = String::new();
    for child in self.children.iter() {
      result += &format!("{} ", child);
    }


    write!(f, "{} ({})", self.rule_name, result)
  }
}
