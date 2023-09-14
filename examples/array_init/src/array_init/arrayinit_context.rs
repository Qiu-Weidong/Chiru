use std::any::Any;
use std::error::Error;


use chiru::runtime::ast::rule_context::RuleContext;
use chiru::runtime::ast::terminal_context::TerminalContext;
use chiru::runtime::ast::to_rule::ToRule;


use super::arrayinit_lexer::ArrayInitLexer;
use super::arrayinit_parser::ArrayInitParser;
use super::arrayinit_visitor::ArrayInitVisitor;
use super::arrayinit_listener::ArrayInitListener;



pub trait NumbersContext: ToRule {
  

  
  fn comma_list(&self) -> Vec<&TerminalContext>;
  fn num_list(&self) -> Vec<&TerminalContext>;

  

  

  fn accept(&self, visitor: &mut dyn ArrayInitVisitor) -> Result<Box<dyn Any>, Box<dyn Error>>;
  fn enter(&self, listener: &mut dyn ArrayInitListener);
  fn exit(&self, listener: &mut dyn ArrayInitListener);
}

impl NumbersContext for RuleContext {

  

  
  fn comma_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(ArrayInitLexer::COMMA)
  } 
  fn num_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(ArrayInitLexer::NUM)
  } 

  

  


  fn accept(&self, visitor: &mut dyn ArrayInitVisitor) -> Result<Box<dyn Any>, Box<dyn Error>> {
    visitor.visit_numbers(self)
  }

  fn enter(&self, listener: &mut dyn ArrayInitListener) {
    listener.enter_numbers(self)
  }

  fn exit(&self, listener: &mut dyn ArrayInitListener) {
    listener.exit_numbers(self)
  }
}


pub trait CompilationUnitContext: ToRule {
  

  

  
  fn numbers(&self) -> Option<&dyn NumbersContext>;

  
  fn rbracket(&self) -> Option<&TerminalContext>;
  fn lbracket(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ArrayInitVisitor) -> Result<Box<dyn Any>, Box<dyn Error>>;
  fn enter(&self, listener: &mut dyn ArrayInitListener);
  fn exit(&self, listener: &mut dyn ArrayInitListener);
}

impl CompilationUnitContext for RuleContext {

  

  

  
  fn numbers(&self) -> Option<&dyn NumbersContext> {
    self.get_rule_context(ArrayInitParser::NUMBERS, 0).map(|ctx| ctx as &dyn NumbersContext)
  } 

  
  fn rbracket(&self) -> Option<&TerminalContext> {
    self.get_terminal(ArrayInitLexer::RBRACKET, 0)
  } 
  fn lbracket(&self) -> Option<&TerminalContext> {
    self.get_terminal(ArrayInitLexer::LBRACKET, 0)
  } 


  fn accept(&self, visitor: &mut dyn ArrayInitVisitor) -> Result<Box<dyn Any>, Box<dyn Error>> {
    visitor.visit_compilation_unit(self)
  }

  fn enter(&self, listener: &mut dyn ArrayInitListener) {
    listener.enter_compilation_unit(self)
  }

  fn exit(&self, listener: &mut dyn ArrayInitListener) {
    listener.exit_compilation_unit(self)
  }
}




