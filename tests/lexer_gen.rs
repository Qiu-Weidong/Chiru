use std::{fs::File, io::Write};

use syntaxis::tool::{serde_ast, visitor::lexer_rule_visitor::{StringLiteralVisitor, LexerRuleVisitor}, syntaxis::syntaxis_visitor::SyntaxisVisitor};
use tera::{Tera, Context};

// mod visitor;
// mod lexer_generate;

#[test]
fn gen_lexer() {
  // 尝试生成 lexer 文件

  // 首先加载语法树
  let file = File::open("tests/lexer_generate/ast.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();


  let mut visitor = StringLiteralVisitor::new();
  visitor.visit(ast.as_ref());

  let mut visitor = LexerRuleVisitor::new(visitor.data, visitor.next_token_type);
  visitor.visit(ast.as_ref());

  // println!("{:?}", visitor.data);


  let mut tera = Tera::new("src/tool/templates/target/rust/*.tera").unwrap();
  tera.autoescape_on(vec![]);

  let mut context = Context::new();
  context.insert("tokens", &visitor.data);
  context.insert("name", "Greet");

  let result = tera.render("lexer.tera", &context).unwrap();
    

  let mut file = std::fs::File::create("tests/lexer_generate/lexer.rs").unwrap();
  file.write(result.as_bytes()).unwrap();
}




