use std::any::Any;

// 定义一个 ToAny trait , 考虑使用 #[derived(ToAny)] 自动实现
pub trait ToAny {
  fn as_any(&self) -> &dyn Any; 

  fn as_any_mut(&mut self) ->  &mut dyn Any; 
}





