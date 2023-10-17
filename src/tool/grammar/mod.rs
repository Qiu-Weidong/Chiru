




pub mod vocabulary;
pub mod lexer_rule;



// 定义一个数据结构来表示文法

use std::{collections::{HashMap, HashSet}, fmt::Display, error::Error};

use chiru::runtime::production::{Production, ProductionItem};
use maplit::hashset;

use crate::tool::visitor::{string_literal_to_token_visitor::StringLiteralToTokenVisitor, lexer_rule_visitor::LexerRuleVisitor, parser_rule_visitor::ParserRuleVisitor, grammar_visitor::GrammarVisitor};

use self::{vocabulary::Vocabulary, lexer_rule::LexerRule};

use super::syntaxis::chiru_context::CompilationUnitContext;




pub struct Grammar {
  
  // 文法的名称
  pub name: String,
  
  pub vocabulary: Vocabulary,
  
  // 所有产生式
  pub productions: HashMap<usize, Production>, 

  pub lexer_rule_map: HashMap<String, LexerRule>,

  // 开始符号 ?
}

// 定义一个存放 first、follow 集合的数据结构, follow 集一定不会包含 epsilon
pub struct Collection {
  pub allow_epsilon: bool,
  pub set: HashSet<usize>,
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionTableElement {
  Shift(usize),
  Reduce(usize)
}


impl Grammar {
  pub fn new(name: &str) -> Self {    
    Self {
      name: name.to_owned(),

      vocabulary: Vocabulary::new(),
      productions: HashMap::new(),
      lexer_rule_map: HashMap::new(),
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
  










  // 根据非终结符的first集合求一个串的first集合
  fn get_first_for_string(slice: &[ProductionItem], first_set: &HashMap<usize, Collection>) -> Collection {
    let mut result = Collection { allow_epsilon: true, set: HashSet::new(), };
    for item in slice.iter() {
      match item {
        ProductionItem::NonTerminal(rule_id) => {
          let c = first_set.get(rule_id).unwrap();
          for item in c.set.iter() { result.set.insert(*item) ; }
          if !c.allow_epsilon {
            result.allow_epsilon = false;
            break;
          }
        },
        ProductionItem::Terminal(token_type) => {
          result.allow_epsilon = false;
          result.set.insert(*token_type);
          break;
        },
      }
    }
    
    result
  }

  // 求非 epsilon 产生式的 first 集
  fn get_first_set_for_non_epsilon_rule(production: &Production, result: &mut Collection, first_set: &HashMap<usize, Collection>) -> bool {
    let mut modified = false; // 标识 result 是否被修改

    // 首先判断是否可以为 epsilon 
    let mut allow_epsilon = true;
    for item in production.right.iter() {
      match item {
        ProductionItem::NonTerminal(id) => {
          let set = first_set.get(id).unwrap();
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
          modified = result.set.insert(*token_type) || modified;
          
          // 遇到终结符就退出
          break;
        },
      }
    }


    modified
  }

  // 返回值 (非终结符的first集合, 产生式的 first 集合)
  pub fn first_set(&self) -> (HashMap<usize, Collection>, HashMap<usize, Collection>) {
    // 求 first 集合
    
    let mut result = HashMap::new();  
    
    // 首先将所有非终结符的 first 集合初始化为空，不包含 epsilon。
    for nonterminal in self.vocabulary.get_all_nonterminals().iter() {
      result.insert(*nonterminal, Collection { allow_epsilon: false, set: HashSet::new() });
    }

    let mut modified = true;
    let mut cache: HashMap<usize, Collection> = HashMap::new();

    // 首先将所有产生式的 first 集合初始化为空，不包含 epsilon。
    for production in self.productions.values() {
      cache.insert(production.id, Collection { allow_epsilon: false, set: HashSet::new() });
    }
    


    // 只要有修改就循环
    while modified {
      modified = false;
      
      for production in self.productions.values() {
        // 不断求每个 production 的 first 集合
        let t = cache.get_mut(&production.id).unwrap();
        
        modified = Grammar::get_first_set_for_non_epsilon_rule(production, t, &result) || modified;

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
  pub fn follow_set(&self, first_set: &HashMap<usize, Collection>) -> HashMap<usize, HashSet<usize>> {
    // 求 follow 集合
    let mut result = HashMap::new();
    for nonterminal in self.vocabulary.get_all_nonterminals().iter() {
      // result.insert(*nonterminal, HashSet::new());
      result.insert(*nonterminal, hashset! { 1 });
    }
    
    // 将 stop 放入开始符号的follow集合, 注意, 我只将 stop 放入了开始符号的 stop 集合, 且默认第一个 rule 是开始符号
    // 可以尝试将 stop 放入所有符号的 follow 集合

    let mut modified = true;

    while modified {
      modified = false;
      // A -> αBβ 将 first β 加入 follow B ε 除外。
      for production in self.productions.values() {

        for i in 0..production.right.len() {
          if let ProductionItem::NonTerminal(item) = production.right[i] {
            let first = Grammar::get_first_for_string(&production.right[(i+1)..], first_set);
            
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
  pub fn ll1_table(&self, first_set: &HashMap<usize, Collection>, follow_set: &HashMap<usize, HashSet<usize>>) 
    -> HashMap<(usize, usize), usize> {
    let mut result: HashMap<(usize, usize), usize> = HashMap::new();
    let productions = self.productions.values().cloned().collect::<Vec<_>>();


    for production in productions.iter() {
      let first = first_set.get(&production.id).unwrap();
      let rule_id = production.left;
      
      // 将 first 集合中的所有元素
      for token_type in first.set.iter() {
        if let Some(p) = result.insert((rule_id, *token_type), production.id) {
          // 这表示不是 ll1 文法
          println!("不是 ll1 文法 {:?}, {}, {}", production, p, rule_id);
        }
      }

      if first.allow_epsilon {
        let follow = follow_set.get(&rule_id).unwrap();
        for token_type in follow.iter() {
          if let Some(p) = result.insert((rule_id, *token_type), production.id) {
            println!("不是 ll1 文法 {:?}, {:?}, {}", production, p, rule_id);
          }
        }
      }
    }
    result

  }




  pub fn action_table(&self) -> HashMap<(usize, usize), ActionTableElement> {
    todo!()
  }

  // 获取 goto 表 (状态id, 非终结符id) -> 状态id
  pub fn goto_table(&self) -> HashMap<(usize, usize), usize> {
    todo!()
  }

}


impl Display for Grammar {
  /**
   * 文法名称
   * 所有非终结符以及id。
   * 所有终结符以及id。
   * 所有产生式。
   */
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for production in self.productions.values() {
      let name = self.vocabulary.get_nonterminal_name_with_default(production.left);

      write!(f, "{} ->", name)?;

      for item in production.right.iter() {
        match item {
          ProductionItem::NonTerminal(id) => {
            write!(f, "{}", self.vocabulary.get_nonterminal_name_with_default(*id))?
          },
          ProductionItem::Terminal(id) => {
            let name = self.vocabulary.get_terminal_name_by_id(*id).unwrap();
            write!(f, " {}", name)?;
          },
        }
      }
      
      write!(f, ";\n")?;
      
    }

    Ok(())

  }
}


