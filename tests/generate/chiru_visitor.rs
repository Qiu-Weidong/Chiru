



// generated from .\src\tool\syntaxis\chiru.chiru by chiru 0.7.0
 


use std::{any::Any, error::Error};

use chiru::runtime::ast::{rule_context::RuleContext, ast_context::ASTContext, error_context::ErrorContext, terminal_context::TerminalContext};

use super::{
  chiru_context::{
    SharpContext,RbracketContext,StartContext,EpsilonContext,CommaContext,StringLiteralContext,OrContext,StarContext,WhiteSpaceContext,LparenContext,GrammarContext,AtContext,LineCommentContext,RuleRefContext,PlusContext,RparenContext,StopContext,TokenRefContext,BlockCommentContext,QuestionContext,RegularLiteralContext,ColonContext,LbracketContext,SemiContext,
  },
  chiru_parser::ChiruParser, 
};


pub trait ChiruVisitor {
  
  fn visit_sharp(&mut self, ctx: &dyn SharpContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_rbracket(&mut self, ctx: &dyn RbracketContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit__start(&mut self, ctx: &dyn StartContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_epsilon(&mut self, ctx: &dyn EpsilonContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_comma(&mut self, ctx: &dyn CommaContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_string_literal(&mut self, ctx: &dyn StringLiteralContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_or(&mut self, ctx: &dyn OrContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_star(&mut self, ctx: &dyn StarContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_white_space(&mut self, ctx: &dyn WhiteSpaceContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_lparen(&mut self, ctx: &dyn LparenContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_grammar(&mut self, ctx: &dyn GrammarContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_at(&mut self, ctx: &dyn AtContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_line_comment(&mut self, ctx: &dyn LineCommentContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_rule_ref(&mut self, ctx: &dyn RuleRefContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_plus(&mut self, ctx: &dyn PlusContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_rparen(&mut self, ctx: &dyn RparenContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit__stop(&mut self, ctx: &dyn StopContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_token_ref(&mut self, ctx: &dyn TokenRefContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_block_comment(&mut self, ctx: &dyn BlockCommentContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_question(&mut self, ctx: &dyn QuestionContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_regular_literal(&mut self, ctx: &dyn RegularLiteralContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_colon(&mut self, ctx: &dyn ColonContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_lbracket(&mut self, ctx: &dyn LbracketContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_semi(&mut self, ctx: &dyn SemiContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  

  
  fn visit(&mut self, ast: &RuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    match ast.get_rule_index() {
      
      ChiruParser::SHARP => self.visit_sharp(ast),
      ChiruParser::RBRACKET => self.visit_rbracket(ast),
      ChiruParser::_START => self.visit__start(ast),
      ChiruParser::EPSILON => self.visit_epsilon(ast),
      ChiruParser::COMMA => self.visit_comma(ast),
      ChiruParser::STRING_LITERAL => self.visit_string_literal(ast),
      ChiruParser::OR => self.visit_or(ast),
      ChiruParser::STAR => self.visit_star(ast),
      ChiruParser::WHITE_SPACE => self.visit_white_space(ast),
      ChiruParser::LPAREN => self.visit_lparen(ast),
      ChiruParser::GRAMMAR => self.visit_grammar(ast),
      ChiruParser::AT => self.visit_at(ast),
      ChiruParser::LINE_COMMENT => self.visit_line_comment(ast),
      ChiruParser::RULE_REF => self.visit_rule_ref(ast),
      ChiruParser::PLUS => self.visit_plus(ast),
      ChiruParser::RPAREN => self.visit_rparen(ast),
      ChiruParser::_STOP => self.visit__stop(ast),
      ChiruParser::TOKEN_REF => self.visit_token_ref(ast),
      ChiruParser::BLOCK_COMMENT => self.visit_block_comment(ast),
      ChiruParser::QUESTION => self.visit_question(ast),
      ChiruParser::REGULAR_LITERAL => self.visit_regular_literal(ast),
      ChiruParser::COLON => self.visit_colon(ast),
      ChiruParser::LBRACKET => self.visit_lbracket(ast),
      ChiruParser::SEMI => self.visit_semi(ast),

      _ => self.visit_children(ast)
    }
  }

  fn visit_terminal(&mut self, _terminal: &TerminalContext) -> Result<Box<dyn Any>, Box<dyn Error>>  { self.default_result() }

  fn visit_errornode(&mut self, _errornode: &ErrorContext) -> Result<Box<dyn Any>, Box<dyn Error>>  { self.default_result() }

  fn visit_children(&mut self, ctx: &RuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    let mut result = self.default_result();
    for child in ctx.children.iter() {
      if ! self.should_visit_next_child(ctx, &result) { break; }

      let child_result = match child {
        ASTContext::Terminal(ctx) => self.visit_terminal(ctx),
        ASTContext::Rule(ctx) => self.visit(ctx),
        ASTContext::Error(ctx) => self.visit_errornode(ctx),
      };

      result = self.aggregate_result(result, child_result);
    }
    result
  }

  fn default_result(&mut self) -> Result<Box<dyn Any>, Box<dyn Error>> { Ok(Box::new(())) }

  fn aggregate_result(&mut self, _aggregate: Result<Box<dyn Any>, Box<dyn Error>> , next_result: Result<Box<dyn Any>, Box<dyn Error>> ) -> Result<Box<dyn Any>, Box<dyn Error>>  { next_result }

  fn should_visit_next_child(&mut self, _context: &RuleContext, _current_result: &dyn Any) -> bool {true}
}







