use std::process::exit;

use crate::executors::traits::Executor;

pub struct ExitExecutor;

impl Executor for ExitExecutor {
  fn execute(&self, input: &str) -> Result<bool, String> {
    if input.eq("exit") || input.eq("quit") {
      exit(0);
    }

    Ok(false)
  }
}
