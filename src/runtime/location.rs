
use super::position::Position;



// 用于描述 token 或者 error 的位置
#[derive(Clone, Debug, Default, Copy)]
pub struct Location {

  // 起始位置和结束位置，左闭右开，行号从0开始编号
  pub start: Position,
  pub stop: Position,

  // 字节索引范围, 如果不对会导致 panic, 左闭右开
  pub byte_index_start: usize,
  pub byte_index_stop: usize,

}

impl Location {
  pub fn new(start: Position, stop: Position, byte_index_start: usize, byte_index_stop: usize) -> Self {
    Self {
      start, stop, byte_index_start, byte_index_stop,
    }
  }
}


