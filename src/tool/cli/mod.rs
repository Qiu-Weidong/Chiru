use std::path::PathBuf;

use clap::Parser;
use clap::ValueEnum;



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

  #[arg(short, long, value_name = "FILE", required = true)]
  pub input: PathBuf,

  #[arg(short, long, value_name = "LANGUAGE", value_enum, default_value_t = Language::Rust)]
  pub language: Language,

  #[arg(short, long, value_name = "OUTPUTDIR")]
  pub output_dir: Option<PathBuf>,

  #[arg(short, long, value_name = "PACKAGE")]
  pub package_name: Option<String>,

  #[arg(long, default_value_t = false)]
  pub gui: bool, 

  #[arg(long, default_value_t = false)]
  pub tokens: bool,
  
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

}



#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Language {
  Rust, 

  TypeScript,

  Ruby, 

  Python,
}
