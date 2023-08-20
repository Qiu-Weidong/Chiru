
use std::collections::HashMap;

use super::token::Token;

// 定义一个词法分析的错误
#[derive(Debug)]
pub struct Error;

pub trait Lexer {
  
  fn scan_all_tokens(&mut self) -> Vec<Token>;
  
  fn scan_all_on_channel_tokens(&mut self, channel: usize) -> Vec<Token>;

  fn scan_all_tokens_and_group_by_channel(&mut self) -> HashMap<usize, Vec<Token>>;


  

  // 向前扫描，如果是 skip 则继续向前扫描, skip 的 token 不会占用序号
  fn scan(&mut self) -> Result<Token, Error>;
  
  // 可以扫描下一个是什么
  fn scan_on_channel(&mut self, channel: usize) -> Result<Token, Error>;
  
  fn look_ahead(&mut self, n: usize) -> Result<Token, Error>;

  fn look_back(&mut self, n:usize) -> Result<Token, Error>;

  fn look_ahead_on_channel(&mut self, channel: usize, n: usize) -> Result<Token, Error>;

  fn look_back_on_channel(&mut self, channel: usize, n:usize) -> Result<Token, Error>;

  fn consume(&mut self);

  fn release(&mut self);




/*
  fn lb(&self, k: i32) -> Option<Token>;

  fn lt(&self, k: i32) -> Option<Token>;

  fn la(&self, k: i32) -> usize;

  fn seek(&self, k: usize);

  fn consume(&self);
 */
}



