

use crate::runtime::token::Token;



pub struct RuleContext {
  pub rule_index: usize,
  pub rule_name: String,
  pub children: Vec<ASTContext>,
}

pub struct TerminalContext {
  pub symbol: Token,
}
pub struct ErrorContext {
  pub symbol: Token,
}

pub enum ASTContext {
  Rule(RuleContext),
  Terminal(TerminalContext),
  Error(ErrorContext),
}






impl RuleContext {
  pub fn get_rule_contexts(&self, rule_type: usize) -> Vec<&RuleContext> { 
    let mut result = Vec::new();
    for child in self.children.iter() {
      if let ASTContext::Rule(child) = child {
        if child.rule_index == rule_type { result.push(child) }
      }
    }
    result
  }

  pub fn get_rule_context(&self, rule_type: usize, index: usize) -> Option<&RuleContext> {  
    let rules = self.get_rule_contexts(rule_type);
    if index < rules.len() { Some(rules[index]) } else { None }
  }

  pub fn get_terminal(&self, token_type: usize, i: usize) -> Option<&TerminalContext> {
    let tokens = self.get_terminals(token_type);
    if i < tokens.len() { Some(tokens[i]) } else { None }
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


}



pub trait StartContext {
  fn stmt_list(&self) -> Vec<&dyn StmtContext>;
}
pub trait StmtContext {
  fn expr(&self) -> Option<&dyn ExprContext>;
}
pub trait ExprContext {
  fn expr(&self) -> Option<&dyn ExprContext>;
  fn number(&self) -> Option<&TerminalContext>;
  fn id(&self) -> Option<&TerminalContext>;
}


impl StartContext for RuleContext {
  fn stmt_list(&self) -> Vec<&dyn StmtContext> {
    let mut result = Vec::new();
    for ctx in self.get_rule_contexts(0).iter() {
      result.push(*ctx as &dyn StmtContext);
    }
    result
  }
}
impl StmtContext for RuleContext {
  fn expr(&self) -> Option<&dyn ExprContext> {
    let ctx =self.get_rule_context(1, 0);
    if let Some(ctx) = ctx {
      // let ctx = ctx as &dyn ExprContext;
      return Some(ctx as &dyn ExprContext);
    }
    None
  }
}
impl ExprContext for RuleContext {
  fn expr(&self) -> Option<&dyn ExprContext> { 
    let ctx = self.get_rule_context(2, 0);
    if let Some(ctx) = ctx {
      Some(ctx as &dyn ExprContext)
    } else { None }
  }
  fn number(&self) -> Option<&TerminalContext> { 
    self.get_terminal(1, 0)
  }
  fn id(&self) -> Option<&TerminalContext> { 
    self.get_terminal(2, 0)
  }
}



