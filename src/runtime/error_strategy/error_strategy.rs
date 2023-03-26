

pub trait ErrorStrategy {
  fn reset(&self) {}

  fn recover(&self) {}

  fn sync(&self) {}

  fn report(&self) {}
}


pub struct BailErrorSrategy;
impl ErrorStrategy for BailErrorSrategy {}



pub struct DefaultErrorStrategy;
impl ErrorStrategy for DefaultErrorStrategy {}


