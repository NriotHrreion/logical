pub mod traits;
mod simple;
mod exit;
mod var;
mod vars;

use crate::executors::{
  traits::Executor,
  simple::SimpleExecutor,
  exit::ExitExecutor,
  var::VarExecutor,
  vars::VarsExecutor,
};

pub fn get_executors() -> Vec<Box<dyn Executor>> {
  vec![
    Box::new(SimpleExecutor),
    Box::new(ExitExecutor),
    Box::new(VarExecutor),
    Box::new(VarsExecutor),
  ]
}
