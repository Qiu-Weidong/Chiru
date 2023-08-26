use std::any::Any;


use crate::runtime::ast::rule_context::RuleContext;
use crate::runtime::ast::terminal_context::TerminalContext;
use crate::runtime::ast::to_rule::ToRule;

use super::syntaxis_lexer::SyntaxisLexer;
use super::syntaxis_listener::SyntaxisListener;
use super::syntaxis_parser::SyntaxisParser;
use super::syntaxis_visitor::SyntaxisVisitor;

pub trait RuleListContext: ToRule {
  fn parser_rule_list(&self) -> Vec<&dyn ParserRuleContext>;

  fn lexer_rule_list(&self) -> Vec<&dyn LexerRuleContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;

  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}

pub trait ParserRuleContext: ToRule {
  fn rule_ref(&self) -> Option<&TerminalContext>;

  fn colon(&self) -> Option<&TerminalContext>;

  fn block(&self) -> Option<&dyn BlockContext>;

  fn semi(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}

pub trait BlockContext: ToRule {
  fn alternative_list(&self) -> Vec<&dyn AlternativeContext>;
  
  fn or_list(&self) -> Vec<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}

pub trait AlternativeContext: ToRule {
  fn element_list(&self) -> Vec<&dyn ElementContext>;

  fn epsilon(&self) -> Option<&dyn EpsilonContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}

pub trait EpsilonContext: ToRule {
  fn epsilon(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}

pub trait ElementContext: ToRule {
  fn token_ref(&self) -> Option<&TerminalContext>;
  fn string_literal(&self) -> Option<&TerminalContext>;
  fn rule_ref(&self) -> Option<&TerminalContext>;
  fn lparen(&self) -> Option<&TerminalContext>;
  fn rparen(&self) -> Option<&TerminalContext>;

  fn block(&self) -> Option<&dyn BlockContext>;
  fn ebnf_suffix(&self) -> Option<&dyn EbnfSuffixContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}

pub trait EbnfSuffixContext: ToRule {
  fn star(&self) -> Option<&TerminalContext>;
  fn plus(&self) -> Option<&TerminalContext>;
  fn question_list(&self) -> Vec<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}

pub trait LexerRuleContext: ToRule {
  fn token_ref(&self) -> Option<&TerminalContext>;
  fn colon(&self) -> Option<&TerminalContext>;
  fn semi(&self) -> Option<&TerminalContext>;

  fn regular(&self) -> Option<&dyn RegularContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}

pub trait RegularContext: ToRule {
  fn regular_literal(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn SyntaxisListener);
  fn exit(&self, listener: &mut dyn SyntaxisListener);
}






impl RuleListContext for RuleContext {
  fn parser_rule_list(&self) -> Vec<&dyn ParserRuleContext> {
    let mut result = Vec::new();
    for ctx in self.get_rule_contexts(SyntaxisParser::PARSER_RULE).iter() {
      result.push(*ctx as &dyn ParserRuleContext);
    }
    result
  }

  fn lexer_rule_list(&self) -> Vec<&dyn LexerRuleContext> {
    let mut result = Vec::new();
    for ctx in self.get_rule_contexts(SyntaxisParser::LEXER_RULE).iter() {
      result.push(*ctx as &dyn LexerRuleContext);
    }
    result
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_rule_list(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_rule_list(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_rule_list(self)
  }
}

impl ParserRuleContext for RuleContext {
  fn rule_ref(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::RULE_REF, 0)
  }

  fn colon(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::COLON, 0)
  }

  fn block(&self) -> Option<&dyn BlockContext> {
    if let Some(result) = self.get_rule_context(SyntaxisParser::BLOCK, 0) {
      Some(result as &dyn BlockContext)
    } else { None }
  }

  fn semi(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::SEMI, 0)
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_parser_rule(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_parser_rule(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_parser_rule(self)
  }
}

impl LexerRuleContext for RuleContext {
  fn token_ref(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::TOKEN_REF, 0)
  }

  fn colon(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::COLON, 0)
  }

  fn semi(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::SEMI, 0)
  }

  fn regular(&self) -> Option<&dyn RegularContext> {
    self.get_rule_context(SyntaxisParser::REGULAR, 0).map(|ctx| ctx as &dyn RegularContext)
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_lexer_rule(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_lexer_rule(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_lexer_rule(self)
  }
}

impl BlockContext for RuleContext {
  fn alternative_list(&self) -> Vec<&dyn AlternativeContext> {
    self.get_rule_contexts(SyntaxisParser::ALTERNATIVE).iter().map(|ctx| *ctx as &dyn AlternativeContext).collect::<Vec<_>>()
  }

  fn or_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(SyntaxisLexer::OR)
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_block(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_block(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_block(self)
  }
}

impl AlternativeContext for RuleContext {
  fn element_list(&self) -> Vec<&dyn ElementContext> {
    let mut result = Vec::new();
    for ctx in self.get_rule_contexts(SyntaxisParser::ELEMENT).iter() {
      result.push(*ctx as &dyn ElementContext);
    }
    result
  }

  fn epsilon(&self) -> Option<&dyn EpsilonContext> {
    if let Some(result) = self.get_rule_context(SyntaxisParser::EPSILON, 0) {
      Some(result as &dyn EpsilonContext)
    } else { None }
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_alternative(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_alternative(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_alternative(self)
  }
}

impl EpsilonContext for RuleContext {
  fn epsilon(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::EPSILON, 0)
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_epsilon(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_epsilon(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_epsilon(self)
  }
}

impl ElementContext for RuleContext {
  fn token_ref(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::TOKEN_REF, 0)
  }

  fn string_literal(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::STRING_LITERAL, 0)
  }

  fn rule_ref(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::RULE_REF, 0)
  }

  fn lparen(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::LPAREN, 0)
  }

  fn rparen(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::RPAREN, 0)
  }

  fn block(&self) -> Option<&dyn BlockContext> {
    if let Some(result) = self.get_rule_context(SyntaxisParser::BLOCK, 0) {
      Some(result as &dyn BlockContext)
    } else { None }
  }

  fn ebnf_suffix(&self) -> Option<&dyn EbnfSuffixContext> {
    if let Some(result) = self.get_rule_context(SyntaxisParser::EBNF_SUFFIX, 0) {
      Some(result as &dyn EbnfSuffixContext)
    } else { None }
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_element(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_element(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_element(self)
  }
}

impl EbnfSuffixContext for RuleContext {
  fn star(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::STAR, 0)
  }

  fn plus(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::PLUS, 0)
  }

  fn question_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(SyntaxisLexer::QUESTION)
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_ebnf_suffix(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_ebnf_suffix(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_ebnf_suffix(self)
  }
}

impl RegularContext for RuleContext {
  fn regular_literal(&self) -> Option<&TerminalContext> {
    self.get_terminal(SyntaxisLexer::REGULAR_LITERAL, 0)
  }

  fn accept(&self, visitor: &mut dyn SyntaxisVisitor) -> Box<dyn Any> {
    visitor.visit_regular(self)
  }

  fn enter(&self, listener: &mut dyn SyntaxisListener) {
    listener.enter_regular(self)
  }

  fn exit(&self, listener: &mut dyn SyntaxisListener) {
    listener.exit_regular(self)
  }
}
