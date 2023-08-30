

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

use chiru::{tool::{visitor::{lexer_rule::LexerRule, string_literal_to_token_visitor::StringLiteralToTokenVisitor, lexer_rule_visitor::LexerRuleVisitor, parser_rule_visitor::ParserRuleVisitor, grammar_visitor::GrammarVisitor}, 
grammar::Grammar, syntaxis::{syntaxis_parser::SyntaxisParser, syntaxis_lexer::SyntaxisLexer}, 
code_gen::visitor_gen::generate_visitor}, 
runtime::{token_stream::TokenStream, ast::rule_context::RuleContext, lexer::Lexer}};



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


  lexer_rule: TOKEN_REF COLON regular SEMI;
  regular: REGULAR_LITERAL;

  RULE_REF: r###"[a-z][a-zA-Z0-9_]*"###;
  TOKEN_REF: r###"[A-Z][a-zA-Z0-9_]*"###;
  COLON: r###"::=|:=|->|=>|:|="###;
  SEMI: r###";"###;
  OR: r###"\|"###;
  EPSILON: r###"ε|epsilon"###;
  STAR: r###"\*"###;
  PLUS: r###"\+"###;
  QUESTION: r###"\?"###;
  LPAREN: r###"\("###;
  RPAREN: r###"\)"###;
  STRING_LITERAL: r###""((\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d|[^\a\d\n\r\t\f\v\\"])*""###;
  REGULAR_LITERAL: r###"(?s)r##".*?"##"###;
  WHITE_SPACE: r###"[ \r\n\t\f]+"###;
  
  "######;

  let lexer = SyntaxisLexer::new(input);

  for token in lexer.iter() {
    println!("{}", token);
  }

  let mut stream = TokenStream::new(&lexer, 0);

  stream.consume().unwrap(); // 注意要先将 _START 消耗掉
  let parser = SyntaxisParser::new();
  let ast = parser.rule_list(&mut stream);


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

  // println!("{}", grammar)

  // 根据产生式构造 ast
  // let file = File::open("src/tool/syntaxis/syntaxis.json").unwrap();
  // let ast = serde_ast::from_reader(file).unwrap();
  // ASTDrawer::new().draw(ast.as_ref().as_rule(), "parser", "output/foo2.html");
  // let (grammar, _) = load_grammar(ast.as_rule());
  // let mut file = File::create("tests/generate/visitor.rs").unwrap();
  
  // // file.write(generate_visitor(&grammar).as_bytes()).unwrap();
  // file.write(generate_visitor(&grammar).as_bytes()).unwrap();
  

}


pub fn load_grammar(ast: &RuleContext) -> (Grammar, Vec<LexerRule>) {
  // let file = File::open(path);
  // let file = File::open("src/tool/syntaxis/syntaxis2.json").unwrap();
  // let ast = serde_ast::from_reader(file).unwrap();


  // file.read_to_string(buf)


  // ASTDrawer::new().draw(&ast, "parser", "ast.html");


  let mut grammar = Grammar::new("Chiru");
  // let token_cnt;
  // let data;
  // {
  //   let mut visitor = StringLiteralToTokenVisitor::new(
  //     &mut grammar, 2
  //   );

  //   ast.accept(&mut visitor);
  //   token_cnt = visitor.next_token_id;
  //   data = visitor.data;
  // }
  
  // let rule_cnt; 
  // let lexer_data;
  // {
  //   let mut visitor = SymbolVisitor::new(&mut grammar, token_cnt, 0, data);
  //   ast.accept(&mut visitor);
  //   rule_cnt = visitor.next_rule_id;
  //   lexer_data = visitor.data;
  // }

  // {
  //   let mut visitor = ProductionVisitor::new(&mut grammar, rule_cnt);
  //   ast.accept(&mut visitor);
  // }

  // (grammar, lexer_data)
  todo!()

}



