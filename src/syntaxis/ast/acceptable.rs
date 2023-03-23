use std::any::Any;

use crate::syntaxis::{to_any::ToAny, visitor::ASTVisitor};



pub trait Acceptable: ToAny {
  fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any>;
}







