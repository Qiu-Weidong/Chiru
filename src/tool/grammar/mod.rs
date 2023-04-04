// 定义一个数据结构来表示文法

use std::{collections::{HashMap, HashSet}, fmt::Display};

pub struct Grammar {
  // 文法的名称
  pub name: String,

  // 命名非终结符的查询缓存
  pub nonterminal_cache: HashMap<String, usize>,

  // 终结符的查询缓存
  pub terminal_cache: HashMap<String, usize>,

  // 所有非终结符，包括未命名
  pub nonterminals: HashMap<usize, Option<String>>,

  // 所有终结符
  pub terminals: HashMap<usize, String>,




  // 通过产生式的左部来管理产生式 产生式的优先级 产生式在 vec 中的顺序即为其优先级
  // pub productions: HashMap<usize, Vec<Production> >
  pub productions: HashSet<Production>, 
  // 当需要查询是否由相同产生式的时候，只比较右部
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
      nonterminal_cache: HashMap::new(),
      terminal_cache: HashMap::new(), 
      nonterminals: HashMap::new(),
      terminals: HashMap::new(),
      productions: HashSet::new(),
    }
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
          for item in c.set.iter() { modified = modified || result.set.insert(*item); }

          if ! c.allow_epsilon {
            break;
          }
        },
        ProductionItem::Terminal(token_type) => {
          modified = modified || result.set.insert(*token_type);
          
          // 遇到终结符就退出
          break;
        },
      }
    }


    modified
  }

  pub fn first_set(&self) -> HashMap<usize, Collection> {
    // 求 first 集合
    
    let mut result = HashMap::new();  
    // 首先将所有产生式的 first 集合初始化为空，不包含 epsilon。

    for (id, _) in self.nonterminals.iter() {
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
        
        modified = modified || Grammar::get_first_set_for_non_epsilon_rule(production, t, &result);

        // 使用产生式的 first 集合来更新非终结符的 first 集合。
        let r = result.get_mut(&production.left).unwrap();
        if t.allow_epsilon && !r.allow_epsilon { // t 可以为空 则 r 一定可以为空，反之，r 可以为空，而 t 不一定可以为空
          r.allow_epsilon = t.allow_epsilon;
          modified = true;
        }

        for item in t.set.iter() { modified = modified || r.set.insert(*item) }
      }
    
    }

    result
  }

  pub fn follow_set(&self) -> HashMap<usize, Collection> {
    // 求 first 集合
    
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
    for production in self.productions.iter() {
      let name = match self.nonterminals.get(&production.left).unwrap() {
        Some(name) => name.to_string(), 
        None => production.left.to_string(),
      };

      write!(f, "{} ->", name)?;

      for item in production.right.iter() {
        match item {
          ProductionItem::NonTerminal(id) => {
            match self.nonterminals.get(id).unwrap() {
              Some(name) => { write!(f, " {}", name)?; },
              None => { write!(f, " {}", id)?; },
            }
          },
          ProductionItem::Terminal(id) => {
            let name = self.terminals.get(id).unwrap();
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

  // 产生式的优先级
}

impl Production {
  pub fn new(left: usize, right: &Vec<ProductionItem>) -> Self {
    Self { left, right: right.clone() }
  }
}


