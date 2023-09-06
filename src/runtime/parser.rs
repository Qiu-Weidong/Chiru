use std::collections::{HashMap, HashSet};

use crate::runtime::ast::error_context::ErrorContext;

use super::{ast::{rule_context::RuleContext, ast_context::ASTContext, terminal_context::TerminalContext}, token_stream::TokenStream, error_strategy::error_listener::ErrorListener, production::{ProductionItem, Production}};




// 定义一个 LL1
pub struct LL1Analyzer<'a> {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,

  pub table: &'a HashMap<(usize, usize), usize>, 
  pub productions: &'a HashMap<usize, Production>, 
  pub rule_names: &'a HashMap<usize, String>,

  pub sync: &'a HashSet<(usize, usize)>,
}

impl<'a> LL1Analyzer<'a> {
  pub fn analyse(&self, token_stream: &mut TokenStream,  rule_index: usize) -> RuleContext {
    
    let name = match self.rule_names.get(&rule_index) {
      Some(name) => name.to_owned().to_string(),
      None => rule_index.to_string(),
    };
    

    let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };

    // 获取 production_id
    let production_id = loop {
      // 先查看一下下一个token是什么
      let token = token_stream.peek_next_token().unwrap();

      if let Some(production_id) = self.table.get(&(rule_index, token.token_type)) {
        break *production_id;
      }
      else if self.sync.contains(&(rule_index, token.token_type)) {
        // 同步
        result.children.push(ASTContext::Error(ErrorContext { symbol: token.clone() }));
        return result;
      }
      else {
        // 丢弃，将其添加到 error node
        result.children.push(ASTContext::Error(ErrorContext { symbol: token.clone() }));
        // 消耗掉该 token
        token_stream.consume().unwrap();
      }
    };
    
    // let token = token_stream.peer_next_token().unwrap();
    // let production_id = self.table.get(&(rule_index, token.token_type)).unwrap();
    let production = self.productions.get(&production_id).unwrap();
    

    
    
    for child in production.right.iter() {
      let token = token_stream.peek_next_token().unwrap();

      match child {
        ProductionItem::NonTerminal(rule_id) => {
          let t = self.analyse(token_stream, *rule_id);
          if let Some(_) = self.rule_names.get(rule_id) {
            result.children.push(ASTContext::Rule(t));
          }
          else {
            result.children.extend(t.children);
          }
        },
        ProductionItem::Terminal(token_type) => {
          while *token_type != token.token_type {
            result.children.push(ASTContext::Error(ErrorContext { symbol: token.clone() }));
            token_stream.consume().unwrap();
          }
          // 匹配了
          result.children.push(ASTContext::Terminal(TerminalContext { symbol: token.clone() }));
          // 消耗掉
          token_stream.consume().unwrap();
        },
      }
    }
    
    
    // ASTContext::Rule(result)
    result
  }

  pub fn new(
    error_listeners: Vec<Box<dyn ErrorListener>>, 
    table: &'a HashMap<(usize, usize), usize>, 
    productions: &'a HashMap<usize, Production>, 
    rule_names: &'a HashMap<usize, String>,
    sync: &'a HashSet<(usize, usize)>
  ) -> Self {
    Self {
      error_listeners, table, productions, rule_names, sync
    }
  }










}


// 定义一个 lalr 分析器









