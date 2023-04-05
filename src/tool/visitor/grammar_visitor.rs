

use std::collections::HashMap;

use crate::tool::{grammar::{Grammar, ProductionItem, Production}, syntaxis::{syntaxis_visitor::SyntaxisVisitor, syntaxis_context::{ElementContext, LexerRuleContext, ParserRuleContext}}};

/**
 * 第一遍 将所有的语法规则中的字符串字面量转换为 token
 * 第二遍 将所有词法符号和语法符号编号并填表
 * 第三遍 构造产生式
 */


pub struct StringLiteralToTokenVisitor<'a> {
  pub grammar: &'a mut Grammar,
  pub next_token_id: usize,
}

impl<'a> StringLiteralToTokenVisitor<'a> {
  fn escape(_input: &str) -> String { todo!() }

  pub fn new(grammar: &'a mut Grammar, first_token_id: usize) -> Self {
    Self { grammar, next_token_id: first_token_id }
  }
}

impl SyntaxisVisitor for StringLiteralToTokenVisitor<'_> {
  fn visit_element(&mut self, ctx: &dyn ElementContext) -> Box<dyn std::any::Any> {
    if let Some(value) = ctx.string_literal() {
      // 需要进行相关转义处理
      let text = StringLiteralToTokenVisitor::escape(&value.symbol.text);// 为了防止字符串的内容和某个 token 同名, 因此不删除引号

      // 检查该字符串是否已经定义
      if self.grammar.terminal_cache.contains_key(&text) { return self.default_result(); } 
      
      // 定义之
      self.grammar.terminal_cache.insert(text, self.next_token_id);
      self.grammar.terminals.insert(self.next_token_id, format!("_T_{}", self.next_token_id));
      self.next_token_id += 1;
    }
    self.default_result()
  }
}



pub struct SymbolVisitor<'a> {
  pub grammar: &'a mut Grammar,
  pub next_token_id: usize,
  pub next_rule_id: usize,
}

impl<'a> SymbolVisitor<'a> {
  pub fn new(grammar: &'a mut Grammar, first_token_id: usize, first_rule_id: usize) -> Self {
    Self { grammar, next_rule_id: first_rule_id, next_token_id: first_token_id }
  }
}

impl SyntaxisVisitor for SymbolVisitor<'_> {
  fn visit_lexer_rule(&mut self, ctx: &dyn LexerRuleContext) -> Box<dyn std::any::Any> {
    let name = &ctx.token_ref().unwrap().symbol.text;
    // 检查是否已经定义
    if self.grammar.terminal_cache.contains_key(name) {
      println!("重复定义 token: {}", name);
      return self.default_result();
    }
    self.grammar.terminal_cache.insert(name.clone(), self.next_token_id);
    self.grammar.terminals.insert(self.next_token_id, name.clone());
    self.next_token_id += 1;
    self.default_result()
  }

  fn visit_parser_rule(&mut self, ctx: &dyn ParserRuleContext) -> Box<dyn std::any::Any> {
    let name = &ctx.rule_ref().unwrap().symbol.text;
    if self.grammar.nonterminal_cache.contains_key(name) {
      println!("重复定义 rule: {}", name);
      return self.default_result();
    }
    self.grammar.nonterminal_cache.insert(name.clone(), self.next_rule_id);
    self.grammar.nonterminals.insert(self.next_rule_id, Some(name.clone()));
    self.next_rule_id += 1;
    self.default_result()
  }
}



pub struct ProductionVisitor<'a> {
  pub grammar: &'a mut Grammar,
  pub next_rule_id: usize,

  // 先在 visitor 中维护一个匿名非终结符产生式的集合，最后再添加到 grammar 中去。
  block_cache: HashMap<Vec<Vec<ProductionItem>>, usize>,


  star_cache: HashMap<usize, usize>,
  plus_cache: HashMap<usize, usize>,
  question_cache: HashMap<usize, usize>,
}

impl<'a> ProductionVisitor<'a> {
  pub fn new(grammar: &'a mut Grammar, first_rule_id: usize) -> Self {
    Self { grammar, next_rule_id: first_rule_id, star_cache: HashMap::new(), plus_cache: HashMap::new(), question_cache: HashMap::new(), block_cache: HashMap::new(), }
  }
}

