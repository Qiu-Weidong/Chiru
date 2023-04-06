use syntaxis::{runtime::lexer::Lexer, tool::{parser::ll1_parser::LL1Parser, gui::ast_drawer::ASTDrawer}};
mod lexer_generate;
mod ast_loader;

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

#[test]
fn parser_test() {
  let input = r####"
  rule_list: (parser_rule | lexer_rule)*;

  parser_rule: RULE_REF COLON block SEMI;
  block: alternative (OR alternative)*;

  alternative: element+ | epsilon;
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


  let mut lexer = lexer_generate::lexer::SyntaxisLexer::new(input);
  let tokens = lexer.scan_all_on_channel_tokens(0);

  let (grammar, _) = ast_loader::load_ast();

  let (first, first_set) = grammar.first_set();

  let follow = grammar.follow_set(&first);

  let table = grammar.ll1_table(&first_set, &follow);

  let mut parser = LL1Parser::new(tokens, table, grammar);
  let ast = parser.parse();


  // 根据产生式构造 ast
  ASTDrawer::new().draw(&ast, "parser", "parser.html");
}


