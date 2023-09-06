

use std::{collections::HashMap, error::Error, any::Any};
use chiru::runtime::production::{Production, ProductionItem};
use crate::tool::{grammar::{Grammar, lexer_rule::LexerRule}, syntaxis::{chiru_visitor::ChiruVisitor, chiru_context::{ElementContext, ParserRuleContext, BlockContext}}};




/**
 * 第一遍 将所有的语法规则中的字符串字面量转换为 token
 * 第二遍 将所有词法符号和语法符号编号并填表
 * 第三遍 构造产生式
 */




// 负责生成产生式
pub struct GrammarVisitor {
  pub grammar: Grammar,
  
  pub next_rule_id: usize, // 为匿名非终结符编号
  pub next_production_id: usize, // 产生式的编号

  // 先在 visitor 中维护一个匿名非终结符产生式的集合，最后再添加到 grammar 中去。map 的键为产生式右部列表
  block_cache: HashMap<Vec<Vec<ProductionItem>>, usize>,
  star_cache: HashMap<ProductionItem, usize>,
  plus_cache: HashMap<ProductionItem, usize>,
  question_cache: HashMap<ProductionItem, usize>,
}

impl GrammarVisitor {
  pub fn new(name: &str, parser_rule_map: &HashMap<String, usize>, lexer_rule_map: &HashMap<String, LexerRule>) -> Self {
    let mut grammar = Grammar::new(name);

    grammar.lexer_rule_map = lexer_rule_map.clone();

    // 将已经识别的终结符加入 vocabulary 
    lexer_rule_map.values().for_each(|v| { grammar.vocabulary.add_terminal(v.token_type, &v.token_name); });

    parser_rule_map.iter().for_each(|(name, id)| {
      grammar.vocabulary.add_named_nonterminal(*id, name);
    });

    let next_rule_id = *parser_rule_map.values().max().unwrap_or(&0) + 1;

    Self { 
      grammar, 
      next_rule_id, 
      next_production_id: 0,
      star_cache: HashMap::new(), plus_cache: HashMap::new(), 
      question_cache: HashMap::new(), block_cache: HashMap::new(), 
    }
  }
}

impl ChiruVisitor for GrammarVisitor {

  fn visit_grammar_name(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::GrammarNameContext) -> Result<Box<dyn Any>, Box<dyn Error>> {

    let name;
    if let Some(name_) = ctx.token_ref() {
      name = &name_.symbol.text;
    } else if let Some(name_) = ctx.rule_ref() {
      name = &name_.symbol.text;
    } else {
      todo!()
    }

    self.grammar.name = name.to_owned();
    self.default_result()
  }
  
  // 只需要访问 parser rule 无需返回值  -> void
  fn visit_rules(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::RulesContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    for rule in ctx.parser_rule_list().iter() {
      rule.accept(self)?;
    }
    self.default_result()
  }



  // 添加命名产生式 无需返回值 -> void
  fn visit_parser_rule(&mut self, ctx: &dyn ParserRuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    // 这个地方不要调用 block.accept
    let name = &ctx.rule_ref().unwrap().symbol.text;
    let left_id = self.grammar.vocabulary.get_nonterminal_id_by_name(name).unwrap();
    
    for alternative in ctx.block().unwrap().alternative_list().iter() {
      let right = alternative.accept(self)?;
      let right = right.downcast::<Vec<ProductionItem>>().unwrap();

      let production_id = self.next_production_id;
      self.next_production_id += 1;
      let production = Production::new(production_id, left_id, right.as_ref());

      // 先不检查重复的产生式 这里不检查才是合理的。
      self.grammar.productions.insert(production_id, production);
    }
    self.default_result()
  }



  // 返回一条产生式的右部分 -> Vec<ProductionItem>
  fn visit_alternative(&mut self, ctx: &dyn crate::tool::syntaxis::chiru_context::AlternativeContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    if let Some(_) = ctx.epsilon() {
      return Ok(Box::new(Vec::<ProductionItem>::new()));
    }

    let mut result: Vec<ProductionItem> = Vec::new();
    for element in ctx.element_list().iter() {
      let elem = element.accept(self)?
        .downcast::<ProductionItem>().unwrap();
      result.push(elem.as_ref().clone());
    }

    Ok(Box::new(result))
  }



