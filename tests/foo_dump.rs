


use chiru::tool::gui::ast_drawer::ASTDrawer;

use crate::foo::create_ast;

pub mod foo;



#[test]
fn foo_draw() {
  let start = create_ast();
  ASTDrawer::new().draw(&start, "foo", "ast.html");
}







