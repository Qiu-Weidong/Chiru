
mod ast_loader;

/**

16 -> STAR;
16 -> PLUS;
16 -> QUESTION;
epsilon -> EPSILON;
11 -> OR alternative;
alternative -> 13;
alternative -> epsilon;
12 ->;
12 -> 11 12;
15 ->;
15 -> ebnf_suffix;
ebnf_suffix -> 16 17;
10 ->;
10 -> 9 10;
rule_list -> 10;
13 -> element;
13 -> element 13;
block -> alternative 12;
element -> 14 15;
17 ->;
17 -> QUESTION;
lexer_rule -> TOKEN_REF COLON regular SEMI;
parser_rule -> RULE_REF COLON block SEMI;
regular -> REGULAR_LITERAL;
9 -> parser_rule;
9 -> lexer_rule;
14 -> TOKEN_REF;
14 -> STRING_LITERAL;
14 -> RULE_REF;
14 -> LPAREN block RPAREN;
 */

#[test]
fn grammar_test() {
  let (grammar, _) = ast_loader::load_ast();


  println!("{}", grammar);
  


}

