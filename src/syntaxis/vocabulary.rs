

pub struct Vocabulary {
  pub literal_names: Vec<String>,
  pub symbolic_names: Vec<String>,
  pub display_names: Vec<String>,
}

impl Vocabulary {
  pub fn get_literal_name(&self, token_type: usize) -> &str {
    &self.literal_names[token_type]
  }

  pub fn get_symbolic_name(&self, token_type: usize) -> &str {
    &self.symbolic_names[token_type]
  }

  pub fn get_display_name(&self, token_type: usize) -> &str {
    &self.display_names[token_type]
  }
}




