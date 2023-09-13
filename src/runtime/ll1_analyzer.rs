use std::{error::Error, collections::{HashMap, HashSet}};

use super::{ast::{rule_context::RuleContext, ast_context::ASTContext, error_context::ErrorContext, terminal_context::TerminalContext}, token_stream::TokenStream, production::{Production, ProductionItem}, error_strategy::error_listener::ErrorListener};




pub fn ll1_analyze(
  token_stream: &mut TokenStream, 
  rule_index: usize, 
  table:  &HashMap<(usize, usize), usize>,
  productions: &HashMap<usize, Production>,
  rule_names: &HashMap<usize, String>,
  sync: &HashSet<(usize, usize)>,
  error_listeners: &[Box<dyn ErrorListener>],
) -> Result<RuleContext, Box<dyn Error>> {
  let name = match rule_names.get(&rule_index) {
    Some(name) => name.to_owned().to_string(),
    None => rule_index.to_string(),
  };
  

  let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };

  // 获取 production_id
  let production_id = loop {
    // 先查看一下下一个token是什么
    let token = token_stream.peek_next_token().unwrap();

    if let Some(production_id) = table.get(&(rule_index, token.token_type)) {
      break *production_id;
    }
    else if sync.contains(&(rule_index, token.token_type)) {
      // 同步
      result.children.push(ASTContext::Error(ErrorContext { symbol: token.clone() }));
      return Ok(result);
    }
    else {
      // 丢弃，将其添加到 error node
      result.children.push(ASTContext::Error(ErrorContext { symbol: token.clone() }));
      // 消耗掉该 token
      token_stream.consume().unwrap();
    }
  };
  
  // let token = token_stream.peer_next_token().unwrap();
  // let production_id = table.get(&(rule_index, token.token_type)).unwrap();
  let production = productions.get(&production_id).unwrap();
  

  
  
  for child in production.right.iter() {
    let token = token_stream.peek_next_token().unwrap();

    match child {
      ProductionItem::NonTerminal(rule_id) => {
        let t = ll1_analyze(token_stream, *rule_id, table, productions, rule_names, sync, error_listeners)?;
        if let Some(_) = rule_names.get(rule_id) {
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
  Ok(result)
}





