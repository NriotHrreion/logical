use std::{collections::HashMap, sync::{RwLock, OnceLock}};

pub struct Variables {
  vars: HashMap<char, String>
}

impl Variables {
  fn new() -> Self {
    Variables {
      vars: HashMap::new()
    }
  }

  pub fn create_var(&mut self, name: &char, value: &str) -> Result<(), &str> {
    if self.vars.contains_key(name) {
      return Err("Variable already exists.");
    }
    self.vars.insert(*name, value.to_string());
    Ok(())
  }

  pub fn get_all_vars(&self) -> &HashMap<char, String> {
    &self.vars
  }

  pub fn get_instance() -> &'static RwLock<Variables> {
    static INSTANCE: OnceLock<RwLock<Variables>> = OnceLock::new();
    
    INSTANCE.get_or_init(|| {
      RwLock::new(Variables::new())
    })
  }
}
