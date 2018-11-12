use crate::actions::PlayerAction;
use crate::game::Game;
use super::selectors::*;

pub trait Command {
	fn execute(&self, game: &Game, args: Vec<Box<Selectable>>) -> PlayerAction;
	
	fn get_description(&self) -> String {
		let mut desc = String::from(self.get_name());
		for selector in self.get_selectors() {
			if selector.is_optional() {
				desc.push_str(&format!(" [{}]", selector.get_name()));
			}
			else {
				desc.push_str(&format!(" {}", selector.get_name()));
			}
		}
		desc
	}
	
	fn get_name(&self) -> &str;
	
	fn get_selectors(&self) -> &Vec<Box<Selector>>;
}

pub struct ExitCommand {
	name: String,
	selectors: Vec<Box<Selector>>
}

impl ExitCommand {
	pub fn new() -> ExitCommand {
		ExitCommand {
			name: String::from("exit"),
			selectors: Vec::new()
		}
	}
}

impl Command for ExitCommand {
	fn execute(&self, game: &Game, args: Vec<Box<Selectable>>) -> PlayerAction {
		println!("Goodbye!");
		PlayerAction::Exit
	}
	
	fn get_name(&self) -> &str { &self.name }
	
	fn get_selectors(&self) -> &Vec<Box<Selector>> { &self.selectors }
}

pub struct HelpCommand {
	name: String,
	selectors: Vec<Box<Selector>>
}

impl HelpCommand {
	pub fn new() -> HelpCommand {
		HelpCommand {
			name: String::from("help"),
			selectors: vec![Box::new(CommandSelector { })]
		}
	}
}

impl Command for HelpCommand {
	fn execute(&self, game: &Game, args: Vec<Box<Selectable>>) -> PlayerAction {
		if let Selectable::Command(i) = *args[0] {
			println!("{}", i);
		}
		else {
			println!("To interact with your environment, type one of the following commands.");
			for command in game.commands.get_all() {
				println!(" - {}", command.get_description());
			}
		}
		PlayerAction::DoNothing
	}
	
	fn get_name(&self) -> &str { &self.name }
	
	fn get_selectors(&self) -> &Vec<Box<Selector>> { &self.selectors }
}

pub struct GoCommand {
	name: String,
	selectors: Vec<Box<Selector>>
}

impl GoCommand {
	pub fn new() -> Self {
		Self {
			name: String::from("go"),
			selectors: vec![Box::new(DirectionSelector { is_optional: false })]
		}
	}
}

impl Command for GoCommand {
	fn execute(&self, game: &Game, args: Vec<Box<Selectable>>) -> PlayerAction {
		if let Selectable::Direction(dir) = *args[0] {
			if game.world.get_room(game.world.player.current_room).can_go_to(dir) {
				return PlayerAction::Go(dir);
			}
			println!("You cannot go that way.");
			return PlayerAction::DoNothing;
		}
		PlayerAction::DoNothing
	}
	
	fn get_name(&self) -> &str { &self.name }
	
	fn get_selectors(&self) -> &Vec<Box<Selector>> { &self.selectors }
}

pub struct LookCommand {
	name: String,
	selectors: Vec<Box<Selector>>
}

impl LookCommand {
	pub fn new() -> Self {
		Self {
			name: String::from("look"),
			selectors: Vec::new()
		}
	}
}

impl Command for LookCommand {
	fn execute(&self, game: &Game, args: Vec<Box<Selectable>>) -> PlayerAction {
		let room = game.world.get_room(game.world.player.current_room);
		println!("{}", room.get_description());
		PlayerAction::DoNothing
	}
	
	fn get_name(&self) -> &str { &self.name }
	
	fn get_selectors(&self) -> &Vec<Box<Selector>> { &self.selectors }
}