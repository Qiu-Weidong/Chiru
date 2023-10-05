
use std::{fs::File, path::Path, io::Write, collections::HashSet};

use chiru::runtime::production::{Production, ProductionItem};
use tera::{Tera, Context};

use crate::tool::{visitor::context_visitor::ContextVisitor, syntaxis::chiru_context::CompilationUnitContext, code_generator::name_case::{NameCaseWithId, NameCase, ContextCase, LexerCase}};
use super::Target;



pub struct RustTarget {
  pub template: Tera,
  pub reserved_words: Vec<&'static str>
}

impl RustTarget {
  pub fn new() -> Self {
    let mut template = Tera::default();
    // 添加
    template.add_raw_template("lexer", include_str!("../../templates/target/rust/lexer.tera")).unwrap();
    template.add_raw_template("parser", include_str!("../../templates/target/rust/parser.tera")).unwrap();
    template.add_raw_template("context", include_str!("../../templates/target/rust/context.tera")).unwrap();
    template.add_raw_template("listener", include_str!("../../templates/target/rust/listener.tera")).unwrap();
    template.add_raw_template("visitor", include_str!("../../templates/target/rust/visitor.tera")).unwrap();
    template.add_raw_template("walker", include_str!("../../templates/target/rust/walker.tera")).unwrap();
    template.add_raw_template("header", include_str!("../../templates/target/rust/header.tera")).unwrap();
    template.autoescape_on(vec![]);

    let template = template;



    let reserved_words = vec! {
      // 严格关键字
      "as", "break", "const", "continue", "crate", "else", "enum",
      "extern", "false", "fn", "for", "if", "impl", "in", "let",
      "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
      "self", "Self", "static", "struct", "super", "trait", "true",
      "type", "unsafe", "use", "where", "while", "async", "await",
      "dyn",

      // 保留关键字
      "abstract", "become", "box", "do", "final", "macro", "override",
      "priv", "typeof", "unsized", "virtual", "yield", "try", "union",
      "'static"

    };
    Self { template, reserved_words }


  }




  fn production_generate(&self, production: &Production) -> String {

    let mut result = String::from("vec![");
    for item in production.right.iter() {
      match item {
        ProductionItem::NonTerminal(id) => {
          result += &format!("ProductionItem::NonTerminal({}),", id);
        },
        ProductionItem::Terminal(id) => {
          result += &format!("ProductionItem::Terminal({}),", id);
        },
      }
    }
    result += "]";
    
    format!("Production::new({}, {}, &{})", production.id, production.left, result)
  }
  
}


impl Target for RustTarget {
  fn get_language(&self) -> crate::tool::cli::Language {
    crate::tool::cli::Language::Rust
  }

  fn get_reserved_words(&self) -> &[&str] {
    &self.reserved_words
  }



  fn generate_visitor(&self, grammar: &crate::tool::grammar::Grammar, _ast: &dyn CompilationUnitContext) -> String {
    let mut nonterminals: Vec<NameCaseWithId> = Vec::new();
    for (id, name) in grammar.vocabulary.named_nonterminals.iter() {
      nonterminals.push( NameCaseWithId::new(name, *id));
    }
  
    let mut context = Context::new();
    context.insert("nonterminals", &nonterminals);
    let grammar_name = NameCase::new(&grammar.name);
    context.insert("grammar_name", &grammar_name);
    self.template.render("visitor", &context).unwrap()
  }

  fn generate_listener(&self, grammar: &crate::tool::grammar::Grammar, _ast: &dyn CompilationUnitContext) -> String {
    let mut nonterminals: Vec<NameCaseWithId> = Vec::new();
    for (id, name) in grammar.vocabulary.named_nonterminals.iter() {
      nonterminals.push( NameCaseWithId::new(name, *id));
    }
  
    let mut context = Context::new();
    context.insert("nonterminals", &nonterminals);
    let grammar_name = NameCase::new(&grammar.name);
    context.insert("grammar_name", &grammar_name);
    self.template.render("listener", &context).unwrap()
  }

  fn generate_walker(&self, grammar: &crate::tool::grammar::Grammar, _ast: &dyn CompilationUnitContext) -> String {
    let mut context = Context::new();
    let grammar_name = NameCase::new(&grammar.name);
    context.insert("grammar_name", &grammar_name);
    self.template.render("walker", &context).unwrap()
  }

  fn generate_context(&self, grammar: &crate::tool::grammar::Grammar, ast: &dyn CompilationUnitContext) -> String {
    // 获取所有的终结符和非终结符
    let terminals = grammar.vocabulary.get_all_terminals_map();
  
    let nonterminals = grammar.vocabulary.get_all_named_nonterminals_map();
  
  
  
  
    // 首先解析 ast 获取 table
    let mut visitor = ContextVisitor::new(nonterminals, terminals);
    ast.accept(&mut visitor).unwrap();
  
    let table = visitor.table;
    let nonterminals = grammar.vocabulary.get_all_named_nonterminals();
  
    nonterminals.iter().for_each(|x| {
      if ! table.contains_key(x) {
        println!("{} {}", x, grammar.vocabulary.get_nonterminal_name_by_id(*x).unwrap())
      }
    });
  
  
  
    let ctx_list = nonterminals.iter()
      .map(|id| { 
        let c = table.get(id).unwrap().clone();

        let rule_name = grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
        let terminal_list = c.0.iter().map(|id| {
          let name = grammar.vocabulary.get_terminal_name_by_id(*id).unwrap();
          NameCaseWithId::new(&name, *id)
        }).collect::<Vec<_>>();
      
        let terminal = c.1.iter().map(|id| {
          let name = grammar.vocabulary.get_terminal_name_by_id(*id).unwrap();
          NameCaseWithId::new(&name, *id)
        }).collect::<Vec<_>>();
      
        let nonterminal_list = c.2.iter().map(|id| {
          let name = grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
          NameCaseWithId::new(&name, *id)
        }).collect::<Vec<_>>();
      
        let nonterminal = c.3.iter().map(|id| {
          let name = grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
          NameCaseWithId::new(&name, *id)
        }).collect::<Vec<_>>();

        ContextCase::new(&rule_name, terminal_list, terminal, nonterminal_list, nonterminal)
        // self.ctx_gen( *id, c, grammar) 
      }).collect::<Vec<ContextCase>>();

    let mut context = Context::new();
    context.insert("ctx_list", &ctx_list);
    let grammar_name = NameCase::new(&grammar.name);
    context.insert("grammar_name", &grammar_name);
  
    self.template.render("context", &context).unwrap()
  }

