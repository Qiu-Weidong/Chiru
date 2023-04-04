use std::fs::File;
use std::rc::Rc;

use syntaxis::tool::grammar::Grammar;
use syntaxis::tool::serde_ast;
use syntaxis::tool::syntaxis::syntaxis_context::RuleListContext;
// use syntaxis::tool::grammar;
use syntaxis::tool::visitor::grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor};

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
  let file = File::open("src/tool/syntaxis/syntaxis.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap() as Rc<dyn RuleListContext>;

  let mut grammar = Grammar::new("我的文法");
  let token_cnt;
  {
    let mut visitor = StringLiteralToTokenVisitor::new(
      &mut grammar, 2
    );

    ast.accept(&mut visitor);
    token_cnt = visitor.next_token_id;
  }
  
  let rule_cnt; {
    let mut visitor = SymbolVisitor::new(&mut grammar, token_cnt, 0);
    ast.accept(&mut visitor);
    rule_cnt = visitor.next_rule_id;
  }

  {
    let mut visitor = ProductionVisitor::new(&mut grammar, rule_cnt);
    ast.accept(&mut visitor);
  }


  println!("{}", grammar);
  


}

