pub mod commands;
pub mod selectors;

use self::commands::*;
use super::utils::*;

pub struct CommandVec {
	commands: Vec<Box<Command>>
}

impl CommandVec {
	pub fn new() -> CommandVec {
		CommandVec { commands: Vec::new() }
	}
	
	pub fn add(&mut self, command: Box<Command>) {
		self.commands.push(command);
	}
	
	pub fn get_all(&self) -> &Vec<Box<Command>> { &self.commands }
	
	pub fn get_at(&self, i: usize) -> &Box<Command> { &self.commands[i] }
	
	pub fn parse(&self, input: &str) -> (usize, usize) {
		let mut max = 0;
		let mut max_index = 0;
		for (i, cmd) in self.commands.iter().enumerate() {
			let cmp = compare_words(input, cmd.get_name());
			if cmp > max {
				max = cmp;
				max_index = i;
			}
		}
		(max, max_index)
	}
}