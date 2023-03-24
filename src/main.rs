// use regex::Regex;

use std::rc::Rc;

pub trait Animal { // Acceptor
  fn eat(&self); // { println!("animal eat.") }
}

pub trait Cat: Animal {
  // fn eat(&self) { println!("cat eat.") }
}

pub struct Tiger;
impl Cat for Tiger {}
impl Animal for Tiger {
  fn eat(&self) {
    println!("animal eat.")
  }
}

impl Tiger {
  fn eat(&self) { println!("tiger eat.") }
}

// trait 中不要包含 supertrait 中相同的函数。



fn main() {

  // 可以实现多态
  let animal: Rc<dyn Animal> = Rc::new(Tiger {});
  animal.eat();

  let animal: Rc<dyn Cat> = Rc::new(Tiger {});
  animal.eat();

  let animal = Rc::new(Tiger {});
  animal.eat();
}





