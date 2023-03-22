use crate::syntaxis::ast::ASTListener;



pub trait DotListener: ASTListener {
  fn enter_graph(&self) {}

  fn enter_stmt_list(&self) {}

  fn enter_stmt(&self) {}

  fn enter_attr_stmt(&self) {}

  fn enter_attr_list(&self) {}

  fn enter_a_list(&self) {}

  fn enter_assign_stmt(&self) {}

  fn enter_edge_rhs(&self) {}

  fn enter_edge_op(&self) {}

  fn enter_node_stmt(&self) {}

  fn enter_node_id(&self) {}

  fn enter_compass_pt(&self) {}

  fn enter_subgraph(&self) {}

  fn enter_id(&self) {}

  fn enter_lexpr(&self) {}

  fn enter_rexpr(&self) {}

  fn exit_graph(&self) {}

  fn exit_stmt_list(&self) {}

  fn exit_stmt(&self) {}

  fn exit_attr_stmt(&self) {}

  fn exit_attr_list(&self) {}

  fn exit_a_list(&self) {}

  fn exit_assign_stmt(&self) {}

  fn exit_edge_rhs(&self) {}

  fn exit_edge_op(&self) {}

  fn exit_node_stmt(&self) {}

  fn exit_node_id(&self) {}

  fn exit_compass_pt(&self) {}

  fn exit_subgraph(&self) {}

  fn exit_id(&self) {}

  fn exit_lexpr(&self) {}

  fn exit_rexpr(&self) {}
  
}



