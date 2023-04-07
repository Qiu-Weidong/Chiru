
// mod lexer_generate;
// mod ast_loader;

/*
  0: "_START", 
  1: "_STOP",
  2: "RULE_REF", 
  3: "TOKEN_REF", 
  4: "COLON", 
  5: "SEMI", 
  6: "OR", 
  7: "EPSILON", 
  8: "STAR", 
  9: "PLUS", 
  10: "QUESTION", 
  11: "LPAREN", 
  12: "RPAREN", 
  13: "STRING_LITERAL",
  14: "REGULAR_LITERAL", 
  15: "WHITE_SPACE", 
  
 */

use std::fs::File;

use syntaxis::tool::{grammar::Grammar, visitor::{lexer_rule_visitor::LexerRuleData, grammar_visitor::{StringLiteralToTokenVisitor, SymbolVisitor, ProductionVisitor}}, serde_ast, syntaxis::syntaxis_context::RuleListContext};



fn main() {

  let (grammar, _) = load_ast();

  // 先查看一下 token 和 rule 的编号
  grammar.terminals.iter().for_each(|(id, name)| {
    println!("{}: {}", name, id);
  });








}


pub fn load_ast() -> (Grammar, Vec<LexerRuleData>) {
  let file = File::open("src/tool/syntaxis/syntaxis2.json").unwrap();
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

