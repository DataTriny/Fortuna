pub mod commands;
pub mod selectors;

use crate::game::Game;
use selectors::Selectable;

pub trait InputParser {
	fn parse(&self, game: &Game, input: &str) -> (usize, Selectable);
}

impl InputParser for Vec<Box<Command>> {
	fn parse(&self, _: &Game, input: &str) -> (usize, Selectable) {
		let mut max_length = 0;
		let mut max_index = 0;
		for (i, cmd) in self.iter().enumerate() {
			let cmp = cmd.find(input);
			if cmp > max_length {
				max_length = cmp;
				max_index = i;
			}
		}
		if max_length == 0 {
			return (0, Selectable::Nothing);
		}
		(max_length, Selectable::Command(max_index))
	}
}

pub use commands::*;