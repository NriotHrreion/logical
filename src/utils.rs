pub fn validate_var_name(name_str: &str) -> Result<(), &str> {
  if name_str.len() != 1 {
    return Err("Length of variable names must be 1.");
  }

  let name = name_str.chars().next().unwrap();
  if !name.is_alphabetic() && name_str != "_" && name_str != "$" {
    return Err("Variable name should be alphabetic, '_' or '$'.");
  }

  Ok(())
}

pub fn implication_forward(val1: bool, val2: bool) -> bool {
  !val1 || val2
}

pub fn implication_reverse(val1: bool, val2: bool) -> bool {
  val1 || !val2
}

pub fn implication_bidirectional(val1: bool, val2: bool) -> bool {
  val1 == val2
}
