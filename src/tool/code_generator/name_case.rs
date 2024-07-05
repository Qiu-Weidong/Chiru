use std::{collections::{HashMap, HashSet}, path::Path};

use crate::tool::{grammar::Grammar, syntaxis::chiru_context::CompilationUnitContext};




#[derive(serde::Serialize, Clone)]
pub struct NameCase {
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

  // 原始名称
  pub origin_case: String,
}

impl NameCase {
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

  fn contains_lowercase(input: &str) -> bool {
    for c in input.chars() {
      if c.is_lowercase() {
        return true;
      }
    }
    return false;
  }

  pub fn new(name: &str) -> Self {
    // 首先将 name 拆分为一串单词
    if name.contains("_") {
      let words = name.split("_").map(|word| {
        word.to_ascii_lowercase()
      }).collect::<Vec<_>>();

      
      Self::from_slice(&words, name)
    } else if Self::contains_lowercase(name) {
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
      Self::from_slice(&words, name)
    } else {
      // 完全大写
      let screaming_snake_case = name.to_ascii_uppercase();
      let snake_case = name.to_ascii_lowercase();
      let pascal_case = Self::uppercase(&snake_case, 0);
    
      Self {
        screaming_snake_case,
        camel_case: snake_case.clone(),
        snake_case,
        pascal_case,
        origin_case: name.to_owned(),
      }
    }
    
  }

  pub fn from_slice(slice: &[String], origin_case: &str) -> Self {
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
    
    
    Self { screaming_snake_case, pascal_case, camel_case, snake_case, origin_case: origin_case.to_owned(), }
  }
}


// 定义一些用于传值的结构体
#[derive(serde::Serialize, Clone)]
pub struct NameCaseWithId {
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

  pub origin_case: String,


  // 编号
  pub id: usize,
}


impl NameCaseWithId {
  pub fn new(name: &str, id: usize) -> Self {
    let case = NameCase::new(name);

    Self {
      origin_case: case.origin_case,
      screaming_snake_case: case.screaming_snake_case,
      pascal_case: case.pascal_case,
      camel_case: case.camel_case,
      snake_case: case.snake_case,
      id,
    }
  }
}


#[derive(serde::Serialize, Clone)]
pub struct LexerCase {
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

  pub token_name: String,

  pub token_type: usize,

  pub regex: String,

  pub channel: usize,

  pub skip: bool,
}

impl LexerCase {
  pub fn new(token_name: &str, token_type: usize, regex: &str, channel: usize, skip: bool) -> Self {
    let case = NameCase::new(token_name);
    
    Self {
      screaming_snake_case: case.screaming_snake_case,
      token_name: token_name.to_owned(),
      snake_case: case.snake_case,
      camel_case: case.camel_case,
      pascal_case: case.pascal_case,
      token_type,
      regex: regex.to_owned(),
      channel, skip,
    }
  }
}



#[derive(serde::Serialize, Clone)]
pub struct ContextCase {
  pub screaming_snake_case: String,

  // 每个单词的首字母大写，单词直接连接在一起，没有分隔符
  pub pascal_case: String,

  // 第一个单词的首字母小写，后续单词的首字母大写，
  // 单词直接连接在一起，没有分隔符
  // 如 firstName
  pub camel_case: String,

  // 全小写, 用下划线连接
  pub snake_case: String,

  pub origin_case: String,



  pub terminal_list: Vec<NameCaseWithId>, 
  pub terminal: Vec<NameCaseWithId>,
  pub nonterminal_list: Vec<NameCaseWithId>,
  pub nonterminal: Vec<NameCaseWithId>,
}
impl ContextCase {
  pub fn new(
    ctx_name: &str, terminal_list: Vec<NameCaseWithId>, terminal: Vec<NameCaseWithId>,nonterminal_list: Vec<NameCaseWithId>,
    nonterminal: Vec<NameCaseWithId>,
  ) -> Self {
    let case = NameCase::new(ctx_name);
    Self {
      origin_case: case.origin_case,
      screaming_snake_case: case.screaming_snake_case,
      pascal_case: case.pascal_case,
      camel_case: case.camel_case,
      snake_case: case.snake_case,
      terminal, terminal_list, nonterminal, nonterminal_list,
    }
  }
}





