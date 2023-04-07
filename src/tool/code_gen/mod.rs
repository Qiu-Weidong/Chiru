use tera::Tera;


pub mod lexer_gen;
pub mod visitor_gen;
use serde::Serialize;

lazy_static! {
  pub static ref TEMPLATES: Tera = {
    let mut tera = Tera::new("src/tool/templates/**/*").unwrap();
    tera.autoescape_on(vec![]);
    tera
  };
}


#[derive(Serialize)]
pub struct RuleRenderContext {
  pub pascal: String,
  pub underscore: String,
  pub upcase: String,
  pub rule_index: usize,

  pub items: Vec<RuleRenderItem>,
}

#[derive(Serialize)]
pub struct RuleRenderItem {
  pub pascal: String,
  pub underscore: String,
  pub upcase: String,
}

