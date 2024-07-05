// 构造分析相关的函数都写在这里, 这个模块不需要暴露出去

use std::{collections::{BTreeSet, HashMap, BTreeMap}, vec};
use chiru::runtime::{production::{Production, ProductionItem}, vocabulary::{NonTerminal, Terminal}};
use maplit::btreeset;
use super::{Grammar, ActionTableElement};


pub struct Collection {
  pub allow_epsilon: bool,
  pub set: BTreeSet<usize>,
}

// 项目
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct Item<'a> {
  pub production: &'a Production<'a>,
  pub dot: usize,
  // 增加一个字段 state, 表示所属的项目集闭包的编号
  pub state: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct ItemWithLookAhead<'a> {
  pub production: &'a Production<'a>,
  pub dot: usize,
  pub look_ahead: usize,
}

// 求 first(βa)
pub fn first(beta: &[ProductionItem], a: usize, first_set_of_nonterminals: &BTreeMap<usize, Collection>) -> BTreeSet<usize> {
  let mut result: Collection = Collection { allow_epsilon: true, set: BTreeSet::new(), };
  for item in beta.iter() {
    match item {
      ProductionItem::NonTerminal(rule_id) => {
        let c = first_set_of_nonterminals.get(&rule_id.id).unwrap();
        for item in c.set.iter() { result.set.insert(*item) ; }
        if !c.allow_epsilon {
          result.allow_epsilon = false;
          break;
        }
      },
      ProductionItem::Terminal(token_type) => {
        result.allow_epsilon = false;
        result.set.insert(token_type.id);
        break;
      },
    }
  }

  if result.allow_epsilon { result.set.insert(a); }

  result.set
}

pub fn get_first_set_for_non_epsilon_rule(production: &Production, result: &mut Collection, first_set: &BTreeMap<usize, Collection>) -> bool {
  
  let mut modified = false; // 标识 result 是否被修改

  // 首先判断是否可以为 epsilon 
  let mut allow_epsilon = true;
  for item in production.right.iter() {
    match item {
      ProductionItem::NonTerminal(id) => {
        let set = first_set.get(&id.id).unwrap();
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
        let c = first_set.get(&rule_id.id).unwrap();
        for item in c.set.iter() { modified = result.set.insert(*item) || modified; }

        if ! c.allow_epsilon {
          break;
        }
      },
      ProductionItem::Terminal(token_type) => {
        modified = result.set.insert(token_type.id) || modified;
        
        // 遇到终结符就退出
        break;
      },
    }
  }


  modified
}

pub fn first_set_for_nonterminal_and_production(grammar: &Grammar) -> (BTreeMap<usize, Collection>, BTreeMap<usize, Collection>) {
  let mut /*非终结符的first集合 */ result:BTreeMap<usize, Collection> = BTreeMap::new();
  for nonterminal in grammar.vocabulary.get_all_nonterminals().iter() {
    result.insert(*nonterminal, Collection { allow_epsilon: false, set: BTreeSet::new() });
  }
  let mut modified = true;
  let mut /*产生式的first集合 */ cache: BTreeMap<usize, Collection> = BTreeMap::new();
  
  for production in grammar.productions.values() {
    cache.insert(production.id, Collection { allow_epsilon: false, set: BTreeSet::new() });
  }
  
  while modified {
    modified = false;
    for production in grammar.productions.values() {
      let t = cache.get_mut(&production.id).unwrap();
      modified = get_first_set_for_non_epsilon_rule(production, t, &result);
      
      let r = result.get_mut(&production.left.id).unwrap();
      if t.allow_epsilon && ! r.allow_epsilon {
        r.allow_epsilon = t.allow_epsilon;
        modified = true;
      }

      for item in t.set.iter() { modified = r.set.insert(*item) || modified }
    }
  }
  (result, cache)
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
            let item = Item { production, dot: 0, state: 0 };
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

#[allow(unused_doc_comments)]
pub fn closure_with_look_ahead<'a>(item_set: BTreeSet<ItemWithLookAhead<'a>>, productions: &'a HashMap<usize, Production>, first_set_of_nonterminals: &BTreeMap<usize, Collection>) -> BTreeSet<ItemWithLookAhead<'a>> {
  /**
   * closure(I):
   *   repeat
   *     for [A->α·Bβ, a] in I
   *       for B -> γ in G'
   *         for b in first(βa)
   *           I.insert([B->·γ, b])
   *   until 不能加入更多的项
   *   return I
   */
  let mut result_set = item_set.to_owned();
  let mut j = item_set.clone();
  loop {
    let mut k: BTreeSet<ItemWithLookAhead> = BTreeSet::new();
    for /*[A->α·Bβ, a] */ item in j.iter() {
      if item.dot >= item.production.right.len() { continue; }
      else if let ProductionItem::NonTerminal(rule_id) = item.production.right[item.dot] {
        for /*B -> γ */ production in productions.values().filter(|production| {
          production.left == rule_id
        }) {
          for b in first(&item.production.right[item.dot+1..], item.look_ahead, first_set_of_nonterminals) {
            let item = ItemWithLookAhead { production, dot:0, look_ahead: b };
            if ! result_set.contains(&item) {
              k.insert(item);
            }
          }
        }
      }
    }
    
    let before = result_set.len();
    result_set.extend(k.clone());
    if result_set.len() <= before { break; }
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
      j.insert(Item { production: item.production, dot: item.dot + 1, state: 0 });
    }
  }
  closure(j, productions)
}

