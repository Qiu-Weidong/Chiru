

// 重新写词法规则的分析器

use std::collections::HashMap;

use crate::tool::syntaxis::{syntaxis_visitor::SyntaxisVisitor, syntaxis_context::ElementContext};

use super::lexer_rule::LexerRule;



// 负责将字符串字面量转换为 token 。并分配 id 
pub struct StringLiteralToTokenVisitor {
  pub next_token_id: usize,

  pub lexer_rule_map: HashMap<String, LexerRule>,
}

impl StringLiteralToTokenVisitor {
  // 将字符串字面量转义为对应的实际字符串
  fn escape(input: &str) -> String { 
    // 对 str 进行转义处理
    input.to_owned()
  }
  
  // 将字符串字面量转义为识别它的正则表达式
  fn regular_escape(input: &str) -> String { 
    input.to_owned()
  }

  pub fn new(next_token_id: usize) -> Self {
    Self {
      next_token_id,
      lexer_rule_map: HashMap::new(),
    }
  }

}

impl SyntaxisVisitor for StringLiteralToTokenVisitor {
  fn visit_element(&mut self, ctx: &dyn ElementContext) -> Box<dyn std::any::Any> {
    
    // 处理 element 中的字符串常量即可
    if let Some(value) = ctx.string_literal() {
      
      // 需要进行相关转义处理
      let text = StringLiteralToTokenVisitor::escape(&value.symbol.text);// 为了防止字符串的内容和某个 token 同名, 因此不删除引号

      // 检查该字符串是否已经定义
      if self.lexer_rule_map.contains_key(&text) {
        // 如果已经定义，则忽略它
        return self.default_result();
      }

      let regular_string = StringLiteralToTokenVisitor::regular_escape(&value.symbol.text);
      
      // 定义之 token name = _T_xx
      let lexer_rule = LexerRule::new(
        self.next_token_id, 
        &format!("_T_{}", self.next_token_id),
        &regular_string,
        
        // 之后需要修改
        0, false
      );
      
      self.next_token_id += 1;

      // 将其插入即可
      self.lexer_rule_map.insert(text, lexer_rule);
      
    }



    self.default_result()
  }

}








