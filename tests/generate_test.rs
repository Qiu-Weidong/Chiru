use chiru::{runtime::token_stream::TokenStream, tool::gui::ast_drawer::ASTDrawer};
use generate::parser::ChiruParser;




#[macro_use]
extern crate lazy_static;

mod generate;

#[test]
fn generate_test() {
  let input = r######"
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


  lexer_rule: annotation ? TOKEN_REF COLON regular SEMI;
  regular: REGULAR_LITERAL;
  annotation: AT attribute
    | SHARP LBRACKET attribute_list RBRACKET
  ;
  attribute_list: attribute (COMMA attribute)* COMMA? ;
  attribute: (TOKEN_REF|RULE_REF) ( LPAREN (TOKEN_REF | RULE_REF) RPAREN )? ;

  RULE_REF: r###"[a-z][a-zA-Z0-9_]*"###;
  TOKEN_REF: r###"[A-Z][a-zA-Z0-9_]*"###;
  COLON: r###"::=|:=|->|=>|:|="###;
  SEMI: r###";"###;
  COMMA: r###","###;
  OR: r###"\|"###;
  EPSILON: r###"ε|epsilon"###;
  STAR: r###"\*"###;
  PLUS: r###"\+"###;
  QUESTION: r###"\?"###;
  LPAREN: r###"\("###;
  RPAREN: r###"\)"###;
  AT: r###"@"###;
  SHARP: r###"#"###;
  LBRACKET: r###"\["###;
  RBRACKET: r###"\]"###;
  STRING_LITERAL: r###""((\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d|[^\a\d\n\r\t\f\v\\"])*""###;
  REGULAR_LITERAL: r###"(?s)r##".*?"##"###;

  @skip
  WHITE_SPACE: r###"[ \r\n\t\f]+"###;

  @channel(HIDDEN)
  LINE_COMMENT: r###"//.*?\n"###;
  @channel(HIDDEN)
  BLOCK_COMMENT: r###"/\*.*?\*\/"###;
  
  "######;

  // 测试一下生成的 lexer 和 parser 是否可以使用


  // 自动生成的 lexer 暂时还需要手动修改一下 regular 的正则和 whitespace 的 skip
  let lexer = generate::lexer::ChiruLexer::new(input);

  let mut token_stream = TokenStream::new(&lexer, 0);


  let parser = ChiruParser::new();
  token_stream.consume().unwrap();
  let ast = parser.rule_list(&mut token_stream);


  print!("{}", ast.as_rule().rule_name);

  ASTDrawer::new().draw(ast.as_ref().as_rule(), "foo", "output/foo.html");


  
}

