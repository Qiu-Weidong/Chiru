
// mod lexer_generate;
// mod ast_loader;

/*
  0: "_START", 
  1: "_STOP",
  2: "RULE_REF", 
  3: "TOKEN_REF", 
  4: "COLON", 
  5: "SEMI", 
  6: "OR", 
  7: "EPSILON", 
  8: "STAR", 
  9: "PLUS", 
  10: "QUESTION", 
  11: "LPAREN", 
  12: "RPAREN", 
  13: "STRING_LITERAL",
  14: "REGULAR_LITERAL", 
  15: "WHITE_SPACE", 
  
 */

use std::fs::File;

use regex::Regex;
use syntaxis::{tool::{parser::ll1_parser::LL1Parser, visitor::{grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor}, lexer_rule_visitor::LexerRuleData}, grammar::Grammar, serde_ast, syntaxis::syntaxis_context::RuleListContext, gui::ast_drawer::ASTDrawer}, runtime::{token::{Token, Position}, lexer::{Lexer, Error}}};


fn main() {
  let input = r####"
  WHITE_SPACE: /[ \r\n\t\f]+/;
  
  "####;
/*
rule_list

TOKEN_REF COLON REGULAR_LITERAL SEMI


 */

  let mut lexer = SyntaxisLexer::new(input);
  let tokens = lexer.scan_all_on_channel_tokens(0);

  let (grammar, _) = load_ast();

  let (first, first_set) = grammar.first_set();

  let follow = grammar.follow_set(&first);

  let table = grammar.ll1_table(&first_set, &follow);

  let mut parser = LL1Parser::new(tokens, table, grammar);
  let ast = parser.parse();


  // 根据产生式构造 ast
  ASTDrawer::new().draw(&ast, "parser", "parser.html");
}


pub fn load_ast() -> (Grammar, Vec<LexerRuleData>) {
  let file = File::open("src/tool/syntaxis/syntaxis2.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();

  let mut grammar = Grammar::new("我的文法");
  let token_cnt;
  let data;
  {
    let mut visitor = StringLiteralToTokenVisitor::new(
      &mut grammar, 2
    );

    ast.accept(&mut visitor);
    token_cnt = visitor.next_token_id;
    data = visitor.data;
  }
  
  let rule_cnt; 
  let lexer_data;
  {
    let mut visitor = SymbolVisitor::new(&mut grammar, token_cnt, 0, data);
    ast.accept(&mut visitor);
    rule_cnt = visitor.next_rule_id;
    lexer_data = visitor.data;
  }

  {
    let mut visitor = ProductionVisitor::new(&mut grammar, rule_cnt);
    ast.accept(&mut visitor);
  }

  (grammar, lexer_data)

}


pub struct SyntaxisLexer {
  pub input: String,
  pub cursor: usize, // 每次都匹配 input[cursor..]
  pub regex_list: Vec<regex::Regex>,
  pub token_names: Vec<&'static str>,
  pub token_index: usize,
  pub position: Position,
}

#[allow(dead_code)]
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
    let regex_list = vec![

      // 这里也需要使用模板
      
      Regex::new(r####"^([a-z][a-zA-Z0-9_]+)"####).unwrap(), // RULE_REF
      Regex::new(r####"^([A-Z][a-zA-Z0-9_]+)"####).unwrap(), // TOKEN_REF
      Regex::new(r####"^(::=|:=|->|=>|:|=)"####).unwrap(), // COLON
      Regex::new(r####"^(;)"####).unwrap(), // SEMI
      Regex::new(r####"^(\|)"####).unwrap(), // OR
      Regex::new(r####"^(ε|epsilon)"####).unwrap(), // EPSILON
      Regex::new(r####"^(\*)"####).unwrap(), // STAR
      Regex::new(r####"^(\+)"####).unwrap(), // PLUS
      Regex::new(r####"^(\?)"####).unwrap(), // QUESTION
      Regex::new(r####"^(\()"####).unwrap(), // LPAREN
      Regex::new(r####"^(\))"####).unwrap(), // RPAREN
      Regex::new(r####"^('([^\a\d\n\r\t\f\v\\']|(\\\\|\\'|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*')"####).unwrap(), // STRING_LITERAL
      Regex::new(r####"^(/(\\/|[^/])+/)"####).unwrap(), // REGULAR_LITERAL
      Regex::new(r####"^([ \r\n\t\f]+)"####).unwrap(), // WHITE_SPACE
    
    ];
    
    // 同样使用模板
    let token_names = vec![
      "_START", "_STOP", 
      "RULE_REF", "TOKEN_REF", "COLON", "SEMI", "OR", "EPSILON", "STAR", "PLUS", "QUESTION", "LPAREN", "RPAREN", "STRING_LITERAL", "REGULAR_LITERAL", "WHITE_SPACE", 
    ];

    SyntaxisLexer { input: input.to_owned(), cursor: 0, regex_list, token_names, token_index: 0, position: Position { line: 0, char_position: 0 } }
  }



}

// pub struct Error;
impl Lexer for SyntaxisLexer {


  fn scan(&mut self) -> Result<Token, syntaxis::runtime::lexer::Error> {
    if self.cursor > self.input.len() { return Err(Error {}) }
    else if self.cursor >= self.input.len() {
      self.cursor += 10;
      return Ok(Token {
        token_type: 1,
        token_name: "_STOP".to_owned(),
  
        start: self.position.clone(),
        stop: self.position.clone(),
        
        channel: 0,
        text: "_STOP".to_owned(),
        token_index: self.token_index,
        char_start_index: self.cursor,
        char_stop_index: self.cursor,
      });
    }
    let mut len = 0;
    let mut token_type = 0;

    for i in 0..self.regex_list.len() {
      let result = self.regex_list[i].find_at(&self.input[self.cursor..], 0) ;
      if let Some(result) = result {
        if result.end() > len {
          len = result.end();
          token_type = i+2;
        }
      }
    }

    if token_type <= 0 { return Err(Error {}) }
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



    let token = Token {
      token_type,
      token_name: String::from(self.token_names[token_type]),

      start: self.position.clone(),
      stop: new_pos.clone(),
      
      channel: 0,
      text,
      token_index: self.token_index,
      char_start_index: self.cursor,
      char_stop_index: self.cursor + len,
    };

    self.cursor += len;
    self.token_index += 1;
    self.position = new_pos;
    return Ok(token);

  }

  fn scan_all_on_channel_tokens(&mut self, channel: usize) -> Vec<Token> {
    let mut result = Vec::new();
    while let Ok(token) = self.scan() {
      if token.token_type == SyntaxisLexer::WHITE_SPACE { continue; }
      if token.channel == channel {
        result.push(token);
      }
    }

    if self.cursor < self.input.len() {
      println!("{}", &self.input[self.cursor..]);
    }
    
    result
  }
}

