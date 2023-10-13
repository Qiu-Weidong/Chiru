use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Language {
  Rust, 

  TypeScript,

  Ruby, 

  Python,

  // 在这里添加新的语言
}


