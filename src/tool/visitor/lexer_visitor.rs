

// 重新写词法规则的分析器

use crate::tool::syntaxis::{syntaxis_visitor::SyntaxisVisitor, syntaxis_context::ElementContext};



// 负责将字符串字面量转换为 token 。
pub struct StringLiteralToTokenVisitor {
  pub next_token_id: usize,
}

impl StringLiteralToTokenVisitor {
  // 将字符串字面量转义为对应的实际字符串
  fn escape(_input: &str) -> String { todo!() }
  
  // 将字符串字面量转义为识别它的正则表达式
  fn regular_escape(_input: &str) -> String { todo!() }


}

impl SyntaxisVisitor for StringLiteralToTokenVisitor {
  fn visit_element(&mut self, ctx: &dyn ElementContext) -> Box<dyn std::any::Any> {
    if let Some(value) = ctx.string_literal() {
      // 需要进行相关转义处理
      let text = StringLiteralToTokenVisitor::escape(&value.symbol.text);// 为了防止字符串的内容和某个 token 同名, 因此不删除引号

      // 检查该字符串是否已经定义
      // if self.grammar.vocabulary.is_terminal_name_defined(&text) { return self.default_result(); } 
      
      // 定义之
      // self.grammar.vocabulary.add_terminal(self.next_token_id, &text);
      
      // 将字符串对应的正则表达式放入 data 中，首先需要进行正则表达式的转义
      let regular_string = StringLiteralToTokenVisitor::regular_escape(&value.symbol.text);
      // self.data.push(LexerRuleData { 
      //   token_type: self.next_token_id, 
      //   token_name: format!("_T_{}", self.next_token_id), 
      //   regex: regular_string, 
      //   skip: false, 
      //   channel: 0,
      // });
      self.next_token_id += 1;
    }
    self.default_result()
  }
}








