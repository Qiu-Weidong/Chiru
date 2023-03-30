use std::fs::File;

// use regex::Regex;
use syntaxis::tool::{serde_ast, gui::ast_drawer::ASTDrawer};

/**
 * syntaxis <grammar-filename> [-o 输出目录] [-encoding 编码] [-listener] [-visitor] [-package 模块名称] [-language 目标语言]
 *
 * syntaxis <grammar-filename> [-tokens] [-tree] [-gui] [-encoding 编码]
 */

fn main() {
  // let re_str = r##""([^\a\d\n\r\t\f\v\\"]|(\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*""##;
  /*   let re_str = r##"^(\+|\*|\?|\(|\))"##;
    println!("{}", re_str);
    let re = Regex::new(re_str).unwrap();

    let mut input = String::new();

    // let mut n = std::io::stdin().read_line(&mut input).unwrap();
    let mut n = 1;
    while n > 0 {
      input.clear();
      n = std::io::stdin().read_line(&mut input).unwrap();

      let result = re.find_at(&input, 0);

      match result {
        Some(result) => { println!("{:?}", result) },
        None => { println!("不匹配"); continue; }
      }

    }
  */

  let file = File::open("src/tool/ast.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();

  ASTDrawer::new().draw(&ast, "syntaxis", "syntaxis.html");

  // let s = r####""([^\a\d\n\r\t\f\v\\"]|(\\\\|\\"|\\a|\\d|\\n|\\r|\\t|\\f|\\v|\\u\{(0x|0)?[a-f0-9]+\})|\d)*""####;
  // let s = s.replace("\\", "\\\\").replace("`", "\\`");
  // println!("{}", s)
}



