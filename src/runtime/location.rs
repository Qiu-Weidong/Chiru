use std::ops::Range;

use super::position::Position;



// 用于描述 token 或者 error 的位置
#[derive(Clone, Debug)]
pub struct Location {

  
  pub start: Position,
  pub stop: Position,

  // 字节索引范围, 如果不对会导致 panic
  pub byte_index_range: Range<usize>,

}

