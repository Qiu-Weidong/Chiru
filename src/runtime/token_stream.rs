
use std::collections::VecDeque;

use super::lexer::TokenIter;
use super::location::Location;
use super::token::Token;
use super::error::Error;

// 词法分析的时候，直接丢弃掉 skip 的 token, 并将不同频道的 token 放入相应的 vector 。
pub struct TokenStream<'a> {
  // 词法分析器
  pub iter: TokenIter<'a>,

  // 当前 token，初始化为 _START
  pub next_token: Option<Token<'a>>,

  // 上一个 token
  pub previous_token: Option<Token<'a>>,

  // 预查看 token 的缓冲队列
  pub cached_tokens: VecDeque<Token<'a>>,

  // 该 stream 对应的通道
  pub channel: usize,

}

impl<'a> TokenStream<'a> {

  // 消耗掉 next_token, 并返回 next token 。
  pub fn consume(&mut self) -> Result<Token, Error> {
    // previous token 是 stop, next token 是 None, 表示已经消耗了 stop, 此时应该返回 None

    if let Some(token) = self.next_token.clone() {
      // 消耗一个 token

      // 更新 previous token
      self.previous_token = self.next_token.clone();
      if self.cached_tokens.len() > 0 {
        // 如果缓存中还有token
        self.next_token = self.cached_tokens.pop_front();
      } else if token.terminal.id == 1 {

        self.next_token = None;
      } else {

        let next_token = loop {
          let next_token = match self.iter.lexer_match() {
            Ok(next_token) => next_token,

            // 如果 lexer 扫描不到 token，则将 next_token 设置为 stop 。
            Err(err) => { 
              match err {
                Error::LexerScanOverflow => {
                  let location = Location::new(self.iter.get_current_position(), self.iter.get_current_position(), self.iter.cursor, self.iter.cursor);
                  // 添加 stop token
                  Token::new(1, "_STOP", "_STOP", location,self.iter.token_index, self.channel)
                },
                _ => { return Err(err); }
              }
            },
          };
          if next_token.channel == self.channel {
            break next_token;
          }
        };
        self.next_token = Some(next_token);
      }
      
      Ok(token)

    } else {
      Err(Error::token_stream_out_of_range())
    }
  }

  pub fn look_ahead(&mut self, n: usize) -> Result<Token, Error> {
    if n <= 0 {
      // 至少向前看一个 token
      return Err(Error::Unknown);
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

  pub fn peek_next_token(&self) -> Result<Token, Error> {
    match &self.next_token {
      Some(next_token) => Ok(next_token.clone()),
      None => Err(Error::token_stream_out_of_range()),
    }
  }

  pub fn peek_previous_token(&self) -> Result<Token, Error> {
    match &self.previous_token {
      Some(previous_token) => Ok(previous_token.clone()),
      None => Err(Error::token_stream_out_of_range()),
    }
  }

  pub fn new(lexer: &'a dyn Lexer, channel: usize) -> Self {
    Self {
      iter: lexer.iter(),
      channel,
      cached_tokens: VecDeque::new(),

      // 我们认为 start 和 stop 永远都和当前 stream 一样。
      next_token: Some(Token::start(channel)),
      previous_token: None,
    }
  }

  // 重置 token_stream
  pub fn reset(&mut self) {
    self.cached_tokens.clear();
    self.iter.reset();
    self.next_token = Some(Token::start(self.channel));
    self.previous_token = None;
  }
}


// // 直接为 TokenStream 实现 Iterator，无需再定义一个 Iter
// impl<'a, 'b> Iterator for TokenStream<'b> {

//   fn next(&mut self) -> Option<Self::Item> {
//     match self.consume() {
//       Ok(token) => Some(token),
//       Err(_) => None,
//     }
//   }

//   // type Item<'next> = Token<'next> where Self: 'next;
//   type Item = Token<'a>;
// }








