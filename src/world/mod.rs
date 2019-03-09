pub mod player;
pub mod rooms;

use crate::actions::PlayerAction;
use player::Player;
use ron::de::from_bytes;
use rooms::*;
use serde::Deserialize;

enum_from_primitive!{
	#[derive(Clone, Copy, Deserialize, Eq, PartialEq)]
	pub enum Direction {
		North,
		East,
		South,
		West
	}
}

#[derive(Deserialize)]
pub struct World {
	pub player: Player,
	rooms: Vec<Room>
}

impl World {
	pub fn new() -> Self {
		from_bytes(include_bytes!("../../world.ron")).unwrap()
	}
	
	pub fn get_room(&self, id: usize) -> &Room { &self.rooms[id] }
	
	pub fn perform_action(&mut self, action: PlayerAction) {
		match action {
			PlayerAction::MoveToRoom(room_id) => self.player.current_room = room_id,
			_ => { }
		}
	}
}