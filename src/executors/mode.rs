use crate::mode::{Mode, switch_mode};
use crate::executors::traits::Executor;

pub struct ModeExecutor;

impl Executor for ModeExecutor {
  fn execute(&self, input: &str) -> Result<bool, String> {
    match input {
      "default" => {
        switch_mode(Mode::Default);
        return Ok(true);
      }
      "table" => {
        switch_mode(Mode::Table);
        return Ok(true);
      }
      "simplify" => {
        switch_mode(Mode::Simplify);
        return Ok(true);
      }
      _ => {}
    }

    Ok(false)
  }
}
