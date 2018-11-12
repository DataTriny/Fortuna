pub mod actions;
pub mod game;
pub mod input;
pub mod utils;
pub mod world;

use self::game::Game;
use self::input::commands::*;

fn main() {
	let mut game = Game::new();
	game.commands.add(Box::new(ExitCommand::new()));
	game.commands.add(Box::new(HelpCommand::new()));
	game.commands.add(Box::new(GoCommand::new()));
	game.commands.add(Box::new(LookCommand::new()));
	game.run();
}
