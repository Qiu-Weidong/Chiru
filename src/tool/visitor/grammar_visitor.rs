

use std::collections::HashMap;

use crate::tool::{grammar::{Grammar, production::{ProductionItem, Production}}, syntaxis::{syntaxis_visitor::SyntaxisVisitor, syntaxis_context::{ElementContext, LexerRuleContext, ParserRuleContext}}};

use super::lexer_rule_visitor::LexerRuleData;

/**
 * 第一遍 将所有的语法规则中的字符串字面量转换为 token
 * 第二遍 将所有词法符号和语法符号编号并填表
 * 第三遍 构造产生式
 */


// 负责将所有词法符号和语法符号编号并填表
pub struct SymbolVisitor<'a> {
  pub grammar: &'a mut Grammar,
  pub data: Vec<LexerRuleData>,
  pub next_token_id: usize,
  pub next_rule_id: usize,
}

impl<'a> SymbolVisitor<'a> {
  pub fn new(grammar: &'a mut Grammar, first_token_id: usize, first_rule_id: usize, 
    data: Vec<LexerRuleData>) -> Self {
    Self { grammar, next_rule_id: first_rule_id, next_token_id: first_token_id, data }
  }
}

impl SyntaxisVisitor for SymbolVisitor<'_> {
  fn visit_lexer_rule(&mut self, ctx: &dyn LexerRuleContext) -> Box<dyn std::any::Any> {
    let name = &ctx.token_ref().unwrap().symbol.text;
    // 检查是否已经定义
    if self.grammar.vocabulary.is_terminal_name_defined(name) {
      println!("重复定义 token: {}", name);
      return self.default_result();
    }

    let regular = &ctx.regular().unwrap().regular_literal().unwrap().symbol.text; // .replace("\\/", "/");


    self.data.push( LexerRuleData { 
      token_type: self.next_token_id, 
      token_name: name.to_owned(), 
      regex: regular.to_owned(),
      skip: false,
      channel: 0, 
    });
    

    self.grammar.vocabulary.add_terminal(self.next_token_id, name);
    self.next_token_id += 1;
    self.default_result()
  }

  fn visit_parser_rule(&mut self, ctx: &dyn ParserRuleContext) -> Box<dyn std::any::Any> {
    let name = &ctx.rule_ref().unwrap().symbol.text;
    if self.grammar.vocabulary.is_terminal_name_defined(name) {
      println!("重复定义 rule: {}", name);
      return self.default_result();
    }

    self.grammar.vocabulary.add_named_nonterminal(self.next_rule_id, name);
    self.next_rule_id += 1;
    self.default_result()
  }
}



// 负责生成产生式
pub struct ProductionVisitor<'a> {
  pub grammar: &'a mut Grammar,
  pub next_rule_id: usize, // 为匿名非终结符编号
  pub next_production_id: usize, // 产生式的编号

  // 先在 visitor 中维护一个匿名非终结符产生式的集合，最后再添加到 grammar 中去。map 的键为产生式右部列表
  block_cache: HashMap<Vec<Vec<ProductionItem>>, usize>,


  star_cache: HashMap<usize, usize>,
  plus_cache: HashMap<usize, usize>,
  question_cache: HashMap<usize, usize>,
}

impl<'a> ProductionVisitor<'a> {
  pub fn new(grammar: &'a mut Grammar, first_rule_id: usize) -> Self {
    Self { grammar, next_rule_id: first_rule_id, next_production_id: 0,
      star_cache: HashMap::new(), plus_cache: HashMap::new(), 
      question_cache: HashMap::new(), block_cache: HashMap::new(), }
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
    let left_id = self.grammar.vocabulary.get_nonterminal_id_by_name(name).unwrap();
    
    for alternative in ctx.block().unwrap().alternative_list().iter() {
      let right = alternative.accept(self);
      let right = right.downcast::<Vec<ProductionItem>>().unwrap();

      let production_id = self.next_production_id;
      self.next_production_id += 1;
      let production = Production::new(production_id, left_id, right.as_ref());

      // 先不检查重复的产生式
      self.grammar.productions.insert(production_id, production);
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
      id = self.grammar.vocabulary.get_terminal_id_by_name(&token.symbol.text).unwrap();
      item = ProductionItem::Terminal(id);
    }
    else if let Some(literal) = ctx.string_literal() {
      id = self.grammar.vocabulary.get_terminal_id_by_name(&literal.symbol.text).unwrap();
      item = ProductionItem::Terminal(id);
    }
    else if let Some(rule) = ctx.rule_ref() {
      id = self.grammar.vocabulary.get_nonterminal_id_by_name(&rule.symbol.text).unwrap();
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

        // 添加一个非终结符 匿名
        self.grammar.vocabulary.add_unnamed_nonterminal(self.next_rule_id);
        let item2 = ProductionItem::NonTerminal(self.next_rule_id);

        let production_id_1 = self.next_production_id;
        let production_id_2 = self.next_production_id + 1;
        self.next_production_id += 2;

        // 添加两条产生式
        let p1 = Production::new(production_id_1, self.next_rule_id, &vec![]);
        let p2 = Production::new(production_id_2, self.next_rule_id, &vec![item, item2.clone()]);

        self.grammar.productions.insert(production_id_1, p1);
        self.grammar.productions.insert(production_id_2, p2);

        self.star_cache.insert(id, self.next_rule_id);

        self.next_rule_id += 1;
        return Box::new(item2);
      }
      else if let Some(_) = suffix.plus() {
        // item * => item2 -> item item2 | item
        if let Some(item_id) = self.plus_cache.get(&id) {
          return Box::new(ProductionItem::NonTerminal(*item_id));
        }

        // 添加一个非终结符 匿名
        self.grammar.vocabulary.add_unnamed_nonterminal(self.next_rule_id);
        let item2 = ProductionItem::NonTerminal(self.next_rule_id);

        // 添加两条产生式
        let production_id_1 = self.next_production_id;
        let production_id_2 = self.next_production_id + 1;
        self.next_production_id += 2;

        let p1 = Production::new(production_id_1, self.next_rule_id, &vec![item.clone()]);
        let p2 = Production::new(production_id_2, self.next_rule_id, &vec![item, item2.clone()]);
        self.grammar.productions.insert(production_id_1, p1);
        self.grammar.productions.insert(production_id_2, p2);

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
        self.grammar.vocabulary.add_unnamed_nonterminal(self.next_rule_id);
        let item2 = ProductionItem::NonTerminal(self.next_rule_id);

        // 添加两条产生式
        let production_id_1 = self.next_production_id;
        let production_id_2 = self.next_production_id + 1;
        self.next_production_id += 2;


        let p1 = Production::new(production_id_1, self.next_rule_id, &vec![]);
        let p2 = Production::new(production_id_2, self.next_rule_id, &vec![item]);
        self.grammar.productions.insert(production_id_1, p1);
        self.grammar.productions.insert(production_id_2, p2);
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
    self.grammar.vocabulary.add_unnamed_nonterminal(id);
    self.block_cache.insert(rights.clone(), id);

    for right in rights.iter() {
      let production_id = self.next_production_id;
      self.next_production_id += 1;
      let production = Production::new(production_id, id, right);
      // if !self.grammar.productions.insert(production_id, production) {
      //   println!("重复产生式");
      // }
      self.grammar.productions.insert(production_id, production);
    } 


    self.next_rule_id += 1;
    Box::new(id)
  }

}







