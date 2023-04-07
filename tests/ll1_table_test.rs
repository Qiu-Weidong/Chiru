mod ast_loader;

#[test]
fn ll1_table_test() {
  
  let (grammar, _) = ast_loader::load_ast();
  let mut grammar = grammar;

  println!("{}", grammar);

  let (first, first_set) = grammar.first_set();

  let follow = grammar.follow_set(&first);

  let table = grammar.ll1_table(&first_set, &follow);

  println!("{:?}", table);

}
