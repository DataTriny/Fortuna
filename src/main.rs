#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate serde_derive;

pub mod actions;
pub mod game;
pub mod input;
pub mod utils;
pub mod world;

use game::Game;
use input::*;

fn main() {
	let mut game = Game::new();
	game.commands.push(Box::new(HelpCommand::new()));
	game.commands.push(Box::new(ExitCommand::default()));
	game.commands.push(Box::new(GoCommand::new()));
	game.commands.push(Box::new(LookCommand::default()));
	game.run();
}
