
pub mod tool;
use clap::Parser;
use tool::cli::Cli;

fn main() {
  
  let cli = Cli::parse();
  
  cli.execute_command().unwrap();
}




