
// 直接写测试函数即可





use crate::foo::{foo_visitor::{FooBaseVisitor, FooAcceptor}, foo_walker::{FooBaseWalker, FooWalker}, foo_listener::FooBaseListener};
use crate::foo::create_ast;
pub mod foo;

#[test]
fn test_foo() {
  let start = create_ast();



  println!("{}", start);


  let visitor = FooBaseVisitor {};
  start.accept(&visitor);

  let walker = FooBaseWalker {};
  let listener = FooBaseListener {};

  walker.walk(&listener, &start);
}


