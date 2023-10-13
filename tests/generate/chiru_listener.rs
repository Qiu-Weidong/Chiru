



// generated from .\src\tool\syntaxis\chiru.chiru by chiru 0.7.0
 


use chiru::runtime::ast::{terminal_context::TerminalContext, error_context::ErrorContext, rule_context::RuleContext};

use super::{
  chiru_parser::ChiruParser, 
  chiru_context::{
    SharpContext,RbracketContext,StartContext,EpsilonContext,CommaContext,StringLiteralContext,OrContext,StarContext,WhiteSpaceContext,LparenContext,GrammarContext,AtContext,LineCommentContext,RuleRefContext,PlusContext,RparenContext,StopContext,TokenRefContext,BlockCommentContext,QuestionContext,RegularLiteralContext,ColonContext,LbracketContext,SemiContext,
  }
};


pub trait ChiruListener {
  
  fn enter_sharp(&mut self, _ctx: &dyn SharpContext) {}
  fn exit_sharp(&mut self, _ctx: &dyn SharpContext) {}
  
  fn enter_rbracket(&mut self, _ctx: &dyn RbracketContext) {}
  fn exit_rbracket(&mut self, _ctx: &dyn RbracketContext) {}
  
  fn enter__start(&mut self, _ctx: &dyn StartContext) {}
  fn exit__start(&mut self, _ctx: &dyn StartContext) {}
  
  fn enter_epsilon(&mut self, _ctx: &dyn EpsilonContext) {}
  fn exit_epsilon(&mut self, _ctx: &dyn EpsilonContext) {}
  
  fn enter_comma(&mut self, _ctx: &dyn CommaContext) {}
  fn exit_comma(&mut self, _ctx: &dyn CommaContext) {}
  
  fn enter_string_literal(&mut self, _ctx: &dyn StringLiteralContext) {}
  fn exit_string_literal(&mut self, _ctx: &dyn StringLiteralContext) {}
  
  fn enter_or(&mut self, _ctx: &dyn OrContext) {}
  fn exit_or(&mut self, _ctx: &dyn OrContext) {}
  
  fn enter_star(&mut self, _ctx: &dyn StarContext) {}
  fn exit_star(&mut self, _ctx: &dyn StarContext) {}
  
  fn enter_white_space(&mut self, _ctx: &dyn WhiteSpaceContext) {}
  fn exit_white_space(&mut self, _ctx: &dyn WhiteSpaceContext) {}
  
  fn enter_lparen(&mut self, _ctx: &dyn LparenContext) {}
  fn exit_lparen(&mut self, _ctx: &dyn LparenContext) {}
  
  fn enter_grammar(&mut self, _ctx: &dyn GrammarContext) {}
  fn exit_grammar(&mut self, _ctx: &dyn GrammarContext) {}
  
  fn enter_at(&mut self, _ctx: &dyn AtContext) {}
  fn exit_at(&mut self, _ctx: &dyn AtContext) {}
  
  fn enter_line_comment(&mut self, _ctx: &dyn LineCommentContext) {}
  fn exit_line_comment(&mut self, _ctx: &dyn LineCommentContext) {}
  
  fn enter_rule_ref(&mut self, _ctx: &dyn RuleRefContext) {}
  fn exit_rule_ref(&mut self, _ctx: &dyn RuleRefContext) {}
  
  fn enter_plus(&mut self, _ctx: &dyn PlusContext) {}
  fn exit_plus(&mut self, _ctx: &dyn PlusContext) {}
  
  fn enter_rparen(&mut self, _ctx: &dyn RparenContext) {}
  fn exit_rparen(&mut self, _ctx: &dyn RparenContext) {}
  
  fn enter__stop(&mut self, _ctx: &dyn StopContext) {}
  fn exit__stop(&mut self, _ctx: &dyn StopContext) {}
  
  fn enter_token_ref(&mut self, _ctx: &dyn TokenRefContext) {}
  fn exit_token_ref(&mut self, _ctx: &dyn TokenRefContext) {}
  
  fn enter_block_comment(&mut self, _ctx: &dyn BlockCommentContext) {}
  fn exit_block_comment(&mut self, _ctx: &dyn BlockCommentContext) {}
  
  fn enter_question(&mut self, _ctx: &dyn QuestionContext) {}
  fn exit_question(&mut self, _ctx: &dyn QuestionContext) {}
  
  fn enter_regular_literal(&mut self, _ctx: &dyn RegularLiteralContext) {}
  fn exit_regular_literal(&mut self, _ctx: &dyn RegularLiteralContext) {}
  
  fn enter_colon(&mut self, _ctx: &dyn ColonContext) {}
  fn exit_colon(&mut self, _ctx: &dyn ColonContext) {}
  
