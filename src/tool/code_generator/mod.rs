

pub mod target;
pub mod language;
pub mod name_case;




// 代码生成器

use std::{path::Path, error::Error, collections::HashSet};


use crate::tool::{grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext};


use self::{target::{Target, rust_target::RustTarget}, name_case::{WriteFileData, LexerCase, LexerGenData, NameCaseWithId, ParserGenData, ContextCase, ContextGenData, VisitorOrListenerGenData, WalkerGenData}, language::Language};

use super::{cli::Analyzer, visitor::context_visitor::ContextVisitor};


pub struct CodeGenerator<'a> {
  // 解析出来的语法
  grammar: &'a Grammar,
  // 还需要 ast ，持有引用
  ast: &'a dyn CompilationUnitContext,
  
  // 输出路径
  output_dir: &'a Path,

  // 输入文件
  input_file: &'a Path,
  
  #[allow(unused)]
  package_name: Option<String>,

  #[allow(unused)]
  version: String,

  // 考虑加入 analyze
  #[allow(unused)]
  analyzer: Analyzer,

  // 以及 target
  target: Box<dyn Target>,







  // 记录一下需要生成哪些文件, 默认全部
  lexer: bool,
  parser: bool,
  context: bool, 
  listener: bool,
  visitor: bool,
  walker: bool,

  
  

  
}

impl<'a> CodeGenerator<'a> {
  pub fn new(
    grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext,
    output_dir: &'a Path, input_file: &'a Path, _language: Language, package_name: Option<String>,
    version: &str, analyzer: Analyzer,
  ) -> Self {

    Self {
      grammar, ast, output_dir,package_name, version: version.to_owned(),target: Box::new(RustTarget::new()),
      input_file, analyzer,
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
    let grammar_file_name = match self.input_file.as_os_str().to_str() {
      Some(name) => name,
      None => "<unknown>",
    };
    // 定义一些常用变量
    let nonterminals: Vec<NameCaseWithId> = self.grammar.vocabulary.named_nonterminals.iter().map(|(k, v)| {
      NameCaseWithId::new(v, *k)
    }).collect::<Vec<_>>();
    let terminals = self.grammar.vocabulary.terminals.iter().map(|(id, t)| {
      NameCaseWithId::new(&t, *id)
    }).collect::<Vec<_>>();
    let lexer: Option<String> = if self.lexer {
      let mut lexer_rules = self.grammar.lexer_rule_map.values().map(|v| {
        LexerCase::new(&v.token_name, v.token_type, &v.regex, v.channel, v.skip)
      }).collect::<Vec<_>>();
    
      // 这里一定要排序
      lexer_rules.sort_by(|a, b| a.token_type.cmp(&b.token_type));
      let data = LexerGenData::new(self.grammar, self.ast, grammar_file_name, &self.version, self.package_name.as_deref(), &self.grammar.name, &lexer_rules);
      
      
      Some(self.target.generate_lexer(&data)?)
    } else { None };

    let parser: Option<String> = if self.parser {

      let (first, first_set) = self.grammar.first_set();
  
      let follow = self.grammar.follow_set(&first);
    
      let table = self.grammar.ll1_table(&first_set, &follow);

      let mut sync: HashSet<(usize, usize)> = HashSet::new();
      // 根据 follow 集合来生成 sync
      for (id, followers) in follow.iter() {
        for x in followers.iter() {
          sync.insert((*id, *x));
        }
      }

      let data = ParserGenData::new(
        self.grammar, self.ast, grammar_file_name, &self.version, self.package_name.as_deref(), 
        &self.grammar.name, &nonterminals, &table, &terminals, &sync
      );
      Some(self.target.generate_parser(&data)?)
    } else { None };

    let context: Option<String> = if self.context {

      let terminals = self.grammar.vocabulary.get_all_terminals_map();
  
      let nonterminals = self.grammar.vocabulary.get_all_named_nonterminals_map();
  
  
  
  
      // 首先解析 ast 获取 table
      let mut visitor = ContextVisitor::new(nonterminals, terminals);
      self.ast.accept(&mut visitor)?;
    
      let table = visitor.table;
      let nonterminals = self.grammar.vocabulary.get_all_named_nonterminals();
    
      nonterminals.iter().for_each(|x| {
        if ! table.contains_key(x) {
          println!("{} {}", x, self.grammar.vocabulary.get_nonterminal_name_by_id(*x).unwrap())
        }
      });
    
    
    
      let ctx_list = nonterminals.iter()
        .map(|id| { 
          let c = table.get(id).unwrap().clone();

          let rule_name = self.grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
          let terminal_list = c.0.iter().map(|id| {
            let name = self.grammar.vocabulary.get_terminal_name_by_id(*id).unwrap();
            NameCaseWithId::new(&name, *id)
          }).collect::<Vec<_>>();
        
          let terminal = c.1.iter().map(|id| {
            let name = self.grammar.vocabulary.get_terminal_name_by_id(*id).unwrap();
            NameCaseWithId::new(&name, *id)
          }).collect::<Vec<_>>();
        
          let nonterminal_list = c.2.iter().map(|id| {
            let name = self.grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
            NameCaseWithId::new(&name, *id)
          }).collect::<Vec<_>>();
        
          let nonterminal = c.3.iter().map(|id| {
            let name = self.grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
            NameCaseWithId::new(&name, *id)
          }).collect::<Vec<_>>();

          ContextCase::new(&rule_name, terminal_list, terminal, nonterminal_list, nonterminal)
          // self.ctx_gen( *id, c, grammar) 
        }).collect::<Vec<ContextCase>>();

      let data = ContextGenData::new(
        self.grammar, self.ast, grammar_file_name, &self.version, self.package_name.as_deref(), &self.grammar.name, &ctx_list
      );
      Some(self.target.generate_context(&data)?)
    } else { None };

    let listener: Option<String> = if self.listener {
      let data = VisitorOrListenerGenData::new(self.grammar, self.ast, grammar_file_name, &self.version, self.package_name.as_deref(), &self.grammar.name, &terminals);

      Some(self.target.generate_listener(&data)?)
    } else { None };

    let visitor: Option<String> = if self.visitor {
      let data = VisitorOrListenerGenData::new(self.grammar, self.ast, grammar_file_name, &self.version, self.package_name.as_deref(), &self.grammar.name, &terminals);

      Some(self.target.generate_visitor(&data)?)
    } else { None };

    let walker: Option<String> = if self.walker {
      let data = WalkerGenData::new(self.grammar, self.ast, grammar_file_name, &self.version, self.package_name.as_deref(), &self.grammar.name);
      Some(self.target.generate_walker(&data)?)
    } else { None };


    // write file
    let data = WriteFileData::new(
      self.grammar, self.ast, 
      grammar_file_name, 
      &self.version, 
      self.package_name.as_deref(),
      &self.grammar.name, &self.output_dir, 
      
      lexer, parser, context, visitor, listener, walker);

    self.target.write_file(&data);
    Ok(())
  }

}




