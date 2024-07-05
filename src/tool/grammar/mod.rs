pub mod vocabulary;
pub mod lexer_rule;
// pub mod utils;

use std::{collections::{BTreeMap, BTreeSet}, error::Error, fmt::Display};
use chiru::runtime::vocabulary::{NonTerminal, Terminal};
use chiru::runtime::{production::{Production, ProductionItem}, vocabulary::Vocabulary};
use lexer_rule::LexerRule;
use maplit::btreeset;

use crate::tool::visitor::{string_literal_to_token_visitor::StringLiteralToTokenVisitor, lexer_rule_visitor::LexerRuleVisitor, parser_rule_visitor::ParserRuleVisitor, grammar_visitor::GrammarVisitor};


use super::syntaxis::chiru_context::CompilationUnitContext;



#[derive(Clone)]
pub struct Grammar<'a> {
  
  // 文法的名称
  pub name: String,
  
  // 终结符和非终结符
  pub vocabulary: Vocabulary<'a>,
  
  // 所有产生式
  pub productions: BTreeMap<usize, Production<'a>>, 

  // 词法分析规则
  pub lexer_rule_map: BTreeMap<String, LexerRule>,
}

// 定义一个存放 first、follow 集合的数据结构, follow 集一定不会包含 epsilon, first 集合的元素为终结符
// 这个数据结构仅用于存放 first 集合, follow 集合直接使用 BTreeSet
pub struct FirstCollection<'a> {
  pub allow_epsilon: bool,
  pub set: BTreeSet<Terminal<'a>>,
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionTableElement {
  Shift(usize),
  Reduce(usize)
}


impl<'a> Grammar<'a> {
  pub fn new(name: &str) -> Self {    
    Self {
      name: name.to_owned(),
      vocabulary: Vocabulary::new(),
      productions: BTreeMap::new(),
      lexer_rule_map: BTreeMap::new(),
    }
  }

  pub fn from_ast(ast: &dyn CompilationUnitContext) -> Result<Self, Box<dyn Error>> {
    let mut visitor = StringLiteralToTokenVisitor::new(2);
    ast.accept(&mut visitor)?;
    
    let mut lexer_visitor = LexerRuleVisitor::new(visitor.next_token_id, visitor.lexer_rule_map);
    ast.accept(&mut lexer_visitor)?;

    let mut parser_visitor = ParserRuleVisitor::new();
    ast.accept(&mut parser_visitor)?;

    let mut grammar_visitor = GrammarVisitor::new("<no name>", &parser_visitor.parser_rule_map, &lexer_visitor.lexer_rule_map);
    ast.accept(&mut grammar_visitor)?;
    Ok(grammar_visitor.grammar)
  }
  




