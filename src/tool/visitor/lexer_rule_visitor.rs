use std::{any::Any, collections::HashMap, error::Error};

use chiru::runtime::lexer_rule::LexerRule;

use crate::tool::syntaxis::{chiru_visitor::ChiruVisitor, chiru_context::LexerRuleContext};









pub struct LexerRuleVisitor {
  
  pub next_token_id: usize, 

  pub next_channel_id: usize,

  pub lexer_rule_map: HashMap<String, LexerRule>,

  pub channel_map: HashMap<String, usize>,
}



impl LexerRuleVisitor {
  pub fn new(next_token_id: usize, lexer_rule_map: HashMap<String, LexerRule>) -> Self {
    Self { 
      next_token_id, 
      lexer_rule_map,
      channel_map: HashMap::new(),
      next_channel_id: 1,
    }
  }
}



impl ChiruVisitor for LexerRuleVisitor {
  
  
  fn visit_lexer_rule(&mut self, ctx: &dyn LexerRuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    // 获取名称
    let name = &ctx.token_ref().unwrap().symbol.text;

    // 检查是否已经定义
    if self.lexer_rule_map.contains_key(name) {
      // 重复定义，报错

      // 字符串字面量都使用了 '' 包裹，一定不会和普通 token 重复
      return self.default_result();
    }


    let mut channel = 0;
    let mut skip = false;

    if let Some(annotation) = ctx.annotation() {
      // 查看注解
      if let Some(att) = annotation.attribute() {
        if let Some(text) = att.rule_ref() {

          // 处理annotation
          let text = &text.symbol.text;
          if text == "ignore" || text == "skip" {
            skip = true;
          }
          else if text == "channel" {
            if let Some(c) = att.token_ref() {
              let text = &c.symbol.text;
              if self.channel_map.contains_key(text) {
                channel = *self.channel_map.get(text).unwrap();
              }
              else {
                channel = self.next_channel_id;
                self.channel_map.insert(text.to_owned(), channel);
                self.next_channel_id += 1;
              }
            }
          }
        }
        
      } else if let Some(attr_list) = annotation.attributes() {
        attr_list.attribute_list().iter().for_each(| att | {
          if let Some(text) = att.rule_ref() {

            // 处理annotation
            let text = &text.symbol.text;
            if text == "ignore" || text == "skip" {
              skip = true;
            }
            else if text == "channel" {
              if let Some(c) = att.token_ref() {
                let text = &c.symbol.text;
                if self.channel_map.contains_key(text) {
                  channel = *self.channel_map.get(text).unwrap();
                }
                else {
                  channel = self.next_channel_id;
                  self.channel_map.insert(text.to_owned(), channel);
                  self.next_channel_id += 1;
                }
              }
            }
          }
          

        });
      }

    } 


    let regular = &ctx.regular().unwrap().regular_literal().unwrap().symbol.text; // .replace("\\/", "/");

    let lexer_rule = LexerRule::new(self.next_token_id, &name, regular, 
      channel, skip
    );

    self.next_token_id += 1;


    self.lexer_rule_map.insert(name.to_string(), lexer_rule);

    
    self.default_result()
  }

}



