use lexer_generate::lexer::GreetLexer;
use syntaxis::runtime::lexer::Lexer;


mod lexer_generate;

#[test]
fn lexer_gen_test() {
  // 尝试使用之
  let mut lexer = lexer_generate::lexer::GreetLexer::new(r####"
hello World   + exp  4445
  1281_IUY if 123 do const QWE
vvv while
  "####);


  while let Ok(token) = lexer.scan() {
    if token.token_type == GreetLexer::WHITE_SPACE { continue; }
    println!("{}", token)
  }

  // println!("hello world")
}
