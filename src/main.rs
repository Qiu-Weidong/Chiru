// use regex::Regex;
use std::{rc::Rc, any::Any};

pub trait Trait: Any {
  fn as_any(&self) -> &dyn Any;

  fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Hello {}

impl Trait for Hello {
  fn as_any(&self) -> &dyn Any {
    self as &dyn Any 
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any 
  }

}


fn main() {

  let hello = Hello {};
  let rc1 = Rc::new(hello);
  let rc2: Rc<dyn Trait> = Rc::new(Hello {});

  // let s = String::from("hello world!");
  // println!("{}", s);


  // let rc1 = Rc::new(s);
  // let rc2 = Rc:: 


  // let re: Regex = Regex::new(r"
  // (?xs)
  //   (?P<login>login) |
  //   (?P<register>register) |
  //   (?P<fuck>fuck) | 
  //   (?<comment>/* .*? */)
  // ").unwrap();


  // [1,].len();

  // re.c
  // let k = 0..1;

}





