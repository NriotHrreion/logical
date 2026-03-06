use crate::{executors::traits::Executor, logic::{parser::parse_to_ast, eval::eval_ast}};

pub struct ExprExecutor;

impl Executor for ExprExecutor {
  fn execute(&self, input: &str) -> Result<bool, String> {
    let ast = parse_to_ast(input)?;
    let val = eval_ast(ast)?;

    if val { println!("1"); }
    else { println!("0"); }

    Ok(true)
  }
}
