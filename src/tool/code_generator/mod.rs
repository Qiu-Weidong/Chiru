

pub mod target;

// 代码生成器

use std::{collections::HashSet, fs::File, path::Path, io::Write};

use chiru::runtime::production::{Production, ProductionItem};
use tera::{Tera, Context};

use crate::tool::{grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext, visitor::context_visitor::ContextVisitor};

pub struct CodeGenerator<'a> {
  // 解析出来的语法
  grammar: Grammar,

  // 还需要 ast ，持有引用
  ast: &'a dyn CompilationUnitContext,

  // 记录一下需要生成哪些文件, 默认全部
  lexer: bool,
  parser: bool,
  context: bool, 
  listener: bool,
  visitor: bool,
  walker: bool,

  // 代码模板 可以放入 target 中去
  template: Tera,

  // 以及 target
}

impl<'a> CodeGenerator<'a> {
  pub fn new(grammar: Grammar, ast: &'a dyn CompilationUnitContext) -> Self {
    let mut template = Tera::default();
    // 添加
    template.add_raw_template("lexer", include_str!("../templates/target/rust/lexer.tera")).unwrap();
    template.add_raw_template("parser", include_str!("../templates/target/rust/parser.tera")).unwrap();
    template.add_raw_template("context", include_str!("../templates/target/rust/context.tera")).unwrap();
    template.add_raw_template("ctx", include_str!("../templates/target/rust/ctx.tera")).unwrap();
    template.add_raw_template("listener", include_str!("../templates/target/rust/listener.tera")).unwrap();
    template.add_raw_template("visitor", include_str!("../templates/target/rust/visitor.tera")).unwrap();
    template.add_raw_template("walker", include_str!("../templates/target/rust/walker.tera")).unwrap();
    template.add_raw_template("production", include_str!("../templates/target/rust/production.tera")).unwrap();
    template.add_raw_template("ll1_table", include_str!("../templates/target/rust/ll1_table.tera")).unwrap();
    template.add_raw_template("parser", include_str!("../templates/target/rust/parser.tera")).unwrap();
    template.autoescape_on(vec![]);


    Self {
      grammar, ast, 
      lexer: true, parser: true, context: true, listener: true, visitor: true, walker: true,
      template,
    }
  }


  pub fn toggle_lexer_generation(&mut self, flag: bool) { self.lexer = flag; }
  pub fn toggle_parser_generation(&mut self, flag: bool) { self.parser = flag; }
  pub fn toggle_context_generation(&mut self, flag: bool) { self.context = flag; }
  pub fn toggle_listener_generation(&mut self, flag: bool) { self.listener = flag; }
  pub fn toggle_visitor_generation(&mut self, flag: bool) { self.visitor = flag; }
  pub fn toggle_walker_generation(&mut self, flag: bool) { self.walker = flag; }



