use std::{collections::{BTreeMap, HashMap, HashSet}, error::Error};

use super::{ast::{ast_context::AstContext, error_context::ErrorContext, rule_context::RuleContext, terminal_context::TerminalContext}, error_strategy::error_listener::ErrorListener, production::{Production, ProductionItem}, token_stream::TokenStream, vocabulary::{NonTerminal, Terminal}};




pub fn ll1_analyze(
  token_stream: &mut TokenStream, 
  rule_index: NonTerminal, 
  table:  &BTreeMap<(NonTerminal, Terminal), usize>,
  productions: &HashMap<usize, Production>,
  sync: &HashSet<(usize, usize)>,
  error_listeners: &[Box<dyn ErrorListener>],
) -> Result<RuleContext, Box<dyn Error>> {

  // 获取名称
  let name = match rule_names.get(&rule_index) {
    Some(name) => name.to_owned(),
    None => rule_index.to_string(),
  };
  

  let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };

  // 获取 production_id
  let production_id = loop {
    // 先查看一下下一个token是什么
    let token = token_stream.peek_next_token()?;

    if let Some(production_id) = table.get(&(rule_index, token.token_type)) {
      break *production_id;
    }
    else if sync.contains(&(rule_index, token.token_type)) {
      // 同步 这里表示整个非终结符都缺失了
      result.children.push(AstContext::Error( ErrorContext::missing() ));
      return Ok(result);
    }
    else {
      // 丢弃，将其添加到 error node, 这里认为该 token 是多余的
      // println!("redundant");
      result.children.push(AstContext::Error( ErrorContext::redundant(&token)  ));
      // 消耗掉该 token
      token_stream.consume()?;
    }
  };
  
  let production = productions.get(&production_id).unwrap();
  

  
  
  for child in production.right.iter() {

    match child {
      ProductionItem::NonTerminal(rule_id) => {
        let t = ll1_analyze(token_stream, rule_id.id, table, productions, rule_names, sync, error_listeners)?;
        if let Some(_) = rule_names.get(&rule_id.id) {
          result.children.push(AstContext::Rule(t));
        }
        else {
          result.children.extend(t.children);
        }
      },
      ProductionItem::Terminal(token_type) => {
        let mut token = token_stream.peek_next_token()?;
        while token_type.id != token.token_type && token.token_type != 1 {
          result.children.push(AstContext::Error( ErrorContext::redundant(&token) ));
          token_stream.consume()?; // 是在这里报的错
          token = token_stream.peek_next_token()?;
        }

        if token.token_type == 1 {
          result.children.push(AstContext::Error(ErrorContext::missing()));
          break;
        }



        // 匹配了
        result.children.push(AstContext::Terminal(TerminalContext { symbol: token.clone() }));
        // 消耗掉
        token_stream.consume()?;
      },
    }
  }
  Ok(result)
}





