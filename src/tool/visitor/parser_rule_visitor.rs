use std::collections::HashMap;

use crate::tool::syntaxis::{syntaxis_visitor::SyntaxisVisitor, syntaxis_context::ParserRuleContext};





pub struct ParserRuleVisitor {
  
  pub next_rule_id: usize,

  pub parser_rule_map: HashMap<String, usize>,
}

impl ParserRuleVisitor {
  pub fn new() -> Self {
    Self { next_rule_id: 0, parser_rule_map: HashMap::new(), }
  }
}

impl SyntaxisVisitor for ParserRuleVisitor {

  fn visit_parser_rule(&mut self, ctx: &dyn ParserRuleContext) -> Box<dyn std::any::Any> {


    let name = &ctx.rule_ref().unwrap().symbol.text;

    if self.parser_rule_map.contains_key(name) {
      return self.default_result();
    }    

    self.parser_rule_map.insert(name.to_owned(), self.next_rule_id);
    self.next_rule_id += 1;

    self.default_result()
  }
}