pub struct VisitorOrListenerGenData<'a> {
  pub grammar_file_name: String, 
  pub version: String, 
  pub package_name: Option<NameCase>,
  pub grammar_name: NameCase,

  pub rule_names: Vec<NameCaseWithId>,

  pub grammar: &'a Grammar,
  pub ast: &'a dyn CompilationUnitContext,
}

impl<'a> VisitorOrListenerGenData<'a> {
  pub fn new(grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext, grammar_file_name: &str, version: &str, package_name: Option<&str>, grammar_name: &str, rule_names: &[NameCaseWithId]) -> Self {
    let package_name: Option<NameCase> = if let Some(name) = package_name {
      Some(NameCase::new(name))
    } else { None };
    let grammar_name = NameCase::new(grammar_name);
    let rule_names = rule_names.iter().cloned().collect::<Vec<_>>();
    
    Self {
      grammar_file_name: grammar_file_name.to_owned(),
      version: version.to_owned(),
      package_name, grammar_name,
      rule_names, grammar, ast,
    }
  }
}

pub struct ContextGenData<'a> {
  pub grammar_file_name: String, 
  pub version: String, 
  pub package_name: Option<NameCase>,
  pub grammar_name: NameCase,

  pub context_list: Vec<ContextCase>,

  pub grammar: &'a Grammar,
  pub ast: &'a dyn CompilationUnitContext,
}

impl<'a> ContextGenData<'a>  {
  pub fn new(grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext,grammar_file_name: &str, version: &str, package_name: Option<&str>, grammar_name: &str, context_list: &[ContextCase]) -> Self {
    let package_name: Option<NameCase> = if let Some(name) = package_name {
      Some(NameCase::new(name))
    } else { None };
    let grammar_name = NameCase::new(grammar_name);
    let context_list = context_list.iter().cloned().collect::<Vec<_>>();
    Self {
      grammar_file_name: grammar_file_name.to_owned(),
      version: version.to_owned(),
      package_name, grammar_name,
      context_list, grammar, ast,
    }
  }
}

pub struct WalkerGenData<'a> {
  pub grammar_file_name: String, 
  pub version: String, 
  pub package_name: Option<NameCase>,
  pub grammar_name: NameCase,

  pub grammar: &'a Grammar,
  pub ast: &'a dyn CompilationUnitContext,
}

impl<'a> WalkerGenData<'a> {
  pub fn new(grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext,grammar_file_name: &str, version: &str, package_name: Option<&str>, grammar_name: &str) -> Self {
    let package_name: Option<NameCase> = if let Some(name) = package_name {
      Some(NameCase::new(name))
    } else { None };
    let grammar_name = NameCase::new(grammar_name);
    
    Self {
      grammar_file_name: grammar_file_name.to_owned(),
      version: version.to_owned(),
      package_name, grammar_name,
      grammar, ast,
    }
  }
}

pub struct LexerGenData<'a> {
  pub grammar_file_name: String, 
  pub version: String, 
  pub package_name: Option<NameCase>,
  pub grammar_name: NameCase,


  pub lexer_rule_list: Vec<LexerCase>,
  pub grammar: &'a Grammar,
  pub ast: &'a dyn CompilationUnitContext,
  
}

impl<'a> LexerGenData<'a> {
  pub fn new(grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext, grammar_file_name: &str, version: &str, package_name: Option<&str>, grammar_name: &str, lexer_rule_list: &[LexerCase]) -> Self {
    let package_name: Option<NameCase> = if let Some(name) = package_name {
      Some(NameCase::new(name))
    } else { None };
    let grammar_name = NameCase::new(grammar_name);
    let lexer_rule_list = lexer_rule_list.iter().cloned().collect::<Vec<_>>();

    
    Self {
      grammar_file_name: grammar_file_name.to_owned(),
      version: version.to_owned(),
      package_name, grammar_name, lexer_rule_list,
      grammar, ast,
    }
  }
}

