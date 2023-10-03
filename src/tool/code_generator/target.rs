use std::{collections::HashSet, vec};

use maplit::hashset;
// use maplit::hashmap;
use tera::{Tera, Context};

use crate::tool::{cli::Language, grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext};



pub struct Target {



  pub template: Tera,
  pub language: Language,
  pub reserved_words: HashSet<&'static str>,

}


#[derive(serde::Serialize)]
struct Identify {
  
  // 全大写, 用下划线连接
  pub screaming_snake_case: String,

  // 每个单词的首字母大写，单词直接连接在一起，没有分隔符
  pub pascal_case: String,

  // 第一个单词的首字母小写，后续单词的首字母大写，
  // 单词直接连接在一起，没有分隔符
  // 如 firstName
  pub camel_case: String,

  // 全小写, 用下划线连接
  pub snake_case: String,
}

impl Identify {
  fn lowercase(input: &str, index: usize) -> String {
    // 将字符串的 index 个字符小写
    let mut chars: Vec<char> = input.chars().collect();
    if chars.len() <= index {
      input.to_owned()
    }
    else {
      chars[index] = chars[index].to_ascii_lowercase();
      chars.into_iter().collect()
    }
  }

  fn uppercase(input: &str, index: usize) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    if chars.len() <= index {
      input.to_owned()
    }
    else {
      chars[index] = chars[index].to_ascii_uppercase();
      chars.into_iter().collect()
    }
  }


  pub fn new(name: &str) -> Self {
    // 首先将 name 拆分为一串单词
    if name.contains("_") {
      let words = name.split("_").map(|word| {
        word.to_ascii_lowercase()
      }).collect::<Vec<_>>();

      Self::from_slice(&words)
    } else {
      // 首先将首字母小写
      let mut current_word = String::new();
      let mut words: Vec<String> = Vec::new();

      for c in name.chars() {
        if c.is_uppercase() {
          if !current_word.is_empty() {
            words.push(current_word.clone());
            current_word.clear();
          }

          current_word.push(c.to_ascii_lowercase());
        }
        else {
          current_word.push(c);
        }
      }


      if ! current_word.is_empty() {
        words.push(current_word);
      }
      Self::from_slice(&words)
    }
    
  }

  pub fn from_slice(slice: &[String]) -> Self {
    // 首先构造 pascal 命名
    let mut pascal_case = String::new();
    let mut snake_case = String::new();


    for s in slice.iter() {
      pascal_case += &Self::uppercase(&s, 0);
      snake_case += s;
      snake_case += "_";
    }
    
    let camel_case = Self::lowercase(&pascal_case, 0);
    if snake_case.len() > 0 {
      snake_case = snake_case[..snake_case.len()-1].to_owned();
    }

    let screaming_snake_case = snake_case.to_ascii_uppercase();
    
    
    Self { screaming_snake_case, pascal_case, camel_case, snake_case }
  }
}



impl Target {
  pub fn rust_target() -> Self {  
    
    let reserved_words = hashset! {
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

    let mut template = Tera::default();
    // 添加
    template.add_raw_template("lexer", include_str!("../templates/target/rust2/lexer.tera")).unwrap();
    template.add_raw_template("parser", include_str!("../templates/target/rust2/parser.tera")).unwrap();
    template.add_raw_template("context", include_str!("../templates/target/rust2/context.tera")).unwrap();
    template.add_raw_template("ctx", include_str!("../templates/target/rust2/ctx.tera")).unwrap();
    template.add_raw_template("listener", include_str!("../templates/target/rust2/listener.tera")).unwrap();
    template.add_raw_template("visitor", include_str!("../templates/target/rust2/visitor.tera")).unwrap();
    template.add_raw_template("walker", include_str!("../templates/target/rust2/walker.tera")).unwrap();
    template.add_raw_template("production", include_str!("../templates/target/rust2/production.tera")).unwrap();
    template.add_raw_template("ll1_table", include_str!("../templates/target/rust2/ll1_table.tera")).unwrap();
    template.add_raw_template("parser", include_str!("../templates/target/rust2/parser.tera")).unwrap();
    template.autoescape_on(vec![]);

    let template = template;
    
    Target { template, language: Language::Rust, reserved_words }
  
  }
  pub fn ruby_target() -> Self { todo!() }
  pub fn typescript_target() -> Self { todo!() }
  pub fn python_target() -> Self { todo!() }



  pub fn new(language: Language) -> Self {
    match language {
      Language::Rust => Target::rust_target(),
      Language::TypeScript => Target::typescript_target(),
      Language::Ruby => Target::ruby_target(),
      Language::Python => Target::python_target(),
    }
  }



  pub fn walker_generate(&self, grammar: &Grammar, _ast: &dyn CompilationUnitContext) -> String {
    let mut context = Context::new();
    let grammar_name = Identify::new(&grammar.name);
    context.insert("grammar_name", &grammar_name);
    self.template.render("walker", &context).unwrap()
  }

  pub fn visitor_generate(&self, grammar: &Grammar, _ast: &dyn CompilationUnitContext) -> String {
    let nonterminals: Vec<(usize, Identify)> = grammar.vocabulary.named_nonterminals.iter().map(
      |(k, v)| {
        (*k, Identify::new(v))
      }
    ).collect();

    let mut context = Context::new();

    let grammar_name = Identify::new(&grammar.name);
    context.insert("grammar_name", &grammar_name);
    context.insert("nonterminals", &nonterminals);
    self.template.render("visitor", &context).unwrap()
  }

}


