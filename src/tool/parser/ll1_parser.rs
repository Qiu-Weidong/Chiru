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

    let mut result = RuleContext {
      rule_index: 0, rule_name: String::from(""), children: Vec::new(),
    };

    let mut cur = &mut result;

    loop {
      if let ProductionItem::Terminal(token) = stack.last().unwrap() {
        if *token == 0 { break; } 
      }

      // 查看栈顶元素
      match stack.last().unwrap() {
        ProductionItem::Terminal(token_type) => {
          if *token_type != self.tokens[cursor].token_type { panic!("终结符不匹配") }
          cur.children.push(ASTContext::Terminal(TerminalContext { symbol: self.tokens[cursor].clone() }));
          cursor += 1;
          stack.pop();
        },
        ProductionItem::NonTerminal(rule) => {
          let token_type = self.tokens[cursor].token_type;
          let production = self.table.get(&(*rule, token_type)).unwrap();
          // pop非终结符，构造 RuleContext , 将其添加到 cur 的 children 中, 然后将 cur 指向该非终结符

          let ctx = RuleContext { rule_index: *rule, rule_name: String::from(""), children:Vec::new(), };
          cur.children.push(ASTContext::Rule(ctx));
          cur = match cur.children.last_mut().unwrap() {
            ASTContext::Rule(ctx) => ctx, 
            _ => { panic!("") }
          };
          
          stack.pop();
          
          

          for item in production.right.iter().rev() {
            stack.push(item.clone());
          }
        },
      }
    }
    



    todo!()
  }




  
}