  fn generate_parser(&self, grammar: &crate::tool::grammar::Grammar, _ast: &dyn CompilationUnitContext) -> String {
    let (first, first_set) = grammar.first_set();
  
    let follow = grammar.follow_set(&first);
  
    let table = grammar.ll1_table(&first_set, &follow);
  
    let table = table.iter().map(|((k1, k2), k3)| (*k1, *k2, *k3)).collect::<Vec<_>>();
    let productions = grammar.productions.iter().map(|(id, production)| {
      return (*id, self.production_generate(production));
    }).collect::<Vec<_>>();
  
  
    let mut sync: HashSet<(usize, usize)> = HashSet::new();
    // 根据 follow 集合来生成 sync
    for (id, followers) in follow.iter() {
      for x in followers.iter() {
        sync.insert((*id, *x));
      }
    }
  
    let sync = sync.iter().cloned().collect::<Vec<_>>();
  
  
  
  
    // 非终结符 0: 编号 1 小写 2 大写 3 pascal
    let mut nonterminals: Vec<NameCaseWithId> = Vec::new();
    for (id, name) in grammar.vocabulary.named_nonterminals.iter() {
      nonterminals.push( NameCaseWithId::new(name, *id));
    }
  
  
    let terminals = grammar.vocabulary.terminals.iter().map(|(id, t)| {
      NameCaseWithId::new(&t, *id)
    }).collect::<Vec<_>>();
  
  
  
  
    let mut context = Context::new();
    context.insert("table", &table);
    context.insert("productions", &productions);
    context.insert("nonterminals", &nonterminals);
    context.insert("terminals", &terminals);
    context.insert("sync_list", &sync);
    let grammar_name = NameCase::new(&grammar.name);
    context.insert("grammar_name", &grammar_name);
  
    let result = self.template.render("parser", &context).unwrap();
  
    
    result
  }

  fn generate_lexer(&self, grammar: &crate::tool::grammar::Grammar, _ast: &dyn CompilationUnitContext) -> String {
    let mut context = Context::new();
  
    let mut lexer_rules = grammar.lexer_rule_map.values().map(|v| {
      LexerCase::new(&v.token_name, v.token_type, &v.regex, v.channel, v.skip)
    }).collect::<Vec<_>>();
  
    // 这里一定要排序
    lexer_rules.sort_by(|a, b| a.token_type.cmp(&b.token_type));
  
    context.insert("lexer_rule_list", &lexer_rules);
    
    let grammar_name = NameCase::new(&grammar.name);
    context.insert("grammar_name", &grammar_name);
    let result = self.template.render("lexer", &context).unwrap();
    result
  }

  // 代码生成并写入对应文件
  fn generate(
    &self, 
    grammar: &crate::tool::grammar::Grammar, 
    ast: &dyn CompilationUnitContext,
    output_dir: &std::path::Path,
    // todo
    _package_name: Option<String>,
    lexer: bool, 
    parser: bool, 
    context: bool,
    listener: bool, 
    visitor: bool, 
    walker: bool,
  )  {

    let name = grammar.name.to_lowercase();
    let mut mod_str = String::new();



    if lexer {
      mod_str += &format!("pub mod {}_lexer;\n", name);
      let path = Path::new(output_dir).join(format!("{}_lexer.rs", name));



      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.generate_lexer(grammar, ast).as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if parser {
      mod_str += &format!("pub mod {}_parser;\n", name);

      let path = Path::new(output_dir).join(format!("{}_parser.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.generate_parser(grammar, ast).as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if context {
      mod_str += &format!("pub mod {}_context;\n", name);

      let path = Path::new(output_dir).join(format!("{}_context.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.generate_context(grammar, ast).as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if listener {
      mod_str += &format!("pub mod {}_listener;\n", name);

      let path = Path::new(output_dir).join(format!("{}_listener.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.generate_listener(grammar, ast).as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if visitor {
      mod_str += &format!("pub mod {}_visitor;\n", name);

      let path = Path::new(output_dir).join(format!("{}_visitor.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.generate_visitor(grammar, ast).as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if walker {
      mod_str += &format!("pub mod {}_walker;\n", name);

      let path = Path::new(output_dir).join(format!("{}_walker.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.generate_walker(grammar, ast).as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }




    let path = Path::new(output_dir).join("mod.rs");
    match File::create(&path) {
      Ok(mut file) => {
        match file.write(mod_str.as_bytes()) {
          Ok(_) => { println!("'{}' generated", path.display()) },
          Err(_) => { println!("fail to write file '{}'", path.display()) },
        }
      },
      Err(_) => { println!("fail to create file '{}'", path.display()) },
    }
  }
}


