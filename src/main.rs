

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

use std::error::Error;
use std::{fs::File, io::Read};
// use std::io::Result;

use chiru::{tool::{visitor::{grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor}, lexer_rule_visitor::LexerRuleData}, 
grammar::Grammar, serde_ast, syntaxis::{syntaxis_context::RuleListContext, syntaxis_parser::SyntaxisParser}}, runtime::{token_stream::TokenStream, parser::Parser}};




use chiru::tool::syntaxis::syntaxis_lexer::SyntaxisLexer;
// use serde::de::Error;

fn main() {
  let (grammar, _) = load_ast();

  
  let (first, first_set) = grammar.first_set();

  let follow = grammar.follow_set(&first);

  let table = grammar.ll1_table(&first_set, &follow);



  // println!("{}", grammar);



  let input = r####"
  rule_list: (parser_rule | lexer_rule)*;

  parser_rule: RULE_REF COLON block SEMI;
  block: alternative (OR alternative)*;

  alternative: element element* | epsilon;
  epsilon: EPSILON;
  element: (
      TOKEN_REF
      | STRING_LITERAL
      | RULE_REF
      | LPAREN block RPAREN
    ) ebnf_suffix?;

  ebnf_suffix: (STAR | PLUS | QUESTION) QUESTION?;


  lexer_rule: TOKEN_REF COLON regular SEMI;
  regular: REGULAR_LITERAL;

  RULE_REF: /[a-z][a-zA-Z0-9_]+/;
  TOKEN_REF: /[A-Z][a-zA-Z0-9_]+/;
  COLON: /::=|:=|->|=>|:|=/;
  SEMI: /;/;
  OR: /\|/;
  EPSILON: /ε|epsilon/;
  STAR: /\* /;
  PLUS: /\+/;
  QUESTION: /\?/;
  LPAREN: /\(/;
  RPAREN: /\)/;
  STRING_LITERAL: /"((\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d|[^\a\d\n\r\t\f\v\\"])*"/;
  REGULAR_LITERAL: /\/(\\\/|[^\/])+\//;
  WHITE_SPACE: /[ \r\n\t\f]+/;
  
  "####;

  let mut lexer = SyntaxisLexer::new(input);


  let mut stream = TokenStream::new(&mut lexer, 0);
  let parser = SyntaxisParser {};
  let ast = parser.parse(&mut stream);


  // for token in stream.iter() {

  // }

  // let parser = SyntaxisParser::new(tokens, table, grammar);
  // let ast = parser.parse();

  // 我现在可以生成 lexer 和 visitor 了。


  // 根据产生式构造 ast
  // let file = File::open("src/tool/syntaxis/syntaxis.json").unwrap();
  // let ast = serde_ast::from_reader(file).unwrap();
  // ASTDrawer::new().draw(&ast, "parser", "output/foo2.html");


  print!("done")
}


pub fn load_ast() -> (Grammar, Vec<LexerRuleData>) {
  // let file = File::open(path);
  let file = File::open("src/tool/syntaxis/syntaxis2.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();


  // file.read_to_string(buf)


  // ASTDrawer::new().draw(&ast, "parser", "ast.html");


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





pub fn foo() -> Result<(), Box<dyn Error>>{
  let x = 0..5;
  
  let file = File::open("")?;
  Ok(())
}
