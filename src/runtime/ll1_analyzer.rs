use std::{collections::{BTreeMap, BTreeSet}, error::Error};

use super::{ast::{ast_context::AstContext, error_context::ErrorContext, rule_context::RuleContext, terminal_context::TerminalContext}, error_strategy::error_listener::ErrorListener, production::{Production, ProductionItem}, token_stream::TokenStream, vocabulary::{NonTerminal, Terminal}};




pub fn ll1_analyze<'a>(
  token_stream: &'a mut TokenStream<'a>, 
  rule_symbol: NonTerminal<'a>, 
  table:  &'a BTreeMap<(NonTerminal<'a>, Terminal<'a>), usize>,
  productions: &'a BTreeMap<usize, Production<'a>>,
  sync: &'a BTreeSet<(NonTerminal, Terminal)>,
  error_listeners: &'a [Box<dyn ErrorListener>],
) -> Result<RuleContext<'a>, Box<dyn Error>> {

  // 获取名称
  let mut result = RuleContext { symbol: rule_symbol, children: Vec::new(), };

  // 获取 production_id
  let production_id = loop {
    // 先查看一下下一个token是什么
    let token = token_stream.peek_next_token()?;

    if let Some(production_id) = table.get(&(rule_symbol, token.terminal)) {
      break *production_id;
    }
    else if sync.contains(&(rule_symbol, token.terminal)) {
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
      ProductionItem::NonTerminal(nonterminal) => {
        let t = ll1_analyze(token_stream, *nonterminal, table, productions,  sync, error_listeners)?;
        if let Some(_) = nonterminal.name {
          result.children.push(AstContext::Rule(t));
        }
        else {
          result.children.extend(t.children);
        }
      },
      ProductionItem::Terminal(token_type) => {
        let mut token = token_stream.peek_next_token()?;
        while token_type.id != token.terminal.id && token.terminal.id != 1 {
          result.children.push(AstContext::Error( ErrorContext::redundant(&token) ));
          token_stream.consume()?; // 是在这里报的错
          token = token_stream.peek_next_token()?;
        }

        if token.terminal.id == 1 {
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





