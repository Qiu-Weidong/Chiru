use std::fs::File;

use syntaxis::tool::grammar::Grammar;
use syntaxis::tool::serde_ast;
use syntaxis::tool::syntaxis::syntaxis_context::AlternativeContext;
// use syntaxis::tool::grammar;
use syntaxis::tool::visitor::grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor};



#[test]
fn grammar_test() {
  let file = File::open("src/tool/syntaxis/syntaxis.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();

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


  println!("fuck {}", grammar);
  


}

