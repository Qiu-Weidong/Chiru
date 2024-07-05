use std::collections::{BTreeMap, BTreeSet};

use once_cell::sync::Lazy;
use serde::Serialize;



#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Copy)]
pub struct Terminal<'a> {
  pub name: &'a str,
  pub id: usize,
}

impl<'a> Terminal<'a> {
  pub fn new(name: &'a str, id: usize) -> Self {
    Self { name, id }
  }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Copy, Serialize)]
pub struct NonTerminal<'a> {
  pub name: Option<&'a str>,
  pub id: usize,
}

impl<'a> NonTerminal<'a> {
  pub fn new(name: Option<&'a str>, id: usize) -> Self {
    Self { name, id }
  }
}


#[allow(unused)]
#[derive(Clone)]
pub struct Vocabulary<'a> {
  // 终结符和非终结符
  pub terminals: BTreeSet<Terminal<'a>>,
  pub nonterminals: BTreeSet<NonTerminal<'a>>,

  // 查询map
  nonterminal_name_map: BTreeMap<String, NonTerminal<'a>>,
  nonterminal_id_map: BTreeMap<usize, NonTerminal<'a>>,

  terminal_name_map: BTreeMap<String, Terminal<'a>>,
  terminal_id_map: BTreeMap<usize, Terminal<'a>>,
}

impl<'a> Vocabulary<'a> {
  pub fn new() -> Self {
    Self {
      terminals: BTreeSet::new(),
      nonterminals: BTreeSet::new(),

      nonterminal_id_map: BTreeMap::new(),
      terminal_id_map: BTreeMap::new(),

      terminal_name_map: BTreeMap::new(),
      nonterminal_name_map: BTreeMap::new(),
    }
  }

  pub fn add_terminal(&mut self, id: usize, name: &'a str) {
    self.terminals.insert(Terminal::new(name, id));
    self.terminal_id_map.insert(id, Terminal::new(name, id));
    self.terminal_name_map.insert(name.to_owned(), Terminal::new(name, id));
  }

  pub fn add_named_nonterminal(&mut self, id: usize, name: &'a str) {
    self.add_nonterminal(id, Some(name))
  }

  pub fn add_unnamed_nonterminal(&mut self, id: usize) {
    self.add_nonterminal(id, None)
  }

  pub fn add_unnamed_nonterminals(&mut self, unnamed_terminal_id_list: &[usize]) {
    unnamed_terminal_id_list.iter().for_each(|id| self.add_nonterminal(*id, None))
  }

  pub fn add_nonterminal(&mut self, id: usize, name: Option<&'a str>) {
    self.nonterminals.insert(NonTerminal::new(name, id));
    self.nonterminal_id_map.insert(id, NonTerminal::new(name, id));
    if let Some(name) = name {
      self.nonterminal_name_map.insert(name.to_owned(), NonTerminal::new(Some(name), id));
    }
  }

  pub fn is_terminal_name_defined(&self, name: &str) -> bool {
    self.terminal_name_map.contains_key(name)
  }

  pub fn is_nonterminal_name_defined(&self, name: &str) -> bool {
    self.nonterminal_name_map.contains_key(name)
  }

  pub fn is_nonterminal_id(&self, id: usize) -> bool {
    self.nonterminal_id_map.contains_key(&id)
  }

  pub fn is_terminal_id(&self, id: usize) -> bool {
    self.terminal_id_map.contains_key(&id)
  }

