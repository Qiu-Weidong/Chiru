use std::path::Path;

use crate::tool::{cli::Language, grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext};



pub trait Target {
  fn get_language(&self) -> Language;

  fn get_reserved_words(&self) -> &[&str];



  fn generate_visitor(&self, grammar: &Grammar, ast: &dyn CompilationUnitContext) -> String;

  fn generate_listener(&self, grammar: &Grammar, ast: &dyn CompilationUnitContext) -> String;

  fn generate_walker(&self, grammar: &Grammar, ast: &dyn CompilationUnitContext) -> String;

  fn generate_context(&self, grammar: &Grammar, ast: &dyn CompilationUnitContext) -> String;

  fn generate_parser(&self, grammar: &Grammar, ast: &dyn CompilationUnitContext) -> String;

  fn generate_lexer(&self, grammar: &Grammar, ast: &dyn CompilationUnitContext) -> String;


  fn generate(
    &self, 
    grammar: &Grammar, 
    ast: &dyn CompilationUnitContext,
    output_dir: &Path,
    package_name: Option<String>,
    lexer: bool, 
    parser: bool, 
    context: bool,
    listener: bool, 
    visitor: bool, 
    walker: bool,
  ) ;


}








// pub struct Target {



//   pub template: Tera,
//   pub language: Language,
//   pub reserved_words: HashSet<&'static str>,

// }


// impl Target {
//   pub fn rust_target() -> Self {  
    
//     let reserved_words = hashset! {
//       // 严格关键字
//       "as", "break", "const", "continue", "crate", "else", "enum",
//       "extern", "false", "fn", "for", "if", "impl", "in", "let",
//       "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
//       "self", "Self", "static", "struct", "super", "trait", "true",
//       "type", "unsafe", "use", "where", "while", "async", "await",
//       "dyn",

//       // 保留关键字
//       "abstract", "become", "box", "do", "final", "macro", "override",
//       "priv", "typeof", "unsized", "virtual", "yield", "try", "union",
//       "'static"

//     };

//     let mut template = Tera::default();
//     // 添加
//     template.add_raw_template("lexer", include_str!("../templates/target/rust2/lexer.tera")).unwrap();
//     template.add_raw_template("parser", include_str!("../templates/target/rust2/parser.tera")).unwrap();
//     template.add_raw_template("context", include_str!("../templates/target/rust2/context.tera")).unwrap();
//     template.add_raw_template("ctx", include_str!("../templates/target/rust2/ctx.tera")).unwrap();
//     template.add_raw_template("listener", include_str!("../templates/target/rust2/listener.tera")).unwrap();
//     template.add_raw_template("visitor", include_str!("../templates/target/rust2/visitor.tera")).unwrap();
//     template.add_raw_template("walker", include_str!("../templates/target/rust2/walker.tera")).unwrap();
//     // template.add_raw_template("production", include_str!("../templates/target/rust2/production.tera")).unwrap();
//     // template.add_raw_template("ll1_table", include_str!("../templates/target/rust2/ll1_table.tera")).unwrap();
//     template.add_raw_template("parser", include_str!("../templates/target/rust2/parser.tera")).unwrap();
//     template.autoescape_on(vec![]);

//     let template = template;
    
//     Target { template, language: Language::Rust, reserved_words }
  
//   }
//   pub fn ruby_target() -> Self { todo!() }
//   pub fn typescript_target() -> Self { todo!() }
//   pub fn python_target() -> Self { todo!() }



//   pub fn new(language: Language) -> Self {
//     match language {
//       Language::Rust => Target::rust_target(),
//       Language::TypeScript => Target::typescript_target(),
//       Language::Ruby => Target::ruby_target(),
//       Language::Python => Target::python_target(),
//     }
//   }



//   pub fn walker_generate(&self, grammar: &Grammar, _ast: &dyn CompilationUnitContext) -> String {
//     let mut context = Context::new();
//     let grammar_name = NameCase::new(&grammar.name);
//     context.insert("grammar_name", &grammar_name);
//     self.template.render("walker", &context).unwrap()
//   }

//   pub fn visitor_generate(&self, grammar: &Grammar, _ast: &dyn CompilationUnitContext) -> String {
//     let nonterminals: Vec<(usize, NameCase)> = grammar.vocabulary.named_nonterminals.iter().map(
//       |(k, v)| {
//         (*k, NameCase::new(v))
//       }
//     ).collect();

//     let mut context = Context::new();

//     let grammar_name = NameCase::new(&grammar.name);
//     context.insert("grammar_name", &grammar_name);
//     context.insert("nonterminals", &nonterminals);
//     self.template.render("visitor", &context).unwrap()
//   }

// }


