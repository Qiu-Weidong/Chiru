


use chiru::runtime::ast::{terminal_context::TerminalContext, error_context::ErrorContext, rule_context::RuleContext};

use super::{
  parser::ChiruParser, 
  context::{
    ElementContext,AttributeListContext,AnnotationContext,BlockContext,RegularContext,AlternativeContext,ParserRuleContext,LexerRuleContext,EpsilonContext,EbnfSuffixContext,RuleListContext,AttributeContext,
  }
};


pub trait ChiruListener {
  
  fn enter_element(&mut self, _ctx: &dyn ElementContext) {}
  fn exit_element(&mut self, _ctx: &dyn ElementContext) {}
  
  fn enter_attribute_list(&mut self, _ctx: &dyn AttributeListContext) {}
  fn exit_attribute_list(&mut self, _ctx: &dyn AttributeListContext) {}
  
  fn enter_annotation(&mut self, _ctx: &dyn AnnotationContext) {}
  fn exit_annotation(&mut self, _ctx: &dyn AnnotationContext) {}
  
  fn enter_block(&mut self, _ctx: &dyn BlockContext) {}
  fn exit_block(&mut self, _ctx: &dyn BlockContext) {}
  
  fn enter_regular(&mut self, _ctx: &dyn RegularContext) {}
  fn exit_regular(&mut self, _ctx: &dyn RegularContext) {}
  
  fn enter_alternative(&mut self, _ctx: &dyn AlternativeContext) {}
  fn exit_alternative(&mut self, _ctx: &dyn AlternativeContext) {}
  
  fn enter_parser_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
  fn exit_parser_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
  
  fn enter_lexer_rule(&mut self, _ctx: &dyn LexerRuleContext) {}
  fn exit_lexer_rule(&mut self, _ctx: &dyn LexerRuleContext) {}
  
  fn enter_epsilon(&mut self, _ctx: &dyn EpsilonContext) {}
  fn exit_epsilon(&mut self, _ctx: &dyn EpsilonContext) {}
  
  fn enter_ebnf_suffix(&mut self, _ctx: &dyn EbnfSuffixContext) {}
  fn exit_ebnf_suffix(&mut self, _ctx: &dyn EbnfSuffixContext) {}
  
  fn enter_rule_list(&mut self, _ctx: &dyn RuleListContext) {}
  fn exit_rule_list(&mut self, _ctx: &dyn RuleListContext) {}
  
  fn enter_attribute(&mut self, _ctx: &dyn AttributeContext) {}
  fn exit_attribute(&mut self, _ctx: &dyn AttributeContext) {}
  



  fn enter_every_rule(&mut self, _ctx: &RuleContext) {}

  fn exit_every_rule(&mut self, _ctx: &RuleContext) {}

  fn enter(&mut self, ctx: &RuleContext) {
    // 在这里进行派发即可
    match ctx.get_rule_index() {
      
      ChiruParser::ELEMENT => self.enter_element(ctx), 
      ChiruParser::ATTRIBUTE_LIST => self.enter_attribute_list(ctx), 
      ChiruParser::ANNOTATION => self.enter_annotation(ctx), 
      ChiruParser::BLOCK => self.enter_block(ctx), 
      ChiruParser::REGULAR => self.enter_regular(ctx), 
      ChiruParser::ALTERNATIVE => self.enter_alternative(ctx), 
      ChiruParser::PARSER_RULE => self.enter_parser_rule(ctx), 
      ChiruParser::LEXER_RULE => self.enter_lexer_rule(ctx), 
      ChiruParser::EPSILON => self.enter_epsilon(ctx), 
      ChiruParser::EBNF_SUFFIX => self.enter_ebnf_suffix(ctx), 
      ChiruParser::RULE_LIST => self.enter_rule_list(ctx), 
      ChiruParser::ATTRIBUTE => self.enter_attribute(ctx), 

      _ => {}
    }
  }

  fn exit(&mut self, ctx: &RuleContext) {
    match ctx.get_rule_index() {
      
      ChiruParser::ELEMENT => self.exit_element(ctx), 
      ChiruParser::ATTRIBUTE_LIST => self.exit_attribute_list(ctx), 
      ChiruParser::ANNOTATION => self.exit_annotation(ctx), 
      ChiruParser::BLOCK => self.exit_block(ctx), 
      ChiruParser::REGULAR => self.exit_regular(ctx), 
      ChiruParser::ALTERNATIVE => self.exit_alternative(ctx), 
      ChiruParser::PARSER_RULE => self.exit_parser_rule(ctx), 
      ChiruParser::LEXER_RULE => self.exit_lexer_rule(ctx), 
      ChiruParser::EPSILON => self.exit_epsilon(ctx), 
      ChiruParser::EBNF_SUFFIX => self.exit_ebnf_suffix(ctx), 
      ChiruParser::RULE_LIST => self.exit_rule_list(ctx), 
      ChiruParser::ATTRIBUTE => self.exit_attribute(ctx), 

      _ => {}
    }
  }

  fn enter_terminal(&mut self, _ctx: &TerminalContext) {}

  fn exit_terminal(&mut self, _ctx: &TerminalContext) {}

  fn enter_errornode(&mut self, _ctx: &ErrorContext) {}

  fn exit_errornode(&mut self, _ctx: &ErrorContext) {}  
}







