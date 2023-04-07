



mod ast_loader;


/**
rule_list -> 10;
10 ->;
10 -> 9 10;
9 -> lexer_rule;
9 -> parser_rule;
parser_rule -> RULE_REF COLON block SEMI;
lexer_rule -> TOKEN_REF COLON regular SEMI;
block -> alternative 12;
12 -> 11 12;
12 ->;
11 -> OR alternative;
alternative -> epsilon;
alternative -> 13;
epsilon -> EPSILON;
element -> 14 15;

14 -> LPAREN block RPAREN;
14 -> RULE_REF;
14 -> TOKEN_REF;
14 -> STRING_LITERAL;

regular -> REGULAR_LITERAL;

16 -> PLUS;
16 -> STAR;
16 -> QUESTION;
17 -> QUESTION;
17 ->;
ebnf_suffix -> 16 17;

15 -> ebnf_suffix;
15 ->;

13 -> element;
13 -> element 13;

 */

/**
element:
{STRING_LITERAL, TOKEN_REF, RULE_REF, LPAREN, }
epsilon:
{EPSILON, }
16:
{QUESTION, PLUS, STAR, }
14:
{TOKEN_REF, RULE_REF, LPAREN, STRING_LITERAL, }
regular:
{REGULAR_LITERAL, }
17:
{QUESTION, ε}
ebnf_suffix:
{QUESTION, PLUS, STAR, }
9:
{TOKEN_REF, RULE_REF, }
11:
{OR, }
rule_list:
{RULE_REF, TOKEN_REF, ε}
parser_rule:
{RULE_REF, }
15:
{QUESTION, STAR, PLUS, ε}
13:
{TOKEN_REF, LPAREN, STRING_LITERAL, RULE_REF, }
block:
{TOKEN_REF, STRING_LITERAL, RULE_REF, EPSILON, LPAREN, }
alternative:
{EPSILON, TOKEN_REF, RULE_REF, STRING_LITERAL, LPAREN, }
10:
{TOKEN_REF, RULE_REF, ε}
lexer_rule:
{TOKEN_REF, }
12:
{OR, ε}
 */

#[test]
fn first_set_test() {
  // 测试求 first 集合


  let (grammar, _) = ast_loader::load_ast();
  let mut grammar = grammar;


  println!("{}", grammar);

  let (first, _) = grammar.first_set();

  let follow = grammar.follow_set(&first);

  // for (id, collection) in first.iter() {
  //   let name = grammar.nonterminals.get(id).unwrap();
  //   let name = match name {
  //     Some(name) => name.clone(),
  //     None => id.to_string(),
  //   };
  //   println!("{}:", name);
  //   print!("{{");
  //   for item in collection.set.iter() {
  //     let name = grammar.terminals.get(item).unwrap();
  //     print!("{}, ", name);
  //   }
  //   if collection.allow_epsilon { print!("ε") }
  //   println!("}}");
  // }



}

