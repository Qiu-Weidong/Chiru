use std::fs::File;

use syntaxis::tool::gui::ast_drawer::ASTDrawer;
use syntaxis::tool::serde_ast;

#[test]
fn load_ast_and_draw() {
  let file = File::open("tests/ast/ast.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();

  ASTDrawer::new().draw(&ast, "syntaxis", "syntaxis.html");
}
