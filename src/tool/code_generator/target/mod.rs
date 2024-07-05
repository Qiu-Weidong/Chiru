pub mod rust_target;




use std::error::Error;

use super::{language::Language, name_case::{ContextGenData, LexerGenData, ParserGenData, VisitorOrListenerGenData, VocabularyGenData, WalkerGenData, WriteFileData}};



pub trait Target {
  fn get_language(&self) -> Language;

  fn get_reserved_words(&self) -> &[&str];



  fn generate_visitor(&self, data: &VisitorOrListenerGenData) -> Result<String, Box<dyn Error>>;

  fn generate_listener(&self, data: &VisitorOrListenerGenData) -> Result<String, Box<dyn Error>>;

  fn generate_walker(&self, data: &WalkerGenData) -> Result<String, Box<dyn Error>>;

  fn generate_context(&self, data: &ContextGenData) -> Result<String, Box<dyn Error>>;

  fn generate_parser(&self, data: &ParserGenData) -> Result<String, Box<dyn Error>>;

  fn generate_lexer(&self, data: &LexerGenData) -> Result<String, Box<dyn Error>>;

  fn generate_vocabulary(&self, data: &VocabularyGenData) -> Result<String, Box<dyn Error>>;

  fn write_file(&self, data: &WriteFileData)  ;


}

