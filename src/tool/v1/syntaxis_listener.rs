use crate::runtime::ast::{terminal_context::TerminalContext, error_context::ErrorContext, rule_context::RuleContext};

use super::{syntaxis_parser::SyntaxisParser, syntaxis_context::{RuleListContext, ParserRuleContext, BlockContext, AlternativeContext, ElementContext, EbnfSuffixContext, EpsilonContext, LexerRuleContext, RegularContext}};


pub trait SyntaxisListener {
  fn enter_rule_list(&mut self, _ctx: &dyn RuleListContext) {}
  fn exit_rule_list(&mut self, _ctx: &dyn RuleListContext) {}

  fn enter_parser_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
  fn exit_parser_rule(&mut self, _ctx: &dyn ParserRuleContext) {}

  fn enter_block(&mut self, _ctx: &dyn BlockContext) {}
  fn exit_block(&mut self, _ctx: &dyn BlockContext) {}

  fn enter_alternative(&mut self, _ctx: &dyn AlternativeContext) {}
  fn exit_alternative(&mut self, _ctx: &dyn AlternativeContext) {}

  fn enter_epsilon(&mut self, _ctx: &dyn EpsilonContext) {}
  fn exit_epsilon(&mut self, _ctx: &dyn EpsilonContext) {}

  fn enter_element(&mut self, _ctx: &dyn ElementContext) {}
  fn exit_element(&mut self, _ctx: &dyn ElementContext) {}

  fn enter_ebnf_suffix(&mut self, _ctx: &dyn EbnfSuffixContext) {}
  fn exit_ebnf_suffix(&mut self, _ctx: &dyn EbnfSuffixContext) {}

  fn enter_lexer_rule(&mut self, _ctx: &dyn LexerRuleContext) {}
  fn exit_lexer_rule(&mut self, _ctx: &dyn LexerRuleContext) {}

  fn enter_regular(&mut self, _ctx: &dyn RegularContext) {}
  fn exit_regular(&mut self, _ctx: &dyn RegularContext) {}



  fn enter_every_rule(&mut self, _ctx: &RuleContext) {}

  fn exit_every_rule(&mut self, _ctx: &RuleContext) {}

  fn enter(&mut self, ctx: &RuleContext) {
    // 在这里进行派发即可
    match ctx.get_rule_index() {
      SyntaxisParser::RULE_LIST => self.enter_rule_list(ctx),
      SyntaxisParser::PARSER_RULE => self.enter_parser_rule(ctx),
      SyntaxisParser::BLOCK => self.enter_block(ctx),
      SyntaxisParser::ALTERNATIVE => self.enter_alternative(ctx),
      SyntaxisParser::EPSILON => self.enter_epsilon(ctx),
      SyntaxisParser::ELEMENT => self.enter_element(ctx),
      SyntaxisParser::EBNF_SUFFIX => self.enter_ebnf_suffix(ctx),
      SyntaxisParser::LEXER_RULE => self.enter_lexer_rule(ctx),
      SyntaxisParser::REGULAR => self.enter_regular(ctx),

      _ => {}
    }
  }

  fn exit(&mut self, ctx: &RuleContext) {
    match ctx.get_rule_index() {
      SyntaxisParser::RULE_LIST => self.exit_rule_list(ctx),
      SyntaxisParser::PARSER_RULE => self.exit_parser_rule(ctx),
      SyntaxisParser::BLOCK => self.exit_block(ctx),
      SyntaxisParser::ALTERNATIVE => self.exit_alternative(ctx),
      SyntaxisParser::EPSILON => self.exit_epsilon(ctx),
      SyntaxisParser::ELEMENT => self.exit_element(ctx),
      SyntaxisParser::EBNF_SUFFIX => self.exit_ebnf_suffix(ctx),
      SyntaxisParser::LEXER_RULE => self.exit_lexer_rule(ctx),
      SyntaxisParser::REGULAR => self.exit_regular(ctx),

      _ => {}
    }
  }

  fn enter_terminal(&mut self, _ctx: &TerminalContext) {}

  fn exit_terminal(&mut self, _ctx: &TerminalContext) {}

  fn enter_errornode(&mut self, _ctx: &ErrorContext) {}

  fn exit_errornode(&mut self, _ctx: &ErrorContext) {}  
}







