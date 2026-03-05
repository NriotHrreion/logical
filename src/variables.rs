use std::{collections::HashMap, sync::{RwLock, OnceLock}};

pub struct Variables {
  vars: HashMap<String, String>
}

impl Variables {
  fn new() -> Self {
    Variables {
      vars: HashMap::new()
    }
  }

  pub fn create_var(&mut self, name: &str, value: &str) -> Result<(), &str> {
    if self.vars.contains_key(name) {
      return Result::Err("Variable already exists.");
    }
    self.vars.insert(name.to_string(), value.to_string());
    Result::Ok(())
  }

  pub fn get_all_vars(&self) -> &HashMap<String, String> {
    &self.vars
  }

  pub fn get_instance() -> &'static RwLock<Variables> {
    static INSTANCE: OnceLock<RwLock<Variables>> = OnceLock::new();
    
    INSTANCE.get_or_init(|| {
      RwLock::new(Variables::new())
    })
  }
}
