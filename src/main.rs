use regex::Regex;

fn main() {
  println!("Hello, world!");


  let re: Regex = Regex::new(r"
  (?xs)
    (?P<login>login) |
    (?P<register>register) |
    (?P<fuck>fuck) | 
    (?<comment>/* .*? */)
  ").unwrap();

  // re.c

}
