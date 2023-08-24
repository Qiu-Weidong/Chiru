
use std::{collections::HashMap, vec};

use super::error::{Error, ErrorKind};

use super::token::Token;

pub trait Lexer {


  fn scan_all_on_channel_tokens(&mut self, channel: usize) -> Vec<Token> {
    let mut result = vec![Token::start(channel)];
    while let Ok(token) = self.scan_on_channel(channel) {
      result.push(token);
    }
    
    result
  }

  fn scan_all_tokens_and_group_by_channel(&mut self) -> HashMap<usize, Vec<Token>> {
    let mut ret: HashMap<usize, Vec<Token>> = HashMap::new();
    while let Ok(token) = self.scan() {
      if token.token_type == 1 {
        // 为所有的 token 序列添加 stop token
        for (key, tokens) in ret.iter_mut() {
          let mut stop = token.clone();
          stop.channel = *key;
          tokens.push(stop);
        }

        break;
      }

      if ret.contains_key(&token.channel) {
        let x = ret.get_mut(&token.channel).unwrap();
        x.push(token)
      }
      else {
        ret.insert(token.channel, vec![Token::start(token.channel), token]);
      }
    }


    
    ret
  }

  // 把所有 token 都读出来，这种情况下，start 和 stop 的 channel 都为 0 
  fn scan_all_tokens(&mut self) -> Vec<Token> {
    let mut result = vec![Token::start(0)];
    while let Ok(token) = self.scan() {
      result.push(token);
    }
    
    result
  }
  





  // 向前扫描，如果是 skip 则继续向前扫描, skip 的 token 不会占用序号
  fn scan(&mut self) -> Result<Token, Error> {
    match self.lexer_match() {
      Ok(token) => Ok(token), 
      Err(err) => match err.kind {
          ErrorKind::LexerNoMatch => self.recover(),
          _ => Err(err),
        },
    }
  }
  
  // 可以扫描下一个是什么
  // 扫描指定 channel 的 token，其余都舍弃, 会识别到 stop。
  fn scan_on_channel(&mut self, channel: usize) -> Result<Token, Error> {
    loop {
      match self.lexer_match() {
        Ok(token) => {
          // stop 需要特判
          if token.token_type == 1 {
            let mut token = token;
            token.channel = channel;
            return Ok(token);
          }

          if token.channel == channel {
            return Ok(token);
          }
        },
        Err(err) => {
          match err.kind {
            ErrorKind::LexerNoMatch => {
              let token = self.recover()?;
              if token.channel == channel { return Ok(token); }
            },
            _ => return Err(err),
          }
        },
      }
    }
  }



  fn reset(&mut self);


  // 这个函数只管匹配，匹配不上就报一个 Error。
  fn lexer_match(&mut self) -> Result<Token, Error>;

  // 这个函数用于当 lexer_match 发生错误的时候进行一些修复工作
  fn recover(&mut self) -> Result<Token, Error>;

}



