

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


use std::fs::File;

use chiru::{tool::{visitor::{grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor}, lexer_rule_visitor::LexerRuleData}, 
grammar::Grammar, serde_ast, syntaxis::{syntaxis_context::RuleListContext, syntaxis_parser::SyntaxisParser, syntaxis_lexer::SyntaxisLexer}, code_gen::parser_gen::{production_generate, ll1_table_generate, parser_generate}, gui::ast_drawer::ASTDrawer}, runtime::{token_stream::TokenStream, parser::Parser, lexer::Lexer}};




// use chiru::tool::syntaxis::syntaxis_lexer::SyntaxisLexer;

#[allow(unused_doc_comments)]
fn main() {
  // let (grammar, _) = load_ast();
  // let mut file = File::create("tests/generate/parser.rs").unwrap();
  
  // file.write(parser_generate(grammar).as_bytes()).unwrap();
  // println!("{}", parser_generate(grammar));


  let input = r####"
  stat : hello | world 
  hello: HELLO +;
  HELLO: /hello/;
  "####;

  let lexer = SyntaxisLexer::new(input);

  for token in lexer.iter() {
    print!("{}, ", token.token_name);
  }

  let mut stream = TokenStream::new(&lexer, 0);

  stream.consume().unwrap();
  let parser = SyntaxisParser::new();
  let ast = parser.rule_list(&mut stream);


  // 根据产生式构造 ast
  // let file = File::open("src/tool/syntaxis/syntaxis.json").unwrap();
  // let ast = serde_ast::from_reader(file).unwrap();
  ASTDrawer::new().draw(ast.as_ref().as_rule(), "parser", "output/foo2.html");


  print!("done")
}


pub fn load_ast() -> (Grammar, Vec<LexerRuleData>) {
  // let file = File::open(path);
  let file = File::open("src/tool/syntaxis/syntaxis2.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();


  // file.read_to_string(buf)


  // ASTDrawer::new().draw(&ast, "parser", "ast.html");


  let mut grammar = Grammar::new("Chiru");
  let token_cnt;
  let data;
  {
    let mut visitor = StringLiteralToTokenVisitor::new(
      &mut grammar, 2
    );

    ast.accept(&mut visitor);
    token_cnt = visitor.next_token_id;
    data = visitor.data;
  }
  
  let rule_cnt; 
  let lexer_data;
  {
    let mut visitor = SymbolVisitor::new(&mut grammar, token_cnt, 0, data);
    ast.accept(&mut visitor);
    rule_cnt = visitor.next_rule_id;
    lexer_data = visitor.data;
  }

  {
    let mut visitor = ProductionVisitor::new(&mut grammar, rule_cnt);
    ast.accept(&mut visitor);
  }

  (grammar, lexer_data)

}



