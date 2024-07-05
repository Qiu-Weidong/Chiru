// 定义一个表示产生式的结构体

#[derive(PartialEq, Eq, Clone, Hash, Debug, Copy, PartialOrd, Ord)]
pub enum ProductionItem {
  NonTerminal(usize),
  Terminal(usize),
}

#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct Production {
  pub id: usize,                  // 添加一个产生式的编号
  pub left: usize,                // 产生式左部
  pub right: Vec<ProductionItem>,
}

impl Production {
  pub fn new(id: usize, left: usize, right: &[ProductionItem]) -> Self {
    Self {
      id,
      left,
      right: right.to_vec(),
    }
  }
}
