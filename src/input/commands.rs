use crate::{
	game::{Game, GameAction},
	Named,
	world::{
		objects::{Exit, ObjectKind},
		WorldAction
	}
};
use std::fmt;
use super::{InputParser, selectors::*};

pub struct Parameter {
	description: String,
	pub error_message: Option<String>,
	pub is_optional: bool,
	name: String,
	selector: Box<InputParser>
}

impl Parameter {
	fn get_signature(&self) -> String {
		match self.is_optional {
			true => format!("[{}]", self.name),
			false => self.name.clone()
		}
	}
	
	pub fn optional(name: String, description: String, selector: Box<InputParser>) -> Self {
		Self { description, error_message: None, is_optional: true, name, selector }
	}
	
	pub fn required(name: String, description: String, error_message: String, selector: Box<InputParser>) -> Self {
		Self { description, error_message: Some(error_message), is_optional: false, name, selector }
	}
}

impl fmt::Display for Parameter {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name)?;
		if self.is_optional {
			write!(f, " (optional)")?;
		}
		write!(f, ": {}", self.description)
	}
	
}

impl InputParser for Parameter {
	fn parse(&self, game: &Game, input: &str) -> (usize, Selectable) {
		self.selector.parse(game, input)
	}
}

pub trait Command: Named {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> GameAction;
	
	fn get_parameters(&self) -> &[Parameter];
	
	fn get_signature(&self) -> String {
		let mut signature = self.get_aliases()[0].clone();
		for p in self.get_parameters() {
			signature.push_str(&format!(" {}", p.get_signature()));
		}
		signature
	}
}

impl fmt::Display for Command {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}\n\n{}", self.get_signature(), self.get_description())?;
		let parameters = self.get_parameters();
		if parameters.len() > 0 {
			write!(f, "\n")?;
			for p in parameters {
				write!(f, "\n{}", p)?;
			}
		}
		Ok(())
	}
}

pub struct ExitCommand {
	aliases: Vec<String>,
	description: String,
	parameters: Vec<Parameter>
}

impl ExitCommand {
	pub fn new() -> Self {
		Self {
			aliases: vec!["exit".to_string(), "quit".to_string()],
			description: "Exit the game.".to_string(),
			parameters: vec![]
		}
	}
}

impl Command for ExitCommand {
	fn execute(&self, _: &Game, _: Vec<Selectable>) -> GameAction {
		println!("Goodbye!");
		GameAction::Exit
	}
	
	fn get_parameters(&self) -> &[Parameter] { &self.parameters }
}

impl Named for ExitCommand {
	fn get_aliases(&self) -> &[String] { &self.aliases }
	
	fn get_description(&self) -> &str { &self.description }
}

pub struct GoCommand {
	aliases: Vec<String>,
	description: String,
	parameters: Vec<Parameter>
}

impl GoCommand {
	pub fn new() -> Self {
		Self {
			aliases: vec!["go".to_string(), "walk".to_string()],
			description: "Move in a given direction.".to_string(),
			parameters: vec![Parameter::required(
				"direction".to_string(), "The direction in which you want to go.".to_string(), "Go where?".to_string(), Box::new(DirectionSelector { }))]
		}
	}
}

impl Command for GoCommand {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> GameAction {
		if let Selectable::Direction(dir) = args[0] {
			if let Some(exit) = game.world.get_place(game.world.player.current_place).get_exit(dir) {
				if let ObjectKind::Exit(Exit { destination, .. }) = exit.kind {
					println!("{}", game.world.get_place(destination));
					return GameAction::WorldAction(WorldAction::MoveToPlace(destination));
				}
			}
			println!("You cannot go that way.");
		}
		GameAction::DoNothing
	}
	
	fn get_parameters(&self) -> &[Parameter] { &self.parameters }
}

impl Named for GoCommand {
	fn get_aliases(&self) -> &[String] { &self.aliases }
	
	fn get_description(&self) -> &str { &self.description }
}

