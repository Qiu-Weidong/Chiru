// 专门用来确认语法特性
use regex::Regex;

#[test]
fn rust_test() {
  let re = Regex::new(r##"^/([^/]|\\/)+/"##).unwrap();

  let ma = re.find_at(r####"/[a-z][a-zA-Z0-9_]+/;
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
  REGULAR_LITERAL: /\/([^\/]|\\\/)+\//;"####, 0);
  
  let ma = ma.unwrap();
  println!("{:?}", ma);
}




