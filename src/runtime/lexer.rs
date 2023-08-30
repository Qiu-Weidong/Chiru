


use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;

use regex::Regex;

use super::error::{Error, ErrorKind};

use super::error_strategy::error_listener::ErrorListener;
use super::position::Position;
use super::token::Token;


// lexer 都不识别 start 和 stop，所有 start 和 stop 都交给 tokenstream 来添加
pub trait Lexer {
  fn iter(&self) -> TokenIter ;


  fn get_all_on_channel_tokens(&self, channel: usize) -> Vec<Token> {
    self.iter().filter(|token| token.channel == channel).collect::<Vec<_>>()
  }

  fn get_all_tokens(&self) -> Vec<Token> {
    self.iter().collect::<Vec<_>>()
  }

  fn scan_all_tokens_and_group_by_channel(&mut self) -> HashMap<usize, Vec<Token>> {
    let mut ret: HashMap<usize, Vec<Token>> = HashMap::new();
    for token in self.iter() {
      if ret.contains_key(&token.channel) {
        ret.get_mut(&token.channel).unwrap().push(token);
      } else {
        ret.insert(token.channel, vec![token]);
      }
    }
    ret
  }


}


// 这里不管 start 和 stop，需要 Token_Stream 自己处理
pub struct TokenIter<'a> {
  // 这些是对应的 Lexer 中成员的引用
  pub input: &'a str, // 输入文本 持有文本的不可变引用
  pub  rules: &'a [(Regex, usize, usize, &'static str, bool)],
  pub error_listeners: &'a [Box<dyn ErrorListener>],

  // 存储一个每行的字符下标范围
  pub ranges: Vec<Range<usize>>,



  // 可变内容放入 Iter
  pub cursor: usize, // 字符游标，当前处理到的文本字符序号
  pub token_index: usize, // token 序号，表示当前扫描到了第几个 token
  // pub position: Position, // 当前处理到的文本字符所在的位置
}




impl<'a> TokenIter<'a> {


  fn get_position_from_char_index(&self, char_index: usize) -> Position {
    let line = self.ranges.binary_search_by(|range| {
      if range.end <= char_index {
        Ordering::Less
      } else if range.start > char_index {
        Ordering::Greater
      } else {
        Ordering::Equal
      }
    }).unwrap();
    let range = self.ranges[line].clone();

    Position::new(line, char_index - range.start)
  }

  // 这个函数只管匹配，匹配不上就报一个 Error。且不会识别到 start 和 stop
  pub fn lexer_match(&mut self) -> Result<Token, Error> {
    if self.cursor >= self.input.len() {
      return Err(Error::new(ErrorKind::LexerScanOverflow, "LexerScanOverflow", 
      self.get_position_from_char_index(self.cursor), 
      self.get_position_from_char_index(self.cursor)));
    }
    
    // 找到 start 最小的 len 最长的匹配
    let mut len = 0;
    let mut start = self.input.len();
    let mut stop = start;

    let mut meta: Option<(Regex, usize, usize, &'static str, bool)> = None;

    for lexer_meta in self.rules.iter() {
      // 为提高效率，可以检查是否匹配
      if ! lexer_meta.0.is_match_at(self.input, self.cursor) { continue; }

      let result = lexer_meta.0.find_at(self.input, self.cursor) ;
      if let Some(result) = result {
        if result.start() < start || result.start() == start && result.end() - result.start() > len {
          meta = Some(lexer_meta.clone());
          start = result.start();
          stop = result.end();
          len = result.end() - result.start();
        }
      }
    }

    // 如果都不匹配，则报错
    if let None = meta { 
      return Err(Error::new(ErrorKind::LexerNoMatch, "", 
      self.get_position_from_char_index(self.cursor), 
      self.get_position_from_char_index(self.cursor))); 
    }


    // 如果不是在当前位置匹配，那么报告给错误监听器
    if start != self.cursor {
      todo!()
    }

    // 将对应的文本找出来
    let text = String::from(&self.input[start..stop]);



    let meta = meta.unwrap();
    let token = Token::new(meta.1, meta.3, &text, 
      self.get_position_from_char_index(start),
      self.get_position_from_char_index(stop), self.token_index, 
      meta.2, 
      self.cursor, 
      self.cursor + len);

    self.cursor = stop;


    // 如果需要跳过，则返回下一个
    if meta.4 {
      return self.lexer_match();
    }
    
    self.token_index += 1;
    return Ok(token);
  }


  pub fn reset(&mut self) {
    self.cursor = 0;
    self.token_index = 1;

  }




  pub fn new(input: &'a str, rules: &'a [(Regex, usize, usize, &'static str, bool)], error_listeners: &'a [Box<dyn ErrorListener>]) -> Self {
    let mut st = 0;
    let ranges = input.split("\n").map(|f| { 
      let ed = st + f.len() + 1; // +1 是为了补上 \n 换行符
      let ret = st..ed;
      st = ed; 
      ret
    }).collect::<Vec<_>>();
    
    Self {
      input, rules, error_listeners, cursor: 0, token_index: 1, 
      ranges,
    }
  }
}


impl Iterator for TokenIter<'_> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    match self.lexer_match() {
      Ok(token) => Some(token), 
      Err(_) => None,
    }
  }
}

