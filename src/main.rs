// use regex::Regex;
// use std::{rc::Rc, any::Any};


pub trait Animal {
  fn eat(&self) {  
    println!("animal eat.")
  }

  // fn eat(&self);
}

pub trait Cat: Animal {
  fn eat(&self) {
    println!("cat eat.")
  }
}

pub trait Tiger: Animal {
  fn eat(&self) { 
    println!("tiger eat.")
  }
}

pub struct Tom;
impl Animal for Tom {}

impl Cat for Tom {}
impl Tiger for Tom {}

impl Tom {
  fn eat(&self) {
    println!("Tom eat.")
  }
}


fn main() {

  let animal: &dyn Animal = &Tom {};
  animal.eat();

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





