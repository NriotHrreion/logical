use crate::global::EQUAL_SYM;

pub fn validate_var_name(name: &str) -> bool {
  if name.len() == 0 {
    return false;
  }
  if !name.starts_with(|c: char| c.is_alphabetic() || c.to_string().eq("_") || c.to_string().eq("$")) {
    return false;
  }
  if name.contains(EQUAL_SYM) || name.contains(" ") || name.contains("<") || name.contains(">") || name.contains("-") {
    return false;
  }

  true
}