pub struct HelpCommand {
	aliases: Vec<String>,
	description: String,
	parameters: Vec<Parameter>
}

impl HelpCommand {
	pub fn new() -> Self {
		Self {
			aliases: vec!["help".to_string()],
			description: "Get a list of all available commands, or get more information on a particular one.".to_string(),
			parameters: vec![Parameter::optional(
				"command".to_string(), "The command for which you want to get some help.".to_string(), Box::new(CommandSelector { }))]
		}
	}
}

impl Command for HelpCommand {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> GameAction {
		if let Selectable::Command(i) = args[0] {
			println!("{}", game.commands[i]);
		}
		else {
			println!("To interact with your environment, type one of the following commands.");
			for command in game.commands.iter() {
				println!(" - {}", command.get_signature());
			}
		}
		GameAction::DoNothing
	}
	
	fn get_parameters(&self) -> &[Parameter] { &self.parameters }
}

impl Named for HelpCommand {
	fn get_aliases(&self) -> &[String] { &self.aliases }
	
	fn get_description(&self) -> &str { &self.description }
}

pub struct InventoryCommand {
	aliases: Vec<String>,
	description: String,
	parameters: Vec<Parameter>
}

impl InventoryCommand {
	pub fn new() -> Self {
		Self {
			aliases: vec!["inventory".to_string()],
			description: "Look at what you are carrying.".to_string(),
			parameters: vec![]
		}
	}
}

impl Command for InventoryCommand {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> GameAction {
		if game.world.player.inventory.len() > 0 {
			println!("You are carrying:");
			for o in game.world.player.inventory.iter() {
				println!(" - {}", o.get_aliases()[0]);
			}
		}
		else {
			println!("You are not carrying anything.");
		}
		GameAction::DoNothing
	}
	
	fn get_parameters(&self) -> &[Parameter] { &self.parameters }
}

impl Named for InventoryCommand {
	fn get_aliases(&self) -> &[String] { &self.aliases }
	
	fn get_description(&self) -> &str { &self.description }
}

pub struct LookCommand {
	aliases: Vec<String>,
	description: String,
	parameters: Vec<Parameter>
}

impl LookCommand {
	pub fn new() -> Self {
		Self {
			aliases: vec!["look".to_string()],
			description: "Look around.".to_string(),
			parameters: vec![]
		}
	}
}

impl Command for LookCommand {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> GameAction {
		println!("{}", game.world.get_place(game.world.player.current_place));
		GameAction::DoNothing
	}
	
	fn get_parameters(&self) -> &[Parameter] { &self.parameters }
}

impl Named for LookCommand {
	fn get_aliases(&self) -> &[String] { &self.aliases }
	
	fn get_description(&self) -> &str { &self.description }
}

pub struct TakeCommand {
	aliases: Vec<String>,
	description: String,
	parameters: Vec<Parameter>
}

impl TakeCommand {
	pub fn new() -> Self {
		Self {
			aliases: vec!["take".to_string(), "grab".to_string()],
			description: "Take an item nearby you.".to_string(),
			parameters: vec![
				Parameter::required("item".to_string(), "The name of the item you want to take.".to_string(), "What do you want to take?".to_string(), Box::new(CurrentPlaceObjectSelector { }))]
		}
	}
}

impl Command for TakeCommand {
	fn execute(&self, game: &Game, args: Vec<Selectable>) -> GameAction {
		if let Selectable::Object(object_id) = args[0] {
			let object = &game.world.get_place(game.world.player.current_place).objects[object_id];
			if object.carryable {
				println!("You took {}.", object.get_aliases()[0]);
				return GameAction::WorldAction(WorldAction::TakeItem(object_id))
			}
		}
		println!("You can't take that!");
		GameAction::DoNothing
	}
	
	fn get_parameters(&self) -> &[Parameter] { &self.parameters }
}

impl Named for TakeCommand {
	fn get_aliases(&self) -> &[String] { &self.aliases }
	
	fn get_description(&self) -> &str { &self.description }
}