pub struct ParserGenData<'a> {
  pub grammar_file_name: String, 
  pub version: String, 
  pub package_name: Option<NameCase>,
  pub grammar_name: NameCase,


  pub table: HashMap<(usize, usize), usize>,
  pub rule_names: Vec<NameCaseWithId>,
  pub terminal_names: Vec<NameCaseWithId>,
  pub sync_list: HashSet<(usize, usize)>,

  pub grammar: &'a Grammar,
  pub ast: &'a dyn CompilationUnitContext,
}

impl<'a> ParserGenData<'a> {
  pub fn new(grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext, 
    grammar_file_name: &str, version: &str, 
    package_name: Option<&str>, grammar_name: &str, 
    rule_names: &[NameCaseWithId],
    table: &HashMap<(usize, usize), usize>, terminal_names: &[NameCaseWithId], sync_list: &HashSet<(usize, usize)>
  ) -> Self {
    let package_name: Option<NameCase> = if let Some(name) = package_name {
      Some(NameCase::new(name))
    } else { None };
    let grammar_name = NameCase::new(grammar_name);
    let rule_names = rule_names.iter().cloned().collect::<Vec<_>>();
    let terminal_names = terminal_names.iter().cloned().collect::<Vec<_>>();

    Self {
      grammar_file_name: grammar_file_name.to_owned(),
      version: version.to_owned(),
      package_name, grammar_name,
      rule_names, table: table.clone(), sync_list: sync_list.clone(),terminal_names,
      grammar, ast,
    }
  }
}

pub struct VocabularyGenData {
  pub rule_names: Vec<NameCaseWithId>,
  pub terminal_names: Vec<NameCaseWithId>,
  pub unnamed_nonterminal_ids: Vec<usize>
}

impl<'a> VocabularyGenData {
  pub fn new(grammar: &'a Grammar) -> Self {
    let terminal_names: Vec<_> = grammar.vocabulary.terminals.iter().map(|(id, name)| NameCaseWithId::new(name, *id)).collect();
    let rule_names: Vec<NameCaseWithId> = grammar.vocabulary.named_nonterminals.iter().map(|(k, v)| {
      NameCaseWithId::new(v, *k)
    }).collect::<Vec<_>>();

    let unnamed_nonterminal_ids = grammar.vocabulary.get_all_nonterminals();
    
    Self {
      terminal_names, rule_names, unnamed_nonterminal_ids
    }
  }
}

pub struct WriteFileData<'a> {
  pub grammar: &'a Grammar,
  pub ast: &'a dyn CompilationUnitContext,

  pub grammar_file_name: String, 
  pub version: String, 
  pub package_name: Option<NameCase>,
  pub grammar_name: NameCase,

  pub lexer: Option<String>,
  pub parser: Option<String>,
  pub context: Option<String>,
  pub visitor: Option<String>,
  pub listener: Option<String>,
  pub walker: Option<String>,

  pub output_dir: &'a Path,
}

impl<'a> WriteFileData<'a> {
  pub fn new(
    grammar: &'a Grammar, ast: &'a dyn CompilationUnitContext, 
    grammar_file_name: &str, version: &str, 
    package_name: Option<&str>, grammar_name: &str,
    output_dir: &'a Path,
    lexer: Option<String>, parser: Option<String>, context: Option<String>, visitor: Option<String>, listener: Option<String>, walker: Option<String>, 
  ) -> Self {
    let package_name: Option<NameCase> = if let Some(name) = package_name {
      Some(NameCase::new(name))
    } else { None };
    let grammar_name = NameCase::new(grammar_name);

    Self {
      grammar, ast, grammar_file_name: grammar_file_name.to_owned(),version: version.to_owned(),package_name, grammar_name,
      lexer, parser, context, visitor, listener, walker, output_dir,
    }
  }
}
