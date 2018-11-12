use crate::actions::PlayerAction;
use crate::input::selectors::Selectable;
use std::cmp::min;
use std::io::{self, BufRead};
use super::input::CommandVec;
use super::world::World;

pub struct Game {
	pub commands: CommandVec,
	pub is_running: bool,
	pub world: World
}

impl Game {
	pub fn new() -> Game {
		Game {
			commands: CommandVec::new(),
			is_running: false,
			world: World::new()
		}
	}
	
	pub fn exit(&mut self) {
		self.is_running = false;
	}
	
	fn perform_action(&mut self, action: PlayerAction) {
		match action {
			PlayerAction::Exit => self.exit(),
			PlayerAction::Go(dir) => {
				let current_room = self.world.get_room(self.world.player.current_room);
				for e in current_room.exits.iter() {
					if e.direction == dir {
						self.world.player.current_room = e.destination;
						println!("{}", self.world.get_room(self.world.player.current_room).get_description());
						break;
					}
				}
			}
			_ => { }
		};
	}
	
	pub fn run(&mut self) {
		let stdin = io::stdin();
		self.is_running = true;
		self.welcome_player();
		while self.is_running {
			let input = &stdin.lock().lines().next().unwrap().unwrap();
			let parsed = self.commands.parse(input);
			if parsed.0 > 0 {
				let command = self.commands.get_at(parsed.1);
				let mut args: Vec<Box<Selectable>> = Vec::new();
				let mut offset = min(parsed.0 + 1, input.len());
				for selector in command.get_selectors() {
					let selected = selector.get(self, &input[offset..]);
					args.push(Box::new(selected.1));
				}
				self.perform_action(command.execute(self, args));
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
		println!("{}", self.world.get_room(self.world.player.current_room).get_description());
	}
}