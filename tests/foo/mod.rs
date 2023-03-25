pub mod foo_parser;
pub mod foo_listener;
pub mod foo_visitor;
pub mod foo_context;
pub mod foo_walker;

use std::rc::Rc;

use syntaxis::{runtime::{ast::{ast_context::ASTContext, rule_context::RuleContext, terminal_context::TerminalContext}, token::{Position, Token}}};

use crate::foo::foo_parser::FooParser;

pub fn create_ast() -> RuleContext {
  let lb = Token {
    token_type: 3,
      token_name: String::from("'('"),
      start: Position { line: 0, char_position: 0 },
      stop: Position { line: 0, char_position: 1 },
      channel: 0,
      text: String::from("\'(\'"),
      token_index: 0,
      char_start_index: 0,
      char_stop_index: 1,
  };

  let number = Token {
    token_type: 1,
    token_name: String::from("Number"),
    start: Position { line: 0, char_position: 2 },
    stop: Position { line: 0, char_position: 5 },
    channel: 0,
    text: String::from("234"),
    token_index: 1,
    char_start_index: 2,
    char_stop_index: 5,
  };

  let rb = Token {
    token_type: 4,
    token_name: String::from("')'"),
    start: Position { line: 0, char_position: 0 },
    stop: Position { line: 0, char_position: 1 },
    channel: 0,
    text: String::from("\')\'"),
    token_index: 0,
    char_start_index: 0,
    char_stop_index: 1,
  } ;
  let semicolone = Token {
    token_type: 5,
    token_name: String::from("';'"),
    start: Position { line: 0, char_position: 0 },
    stop: Position { line: 0, char_position: 1 },
    channel: 0,
    text: String::from("\';\'"),
    token_index: 0,
    char_start_index: 0,
    char_stop_index: 1,
  } ;


  let lbcontext = TerminalContext { 
    symbol: lb.clone()
  };


  let n1 = TerminalContext { 
    symbol: number.clone()
  };

  let rbcontext = TerminalContext { 
    symbol: rb.clone()
  };
  let semicontext = TerminalContext { symbol: semicolone.clone() };

  // 构造 expr
  let children = vec![
    ASTContext::Ternimal(Rc::new(lbcontext)),
    ASTContext::Ternimal(Rc::new(n1)),
    ASTContext::Ternimal(Rc::new(rbcontext)),
  ];

  let expr = RuleContext {
    rule_index: FooParser::RULE_EXPR,
    rule_name: String::from("expr"),
    children
  };

  let children = vec![
    ASTContext::Rule(Rc::new(expr)),
    ASTContext::Ternimal(Rc::new(semicontext)),
  ];

  let stmt = RuleContext {
    rule_index: 2, 
    rule_name: String::from("stmt"),
    children
  };

  let children = vec![ASTContext::Rule(Rc::new(stmt))];
  let start = RuleContext {
    rule_index: 1, 
    rule_name: String::from("start"), 
    children
  };

  start
}
