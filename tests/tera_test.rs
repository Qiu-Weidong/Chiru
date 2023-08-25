
#[macro_use]
extern crate lazy_static;

// use serde::Serialize;
use tera::{Tera, Context};

lazy_static! {
  pub static ref TEMPLATES: Tera = {
    let mut tera = Tera::new("src/tool/templates/**/*").unwrap();
    tera.autoescape_on(vec![]);
    tera
  };
}

#[test]
fn tera_test() {
  let items = vec![(0, 0, 1), (2, 3, 4)];
  let mut context = Context::new();
  context.insert("items", &items);
  // context.insert("name", "Syntaxis");

  let result = TEMPLATES.render("target/rust/ll1_table.tera", &context).unwrap();


  println!("{}", result);
}

