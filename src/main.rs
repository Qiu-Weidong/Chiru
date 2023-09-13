pub mod tool;
use clap::Parser;
use tool::cli::Cli;





#[allow(unused_doc_comments)]
fn main() {
  
  let cli = Cli::parse();
  
  cli.execute_command().unwrap();
}



