use std::{any::Any, error::Error};

use chiru::runtime::ast::{rule_context::RuleContext, ast_context::ASTContext, error_context::ErrorContext, terminal_context::TerminalContext};

use super::{
  chiru_context::{
    LexerRuleContext,RulesContext,AttributesContext,EpsilonContext,AnnotationContext,CompilationUnitContext,ElementContext,BlockContext,GrammarNameContext,RegularContext,AttributeContext,ParserRuleContext,AlternativeContext,EbnfSuffixContext,
  },
  chiru_parser::ChiruParser, 
};


pub trait ChiruVisitor {
  
  fn visit_lexer_rule(&mut self, ctx: &dyn LexerRuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_rules(&mut self, ctx: &dyn RulesContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_attributes(&mut self, ctx: &dyn AttributesContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_epsilon(&mut self, ctx: &dyn EpsilonContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_annotation(&mut self, ctx: &dyn AnnotationContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_compilation_unit(&mut self, ctx: &dyn CompilationUnitContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_element(&mut self, ctx: &dyn ElementContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_block(&mut self, ctx: &dyn BlockContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_grammar_name(&mut self, ctx: &dyn GrammarNameContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_regular(&mut self, ctx: &dyn RegularContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_attribute(&mut self, ctx: &dyn AttributeContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_parser_rule(&mut self, ctx: &dyn ParserRuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_alternative(&mut self, ctx: &dyn AlternativeContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  
  fn visit_ebnf_suffix(&mut self, ctx: &dyn EbnfSuffixContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    self.visit_children(ctx.as_rule())
  }
  

  
  fn visit(&mut self, ast: &RuleContext) -> Result<Box<dyn Any>, Box<dyn Error>> {
    match ast.get_rule_index() {
      
      ChiruParser::LEXER_RULE => self.visit_lexer_rule(ast),
      ChiruParser::RULES => self.visit_rules(ast),
      ChiruParser::ATTRIBUTES => self.visit_attributes(ast),
      ChiruParser::EPSILON => self.visit_epsilon(ast),
      ChiruParser::ANNOTATION => self.visit_annotation(ast),
      ChiruParser::COMPILATION_UNIT => self.visit_compilation_unit(ast),
      ChiruParser::ELEMENT => self.visit_element(ast),
      ChiruParser::BLOCK => self.visit_block(ast),
      ChiruParser::GRAMMAR_NAME => self.visit_grammar_name(ast),
      ChiruParser::REGULAR => self.visit_regular(ast),
      ChiruParser::ATTRIBUTE => self.visit_attribute(ast),
      ChiruParser::PARSER_RULE => self.visit_parser_rule(ast),
      ChiruParser::ALTERNATIVE => self.visit_alternative(ast),
      ChiruParser::EBNF_SUFFIX => self.visit_ebnf_suffix(ast),

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







