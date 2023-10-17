
use std::{io::Write, fs::File};
use tera::{Context, Tera};

use chiru::runtime::ast::{rule_context::RuleContext, ast_context::AstContext};



pub struct ASTDrawer {
  pub tera: Tera,
}

impl ASTDrawer {
  pub fn new() -> Self {
    let mut tera = Tera::default();
    tera.add_raw_template("ast", include_str!("../templates/gui/ast.html")).unwrap();
    tera.autoescape_on(vec![]);

    ASTDrawer { tera }
  }

  fn escape(s: &str) -> String {
    s.replace("\\", "\\\\").replace("`", "\\`")
  }

  fn dump(ast: &RuleContext) -> String {
    // println!("{}", ast);
    let mut children = String::from("[");
    for child in ast.children.iter() {
      match child {
        AstContext::Terminal(ctx) => children += &format!("{{ token_name:`{}`, text: `{}`, token_type: `{}` }}", 
          ASTDrawer::escape(&ctx.symbol.token_name), ASTDrawer::escape(&ctx.symbol.text), ctx.symbol.token_type),
        AstContext::Rule(ctx) => children += &ASTDrawer::dump(ctx),
        AstContext::Error(ctx) => {
          use chiru::runtime::ast::error_context::ErrorSymbol::*;
          match &ctx.symbol {
            Redundant(token) => {
              children += &format!("{{ token_name: `{}`, text: `{}`, token_type: `{}`, error_type: `redundant` }}", 
                ASTDrawer::escape(&token.token_name), ASTDrawer::escape(&token.text), token.token_type
              )
            },
            Mistake(token) => children += &format!("{{ token_name: `{}`, text: `{}`, token_type: `{}`, error_type: `mistake` }}", 
              ASTDrawer::escape(&token.token_name), ASTDrawer::escape(&token.text), token.token_type
            ),
            Missing => children += "{ error_type: `missing` }",
          }
        },
      }

      children += ",";
    }
    children += "]";
    format!("{{ rule_name:`{}`, rule_index: `{}`, children:{},}}", ASTDrawer::escape(&ast.rule_name), ast.rule_index, children)
  }

  pub fn draw(&self, ast: &RuleContext, name: &str, file: &mut File) {
    let mut context = Context::new();
    let ast = ASTDrawer::dump(ast);
    context.insert("ast", &format!("const ast = {};", ast));
    context.insert("name", name);


    let result = self.tera.render("ast", &context).unwrap();
    file.write(result.as_bytes()).unwrap();
  }


}












