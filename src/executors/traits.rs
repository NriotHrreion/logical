pub trait Executor {
  fn execute(&self, input: &str) -> Result<bool, &str>;
}
