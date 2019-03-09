use crate::{
	game::Game,
	utils::compare_words,
	world::Direction
};
use std::fmt;
use super::CommandVec;

#[derive(PartialEq)]
pub enum Selectable {
	Command(usize),
	Direction(Direction),
	Nothing
}

pub trait Selector {
	fn get_error_message(&self) -> &str;
	
	fn get_name(&self) -> &str;
	
	fn is_optional(&self) -> bool;
	
	fn parse(&self, game: &Game, input: &str) -> (usize, Selectable);
}

impl fmt::Display for Selector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.is_optional() {
			true => write!(f, "[{}]", self.get_name()),
			false => write!(f, "{}", self.get_name())
		}
	}
}

pub struct CommandSelector;

impl Selector for CommandSelector {
	fn is_optional(&self) -> bool { true }
	
	fn get_error_message(&self) -> &str { "Please provide a valid command name." }
	
	fn get_name(&self) -> &str { "command" }
	
	fn parse(&self, game: &Game, input: &str) -> (usize, Selectable) {
		match game.commands.parse(input) {
			(0, _) => (0, Selectable::Nothing),
			(i, j) => (i, Selectable::Command(j))
		}
	}
}

pub struct DirectionSelector;

impl Selector for DirectionSelector {
	fn is_optional(&self) -> bool { false }
	
	fn get_error_message(&self) -> &str { "Please provide a valid direction." }
	
	fn get_name(&self) -> &str { "direction" }
	
	fn parse(&self, game: &Game, input: &str) -> (usize, Selectable) {
		let mut max_length = 0;
		let mut max_index = 0;
		for (i, dir) in vec!["north", "east", "south", "west"].iter().enumerate() {
			let cmp = compare_words(input, dir);
			if cmp > max_length {
				max_length = cmp;
				max_index = i;
			}
		}
		if max_length == 0 {
			return (0, Selectable::Nothing);
		}
		(max_length, Selectable::Direction(Direction::from(max_index)))
	}
}