use std::fmt;
use super::Direction;

#[derive(Deserialize)]
pub struct Exit {
	pub description: String,
	pub destination: usize,
	pub direction: Direction
}

impl Exit {
	pub fn new(direction: Direction, destination: usize, description: String) -> Self {
		Self { description, destination, direction }
	}
}

#[derive(Deserialize)]
pub struct Room {
	description: String,
	pub exits: Vec<Exit>
}

impl Room {
	pub fn new(description: String, exits: Vec<Exit>) -> Room {
		Room { description, exits }
	}
	
	pub fn get_exit(&self, dir: Direction) -> Option<&Exit> {
		for e in self.exits.iter() {
			if e.direction == dir {
				return Some(e);
			}
		}
		None
	}
}

impl fmt::Display for Room {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description)?;
		for e in self.exits.iter() {
			write!(f, "\n{}", e.description)?;
		}
		Ok(())
	}
}