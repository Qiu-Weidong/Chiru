// use std::any::Any;

// use crate::syntaxis::ast::ASTListener;
// use super::expr_parser::{ProgContext, StatContext, ExprContext};


// pub trait AbstractExprListener: ASTListener {
//   fn enter_prog(&self, _ctx: &dyn ProgContext) {}

//   fn enter_stat(&self, _ctx: &dyn StatContext) {}

//   fn enter_expr(&self, _ctx: &dyn ExprContext) {}


//   fn exit_prog(&self, _ctx: &dyn ProgContext) {}

//   fn exit_stat(&self, _ctx: &dyn StatContext) {}

//   fn exit_expr(&self, _ctx: &dyn ExprContext) {}
// }


// pub struct ExprListener {
//   // 定义需要的数据结构
// }

// impl ASTListener for ExprListener {
//   fn as_any(&self) -> &dyn Any {
//     self as &dyn Any 
//   }

//   fn as_any_mut(&mut self) -> &mut dyn Any {
//     self as &mut dyn Any 
//   }

// }

// impl AbstractExprListener for ExprListener {}






