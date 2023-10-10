mod generate;


trait Listener {
  fn enter(&self);
}

trait ChiruListener: Listener {
  fn enter(&self) {
    // 将
  }
}

struct MyListener;

impl ChiruListener for MyListener {
}

impl Listener for MyListener {
  fn enter(&self) {
    
  }  
}



#[test]
fn foo_test() {
}


// #[test]
// fn foo_test() {
//   let listener = ListenerImpl {};
//   walk(&listener)
// }

