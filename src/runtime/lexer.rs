
// use std::{collections::HashMap, vec};

use regex::Regex;

use super::error::{Error, ErrorKind};

use super::error_strategy::error_listener::ErrorListener;
use super::position::Position;
use super::token::Token;


pub trait Lexer {
  fn iter(&self) -> LexerIter ;


  // fn scan_all_on_channel_tokens(&mut self, channel: usize) -> Vec<Token> {
  //   let mut result = vec![Token::start(channel)];
  //   while let Ok(token) = self.scan_on_channel(channel) {
  //     result.push(token);
  //   }
    
  //   result
  // }

  // fn scan_all_tokens_and_group_by_channel(&mut self) -> HashMap<usize, Vec<Token>> {
  //   let mut ret: HashMap<usize, Vec<Token>> = HashMap::new();
  //   while let Ok(token) = self.scan() {
  //     if token.token_type == 1 {
  //       // 为所有的 token 序列添加 stop token
  //       for (key, tokens) in ret.iter_mut() {
  //         let mut stop = token.clone();
  //         stop.channel = *key;
  //         tokens.push(stop);
  //       }

  //       break;
  //     }

  //     if ret.contains_key(&token.channel) {
  //       let x = ret.get_mut(&token.channel).unwrap();
  //       x.push(token)
  //     }
  //     else {
  //       ret.insert(token.channel, vec![Token::start(token.channel), token]);
  //     }
  //   }


    
  //   ret
  // }

  // // 把所有 token 都读出来，这种情况下，start 和 stop 的 channel 都为 0 
  // fn scan_all_tokens(&mut self) -> Vec<Token> {
  //   let mut result = vec![Token::start(0)];
  //   while let Ok(token) = self.scan() {
  //     result.push(token);
  //   }
    
  //   result
  // }
  
  // // 向前扫描，如果是 skip 则继续向前扫描, skip 的 token 不会占用序号
  // fn scan(&mut self) -> Result<Token, Error> {
  //   match self.lexer_match() {
  //     Ok(token) => Ok(token), 
  //     Err(err) => match err.kind {
  //         ErrorKind::LexerNoMatch => self.recover(),
  //         _ => Err(err),
  //       },
  //   }
  // }
  
  // // 可以扫描下一个是什么
  // // 扫描指定 channel 的 token，其余都舍弃, 会识别到 stop。
  // fn scan_on_channel(&mut self, channel: usize) -> Result<Token, Error> {
  //   loop {
  //     match self.lexer_match() {
  //       Ok(token) => {
  //         // stop 需要特判
  //         if token.token_type == 1 {
  //           let mut token = token;
  //           token.channel = channel;
  //           return Ok(token);
  //         }

  //         if token.channel == channel {
  //           return Ok(token);
  //         }
  //       },
  //       Err(err) => {
  //         match err.kind {
  //           ErrorKind::LexerNoMatch => {
  //             let token = self.recover()?;
  //             if token.channel == channel { return Ok(token); }
  //           },
  //           _ => return Err(err),
  //         }
  //       },
  //     }
  //   }
  // }


}



// 好像连 Syntaxis_lexer 都不需要了。
// 这里不管 start 和 stop，需要 Token_Stream 自己处理
pub struct LexerIter<'a> {
  // 这些是对应的 Lexer 中成员的引用
  pub input: &'a str, // 输入文本 持有文本的不可变引用
  pub rules: &'a [(Regex, usize, usize, &'static str, bool)],
  pub error_listeners: &'a [Box<dyn ErrorListener>],



  // 可变内容放入 Iter
  pub cursor: usize, // 字符游标，当前处理到的文本字符序号
  pub token_index: usize, // token 序号，表示当前扫描到了第几个 token
  pub position: Position, // 当前处理到的文本字符所在的位置
}




impl LexerIter<'_> {

  // 这个函数只管匹配，匹配不上就报一个 Error。且不会识别到 start 和 stop
  fn lexer_match(&mut self) -> Result<Token, Error> {
    if self.cursor >= self.input.len() {
      return Err(Error::new(ErrorKind::LexerScanOverflow, "LexerScanOverflow", self.position, self.position));
    }
    let mut len = 0;
    let mut meta: Option<(Regex, usize, usize, &'static str, bool)> = None;

    for lexer_meta in self.rules.iter() {
      let result = lexer_meta.0.find_at(&self.input[self.cursor..], 0) ;
      if let Some(result) = result {
        if result.end() > len {
          len = result.end();
          meta = Some(lexer_meta.clone())
        }
      }
    }

    // 如果都不匹配，则报错
    if let None = meta { 
      return Err(Error::new(ErrorKind::LexerNoMatch, "", self.position, self.position)) 
    }

    // 将对应的文本找出来
    let text = String::from(&self.input[self.cursor..self.cursor+len]);
    let lines: Vec<_> = text.split("\n").collect();
    let new_pos;
    if lines.len() <= 1 {
      // 没有跨行
      new_pos = Position {
        line: self.position.line,
        char_position: self.position.char_position + len
      }
    }
    else {
      // 跨行
      new_pos = Position {
        line: self.position.line + lines.len()-1,
        char_position: lines.last().unwrap().len(),
      }
    }

    let meta = meta.unwrap();
    let token = Token::new(meta.1, meta.3, &text, 
      self.position.clone(),new_pos.clone(), self.token_index, meta.2, self.cursor, self.cursor + len);

    self.cursor += len;
    self.position = new_pos;

    // 如果需要跳过，则返回下一个
    if meta.4 {
      return self.lexer_match();
    }
    
    self.token_index += 1;
    return Ok(token);
  }

  // 这个函数用于当 lexer_match 发生错误的时候进行一些修复工作
  fn recover(&mut self) -> Result<Token, Error> {
    // 向 error_listeners 报告错误


    todo!()
  }

}


impl Iterator for LexerIter<'_> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    match self.lexer_match() {
      Ok(token) => Some(token), 
      Err(err) => match err.kind {
          ErrorKind::LexerNoMatch => match self.recover() {
            Ok(token) => Some(token),
            Err(_) => None,
          },
          _ => None,
        },
    }
  }
}



