use std::fmt::Display;


use crate::runtime::token::Token;
use serde::Serialize;
use super::{terminal_context::TerminalContext, ast_context::AstContext, error_context::ErrorContext};


pub trait ToRule {
  fn as_rule(&self) -> &RuleContext;

  fn as_mut_rule(&mut self) -> &mut RuleContext;
}



#[derive(Clone, Debug, Serialize)]
pub struct RuleContext {
  // 非终结符 rule 的编号
  pub rule_index: usize,
  pub rule_name: String,
  pub children: Vec<AstContext>,

  
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
  pub fn get_children(&self) -> &[AstContext] { &self.children }

  pub fn get_child_count(&self) -> usize { self.children.len() }

  pub fn get_rule_index(&self) -> usize { self.rule_index }

  pub fn get_first_terminal(&self) -> Option<&TerminalContext> { 
    match self.children.first()? {
      AstContext::Terminal(ctx) => Some(ctx),
      AstContext::Rule(ctx) => ctx.get_first_terminal(),
      AstContext::Error(_) => None,
    }
  }

  pub fn get_last_terminal(&self) -> Option<&TerminalContext> { 
    match self.children.last()? {
      AstContext::Terminal(ctx) => Some(ctx),
      AstContext::Rule(ctx) => ctx.get_first_terminal(),
      AstContext::Error(_) => None,
    }
  }

  pub fn get_start_token(&self) -> Option<Token> {
    match self.children.first()? {
      AstContext::Terminal(ctx) => Some(ctx.symbol.clone()),
      AstContext::Rule(ctx) => ctx.get_start_token(),
      AstContext::Error(_) => None,
    }
  }

  pub fn get_stop_token(&self) -> Option<Token> {
    match self.children.last()? {
      AstContext::Terminal(ctx) => Some(ctx.symbol.clone()),
      AstContext::Rule(ctx) => ctx.get_start_token(),
      AstContext::Error(_) => None,
    }
  }

  pub fn get_terminal(&self, token_type: usize, i: usize) -> Option<&TerminalContext> {
    let tokens = self.get_terminals(token_type);
    let token = tokens.get(i)?;
    Some(token)
  }

  pub fn get_terminals(&self, token_type: usize) -> Vec<&TerminalContext> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if let AstContext::Terminal(child) = child {
        if child.symbol.token_type == token_type { result.push(child) }
      }
    }
    result
  }

  pub fn get_errornode(&self, i: usize) -> Option<&ErrorContext> { 
    let errors = self.get_errornodes();
    let error = errors.get(i)?;
    Some(error)
  }

  pub fn get_errornodes(&self) -> Vec<&ErrorContext> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if let AstContext::Error(child) = child {
        result.push(child) 
      }
    }
    result

  }

  pub fn get_rule_context(&self, rule_type: usize, index: usize) -> Option<&RuleContext> {  
    let rules = self.get_rule_contexts(rule_type);
    let rule = rules.get(index)?;
    Some(rule)
  }

  pub fn get_rule_contexts(&self, rule_type: usize) -> Vec<&RuleContext> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if let AstContext::Rule(child) = child {
        if child.get_rule_index() == rule_type { result.push(child) }
      }
    }
    result
  }

  pub fn get_test<'a>(&self, input: &'a str) -> Option<&'a str> {
    if let Some(start) = self.get_start_token() {
      if let Some(stop) = self.get_stop_token() {
        Some(&input[start.location.byte_index_start..stop.location.byte_index_stop])
      } else {
        None
      }
    } else {
      None
    }

  }




  pub fn to_string(&self) -> String {
    let mut result = format!("({}", self.rule_name);
    self.children.iter().for_each(|child| {
      result += " ";
      match child {
        AstContext::Terminal(ctx) => result += &ctx.symbol.token_name,
        AstContext::Rule(ctx) => result += &ctx.to_string(),
        AstContext::Error(ctx) => result += &ctx.to_string(),
      }
    });

    result += ")";
    result
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
