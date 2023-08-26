

// 专门用来确认语法特性
use regex::Regex;

#[test]
fn rust_test() {
  // let re = Regex::new(r####"r(#*)".*?"\\1"####).unwrap();

  // let input = r#####################"rule_list, foo, RTE, kkkk;
  // r#"xxx"#
  // "#####################;



  // println!("{}", re.is_match(input));


  // for mat in re.find_iter(input) {
  //   println!("{}", &input[mat.start()..mat.end()])
  // }

  let re = Regex::new(r"(a*)b$1").unwrap();
  let text = r"aab$1";
  if re.is_match(text) {
    println!("Pattern matched!");
  } else {
    println!("不匹配");
  }
  
}


