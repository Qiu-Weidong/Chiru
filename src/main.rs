// use regex::Regex;

fn main() {
  println!("Hello, world!");


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


/*
 * rust 的 trait 可以继承。
 * 设计一个 trait 叫做 ASTContext，用作语法树的节点, 在定义一个名为 RuleContext 的trait，继承自 ASTContext
 * 定义两个结构体， TerminalContext、 ErrorContext 都实现 ASTContext 这个trait，表示终结符和非终结符。
 * 为语法中的每个规则定义 xxxContext 结构体，实现 RuleContext 这个 trait 。
 * 
 * 
 * 
 * 定义 Lexer trait 用于词法分析，定义 Parser/Recognizer trait 用于语法分析
 * 
 */




