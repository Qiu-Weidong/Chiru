use std::fmt::Display;

// 定义一个表示产生式的结构体
use super::vocabulary::{NonTerminal, Terminal};

#[derive(PartialEq, Eq, Clone, Hash, Debug, Copy, PartialOrd, Ord)]
pub enum ProductionItem<'a> {
  NonTerminal(NonTerminal<'a>),
  Terminal(Terminal<'a>),
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct Production<'a> {
  pub id: usize,                  // 添加一个产生式的编号
  pub left: NonTerminal<'a>,                // 产生式左部
  pub right: Vec<ProductionItem<'a>>,
}

impl<'a> Production<'a> {
  pub fn new(id: usize, left: NonTerminal<'a>, right: &[ProductionItem<'a>]) -> Self {
    Self {
      id,
      left,
      right: right.to_vec(),
    }
  }
}

impl<'a> Display for Production<'a> {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}


