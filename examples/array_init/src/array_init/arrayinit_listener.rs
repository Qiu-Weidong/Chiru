


use chiru::runtime::ast::{terminal_context::TerminalContext, error_context::ErrorContext, rule_context::RuleContext};

use super::{
  arrayinit_parser::ArrayInitParser, 
  arrayinit_context::{
    CompilationUnitContext,NumbersContext,
  }
};


pub trait ArrayInitListener {
  
  fn enter_compilation_unit(&mut self, _ctx: &dyn CompilationUnitContext) {}
  fn exit_compilation_unit(&mut self, _ctx: &dyn CompilationUnitContext) {}
  
  fn enter_numbers(&mut self, _ctx: &dyn NumbersContext) {}
  fn exit_numbers(&mut self, _ctx: &dyn NumbersContext) {}
  



  fn enter_every_rule(&mut self, _ctx: &RuleContext) {}

  fn exit_every_rule(&mut self, _ctx: &RuleContext) {}

  fn enter(&mut self, ctx: &RuleContext) {
    // 在这里进行派发即可
    match ctx.get_rule_index() {
      
      ArrayInitParser::COMPILATION_UNIT => self.enter_compilation_unit(ctx), 
      ArrayInitParser::NUMBERS => self.enter_numbers(ctx), 

      _ => {}
    }
  }

  fn exit(&mut self, ctx: &RuleContext) {
    match ctx.get_rule_index() {
      
      ArrayInitParser::COMPILATION_UNIT => self.exit_compilation_unit(ctx), 
      ArrayInitParser::NUMBERS => self.exit_numbers(ctx), 

      _ => {}
    }
  }

  fn enter_terminal(&mut self, _ctx: &TerminalContext) {}

  fn exit_terminal(&mut self, _ctx: &TerminalContext) {}

  fn enter_errornode(&mut self, _ctx: &ErrorContext) {}

  fn exit_errornode(&mut self, _ctx: &ErrorContext) {}  
}







