use std::{collections::HashMap, rc::Rc};

use crate::{tool::grammar::{ProductionItem, Production, Grammar}, runtime::{token::Token, ast::{rule_context::RuleContext, ast_context::ASTContext}}};



pub struct LL1Parser {
  pub stack: Vec<ProductionItem>,
  pub tokens: Vec<Token>,
  pub table: HashMap<(usize, usize), Rc<Production>>,
  pub grammar: Grammar,
  pub cursor: usize,
}

impl LL1Parser {
  pub fn new(tokens: Vec<Token>, table: HashMap<(usize, usize), Rc<Production>>, grammar: Grammar) -> Self {

    Self {
      stack: Vec::new(),
      tokens,
      cursor: 0,
      table,
      grammar,
    }
  }

  pub fn parse(&mut self) -> Vec<Rc<Production>> {
    // 将 _STOP 放入栈中
    self.stack.push(ProductionItem::Terminal(1));
    // 首先第一步将开始符号放入栈中
    self.stack.push(ProductionItem::NonTerminal(0));

    let mut result = Vec::new();



    loop {
      if let ProductionItem::Terminal(token) = self.stack.last().unwrap() {
        if *token == 1 { break; } 
      }

      // 查看栈顶元素
      match self.stack.last().unwrap() {
        ProductionItem::Terminal(token_type) => {
          if *token_type != self.tokens[self.cursor].token_type { panic!("终结符不匹配") }
          self.cursor += 1;
          self.stack.pop();

          // 往 current_children 中push self.tokens[self.cursor]
        },
        ProductionItem::NonTerminal(rule) => {
          let token_type = self.tokens[self.cursor].token_type;
          let production = self.table.get(&(*rule, token_type)).unwrap();
          self.stack.pop();
          for item in production.right.iter().rev() {
            self.stack.push(item.clone());
          }


          result.push(Rc::clone(production));

        },
      }
    }
    
    result
  }



  pub fn get_ast(&mut self) -> Rc<RuleContext> {
    let productions = self.parse();
    // productions.reverse(); // 从下往上构建

    let mut child_stack: Vec<Rc<RuleContext>> = Vec::new();

    for production in productions.iter().rev() {
      let mut children: Vec<ASTContext> = Vec::new();
      for item in production.right.iter() {
        match item {
          ProductionItem::Terminal(id) => {

          },
          ProductionItem::NonTerminal(id) => {

          }
        }
      }
    }
    
    todo!()
  }

  // fn get_ast_(&self, productions: &[Rc<Production>], index: usize)

  
}


