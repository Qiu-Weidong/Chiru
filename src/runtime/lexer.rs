

// use super::{token::Token, token_stream::TokenStream};

use super::token::Token;

// 定义一个词法分析的错误
pub struct Error;

pub trait Lexer {
  // scan 的过程中会直接丢弃掉 skip 和 ignore 的 token 。
  fn scan(&self) -> Result<Token, Error>;

  fn scan_on_channel(&self, channel: usize) -> Result<Token, Error>;

  fn scan_all_tokens(&self) -> Vec<Token>;

  fn scan_all_tokens_on_channel(&self, channel: usize) -> Vec<Token>;


  fn lb(&self, k: i32) -> Option<Token>;

  fn lt(&self, k: i32) -> Option<Token>;

  fn la(&self, k: i32) -> usize;

  fn seek(&self, k: usize);

  fn consume(&self);

}



