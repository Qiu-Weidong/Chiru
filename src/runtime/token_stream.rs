
use std::collections::VecDeque;

use crate::runtime::error::ErrorKind;

use super::lexer::TokenIter;
use super::{token::Token, lexer::Lexer};
use super::error::Error;

// 词法分析的时候，直接丢弃掉 skip 的 token, 并将不同频道的 token 放入相应的 vector 。
pub struct TokenStream<'a> {
  // 词法分析器
  // 只需要保存一个迭代器即可
  pub iter: TokenIter<'a>,

  // 当前 token，初始化为 _START
  pub next_token: Option<Token>,

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
    todo!()
  }


  pub fn release(&mut self) -> Result<Token, Error> {
    // 如果缓存的都已经消耗完了
    if self.consumed_tokens.len() <= 0 { return Err(Error::create_error_without_location(ErrorKind::ConsumedTokenExhausted, "consumed token exhausted.")) }
    
    if let Some(next_token) = &self.next_token {
      self.cached_tokens.push_front(next_token.clone())
    }

    // // 将 consumed_tokens 的最后一个恢复
    // self.current_token = self.consumed_tokens.pop_back().unwrap();
    // Ok(self.current_token.clone())
    todo!()
  }


  pub fn look_ahead(&mut self, n: usize) -> Result<Token, Error> {
    // if n <= 0 {
    //   // 向前看 0 个 token, 那么就是当前 token
    //   return Ok(self.current_token.clone())
    // }
    // else if n > self.cached_tokens.len() {
    //   // 还需要再扫描一些 token 
    //   for _ in 0..(n-self.cached_tokens.len()) {
    //     // let token = self.lexer.scan_on_channel(self.channel)?;
    //     // self.cached_tokens.push_back(token);
    //     todo!()
    //   }
      
    // }

    // Ok(self.cached_tokens[n-1].clone())
    todo!()
  }

  pub fn look_back(&mut self, n:usize) -> Result<Token, Error> {
    // if n <= 0 {
    //   // 向后看 0 个 token, 那么就是当前 token
    //   return Ok(self.current_token.clone())
    // }
    // else if n > self.consumed_tokens.len() {
    //   // 直接报错
    //   return Err(Error::create_error_without_location(ErrorKind::ConsumedTokenExhausted, "consumed token exhausted."))
      
    // }

    // Ok(self.consumed_tokens[self.consumed_tokens.len() - n].clone())
    todo!()
  }

  pub fn peer_next_token(&mut self) -> Result<Token, Error> {
    self.look_ahead(1)
  }

  pub fn peer_previous_token(&mut self) -> Result<Token, Error> {
    self.look_back(1)
  }

  pub fn new(lexer: &'a dyn Lexer, channel: usize) -> Self {
    Self {
      iter: lexer.iter(),
      channel,
      consumed_tokens: VecDeque::new(),
      cached_tokens: VecDeque::new(),

      // 我们认为 start 和 stop 永远都和当前 stream 一样。
      next_token: Some(Token::start(channel)),
    }
  }


}


// 直接为 TokenStream 实现 Iterator，无需再定义一个 Iter
impl Iterator for TokenStream<'_> {
  fn next(&mut self) -> Option<Self::Item> {
    // 首先保存一下原来的 next_token, 即现在的 current_token 。
    let current_token = self.next_token.clone();

    // 更新 next_token
    if let Some(current_token) = current_token {
      if self.cached_tokens.len() > 0 {
        // 如果缓存中还有 token, 那么直接使用缓存中的 token 。
        self.next_token = self.cached_tokens.pop_front();
      }
      else if current_token.token_type == 1 {
        // 如果 current_token 已经是 stop 了。
        self.next_token = None
      }
      else  {
        let next_token = loop {
          let next_token = match self.iter.next() {
            Some(next_token) => next_token,

            // 如果 lexer 扫描不到 token，则将 next_token 设置为 stop 。
            None => Token::new(1, "_STOP", "_STOP", self.iter.position, self.iter.position, self.iter.token_index, self.channel, self.iter.cursor, self.iter.cursor),
          };
          if next_token.channel == self.channel {
            break next_token;
          }
        };


        self.next_token = Some(next_token);

      }

      // 还要将 current_token 放入 consumed_tokens 中
      self.consumed_tokens.push_back(current_token.clone());
      if self.consumed_tokens.len() > 1024 { self.consumed_tokens.pop_front(); }
      
      Some(current_token)
    }
    else {
      None
    }
    
  }

  type Item = Token;
}








