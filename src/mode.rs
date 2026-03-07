use std::sync::{LazyLock, RwLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
	Default, Table
}

pub static MODE: LazyLock<RwLock<Mode>> = LazyLock::new(|| {
	RwLock::new(Mode::Default)
});

pub fn get_mode() -> Mode {
	*MODE.read().unwrap()
}

pub fn switch_mode(new_mode: Mode) {
	let mut mode = MODE.write().unwrap();
	*mode = new_mode;
}

pub fn get_mode_name() -> String {
	match get_mode() {
		Mode::Default => "default".to_string(),
		Mode::Table => "table".to_string()
	}
}
