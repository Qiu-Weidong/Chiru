use std::any::Any;


use chiru::runtime::ast::rule_context::RuleContext;
use chiru::runtime::ast::terminal_context::TerminalContext;
use chiru::runtime::ast::to_rule::ToRule;


use super::chiru_lexer::ChiruLexer;
use super::chiru_parser::ChiruParser;
use super::chiru_visitor::ChiruVisitor;
use super::chiru_listener::ChiruListener;



pub trait AlternativeContext: ToRule {
  
  fn element_list(&self) -> Vec<&dyn ElementContext>;

  

  
  fn epsilon(&self) -> Option<&dyn EpsilonContext>;

  

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl AlternativeContext for RuleContext {

  
  fn element_list(&self) -> Vec<&dyn ElementContext> {
    self.get_rule_contexts(ChiruParser::ELEMENT).iter().map(|ctx| *ctx as &dyn ElementContext).collect::<Vec<_>>()
  } 

  

  
  fn epsilon(&self) -> Option<&dyn EpsilonContext> {
    self.get_rule_context(ChiruParser::EPSILON, 0).map(|ctx| ctx as &dyn EpsilonContext)
  } 

  


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_alternative(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_alternative(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_alternative(self)
  }
}


pub trait BlockContext: ToRule {
  
  fn alternative_list(&self) -> Vec<&dyn AlternativeContext>;

  
  fn or_list(&self) -> Vec<&TerminalContext>;

  

  

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl BlockContext for RuleContext {

  
  fn alternative_list(&self) -> Vec<&dyn AlternativeContext> {
    self.get_rule_contexts(ChiruParser::ALTERNATIVE).iter().map(|ctx| *ctx as &dyn AlternativeContext).collect::<Vec<_>>()
  } 

  
  fn or_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(ChiruLexer::OR)
  } 

  

  


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_block(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_block(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_block(self)
  }
}


pub trait AttributeListContext: ToRule {
  
  fn attribute_list(&self) -> Vec<&dyn AttributeContext>;

  
  fn comma_list(&self) -> Vec<&TerminalContext>;

  

  

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl AttributeListContext for RuleContext {

  
  fn attribute_list(&self) -> Vec<&dyn AttributeContext> {
    self.get_rule_contexts(ChiruParser::ATTRIBUTE).iter().map(|ctx| *ctx as &dyn AttributeContext).collect::<Vec<_>>()
  } 

  
  fn comma_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(ChiruLexer::COMMA)
  } 

  

  


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_attribute_list(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_attribute_list(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_attribute_list(self)
  }
}


pub trait ElementContext: ToRule {
  

  

  
  fn ebnf_suffix(&self) -> Option<&dyn EbnfSuffixContext>;
  fn block(&self) -> Option<&dyn BlockContext>;

  
  fn token_ref(&self) -> Option<&TerminalContext>;
  fn rparen(&self) -> Option<&TerminalContext>;
  fn string_literal(&self) -> Option<&TerminalContext>;
  fn rule_ref(&self) -> Option<&TerminalContext>;
  fn lparen(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl ElementContext for RuleContext {

  

  

  
  fn ebnf_suffix(&self) -> Option<&dyn EbnfSuffixContext> {
    self.get_rule_context(ChiruParser::EBNF_SUFFIX, 0).map(|ctx| ctx as &dyn EbnfSuffixContext)
  } 
  fn block(&self) -> Option<&dyn BlockContext> {
    self.get_rule_context(ChiruParser::BLOCK, 0).map(|ctx| ctx as &dyn BlockContext)
  } 

  
  fn token_ref(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::TOKEN_REF, 0)
  } 
  fn rparen(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::RPAREN, 0)
  } 
  fn string_literal(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::STRING_LITERAL, 0)
  } 
  fn rule_ref(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::RULE_REF, 0)
  } 
  fn lparen(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::LPAREN, 0)
  } 


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_element(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_element(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_element(self)
  }
}


pub trait LexerRuleContext: ToRule {
  

  

  
  fn annotation(&self) -> Option<&dyn AnnotationContext>;
  fn regular(&self) -> Option<&dyn RegularContext>;

  
  fn colon(&self) -> Option<&TerminalContext>;
  fn semi(&self) -> Option<&TerminalContext>;
  fn token_ref(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl LexerRuleContext for RuleContext {

  

  

  
  fn annotation(&self) -> Option<&dyn AnnotationContext> {
    self.get_rule_context(ChiruParser::ANNOTATION, 0).map(|ctx| ctx as &dyn AnnotationContext)
  } 
  fn regular(&self) -> Option<&dyn RegularContext> {
    self.get_rule_context(ChiruParser::REGULAR, 0).map(|ctx| ctx as &dyn RegularContext)
  } 

  
  fn colon(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::COLON, 0)
  } 
  fn semi(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::SEMI, 0)
  } 
  fn token_ref(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::TOKEN_REF, 0)
  } 


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_lexer_rule(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_lexer_rule(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_lexer_rule(self)
  }
}


pub trait ParserRuleContext: ToRule {
  

  

  
  fn block(&self) -> Option<&dyn BlockContext>;

  
  fn rule_ref(&self) -> Option<&TerminalContext>;
  fn colon(&self) -> Option<&TerminalContext>;
  fn semi(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl ParserRuleContext for RuleContext {

  

  

  
  fn block(&self) -> Option<&dyn BlockContext> {
    self.get_rule_context(ChiruParser::BLOCK, 0).map(|ctx| ctx as &dyn BlockContext)
  } 

  
  fn rule_ref(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::RULE_REF, 0)
  } 
  fn colon(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::COLON, 0)
  } 
  fn semi(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::SEMI, 0)
  } 


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_parser_rule(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_parser_rule(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_parser_rule(self)
  }
}


pub trait RuleListContext: ToRule {
  
  fn lexer_rule_list(&self) -> Vec<&dyn LexerRuleContext>;
  fn parser_rule_list(&self) -> Vec<&dyn ParserRuleContext>;

  

  

  

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl RuleListContext for RuleContext {

  
  fn lexer_rule_list(&self) -> Vec<&dyn LexerRuleContext> {
    self.get_rule_contexts(ChiruParser::LEXER_RULE).iter().map(|ctx| *ctx as &dyn LexerRuleContext).collect::<Vec<_>>()
  } 
  fn parser_rule_list(&self) -> Vec<&dyn ParserRuleContext> {
    self.get_rule_contexts(ChiruParser::PARSER_RULE).iter().map(|ctx| *ctx as &dyn ParserRuleContext).collect::<Vec<_>>()
  } 

  

  

  


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_rule_list(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_rule_list(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_rule_list(self)
  }
}


pub trait EbnfSuffixContext: ToRule {
  

  
  fn question_list(&self) -> Vec<&TerminalContext>;

  

  
  fn star(&self) -> Option<&TerminalContext>;
  fn plus(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl EbnfSuffixContext for RuleContext {

  

  
  fn question_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(ChiruLexer::QUESTION)
  } 

  

  
  fn star(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::STAR, 0)
  } 
  fn plus(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::PLUS, 0)
  } 


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_ebnf_suffix(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_ebnf_suffix(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_ebnf_suffix(self)
  }
}


pub trait EpsilonContext: ToRule {
  

  

  

  
  fn epsilon(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl EpsilonContext for RuleContext {

  

  

  

  
  fn epsilon(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::EPSILON, 0)
  } 


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_epsilon(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_epsilon(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_epsilon(self)
  }
}


pub trait AttributeContext: ToRule {
  

  
  fn token_ref_list(&self) -> Vec<&TerminalContext>;
  fn rule_ref_list(&self) -> Vec<&TerminalContext>;

  

  
  fn rparen(&self) -> Option<&TerminalContext>;
  fn lparen(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl AttributeContext for RuleContext {

  

  
  fn token_ref_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(ChiruLexer::TOKEN_REF)
  } 
  fn rule_ref_list(&self) -> Vec<&TerminalContext> {
    self.get_terminals(ChiruLexer::RULE_REF)
  } 

  

  
  fn rparen(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::RPAREN, 0)
  } 
  fn lparen(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::LPAREN, 0)
  } 


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_attribute(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_attribute(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_attribute(self)
  }
}


pub trait RegularContext: ToRule {
  

  

  

  
  fn regular_literal(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl RegularContext for RuleContext {

  

  

  

  
  fn regular_literal(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::REGULAR_LITERAL, 0)
  } 


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_regular(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_regular(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_regular(self)
  }
}


pub trait AnnotationContext: ToRule {
  

  

  
  fn attribute_list(&self) -> Option<&dyn AttributeListContext>;
  fn attribute(&self) -> Option<&dyn AttributeContext>;

  
  fn sharp(&self) -> Option<&TerminalContext>;
  fn lbracket(&self) -> Option<&TerminalContext>;
  fn at(&self) -> Option<&TerminalContext>;
  fn rbracket(&self) -> Option<&TerminalContext>;

  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any>;
  fn enter(&self, listener: &mut dyn ChiruListener);
  fn exit(&self, listener: &mut dyn ChiruListener);
}

impl AnnotationContext for RuleContext {

  

  

  
  fn attribute_list(&self) -> Option<&dyn AttributeListContext> {
    self.get_rule_context(ChiruParser::ATTRIBUTE_LIST, 0).map(|ctx| ctx as &dyn AttributeListContext)
  } 
  fn attribute(&self) -> Option<&dyn AttributeContext> {
    self.get_rule_context(ChiruParser::ATTRIBUTE, 0).map(|ctx| ctx as &dyn AttributeContext)
  } 

  
  fn sharp(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::SHARP, 0)
  } 
  fn lbracket(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::LBRACKET, 0)
  } 
  fn at(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::AT, 0)
  } 
  fn rbracket(&self) -> Option<&TerminalContext> {
    self.get_terminal(ChiruLexer::RBRACKET, 0)
  } 


  fn accept(&self, visitor: &mut dyn ChiruVisitor) -> Box<dyn Any> {
    visitor.visit_annotation(self)
  }

  fn enter(&self, listener: &mut dyn ChiruListener) {
    listener.enter_annotation(self)
  }

  fn exit(&self, listener: &mut dyn ChiruListener) {
    listener.exit_annotation(self)
  }
}




