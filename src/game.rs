use crate::{
	actions::PlayerAction,
	input::{Command, CommandVec, selectors::Selectable},
	world::World
};
use std::{
	cmp::min,
	io::{self, BufRead, Stdin},
};

pub struct Game {
	pub commands: Vec<Box<Command>>,
	is_running: bool,
	std_in: Stdin,
	pub world: World
}

impl Game {
	pub fn new() -> Self {
		Self {
			commands: Vec::new(),
			is_running: false,
			std_in: io::stdin(),
			world: World::new()
		}
	}
	
	fn get_user_input(&self) -> String {
		self.std_in.lock().lines().next().unwrap().unwrap().split_whitespace().collect::<Vec<&str>>().join(" ")
	}
	
	fn perform_action(&mut self, action: PlayerAction) {
		match action {
			PlayerAction::Exit => self.is_running = false,
			PlayerAction::DoNothing => { },
			_ => self.world.perform_action(action)
		}
	}
	
	pub fn run(&mut self) {
		self.is_running = true;
		self.welcome_player();
		while self.is_running {
			let input = &self.get_user_input();
			let parsed = self.commands.parse(input);
			if parsed.0 > 0 {
				let command = &self.commands[parsed.1];
				let mut args = Vec::new();
				let mut offset = min(parsed.0 + 1, input.len());
				let mut has_error = false;
				for selector in command.get_selectors() {
					let selected = selector.parse(self, &input[offset..]);
					if !selector.is_optional() && selected.1 == Selectable::Nothing {
						println!("Error: {}", selector.get_error_message());
						has_error = true;
						break;
					}
					args.push(selected.1);
					offset = min(offset + selected.0 + 1, input.len());
				}
				if !has_error {
					self.perform_action(command.execute(self, args));
				}
			}
			else {
				println!("Command not found.")
			}
		}
	}
	
	fn welcome_player(&self) {
		println!("Welcom to Fortuna!");
		println!("A text-based game by DataTriny.");
		println!("Type \"help\" to get a list of available commands.\n");
		println!("{}", self.world.get_room(self.world.player.current_room));
	}
}