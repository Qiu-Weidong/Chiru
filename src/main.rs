// use regex::Regex;

use std::rc::Rc;

pub trait Animal {
  fn eat(&self) { println!("animal eat.") }
}

struct Cat;
impl Animal for Cat {
  fn eat(&self) {
    println!("cat eat.")
  }
}



fn main() {

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

  // 可以实现多态
  let animal: Rc<dyn Animal> = Rc::new(Cat {});
  animal.eat();
}