// 直接求出 action 和 goto 表
#[allow(unused_doc_comments)]
pub fn lalr_table(grammar: &Grammar) -> (BTreeMap<(usize, usize), ActionTableElement>, BTreeMap<(usize, usize), usize>, BTreeMap<usize, usize>) {
  // 用于记录命名非终结符对应的初始状态
  let mut nonterminal_to_start_state: BTreeMap<usize, usize> = BTreeMap::new();
  

  // 对文法做增广, 给每个命名非终结符增加一条产生式
  let mut /*下一个产生式的id */ next_production_id = grammar.productions.iter().map(|(_, production)| production.id).max().unwrap_or(0) + 1;
  let mut /*下一个非终结符id */ next_nonterminal_id = grammar.vocabulary.nonterminals.iter().max().unwrap_or(&0) + 1;
  let mut /*产生式 */ productions = grammar.productions.clone();
  let first_augmented_production_id = next_production_id;
  for (nonterminal, _) in grammar.vocabulary.named_nonterminals.iter() {
    let p = Production::new(next_production_id, NonTerminal::new(None, next_nonterminal_id), 
      &vec![ProductionItem::NonTerminal(NonTerminal::new(None, *nonterminal))]);
    productions.insert(next_production_id, p);
    next_nonterminal_id += 1;
    next_production_id += 1;
  }
  let productions = productions;/*重新绑定为不可变 */



  // 求所有增广产生式的闭包并编号
  let mut /*项目集闭包编号 */ next_closure_id = 0;
  let mut /*lr0 项目集族 */ c: BTreeMap<BTreeSet<Item>, usize> = BTreeMap::new();
  for production_id in first_augmented_production_id..next_production_id {
    let production = productions.get(&production_id).unwrap();
    let item = Item { production, dot: 0, state: 0 };
    let closure = closure(btreeset! { item }, &productions);
    c.insert(closure, next_closure_id);
    if let ProductionItem::NonTerminal(nonterminal_id) = production.right[0] {
      nonterminal_to_start_state.insert(nonterminal_id.id, next_closure_id);
    }
    next_closure_id += 1;
  }
  let nonterminal_to_start_state: BTreeMap<usize, usize> = nonterminal_to_start_state; /*重新绑定为不可变 */

  // 构造文法符号集合
  let mut symbols: Vec<ProductionItem> = Vec::new();
  for (terminal, _) in grammar.vocabulary.terminals.iter() {
    symbols.push(ProductionItem::Terminal(Terminal::new(name, id)));
  }
  for nonterminal in grammar.vocabulary.nonterminals.iter() {
    symbols.push(ProductionItem::NonTerminal(*nonterminal));
  }
  let symbols = symbols;


  // 构造 lr0 项目集族, 以及 goto 表
  let mut goto_table: BTreeMap<(usize, ProductionItem), usize> = BTreeMap::new();
  let mut j = c.clone();
  loop {
    let mut k: BTreeMap<BTreeSet<Item>, usize> = BTreeMap::new();
    for (closure, id) in j.iter() {
      for x in symbols.iter() {
        let n = goto(closure.clone(), *x, &productions);
        if n.len() > 0 && ! c.contains_key(&n) {
          k.insert(n, next_closure_id);
          // 这里可以顺便把 goto 表也完成了
          goto_table.insert((*id, *x), next_closure_id);
          next_closure_id += 1;
        }
      }
    }

    let before = c.len();
    c.extend(k.clone());
    if c.len() <= before { break; }  
    j = k;
  }
  drop(j);
  let c: BTreeMap<BTreeSet<Item<'_>>, usize> = c;
  let goto_table: BTreeMap<(usize, ProductionItem), usize> = goto_table;

  // 初始化扩散关系映射和展望符映射
  let mut look_ahead_map: BTreeMap<Item, BTreeSet<usize>> = BTreeMap::new();
  let mut dissemination_map: BTreeMap<Item, BTreeSet<Item>> = BTreeMap::new();
  for (closure, state) in c.iter() {
    for item in closure.iter().filter(| item | item.dot != 0 || item.dot == 0 && first_augmented_production_id <= item.production.id && next_production_id > item.production.id ) {
      let item = Item { dot: item.dot, production: item.production, state: *state };
      look_ahead_map.insert(item, BTreeSet::new());
      dissemination_map.insert(item, BTreeSet::new());
    }
  }
  drop(c);
  drop(symbols);
  
  let items = look_ahead_map.keys().cloned().collect::<Vec<_>>();
  // 求所有非终结符的 first 集合
  let (first_set, _) = first_set_for_nonterminal_and_production(grammar);

  /**
   * for [state, A -> α·β] in 所有的内核项
   *   J := closure({ [A -> α·β, #] })
   *   for [B -> γ·Xδ, a] in J
   *     if a != #
   *       look_ahead_map[goto(state, X), B->γX·δ].insert(a)
   *     else
   *       dissemination_map[state, A -> α·β].insert([goto(state, X), B->γX·δ])
   */
  let sharp = grammar.vocabulary.terminals.keys().cloned().max().unwrap_or(0) + 114514;
  for item in items {
    let j = closure_with_look_ahead(
      btreeset! { ItemWithLookAhead { production: item.production, dot: item.dot, look_ahead: sharp } }, &productions, &first_set
    );
    for /*[B -> γ·Xδ, a] */ item2 in j {
      if item2.dot >= item2.production.right.len() { continue; }
      let y = goto_table.get(&(item.state, item2.production.right[item2.dot])).unwrap();
      if sharp != item2.look_ahead {
        look_ahead_map.get_mut(&Item { production: item2.production, dot: item2.dot + 1, state: *y })
          .unwrap().insert(item2.look_ahead);
      }
      else {
        dissemination_map.get_mut(&item)
          .unwrap().insert(Item { production: item2.production, dot: item2.dot + 1, state: *y });
      }
    }
  }
  drop(first_set);

  let dissemination_map = dissemination_map;
  
  // 将 $ 添加到初始状态中
  dissemination_map.keys().filter(| item | { item.dot == 0 }).for_each(|item| {
    look_ahead_map.get_mut(item).unwrap().insert(0);
  });

  let mut modified = false;
  loop {
    // 不断向前传播即可
    for (k, v) in dissemination_map.iter() {
      let look_ahead = look_ahead_map.get(k).unwrap().clone();
      for item in v.iter() {
        let t = look_ahead_map.get_mut(item).unwrap();
        let before = t.len();
        t.extend(&look_ahead);
        if t.len() > before { modified = true }
      }
    }

    if ! modified { break }
  }

  let look_ahead_map = look_ahead_map;
  drop(dissemination_map);

  // 最后, 构造 action 和 goto 表
  let mut action: BTreeMap<(usize, usize), ActionTableElement> = BTreeMap::new(); 
  let mut goto_table2: BTreeMap<(usize, usize), usize> = BTreeMap::new();
  for item in look_ahead_map.keys() {
    if item.dot >= item.production.right.len() {
      // 是归约项目
      for look_ahead in look_ahead_map.get(item).unwrap() {
        action.insert((item.state, *look_ahead), ActionTableElement::Reduce(item.production.id));
      }
    }
    else {
      let d = goto_table.get(&(item.state, item.production.right[item.dot])).unwrap();
      match item.production.right[item.dot] {
        ProductionItem::NonTerminal(rule_id) => {
          goto_table2.insert((item.state, rule_id), *d);
        },
        ProductionItem::Terminal(terminal_id) => {
          action.insert((item.state, terminal_id), ActionTableElement::Shift(*d));
        },
      }
    }
  }
  drop(look_ahead_map);

  let action = action;
  let goto_table = goto_table2;
  
  (action, goto_table, nonterminal_to_start_state)
}

