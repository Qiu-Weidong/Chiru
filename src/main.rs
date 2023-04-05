

use syntaxis::runtime::lexer::Lexer;
use syntaxis::tool::syntaxis::syntaxis_lexer::SyntaxisLexer;
// use syntaxis::tool::syntaxis::syntaxis_context::AlternativeContext;
// use syntaxis::tool::grammar;





/**
 * syntaxis <grammar-filename> [-o 输出目录] [-encoding 编码] [-listener] [-visitor] [-package 模块名称] [-language 目标语言]
 *
 * syntaxis <grammar-filename> [-tokens] [-tree] [-gui] [-encoding 编码]
 */

 fn main() {
  let input = r####"/[a-z][a-zA-Z0-9_]+/;
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
  STRING_LITERAL: /"([^\a\d\n\r\t\f\v\\"]|(\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*"/;
  REGULAR_LITERAL: /\/(\\\/|[^\/])+\//;
  
  "####;


  let mut lexer = SyntaxisLexer::new(input);
  let tokens = lexer.scan_all_on_channel_tokens(0);

  
  for token in tokens {
    println!("{}, {}", token.text, token.token_name);
  }
}


