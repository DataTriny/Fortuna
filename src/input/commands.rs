use crate::{
	actions::PlayerAction,
	game::Game,
	utils::compare_words
};
use std::fmt;
use super::selectors::*;

pub trait Command {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> PlayerAction;
	
	fn get_description(&self) -> &str;
	
	fn get_name(&self) -> &str;
	
	fn get_selectors(&self) -> &Vec<Box<Selector>>;
	
	fn get_signature(&self) -> String {
		let mut signature = String::from(self.get_name());
		for s in self.get_selectors() {
			signature.push_str(&format!(" {}", s));
		}
		signature
	}
}

impl fmt::Display for Command {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}\n\n{}", self.get_signature(), self.get_description())
	}
}

pub trait CommandVec {
	fn parse(&self, input: &str) -> (usize, usize);
}

impl CommandVec for Vec<Box<Command>> {
	fn parse(&self, input: &str) -> (usize, usize) {
		let mut max_length = 0;
		let mut max_index = 0;
		for (i, cmd) in self.iter().enumerate() {
			let cmp = compare_words(input, cmd.get_name());
			if cmp > max_length {
				max_length = cmp;
				max_index = i;
			}
		}
		(max_length, max_index)
	}
}

#[derive(Default)]
pub struct ExitCommand {
	selectors: Vec<Box<Selector>>
}

impl Command for ExitCommand {
	fn execute(&self, _: &Game, _: Vec<Selectable>) -> PlayerAction {
		println!("Goodbye!");
		PlayerAction::Exit
	}
	
	fn get_description(&self) -> &str { "Exit the game." }
	
	fn get_name(&self) -> &str { "exit" }
	
	fn get_selectors(&self) -> &Vec<Box<Selector>> { &self.selectors }
}

pub struct GoCommand {
	selectors: Vec<Box<Selector>>
}

impl GoCommand {
	pub fn new() -> Self {
		Self {
			selectors: vec![Box::new(DirectionSelector { })]
		}
	}
}

impl Command for GoCommand {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> PlayerAction {
		if let Selectable::Direction(dir) = args[0] {
			if game.world.get_room(game.world.player.current_room).can_go_to(dir) {
				return PlayerAction::Go(dir);
			}
			println!("You cannot go that way.");
			return PlayerAction::DoNothing;
		}
		PlayerAction::DoNothing
	}
	
	fn get_description(&self) -> &str { "Move in a given direction." }
	
	fn get_name(&self) -> &str { "go" }
	
	fn get_selectors(&self) -> &Vec<Box<Selector>> { &self.selectors }
}

pub struct HelpCommand {
	selectors: Vec<Box<Selector>>
}

impl HelpCommand {
	pub fn new() -> Self {
		Self {
			selectors: vec![Box::new(CommandSelector { })]
		}
	}
}

impl Command for HelpCommand {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> PlayerAction {
		if let Selectable::Command(i) = args[0] {
			println!("{}", game.commands[i]);
		}
		else {
			println!("To interact with your environment, type one of the following commands.");
			for command in game.commands.iter() {
				println!(" - {}", command.get_signature());
			}
		}
		PlayerAction::DoNothing
	}
	
	fn get_description(&self) -> &str { "Get a list of all available commands, or get more information on a particular one." }
	
	fn get_name(&self) -> &str { "help" }
	
	fn get_selectors(&self) -> &Vec<Box<Selector>> { &self.selectors }
}

#[derive(Default)]
pub struct LookCommand {
	selectors: Vec<Box<Selector>>
}

impl Command for LookCommand {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> PlayerAction {
		let room = game.world.get_room(game.world.player.current_room);
		println!("{}", room.get_description());
		PlayerAction::DoNothing
	}
	
	fn get_description(&self) -> &str { "Look around." }
	
	fn get_name(&self) -> &str { "look" }
	
	fn get_selectors(&self) -> &Vec<Box<Selector>> { &self.selectors }
}