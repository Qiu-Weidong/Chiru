use std::{collections::HashMap, rc::Rc};

use crate::{tool::grammar::{ProductionItem, Production, Grammar}, runtime::{token::Token, ast::{rule_context::RuleContext, ast_context::ASTContext, terminal_context::TerminalContext}}};


/*
      id      +     *     (     )     $
E   E->TE1              E->TE1
E1          E1->+TE1          E1->ε   E1->ε
T   T->FT1              T->FT1
T1          T1->ε T1->*FT1    T1->ε   T1->ε
F   F->id               F->(E)

id+id*id

E       id+id*id  null  E->TE1
E1 T    id+id*id  E     T->FT1
E1 T1 F id+id*id  E T   F->id
E1 T1   +id*id E T F 





E
T
F
id
 */



pub struct LL1Parser {
  pub tokens: Vec<Token>,
  pub table: HashMap<(usize, usize), Rc<Production>>,
  pub grammar: Grammar,
}

impl LL1Parser {
  pub fn new(tokens: Vec<Token>, table: HashMap<(usize, usize), Rc<Production>>, grammar: Grammar) -> Self {

    Self {
      tokens,
      table,
      grammar,
    }
  }

  pub fn parse(&mut self) -> RuleContext {
    let mut stack: Vec<ProductionItem> = Vec::new();
    let mut cursor = 0;

    // 将 _START 放入栈中
    stack.push(ProductionItem::Terminal(0));
    // 首先第一步将开始符号放入栈中
    stack.push(ProductionItem::NonTerminal(0));

    // let mut result = Vec::new();

    loop {
      if let ProductionItem::Terminal(token) = stack.last().unwrap() {
        if *token == 0 { break; } 
      }

      // 查看栈顶元素
      match stack.last().unwrap() {
        ProductionItem::Terminal(token_type) => {
          if *token_type != self.tokens[cursor].token_type { panic!("终结符不匹配") }
          cursor += 1;

          stack.pop();
        },
        ProductionItem::NonTerminal(rule) => {
          let token_type = self.tokens[cursor].token_type;
          let production = self.table.get(&(*rule, token_type)).unwrap();
          
          stack.pop();
          for item in production.right.iter().rev() {
            stack.push(item.clone());
          }


          // result.push(Rc::clone(production));
        },
      }
    }
    



    todo!()
    // let mut non_terminals: Vec<Rc<RuleContext>> = Vec::new();

    // for production in result.iter().rev() {

    //   let name = match self.grammar.nonterminals.get(&production.left).unwrap() {
    //     Some(name) => name.clone(),
    //     None => production.left.to_string(),
    //   };

    //   let mut tree = RuleContext { rule_index: production.left, rule_name: name, children: Vec::new(), };
    //   for item in production.right.iter() {
    //     match item {
    //       ProductionItem::Terminal(_) => {
    //         tree.children.push(terminals.pop().unwrap());
    //       },
    //       ProductionItem::NonTerminal(_) => {
    //         let rule = non_terminals.pop().unwrap();
    //         // tree.children.push(ASTContext::Rule(rule));
    //         // if let Some(_) = self.grammar.nonterminals.get(&rule.rule_index) {
    //         //   tree.children.push(ASTContext::Rule(rule));
    //         // } else {
    //         //   // 将 rule 的 children 全部添加
    //         //   for child in rule.children.iter() { 
    //         //     let child = child.clone();
    //         //     tree.children.push(child); 
    //         //   }
    //         // } 
    //       },
    //     }
    //   }

    //   tree.children.reverse();
    //   non_terminals.push(Rc::new(tree));
    // }
    

    // non_terminals.pop().unwrap()
  }




  
}





