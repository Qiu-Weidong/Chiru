use std::fmt::Display;


use crate::runtime::token::Token;

use super::{terminal_context::TerminalContext, error_context::ErrorContext, ast_context::ASTContext, to_rule::ToRule};

#[derive(Clone, Debug)]
pub struct RuleContext {
  pub rule_name: String,
  pub children: Vec<ASTContext>,

  // 非终结符 rule 的编号
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

  pub fn get_first_terminal(&self) -> Option<&TerminalContext> { 
    if self.children.len() <= 0 { return None }
    match &self.children[0] {
      ASTContext::Terminal(ctx) => Some(ctx),
      ASTContext::Rule(ctx) => ctx.get_first_terminal(),
      ASTContext::Error(_) => None,
    }

  }

  pub fn get_last_terminal(&self) -> Option<&TerminalContext> { 
    if self.children.len() <= 0 { return None }
    match &self.children.last().unwrap() {
      ASTContext::Terminal(ctx) => Some(ctx),
      ASTContext::Rule(ctx) => ctx.get_last_terminal(),
      ASTContext::Error(_) => None,
    }
  }

  pub fn get_start_token(&self) -> Option<Token> {
    if self.children.len() <= 0 { return None }
    match &self.children[0] {
      ASTContext::Terminal(ctx) => Some(ctx.symbol.clone()),
      ASTContext::Rule(ctx) => ctx.get_start_token(),
      ASTContext::Error(ctx) => Some(ctx.symbol.clone()),
    }

  }

  pub fn get_stop_token(&self) -> Option<Token> {
    if self.children.len() <= 0 { return None }
    match self.children.last().unwrap() {
      ASTContext::Terminal(ctx) => Some(ctx.symbol.clone()),
      ASTContext::Rule(ctx) => ctx.get_stop_token(),
      ASTContext::Error(ctx) => Some(ctx.symbol.clone()),
    }
  }

  pub fn get_terminal(&self, token_type: usize, i: usize) -> Option<&TerminalContext> {
    let tokens = self.get_terminals(token_type);
    if i < tokens.len() { Some(&tokens[i]) } else { None }
  }

  pub fn get_terminals(&self, token_type: usize) -> Vec<&TerminalContext> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if let ASTContext::Terminal(child) = child {
        if child.symbol.token_type == token_type { result.push(child) }
      }
    }
    result
  }

  pub fn get_errornode(&self, token_type: usize, i: usize) -> Option<&ErrorContext> { 
    let errors = self.get_errornodes(token_type);
    if i < errors.len() { Some(&errors[i]) } else { None }
  }

  pub fn get_errornodes(&self, token_type: usize) -> Vec<&ErrorContext> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if let ASTContext::Error(child) = child {
        if child.symbol.token_type == token_type { result.push(child) }
      }
    }
    result
  }

  pub fn get_rule_context(&self, rule_type: usize, index: usize) -> Option<&RuleContext> {  
    let rules = self.get_rule_contexts(rule_type);
    if index < rules.len() { Some(&rules[index]) } else { None }
  }

  pub fn get_rule_contexts(&self, rule_type: usize) -> Vec<&RuleContext> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if let ASTContext::Rule(child) = child {
        if child.get_rule_index() == rule_type { result.push(child) }
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




  pub fn to_string(&self) -> String {
    let mut result = String::new();
    for child in self.children.iter() {
      result += &format!("{} ", child);
    }

    format!("{} ({})", self.rule_name, result)
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
