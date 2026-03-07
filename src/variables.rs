use std::{collections::HashMap, sync::{RwLock, OnceLock}};

pub struct Variables {
  vars: HashMap<char, bool>
}

impl Variables {
  fn new() -> Self {
    Variables {
      vars: HashMap::new()
    }
  }

  pub fn set_var(&mut self, name: &char, value: bool) {
    self.vars.insert(*name, value);
  }

  pub fn get_var(&self, name: &char) -> Option<bool> {
    self.vars.get(name).copied()
  }

  pub fn get_all_vars(&self) -> &HashMap<char, bool> {
    &self.vars
  }

  pub fn get_instance() -> &'static RwLock<Variables> {
    static INSTANCE: OnceLock<RwLock<Variables>> = OnceLock::new();
    
    INSTANCE.get_or_init(|| {
      RwLock::new(Variables::new())
    })
  }
}
