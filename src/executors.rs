pub mod traits;
mod simple;
mod exit;
mod var;
mod vars;
mod expr;

use crate::executors::{
  traits::Executor,
  exit::ExitExecutor,
  simple::SimpleExecutor,
  var::VarExecutor,
  vars::VarsExecutor,
  expr::ExprExecutor,
};

pub fn get_executors() -> Vec<Box<dyn Executor>> {
  vec![
    Box::new(ExitExecutor),
    Box::new(SimpleExecutor),
    Box::new(VarExecutor),
    Box::new(VarsExecutor),
    Box::new(ExprExecutor)
  ]
}
