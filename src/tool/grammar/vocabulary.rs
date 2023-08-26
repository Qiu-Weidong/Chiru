


use std::collections::HashMap;

use super::{terminal::Terminal, non_terminal::NonTerminal};


/**
 * 文法由终结符、非终结符、产生式和开始符号组成
 * Vocabulay 用于管理终结符和非终结符
 */
pub struct Vocabulary {
  // 根据非终结符的名称来查询其id, 只包含命名非终结符
  non_terminal_name_to_id: HashMap<String, usize>,
  // 所有非终结符，包括未命名
  pub nonterminals: HashMap<usize, NonTerminal>,

  // 根据终结符的名称来查询其id
  terminal_name_to_id: HashMap<String, usize>,
  // 所有终结符
  pub terminals: HashMap<usize, Terminal>,
}

impl Vocabulary {

  // 根据非终结符的名称获取其id
  pub fn get_nonterminal_name_by_id(&self, id: usize) -> Option<String> {
    match self.nonterminals.get(&id) {
      Some(val) => val.name.clone(),
      None => None,
    }
  }

  pub fn get_terminal_name_by_id(&self, id: usize) -> Option<String> {
    match self.terminals.get(&id) {
      Some(val) => Some(val.name.clone()),
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
    self.nonterminals.contains_key(&id)
  } 

  pub fn is_terminal_id(&self, id: usize) -> bool {
    self.terminals.contains_key(&id)
  }

  pub fn get_nonterminal_name_with_default(&self, id: usize) -> String {
    match self.nonterminals.get(&id) {
      Some(val) => {
        match &val.name {
          Some(name) => name.clone(),
          None => id.to_string(),
        }
      },
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

    let termianl = Terminal::new(id, name);
    self.terminals.insert(id, termianl);
    self.terminal_name_to_id.insert(name.to_owned(), id);
    true
  }

  pub fn add_named_nonterminal(&mut self, id: usize, name: &str) -> bool {  
    // 添加非终结符
    if self.non_terminal_name_to_id.contains_key(name) || self.nonterminals.contains_key(&id) { return  false; }

    let nonterminal = NonTerminal { id, name: Some(name.to_owned()) };
    self.nonterminals.insert(id, nonterminal);
    self.non_terminal_name_to_id.insert(name.to_owned(), id);
    true
  }
  
  pub fn add_unnamed_nonterminal(&mut self, id: usize) -> bool {
    if self.nonterminals.contains_key(&id) { return false; }
    let nonterminal = NonTerminal { id, name: None };
    self.nonterminals.insert(id, nonterminal);
    true
  }

  pub fn new() -> Self {
    let mut result = Self {
      non_terminal_name_to_id: HashMap::new(), 
      terminal_name_to_id: HashMap::new(), nonterminals: HashMap::new(), terminals: HashMap::new(),
    };

    result.add_terminal(0, "_START");
    result.add_terminal(1, "_STOP");
    result
  }


  pub fn get_all_terminals(&self) -> Vec<Terminal> {
    self.terminals.values().cloned().collect::<Vec<_>>()
  }
  pub fn get_all_nonterminals(&self) -> Vec<NonTerminal> {
    self.nonterminals.values().cloned().collect::<Vec<_>>()
  }

  pub fn get_all_named_nonterminals(&self) -> Vec<usize> {
    self.non_terminal_name_to_id.iter().map(|(_, v)| *v).collect()
  }

}




