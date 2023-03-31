use std::any::Any;

use crate::runtime::ast::rule_context::RuleContext;


pub trait SyntaxisAcceptor {
  fn accept(&self, visitor: &dyn SyntaxisAcceptor) -> Box<dyn Any>;
}


impl SyntaxisAcceptor for RuleContext {
  fn accept(&self, visitor: &dyn SyntaxisAcceptor) -> Box<dyn Any> {
    todo!()
  }
}

pub trait SyntaxisVisitor {
  
}







