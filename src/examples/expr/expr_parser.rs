

// use std::{rc::Rc, any::Any};

// use crate::syntaxis::ast::{Acceptable, ASTListener, ASTVisitor, TerminalContext, RuleContext};

// use super::{expr_listener::{ExprListener, AbstractExprListener}, expr_visitor::{ExprVisitor, AbstractExprVisitor}};

// pub trait ProgContext: Acceptable {
//   fn stat_list(&self) -> Vec<Rc<dyn StatContext>>;

//   fn stat(&self, i: usize) -> Rc<dyn StatContext>;

//   fn get_rule_index(&self) -> usize;

//   fn enter_rule(&self, listener: &dyn ASTListener);

//   fn exit_rule(&self, listener: &dyn ASTListener);

//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any>;
// }

// pub trait StatContext: Acceptable {
//   fn expr(&self) -> Rc<dyn ExprContext>;

//   fn newline(&self) -> Rc<TerminalContext>;

//   fn id(&self) -> Rc<TerminalContext>;

//   fn get_rule_index(&self) -> usize;

//   fn enter_rule(&self, listener: &dyn ASTListener);

//   fn exit_rule(&self, listener: &dyn ASTListener);

//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any>;
// }

// pub trait ExprContext: Acceptable {
//   fn int(&self) -> Rc<TerminalContext>;

//   fn id(&self) -> Rc<TerminalContext>;

//   fn expr_list(&self) -> Vec<Rc<dyn ExprContext>>;

//   fn expr(&self, i: usize) -> Rc<dyn ExprContext>;

//   fn get_rule_index(&self) -> usize;

//   fn enter_rule(&self, listener: &dyn ASTListener);

//   fn exit_rule(&self, listener: &dyn ASTListener);

//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any>;
// }






// impl ProgContext for RuleContext {
//   fn stat_list(&self) -> Vec<Rc<dyn StatContext>> { todo!() }

//   fn stat(&self, _i: usize) -> Rc<dyn StatContext> { todo!() }

//   fn get_rule_index(&self) -> usize { todo!() }

//   fn enter_rule(&self, listener: &dyn ASTListener) { 
//     if listener.as_any().is::<ExprListener>() {
//       listener.as_any().downcast_ref::<ExprListener>().unwrap().enter_prog(self);
//     }
//   }

//   fn exit_rule(&self, listener: &dyn ASTListener) { 
//     if listener.as_any().is::<ExprListener>() {
//       listener.as_any().downcast_ref::<ExprListener>().unwrap().exit_prog(self);
//     }
//   }

//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> { 
//     if visitor.as_any().is::<ExprVisitor>() {
//       visitor.as_any().downcast_ref::<ExprVisitor>().unwrap().visit_prog(self)
//     } else {
//       visitor.visit_children(self)
//     }
//   }
// }

// impl StatContext for RuleContext {
//   fn expr(&self) -> Rc<dyn ExprContext> { todo!() }
//   fn newline(&self) -> Rc<TerminalContext> { todo!() }
//   fn id(&self) -> Rc<TerminalContext> { todo!() }

//   fn get_rule_index(&self) -> usize { todo!() }

//   fn enter_rule(&self, listener: &dyn ASTListener) { 
//     if listener.as_any().is::<ExprListener>() {
//       listener.as_any().downcast_ref::<ExprListener>().unwrap().enter_stat(self);
//     }
//   }

//   fn exit_rule(&self, listener: &dyn ASTListener) { 
//     if listener.as_any().is::<ExprListener>() {
//       listener.as_any().downcast_ref::<ExprListener>().unwrap().exit_stat(self);
//     }
//   }

//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> { 
//     if visitor.as_any().is::<ExprVisitor>() {
//       visitor.as_any().downcast_ref::<ExprVisitor>().unwrap().visit_stat(self)
//     } else {
//       visitor.visit_children(self)
//     }
//   }
// }

// impl ExprContext for RuleContext {
//   fn int(&self) -> Rc<TerminalContext> { todo!() }

//   fn id(&self) -> Rc<TerminalContext> { todo!() }

//   fn expr_list(&self) -> Vec<Rc<dyn ExprContext>> { todo!() }

//   fn expr(&self, _i: usize) -> Rc<dyn ExprContext> { todo!() }

//   fn get_rule_index(&self) -> usize { todo!() }

//   fn enter_rule(&self, listener: &dyn ASTListener) { 
//     if listener.as_any().is::<ExprListener>() {
//       listener.as_any().downcast_ref::<ExprListener>().unwrap().enter_expr(self);
//     }
//   }

//   fn exit_rule(&self, listener: &dyn ASTListener) { 
//     if listener.as_any().is::<ExprListener>() {
//       listener.as_any().downcast_ref::<ExprListener>().unwrap().exit_expr(self);
//     }
//   }

//   fn accept(&self, visitor: &dyn ASTVisitor) -> Box<dyn Any> { 
//     if visitor.as_any().is::<ExprVisitor>() {
//       visitor.as_any().downcast_ref::<ExprVisitor>().unwrap().visit_expr(self)
//     } else {
//       visitor.visit_children(self)
//     }
//   }
// }





