
use std::{error::Error, path::Path, fs::File, io::Write};

use chiru::runtime::production::{Production, ProductionItem};
use tera::{Tera, Context};

use crate::tool::code_generator::{language::Language, name_case::{ContextGenData, LexerGenData, ParserGenData, VisitorOrListenerGenData, VocabularyGenData, WalkerGenData, WriteFileData}};
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
    template.add_raw_template("vocabulary", include_str!("../../templates/target/rust/vocabulary.tera")).unwrap();
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
          result += &format!("ProductionItem::NonTerminal({}),", id.id);
        },
        ProductionItem::Terminal(id) => {
          result += &format!("ProductionItem::Terminal({}),", id.id);
        },
      }
    }
    result += "]";
    
    format!("Production::new({}, {}, &{})", production.id, production.left.id, result)
  }
  
}


impl Target for RustTarget {
  fn get_language(&self) -> Language {
    Language::Rust
  }

  fn get_reserved_words(&self) -> &[&str] {
    &self.reserved_words
  }



  fn generate_visitor(&self, data: &VisitorOrListenerGenData) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();

    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);

    context.insert("nonterminals", &data.rule_names);
    let result = self.template.render("visitor", &context)?;
    Ok(result)
  }

  fn generate_listener(&self, data: &VisitorOrListenerGenData) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);


    context.insert("rule_names", &data.rule_names);
    let result = self.template.render("listener", &context)?;
    Ok(result)
  }

  fn generate_walker(&self, data: &WalkerGenData) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);

    let result =  self.template.render("walker", &context)?;
    Ok(result)
  }

  fn generate_context(&self, data: &ContextGenData) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);

    context.insert("context_list", &data.context_list);
  
    let result = self.template.render("context", &context)?;
    Ok(result)
  }

  fn generate_parser(&self, data: &ParserGenData) -> Result<String, Box<dyn Error>> {
    let productions = data.grammar.productions.iter().map(|(id, production)| {
      return (*id, self.production_generate(production));
    }).collect::<Vec<_>>();
    let table = data.table.iter().map(|((k1, k2), k3)| (*k1, *k2, *k3)).collect::<Vec<_>>();
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
  
    let result = self.template.render("parser", &context)?;
  
    
    Ok(result)
  }

  fn generate_lexer(&self, data: &LexerGenData) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();

    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    context.insert("grammar_name", &data.grammar_name);
    context.insert("package_name", &data.package_name);
    context.insert("lexer_rule_list", &data.lexer_rule_list);

    let result = self.template.render("lexer", &context)?;
    Ok(result)
  }

  fn generate_vocabulary(&self, data: &VocabularyGenData) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("terminal_names", &data.terminal_names);
    context.insert("rule_names", &data.rule_names);
    context.insert("unnamed_nonterminals", &data.unnamed_nonterminal_ids);

    let result = self.template.render("vocabulary", &context)?;
    Ok(result)
  }

  fn write_file(&self, data: &WriteFileData) {
    // rust 需要一个 mod.rs 文件

    // 首先生成文件头
    let mut context = Context::new();
    context.insert("grammar_file_name", &data.grammar_file_name);
    context.insert("version", &data.version);
    let mut mod_str = match self.template.render("header", &context) {
      Ok(content) => content,
      Err(_) => String::from("// error while generate header.\n")
    };

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


