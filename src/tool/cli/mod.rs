use std::path::PathBuf;

use clap::Parser;
use clap::ValueEnum;



#[derive(Parser)]
#[command(author, version, about, long_about = None, next_line_help = true)]
pub struct Cli {

  #[arg(short, long, value_name = "FILE", required = true)]
  pub input: PathBuf,

  #[arg(short, long, value_name = "OUTPUT")]
  pub output: Option<PathBuf>,



  #[arg(short, long, value_name = "LANGUAGE", value_enum, default_value_t = Language::Rust)]
  pub language: Language,

  #[arg(short, long, value_name = "PACKAGE")]
  pub package_name: Option<String>,

  #[arg(long, default_value_t = true)]
  pub visitor: bool,

  #[arg(long, default_value_t = true)]
  pub listener: bool,

  #[arg(long, default_value_t = true)]
  pub walker: bool,

  #[arg(long, default_value_t = false)]
  pub no_visitor: bool,

  #[arg(long, default_value_t = false)]
  pub no_listener: bool,

  #[arg(long, default_value_t = false)]
  pub no_walker: bool,






  #[arg(long, default_value_t = false)]
  pub gui: bool, 

  #[arg(long)]
  pub start_rule: Option<String>,

  #[arg(long)]
  pub test_file: Option<PathBuf>,


  #[arg(long, default_value_t = false)]
  pub tokens: bool,
  


  #[arg(long, default_value_t = false)]
  pub string_ast: bool,


  

}



#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Language {
  Rust, 

  TypeScript,

  Ruby, 

  Python,
}
