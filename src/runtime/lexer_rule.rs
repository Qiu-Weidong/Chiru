
use regex::Regex;

// 这个是给 runtime 用的
#[derive(Debug, Clone)]
pub struct LexerRule {
  pub token_type: usize,
  pub token_name: String,
  pub rule: Regex,

  pub channel: usize,
  pub skip: bool,
}