impl SyntaxisVisitor for ProductionVisitor<'_> {
  // 只需要访问 parser rule
  fn visit_rule_list(&mut self, ctx: &dyn crate::tool::syntaxis::syntaxis_context::RuleListContext) -> Box<dyn std::any::Any> {
    for rule in ctx.parser_rule_list().iter() {
      rule.accept(self);
    }
    self.default_result()
  }

  // 添加命名产生式
  fn visit_parser_rule(&mut self, ctx: &dyn ParserRuleContext) -> Box<dyn std::any::Any> {
    // 这个地方不要调用 block.accept
    let name = &ctx.rule_ref().unwrap().symbol.text;
    let id = *self.grammar.nonterminal_cache.get(name).unwrap();
    
    for alternative in ctx.block().unwrap().alternative_list().iter() {
      let right = alternative.accept(self);
      let right = right.downcast::<Vec<ProductionItem>>().unwrap();
      let production = Production::new(id, right.as_ref());
      
      if !self.grammar.productions.insert(production) {
        println!("产生式重复");
      }
    }
    self.default_result()
  }

  // 返回一条产生式的右部分
  fn visit_alternative(&mut self, ctx: &dyn crate::tool::syntaxis::syntaxis_context::AlternativeContext) -> Box<dyn std::any::Any> {
    if let Some(_) = ctx.epsilon() {
      return Box::new(Vec::<ProductionItem>::new());
    }

    let mut result: Vec<ProductionItem> = Vec::new();
    for element in ctx.element_list().iter() {
      let elem = element.accept(self)
        .downcast::<ProductionItem>().unwrap();
      result.push(elem.as_ref().clone());
    }

    Box::new(result)
  }

  // 返回产生式的元素
  fn visit_element(&mut self, ctx: &dyn ElementContext) -> Box<dyn std::any::Any> {
    // 首先解析出一个 item
    let item: ProductionItem; let id: usize;
    if let Some(token) = ctx.token_ref() {
      // if let None = self.grammar.terminal_cache.get(&token.symbol.text) {
      //   println!("{}", token.symbol.text);
      // }
      id = *self.grammar.terminal_cache.get(&token.symbol.text).unwrap();
      item = ProductionItem::Terminal(id);
    }
    else if let Some(literal) = ctx.string_literal() {
      id = *self.grammar.terminal_cache.get(&literal.symbol.text).unwrap();
      item = ProductionItem::Terminal(id);
    }
    else if let Some(rule) = ctx.rule_ref() {
      id = *self.grammar.nonterminal_cache.get(&rule.symbol.text).unwrap();
      item = ProductionItem::NonTerminal(id);
    }
    else if let Some(block) = ctx.block() {
      id = *block.accept(self).downcast::<usize>().unwrap();
      item = ProductionItem::NonTerminal(id);
    }
    else {
      panic!("解析element出错");
    }



    if let Some(suffix) = ctx.ebnf_suffix() {
      if let Some(_) = suffix.star() {
        // item * => item2 -> item item2 | epsilon

        // 查看是否已存在 item_id 即为 新产生式的 id。
        if let Some(item_id) = self.star_cache.get(&id) {
          return Box::new(ProductionItem::NonTerminal(*item_id));
        }

        // 添加一个非终结符
        self.grammar.nonterminals.insert(self.next_rule_id, None);
        let item2 = ProductionItem::NonTerminal(self.next_rule_id);

        // 添加两条产生式
        let p1 = Production::new(self.next_rule_id, &vec![]);
        let p2 = Production::new(self.next_rule_id, &vec![item, item2.clone()]);

        self.grammar.productions.insert(p1);
        self.grammar.productions.insert(p2);

        self.star_cache.insert(id, self.next_rule_id);

        self.next_rule_id += 1;
        return Box::new(item2);
      }
      else if let Some(_) = suffix.plus() {
        // item * => item2 -> item item2 | item
        if let Some(item_id) = self.plus_cache.get(&id) {
          return Box::new(ProductionItem::NonTerminal(*item_id));
        }

        // 添加一个非终结符
        self.grammar.nonterminals.insert(self.next_rule_id, None);
        let item2 = ProductionItem::NonTerminal(self.next_rule_id);

        // 添加两条产生式
        let p1 = Production::new(self.next_rule_id, &vec![item.clone()]);
        let p2 = Production::new(self.next_rule_id, &vec![item, item2.clone()]);
        self.grammar.productions.insert(p1);
        self.grammar.productions.insert(p2);

        self.plus_cache.insert(id, self.next_rule_id);
        self.next_rule_id += 1;
        return Box::new(item2);
      }
      else {
        // item * => item2 -> item | epsilon
        if let Some(item_id) = self.question_cache.get(&id) {
          return Box::new(ProductionItem::NonTerminal(*item_id));
        }

        // 添加一个非终结符
        self.grammar.nonterminals.insert(self.next_rule_id, None);
        let item2 = ProductionItem::NonTerminal(self.next_rule_id);

        // 添加两条产生式
        let p1 = Production::new(self.next_rule_id, &vec![]);
        let p2 = Production::new(self.next_rule_id, &vec![item]);
        self.grammar.productions.insert(p1);
        self.grammar.productions.insert(p2);
        self.question_cache.insert(id, self.next_rule_id);
        self.next_rule_id += 1;
        return Box::new(item2);
      }
    }
    else {
      Box::new(item)
    }
    // 先不管合并，直接走
  }

  // 返回产生式的元素
  fn visit_block(&mut self, ctx: &dyn crate::tool::syntaxis::syntaxis_context::BlockContext) -> Box<dyn std::any::Any> {
    // 先得出一个产生式右部的集合
    let mut rights = Vec::new();
    for alternative in ctx.alternative_list().iter() {
      let right = alternative.accept(self).downcast::<Vec<ProductionItem>>().unwrap();
      rights.push(right.as_ref().clone());
    }

    if let Some(id) = self.block_cache.get(&rights) {
      return Box::new(*id);
    }



    // 添加一条产生式, 一个非终结符，并返回其 id  ( xx | xxx)  (xxx xxx)* 检查是否已经存在, 否则新建并返回 NonTerminal(id)。
    let id = self.next_rule_id;
    self.grammar.nonterminals.insert(id, None);
    self.block_cache.insert(rights.clone(), id);

    for right in rights.iter() {
      let production = Production::new(id, right);
      if !self.grammar.productions.insert(production) {
        println!("重复产生式");
      }
    } 


    self.next_rule_id += 1;
    Box::new(id)
  }

}