  // 返回产生式的元素 -> ProductionItem
  fn visit_element(&mut self, ctx: &dyn ElementContext) -> Result<Box<dyn Any>, Box<dyn Error>> {

    // 首先解析出一个 item
    let item: ProductionItem; // let id: usize;
    if let Some(token) = ctx.token_ref() {
      let id = self.grammar.vocabulary.get_terminal_id_by_name(&token.symbol.text).unwrap();
      item = ProductionItem::Terminal(id);
    }
    else if let Some(literal) = ctx.string_literal() {
      let id = self.grammar.vocabulary.get_terminal_id_by_name(&literal.symbol.text).unwrap();
      item = ProductionItem::Terminal(id);
    }
    else if let Some(rule) = ctx.rule_ref() {
      let id = self.grammar.vocabulary.get_nonterminal_id_by_name(&rule.symbol.text).unwrap();
      item = ProductionItem::NonTerminal(id);
    }
    else if let Some(block) = ctx.block() {
      item = *block.accept(self)?.downcast::<ProductionItem>().unwrap();
    }
    else {
      panic!("解析element出错");
    }



    if let Some(suffix) = ctx.ebnf_suffix() {
      if let Some(_) = suffix.star() {
        // item * => item2 -> item item2 | epsilon

        if let Some(item_id) = self.star_cache.get(&item) {
          return Ok(Box::new(ProductionItem::NonTerminal(*item_id)));
        }

        // 添加一个非终结符 匿名 item2
        self.grammar.vocabulary.add_unnamed_nonterminal(self.next_rule_id);
        let item2 = ProductionItem::NonTerminal(self.next_rule_id);

        let production_id_1 = self.next_production_id;
        let production_id_2 = self.next_production_id + 1;
        self.next_production_id += 2;

        // 添加两条产生式
        let p1 = Production::new(production_id_1, self.next_rule_id, &vec![]);
        let p2 = Production::new(production_id_2, self.next_rule_id, &vec![item, item2]);

        self.grammar.productions.insert(production_id_1, p1);
        self.grammar.productions.insert(production_id_2, p2);

        self.star_cache.insert(item, self.next_rule_id);

        self.next_rule_id += 1;
        return Ok(Box::new(item2));
      }
      else if let Some(_) = suffix.plus() {
        // item * => item2 -> item item2 | item
        if let Some(item_id) = self.plus_cache.get(&item) {
          return Ok(Box::new(ProductionItem::NonTerminal(*item_id)));
        }

        // 添加一个非终结符 匿名
        self.grammar.vocabulary.add_unnamed_nonterminal(self.next_rule_id);
        let item2 = ProductionItem::NonTerminal(self.next_rule_id);

        // 添加两条产生式
        let production_id_1 = self.next_production_id;
        let production_id_2 = self.next_production_id + 1;
        self.next_production_id += 2;

        let p1 = Production::new(production_id_1, self.next_rule_id, &vec![item]);
        let p2 = Production::new(production_id_2, self.next_rule_id, &vec![item, item2]);
        self.grammar.productions.insert(production_id_1, p1);
        self.grammar.productions.insert(production_id_2, p2);

        self.plus_cache.insert(item, self.next_rule_id);
        self.next_rule_id += 1;
        return Ok(Box::new(item2));
      }
      else {
        // item * => item2 -> item | epsilon
        if let Some(item_id) = self.question_cache.get(&item) {
          return Ok(Box::new(ProductionItem::NonTerminal(*item_id)));
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
        self.question_cache.insert(item, self.next_rule_id);
        self.next_rule_id += 1;
        return Ok(Box::new(item2));
      }
    }
    else {
      // 直接返回 item 即可
      Ok(Box::new(item))
    }
    // 先不管合并，直接走
  }



  // 返回产生式的元素 -> ProductionItem
  fn visit_block(&mut self, ctx: &dyn BlockContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    // 先得出一个产生式右部的集合
    let rights = 
      ctx.alternative_list().iter().map(|alternative| {
        alternative.accept(self).unwrap()
          .downcast::<Vec<ProductionItem>>().unwrap().as_ref().clone()
      }).collect::<Vec<_>>();

    // 先检查是否缓存中存在  
    if let Some(id) = self.block_cache.get(&rights) {
      return Ok(Box::new(ProductionItem::NonTerminal(*id)));
    }



    // 添加一个非终结符，并返回其 id  ( xx | xxx)  (xxx xxx)* 检查是否已经存在, 否则新建并返回 NonTerminal(id)。
    // 每个右部都添加一个产生式
    let id = self.next_rule_id;
    self.next_rule_id += 1;

    self.grammar.vocabulary.add_unnamed_nonterminal(id);

    // 插入缓存
    self.block_cache.insert(rights.clone(), id);

    for right in rights.iter() {
      let production_id = self.next_production_id;
      self.next_production_id += 1;
      let production = Production::new(production_id, id, right);

      self.grammar.productions.insert(production_id, production);
    } 


    
    Ok(Box::new(ProductionItem::NonTerminal(id)))
  }

}







