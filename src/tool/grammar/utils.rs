// 构造分析相关的函数都写在这里, 这个模块不需要暴露出去

use std::{collections::{BTreeSet, HashMap, BTreeMap}, vec};
use chiru::runtime::production::{Production, ProductionItem};
use maplit::btreeset;

use super::{Grammar, ActionTableElement};




// 项目
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item<'a> {
  pub production: &'a Production,
  pub dot: usize,
  // 增加一个字段 state, 表示所属的项目集闭包的编号
}

// 求项目集闭包
pub fn closure<'a>(item_set: BTreeSet<Item<'a>>, productions: &'a HashMap<usize, Production>) -> BTreeSet<Item<'a>> {
  let mut result_set = item_set.to_owned();
  let mut j = item_set.to_owned();
  loop {
    let mut k: BTreeSet<Item> = BTreeSet::new();
    
    // 将本轮循环产生的项目都放到了 k 中
    for item in j.iter() {
      if item.dot >= item.production.right.len() { continue; }
      use ProductionItem::*;
      match item.production.right[item.dot] {
        Terminal(_) => continue,
        NonTerminal(rule_id) => {
          for production in productions.values().filter(|production| {
            production.left == rule_id
          }) {
            let item = Item { production, dot: 0 };
            if ! result_set.contains(&item) {
              k.insert(item);
            }
          }
        },
      }
    }

    let before = result_set.len();
    result_set.extend(k.clone());
    if result_set.len() <= before {
      break;
    }
    j = k;

  }
  result_set
}

// goto 转换函数
pub fn goto<'a>(item_set: BTreeSet<Item<'a>>, x: ProductionItem, productions: &'a HashMap<usize, Production>) -> BTreeSet<Item<'a>> {
  let mut j: BTreeSet<Item> = BTreeSet::new();
  for item in item_set.iter() {
    if item.dot >= item.production.right.len() { continue; }
    else if x == item.production.right[item.dot] {
      j.insert(Item { production: item.production, dot: item.dot + 1 });
    }
  }
  closure(j, productions)
}


// #[allow(unused)]
pub fn canonical_lr0_collection(grammar: &mut Grammar) -> BTreeMap<BTreeSet<Item>, usize> {
  // 首先, 对文法做增广, 需要真正修改 grammar 只需要对所有的命名非终结符做增广
  let mut next_production_id = grammar.productions.iter().map(|(_, production)| production.id).max().unwrap_or(0) + 1;
  let mut next_nonterminal_id = grammar.vocabulary.nonterminals.iter().max().unwrap_or(&0) + 1;

  // 增广的产生式 ()
  let mut production_id: Vec<(usize, usize)> = Vec::new();

  // 扩充了非终结符和产生式
  for (nonterminal, _) in grammar.vocabulary.named_nonterminals.iter() {
    let p = Production::new(next_production_id, next_nonterminal_id, &vec![ProductionItem::NonTerminal(*nonterminal)]);
    grammar.productions.insert(next_production_id, p);
    production_id.push((next_production_id, next_nonterminal_id));
    next_nonterminal_id += 1;
    next_production_id += 1;
  }


  // 先求所有增广产生式的闭包, 并编号, todo 还需要记录每个非终结符对应的初始项目集编号 非终结符 -> 初始项目集编号
  let mut c: BTreeMap<BTreeSet<Item>, usize> = BTreeMap::new();
  let mut next_closure_id = 0;
  for (id, _) in production_id.iter() {
    let production = grammar.productions.get(id).unwrap();
    let item = Item { production, dot: 0 };
    let closure = closure(btreeset! { item }, &grammar.productions);
    c.insert(closure, next_closure_id); // 一定不会重复
    next_closure_id += 1;
  }

  // 构造所有的文法符号
  let mut symbols: Vec<ProductionItem> = Vec::new();
  for (terminal, _) in grammar.vocabulary.terminals.iter() {
    symbols.push(ProductionItem::Terminal(*terminal));
  }
  for nonterminal in grammar.vocabulary.nonterminals.iter() {
    symbols.push(ProductionItem::NonTerminal(*nonterminal));
  }

  // 构造 lr0 项目集族, 以及goto函数
  let mut j = c.clone();
  loop {
    let mut k: BTreeMap<BTreeSet<Item>, usize> = BTreeMap::new();
    for (closure, _id) in j.iter() {
      for x in symbols.iter() {
        let n = goto(closure.clone(), *x, &grammar.productions);
        if n.len() > 0 && ! c.contains_key(&n) {
          k.insert(n, next_closure_id);
          // 这里可以顺便把 goto 表也完成了
          // (id, x) -> next_closure_id 
          next_closure_id += 1;
        }
      }
    }

    let before = c.len();
    c.extend(k.clone());
    if c.len() <= before { break; }  
    j = k;
  }


  c

}


// 直接求出 action 和 goto 表
pub fn lalr_table(grammar: &Grammar) -> (BTreeMap<(usize, usize), ActionTableElement>, BTreeMap<(usize, usize), usize>) {
  // 对文法做增广, 给每个命名非终结符增加一条产生式
  // augmented 增广的产生式

  // 将增广的产生式和原本的产生式合并
  // 对增广的产生式求闭包, 并编号, 记录下每个命名非终结符对应的闭包id, 相当于是完成了 lr0 项目集族的初始化

  // 构造文法符号集合
  // 构造 lr0 项目集族, 以及 goto 表


  todo!()
}

