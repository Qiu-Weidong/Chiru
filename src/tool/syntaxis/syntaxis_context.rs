use std::rc::Rc;

use crate::runtime::ast::terminal_context::TerminalContext;
// use crate::runtime::lexer::Lexer;
use crate::tool::syntaxis::syntaxis_visitor::SyntaxisAcceptor;
use crate::runtime::ast::to_rule::ToRule;

pub trait RuleListContext: SyntaxisAcceptor + ToRule {
  fn parser_rule_list(&self) -> Vec<Rc<dyn ParserRuleContext>>;

  fn lexer_rule_list(&self) -> Vec<Rc<dyn LexerRuleContext>>;
}

pub trait ParserRuleContext: SyntaxisAcceptor + ToRule {
  fn rule_ref(&self) -> Option<Rc<TerminalContext>>;

  fn colon(&self) -> Option<Rc<TerminalContext>>;

  fn block(&self) -> Option<Rc<dyn BlockContext>>;

  fn semi(&self) -> Option<Rc<TerminalContext>>;
}

pub trait BlockContext: SyntaxisAcceptor + ToRule {
  fn alternative_list(&self) -> Vec<Rc<dyn AlternativeContext>>;
  
}

pub trait AlternativeContext: SyntaxisAcceptor + ToRule {
  
}

pub trait EpsilonContext: SyntaxisAcceptor + ToRule {
  
}

pub trait ElementContext: SyntaxisAcceptor + ToRule {
  
}

pub trait EbnfSuffixContext: SyntaxisAcceptor + ToRule {
  
}

pub trait LexerRuleContext: SyntaxisAcceptor + ToRule {
  
}

pub trait RegularContext: SyntaxisAcceptor + ToRule {
  
}

