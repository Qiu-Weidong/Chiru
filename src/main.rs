

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
syntaxis::{chiru_parser::ChiruParser, chiru_lexer::ChiruLexer}, 
code_gen::{visitor_gen::generate_visitor, listener_gen::listener_generate, parser_gen::parser_generate, lexer_gen::lexer_generate, context_gen::context_generate, walker_gen::walker_generate, mod_gen::mod_generate}, gui::ast_drawer::ASTDrawer}, 
runtime::token_stream::TokenStream};



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

  // 打印一下语法树
  ASTDrawer::new().draw(ast.as_rule(), "foo", "output/foo3.html");


  // 从语法树解析出 grammar


  let mut visitor = StringLiteralToTokenVisitor::new(2);
  ast.accept(&mut visitor).unwrap();

  // println!("{} {:?}", visitor.next_token_id, visitor.lexer_rule_map);
  
  let mut lexer_visitor = LexerRuleVisitor::new(visitor.next_token_id, visitor.lexer_rule_map);
  ast.accept(&mut lexer_visitor).unwrap();

  // println!("{} {:?}", lexer_visitor.next_token_id, lexer_visitor.lexer_rule_map);

  let mut parser_visitor = ParserRuleVisitor::new();
  ast.accept(&mut parser_visitor).unwrap();

  // println!("{} {:?}", parser_visitor.next_rule_id, parser_visitor.parser_rule_map);

  let mut grammar_visitor = GrammarVisitor::new("chiru", &parser_visitor.parser_rule_map, &lexer_visitor.lexer_rule_map);
  ast.accept(&mut grammar_visitor).unwrap();

  let grammar = grammar_visitor.grammar;

  println!("{:?}", grammar.productions);

  // 根据产生式构造 ast






  let base_dir = "src/tool/syntaxis";
  // let base_dir = "tests/generate";

  // 生成 visitor 暂不写入文件
  let mut file = File::create(format!("{}/{}_visitor.rs",base_dir, grammar.name.to_lowercase())).unwrap();
  file.write(generate_visitor(&grammar).as_bytes()).unwrap();

  // 生成 listener
  let mut file = File::create(format!("{}/{}_listener.rs", base_dir, grammar.name.to_lowercase())).unwrap();
  file.write(listener_generate(&grammar).as_bytes()).unwrap();

  // 生成 parser
  let mut file = File::create(format!("{}/{}_parser.rs", base_dir, grammar.name.to_lowercase())).unwrap();
  file.write(parser_generate(&grammar).as_bytes()).unwrap();
  
  let mut file = File::create(format!("{}/{}_lexer.rs", base_dir, grammar.name.to_lowercase())).unwrap();
  file.write(lexer_generate(&lexer_visitor.lexer_rule_map, "chiru").as_bytes()).unwrap();

  let mut file = File::create(format!("{}/{}_context.rs", base_dir, grammar.name.to_lowercase())).unwrap();
  file.write(context_generate(&grammar, ast.rules().unwrap()).as_bytes()).unwrap();

  let mut file = File::create(format!("{}/{}_walker.rs", base_dir, grammar.name.to_lowercase())).unwrap();
  file.write(walker_generate(&grammar).as_bytes()).unwrap();

  // 最后再写一个 mod 文件
  let mut file = File::create(format!("{}/mod.rs", base_dir)).unwrap();
  file.write(mod_generate(&grammar, true, true, true, true, true, true).as_bytes()).unwrap();

}