  // 直接写文件即可
  pub fn generate(&self, base_dir: &Path) {
    let name = self.grammar.name.to_lowercase();
    
    // 生成 mod.rs 文件
    let mut mod_str = String::new();

    if self.lexer {
      mod_str += &format!("pub mod {}_lexer;\n", name);
      let path = Path::new(base_dir).join(format!("{}_lexer.rs", name));


      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.lexer_generate().as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
      
    }

    if self.parser {
      mod_str += &format!("pub mod {}_parser;\n", name);

      let path = Path::new(base_dir).join(format!("{}_parser.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.parser_generate().as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
      
    }

    if self.context {
      mod_str += &format!("pub mod {}_context;\n", name);

      let path = Path::new(base_dir).join(format!("{}_context.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.context_generate().as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
      
    }

    if self.listener {
      mod_str += &format!("pub mod {}_listener;\n", name);

      let path = Path::new(base_dir).join(format!("{}_listener.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.listener_generate().as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
      
    }

    if self.visitor {
      mod_str += &format!("pub mod {}_visitor;\n", name);

      let path = Path::new(base_dir).join(format!("{}_visitor.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.visitor_generate().as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
      
    }

    if self.walker {
      mod_str += &format!("pub mod {}_walker;\n", name);

      let path = Path::new(base_dir).join(format!("{}_walker.rs", name));
      match  File::create(&path) {
        Ok(mut file) => {
          match file.write(self.walker_generate().as_bytes()) {
            Ok(_) => { println!("'{}' generated", path.display()) },
            Err(_) => { println!("fail to write file '{}'", path.display()) },
          }
        },
        Err(_) => { println!("fail to create file '{}'", path.display()) },
      }
    }



    // 最后写 mod.rs 文件
    let path = Path::new(base_dir).join("mod.rs");
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






  // 私有函数
  fn pascal(&self, input: &str) -> String {
    let mut result = String::new();
  
    input.split("_").for_each(|word| {
      let mut chars: Vec<char> = word.chars().collect();
      if chars.len() >= 1 {
        chars[0] = chars[0].to_ascii_uppercase();
        let s: String = chars.into_iter().collect();
        result += &s;
      }
      
    });
    result
  }
  
  fn ctx_gen(&self, rule_id: usize, ctx: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>)) -> String {
    let rule_name = self.grammar.vocabulary.get_nonterminal_name_by_id(rule_id).unwrap();
  
  
    // 首先将 HashSet 转换为 vec
    let terminal_list = ctx.0.iter().map(|id| {
      let name = self.grammar.vocabulary.get_terminal_name_by_id(*id).unwrap();
      (name.to_lowercase(), name.to_uppercase(), self.pascal(&name))
    }).collect::<Vec<_>>();
  
    let terminal = ctx.1.iter().map(|id| {
      let name = self.grammar.vocabulary.get_terminal_name_by_id(*id).unwrap();
      (name.to_lowercase(), name.to_uppercase(), self.pascal(&name))
    }).collect::<Vec<_>>();
  
    let nonterminal_list = ctx.2.iter().map(|id| {
      let name = self.grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
      (name.to_lowercase(), name.to_uppercase(), self.pascal(&name))
    }).collect::<Vec<_>>();
  
    let nonterminal = ctx.3.iter().map(|id| {
      let name = self.grammar.vocabulary.get_nonterminal_name_by_id(*id).unwrap();
      (name.to_lowercase(), name.to_uppercase(), self.pascal(&name))
    }).collect::<Vec<_>>();
  
  
  
  
  
  
  
    let mut context = Context::new();
    context.insert("grammar_name", &(self.grammar.name.to_lowercase(), self.grammar.name.to_uppercase(), self.pascal(&self.grammar.name)));
    context.insert("ctx_name", &(rule_name.to_lowercase(), rule_name.to_uppercase(), self.pascal(&rule_name)));
    context.insert("nonterminal_list", &nonterminal_list);
    context.insert("terminal_list", &terminal_list);
    context.insert("nonterminal", &nonterminal);
    context.insert("terminal", &terminal);
  
  
  
  
    self.template.render("ctx", &context).unwrap()
  }
  fn production_right_generate(&self, production: &Production) -> String {
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
    result
  }
  
  



  pub fn context_generate(&self) -> String {
  
    // 获取所有的终结符和非终结符
    let terminals = self.grammar.vocabulary.get_all_terminals_map();
  
    let nonterminals = self.grammar.vocabulary.get_all_named_nonterminals_map();
  
  
  
  
    // 首先解析 ast 获取 table
    let mut visitor = ContextVisitor::new(nonterminals, terminals);
    self.ast.accept(&mut visitor).unwrap();
  
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
        
        self.ctx_gen( *id, c) 
      }).collect::<Vec<String>>();
    let mut context = Context::new();
    context.insert("ctx_list", &ctx_list);
    context.insert("grammar_name", &(self.grammar.name.to_lowercase(), self.grammar.name.to_uppercase(), self.pascal(&self.grammar.name)));
  
    self.template.render("context", &context).unwrap()
  }
  
  pub fn lexer_generate(&self) -> String {
    let mut context = Context::new();
  
    let mut lexer_rules = self.grammar.lexer_rule_map.values().map(|v| {
      (
        v.token_name.to_lowercase(), 
        v.token_name.to_uppercase(), 
        self.pascal(&v.token_name),
        v.token_type,
        v.regex.clone(),
        v.channel, 
        v.skip,
      )
    }).collect::<Vec<_>>();
  
    // 这里一定要排序
    lexer_rules.sort_by(|a, b| a.3.cmp(&b.3));
  
    context.insert("lexer_rule_list", &lexer_rules);
    context.insert("grammar_name", &(self.grammar.name.to_lowercase(), self.grammar.name.to_uppercase(), self.pascal(&self.grammar.name)));
  
  
  
    let result = self.template.render("lexer", &context).unwrap();
    result
  }

  pub fn listener_generate(&self) -> String {

    let mut nonterminals: Vec<(usize, String, String, String)> = Vec::new();
    for (id, name) in self.grammar.vocabulary.named_nonterminals.iter() {
      // if let Some(name) = &t.name {
      nonterminals.push((*id, name.clone(), name.to_uppercase(), self.pascal(&name)));
      // }
    }
  
    let mut context = Context::new();
    context.insert("nonterminals", &nonterminals);
    context.insert("grammar_name", &(self.grammar.name.to_lowercase(), self.grammar.name.to_uppercase(), self.pascal(&self.grammar.name)));
    self.template.render("listener", &context).unwrap()
  
  }

  pub fn parser_generate(&self) -> String {
  
    let (first, first_set) = self.grammar.first_set();
  
    let follow = self.grammar.follow_set(&first);
  
    let table = self.grammar.ll1_table(&first_set, &follow);
  
    let table = table.iter().map(|((k1, k2), k3)| (*k1, *k2, *k3)).collect::<Vec<_>>();
    let productions = self.grammar.productions.iter().map(|(id, production)| {
      // 求出 production 的 right 对应的字符串
      return (*id, production.left, self.production_right_generate(production));
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
    let mut nonterminals: Vec<(usize, String, String, String)> = Vec::new();
    for (id, name) in self.grammar.vocabulary.named_nonterminals.iter() {
      // if let Some(name) = &t.name {
      nonterminals.push((*id, name.clone(), name.to_uppercase(), self.pascal(&name)));
      // }
    }
  
  
    let terminals = self.grammar.vocabulary.terminals.iter().map(|(id, t)| {
      (*id, t.clone())
    }).collect::<Vec<_>>();
  
  
  
  
    let mut context = Context::new();
    context.insert("table", &table);
    context.insert("productions", &productions);
    context.insert("nonterminals", &nonterminals);
    context.insert("terminals", &terminals);
    context.insert("sync_list", &sync);
    context.insert("grammar_name", &(self.grammar.name.to_lowercase(), self.grammar.name.to_uppercase(), self.pascal(&self.grammar.name)));
  
    let result = self.template.render("parser", &context).unwrap();
  
    
    result
  }
  
  pub fn visitor_generate(&self) -> String {



    let mut nonterminals: Vec<(usize, String, String, String)> = Vec::new();
    for (id, name) in self.grammar.vocabulary.named_nonterminals.iter() {
      // if let Some(name) = &t.name {
      nonterminals.push((*id, name.clone(), name.to_uppercase(), self.pascal(&name)));
      // }
    }
  
    let mut context = Context::new();
    context.insert("nonterminals", &nonterminals);
    context.insert("grammar_name", &(self.grammar.name.to_lowercase(), self.grammar.name.to_uppercase(), self.pascal(&self.grammar.name)));
    self.template.render("visitor", &context).unwrap()
  
  
  }
  
  pub fn walker_generate(&self) -> String {
    let mut context = Context::new();
    context.insert("grammar_name", &(self.grammar.name.to_lowercase(), self.grammar.name.to_uppercase(), self.pascal(&self.grammar.name)));
    self.template.render("walker", &context).unwrap()
  }
}




