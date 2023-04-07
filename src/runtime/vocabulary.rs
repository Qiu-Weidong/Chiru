use std::collections::{HashMap, HashSet};



pub struct Vocabulary {
  // 根据非终结符的名称来查询其id
  non_terminal_name_to_id: HashMap<String, usize>,

  // 根据非终结符的 id 来查询名称
  non_terminal_id_to_name: HashMap<usize, String>,

  // 根据非终结符的名称来查询其id
  terminal_name_to_id: HashMap<String, usize>,

  // 根据非终结符的 id 来查询名称
  terminal_id_to_name: HashMap<usize, String>,

  // 所有非终结符，包括未命名
  nonterminals: HashSet<usize>,

  // 所有终结符
  terminals: HashSet<usize>,
}

impl Vocabulary {
  pub fn get_nonterminal_name_by_id(&self, id: usize) -> Option<String> {
    match self.non_terminal_id_to_name.get(&id) {
      Some(name) => Some(name.clone()),
      None => None,
    }
  }

  pub fn get_terminal_name_by_id(&self, id: usize) -> Option<String> {
    match self.terminal_id_to_name.get(&id) {
      Some(name) => Some(name.clone()),
      None => None,
    }
  }

  pub fn get_nonterminal_id_by_name(&self, name: &str) -> Option<usize> {
    match self.non_terminal_name_to_id.get(name) {
      Some(id) => Some(*id),
      None => None,
    }
  }

  pub fn get_terminal_id_by_name(&self, name: &str) -> Option<usize> {
    match self.terminal_name_to_id.get(name) {
      Some(id) => Some(*id),
      None => None,
    }
  }

  pub fn is_nonterminal_id(&self, id: usize) -> bool {  
    self.nonterminals.contains(&id)
  } 

  pub fn is_terminal_id(&self, id: usize) -> bool {
    self.terminals.contains(&id)
  }

  pub fn get_nonterminal_name_with_default(&self, id: usize) -> String {
    match self.non_terminal_id_to_name.get(&id) {
      Some(name) => name.clone(),
      None => id.to_string(),
    }
  }

  pub fn get_terminal_name(&self, id: usize) -> String {
    self.terminal_id_to_name.get(&id).unwrap().clone()
  }

  pub fn is_terminal_name_defined(&self, name: &str) -> bool {
    self.terminal_name_to_id.contains_key(name)
  }

  pub fn is_nonterminal_name_defined(&self, name: &str) -> bool {
    self.non_terminal_name_to_id.contains_key(name)
  }

  pub fn add_terminal(&mut self, id: usize, name: &str) -> bool {
    if self.terminal_name_to_id.contains_key(name) || ! self.terminals.insert(id) { return false; }
    self.terminal_name_to_id.insert(name.to_owned(), id);
    self.terminal_id_to_name.insert(id, name.to_owned());
    true
  }

  pub fn add_named_nonterminal(&mut self, id: usize, name: &str) -> bool {  
    if self.non_terminal_name_to_id.contains_key(name) || self.nonterminals.contains(&id) { return false; }
    self.nonterminals.insert(id);
    self.non_terminal_name_to_id.insert(name.to_owned(), id);
    self.non_terminal_id_to_name.insert(id, name.to_owned());
    true
  }
  pub fn add_unnamed_nonterminal(&mut self, id: usize) -> bool {
    if self.non_terminal_id_to_name.contains_key(&id) || self.nonterminals.contains(&id) { return false; }
    self.nonterminals.insert(id);
    true
  }

  pub fn new() -> Self {
    let mut result = Self {
      non_terminal_id_to_name: HashMap::new(), non_terminal_name_to_id: HashMap::new(), terminal_id_to_name: HashMap::new(),
      terminal_name_to_id: HashMap::new(), nonterminals: HashSet::new(), terminals: HashSet::new(),
    };

    result.add_terminal(0, "_START");
    result.add_terminal(1, "_STOP");
    result
  }


  pub fn get_all_terminals(&self) -> &HashSet<usize> {
    &self.terminals
  }
  pub fn get_all_nonterminals(&self) -> &HashSet<usize> {
    &self.nonterminals
  }
}




