
use std::error::Error;
use std::collections::{HashMap, HashSet};

use chiru::runtime::error_strategy::error_listener::ErrorListener;
use chiru::runtime::ll1_analyzer::ll1_analyze;

use chiru::maplit::hashmap;
use chiru::maplit::hashset;
use chiru::once_cell::sync::Lazy;

use chiru::runtime::{
  token_stream::TokenStream, 
  error_strategy::error_listener::ConsoleErrorListener,
  production::Production,
  production::ProductionItem
};

use super::arrayinit_context::{
   NumbersContext, CompilationUnitContext,
};


pub struct ArrayInitParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
}


static LL1_TABLE: Lazy<HashMap<(usize, usize), usize>> = Lazy::new(|| { 
  hashmap!{
    
    (1, 4) => 4,
    (2, 5) => 1,
    (3, 5) => 3,
    (0, 2) => 0,
    (3, 3) => 2,
  }
});


static PRODUCTIONS: Lazy<HashMap<usize, Production>>  = Lazy::new(|| {
  hashmap!{
    
    1 => Production::new(1, 2, &vec![ProductionItem::Terminal(5),ProductionItem::Terminal(4),]),
    3 => Production::new(3, 3, &vec![ProductionItem::NonTerminal(2),ProductionItem::NonTerminal(3),]),
    0 => Production::new(0, 0, &vec![ProductionItem::Terminal(2),ProductionItem::NonTerminal(1),ProductionItem::Terminal(3),]),
    4 => Production::new(4, 1, &vec![ProductionItem::Terminal(4),ProductionItem::NonTerminal(3),]),
    2 => Production::new(2, 3, &vec![]),
  }
}); 

// 非终结符
pub static NONTERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    1 => String::from("numbers"),
    0 => String::from("compilation_unit"),
  }
});

// 终结符
pub static TERMINALS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
  hashmap! {
    
    5 => String::from("COMMA"),
    2 => String::from("LBRACKET"),
    1 => String::from("_STOP"),
    8 => String::from("BLOCK_COMMENT"),
    6 => String::from("WHITE_SPACE"),
    3 => String::from("RBRACKET"),
    0 => String::from("_START"),
    4 => String::from("NUM"),
    7 => String::from("LINE_COMMENT"),
  }
});

pub static SYNC: Lazy<HashSet<(usize, usize)>> = Lazy::new(|| {
  hashset! {
    
    (0, 1),
    (2, 3),
    (2, 5),
    (1, 3),
    (3, 3),
  }
});


#[allow(unused)]
impl ArrayInitParser {

  // 使用模板生成 每个非终结符的编号
  
  pub const NUMBERS: usize = 1; 
  pub const COMPILATION_UNIT: usize = 0; 



  pub fn new() -> Self {
    Self {
      error_listeners: vec![Box::new(ConsoleErrorListener::new()),], 
    }
  }


  // 使用模板生成
  
  pub fn numbers(&self, token_stream: &mut TokenStream) -> Result<Box<dyn NumbersContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::NUMBERS,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 
  pub fn compilation_unit(&self, token_stream: &mut TokenStream) -> Result<Box<dyn CompilationUnitContext>, Box<dyn Error>> {

    let result = ll1_analyze(token_stream, Self::COMPILATION_UNIT,
      &LL1_TABLE, &PRODUCTIONS,&NONTERMINALS,&SYNC, &self.error_listeners)?;
    Ok(Box::new(result))
  } 

}






