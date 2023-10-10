
use std::{error::Error, path::Path, fs::File, io::Write};

use chiru::runtime::production::{Production, ProductionItem};
use tera::{Tera, Context};

use crate::tool::code_generator::name_case::{VisitorOrListenerGenData, WalkerGenData, ContextGenData, ParserGenData, LexerGenData, WriteFileData};
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



  fn generate_visitor(&self, data: &VisitorOrListenerGenData) -> Result<String, Box<dyn Error>> {
    // let mut nonterminals: Vec<NameCaseWithId> = Vec::new();
    // for (id, name) in grammar.vocabulary.named_nonterminals.iter() {
    //   nonterminals.push( NameCaseWithId::new(name, *id));
    // }
  
    


    let mut context = Context::new();

    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);

    context.insert("nonterminals", &data.rule_names);
    let result = self.template.render("visitor", &context).unwrap();
    Ok(result)
  }

  fn generate_listener(&self, data: &VisitorOrListenerGenData) -> Result<String, Box<dyn Error>> {
    // let mut nonterminals: Vec<NameCaseWithId> = Vec::new();
    // for (id, name) in grammar.vocabulary.named_nonterminals.iter() {
    //   nonterminals.push( NameCaseWithId::new(name, *id));
    // }
  
    let mut context = Context::new();
    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);


    context.insert("rule_names", &data.rule_names);
    let result = self.template.render("listener", &context).unwrap();
    Ok(result)
  }

  fn generate_walker(&self, data: &WalkerGenData) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);

    let result =  self.template.render("walker", &context).unwrap();
    Ok(result)
  }

  fn generate_context(&self, data: &ContextGenData) -> Result<String, Box<dyn Error>> {
    // 获取所有的终结符和非终结符
    // let terminals = grammar.vocabulary.get_all_terminals_map();
  
    // let nonterminals = grammar.vocabulary.get_all_named_nonterminals_map();
  
  
  
  
    // // 首先解析 ast 获取 table
    // let mut visitor = ContextVisitor::new(nonterminals, terminals);
    // ast.accept(&mut visitor).unwrap();
  
    // let table = visitor.table;
    // let nonterminals = grammar.vocabulary.get_all_named_nonterminals();
  
    // nonterminals.iter().for_each(|x| {
    //   if ! table.contains_key(x) {
    //     println!("{} {}", x, grammar.vocabulary.get_nonterminal_name_by_id(*x).unwrap())
    //   }
    // });
  
  
  
    // let ctx_list = nonterminals.iter()
    //   .map(|id| { 
    //     let c = table.get(id).unwrap().clone();

    //     let rule_name = grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
    //     let terminal_list = c.0.iter().map(|id| {
    //       let name = grammar.vocabulary.get_terminal_name_by_id(*id).unwrap();
    //       NameCaseWithId::new(&name, *id)
    //     }).collect::<Vec<_>>();
      
    //     let terminal = c.1.iter().map(|id| {
    //       let name = grammar.vocabulary.get_terminal_name_by_id(*id).unwrap();
    //       NameCaseWithId::new(&name, *id)
    //     }).collect::<Vec<_>>();
      
    //     let nonterminal_list = c.2.iter().map(|id| {
    //       let name = grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
    //       NameCaseWithId::new(&name, *id)
    //     }).collect::<Vec<_>>();
      
    //     let nonterminal = c.3.iter().map(|id| {
    //       let name = grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
    //       NameCaseWithId::new(&name, *id)
    //     }).collect::<Vec<_>>();

    //     ContextCase::new(&rule_name, terminal_list, terminal, nonterminal_list, nonterminal)
    //     // self.ctx_gen( *id, c, grammar) 
    //   }).collect::<Vec<ContextCase>>();

    let mut context = Context::new();
    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);



    context.insert("context_list", &data.context_list);
  
    let result = self.template.render("context", &context).unwrap();
    Ok(result)
  }

  fn generate_parser(&self, data: &ParserGenData) -> Result<String, Box<dyn Error>> {
    let productions = data.grammar.productions.iter().map(|(id, production)| {
      return (*id, self.production_generate(production));
    }).collect::<Vec<_>>();




    // let (first, first_set) = grammar.first_set();
  
    // let follow = grammar.follow_set(&first);
  
    // let table = grammar.ll1_table(&first_set, &follow);

  
    let table = data.table.iter().map(|((k1, k2), k3)| (*k1, *k2, *k3)).collect::<Vec<_>>();
    
    
  
  
    // let mut sync: HashSet<(usize, usize)> = HashSet::new();
    // // 根据 follow 集合来生成 sync
    // for (id, followers) in follow.iter() {
    //   for x in followers.iter() {
    //     sync.insert((*id, *x));
    //   }
    // }
  
    // let mut nonterminals: Vec<NameCaseWithId> = Vec::new();
    // for (id, name) in grammar.vocabulary.named_nonterminals.iter() {
    //   nonterminals.push( NameCaseWithId::new(name, *id));
    // }
  
  
    // let terminals = grammar.vocabulary.terminals.iter().map(|(id, t)| {
    //   NameCaseWithId::new(&t, *id)
    // }).collect::<Vec<_>>();
  
  
  
  
    let mut context = Context::new();


    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);




    context.insert("table", &table);
    context.insert("productions", &productions);
    context.insert("rule_names", &data.rule_names);
    context.insert("terminal_names", &data.terminal_names);
    context.insert("sync_list", &data.sync_list);
  
    let result = self.template.render("parser", &context).unwrap();
  
    
    Ok(result)
  }

  fn generate_lexer(&self, data: &LexerGenData) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
  
    // let mut lexer_rules = grammar.lexer_rule_map.values().map(|v| {
    //   LexerCase::new(&v.token_name, v.token_type, &v.regex, v.channel, v.skip)
    // }).collect::<Vec<_>>();
  
    // // 这里一定要排序
    // lexer_rules.sort_by(|a, b| a.token_type.cmp(&b.token_type));
    // let grammar_name = NameCase::new(&grammar.name);




    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);
    context.insert("lexer_rule_list", &data.lexer_rule_list);

    let result = self.template.render("lexer", &context).unwrap();
    Ok(result)
  }

  fn write_file(&self, data: &WriteFileData) {
    // rust 需要一个 mod.rs 文件

    // 首先生成文件头
    let mut context = Context::new();
    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    let mut mod_str = self.template.render("header", &context).unwrap();

    if let Some(lexer) = &data.lexer {
      let path = Path::new(data.output_dir).join(format!("{}_lexer.rs", data.grammar_name.snake_case));
      mod_str += &format!("pub mod {}_lexer;\n", data.grammar_name.snake_case);
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(lexer.as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }
    
    if let Some(parser) = &data.parser {
      mod_str += &format!("pub mod {}_parser;\n", data.grammar_name.snake_case);

      let path = Path::new(data.output_dir).join(format!("{}_parser.rs", data.grammar_name.snake_case));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(parser.as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if let Some(context) = &data.context {
      mod_str += &format!("pub mod {}_context;\n", data.grammar_name.snake_case);

      let path = Path::new(data.output_dir).join(format!("{}_context.rs", data.grammar_name.snake_case));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(context.as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if let Some(listener) = &data.listener {
      mod_str += &format!("pub mod {}_listener;\n", data.grammar_name.snake_case);

      let path = Path::new(data.output_dir).join(format!("{}_listener.rs", data.grammar_name.snake_case));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(listener.as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if let Some(visitor) = &data.visitor {
      mod_str += &format!("pub mod {}_visitor;\n", data.grammar_name.snake_case);

      let path = Path::new(data.output_dir).join(format!("{}_visitor.rs", data.grammar_name.snake_case));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(visitor.as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }

    if let Some(walker) = &data.walker {
      mod_str += &format!("pub mod {}_walker;\n", data.grammar_name.snake_case);

      let path = Path::new(data.output_dir).join(format!("{}_walker.rs", data.grammar_name.snake_case));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(walker.as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }




    let path = Path::new(data.output_dir).join("mod.rs");
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


