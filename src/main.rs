use syntaxis::{tool::syntaxis::syntaxis_lexer::SyntaxisLexer, runtime::lexer::Lexer};



/**
 * syntaxis <grammar-filename> [-o 输出目录] [-encoding 编码] [-listener] [-visitor] [-package 模块名称] [-language 目标语言]
 *
 * syntaxis <grammar-filename> [-tokens] [-tree] [-gui] [-encoding 编码]
 */

fn main() {
  let input = r##"
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
  STAR: /\*/;
  PLUS: /\+/;
  QUESTION: /\?/;
  LPAREN: /\(/;
  RPAREN: /\)/;
  STRING_LITERAL: /"([^\a\d\n\r\t\f\v\\"]|(\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*"/;
  REGULAR_LITERAL: /\/([^/]|\\\/)+\//;
  "##;
  
  let mut lexer = SyntaxisLexer::new(input);
  
  while let Ok(token) = lexer.scan() {
    if token.token_type == 4 { continue; }
    println!("{}", token)
  }

  // 出错了 不断将 cursor + 1 , 直到不再出错为止。
}



