

use std::collections::{HashMap, HashSet};
use tera::Context;

use crate::tool::{grammar::{production::Production, Grammar}, code_gen::pascal};

use super::TEMPLATES;


pub fn ll1_table_generate(table: &HashMap<(usize, usize), usize>) -> String {
  // 输出预测分析表

  // 首先将预测分析表转换成元组列表
  let table = table.iter().map(|((k1, k2), k3)| (*k1, *k2, *k3)).collect::<Vec<_>>();

  let mut context = Context::new();
  context.insert("table", &table);

  let result = TEMPLATES.render("target/rust/ll1_table.tera", &context).unwrap();

  
  result
}


fn production_right_generate(production: &Production) -> String {
  let mut result = String::from("vec![");
  for item in production.right.iter() {
    match item {
      crate::tool::grammar::production::ProductionItem::NonTerminal(id) => {
        result += &format!("ProductionItem::NonTerminal({}),", id);
      },
      crate::tool::grammar::production::ProductionItem::Terminal(id) => {
        result += &format!("ProductionItem::Terminal({}),", id);
      },
    }
  }
  result += "]";
  result
}

pub fn production_generate(productions: &HashMap<usize, Production>) -> String {
  // (production_id: usize, left_id: usize, right: String )
  let productions = productions.iter().map(|(id, production)| {
    // 求出 production 的 right 对应的字符串
    return (*id, production.left, production_right_generate(production));
  }).collect::<Vec<_>>();


  let mut context = Context::new();
  context.insert("productions", &productions);

  let result = TEMPLATES.render("target/rust/production.tera", &context).unwrap();

  
  result
}


// 生成 parser 的函数
pub fn parser_generate(grammar: Grammar) -> String {
  
  let (first, first_set) = grammar.first_set();

  let follow = grammar.follow_set(&first);

  let table = grammar.ll1_table(&first_set, &follow);

  let table = table.iter().map(|((k1, k2), k3)| (*k1, *k2, *k3)).collect::<Vec<_>>();
  let productions = grammar.productions.iter().map(|(id, production)| {
    // 求出 production 的 right 对应的字符串
    return (*id, production.left, production_right_generate(production));
  }).collect::<Vec<_>>();


  let mut sync: HashSet<(usize, usize)> = HashSet::new();
  // 根据 follow 集合来生成 sync
  for (id, followers) in follow.iter() {
    for x in followers.iter() {
      sync.insert((*id, *x));
    }
  }

  // let sync = sync.iter().map(|(x, y)| {

  // }).collect::<Vec<_>>();
  let sync = sync.iter().cloned().collect::<Vec<_>>();




  // 非终结符 0: 编号 1 小写 2 大写 3 pascal
  let mut nonterminals: Vec<(usize, String, String, String)> = Vec::new();
  for (id, t) in grammar.vocabulary.nonterminals.iter() {
    if let Some(name) = &t.name {
      nonterminals.push((*id, name.clone(), name.to_uppercase(), pascal(&name)));
    }
  }


  let terminals = grammar.vocabulary.terminals.iter().map(|(id, t)| {
    (*id, t.name.clone())
  }).collect::<Vec<_>>();




  let mut context = Context::new();
  context.insert("table", &table);
  context.insert("productions", &productions);
  context.insert("nonterminals", &nonterminals);
  context.insert("terminals", &terminals);
  context.insert("sync_list", &sync);
  context.insert("name", &pascal(&grammar.name));

  let result = TEMPLATES.render("target/rust/parser.tera", &context).unwrap();

  
  result
}








