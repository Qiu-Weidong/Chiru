use std::{any::Any, collections::HashMap};

use crate::tool::syntaxis::{syntaxis_visitor::SyntaxisVisitor, syntaxis_context::LexerRuleContext};

use super::lexer_rule::LexerRule;







pub struct LexerRuleVisitor {
  
  pub next_token_id: usize, 

  pub lexer_rule_map: HashMap<String, LexerRule>,
}



impl LexerRuleVisitor {
  pub fn new(next_token_id: usize, lexer_rule_map: HashMap<String, LexerRule>) -> Self {
    Self { 
      next_token_id, 
      lexer_rule_map,
    }
  }
}



impl SyntaxisVisitor for LexerRuleVisitor {
  
  
  fn visit_lexer_rule(&mut self, ctx: &dyn LexerRuleContext) -> Box<dyn Any> {
    // 获取名称
    let name = &ctx.token_ref().unwrap().symbol.text;

    // 检查是否已经定义
    if self.lexer_rule_map.contains_key(name) {
      // 重复定义，报错

      // 字符串字面量都使用了 '' 包裹，一定不会和普通 token 重复
      return self.default_result();
    }



    let regular = &ctx.regular().unwrap().regular_literal().unwrap().symbol.text; // .replace("\\/", "/");

    let lexer_rule = LexerRule::new(self.next_token_id, &name, regular, 
      0, false
    );

    self.next_token_id += 1;


    self.lexer_rule_map.insert(name.to_string(), lexer_rule);

    
    self.default_result()
  }

}



