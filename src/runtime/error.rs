

use std::{error, fmt::Display};

use super::location::Location;


#[derive(Debug, Clone)]
pub enum Error {


  LexerScanOverflow, // 当词法分析器扫描到输入结束的时候，会产生该错误
  LexerNoMatch(Location), // 所有 token 都匹配不上
  LexerRecoverFail, // 尝试修复但还是失败


  TokenStreamOutOfRange, // look_ahead 超出了范围

  Unknown, // 未知错误
}

impl Error {
  pub fn token_stream_out_of_range() -> Self {
    Self::TokenStreamOutOfRange
  }

  pub fn lexer_no_match(location: Location) -> Self {
    Self::LexerNoMatch(location)
  }

  pub fn lexer_scan_overflow() -> Self {
    Self::LexerScanOverflow
  }

}




// #[derive(Debug, Clone)]
// pub enum ErrorKind {
//   // 定义一些错误类型

//   // 词法分析过程中产生的错误
//   LexerScanOverflow, // 当词法分析器扫描到输入结束的时候，会产生该错误
//   LexerNoMatch, // 所有 token 都匹配不上
//   LexerRecoverFail, // 尝试修复但还是失败


//   TokenStreamOutOfRange, // look_ahead 超出了范围
//   ConsumedTokenExhausted, // look_back 查看的缓存耗尽

//   Unknown, // 未知错误
// }

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
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}




