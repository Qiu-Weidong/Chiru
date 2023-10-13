


use std::collections::{HashMap, HashSet};


/**
 * 文法由终结符、非终结符、产生式和开始符号组成
 * Vocabulay 用于管理终结符和非终结符
 */
pub struct Vocabulary {


  // 根据非终结符的名称来查询其id, 只包含命名非终结符
  non_terminal_name_to_id: HashMap<String, usize>,

  // 根据终结符的名称来查询其id
  terminal_name_to_id: HashMap<String, usize>,







  // 所有命名非终结符
  pub named_nonterminals: HashMap<usize, String>,

  // 所有终结符
  pub terminals: HashMap<usize, String>,

  // 所有非终结符, 包括未命名
  pub nonterminals: HashSet<usize>,
  

}

impl Vocabulary {

  // 根据非终结符的名称获取其id
  pub fn get_nonterminal_name_by_id(&self, id: usize) -> Option<String> {
    self.named_nonterminals.get(&id).cloned()
  }

  pub fn get_terminal_name_by_id(&self, id: usize) -> Option<String> {
    self.terminals.get(&id).cloned()
  }




  pub fn get_nonterminal_id_by_name(&self, name: &str) -> Option<usize> {
    self.non_terminal_name_to_id.get(name).cloned()
  }

  pub fn get_terminal_id_by_name(&self, name: &str) -> Option<usize> {
    self.terminal_name_to_id.get(name).cloned()
  }



  
  pub fn is_nonterminal_id(&self, id: usize) -> bool {  
    self.nonterminals.contains(&id)
  } 

  pub fn is_terminal_id(&self, id: usize) -> bool {
    self.terminals.contains_key(&id)
  }

  pub fn get_nonterminal_name_with_default(&self, id: usize) -> String {
    match self.named_nonterminals.get(&id) {
      Some(val) => val.clone(),
      None => id.to_string(),
    }
  }

  pub fn is_terminal_name_defined(&self, name: &str) -> bool {
    self.terminal_name_to_id.contains_key(name)
  }

  pub fn is_nonterminal_name_defined(&self, name: &str) -> bool {
    self.non_terminal_name_to_id.contains_key(name)
  }

  pub fn add_terminal(&mut self, id: usize, name: &str) -> bool {
    // 添加终结符
    if self.terminal_name_to_id.contains_key(name) || self.terminals.contains_key(&id) { return false }

    self.terminals.insert(id, name.to_owned());
    self.terminal_name_to_id.insert(name.to_owned(), id);
    true
  }

  pub fn add_named_nonterminal(&mut self, id: usize, name: &str) -> bool {  
    // 添加非终结符
    if self.non_terminal_name_to_id.contains_key(name) || self.nonterminals.contains(&id) { return  false; }

    self.named_nonterminals.insert(id, name.to_owned());
    self.nonterminals.insert(id);
    self.non_terminal_name_to_id.insert(name.to_owned(), id);
    true
  }
  
  pub fn add_unnamed_nonterminal(&mut self, id: usize) -> bool {
    if self.nonterminals.contains(&id) { return false; }
    self.nonterminals.insert(id);
    true
  }

  pub fn new() -> Self {
    let mut result = Self {
      non_terminal_name_to_id: HashMap::new(), 
      terminal_name_to_id: HashMap::new(), 
      nonterminals: HashSet::new(), 
      terminals: HashMap::new(),
      named_nonterminals: HashMap::new(),
    };

    result.add_terminal(0, "_START");
    result.add_terminal(1, "_STOP");
    result
  }


  pub fn get_all_named_nonterminals(&self) -> Vec<usize> {
    self.non_terminal_name_to_id.iter().map(|(_, v)| *v).collect()
  }

  pub fn get_all_nonterminals(&self) -> Vec<usize> {
    self.nonterminals.iter().map(|id| { *id }).collect()
  }

  pub fn get_all_terminals(&self) -> Vec<usize> {
    self.terminals.iter().map(|(k, _v)| { *k}).collect()
  }

  pub fn get_all_terminal_names(&self) -> Vec<String> {
    self.terminals.iter().map(|(_, v)| { v.to_owned() }).collect()
  }

  pub fn get_all_nonterminal_names(&self) -> Vec<String> {
    self.non_terminal_name_to_id.iter().map(|(name, _)| { name.to_owned() }).collect()
  }



  pub fn get_all_named_nonterminals_map(&self) -> HashMap<String, usize> {
    self.non_terminal_name_to_id.clone()
  }

  pub fn get_all_terminals_map(&self) -> HashMap<String, usize> {
    self.terminal_name_to_id.clone()
  }

}




