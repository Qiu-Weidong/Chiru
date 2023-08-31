use std::collections::{HashMap, HashSet};

use crate::tool::syntaxis::syntaxis_visitor::SyntaxisVisitor;



/**
 * 这个 visitor 负责找到以下信息
 * 1 nonterminal_list 0~n
 * 2 terminal_list 0~n
 * 3 nonterminal 0/1
 * 4 terminal 0/1
 */
pub struct ContextVisitor {
  // 显然，需要命名非终结符的映射
  nonterminals: HashMap<String, usize>,

  terminals: HashMap<String, usize>,
}



impl ContextVisitor {
  
}


impl SyntaxisVisitor for ContextVisitor {
  fn visit_rule_list(&mut self, ctx: &dyn crate::tool::syntaxis::syntaxis_context::RuleListContext) -> Box<dyn std::any::Any> {
    // 只需要访问 parser_rule
    ctx.parser_rule_list().iter().for_each(|ctx|  { ctx.accept(self); } );
    self.default_result()
  }

  fn visit_parser_rule(&mut self, ctx: &dyn crate::tool::syntaxis::syntaxis_context::ParserRuleContext) -> Box<dyn std::any::Any> {
    // 解析并填表
    
    self.default_result()
  }

  // 返回一个 hashset 的元组 (terminal_list, terminal, nonterminal_list, nonterminal) : (HashSet<usize>, ...)
  fn visit_block(&mut self, ctx: &dyn crate::tool::syntaxis::syntaxis_context::BlockContext) -> Box<dyn std::any::Any> {
    let mut result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = (HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new());
      
    ctx.alternative_list().iter().for_each(|v| {
      let re = v.accept(self).downcast::<(HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>)>().unwrap();
      result.0.extend(re.0);
      result.1.extend(re.1);
      result.2.extend(re.2);
      result.3.extend(re.3);
    });
    Box::new(result)
  }

  fn visit_alternative(&mut self, ctx: &dyn crate::tool::syntaxis::syntaxis_context::AlternativeContext) -> Box<dyn std::any::Any> {
    todo!()
  }

  fn visit_element(&mut self, ctx: &dyn crate::tool::syntaxis::syntaxis_context::ElementContext) -> Box<dyn std::any::Any> {
    todo!()
  }


}