  // 根据非终结符的first集合求一个串的first集合, 传入参数为非终结符的first集合, 返回结果为串的first集合
  fn get_firstset_for_string(slice: &[ProductionItem], first_set: &BTreeMap<NonTerminal, FirstCollection<'a>>) -> FirstCollection<'a> {
    // 初始化返回结果
    let mut result: FirstCollection = FirstCollection { allow_epsilon: true, set: BTreeSet::new(), };
    
    for item in slice.iter() {
      match item {
        ProductionItem::NonTerminal(nontermimal) => {
          // 如果是非终结符, 先获取到非终结符的first集合.
          let c = first_set.get(&nontermimal).unwrap();
          // 将非终结符的first集合添加到串的first集合中.
          for item in c.set.iter() { result.set.insert(*item) ; }
          if !c.allow_epsilon {
            // 如果非终结符的first集合不包含空串,那么串的first集合也不包含空串.
            result.allow_epsilon = false;
            break;
          }
        },
        ProductionItem::Terminal(terminal) => {
          result.allow_epsilon = false;
          result.set.insert(terminal.clone());
          break;
        },
      }
    }
    
    result
  }

  // 求非 epsilon 产生式的 first 集, production: 待求产生式, result: 求得的结果, firstset: 非终结符的first集合(不断更新)
  fn get_firstset_for_production(production: &Production, result: &mut FirstCollection, first_set: &BTreeMap<NonTerminal, FirstCollection>) -> bool {
    let mut modified = false; // 标识 result 是否被修改

    // 首先判断是否可以为 epsilon 
    let mut allow_epsilon = true;
    // 这个循环的目的是判断是否可以为 epsilon
    for item in production.right.iter() {
      match item {
        ProductionItem::NonTerminal(nonterminal) => {
          let set = first_set.get(nonterminal).unwrap();
          if !set.allow_epsilon {
            allow_epsilon = false;
            break;
          }
        },
        ProductionItem::Terminal(_) => {
          allow_epsilon = false;
          break;
        },
      }
    }

    if result.allow_epsilon != allow_epsilon {
      modified = true; // 标记为已经修改
      result.allow_epsilon = allow_epsilon;
    }

    for item in production.right.iter() {
      match item {
        ProductionItem::NonTerminal(rule_id) => {
          let c = first_set.get(rule_id).unwrap();
          for item in c.set.iter() { modified = result.set.insert(*item) || modified; }

          if ! c.allow_epsilon {
            break;
          }
        },
        ProductionItem::Terminal(token_type) => {
          modified = result.set.insert(token_type.clone()) || modified;
          
          // 遇到终结符就退出
          break;
        },
      }
    }


    modified
  }



  // 返回值 (非终结符的first集合, 产生式的 first 集合)
  pub fn first_set(&self) -> (BTreeMap<NonTerminal, FirstCollection>, BTreeMap<usize, FirstCollection>) {
    // 求 first 集合
    
    // result 为非终结符的 first 集合
    let mut result = BTreeMap::new();  
    
    // 首先将所有非终结符的 first 集合初始化为空，不包含 epsilon。
    for nonterminal in self.vocabulary.get_all_nonterminals().iter() {
      result.insert(*nonterminal, FirstCollection { allow_epsilon: false, set: BTreeSet::new() });
    }

    let mut modified = true;

    // 产生式的 first 集合
    let mut cache: BTreeMap<usize, FirstCollection> = BTreeMap::new();

    // 首先将所有产生式的 first 集合初始化为空，不包含 epsilon。
    for production in self.productions.values() {
      cache.insert(production.id, FirstCollection { allow_epsilon: false, set: BTreeSet::new() });
    }
    


    // 只要有修改就循环
    while modified {
      modified = false;
      
      // 遍历产生式
      for production in self.productions.values() {
        // 不断求每个 production 的 first 集合
        let t = cache.get_mut(&production.id).unwrap();
        
        
        modified = Grammar::get_firstset_for_production(production, t, &result) || modified;

        // 使用产生式的 first 集合来更新非终结符的 first 集合。
        let r = result.get_mut(&production.left).unwrap();
        if t.allow_epsilon && !r.allow_epsilon { // t 可以为空 则 r 一定可以为空，反之，r 可以为空，而 t 不一定可以为空
          r.allow_epsilon = t.allow_epsilon;
          modified = true;
        }

        for item in t.set.iter() { modified = r.set.insert(*item) || modified }
      }
    
    }

    (result, cache)
  }

  // follow 集合不可能包含 ε 返回每个非终结符的 follow 集合
  pub fn follow_set(&self, first_set: &BTreeMap<NonTerminal, FirstCollection>) -> BTreeMap<NonTerminal, BTreeSet<Terminal>> {
    // 求 follow 集合
    let mut result = BTreeMap::new();

    // 将 stop 放入所有非终结符的 follow 集合
    for nonterminal in self.vocabulary.get_all_nonterminals().iter() {
      result.insert(*nonterminal, btreeset! { Terminal::new("_STOP", 1) });
    }

    let mut modified = true;

    while modified {
      modified = false;
      // A -> αBβ 将 first β 加入 follow B ε 除外。
      for production in self.productions.values() {

        for i in 0..production.right.len() {
          if let ProductionItem::NonTerminal(item) = production.right[i] {
            let first = Grammar::get_firstset_for_string(&production.right[(i+1)..], first_set);
            
            // A 的 follow 集合
            let s = result.get(&production.left).unwrap().clone();

            let t = result.get_mut(&item).unwrap();

            for item in first.set.iter() { modified = t.insert(*item) || modified; }
            if first.allow_epsilon {
              // 将 follow A 中的元素都添加到 follow B 中
              for item in s { modified = t.insert(item) || modified; }
            }
          }
        }
      }
    
    }

    
    result


  }

  // 构造预测分析表 这里注意传入的 first 集合是产生式的 first 集合  预测分析表 (非终结符, 终结符) -> 产生式
  pub fn ll1_table(&self, first_set: &BTreeMap<usize, FirstCollection>, follow_set: &BTreeMap<NonTerminal, BTreeSet<Terminal>>) 
    -> BTreeMap<(NonTerminal, Terminal), usize> {
    let mut result: BTreeMap<(NonTerminal, Terminal), usize> = BTreeMap::new();
    let productions = self.productions.values().cloned().collect::<Vec<_>>();


    for production in productions.iter() {
      let first = first_set.get(&production.id).unwrap();
      let rule = production.left;
      
      // 将 first 集合中的所有元素
      for token_type in first.set.iter() {
        if let Some(p) = result.insert((rule, *token_type), production.id) {
          // 这表示不是 ll1 文法
          println!("不是 ll1 文法 {:?}, {}, {}", production, p, rule.id);
        }
      }

      if first.allow_epsilon {
        let follow = follow_set.get(&rule).unwrap();
        for terminal in follow.iter() {
          if let Some(p) = result.insert((rule, *terminal), production.id) {
            println!("不是 ll1 文法 {:?}, {:?}, {}", production, p, rule.id);
          }
        }
      }
    }
    result

  }




  pub fn action_table(&self) -> BTreeMap<(usize, usize), ActionTableElement> {
    todo!()
  }

  // 获取 goto 表 (状态id, 非终结符id) -> 状态id
  pub fn goto_table(&self) -> BTreeMap<(usize, usize), usize> {
    todo!()
  }

}


impl<'a> Display for Grammar<'a> {
  /**
   * 文法名称
   * 所有非终结符以及id。
   * 所有终结符以及id。
   * 所有产生式。
   */
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for production in self.productions.values() {
      write!(f, "{}\n", production);
    }
    Ok(())
  }
}


