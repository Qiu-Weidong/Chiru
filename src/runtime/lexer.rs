
use std::{collections::HashMap, vec};
use regex::Regex;

use super::error::Error;

use super::token::Token;

pub trait Lexer {


  fn scan_all_on_channel_tokens(&mut self, channel: usize) -> Vec<Token> {
    let mut result = vec![Token::start(channel)];
    while let Ok(token) = self.scan() {
      if token.channel == channel {
        result.push(token);
      } else if token.token_type == 1 {
        let mut token = token;
        token.channel = channel;
        result.push(token)
      }
    }
    
    result
  }

  fn scan_all_tokens_and_group_by_channel(&mut self) -> HashMap<usize, Vec<Token>> {
    let mut ret: HashMap<usize, Vec<Token>> = HashMap::new();
    while let Ok(token) = self.scan() {
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

  // 把所有 token 都读出来
  fn scan_all_tokens(&mut self) -> Vec<Token> {
    let mut result = vec![Token::start(0)];
    while let Ok(token) = self.scan() {
      result.push(token);
    }
    
    result
  }
  

  // 向前扫描，如果是 skip 则继续向前扫描, skip 的 token 不会占用序号
  fn scan(&mut self) -> Result<Token, Error>;
  
  // 可以扫描下一个是什么
  // 扫描指定 channel 的 token，其余都舍弃
  fn scan_on_channel(&mut self, channel: usize) -> Result<Token, Error> {
    loop {
      let mut token = self.scan()?;
      if token.token_type == 1 {
        token.channel = channel;
      }

      if token.channel == channel {
        return Ok(token)
      }
    }
  }

  fn reset(&mut self);


  fn lexer_match(&self, regex_list: &[(Regex, usize, &'static str, bool)]) -> Result<Token, Error> {
    todo!()
  }

}



