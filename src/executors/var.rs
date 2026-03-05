use crate::executors::traits::Executor;
use crate::global::EQUAL_SYM;
use crate::variables::Variables;
use crate::utils::validate_var_name;

pub struct VarExecutor;

impl Executor for VarExecutor {
  fn execute(&self, input: &str) -> Result<bool, &str> {
    if input.contains(EQUAL_SYM) && !input.ends_with(EQUAL_SYM) {
      let v: Vec<&str> = input.split(EQUAL_SYM).collect();
      let name: &str = v.get(0).unwrap_or(&"");
      let value: &str = v.get(1).unwrap_or(&"");
      if name.len() == 0 || value.len() == 0 {
        return Result::Ok(true);
      }

      if !validate_var_name(name) {
        eprintln!("Cannot create variable {}: Invalid variable name.", name);
        return Result::Ok(true);
      }

      match Variables::get_instance().write().unwrap().create_var(name, value) {
        Ok(()) => {}
        Err(e) => {
          eprintln!("Cannot create variable {}: {}", name, e)
        }
      }
      return Result::Ok(true);
    }

    Result::Ok(false)
  }
}
