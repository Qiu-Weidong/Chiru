// 定义一个数据结构来表示文法

use std::collections::{HashMap, HashSet};

pub struct Grammar {
  // 命名非终结符的查询缓存
  pub nonterminal_cache: HashMap<String, usize>,

  // 终结符的查询缓存
  pub terminal_cache: HashMap<String, usize>,

  // 所有非终结符，包括未命名
  pub nonterminals: HashMap<usize, Option<String>>,

  // 所有终结符
  pub terminals: HashMap<usize, String>,

  // 产生式, 显然不能重复
  pub productions: HashSet<Production>,
}

impl Grammar {
  pub fn get_productions_by_rule_name(&self, name: &str) -> Vec<Production> {
    let mut result = Vec::new();
    let id = self.nonterminal_cache[name];
    for production in self.productions.iter() {
      if production.left == id { result.push(production.clone()) }
    }
    result
  }

  pub fn get_productions_by_rule_id(&self, id: usize) -> Vec<Production> {
    let mut result = Vec::new();
    for production in self.productions.iter() {
      if production.left == id { result.push(production.clone()) }
    }
    result
  }

  // 通过产生式右部来查询该产生式是否已经存在
  pub fn get_unnamed_production_by_right(&self, right: &Vec<ProductionItem>) -> Option<Production> {
    for production in self.productions.iter() {
      if production.right == *right { return Some(production.clone()) }
    }
    None
  }


}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum ProductionItem {
  NonTerminal(usize),
  Terminal(usize),
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Production {
  pub left: usize, // 产生式的左部
  pub right: Vec<ProductionItem>,
}

impl Production {
  pub fn new(left: usize, right: &Vec<ProductionItem>) -> Self {
    Self { left, right: right.clone() }
  }
}