  fn enter_lbracket(&mut self, _ctx: &dyn LbracketContext) {}
  fn exit_lbracket(&mut self, _ctx: &dyn LbracketContext) {}
  
  fn enter_semi(&mut self, _ctx: &dyn SemiContext) {}
  fn exit_semi(&mut self, _ctx: &dyn SemiContext) {}
  



  fn enter_every_rule(&mut self, _ctx: &RuleContext) {}

  fn exit_every_rule(&mut self, _ctx: &RuleContext) {}

  fn enter(&mut self, ctx: &RuleContext) {
    // 在这里进行派发即可
    match ctx.get_rule_index() {
      
      ChiruParser::SHARP => self.enter_sharp(ctx), 
      ChiruParser::RBRACKET => self.enter_rbracket(ctx), 
      ChiruParser::_START => self.enter__start(ctx), 
      ChiruParser::EPSILON => self.enter_epsilon(ctx), 
      ChiruParser::COMMA => self.enter_comma(ctx), 
      ChiruParser::STRING_LITERAL => self.enter_string_literal(ctx), 
      ChiruParser::OR => self.enter_or(ctx), 
      ChiruParser::STAR => self.enter_star(ctx), 
      ChiruParser::WHITE_SPACE => self.enter_white_space(ctx), 
      ChiruParser::LPAREN => self.enter_lparen(ctx), 
      ChiruParser::GRAMMAR => self.enter_grammar(ctx), 
      ChiruParser::AT => self.enter_at(ctx), 
      ChiruParser::LINE_COMMENT => self.enter_line_comment(ctx), 
      ChiruParser::RULE_REF => self.enter_rule_ref(ctx), 
      ChiruParser::PLUS => self.enter_plus(ctx), 
      ChiruParser::RPAREN => self.enter_rparen(ctx), 
      ChiruParser::_STOP => self.enter__stop(ctx), 
      ChiruParser::TOKEN_REF => self.enter_token_ref(ctx), 
      ChiruParser::BLOCK_COMMENT => self.enter_block_comment(ctx), 
      ChiruParser::QUESTION => self.enter_question(ctx), 
      ChiruParser::REGULAR_LITERAL => self.enter_regular_literal(ctx), 
      ChiruParser::COLON => self.enter_colon(ctx), 
      ChiruParser::LBRACKET => self.enter_lbracket(ctx), 
      ChiruParser::SEMI => self.enter_semi(ctx), 

      _ => {}
    }
  }

  fn exit(&mut self, ctx: &RuleContext) {
    match ctx.get_rule_index() {
      
      ChiruParser::SHARP => self.exit_sharp(ctx), 
      ChiruParser::RBRACKET => self.exit_rbracket(ctx), 
      ChiruParser::_START => self.exit__start(ctx), 
      ChiruParser::EPSILON => self.exit_epsilon(ctx), 
      ChiruParser::COMMA => self.exit_comma(ctx), 
      ChiruParser::STRING_LITERAL => self.exit_string_literal(ctx), 
      ChiruParser::OR => self.exit_or(ctx), 
      ChiruParser::STAR => self.exit_star(ctx), 
      ChiruParser::WHITE_SPACE => self.exit_white_space(ctx), 
      ChiruParser::LPAREN => self.exit_lparen(ctx), 
      ChiruParser::GRAMMAR => self.exit_grammar(ctx), 
      ChiruParser::AT => self.exit_at(ctx), 
      ChiruParser::LINE_COMMENT => self.exit_line_comment(ctx), 
      ChiruParser::RULE_REF => self.exit_rule_ref(ctx), 
      ChiruParser::PLUS => self.exit_plus(ctx), 
      ChiruParser::RPAREN => self.exit_rparen(ctx), 
      ChiruParser::_STOP => self.exit__stop(ctx), 
      ChiruParser::TOKEN_REF => self.exit_token_ref(ctx), 
      ChiruParser::BLOCK_COMMENT => self.exit_block_comment(ctx), 
      ChiruParser::QUESTION => self.exit_question(ctx), 
      ChiruParser::REGULAR_LITERAL => self.exit_regular_literal(ctx), 
      ChiruParser::COLON => self.exit_colon(ctx), 
      ChiruParser::LBRACKET => self.exit_lbracket(ctx), 
      ChiruParser::SEMI => self.exit_semi(ctx), 

      _ => {}
    }
  }

  fn enter_terminal(&mut self, _ctx: &TerminalContext) {}

  fn exit_terminal(&mut self, _ctx: &TerminalContext) {}

  fn enter_errornode(&mut self, _ctx: &ErrorContext) {}

  fn exit_errornode(&mut self, _ctx: &ErrorContext) {}  
}







