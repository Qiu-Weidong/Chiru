use std::fs::File;

use chiru::tool::grammar::Grammar;
use chiru::tool::serde_ast;
use chiru::tool::syntaxis::syntaxis_context::RuleListContext;
use chiru::tool::visitor::grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor};
use chiru::tool::visitor::lexer_rule_visitor::LexerRuleData;

#[allow(dead_code)]
pub fn load_ast() -> (Grammar, Vec<LexerRuleData>) {
  let file = File::open("src/tool/chiru/chiru2.json").unwrap();
  let ast = serde_ast::from_reader(file).unwrap();

  let mut grammar = Grammar::new("我的文法");
  let token_cnt;
  let data;
  {
    let mut visitor = StringLiteralToTokenVisitor::new(
      &mut grammar, 2
    );

    ast.accept(&mut visitor);
    token_cnt = visitor.next_token_id;
    data = visitor.data;
  }
  
  let rule_cnt; 
  let lexer_data;
  {
    let mut visitor = SymbolVisitor::new(&mut grammar, token_cnt, 0, data);
    ast.accept(&mut visitor);
    rule_cnt = visitor.next_rule_id;
    lexer_data = visitor.data;
  }

  {
    let mut visitor = ProductionVisitor::new(&mut grammar, rule_cnt);
    ast.accept(&mut visitor);
  }

  (grammar, lexer_data)

}



