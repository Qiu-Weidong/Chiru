
use std::collections::VecDeque;
// use std::slice::{Iter, IterMut};

// use serde_json::map::Iter;

use super::{token::Token, position::Position, lexer::Lexer, error::ErrorKind};
use super::error::Error;

// 词法分析的时候，直接丢弃掉 skip 的 token, 并将不同频道的 token 放入相应的 vector 。
pub struct TokenStream<'a> {
  // 词法分析器
  // pub lexer: Box<dyn Lexer>,
  pub lexer: &'a mut dyn Lexer,

  // 当前 token，初始化为 _START
  pub current_token: Token,

  // 已消耗 token 的缓冲队列
  pub consumed_tokens: VecDeque<Token>,

  // 预查看 token 的缓冲队列
  pub cached_tokens: VecDeque<Token>,

  // 该 stream 对应的通道
  pub channel: usize,

}


impl<'a> TokenStream<'a> {

  // 消耗掉当前 token, 并返回 next token 。
  pub fn consume(&mut self) -> Result<Token, Error> {
    // 首先将 current_token 放入 consumed token 中去。
    self.consumed_tokens.push_back(self.current_token.clone());
    if self.consumed_tokens.len() > 1024 { self.consumed_tokens.pop_front(); }

    if self.cached_tokens.len() > 0 {
      self.current_token = self.cached_tokens.pop_front().unwrap();
    } else {
      // 使用 scan_on_channel 这个方法来获取 token。
      self.current_token = self.lexer.scan_on_channel(self.channel)?
    }


    Ok(self.current_token.clone())
  }


  pub fn release(&mut self) -> Result<Token, Error> {
    // 如果缓存的都已经消耗完了
    if self.consumed_tokens.len() <= 0 { return Err(Error::create_error_without_location(ErrorKind::ConsumedTokenExhausted, "consumed token exhausted.")) }
    // 将 current_token 插回到 cache_tokens
    self.cached_tokens.push_front(self.current_token.clone());

    // 将 consumed_tokens 的最后一个恢复
    self.current_token = self.consumed_tokens.pop_back().unwrap();
    Ok(self.current_token.clone())
  }


  pub fn look_ahead(&mut self, n: usize) -> Result<Token, Error> {
    if n <= 0 {
      // 向前看 0 个 token, 那么就是当前 token
      return Ok(self.current_token.clone())
    }
    else if n > self.cached_tokens.len() {
      // 还需要再扫描一些 token 
      for _ in 0..(n-self.cached_tokens.len()) {
        let token = self.lexer.scan_on_channel(self.channel)?;
        self.cached_tokens.push_back(token);
      }
      
    }

    Ok(self.cached_tokens[n-1].clone())
  }

  pub fn look_back(&mut self, n:usize) -> Result<Token, Error> {
    if n <= 0 {
      // 向后看 0 个 token, 那么就是当前 token
      return Ok(self.current_token.clone())
    }
    else if n > self.consumed_tokens.len() {
      // 直接报错
      return Err(Error::create_error_without_location(ErrorKind::ConsumedTokenExhausted, "consumed token exhausted."))
      
    }

    Ok(self.consumed_tokens[self.consumed_tokens.len() - n].clone())
  }

  pub fn peer_next_token(&mut self) -> Result<Token, Error> {
    self.look_ahead(1)
  }

  pub fn peer_previous_token(&mut self) -> Result<Token, Error> {
    self.look_back(1)
  }

  pub fn new(lexer: &'a mut dyn Lexer, channel: usize) -> Self {
    let pos = Position { line: 0, char_position: 0 };
    Self {
      lexer,
      channel,
      consumed_tokens: VecDeque::new(),
      cached_tokens: VecDeque::new(),

      // 我们认为 start 和 stop 永远都和当前 stream 一样。
      current_token: Token::new(0, "_START", "_START", pos.clone(), pos, 0, 
      channel, 0, 0),
    }
  }

  
  // pub fn iter(&self) -> Iter<'_, Token> {
  //   todo!()
  // }

  // pub fn iter_mut(&mut self) -> IterMut<'_, Token> {
  //   todo!()
  // }

  // fn foo() -> Iter
}

