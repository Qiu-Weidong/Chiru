use std::{collections::HashMap, rc::Rc};

use crate::{runtime::{token::Token, ast::{ast_context::ASTContext, terminal_context::TerminalContext, rule_context::RuleContext}}, tool::grammar::{Grammar, Production, ProductionItem}};



pub struct SyntaxisParser {
  pub tokens: Vec<Token>,
  pub table: HashMap<(usize, usize), Rc<Production>>,
  pub grammar: Grammar,
}


impl SyntaxisParser {
  pub const RULE_LIST: usize = 0;
  pub const PARSER_RULE: usize = 1;
  pub const BLOCK: usize = 2;
  pub const ALTERNATIVE: usize = 3;
  pub const EPSILON: usize = 4;
  pub const ELEMENT: usize = 5;
  pub const EBNF_SUFFIX: usize = 6;
  pub const LEXER_RULE: usize = 7;
  pub const REGULAR: usize = 8;

  pub fn new(tokens: Vec<Token>, table: HashMap<(usize, usize), Rc<Production>>, grammar: Grammar) -> Self {

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
    let production = self.table.get(&(rule_index, token_type)).unwrap();
    let name = match self.grammar.nonterminals.get(&rule_index).unwrap() {
      Some(name) => name.clone(),
      None => rule_index.to_string(),
    };

    let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };
    
    for child in production.right.iter() {
      match child {
        ProductionItem::NonTerminal(rule_id) => {
          let (rule, new_cursor) = self.parse_ast(cursor, *rule_id);
          cursor = new_cursor;
          if let Some(_) = self.grammar.nonterminals.get(rule_id).unwrap() {
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




