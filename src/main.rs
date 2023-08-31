

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


use std::{fs::File, io::Write};

use chiru::{tool::{visitor::{string_literal_to_token_visitor::StringLiteralToTokenVisitor, lexer_rule_visitor::LexerRuleVisitor, parser_rule_visitor::ParserRuleVisitor, grammar_visitor::GrammarVisitor}, 
syntaxis::{syntaxis_parser::SyntaxisParser, syntaxis_lexer::SyntaxisLexer}, 
code_gen::{visitor_gen::generate_visitor, listener_gen::listener_generate, parser_gen::parser_generate, lexer_gen::lexer_generate, context_gen::context_generate}, gui::ast_drawer::ASTDrawer}, 
runtime::token_stream::TokenStream};



#[allow(unused_doc_comments)]
fn main() {

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

  let lexer = SyntaxisLexer::new(input);

  // for token in lexer.iter() {
  //   println!("{}", token);
  // }

  let mut stream = TokenStream::new(&lexer, 0);

  stream.consume().unwrap(); // 注意要先将 _START 消耗掉
  let parser = SyntaxisParser::new();
  let ast = parser.rule_list(&mut stream);

  // 打印一下语法树
  ASTDrawer::new().draw(ast.as_rule(), "foo", "output/foo.html");


  // 从语法树解析出 grammar


  let mut visitor = StringLiteralToTokenVisitor::new(2);
  ast.accept(&mut visitor);
  
  let mut lexer_visitor = LexerRuleVisitor::new(visitor.next_token_id, visitor.lexer_rule_map);
  ast.accept(&mut lexer_visitor);

  let mut parser_visitor = ParserRuleVisitor::new();
  ast.accept(&mut parser_visitor);

  let mut grammar_visitor = GrammarVisitor::new("chiru", &parser_visitor.parser_rule_map, &lexer_visitor.lexer_rule_map);
  ast.accept(&mut grammar_visitor);

  let grammar = grammar_visitor.grammar;

  // println!("{}", grammar);

  // 根据产生式构造 ast

  // 生成 visitor 暂不写入文件
  let mut file = File::create("tests/generate/visitor.rs").unwrap();
  file.write(generate_visitor(&grammar).as_bytes()).unwrap();

  // 生成 listener
  let mut file = File::create("tests/generate/listener.rs").unwrap();
  file.write(listener_generate(&grammar).as_bytes()).unwrap();

  // 生成 parser
  let mut file = File::create("tests/generate/parser.rs").unwrap();
  file.write(parser_generate(&grammar).as_bytes()).unwrap();
  
  let mut file = File::create("tests/generate/lexer.rs").unwrap();
  file.write(lexer_generate(&lexer_visitor.lexer_rule_map, "chiru").as_bytes()).unwrap();

  let mut file = File::create("tests/generate/context.rs").unwrap();
  file.write(context_generate(&grammar, ast.as_ref()).as_bytes()).unwrap();

}


