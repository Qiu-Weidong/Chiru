use std::{fs::File, io::Write};

use chiru::{tool::{syntaxis::{syntaxis_lexer::SyntaxisLexer, syntaxis_parser::SyntaxisParser}, grammar::Grammar, visitor::grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor}, code_gen::parser_gen::parser_generate}, runtime::token_stream::TokenStream};


#[macro_use]
extern crate lazy_static;

mod generate;

#[test]
fn generate_test() {
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


  let mut file = File::create("tests/generate/parser.rs").unwrap();


  let lexer = SyntaxisLexer::new(input);

  let mut stream = TokenStream::new(&lexer, 0);

  stream.consume().unwrap(); // 注意要先将 _START 消耗掉
  let parser = SyntaxisParser::new();
  let ast = parser.rule_list(&mut stream);

  let mut grammar = Grammar::new("Chiru");
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

  file.write(parser_generate(grammar).as_bytes()).unwrap();


  
}

