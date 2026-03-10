use logical::logic::eval::eval_ast;
use logical::logic::parser::parse_to_ast;
use logical::mode::Mode;

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
      let expr: &str = v.get(1).unwrap_or(&"");
      if name_str.len() == 0 || expr.len() == 0 {
        return Ok(true);
      }

      match validate_var_name(name_str) {
        Ok(()) => {}
        Err(reason) => {
          eprintln!("Cannot create variable {}: {}", name_str, reason);
          return Ok(true);
        }
      }

      let mut variables = Variables::get_instance().write().unwrap();
      let ast = parse_to_ast(expr, Mode::Default, false)?;
      let val = eval_ast(ast, variables.get_all_vars())?;

      let name = name_str.chars().next().unwrap();
      variables.set_var(&name, val);
      return Ok(true);
    }

    Ok(false)
  }
}
