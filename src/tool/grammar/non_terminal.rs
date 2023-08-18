// 定义一个表示非终结符的结构体

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonTerminal {
  pub id: usize,

  // 注意，非终结符可能匿名
  pub name: Option<String>,
}

impl NonTerminal {
  pub fn new(id: usize, name: Option<String>) -> Self {
    NonTerminal { id, name }
  }
}




