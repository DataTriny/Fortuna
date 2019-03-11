use crate::{
	game::Game,
	Named,
	utils::compare_words,
	world::{Direction, objects::Object}
};
use enum_primitive::FromPrimitive;
use super::InputParser;

#[derive(PartialEq)]
pub enum Selectable {
	Command(usize),
	Direction(Direction),
	Nothing,
	Object(usize)
}

pub struct CommandSelector;

impl InputParser for CommandSelector {
	fn parse(&self, game: &Game, input: &str) -> (usize, Selectable) {
		match game.commands.parse(game, input) {
			(0, _) => (0, Selectable::Nothing),
			(i, j) => (i, j)
		}
	}
}

pub struct DirectionSelector;

impl InputParser for DirectionSelector {
	fn parse(&self, game: &Game, input: &str) -> (usize, Selectable) {
		let mut max_index = 0;
		let mut max_length = 0;
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
		(max_length, Selectable::Direction(Direction::from_usize(max_index).unwrap()))
	}
}

pub struct CurrentPlaceObjectSelector;

impl InputParser for CurrentPlaceObjectSelector {
	fn parse(&self, game: &Game, input: &str) -> (usize, Selectable) {
		let mut max_index = 0;
		let mut max_length = 0;
		for (i, o) in game.world.get_place(game.world.player.current_place).objects.iter().enumerate() {
			let cmp = o.find(input);
			if cmp > max_length {
				max_index = i;
				max_length = cmp;
			}
		}
		if max_length == 0 {
			return (0, Selectable::Nothing);
		}
		(max_length, Selectable::Object(max_index))
	}
}