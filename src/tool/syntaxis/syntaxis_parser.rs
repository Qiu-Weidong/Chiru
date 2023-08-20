

use std::collections::HashMap;
use maplit::hashmap;

use crate::{runtime::{token::Token, 
  ast::{rule_context::RuleContext, terminal_context::TerminalContext, ast_context::ASTContext}}, 
  tool::{grammar::{production::ProductionItem,  Grammar, production::Production}, 
  syntaxis::syntaxis_context::{RuleListContext, ParserRuleContext, BlockContext, 
    AlternativeContext, EpsilonContext, ElementContext, EbnfSuffixContext, LexerRuleContext, RegularContext}}};


pub struct SyntaxisParser {
  // token 流
  pub tokens: Vec<Token>,

  // 这两个应该声明为常量，直接放在 lazy_static! 中
  pub table: HashMap<(usize, usize), usize>,
  pub grammar: Grammar,
}


lazy_static!{
  // 直接写预测分析表
  static ref LL1_TABLE: HashMap<(usize, usize), usize> = hashmap!{
    (0, 0) => 1,
  };

  // 产生式
  static ref PRODUCTIONS: HashMap<usize, Production>  = hashmap!{
    0 => Production::new(0, 0, &vec![ProductionItem::NonTerminal(0)]),
  };

  // 终结符
  pub static ref NONTERMINALS: HashMap<usize, &'static str> = hashmap! {
    0 => "",
  };

  // 非终结符
  pub static ref TERMINALS: HashMap<usize, &'static str> = hashmap! {
    0 => "",
  };
}


impl SyntaxisParser {

  // pub static ref GLOBAL_VALUE: i32 = 42;

  // 使用模板生成 每个非终结符的编号
  pub const RULE_LIST: usize = 0;
  pub const PARSER_RULE: usize = 1;
  pub const BLOCK: usize = 2;
  pub const ALTERNATIVE: usize = 3;
  pub const EPSILON: usize = 4;
  pub const ELEMENT: usize = 5;
  pub const EBNF_SUFFIX: usize = 6;
  pub const LEXER_RULE: usize = 7;
  pub const REGULAR: usize = 8;

  // 生成一个预测分析表



  pub fn new(tokens: Vec<Token>, table: HashMap<(usize, usize), usize>, grammar: Grammar) -> Self {
    // table 类型变为 (usize, usize) -> usize
    // productions = vec![
    //   vec![1, -2, 3], vec![], ...
    // ];
    // table.insert((0, 1), 1);
    Self {
      tokens,
      table,
      grammar,
    }
  }


  // 使用模板生成
  pub fn rule_list(&self) -> Box<dyn RuleListContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::RULE_LIST);
    Box::new(result)
  }

  pub fn parser_rule(&self) -> Box<dyn ParserRuleContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::PARSER_RULE);
    Box::new(result)
  }

  pub fn block(&self) -> Box<dyn BlockContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::BLOCK);
    Box::new(result)
  }

  pub fn alternative(&self) -> Box<dyn AlternativeContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::ALTERNATIVE);
    Box::new(result)
  }

  pub fn epsilon(&self) -> Box<dyn EpsilonContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::EPSILON);
    Box::new(result)
  }

  pub fn element(&self) -> Box<dyn ElementContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::ELEMENT);
    Box::new(result)
  }

  pub fn ebnf_suffix(&self) -> Box<dyn EbnfSuffixContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::EBNF_SUFFIX);
    Box::new(result)
  }

  pub fn lexer_rule(&self) -> Box<dyn LexerRuleContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::LEXER_RULE);
    Box::new(result)
  }

  pub fn regular(&self) -> Box<dyn RegularContext> {
    let mut cursor = 0;
    let result = self.parse_ast(&mut cursor, Self::REGULAR);
    Box::new(result)
  }

  // 下面两个函数放到 trait 中
  // 可有可无
  pub fn parse(&self) -> RuleContext {
    let mut cursor = 0;
    self.parse_ast(&mut cursor, Self::RULE_LIST)
  }


  // 直接照抄
  fn parse_ast(&self, cursor: &mut usize, rule_index: usize) -> RuleContext {

    let token_type = self.tokens[*cursor].token_type;
    let production_id = self.table.get(&(rule_index, token_type)).unwrap();
    let production = self.grammar.productions.get(production_id).unwrap();
    let name = self.grammar.vocabulary.get_nonterminal_name_with_default(rule_index);

    let mut result = RuleContext { rule_index, rule_name: name, children: Vec::new(), };
    
    for child in production.right.iter() {
      match child {
        ProductionItem::NonTerminal(rule_id) => {
          let rule = self.parse_ast(cursor, *rule_id);
          if let Some(_) = self.grammar.vocabulary.get_nonterminal_name_by_id(*rule_id) {
            // 如果有名字
            let child = ASTContext::Rule(rule);
            result.children.push(child);
          } else {
            // 否则将其 child 直接添加进来
            // for child in rule.children.iter() {
            //   result.children.push(child.clone());
            // }

            // 我似乎可以直接将整个数组 move 过来
            // result.children.append(&mut rule.children);
            result.children.extend(rule.children);
          }
          
        },
        ProductionItem::Terminal(token_type) => {
          // 检查是否匹配
          if *token_type != self.tokens[*cursor].token_type { println!("符号不匹配") }
          let terminal = TerminalContext { symbol: self.tokens[*cursor].clone() };
          *cursor += 1;
          let child = ASTContext::Terminal(terminal);
          result.children.push(child);
        },
      };
    }
    
    result
  }

}




