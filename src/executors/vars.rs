use colored::Colorize;

use crate::executors::traits::Executor;
use crate::variables::Variables;

pub struct VarsExecutor;

impl Executor for VarsExecutor {
  fn execute(&self, input: &str) -> Result<bool, &str> {
    if input.eq("vars") {
      let variables = Variables::get_instance().read().unwrap();
      let vars_map = variables.get_all_vars();
      for (name, value) in vars_map {
        println!(
          "{}{}{}",
          name.bold(),
          "=".bright_black(),
          value
        );
      }
      return Result::Ok(true);
    }

    Result::Ok(false)
  }
}

