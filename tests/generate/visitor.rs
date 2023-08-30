use std::any::Any;

use chiru::runtime::ast::{rule_context::RuleContext, ast_context::ASTContext, error_context::ErrorContext, terminal_context::TerminalContext};

use super::{
  context::{
    AlternativeContext,RegularContext,LexerRuleContext,ParserRuleContext,ElementContext,EbnfSuffixContext,BlockContext,EpsilonContext,RuleListContext,
  },
  parser::ChiruParser, 
};


pub trait ChiruVisitor {
  
  fn visit_alternative(&mut self, ctx: &dyn AlternativeContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_regular(&mut self, ctx: &dyn RegularContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_lexer_rule(&mut self, ctx: &dyn LexerRuleContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_parser_rule(&mut self, ctx: &dyn ParserRuleContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_element(&mut self, ctx: &dyn ElementContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_ebnf_suffix(&mut self, ctx: &dyn EbnfSuffixContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_block(&mut self, ctx: &dyn BlockContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_epsilon(&mut self, ctx: &dyn EpsilonContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_rule_list(&mut self, ctx: &dyn RuleListContext) -> Box<dyn Any> {
    self.visit_children(ctx.as_rule())
  }
  

  
  fn visit(&mut self, ast: &RuleContext) -> Box<dyn Any> {
    match ast.get_rule_index() {
      
      ChiruParser::ALTERNATIVE => self.visit_alternative(ast),
      ChiruParser::REGULAR => self.visit_regular(ast),
      ChiruParser::LEXER_RULE => self.visit_lexer_rule(ast),
      ChiruParser::PARSER_RULE => self.visit_parser_rule(ast),
      ChiruParser::ELEMENT => self.visit_element(ast),
      ChiruParser::EBNF_SUFFIX => self.visit_ebnf_suffix(ast),
      ChiruParser::BLOCK => self.visit_block(ast),
      ChiruParser::EPSILON => self.visit_epsilon(ast),
      ChiruParser::RULE_LIST => self.visit_rule_list(ast),

      _ => self.visit_children(ast)
    }
  }

  fn visit_terminal(&mut self, _terminal: &TerminalContext) -> Box<dyn Any>  { self.default_result() }

  fn visit_errornode(&mut self, _errornode: &ErrorContext) -> Box<dyn Any>  { self.default_result() }

  fn visit_children(&mut self, ctx: &RuleContext) -> Box<dyn Any> {
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

  fn default_result(&mut self) -> Box<dyn Any> { Box::new(()) }

  fn aggregate_result(&mut self, _aggregate: Box<dyn Any> , next_result: Box<dyn Any> ) -> Box<dyn Any>  { next_result }

  fn should_visit_next_child(&mut self, _context: &RuleContext, _current_result: &dyn Any) -> bool {true}
}







