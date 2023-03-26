use std::collections::HashMap;

use super::token::Token;

// 词法分析的时候，直接丢弃掉 skip 的 token, 并将不同频道的 token 放入相应的 vector 。
pub struct TokenStream {
  pub streams: HashMap<usize, Vec<Token>>,
}




