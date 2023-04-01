// 定义一个数据结构来表示文法

use std::{collections::{HashMap, HashSet}, fmt::Display};

pub struct Grammar {
  // 文法的名称
  pub name: String,

  // 命名非终结符的查询缓存
  pub nonterminal_cache: HashMap<String, usize>,

  // 终结符的查询缓存
  pub terminal_cache: HashMap<String, usize>,

  // 产生式的查询缓存(匿名)
  pub production_cache: HashMap<Vec<Production>, usize>,


  // 所有非终结符，包括未命名
  pub nonterminals: HashMap<usize, Option<String>>,

  // 所有终结符
  pub terminals: HashMap<usize, String>,

  // 产生式, 显然不能重复
  pub productions: HashSet<Production>,

  // 通过产生式的左部来管理产生式 产生式的优先级 产生式在 vec 中的顺序即为其优先级
  // pub productions: HashMap<usize, Vec<Production> >
  // 当需要查询是否由相同产生式的时候，只比较右部
}


impl Display for Grammar {
  /**
   * 文法名称
   * 所有非终结符以及id。
   * 所有终结符以及id。
   * 所有产生式。
   */
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}


#[derive(PartialEq, Eq, Clone, Hash)]
pub enum ProductionItem {
  NonTerminal(usize),
  Terminal(usize),
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Production {
  pub left: usize, // 产生式的左部 删除
  pub right: Vec<ProductionItem>,

  // 产生式的优先级
}

impl Production {
  pub fn new(left: usize, right: &Vec<ProductionItem>) -> Self {
    Self { left, right: right.clone() }
  }
}


