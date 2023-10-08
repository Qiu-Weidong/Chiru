pub mod rust_target;




use std::{path::Path, error::Error};

use crate::tool::cli::Language;

use super::name_case::{VisitorOrListenerGenData, WalkerGenData, ContextGenData, ParserGenData, LexerGenData};



pub trait Target {
  fn get_language(&self) -> Language;

  fn get_reserved_words(&self) -> &[&str];



  fn generate_visitor(&self, data: &VisitorOrListenerGenData,) -> Result<String, Box<dyn Error>>;

  fn generate_listener(&self, data: &VisitorOrListenerGenData,) -> Result<String, Box<dyn Error>>;

  fn generate_walker(&self, data: &WalkerGenData,) -> Result<String, Box<dyn Error>>;

  fn generate_context(&self, data: &ContextGenData,) -> Result<String, Box<dyn Error>>;

  fn generate_parser(&self, data: &ParserGenData,) -> Result<String, Box<dyn Error>>;

  fn generate_lexer(&self, data: &LexerGenData) -> Result<String, Box<dyn Error>>;




  // fn write_file(
  //   &self, grammar: &Grammar, ast: &dyn CompilationUnitContext, output_dir: &Path, package_name: Option<&str>
    
  // );


}

