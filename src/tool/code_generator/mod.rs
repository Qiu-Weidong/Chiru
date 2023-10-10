

pub mod target;
pub mod name_case;




// 代码生成器

use std::{path::PathBuf, error::Error};


use crate::tool::{grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext};

// use self::target::{Target, rust_target::RustTarget};

use self::{target::{Target, rust_target::RustTarget}, name_case::WriteFileData};

use super::cli::Language;

pub struct CodeGenerator<'a> {
  // 解析出来的语法
  grammar: &'a Grammar,
  // 还需要 ast ，持有引用
  ast: &'a dyn CompilationUnitContext,

  // 记录一下需要生成哪些文件, 默认全部
  lexer: bool,
  parser: bool,
  context: bool, 
  listener: bool,
  visitor: bool,
  walker: bool,

  
  // 输出路径
  output_dir: PathBuf,

  #[allow(unused)]
  package_name: Option<String>,

  #[allow(unused)]
  version: String,

  // 以及 target
  target: Box<dyn Target>,
}

impl<'a> CodeGenerator<'a> {
  pub fn new(
    grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext,
    output_dir: PathBuf, _language: Language, package_name: Option<String>,
    version: &str
  ) -> Self {

    Self {
      grammar, ast, output_dir,package_name, version: version.to_owned(),target: Box::new(RustTarget::new()),
      lexer: true, parser: true, context: true, listener: true, visitor: true, walker: true,
    }
  }


  pub fn toggle_lexer_generation(&mut self, flag: bool) { self.lexer = flag; }
  pub fn toggle_parser_generation(&mut self, flag: bool) { self.parser = flag; }
  pub fn toggle_context_generation(&mut self, flag: bool) { self.context = flag; }
  pub fn toggle_listener_generation(&mut self, flag: bool) { self.listener = flag; }
  pub fn toggle_visitor_generation(&mut self, flag: bool) { self.visitor = flag; }
  pub fn toggle_walker_generation(&mut self, flag: bool) { self.walker = flag; }



  // 直接写文件即可
  pub fn generate(&self) -> Result<(), Box<dyn Error>> {

    // let grammar_name = NameCase::new(&self.grammar.name);
    let package_name = if let Some(package_name) = &self.package_name { Some(package_name) } else { None };

    let lexer: Option<String> = if self.lexer {
      None
    } else { None };

    let parser: Option<String> = if self.parser {
      None
    } else { None };

    let context: Option<String> = if self.context {
      None
    } else { None };

    let listener: Option<String> = if self.listener {
      None
    } else { None };

    let visitor: Option<String> = if self.visitor {
      None
    } else { None };

    let walker: Option<String> = if self.walker {
      None
    } else { None };


    // write file
    let data = WriteFileData::new(
      self.grammar, self.ast, "", "", package_name.map(|x| x.as_str()), 
      &self.grammar.name, &self.output_dir, 
      lexer, parser, context, visitor, listener, walker);

    self.target.write_file(&data);
    Ok(())
  }

}




