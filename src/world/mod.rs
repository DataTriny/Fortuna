pub mod player;
pub mod rooms;

use self::player::Player;
use self::rooms::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
	North,
	East,
	South,
	West
}

impl From<usize> for Direction {
	fn from(value: usize) -> Direction {
		match value {
			0 => Direction::North,
			1 => Direction::East,
			2 => Direction::South,
			3 => Direction::West,
			_ => Direction::North
		}
	}
}

pub struct World {
	pub player: Player,
	rooms: Vec<Room>
}

impl World {
	pub fn new() -> World {
		World {
			player: Player::new(0),
			rooms: World::create_rooms()
		}
	}
	
	fn create_rooms() -> Vec<Room> {
		vec![
			Room::new(String::from("A kitchen"), vec![
				Exit::new(Direction::North, 1, String::from("There is an open door to the north."))
			]),
			Room::new(String::from("A livingroom"), vec![
				Exit::new(Direction::South, 0, String::from("There is an open door to the south."))
			])
		]
	}
	
	pub fn get_room(&self, id: usize) -> &Room { &self.rooms[id] }
}