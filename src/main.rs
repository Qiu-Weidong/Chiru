use std::io::Write;

use serde::Serialize;
// use regex::Regex;
use tera::{Context, Tera};

pub mod tool;
pub mod runtime;


#[derive(Serialize)]
struct Product {
  name: String,
}

fn main() {
  let mut tera = Tera::new("src/tool/templates/**/*.html").unwrap();
  tera.autoescape_on(vec![".html"]);

  let product = Product { name: String::from("邱维东") };
  let mut context = Context::new();
  context.insert("product", &product);
  context.insert("vat_rate", &0.20);
  let result = tera.render("gui/product.html", &context).unwrap();
  
  let mut file = std::fs::File::create("data.txt").expect("create failed");
  file.write(result.as_bytes()).expect("write failed");
}





