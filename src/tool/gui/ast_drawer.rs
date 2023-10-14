
use std::{io::Write, fs::File};
use tera::{Context, Tera};

use chiru::runtime::ast::{rule_context::RuleContext, ast_context::AstContext};



pub struct ASTDrawer {
  pub tera: Tera,
}

impl ASTDrawer {
  pub fn new() -> Self {

    // let mut tera = Tera::new("src/tool/templates/**/*.html").unwrap();
    let mut tera = Tera::default();
    tera.add_raw_template("ast", include_str!("../templates/gui/ast.html")).unwrap();
    tera.autoescape_on(vec![]);

    ASTDrawer { tera }
  }

  fn escape(s: &str) -> String {
    s.replace("\\", "\\\\").replace("`", "\\`")
  }

  fn dump(ast: &RuleContext) -> String {
  
    let mut children = String::from("[");
    for child in ast.children.iter() {
      match child {
        AstContext::Terminal(ctx) => children += &format!("{{ name:`{}`, text: `{}`, id: `{}`}}", 
          ASTDrawer::escape(&ctx.symbol.token_name), ASTDrawer::escape(&ctx.symbol.text), ctx.symbol.token_type),
        AstContext::Rule(ctx) => children += &ASTDrawer::dump(ctx),
        AstContext::Error(ctx) => children += &ctx.to_string(),
      }

      children += ",";
    }
    children += "]";
    format!("{{ name:`{}`,id: `{}`, children:{},}}", ASTDrawer::escape(&ast.rule_name), ast.rule_index, children)
  }

  pub fn draw(&self, ast: &RuleContext, name: &str, file: &mut File) {
    let mut context = Context::new();
    context.insert("ast", &format!("const ast = {};", ASTDrawer::dump(ast)));
    context.insert("name", name);


    let result = self.tera.render("ast", &context).unwrap();
    

    // let mut file = std::fs::File::create(filename).unwrap();
    file.write(result.as_bytes()).unwrap();
  }


}












