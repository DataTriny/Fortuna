pub mod actions;
pub mod game;
pub mod input;
pub mod utils;
pub mod world;

use self::game::Game;
use self::input::*;

fn main() {
	let mut game = Game::new();
	game.commands.push(Box::new(HelpCommand::new()));
	game.commands.push(Box::new(ExitCommand::default()));
	game.commands.push(Box::new(GoCommand::new()));
	game.commands.push(Box::new(LookCommand::default()));
	game.run();
}
