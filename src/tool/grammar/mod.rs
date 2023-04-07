// 定义一个数据结构来表示文法

use std::{collections::{HashMap, HashSet}, fmt::Display, rc::Rc};

use crate::runtime::vocabulary::Vocabulary;

pub struct Grammar {
  
  // 文法的名称
  pub name: String,
  
  pub vocabulary: Vocabulary,
  
  // 所有产生式
  pub productions: HashSet<Production>, 
}

// 定义一个存放 first、follow 集合的数据结构
pub struct Collection {
  pub allow_epsilon: bool,
  pub set: HashSet<usize>,
}

impl Grammar {
  pub fn new(name: &str) -> Self {    
    Self {
      name: name.to_owned(),
      vocabulary: Vocabulary::new(),
      productions: HashSet::new(),
    }
  }

  fn get_first_for_string(slice: &[ProductionItem], first_set: &HashMap<usize, Collection>) -> Collection {
    // 根据非终结符的first集合求一个串的first集合
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

  // 同时返回每条产生式的 first 集合
  pub fn first_set(&self) -> (HashMap<usize, Collection>, HashMap<Production, Collection>) {
    // 求 first 集合
    
    let mut result = HashMap::new();  
    // 首先将所有产生式的 first 集合初始化为空，不包含 epsilon。

    for id in self.vocabulary.get_all_nonterminals().iter() {
      result.insert(*id, Collection { allow_epsilon: false, set: HashSet::new() });
    }

    let mut modified = true;
    let mut cache: HashMap<Production, Collection> = HashMap::new();
    for production in self.productions.iter() {
      cache.insert(production.clone(), Collection { allow_epsilon: false, set: HashSet::new() });
    }
    


    // 只要有修改就循环
    while modified {
      modified = false;
      
      for production in self.productions.iter() {
        // 不断求每个 production 的 first 集合
        let t = cache.get_mut(production).unwrap();
        
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

  // follow 集合不可能包含 ε
  pub fn follow_set(&self, first_set: &HashMap<usize, Collection>) -> HashMap<usize, HashSet<usize>> {
    // 求 follow 集合
    let mut result = HashMap::new();
    for id in self.vocabulary.get_all_nonterminals().iter() {
      result.insert(*id, HashSet::new());
    }
    
    // 将 stop 放入开始符号的follow集合
    result.get_mut(&0).unwrap().insert(1);

    let mut modified = true;

    while modified {
      modified = false;
      // A -> αBβ 将 first β 加入 follow B ε 除外。
      for production in self.productions.iter() {

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

  // 构造预测分析表
  pub fn ll1_table(&self, first_set: &HashMap<Production, Collection>, follow_set: &HashMap<usize, HashSet<usize>>) -> HashMap<(usize, usize), Rc<Production>> {
    let mut result: HashMap<(usize, usize), Rc<Production>> = HashMap::new();
    let productions: Vec<Rc<Production>> = self.productions.iter().map(|p| { Rc::new(p.clone()) }).collect();
    for production in productions.iter() {
      let first = first_set.get(&production).unwrap();
      let rule_id = production.left;
      
      // 将 first 集合中的所有元素
      for token_type in first.set.iter() {
        if let Some(p) = result.insert((rule_id, *token_type), Rc::clone(production)) {
          println!("{:?}, {:?}", production, p);
        }
      }

      if first.allow_epsilon {
        let follow = follow_set.get(&rule_id).unwrap();
        for token_type in follow.iter() {
          if let Some(p) = result.insert((rule_id, *token_type), Rc::clone(production)) {
            println!("{:?}, {:?}", production, p);
          }
        }
      }
    }
    result
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
    for production in self.productions.iter() {
      let name = self.vocabulary.get_nonterminal_name_with_default(production.left);

      write!(f, "{} ->", name)?;

      for item in production.right.iter() {
        match item {
          ProductionItem::NonTerminal(id) => {
            write!(f, "{}", self.vocabulary.get_nonterminal_name_with_default(*id))?
          },
          ProductionItem::Terminal(id) => {
            let name = self.vocabulary.get_terminal_name(*id);
            write!(f, " {}", name)?;
          },
        }
      }
      
      write!(f, ";\n")?;
      
    }

    Ok(())
  }
}


#[derive(PartialEq, Eq, Clone, Hash, Debug, Copy)]
pub enum ProductionItem {
  NonTerminal(usize),
  Terminal(usize),
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Production {
  pub left: usize, 
  pub right: Vec<ProductionItem>,
}

impl Production {
  pub fn new(left: usize, right: &Vec<ProductionItem>) -> Self {
    Self { left, right: right.clone() }
  }
}


