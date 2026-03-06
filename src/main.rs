use std::io::{self, Write};

use crate::executors::get_executors;

mod global;
mod executors;
mod variables;
mod logic;
mod utils;

fn main() {
	loop {
		print!("{}", global::CLI_PREFIX);
		io::stdout().flush().unwrap();

		let mut line = String::new();
		io::stdin()
			.read_line(&mut line)
			.expect("Error: Failed to read line.");

		let input = line.trim();
		for executor in get_executors() {
			match executor.execute(input) {
				Ok(done) => {
					if done {
						break;
					}
				}
				Err(e) => {
					eprintln!("Error: {}", e);
					break;
				}
			}
		}
	}
}
