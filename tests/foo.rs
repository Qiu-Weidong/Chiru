mod generate;


pub trait Listener {
  fn enter(&self) {
    println!("enter listener.")
  }
}

pub trait MyListener : Listener {
  fn enter(&self) {
    println!("enter my listener.")
  }
}



pub fn walk(listener: &dyn Listener) {
  listener.enter();
}

struct ListenerImpl;
impl MyListener for ListenerImpl {}
impl Listener for ListenerImpl {}


#[test]
fn foo_test() {
  let listener = ListenerImpl {};
  walk(&listener)
}

