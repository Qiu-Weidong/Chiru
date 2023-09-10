pub mod tool;

use std::{fs::File, io::Read};
use std::env;
use chiru::runtime::token_stream::TokenStream;
use clap::Parser;
use std::error::Error;
use tool::{syntaxis::{chiru_lexer::ChiruLexer, chiru_parser::ChiruParser}, grammar::Grammar, code_generator::CodeGenerator};
use tool::cli::Cli;





#[allow(unused_doc_comments)]
fn main() -> Result<(), Box<dyn Error>> {
  todo!()
}



