use crate::game::Game;
use crate::utils::*;
use crate::world::Direction;

pub enum Selectable {
	Command(usize),
	Direction(Direction),
	None
}

pub trait Selector {
	fn get(&self, game: &Game, input: &str) -> (usize, Selectable);
	
	fn get_name(&self) -> &str;
	
	fn is_optional(&self) -> bool;
}

pub struct CommandSelector { }

impl Selector for CommandSelector {
	fn get(&self, game: &Game, input: &str) -> (usize, Selectable) {
		match game.commands.parse(input) {
			(0, _) => (0, Selectable::None),
			(i, j) => (i, Selectable::Command(j))
		}
	}
	
	fn get_name(&self) -> &str { "command" }
	
	fn is_optional(&self) -> bool { true }
}

pub struct DirectionSelector { pub is_optional: bool }

impl Selector for DirectionSelector {
	fn get(&self, game: &Game, input: &str) -> (usize, Selectable) {
		let mut max = 0;
		let mut max_index = 0;
		for (i, dir) in vec!["north", "east", "south", "west"].iter().enumerate() {
			let cmp = compare_words(input, dir);
			if cmp > max {
				max = cmp;
				max_index = i;
			}
		}
		if max == 0 {
			return (0, Selectable::None);
		}
		(max, Selectable::Direction(Direction::from(max_index)))
	}
	
	fn get_name(&self) -> &str { "direction" }
	
	fn is_optional(&self) -> bool { self.is_optional }
}