use colored::Colorize;

use crate::executors::traits::Executor;
use crate::variables::Variables;

pub struct VarsExecutor;

impl Executor for VarsExecutor {
  fn execute(&self, input: &str) -> Result<bool, String> {
    if input.eq("vars") {
      let variables = Variables::get_instance().read().unwrap();
      let vars_map = variables.get_all_vars();
      for (name, val) in vars_map {
        let val_str = match val {
          true => "1",
          false => "0"
        };
        println!(
          "{}{}{}",
          name.to_string().bold().green(),
          "=".bright_black(),
          val_str
        );
      }
      return Ok(true);
    }

    Ok(false)
  }
}

