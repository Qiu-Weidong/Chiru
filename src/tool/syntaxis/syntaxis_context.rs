use std::rc::Rc;

use crate::runtime::ast::rule_context::RuleContext;
use crate::runtime::ast::terminal_context::TerminalContext;
use crate::tool::syntaxis::syntaxis_visitor::SyntaxisAcceptor;
use crate::runtime::ast::to_rule::ToRule;

use super::syntaxis_parser::SyntaxisParser;

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
  
  fn or_list(&self) -> Vec<Rc<TerminalContext>>;
}

pub trait AlternativeContext: SyntaxisAcceptor + ToRule {
  fn element_list(&self) -> Vec<Rc<dyn ElementContext>>;

  fn epsilon(&self) -> Option<Rc<dyn EpsilonContext>>;
}

pub trait EpsilonContext: SyntaxisAcceptor + ToRule {
  fn epsilon(&self) -> Option<Rc<TerminalContext>>;
}

pub trait ElementContext: SyntaxisAcceptor + ToRule {
  fn token_ref(&self) -> Option<Rc<TerminalContext>>;
  fn string_literal(&self) -> Option<Rc<TerminalContext>>;
  fn rule_ref(&self) -> Option<Rc<TerminalContext>>;
  fn lparen(&self) -> Option<Rc<TerminalContext>>;
  fn rparen(&self) -> Option<Rc<TerminalContext>>;

  fn block(&self) -> Option<Rc<dyn BlockContext>>;
  fn ebnf_suffix(&self) -> Option<Rc<dyn EbnfSuffixContext>>;
}

pub trait EbnfSuffixContext: SyntaxisAcceptor + ToRule {
  fn star(&self) -> Option<Rc<TerminalContext>>;
  fn plus(&self) -> Option<Rc<TerminalContext>>;
  fn question_list(&self) -> Vec<Rc<TerminalContext>>;
}

pub trait LexerRuleContext: SyntaxisAcceptor + ToRule {
  fn token_ref(&self) -> Option<Rc<TerminalContext>>;
  fn colon(&self) -> Option<Rc<TerminalContext>>;
  fn semi(&self) -> Option<Rc<TerminalContext>>;

  fn regular(&self) -> Option<Rc<dyn RegularContext>>;
}

pub trait RegularContext: SyntaxisAcceptor + ToRule {
  fn regular_literal(&self) -> Option<Rc<TerminalContext>>;
}






impl RuleListContext for RuleContext {
  fn parser_rule_list(&self) -> Vec<Rc<dyn ParserRuleContext>> {
    let mut result = Vec::new();
    for ctx in self.get_rule_contexts(SyntaxisParser::PARSER_RULE).iter() {
      result.push(Rc::clone(ctx) as Rc<dyn ParserRuleContext>);
    }
    result
  }

  fn lexer_rule_list(&self) -> Vec<Rc<dyn LexerRuleContext>> {
    todo!()
  }
}

impl ParserRuleContext for RuleContext {
  fn rule_ref(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn colon(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn block(&self) -> Option<Rc<dyn BlockContext>> {
    todo!()
  }

  fn semi(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }
}

impl LexerRuleContext for RuleContext {
  fn token_ref(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn colon(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn semi(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn regular(&self) -> Option<Rc<dyn RegularContext>> {
    todo!()
  }
}

impl BlockContext for RuleContext {
  fn alternative_list(&self) -> Vec<Rc<dyn AlternativeContext>> {
    todo!()
  }

  fn or_list(&self) -> Vec<Rc<TerminalContext>> {
    todo!()
  }
}

impl AlternativeContext for RuleContext {
  fn element_list(&self) -> Vec<Rc<dyn ElementContext>> {
    todo!()
  }

  fn epsilon(&self) -> Option<Rc<dyn EpsilonContext>> {
    todo!()
  }
}

impl EpsilonContext for RuleContext {
  fn epsilon(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }
}

impl ElementContext for RuleContext {
  fn token_ref(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn string_literal(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn rule_ref(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn lparen(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn rparen(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn block(&self) -> Option<Rc<dyn BlockContext>> {
    todo!()
  }

  fn ebnf_suffix(&self) -> Option<Rc<dyn EbnfSuffixContext>> {
    todo!()
  }
}

impl EbnfSuffixContext for RuleContext {
  fn star(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn plus(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }

  fn question_list(&self) -> Vec<Rc<TerminalContext>> {
    todo!()
  }
}

impl RegularContext for RuleContext {
  fn regular_literal(&self) -> Option<Rc<TerminalContext>> {
    todo!()
  }
}
