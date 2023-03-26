
use std::io::stdin;

use syntaxis::tool::serde_ast::from_reader;



fn main() {
  let ast = from_reader(stdin()).unwrap();


  println!("{}", ast)
}





