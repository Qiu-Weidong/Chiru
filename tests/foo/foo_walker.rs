use chiru::runtime::ast::{ast_context::ASTContext, rule_context::RuleContext, to_rule::ToRule};

use super::foo_listener::FooListener;

pub trait FooWalker {
  // 类似于 visit , 使用 Walkerable 的好处是 xxxContext 都可以作为参数传入, 而不一定都是 RuleContext 。
  fn walk(&self, listener: &dyn FooListener, ast: &RuleContext) {
    let ast = ast.as_rule();

    listener.enter_every_rule(ast);
    listener.enter_rule(ast);

    for child in ast.children.iter() {
      match child {
        ASTContext::Terminal(ctx) => {
          listener.enter_terminal(ctx);
          listener.exit_terminal(ctx);
        },
        ASTContext::Rule(ctx) => self.walk(listener, ctx),
        ASTContext::Error(ctx) => {
          listener.enter_errornode(ctx);
          listener.exit_errornode(ctx);
        },
      }
    }

    listener.exit_rule(ast);
    listener.exit_every_rule(ast);
  }

}

pub struct FooBaseWalker;

impl FooWalker for FooBaseWalker {}