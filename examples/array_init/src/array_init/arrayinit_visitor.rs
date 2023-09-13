use std::{any::Any, error::Error};

use chiru::runtime::ast::{rule_context::RuleContext, ast_context::ASTContext, error_context::ErrorContext, terminal_context::TerminalContext};

use super::{
  arrayinit_context::{
    CompilationUnitContext,NumbersContext,
  },
  arrayinit_parser::ArrayInitParser, 
};


pub trait ArrayInitVisitor {
  
  fn visit_compilation_unit(&mut self, ctx: &dyn CompilationUnitContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_numbers(&mut self, ctx: &dyn NumbersContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  

  
  fn visit(&mut self, ast: &RuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    match ast.get_rule_index() {
      
      ArrayInitParser::COMPILATION_UNIT => self.visit_compilation_unit(ast),
      ArrayInitParser::NUMBERS => self.visit_numbers(ast),

      _ => self.visit_children(ast)
    }
  }

  fn visit_terminal(&mut self, _terminal: &TerminalContext) -> Result<Box<dyn Any>, Box<dyn Error>>  { self.default_result() }

  fn visit_errornode(&mut self, _errornode: &ErrorContext) -> Result<Box<dyn Any>, Box<dyn Error>>  { self.default_result() }

  fn visit_children(&mut self, ctx: &RuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    let mut result = self.default_result();
    for child in ctx.children.iter() {
      if ! self.should_visit_next_child(ctx, &result) { break; }

      let child_result = match child {
        ASTContext::Terminal(ctx) => self.visit_terminal(ctx),
        ASTContext::Rule(ctx) => self.visit(ctx),
        ASTContext::Error(ctx) => self.visit_errornode(ctx),
      };

      result = self.aggregate_result(result, child_result);
    }
    result
  }

  fn default_result(&mut self) -> Result<Box<dyn Any>, Box<dyn Error>> { Ok(Box::new(())) }

  fn aggregate_result(&mut self, _aggregate: Result<Box<dyn Any>, Box<dyn Error>> , next_result: Result<Box<dyn Any>, Box<dyn Error>> ) -> Result<Box<dyn Any>, Box<dyn Error>>  { next_result }

  fn should_visit_next_child(&mut self, _context: &RuleContext, _current_result: &dyn Any) -> bool {true}
}







