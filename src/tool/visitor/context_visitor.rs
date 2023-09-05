use std::{collections::{HashMap, HashSet}, any::Any, error::Error};

use maplit::hashset;

use crate::tool::syntaxis::chiru_visitor::ChiruVisitor;



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

  pub table: HashMap<usize, (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>)>,
}



impl ContextVisitor {
  pub fn new(nonterminals: HashMap<String, usize>, terminals: HashMap<String, usize>) -> Self {
    Self {
      table: HashMap::new(),
      nonterminals, 
      terminals,
    }
  }
}


impl ChiruVisitor for ContextVisitor {
  fn visit_compilation_unit(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::CompilationUnitContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    if let Some(rules) = ctx.rules() {
      rules.accept(self)
    } else {
      self.default_result()
    }
  }

  fn visit_rules(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::RulesContext) -> Result<Box<dyn Any>, Box<dyn Error>> {

    // 只需要访问 parser_rule
    for ctx in ctx.parser_rule_list().iter() { 
      ctx.accept(self)?; 
    } 
    self.default_result()
  }

  fn visit_parser_rule(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::ParserRuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    let name = &ctx.rule_ref().unwrap().symbol.text;
    let id = *self.nonterminals.get(name).unwrap();
    
    // 解析并填表
    let result = ctx.block().unwrap().accept(self)?.downcast::<(HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>)>().unwrap();
    self.table.insert(id, *result);
    self.default_result()
  }

  // 返回一个 hashset 的元组 (terminal_list, terminal, nonterminal_list, nonterminal) : (HashSet<usize>, ...)
  fn visit_block(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::BlockContext) -> Result<Box<dyn Any>, Box<dyn Error>> {

    let mut result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = (HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new());
      
    for v in ctx.alternative_list().iter() {
      let re = v.accept(self)?.downcast::<(HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>)>().unwrap();
      result.0.extend(re.0);
      result.1.extend(re.1);
      result.2.extend(re.2);
      result.3.extend(re.3);
    }
    Ok(Box::new(result))
  }

  // (terminal_list, terminal, nonterminal_list, nonterminal)
  fn visit_alternative(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::AlternativeContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    let mut result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = (HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new());
    let mut children: Vec<Box<(HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>)>> = Vec::new();
    for elem in ctx.element_list().iter() {
      let child = elem.accept(self)?.downcast::<(HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>)>().unwrap();
      children.push(child);
    }


    for child in children.iter() {
      result.0.extend(&child.0);
      result.2.extend(&child.2);
    }

    for child in children.iter() {
      // // 先处理终结符
      for terminal in child.1.iter() { 
        if result.1.contains(terminal) {
          result.0.insert(*terminal);
          result.1.remove(terminal);
        } else if result.0.contains(terminal) {
          continue;
        } 
        else {
          result.1.insert(*terminal);
        }
      }

      // 然后处理非终结符
      for nonterminal in child.3.iter() {
        if result.3.contains(nonterminal) {
          result.2.insert(*nonterminal);
          result.3.remove(nonterminal);
        } else if result.2.contains(nonterminal) {
          continue;
        } 
        else {
          result.3.insert(*nonterminal);
        }
      }
    }


    Ok(Box::new(result))
  }

  // (terminal_list, terminal, nonterminal_list, nonterminal)
  fn visit_element(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::ElementContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    if let Some(token) = ctx.token_ref() {
      let name = &token.symbol.text;

      // 获取其 id
      let token_id = self.terminals.get(name).unwrap();


      if let Some(suffix) = ctx.ebnf_suffix() {
        // 如果有后缀
        if let Some(_) = suffix.star() {
          // item *
          let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
            (hashset! { *token_id }, hashset! {}, hashset! {}, hashset! {});
          return Ok(Box::new(result));
        } else if let Some(_) = suffix.plus() {
          // item +
          let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
            (hashset! { *token_id }, hashset! {}, hashset! {}, hashset! {});
          return Ok(Box::new(result));
        } else {
          // item ?
          let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
            (hashset! {}, hashset! { *token_id }, hashset! {}, hashset! {});
          return Ok(Box::new(result));
        }
        
      } else {
        // 如果没有后缀
        let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
          (hashset! {}, hashset! { *token_id }, hashset! {}, hashset! {});
        return Ok(Box::new(result));
      }
      
    }
    else if let Some(rule) = ctx.rule_ref() {
      let name = &rule.symbol.text;
      let rule_id = self.nonterminals.get(name).unwrap();

      if let Some(suffix) = ctx.ebnf_suffix() {
        if let Some(_) = suffix.star() {
          // item *
          let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
            (hashset! {}, hashset! {}, hashset! { *rule_id }, hashset! {});
          return Ok(Box::new(result));
        } else if let Some(_) = suffix.plus() {
          // item +
          let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
            (hashset! {}, hashset! {}, hashset! { *rule_id }, hashset! {});
          return Ok(Box::new(result));
        } else {
          // item ?
          let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
            (hashset! {}, hashset! {}, hashset! {}, hashset! { *rule_id });
          return Ok(Box::new(result));
        }
      } else {
        let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
          (hashset! {}, hashset! {}, hashset! {}, hashset! { *rule_id });
        return Ok(Box::new(result));
      }
      
    } else if let Some(block) = ctx.block() {

      let mut result = block.accept(self)?.downcast::<(HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>)>().unwrap();
      if let Some(suffix) = ctx.ebnf_suffix() {
        // 将 0/1 全部添加到 list 中
        if let Some(_) = suffix.star() {
          result.0.extend(&result.1);
          result.2.extend(&result.3);

          result.1.clear();
          result.3.clear();

          return Ok(result);

        } else if let Some(_) = suffix.plus() {
          result.0.extend(&result.1);
          result.2.extend(&result.3);

          result.1.clear();
          result.3.clear();
          return Ok(result);

        }
      }


      return Ok(result);
    } else {
      // 字符串常量，不管
      let result: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>) = 
        (hashset! {}, hashset! {}, hashset! {}, hashset! {});
      return Ok(Box::new(result));
    }
  }


}


