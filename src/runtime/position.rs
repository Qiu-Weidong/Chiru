
use std::fmt::Display;


#[derive(Clone, Debug, Copy)]
pub struct Position {
  pub line: usize, 
  pub char_position: usize,
}

impl Position {
  pub fn new(line: usize, char_position: usize) -> Self {
    Self { line, char_position, }
  }

  pub fn default_position() -> Self { Self::new(0, 0) }

  pub fn to_string(&self) -> String {
    format!("line: {}, position: {}", self.line, self.char_position)
  }

}



impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "line: {}, position: {}", self.line, self.char_position)
  }
}






