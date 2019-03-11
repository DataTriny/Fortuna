#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate serde_derive;

pub mod game;
pub mod input;
pub mod utils;
pub mod world;

use game::Game;
use input::*;
use utils::compare_words;

pub trait Named {
	fn get_aliases(&self) -> &[String];
	
	fn get_description(&self) -> &str;
	
	fn find(&self, input: &str) -> usize {
		let mut max_length = 0;
		for a in self.get_aliases() {
			let cmp = compare_words(input, a);
			if cmp > max_length {
				max_length = cmp;
			}
		}
		max_length
	}
}

fn main() {
	let mut game = Game::new();
	game.commands.push(Box::new(HelpCommand::new()));
	game.commands.push(Box::new(ExitCommand::new()));
	game.commands.push(Box::new(GoCommand::new()));
	game.commands.push(Box::new(InventoryCommand::new()));
	game.commands.push(Box::new(LookCommand::new()));
	game.commands.push(Box::new(TakeCommand::new()));
	game.run();
}
