pub mod tool;


use chiru::runtime::token_stream::TokenStream;
use tool::{syntaxis::{chiru_lexer::ChiruLexer, chiru_parser::ChiruParser}, grammar::Grammar, code_generator::CodeGenerator};


#[allow(unused_doc_comments)]
fn main() {

  let input = r######"
  /****
   * 注释
   * 多行注释
   * 看看是否会有问题
   */
  grammar Chiru;
  compilation_unit: grammar_name rules;
  grammar_name: GRAMMAR (TOKEN_REF | RULE_REF) SEMI;
  rules: (parser_rule | lexer_rule)*;

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

  // 这是一行注释
  ebnf_suffix: (STAR | PLUS | QUESTION) QUESTION?;


  lexer_rule: annotation ? TOKEN_REF COLON regular SEMI;
  regular: REGULAR_LITERAL;
  annotation: AT attribute
    | SHARP LBRACKET attributes RBRACKET
  ;
  attributes: attribute (COMMA attribute)* ;
  attribute: RULE_REF ( LPAREN TOKEN_REF RPAREN )? ;

  GRAMMAR: r###"grammar"###;
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

  @ignore
  WHITE_SPACE: r###"[ \r\n\t\f]+"###;

  @channel(HIDDEN)
  LINE_COMMENT: r###"//.*?\n"###;
  @channel(HIDDEN)
  BLOCK_COMMENT: r###"(?s)/\*.*?\*\/"###;
  
  "######;

  let lexer = ChiruLexer::new(input);
  let mut stream = TokenStream::new(&lexer, 0);

  stream.consume().unwrap(); // 注意要先将 _START 消耗掉
  let parser = ChiruParser::new();
  let ast = parser.compilation_unit(&mut stream);


  // 从语法树解析出 grammar
  let grammar = Grammar::from_ast(ast.as_ref());






  // let base_dir = "src/tool/syntaxis";
  let base_dir = "tests/generate";



  let code_generator = CodeGenerator::new(grammar, ast.as_ref());
  code_generator.generate(base_dir);

}


