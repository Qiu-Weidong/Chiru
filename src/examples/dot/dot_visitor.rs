// use std::{rc::Rc, any::Any};

// use crate::syntaxis::ast::ASTVisitor;


// pub trait AbstractDotVisitor : ASTVisitor {

//   fn visit_graph(&self) -> Rc<dyn Any>;  

//   fn visit_stmt_list(&self) -> Rc<dyn Any>;  

//   fn visit_stmt(&self) -> Rc<dyn Any>;  

//   fn visit_attr_stmt(&self) -> Rc<dyn Any>;  

//   fn visit_attr_list(&self) -> Rc<dyn Any>;  

//   fn visit_a_list(&self) -> Rc<dyn Any>;  

//   fn visit_assign_stmt(&self) -> Rc<dyn Any>;  

//   fn visit_edge_rhs(&self) -> Rc<dyn Any>;  

//   fn visit_edge_op(&self) -> Rc<dyn Any>;  

//   fn visit_node_stmt(&self) -> Rc<dyn Any>;  

//   fn visit_node_id(&self) -> Rc<dyn Any>;  

//   fn visit_compass_pt(&self) -> Rc<dyn Any>;  

//   fn visit_subgraph(&self) -> Rc<dyn Any>;  

//   fn visit_id(&self) -> Rc<dyn Any>;  

//   fn visit_lexpr(&self) -> Rc<dyn Any>;  

//   fn visit_rexpr(&self) -> Rc<dyn Any>;  
  
// }


// pub struct DotVisitor {
//   // 添加需要的数据结构
// }

// impl ASTVisitor for DotVisitor {
//   fn visit(&self, ast: &dyn crate::syntaxis::ast::ASTContext) -> Rc<dyn Any>  {
//     todo!()
//   }

//   fn visit_children(&self, context: &dyn crate::syntaxis::ast::RuleContext) -> Rc<dyn Any>  {
//     todo!()
//   }

//   fn default_result(&self) -> Rc<dyn Any> {
//     todo!()
//   }
// }
// impl AbstractDotVisitor for DotVisitor {
//   fn visit_graph(&self) -> Rc<dyn Any> {
//     // self.visit_children(self)
//     todo!()
//   }

//   fn visit_stmt_list(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_stmt(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_attr_stmt(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_attr_list(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_a_list(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_assign_stmt(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_edge_rhs(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_edge_op(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_node_stmt(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_node_id(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_compass_pt(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_subgraph(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_id(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_lexpr(&self) -> Rc<dyn Any> {
//     todo!()
//   }

//   fn visit_rexpr(&self) -> Rc<dyn Any> {
//     todo!()
//   }
// }



