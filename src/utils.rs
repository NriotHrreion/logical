use crate::global::EQUAL_SYM;

pub fn validate_var_name(name_str: &str) -> bool {
  if name_str.len() != 1 {
    return false;
  }

  let name = name_str.chars().next().unwrap();
  if !name.is_alphabetic() && name_str != "_" && name_str != "$" {
    return false;
  }
  if name == EQUAL_SYM || name_str == " " || name_str == "<" || name_str == ">" || name_str == "-" {
    return false;
  }

  true
}
