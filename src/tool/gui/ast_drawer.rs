
use std::io::Write;
use tera::{Context, Tera};

use chiru::runtime::ast::{rule_context::RuleContext, ast_context::ASTContext};



pub struct ASTDrawer {
  pub tera: Tera,
}

impl ASTDrawer {
  pub fn new() -> Self {
    let mut tera = Tera::new("src/tool/templates/**/*.html").unwrap();
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
        ASTContext::Terminal(ctx) => children += &format!("{{ name:`{}`, text: `{}`, id: `{}`}}", 
          ASTDrawer::escape(&ctx.symbol.token_name), ASTDrawer::escape(&ctx.symbol.text), ctx.symbol.token_type),
        ASTContext::Rule(ctx) => children += &ASTDrawer::dump(ctx),
        ASTContext::Error(ctx) => children += &format!("{{ name:`{}`, text: `{}`}}", 
          ASTDrawer::escape(&ctx.symbol.token_name), ASTDrawer::escape(&ctx.symbol.text)),
      }

      children += ",";
    }
    children += "]";
    format!("{{ name:`{}`,id: `{}`, children:{},}}", ASTDrawer::escape(&ast.rule_name), ast.rule_index, children)
  }

  pub fn draw(&self, ast: &RuleContext, name: &str, filename: &str) {
    let mut context = Context::new();
    context.insert("ast", &format!("const ast = {};", ASTDrawer::dump(ast)));
    context.insert("name", name);


    let result = self.tera.render("gui/ast.html", &context).unwrap();
    

    let mut file = std::fs::File::create(filename).unwrap();
    file.write(result.as_bytes()).unwrap();
  }


}












