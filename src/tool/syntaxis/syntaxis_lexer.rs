

use regex::Regex;

use crate::runtime::{lexer::Lexer, token::Token, position::Position};
use crate::runtime::error::{Error, ErrorKind};

pub struct SyntaxisLexer {
  pub input: String, // 输入文本 至少持有文本的引用




  pub cursor: usize, // 字符游标，当前处理到的文本字符序号
  pub token_index: usize, // token 序号，表示当前扫描到了第几个 token
  pub position: Position, // 当前处理到的文本字符所在的位置


  // todo 增添一个 scope

  // todo 增添错误处理器 或 错误监听器
}


// 词法分析的相关信息 用元组即可
#[derive(Clone)]
struct LexerMeta {
  pub rule: Regex,


  pub token_type: usize,
  pub channel: usize,
  pub name: &'static str,
  pub skip: bool,
}


// 使用模板生成正则列表和token名称列表
lazy_static!{


  static ref LEXER_META_LIST: [LexerMeta; 14] = [
    LexerMeta { rule: Regex::new(r####"^([a-z][a-zA-Z0-9_]+)"####).unwrap(), token_type:2,channel:0, skip:false,name:"RULE_REF"},
    LexerMeta { rule: Regex::new(r####"^([A-Z][a-zA-Z0-9_]+)"####).unwrap(), token_type:3,channel:0, skip:false,name:"TOKEN_REF"},
    LexerMeta { rule: Regex::new(r####"^(::=|:=|->|=>|:|=)"####).unwrap(), token_type:4,channel:0, skip:false,name:"COLON"},
    LexerMeta { rule: Regex::new(r####"^(;)"####).unwrap(), token_type:5,channel:0, skip:false,name:"SEMI"},
    LexerMeta { rule: Regex::new(r####"^(\|)"####).unwrap(), token_type:6,channel:0, skip:false,name:"OR"},
    LexerMeta { rule: Regex::new(r####"^(ε|epsilon)"####).unwrap(), token_type:7,channel:0, skip:false,name:"EPSILON"},
    LexerMeta { rule: Regex::new(r####"^(\*)"####).unwrap(), token_type:8,channel:0, skip:false,name:"STAR"},
    LexerMeta { rule: Regex::new(r####"^(\+)"####).unwrap(), token_type:9,channel:0, skip:false,name:"PLUS"},
    LexerMeta { rule: Regex::new(r####"^(\?)"####).unwrap(), token_type:10,channel:0, skip:false,name:"QUESTION"},
    LexerMeta { rule: Regex::new(r####"^(\()"####).unwrap(), token_type:11,channel:0, skip:false,name:"LPAREN"},
    LexerMeta { rule: Regex::new(r####"^(\))"####).unwrap(), token_type:12,channel:0, skip:false,name:"RPAREN"},
    LexerMeta { rule: Regex::new(r####"^('([^\a\d\n\r\t\f\v\\']|(\\\\|\\'|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*')"####).unwrap(), 
      token_type:13,channel:0, skip:false,name:"STRING_LITERAL"},
    LexerMeta { rule: Regex::new(r####"^(/(\\/|[^/])+/)"####).unwrap(), token_type:14,channel:0, skip:false,name:"REGULAR_LITERAL"},
    LexerMeta { rule: Regex::new(r####"^([ \r\n\t\f]+)"####).unwrap(), token_type:15,channel:0, skip:true,name:"WHITE_SPACE"},
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

    SyntaxisLexer { input: input.to_owned(), cursor: 0, token_index: 1, position: pos.clone(),}
  }




  // 定义一些私有函数
}

impl Lexer for SyntaxisLexer {
  // todo match 和 recover

  fn scan(&mut self) -> Result<Token, Error> {
    // if self.cursor > self.input.len() { return Err(Error::new(kind, msg, start, stop)) }

    if self.cursor >= self.input.len() {
      self.cursor += 10;

      // 返回结束 token _stop stop 的 channel 直接设为 0 即可
      // return Ok(Token::new(SyntaxisLexer::_STOP, "_STOP", "_STOP",         
      //   self.position.clone(), self.position.clone(), self.token_index, 0, 
      //   self.cursor, self.cursor))


      // 返回一个 stop 错误
      return Err(Error::new(ErrorKind::LexerEof,  "", self.position, self.position))
    }


    let mut len = 0;
    let mut meta: Option<LexerMeta> = None;

    for lexer_meta in LEXER_META_LIST.iter() {
      let result = lexer_meta.rule.find_at(&self.input[self.cursor..], 0) ;
      if let Some(result) = result {
        if result.end() > len {
          len = result.end();
          meta = Some(lexer_meta.clone())
        }
      }
    }

    // 如果都不匹配，则报错
    if let None = meta { return Err(Error::new(ErrorKind::LexerNoMatch, "", self.position, self.position)) }

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
    let token = Token::new(meta.token_type, meta.name,&text, 
      self.position.clone(),new_pos.clone(), self.token_index, meta.channel, self.cursor, self.cursor + len);

    self.cursor += len;
    // self.token_index += 1;
    self.position = new_pos;

    // 如果需要跳过，则返回下一个
    if meta.skip {
      return self.scan();
    }
    
    self.token_index += 1;
    return Ok(token);

  }

  fn reset(&mut self) {
    self.cursor = 0;
    self.token_index = 0;
    self.position = Position { line: 0, char_position: 0 };
  }


}

