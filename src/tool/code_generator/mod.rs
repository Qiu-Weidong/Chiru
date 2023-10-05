

pub mod target;
pub mod name_case;

// 定义相关的 target 
pub mod rust_target; 




// 代码生成器

use std::path::PathBuf;


use crate::tool::{grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext};

use self::{target::Target, rust_target::RustTarget};

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

  // package name todo


  // 以及 target
  target: Box<dyn Target>,
}

impl<'a> CodeGenerator<'a> {
  pub fn new(
    grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext,
    output_dir: PathBuf, _language: Language
  ) -> Self {



    // 根据 language 来生成 target

    Self {
      grammar, ast, output_dir,
      lexer: true, parser: true, context: true, listener: true, visitor: true, walker: true,
      target: Box::new(RustTarget::new())
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
    // 在这里使用 target 生成相关的文件, 最后使用 target 将结果按结构写入
    self.target.generate(&self.grammar, self.ast, &self.output_dir, None, self.lexer, self.parser, self.context, self.listener, self.visitor, self.walker);
  }

}




