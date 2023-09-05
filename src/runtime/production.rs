// 定义一个表示产生式的结构体



#[derive(PartialEq, Eq, Clone, Hash, Debug, Copy)]
pub enum ProductionItem {
  NonTerminal(usize),
  Terminal(usize),
}


#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Production {
  pub id: usize, // 添加一个产生式的编号
  pub left: usize, 
  pub right: Vec<ProductionItem>,
}

impl Production {
  pub fn new(id: usize, left: usize, right: &Vec<ProductionItem>) -> Self {
    Self { id, left, right: right.clone() }
  }
}





