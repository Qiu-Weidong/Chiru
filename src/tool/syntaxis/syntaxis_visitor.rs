use std::any::Any;


pub trait SyntaxisAcceptor {
  fn accept(&self, visitor: &dyn SyntaxisAcceptor) -> Box<dyn Any>;
}

pub trait SyntaxisVisitor {
  
}




