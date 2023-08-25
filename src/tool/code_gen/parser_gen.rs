

use std::collections::HashMap;
use tera::Context;

use crate::tool::grammar::production::Production;

use super::TEMPLATES;


pub fn ll1_table_generate(table: &HashMap<(usize, usize), usize>) -> String {
  // 输出预测分析表

  // 首先将预测分析表转换成元组列表
  let table = table.iter().map(|((k1, k2), k3)| (*k1, *k2, *k3)).collect::<Vec<_>>();

  let mut context = Context::new();
  context.insert("table", &table);

  let result = TEMPLATES.render("target/rust/ll1_table.tera", &context).unwrap();

  
  result
}


fn production_right_generate(production: &Production) -> String {
  let mut result = String::from("vec![");
  for item in production.right.iter() {
    match item {
      crate::tool::grammar::production::ProductionItem::NonTerminal(id) => {
        result += &format!("ProductionItem::NonTerminal({}),", id);
      },
      crate::tool::grammar::production::ProductionItem::Terminal(id) => {
        result += &format!("ProductionItem::Terminal({}),", id);
      },
    }
  }
  result += "]";
  result
}

pub fn production_generate(productions: &HashMap<usize, Production>) -> String {
  // (production_id: usize, left_id: usize, right: String )
  let productions = productions.iter().map(|(id, production)| {
    // 求出 production 的 right 对应的字符串
    return (*id, production.left, production_right_generate(production));
  }).collect::<Vec<_>>();


  let mut context = Context::new();
  context.insert("productions", &productions);

  let result = TEMPLATES.render("target/rust/production.tera", &context).unwrap();

  
  result
}



