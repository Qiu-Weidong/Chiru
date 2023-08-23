

use std::{error, fmt::Display};

use super::position::Position;


#[derive(Debug, Clone)]
pub struct Error {
  


  pub kind: ErrorKind,

  pub msg: String,

  pub location: Option<(Position, Position)>,
}



impl Error {
  pub fn new(kind: ErrorKind, msg: &str, start: Position, stop: Position) -> Self {
    Self { kind, msg: msg.to_owned(), location: Some((start, stop)) }
  }

  pub fn start_position(&self) -> Option<Position> {  
    match self.location {
      Some(pos) => Some(pos.0),
      _ => None,
    }
  }

  pub fn stop_position(&self) -> Option<Position> {
    match self.location {
      Some((st, _)) => Some(st),
      None => None,
    }
  }

  pub fn create_error_without_location(kind: ErrorKind, msg: &str) -> Self {
    Self { kind, msg: msg.to_owned(), location:None }
  }
}



#[derive(Debug, Clone)]
pub enum ErrorKind {
  // 定义一些错误类型

  // 词法分析过程中产生的错误
  LexerEof, // 当词法分析器扫描到输入结束的时候，会产生该错误
  LexerNoMatch, // 所有 token 都匹配不上


  TokenStreamOutOfRange, // look_ahead 超出了范围
  ConsumedTokenExhausted, // look_back 查看的缓存耗尽

  Unknown, // 未知错误
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }

  fn description(&self) -> &str {
    "description() is deprecated; use Display"
  }

  fn cause(&self) -> Option<&dyn error::Error> {
    self.source()
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}



