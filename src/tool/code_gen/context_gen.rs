
// use std::task::Context;

use std::collections::{HashSet, HashMap};

use tera::Context;

use crate::tool::{grammar::{Grammar, vocabulary::Vocabulary}, syntaxis::chiru_context::RuleListContext, visitor::context_visitor::ContextVisitor};

use super::{TEMPLATES, pascal};


// 生成 context 文件 生成某一个非终结符的 context
fn ctx_gen(grammar: &Grammar, rule_id: usize, ctx: (HashSet<usize>, HashSet<usize>, HashSet<usize>, HashSet<usize>), vocabulary: &Vocabulary) -> String {
  let rule_name = grammar.vocabulary.get_nonterminal_name_by_id(rule_id).unwrap();


  // 首先将 HashSet 转换为 vec
  let terminal_list = ctx.0.iter().map(|id| {
    let name = vocabulary.get_terminal_name_by_id(*id).unwrap();
    (name.to_lowercase(), name.to_uppercase(), pascal(&name))
  }).collect::<Vec<_>>();

  let terminal = ctx.1.iter().map(|id| {
    let name = vocabulary.get_terminal_name_by_id(*id).unwrap();
    (name.to_lowercase(), name.to_uppercase(), pascal(&name))
  }).collect::<Vec<_>>();

  let nonterminal_list = ctx.2.iter().map(|id| {
    let name = vocabulary.get_nonterminal_name_by_id(*id).unwrap();
    (name.to_lowercase(), name.to_uppercase(), pascal(&name))
  }).collect::<Vec<_>>();

  let nonterminal = ctx.3.iter().map(|id| {
    let name = vocabulary.get_nonterminal_name_by_id(*id).unwrap();
    (name.to_lowercase(), name.to_uppercase(), pascal(&name))
  }).collect::<Vec<_>>();







  let mut context = Context::new();
  context.insert("grammar_name", &(grammar.name.to_lowercase(), grammar.name.to_uppercase(), pascal(&grammar.name)));
  context.insert("ctx_name", &(rule_name.to_lowercase(), rule_name.to_uppercase(), pascal(&rule_name)));
  context.insert("nonterminal_list", &nonterminal_list);
  context.insert("terminal_list", &terminal_list);
  context.insert("nonterminal", &nonterminal);
  context.insert("terminal", &terminal);




  TEMPLATES.render("target/rust/ctx.tera", &context).unwrap()
}


// 需要 ast
pub fn context_generate(grammar: &Grammar, ast: &dyn RuleListContext) -> String {

  // 获取所有的终结符和非终结符
  let terminals = grammar.vocabulary.terminals.iter().map(|(id, terminal)| {
    (terminal.name.to_owned(), *id)
  }).collect::<HashMap<_, _>>();

  let nonterminals = grammar.vocabulary.get_all_named_nonterminals_map();




  // 首先解析 ast 获取 table
  let mut visitor = ContextVisitor::new(nonterminals, terminals);
  ast.accept(&mut visitor).unwrap();

  let table = visitor.table;

  // println!("{:?}",table);

  let nonterminals = grammar.vocabulary.get_all_named_nonterminals();

  nonterminals.iter().for_each(|x| {
    if ! table.contains_key(x) {
      println!("{} {}", x, grammar.vocabulary.get_nonterminal_name_by_id(*x).unwrap())
    }
  });



  let ctx_list = nonterminals.iter()
    .map(|id| { 
      let c = table.get(id).unwrap().clone();
      
      ctx_gen(grammar, *id, c, &grammar.vocabulary) 
    }).collect::<Vec<String>>();
  let mut context = Context::new();
  context.insert("ctx_list", &ctx_list);
  context.insert("grammar_name", &(grammar.name.to_lowercase(), grammar.name.to_uppercase(), pascal(&grammar.name)));

  TEMPLATES.render("target/rust/context.tera", &context).unwrap()
}










