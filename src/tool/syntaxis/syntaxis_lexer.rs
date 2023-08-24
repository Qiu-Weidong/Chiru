

use regex::Regex;

use crate::runtime::error_strategy::error_listener::{ErrorListener, ConsoleErrorListener};
use crate::runtime::{lexer::Lexer, token::Token, position::Position};
use crate::runtime::error::{Error, ErrorKind};

pub struct SyntaxisLexer {
  pub input: String, // 输入文本 至少持有文本的引用




  pub cursor: usize, // 字符游标，当前处理到的文本字符序号
  pub token_index: usize, // token 序号，表示当前扫描到了第几个 token
  pub position: Position, // 当前处理到的文本字符所在的位置


  // todo 增添一个 scope

  // todo 增添错误处理器 或 错误监听器

  // 错误监听器列表
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


// 使用模板生成正则列表和token名称列表
lazy_static!{


  static ref LEXER_META_LIST: [(Regex, usize, usize, &'static str, bool); 14] = [
     (Regex::new(r####"^([a-z][a-zA-Z0-9_]+)"####).unwrap(), 2,0, "RULE_REF", false),
     (Regex::new(r####"^([A-Z][a-zA-Z0-9_]+)"####).unwrap(), 3,0, "TOKEN_REF", false),
     (Regex::new(r####"^(::=|:=|->|=>|:|=)"####).unwrap(), 4,0, "COLON", false),
     (Regex::new(r####"^(;)"####).unwrap(), 5,0, "SEMI", false),
     (Regex::new(r####"^(\|)"####).unwrap(), 6,0, "OR", false),
     (Regex::new(r####"^(ε|epsilon)"####).unwrap(), 7,0, "EPSILON", false),
     (Regex::new(r####"^(\*)"####).unwrap(), 8,0, "STAR", false),
     (Regex::new(r####"^(\+)"####).unwrap(), 9,0, "PLUS", false),
     (Regex::new(r####"^(\?)"####).unwrap(), 10,0, "QUESTION", false),
     (Regex::new(r####"^(\()"####).unwrap(), 11,0, "LPAREN", false),
     (Regex::new(r####"^(\))"####).unwrap(), 12,0, "RPAREN", false),
     (Regex::new(r####"^('([^\a\d\n\r\t\f\v\\']|(\\\\|\\'|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*')"####).unwrap(), 
      13,0, "STRING_LITERAL", false),
     (Regex::new(r####"^(/(\\/|[^/])+/)"####).unwrap(), 14,0, "REGULAR_LITERAL", false),
     (Regex::new(r####"^([ \r\n\t\f]+)"####).unwrap(), 15,0, "WHITE_SPACE", true),
  ];
}


impl SyntaxisLexer {
  // 前两个是开始符号和结束符号
  pub const _START: usize = 0;
  pub const _STOP: usize = 1;

  // 从这里开始使用模板
  
  pub const RULE_REF: usize = 2;
  pub const TOKEN_REF: usize = 3;
  pub const COLON: usize = 4;
  pub const SEMI: usize = 5;
  pub const OR: usize = 6;
  pub const EPSILON: usize = 7;
  pub const STAR: usize = 8;
  pub const PLUS: usize = 9;
  pub const QUESTION: usize = 10;
  pub const LPAREN: usize = 11;
  pub const RPAREN: usize = 12;
  pub const STRING_LITERAL: usize = 13;
  pub const REGULAR_LITERAL: usize = 14;
  pub const WHITE_SPACE: usize = 15;

  


  pub fn new(input: &str) -> Self {
    let pos = Position { line: 0, char_position: 0 };

    SyntaxisLexer { input: input.to_owned(), 
      // 默认情况下，添加一个 ConsoleErrorListener
      error_listeners: vec![Box::new(ConsoleErrorListener::new())],
      cursor: 0, token_index: 1, position: pos.clone(),}
  }

  // 考虑是否放入 trait 中
  pub fn remove_all_error_listeners(&mut self) {
    self.error_listeners.clear()
  }

  pub fn add_error_listener(&mut self, listener: Box<dyn ErrorListener>) {
    self.error_listeners.push(listener)
  }


  // 定义一些私有函数
}

impl Lexer for SyntaxisLexer {

  fn reset(&mut self) {
    self.cursor = 0;
    self.token_index = 0;
    self.position = Position { line: 0, char_position: 0 };
  }

  // 实际的匹配函数
  fn lexer_match(&mut self) -> Result<Token, Error> {
    
    if self.cursor >= self.input.len() {
      return Ok(Token::new(SyntaxisLexer::_STOP, "_STOP", "_STOP", 
        self.position, self.position, self.token_index, 
        0, self.cursor, self.cursor));
    }


    let mut len = 0;
    let mut meta: Option<(Regex, usize, usize, &'static str, bool)> = None;

    for lexer_meta in LEXER_META_LIST.iter() {
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

  fn recover(&mut self) -> Result<Token, Error> {
    // 向 error_listeners 报告错误


    todo!()
  }



}

