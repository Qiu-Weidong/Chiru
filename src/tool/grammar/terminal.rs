// 定义一个表示终结符的结构体
#[derive(Debug, Clone)]
pub struct  Terminal {
  pub id: usize,
  pub name: String,
}

impl Terminal {
  pub fn new(id: usize, name: &str) -> Self {
    Terminal { id, name: name.to_owned() }
  }
}




