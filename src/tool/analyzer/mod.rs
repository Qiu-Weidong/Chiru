use std::{collections::{HashMap, HashSet}, error::Error};

use chiru::runtime::{error_strategy::error_listener::{ErrorListener, ConsoleErrorListener}, lexer_rule::LexerRule, lexer::{Lexer, TokenIter}, production::Production, token_stream::TokenStream, ast::rule_context::RuleContext, ll1_analyzer::ll1_analyze};

use super::grammar::Grammar;



pub struct CommonLexer<'a> {
  pub input: &'a str,
  pub error_listeners: Vec<Box<dyn ErrorListener>>,

  pub rules: Vec<LexerRule>,
}

impl Lexer for CommonLexer<'_> {
  fn iter(&self) -> chiru::runtime::lexer::TokenIter {
    TokenIter::new(self.input, &self.rules, &self.error_listeners)
  }
}


impl<'a> CommonLexer<'a> {
  pub fn from_grammar(grammar: &Grammar, input: &'a str) -> Self {
    let mut rules = grammar.lexer_rule_map.values().map(|v| {
      let re = regex::Regex::new(r####"(^r#*")|("#*$)"####).unwrap();
      let c = re.replace_all(&v.regex, "");
      let rule = regex::Regex::new(&c).unwrap();
      return LexerRule {
        channel: v.channel,
        token_type: v.token_type,
        token_name: v.token_name.to_owned(),
        skip: v.skip,
        rule
      };

    }).collect::<Vec<_>>();
  
    rules.sort_by(|a, b| a.token_type.cmp(&b.token_type));
    
    Self {
      input, 
      rules,
      error_listeners: vec![Box::new(ConsoleErrorListener::new())],
    }
  }
}



pub struct CommonParser {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
  pub table: HashMap<(usize, usize), usize>,
  pub productions: HashMap<usize, Production>,
  pub nonterminals: HashMap<usize, String>,
  // pub terminals: HashMap<usize, String>,
  pub sync: HashSet<(usize, usize)>,
}

impl CommonParser {
  pub fn parse(&self, token_stream: &mut TokenStream, rule_index: usize) -> Result<RuleContext, Box<dyn Error>> {
    if token_stream.peek_next_token()?.token_type == 0 {
      token_stream.consume()?;
    }

    ll1_analyze(token_stream, rule_index, &self.table, &self.productions, &self.nonterminals, &self.sync, &self.error_listeners)
  }



  pub fn from_grammar(grammar: &Grammar) -> Self {
    let (first, first_set) = grammar.first_set();
  
    let follow = grammar.follow_set(&first);
  
    let table = grammar.ll1_table(&first_set, &follow);
  

    let productions = grammar.productions.clone();

    let mut sync: HashSet<(usize, usize)> = HashSet::new();
    // 根据 follow 集合来生成 sync
    for (id, followers) in follow.iter() {
      for x in followers.iter() {
        sync.insert((*id, *x));
      }
    }

    let nonterminals = grammar.vocabulary.named_nonterminals.clone();


    Self {
      table, productions, sync, nonterminals, error_listeners: vec![Box::new(ConsoleErrorListener::new())],
    }
  }
}













