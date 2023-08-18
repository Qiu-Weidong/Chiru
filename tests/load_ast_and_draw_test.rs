use std::fs::File;

use chiru::tool::gui::ast_drawer::ASTDrawer;
use chiru::tool::serde_ast;

#[test]
fn load_ast_and_draw() {
  let file = File::open("src/tool/chiru/chiru2.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();

  ASTDrawer::new().draw(&ast, "chiru", "chiru.html");
}
