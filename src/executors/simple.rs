use colored::Colorize;

use crate::executors::traits::Executor;

pub struct SimpleExecutor;

impl Executor for SimpleExecutor {
  fn execute(&self, input: &str) -> Result<bool, String> {
    println!("{}", input.italic().bright_black());
    Ok(false)
  }
}
