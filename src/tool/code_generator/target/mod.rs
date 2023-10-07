pub mod rust_target;




use std::path::Path;

use crate::tool::{cli::Language, grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext};

use super::name_case::{VisitorOrListenerGenData, WalkerGenData, ContextGenData, ParserGenData, LexerGenData};



pub trait Target {
  fn get_language(&self) -> Language;

  fn get_reserved_words(&self) -> &[&str];



  fn generate_visitor(
    &self, 
    grammar: &Grammar, 
    ast: &dyn CompilationUnitContext,
    data: &VisitorOrListenerGenData,
  ) -> String;

  fn generate_listener(
    &self, 
    grammar: &Grammar, 
    ast: &dyn CompilationUnitContext,
    data: &VisitorOrListenerGenData,
  ) -> String;

  fn generate_walker(
    &self, 
    grammar: &Grammar, 
    ast: &dyn CompilationUnitContext,
    data: &WalkerGenData,
  ) -> String;

  fn generate_context(
    &self, 
    grammar: &Grammar, 
    ast: &dyn CompilationUnitContext,
    data: &ContextGenData,
  ) -> String;

  fn generate_parser(
    &self, 
    grammar: &Grammar, 
    ast: &dyn CompilationUnitContext,
    data: &ParserGenData,
  ) -> String;

  fn generate_lexer(&self, grammar: &Grammar, ast: &dyn CompilationUnitContext, data: &LexerGenData) -> String;


  fn generate(
    &self, 
    grammar: &Grammar, 
    ast: &dyn CompilationUnitContext,
    output_dir: &Path,
    package_name: Option<String>,
    lexer: bool, 
    parser: bool, 
    context: bool,
    listener: bool, 
    visitor: bool, 
    walker: bool,
  ) ;


}

