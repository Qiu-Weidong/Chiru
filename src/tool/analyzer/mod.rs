use std::{collections::{BTreeMap, HashMap, HashSet}, error::Error};

use chiru::runtime::{ast::rule_context::RuleContext, error_strategy::error_listener::{ConsoleErrorListener, ErrorListener}, lexer::{Lexer, TokenIter}, lexer_rule::LexerRule, ll1_analyzer::ll1_analyze, production::Production, token_stream::TokenStream, vocabulary::{NonTerminal, Terminal}};

use super::grammar::Grammar;



pub struct CommonLexer<'a> {
  pub input: &'a str,
  pub error_listeners: Vec<Box<dyn ErrorListener>>,

  // 只需要自定义词法分析规则即可
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



pub struct CommonParser<'a> {
  pub error_listeners: Vec<Box<dyn ErrorListener>>,
  pub table: BTreeMap<(NonTerminal<'a>, Terminal<'a>), usize>,
  pub productions: HashMap<usize, Production<'a>>,
  pub nonterminals: HashMap<usize, String>,
  pub sync: HashSet<(usize, usize)>,
}

impl<'a> CommonParser<'a> {
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

    let mut sync: HashSet<(NonTerminal, Terminal)> = HashSet::new();
    // 根据 follow 集合来生成 sync
    for (id, followers) in follow.iter() {
      for x in followers.iter() {
        sync.insert((*id, *x));
      }
    }

    let nonterminals = grammar.vocabulary.nonterminals.iter().map(|item| {
      (item.id, item.name.clone())
    }).collect();



    // 生成 action 和 goto 表格, 有了这两个表格, 就可以进行 lalr 分析了
    Self {
      table, productions, sync, nonterminals, error_listeners: vec![Box::new(ConsoleErrorListener::new())],
    }
  }
}













