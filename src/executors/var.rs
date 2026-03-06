use crate::executors::traits::Executor;
use crate::global::EQUAL_SYM;
use crate::variables::Variables;
use crate::utils::validate_var_name;

pub struct VarExecutor;

impl Executor for VarExecutor {
  fn execute(&self, input: &str) -> Result<bool, String> {
    if input.contains(EQUAL_SYM) && !input.ends_with(EQUAL_SYM) {
      let v: Vec<&str> = input.split(EQUAL_SYM).collect();
      let name_str: &str = v.get(0).unwrap_or(&"");
      let value: &str = v.get(1).unwrap_or(&"");
      if name_str.len() == 0 || value.len() == 0 {
        return Ok(true);
      }

      if !validate_var_name(name_str) {
        eprintln!("Cannot create variable {}: Invalid variable name.", name_str);
        return Ok(true);
      }

      let name = name_str.chars().next().unwrap();
      Variables::get_instance().write().unwrap().create_var(&name, value)?;
      return Ok(true);
    }

    Ok(false)
  }
}
