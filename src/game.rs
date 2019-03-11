use crate::{
	input::{Command, InputParser, selectors::Selectable},
	world::{WorldAction, World}
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
	
	fn perform_action(&mut self, action: GameAction) {
		match action {
			GameAction::DoNothing => { },
			GameAction::Exit => self.is_running = false,
			GameAction::WorldAction(action) => self.world.perform_action(action)
		}
	}
	
	pub fn run(&mut self) {
		self.is_running = true;
		self.welcome_player();
		while self.is_running {
			let input = &self.get_user_input();
			let parsed = self.commands.parse(self, input);
			if let Selectable::Command(command_id) = parsed.1 {
				let command = &self.commands[command_id];
				let mut args = Vec::new();
				let mut offset = min(parsed.0 + 1, input.len());
				let mut has_error = false;
				for parameter in command.get_parameters() {
					let selected = parameter.parse(self, &input[offset..]);
					if !parameter.is_optional && selected.1 == Selectable::Nothing {
						println!("{}", parameter.error_message.as_ref().unwrap());
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
		println!("{}", self.world.get_place(self.world.player.current_place));
	}
}

pub enum GameAction {
	DoNothing,
	Exit,
	WorldAction(WorldAction)
}