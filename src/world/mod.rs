pub mod objects;
pub mod places;
pub mod player;

use player::Player;
use ron::de::from_bytes;
use places::*;
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
	places: Vec<Place>
}

impl World {
	pub fn new() -> Self {
		from_bytes(include_bytes!("../../world.ron")).unwrap()
	}
	
	pub fn get_place(&self, id: usize) -> &Place { &self.places[id] }
	
	pub fn perform_action(&mut self, action: WorldAction) {
		match action {
			WorldAction::MoveToPlace(place_id) => self.player.current_place = place_id,
			WorldAction::TakeItem(object_id) => self.player.inventory.push(self.places[self.player.current_place].objects.remove(object_id))
		}
	}
}

pub enum WorldAction {
	MoveToPlace(usize),
	TakeItem(usize)
}