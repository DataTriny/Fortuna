use super::Direction;

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

pub struct Room {
	description: String,
	pub exits: Vec<Exit>
}

impl Room {
	pub fn new(description: String, exits: Vec<Exit>) -> Room {
		Room { description, exits }
	}
	
	pub fn can_go_to(&self, dir: Direction) -> bool {
		for e in self.exits.iter() {
			if e.direction == dir {
				return true;
			}
		}
		false
	}
	
	pub fn get_description(&self) -> String {
		let mut description = self.description.clone();
		for e in self.exits.iter() {
			description.push_str(&format!("\n{}", &e.description));
		}
		description
	}
}