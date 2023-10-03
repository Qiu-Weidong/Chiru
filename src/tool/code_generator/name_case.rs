



#[derive(serde::Serialize)]
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
#[derive(serde::Serialize)]
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


#[derive(serde::Serialize)]
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



#[derive(serde::Serialize)]
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



