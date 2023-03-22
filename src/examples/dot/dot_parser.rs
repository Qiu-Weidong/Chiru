// use std::rc::Rc;

// use crate::syntaxis::ast::{ASTContext, RuleContext};

// struct GraphContext {
//   pub parent: Option<Rc<RuleContext>>,
//   pub children: Vec<Rc<dyn ASTContext>>,

// }

// impl ASTContext for GraphContext {
//   fn get_parent(&self) -> Option<std::rc::Rc<dyn RuleContext>> {
//     todo!()
//   }

//   fn set_parent(&mut self, parent: &dyn RuleContext) {
//     todo!()
//   }

//   fn get_text(&self) -> &str {
//     todo!()
//   }

//   fn as_any(&self) -> &dyn std::any::Any {
//     todo!()
//   }

//   fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//     todo!()
//   }

//   fn accept(
//     &self,
//     visitor: &dyn crate::syntaxis::ast::ASTVisitor,
//   ) -> std::rc::Rc<dyn std::any::Any> {
//     todo!()
//   }
// }

// impl RuleContext for GraphContext {
//   fn add_child(&mut self, child: std::rc::Rc<dyn crate::syntaxis::ast::ASTContext>) {
//     todo!()
//   }

//   fn get_child(&self) -> Option<std::rc::Rc<dyn crate::syntaxis::ast::ASTContext>> {
//     todo!()
//   }

//   fn get_children(&self) -> Vec<std::rc::Rc<dyn crate::syntaxis::ast::ASTContext>> {
//     todo!()
//   }

//   fn get_child_count(&self) -> i32 {
//     todo!()
//   }

//   fn get_start_token(&self) -> std::rc::Rc<crate::syntaxis::ast::TerminalContext> {
//     todo!()
//   }

//   fn get_stop_token(&self) -> std::rc::Rc<crate::syntaxis::ast::TerminalContext> {
//     todo!()
//   }

//   fn get_token(
//     &self,
//     token_type: usize,
//     i: usize,
//   ) -> Option<std::rc::Rc<crate::syntaxis::ast::TerminalContext>> {
//     todo!()
//   }

//   fn get_tokens(
//     &self,
//     token_type: usize,
//   ) -> Vec<std::rc::Rc<crate::syntaxis::ast::TerminalContext>> {
//     todo!()
//   }

//   fn get_errornode(
//     &self,
//     token_type: usize,
//     i: usize,
//   ) -> Option<std::rc::Rc<crate::syntaxis::ast::TerminalContext>> {
//     todo!()
//   }

//   fn get_errornodes(
//     &self,
//     token_type: usize,
//   ) -> Vec<std::rc::Rc<crate::syntaxis::ast::TerminalContext>> {
//     todo!()
//   }

//   fn get_rule_context(
//     &self,
//     rule_type: usize,
//     index: usize,
//   ) -> Option<std::rc::Rc<dyn RuleContext>> {
//     todo!()
//   }

//   fn get_rule_contexts(&self, rule_type: usize) -> Vec<std::rc::Rc<dyn RuleContext>> {
//     todo!()
//   }

//     // fn get_rule_index(&self) -> i32 { -1 }

//     // fn enter_rule(&self) {}

//     // fn exit_rule(&self) {}
// }