  pub fn get_nonterminal_by_id(&self, id: usize) -> Option<NonTerminal<'a>> {
    self.nonterminal_id_map.get(&id).cloned()
  }

  pub fn get_nonterminal_by_name(&self, name: &str) -> Option<NonTerminal<'a>> {
    self.nonterminal_name_map.get(name).cloned()
  }

  pub fn get_terminal_by_id(&self, id: usize) -> Option<Terminal<'a>> {
    self.terminal_id_map.get(&id).cloned()
  }

  pub fn get_terminal_by_name(&self, name: &str) -> Option<Terminal<'a>> {
    self.terminal_name_map.get(name).cloned()
  }

  pub fn get_all_named_nonterminals(&self) -> Vec<NonTerminal<'a>> {
    let result = self.nonterminals.iter().filter(|item| item.name != None).cloned().collect();
    result
  }

  pub fn get_all_nonterminals(&self) -> Vec<NonTerminal<'a>> {
    self.nonterminals.iter().cloned().collect()
  }

  pub fn get_all_terminals(&self) -> Vec<Terminal<'a>> {
    self.terminals.iter().cloned().collect()
  }

  pub fn get_all_terminal_names(&self) -> Vec<String> {
    self.terminal_name_map.keys().cloned().collect()
  }

  pub fn get_all_nonterminal_names(&self) -> Vec<String> {
    self.nonterminal_name_map.keys().cloned().collect()
  }

}


pub static VOCABULARY: Lazy<Vocabulary> = Lazy::new(|| {
  let mut result = Vocabulary { 
    terminals: BTreeSet::new(), 
    nonterminals: BTreeSet::new(), 
    nonterminal_name_map: BTreeMap::new(), 
    nonterminal_id_map: BTreeMap::new(), 
    terminal_name_map: BTreeMap::new(), 
    terminal_id_map: BTreeMap::new(),
  };

  // 添加匿名非终结符
  result.add_unnamed_nonterminals(&vec![7, 17, 13, 4, 5, 6, 10, 1, 3, 16, 18, 11, 9, 15, 0, 8, 27, 21, 19, 26, 28, 24, 23, 22, 14, 20, 12, 2, 25]);

  // 添加命名非终结符
  result.add_named_nonterminal(4, "block");
  result.add_named_nonterminal(13, "attribute");
  result.add_named_nonterminal(9, "lexer_rule");
  result.add_named_nonterminal(0, "compilation_unit");
  result.add_named_nonterminal(11, "annotation");
  result.add_named_nonterminal(7, "element");
  result.add_named_nonterminal(1, "grammar_name");
  result.add_named_nonterminal(5, "alternative");
  result.add_named_nonterminal(12, "attributes");
  result.add_named_nonterminal(2, "rules");
  result.add_named_nonterminal(10, "regular");
  result.add_named_nonterminal(3, "parser_rule");
  result.add_named_nonterminal(6, "epsilon");
  result.add_named_nonterminal(8, "ebnf_suffix");

  // 添加终结符
  
  result.add_terminal(10 ,"STAR");
  result.add_terminal(13 ,"LPAREN");
  result.add_terminal(6 ,"SEMI");
  result.add_terminal(0 ,"_START");
  result.add_terminal(5 ,"COLON");
  result.add_terminal(7 ,"COMMA");
  result.add_terminal(9 ,"EPSILON");
  result.add_terminal(20 ,"REGULAR_LITERAL");
  result.add_terminal(2 ,"GRAMMAR");
  result.add_terminal(15 ,"AT");
  result.add_terminal(18 ,"RBRACKET");
  result.add_terminal(4 ,"TOKEN_REF");
  result.add_terminal(22 ,"LINE_COMMENT");
  result.add_terminal(21 ,"WHITE_SPACE");
  result.add_terminal(1 ,"_STOP");
  result.add_terminal(17 ,"LBRACKET");
  result.add_terminal(23 ,"BLOCK_COMMENT");
  result.add_terminal(3 ,"RULE_REF");
  result.add_terminal(14 ,"RPAREN");
  result.add_terminal(19 ,"STRING_LITERAL");
  result.add_terminal(11 ,"PLUS");
  result.add_terminal(16 ,"SHARP");
  result.add_terminal(8 ,"OR");
  result.add_terminal(12 ,"QUESTION");


  result
});



