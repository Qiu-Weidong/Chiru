use chiru::runtime::lexer::Lexer;
mod lexer_generate;

#[test]

fn lexer_test() {
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


  let mut lexer = lexer_generate::lexer::chiruLexer::new(input);
  let tokens = lexer.scan_all_on_channel_tokens(0);

  
  for token in tokens {
    println!("{}, {}", token.text, token.token_name);
  }


}
