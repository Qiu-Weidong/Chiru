




#[test]
fn rust_test() {


  let input = r#####################"


  fn main() {
    let mut value = 42;

    let mut closure = || {
        value += 1;
        println!("The value is: {}", value);
    };

    closure();

    value += 1;
    closure();
  }
  
  
  
  "#####################;



  input.split("\n").for_each(|f| {
    print!("{}, ", f.len())
  });

  // let r: &Range<usize> = &(0..4);
  // let s = &input[*r];


  let mut st = 0;

  let ranges = input.split("\n").map(|f| {
    let ed = st + f.len();
    let ret = st..ed;
    st = ed + 1;
    ret
  }).collect::<Vec<_>>();

  println!("");
  for (index, range) in ranges.iter().enumerate() {
    println!("line {}: {}", index, &input[range.start..range.end])
  }
  
}


