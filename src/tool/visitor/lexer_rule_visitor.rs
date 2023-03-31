use std::{any::Any, collections::HashSet};

use serde::Serialize;

use crate::tool::syntaxis::{syntaxis_visitor::SyntaxisVisitor, syntaxis_context::LexerRuleContext, syntaxis_lexer::SyntaxisLexer};



// 这个 visitor 负责处理 lexer。
/**
 * {
 *   token_type: number,
 *   token_name: string,
 *   regex: Regex,
 * }
 */

#[derive(Debug, Serialize)]
pub struct LexerRuleData {
  pub token_type: usize,
  pub token_name: String,
  pub regex: String,
}


pub struct LexerRuleVisitor {
  pub data: Vec<LexerRuleData>,
  pub next_token_type: usize, // 注意初始化为 2
}

impl SyntaxisVisitor for LexerRuleVisitor {
  fn visit_lexer_rule(&mut self, ctx: &dyn LexerRuleContext) -> Box<dyn Any> {
    // 首先检查是否重复定义
    let name = &ctx.token_ref().unwrap().symbol.text;
    for item in self.data.iter() {
      if item.token_name.eq(name)  {
        println!("{} 重复定义", name);
        return self.default_result();
      }
    }

    let regular = &ctx.regular().unwrap().regular_literal().unwrap().symbol.text; // .replace("\\/", "/");
    if regular.len() < 2 {
      println!("非法正则表达式");
      return self.default_result();
    }

    let regular = &regular[1..regular.len()-1].replace("\\/", "/");
    self.data.push( LexerRuleData { token_type: self.next_token_type, token_name: name.to_owned(), regex: format!("^({})", regular) });
    self.next_token_type += 1;
    
    self.default_result()
  }

}

impl LexerRuleVisitor {
  pub fn new(data: Vec<LexerRuleData>, next_token_type: usize) -> Self {
    Self {
      data,
      next_token_type,
    }
  }
}


pub struct StringLiteralVisitor {
  pub data: Vec<LexerRuleData>,
  pub next_token_type: usize, // 注意初始化为 2
  pub cache: HashSet<String>,
}

impl SyntaxisVisitor for StringLiteralVisitor {
  fn visit_terminal(&mut self, terminal: &crate::runtime::ast::terminal_context::TerminalContext) -> Box<dyn Any> {
    if terminal.symbol.token_type == SyntaxisLexer::STRING_LITERAL {
      let name = &terminal.symbol.text;
      if self.cache.contains(name) {
        return self.default_result();
      }

      self.cache.insert(name.to_string());


      // abc  ab\n
      let literal = &name[1..name.len()-1]
        .replace("\\n", "\n").replace("\\t", "\t").replace("\\\\", "\\")
        .replace("\\r", "\r").replace("\\'", "\'").replace("+", "\\+")
        .replace("*", "\\*")
      ;

      self.data.push( LexerRuleData {token_type: self.next_token_type, token_name: format!("T_{}", self.next_token_type), regex: format!("^({})", literal) });
      self.next_token_type += 1;
    }
    self.default_result()
  }
}
impl StringLiteralVisitor {
  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      next_token_type: 2,
      cache: HashSet::new(),
    }
  }
}
