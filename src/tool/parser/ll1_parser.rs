use std::collections::HashMap;
use crate::tool::grammar::Grammar;
use chiru::runtime::{token::Token, ast::{rule_context::RuleContext, ast_context::ASTContext, terminal_context::TerminalContext}, production::ProductionItem};


pub struct LL1Parser {
  pub tokens: Vec<Token>,
  pub table: HashMap<(usize, usize), usize>,
  pub grammar: Grammar,
}

impl LL1Parser {
  pub fn new(tokens: Vec<Token>, table: HashMap<(usize, usize), usize>, grammar: Grammar) -> Self {

    Self {
      tokens,
      table,
      grammar,
    }
  }

  pub fn parse(&self) -> RuleContext {
    self.parse_ast(0, 0).0
  }


  fn parse_ast(&self, cursor: usize, rule_index: usize) -> (RuleContext, usize) {
    let mut cursor = cursor;

    let token_type = self.tokens[cursor].token_type;
    let production_id = self.table.get(&(rule_index, token_type)).unwrap();


    let production = self.grammar.productions.get(production_id).unwrap();
    let name = self.grammar.vocabulary.get_nonterminal_name_with_default(rule_index);
    let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };
    
    for child in production.right.iter() {
      match child {
        ProductionItem::NonTerminal(rule_id) => {
          let (rule, new_cursor) = self.parse_ast(cursor, *rule_id);
          cursor = new_cursor;
          if let Some(_) = self.grammar.vocabulary.get_nonterminal_name_by_id(*rule_id) {
            let child = ASTContext::Rule(rule);
            result.children.push(child);
          } else {
            for child in rule.children.iter() {
              result.children.push(child.clone());
            }
          }
          
        },
        ProductionItem::Terminal(token_type) => {
          // 检查是否匹配
          if *token_type != self.tokens[cursor].token_type { println!("符号不匹配") }
          let terminal = TerminalContext { symbol: self.tokens[cursor].clone() };
          cursor += 1;
          let child = ASTContext::Terminal(terminal);
          result.children.push(child);
        },
      };
    }
    (result, cursor)
  }


  
}





