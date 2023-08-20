use std::collections::HashMap;

use crate::tool::grammar::{production::{Production, ProductionItem}, vocabulary::Vocabulary};

use super::{ast::{rule_context::RuleContext, ast_context::ASTContext, terminal_context::TerminalContext}, token::Token};



pub trait Parser {
  fn parse_ast(&self, tokens: &[Token], 
    table: &HashMap<(usize, usize), usize>, 
    productions: &HashMap<usize, Production>, vocabulary: &Vocabulary,
    cursor: &mut usize, rule_index: usize
  ) -> RuleContext {

    let token_type = tokens[*cursor].token_type;
    let production_id = table.get(&(rule_index, token_type)).unwrap();
    let production = productions.get(production_id).unwrap();
    let name = vocabulary.get_nonterminal_name_with_default(rule_index);

    let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };
    
    for child in production.right.iter() {
      match child {
        ProductionItem::NonTerminal(rule_id) => {
          let rule = self.parse_ast(tokens, table, productions, vocabulary, cursor, *rule_id);
          if let Some(_) = vocabulary.get_nonterminal_name_by_id(*rule_id) {
            let child = ASTContext::Rule(rule);
            result.children.push(child);
          } else {
            result.children.extend(rule.children);
          }
          
        },
        ProductionItem::Terminal(token_type) => {
          // 检查是否匹配
          if *token_type != tokens[*cursor].token_type { println!("符号不匹配") }
          let terminal = TerminalContext { symbol: tokens[*cursor].clone() };
          *cursor += 1;
          let child = ASTContext::Terminal(terminal);
          result.children.push(child);
        },
      };
    }
    
    result
  }

}

