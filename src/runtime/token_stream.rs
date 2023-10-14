
use std::collections::VecDeque;

use crate::runtime::error::ErrorKind;

use super::lexer::TokenIter;
use super::location::Location;
use super::{token::Token, lexer::Lexer};
use super::error::Error;

// 词法分析的时候，直接丢弃掉 skip 的 token, 并将不同频道的 token 放入相应的 vector 。
pub struct TokenStream<'a> {
  // 词法分析器
  // 只需要保存一个迭代器即可
  pub iter: TokenIter<'a>,

  // 当前 token，初始化为 _START
  pub next_token: Option<Token>,

  // 已消耗 token 的缓冲队列, 暂时移除, 替换为 previous token, 也就是只缓存一个 token
  pub consumed_tokens: VecDeque<Token>,

  // 预查看 token 的缓冲队列
  pub cached_tokens: VecDeque<Token>,

  // 该 stream 对应的通道
  pub channel: usize,

}

impl<'a> TokenStream<'a> {

  // 消耗掉 next_token, 并返回 next token 。
  pub fn consume(&mut self) -> Result<Token, Error> {
    match self.next() {
      Some(token) => Ok(token),
      None => Err(Error::create_error_without_location(ErrorKind::TokenStreamOutOfRange, "msg")),
    }
  }


  pub fn release(&mut self) -> Result<Token, Error> {
    // 如果缓存的都已经消耗完了
    if self.consumed_tokens.len() <= 0 { return Err(Error::create_error_without_location(ErrorKind::ConsumedTokenExhausted, "consumed token exhausted.")) }
    
    if let Some(next_token) = &self.next_token {
      // 将 next_token 放回 cached_tokens
      self.cached_tokens.push_front(next_token.clone())
    }

    self.next_token = self.consumed_tokens.pop_back();
    // // 将 consumed_tokens 的最后一个恢复
    Ok(self.next_token.clone().unwrap())
  }


  pub fn look_ahead(&mut self, n: usize) -> Result<Token, Error> {
    if n <= 0 {
      // 至少向前看一个 token
      return Err(Error::create_error_without_location(ErrorKind::Unknown, "look_ahead should gt 0"));
    }
    else if n == 1 {
      return self.peek_next_token();
    }
    else {
      let n = n - 2;
      if n >= self.cached_tokens.len() {
        // 需要再获取 n - len + 1 个 token
        for _ in 0..(n - self.cached_tokens.len() + 2) {
          let next_token = loop {
            let next_token = self.iter.lexer_match()?;
            if next_token.channel == self.channel {
              break next_token;
            }
          };

          self.cached_tokens.push_back(next_token);
        }
        
      } 
      
      return Ok(self.cached_tokens[n].clone());
    }
  }

  pub fn look_back(&mut self, n:usize) -> Result<Token, Error> {
    if n <= 0 {
      // 向后看 0 个 token, 那么就是当前 token
      return Err(Error::create_error_without_location(ErrorKind::Unknown, "look back should gt 0"));
    }
    else if n > self.consumed_tokens.len() {
      // 直接报错
      return Err(Error::create_error_without_location(ErrorKind::ConsumedTokenExhausted, "consumed token exhausted."))
      
    }

    Ok(self.consumed_tokens[self.consumed_tokens.len() - n].clone())
  }

  pub fn peek_next_token(&self) -> Result<Token, Error> {
    match &self.next_token {
      Some(next_token) => Ok(next_token.clone()),
      None => Err(Error::create_error_without_location(ErrorKind::TokenStreamOutOfRange, "msg")),
    }
  }

  pub fn peek_previous_token(&self) -> Result<Token, Error> {
    match self.consumed_tokens.back() {
      Some(token) => Ok(token.clone()),
      None => Err(Error::create_error_without_location(ErrorKind::ConsumedTokenExhausted, "msg")),
    }
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

  pub fn reset(&mut self) {
    self.consumed_tokens.clear();
    self.cached_tokens.clear();
    self.iter.reset();
    self.next_token = Some(Token::start(self.channel));
  }
}


// 直接为 TokenStream 实现 Iterator，无需再定义一个 Iter
impl Iterator for TokenStream<'_> {



  fn next(&mut self) -> Option<Self::Item> {
    // 如果 next_token 已经是 None，表示已经返回了 stop ，此时直接返回 None 即可。

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
          let next_token = match self.iter.lexer_match() {
            Ok(next_token) => next_token,

            // 如果 lexer 扫描不到 token，则将 next_token 设置为 stop 。
            Err(err) => { 
              match err.kind {
                ErrorKind::LexerScanOverflow => {
                  let location = Location::new(self.iter.get_current_position(), self.iter.get_current_position(), self.iter.cursor, self.iter.cursor);
                  // 添加 stop token
                  Token::new(1, "_STOP", "_STOP", 
                    location,
                    self.iter.token_index, 
                    self.channel)
                },
                _ => { return None; }
              }
            },
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








