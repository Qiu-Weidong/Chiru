



use chiru::runtime::ast::{rule_context::RuleContext, ast_context::AstContext};

use super::arrayinit_listener::ArrayInitListener;



pub trait ArrayInitWalker {
  fn walk(&self, listener: &mut dyn ArrayInitListener, ast: &RuleContext) {
    listener.enter_every_rule(ast);
    listener.enter(ast);

    for child in ast.children.iter() {
      match child {
        AstContext::Terminal(ctx) => {
          listener.enter_terminal(ctx);
          listener.exit_terminal(ctx);
        },
        AstContext::Rule(ctx) => self.walk(listener, ctx),
        AstContext::Error(ctx) => {
          listener.enter_errornode(ctx);
          listener.exit_errornode(ctx);
        },
      }
    }

    listener.exit(ast);
    listener.exit_every_rule(ast);
  }
}







