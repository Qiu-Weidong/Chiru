

pub mod target;
pub mod name_case;




// 代码生成器

use std::path::PathBuf;


use crate::tool::{grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext};

use self::{target::{Target, rust_target::RustTarget}, name_case::NameCase};

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

  // 输入文件
  input_file: PathBuf,

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
    output_dir: PathBuf, input_file: PathBuf, _language: Language, package_name: Option<String>,
    version: &str
  ) -> Self {

    Self {
      grammar, ast, output_dir,package_name, version: version.to_owned(),target: Box::new(RustTarget::new()),
      input_file,
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
  pub fn generate(&self) {
    // grammar_file_name: String       语法文件名,如 Chiru.chiru
    // chiru_version: String           生成该文件所使用的 chiru 版本,如 0.7.0
    // package_name: Option<NameCase>  包名, 可能为空
    // grammar_name: NameCase          语法名称
    let grammar_file_name = self.input_file.to_str().unwrap().to_owned();
    let chiru_version = self.version.clone();
    let package_name = match &self.package_name {
      Some(package_name) => Some(NameCase::new(package_name)),
      None => None,
    };
    let grammar_name = NameCase::new(&self.grammar.name);


    let lexer_result: Option<String> = if self.lexer { 
      Some(self.target.generate_lexer(self.grammar, self.ast))
    } else {  None };





    // 在这里使用 target 生成相关的文件, 最后使用 target 将结果按结构写入
    self.target.generate(&self.grammar, self.ast, &self.output_dir, None, self.lexer, self.parser, self.context, self.listener, self.visitor, self.walker);
  }

}




