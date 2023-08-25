use std::collections::HashMap;

use crate::tool::grammar::production::{Production, ProductionItem};

use super::{ast::{rule_context::RuleContext, ast_context::ASTContext, terminal_context::TerminalContext}, token_stream::TokenStream};



pub trait Parser {

  
  fn ll1_parse(&self, 
    token_stream: &mut TokenStream,
    table: &HashMap<(usize, usize), usize>, 
    productions: &HashMap<usize, Production>, 
    rule_names: &HashMap<usize, &'static str>,
    rule_index: usize
  ) -> RuleContext {
    
    let token = token_stream.consume().unwrap();


    let production_id = table.get(&(rule_index, token.token_type)).unwrap();
    let production = productions.get(production_id).unwrap();
    let name = match rule_names.get(&rule_index) {
      Some(name) => name.to_owned().to_string(),
      None => rule_index.to_string(),
    };

    let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };
    
    for child in production.right.iter() {
      match child {
        ProductionItem::NonTerminal(rule_id) => {
          let rule = self.ll1_parse(token_stream, table, productions, rule_names,  *rule_id);
          if let Some(_) = rule_names.get(rule_id) {
            let child = ASTContext::Rule(rule);
            result.children.push(child);
          } else {
            result.children.extend(rule.children);
          }
          
        },
        ProductionItem::Terminal(token_type) => {
          // 检查是否匹配
          if *token_type != token.token_type { 
            // todo 增加容错机制
            println!("符号不匹配") }
          let terminal = TerminalContext { symbol: token.clone() };
          let child = ASTContext::Terminal(terminal);
          result.children.push(child);
        },
      };
    }
    
    result
  }







  // fn parse(&self, token_stream: &mut TokenStream) -> RuleContext;



}

