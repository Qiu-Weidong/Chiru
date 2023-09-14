use std::collections::{HashMap, HashSet};

use tera::Tera;

use crate::tool::cli::Language;



pub struct Target {
  pub template: Tera,
  pub char_value_escape: HashMap<char, String>,
  pub language: Language,
  pub reserved_words: HashSet<String>,

}


impl Target {
  pub fn rust_target() -> Self {  todo!()  }
  pub fn ruby_target() -> Self { todo!() }
  pub fn typescript_target() -> Self { todo!() }
  pub fn python_target() -> Self { todo!() }
}


